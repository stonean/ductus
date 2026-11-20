//! `prune-tasks` — reduce a feature's `tasks.md`.
//!
//! Two modes, selected by `args.reset`:
//!
//! - **keep-pending** (default) — drop every *spent* task section (a section
//!   with ≥ 1 checkbox, all checked) and every phase container left with no
//!   surviving task section; preserve the preamble and every pending /
//!   no-checkbox section verbatim.
//! - **reset** (`--reset`) — rewrite the file to the template's initial state:
//!   the file's frontmatter block, when it has one, the existing `# …` heading,
//!   and [`CANONICAL_EMPTY_TASKS_BODY`]. Gated on spec status: permitted only
//!   when the spec is `done`, unless `--force` is supplied.
//!
//! Parsing reuses the shared `tasks.md` machinery
//! (`detect_tasks_structure`, `parse_atx_heading`, `checkbox::find_checkbox_line`)
//! so `prune-tasks` recognizes exactly the task set `read-tasks` /
//! `mark-task` see. The result is a compact summary — it never carries the
//! file body; the reduced content is produced and written entirely inside the
//! runtime (`apply: true`) or withheld (`apply: false` preview).

use std::collections::HashSet;
use std::path::Path;

use serde::Serialize;

use crate::primitives::{
    PrimitiveError, ReadSnapshot, Result, SkipScanner, TasksStructure, checkbox,
    detect_tasks_structure, join_blocks, list_feature_dirs, parse_atx_heading, read_text, rel_path,
    split_frontmatter, split_numbered_heading,
};
use crate::schema::paths;
use crate::schema::primitives::{
    Classification, ListsForm, PruneAction, PruneGate, PruneMode, PruneSection, PruneTasksArgs,
    PruneTasksLine, PruneTasksResult, PruneTasksSummary, PruneWalk, PruneWalkEntry, SizeSummary,
    SkipReason, SkippedFeature, TailCause, UnexaminedTail,
};

/// The template `tasks.md` body with its `# …` H1 line removed — the reset
/// target that follows the preserved feature heading. A unit test
/// ([`tests::canonical_empty_body_matches_template`]) asserts this equals
/// `framework/templates/spec/tasks.md` minus its H1 so the two never drift.
const CANONICAL_EMPTY_TASKS_BODY: &str = "Tasks derived from the [plan](plan.md). Complete in order.\n\n<!-- Each task should be small enough to implement and verify independently.\n     Mark subtasks as they are completed. Every task MUST close with a\n     `- **Done when**: …` line stating its completion condition — the\n     tooling reads this exact form to confirm the task is fully specified.\n\n     A task body may also carry working notes — what the next session needs\n     to resume it: the mechanics, the order, what is left on disk — placed\n     as §tasks-phase says. Write them as prose on the pending task they\n     concern, so they go when the task is pruned. Put an ordering constraint\n     on the task that must wait, never above the first task or under a\n     heading of its own, which outlive every task; and never write a note as\n     a checkbox, which would count toward the task's completion. Anything\n     that must outlast the task belongs in its durable home first.\n\n     Schedule no review, analysis or completion gate as a task: they run\n     once every box here is ticked, and refuse to record before then, so\n     what a spec owes them is prose on its pending task. Example:\n\n## 1. Create sessions table migration\n\n- [ ] Write SQL migration for `sessions` table\n- [ ] Run migration and verify schema\n\n- **Done when**: the migration applies cleanly and `sessions` matches the data model.\n\n## 2. Implement session store\n\nBlocked on task 1: the store tests run against the migrated schema.\n\n- [ ] Create `shared/auth/session.go` with Create, Get, Delete methods\n- [ ] Write store integration tests against real PostgreSQL\n\n- **Done when**: all store methods are covered by passing integration tests.\n\n## 3. Update README link to migration guide\n\n- [ ] Edit `README.md` to point at the new path\n\n- **Done when**: the README link resolves to the new path.\n\n-->\n";

/// A [`SkipScanner`] that also remembers where the region it sits in opened,
/// so a caller can tell, once the whole file is fed, that it ends inside an
/// open fenced block or HTML comment. It asks only whether a region is open
/// at the end, never which line closes one, so it holds under any closing
/// rule `SkipScanner` applies.
#[derive(Default)]
pub(crate) struct OpenRegion {
    scanner: SkipScanner,
    /// 0-based index of the line that opened the region still open.
    opened_at: usize,
}

impl OpenRegion {
    /// Advance over line `index` (0-based, in order) and report whether it is
    /// to be skipped, as [`SkipScanner::skip`] does.
    pub(crate) fn observe(&mut self, index: usize, line: &str) -> bool {
        let was_open = self.scanner.in_region();
        let skipped = self.scanner.skip(line);
        if !was_open && self.scanner.in_region() {
            self.opened_at = index;
        }
        skipped
    }

