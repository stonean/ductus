//! `process-decisions` — classify the deferred and waived decisions an audit
//! record carries against this run's finding keys (specs 058 and 076).
//!
//! The host must know which findings already carry a decision **before** it
//! proposes dispositions, or it asks questions that were answered on an
//! earlier run. So classification happens here, ahead of the writer, the way
//! `process-waivers` classifies waivers ahead of `write-review`:
//!
//! - **matched** — the key fired this run. The finding counts under the stored
//!   outcome and nothing is asked.
//! - **lapsed** — the key fired, but the pipeline work its deferral names is
//!   complete: the deferred work landed and the finding still stands, so the
//!   finding needs a new disposition and the writer drops the entry (spec 076).
//!   A deferral naming only an external work item never lapses, because the
//!   pipeline never reads the tracker; one whose named `tasks.md`, or owning
//!   spec's status, cannot be read — or whose feature directory, or the
//!   `spec.md` or `tasks.md` read in it, resolves outside the project — is
//!   matched, with a notice, so a check that could not run looks like neither.
//! - **expired** — the key did not fire on an unrestricted run. The writer
//!   drops the entry. For a deferred decision this is the moment the deferred
//!   work landed.
//! - **retained** — the key did not fire, but the run did not evaluate every
//!   source, so absence proves nothing and the entry is kept.
//! - **malformed** — a required field or an outcome's companion is missing,
//!   or a field its outcome forbids is present; reported, applied to nothing,
//!   and never pruned.
//! - **duplicate** — a repeated key; the first entry applies.
//!
//! Read-only: frontmatter mutation belongs to `write-review` and
//! `write-analysis`.

use std::path::Path;

use crate::primitives::decisions::{self, Work, read_decisions};
use crate::primitives::{
    ANALYSIS_RECORD_FILE, PrimitiveError, REVIEW_RECORD_FILE, Result, flatten_line,
};
use crate::schema::paths;
use crate::schema::primitives::{
    DecisionOutcome, DecisionRecord, DecisionRef, ProcessDecisionsArgs, ProcessDecisionsResult,
};

/// Execute the `process-decisions` primitive.
///
/// # Errors
///
/// Returns [`PrimitiveError::InvalidPath`] when `feature` is empty, absolute,
/// or carries a parent-directory component,
/// or when the feature directory, or a link inside it, resolves outside the
/// project, [`PrimitiveError::FeatureNotFound`]
/// when the feature has no `spec.md`, [`PrimitiveError::UnclosedFrontmatter`]
/// when the record's frontmatter block never closes,
/// [`PrimitiveError::Yaml`] when its frontmatter or `decisions:` list does
/// not parse — never an empty result, which would re-ask every settled
/// question — or [`PrimitiveError::Io`] on read failure. An absent record, or
/// one that opens no frontmatter block, holds no decisions: the writers
/// overwrite such a file.
pub fn run(args: &ProcessDecisionsArgs, repo: &Path) -> Result<ProcessDecisionsResult> {
    let layout = paths::Paths::load(repo);
    super::validate_feature_argument(repo, &layout.specs_root, &args.feature)?;
    let feature_dir = repo.join(&layout.specs_root).join(&args.feature);
    if !feature_dir.join("spec.md").is_file() {
        return Err(PrimitiveError::FeatureNotFound {
            root: layout.specs_root,
            feature: args.feature.clone(),
        });
    }
    let file = match args.record {
        DecisionRecord::Review => REVIEW_RECORD_FILE,
        DecisionRecord::Analysis => ANALYSIS_RECORD_FILE,
    };
    let stored = read_decisions(&feature_dir, file)?;
    // The writers store every key through `flatten_line`, so a fired key is
    // compared in the same form: a finding whose message carried a stray line
    // break still matches the decision stored for it.
    let fired: Vec<String> = args.fired.iter().map(|key| flatten_line(key)).collect();

    let mut result = ProcessDecisionsResult::default();
    let mut seen: Vec<String> = Vec::new();
    for (index, entry) in stored.iter().enumerate() {
        let decision = match entry.to_ref() {
            Ok(decision) => decision,
            Err(field) => {
                result.notices.push(format!(
                    "malformed decision at {file} decisions[{index}]: missing or invalid '{field}'"
                ));
                continue;
            }
        };
        // Keys compare flattened, as the writers store them and `merge`
        // compares them, so a hand-edited key is one decision to every reader.
        let key = flatten_line(&decision.key);
        if seen.contains(&key) {
            result.notices.push(format!(
                "duplicate decision: {} — entry [{index}] ignored",
                decision.key
            ));
            continue;
        }
        seen.push(key.clone());

        if fired.contains(&key) {
            if decision.outcome == DecisionOutcome::Deferred {
                match decisions::work_state(
                    repo,
                    &layout.specs_root,
                    decision.target.as_deref(),
                    decision.task.as_deref(),
                ) {
                    Work::Complete => {
                        result.notices.push(format!(
                            "deferral lapsed: {} — {} is complete and the finding still fires",
                            decision.key,
                            decisions::work_reference(
                                decision.target.as_deref(),
                                decision.task.as_deref(),
                                None,
                                crate::primitives::flatten_line,
                            )
                        ));
                        result.lapsed.push(decision);
                        continue;
                    }
                    Work::Unreadable => result.notices.push(format!(
                        "deferral target unreadable: {} — `{}`",
                        decision.key,
                        decision.target.as_deref().unwrap_or_default()
                    )),
                    Work::Pending | Work::External => {}
                }
            }
            result.matched.push(decision);
        } else if args.restricted {
            result.notices.push(format!(
                "decision retained: {} — its source was not evaluated this run ({})",
                decision.key,
                describe(&decision)
            ));
            result.retained.push(decision);
        } else {
            result.notices.push(format!(
                "decision expired: {} — the finding no longer fires ({})",
                decision.key,
                describe(&decision)
            ));
            result.expired.push(decision);
        }
    }
    Ok(result)
}

