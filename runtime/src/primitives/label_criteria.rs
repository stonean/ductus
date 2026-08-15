//! `label-criteria` — assign stable `AC{n}:` labels to acceptance criteria.
//!
//! The labelling pass [013 — Text-First Artifacts][013] specifies: every
//! unlabelled criterion receives the next label, already-labelled criteria
//! are left byte-identical, and the spec's `next-criterion` frontmatter
//! field records the label the next assignment will use.
//!
//! Assignment takes `max(highest label in body, next-criterion)` rather
//! than the body maximum alone. That is the whole retirement mechanism:
//! deleting the highest-numbered criterion lowers the body maximum, and
//! without the stored counter the next assignment would reissue a retired
//! label to a different requirement. The counter never decreases, so it
//! cannot.
//!
//! [013]: <https://github.com/stonean/govern/blob/main/specs/013-text-first-artifacts/scenarios/criterion-identifiers.md>

use std::path::Path;

use crate::primitives::{
    PrimitiveError, Result, read_text, rel_path, section_line_indices, write_atomic,
};
use crate::schema::paths;
use crate::schema::primitives::{LabelAssignment, LabelCriteriaArgs, LabelCriteriaResult};

use super::checkbox::find_checkbox_line;

const ACCEPTANCE_HEADING: &str = "Acceptance Criteria";
const NEXT_FIELD: &str = "next-criterion:";

/// Execute the `label-criteria` primitive.
///
/// Idempotent: running it twice in a row produces no second write, because
/// the first run leaves nothing unlabelled and the counter already exceeds
/// every label in the body.
///
/// # Errors
///
/// Returns [`PrimitiveError::FeatureNotFound`] when the feature directory
/// is missing, [`PrimitiveError::MissingFrontmatter`] when `spec.md` has no
/// leading `---` block, [`PrimitiveError::InvalidNextCriterion`] when the
/// stored counter is not a positive integer (a corrupted counter may mean a
/// label was already reissued, so the pass refuses rather than repairing it
/// in place), or [`PrimitiveError::Io`] for filesystem failures.
pub fn run(args: &LabelCriteriaArgs, repo: &Path) -> Result<LabelCriteriaResult> {
    super::validate_no_traversal(&args.feature)?;
    let root = paths::Paths::load(repo).specs_root;
    let feature_dir = repo.join(&root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root: root.clone(),
            feature: args.feature.clone(),
        });
    }
    let spec_path = feature_dir.join("spec.md");
    let content = read_text(&spec_path)?;
    let lines: Vec<&str> = content.split_inclusive('\n').collect();

    let fm = frontmatter_bounds(&lines).ok_or_else(|| PrimitiveError::MissingFrontmatter {
        path: spec_path.clone(),
    })?;
    let stored = read_next_criterion(&lines, &fm, &spec_path)?;

    // Criteria in body order, paired with any label they already carry.
    let criteria: Vec<(usize, usize, Option<u32>)> =
        section_line_indices(&lines, ACCEPTANCE_HEADING)
            .into_iter()
            .filter_map(|idx| {
                find_checkbox_line(lines[idx]).map(|(_bracket, marker_idx)| {
                    (idx, marker_idx, parse_label(lines[idx], marker_idx))
                })
            })
            .collect();

    // A spec with no criteria has nothing to label and gains no counter —
    // an absent `next-criterion` means "no labels assigned yet", which is a
    // truthful state rather than a defect to pre-empt.
    if criteria.is_empty() {
        return Ok(LabelCriteriaResult {
            assigned: Vec::new(),
            next_criterion: stored.unwrap_or(1),
            path: rel_path(&spec_path, repo),
            changed: false,
        });
    }

    let body_max = criteria.iter().filter_map(|(_, _, label)| *label).max();
    let mut counter = match (body_max, stored) {
        (Some(max_label), Some(next)) => next.max(max_label + 1),
        (Some(max_label), None) => max_label + 1,
        (None, Some(next)) => next,
        (None, None) => 1,
    };

    let mut assigned = Vec::new();
    let mut labelled_lines: Vec<(usize, String)> = Vec::new();
    for (criterion_index, (line_idx, marker_idx, existing)) in criteria.iter().enumerate() {
        if existing.is_some() {
            continue;
        }
        let label = counter;
        counter += 1;
        labelled_lines.push((
            *line_idx,
            insert_label(lines[*line_idx], *marker_idx, label),
        ));
        assigned.push(LabelAssignment {
            label: format!("AC{label}"),
            criterion_index,
        });
    }

    let counter_current = stored == Some(counter);
    if assigned.is_empty() && counter_current {
        return Ok(LabelCriteriaResult {
            assigned,
            next_criterion: counter,
            path: rel_path(&spec_path, repo),
            changed: false,
        });
    }

    let new_content = render(&lines, &fm, &labelled_lines, counter);
    write_atomic(&spec_path, &new_content)?;

    Ok(LabelCriteriaResult {
        assigned,
        next_criterion: counter,
        path: rel_path(&spec_path, repo),
        changed: true,
    })
}