    /// The tail left unexamined when the lines fed end inside an open region;
    /// `line_offset` is the number of lines before the first one fed, so
    /// `from-line` is a line of the whole file.
    pub(crate) fn tail(&self, line_offset: usize) -> Option<UnexaminedTail> {
        self.scanner.in_region().then(|| UnexaminedTail {
            from_line: self.opened_at + line_offset + 1,
            cause: if self.scanner.in_fence() {
                TailCause::OpenFence
            } else {
                TailCause::OpenComment
            },
        })
    }
}

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
/// - [`PrimitiveError::InvalidReadSize`] for a malformed `[artifacts]
///   read-size-bytes`, before any write.
/// - [`PrimitiveError::MissingArgument`] when neither `feature` nor `all` is
///   given, and [`PrimitiveError::InvalidArgument`] when both are, or when
///   `all` comes with `force`.
/// - [`PrimitiveError::FeatureNotFound`] when the feature directory is absent.
/// - [`PrimitiveError::TasksFileMissing`] when the named feature has no
///   `tasks.md`; under `all` such a feature is skipped instead.
/// - [`PrimitiveError::MalformedTasks`] when a `--reset` file has no `# …`
///   heading, or, in either mode, its frontmatter never closes or its
///   `next-task:` is not a positive integer, or no counter can follow its
///   highest task number.
/// - [`PrimitiveError::MissingSpecFile`] / [`PrimitiveError::StatusFieldMissing`]
///   when the spec status cannot be read: by a `--reset`, for its gate, and by
///   an `all` walk, for each spec it lists.
/// - [`PrimitiveError::Io`] / [`PrimitiveError::Yaml`] on filesystem or
///   frontmatter failure.
///
/// - [`PrimitiveError::ChangedSinceRead`] when a single feature's `tasks.md`
///   changed between the apply's read and its write (`BE-TXN-002`): nothing is
///   written, and the other writer's change stands. A walk reports such a
///   feature in `skipped` instead and goes on, so its result names what it
///   wrote and what it refused.
///
/// Each error writes nothing to the feature it fires on. An applying `all`
/// walk computes every feature's reduction and reads every status before it
/// writes any, so an error in either, a path check refusing one feature's
/// directory among them, leaves the whole corpus untouched. Only a write
/// that fails partway keeps the reductions written before it; each is
/// atomic, so running the walk again resumes.
pub fn run(args: &PruneTasksArgs, repo: &Path) -> Result<PruneTasksResult> {
    let root = paths::Paths::load(repo)?.specs_root;
    // The read size bounds the result only after an applied reduction is
    // written, so a malformed one halts here, before any write, rather than
    // fail a call whose writes stand.
    super::check_artifact_size::resolve_threshold(repo)?;
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
        let mut result = PruneTasksResult {
            summary: None,
            walk: Some(walk(args, repo, &root)?),
            form: ListsForm::Inline,
            lists_file: None,
        };
        bound_result(
            repo,
            "prune-tasks",
            super::bounded_lists::ALL_SPECS,
            &mut result,
        )?;
        return Ok(result);
    };
    let mut result = PruneTasksResult {
        summary: Some(summarize(feature, args, repo, &root)?),
        walk: None,
        form: ListsForm::Inline,
        lists_file: None,
    };
    bound_result(repo, "prune-tasks", feature, &mut result)?;
    Ok(result)
}

impl BoundedPruneResult for PruneTasksResult {
    fn lists(&self) -> serde_json::Value {
        match (&self.summary, &self.walk) {
            (Some(summary), _) => serde_json::json!({ "sections": &summary.sections }),
            (None, Some(walk)) => {
                serde_json::json!({ "features": &walk.features, "skipped": &walk.skipped })
            }
            (None, None) => serde_json::json!({}),
        }
    }

    fn move_to_file(&mut self, file: String) {
        self.form = ListsForm::File;
        self.lists_file = Some(file);
        if let Some(summary) = &mut self.summary {
            summary.sections = None;
        }
        if let Some(walk) = &mut self.walk {
            walk.features = None;
            walk.skipped = None;
        }
    }
}

/// A prune result whose lists can move to a file: both prune primitives'
/// results, bounded by [`bound_result`].
pub(crate) trait BoundedPruneResult: Serialize {
    /// The lists the file holds whole: a feature's `sections`, or a walk's
    /// `features` and `skipped`.
    fn lists(&self) -> serde_json::Value;

    /// Move to the file form: drop those lists, set `form` to `file`, and
    /// name `file` in `lists-file`.
    fn move_to_file(&mut self, file: String);
}

