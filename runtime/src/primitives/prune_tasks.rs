//! `prune-tasks` — reduce a feature's `tasks.md`.
//!
//! Two modes, selected by `args.reset`:
//!
//! - **keep-pending** (default) — drop every *spent* task section (a section
//!   with ≥ 1 checkbox, all checked) and every phase container left with no
//!   surviving task section; preserve the preamble and every pending /
//!   no-checkbox section verbatim.
//! - **reset** (`--reset`) — rewrite the file to the template's initial state:
//!   the existing `# …` heading followed by [`CANONICAL_EMPTY_TASKS_BODY`].
//!   Gated on spec status: permitted only when the spec is `done`, unless
//!   `--force` is supplied.
//!
//! Parsing reuses the shared `tasks.md` machinery
//! (`detect_tasks_structure`, `parse_atx_heading`, `checkbox::find_checkbox_line`)
//! so `prune-tasks` recognizes exactly the task set `read-tasks` /
//! `mark-task` see. The result is a compact summary — it never carries the
//! file body; the reduced content is produced and written entirely inside the
//! runtime (`apply: true`) or withheld (`apply: false` preview).

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::primitives::{
    PrimitiveError, Result, SkipScanner, TasksStructure, checkbox, detect_tasks_structure,
    join_blocks, list_feature_dirs, parse_atx_heading, read_text, rel_path, split_frontmatter,
    split_numbered_heading, write_atomic,
};
use crate::schema::paths;
use crate::schema::primitives::{
    Classification, PruneAction, PruneGate, PruneMode, PruneSection, PruneTasksArgs,
    PruneTasksLine, PruneTasksResult, PruneTasksSummary, PruneWalk, PruneWalkEntry, SizeSummary,
    SkipReason, SkippedFeature,
};

/// The template `tasks.md` body with its `# …` H1 line removed — the reset
/// target that follows the preserved feature heading. A unit test
/// ([`tests::canonical_empty_body_matches_template`]) asserts this equals
/// `framework/templates/spec/tasks.md` minus its H1 so the two never drift.
const CANONICAL_EMPTY_TASKS_BODY: &str = "Tasks derived from the [plan](plan.md). Complete in order.\n\n<!-- Each task should be small enough to implement and verify independently.\n     Mark subtasks as they are completed. Every task MUST close with a\n     `- **Done when**: …` line stating its completion condition — the\n     tooling reads this exact form to confirm the task is fully specified.\n\n     A task body may also carry working notes — what the next session needs\n     to resume it: the mechanics, the order, what is left on disk — placed\n     as §tasks-phase says. Write them as prose on the pending task they\n     concern, so they go when the task is pruned. Put an ordering constraint\n     on the task that must wait, never above the first task or under a\n     heading of its own, which outlive every task; and never write a note as\n     a checkbox, which would count toward the task's completion. Anything\n     that must outlast the task belongs in its durable home first. Example:\n\n## 1. Create sessions table migration\n\n- [ ] Write SQL migration for `sessions` table\n- [ ] Run migration and verify schema\n\n- **Done when**: the migration applies cleanly and `sessions` matches the data model.\n\n## 2. Implement session store\n\nBlocked on task 1: the store tests run against the migrated schema.\n\n- [ ] Create `shared/auth/session.go` with Create, Get, Delete methods\n- [ ] Write store integration tests against real PostgreSQL\n\n- **Done when**: all store methods are covered by passing integration tests.\n\n## 3. Update README link to migration guide\n\n- [ ] Edit `README.md` to point at the new path\n\n- **Done when**: the README link resolves to the new path.\n\n-->\n";

/// Frontmatter shape used only to read `status` (see [`read_status`]).
#[derive(serde::Deserialize)]
struct StatusOnly {
    status: Option<String>,
}

/// Kind of a segmented block.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// Preamble, H1, or any non-task structural heading group. Always kept.
    Structure,
    /// A `## …` phase container (phased files only). Kept iff a task section
    /// within it survives.
    Phase,
    /// A numbered task section. Dropped when spent.
    Task,
}

/// One segmented block of the file.
struct Block {
    kind: Kind,
    lines: Vec<String>,
    number: String,
    heading: String,
    phase: Option<String>,
    checkbox_total: u32,
    checkbox_checked: u32,
    /// Index into `blocks` of the governing phase container (Task blocks in
    /// phased files); `None` in flat files.
    governing_phase: Option<usize>,
}