/// Frontmatter block bounds as `(opening_index, closing_index)` over the
/// line vector, or `None` when the file has no leading `---` fence.
struct Frontmatter {
    closing: usize,
    newline: &'static str,
}

fn frontmatter_bounds(lines: &[&str]) -> Option<Frontmatter> {
    let first = lines.first()?;
    if first.trim_end() != "---" {
        return None;
    }
    let closing = lines
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, line)| line.trim_end() == "---")
        .map(|(idx, _)| idx)?;
    let newline = if lines[closing].ends_with("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    Some(Frontmatter { closing, newline })
}

/// Read the stored counter. A present-but-unparseable value is an error
/// rather than a value to overwrite: the pass repairing it silently could
/// hide that a retired label was already reissued.
fn read_next_criterion(lines: &[&str], fm: &Frontmatter, spec_path: &Path) -> Result<Option<u32>> {
    for line in lines.iter().take(fm.closing).skip(1) {
        let Some(rest) = line.trim_start().strip_prefix(NEXT_FIELD) else {
            continue;
        };
        let value = rest.trim();
        return match value.parse::<u32>() {
            Ok(parsed) if parsed >= 1 => Ok(Some(parsed)),
            _ => Err(PrimitiveError::InvalidNextCriterion {
                path: spec_path.to_path_buf(),
                value: value.to_string(),
            }),
        };
    }
    Ok(None)
}

/// Parse the `AC{n}:` label immediately after the checkbox, if present.
/// Anchored there so prose elsewhere in the line — "supersedes AC5 of 017"
/// — is never mistaken for this criterion's own label.
///
/// Shared with `mark-criterion` and `read-spec` so every surface answering
/// "what label does this criterion carry?" answers it identically. A second
/// implementation is the drift this project has already been bitten by once
/// (see `framework/commands/amend.md`'s reconcile pass and the
/// `scenario-consistency` matching rule).
pub(crate) fn parse_label(line: &str, marker_idx: usize) -> Option<u32> {
    let after_bracket = line.get(marker_idx + 2..)?;
    let rest = after_bracket.trim_start();
    let digits = rest.strip_prefix("AC")?;
    let end = digits.find(':')?;
    digits.get(..end)?.parse::<u32>().ok()
}

/// Insert `AC{label}: ` between the checkbox and the criterion's text,
/// leaving the text itself untouched.
fn insert_label(line: &str, marker_idx: usize, label: u32) -> String {
    let split_at = marker_idx + 2;
    let (head, tail) = line.split_at(split_at);
    let trimmed = tail.trim_start_matches([' ', '\t']);
    format!("{head} AC{label}: {trimmed}")
}

