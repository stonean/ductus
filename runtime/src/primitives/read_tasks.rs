//! `read-tasks` — parse `tasks.md` into a structured task list.
//!
//! Handles both file shapes:
//!
//! - **Flat** — task headings are `## N. Title` at level 2. No phase
//!   containers. Tasks return with `phase: None`.
//! - **Phased** — task headings are `### N. Title` at level 3, nested
//!   under `## …` phase containers (e.g., 023's
//!   `## Phase A — Refactor` containing a `### 1. Task`). Each task
//!   returns with `phase` set to the heading text of its containing
//!   phase. Detection matches the
//!   [scenario][runtime-primitive-structural-bugs] edge case: any
//!   `### N.` heading anywhere in the file signals phased structure,
//!   even when mixed with `## N.` flat headings.
//!
//! [runtime-primitive-structural-bugs]: <https://github.com/stonean/govern/blob/main/specs/022-deterministic-runtime/scenarios/runtime-primitive-structural-bugs.md>

use std::path::Path;

use crate::primitives::{
    PrimitiveError, Result, SkipScanner, TasksStructure, checkbox, detect_tasks_structure,
    heading_is_numeric, parse_atx_heading, parse_done_when, read_text, rel_path,
    split_numbered_heading,
};
use crate::schema::paths;
use crate::schema::primitives::{ReadTasksArgs, ReadTasksResult, Subtask, Task};