impl Block {
    fn new(kind: Kind, first_line: &str) -> Self {
        Self {
            kind,
            lines: vec![first_line.to_string()],
            number: String::new(),
            heading: String::new(),
            phase: None,
            checkbox_total: 0,
            checkbox_checked: 0,
            governing_phase: None,
        }
    }

    fn classification(&self) -> Classification {
        if self.checkbox_total == 0 {
            Classification::NoCheckbox
        } else if self.checkbox_checked == self.checkbox_total {
            Classification::Spent
        } else {
            Classification::Pending
        }
    }
}

/// Execute the `prune-tasks` primitive against the given repo root: one
/// feature's reduction, or every feature's under `all`.
///
/// # Errors
///
/// - [`PrimitiveError::MissingArgument`] when neither `feature` nor `all` is
///   given, and [`PrimitiveError::InvalidArgument`] when both are, or when
///   `all` comes with `force`.
/// - [`PrimitiveError::FeatureNotFound`] when the feature directory is absent.
/// - [`PrimitiveError::TasksFileMissing`] when the named feature has no
///   `tasks.md`; under `all` such a feature is skipped instead.
/// - [`PrimitiveError::MalformedTasks`] when a `--reset` file has no `# …`
///   heading.
/// - [`PrimitiveError::MissingSpecFile`] / [`PrimitiveError::StatusFieldMissing`]
///   when the spec status cannot be read: by a `--reset`, for its gate, and by
///   an `all` walk, for each spec it lists.
/// - [`PrimitiveError::Io`] / [`PrimitiveError::Yaml`] on filesystem or
///   frontmatter failure.
///
/// Each error writes nothing to the feature it fires on. An applying `all`
/// walk keeps the reductions it already wrote to the features before it;
/// each is atomic, so running the walk again resumes.
pub fn run(args: &PruneTasksArgs, repo: &Path) -> Result<PruneTasksResult> {
    let root = paths::Paths::load(repo).specs_root;
    let Some(feature) = one_feature_or_all("prune-tasks", args.feature.as_deref(), args.all)?
    else {
        if args.force {
            return Err(PrimitiveError::InvalidArgument {
                primitive: "prune-tasks".into(),
                argument: "force".into(),
                reason: "a forced reset across every spec would discard every in-flight todo \
                         under one confirmation; force a reset one spec at a time"
                    .into(),
            });
        }
        return Ok(PruneTasksResult {
            summary: None,
            walk: Some(walk(args, repo, &root)?),
        });
    };
    Ok(PruneTasksResult {
        summary: Some(summarize(feature, args, repo, &root)?),
        walk: None,
    })
}

/// The feature a call names, or `None` for an `all` walk. Exactly one of the
/// two is given, for both prune primitives.
///
/// # Errors
///
/// [`PrimitiveError::MissingArgument`] when neither is given, and
/// [`PrimitiveError::InvalidArgument`] when both are.
pub(crate) fn one_feature_or_all<'a>(
    primitive: &str,
    feature: Option<&'a str>,
    all: bool,
) -> Result<Option<&'a str>> {
    match (feature, all) {
        (Some(feature), false) => Ok(Some(feature)),
        (None, true) => Ok(None),
        (Some(_), true) => Err(PrimitiveError::InvalidArgument {
            primitive: primitive.into(),
            argument: "all".into(),
            reason: "name one feature or walk them all, not both".into(),
        }),
        (None, false) => Err(PrimitiveError::MissingArgument {
            primitive: primitive.into(),
            argument: "feature".into(),
            reason: "name the feature to prune, or pass all to walk every feature".into(),
        }),
    }
}

/// Every feature's reduction, in corpus order, one line each carrying the
/// spec's status. A feature with nothing to reduce is examined and left out;
/// one with no `tasks.md` is skipped with that reason. Under `apply` each
/// permitted reduction is written, and a `--reset` stays gated per spec.
fn walk(args: &PruneTasksArgs, repo: &Path, root: &str) -> Result<PruneWalk<PruneTasksLine>> {
    let names = list_feature_dirs(&repo.join(root));
    let mut features = Vec::new();
    let mut skipped = Vec::new();
    for feature in &names {
        match reduce(feature, args, repo, root) {
            Ok(reduction) if reduction.summary.nothing_to_prune => {}
            Ok(reduction) => {
                // Read before an apply writes this spec's reduction, so a
                // status that will not read stops the walk with its file
                // untouched. A reset read it for its gate; keep-pending did not.
                let status = match &reduction.summary.status {
                    Some(status) => status.clone(),
                    None => read_status(&repo.join(root).join(feature), root, feature)?,
                };
                features.push(PruneWalkEntry {
                    feature: feature.clone(),
                    summary: PruneTasksLine::new(reduction.write()?, status),
                });
            }
            Err(PrimitiveError::TasksFileMissing { .. }) => skipped.push(SkippedFeature {
                feature: feature.clone(),
                reason: SkipReason::NoTasksFile,
            }),
            Err(other) => return Err(other),
        }
    }
    Ok(PruneWalk {
        examined: u32::try_from(names.len()).unwrap_or(u32::MAX),
        features,
        skipped,
    })
}