/// Rebuild the file with the labelled criterion lines spliced in and the
/// counter written into the frontmatter — replacing an existing
/// `next-criterion:` line, or inserting one just above the closing fence.
fn render(lines: &[&str], fm: &Frontmatter, labelled: &[(usize, String)], counter: u32) -> String {
    let counter_line = format!("next-criterion: {counter}{}", fm.newline);
    let mut out = String::with_capacity(lines.iter().map(|l| l.len()).sum::<usize>() + 32);
    let mut wrote_counter = false;
    for (idx, line) in lines.iter().enumerate() {
        if idx > 0 && idx < fm.closing && line.trim_start().starts_with(NEXT_FIELD) {
            out.push_str(&counter_line);
            wrote_counter = true;
            continue;
        }
        if idx == fm.closing && !wrote_counter {
            out.push_str(&counter_line);
            wrote_counter = true;
        }
        if let Some((_, replacement)) = labelled.iter().find(|(target, _)| *target == idx) {
            out.push_str(replacement);
        } else {
            out.push_str(line);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn write_spec(tmp: &Path, body: &str) {
        let feature_dir = tmp.join("specs/feat");
        fs::create_dir_all(&feature_dir).unwrap();
        fs::write(feature_dir.join("spec.md"), body).unwrap();
    }

    fn args() -> LabelCriteriaArgs {
        LabelCriteriaArgs {
            feature: "feat".into(),
        }
    }

    fn spec_text(tmp: &Path) -> String {
        fs::read_to_string(tmp.join("specs/feat/spec.md")).unwrap()
    }

    const UNLABELLED: &str = "---\nstatus: in-progress\ndependencies: []\n---\n\n# feat\n\n## Acceptance Criteria\n\n- [ ] First.\n- [x] Second.\n- [ ] Third.\n\n## Non-Goals\n\n- [ ] Not a criterion.\n";

    #[test]
    fn labels_every_unlabelled_criterion_in_body_order() {
        let tmp = tempdir().unwrap();
        write_spec(tmp.path(), UNLABELLED);
        let result = run(&args(), tmp.path()).unwrap();

        assert!(result.changed);
        assert_eq!(result.next_criterion, 4);
        assert_eq!(
            result
                .assigned
                .iter()
                .map(|a| a.label.as_str())
                .collect::<Vec<_>>(),
            vec!["AC1", "AC2", "AC3"]
        );
        let text = spec_text(tmp.path());
        assert!(text.contains("- [ ] AC1: First."));
        assert!(text.contains("- [x] AC2: Second."));
        assert!(text.contains("- [ ] AC3: Third."));
        assert!(text.contains("next-criterion: 4"));
        // The checkbox outside the section is not a criterion.
        assert!(text.contains("- [ ] Not a criterion."));
    }

    #[test]
    fn a_second_run_writes_nothing() {
        let tmp = tempdir().unwrap();
        write_spec(tmp.path(), UNLABELLED);
        run(&args(), tmp.path()).unwrap();
        let after_first = spec_text(tmp.path());

        let result = run(&args(), tmp.path()).unwrap();
        assert!(!result.changed, "idempotent: {:?}", result.assigned);
        assert!(result.assigned.is_empty());
        assert_eq!(spec_text(tmp.path()), after_first);
    }

    #[test]
    fn existing_labels_are_never_renumbered() {
        let tmp = tempdir().unwrap();
        write_spec(
            tmp.path(),
            "---\nstatus: done\ndependencies: []\nnext-criterion: 25\n---\n\n# feat\n\n## Acceptance Criteria\n\n- [x] AC7: Labelled by hand.\n- [ ] Unlabelled.\n- [x] AC24: Also by hand.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();

        assert_eq!(result.assigned.len(), 1);
        assert_eq!(result.assigned[0].label, "AC25");
        assert_eq!(result.assigned[0].criterion_index, 1);
        let text = spec_text(tmp.path());
        assert!(text.contains("- [x] AC7: Labelled by hand."));
        assert!(text.contains("- [x] AC24: Also by hand."));
        assert!(text.contains("- [ ] AC25: Unlabelled."));
        assert!(text.contains("next-criterion: 26"));
    }

    #[test]
    fn deleting_the_highest_criterion_does_not_reissue_its_label() {
        // The retirement mechanism: body max drops to 2, the stored
        // counter stands at 4, so the next criterion is AC4 — never a
        // second AC3 pointing at a different requirement.
        let tmp = tempdir().unwrap();
        write_spec(
            tmp.path(),
            "---\nstatus: in-progress\ndependencies: []\nnext-criterion: 4\n---\n\n# feat\n\n## Acceptance Criteria\n\n- [x] AC1: First.\n- [x] AC2: Second.\n- [ ] Added after AC3 was deleted.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();

        assert_eq!(result.assigned[0].label, "AC4");
        assert_eq!(result.next_criterion, 5);
        // No criterion carries AC3 as its label. The prose mentioning the
        // deleted AC3 stays, which is the point of anchoring the label
        // immediately after the checkbox.
        assert!(!spec_text(tmp.path()).contains("] AC3:"));
    }

    #[test]
    fn a_spec_emptied_of_criteria_keeps_its_counter() {
        let tmp = tempdir().unwrap();
        write_spec(
            tmp.path(),
            "---\nstatus: in-progress\ndependencies: []\nnext-criterion: 12\n---\n\n# feat\n\n## Acceptance Criteria\n\n- [ ] The only one left.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();

        assert_eq!(result.assigned[0].label, "AC12");
        assert!(spec_text(tmp.path()).contains("next-criterion: 13"));
    }

    #[test]
    fn no_criteria_section_writes_nothing() {
        let tmp = tempdir().unwrap();
        write_spec(
            tmp.path(),
            "---\nstatus: draft\ndependencies: []\n---\n\n# feat\n\n## Motivation\n\nProse only.\n",
        );
        let before = spec_text(tmp.path());
        let result = run(&args(), tmp.path()).unwrap();

        assert!(!result.changed);
        assert!(result.assigned.is_empty());
        assert_eq!(spec_text(tmp.path()), before, "no counter is created");
    }

    #[test]
    fn prose_naming_another_specs_label_is_not_read_as_one() {
        let tmp = tempdir().unwrap();
        write_spec(
            tmp.path(),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# feat\n\n## Acceptance Criteria\n\n- [ ] Supersedes AC5: the rule in 017 no longer applies.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();

        assert_eq!(result.assigned[0].label, "AC1");
        assert!(
            spec_text(tmp.path()).contains("- [ ] AC1: Supersedes AC5: the rule in 017"),
            "the label anchors after the checkbox, not at the first AC-shaped token"
        );
    }

    #[test]
    fn a_corrupted_counter_is_refused_not_repaired() {
        let tmp = tempdir().unwrap();
        write_spec(
            tmp.path(),
            "---\nstatus: in-progress\ndependencies: []\nnext-criterion: zero\n---\n\n# feat\n\n## Acceptance Criteria\n\n- [ ] First.\n",
        );
        let before = spec_text(tmp.path());
        let err = run(&args(), tmp.path()).unwrap_err();

        assert!(matches!(err, PrimitiveError::InvalidNextCriterion { .. }));
        assert_eq!(spec_text(tmp.path()), before);
    }

    #[test]
    fn a_spec_without_frontmatter_is_refused() {
        let tmp = tempdir().unwrap();
        write_spec(
            tmp.path(),
            "# feat\n\n## Acceptance Criteria\n\n- [ ] First.\n",
        );
        let err = run(&args(), tmp.path()).unwrap_err();

        assert!(matches!(err, PrimitiveError::MissingFrontmatter { .. }));
    }

    #[test]
    fn crlf_specs_keep_their_line_endings() {
        let tmp = tempdir().unwrap();
        write_spec(
            tmp.path(),
            "---\r\nstatus: in-progress\r\ndependencies: []\r\n---\r\n\r\n# feat\r\n\r\n## Acceptance Criteria\r\n\r\n- [ ] First.\r\n",
        );
        run(&args(), tmp.path()).unwrap();

        let text = spec_text(tmp.path());
        assert!(text.contains("next-criterion: 2\r\n"), "{text:?}");
        assert!(!text.contains("next-criterion: 2\n"));
        assert!(text.contains("- [ ] AC1: First.\r\n"));
    }
}