/// Execute the `read-tasks` primitive against the given repo root.
///
/// # Errors
///
/// Returns [`PrimitiveError::FeatureNotFound`] when `specs/<feature>/` does
/// not exist, or [`PrimitiveError::Io`] when `tasks.md` cannot be read.
pub fn run(args: &ReadTasksArgs, repo: &Path) -> Result<ReadTasksResult> {
    super::validate_no_traversal(&args.feature)?;
    let root = paths::Paths::load(repo).specs_root;
    let feature_dir = repo.join(&root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root,
            feature: args.feature.clone(),
        });
    }
    let tasks_path = feature_dir.join("tasks.md");
    let content = read_text(&tasks_path)?;

    let task_level = match detect_tasks_structure(&content) {
        TasksStructure::Flat => 2,
        TasksStructure::Phased => 3,
    };

    let mut tasks: Vec<Task> = Vec::new();
    let mut current: Option<Task> = None;
    // Phase tracking: any non-numeric `## …` heading sets the current
    // phase context. Numeric `## N.` headings in a phased file are
    // ignored (they're flat-task remnants in a mixed file; the phased
    // task-level is 3, so they don't open a new task here).
    let mut current_phase: Option<String> = None;
    let mut skip = SkipScanner::default();

    for line in content.lines() {
        if skip.skip(line) {
            continue;
        }
        if let Some((level, heading)) = parse_atx_heading(line) {
            // Level 1 ends the previous task (and resets phase context for
            // the unusual case of multiple H1s in one file).
            if level == 1 {
                if let Some(task) = current.take() {
                    tasks.push(task);
                }
                current = None;
                current_phase = None;
                continue;
            }
            // Phased mode: level-2 non-numeric headings open new phases.
            // Level-2 numeric headings (`## N.`) are flat remnants and
            // ignored as task openers, though their content (subtasks /
            // done-when below) won't attach to anything either.
            if level == 2 && task_level == 3 {
                if let Some(task) = current.take() {
                    tasks.push(task);
                }
                current = None;
                if !heading_is_numeric(&heading) {
                    current_phase = Some(heading);
                }
                continue;
            }
            // Task heading at the structure's task level.
            if level == task_level {
                if let Some(task) = current.take() {
                    tasks.push(task);
                }
                if let Some((number, title)) = split_numbered_heading(&heading) {
                    current = Some(Task {
                        number: number.to_string(),
                        heading: title.to_string(),
                        subtasks: Vec::new(),
                        done_when: None,
                        done_when_checked: None,
                        phase: current_phase.clone(),
                    });
                }
                continue;
            }
            // Any other heading level is informational; skip.
            continue;
        }
        let Some(task) = current.as_mut() else {
            continue;
        };
        // A "Done when" clause in any authoring form (bold, checkbox, or
        // bulletless) is recorded as the task's done-when, never as a
        // subtask — the shared recognizer keeps this decision identical to
        // `mark-task`'s subtask-exclusion (the read/mark index contract).
        if let Some(done) = parse_done_when(line) {
            task.done_when = Some(done);
            // Record the clause's checked state only for the checkbox form.
            // That is the one shape whose box is visible to a reader while
            // sitting outside the subtask index space, so it is the only
            // one that can disagree with the tally
            // (scenario unchecked-done-when-clause-tally).
            task.done_when_checked =
                checkbox::parse_checkbox_line(line).map(|(checked, _)| checked);
            continue;
        }
        // Subtask recognition shares the mark-side checkbox grammar
        // (`checkbox::parse_checkbox_line`) so the subtask indexes
        // returned here stay in lockstep with `mark-task`'s addressing —
        // the read/mark index contract.
        if let Some((checked, text)) = checkbox::parse_checkbox_line(line) {
            task.subtasks.push(Subtask { text, checked });
        }
    }
    if let Some(task) = current {
        tasks.push(task);
    }

    Ok(ReadTasksResult {
        tasks,
        path: rel_path(&tasks_path, repo),
    })
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
    fn parses_fixture_tasks() {
        let repo = fixture_repo();
        let result = run(
            &ReadTasksArgs {
                feature: "001-basic".into(),
            },
            &repo,
        )
        .unwrap();
        assert_eq!(result.path, "specs/001-basic/tasks.md");
        assert_eq!(result.tasks.len(), 2);

        let first = &result.tasks[0];
        assert_eq!(first.number, "1");
        assert_eq!(first.heading, "First task heading");
        assert_eq!(first.subtasks.len(), 2);
        assert!(first.subtasks[0].checked);
        assert_eq!(first.subtasks[0].text, "Subtask one — completed.");
        assert!(!first.subtasks[1].checked);
        assert_eq!(first.done_when.as_deref(), Some("both subtasks check."));

        let second = &result.tasks[1];
        assert_eq!(second.number, "2");
        assert_eq!(second.subtasks.len(), 1);
        assert_eq!(second.done_when.as_deref(), Some("the subtask is checked."));
    }

    // `split_numbered_heading` moved to `primitives::mod` (scenario
    // numbered-heading-grammar-single-source); its unit tests moved with it.

    // --- done-when authoring forms -------------------------------------------

    #[test]
    fn checkbox_form_clause_reports_its_checked_state() {
        // Scenario unchecked-done-when-clause-tally: the host needs to tell
        // "all subtasks checked" from "task block fully checked". Only the
        // checkbox form can differ, so only it carries a state.
        let tmp = tempdir().unwrap();
        let feature_dir = tmp.path().join("specs/feat");
        fs::create_dir_all(&feature_dir).unwrap();
        let body = "# feat\n\n## 1. Unchecked clause\n\n- [x] done subtask\n- [ ] Done when: not yet ticked\n\n## 2. Checked clause\n\n- [x] done subtask\n- [x] Done when: ticked\n\n## 3. Bold form\n\n- [x] done subtask\n- **Done when**: no checkbox at all\n";
        fs::write(feature_dir.join("tasks.md"), body).unwrap();

        let result = run(
            &ReadTasksArgs {
                feature: "feat".into(),
            },
            tmp.path(),
        )
        .unwrap();

        // Every subtask is checked in all three tasks; only task 1 has an
        // unchecked box still visible in its block.
        assert!(
            result
                .tasks
                .iter()
                .all(|t| t.subtasks.iter().all(|s| s.checked))
        );
        assert_eq!(result.tasks[0].done_when_checked, Some(false));
        assert_eq!(result.tasks[1].done_when_checked, Some(true));
        assert_eq!(
            result.tasks[2].done_when_checked, None,
            "no checkbox to disagree with the tally"
        );
    }

    #[test]
    fn recognizes_all_done_when_authoring_forms() {
        // The bold, checkbox, and bulletless forms the writers and the
        // corpus produce must all populate `done_when` — and none of them
        // may leak into the subtask list.
        let tmp = tempdir().unwrap();
        let content = "# Foo\n\n\
                       ## 1. Bold bullet\n\n\
                       - [x] real subtask\n\n\
                       - **Done when**: the canonical form parses.\n\n\
                       ## 2. Checkbox form (colon)\n\n\
                       - [x] first subtask\n\
                       - [ ] second subtask\n\
                       - [x] Done when: the LLM checkbox form parses.\n\n\
                       ## 3. Checkbox form (colon-less)\n\n\
                       - [x] only subtask\n\
                       - [x] Done when `the bare condition` holds\n\n\
                       ## 4. Bulletless form\n\n\
                       - [ ] only subtask\n\n\
                       Done when: the bare form parses.\n";
        make_phased_fixture(tmp.path(), "001-forms", content);
        let result = run(
            &ReadTasksArgs {
                feature: "001-forms".into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.tasks.len(), 4);

        assert_eq!(
            result.tasks[0].done_when.as_deref(),
            Some("the canonical form parses.")
        );
        assert_eq!(
            result.tasks[0].subtasks.len(),
            1,
            "bold done-when must not count as a subtask"
        );

        assert_eq!(
            result.tasks[1].done_when.as_deref(),
            Some("the LLM checkbox form parses.")
        );
        assert_eq!(
            result.tasks[1].subtasks.len(),
            2,
            "checkbox-form done-when must not count as a subtask"
        );
        assert!(
            result.tasks[1]
                .subtasks
                .iter()
                .all(|s| !s.text.starts_with("Done when")),
            "the done-when line leaked into the subtask list"
        );

        // The colon-less checkbox form (spec 024's shape) parses too.
        assert_eq!(
            result.tasks[2].done_when.as_deref(),
            Some("`the bare condition` holds")
        );
        assert_eq!(result.tasks[2].subtasks.len(), 1);

        assert_eq!(
            result.tasks[3].done_when.as_deref(),
            Some("the bare form parses.")
        );
        assert_eq!(result.tasks[3].subtasks.len(), 1);
    }

    #[test]
    fn word_boundary_guard_rejects_longer_words() {
        // The recognizer keys on the exact label `Done when`; a subtask that
        // opens with a longer word (`Done whenever …`) must NOT be read as
        // `Done when` + `ever …`. It stays an ordinary subtask.
        let tmp = tempdir().unwrap();
        let content = "# Foo\n\n\
                       ## 1. Guard\n\n\
                       - [ ] Done whenever the cache warms, refresh it\n";
        make_phased_fixture(tmp.path(), "001-guard", content);
        let result = run(
            &ReadTasksArgs {
                feature: "001-guard".into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.tasks.len(), 1);
        assert!(
            result.tasks[0].done_when.is_none(),
            "`Done whenever` is not the `Done when` label"
        );
        assert_eq!(result.tasks[0].subtasks.len(), 1);
    }

    // --- phased-structure tests -----------------------------------------------

    use std::fs;
    use tempfile::tempdir;

    fn make_phased_fixture(repo: &Path, feature: &str, content: &str) {
        let dir = repo.join("specs").join(feature);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("tasks.md"), content).unwrap();
    }

    #[test]
    fn parses_phased_tasks_with_phase_metadata() {
        let tmp = tempdir().unwrap();
        let content = "# Foo\n\n\
                       ## Phase A — Bootstrap\n\n\
                       ### 1. Wire crate\n\n\
                       - [x] Create Cargo.toml\n- [ ] Create lib.rs\n\n\
                       - **Done when**: cargo build succeeds.\n\n\
                       ### 2. Add CI\n\n\
                       - [x] Workflow file\n\n\
                       - **Done when**: CI is green.\n\n\
                       ## Phase B — Implementation\n\n\
                       ### 3. Build the thing\n\n\
                       - [ ] Code it up\n";
        make_phased_fixture(tmp.path(), "001-phased", content);
        let result = run(
            &ReadTasksArgs {
                feature: "001-phased".into(),
            },
            tmp.path(),
        )
        .unwrap();
        // Critical: phased file with 3 tasks must not return tasks: [].
        assert_eq!(result.tasks.len(), 3, "phased file returned empty list");
        assert_eq!(result.tasks[0].number, "1");
        assert_eq!(result.tasks[0].heading, "Wire crate");
        assert_eq!(
            result.tasks[0].phase.as_deref(),
            Some("Phase A — Bootstrap")
        );
        assert_eq!(result.tasks[0].subtasks.len(), 2);
        assert!(result.tasks[0].subtasks[0].checked);
        assert!(!result.tasks[0].subtasks[1].checked);
        assert_eq!(
            result.tasks[0].done_when.as_deref(),
            Some("cargo build succeeds.")
        );
        assert_eq!(
            result.tasks[1].phase.as_deref(),
            Some("Phase A — Bootstrap")
        );
        assert_eq!(
            result.tasks[2].phase.as_deref(),
            Some("Phase B — Implementation")
        );
    }

    #[test]
    fn parses_flat_tasks_with_no_phase_metadata() {
        let tmp = tempdir().unwrap();
        let content = "# Foo\n\n\
                       ## 1. First\n\n\
                       - [x] sub one\n\n\
                       - **Done when**: done.\n\n\
                       ## 2. Second\n\n\
                       - [ ] sub two\n";
        make_phased_fixture(tmp.path(), "001-flat", content);
        let result = run(
            &ReadTasksArgs {
                feature: "001-flat".into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.tasks.len(), 2);
        assert!(result.tasks.iter().all(|t| t.phase.is_none()));
    }

    #[test]
    fn parses_mixed_structure_as_phased() {
        // The scenario's edge case: mixed `## N.` + `### N.` is phased.
        // The `## 1.` flat-task remnant is skipped (we only walk task_level=3),
        // so it does not appear in the result. The phased tasks return.
        let tmp = tempdir().unwrap();
        let content = "# Foo\n\n\
                       ## 1. Legacy flat\n\n\
                       - [x] orphaned subtask\n\n\
                       ## Phase A — New work\n\n\
                       ### 2. Real task\n\n\
                       - [x] sub\n\n\
                       - **Done when**: done.\n";
        make_phased_fixture(tmp.path(), "001-mixed", content);
        let result = run(
            &ReadTasksArgs {
                feature: "001-mixed".into(),
            },
            tmp.path(),
        )
        .unwrap();
        // Phased mode: task_level=3 only. ## 1. is not returned.
        assert_eq!(result.tasks.len(), 1);
        assert_eq!(result.tasks[0].number, "2");
        assert_eq!(result.tasks[0].phase.as_deref(), Some("Phase A — New work"));
    }

    #[test]
    fn alternate_phase_label_still_recognized_as_phase_container() {
        // The scenario's edge case: any `## …` heading above the first
        // `### N.` task qualifies as a phase container. Stage 1 instead of
        // Phase A should still attach the right phase metadata.
        let tmp = tempdir().unwrap();
        let content = "# Foo\n\n\
                       ## Stage 1 — Bootstrap\n\n\
                       ### 1. Wire up\n\n\
                       - [x] subtask\n\n\
                       - **Done when**: done.\n";
        make_phased_fixture(tmp.path(), "001-stage", content);
        let result = run(
            &ReadTasksArgs {
                feature: "001-stage".into(),
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.tasks.len(), 1);
        assert_eq!(
            result.tasks[0].phase.as_deref(),
            Some("Stage 1 — Bootstrap")
        );
    }
}