/// One feature's reduction, written when the call applies it.
fn summarize(
    feature: &str,
    args: &PruneTasksArgs,
    repo: &Path,
    root: &str,
) -> Result<PruneTasksSummary> {
    reduce(feature, args, repo, root)?.write()
}

/// One feature's reduction, computed and not yet written.
struct Reduction {
    /// The summary, its `applied` saying whether [`Reduction::write`] writes.
    summary: PruneTasksSummary,
    tasks_path: PathBuf,
    new_content: String,
}

impl Reduction {
    /// Write the reduction when the call applies it, and return its summary.
    fn write(self) -> Result<PruneTasksSummary> {
        if self.summary.applied {
            write_atomic(&self.tasks_path, &self.new_content)?;
        }
        Ok(self.summary)
    }
}

/// Compute one feature's reduction without writing it.
fn reduce(feature: &str, args: &PruneTasksArgs, repo: &Path, root: &str) -> Result<Reduction> {
    super::validate_feature_argument(repo, root, feature)?;
    let feature_dir = repo.join(root).join(feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root: root.to_string(),
            feature: feature.to_string(),
        });
    }
    let tasks_path = feature_dir.join("tasks.md");
    if !tasks_path.is_file() {
        return Err(PrimitiveError::TasksFileMissing {
            root: root.to_string(),
            feature: feature.to_string(),
        });
    }
    let content = read_text(&tasks_path)?;

    // The `--reset` status gate reads the spec status before touching tasks.
    let (mode, gate, status) = if args.reset {
        let status = read_status(&feature_dir, root, feature)?;
        let gate = if status == "done" || args.force {
            PruneGate::Allowed
        } else {
            PruneGate::BlockedNeedsForce
        };
        (PruneMode::Reset, gate, Some(status))
    } else {
        (PruneMode::KeepPending, PruneGate::NotApplicable, None)
    };

    let blocks = segment(&content);

    // Per-mode reduction: compute the would-be output, the section records,
    // and the removed/kept counts.
    let (new_content, sections, removed, kept) = match mode {
        PruneMode::KeepPending => reduce_keep_pending(&content, &blocks),
        PruneMode::Reset => reduce_reset(&content, &blocks, &tasks_path)?,
    };

    let nothing_to_prune = new_content == content;
    // A write happens only on `apply`, only when there is a change, and (for
    // reset) only when the gate permits it. keep-pending is never gated.
    let gate_permits = !matches!(gate, PruneGate::BlockedNeedsForce);
    let applied = args.apply && !nothing_to_prune && gate_permits;

    Ok(Reduction {
        summary: PruneTasksSummary {
            mode,
            applied,
            gate,
            status,
            nothing_to_prune,
            removed_count: removed,
            kept_count: kept,
            size_before: size_of(&content),
            size_after: size_of(&new_content),
            sections,
            path: rel_path(&tasks_path, repo),
        },
        tasks_path,
        new_content,
    })
}

/// Read the spec's frontmatter `status` — for the `--reset` gate and the
/// `all` walk's per-spec line here, and for `prune-plan`'s preview and
/// reopen trigger.
pub(crate) fn read_status(feature_dir: &Path, root: &str, feature: &str) -> Result<String> {
    let spec_path = feature_dir.join("spec.md");
    if !spec_path.is_file() {
        return Err(PrimitiveError::MissingSpecFile {
            root: root.to_string(),
            feature: feature.to_string(),
        });
    }
    let spec = read_text(&spec_path)?;
    let (frontmatter, _body) = split_frontmatter(&spec, &spec_path)?;
    let parsed: StatusOnly =
        serde_norway::from_str(frontmatter).map_err(|source| PrimitiveError::Yaml {
            path: spec_path.clone(),
            source,
        })?;
    parsed.status.ok_or(PrimitiveError::StatusFieldMissing {
        root: root.to_string(),
        feature: feature.to_string(),
    })
}

