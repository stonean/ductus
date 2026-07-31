//! `read-spec` — parse spec frontmatter and body sections.

use std::path::Path;

use crate::primitives::{
    PrimitiveError, Result, checkbox, list_scenario_files, parse_atx_heading, read_text, rel_path,
    section_line_indices, split_frontmatter,
};
use crate::schema::paths;
use crate::schema::primitives::{
    AcceptanceCriterion, Frontmatter, OpenQuestion, ReadSpecArgs, ReadSpecResult,
    ScenarioOpenQuestion, SpecSection,
};

/// Execute the `read-spec` primitive against the given repo root.
///
/// # Errors
///
/// Returns [`PrimitiveError::FeatureNotFound`] when `specs/<feature>/` does
/// not exist, [`PrimitiveError::Io`] on filesystem failures,
/// [`PrimitiveError::MissingFrontmatter`] when the spec lacks `---` fences,
/// or [`PrimitiveError::Yaml`] when the frontmatter is not valid YAML.
pub fn run(args: &ReadSpecArgs, repo: &Path) -> Result<ReadSpecResult> {
    super::validate_no_traversal(&args.feature)?;
    let root = paths::Paths::load(repo).specs_root;
    let feature_dir = repo.join(&root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root,
            feature: args.feature.clone(),
        });
    }
    let spec_path = feature_dir.join("spec.md");
    let content = read_text(&spec_path)?;
    let (fm_text, body) = split_frontmatter(&content, &spec_path)?;
    let frontmatter: Frontmatter =
        serde_norway::from_str(fm_text).map_err(|source| PrimitiveError::Yaml {
            path: spec_path.clone(),
            source,
        })?;

    let sections = parse_sections(body, args.include_body);
    let acceptance_criteria = parse_checkboxes(body, "Acceptance Criteria");
    let open_questions = parse_open_questions(body, "Open Questions");
    let scenario_open_questions = collect_scenario_open_questions(&feature_dir);

    Ok(ReadSpecResult {
        frontmatter,
        sections,
        acceptance_criteria,
        open_questions,
        scenario_open_questions,
        path: rel_path(&spec_path, repo),
    })
}

/// Collect every unresolved question from this feature's `scenarios/*.md`,
/// tagged with its source scenario slug, in shared scenario order.
///
/// Enumerates through [`list_scenario_files`] so the set and the order
/// match every other scenario-aware surface, and parses each file's
/// `## Open Questions` with the same [`parse_open_questions`] the spec body
/// uses — so one parser decides what counts as a question everywhere
/// (spec 046).
///
/// An absent `scenarios/` directory yields an empty list. So does a
/// scenario whose file cannot be read or has no questions section: nothing
/// can be proven about a file that will not parse, and an unknown is never
/// escalated into a `done`-blocking finding.
///
/// `pub(crate)`: shared with `check-review-gate`, whose completion check
/// MUST block on exactly the list this reader reports. A second, private
/// scenario-question reader could disagree with the count surfaced to the
/// user, so there is deliberately only one (spec 046).
pub(crate) fn collect_scenario_open_questions(feature_dir: &Path) -> Vec<ScenarioOpenQuestion> {
    let scenarios_dir = feature_dir.join("scenarios");
    let mut out = Vec::new();
    for name in list_scenario_files(&scenarios_dir) {
        let scenario_path = scenarios_dir.join(&name);
        let Ok(content) = read_text(&scenario_path) else {
            continue;
        };
        // A scenario normally carries `section:` frontmatter, but a
        // hand-written one may not. Reuse the shared splitter and fall back
        // to the whole file when there is no frontmatter block — the
        // questions section is found by heading either way.
        let body =
            split_frontmatter(&content, &scenario_path).map_or(content.as_str(), |(_, body)| body);
        let slug = Path::new(&name)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or(name.as_str())
            .to_string();
        out.extend(
            parse_open_questions(body, "Open Questions")
                .into_iter()
                .map(|q| ScenarioOpenQuestion {
                    scenario: slug.clone(),
                    text: q.text,
                }),
        );
    }
    out
}

