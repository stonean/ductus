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
//! [runtime-primitive-structural-bugs]: <https://github.com/stonean/ductus/blob/main/specs/022-deterministic-runtime/scenarios/runtime-primitive-structural-bugs.md>

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::primitives::{
    PrimitiveError, Result, SkipScanner, TasksStructure, checkbox, detect_tasks_structure,
    heading_is_numeric, parse_atx_heading, parse_done_when, read_text, rel_path,
    split_numbered_heading,
};
use crate::schema::paths;
use crate::schema::primitives::{ListsForm, ReadTasksArgs, ReadTasksResult, Subtask, Task};

/// Execute the `read-tasks` primitive: [`compute`], returned inline while it
/// fits `[artifacts] read-size-bytes`, and past it with its task list in the
/// file it names and only its count inline (`bounded_lists::place`; spec 080,
/// AC7).
///
/// # Errors
///
/// Every error [`compute`] returns, and those `bounded_lists::place` returns
/// for a malformed read size or a list file that cannot be written.
pub fn run(args: &ReadTasksArgs, repo: &Path) -> Result<ReadTasksResult> {
    let list = compute(args, repo)?;
    let mut result = ReadTasksResult {
        tasks_count: list.tasks.len(),
        tasks: Some(list.tasks),
        path: list.path,
        form: ListsForm::Inline,
        lists_file: None,
    };
    let lists = serde_json::json!({ "tasks": &result.tasks });
    if let Some(file) =
        super::bounded_lists::place(repo, "read-tasks", &args.feature, &result, &lists)?
    {
        result.form = ListsForm::File;
        result.tasks = None;
        result.lists_file = Some(file);
    }
    Ok(result)
}

/// A feature's task list, whole: what `read-tasks` reads, before its result
/// is bounded.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TaskList {
    /// Tasks in declaration order.
    pub(crate) tasks: Vec<Task>,
    /// Repo-relative path to the tasks file.
    pub(crate) path: String,
}

/// Parse the feature's `tasks.md`, every task whole. A caller in the runtime
/// reads this, never the bounded result.
///
/// # Errors
///
/// Returns [`PrimitiveError::FeatureNotFound`] when `specs/<feature>/` does
/// not exist, or [`PrimitiveError::Io`] when `tasks.md` cannot be read.
pub(crate) fn compute(args: &ReadTasksArgs, repo: &Path) -> Result<TaskList> {
    let root = paths::Paths::load(repo).specs_root;
    super::validate_feature_argument(repo, &root, &args.feature)?;
    read_checked(args, repo, &root)
}

/// [`compute`] without its walk of the feature directory, for a caller that
/// has already checked the feature argument itself: `check-artifacts`, which
/// is exempt from the walk (spec 022, scenario
/// `a-path-argument-resolves-inside-its-base`) and reads its task list here.
///
/// # Errors
///
/// As [`compute`], less the walk's refusal.
pub(crate) fn read_checked(args: &ReadTasksArgs, repo: &Path, root: &str) -> Result<TaskList> {
    let feature_dir = repo.join(root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root: root.to_string(),
            feature: args.feature.clone(),
        });
    }
    let tasks_path = feature_dir.join("tasks.md");
    let content = read_text(&tasks_path)?;
    Ok(TaskList {
        tasks: parse_tasks(&content),
        path: rel_path(&tasks_path, repo),
    })
}

/// Each unchecked task-list checkbox line in `content`, outside fenced blocks and HTML
/// comments, trimmed, sorted, with how many times it occurs. By line, not through
/// [`parse_tasks`], so a box counts under any heading, as it does to a reader: this is
/// the one test of what `tasks.md` still owes (owner, 2026-11-06).
pub(crate) fn unchecked_lines(content: &str) -> BTreeMap<&str, usize> {
    let mut skip = SkipScanner::default();
    let mut lines = BTreeMap::new();
    for line in content.lines().filter(|line| !skip.skip(line)) {
        if checkbox::find_checkbox_line(line)
            .is_some_and(|(_bracket, marker)| line.as_bytes()[marker] == b' ')
        {
            *lines.entry(line.trim()).or_insert(0) += 1;
        }
    }
    lines
}