/// Segment `content` into structure / phase / task blocks in document order.
fn segment(content: &str) -> Vec<Block> {
    let task_level: u8 = match detect_tasks_structure(content) {
        TasksStructure::Flat => 2,
        TasksStructure::Phased => 3,
    };
    let phased = task_level == 3;

    let mut blocks: Vec<Block> = Vec::new();
    let mut cur = Block::new(Kind::Structure, "");
    // The initial block starts empty (no first line); clear the placeholder.
    cur.lines.clear();
    let mut skip = SkipScanner::default();
    let mut current_phase_name: Option<String> = None;
    let mut current_phase_idx: Option<usize> = None;

    for line in content.lines() {
        if skip.skip(line) {
            cur.lines.push(line.to_string());
            continue;
        }
        if let Some((level, heading)) = parse_atx_heading(line)
            && level <= task_level
        {
            // Close the current block; record a pushed phase's index.
            let closed_kind = cur.kind;
            blocks.push(std::mem::replace(
                &mut cur,
                Block::new(Kind::Structure, line),
            ));
            if closed_kind == Kind::Phase {
                current_phase_idx = Some(blocks.len() - 1);
            }

            // One parse, not a predicate followed by a re-parse: the `Some`
            // arm *is* the task test at the task level. The split borrows from
            // `heading` and is `Copy`, so classification is allocation-free;
            // only the task branch — the one that keeps the strings — pays for
            // them. A numbered heading above the task level (a flat-task
            // remnant in a phased file) is classified without allocating two
            // `String`s it would immediately discard.
            let numbered = split_numbered_heading(&heading);
            let is_phase = phased && level == 2 && numbered.is_none();
            let task = if level == task_level {
                numbered.map(|(number, title)| (number.to_string(), title.to_string()))
            } else {
                None
            };
            if let Some((number, title)) = task {
                cur.kind = Kind::Task;
                cur.number = number;
                cur.heading = title;
                cur.phase.clone_from(&current_phase_name);
                cur.governing_phase = current_phase_idx;
            } else if is_phase {
                cur.kind = Kind::Phase;
                current_phase_name = Some(heading);
            } else {
                cur.kind = Kind::Structure;
                if level == 1 {
                    // A top-level heading resets phase context.
                    current_phase_name = None;
                    current_phase_idx = None;
                }
            }
            continue;
        }

        // Body line: attach to the current block, counting checkboxes when
        // this is a task section.
        if cur.kind == Kind::Task
            && let Some((_bracket, marker)) = checkbox::find_checkbox_line(line)
        {
            cur.checkbox_total += 1;
            if matches!(line.as_bytes()[marker], b'x' | b'X') {
                cur.checkbox_checked += 1;
            }
        }
        cur.lines.push(line.to_string());
    }
    blocks.push(cur);
    blocks
}

/// keep-pending reduction: drop spent task sections and emptied phase
/// containers. Returns `(new_content, sections, removed, kept)`.
fn reduce_keep_pending(content: &str, blocks: &[Block]) -> (String, Vec<PruneSection>, u32, u32) {
    // A phase is dropped only when this reduction empties it: it governs a
    // task section, and every one it governs is spent. A phase governing no
    // task section is structure the prune did not empty, so it is kept, and
    // a file with nothing spent comes back byte-for-byte.
    let mut phase_has_task: HashSet<usize> = HashSet::new();
    let mut phase_has_survivor: HashSet<usize> = HashSet::new();
    for block in blocks {
        if block.kind == Kind::Task
            && let Some(p) = block.governing_phase
        {
            phase_has_task.insert(p);
            if block.classification() != Classification::Spent {
                phase_has_survivor.insert(p);
            }
        }
    }

    let mut sections = Vec::new();
    let mut removed = 0u32;
    let mut kept = 0u32;
    let mut kept_lines: Vec<&Block> = Vec::new();
    let mut dropped_any = false;

    for (idx, block) in blocks.iter().enumerate() {
        match block.kind {
            Kind::Structure => kept_lines.push(block),
            Kind::Phase => {
                if phase_has_task.contains(&idx) && !phase_has_survivor.contains(&idx) {
                    dropped_any = true;
                } else {
                    kept_lines.push(block);
                }
            }
            Kind::Task => {
                let spent = block.classification() == Classification::Spent;
                sections.push(section_record(
                    block,
                    if spent {
                        PruneAction::Removed
                    } else {
                        PruneAction::Kept
                    },
                ));
                if spent {
                    removed += 1;
                    dropped_any = true;
                } else {
                    kept += 1;
                    kept_lines.push(block);
                }
            }
        }
    }

    // No spent section and no dropped phase: leave the file byte-for-byte
    // unchanged rather than reformat seams.
    let new_content = if dropped_any {
        let kept: Vec<&[String]> = kept_lines.iter().map(|b| b.lines.as_slice()).collect();
        join_blocks(&kept, super::line_ending_of(content))
    } else {
        content.to_string()
    };
    (new_content, sections, removed, kept)
}