/// The distinct scenario slugs carrying questions, in the order they
/// appear — the name list every surface that reports scenario questions
/// renders.
///
/// `dedup` (not a sort-and-dedup) is correct because
/// [`collect_scenario_open_questions`] walks scenarios in shared order and
/// appends each file's questions together, so all entries for one scenario
/// are contiguous. That precondition is the reason this lives beside the
/// collector rather than being re-derived per call site: the gate, the
/// analyze finding, and the dashboard must render the same names in the
/// same order, and a change to the grouping has exactly one place to
/// follow (spec 046).
pub(crate) fn scenario_names(questions: &[ScenarioOpenQuestion]) -> Vec<&str> {
    let mut names: Vec<&str> = questions.iter().map(|q| q.scenario.as_str()).collect();
    names.dedup();
    names
}

fn parse_sections(body: &str, include_body: bool) -> Vec<SpecSection> {
    let mut sections: Vec<SpecSection> = Vec::new();
    let mut pending_body: Vec<&str> = Vec::new();
    let mut current: Option<(String, u8)> = None;

    for line in body.lines() {
        if let Some((level, heading)) = parse_atx_heading(line)
            && level >= 2
        {
            if let Some((h, l)) = current.take() {
                sections.push(SpecSection {
                    heading: h,
                    level: l,
                    body: if include_body {
                        pending_body.join("\n").trim().to_string()
                    } else {
                        String::new()
                    },
                });
            }
            pending_body.clear();
            current = Some((heading, level));
            continue;
        }
        if current.is_some() {
            pending_body.push(line);
        }
    }
    if let Some((heading, level)) = current {
        sections.push(SpecSection {
            heading,
            level,
            body: if include_body {
                pending_body.join("\n").trim().to_string()
            } else {
                String::new()
            },
        });
    }
    sections
}

/// Walk the named section's checkboxes with comment/fence awareness
/// ([`section_line_indices`]): example checkboxes inside a template
/// guidance comment or a fenced code block are not criteria. The indexes
/// of the returned criteria form a contract with `mark-criterion`'s
/// addressing — both consume the same shared walker AND the same checkbox
/// grammar ([`checkbox::parse_checkbox_line`]), so index N here is the
/// checkbox index N flips.
///
/// A wrapped acceptance criterion spans multiple source lines: an indented
/// non-checkbox continuation line folds into the preceding criterion's
/// text (mirroring [`parse_open_questions`]) rather than being dropped
/// mid-sentence. The index derivation stays keyed to checkbox lines only —
/// a continuation line never pushes a new entry — so the read/mark index
/// contract is preserved.
fn parse_checkboxes(body: &str, section_heading: &str) -> Vec<AcceptanceCriterion> {
    let lines: Vec<&str> = body.lines().collect();
    let mut out: Vec<AcceptanceCriterion> = Vec::new();
    for idx in section_line_indices(&lines, section_heading) {
        let line = lines[idx];
        if let Some((checked, text)) = checkbox::parse_checkbox_line(line) {
            out.push(AcceptanceCriterion { checked, text });
            continue;
        }
        // Fold an indented, non-checkbox continuation line into the last
        // criterion. Indentation is the wrap signal (markdown continuation
        // lines are indented under their list item); a non-indented,
        // non-checkbox line is not a continuation and is ignored.
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if line.starts_with([' ', '\t'])
            && let Some(current) = out.last_mut()
        {
            current.text.push(' ');
            current.text.push_str(trimmed);
        }
    }
    out
}

