//! `label-criteria` — assign stable `AC{n}:` labels to acceptance criteria.
//!
//! The labelling pass [013 — Text-First Artifacts][013] specifies: every
//! unlabelled criterion receives the next label, already-labelled criteria
//! are left byte-identical, and the spec's `next-criterion` frontmatter
//! field records the label the next assignment will use.
//!
//! Assignment takes `max(highest label in body + 1, next-criterion)` rather
//! than one past the body maximum alone. That is the whole retirement mechanism:
//! deleting the highest-numbered criterion lowers the body maximum, and
//! without the stored counter the next assignment would reissue a retired
//! label to a different requirement. The counter never decreases, so it
//! cannot.
//!
//! [013]: <https://github.com/stonean/ductus/blob/main/specs/013-text-first-artifacts/scenarios/criterion-identifiers.md>

use std::path::Path;

use crate::primitives::{PrimitiveError, Result, guarded, rel_path, section_line_indices};
use crate::schema::paths;
use crate::schema::primitives::{
    LabelAssignment, LabelCriteriaArgs, LabelCriteriaResult, ListsForm,
};

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
/// leading `---` block, [`PrimitiveError::UnclosedFrontmatter`] when its block
/// never closes, [`PrimitiveError::InvalidNextCriterion`] when the
/// stored counter is not a positive integer (a corrupted counter may mean a
/// label was already reissued, so the pass refuses rather than repairing it
/// in place), [`PrimitiveError::ChangedSinceRead`] when `spec.md` changed
/// between the read and the write (nothing is written; run it again), or
/// [`PrimitiveError::Io`] for filesystem failures.
pub fn run(args: &LabelCriteriaArgs, repo: &Path) -> Result<LabelCriteriaResult> {
    let root = paths::Paths::load(repo)?.specs_root;
    super::validate_feature_argument(repo, &root, &args.feature)?;
    let feature_dir = repo.join(&root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root: root.clone(),
            feature: args.feature.clone(),
        });
    }
    let spec_path = feature_dir.join("spec.md");
    let (snapshot, content) = guarded::read_required(repo, &rel_path(&spec_path, repo))?;
    let lines: Vec<&str> = content.split_inclusive('\n').collect();

    let fm = frontmatter_bounds(&lines).ok_or_else(|| {
        let path = spec_path.clone();
        if lines.first().is_some_and(|first| first.trim_end() == "---") {
            PrimitiveError::UnclosedFrontmatter { path }
        } else {
            PrimitiveError::MissingFrontmatter { path }
        }
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
        return Ok(result(
            Vec::new(),
            stored.unwrap_or(1),
            rel_path(&spec_path, repo),
            false,
        ));
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
        return Ok(result(assigned, counter, rel_path(&spec_path, repo), false));
    }

    let new_content = render(&lines, &fm, &labelled_lines, counter);
    // Bounded before the write, so a list file that cannot be written or a
    // malformed read size refuses the call with the spec untouched.
    let mut labelled = result(assigned, counter, rel_path(&spec_path, repo), true);
    bound(repo, &args.feature, &mut labelled)?;
    guarded::write_guarded(&snapshot, &new_content)?;
    Ok(labelled)
}

/// The result with `assigned` inline.
fn result(
    assigned: Vec<LabelAssignment>,
    next_criterion: u32,
    path: String,
    changed: bool,
) -> LabelCriteriaResult {
    LabelCriteriaResult {
        assigned_count: assigned.len(),
        assigned: Some(assigned),
        next_criterion,
        path,
        changed,
        form: ListsForm::Inline,
        lists_file: None,
    }
}

/// `result` held to `[artifacts] read-size-bytes`: past it `assigned`, one
/// entry per criterion labelled, moves whole to the file `lists-file` names
/// and only its count stays inline, as `read-spec`'s criteria do
/// (`bounded_lists::place`; `BE-PAYLOAD-001`, 022 task 42).
fn bound(repo: &Path, feature: &str, result: &mut LabelCriteriaResult) -> Result<()> {
    let lists = serde_json::json!({ "assigned": &result.assigned });
    if let Some(file) =
        super::bounded_lists::place(repo, "label-criteria", feature, &*result, &lists)?
    {
        result.assigned = None;
        result.form = ListsForm::File;
        result.lists_file = Some(file);
    }
    Ok(())
}

/// Where a spec's frontmatter closes, and the line ending that closing line
/// uses.
struct Frontmatter {
    closing: usize,
    newline: &'static str,
}

/// The frontmatter block's bounds, or `None` when the file has no leading `---`
/// fence or no closing one.
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