/// reset reduction: existing H1 + [`CANONICAL_EMPTY_TASKS_BODY`]. Every task
/// section is reported as removed.
fn reduce_reset(
    content: &str,
    blocks: &[Block],
    tasks_path: &Path,
) -> Result<(String, Vec<PruneSection>, u32, u32)> {
    // Outside fences and comments, as `segment` reads structure: a `# ` line
    // in an example block is not the feature's identity.
    let mut skip = SkipScanner::default();
    let h1 = blocks
        .iter()
        .flat_map(|b| b.lines.iter())
        .find(|line| !skip.skip(line) && matches!(parse_atx_heading(line.as_str()), Some((1, _))))
        .ok_or_else(|| PrimitiveError::MalformedTasks {
            path: tasks_path.to_path_buf(),
            reason: "no top-level (`#`) heading to preserve the feature identity".to_string(),
        })?;

    // The H1 is lifted from the file being reset, so this is a rewrite of an
    // existing file and not the creation of a new one — the ending it had is
    // the ending it gets back.
    let new_content = super::with_line_ending(
        &format!("{h1}\n\n{CANONICAL_EMPTY_TASKS_BODY}"),
        super::line_ending_of(content),
    );

    let mut sections = Vec::new();
    let mut removed = 0u32;
    for block in blocks {
        if block.kind == Kind::Task {
            sections.push(section_record(block, PruneAction::Removed));
            removed += 1;
        }
    }
    Ok((new_content, sections, removed, 0))
}

/// Build a compact per-section record (identity + classification + counts).
fn section_record(block: &Block, action: PruneAction) -> PruneSection {
    PruneSection {
        number: block.number.clone(),
        heading: block.heading.clone(),
        phase: block.phase.clone(),
        classification: block.classification(),
        checkbox_total: block.checkbox_total,
        checkbox_checked: block.checkbox_checked,
        action,
    }
}

