//! `check-artifacts` — the residual deterministic check families from
//! `/ductus:analyze`'s markdown-only reference, mechanized for one feature.
//!
//! Owns nine families (spec 022, scenarios analyze-artifact-checks,
//! scenario-open-question-signal, link-adjacent-drift-family,
//! criterion-path-existence-family, and criterion-label-assignment; spec 058
//! for disposition drift). Each family MIRRORS
//! `framework/commands/analyze.md`'s markdown-only reference — severity
//! tiers and skip rules come from the reference, the primitive introduces
//! no policy of its own:
//!
//! - **artifact-completeness** (blocking) — reference §"Artifact
//!   completeness (blocking)": `plan.md` and
//!   `tasks.md` are required when status is `planned` or later
//!   (`planned` / `in-progress` / `done`). `data-model.md` is **never**
//!   required here: the reference conditions it on "feature introduces or
//!   modifies domain entities" — a semantic judgment the runtime cannot
//!   make deterministically, so it stays optional (and with the prose
//!   check on the markdown-only path).
//! - **task-consistency** (blocking) — reference §"Task consistency
//!   (blocking if tasks exist)": task numbers
//!   are strictly increasing in declaration order, and every task section
//!   carries a `Done when` clause. The reference's "tasks reference the
//!   plan" item is a semantic-link judgment and stays in the
//!   markdown-only reference. Runs only when `tasks.md` exists.
//! - **scenario-consistency** (advisory) — reference §"Scenario
//!   consistency (advisory)": every
//!   `scenarios/*.md` has a referencing task in `tasks.md` *only while
//!   that task is still pending*. Under a `done` spec a scenario with no
//!   task in the current file is flagged only when no revision of
//!   `tasks.md` ever named it, since a done feature's tasks may have been
//!   pruned (000's `scenario-without-task-visibility`); a history that cannot
//!   be consulted flags nothing. It never requires a pruned spent task to
//!   persist (constitution §tasks-phase — `tasks.md` is ephemeral; see
//!   [`pruning_evidence`] for the documented heuristic).
//! - **review-state-drift** (blocking) — reference §"Review state drift
//!   (blocking)": a `done` spec whose `review.md` record has `last-run`
//!   unset, `blocking: true`, or a non-zero `should-violations`,
//!   drifted. The third condition arrived with 045's task 15: §implement-phase
//!   forbids reaching `done` over an outstanding SHOULD, and the count is what
//!   states whether one is outstanding. The grandfather rule applies: a `done`
//!   spec with no `review:` block at all predates `/ductus:review` and is
//!   exempt.
//! - **artifact-unreadable at `done`** (blocking, across families) — an
//!   artifact a family was meant to scan but could not read is a skipped
//!   target below `done` and a **blocking finding at it**. The general rule
//!   is that an unknown is never escalated into a defect, and this is the one
//!   exception: that rule is about *other* files — another spec's
//!   frontmatter, an upstream service — where the defect is not this spec's.
//!   Here the subject is the spec's own artifact, in its own directory, that
//!   its own analysis could not read. The concrete case is
//!   `scenario-open-questions`, which blocks at `done`: an unreadable
//!   scenario contributed no questions and, as a skip, no finding, so a
//!   scenario carrying unresolved questions that would not parse passed the
//!   gate built to catch exactly that. See [`record_unreadable_artifact`].
//! - **disposition-drift** (blocking) — a `done` spec whose `review.md`
//!   records one or more undispositioned findings (spec 058). The pre-`done`
//!   gate blocks on the same count, so a `done` spec carrying one either
//!   predates the gate or had its record re-run after it closed, and either
//!   way `done` no longer means done. A record without a `dispositions:` map
//!   produces nothing here: it predates the field, and a backfilled map would
//!   assert dispositions nobody made. `/{project}:analyze --fix` reverts on
//!   this family by name.
//! - **scenario-open-questions** (blocking at `done`, advisory otherwise)
//!   — a scenario is an organizational split of the spec, so its
//!   unresolved questions are the spec's questions for completeness. At
//!   `done` that state contradicts the completion rule outright; before
//!   `done` the questions are real remaining work but not yet a defect.
//!   Deliberately **no grandfather rule**, unlike review-state drift: an
//!   unresolved question is a present-tense defect whenever it arrived
//!   (spec 046).
//! - **link-adjacent-drift** (advisory) — an artifact's own prose asserting
//!   an open state that its own sibling link's target contradicts: the
//!   question called open while the target reports none, the work called
//!   absent while the target is `in-progress` or `done`. The grounding
//!   check is structurally blind to this — it verifies a claim is *cited*,
//!   not that it is *true*, so a stale claim citing its source passes
//!   (spec 045). Advisory at introduction, with a documented promotion
//!   criterion in `analyze.md`.
//! - **criterion-path-existence** (advisory) — a filesystem path named in a
//!   `done` spec's acceptance criterion that no longer resolves. An AC is a
//!   contract, so naming a path asserts it is part of the delivered system;
//!   nothing re-verifies that after a later spec deletes the subject. Reads
//!   **inside** inline code spans, the inverse of the family above — which
//!   is why the two are separate families rather than one check with a flag
//!   (spec 045).
//! - **criterion-labels** (advisory) — the enforcement half of the `AC{n}`
//!   labelling pass: a duplicate label within one spec, a `next-criterion`
//!   that no longer exceeds the body, and an unlabelled criterion in a spec
//!   that has been labelled. Assignment is `label-criteria`'s, enforcement
//!   is this family's, because a criterion typed by hand in an editor never
//!   touches a primitive (spec 013).
//!
//! **`analysis.md` is judged by no family here** — neither its dispositions
//! nor its own state (`last-run`, `blocking`). Every family runs inside
//! `/{project}:analyze`, whose own run is about to replace that record, so a
//! finding read from it could never clear: it is blocking, cannot be
//! discarded, and outlives every re-run. `/{project}:analyze` judges both from
//! the record it writes instead (scenarios
//! `analysis-drift-judges-the-record-it-writes` and
//! `analyze-state-drift-judges-the-record-it-writes`). The analyze-state drift
//! family spec 047 added here is gone for that reason.
//!
//! Parsing reuses the shared machinery — `split_frontmatter` for the spec
//! frontmatter, [`crate::primitives::read_tasks`] for the task list,
//! [`crate::primitives::read_spec`] for spec state and scenario questions,
//! [`crate::primitives::split_blocks`] for the prose unit, and
//! `label_criteria::stored_counter` for the criterion counter — so this
//! primitive sees exactly the artifact structure every other primitive
//! sees (no hand-rolled parsers).

use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::{Component, Path, PathBuf};

use crate::primitives::label_criteria::StoredCounter;
use crate::primitives::{
    MarkdownBlock, PrimitiveError, ProjectRepository, Result, inline_code_spans, label_criteria,
    list_scenario_files, read_spec, read_tasks, read_text, rel_path, scenario_name_cmp,
    section_line_indices, split_blocks,
};
use crate::schema::paths;
use crate::schema::primitives::{
    ArtifactFinding, CheckArtifactsArgs, CheckArtifactsResult, ReadSpecArgs, ReadSpecResult,
    ReadTasksArgs, SkippedTarget, Task,
};
use crate::schema::severity::AnalyzeSeverity;
use crate::schema::status::COMPATIBLE_STATUSES;

/// Execute the `check-artifacts` primitive against the given repo root.
///
/// # Errors
///
/// Returns [`PrimitiveError::FeatureNotFound`] when the feature directory
/// is absent, [`PrimitiveError::MissingFrontmatter`] /
/// [`PrimitiveError::UnclosedFrontmatter`] / [`PrimitiveError::Yaml`] when
/// `spec.md` has no parseable frontmatter (the frontmatter-schema family is
/// `validate-frontmatter`'s job — this primitive needs a readable `status` to
/// classify tiers at all), or
/// [`PrimitiveError::Io`] on filesystem failures.
pub fn run(args: &CheckArtifactsArgs, repo: &Path) -> Result<CheckArtifactsResult> {
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
    // One read of the spec, through `read-spec` — the same delegation the task
    // list and the scenario questions already use. Parsing it a second time
    // here would leave two independent notions of the spec's frontmatter in
    // one function, which is the drift the no-hand-rolled-parsers constraint
    // in the module docs exists to prevent. `read-spec` raises the same
    // FeatureNotFound / MissingFrontmatter / UnclosedFrontmatter / Yaml / Io
    // variants on the same file, so the documented error contract above is
    // unchanged.
    let spec = read_spec::run(
        &ReadSpecArgs {
            feature: args.feature.clone(),
            include_body: false,
        },
        repo,
    )?;
    let frontmatter = &spec.frontmatter;
    let status = frontmatter.status.clone();

    let mut findings: Vec<ArtifactFinding> = Vec::new();
    check_completeness(&mut findings, &feature_dir, &root, &args.feature, &status);

    // Task parsing is shared by families (b) and (c); parse once.
    let tasks = if feature_dir.join("tasks.md").is_file() {
        Some(read_tasks::run(
            &ReadTasksArgs {
                feature: args.feature.clone(),
            },
            repo,
        )?)
    } else {
        None
    };
    if let Some(tasks) = &tasks {
        check_task_consistency(&mut findings, &tasks.tasks, &tasks.path);
    }
    check_scenario_consistency(
        &mut findings,
        &feature_dir,
        &root,
        &args.feature,
        &status,
        tasks.as_ref().map(|t| t.tasks.as_slice()),
        repo,
    );
    // The records live in their own artifacts now (spec 057), and `read-spec`
    // has already loaded the review record. An unreadable one is deliberately
    // treated as absent here rather than reported — `read-spec` collapses it
    // the same way: this family's subject is *drift between a done spec and
    // its record*, and a record that will not parse is a different defect,
    // owned by `validate-frontmatter` and by the pre-done gate. Reporting it
    // twice, in two vocabularies, is how one problem becomes two findings.
    let review_record = spec.review.as_ref();
    check_review_drift(&mut findings, review_record, &status, &spec_path, repo);
    check_disposition_drift(&mut findings, review_record, &status, &spec_path, repo);

    let mut skipped: Vec<SkippedTarget> = Vec::new();
    check_scenario_open_questions(
        &mut findings,
        &mut skipped,
        &feature_dir,
        &spec,
        &spec_path,
        repo,
    );
    check_link_adjacent_drift(&mut findings, &mut skipped, &feature_dir, &spec, repo);
    check_criterion_path_existence(
        &mut findings,
        &mut skipped,
        &status,
        &spec,
        &spec_path,
        repo,
    );
    check_criterion_labels(&mut findings, &spec, &spec_path, repo);

    let clean = findings.is_empty();
    Ok(CheckArtifactsResult {
        feature: args.feature.clone(),
        status,
        findings,
        clean,
        skipped,
        path: rel_path(&spec_path, repo),
    })
}

/// (a) Artifact completeness — reference §"Artifact completeness
/// (blocking)": `plan.md` / `tasks.md` required at `planned` or later. A
/// `draft` or `clarified` spec with neither file produces no finding
/// (files are required by status tier, not universally). `data-model.md`
/// is never required (see module docs).
fn check_completeness(
    findings: &mut Vec<ArtifactFinding>,
    feature_dir: &Path,
    root: &str,
    feature: &str,
    status: &str,
) {
    // "planned or later" is the same lifecycle tail `schema::status`
    // derives as `COMPATIBLE_STATUSES` (planned / in-progress / done).
    if !COMPATIBLE_STATUSES.contains(&status) {
        return;
    }
    for file in ["plan.md", "tasks.md"] {
        if !feature_dir.join(file).is_file() {
            findings.push(ArtifactFinding {
                family: "artifact-completeness".into(),
                severity: AnalyzeSeverity::Blocking,
                message: format!("{file} is required at status '{status}' but does not exist"),
                path: format!("{root}/{feature}/{file}"),
            });
        }
    }
}

/// (b) Task consistency — reference §"Task consistency (blocking if
/// tasks exist)": numbered headings strictly increasing in declaration
/// order, and every task section carries a `Done when` clause.
fn check_task_consistency(findings: &mut Vec<ArtifactFinding>, tasks: &[Task], tasks_path: &str) {
    let mut prev: Option<u32> = None;
    for task in tasks {
        if let Ok(number) = task.number.parse::<u32>() {
            if let Some(previous) = prev
                && number <= previous
            {
                findings.push(ArtifactFinding {
                    family: "task-consistency".into(),
                    severity: AnalyzeSeverity::Blocking,
                    message: format!(
                        "task numbering is not strictly increasing: task {number} follows task {previous}"
                    ),
                    path: tasks_path.to_string(),
                });
            }
            prev = Some(number);
        }
        if task.done_when.is_none() {
            findings.push(ArtifactFinding {
                family: "task-consistency".into(),
                severity: AnalyzeSeverity::Blocking,
                message: format!(
                    "task {} ({}) has no Done when clause",
                    task.number, task.heading
                ),
                path: tasks_path.to_string(),
            });
        }
    }
}

/// (c) Scenario→task mapping — reference §"Scenario consistency
/// (advisory)". Skip rules, in order:
///
/// - Spec at `done` → an unmapped scenario is a finding **only when no task
///   for it ever existed**, proven from `tasks.md` history rather than
///   inferred from the file. `tasks.md` is not a durable index
///   (§tasks-phase), so its *current* silence proves nothing: a spent task
///   may have been pruned or the file reset. Its *history* does — a
///   scenario that was implemented had a task at some point, and one that
///   was hand-added and never implemented never did. The family used to
///   skip `done` wholesale, which left a committed, question-free,
///   never-tasked scenario invisible to every check while its spec stayed
///   `done` (spec 000 scenario `scenario-without-task-visibility`).
///   Measured over this repo's 46 `done` specs before the rule shipped:
///   one unmapped scenario, which *was* tasked historically — so the
///   probe fires zero times here, and the file-shape alternative would
///   have produced exactly one false positive, the direction §tasks-phase
///   forbids.
/// - No `tasks.md` → not evaluable; the completeness family already owns
///   the missing-file signal at `planned`+, and a pre-plan spec's
///   scenarios have no tasks yet by design.
/// - `tasks.md` shows [`pruning_evidence`] → the mapping is satisfied for
///   every unmapped scenario (§tasks-phase: a pruned spent task never
///   produces a finding).
///
/// A scenario is *mapped* when its slug appears in any task's heading,
/// subtask text, or `Done when` clause — this matches `append-task`'s
/// default-body convention (`scenarios/{slug}.md`) while tolerating
/// hand-written references that name the slug without the path.
fn check_scenario_consistency(
    findings: &mut Vec<ArtifactFinding>,
    feature_dir: &Path,
    root: &str,
    feature: &str,
    status: &str,
    tasks: Option<&[Task]>,
    repo: &Path,
) {
    let Some(tasks) = tasks else {
        return;
    };
    let slugs = scenario_slugs(feature_dir);
    if slugs.is_empty() {
        return;
    }
    if pruning_evidence(tasks) {
        return;
    }
    let done = status == "done";
    let unmapped: Vec<String> = slugs
        .into_iter()
        .filter(|slug| !scenario_mapped(tasks, slug))
        .collect();
    // One history walk for every unmapped slug, and only when a `done` spec
    // actually has one — the common case does no git work at all.
    let ever = if done && !unmapped.is_empty() {
        ever_tasked_slugs(repo, root, feature, &unmapped)
    } else {
        None
    };
    for slug in unmapped {
        // On a `done` spec the mapping is not a durable index, so absence in
        // the current file is not evidence of anything. Only a scenario that
        // never had a task in any revision is a finding; an unconsultable
        // history (`None`) suppresses every one.
        if done && ever.as_ref().is_none_or(|seen| seen.contains(&slug)) {
            continue;
        }
        let message = if done {
            format!(
                "scenario {slug}.md has no task in tasks.md and never had one in its history, \
                 so it may describe behavior that was never implemented — or it may document \
                 already-shipped behavior written after the fact, which this check cannot \
                 distinguish. The operator decides; nothing is reopened automatically"
            )
        } else {
            format!(
                "scenario {slug}.md has no corresponding task in tasks.md and the file \
                 shows no pruning evidence"
            )
        };
        findings.push(ArtifactFinding {
            family: "scenario-consistency".into(),
            severity: AnalyzeSeverity::Advisory,
            message,
            path: format!("{root}/{feature}/scenarios/{slug}.md"),
        });
    }
}

/// Which of `slugs` any revision of the feature's `tasks.md` ever named.
///
/// The pickaxe question — *did a task for this scenario exist at any point* —
/// answered by walking the file's history and testing each blob. A scenario
/// that was implemented had a task before it was pruned; one that was
/// hand-added and never implemented never did.
///
/// **Fails safe toward the missed finding.** `None` means the history could not
/// be consulted at all — no git repository, an unreadable walk — and the caller
/// treats that as *every slug was tasked*, suppressing every finding. §tasks-phase mandates that direction explicitly
/// (*"a pruned spent task never produces a finding"*), so a check that cannot
/// consult history must not manufacture one from its own blindness. This is the
/// opposite default from [`crate::primitives::mechanical_sweep`], where an
/// unreadable diff withholds an *exemption*; there the unprovable thing is a
/// claim of sameness, here it is a claim of absence.
fn ever_tasked_slugs(
    repo: &Path,
    root: &str,
    feature: &str,
    slugs: &[String],
) -> Option<BTreeSet<String>> {
    // History names the file from the git work tree, which is not the
    // project root when the project sits in a subdirectory of its repository.
    let project = ProjectRepository::discover(repo).ok()?;
    let repository = &project.repository;
    let rel = project.to_git(&format!("{root}/{feature}/tasks.md"));
    // A shallow clone holds only part of the history, so a scenario whose
    // task lived in a commit it lacks would read as never tasked. The history
    // cannot be consulted in full, which fails safe below.
    if repository.is_shallow() {
        return None;
    }
    let mut walk = repository.revwalk().ok()?;
    walk.push_head().ok()?;
    let mut seen: BTreeSet<String> = BTreeSet::new();
    // `tasks.md` is unchanged across most commits, so each distinct blob is
    // decoded and scanned once however many commits carry it.
    let mut scanned: HashSet<git2::Oid> = HashSet::new();
    for oid in walk {
        if seen.len() == slugs.len() {
            break;
        }
        // Any revision the walk cannot read fails the whole walk safe, as
        // 000's `scenario-without-task-visibility` requires of an unreadable
        // history or a non-UTF-8 blob: skipping it could flag a scenario whose
        // task appeared only there.
        let tree = repository.find_commit(oid.ok()?).ok()?.tree().ok()?;
        let Ok(entry) = tree.get_path(Path::new(&rel)) else {
            // `tasks.md` did not exist at this revision.
            continue;
        };
        if !scanned.insert(entry.id()) {
            continue;
        }
        let blob = repository.find_blob(entry.id()).ok()?;
        let text = std::str::from_utf8(blob.content()).ok()?;
        for slug in slugs {
            if !seen.contains(slug) && text.contains(slug.as_str()) {
                seen.insert(slug.clone());
            }
        }
    }
    Some(seen)
}