/// Hold `result` to `[artifacts] read-size-bytes`: past it, its lists go to
/// the file it names, with only their counts inline (`bounded_lists::place`;
/// spec 080, AC7). The size judged is the result as it is sent, its own
/// `form` included, not either half alone. Shared by both prune primitives,
/// for one feature and for an `all` walk. A call that applied its reductions
/// has written them by then, so a list file that cannot be written fails a
/// call whose writes stand.
///
/// # Errors
///
/// Those `bounded_lists::place` returns for a malformed read size or a list
/// file that cannot be written.
pub(crate) fn bound_result<R: BoundedPruneResult>(
    repo: &Path,
    primitive: &str,
    key: &str,
    result: &mut R,
) -> Result<()> {
    let lists = result.lists();
    if let Some(file) = super::bounded_lists::place(repo, primitive, key, &*result, &lists)? {
        result.move_to_file(file);
    }
    Ok(())
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
/// spec's status. A feature with nothing to reduce and no `unexamined-tail` is
/// examined and left out; one with no `tasks.md` is skipped with that reason. Under `apply` each
/// permitted reduction is written, and a `--reset` stays gated per spec.
/// Every reduction is computed and every status read before the first
/// write, so a feature that fails either stops the walk with the corpus
/// untouched (scenario `a-path-argument-resolves-inside-its-base`: such a
/// walk is refused as a whole). A reduction whose file changed after it was
/// read is refused by [`Reduction::write`] and listed in `skipped` as
/// `changed-since-read`; the walk goes on, so every other feature's reduction
/// is written and the result names both.
fn walk(args: &PruneTasksArgs, repo: &Path, root: &str) -> Result<PruneWalk<PruneTasksLine>> {
    write_walk(plan_walk(args, repo, root)?)
}

/// A walk's reductions, computed and not yet written.
struct PlannedWalk {
    /// Feature directories walked.
    examined: usize,
    /// Each feature with something to reduce, its reduction and its status.
    pending: Vec<(String, Reduction, String)>,
    /// Features with no `tasks.md`.
    skipped: Vec<SkippedFeature>,
}

/// The first half of [`walk`]: every feature's reduction and status, with
/// nothing written.
fn plan_walk(args: &PruneTasksArgs, repo: &Path, root: &str) -> Result<PlannedWalk> {
    let names = list_feature_dirs(repo, root)?;
    let mut pending = Vec::new();
    let mut skipped = Vec::new();
    for feature in &names {
        match reduce(feature, args, repo, root) {
            Ok(reduction)
                if reduction.summary.nothing_to_prune
                    && reduction.summary.unexamined_tail.is_none() => {}
            Ok(reduction) => {
                // The walk's line carries the status. A reset read it for its
                // gate; keep-pending did not, so read it here, before any
                // write, for a status that will not read stops the walk.
                let status = match &reduction.summary.status {
                    Some(status) => status.clone(),
                    None => read_status(&repo.join(root).join(feature), root, feature)?,
                };
                pending.push((feature.clone(), reduction, status));
            }
            Err(PrimitiveError::TasksFileMissing { .. }) => skipped.push(SkippedFeature {
                feature: feature.clone(),
                reason: SkipReason::NoTasksFile,
            }),
            Err(other) => return Err(other),
        }
    }
    Ok(PlannedWalk {
        examined: names.len(),
        pending,
        skipped,
    })
}

/// The second half of [`walk`]: write each reduction, listing in `skipped`
/// the ones [`PrimitiveError::ChangedSinceRead`] refuses.
fn write_walk(planned: PlannedWalk) -> Result<PruneWalk<PruneTasksLine>> {
    let PlannedWalk {
        examined,
        pending,
        mut skipped,
    } = planned;
    let mut features = Vec::with_capacity(pending.len());
    for (feature, reduction, status) in pending {
        match reduction.write() {
            Ok(summary) => features.push(PruneWalkEntry {
                feature,
                summary: PruneTasksLine::new(summary, status),
            }),
            Err(PrimitiveError::ChangedSinceRead { .. }) => skipped.push(SkippedFeature {
                feature,
                reason: SkipReason::ChangedSinceRead,
            }),
            Err(other) => return Err(other),
        }
    }
    Ok(PruneWalk {
        examined: u32::try_from(examined).unwrap_or(u32::MAX),
        features_count: features.len(),
        unexamined_count: features
            .iter()
            .filter(|entry| entry.summary.unexamined_tail.is_some())
            .count(),
        features: Some(features),
        skipped_count: skipped.len(),
        skipped: Some(skipped),
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
    /// `tasks.md` as the reduction read it, which the write compares against.
    snapshot: ReadSnapshot,
    new_content: String,
}

impl Reduction {
    /// Write the reduction when the call applies it, and return its summary.
    ///
    /// # Errors
    ///
    /// [`PrimitiveError::ChangedSinceRead`] when `tasks.md` changed since the
    /// reduction read it: nothing is written (`BE-TXN-002`).
    fn write(self) -> Result<PruneTasksSummary> {
        if self.summary.applied {
            self.snapshot.write_if_unchanged(&self.new_content)?;
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
    let snapshot = ReadSnapshot::read(repo, &format!("{root}/{feature}/tasks.md"))?;
    let Some(content) = snapshot.text()?.map(str::to_string) else {
        return Err(PrimitiveError::TasksFileMissing {
            root: root.to_string(),
            feature: feature.to_string(),
        });
    };

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

    let (blocks, unexamined_tail) = segment(&content, &tasks_path)?;

    // Per-mode reduction: compute the would-be output, the section records,
    // and the removed/kept counts.
    let (new_content, sections, removed, kept) = match mode {
        PruneMode::KeepPending => reduce_keep_pending(&content, &blocks),
        PruneMode::Reset => reduce_reset(&content, &blocks, &tasks_path)?,
    };
    // The `next-task:` counter (spec 041): written past the highest number the
    // file held before this reduction removed any section, so the next append
    // never reissues a pruned number; a counter already higher stands, and a
    // reduction that removed nothing writes none. A reset of a file that
    // carries one keeps it.
    let held = super::tasks_counter::read(&content, &tasks_path)?;
    let new_content = if removed > 0 {
        let past = super::tasks_counter::after(
            super::tasks_counter::highest_present(&content),
            &tasks_path,
        )?;
        let counted =
            super::tasks_counter::set(&new_content, past.max(held.unwrap_or(0)), &tasks_path)?;
        // `set` writes LF where it adds the block; the file keeps its ending.
        super::with_line_ending(&counted, super::line_ending_of(&content))
    } else {
        new_content
    };

    let nothing_to_prune = new_content == content;
    // A write happens only on `apply`, only when there is a change, and (for
    // reset) only when the gate permits it. keep-pending is never gated.
    let gate_permits = !matches!(gate, PruneGate::BlockedNeedsForce);
    // A file that ends inside an open region is never written: the lines
    // after the opener sit inside the block holding it, so no later section
    // was judged.
    let applied = args.apply && !nothing_to_prune && gate_permits && unexamined_tail.is_none();

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
            sections_count: sections.len(),
            sections: Some(sections),
            path: rel_path(&tasks_path, repo),
            unexamined_tail,
        },
        snapshot,
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
///
/// The frontmatter block the file opens with is the first structure block,
/// whole and unscanned: a `#` line in it is a YAML comment, not a heading, so
/// it opens no block and a prune leaves the block as it found it.
///
/// # Errors
///
/// [`PrimitiveError::MalformedTasks`] when the frontmatter never closes.
fn segment(content: &str, path: &Path) -> Result<(Vec<Block>, Option<UnexaminedTail>)> {
    let frontmatter = super::tasks_counter::frontmatter(content, path)?;
    let body = &content[frontmatter.len()..];
    let task_level: u8 = match detect_tasks_structure(content) {
        TasksStructure::Flat => 2,
        TasksStructure::Phased => 3,
    };
    let phased = task_level == 3;

    let mut blocks: Vec<Block> = Vec::new();
    let mut cur = Block::new(Kind::Structure, "");
    // The initial block starts empty (no first line); clear the placeholder.
    cur.lines.clear();
    cur.lines.extend(frontmatter.lines().map(str::to_string));
    let mut region = OpenRegion::default();
    let mut current_phase_name: Option<String> = None;
    let mut current_phase_idx: Option<usize> = None;

    for (index, line) in body.lines().enumerate() {
        if region.observe(index, line) {
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
    Ok((blocks, region.tail(frontmatter.lines().count())))
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

/// reset reduction: the frontmatter block, existing H1 and
/// [`CANONICAL_EMPTY_TASKS_BODY`]. Every task section is reported as removed.
fn reduce_reset(
    content: &str,
    blocks: &[Block],
    tasks_path: &Path,
) -> Result<(String, Vec<PruneSection>, u32, u32)> {
    // The frontmatter block is not the feature's body: it stays, so the
    // `next-task:` counter survives the reset, and the H1 is searched only
    // below it — a `#` line there is a YAML comment, not the feature's title.
    let frontmatter = super::tasks_counter::frontmatter(content, tasks_path)?;
    // Outside fences and comments, as `segment` reads structure: a `# ` line
    // in an example block is not the feature's identity.
    let mut skip = SkipScanner::default();
    let h1 = content[frontmatter.len()..]
        .lines()
        .find(|line| !skip.skip(line) && matches!(parse_atx_heading(line), Some((1, _))))
        .ok_or_else(|| PrimitiveError::MalformedTasks {
            path: tasks_path.to_path_buf(),
            reason: "no top-level (`#`) heading to preserve the feature identity".to_string(),
        })?;

    // The H1 is lifted from the file being reset, so this is a rewrite of an
    // existing file and not the creation of a new one — the ending it had is
    // the ending it gets back.
    let lead = if frontmatter.is_empty() { "" } else { "\n" };
    let new_content = super::with_line_ending(
        &format!("{frontmatter}{lead}{h1}\n\n{CANONICAL_EMPTY_TASKS_BODY}"),
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

    /// A malformed read size halts an applying call before it writes, so no
    /// reduction stands under an error.
    #[test]
    fn a_malformed_read_size_halts_an_applying_call_before_any_write() {
        let (_tmp, repo) = write_repo(FLAT, Some("in-progress"));
        fs::create_dir_all(repo.join(".ductus")).unwrap();
        fs::write(
            repo.join(".ductus/config.toml"),
            "[artifacts]\nread-size-bytes = 0\n",
        )
        .unwrap();
        let tasks = repo.join("specs/041-task-pruning/tasks.md");
        let err = run(&args(false, false, true), &repo).unwrap_err();
        assert!(
            matches!(err, PrimitiveError::InvalidReadSize { .. }),
            "{err:?}"
        );
        assert_eq!(fs::read_to_string(tasks).unwrap(), FLAT, "nothing written");
    }

    /// Past the read size one feature's `sections` move to a file, while its
    /// `removed-count` and `kept-count`, which count no list, stay inline
    /// beside `sections-count`: the counts the walker reads the file by.
    #[test]
    fn a_summary_past_the_read_size_moves_its_sections_to_a_file() {
        let (_tmp, repo) = write_repo(FLAT, Some("in-progress"));
        fs::create_dir_all(repo.join(".ductus")).unwrap();
        fs::write(
            repo.join(".ductus/config.toml"),
            "[artifacts]\nread-size-bytes = 1\n",
        )
        .unwrap();
        let result = super::run(&args(false, false, false), &repo).unwrap();
        assert_eq!(result.form, ListsForm::File);
        let summary = result.summary.clone().expect("a single-feature summary");
        assert_eq!(summary.sections, None);
        assert_eq!((summary.removed_count, summary.kept_count), (1, 2));
        let file = repo.join(result.lists_file.clone().unwrap());
        let lists: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(file).unwrap()).unwrap();
        assert_eq!(lists["sections"].as_array().map(Vec::len), Some(3));
        crate::primitives::bounded_lists::assert_counts_name_the_file(
            "prune-tasks",
            &serde_json::to_value(&result).unwrap(),
            &lists,
        );
    }

    /// A `#` line in the frontmatter block is a YAML comment, not a heading:
    /// a keep-pending prune leaves the block as it found it, with no blank line
    /// inserted after its opening fence, and a reset keeps the file's own
    /// title (spec 041, review).
    #[test]
    fn a_comment_in_the_frontmatter_is_no_heading() {
        let tasks = "---\n# owner note\nnext-task: 3\n---\n\n# T\n\n## 1. Spent\n\n- [x] a\n\n## 4. Pending\n\n- [ ] b\n";
        let (_tmp, repo) = write_repo(tasks, Some("done"));
        let path = repo.join("specs/041-task-pruning/tasks.md");
        run(&args(false, false, true), &repo).unwrap();
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            "---\n# owner note\nnext-task: 5\n---\n\n# T\n\n## 4. Pending\n\n- [ ] b\n"
        );
        fs::write(&path, tasks).unwrap();
        run(&args(true, false, true), &repo).unwrap();
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            format!("---\n# owner note\nnext-task: 5\n---\n\n# T\n\n{CANONICAL_EMPTY_TASKS_BODY}")
        );
    }

    /// A counter block the prune adds to a CRLF file is written in CRLF, so
    /// the file keeps one ending (spec 041, review).
    #[test]
    fn a_crlf_file_gets_its_counter_block_in_crlf() {
        let tasks = "# T\r\n\r\n## 1. Spent\r\n\r\n- [x] a\r\n\r\n## 2. Pending\r\n\r\n- [ ] b\r\n";
        let (_tmp, repo) = write_repo(tasks, None);
        run(&args(false, false, true), &repo).unwrap();
        let body = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert_eq!(
            body,
            "---\r\nnext-task: 3\r\n---\r\n\r\n# T\r\n\r\n## 2. Pending\r\n\r\n- [ ] b\r\n"
        );
    }

    /// A task numbered `u32::MAX` leaves no counter to write past it: the
    /// prune refuses with the file untouched, rather than write the wrapped
    /// `0` its own reader refuses (spec 041, review).
    #[test]
    fn a_number_with_no_counter_past_it_stops_the_prune() {
        let tasks = "# T\n\n## 4294967295. Spent\n\n- [x] a\n\n## 2. Pending\n\n- [ ] b\n";
        let (_tmp, repo) = write_repo(tasks, None);
        let err = run(&args(false, false, true), &repo).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::MalformedTasks { reason, .. } if reason.contains("4294967295")),
            "{err:?}"
        );
        let body = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert_eq!(body, tasks, "nothing written");
    }

    /// The number a prune removed from the top is not reissued: the prune
    /// writes the counter past the highest number it held, and the next append
    /// takes it (spec 041, scenario `a-pruned-task-number-is-never-reissued`).
    #[test]
    fn a_pruned_top_number_is_not_reissued_by_the_next_append() {
        let tasks = "# T\n\n## 2. Kept\n\n- [ ] a\n\n## 3. Spent\n\n- [x] b\n";
        let (_tmp, repo) = write_repo(tasks, None);
        let result = run(&args(false, false, true), &repo).unwrap();
        assert_eq!(result.removed_count, 1);
        let path = repo.join("specs/041-task-pruning/tasks.md");
        let body = fs::read_to_string(&path).unwrap();
        assert_eq!(
            body,
            "---\nnext-task: 4\n---\n\n# T\n\n## 2. Kept\n\n- [ ] a\n"
        );
        let appended = crate::primitives::append_task::run(
            &crate::schema::primitives::AppendTaskArgs {
                feature_path: "specs/041-task-pruning".into(),
                title: "New".into(),
                done_when: "done.".into(),
                body: Some(vec!["x".into()]),
                slug: None,
                parent_heading: None,
                dedup_title: false,
            },
            &repo,
        )
        .unwrap();
        assert_eq!(appended.task_number, "4");
    }

    /// A prune that removes nothing writes no counter, and a file that was
    /// never pruned stays byte-for-byte.
    #[test]
    fn a_prune_that_removes_nothing_writes_no_counter() {
        let tasks = "# T\n\n## 1. Pending\n\n- [ ] a\n";
        let (_tmp, repo) = write_repo(tasks, None);
        let result = run(&args(false, false, true), &repo).unwrap();
        assert!(result.nothing_to_prune);
        let body = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert_eq!(body, tasks);
    }

    /// A counter that lags the highest number (a hand edit) is left as it is
    /// when nothing is removed: the reduction writes nothing, and the next
    /// append still outvotes the counter, so no number is duplicated.
    #[test]
    fn a_lagging_counter_is_not_rewritten_when_nothing_is_removed() {
        let tasks = "---\nnext-task: 3\n---\n\n# T\n\n## 5. Pending\n\n- [ ] a\n";
        let (_tmp, repo) = write_repo(tasks, None);
        let result = run(&args(false, false, true), &repo).unwrap();
        assert!(result.nothing_to_prune);
        assert!(!result.applied);
        let body = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert_eq!(body, tasks);
    }

    /// A counter already past the highest number stands: a prune never lowers it.
    #[test]
    fn a_prune_never_lowers_an_existing_counter() {
        let tasks = "---\nnext-task: 20\n---\n\n# T\n\n## 1. Spent\n\n- [x] a\n\n## 2. Pending\n\n- [ ] b\n";
        let (_tmp, repo) = write_repo(tasks, None);
        run(&args(false, false, true), &repo).unwrap();
        let body = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert!(body.starts_with("---\nnext-task: 20\n---\n"), "{body}");
        assert!(!body.contains("## 1."));
    }

    /// A reset keeps the counter, so the next append continues past every
    /// number the reset removed; a file with none yields task 1 as AC13 says.
    #[test]
    fn a_reset_keeps_the_counter_and_a_file_without_one_gains_none() {
        let tasks = "# T\n\n## 1. Spent\n\n- [x] a\n\n## 5. Spent\n\n- [x] b\n";
        let (_tmp, repo) = write_repo(tasks, Some("done"));
        run(&args(true, false, true), &repo).unwrap();
        let path = repo.join("specs/041-task-pruning/tasks.md");
        let body = fs::read_to_string(&path).unwrap();
        assert!(
            body.starts_with("---\nnext-task: 6\n---\n\n# T\n\nTasks derived"),
            "{body}"
        );
        // A second reset of the reset file is a no-op, the counter standing.
        let again = run(&args(true, false, true), &repo).unwrap();
        assert!(again.nothing_to_prune);
        assert_eq!(fs::read_to_string(&path).unwrap(), body);

        let (_tmp2, repo2) = write_repo("# T\n\n## Notes\n\nNo tasks.\n", Some("done"));
        run(&args(true, false, true), &repo2).unwrap();
        let plain = fs::read_to_string(repo2.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert!(plain.starts_with("# T\n\nTasks derived"), "{plain}");
    }

    /// The frontmatter block is invisible to the other `tasks.md` readers:
    /// `read-tasks` lists the same tasks and `mark-task` ticks the same box
    /// with and without the counter.
    #[test]
    fn the_counter_block_changes_nothing_the_other_readers_see() {
        use crate::primitives::read_tasks;
        use crate::schema::primitives::ReadTasksArgs;
        let plain = "# T\n\n## 2. Kept\n\n- [ ] a\n- [ ] b\n- **Done when**: done.\n";
        let counted = format!("---\nnext-task: 9\n---\n\n{plain}");
        let (_tmp, repo) = write_repo(&counted, None);
        let tasks = read_tasks::compute(
            &ReadTasksArgs {
                feature: "041-task-pruning".into(),
            },
            &repo,
        )
        .unwrap();
        assert_eq!(tasks.tasks, read_tasks::parse_tasks(plain));
        assert_eq!(tasks.tasks.len(), 1);
        crate::primitives::mark_task::run(
            &crate::schema::primitives::MarkTaskArgs {
                feature: "041-task-pruning".into(),
                task_number: "2".into(),
                subtask_index: 1,
                checked: true,
            },
            &repo,
        )
        .unwrap();
        let body = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert_eq!(
            body,
            counted.replace("- [ ] b", "- [x] b"),
            "only the box changed"
        );
    }

    /// A reset keeps the whole frontmatter block, not the counter alone: the
    /// block precedes the heading the reset preserves.
    #[test]
    fn a_reset_keeps_the_frontmatter_block() {
        let tasks = "---\nowner: x\nnext-task: 2\n---\n\n# T\n\n## 5. Spent\n\n- [x] a\n";
        let (_tmp, repo) = write_repo(tasks, Some("done"));
        run(&args(true, false, true), &repo).unwrap();
        let body = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert!(
            body.starts_with("---\nowner: x\nnext-task: 6\n---\n\n# T\n\nTasks derived"),
            "{body}"
        );
    }

    /// A phased file's counter covers every number it held.
    #[test]
    fn a_phased_prune_counts_past_the_highest_number() {
        let tasks = "# T\n\n## Phase A\n\n### 1. Spent\n\n- [x] a\n\n### 12. Spent\n\n- [x] b\n";
        let (_tmp, repo) = write_repo(tasks, None);
        run(&args(false, false, true), &repo).unwrap();
        let body = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert!(body.starts_with("---\nnext-task: 13\n---\n"), "{body}");
    }

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
        assert!(
            written.starts_with(
                "---\nnext-task: 4\n---\n\n# 041 — Task Pruning Tasks\n\nTasks derived"
            )
        );
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
        // FLAT held tasks 1-3, so the reset keeps a counter past them.
        let expected = format!(
            "---\nnext-task: 4\n---\n\n# 041 — Task Pruning Tasks\n\n{CANONICAL_EMPTY_TASKS_BODY}"
        );
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

    /// A task appended by another writer after the apply read the file is
    /// kept: the apply is refused, never written over it (`BE-TXN-002`).
    #[test]
    fn an_apply_refuses_a_file_changed_after_it_was_read() {
        let (_tmp, repo) = write_repo(FLAT, Some("in-progress"));
        let tasks = repo.join("specs/041-task-pruning/tasks.md");
        let reduction = reduce(
            "041-task-pruning",
            &args(false, false, true),
            &repo,
            "specs",
        )
        .unwrap();
        assert!(reduction.summary.applied, "the reduction would write");
        let theirs = format!("{FLAT}\n## 4. Appended meanwhile\n\n- [ ] e\n");
        fs::write(&tasks, &theirs).unwrap();
        let err = reduction.write().unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::ChangedSinceRead { path }
                if path == Path::new("specs/041-task-pruning/tasks.md")),
            "{err:?}"
        );
        assert_eq!(fs::read_to_string(&tasks).unwrap(), theirs);
    }

    /// The same apply, nothing changing in between, writes.
    #[test]
    fn an_apply_over_an_unchanged_file_writes() {
        let (_tmp, repo) = write_repo(FLAT, Some("in-progress"));
        let summary = run(&args(false, false, true), &repo).unwrap();
        assert!(summary.applied);
        let written = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert!(!written.contains("Done task") && written.contains("Pending task"));
    }

    /// A walk that finds one feature's file changed after it read it writes the
    /// others and lists that one in `skipped` as `changed-since-read`, its file
    /// as the other writer left it.
    #[test]
    fn a_walk_refuses_a_changed_feature_and_writes_the_others() {
        let tmp = corpus(&[
            ("002-first", "in-progress", Some(FLAT)),
            ("003-second", "in-progress", Some(FLAT)),
        ]);
        let planned = plan_walk(&all(false, false, true), tmp.path(), "specs").unwrap();
        let theirs = format!("{FLAT}\n## 4. Appended meanwhile\n\n- [ ] e\n");
        fs::write(tmp.path().join("specs/002-first/tasks.md"), &theirs).unwrap();
        let walked = write_walk(planned).unwrap();
        let written: Vec<&str> = walked
            .features
            .as_ref()
            .unwrap()
            .iter()
            .map(|entry| entry.feature.as_str())
            .collect();
        assert_eq!(written, ["003-second"]);
        let skipped = walked.skipped.unwrap();
        assert_eq!(skipped.len(), 1);
        assert_eq!(skipped[0].feature, "002-first");
        assert_eq!(skipped[0].reason, SkipReason::ChangedSinceRead);
        assert_eq!(
            fs::read_to_string(tmp.path().join("specs/002-first/tasks.md")).unwrap(),
            theirs
        );
        let second = fs::read_to_string(tmp.path().join("specs/003-second/tasks.md")).unwrap();
        assert!(!second.contains("Done task"));
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

    fn set_read_size(repo: &Path, bytes: usize) {
        fs::create_dir_all(repo.join(".ductus")).unwrap();
        fs::write(
            repo.join(".ductus/config.toml"),
            format!("[artifacts]\nread-size-bytes = {bytes}\n"),
        )
        .unwrap();
    }

    /// The bound judges the result as it is sent, its own `form` included: a
    /// result exactly at the read size is inline, and one byte under it is
    /// not, for one feature and for a walk alike.
    #[test]
    fn a_result_is_held_to_the_read_size_as_it_is_sent() {
        let (_single, repo) = write_repo(FLAT, Some("in-progress"));
        let walked = corpus(&[
            ("002-first", "in-progress", Some(FLAT)),
            ("003-second", "in-progress", Some(FLAT)),
        ]);
        for (repo, args) in [
            (repo.as_path(), args(false, false, false)),
            (walked.path(), all(false, false, false)),
        ] {
            let whole = super::run(&args, repo).unwrap();
            assert_eq!(whole.form, ListsForm::Inline);
            let sent = serde_json::to_vec(&whole).unwrap().len();
            set_read_size(repo, sent);
            let fits = super::run(&args, repo).unwrap();
            assert_eq!(fits.form, ListsForm::Inline, "exactly at the size fits");
            set_read_size(repo, sent - 1);
            let over = super::run(&args, repo).unwrap();
            assert_eq!(over.form, ListsForm::File, "one byte over does not");
            assert!(over.lists_file.is_some());
        }
    }

    /// Past the read size a walk's `features` and `skipped` move to a file,
    /// with their counts inline, and the result carries `form` and
    /// `lists-file` once.
    #[test]
    fn a_walk_past_the_read_size_moves_its_lists_to_a_file() {
        let tmp = corpus(&[
            ("002-first", "in-progress", Some(FLAT)),
            ("003-no-tasks", "clarified", None),
        ]);
        set_read_size(tmp.path(), 1);
        let result = super::run(&all(false, false, false), tmp.path()).unwrap();
        assert_eq!(result.form, ListsForm::File);
        assert!(result.summary.is_none());
        let walk = result.walk.clone().expect("an all walk");
        assert_eq!((walk.features, walk.skipped), (None, None));
        assert_eq!((walk.features_count, walk.skipped_count), (1, 1));
        let file = tmp.path().join(result.lists_file.clone().unwrap());
        let lists: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(file).unwrap()).unwrap();
        assert_eq!(lists["features"].as_array().map(Vec::len), Some(1));
        assert_eq!(lists["skipped"].as_array().map(Vec::len), Some(1));
        let wire = serde_json::to_value(&result).unwrap();
        assert!(wire.get("features").is_none() && wire.get("skipped").is_none());
        assert_eq!(
            (
                wire["features-count"].as_u64(),
                wire["skipped-count"].as_u64()
            ),
            (Some(1), Some(1))
        );
    }

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
        let order: Vec<&str> = walk
            .features
            .as_deref()
            .unwrap()
            .iter()
            .map(|f| f.feature.as_str())
            .collect();
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
            walk.skipped.as_deref().unwrap(),
            [SkippedFeature {
                feature: "003-no-tasks".into(),
                reason: SkipReason::NoTasksFile,
            }]
        );
        // The lean feature is examined but, with nothing to reduce, not listed.
        assert_eq!(walk.examined, 6);
        assert!(
            walk.features
                .as_deref()
                .unwrap()
                .iter()
                .all(|f| !f.summary.applied)
        );
        // Keep-pending reads no status for its own work; the walk reads each
        // listed spec's own.
        let statuses: Vec<(&str, &str)> = walk
            .features
            .as_deref()
            .unwrap()
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

    /// A walk is refused as a whole when one feature fails the path check:
    /// the feature before it in corpus order keeps its file, though its
    /// reduction was computed first.
    #[cfg(unix)]
    #[test]
    fn an_applying_walk_refused_by_a_later_feature_writes_nothing() {
        let tmp = corpus(&[
            ("002-first", "in-progress", Some(FLAT)),
            ("003-linked", "in-progress", Some(FLAT)),
        ]);
        let outside = tempdir().unwrap();
        fs::write(outside.path().join("tasks.md"), FLAT).unwrap();
        let linked = tmp.path().join("specs/003-linked/tasks.md");
        fs::remove_file(&linked).unwrap();
        std::os::unix::fs::symlink(outside.path().join("tasks.md"), &linked).unwrap();
        let err = super::run(&all(false, false, true), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidPath { .. }), "{err}");
        let first = fs::read_to_string(tmp.path().join("specs/002-first/tasks.md")).unwrap();
        assert_eq!(first, FLAT, "the earlier feature was not written");
        let outer = fs::read_to_string(outside.path().join("tasks.md")).unwrap();
        assert_eq!(outer, FLAT, "nothing outside the project was written");
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
            .as_deref()
            .unwrap()
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
            format!(
                "---\nnext-task: 4\n---\n\n# 041 — Task Pruning Tasks\n\n{CANONICAL_EMPTY_TASKS_BODY}"
            )
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
        let tasks = read_tasks::compute(
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

        // Strip the counter the reset kept: a reset file with none (AC13)
        // numbers from 1, not from the commented examples.
        let tasks_path = repo.join("specs/041-task-pruning/tasks.md");
        let kept = fs::read_to_string(&tasks_path).unwrap();
        fs::write(&tasks_path, kept.replace("---\nnext-task: 4\n---\n\n", "")).unwrap();
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
            appended.task_number, "1",
            "append-task on a reset file must number from 1"
        );
    }

    /// The adopter's `tasks.md` (spec 041, scenario
    /// `a-suffixed-task-number-is-a-task`): a task hand-inserted between
    /// two others as `11a`, under a phase, each section fully ticked.
    const SUFFIXED_SPENT: &str = "# T\n\nTasks derived from the [plan](plan.md). Complete in order.\n\n## Phase 1\n\n### 11. Spent task\n\n- [x] a\n- **Done when**: done.\n\n### 11a. Spent task with a suffixed number\n\n- [x] b\n- **Done when**: done.\n";

    /// Both spent sections leave and their phase follows, so no `###` is
    /// stranded directly under the `#` title (MD001).
    #[test]
    fn suffixed_task_spent_leaves_with_its_phase() {
        let (_tmp, repo) = write_repo(SUFFIXED_SPENT, None);
        let result = run(&args(false, false, true), &repo).unwrap();
        assert_eq!(result.removed_count, 2);
        assert_eq!(result.kept_count, 0);
        let written = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert!(!written.contains("###"), "{written}");
        assert!(!written.contains("Phase 1"), "{written}");
    }

    #[test]
    fn suffixed_task_pending_stays_with_its_phase() {
        let tasks = SUFFIXED_SPENT.replace("- [x] b", "- [ ] b");
        let (_tmp, repo) = write_repo(&tasks, None);
        let result = run(&args(false, false, true), &repo).unwrap();
        assert_eq!(result.removed_count, 1);
        assert_eq!(result.kept_count, 1);
        let written = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert!(written.contains("### 11a. Spent task with a suffixed number"));
        assert!(written.contains("## Phase 1"));
        assert!(!written.contains("### 11. Spent task"));
    }

    /// A walk of a spec root that is a link out of the project is refused,
    /// not reported as a corpus that held no feature (`BE-INPUT-004`; 022
    /// task 48).
    #[cfg(unix)]
    #[test]
    fn a_walk_of_a_spec_root_linked_outside_the_project_is_refused() {
        let tmp = tempdir().unwrap();
        let _outside = crate::primitives::outward::dir(tmp.path(), "specs", |_| {});
        let result = super::run(&all(false, false, true), tmp.path());
        assert!(crate::primitives::outward::refused(&result), "{result:?}");
    }

    /// A spent task, then a fence that never closes, then a pending task: the
    /// pending task's heading is inside the open fence, so the file is
    /// reported unexamined from the opener and no apply may write it.
    const OPEN_FENCE: &str =
        "# T\n\n## 1. Done one\n\n- [x] a\n\n```sh\n\n## 2. Pending two\n\n- [ ] b\n";

    /// A spent task, then an HTML comment that never closes, then a pending
    /// task.
    const OPEN_COMMENT: &str =
        "# T\n\n## 1. Done one\n\n- [x] a\n\n<!-- a note\n\n## 2. Pending two\n\n- [ ] b\n";

    /// An unclosed fence is reported, never read as clean, and an apply
    /// writes nothing: before, the pending task fell into the spent task's
    /// block and an apply deleted it (spec 041, scenario
    /// `an-open-fenced-block-or-comment-is-not-examined`).
    #[test]
    fn an_unclosed_fence_is_reported_and_an_apply_writes_nothing() {
        let (_tmp, repo) = write_repo(OPEN_FENCE, Some("in-progress"));
        let preview = run(&args(false, false, false), &repo).unwrap();
        assert_eq!(
            preview.unexamined_tail,
            Some(UnexaminedTail {
                from_line: 7,
                cause: TailCause::OpenFence
            })
        );
        let applied = run(&args(false, false, true), &repo).unwrap();
        assert!(!applied.applied);
        assert_eq!(applied.unexamined_tail, preview.unexamined_tail);
        let body = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert_eq!(body, OPEN_FENCE, "nothing written; task 2 still present");
    }

    #[test]
    fn an_unclosed_comment_is_reported_and_an_apply_writes_nothing() {
        let (_tmp, repo) = write_repo(OPEN_COMMENT, Some("in-progress"));
        let applied = run(&args(false, false, true), &repo).unwrap();
        assert_eq!(
            applied.unexamined_tail,
            Some(UnexaminedTail {
                from_line: 7,
                cause: TailCause::OpenComment
            })
        );
        assert!(!applied.applied);
        let body = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert_eq!(body, OPEN_COMMENT);
    }

    /// `from-line` is a line of the whole file, frontmatter included.
    #[test]
    fn the_tail_line_counts_the_frontmatter() {
        let tasks = format!("---\nnext-task: 3\n---\n{OPEN_FENCE}");
        let (_tmp, repo) = write_repo(&tasks, Some("in-progress"));
        let result = run(&args(false, false, false), &repo).unwrap();
        assert_eq!(result.unexamined_tail.map(|tail| tail.from_line), Some(10));
    }

    /// A fence that closes reports no tail and the reduction is the one it
    /// always was; the key is absent from the wire.
    #[test]
    fn a_closed_fence_reports_no_tail() {
        let tasks = OPEN_FENCE.replace("```sh\n\n", "```sh\n\n```\n\n");
        let (_tmp, repo) = write_repo(&tasks, Some("in-progress"));
        let result = super::run(&args(false, false, true), &repo).unwrap();
        let summary = result.summary.expect("a summary");
        assert_eq!(summary.unexamined_tail, None);
        assert_eq!(summary.removed_count, 1);
        assert!(summary.applied);
        let wire = serde_json::to_value(&summary).unwrap();
        assert!(!wire.as_object().unwrap().contains_key("unexamined-tail"));
    }

    /// A reset discards every section, so one over an open region is held back
    /// like any apply.
    #[test]
    fn a_reset_with_an_open_region_writes_nothing() {
        let (_tmp, repo) = write_repo(OPEN_FENCE, Some("done"));
        let result = run(&args(true, false, true), &repo).unwrap();
        assert!(!result.applied);
        assert!(result.unexamined_tail.is_some());
        let body = fs::read_to_string(repo.join("specs/041-task-pruning/tasks.md")).unwrap();
        assert_eq!(body, OPEN_FENCE);
    }

    /// A spec with nothing to reduce whose file ends in an open fence.
    const PENDING_ONLY_OPEN: &str = "# T\n\n## 1. Pending\n\n- [ ] a\n\n```sh\n\n## 2. Hidden\n";

    /// Under `all` a spec with a tail is listed with it and counted, even when
    /// it has nothing to reduce, and every other spec is still reduced.
    #[test]
    fn a_walk_lists_a_spec_with_a_tail_and_writes_the_others() {
        let spent_only = "# T\n\n## 1. Done one\n\n- [x] a\n";
        let tmp = corpus(&[
            ("002-open", "in-progress", Some(OPEN_FENCE)),
            ("003-clean", "in-progress", Some(spent_only)),
            ("004-pending", "in-progress", Some(PENDING_ONLY)),
            ("005-quiet", "in-progress", Some(PENDING_ONLY_OPEN)),
        ]);
        let result = super::run(&all(false, false, true), tmp.path()).unwrap();
        let walk = result.walk.expect("a walk");
        assert_eq!(walk.unexamined_count, 2);
        let listed: Vec<&str> = walk
            .features
            .as_ref()
            .unwrap()
            .iter()
            .map(|entry| entry.feature.as_str())
            .collect();
        assert_eq!(listed, ["002-open", "003-clean", "005-quiet"]);
        let open = &walk.features.as_ref().unwrap()[0].summary;
        assert!(!open.applied);
        assert_eq!(open.unexamined_tail.map(|tail| tail.from_line), Some(7));
        assert!(walk.features.as_ref().unwrap()[1].summary.applied);
        let read = |name: &str| {
            fs::read_to_string(tmp.path().join(format!("specs/{name}/tasks.md"))).unwrap()
        };
        assert_eq!(read("002-open"), OPEN_FENCE);
        assert_ne!(read("003-clean"), spent_only);
        let wire = serde_json::to_value(&walk).unwrap();
        assert_eq!(wire["unexamined-count"], 2);
    }
}