/// The decision's outcome and its companion, for a notice.
fn describe(decision: &DecisionRef) -> String {
    match decision.outcome {
        DecisionOutcome::Deferred => format!(
            "deferred to {}",
            decisions::work_reference(
                decision.target.as_deref(),
                decision.task.as_deref(),
                decision.work_item.as_deref(),
                crate::primitives::flatten_line,
            )
        ),
        DecisionOutcome::Waived => {
            format!("waived: {}", decision.reason.as_deref().unwrap_or_default())
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    const DEFERRED: &str = "  - key: \"grounding — plan.md cites a missing path\"\n    \
                          outcome: deferred\n    target: specs/031-x/tasks.md\n    \
                          decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n";
    const WAIVED: &str = "  - key: \"applicable-rules — BE-AUTHN-001 does not fire\"\n    \
                             outcome: waived\n    reason: cited for a future endpoint\n    \
                             decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n";

    fn repo_with(record: &str, decisions: &str) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("spec.md"),
            "---\nstatus: in-progress\n---\n# 001\n",
        )
        .unwrap();
        let target = tmp.path().join("specs/031-x");
        std::fs::create_dir_all(&target).unwrap();
        std::fs::write(
            target.join("spec.md"),
            "---\nstatus: in-progress\n---\n# 031\n",
        )
        .unwrap();
        if !decisions.is_empty() {
            std::fs::write(
                dir.join(record),
                format!("---\nlast-run: 2026-09-25T00:00:00Z\ndecisions:\n{decisions}---\n"),
            )
            .unwrap();
        }
        tmp
    }

    fn args(record: DecisionRecord, fired: &[&str], restricted: bool) -> ProcessDecisionsArgs {
        ProcessDecisionsArgs {
            feature: "001-x".into(),
            record,
            fired: fired.iter().map(|k| (*k).to_string()).collect(),
            restricted,
        }
    }

    #[test]
    fn a_firing_key_matches_under_its_stored_outcome() {
        let tmp = repo_with("analysis.md", &format!("{DEFERRED}{WAIVED}"));
        let result = run(
            &args(
                DecisionRecord::Analysis,
                &[
                    "grounding — plan.md cites a missing path",
                    "applicable-rules — BE-AUTHN-001 does not fire",
                ],
                false,
            ),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.matched.len(), 2);
        assert_eq!(result.matched[0].outcome, DecisionOutcome::Deferred);
        assert_eq!(
            result.matched[0].target.as_deref(),
            Some("specs/031-x/tasks.md")
        );
        assert_eq!(result.matched[1].outcome, DecisionOutcome::Waived);
        assert!(result.expired.is_empty() && result.retained.is_empty());
        assert_eq!(result.notices, [] as [String; 0]);
    }

    #[test]
    fn a_silent_key_expires_on_an_unrestricted_run() {
        let tmp = repo_with("analysis.md", DEFERRED);
        let result = run(&args(DecisionRecord::Analysis, &[], false), tmp.path()).unwrap();
        assert_eq!(result.expired.len(), 1, "the deferred work landed");
        assert!(result.notices[0].starts_with("decision expired: grounding"));
    }

    #[test]
    fn a_silent_key_is_retained_on_a_restricted_run() {
        let tmp = repo_with("review.md", WAIVED);
        let result = run(&args(DecisionRecord::Review, &[], true), tmp.path()).unwrap();
        assert!(
            result.expired.is_empty(),
            "absence proves nothing when the source was skipped"
        );
        assert_eq!(result.retained.len(), 1);
        assert!(result.notices[0].starts_with("decision retained:"));
    }

    #[test]
    fn a_restricted_run_still_matches_a_firing_key() {
        let tmp = repo_with("review.md", WAIVED);
        let result = run(
            &args(
                DecisionRecord::Review,
                &["applicable-rules — BE-AUTHN-001 does not fire"],
                true,
            ),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.matched.len(), 1);
    }

    #[test]
    fn a_missing_companion_is_malformed_and_never_pruned() {
        let no_target = "  - key: k\n    outcome: deferred\n    \
                         decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n";
        let no_reason = "  - key: j\n    outcome: waived\n    \
                         decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n";
        let tmp = repo_with("analysis.md", &format!("{no_target}{no_reason}"));
        let result = run(&args(DecisionRecord::Analysis, &[], false), tmp.path()).unwrap();
        assert!(
            result.matched.is_empty() && result.expired.is_empty() && result.retained.is_empty()
        );
        assert_eq!(
            result.notices,
            vec![
                "malformed decision at analysis.md decisions[0]: missing or invalid 'target'",
                "malformed decision at analysis.md decisions[1]: missing or invalid 'reason'",
            ]
        );
    }

    #[test]
    fn the_first_of_a_duplicated_key_wins() {
        let tmp = repo_with("analysis.md", &format!("{DEFERRED}{DEFERRED}"));
        let result = run(
            &args(
                DecisionRecord::Analysis,
                &["grounding — plan.md cites a missing path"],
                false,
            ),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.matched.len(), 1);
        assert_eq!(result.notices.len(), 1);
        assert!(result.notices[0].starts_with("duplicate decision:"));
    }

    /// A key hand-edited with surrounding spaces is the same key: it matches
    /// the finding it names, and a second entry for that key is a duplicate,
    /// exactly as `merge` judges them when it rewrites the list.
    #[test]
    fn keys_compare_flattened_for_matching_and_duplicates() {
        let padded = DEFERRED.replace(
            "\"grounding — plan.md cites a missing path\"",
            "\" grounding — plan.md cites a missing path \"",
        );
        let tmp = repo_with("analysis.md", &format!("{padded}{DEFERRED}"));
        let result = run(
            &args(
                DecisionRecord::Analysis,
                &["grounding — plan.md cites a missing path"],
                false,
            ),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.matched.len(), 1);
        assert_eq!(
            result.matched[0].key,
            " grounding — plan.md cites a missing path "
        );
        assert_eq!(result.notices.len(), 1);
        assert!(result.notices[0].starts_with("duplicate decision:"));
    }

    #[test]
    fn a_record_with_no_decisions_yields_empty_lists() {
        let tmp = repo_with("analysis.md", "");
        let result = run(
            &args(DecisionRecord::Analysis, &["anything"], false),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result, ProcessDecisionsResult::default());
    }

    /// A record that opens no frontmatter block holds no decisions — as
    /// `write-analysis` and `write-review`, which overwrite such a record,
    /// read it — rather than refusing the run.
    #[test]
    fn a_record_without_frontmatter_holds_no_decisions() {
        for record in [DecisionRecord::Analysis, DecisionRecord::Review] {
            let tmp = repo_with("analysis.md", "");
            let dir = tmp.path().join("specs/001-x");
            std::fs::write(dir.join("analysis.md"), "# Analysis\n\nhand-edited\n").unwrap();
            std::fs::write(dir.join("review.md"), "# Review\n\nhand-edited\n").unwrap();
            let result = run(&args(record, &["anything"], false), tmp.path()).unwrap();
            assert_eq!(result, ProcessDecisionsResult::default());
        }
    }

    /// A frontmatter block that opens and never closes may hold decisions
    /// nobody can read, so it is refused and named as unclosed.
    #[test]
    fn a_record_whose_frontmatter_never_closes_is_refused() {
        let tmp = repo_with("analysis.md", "");
        std::fs::write(
            tmp.path().join("specs/001-x/analysis.md"),
            format!("---\nspec: 001-x\ndecisions:\n{DEFERRED}\n# Analysis\n"),
        )
        .unwrap();
        let error = run(&args(DecisionRecord::Analysis, &[], false), tmp.path()).unwrap_err();
        assert!(error.to_string().contains("never closes"), "{error}");
        assert!(matches!(error, PrimitiveError::UnclosedFrontmatter { .. }));
    }

    #[test]
    fn the_record_argument_selects_the_file() {
        let tmp = repo_with("review.md", DEFERRED);
        let from_analysis = run(&args(DecisionRecord::Analysis, &[], false), tmp.path()).unwrap();
        assert!(
            from_analysis.expired.is_empty(),
            "analysis.md holds no decisions here"
        );
        let from_review = run(&args(DecisionRecord::Review, &[], false), tmp.path()).unwrap();
        assert_eq!(from_review.expired.len(), 1);
    }

    #[test]
    fn a_missing_feature_is_an_error() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(matches!(
            run(&args(DecisionRecord::Review, &[], false), tmp.path()),
            Err(PrimitiveError::FeatureNotFound { .. })
        ));
    }

    /// A fired key is compared in the form the writers store it: a finding
    /// whose message carried a stray line break still matches its decision.
    #[test]
    fn a_fired_key_with_a_line_break_matches_its_flattened_decision() {
        let tmp = repo_with("analysis.md", DEFERRED);
        let result = run(
            &args(
                DecisionRecord::Analysis,
                &["grounding — plan.md cites\na missing path"],
                false,
            ),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.matched.len(), 1);
        assert_eq!(result.expired, [] as [DecisionRef; 0]);
    }

    /// A deferral of `key` naming `target`, and `task` when given.
    fn deferral(key: &str, target: &str, task: Option<&str>) -> String {
        let task = task
            .map(|n| format!("    task: \"{n}\"\n"))
            .unwrap_or_default();
        format!(
            "  - key: \"{key}\"\n    outcome: deferred\n    target: {target}\n{task}    \
             decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n"
        )
    }

    fn write(tmp: &tempfile::TempDir, path: &str, content: &str) {
        let path = tmp.path().join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }

    /// Run with every key in `fired` firing, unrestricted.
    fn classify(tmp: &tempfile::TempDir, fired: &[&str]) -> ProcessDecisionsResult {
        run(&args(DecisionRecord::Analysis, fired, false), tmp.path()).unwrap()
    }

    fn keys(decisions: &[DecisionRef]) -> Vec<&str> {
        decisions.iter().map(|d| d.key.as_str()).collect()
    }

    /// Spec 076: a deferral to a task lapses once the task is checked or gone,
    /// and holds while it has an unchecked box.
    #[test]
    fn a_deferral_to_a_task_lapses_when_the_task_is_complete() {
        let tasks = "specs/031-x/tasks.md";
        let tmp = repo_with(
            "analysis.md",
            &[
                deferral("open", tasks, Some("1")),
                deferral("checked", tasks, Some("2")),
                deferral("gone", tasks, Some("9")),
            ]
            .concat(),
        );
        write(
            &tmp,
            tasks,
            "# Tasks\n\n## 1. Open\n\n- [x] a\n- [ ] b\n\n- **Done when**: b.\n\n\
             ## 2. Checked\n\n- [x] a\n\n- **Done when**: a.\n",
        );
        let result = classify(&tmp, &["open", "checked", "gone"]);
        assert_eq!(keys(&result.matched), ["open"]);
        assert_eq!(keys(&result.lapsed), ["checked", "gone"]);
        assert!(
            result.notices.iter().any(|n| n
                == "deferral lapsed: checked — `specs/031-x/tasks.md` task 2 is complete \
                    and the finding still fires"),
            "{:?}",
            result.notices
        );
    }

    /// Spec 076: a deferral to a spec's work lapses once that spec is `done`
    /// or gone; one to a rule file lapses as soon as its finding fires again.
    #[test]
    fn a_deferral_to_a_spec_or_rule_lapses_when_its_work_is_complete() {
        let tmp = repo_with(
            "analysis.md",
            &[
                deferral("pending", "specs/031-x/scenarios/a.md", None),
                deferral("done", "specs/032-y/spec.md", None),
                deferral("retired", "specs/033-z/spec.md", None),
                deferral("rule", "specs/rules/security-backend.md", None),
                deferral(
                    "framework rule",
                    "framework/rules/security-backend.md",
                    None,
                ),
            ]
            .concat(),
        );
        write(
            &tmp,
            "specs/032-y/spec.md",
            "---\nstatus: done\n---\n# 032\n",
        );
        let result = classify(
            &tmp,
            &["pending", "done", "retired", "rule", "framework rule"],
        );
        assert_eq!(keys(&result.matched), ["pending"]);
        assert_eq!(
            keys(&result.lapsed),
            ["done", "retired", "rule", "framework rule"]
        );
    }

    /// Spec 076: a deferral naming only a work item never lapses, and one
    /// whose target cannot be read is matched with a notice.
    #[test]
    fn external_and_unreadable_deferrals_are_matched() {
        let external = "  - key: \"external\"\n    outcome: deferred\n    work-item: PROJ-1\n    \
                        decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n";
        let tmp = repo_with(
            "analysis.md",
            &[
                external.to_string(),
                deferral("unreadable", "specs/034-w/spec.md", None),
            ]
            .concat(),
        );
        write(
            &tmp,
            "specs/034-w/spec.md",
            "---\nstatus: [not, a, string\n---\n",
        );
        let result = classify(&tmp, &["external", "unreadable"]);
        assert_eq!(keys(&result.matched), ["external", "unreadable"]);
        assert_eq!(result.lapsed, [] as [DecisionRef; 0]);
        assert!(
            result
                .notices
                .iter()
                .any(|n| n == "deferral target unreadable: unreadable — `specs/034-w/spec.md`"),
            "{:?}",
            result.notices
        );
    }

    /// Spec 076: a stored deferral's work is kept as written, so the notices
    /// flatten it before naming it — the target a lapse names, and the work
    /// item an expiry names — and an escape in the record reaches neither.
    #[test]
    fn notices_name_the_work_flattened() {
        let external = "  - key: \"external\"\n    outcome: deferred\n    \
                        work-item: \"PROJ-2\\u001b[0m\"\n    \
                        decided-at: 2026-09-25T00:00:00Z\n    decided-by: dev@example.com\n";
        let tmp = repo_with(
            "analysis.md",
            &[
                deferral("rule", "\"framework/rules/a\\u001b.md\"", None),
                external.to_string(),
            ]
            .concat(),
        );
        let result = classify(&tmp, &["rule"]);
        assert_eq!(keys(&result.lapsed), ["rule"]);
        assert_eq!(keys(&result.expired), ["external"]);
        let lapsed = "deferral lapsed: rule — `framework/rules/a .md` is complete and the \
                      finding still fires";
        let expired = "decision expired: external — the finding no longer fires (deferred \
                       to work item `PROJ-2 [0m`)";
        for notice in [lapsed, expired] {
            assert!(
                result.notices.iter().any(|n| n == notice),
                "{:?}",
                result.notices
            );
        }
    }
}