/// List scenario slugs (`*.md` basenames without extension) under the
/// feature's `scenarios/` directory, sorted. Empty when the directory is
/// absent. Enumerates via the shared [`list_scenario_files`] so the `.md`
/// match is CASE-INSENSITIVE — the same set `dashboard` counts, closing
/// the `FOO.MD`-counted-by-one-surface-only divergence.
fn scenario_slugs(feature_dir: &Path) -> Vec<String> {
    let mut slugs: Vec<String> = list_scenario_files(&feature_dir.join("scenarios"))
        .iter()
        .filter_map(|name| {
            Path::new(name)
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(str::to_string)
        })
        .collect();
    // Same comparator every scenario-presenting surface uses, so stripping
    // the extension here cannot reintroduce a second order (spec 046).
    slugs.sort_by(|a, b| scenario_name_cmp(a, b));
    slugs
}

/// `true` when any task references the scenario slug (heading, subtask
/// text, or `Done when` clause).
fn scenario_mapped(tasks: &[Task], slug: &str) -> bool {
    tasks.iter().any(|task| {
        task.heading.contains(slug)
            || task.subtasks.iter().any(|s| s.text.contains(slug))
            || task.done_when.as_deref().is_some_and(|d| d.contains(slug))
    })
}

/// Pruning-evidence heuristic (§tasks-phase). `prune-tasks` reduces a
/// `tasks.md` in two shapes, and each leaves a deterministic fingerprint:
///
/// - **reset** rewrites the file to template state → the file parses to
///   **zero task sections**;
/// - **keep-pending** drops spent sections verbatim without renumbering
///   the survivors → the surviving numbers are **non-contiguous** (the
///   first number exceeds 1, or a gap appears between consecutive
///   numbers).
///
/// Either fingerprint counts as evidence. The heuristic is deliberately
/// coarse: evidence anywhere in the file vouches for *every* unmapped
/// scenario, because the primitive cannot know which pruned section
/// referenced which scenario — and §tasks-phase forbids requiring a spent
/// task to persist, so the mandated direction of error is the missed
/// finding, never the false one. (A fresh template-state `tasks.md` on a
/// pre-implementation spec also matches the zero-sections fingerprint and
/// is likewise not flagged — same direction of error.)
fn pruning_evidence(tasks: &[Task]) -> bool {
    if tasks.is_empty() {
        return true;
    }
    let numbers: Vec<u32> = tasks
        .iter()
        .filter_map(|t| t.number.parse::<u32>().ok())
        .collect();
    if let Some(first) = numbers.first()
        && *first > 1
    {
        return true;
    }
    numbers.windows(2).any(|pair| pair[1] > pair[0] + 1)
}

/// Record an artifact the family was meant to scan but could not read.
///
/// **At `done` this is a blocking finding, not a skipped target**, and the
/// asymmetry with every other skip reason is deliberate. The general rule —
/// stated in `/{project}:analyze`'s Unexamined targets section and inherited
/// from the `status-unreadable` precedent for cross-service references — is
/// that an unknown is never escalated into a defect. That rule is about
/// *other* files: another spec's frontmatter, an upstream service's state.
/// Nothing about them is this spec's fault, and blocking it for their defect
/// would punish the wrong artifact.
///
/// This reason is the exception because its subject is the spec's **own**
/// artifact, in its own directory, that its own analysis could not read. That
/// is not an unknown about someone else; it is the analysis unable to examine
/// its own subject.
///
/// The concrete case, and the reason this is not merely principled:
/// `scenario-open-questions` is **blocking at `done`** — a spec is not
/// complete while its scenarios carry unresolved questions (spec 046). An
/// unreadable scenario contributes no questions and, as a skip, no finding —
/// so a scenario carrying unresolved questions that happens not to parse
/// passed the gate built to catch exactly that. A check that could not run,
/// wearing the costume of one that passed, *inside* the gate rather than
/// beside it.
///
/// Below `done` it stays a skipped target. There the questions check is
/// advisory anyway, the spec is still in flight, and an unreadable artifact
/// mid-work is a state to report rather than a gate to fail.
fn record_unreadable_artifact(
    findings: &mut Vec<ArtifactFinding>,
    skipped: &mut Vec<SkippedTarget>,
    family: &str,
    status: &str,
    path: String,
) {
    if status == "done" {
        findings.push(ArtifactFinding {
            family: family.into(),
            severity: AnalyzeSeverity::Blocking,
            message: format!(
                "unreadable artifact: {path} could not be read, so this check never examined it — a done spec cannot rest on an analysis that could not read its own subject"
            ),
            path,
        });
    } else {
        skipped.push(SkippedTarget {
            family: family.into(),
            reason: "artifact-unreadable".into(),
            path,
        });
    }
}

/// (d3) Disposition drift — a `done` spec whose `review.md` counts
/// undispositioned findings (spec 058), naming the command that decides them.
/// A map-less record is silent, and `analysis.md` is judged by
/// `/{project}:analyze` from the record it writes: see the module doc.
fn check_disposition_drift(
    findings: &mut Vec<ArtifactFinding>,
    review: Option<&crate::schema::primitives::ReviewBlock>,
    status: &str,
    spec_path: &Path,
    repo: &Path,
) {
    if status != "done" {
        return;
    }
    let Some(counts) = review.and_then(|r| r.dispositions) else {
        return; // predates dispositions: grandfathered, never backfilled
    };
    if counts.undispositioned == 0 {
        return;
    }
    findings.push(ArtifactFinding {
        family: "disposition-drift".into(),
        severity: AnalyzeSeverity::Blocking,
        message: format!(
            "disposition drift: done spec's review.md records {} undispositioned finding(s) — \
             re-run the review command and fix, route, or discard each",
            counts.undispositioned
        ),
        path: rel_path(spec_path, repo),
    });
}

/// (d) Review-state drift — reference §"Review state drift (blocking)":
/// for a `done` spec, `review.last-run` must be set and `review.blocking`
/// must be `false`. Grandfather rule: a `done` spec with **no** `review:`
/// block at all predates `/ductus:review` and is exempt. Specs not at `done`
/// are silently exempt (the block populates lazily on first review).
fn check_review_drift(
    findings: &mut Vec<ArtifactFinding>,
    review: Option<&crate::schema::primitives::ReviewBlock>,
    status: &str,
    spec_path: &Path,
    repo: &Path,
) {
    if status != "done" {
        return;
    }
    let Some(review) = review else {
        return; // grandfathered: no review record at all
    };
    let spec_rel = rel_path(spec_path, repo);
    if review.last_run.is_none() {
        findings.push(ArtifactFinding {
            family: "review-state-drift".into(),
            severity: AnalyzeSeverity::Blocking,
            message: "review drift: done spec missing review (review.last-run unset) — \
                      run the review command"
                .into(),
            path: spec_rel.clone(),
        });
    }
    if review.blocking {
        findings.push(ArtifactFinding {
            family: "review-state-drift".into(),
            severity: AnalyzeSeverity::Blocking,
            message: "review drift: done spec has unresolved MUST violations \
                      (review.blocking true) — see review.md"
                .into(),
            path: spec_rel.clone(),
        });
    }
    // An outstanding SHOULD at `done` is the state §implement-phase names
    // and forbids: "advisory is not ignorable at the gate". A SHOULD is
    // addressed by being **fixed** — which drops it from the count — or by
    // being moved under the review's waived section with its rationale, which
    // also drops it. A non-zero count therefore means neither happened, and
    // the finding is still filed under its original heading.
    //
    // Nothing caught this before, and the reason is worth recording: Family
    // 31 compared the spec's `review:` block against `review.md`, but
    // one `/{project}:review` run wrote both. A fix applied *during* a pass
    // that never re-runs left the two consistently stale and that family
    // clean. 023 sat at `done` with `should-violations: 1` while its own
    // report said "Fixed during this pass", and a hand sweep found it, not
    // the tooling (spec 023 review, 2026-08-30).
    //
    // Blocking, like its two siblings above, because the claim is the same
    // shape: this spec should not be `done` in this state, and `--fix`
    // reverts it rather than editing the count — the count is the review's to
    // write, and rewriting it here would erase the finding instead of
    // resolving it.
    if review.should_violations > 0 {
        findings.push(ArtifactFinding {
            family: "review-state-drift".into(),
            severity: AnalyzeSeverity::Blocking,
            message: format!(
                "review drift: done spec has {} outstanding SHOULD violation(s) — fix each, or \
                 move it under review.md's Waived findings with its rationale, then re-run the \
                 review so the count states what is outstanding",
                review.should_violations
            ),
            path: spec_rel,
        });
    }
}

/// (e) Scenario open questions — a scenario is an organizational split of
/// the spec, so its unresolved questions are the spec's questions for the
/// purpose of completeness. **Blocking at `done`**: that state directly
/// contradicts the completion rule, and `--fix` reverts it the way it
/// reverts review-state drift. **Advisory otherwise**: the questions are
/// real remaining work, but a spec still in flight is allowed to carry
/// them.
///
/// Deliberately **no grandfather rule**, unlike [`check_review_drift`]. An
/// absent `review:` block genuinely marks a spec as predating that
/// feature; an unresolved scenario question is a present-tense defect
/// whenever it arrived, and exempting it would preserve exactly the state
/// this check exists to surface (spec 046).
///
/// Reads the scan `read-spec` already took, through its collector, so this
/// finding, the `check-review-gate` block, and the count surfaced to the user
/// can never disagree.
fn check_scenario_open_questions(
    findings: &mut Vec<ArtifactFinding>,
    skipped: &mut Vec<SkippedTarget>,
    feature_dir: &Path,
    spec: &ReadSpecResult,
    spec_path: &Path,
    repo: &Path,
) {
    let status = spec.frontmatter.status.as_str();
    // A scenario that could not be read is reported as a skipped target, not
    // as a finding and not as silence. It is not a defect — nothing can be
    // proven about a file that will not parse — but a zero-finding result
    // over a subject the family never read would be indistinguishable from a
    // fully-examined clean one (QUAL-CLAIM-001).
    for slug in &spec.scenario_files_unreadable {
        record_unreadable_artifact(
            findings,
            skipped,
            "scenario-open-questions",
            status,
            rel_path(
                &feature_dir.join("scenarios").join(format!("{slug}.md")),
                repo,
            ),
        );
    }
    let questions = &spec.scenario_open_questions;
    if questions.is_empty() {
        return;
    }
    let scenarios = read_spec::scenario_names(questions);
    let severity = if status == "done" {
        AnalyzeSeverity::Blocking
    } else {
        AnalyzeSeverity::Advisory
    };
    findings.push(ArtifactFinding {
        family: "scenario-open-questions".into(),
        severity,
        message: format!(
            "{} unresolved open question(s) in scenario(s) {} — a spec is not complete while its scenarios carry questions",
            questions.len(),
            scenarios.join(", ")
        ),
        path: rel_path(spec_path, repo),
    });
}

// --- (f) link-adjacent decision drift ---------------------------------------

/// The six closed open-state tells (spec 045), each with the class of target
/// state it contradicts. Framework-fixed with no per-project configuration
/// surface: the promotion criterion counts findings across a repo, so a
/// per-project list would make that threshold measure configuration rather
/// than drift. `TBD` and `deferred` were dropped from the seed list — the
/// first asserts nothing about the *target*, the second contradicts the
/// convention that a deferral is a resolution with a condition.
const TELLS: [(&str, TellClass); 6] = [
    ("open question", TellClass::Question),
    ("unresolved", TellClass::Question),
    ("still open", TellClass::Question),
    ("not yet", TellClass::Implementation),
    ("does not exist", TellClass::Implementation),
    ("left unimplemented", TellClass::Implementation),
];

/// The kind of target state a tell makes a claim about.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TellClass {
    /// "this question is open" — contradicted by a zero question count.
    Question,
    /// "this work is unbuilt" — contradicted by a spec at `in-progress`/`done`.
    ///
    /// `does not exist` belongs here rather than to a file-existence test. A
    /// link that resolves always points at a present file, so testing presence
    /// could only ever *fire*, never filter — a test that cannot fail is not a
    /// test. The full-repo run confirmed it: the single finding it produced
    /// across 47 specs was `017/detect-dependency-cycles.md`, whose prose says
    /// an *override mechanism* "does not exist today" while linking to a
    /// scenario that does. Judging the tell against the target's lifecycle
    /// status is the reading in this spec's Behavior section, and the one that
    /// can be wrong.
    Implementation,
}

/// What a resolved link target can be read for. A scenario deliberately has
/// no status: deriving one from its task checkbox was rejected because a
/// spent task pruned per §tasks-phase leaves the same absence as an
/// unimplemented one.
enum TargetState {
    Spec {
        status: String,
        open_questions: usize,
    },
    Scenario {
        open_questions: usize,
    },
    /// A sibling artifact carrying neither a status nor questions
    /// (`plan.md`, `tasks.md`, `data-model.md`): existence only.
    Opaque,
}

/// Outcome of testing one tell class against one target's readable state.
enum Contradiction {
    /// The state contradicts the tell; the string describes it for the message.
    Yes(String),
    /// The state is readable and agrees with the tell.
    No,
    /// The target carries no state this class can be evaluated against.
    Unreadable,
}

/// (f) Link-adjacent decision drift (advisory) — an artifact's own prose
/// asserting an open state that its own sibling link's target contradicts.
///
/// Scans `spec.md`, `plan.md`, `tasks.md`, and `scenarios/*.md`. For each
/// block-level element carrying at least one tell, every sibling link in that
/// block is evaluated independently, so a block with three links fires only
/// for the target whose state actually contradicts.
///
/// An unreadable target is recorded in `skipped`, never escalated into a
/// finding: an unknown is not a defect (spec 045), but a family that emits
/// nothing because it could not look must say so (`QUAL-CLAIM-001`).
fn check_link_adjacent_drift(
    findings: &mut Vec<ArtifactFinding>,
    skipped: &mut Vec<SkippedTarget>,
    feature_dir: &Path,
    spec: &ReadSpecResult,
    repo: &Path,
) {
    let spec_status = spec.frontmatter.status.clone();
    let spec_questions = spec.open_questions.len();
    // Both per-scenario signals are derived once, up front: the counts come
    // from the `read-spec` result already in hand, and readability from one
    // pass over the scenario directory. Testing either per link would re-open
    // the same file once for every citation of it.
    let mut scenario_questions: HashMap<&str, usize> = HashMap::new();
    for question in &spec.scenario_open_questions {
        *scenario_questions
            .entry(question.scenario.as_str())
            .or_insert(0) += 1;
    }
    let scenarios_dir = feature_dir.join("scenarios");
    let readable_scenarios: HashSet<String> = list_scenario_files(&scenarios_dir)
        .iter()
        // The symlink test short-circuits the read: a linked entry is never
        // opened, so its destination is never touched. It lands in the
        // `target-unparseable` outcome downstream, the same as a file that
        // cannot be read.
        .filter(|name| {
            let path = scenarios_dir.join(name);
            !traverses_symlink(&path, feature_dir) && read_text(&path).is_ok()
        })
        .map(|name| Path::new(name).file_stem().unwrap_or_default())
        .filter_map(|stem| stem.to_str().map(str::to_string))
        .collect();

    for artifact in scanned_artifacts(feature_dir) {
        let citing = rel_path(&artifact, repo);
        let (Ok(content), Some(from_dir)) = (read_text(&artifact), artifact.parent()) else {
            // The family could not read an artifact it was meant to scan.
            // Dropping it silently would let a partially-scanned feature
            // report exactly what a fully-scanned clean one reports
            // (`QUAL-CLAIM-001`), so the gap is recorded instead.
            record_unreadable_artifact(
                findings,
                skipped,
                "link-adjacent-drift",
                &spec_status,
                citing.clone(),
            );
            continue;
        };
        for block in split_blocks(&content) {
            let fired = fired_tells(&block.text);
            if fired.is_empty() {
                continue;
            }
            for target in sibling_targets(&block, from_dir, feature_dir) {
                let state = read_target_state(
                    &target,
                    &scenarios_dir,
                    feature_dir,
                    &spec_status,
                    spec_questions,
                    &scenario_questions,
                    &readable_scenarios,
                );
                let target_rel = rel_path(&target, repo);
                match state {
                    Err(reason) => record_skip(skipped, "link-adjacent-drift", reason, &target_rel),
                    Ok(state) => evaluate(
                        findings,
                        skipped,
                        &fired,
                        &state,
                        &block,
                        &citing,
                        &target_rel,
                    ),
                }
            }
        }
    }
}

/// Emit at most one finding for one (block, link) pair, or record the skip.
fn evaluate(
    findings: &mut Vec<ArtifactFinding>,
    skipped: &mut Vec<SkippedTarget>,
    fired: &[usize],
    state: &TargetState,
    block: &MarkdownBlock,
    citing: &str,
    target_rel: &str,
) {
    let mut contradicting: Vec<&str> = Vec::new();
    let mut description: Option<String> = None;
    let mut unreadable = false;
    // `fired` is in TELLS order, so the rendered list is stable across runs.
    for &idx in fired {
        let (tell, class) = TELLS[idx];
        match contradiction(class, state) {
            Contradiction::Yes(desc) => {
                contradicting.push(tell);
                description.get_or_insert(desc);
            }
            Contradiction::No => {}
            Contradiction::Unreadable => unreadable = true,
        }
    }
    if contradicting.is_empty() {
        // Only a tell that could not be evaluated is worth recording — a tell
        // the target simply agrees with is an ordinary clean result.
        if unreadable {
            record_skip(
                skipped,
                "link-adjacent-drift",
                "no-readable-state",
                target_rel,
            );
        }
        return;
    }
    let tells = contradicting
        .iter()
        .map(|t| format!("`{t}`"))
        .collect::<Vec<_>>()
        .join(", ");
    let desc = description.unwrap_or_default();
    findings.push(ArtifactFinding {
        family: "link-adjacent-drift".into(),
        severity: AnalyzeSeverity::Advisory,
        message: format!(
            "line {}: prose asserting {tells} is contradicted by its link target \
             {target_rel}, which {desc}",
            block.line
        ),
        path: citing.to_string(),
    });
}

/// Test one tell class against one target's readable state.
fn contradiction(class: TellClass, state: &TargetState) -> Contradiction {
    match (class, state) {
        (
            TellClass::Question,
            TargetState::Spec { open_questions, .. } | TargetState::Scenario { open_questions },
        ) => {
            if *open_questions == 0 {
                Contradiction::Yes("reports zero open questions".into())
            } else {
                Contradiction::No
            }
        }
        (TellClass::Implementation, TargetState::Spec { status, .. }) => {
            if status == "in-progress" || status == "done" {
                Contradiction::Yes(format!("is `{status}`"))
            } else {
                Contradiction::No
            }
        }
        // An implementation-state tell against a scenario or an opaque
        // artifact: nothing readable to judge it by (spec 045, AC14).
        _ => Contradiction::Unreadable,
    }
}