pub(crate) fn size_of(content: &str) -> SizeSummary {
    SizeSummary {
        lines: content.lines().count(),
        bytes: content.len(),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn write_repo(tasks: &str, status: Option<&str>) -> (tempfile::TempDir, PathBuf) {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/041-task-pruning");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("tasks.md"), tasks).unwrap();
        if let Some(s) = status {
            fs::write(
                dir.join("spec.md"),
                format!("---\nstatus: {s}\ndependencies: []\n---\n\n# Spec\n"),
            )
            .unwrap();
        }
        let repo = tmp.path().to_path_buf();
        (tmp, repo)
    }

    fn args(reset: bool, force: bool, apply: bool) -> PruneTasksArgs {
        PruneTasksArgs {
            feature: Some("041-task-pruning".into()),
            all: false,
            reset,
            force,
            apply,
        }
    }

    /// One feature's summary from a single-feature call.
    fn run(args: &PruneTasksArgs, repo: &Path) -> Result<PruneTasksSummary> {
        super::run(args, repo).map(|result| result.summary.expect("a single-feature summary"))
    }

    const FLAT: &str = "# 041 — Task Pruning Tasks\n\nTasks derived from the [plan](plan.md). Complete in order.\n\n## 1. Done task\n\n- [x] a\n- [x] b\n\n## 2. Pending task\n\n- [ ] c\n- [x] d\n\n## 3. Prose task\n\nNo checkboxes here.\n";

    #[test]
    fn keep_pending_drops_spent_preserves_pending_and_prose() {
        let (_tmp, repo) = write_repo(FLAT, None);
        let result = run(&args(false, false, true), &repo).unwrap();
        assert_eq!(result.mode, PruneMode::KeepPending);
        assert_eq!(result.gate, PruneGate::NotApplicable);
        assert!(result.applied);
        assert_eq!(result.removed_count, 1);
        assert_eq!(result.kept_count, 2);
        let written = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert!(!written.contains("## 1. Done task"));
        assert!(written.contains("## 2. Pending task"));
        assert!(written.contains("## 3. Prose task"));
        // Preamble preserved.
        assert!(written.starts_with("# 041 — Task Pruning Tasks\n\nTasks derived"));
        // Output ends with exactly one trailing newline and no double blanks.
        assert!(written.ends_with('\n') && !written.ends_with("\n\n"));
        assert!(!written.contains("\n\n\n"));
    }

    #[test]
    fn preview_does_not_write() {
        let (_tmp, repo) = write_repo(FLAT, None);
        let before = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        let result = run(&args(false, false, false), &repo).unwrap();
        assert!(!result.applied);
        assert_eq!(result.removed_count, 1);
        let after = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert_eq!(before, after, "preview must not modify the file");
    }

    #[test]
    fn keep_pending_no_op_when_nothing_spent() {
        let tasks = "# T\n\nTasks derived from the [plan](plan.md). Complete in order.\n\n## 1. Pending\n\n- [ ] a\n";
        let (_tmp, repo) = write_repo(tasks, None);
        let result = run(&args(false, false, true), &repo).unwrap();
        assert!(result.nothing_to_prune);
        assert!(!result.applied);
        assert_eq!(result.removed_count, 0);
        let after = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert_eq!(after, tasks, "no-op must leave the file byte-for-byte");
    }

    const PHASED: &str = "# T\n\nTasks derived from the [plan](plan.md). Complete in order.\n\n## Phase A — Done\n\n### 1. Done one\n\n- [x] a\n\n### 2. Done two\n\n- [x] b\n\n## Phase B — Live\n\n### 3. Pending\n\n- [ ] c\n";

    #[test]
    fn keep_pending_phased_drops_spent_and_empty_phase() {
        let (_tmp, repo) = write_repo(PHASED, None);
        let result = run(&args(false, false, true), &repo).unwrap();
        assert_eq!(result.removed_count, 2);
        assert_eq!(result.kept_count, 1);
        let written = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        // Phase A had only spent tasks → dropped entirely.
        assert!(!written.contains("Phase A — Done"));
        assert!(!written.contains("### 1. Done one"));
        // Phase B and its pending task survive.
        assert!(written.contains("## Phase B — Live"));
        assert!(written.contains("### 3. Pending"));
        assert!(!written.contains("\n\n\n"));
    }

    #[test]
    fn keep_pending_keeps_a_phase_it_did_not_empty() {
        // A phase governing no task — a heading a person added for a note —
        // is not emptied by the prune, so nothing is written.
        let tasks = "# T\n\nTasks derived from the [plan](plan.md). Complete in order.\n\n## Phase A — Live\n\n### 1. Pending\n\n- [ ] a\n\n## Notes for the next session\n\nRun the suites after task 1.\n";
        let (_tmp, repo) = write_repo(tasks, None);
        let result = run(&args(false, false, true), &repo).unwrap();
        assert!(result.nothing_to_prune);
        assert!(!result.applied);
        let after = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert_eq!(after, tasks, "no write when nothing is spent");
    }

    #[test]
    fn reset_produces_template_state_when_done() {
        let (_tmp, repo) = write_repo(FLAT, Some("done"));
        let result = run(&args(true, false, true), &repo).unwrap();
        assert_eq!(result.mode, PruneMode::Reset);
        assert_eq!(result.gate, PruneGate::Allowed);
        assert!(result.applied);
        let written = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        let expected = format!("# 041 — Task Pruning Tasks\n\n{CANONICAL_EMPTY_TASKS_BODY}");
        assert_eq!(written, expected);
    }

    #[test]
    fn reset_blocked_on_non_done_without_force() {
        let (_tmp, repo) = write_repo(FLAT, Some("in-progress"));
        let before = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        let result = run(&args(true, false, true), &repo).unwrap();
        assert_eq!(result.gate, PruneGate::BlockedNeedsForce);
        assert!(!result.applied, "blocked reset must not write");
        let after = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert_eq!(before, after);
    }

    #[test]
    fn reset_forced_on_non_done_writes() {
        let (_tmp, repo) = write_repo(FLAT, Some("in-progress"));
        let result = run(&args(true, true, true), &repo).unwrap();
        assert_eq!(result.gate, PruneGate::Allowed);
        assert!(result.applied);
        assert_eq!(result.status.as_deref(), Some("in-progress"));
    }

    #[test]
    fn missing_tasks_file_errors() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/041-task-pruning");
        fs::create_dir_all(&dir).unwrap();
        let err = run(&args(false, false, false), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::TasksFileMissing { .. }));
    }

    #[test]
    fn reset_on_file_without_h1_errors() {
        let tasks = "Tasks derived from the [plan](plan.md).\n\n## 1. X\n\n- [x] a\n";
        let (_tmp, repo) = write_repo(tasks, Some("done"));
        let err = run(&args(true, false, true), &repo).unwrap_err();
        assert!(matches!(err, PrimitiveError::MalformedTasks { .. }));
    }

    /// A `# ` line inside a fence is an example, not the feature's identity,
    /// so a file whose only H1 sits in one is still malformed for a reset.
    #[test]
    fn reset_ignores_an_h1_inside_a_fence() {
        let tasks = "Tasks.\n\n```markdown\n# Example heading\n```\n\n## 1. X\n\n- [x] a\n";
        let (_tmp, repo) = write_repo(tasks, Some("done"));
        let err = run(&args(true, false, true), &repo).unwrap_err();
        assert!(
            matches!(err, PrimitiveError::MalformedTasks { .. }),
            "{err}"
        );
    }

    /// A corpus of features, each `(name, status, tasks)`; `None` tasks means
    /// the feature has no `tasks.md`.
    fn corpus(features: &[(&str, &str, Option<&str>)]) -> tempfile::TempDir {
        let tmp = tempdir().unwrap();
        for (name, status, tasks) in features {
            let dir = tmp.path().join("specs").join(name);
            fs::create_dir_all(&dir).unwrap();
            fs::write(
                dir.join("spec.md"),
                format!("---\nstatus: {status}\ndependencies: []\n---\n\n# Spec\n"),
            )
            .unwrap();
            if let Some(tasks) = tasks {
                fs::write(dir.join("tasks.md"), tasks).unwrap();
            }
        }
        tmp
    }

    fn all(reset: bool, force: bool, apply: bool) -> PruneTasksArgs {
        PruneTasksArgs {
            feature: None,
            all: true,
            reset,
            force,
            apply,
        }
    }

    const PENDING_ONLY: &str = "# T\n\n## 1. Pending\n\n- [ ] a\n";

    #[test]
    fn a_walk_reports_features_in_corpus_order_and_names_each_skip() {
        let tmp = corpus(&[
            ("1234.10-late", "in-progress", Some(FLAT)),
            ("1234.2-early", "done", Some(FLAT)),
            ("010-sequential", "planned", Some(FLAT)),
            ("002-first", "in-progress", Some(FLAT)),
            ("003-no-tasks", "clarified", None),
            ("004-lean", "in-progress", Some(PENDING_ONLY)),
        ]);
        let walk = super::run(&all(false, false, false), tmp.path())
            .unwrap()
            .walk
            .expect("an all walk");
        let order: Vec<&str> = walk.features.iter().map(|f| f.feature.as_str()).collect();
        // Sequential by number, then branch-scoped by counter, numerically.
        assert_eq!(
            order,
            [
                "002-first",
                "010-sequential",
                "1234.2-early",
                "1234.10-late"
            ]
        );
        assert_eq!(
            walk.skipped,
            [SkippedFeature {
                feature: "003-no-tasks".into(),
                reason: SkipReason::NoTasksFile,
            }]
        );
        // The lean feature is examined but, with nothing to reduce, not listed.
        assert_eq!(walk.examined, 6);
        assert!(walk.features.iter().all(|f| !f.summary.applied));
        // Keep-pending reads no status for its own work; the walk reads each
        // listed spec's own.
        let statuses: Vec<(&str, &str)> = walk
            .features
            .iter()
            .map(|f| (f.feature.as_str(), f.summary.status.as_str()))
            .collect();
        assert_eq!(
            statuses,
            [
                ("002-first", "in-progress"),
                ("010-sequential", "planned"),
                ("1234.2-early", "done"),
                ("1234.10-late", "in-progress")
            ]
        );
    }

    #[test]
    fn a_walk_is_either_one_feature_or_all_and_never_forced() {
        let tmp = corpus(&[("002-first", "in-progress", Some(FLAT))]);
        let mut neither = all(false, false, false);
        neither.all = false;
        assert!(matches!(
            super::run(&neither, tmp.path()).unwrap_err(),
            PrimitiveError::MissingArgument { .. }
        ));
        let mut both = all(false, false, false);
        both.feature = Some("002-first".into());
        assert!(matches!(
            super::run(&both, tmp.path()).unwrap_err(),
            PrimitiveError::InvalidArgument { .. }
        ));
        let before = fs::read_to_string(tmp.path().join("specs/002-first/tasks.md")).unwrap();
        let err = super::run(&all(true, true, true), tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidArgument { argument, .. } if argument == "force"),
            "{err}"
        );
        let after = fs::read_to_string(tmp.path().join("specs/002-first/tasks.md")).unwrap();
        assert_eq!(before, after, "a refused forced walk writes nothing");
    }

    #[test]
    fn an_applying_walk_reads_the_status_before_it_writes() {
        // A keep-pending reduction reads no status for its own work, but the
        // walk's line carries one, so a spec whose status will not read must
        // stop the walk before its reduction is written.
        let tmp = corpus(&[("002-first", "in-progress", Some(FLAT))]);
        fs::remove_file(tmp.path().join("specs/002-first/spec.md")).unwrap();
        let err = super::run(&all(false, false, true), tmp.path()).unwrap_err();
        assert!(
            matches!(err, PrimitiveError::MissingSpecFile { .. }),
            "{err}"
        );
        let after = fs::read_to_string(tmp.path().join("specs/002-first/tasks.md")).unwrap();
        assert_eq!(after, FLAT, "the walk stopped with the file untouched");
    }

    #[test]
    fn a_reset_walk_keeps_its_gate_per_spec() {
        let tmp = corpus(&[
            ("002-done", "done", Some(FLAT)),
            ("003-live", "in-progress", Some(FLAT)),
        ]);
        let walk = super::run(&all(true, false, true), tmp.path())
            .unwrap()
            .walk
            .expect("an all walk");
        let gates: Vec<(&str, PruneGate, bool, &str)> = walk
            .features
            .iter()
            .map(|f| {
                (
                    f.feature.as_str(),
                    f.summary.gate,
                    f.summary.applied,
                    f.summary.status.as_str(),
                )
            })
            .collect();
        assert_eq!(
            gates,
            [
                ("002-done", PruneGate::Allowed, true, "done"),
                (
                    "003-live",
                    PruneGate::BlockedNeedsForce,
                    false,
                    "in-progress"
                ),
            ]
        );
        let live = fs::read_to_string(tmp.path().join("specs/003-live/tasks.md")).unwrap();
        assert_eq!(live, FLAT, "the in-flight spec keeps its todos");
        let done = fs::read_to_string(tmp.path().join("specs/002-done/tasks.md")).unwrap();
        assert_eq!(
            done,
            format!("# 041 — Task Pruning Tasks\n\n{CANONICAL_EMPTY_TASKS_BODY}")
        );
    }

    #[test]
    fn canonical_empty_body_matches_template() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let template =
            fs::read_to_string(repo_root.join("framework/templates/spec/tasks.md")).unwrap();
        // Strip the H1 line and any leading blank lines that follow it.
        let after_h1 = template.split_once('\n').map(|(_, rest)| rest).unwrap();
        let body = after_h1.trim_start_matches('\n');
        assert_eq!(
            body.trim_end(),
            CANONICAL_EMPTY_TASKS_BODY.trim_end(),
            "CANONICAL_EMPTY_TASKS_BODY drifted from framework/templates/spec/tasks.md"
        );
    }

    #[test]
    fn reset_output_parses_as_zero_tasks() {
        use crate::primitives::{append_task, read_tasks};
        use crate::schema::primitives::{AppendTaskArgs, ReadTasksArgs};

        let (_tmp, repo) = write_repo(FLAT, Some("done"));
        run(&args(true, false, true), &repo).unwrap();

        // The reset file's guidance comment embeds `## 1/2/3` example
        // headings; the HTML-comment-aware parsers must see zero real tasks.
        let tasks = read_tasks::run(
            &ReadTasksArgs {
                feature: "041-task-pruning".into(),
            },
            &repo,
        )
        .unwrap();
        assert!(
            tasks.tasks.is_empty(),
            "reset file must parse to zero tasks, got {}",
            tasks.tasks.len()
        );

        // append-task must number from 1, not from the commented examples.
        let appended = append_task::run(
            &AppendTaskArgs {
                feature_path: "specs/041-task-pruning".into(),
                title: "First real task".into(),
                done_when: "it lands".into(),
                body: None,
                slug: Some("x".into()),
                parent_heading: None,
                dedup_title: false,
            },
            &repo,
        )
        .unwrap();
        assert_eq!(
            appended.task_number, 1,
            "append-task on a reset file must number from 1"
        );
    }
}