/// Parse the `- ` bullet entries of the named questions section
/// (continuation lines fold in; the placeholder lines in
/// [`QUESTION_PLACEHOLDERS`] are skipped). `pub(crate)`: shared with
/// `append-question`, whose dedup must see exactly the entries this
/// reader reports.
///
/// Walks [`section_line_indices`] — the comment- and fence-aware section
/// helper [`parse_checkboxes`] already uses — so the example questions a
/// freshly-scaffolded spec carries inside its template guidance comment,
/// and any question shown inside a fenced block, are not read as real
/// entries. Blank lines are still yielded by that helper, so the
/// blank-line terminator below is unaffected (spec 046).
///
/// A comment that opens and closes on one line stays *inline* per
/// [`SkipScanner`](super::SkipScanner)'s documented exemption, so a
/// standalone one-line comment after a question folds into that
/// question's text as a lazy list continuation. It adds no entry, so the
/// count this feeds is correct either way.
pub(crate) fn parse_open_questions(body: &str, section_heading: &str) -> Vec<OpenQuestion> {
    let mut out = Vec::new();
    let mut current: Option<String> = None;
    let lines: Vec<&str> = body.lines().collect();
    for idx in section_line_indices(&lines, section_heading) {
        let trimmed = lines[idx].trim_start();
        if let Some(rest) = trimmed.strip_prefix("- ") {
            if let Some(prev) = current.take() {
                push_question(&mut out, &prev);
            }
            current = Some(rest.trim().to_string());
        } else if !trimmed.is_empty() && current.is_some() {
            let continuation = trimmed.to_string();
            if let Some(buf) = current.as_mut() {
                buf.push(' ');
                buf.push_str(&continuation);
            }
        } else if trimmed.is_empty()
            && let Some(prev) = current.take()
        {
            push_question(&mut out, &prev);
        }
    }
    if let Some(prev) = current {
        push_question(&mut out, &prev);
    }
    out
}

/// The "no entries here" placeholder lines a questions section may carry:
/// the spec template's, and the one `create-scenario` compiles into every
/// new scenario. Neither is authored as a `- ` bullet today, so the guard
/// is belt-and-braces — but the set means the behavior no longer depends
/// on that (spec 046).
const QUESTION_PLACEHOLDERS: [&str; 3] = [
    "*None — all resolved.*",
    "*None — captured during scenario authoring.*",
    "*None yet.*",
];