/// Record a skipped target once. The fact is about the target, so the same
/// target reached twice by one family collapses to one entry.
fn record_skip(skipped: &mut Vec<SkippedTarget>, family: &str, reason: &str, path: &str) {
    if skipped
        .iter()
        .any(|s| s.family == family && s.reason == reason && s.path == path)
    {
        return;
    }
    skipped.push(SkippedTarget {
        family: family.into(),
        reason: reason.into(),
        path: path.into(),
    });
}

/// The artifacts this family scans, in a fixed order (spec 045, AC6).
///
/// `review.md` is deliberately absent: a review record is pinned to its
/// `reviewed-against` sha and describes the state at that commit, so its prose
/// is correct as written and would flag systematically.
fn scanned_artifacts(feature_dir: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = ["spec.md", "plan.md", "tasks.md"]
        .iter()
        .map(|name| feature_dir.join(name))
        .filter(|path| path.is_file())
        .collect();
    let scenarios_dir = feature_dir.join("scenarios");
    out.extend(
        list_scenario_files(&scenarios_dir)
            .iter()
            .map(|name| scenarios_dir.join(name)),
    );
    out
}

/// The distinct sibling-link targets in `block`, in first-appearance order.
fn sibling_targets(block: &MarkdownBlock, from_dir: &Path, feature_dir: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = Vec::new();
    for line in block.text.lines() {
        for href in link_hrefs(line) {
            if let Some(path) = resolve_sibling(&href, from_dir, feature_dir)
                && !out.contains(&path)
            {
                out.push(path);
            }
        }
    }
    out
}

/// Inline-link hrefs in `line` that sit outside every inline-code span.
fn link_hrefs(line: &str) -> Vec<String> {
    let spans = inline_code_spans(line);
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(rel) = line[from..].find("](") {
        let open = from + rel + 2;
        let Some(close_rel) = line[open..].find(')') else {
            break;
        };
        if !spans.iter().any(|span| span.contains(&open)) {
            out.push(line[open..open + close_rel].to_string());
        }
        from = open + close_rel + 1;
    }
    out
}

/// Resolve a link href against the citing file's directory, keeping it only
/// when it lands inside the feature directory.
///
/// Resolution is **lexical**: a target may legitimately not exist, and
/// `canonicalize` both fails on a missing path and makes the answer depend on
/// symlinks — which would break the repeat-run determinism guarantee.
fn resolve_sibling(href: &str, from_dir: &Path, feature_dir: &Path) -> Option<PathBuf> {
    // A fragment on a sibling target is stripped and the file part used; a
    // bare fragment names no file at all.
    let file_part = href.split('#').next()?.trim();
    if file_part.is_empty() {
        return None;
    }
    // A scheme-bearing target (`https:`, `mailto:`) is not a sibling. Testing
    // before the first `/` keeps a path containing a colon from being mistaken
    // for one.
    let head = file_part.split('/').next().unwrap_or(file_part);
    if head.contains(':') {
        return None;
    }
    let mut resolved = from_dir.to_path_buf();
    for component in Path::new(file_part).components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                resolved.pop();
            }
            Component::Normal(part) => resolved.push(part),
            Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    resolved.starts_with(feature_dir).then_some(resolved)
}

/// `true` when any component of `target` at or below `base` is a symbolic
/// link.
///
/// [`resolve_sibling`] resolves lexically and tests containment on the result,
/// which closes the escape a link *target* could attempt — `..` is consumed by
/// `PathBuf::pop`, so `../../../etc/passwd` never passes `starts_with`. What
/// lexical resolution cannot see is a symlink committed **inside** the feature
/// directory (`scenarios/evil.md -> /etc/shadow`): it resolves inside the base
/// and would then be opened.
///
/// Testing for a link rather than canonicalizing is deliberate. Canonicalizing
/// fails on a legitimately-missing target and makes the answer depend on where
/// a link points, which would break the repeat-run determinism AC8 requires.
/// This test depends only on *whether* a component is a link, never on its
/// destination, so a repeat run still yields the same answer.
fn traverses_symlink(target: &Path, base: &Path) -> bool {
    let Ok(rest) = target.strip_prefix(base) else {
        return false;
    };
    let mut probe = base.to_path_buf();
    for component in rest.components() {
        probe.push(component);
        match std::fs::symlink_metadata(&probe) {
            Ok(meta) if meta.file_type().is_symlink() => return true,
            Ok(_) => {}
            // A component that does not exist cannot be a link. The
            // missing-target outcome downstream reports it.
            Err(_) => return false,
        }
    }
    false
}

/// Indices into [`TELLS`] whose tell appears in `text` outside every
/// inline-code span, in `TELLS` order. The code-span exemption is what lets a
/// document *describe* this check without tripping it.
fn fired_tells(text: &str) -> Vec<usize> {
    let mut fired = Vec::new();
    for (idx, (tell, _)) in TELLS.iter().enumerate() {
        if text.lines().any(|line| contains_outside_code(line, tell)) {
            fired.push(idx);
        }
    }
    fired
}

/// `true` when `needle` appears in `line` outside every inline-code span.
/// Matching is ASCII-case-insensitive; `to_ascii_lowercase` preserves byte
/// length, so offsets into the lowered copy still index the original's spans.
fn contains_outside_code(line: &str, needle: &str) -> bool {
    let lowered = line.to_ascii_lowercase();
    if !lowered.contains(needle) {
        return false;
    }
    let spans = inline_code_spans(line);
    let mut from = 0;
    while let Some(rel) = lowered[from..].find(needle) {
        let pos = from + rel;
        if !spans.iter().any(|span| span.contains(&pos)) {
            return true;
        }
        from = pos + 1;
    }
    false
}

/// Read whatever state `target` carries, or the reason it cannot be examined.
fn read_target_state(
    target: &Path,
    scenarios_dir: &Path,
    feature_dir: &Path,
    spec_status: &str,
    spec_questions: usize,
    scenario_questions: &HashMap<&str, usize>,
    readable_scenarios: &HashSet<String>,
) -> std::result::Result<TargetState, &'static str> {
    // Before `is_file`, which follows links: a symlinked sibling is reported
    // as unexaminable rather than read through. See [`traverses_symlink`].
    if traverses_symlink(target, feature_dir) {
        return Err("target-unparseable");
    }
    if !target.is_file() {
        return Err("target-missing");
    }
    if target == feature_dir.join("spec.md") {
        return Ok(TargetState::Spec {
            status: spec_status.to_string(),
            open_questions: spec_questions,
        });
    }
    if target.parent() == Some(scenarios_dir) {
        let slug = target
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or_default();
        // The question collector tolerates absent or malformed frontmatter and
        // still finds the questions section, so only a file that cannot be read
        // at all is opaque — and that was established in the single up-front
        // pass rather than by re-opening the file here.
        if !readable_scenarios.contains(slug) {
            return Err("target-unparseable");
        }
        return Ok(TargetState::Scenario {
            open_questions: scenario_questions.get(slug).copied().unwrap_or(0),
        });
    }
    Ok(TargetState::Opaque)
}

// --- (g) acceptance-criterion path existence --------------------------------

/// (g) Criterion path existence (advisory) — a filesystem path named in a
/// `done` spec's acceptance criterion that no longer resolves.
///
/// Scoped to `## Acceptance Criteria` on `done` specs, and nothing else. An
/// acceptance criterion is a **contract**: naming a path asserts that path is
/// part of the delivered system. Body prose may name a dead path perfectly
/// correctly while describing history — 026's own Behavior section records
/// that spec 043 deleted `framework/workflows/` — so widening this check to
/// whole spec bodies would flag true statements.
///
/// Reads **only inside** inline code spans, the inverse of
/// [`check_link_adjacent_drift`]'s rule. Paths are backticked by convention,
/// which is exactly the context the tell scan must ignore; one family cannot
/// hold both rules coherently, which is why these are two.
///
/// A criterion only counts as a live assertion when it actually claims the
/// path is *present* — see [`NON_ASSERTION_MARKERS`].
fn check_criterion_path_existence(
    findings: &mut Vec<ArtifactFinding>,
    skipped: &mut Vec<SkippedTarget>,
    status: &str,
    spec: &ReadSpecResult,
    spec_path: &Path,
    repo: &Path,
) {
    if status != "done" {
        return;
    }
    let citing = rel_path(spec_path, repo);
    // Read once per feature, not per candidate: the manifest is one file and
    // an empty set (the adopter case) costs a single failed open.
    let ships_elsewhere = adopter_destinations(repo);
    for criterion in &spec.acceptance_criteria {
        let asserts = is_live_assertion(&criterion.text);
        for candidate in candidate_paths(&criterion.text) {
            // A criterion that describes a deletion, a rename, an adopter's
            // checkout, or an example is not claiming its paths are present
            // here, so a path that fails to resolve confirms it or is
            // irrelevant to it — never contradicts it. Recorded rather than
            // silently dropped, the same way `root-absent` is.
            if !asserts {
                record_skip(
                    skipped,
                    "criterion-path-existence",
                    "not-a-live-claim",
                    &candidate,
                );
                continue;
            }
            // A trailing slash marks a directory reference; either kind of
            // entry satisfies the criterion.
            let trimmed = candidate.trim_end_matches('/');
            if repo.join(trimmed).exists() {
                continue;
            }
            // When the candidate's own top-level segment is absent, this repo
            // has nothing to say about the path — a framework repo's criteria
            // legitimately name paths that live in an *adopter's* checkout
            // (`.ductus/…`, `.agents/…`, `.githooks/…`), and calling those
            // drifted would be asserting a defect from an absence of evidence.
            // Recorded rather than exempted, so the report says what went
            // unexamined instead of quietly reading as clean. In an adopter
            // repo the root does exist, so real drift beneath it is still
            // caught — the rule self-corrects where it matters.
            let root = trimmed.split('/').next().unwrap_or(trimmed);
            if !repo.join(root).exists() {
                record_skip(
                    skipped,
                    "criterion-path-existence",
                    "root-absent",
                    &candidate,
                );
                continue;
            }
            // The path resolves in the repo this criterion is *about*, which
            // is not this one: it is a destination this repo declares it
            // ships into an adopter's checkout. `root-absent` above cannot
            // catch these — their top-level segments (`specs`, `.ductus`,
            // `.githooks`) all exist here — so without this arm a framework
            // repo reports a defect for every file it correctly delivers
            // elsewhere. Recorded, never dropped: the report still says the
            // path went unexamined and why.
            if ships_to_adopter(&ships_elsewhere, trimmed) {
                record_skip(
                    skipped,
                    "criterion-path-existence",
                    "ships-to-adopter",
                    &candidate,
                );
                continue;
            }
            findings.push(ArtifactFinding {
                family: "criterion-path-existence".into(),
                severity: AnalyzeSeverity::Advisory,
                message: format!(
                    "acceptance criterion names `{candidate}`, which no longer resolves: \
                     \"{}\"",
                    criterion.text
                ),
                path: citing.clone(),
            });
        }
    }
}

/// Phrases that make a criterion something other than a live claim that its
/// paths are present. Closed and framework-fixed, for the same reason the
/// open-state tell list is: the promotion criterion counts findings across a
/// repo, so a per-project list would make that threshold measure
/// configuration rather than drift.
///
/// This is the tell scan's co-occurrence design inverted. There, a phrase
/// asserting an open state is contradicted by a target that is closed. Here, a
/// phrase asserting *absence* — or scoping the path to somewhere other than
/// this repo — is **confirmed** by a path that does not resolve, so the finding
/// would be exactly backwards. Five groups, each earned against real criteria
/// in this repo's `done` specs:
///
/// - **deletion / retirement** — `framework/commands/capture.md is deleted` is
///   satisfied *because* the path is gone;
/// - **rename** — `framework/rules/configuration.md is renamed to …-cross.md`
///   names the old path deliberately, as does the parenthetical history form
///   `(was `.claude/gov-session.json` pre-0.10.0)`;
/// - **migration subject** — `whose target paths cover …` names paths a
///   migration exists to remove, i.e. manifest data rather than a delivery
///   claim;
/// - **adopter scope** — `writes it to specs/rules/security-backend.md in the
///   project` describes a scaffolded checkout, not this one;
/// - **hedge / example** — `(e.g., docs/rules/internal-api.md)` and
///   `scripts/lint-ductus-toml.sh (if it exists)` claim nothing at all.
///
/// The whole criterion is exempted, not just the matched path: these phrases
/// describe a *transition*, and a criterion about a transition names its
/// endpoints together. Erring toward silence matches how the rest of this
/// family already errs (code-spans only, `root-absent`). The groups are five,
/// not four, since the migration-subject group was added.
///
/// A sixth group exists and is deliberately **not** in this list: a criterion
/// asserting a path was *never created*. Its negator and its verb are split by
/// the noun between them, so no fixed phrase spans the construction, and the
/// phrase that would (`was created`) exempts positive delivery claims too.
/// [`asserts_negated_creation`] carries it as a clause-scoped predicate
/// instead — same inversion, one tense earlier.
const NON_ASSERTION_MARKERS: [&str; 14] = [
    // `deleted` subsumes the former `is deleted` / `are deleted` pair. The
    // narrower forms missed the past-tense-agent phrasing a criterion reaches
    // for when it names the commit that did the deleting — 045's own AC18,
    // `… after `531e3ea` deleted both`, is the case that earned the widening.
    // A criterion carrying the word at all is describing a removal, which is
    // the group's whole premise.
    "deleted",
    "does not exist",
    "no longer exists",
    "is removed",
    "are removed",
    "since retired",
    "is renamed to",
    "are renamed to",
    "renamed from",
    // The parenthetical-history form of a rename: `… for session state (was
    // `.claude/gov-session.json` pre-0.10.0)`. The old path is named to date
    // the change, never to claim it is still there. The opening paren keeps
    // this from matching an ordinary past-tense `was`.
    "(was ",
    // A path named as the *subject of a migration record* — `whose target
    // paths cover … `framework/workflows/`` — is data inside a manifest
    // describing what to remove, not a claim that the path is delivered.
    "target paths",
    "in the project",
    "if it exists",
    "e.g.",
];

/// Repo-relative destinations this repo declares it scaffolds into an
/// adopter's checkout, derived from `framework/bootstrap/ductus.md` — the
/// canonical registry of what lands where, per the constitution's
/// canonical-sources map. Two arms, read independently: the **Shared Files**
/// manifest tables ([`manifest_destinations`]) and each registered agent's
/// scaffold paths ([`agent_scaffold_destinations`]).
///
/// Empty when that file is absent, which is the discriminator: an adopter
/// checkout has no `framework/bootstrap/` (it receives the installed command,
/// not the framework source), so the suppression below simply never engages
/// there. That is the correct shape rather than a limitation — in an adopter
/// these destinations *do* resolve, so they produce no finding to suppress.
///
/// Derivation failure yields an empty set, which fails toward **reporting**:
/// a broken parse means findings are emitted, never silently swallowed.
/// Family 18 of `/{project}:audit` guards the inverse direction for the marker
/// list; here the safe default is built into the return value.
pub(crate) fn adopter_destinations(repo: &Path) -> BTreeSet<String> {
    let Ok(text) = std::fs::read_to_string(repo.join("framework/bootstrap/ductus.md")) else {
        return BTreeSet::new();
    };
    let mut out = manifest_destinations(&text);
    out.extend(agent_scaffold_destinations(&text));
    out
}

/// The **Shared Files** arm of [`adopter_destinations`]: the destination
/// column of every table row whose cell is exactly one backticked span.
fn manifest_destinations(text: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for line in text.lines() {
        if !line.starts_with('|') {
            continue;
        }
        let cols: Vec<&str> = line.trim().trim_matches('|').split('|').collect();
        if cols.len() < 2 {
            continue;
        }
        let cell = cols[1].trim();
        // Exactly one backticked span, nothing else — skips header rows
        // (`Destination Path`), separator rows, and prose cells.
        let Some(inner) = cell.strip_prefix('`').and_then(|c| c.strip_suffix('`')) else {
            continue;
        };
        if !inner.is_empty() && !inner.contains('`') {
            out.insert(inner.to_string());
        }
    }
    out
}

/// The §Derived values rows whose per-layout formula names a file `/ductus`
/// scaffolds into an agent's config directory, compared with the row label's
/// backticks dropped and case folded.
const SCAFFOLD_ROWS: [&str; 2] = ["ductus install path", "settings file"];

/// §MCP registration scopes whose target lands inside the adopter's checkout.
/// A `user-global` or `home-level` target lives outside any checkout, so no
/// criterion naming one is a repo-relative path claim.
const PROJECT_SCOPES: [&str; 2] = ["project-committed", "project-local"];

/// The **registry** arm of [`adopter_destinations`] (spec 022 scenario
/// `per-agent-scaffold-paths-ship-to-adopter`). A per-agent scaffold path is
/// not a Shared Files row: it is the agent's `config_dir` substituted into a
/// layout-derived formula, which the manifest carries only as an inert
/// placeholder. So for every `## Agent Registry` row this substitutes the
/// row's `config_dir` into the §Derived values `ductus` install path and
/// Settings file formulas for its `layout`, and takes its §MCP registration
/// target when that target is project-scoped.
///
/// Every registered agent contributes, not only the ones dogfooded here — the
/// set describes what this repo ships, not what it happens to materialize. A
/// cell is read for its first backticked span, so a trailing note beside the
/// path (`opencode.json` (repo root; …)) does not drop the row. A result still
/// carrying a placeholder, or a home-relative one, is left out. Each table is
/// read on its own: one that is missing contributes nothing, which fails
/// toward reporting and never empties the Shared Files arm.
fn agent_scaffold_destinations(text: &str) -> BTreeSet<String> {
    let mut formulas: HashMap<&str, Vec<&str>> = HashMap::new();
    for table in tables_under(text, "Derived values") {
        let Some((header, rows)) = table.split_first() else {
            continue;
        };
        for row in rows {
            let label = row.first().map(|cell| cell.replace('`', "").to_lowercase());
            if !label.is_some_and(|l| SCAFFOLD_ROWS.contains(&l.trim())) {
                continue;
            }
            for (layout, cell) in header.iter().zip(row).skip(1) {
                if let (Some(layout), Some(formula)) = (first_span(layout), first_span(cell)) {
                    formulas.entry(layout).or_default().push(formula);
                }
            }
        }
    }

    let mut mcp_targets: HashMap<&str, &str> = HashMap::new();
    for table in tables_under(text, "MCP registration (per-agent)") {
        let Some((header, rows)) = table.split_first() else {
            continue;
        };
        let (Some(k), Some(t), Some(s)) = (
            column(header, "key"),
            column(header, "mcp target"),
            column(header, "scope"),
        ) else {
            continue;
        };
        for row in rows {
            let span = |i: usize| row.get(i).and_then(|cell| first_span(cell));
            if let (Some(key), Some(target), Some(scope)) = (span(k), span(t), span(s))
                && PROJECT_SCOPES.contains(&scope)
            {
                mcp_targets.insert(key, target);
            }
        }
    }

    let mut out = BTreeSet::new();
    for table in tables_under(text, "Agent Registry") {
        let Some((header, rows)) = table.split_first() else {
            continue;
        };
        let (Some(k), Some(c), Some(l)) = (
            column(header, "key"),
            column(header, "config_dir"),
            column(header, "layout"),
        ) else {
            continue;
        };
        for row in rows {
            let span = |i: usize| row.get(i).and_then(|cell| first_span(cell));
            let (Some(key), Some(config_dir), Some(layout)) = (span(k), span(c), span(l)) else {
                continue;
            };
            let derived = formulas.get(layout).into_iter().flatten().copied();
            for formula in derived.chain(mcp_targets.get(key).copied()) {
                let path = formula.replace("{config_dir}", config_dir);
                if !path.contains(['{', '}', '<', '>']) && !path.starts_with('~') {
                    out.insert(path);
                }
            }
        }
    }
    out
}