/// The `next-criterion:` field as it stands in a spec's frontmatter.
///
/// Shared with `check-artifacts`'s `criterion-labels` family so the pass
/// that *maintains* the counter and the audit that *validates* it read it
/// through one parser, for the same reason [`parse_label`] is shared: two
/// implementations of one grammar disagree eventually, and here they would
/// disagree about whether a label can be reissued.
pub(crate) enum StoredCounter {
    /// No `next-criterion:` line. The spec has never been through the pass,
    /// which 013 defines as "no labels assigned yet" rather than a defect.
    Absent,
    /// A positive integer.
    Valid(u32),
    /// Present but not a positive integer, carrying the raw text so a
    /// caller can report what it found rather than what it expected.
    Malformed(String),
}

/// Read the counter from a spec's full text.
///
/// A file with no frontmatter fence carries no counter, which reads as
/// [`StoredCounter::Absent`] — the labelling pass refuses such a spec on
/// its own account, before it ever asks for the counter.
pub(crate) fn stored_counter(content: &str) -> StoredCounter {
    let lines: Vec<&str> = content.split_inclusive('\n').collect();
    frontmatter_bounds(&lines).map_or(StoredCounter::Absent, |fm| scan_counter(&lines, fm.closing))
}

/// Scan the frontmatter block for `next-criterion:`.
fn scan_counter(lines: &[&str], closing: usize) -> StoredCounter {
    for line in lines.iter().take(closing).skip(1) {
        let Some(rest) = line.trim_start().strip_prefix(NEXT_FIELD) else {
            continue;
        };
        let value = rest.trim();
        return match value.parse::<u32>() {
            Ok(parsed) if parsed >= 1 => StoredCounter::Valid(parsed),
            _ => StoredCounter::Malformed(value.to_string()),
        };
    }
    StoredCounter::Absent
}