fn push_question(out: &mut Vec<OpenQuestion>, text: &str) {
    let trimmed = text.trim();
    if trimmed.is_empty() || QUESTION_PLACEHOLDERS.contains(&trimmed) {
        return;
    }
    out.push(OpenQuestion {
        text: trimmed.to_string(),
    });
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::path::PathBuf;

    fn fixture_repo() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/primitives/sample-repo")
    }

    #[test]
    fn parses_basic_spec() {
        let repo = fixture_repo();
        let result = run(
            &ReadSpecArgs {
                feature: "001-basic".into(),
                include_body: false,
            },
            &repo,
        )
        .unwrap();

        assert_eq!(result.frontmatter.status, "clarified");
        assert!(result.frontmatter.dependencies.is_empty());
        assert_eq!(result.path, "specs/001-basic/spec.md");

        let section_headings: Vec<&str> =
            result.sections.iter().map(|s| s.heading.as_str()).collect();
        assert_eq!(
            section_headings,
            vec![
                "Motivation",
                "Acceptance Criteria",
                "Open Questions",
                "Resolved Questions",
            ]
        );
        for section in &result.sections {
            assert!(
                section.body.is_empty(),
                "body skipped when include_body=false"
            );
        }

        assert_eq!(result.acceptance_criteria.len(), 3);
        assert!(!result.acceptance_criteria[0].checked);
        assert!(result.acceptance_criteria[1].checked);
        assert!(!result.acceptance_criteria[2].checked);

        assert_eq!(result.open_questions.len(), 1);
        assert!(
            result.open_questions[0]
                .text
                .starts_with("Should fixtures embed binary assets")
        );
    }

    #[test]
    fn include_body_populates_section_text() {
        let repo = fixture_repo();
        let result = run(
            &ReadSpecArgs {
                feature: "001-basic".into(),
                include_body: true,
            },
            &repo,
        )
        .unwrap();
        let motivation = result
            .sections
            .iter()
            .find(|s| s.heading == "Motivation")
            .unwrap();
        assert!(motivation.body.contains("deterministic input"));
    }

    #[test]
    fn dependent_spec_lists_dependencies() {
        let repo = fixture_repo();
        let result = run(
            &ReadSpecArgs {
                feature: "002-dependent".into(),
                include_body: false,
            },
            &repo,
        )
        .unwrap();
        assert_eq!(result.frontmatter.status, "planned");
        assert_eq!(result.frontmatter.dependencies, vec!["001-basic"]);
    }

    #[test]
    fn template_state_spec_reports_zero_criteria() {
        // The shipped spec template embeds example `- [ ]` checkboxes inside
        // the Acceptance Criteria guidance comment; a template-state spec
        // must report zero criteria (scenario spec-side-parser-hardening).
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let template =
            std::fs::read_to_string(repo_root.join("framework/templates/spec/spec.md")).unwrap();
        let tmp = tempfile::tempdir().unwrap();
        let feature_dir = tmp.path().join("specs/042-fresh");
        std::fs::create_dir_all(&feature_dir).unwrap();
        std::fs::write(feature_dir.join("spec.md"), template).unwrap();

        let result = run(
            &ReadSpecArgs {
                feature: "042-fresh".into(),
                include_body: false,
            },
            tmp.path(),
        )
        .unwrap();
        assert!(
            result.acceptance_criteria.is_empty(),
            "template guidance-comment checkboxes counted as criteria: {:?}",
            result.acceptance_criteria
        );
    }

    #[test]
    fn criteria_inside_comments_and_fences_are_skipped() {
        let tmp = tempfile::tempdir().unwrap();
        let feature_dir = tmp.path().join("specs/042-fresh");
        std::fs::create_dir_all(&feature_dir).unwrap();
        let spec = "---\nstatus: draft\ndependencies: []\n---\n\n# T\n\n\
                    ## Acceptance Criteria\n\n\
                    <!--\n- [ ] Example inside comment\n-->\n\
                    - [ ] Real criterion.\n\
                    ```text\n- [ ] Example inside fence\n```\n\
                    - [x] Second real criterion.\n";
        std::fs::write(feature_dir.join("spec.md"), spec).unwrap();

        let result = run(
            &ReadSpecArgs {
                feature: "042-fresh".into(),
                include_body: false,
            },
            tmp.path(),
        )
        .unwrap();
        let texts: Vec<&str> = result
            .acceptance_criteria
            .iter()
            .map(|c| c.text.as_str())
            .collect();
        assert_eq!(texts, vec!["Real criterion.", "Second real criterion."]);
        assert!(!result.acceptance_criteria[0].checked);
        assert!(result.acceptance_criteria[1].checked);
    }

    #[test]
    fn folds_wrapped_continuation_into_criterion_text() {
        // A multi-line acceptance criterion must reach verifyCriteria whole,
        // not truncated mid-sentence. The index contract is preserved: the
        // wrapped criterion is still one entry at its checkbox index.
        let tmp = tempfile::tempdir().unwrap();
        let feature_dir = tmp.path().join("specs/042-fresh");
        std::fs::create_dir_all(&feature_dir).unwrap();
        let spec = "---\nstatus: draft\ndependencies: []\n---\n\n# T\n\n\
                    ## Acceptance Criteria\n\n\
                    - [ ] A criterion that wraps across\n  two source lines.\n\
                    - [x] A single-line criterion.\n";
        std::fs::write(feature_dir.join("spec.md"), spec).unwrap();

        let result = run(
            &ReadSpecArgs {
                feature: "042-fresh".into(),
                include_body: false,
            },
            tmp.path(),
        )
        .unwrap();
        let texts: Vec<&str> = result
            .acceptance_criteria
            .iter()
            .map(|c| c.text.as_str())
            .collect();
        assert_eq!(
            texts,
            vec![
                "A criterion that wraps across two source lines.",
                "A single-line criterion.",
            ]
        );
        assert_eq!(
            result.acceptance_criteria.len(),
            2,
            "wrap must not add an entry"
        );
        assert!(!result.acceptance_criteria[0].checked);
        assert!(result.acceptance_criteria[1].checked);
    }

    #[test]
    fn missing_feature_errors() {
        let repo = fixture_repo();
        let err = run(
            &ReadSpecArgs {
                feature: "999-nonexistent".into(),
                include_body: false,
            },
            &repo,
        )
        .unwrap_err();
        assert!(matches!(err, PrimitiveError::FeatureNotFound { .. }));
    }

    #[test]
    fn commented_out_example_questions_are_not_counted() {
        // The shipped spec template carries its example questions inside an
        // HTML comment. A freshly-scaffolded spec must report zero open
        // questions — before spec 046 the non-comment-aware walker read all
        // three as real, so every new spec looked like it had questions.
        let body = "\
## Open Questions

<!-- Uncertainties, unresolved decisions, and areas needing investigation.

- Should rate limits be configurable per tenant or fixed globally?
- What is the retention policy for audit log entries?
- What happens when the sessions table grows unbounded?

     When a question is resolved, move it to a \"Resolved Questions\" section.
-->
";
        assert!(parse_open_questions(body, "Open Questions").is_empty());
    }

    #[test]
    fn fenced_questions_are_not_counted() {
        let body = "\
## Open Questions

```markdown
- A question shown as an example, not asked
```

- A real question
";
        let questions = parse_open_questions(body, "Open Questions");
        assert_eq!(questions.len(), 1);
        assert_eq!(questions[0].text, "A real question");
    }

    #[test]
    fn single_line_comment_after_a_question_does_not_change_the_count() {
        // A comment opening and closing on one line is *inline* by
        // SkipScanner's documented exemption — the line is not skipped, so
        // its surrounding markdown survives (that exemption is what keeps
        // `- [ ] criterion <!-- note -->` intact). A standalone one-line
        // comment after a question therefore folds into that question's
        // text as a lazy list continuation. The entry count — the value the
        // spec 046 completion gate reads — is unaffected, which is the
        // contract that matters here.
        let body = "\
## Open Questions

- A real question
<!-- an authoring note -->
";
        let questions = parse_open_questions(body, "Open Questions");
        assert_eq!(questions.len(), 1, "count must not gain a phantom entry");
        assert!(questions[0].text.starts_with("A real question"));
    }

    #[test]
    fn every_placeholder_is_skipped_even_as_a_bullet() {
        for placeholder in QUESTION_PLACEHOLDERS {
            let plain = format!("## Open Questions\n\n{placeholder}\n");
            assert!(
                parse_open_questions(&plain, "Open Questions").is_empty(),
                "bare placeholder not skipped: {placeholder}"
            );

            let bulleted = format!("## Open Questions\n\n- {placeholder}\n");
            assert!(
                parse_open_questions(&bulleted, "Open Questions").is_empty(),
                "bulleted placeholder not skipped: {placeholder}"
            );
        }
    }

    #[test]
    fn wrapped_question_still_folds_its_continuation() {
        // The blank-line terminator and continuation folding must survive
        // the switch to the comment-aware walker.
        let body = "\
## Open Questions

- A question that wraps
  onto a second line

- A second question
";
        let questions = parse_open_questions(body, "Open Questions");
        assert_eq!(questions.len(), 2);
        assert_eq!(
            questions[0].text,
            "A question that wraps onto a second line"
        );
        assert_eq!(questions[1].text, "A second question");
    }

    /// Build a throwaway repo with one feature, optionally seeding
    /// `scenarios/` files as `(filename, contents)` pairs. Kept local to
    /// these tests so the shared `sample-repo` fixture — which several
    /// other primitives enumerate — gains no new feature directory.
    fn seed_feature(spec_body: &str, scenarios: &[(&str, &str)]) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let feature = tmp.path().join("specs/046-probe");
        std::fs::create_dir_all(&feature).unwrap();
        std::fs::write(feature.join("spec.md"), spec_body).unwrap();
        if !scenarios.is_empty() {
            let dir = feature.join("scenarios");
            std::fs::create_dir_all(&dir).unwrap();
            for (name, body) in scenarios {
                std::fs::write(dir.join(name), body).unwrap();
            }
        }
        tmp
    }

    fn probe(tmp: &tempfile::TempDir) -> ReadSpecResult {
        run(
            &ReadSpecArgs {
                feature: "046-probe".into(),
                include_body: false,
            },
            tmp.path(),
        )
        .unwrap()
    }

    const SPEC_NO_QUESTIONS: &str = "\
---
status: in-progress
dependencies: []
---