/// The tables in the section `heading` names, each as rows of cells, header
/// first, separator rows dropped. The section is located by the shared
/// [`section_line_indices`], so it spans its subsections and a table quoted
/// inside a fenced block or an HTML comment is never read as the registry.
/// A table ends at the first line that is not one of its rows. A heading that
/// does not occur yields none.
fn tables_under<'a>(text: &'a str, heading: &str) -> Vec<Vec<Vec<&'a str>>> {
    let lines: Vec<&'a str> = text.lines().collect();
    let mut tables: Vec<Vec<Vec<&'a str>>> = Vec::new();
    let mut last_row: Option<usize> = None;
    for idx in section_line_indices(&lines, heading) {
        let line = lines[idx].trim();
        if !line.starts_with('|') {
            continue;
        }
        let continues = last_row.is_some_and(|prev| prev + 1 == idx);
        last_row = Some(idx);
        let cells: Vec<&str> = line.trim_matches('|').split('|').collect();
        if cells
            .iter()
            .all(|cell| cell.trim().chars().all(|ch| matches!(ch, '-' | ':')))
        {
            continue;
        }
        if !continues {
            tables.push(Vec::new());
        }
        if let Some(table) = tables.last_mut() {
            table.push(cells);
        }
    }
    tables
}

/// The index of the header cell naming `name`, backticks dropped, case folded.
fn column(header: &[&str], name: &str) -> Option<usize> {
    header
        .iter()
        .position(|cell| cell.replace('`', "").trim().eq_ignore_ascii_case(name))
}

/// The content of the first inline-code span in `cell`, when it has one.
fn first_span(cell: &str) -> Option<&str> {
    let range = inline_code_spans(cell).into_iter().next()?;
    Some(cell[range].trim()).filter(|span| !span.is_empty())
}

/// `true` when `candidate` is one of the adopter destinations, or is a
/// directory containing one. The directory case is what lets a criterion name
/// `specs/templates/` and match the six per-file template rows beneath it.
///
/// The candidate is trailing-slash-normalized before the directory test. A
/// candidate already written with the slash — which is how a criterion or an
/// `AGENTS.md` line usually spells a directory, and exactly the spelling this
/// doc comment's own example uses — otherwise built the prefix
/// `specs/templates//` and matched nothing, so a reference to a shipped
/// directory escaped the exclusion and was reported as local breakage.
/// `is_path_like` trims the same way one file over; this is the second half of
/// that normalization.
pub(crate) fn ships_to_adopter(destinations: &BTreeSet<String>, candidate: &str) -> bool {
    let candidate = candidate.trim_end_matches('/');
    if destinations.contains(candidate) {
        return true;
    }
    let prefix = format!("{candidate}/");
    destinations.iter().any(|d| d.starts_with(&prefix))
}

/// The clause-scoped vocabulary behind [`asserts_negated_creation`]. Both
/// lists are closed, matched as whole words, and framework-fixed for the same
/// reason [`NON_ASSERTION_MARKERS`] is: a per-project vocabulary would make
/// the family's promotion threshold measure configuration rather than drift.
const CREATION_NEGATORS: [&str; 4] = ["no", "not", "never", "without"];
/// Companion of [`CREATION_NEGATORS`]; see [`asserts_negated_creation`].
const CREATION_VERBS: [&str; 3] = ["created", "added", "introduced"];

/// `true` when the criterion asserts a path was **never brought into
/// existence** — the deletion group's inversion, one tense earlier. Such a
/// criterion is *satisfied* by its path failing to resolve, exactly as
/// `X is deleted` is, so flagging it is equally backwards.
///
/// This is a predicate rather than a sixth entry in [`NON_ASSERTION_MARKERS`],
/// and the reason is the construction rather than the vocabulary. The negator
/// is separated from the verb by the noun it negates — ``no nested
/// `server/.git/` repository was created`` — so no fixed phrase spans them,
/// while the phrase that *would* match every such criterion (`was created`)
/// exempts positive delivery claims too and blinds the family. That is
/// strictly worse than the false positive it fixes.
///
/// Three rules keep the predicate from collapsing into that phrase:
///
/// - **Clause-scoped.** A bare `no` elsewhere in the criterion must not exempt
///   a creation claim in another clause — ``…`scripts/gen-spec-deps.sh` was
///   added by this spec; no further generators are needed`` is a live claim
///   about a path that must still resolve.
/// - **Word-matched.** `not` hides as a substring inside `cannot` and `note`,
///   `no` inside `nano`; the negator has to be the whole word.
/// - **Ordered.** The negator must precede the verb. English negates a verb
///   from in front of it (`no X was created`, `was never added`, `without a
///   lock file being created`), so a negator *after* the verb belongs to
///   something else: ``was created and not modified since`` is a delivery
///   claim carrying a trailing qualifier.
fn asserts_negated_creation(lowered: &str) -> bool {
    clauses(lowered).any(|clause| {
        let mut negated = false;
        for word in clause.split(|c: char| !c.is_ascii_alphanumeric()) {
            if CREATION_NEGATORS.contains(&word) {
                negated = true;
            } else if negated && CREATION_VERBS.contains(&word) {
                return true;
            }
        }
        false
    })
}

/// A criterion's clauses: split on `;`, `,`, and a sentence-ending `. `.
///
/// The period test is **a period followed by a space**, never the bare
/// character, and that is what makes clause splitting usable in a family whose
/// whole subject is paths. A bare `.` splitter cuts `server/.git/` and
/// `master.key` in half, stranding a negator in one fragment and its verb in
/// the next — which is exactly how the first draft of this predicate failed to
/// exempt the criterion it was written for. A criterion-final period needs no
/// split, and `e.g.` never reaches here: it is a marker, so the criterion is
/// already exempt.
///
/// Over-splitting errs toward *flagging* — a negated-creation clause
/// interrupted by a comma (`no repository, nor its index, was created`) stays
/// a finding. That is the safe direction for a predicate whose failure mode is
/// blinding the family, and it is the one place this family deliberately does
/// not err toward silence.
fn clauses(text: &str) -> impl Iterator<Item = &str> {
    text.split([';', ',']).flat_map(|part| part.split(". "))
}

/// `true` when the criterion claims its paths are present — i.e. it carries
/// none of [`NON_ASSERTION_MARKERS`] and does not assert that a path was never
/// created. Marker matching is ASCII-case-insensitive over the whole
/// criterion, code spans included: a marker is prose, and a criterion that
/// carries one anywhere is describing a transition throughout.
/// [`asserts_negated_creation`] is the one clause-scoped half, for the reason
/// its own docs give.
fn is_live_assertion(text: &str) -> bool {
    let lowered = text.to_ascii_lowercase();
    !NON_ASSERTION_MARKERS
        .iter()
        .any(|marker| lowered.contains(marker))
        && !asserts_negated_creation(&lowered)
}

/// Candidate filesystem paths named inside `text`'s inline code spans, in
/// first-appearance order.
fn candidate_paths(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for line in text.lines() {
        for span in inline_code_spans(line) {
            let content = normalize_candidate(line[span].trim());
            if is_path_like(content) && !out.iter().any(|seen| seen == content) {
                out.push(content.to_string());
            }
        }
    }
    out
}

/// Strip the quoting a criterion may wrap a path in, and a leading `./` that
/// names the same file as the bare form.
fn normalize_candidate(content: &str) -> &str {
    let unquoted = content
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
        .or_else(|| {
            content
                .strip_prefix('\'')
                .and_then(|rest| rest.strip_suffix('\''))
        })
        .unwrap_or(content);
    unquoted.strip_prefix("./").unwrap_or(unquoted)
}

/// The acceptance-criterion path grammar (spec 045 data-model).
///
/// Every exclusion earns its place against real criteria text in this repo:
/// `:` rejects URLs, `path:line` citations, and every slash-command
/// reference; the braces reject placeholders; the bracket and star forms
/// reject globs; a leading `-` rejects flags; a leading `/` rejects an
/// absolute path, since the check makes claims about this repository only.
///
/// The separator must be **internal**: a token whose only `/` is its last
/// character is a bare directory *name* used conceptually — "the feature's
/// `scenarios/` directory" — not a path to resolve against the repo root.
/// Two further rejections, both for tokens that are *written* as paths but
/// name no file: a non-ASCII character, which in practice is the `…` of an
/// elided path (`scripts/…`); and the framework's own spec-number placeholder
/// `NNN` (`specs/NNN-feature/review.md`), the unbraced sibling of the `{…}`
/// forms already excluded.
fn is_path_like(content: &str) -> bool {
    const REJECTED: [char; 11] = ['{', '}', '*', '?', '[', ']', '<', '>', '$', '|', ':'];
    content.trim_end_matches('/').contains('/')
        && !content.starts_with('-')
        && !content.starts_with('/')
        && !content.contains("NNN")
        && content.is_ascii()
        && !content.chars().any(char::is_whitespace)
        && !content.chars().any(|c| REJECTED.contains(&c))
}

// --- (h) acceptance-criterion labels ----------------------------------------

/// (h) Criterion labels (advisory) — the enforcement half of the `AC{n}`
/// labelling pass (spec 013). Assignment belongs to `label-criteria`;
/// enforcement has to live here because a criterion typed by hand in an
/// editor never touches a primitive. Three invariants, each checkable from
/// the artifact alone with no git history read:
///
/// - **A duplicate `AC{n}` within one spec.** Ambiguous, so it is a defect
///   the audit reports rather than a state a tool resolves by picking the
///   first match.
/// - **A counter that no longer exceeds the body.** `next-criterion` is
///   what makes a retired label unreissuable, so one that has fallen to or
///   below the highest label present means the next assignment hands a
///   *live* label to a second requirement. A value that is not a positive
///   integer is reported the same way and never repaired: a corrupted
///   counter may mean a label was already reissued, and repairing it in
///   place would hide that.
/// - **An unlabelled criterion in a spec that has been labelled.** The gate
///   is the counter's presence, not a grandfather date — 013 defines an
///   absent `next-criterion` as "no labels assigned yet" rather than a
///   defect, and rejects per-spec exemption state outright. The corpus
///   backfill is what makes the check universal: once every spec carries a
///   counter, an unlabelled criterion means a hand edit the pre-commit hook
///   never saw.
///
/// Runs at every status. A label is an identifier rather than a contract
/// about the delivered system, so — unlike [`check_criterion_path_existence`]
/// — it is as wrong to duplicate one in a `draft` as in a `done` spec.
///
/// Contributes nothing to [`SkippedTarget`]: its entire subject is the
/// spec's own frontmatter and criteria list, both already parsed and in
/// hand, so there is no target it can fail to examine.
fn check_criterion_labels(
    findings: &mut Vec<ArtifactFinding>,
    spec: &ReadSpecResult,
    spec_path: &Path,
    repo: &Path,
) {
    let citing = rel_path(spec_path, repo);
    let labels: Vec<Option<u32>> = spec
        .acceptance_criteria
        .iter()
        .map(|criterion| criterion.label.as_deref().and_then(label_number))
        .collect();

    // Reported at the second occurrence, so the order is body order and a
    // label repeated three times still yields one finding.
    let mut seen: HashSet<u32> = HashSet::new();
    let mut reported: HashSet<u32> = HashSet::new();
    for label in labels.iter().flatten() {
        if !seen.insert(*label) && reported.insert(*label) {
            findings.push(ArtifactFinding {
                family: "criterion-labels".into(),
                severity: AnalyzeSeverity::Advisory,
                message: format!(
                    "duplicate acceptance-criterion label AC{label} — a label addresses \
                     exactly one criterion, so an ambiguous one cannot be resolved"
                ),
                path: citing.clone(),
            });
        }
    }

    // The counter is read through the labelling pass's own parser, so the
    // pass and the audit can never disagree about what the field says.
    let counter = match read_text(spec_path) {
        Ok(content) => label_criteria::stored_counter(&content),
        // Unreachable in practice — `read-spec` already read this file to
        // produce `spec` — and a re-read that fails says nothing about the
        // labels, so it yields no finding rather than a speculative one.
        Err(_) => return,
    };
    let body_max = labels.iter().flatten().copied().max();
    match &counter {
        StoredCounter::Malformed(value) => findings.push(ArtifactFinding {
            family: "criterion-labels".into(),
            severity: AnalyzeSeverity::Advisory,
            message: format!(
                "next-criterion is `{value}`, which is not a positive integer — the \
                 labelling pass refuses a spec with a corrupted counter rather than \
                 repairing it, since the corruption may mean a label was already reissued"
            ),
            path: citing.clone(),
        }),
        StoredCounter::Valid(next) => {
            if let Some(max_label) = body_max
                && *next <= max_label
            {
                findings.push(ArtifactFinding {
                    family: "criterion-labels".into(),
                    severity: AnalyzeSeverity::Advisory,
                    message: format!(
                        "next-criterion {next} is at or below AC{max_label}, the highest \
                         label in the body — the next assignment would reissue a label \
                         a live criterion already carries"
                    ),
                    path: citing.clone(),
                });
            }
        }
        // Never labelled: 013's edge case, and not a defect.
        StoredCounter::Absent => {}
    }

    if matches!(counter, StoredCounter::Absent) {
        return;
    }
    for (index, criterion) in spec.acceptance_criteria.iter().enumerate() {
        if criterion.label.is_none() {
            findings.push(ArtifactFinding {
                family: "criterion-labels".into(),
                severity: AnalyzeSeverity::Advisory,
                message: format!(
                    "acceptance criterion {index} carries no AC label in a spec that has \
                     been labelled — run the labelling pass: \"{}\"",
                    criterion.text
                ),
                path: citing.clone(),
            });
        }
    }
}