/// Read the stored counter for the pass itself. A present-but-unparseable
/// value is an error rather than a value to overwrite: the pass repairing
/// it silently could hide that a retired label was already reissued. The
/// audit reports the same state as a finding instead — assignment refuses,
/// enforcement reports.
fn read_next_criterion(lines: &[&str], fm: &Frontmatter, spec_path: &Path) -> Result<Option<u32>> {
    match scan_counter(lines, fm.closing) {
        StoredCounter::Absent => Ok(None),
        StoredCounter::Valid(parsed) => Ok(Some(parsed)),
        StoredCounter::Malformed(value) => Err(PrimitiveError::InvalidNextCriterion {
            path: spec_path.to_path_buf(),
            value,
        }),
    }
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
///
/// A checkbox with no text yet (`- [ ]`, a shape the shared checkbox parser
/// accepts) takes `AC{label}:` with **no** trailing space: the separating
/// space would sit at end of line, which `lint-markdown` flags as MD009 —
/// and in `/{project}:specify` that lint runs on the very next step, so the
/// pass would hand the host a violation it had just created.
fn insert_label(line: &str, marker_idx: usize, label: u32) -> String {
    let split_at = marker_idx + 2;
    let (head, tail) = line.split_at(split_at);
    let trimmed = tail.trim_start_matches([' ', '\t']);
    if trimmed.trim_end_matches(['\r', '\n']).is_empty() {
        format!("{head} AC{label}:{trimmed}")
    } else {
        format!("{head} AC{label}: {trimmed}")
    }
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

    fn assigned(result: &LabelCriteriaResult) -> &[LabelAssignment] {
        result.assigned.as_deref().unwrap()
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
            assigned(&result)
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
        assert!(!result.changed, "idempotent: {:?}", assigned(&result));
        assert_eq!(assigned(&result), [] as [LabelAssignment; 0]);
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

        assert_eq!(assigned(&result).len(), 1);
        assert_eq!(assigned(&result)[0].label, "AC25");
        assert_eq!(assigned(&result)[0].criterion_index, 1);
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

        assert_eq!(assigned(&result)[0].label, "AC4");
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

        assert_eq!(assigned(&result)[0].label, "AC12");
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
        assert_eq!(assigned(&result), [] as [LabelAssignment; 0]);
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

        assert_eq!(assigned(&result)[0].label, "AC1");
        assert!(
            spec_text(tmp.path()).contains("- [ ] AC1: Supersedes AC5: the rule in 017"),
            "the label anchors after the checkbox, not at the first AC-shaped token"
        );
    }

    #[test]
    fn an_empty_criterion_gains_no_trailing_space() {
        // `- [ ]` with no text is a shape the shared checkbox parser accepts.
        // Labelling it as `- [ ] AC1: ` would leave a trailing space, which
        // MD009 flags — and `/ductus:specify` lints the file one step later.
        let tmp = tempdir().unwrap();
        write_spec(
            tmp.path(),
            "---\nstatus: draft\ndependencies: []\n---\n\n# feat\n\n## Acceptance Criteria\n\n- [ ]\n",
        );
        run(&args(), tmp.path()).unwrap();

        let text = spec_text(tmp.path());
        assert!(text.contains("- [ ] AC1:\n"), "{text:?}");
        assert!(!text.contains("AC1: \n"), "trailing space: {text:?}");
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

    /// Appends a line to the file as a second writer landing between the
    /// primitive's read and its write would.
    fn theirs_lands(path: &std::path::Path) -> impl FnOnce() + 'static {
        let path = path.to_path_buf();
        move || {
            let mut text = std::fs::read_to_string(&path).unwrap();
            text.push_str("\nTheirs.\n");
            std::fs::write(&path, text).unwrap();
        }
    }

    /// A change another writer lands between the read and the write is kept:
    /// the write is refused, never made over it (`BE-TXN-002`).
    #[test]
    fn a_spec_changed_after_the_read_is_refused_and_kept() {
        let tmp = tempdir().unwrap();
        write_spec(tmp.path(), UNLABELLED);
        let file = tmp.path().join("specs/feat/spec.md");
        let before = fs::read_to_string(&file).unwrap();
        let err = guarded::between_read_and_write(theirs_lands(&file), || run(&args(), tmp.path()))
            .unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::ChangedSinceRead { path } if path == Path::new("specs/feat/spec.md")),
            "{err:?}"
        );
        assert_eq!(
            fs::read_to_string(&file).unwrap(),
            format!("{before}\nTheirs.\n")
        );
    }

    /// The next label is `max(highest label in the body + 1, next-criterion)`:
    /// with no stored counter it is one past the body maximum, and with a
    /// stored counter below that it is the body maximum plus one, never the
    /// counter (fbb62dfe; 022 task 42, F11).
    #[test]
    fn the_next_label_is_one_past_the_body_maximum_when_the_counter_is_absent_or_behind() {
        for (counter, label) in [("", "AC25"), ("next-criterion: 3\n", "AC25")] {
            let tmp = tempdir().unwrap();
            write_spec(
                tmp.path(),
                &format!(
                    "---\nstatus: done\ndependencies: []\n{counter}---\n\n# feat\n\n## Acceptance Criteria\n\n- [x] AC24: Labelled by hand.\n- [ ] Unlabelled.\n"
                ),
            );
            let result = run(&args(), tmp.path()).unwrap();
            assert_eq!(assigned(&result)[0].label, label, "{counter:?}");
            assert_eq!(result.next_criterion, 26, "{counter:?}");
            assert!(spec_text(tmp.path()).contains("- [ ] AC25: Unlabelled."));
        }
    }

    /// Past the read size `assigned` moves whole to the file `lists-file`
    /// names, its count inline, and the spec is labelled all the same
    /// (`BE-PAYLOAD-001`; 022 task 42, F14).
    #[test]
    fn past_the_read_size_assigned_moves_to_the_file() {
        let tmp = tempdir().unwrap();
        let criteria: Vec<String> = (0..200)
            .map(|n| format!("- [ ] Criterion {n}.\n"))
            .collect();
        let criteria = criteria.concat();
        write_spec(
            tmp.path(),
            &format!(
                "---\nstatus: draft\ndependencies: []\n---\n\n# feat\n\n## Acceptance Criteria\n\n{criteria}"
            ),
        );
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[artifacts]\nread-size-bytes = 2000\n",
        )
        .unwrap();
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(result.form, ListsForm::File);
        assert!(result.assigned.is_none(), "assigned stayed inline");
        assert_eq!(result.assigned_count, 200);
        assert!(result.changed);
        let size = serde_json::to_vec(&result).unwrap().len();
        assert!(size <= 2000, "{size} bytes");
        let lists: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(tmp.path().join(result.lists_file.as_deref().unwrap())).unwrap(),
        )
        .unwrap();
        assert_eq!(lists["assigned"].as_array().unwrap().len(), 200);
        assert_eq!(lists["assigned"][0]["label"], "AC1");
        crate::primitives::bounded_lists::assert_counts_name_the_file(
            "label-criteria",
            &serde_json::to_value(&result).unwrap(),
            &lists,
        );
        assert!(spec_text(tmp.path()).contains("- [ ] AC200: Criterion 199."));
    }

    /// A malformed read size refuses the call before the spec is written, so
    /// a result that cannot be bounded never follows a write it cannot report
    /// (`BE-PAYLOAD-001`; 022 task 42, F14).
    #[test]
    fn a_malformed_read_size_refuses_the_call_with_the_spec_untouched() {
        let tmp = tempdir().unwrap();
        write_spec(tmp.path(), UNLABELLED);
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[artifacts]\nread-size-bytes = 0\n",
        )
        .unwrap();
        let err = run(&args(), tmp.path()).unwrap_err();
        assert!(
            matches!(err, PrimitiveError::InvalidReadSize { .. }),
            "{err:?}"
        );
        assert_eq!(spec_text(tmp.path()), UNLABELLED);
    }
}