/// Whether `now` adds an unchecked checkbox to `head`: holds an unchecked checkbox line
/// more times than `head` does. A line moved, re-indented or under a renamed task adds
/// nothing; a line added, reworded or unticked does.
pub(crate) fn adds_unchecked_box(now: &str, head: &str) -> bool {
    let before = unchecked_lines(head);
    unchecked_lines(now)
        .into_iter()
        .any(|(line, count)| count > before.get(line).copied().unwrap_or(0))
}

/// Refuse a review's or an analysis's record while `tasks.md` under `feature_dir` holds an
/// unchecked box: nothing can say whether a task still open would change what the run read, so
/// a run records once implementation is complete (spec 079). No `tasks.md` holds no box. A box
/// under a task numbered in `deferred_here`, which the run's own deferral names, is the work
/// this record holds, so it does not count.
///
/// # Errors
///
/// [`PrimitiveError::InvalidArgument`] on `feature`, naming the open boxes, and the read's
/// error when `tasks.md` exists and cannot be read.
pub(crate) fn refuse_while_tasks_are_open(
    primitive: &str,
    feature_dir: &Path,
    deferred_here: &BTreeSet<String>,
) -> Result<()> {
    let tasks = feature_dir.join("tasks.md");
    if !tasks.is_file() {
        return Ok(());
    }
    match describe_unchecked(&without_tasks(&read_text(&tasks)?, deferred_here)) {
        Some(owed) => Err(PrimitiveError::InvalidArgument {
            primitive: primitive.into(),
            argument: "feature".into(),
            reason: format!(
                "tasks.md holds {owed}; a run records once every task is done, since one still \
                 open may change what it read"
            ),
        }),
        None => Ok(()),
    }
}

/// `content` without the sections of the tasks numbered in `numbers`, each closed by the next
/// heading at its level or above, read outside fences and comments as the boxes are.
fn without_tasks(content: &str, numbers: &BTreeSet<String>) -> String {
    let mut skip = SkipScanner::default();
    let mut inside: Option<u8> = None;
    let mut kept = String::with_capacity(content.len());
    for line in content.lines() {
        if !skip.skip(line)
            && let Some((level, heading)) = parse_atx_heading(line)
        {
            if inside.is_some_and(|open| level <= open) {
                inside = None;
            }
            if inside.is_none()
                && split_numbered_heading(&heading)
                    .is_some_and(|(number, _)| numbers.contains(number))
            {
                inside = Some(level);
            }
        }
        if inside.is_none() {
            kept.push_str(line);
            kept.push('\n');
        }
    }
    kept
}

/// The unchecked boxes `content` holds, for a message: the count, and the first few.
pub(crate) fn describe_unchecked(content: &str) -> Option<String> {
    let lines = unchecked_lines(content);
    let count: usize = lines.values().sum();
    if count == 0 {
        return None;
    }
    let named: Vec<&str> = lines.keys().take(3).copied().collect();
    let more = if lines.len() > named.len() {
        ", and more"
    } else {
        ""
    };
    Some(format!(
        "{count} unchecked box(es): {}{more}",
        named.join("; ")
    ))
}

/// The tasks in a `tasks.md`'s text, in file order: one grammar for the working
/// tree and for a blob at a commit, which `classify-spec-edit` compares.
pub(crate) fn parse_tasks(content: &str) -> Vec<Task> {
    let task_level = match detect_tasks_structure(content) {
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
    tasks
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
        let result = compute(
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

        let result = compute(
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
        let result = compute(
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
        let result = compute(
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
        let result = compute(
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
        let result = compute(
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
        let result = compute(
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
        let result = compute(
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