# 046 — Probe

## Open Questions

*None — all resolved.*
";

    #[test]
    fn scenario_questions_are_reported_separately_from_the_body_count() {
        let tmp = seed_feature(
            SPEC_NO_QUESTIONS,
            &[(
                "wire-contract.md",
                "---\nsection: Behavior\n---\n\n# Wire contract\n\n## Open Questions\n\n- Bracket operator or empty operand?\n- Does the filter apply to joins?\n",
            )],
        );
        let result = probe(&tmp);

        // The spec body's own count is untouched — this is the whole point
        // of keeping the two signals separate.
        assert!(
            result.open_questions.is_empty(),
            "spec-body count must stay zero"
        );
        assert_eq!(result.scenario_open_questions.len(), 2);
        assert!(
            result
                .scenario_open_questions
                .iter()
                .all(|q| q.scenario == "wire-contract"),
            "each entry carries its source scenario slug"
        );
        assert_eq!(
            result.scenario_open_questions[0].text,
            "Bracket operator or empty operand?"
        );
    }

    #[test]
    fn scenario_names_collapses_runs_and_keeps_collection_order() {
        // The helper the gate, the analyze finding, and the dashboard all
        // render from. `dedup` (not sort-and-dedup) is only correct while
        // entries stay grouped by scenario, so pin both halves: adjacent
        // duplicates collapse, and a scenario reappearing after another
        // would NOT collapse — which is exactly why the collector must keep
        // each file's questions contiguous.
        let q = |scenario: &str| ScenarioOpenQuestion {
            scenario: scenario.into(),
            text: "?".into(),
        };
        assert_eq!(
            scenario_names(&[q("alpha"), q("alpha"), q("beta"), q("beta"), q("beta")]),
            vec!["alpha", "beta"]
        );
        assert!(scenario_names(&[]).is_empty());
        // Ungrouped input is not silently repaired — the precondition is
        // the collector's to hold, and this documents the consequence.
        assert_eq!(
            scenario_names(&[q("alpha"), q("beta"), q("alpha")]),
            vec!["alpha", "beta", "alpha"]
        );
    }

    #[test]
    fn feature_without_scenarios_reports_no_scenario_questions() {
        let tmp = seed_feature(SPEC_NO_QUESTIONS, &[]);
        assert!(probe(&tmp).scenario_open_questions.is_empty());
    }

    #[test]
    fn scenario_placeholder_and_resolved_sections_yield_no_questions() {
        // What `create-scenario` compiles into every new scenario must read
        // as zero, or every scenario would block its parent spec's `done`.
        let tmp = seed_feature(
            SPEC_NO_QUESTIONS,
            &[(
                "fresh.md",
                "---\nsection: Behavior\n---\n\n# Fresh\n\n## Open Questions\n\n*None — captured during scenario authoring.*\n\n## Resolved Questions\n\n*None yet.*\n",
            )],
        );
        assert!(probe(&tmp).scenario_open_questions.is_empty());
    }

    #[test]
    fn scenario_questions_follow_shared_case_insensitive_order() {
        let tmp = seed_feature(
            SPEC_NO_QUESTIONS,
            &[
                (
                    "Beta.md",
                    "---\nsection: B\n---\n\n## Open Questions\n\n- from beta\n",
                ),
                (
                    "alpha.md",
                    "---\nsection: A\n---\n\n## Open Questions\n\n- from alpha\n",
                ),
            ],
        );
        let result = probe(&tmp);
        let slugs: Vec<&str> = result
            .scenario_open_questions
            .iter()
            .map(|q| q.scenario.as_str())
            .collect();
        // Case-insensitive: `alpha` precedes `Beta`. A byte-order sort would
        // invert these.
        assert_eq!(slugs, vec!["alpha", "Beta"]);
    }

    #[test]
    fn scenario_without_a_questions_section_contributes_nothing() {
        let tmp = seed_feature(
            SPEC_NO_QUESTIONS,
            &[(
                "plain.md",
                "---\nsection: Behavior\n---\n\n# Plain\n\n## Behavior\n\nNo questions section at all.\n",
            )],
        );
        assert!(probe(&tmp).scenario_open_questions.is_empty());
    }

    #[test]
    fn resolved_questions_section_is_not_read_as_open() {
        let body = "\
## Open Questions

- Still open

## Resolved Questions

- Already answered
";
        let questions = parse_open_questions(body, "Open Questions");
        assert_eq!(questions.len(), 1);
        assert_eq!(questions[0].text, "Still open");
    }
}