/// The numeric part of an `AC{n}` label as `read-spec` reports it. `read-spec`
/// builds the string from the shared parser's integer, so the round-trip is
/// total; the fallible signature keeps this from being an assumption the
/// audit asserts.
fn label_number(label: &str) -> Option<u32> {
    label.strip_prefix("AC")?.parse().ok()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use tempfile::tempdir;

    const FEATURE: &str = "042-demo";

    fn args() -> CheckArtifactsArgs {
        CheckArtifactsArgs {
            feature: FEATURE.into(),
        }
    }

    fn write(repo: &Path, rel: &str, body: &str) {
        let path = repo.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, body).unwrap();
    }

    /// The spec body alone. The `review` argument survives for the callers
    /// that describe a review state, but the block it names no longer lands in
    /// this file — see [`seed_review`], which the same callers use to put it
    /// where the record now lives (spec 057).
    fn spec(status: &str) -> String {
        format!("---\nstatus: {status}\ndependencies: []\n---\n\n# Demo\n")
    }

    /// Write a review record to the artifact that owns it.
    fn seed_review(repo: &Path, block: &str) {
        let mut body = String::new();
        for line in block.lines() {
            body.push_str(line.strip_prefix("  ").unwrap_or(line));
            body.push('\n');
        }
        write(
            repo,
            &format!("specs/{FEATURE}/review.md"),
            &format!("---\nspec: {FEATURE}\n{body}---\n\n# Review — {FEATURE}\n"),
        );
    }

    /// Write the spec plus whichever records the case carries, each into the
    /// artifact that owns it (spec 057).
    ///
    /// `None` means the record is absent, which is the grandfather case these
    /// families exempt — a `done` spec predating the command. That was "no
    /// block in the frontmatter" and is now "no artifact"; the state the tests
    /// describe is unchanged.
    fn seed_feature(repo: &Path, status: &str, review: Option<&str>, analyze: Option<&str>) {
        write(
            repo,
            &format!("specs/{FEATURE}/spec.md"),
            &format!("---\nstatus: {status}\ndependencies: []\n---\n\n# Demo\n"),
        );
        for (block, file, heading) in [
            (review, "review.md", "Review"),
            (analyze, "analysis.md", "Analysis"),
        ] {
            let Some(block) = block else { continue };
            let mut body = String::new();
            for line in block.lines() {
                body.push_str(line.strip_prefix("  ").unwrap_or(line));
                body.push('\n');
            }
            write(
                repo,
                &format!("specs/{FEATURE}/{file}"),
                &format!("---\nspec: {FEATURE}\n{body}---\n\n# {heading} — {FEATURE}\n"),
            );
        }
    }

    /// `analysis.md` is the record the calling `/analyze` run is about to
    /// replace, so no family judges its state: a blocking record, or one whose
    /// `last-run` is unset, produces nothing here. `/analyze` judges both from
    /// the record it writes (scenario
    /// `analyze-state-drift-judges-the-record-it-writes`); read from the old
    /// record, the finding blocked every re-run, even one that fixed its cause.
    #[test]
    fn the_analysis_record_s_own_state_is_never_judged_here() {
        for record in [
            "  last-run: null\n  blocking: false",
            "  last-run: 2026-07-10T00:00:00Z\n  hard-fail: 1\n  blocking-findings: 2\n  blocking: true",
        ] {
            let tmp = tempdir().unwrap();
            seed_feature(tmp.path(), "done", Some(CLEAN_REVIEW), Some(record));
            write(
                tmp.path(),
                &format!("specs/{FEATURE}/plan.md"),
                "# Demo Plan\n",
            );
            write(tmp.path(), &format!("specs/{FEATURE}/tasks.md"), GOOD_TASKS);
            let result = run(&args(), tmp.path()).unwrap();
            assert!(result.findings.is_empty(), "{:?}", families(&result));
        }
    }

    const CLEAN_ANALYZE: &str = "  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 2\n  unexamined: 1\n  blocking: false";

    const OWED: &str =
        "  dispositions:\n    fixed: 0\n    routed: 0\n    discarded: 0\n    undispositioned: 2";

    fn disposition_families(
        result: &crate::schema::primitives::CheckArtifactsResult,
    ) -> Vec<String> {
        result
            .findings
            .iter()
            .filter(|f| f.family == "disposition-drift")
            .map(|f| f.message.clone())
            .collect()
    }

    fn seed_done_with(repo: &Path, status: &str, review: &str, analyze: &str) {
        seed_feature(repo, status, Some(review), Some(analyze));
        write(repo, &format!("specs/{FEATURE}/plan.md"), "# Demo Plan\n");
        write(repo, &format!("specs/{FEATURE}/tasks.md"), GOOD_TASKS);
    }

    #[test]
    fn a_done_spec_whose_review_owes_decisions_is_disposition_drift() {
        let tmp = tempdir().unwrap();
        seed_done_with(
            tmp.path(),
            "done",
            &format!("{CLEAN_REVIEW}\n{OWED}"),
            CLEAN_ANALYZE,
        );
        let result = run(&args(), tmp.path()).unwrap();
        let drift = disposition_families(&result);
        assert_eq!(drift.len(), 1, "{drift:?}");
        assert!(
            drift[0].contains("review.md records 2 undispositioned"),
            "{drift:?}"
        );
        assert!(drift[0].contains("re-run the review command"), "{drift:?}");
        assert!(result.findings.iter().any(|f| f.family == "disposition-drift"
            && f.severity == AnalyzeSeverity::Blocking));
    }

    /// `analysis.md` is the record the calling `/analyze` run is about to
    /// replace, so its drift is judged from the record that run writes, not
    /// here: a finding read from the old record could never clear (scenario
    /// `analysis-drift-judges-the-record-it-writes`).
    #[test]
    fn the_analysis_record_is_not_judged_by_this_family() {
        let tmp = tempdir().unwrap();
        seed_done_with(
            tmp.path(),
            "done",
            CLEAN_REVIEW,
            &format!("{CLEAN_ANALYZE}\n{OWED}"),
        );
        assert!(disposition_families(&run(&args(), tmp.path()).unwrap()).is_empty());
    }

    #[test]
    fn disposition_drift_is_silent_below_done_and_for_a_map_less_record() {
        let tmp = tempdir().unwrap();
        seed_done_with(
            tmp.path(),
            "in-progress",
            &format!("{CLEAN_REVIEW}\n{OWED}"),
            &format!("{CLEAN_ANALYZE}\n{OWED}"),
        );
        assert!(disposition_families(&run(&args(), tmp.path()).unwrap()).is_empty());

        let tmp = tempdir().unwrap();
        seed_done_with(tmp.path(), "done", CLEAN_REVIEW, CLEAN_ANALYZE);
        assert!(
            disposition_families(&run(&args(), tmp.path()).unwrap()).is_empty(),
            "a record predating the map is grandfathered, never backfilled"
        );
    }

    #[test]
    fn disposition_drift_follows_review_state_drift() {
        let tmp = tempdir().unwrap();
        seed_done_with(
            tmp.path(),
            "done",
            &format!(
                "{}\n{OWED}",
                CLEAN_REVIEW.replace("blocking: false", "blocking: true")
            ),
            CLEAN_ANALYZE,
        );
        let result = run(&args(), tmp.path()).unwrap();
        let order: Vec<&str> = result.findings.iter().map(|f| f.family.as_str()).collect();
        let review_at = order
            .iter()
            .position(|f| *f == "review-state-drift")
            .unwrap();
        let disposition_at = order
            .iter()
            .position(|f| *f == "disposition-drift")
            .unwrap();
        assert!(review_at < disposition_at, "{order:?}");
    }

    const GOOD_TASKS: &str = "# Demo Tasks\n\n\
        ## 1. Implement retry\n\n\
        - [x] Implement the behavior described in `scenarios/retry-on-timeout.md`\n\n\
        - **Done when**: retries pass.\n\n\
        ## 2. Wire CLI\n\n\
        - [ ] sub\n\n\
        - **Done when**: CLI works.\n";

    fn families(result: &CheckArtifactsResult) -> Vec<(&str, &str)> {
        result
            .findings
            .iter()
            .map(|f| (f.family.as_str(), f.severity.as_str()))
            .collect()
    }

    // --- artifact completeness -------------------------------------------------

    #[test]
    fn draft_spec_with_no_plan_or_tasks_is_clean() {
        // Edge case from the scenario: files are required by status tier,
        // not universally.
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("draft"));
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.clean, "{:?}", result.findings);
        assert_eq!(result.status, "draft");
        assert_eq!(result.path, "specs/042-demo/spec.md");
    }

    #[test]
    fn planned_spec_missing_plan_and_tasks_yields_blocking_findings() {
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("planned"));
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(
            families(&result),
            vec![
                ("artifact-completeness", "blocking"),
                ("artifact-completeness", "blocking")
            ]
        );
        assert!(result.findings[0].message.contains("plan.md"));
        assert_eq!(result.findings[0].path, "specs/042-demo/plan.md");
        assert!(result.findings[1].message.contains("tasks.md"));
    }

    #[test]
    fn data_model_is_never_required() {
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("planned"));
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        write(tmp.path(), "specs/042-demo/tasks.md", GOOD_TASKS);
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            result.clean,
            "no data-model.md finding expected: {:?}",
            result.findings
        );
    }

    // --- task consistency --------------------------------------------------------

    #[test]
    fn strictly_increasing_numbered_tasks_with_done_when_are_clean() {
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("in-progress"));
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        write(tmp.path(), "specs/042-demo/tasks.md", GOOD_TASKS);
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.clean, "{:?}", result.findings);
    }

    #[test]
    fn non_increasing_numbering_yields_blocking_finding() {
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("in-progress"));
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        let tasks = "# T\n\n\
            ## 2. Second\n\n- [ ] a\n\n- **Done when**: done.\n\n\
            ## 1. Out of order\n\n- [ ] b\n\n- **Done when**: done.\n";
        write(tmp.path(), "specs/042-demo/tasks.md", tasks);
        let result = run(&args(), tmp.path()).unwrap();
        let numbering: Vec<&ArtifactFinding> = result
            .findings
            .iter()
            .filter(|f| f.message.contains("strictly increasing"))
            .collect();
        assert_eq!(numbering.len(), 1);
        assert_eq!(numbering[0].family, "task-consistency");
        assert_eq!(numbering[0].severity, AnalyzeSeverity::Blocking);
        assert_eq!(numbering[0].path, "specs/042-demo/tasks.md");
        assert!(numbering[0].message.contains("task 1 follows task 2"));
    }

    #[test]
    fn missing_done_when_yields_blocking_finding() {
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("in-progress"));
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        let tasks = "# T\n\n## 1. No done-when\n\n- [ ] a\n";
        write(tmp.path(), "specs/042-demo/tasks.md", tasks);
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(families(&result), vec![("task-consistency", "blocking")]);
        assert!(
            result.findings[0]
                .message
                .contains("task 1 (No done-when) has no Done when clause")
        );
    }

    #[test]
    fn task_checks_skip_when_tasks_file_absent() {
        // "blocking if tasks exist" — a clarified spec with no tasks.md
        // gets no task-consistency findings (and no completeness ones
        // either, below the planned tier).
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("clarified"));
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.clean, "{:?}", result.findings);
    }

    // --- scenario consistency ------------------------------------------------------

    #[test]
    fn unmapped_scenario_yields_advisory_finding() {
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("in-progress"));
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        write(tmp.path(), "specs/042-demo/tasks.md", GOOD_TASKS);
        write(
            tmp.path(),
            "specs/042-demo/scenarios/unmapped-scenario.md",
            "---\nsection: \"X\"\n---\n\n# Unmapped\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(
            families(&result),
            vec![("scenario-consistency", "advisory")]
        );
        assert_eq!(
            result.findings[0].path,
            "specs/042-demo/scenarios/unmapped-scenario.md"
        );
    }

    #[test]
    fn mapped_scenario_produces_no_finding() {
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("in-progress"));
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        write(tmp.path(), "specs/042-demo/tasks.md", GOOD_TASKS);
        write(
            tmp.path(),
            "specs/042-demo/scenarios/retry-on-timeout.md",
            "---\nsection: \"X\"\n---\n\n# Retry\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.clean, "{:?}", result.findings);
    }

    #[test]
    fn bare_slug_reference_satisfies_the_mapping() {
        // The referencing-task rule is a SLUG match across heading,
        // subtask text, and `Done when` — not a `scenarios/{slug}.md`
        // path match. `mapped_scenario_produces_no_finding` covers the
        // path form that `append-task`'s default body emits; this covers
        // the hand-written form the rule deliberately tolerates. Both
        // exist because a second surface applying the narrower path rule
        // disagrees with this family asymmetrically: /ductus:amend's
        // reconcile pass would offer a task for a scenario already
        // mapped here (specs/022-deterministic-runtime/data-model.md,
        // registered canonical in constitution §drift-prevention).
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("in-progress"));
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        let hand_written = "# Demo Tasks\n\n\
            ## 1. Implement scenario: retry-on-timeout\n\n\
            - [ ] wire the backoff\n\n\
            - **Done when**: retries pass.\n\n\
            ## 2. Wire CLI\n\n\
            - [ ] sub\n\n\
            - **Done when**: CLI works.\n";
        write(tmp.path(), "specs/042-demo/tasks.md", hand_written);
        write(
            tmp.path(),
            "specs/042-demo/scenarios/retry-on-timeout.md",
            "---\nsection: \"X\"\n---\n\n# Retry\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            !families(&result)
                .iter()
                .any(|(f, _)| *f == "scenario-consistency"),
            "a task naming the scenario by bare slug is a reference: {:?}",
            result.findings
        );
    }

    #[test]
    fn pruned_gap_numbering_satisfies_the_mapping() {
        // Scenario edge case: a scenario whose task was pruned after
        // completion produces no finding. keep-pending pruning leaves
        // non-contiguous numbers (task 1 was dropped; 2 survives).
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("in-progress"));
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        let pruned = "# T\n\n## 2. Wire CLI\n\n- [ ] sub\n\n- **Done when**: CLI works.\n";
        write(tmp.path(), "specs/042-demo/tasks.md", pruned);
        write(
            tmp.path(),
            "specs/042-demo/scenarios/pruned-away.md",
            "---\nsection: \"X\"\n---\n\n# Pruned\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            result.clean,
            "pruning evidence must satisfy the mapping: {:?}",
            result.findings
        );
    }

    #[test]
    fn reset_template_tasks_satisfy_the_mapping() {
        // Reset-to-template parses as zero tasks — the other pruning
        // fingerprint (§tasks-phase).
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("in-progress"));
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        write(
            tmp.path(),
            "specs/042-demo/tasks.md",
            "# T\n\nTasks derived from the [plan](plan.md). Complete in order.\n",
        );
        write(
            tmp.path(),
            "specs/042-demo/scenarios/reset-away.md",
            "---\nsection: \"X\"\n---\n\n# Reset\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.clean, "{:?}", result.findings);
    }

    /// A `done` spec in a subdirectory of its repository: `alpha`'s task was
    /// pruned from `tasks.md` and `beta` never had one. History names
    /// `tasks.md` from the work tree, so asked by the path from the project
    /// root it found no revision at all and flagged `alpha` as never tasked
    /// too, which is the finding this walk exists to suppress (spec 059).
    #[test]
    fn a_subdirectory_projects_pruned_task_is_found_in_history() {
        use crate::primitives::git_fixture;
        let tmp = tempdir().unwrap();
        let (repository, project) = git_fixture::subdirectory_project(tmp.path());
        write(&project, &format!("specs/{FEATURE}/spec.md"), &spec("done"));
        write(&project, &format!("specs/{FEATURE}/plan.md"), "# Plan\n");
        for slug in ["alpha", "beta"] {
            write(
                &project,
                &format!("specs/{FEATURE}/scenarios/{slug}.md"),
                "---\nsection: \"X\"\n---\n\n# X\n",
            );
        }
        let tasks = format!("specs/{FEATURE}/tasks.md");
        write(
            &project,
            &tasks,
            "# T\n\n## 1. Alpha\n\n- [x] Implement `scenarios/alpha.md`\n\n- **Done when**: alpha.\n",
        );
        git_fixture::commit_all(&repository, "feat: alpha");
        // Reset to a file with no pruning evidence, so only history can
        // answer whether `alpha` was ever tasked.
        write(
            &project,
            &tasks,
            "# T\n\n## 1. Other\n\n- [x] other\n\n- **Done when**: other.\n",
        );
        git_fixture::commit_all(&repository, "chore: reset tasks");

        let result = run(&args(), &project).unwrap();
        let flagged: Vec<&str> = result
            .findings
            .iter()
            .filter(|f| f.family == "scenario-consistency")
            .map(|f| f.path.as_str())
            .collect();
        assert_eq!(flagged, vec![format!("specs/{FEATURE}/scenarios/beta.md")]);
    }

    /// A `done` spec whose history cannot be read in full flags nothing, as
    /// 000's `scenario-without-task-visibility` requires. Builds the repository
    /// the history walk reads: `alpha`'s task, then a reset `tasks.md` with no
    /// pruning evidence, and a `beta` scenario that never had a task.
    fn history_fixture(first_tasks: &[u8]) -> (tempfile::TempDir, git2::Repository) {
        use crate::primitives::git_fixture;
        let tmp = tempdir().unwrap();
        let repository = git2::Repository::init(tmp.path()).unwrap();
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/spec.md"),
            &spec("done"),
        );
        write(tmp.path(), &format!("specs/{FEATURE}/plan.md"), "# Plan\n");
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/scenarios/alpha.md"),
            "---\nsection: \"X\"\n---\n\n# X\n",
        );
        let tasks = tmp.path().join(format!("specs/{FEATURE}/tasks.md"));
        std::fs::write(&tasks, first_tasks).unwrap();
        git_fixture::commit_all(&repository, "feat: alpha");
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/tasks.md"),
            "# T\n\n## 1. Other\n\n- [x] other\n\n- **Done when**: other.\n",
        );
        git_fixture::commit_all(&repository, "chore: reset tasks");
        (tmp, repository)
    }

    fn scenario_findings(repo: &Path) -> Vec<String> {
        run(&args(), repo)
            .unwrap()
            .findings
            .iter()
            .filter(|f| f.family == "scenario-consistency")
            .map(|f| f.path.clone())
            .collect()
    }

    /// A revision whose `tasks.md` is not UTF-8 is one the walk cannot read.
    /// Skipping it flagged `alpha`, whose task appeared only there.
    #[test]
    fn a_revision_the_walk_cannot_decode_flags_nothing() {
        let (tmp, _repository) =
            history_fixture(b"# T\n\n## 1. Alpha \xff\n\n- [x] scenarios/alpha.md\n");
        assert_eq!(scenario_findings(tmp.path()), Vec::<String>::new());
    }

    /// A shallow clone lacks commits, so a scenario whose task lived in one
    /// would read as never tasked; the walk fails safe instead.
    #[test]
    fn a_shallow_history_flags_nothing() {
        let (tmp, repository) = history_fixture(b"# T\n\n## 1. Other\n\n- [x] other\n");
        let head = repository.head().unwrap().target().unwrap();
        std::fs::write(tmp.path().join(".git/shallow"), format!("{head}\n")).unwrap();
        assert!(repository.is_shallow());
        assert_eq!(scenario_findings(tmp.path()), Vec::<String>::new());
    }

    /// With no repository to consult, the history walk fails safe (000's
    /// `scenario-without-task-visibility`), so an unmapped scenario under a
    /// `done` spec flags nothing.
    #[test]
    fn a_done_spec_outside_a_repository_flags_no_scenario() {
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("done"));
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        write(tmp.path(), "specs/042-demo/tasks.md", GOOD_TASKS);
        write(
            tmp.path(),
            "specs/042-demo/scenarios/unmapped-under-done.md",
            "---\nsection: \"X\"\n---\n\n# X\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.clean, "{:?}", result.findings);
    }

    // --- review state drift ---------------------------------------------------------

    #[test]
    fn done_spec_with_unset_last_run_yields_blocking_finding() {
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("done"));
        seed_review(tmp.path(), "  blocking: false");
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        write(tmp.path(), "specs/042-demo/tasks.md", GOOD_TASKS);
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(families(&result), vec![("review-state-drift", "blocking")]);
        assert!(result.findings[0].message.contains("review.last-run unset"));
        assert_eq!(result.findings[0].path, "specs/042-demo/spec.md");
    }

    #[test]
    fn done_spec_with_blocking_review_yields_blocking_finding() {
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("done"));
        seed_review(
            tmp.path(),
            "  last-run: 2026-07-01T00:00:00Z\n  blocking: true\n  must-violations: 2",
        );
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        write(tmp.path(), "specs/042-demo/tasks.md", GOOD_TASKS);
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(families(&result), vec![("review-state-drift", "blocking")]);
        assert!(
            result.findings[0]
                .message
                .contains("unresolved MUST violations")
        );
    }

    #[test]
    fn done_spec_with_outstanding_should_yields_blocking_finding() {
        // The 023 shape, reproduced. It sat at `done` with
        // `should-violations: 1` while its own report said the finding was
        // "Fixed during this pass" — and nothing caught it, because Family 31
        // compares the frontmatter block against review.md and one review run
        // writes both, so the two were consistently stale.
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("done"));
        seed_review(
            tmp.path(),
            "  last-run: 2026-07-01T00:00:00Z\n  blocking: false\n  must-violations: 0\n  should-violations: 1",
        );
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        write(tmp.path(), "specs/042-demo/tasks.md", GOOD_TASKS);
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(families(&result), vec![("review-state-drift", "blocking")]);
        let msg = &result.findings[0].message;
        assert!(msg.contains("1 outstanding SHOULD"), "{msg}");
        // The fix names both dispositions, because a SHOULD whose answer is
        // "keep as-is" is waived rather than fixed, and a message naming only
        // the first would push an operator toward the wrong one.
        assert!(msg.contains("Waived findings"), "{msg}");
    }

    #[test]
    fn done_spec_with_zero_should_is_clean() {
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("done"));
        seed_review(
            tmp.path(),
            "  last-run: 2026-07-01T00:00:00Z\n  blocking: false\n  must-violations: 0\n  should-violations: 0",
        );
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        write(tmp.path(), "specs/042-demo/tasks.md", GOOD_TASKS);
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.clean, "{:?}", result.findings);
    }

    #[test]
    fn an_outstanding_should_on_a_spec_still_in_flight_is_not_a_finding() {
        // A SHOULD is real remaining work, and a spec that has not claimed
        // completion is allowed to carry it. The rule is about the *state*
        // `done` asserts, not about the finding existing.
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("in-progress"));
        seed_review(
            tmp.path(),
            "  last-run: 2026-07-01T00:00:00Z\n  blocking: false\n  must-violations: 0\n  should-violations: 3",
        );
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        write(tmp.path(), "specs/042-demo/tasks.md", GOOD_TASKS);
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            !families(&result)
                .iter()
                .any(|(f, _)| *f == "review-state-drift"),
            "{:?}",
            result.findings
        );
    }

    #[test]
    fn done_spec_without_review_block_is_grandfathered() {
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("done"));
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        write(tmp.path(), "specs/042-demo/tasks.md", GOOD_TASKS);
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.clean, "{:?}", result.findings);
    }

    #[test]
    fn non_done_spec_with_empty_review_block_is_exempt() {
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("in-progress"));
        seed_review(tmp.path(), "  blocking: false");
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        write(tmp.path(), "specs/042-demo/tasks.md", GOOD_TASKS);
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.clean, "{:?}", result.findings);
    }

    // --- plumbing --------------------------------------------------------------------

    #[test]
    fn missing_feature_errors() {
        let tmp = tempdir().unwrap();
        let err = run(&args(), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::FeatureNotFound { .. }));
    }

    #[test]
    fn multiple_families_report_in_declared_order() {
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("done"));
        seed_review(tmp.path(), "  blocking: true");
        // done + no plan.md/tasks.md + review drift (last-run unset AND
        // blocking true) → completeness ×2, then review drift ×2. The
        // scenario family is silent: there is no `tasks.md` to map against.
        write(
            tmp.path(),
            "specs/042-demo/scenarios/some-scenario.md",
            "---\nsection: \"X\"\n---\n\n# X\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(
            families(&result),
            vec![
                ("artifact-completeness", "blocking"),
                ("artifact-completeness", "blocking"),
                ("review-state-drift", "blocking"),
                ("review-state-drift", "blocking"),
            ]
        );
        assert!(!result.clean);
    }

    // --- scenario open questions ----------------------------------------------

    const CLEAN_REVIEW: &str = "  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false";

    const SCENARIO_ASKING: &str = "---\nsection: Behavior\n---\n\n# Retry on timeout\n\n## Open Questions\n\n- Retry budget per call or per request?\n";

    /// Seed a feature at `status` whose single scenario carries one
    /// unresolved question, with plan/tasks present so completeness and
    /// task-consistency stay quiet and the assertion isolates this family.
    fn seed_with_questioning_scenario(repo: &Path, status: &str) {
        write(repo, &format!("specs/{FEATURE}/spec.md"), &spec(status));
        write(repo, &format!("specs/{FEATURE}/plan.md"), "# Demo Plan\n");
        write(repo, &format!("specs/{FEATURE}/tasks.md"), GOOD_TASKS);
        write(
            repo,
            &format!("specs/{FEATURE}/scenarios/retry-on-timeout.md"),
            SCENARIO_ASKING,
        );
    }

    #[test]
    fn scenario_questions_are_blocking_on_a_done_spec() {
        let tmp = tempdir().unwrap();
        seed_with_questioning_scenario(tmp.path(), "done");
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(
            families(&result),
            vec![("scenario-open-questions", "blocking")]
        );
        let message = &result.findings[0].message;
        assert!(
            message.contains("retry-on-timeout"),
            "the finding names the scenario, got: {message}"
        );
    }

    #[test]
    fn scenario_questions_are_advisory_before_done() {
        for status in ["draft", "clarified", "planned", "in-progress"] {
            let tmp = tempdir().unwrap();
            seed_with_questioning_scenario(tmp.path(), status);
            let result = run(&args(), tmp.path()).unwrap();
            assert!(
                families(&result).contains(&("scenario-open-questions", "advisory")),
                "expected an advisory finding at {status}, got {:?}",
                families(&result)
            );
        }
    }

    #[test]
    fn a_done_spec_predating_this_check_is_not_grandfathered() {
        // Unlike review-state drift, where an absent `review:` block marks
        // a spec as predating that feature, an unresolved scenario question
        // is a present-tense defect whenever it arrived (spec 046).
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/spec.md"),
            &spec("done"),
        );
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/plan.md"),
            "# Demo Plan\n",
        );
        write(tmp.path(), &format!("specs/{FEATURE}/tasks.md"), GOOD_TASKS);
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/scenarios/retry-on-timeout.md"),
            SCENARIO_ASKING,
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            families(&result).contains(&("scenario-open-questions", "blocking")),
            "no review block must not exempt the scenario check, got {:?}",
            families(&result)
        );
    }

    #[test]
    fn an_unparseable_scenario_produces_no_blocking_finding() {
        // The gate has the matching test; this pins the finding half of
        // the same rule. Nothing can be proven about the questions of a
        // scenario whose frontmatter will not parse, so it contributes none
        // and raises no finding (spec 046). That is not the unreadable-
        // artifact case `record_unreadable_artifact` makes blocking at
        // `done`: this file reads as text.
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/spec.md"),
            &spec("done"),
        );
        seed_review(tmp.path(), CLEAN_REVIEW);
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/plan.md"),
            "# Demo Plan\n",
        );
        write(tmp.path(), &format!("specs/{FEATURE}/tasks.md"), GOOD_TASKS);
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/scenarios/retry-on-timeout.md"),
            "---\nsection: Behavior\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            !families(&result)
                .iter()
                .any(|(f, _)| *f == "scenario-open-questions"),
            "got {:?}",
            families(&result)
        );
    }

    // --- link-adjacent drift ---------------------------------------------------

    const SCENARIO_SETTLED: &str = "---\nsection: Behavior\n---\n\n# Retry on timeout\n\n## Open Questions\n\n*None — captured during scenario authoring.*\n";

    /// Seed an `in-progress` feature whose one scenario carries no questions,
    /// with `plan.md` supplied by the test. `in-progress` keeps review drift
    /// exempt and lets the scenario→task mapping stay satisfied, so the
    /// assertions isolate this family.
    fn seed_for_drift(repo: &Path, plan_body: &str) {
        write(
            repo,
            &format!("specs/{FEATURE}/spec.md"),
            &spec("in-progress"),
        );
        seed_review(repo, CLEAN_REVIEW);
        write(repo, &format!("specs/{FEATURE}/plan.md"), plan_body);
        write(repo, &format!("specs/{FEATURE}/tasks.md"), GOOD_TASKS);
        write(
            repo,
            &format!("specs/{FEATURE}/scenarios/retry-on-timeout.md"),
            SCENARIO_SETTLED,
        );
    }

    fn drift(result: &CheckArtifactsResult) -> Vec<&ArtifactFinding> {
        result
            .findings
            .iter()
            .filter(|f| f.family == "link-adjacent-drift")
            .collect()
    }

    #[test]
    fn a_stale_open_question_claim_yields_an_advisory_finding() {
        let tmp = tempdir().unwrap();
        seed_for_drift(
            tmp.path(),
            "# Plan\n\nThe [retry scenario](scenarios/retry-on-timeout.md) still has an open question.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let found = drift(&result);
        assert_eq!(found.len(), 1, "{:?}", result.findings);
        // AC7: advisory, never blocking.
        assert_eq!(found[0].severity, AnalyzeSeverity::Advisory);
        // AC4: the citing file and line, the target, and the contradicting state.
        assert_eq!(found[0].path, "specs/042-demo/plan.md");
        let message = &found[0].message;
        assert!(message.contains("line 3"), "{message}");
        assert!(
            message.contains("specs/042-demo/scenarios/retry-on-timeout.md"),
            "{message}"
        );
        assert!(message.contains("zero open questions"), "{message}");
        assert!(message.contains("`open question`"), "{message}");
    }

    #[test]
    fn prose_matching_its_link_targets_produces_nothing() {
        // AC5, and the result that has to stay quiet for the check to be
        // worth running.
        let tmp = tempdir().unwrap();
        seed_for_drift(
            tmp.path(),
            "# Plan\n\nThe [retry scenario](scenarios/retry-on-timeout.md) is settled.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(drift(&result).is_empty(), "{:?}", result.findings);
        assert!(result.skipped.is_empty(), "{:?}", result.skipped);
    }

    #[test]
    fn a_multi_link_block_fires_only_for_the_contradicting_target() {
        // AC12: evaluation is per link.
        let tmp = tempdir().unwrap();
        seed_for_drift(
            tmp.path(),
            "# Plan\n\n- The [asking one](scenarios/asking.md) and the \
             [settled one](scenarios/retry-on-timeout.md) still have an open question.\n",
        );
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/scenarios/asking.md"),
            "---\nsection: Behavior\n---\n\n# Asking\n\n## Open Questions\n\n- Which budget?\n",
        );
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/tasks.md"),
            "# T\n\n## 1. Both\n\n- [ ] `scenarios/retry-on-timeout.md` and `scenarios/asking.md`\n\n- **Done when**: done.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let found = drift(&result);
        assert_eq!(found.len(), 1, "{:?}", result.findings);
        assert!(
            found[0].message.contains("retry-on-timeout.md"),
            "only the settled target contradicts: {}",
            found[0].message
        );
    }

    #[test]
    fn a_tell_in_an_exempt_context_produces_no_finding() {
        // AC13: fenced code, HTML comment, blockquote, inline code span.
        let tmp = tempdir().unwrap();
        seed_for_drift(
            tmp.path(),
            "# Plan\n\n\
             ```\n[a](scenarios/retry-on-timeout.md) is still open\n```\n\n\
             <!-- [b](scenarios/retry-on-timeout.md) is still open -->\n\n\
             > [c](scenarios/retry-on-timeout.md) is still open\n\n\
             The [d](scenarios/retry-on-timeout.md) tell `still open` sits in code font.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(drift(&result).is_empty(), "{:?}", result.findings);
    }

    #[test]
    fn an_implementation_tell_against_a_scenario_is_skipped_not_flagged() {
        // AC14 applying AC9: a scenario carries no lifecycle status, so a
        // tell needing one has nothing to be judged against.
        let tmp = tempdir().unwrap();
        seed_for_drift(
            tmp.path(),
            "# Plan\n\nThe [retry scenario](scenarios/retry-on-timeout.md) is not yet implemented.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(drift(&result).is_empty(), "{:?}", result.findings);
        assert_eq!(result.skipped.len(), 1, "{:?}", result.skipped);
        assert_eq!(result.skipped[0].family, "link-adjacent-drift");
        assert_eq!(result.skipped[0].reason, "no-readable-state");
        assert_eq!(
            result.skipped[0].path,
            "specs/042-demo/scenarios/retry-on-timeout.md"
        );
    }

    #[cfg(unix)]
    #[test]
    fn a_symlinked_sibling_is_skipped_not_read_through() {
        // Scenario sibling-symlink-trust-boundary. Lexical resolution keeps a
        // link *target* from escaping; this covers the other half — a link
        // committed inside the feature dir pointing outside it. The check must
        // report it as unexaminable, never follow it.
        let tmp = tempdir().unwrap();
        seed_for_drift(
            tmp.path(),
            "# Plan\n\nThe [linked](scenarios/linked.md) question is still open.\n",
        );
        let outside = tmp.path().join("outside-the-feature.md");
        std::fs::write(&outside, "secret\n").unwrap();
        std::os::unix::fs::symlink(
            &outside,
            tmp.path()
                .join(format!("specs/{FEATURE}/scenarios/linked.md")),
        )
        .unwrap();

        let result = run(&args(), tmp.path()).unwrap();
        assert!(drift(&result).is_empty(), "{:?}", result.findings);
        let skipped: Vec<_> = result
            .skipped
            .iter()
            .filter(|s| s.path.ends_with("scenarios/linked.md"))
            .collect();
        assert_eq!(skipped.len(), 1, "{:?}", result.skipped);
        assert_eq!(skipped[0].reason, "target-unparseable");
        // The escape stays closed in the other direction too: a link whose
        // target climbs out lexically is refused before any probe.
        assert_eq!(
            resolve_sibling(
                "../../../etc/passwd",
                &tmp.path().join(format!("specs/{FEATURE}/scenarios")),
                &tmp.path().join(format!("specs/{FEATURE}")),
            ),
            None
        );
    }

    #[test]
    fn a_missing_target_is_skipped_not_flagged() {
        let tmp = tempdir().unwrap();
        seed_for_drift(
            tmp.path(),
            "# Plan\n\nThe [gone](scenarios/gone.md) question is still open.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(drift(&result).is_empty(), "{:?}", result.findings);
        assert_eq!(result.skipped.len(), 1, "{:?}", result.skipped);
        assert_eq!(result.skipped[0].reason, "target-missing");
        // The honesty contract: nothing was found, and the result says the
        // subject could not be examined rather than reading as clean.
        assert!(result.clean, "no finding was produced");
    }

    /// The gate this closes. `scenario-open-questions` is blocking at `done`,
    /// but an unreadable scenario contributed no questions and — as a skip —
    /// no finding, so a scenario carrying unresolved questions that happens
    /// not to parse passed the gate built to catch exactly that.
    #[test]
    fn an_unreadable_artifact_blocks_a_done_spec() {
        let tmp = tempdir().unwrap();
        seed_feature(tmp.path(), "done", Some(CLEAN_REVIEW), Some(CLEAN_ANALYZE));
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/plan.md"),
            "# Plan\n\nNothing to see.\n",
        );
        write(tmp.path(), &format!("specs/{FEATURE}/tasks.md"), GOOD_TASKS);
        // Creates scenarios/ so the unreadable file below has a home.
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/scenarios/retry-on-timeout.md"),
            SCENARIO_SETTLED,
        );
        fs::write(
            tmp.path()
                .join(format!("specs/{FEATURE}/scenarios/broken.md")),
            [0xff, 0xfe, 0xfd],
        )
        .unwrap();
        let result = run(&args(), tmp.path()).unwrap();

        let blocking: Vec<_> = result
            .findings
            .iter()
            .filter(|f| {
                f.severity == AnalyzeSeverity::Blocking && f.message.contains("unreadable artifact")
            })
            .collect();
        assert!(!blocking.is_empty(), "{:?}", result.findings);
        assert!(
            blocking
                .iter()
                .all(|f| f.path == "specs/042-demo/scenarios/broken.md")
        );
        // It is a defect now, not an unknown: it must not also be counted as
        // unexamined, or one file would be both.
        assert!(
            !result
                .skipped
                .iter()
                .any(|s| s.reason == "artifact-unreadable"),
            "{:?}",
            result.skipped
        );
    }

    /// Below `done` it stays a skipped target: the questions check is advisory
    /// there, the spec is in flight, and an unreadable artifact mid-work is a
    /// state to report rather than a gate to fail.
    #[test]
    fn an_unreadable_artifact_below_done_is_still_only_skipped() {
        let tmp = tempdir().unwrap();
        seed_for_drift(tmp.path(), "# Plan\n\nNothing to see.\n");
        fs::write(
            tmp.path()
                .join(format!("specs/{FEATURE}/scenarios/broken.md")),
            [0xff, 0xfe, 0xfd],
        )
        .unwrap();
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            !result
                .findings
                .iter()
                .any(|f| f.message.contains("unreadable artifact")),
            "{:?}",
            result.findings
        );
        assert!(
            result
                .skipped
                .iter()
                .any(|s| s.reason == "artifact-unreadable")
        );
    }

    #[test]
    fn an_unreadable_citing_artifact_is_recorded_not_dropped() {
        // The citing side of QUAL-CLAIM-001: a scanned artifact the family
        // could not read must not leave a partially-scanned feature looking
        // exactly like a fully-scanned clean one.
        let tmp = tempdir().unwrap();
        seed_for_drift(tmp.path(), "# Plan\n\nNothing to see.\n");
        // Invalid UTF-8 is the reachable form of unreadable here.
        fs::write(
            tmp.path()
                .join(format!("specs/{FEATURE}/scenarios/broken.md")),
            [0xff, 0xfe, 0xfd],
        )
        .unwrap();
        let result = run(&args(), tmp.path()).unwrap();
        assert!(drift(&result).is_empty(), "{:?}", result.findings);
        // Scoped by family: one unreadable file is legitimately skipped by
        // every family whose subject it is, which is what `family` on
        // SkippedTarget distinguishes. `scenario-open-questions` also records
        // it (asserted below) — two records of one file, not a duplicate.
        let skips: Vec<_> = result
            .skipped
            .iter()
            .filter(|s| s.reason == "artifact-unreadable" && s.family == "link-adjacent-drift")
            .collect();
        assert_eq!(skips.len(), 1, "{:?}", result.skipped);
        assert_eq!(skips[0].path, "specs/042-demo/scenarios/broken.md");

        // The same file is the scenario-question collector's subject too, and
        // it yields no questions — so without this record the family would
        // report clean over a scenario it never read (QUAL-CLAIM-001).
        let scenario_skips: Vec<_> = result
            .skipped
            .iter()
            .filter(|s| s.family == "scenario-open-questions")
            .collect();
        assert_eq!(scenario_skips.len(), 1, "{:?}", result.skipped);
        assert_eq!(scenario_skips[0].reason, "artifact-unreadable");
        assert_eq!(scenario_skips[0].path, "specs/042-demo/scenarios/broken.md");
    }

    #[test]
    fn cross_feature_and_external_links_are_out_of_scope() {
        let tmp = tempdir().unwrap();
        seed_for_drift(
            tmp.path(),
            "# Plan\n\nSee [other](../099-other/spec.md) and [web](https://example.com/x) \
             — the question is still open.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(drift(&result).is_empty(), "{:?}", result.findings);
        assert!(
            result.skipped.is_empty(),
            "a non-sibling is not a skipped target: {:?}",
            result.skipped
        );
    }

    #[test]
    fn every_artifact_kind_in_the_feature_directory_is_scanned() {
        // AC6: spec.md, plan.md, tasks.md, scenarios/*.md.
        let tmp = tempdir().unwrap();
        seed_for_drift(
            tmp.path(),
            "# Plan\n\nThe [scenario](scenarios/retry-on-timeout.md) still has an open question.\n",
        );
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/spec.md"),
            &format!(
                "{}\nThe [scenario](scenarios/retry-on-timeout.md) still has an open question.\n",
                spec("in-progress")
            ),
        );
        seed_review(tmp.path(), CLEAN_REVIEW);
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/tasks.md"),
            "# T\n\n## 1. Retry\n\n- [ ] The [scenario](scenarios/retry-on-timeout.md) still has an open question.\n\n- **Done when**: done.\n",
        );
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/scenarios/retry-on-timeout.md"),
            "---\nsection: Behavior\n---\n\n# Retry on timeout\n\nThe [spec](../spec.md) still has an open question.\n\n## Open Questions\n\n*None — captured during scenario authoring.*\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let citing: Vec<&str> = drift(&result).iter().map(|f| f.path.as_str()).collect();
        assert_eq!(
            citing,
            vec![
                "specs/042-demo/spec.md",
                "specs/042-demo/plan.md",
                "specs/042-demo/tasks.md",
                "specs/042-demo/scenarios/retry-on-timeout.md",
            ],
            "{:?}",
            result.findings
        );
    }

    #[test]
    fn repeat_runs_produce_identical_findings_and_skips() {
        // AC8. Nothing on this path reads wall-clock time or raw directory
        // order, so two runs over an unchanged tree must agree exactly.
        let tmp = tempdir().unwrap();
        seed_for_drift(
            tmp.path(),
            "# Plan\n\nThe [retry scenario](scenarios/retry-on-timeout.md) still has an open question, \
             and the [gone one](scenarios/gone.md) is not yet built.\n",
        );
        let first = run(&args(), tmp.path()).unwrap();
        let second = run(&args(), tmp.path()).unwrap();
        assert_eq!(first.findings, second.findings);
        assert_eq!(first.skipped, second.skipped);
        assert!(
            !first.findings.is_empty(),
            "the fixture must produce output"
        );
        assert!(!first.skipped.is_empty(), "the fixture must produce skips");
    }

    #[test]
    fn a_changed_section_behind_a_working_link_is_not_flagged() {
        // The recorded non-goal: the link resolves, but the cited section no
        // longer says what the prose claims. Verifying that needs a fragment
        // anchor or semantic reading, so the check has no opinion.
        let tmp = tempdir().unwrap();
        seed_for_drift(
            tmp.path(),
            "# Plan\n\nSee the note in [tasks](tasks.md) about the retry budget.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(drift(&result).is_empty(), "{:?}", result.findings);
    }

    #[test]
    fn no_new_family_can_block_a_gate() {
        // AC7 as an invariant rather than a per-test assertion.
        let tmp = tempdir().unwrap();
        seed_for_drift(
            tmp.path(),
            "# Plan\n\nThe [retry scenario](scenarios/retry-on-timeout.md) still has an open question.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            result
                .findings
                .iter()
                .filter(|f| f.family == "link-adjacent-drift")
                .all(|f| f.severity == AnalyzeSeverity::Advisory),
            "{:?}",
            result.findings
        );
    }

    // --- criterion path existence ----------------------------------------------

    /// A spec at `status` whose Acceptance Criteria section is supplied by the
    /// test, with plan/tasks present so the other families stay quiet.
    fn seed_with_criteria(repo: &Path, status: &str, criteria: &str) {
        write(
            repo,
            &format!("specs/{FEATURE}/spec.md"),
            &format!("{}\n## Acceptance Criteria\n\n{criteria}", spec(status)),
        );
        write(repo, &format!("specs/{FEATURE}/plan.md"), "# Demo Plan\n");
        write(repo, &format!("specs/{FEATURE}/tasks.md"), GOOD_TASKS);
    }

    /// A minimal **Shared Files** manifest, enough for `adopter_destinations`
    /// to derive from. Mirrors the real table's two-column shape.
    fn seed_manifest(repo: &Path) {
        write(
            repo,
            "framework/bootstrap/ductus.md",
            "# ductus\n\n## Shared Files\n\n\
             | Source Path | Destination Path |\n\
             | --- | --- |\n\
             | `framework/constitution.md` | `.ductus/constitution.md` |\n\
             | `framework/templates/spec/spec.md` | `specs/templates/spec.md` |\n\
             | `framework/rules/security-backend.md` | `specs/rules/security-backend.md` |\n",
        );
    }

    /// The helper normalizes its own candidate rather than trusting the
    /// caller to. Both call sites reach it differently:
    /// `criterion-path-existence` passes an already-trimmed span, so it was
    /// never affected; `check-orphaned-references` passes the raw target, and
    /// a directory written with its trailing slash — which is how prose
    /// normally spells one — built the prefix `specs/rules//` and matched
    /// nothing, so a reference to a shipped directory was reported as local
    /// breakage on every run.
    #[test]
    fn ships_to_adopter_matches_a_directory_written_with_a_trailing_slash() {
        let destinations: BTreeSet<String> = [
            "specs/rules/security-backend.md".to_string(),
            "specs/templates/spec.md".to_string(),
            ".ductus/constitution.md".to_string(),
        ]
        .into_iter()
        .collect();

        // The regression: both spellings of a shipped directory must match.
        assert!(ships_to_adopter(&destinations, "specs/rules"));
        assert!(
            ships_to_adopter(&destinations, "specs/rules/"),
            "a trailing slash must not defeat the directory prefix test"
        );
        assert!(ships_to_adopter(&destinations, "specs/templates/"));

        // Exact destinations still match, with or without stray slashes.
        assert!(ships_to_adopter(&destinations, ".ductus/constitution.md"));

        // And a directory this repo does not ship into still does not match,
        // so the normalization widened nothing.
        assert!(!ships_to_adopter(&destinations, "specs/scenarios/"));
        assert!(!ships_to_adopter(&destinations, "runtime/src/"));
    }

    #[test]
    fn a_path_this_repo_ships_to_adopters_is_skipped_not_flagged() {
        // Scenario criterion-adopter-scope-destinations. `root-absent` cannot
        // catch these — `specs` and `.ductus` both exist here — so without the
        // manifest check a framework repo reports a defect for every file it
        // correctly delivers elsewhere.
        for (criterion, subject) in [
            (
                "- [x] A freshly adopted project has the constitution at `.ductus/constitution.md`.\n",
                ".ductus/constitution.md",
            ),
            (
                "- [x] The \"Secure\" principle references `specs/rules/security-backend.md`.\n",
                "specs/rules/security-backend.md",
            ),
            // The directory case: the criterion names a folder that contains
            // manifest destinations rather than being one itself.
            (
                "- [x] Copies spec templates into `specs/templates/`.\n",
                "specs/templates/",
            ),
        ] {
            let tmp = tempdir().unwrap();
            seed_with_criteria(tmp.path(), "done", criterion);
            seed_manifest(tmp.path());
            fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
            let result = run(&args(), tmp.path()).unwrap();
            assert!(
                path_findings(&result).is_empty(),
                "ships to an adopter, must not flag: {criterion} -> {:?}",
                result.findings
            );
            assert!(
                result
                    .skipped
                    .iter()
                    .any(|s| s.reason == "ships-to-adopter" && s.path == subject),
                "the skip must be recorded, not silent: {criterion} -> {:?}",
                result.skipped
            );
        }
    }

    /// The three registry tables `agent_scaffold_destinations` reads, in the
    /// real file's shapes: a dogfooded agent (`pi`), one that is not
    /// (`opencode`), cells carrying a trailing note, a formula still holding
    /// a placeholder after substitution, and a user-global MCP target.
    const REGISTRY: &str = "## Agent Registry\n\n\
        | `key` | `name` | `config_dir` | `layout` |\n\
        | --- | --- | --- | --- |\n\
        | `opencode` | OpenCode | `.opencode` | `opencode` |\n\
        | `auggie` | Auggie | `.augment` | `claude-style` |\n\
        | `pi` | Pi | `.pi` | `pi` |\n\n\
        ### Derived values\n\n\
        | Derived value | Formula |\n\
        | --- | --- |\n\
        | Configure source path | `framework/bootstrap/configure/{key}.md` |\n\n\
        | Derived value | `claude-style` | `opencode` | `pi` |\n\
        | --- | --- | --- | --- |\n\
        | Command/skill path | `{config_dir}/commands/{project}/<name>.md` | `{config_dir}/command/{project}/<name>.md` | `{config_dir}/prompts/{project}-<name>.md` |\n\
        | `ductus` install path | `{config_dir}/commands/ductus.md` | `{config_dir}/command/ductus.md` | `{config_dir}/prompts/ductus.md` |\n\
        | Settings file | `{config_dir}/settings.local.json` | `opencode.json` (repo root; same file as MCP wiring) | `{config_dir}/settings.json` (no permission-gating surface) |\n\
        | Native rules file | `CLAUDE.md` | `AGENTS.md` | `AGENTS.md` |\n\n\
        ### MCP registration (per-agent)\n\n\
        | `key` | MCP target | scope | mechanism |\n\
        | --- | --- | --- | --- |\n\
        | `auggie` | `~/.augment/settings.json` | `user-global` | `surface-instruction` |\n\
        | `opencode` | `opencode.json` (repo root) `mcp` block | `project-committed` | `write-file` |\n\
        | `pi` | `.pi/extensions/ductus.ts` (the bridge) | `project-local` (gitignored) | `write-file` |\n\n\
        ## Shared Files\n";

    #[test]
    fn the_registry_arm_derives_each_agents_scaffold_paths() {
        // Scenario per-agent-scaffold-paths-ship-to-adopter. Exactly these:
        // the install path and settings file per registered agent, plus the
        // project-scoped MCP targets. Not the command path (`{project}` and
        // `<name>` survive substitution), not the rules file (a row this
        // arm does not read), not Auggie's home-relative MCP target.
        let got = agent_scaffold_destinations(REGISTRY);
        let want: BTreeSet<String> = [
            ".augment/commands/ductus.md",
            ".augment/settings.local.json",
            ".opencode/command/ductus.md",
            ".pi/extensions/ductus.ts",
            ".pi/prompts/ductus.md",
            ".pi/settings.json",
            "opencode.json",
        ]
        .into_iter()
        .map(str::to_string)
        .collect();
        assert_eq!(got, want);
    }

    #[test]
    fn a_table_quoted_in_a_fence_or_comment_is_not_the_registry() {
        // The section is read through the shared fence- and comment-aware
        // walker: an example registry row inside a code fence or an HTML
        // comment under the heading describes a shape, and must not ship an
        // agent that is not registered.
        let quoted = REGISTRY.replace(
            "| `pi` | Pi | `.pi` | `pi` |\n",
            "| `pi` | Pi | `.pi` | `pi` |\n\n\
             ```text\n\
             | `key` | `name` | `config_dir` | `layout` |\n\
             | --- | --- | --- | --- |\n\
             | `fenced` | Fenced | `.fenced` | `pi` |\n\
             ```\n\n\
             <!--\n\
             | `key` | `name` | `config_dir` | `layout` |\n\
             | --- | --- | --- | --- |\n\
             | `commented` | Commented | `.commented` | `pi` |\n\
             -->\n",
        );
        let got = agent_scaffold_destinations(&quoted);
        assert!(
            got.iter()
                .all(|p| !p.starts_with(".fenced/") && !p.starts_with(".commented/")),
            "a quoted row was read as a registered agent: {got:?}"
        );
        assert!(got.contains(".pi/prompts/ductus.md"), "{got:?}");
    }

    #[test]
    fn a_missing_registry_table_contributes_nothing_from_its_arm() {
        // Each table is read on its own. Without the registry there is no
        // agent to substitute into the formulas, so the arm is empty; the
        // Derived values and MCP tables alone name no concrete path.
        let without_registry = REGISTRY.replace("## Agent Registry", "## Something Else");
        assert!(agent_scaffold_destinations(&without_registry).is_empty());
        // Without Derived values only the project-scoped MCP targets remain.
        let without_derived = REGISTRY.replace("### Derived values", "### Not The Table");
        assert_eq!(
            agent_scaffold_destinations(&without_derived),
            ["opencode.json", ".pi/extensions/ductus.ts"]
                .into_iter()
                .map(str::to_string)
                .collect::<BTreeSet<_>>()
        );
    }

    #[test]
    fn the_real_registry_yields_every_agents_scaffold_paths() {
        // The guard against the subject silently narrowing: a change to the
        // bootstrap's table shapes that broke the parse would empty the arm,
        // and the family would go back to flagging 064's install criteria.
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let text = fs::read_to_string(root.join("framework/bootstrap/ductus.md")).unwrap();
        let got = agent_scaffold_destinations(&text);
        for path in [
            // 064 AC1/AC3 — the case that surfaced this.
            ".pi/prompts/ductus.md",
            ".pi/settings.json",
            ".pi/extensions/ductus.ts",
            // Agents not dogfooded here contribute all the same.
            ".opencode/command/ductus.md",
            ".agents/skills/ductus/SKILL.md",
            ".claude/commands/ductus.md",
            ".claude/settings.local.json",
            ".mcp.json",
        ] {
            assert!(got.contains(path), "missing {path}: {got:?}");
        }
        assert!(
            got.iter()
                .all(|p| !p.contains(['{', '<']) && !p.starts_with('~')),
            "a placeholder or home-relative path leaked: {got:?}"
        );
    }

    #[test]
    fn a_registered_agents_scaffold_path_is_skipped_not_flagged() {
        // The dogfooded agent's paths (`.pi` exists here, so root-absent does
        // not fire) and an agent this tree does not dogfood but whose segment
        // exists all the same.
        for (criterion, subject) in [
            (
                "- [x] The self-install lands at `.pi/prompts/ductus.md`.\n",
                ".pi/prompts/ductus.md",
            ),
            (
                "- [x] Pi's settings seed is `.pi/settings.json`.\n",
                ".pi/settings.json",
            ),
            (
                "- [x] The installer lands at `.opencode/command/ductus.md`.\n",
                ".opencode/command/ductus.md",
            ),
        ] {
            let tmp = tempdir().unwrap();
            seed_with_criteria(tmp.path(), "done", criterion);
            write(tmp.path(), "framework/bootstrap/ductus.md", REGISTRY);
            fs::create_dir_all(tmp.path().join(".pi/prompts")).unwrap();
            fs::create_dir_all(tmp.path().join(".opencode")).unwrap();
            let result = run(&args(), tmp.path()).unwrap();
            assert!(
                path_findings(&result).is_empty(),
                "ships to an adopter, must not flag: {criterion} -> {:?}",
                result.findings
            );
            assert!(
                result
                    .skipped
                    .iter()
                    .any(|s| s.reason == "ships-to-adopter" && s.path == subject),
                "the skip must be recorded, not silent: {criterion} -> {:?}",
                result.skipped
            );
        }
    }

    #[test]
    fn a_config_dir_path_no_registry_row_derives_still_flags() {
        // The suppression is scoped to derived destinations, not a blanket
        // `{config_dir}/` exemption.
        let tmp = tempdir().unwrap();
        seed_with_criteria(
            tmp.path(),
            "done",
            "- [x] The prompt lives at `.pi/prompts/stale-prompt.md`.\n",
        );
        write(tmp.path(), "framework/bootstrap/ductus.md", REGISTRY);
        fs::create_dir_all(tmp.path().join(".pi/prompts")).unwrap();
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(path_findings(&result).len(), 1, "{:?}", result.findings);
    }

    #[test]
    fn the_shared_files_arm_survives_a_missing_registry() {
        // The two arms fail independently: a manifest with Shared Files and
        // no registry still suppresses its rows, while a registry path then
        // flags — reported rather than swallowed.
        let tmp = tempdir().unwrap();
        seed_with_criteria(
            tmp.path(),
            "done",
            "- [x] Adopted at `.ductus/constitution.md`; installer at `.pi/prompts/ductus.md`.\n",
        );
        seed_manifest(tmp.path());
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::create_dir_all(tmp.path().join(".pi/prompts")).unwrap();
        let result = run(&args(), tmp.path()).unwrap();
        let found = path_findings(&result);
        assert_eq!(found.len(), 1, "{:?}", result.findings);
        assert!(
            found[0]
                .message
                .starts_with("acceptance criterion names `.pi/prompts/ductus.md`"),
            "{found:?}"
        );
        assert!(
            result
                .skipped
                .iter()
                .any(|s| s.reason == "ships-to-adopter" && s.path == ".ductus/constitution.md"),
            "{:?}",
            result.skipped
        );
    }

    #[test]
    fn without_a_manifest_nothing_is_suppressed() {
        // The adopter case: no `framework/bootstrap/ductus.md`, so derivation
        // yields an empty set and the check reports as before. Fails toward
        // reporting, never toward silence.
        let tmp = tempdir().unwrap();
        seed_with_criteria(
            tmp.path(),
            "done",
            "- [x] A freshly adopted project has the constitution at `.ductus/constitution.md`.\n",
        );
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(path_findings(&result).len(), 1, "{:?}", result.findings);
        assert!(
            !result
                .skipped
                .iter()
                .any(|s| s.reason == "ships-to-adopter"),
            "{:?}",
            result.skipped
        );
    }

    #[test]
    fn a_genuinely_stale_path_still_flags_alongside_the_manifest() {
        // The suppression must be scoped to declared destinations, not a
        // blanket exemption: a path the manifest does not ship is still drift.
        let tmp = tempdir().unwrap();
        seed_with_criteria(
            tmp.path(),
            "done",
            "- [x] The hygiene tool lives at `scripts/lint-ductus-toml.sh`.\n",
        );
        seed_manifest(tmp.path());
        fs::create_dir_all(tmp.path().join("scripts")).unwrap();
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(path_findings(&result).len(), 1, "{:?}", result.findings);
    }

    fn path_findings(result: &CheckArtifactsResult) -> Vec<&ArtifactFinding> {
        result
            .findings
            .iter()
            .filter(|f| f.family == "criterion-path-existence")
            .collect()
    }

    /// The originating case: 026's AC5, after `531e3ea` deleted both subjects.
    const ORIGINATING_CRITERION: &str = "- [x] Registry equivalence verifies every entry in `framework/workflows/registry.json` against `scripts/audit/registry-equivalence.sh`\n";

    #[test]
    fn the_originating_case_is_reproduced() {
        // AC18. Both named paths are gone, exactly as they are gone from
        // `ductus` after spec 043 sunset the workflows feature — while their
        // parent trees survive, which is what makes their absence provable
        // rather than merely unknown. The fixture creates `framework/` and
        // `scripts/` for that reason: without them the root-absent rule would
        // (correctly) call the paths unexaminable instead.
        let tmp = tempdir().unwrap();
        seed_with_criteria(tmp.path(), "done", ORIGINATING_CRITERION);
        fs::create_dir_all(tmp.path().join("framework")).unwrap();
        fs::create_dir_all(tmp.path().join("scripts/audit")).unwrap();
        let result = run(&args(), tmp.path()).unwrap();
        let found = path_findings(&result);
        assert_eq!(found.len(), 2, "{:?}", result.findings);
        assert!(
            found[0]
                .message
                .contains("framework/workflows/registry.json"),
            "{}",
            found[0].message
        );
        assert!(
            found[1]
                .message
                .contains("scripts/audit/registry-equivalence.sh"),
            "{}",
            found[1].message
        );
        // AC7: advisory, and anchored on the citing spec.
        assert_eq!(found[0].severity, AnalyzeSeverity::Advisory);
        assert_eq!(found[0].path, "specs/042-demo/spec.md");
        // The criterion text is carried so the reader sees which contract broke.
        assert!(found[0].message.contains("Registry equivalence"));
    }

    #[test]
    fn a_spec_below_done_is_not_scanned() {
        // Criteria below `done` describe work in flight, so a path that does
        // not exist yet is expected rather than drifted.
        for status in ["draft", "clarified", "planned", "in-progress"] {
            let tmp = tempdir().unwrap();
            seed_with_criteria(tmp.path(), status, ORIGINATING_CRITERION);
            let result = run(&args(), tmp.path()).unwrap();
            assert!(
                path_findings(&result).is_empty(),
                "expected no findings at {status}: {:?}",
                result.findings
            );
        }
        // …and being not-applicable is not the same as having tried and
        // failed, so nothing is recorded as skipped either.
        let tmp = tempdir().unwrap();
        seed_with_criteria(tmp.path(), "in-progress", ORIGINATING_CRITERION);
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            !result
                .skipped
                .iter()
                .any(|s| s.family == "criterion-path-existence"),
            "{:?}",
            result.skipped
        );
    }

    #[test]
    fn body_prose_naming_a_deleted_path_is_not_flagged() {
        // AC17, and the reason the check is scoped to criteria: 026's own
        // Behavior section correctly records a path that is supposed to be
        // gone. Widening the scope would flag a true statement.
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/spec.md"),
            &format!(
                "{}\n## Behavior\n\nRetired by spec 043, which deleted `framework/workflows/`.\n\n\
                 ## Acceptance Criteria\n\n- [x] The audit runs in CI\n",
                spec("done")
            ),
        );
        seed_review(tmp.path(), CLEAN_REVIEW);
        write(tmp.path(), &format!("specs/{FEATURE}/plan.md"), "# Plan\n");
        write(tmp.path(), &format!("specs/{FEATURE}/tasks.md"), GOOD_TASKS);
        let result = run(&args(), tmp.path()).unwrap();
        assert!(path_findings(&result).is_empty(), "{:?}", result.findings);
    }

    #[test]
    fn the_grammar_rejects_everything_that_is_not_a_path() {
        // Each rejection is load-bearing against real criteria text: without
        // it the check would be a noise generator rather than a signal.
        let tmp = tempdir().unwrap();
        seed_with_criteria(
            tmp.path(),
            "done",
            "- [x] `/{project}:analyze` documents it, per `https://example.com/spec` and \
             `runtime/src/primitives/mod.rs:841`, across `specs/*/spec.md`, passing `--exclude=a/b`, \
             touching `scripts/…` and `specs/NNN-feature/review.md`\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            path_findings(&result).is_empty(),
            "no candidate should survive the grammar: {:?}",
            result.findings
        );
    }

    #[test]
    fn a_path_whose_root_is_absent_is_skipped_not_flagged() {
        // A framework repo's criteria legitimately name paths that live in an
        // *adopter's* checkout. Calling those drifted would assert a defect
        // from an absence of evidence — so they are recorded, not flagged.
        let tmp = tempdir().unwrap();
        seed_with_criteria(
            tmp.path(),
            "done",
            "- [x] A freshly adopted project has the constitution at `.ductus/constitution.md`\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(path_findings(&result).is_empty(), "{:?}", result.findings);
        let skips: Vec<_> = result
            .skipped
            .iter()
            .filter(|s| s.family == "criterion-path-existence")
            .collect();
        assert_eq!(skips.len(), 1, "{:?}", result.skipped);
        assert_eq!(skips[0].reason, "root-absent");
        assert_eq!(skips[0].path, ".ductus/constitution.md");
    }

    #[test]
    fn a_present_root_still_proves_a_missing_path() {
        // The rule self-corrects: where the root exists — an adopter repo, or
        // `framework/` here — a missing path beneath it is provable again.
        let tmp = tempdir().unwrap();
        seed_with_criteria(
            tmp.path(),
            "done",
            "- [x] The constitution ships to `.ductus/constitution.md`\n",
        );
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(path_findings(&result).len(), 1, "{:?}", result.findings);
    }

    #[test]
    fn a_criterion_that_is_not_a_live_claim_is_skipped_not_flagged() {
        // The sharpest of the five: a deletion criterion is *satisfied* by the
        // path being gone, so flagging it is exactly backwards. All five
        // groups are covered here because they share one exemption.
        for criterion in [
            "- [x] `framework/commands/capture.md` is deleted; its generated copy is regenerated as deleted.\n",
            "- [x] `framework/workflows/` does not exist, and no live artifact references it.\n",
            "- [x] `framework/rules/configuration.md` is renamed to `framework/rules/configuration-cross.md`.\n",
            "- [x] The ductus command writes it to `specs/rules/security-backend.md` in the project.\n",
            "- [x] Project-local rule files outside the rule dir (e.g., `docs/rules/internal-api.md`) still load.\n",
            "- [x] The key is validated by `scripts/lint-ductus-toml.sh` (if it exists).\n",
            // Scenario criterion-non-assertion-phrasings — the three forms the
            // narrower list missed, each earned against a real criterion in
            // this repo. Past-tense agent (045 AC18):
            "- [x] The check reproduces the case: 026's AC5 naming `framework/workflows/registry.json` after `531e3ea` deleted both.\n",
            // Parenthetical rename history (003):
            "- [x] Commands reference `.govern.session.toml` for session state (was `.claude/gov-session.json` pre-0.10.0).\n",
            // Migration subject (043) — manifest data naming what to remove:
            "- [x] `framework/migrations.toml` carries an entry whose target paths cover `framework/workflows/`.\n",
            // Scenario criterion-negated-creation-phrasing — the sixth group,
            // carried by a clause-scoped predicate rather than a phrase. The
            // first is the reported adopter criterion verbatim: its negator
            // and verb are split by the noun between them, and the path it
            // asserts was never created carries a `.` that a naive clause
            // splitter cuts in half.
            "- [x] AC13: `server/.gitignore` exists (Rails-generated) and `server/config/master.key` is git-ignored and not committed; no nested `server/.git/` repository was created.\n",
            // Negator adjacent to the verb, past tense:
            "- [x] `framework/workflows/registry.json` was never created, and nothing references it.\n",
            // Negator adjacent to the verb, present participle:
            "- [x] The run completes without a `.ductus/lock` file being created.\n",
            // A creation verb other than `created`:
            "- [x] No `scripts/lint-ductus-toml.sh` was added by this spec.\n",
        ] {
            let tmp = tempdir().unwrap();
            seed_with_criteria(tmp.path(), "done", criterion);
            fs::create_dir_all(tmp.path().join("framework/rules")).unwrap();
            fs::create_dir_all(tmp.path().join("specs/rules")).unwrap();
            fs::create_dir_all(tmp.path().join("docs/rules")).unwrap();
            let result = run(&args(), tmp.path()).unwrap();
            assert!(
                path_findings(&result).is_empty(),
                "not a live claim, must not flag: {criterion} -> {:?}",
                result.findings
            );
            assert!(
                result
                    .skipped
                    .iter()
                    .any(|s| s.reason == "not-a-live-claim"),
                "the exemption must be recorded, not silent: {criterion} -> {:?}",
                result.skipped
            );
        }
    }

    #[test]
    fn a_live_claim_alongside_a_transition_word_still_flags() {
        // The marker list is closed and phrase-shaped on purpose: "adopter"
        // alone must not exempt a criterion, or 018's real stale path
        // ("runs the adopter-relevant generators (currently
        // `scripts/gen-spec-deps.sh`)") would go unreported.
        let tmp = tempdir().unwrap();
        seed_with_criteria(
            tmp.path(),
            "done",
            "- [x] The hook runs the adopter-relevant generators (currently `scripts/gen-spec-deps.sh`).\n",
        );
        fs::create_dir_all(tmp.path().join("scripts")).unwrap();
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(path_findings(&result).len(), 1, "{:?}", result.findings);
    }

    #[test]
    fn a_negator_outside_the_creation_clause_still_flags() {
        // The guard on the negated-creation predicate, and the reason it is
        // clause-scoped rather than criterion-scoped: `no` and `not` are
        // common in criteria prose, so matching them anywhere in the criterion
        // would exempt the delivery claim sitting in a different clause — the
        // over-exemption that makes a blinded family worse than a false
        // positive. Each case names a path that does not resolve.
        for criterion in [
            // Negator in a later clause than the creation verb:
            "- [x] `scripts/gen-spec-deps.sh` was added by this spec; no further generators are needed.\n",
            // Negator in the same clause but *after* the verb — a delivery
            // claim carrying a trailing qualifier, not a negated creation.
            "- [x] `scripts/gen-spec-deps.sh` was created and not modified since.\n",
            // A creation verb with no negator anywhere.
            "- [x] `scripts/gen-spec-deps.sh` was introduced by this spec.\n",
        ] {
            let tmp = tempdir().unwrap();
            seed_with_criteria(tmp.path(), "done", criterion);
            fs::create_dir_all(tmp.path().join("scripts")).unwrap();
            let result = run(&args(), tmp.path()).unwrap();
            assert_eq!(
                path_findings(&result).len(),
                1,
                "a live claim must still flag: {criterion} -> {:?}",
                result.findings
            );
        }
    }

    #[test]
    fn a_dotted_path_does_not_split_the_clause_that_negates_its_creation() {
        // The regression this predicate's first draft had: splitting clauses
        // on a bare `.` cuts `server/.git/` between the negator and the verb,
        // so the criterion that motivated the whole change went on flagging.
        // Asserted at the predicate rather than through `run`, because the
        // failure is invisible once the criterion is exempted by other means.
        let ac13 = "ac13: `server/.gitignore` exists (rails-generated) and \
                    `server/config/master.key` is git-ignored and not committed; \
                    no nested `server/.git/` repository was created.";
        assert!(
            asserts_negated_creation(ac13),
            "the negator and its verb share a clause: {ac13}"
        );
        assert!(!is_live_assertion(ac13));
    }

    #[test]
    fn a_bare_directory_name_is_not_a_path() {
        // "the feature's `scenarios/` directory" names a concept, not a path
        // to resolve — the separator has to be internal to count.
        let tmp = tempdir().unwrap();
        seed_with_criteria(
            tmp.path(),
            "done",
            "- [x] `target` reports no scenarios when the feature has no `scenarios/` directory\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(path_findings(&result).is_empty(), "{:?}", result.findings);
        assert!(result.skipped.is_empty(), "{:?}", result.skipped);
    }

    #[test]
    fn quoting_and_dot_slash_are_normalized_away() {
        let tmp = tempdir().unwrap();
        seed_with_criteria(
            tmp.path(),
            "done",
            "- [x] The loader reads `\"framework/rules/demo-cross.md\"` and `./framework/rules/demo-cross.md`\n",
        );
        fs::create_dir_all(tmp.path().join("framework/rules")).unwrap();
        write(tmp.path(), "framework/rules/demo-cross.md", "# Demo\n");
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            path_findings(&result).is_empty(),
            "both forms name the same present file: {:?}",
            result.findings
        );
    }

    #[test]
    fn a_resolving_path_produces_no_finding() {
        let tmp = tempdir().unwrap();
        seed_with_criteria(
            tmp.path(),
            "done",
            "- [x] The generator lives at `scripts/gen-demo.sh` and its rules at `framework/rules/`\n",
        );
        write(tmp.path(), "scripts/gen-demo.sh", "#!/bin/sh\n");
        // A directory reference, trailing slash and all, resolves too.
        fs::create_dir_all(tmp.path().join("framework/rules")).unwrap();
        let result = run(&args(), tmp.path()).unwrap();
        assert!(path_findings(&result).is_empty(), "{:?}", result.findings);
    }

    #[test]
    fn scenarios_without_questions_produce_no_finding() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/spec.md"),
            &spec("done"),
        );
        seed_review(tmp.path(), CLEAN_REVIEW);
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/plan.md"),
            "# Demo Plan\n",
        );
        write(tmp.path(), &format!("specs/{FEATURE}/tasks.md"), GOOD_TASKS);
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/scenarios/retry-on-timeout.md"),
            "---\nsection: Behavior\n---\n\n## Open Questions\n\n*None — captured during scenario authoring.*\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            !families(&result)
                .iter()
                .any(|(f, _)| *f == "scenario-open-questions"),
            "got {:?}",
            families(&result)
        );
    }

    // --- criterion labels ----------------------------------------------------

    /// Seed a feature whose spec carries `criteria` under `## Acceptance
    /// Criteria` and, when `counter` is set, that raw `next-criterion:`
    /// value. The counter is a string rather than an integer so a corrupted
    /// one — a state this family reports — can be seeded at all.
    fn seed_with_labels(repo: &Path, status: &str, counter: Option<&str>, criteria: &str) {
        let counter_line = counter.map_or_else(String::new, |c| format!("next-criterion: {c}\n"));
        write(
            repo,
            &format!("specs/{FEATURE}/spec.md"),
            &format!(
                "---\nstatus: {status}\ndependencies: []\n{counter_line}---\n\n\
                 # Demo\n\n## Acceptance Criteria\n\n{criteria}"
            ),
        );
        write(repo, &format!("specs/{FEATURE}/plan.md"), "# Demo Plan\n");
        write(repo, &format!("specs/{FEATURE}/tasks.md"), GOOD_TASKS);
    }

    fn label_findings(result: &CheckArtifactsResult) -> Vec<&ArtifactFinding> {
        result
            .findings
            .iter()
            .filter(|f| f.family == "criterion-labels")
            .collect()
    }

    #[test]
    fn a_labelled_spec_with_a_current_counter_is_clean() {
        let tmp = tempdir().unwrap();
        seed_with_labels(
            tmp.path(),
            "in-progress",
            Some("3"),
            "- [x] AC1: First.\n- [ ] AC2: Second.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(label_findings(&result).is_empty(), "{:?}", result.findings);
        assert!(result.skipped.is_empty(), "{:?}", result.skipped);
    }

    #[test]
    fn a_duplicate_label_is_reported_once_per_label() {
        // Three occurrences of AC2, one finding: the defect is the label,
        // not each line carrying it.
        let tmp = tempdir().unwrap();
        seed_with_labels(
            tmp.path(),
            "in-progress",
            Some("9"),
            "- [x] AC1: First.\n- [ ] AC2: Second.\n- [ ] AC2: Third.\n- [ ] AC2: Fourth.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let found = label_findings(&result);
        assert_eq!(found.len(), 1, "{:?}", result.findings);
        assert_eq!(found[0].severity, AnalyzeSeverity::Advisory);
        assert_eq!(found[0].path, "specs/042-demo/spec.md");
        assert!(found[0].message.contains("duplicate"), "{:?}", found[0]);
        assert!(found[0].message.contains("AC2"), "{:?}", found[0]);
    }

    #[test]
    fn a_duplicate_is_reported_at_every_status() {
        // A label is an identifier, not a contract about the delivered
        // system, so — unlike criterion-path-existence — a draft is as wrong
        // to duplicate one in as a done spec.
        let tmp = tempdir().unwrap();
        seed_with_labels(
            tmp.path(),
            "draft",
            Some("4"),
            "- [ ] AC3: First.\n- [ ] AC3: Second.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(label_findings(&result).len(), 1, "{:?}", result.findings);
    }

    #[test]
    fn a_counter_at_or_below_the_body_maximum_is_reported() {
        // The retirement mechanism failing: the next assignment would hand
        // AC3 to a second requirement.
        let tmp = tempdir().unwrap();
        seed_with_labels(
            tmp.path(),
            "in-progress",
            Some("3"),
            "- [x] AC1: First.\n- [x] AC2: Second.\n- [ ] AC3: Third.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let found = label_findings(&result);
        assert_eq!(found.len(), 1, "{:?}", result.findings);
        assert!(
            found[0].message.contains("next-criterion 3"),
            "{:?}",
            found[0]
        );
        assert!(found[0].message.contains("AC3"), "{:?}", found[0]);
    }

    #[test]
    fn a_counter_above_the_body_maximum_is_clean_across_a_gap() {
        // Gaps are legal and mean retired labels — AC2 missing is not a
        // defect, and the counter still exceeds every label present.
        let tmp = tempdir().unwrap();
        seed_with_labels(
            tmp.path(),
            "in-progress",
            Some("42"),
            "- [x] AC1: First.\n- [ ] AC7: Seventh.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(label_findings(&result).is_empty(), "{:?}", result.findings);
    }

    #[test]
    fn a_malformed_counter_is_reported() {
        let tmp = tempdir().unwrap();
        seed_with_labels(
            tmp.path(),
            "in-progress",
            Some("zero"),
            "- [x] AC1: First.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let found = label_findings(&result);
        assert_eq!(found.len(), 1, "{:?}", result.findings);
        assert!(found[0].message.contains("`zero`"), "{:?}", found[0]);
    }

    #[test]
    fn a_counter_below_one_is_reported() {
        let tmp = tempdir().unwrap();
        seed_with_labels(tmp.path(), "in-progress", Some("0"), "- [x] AC1: First.\n");
        let result = run(&args(), tmp.path()).unwrap();
        let found = label_findings(&result);
        assert_eq!(found.len(), 1, "{:?}", result.findings);
        assert!(
            found[0].message.contains("not a positive integer"),
            "{:?}",
            found[0]
        );
    }

    #[test]
    fn an_unlabelled_criterion_is_reported_in_a_labelled_spec() {
        let tmp = tempdir().unwrap();
        seed_with_labels(
            tmp.path(),
            "in-progress",
            Some("3"),
            "- [x] AC1: First.\n- [ ] Typed by hand.\n- [ ] AC2: Third.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        let found = label_findings(&result);
        assert_eq!(found.len(), 1, "{:?}", result.findings);
        assert!(
            found[0].message.contains("acceptance criterion 1"),
            "{:?}",
            found[0]
        );
        assert!(
            found[0].message.contains("Typed by hand."),
            "{:?}",
            found[0]
        );
    }

    #[test]
    fn a_spec_that_has_never_been_labelled_is_clean() {
        // 013's edge case: an absent next-criterion means "no labels
        // assigned yet", not a defect. The corpus backfill is what makes the
        // unlabelled check universal — not a per-spec grandfather date.
        let tmp = tempdir().unwrap();
        seed_with_labels(
            tmp.path(),
            "in-progress",
            None,
            "- [x] First.\n- [ ] Second.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(label_findings(&result).is_empty(), "{:?}", result.findings);
    }

    #[test]
    fn the_family_never_records_a_skipped_target() {
        // Its whole subject is the spec's own frontmatter and criteria list,
        // both already parsed — there is no target it can fail to examine,
        // so a finding-producing run still skips nothing.
        let tmp = tempdir().unwrap();
        seed_with_labels(
            tmp.path(),
            "in-progress",
            Some("1"),
            "- [x] AC1: First.\n- [ ] AC1: Second.\n- [ ] Unlabelled.\n",
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(label_findings(&result).len(), 3, "{:?}", result.findings);
        assert!(
            !result
                .skipped
                .iter()
                .any(|s| s.family == "criterion-labels"),
            "{:?}",
            result.skipped
        );
    }
}
