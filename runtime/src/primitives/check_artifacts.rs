//! `check-artifacts` — the residual deterministic check families from
//! `/ductus:analyze`'s markdown-only reference, mechanized for one feature.
//!
//! Owns nine families (spec 022, scenarios analyze-artifact-checks,
//! scenario-open-question-signal, link-adjacent-drift-family,
//! criterion-path-existence-family, and criterion-label-assignment). Each
//! family MIRRORS
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
//!   that task is still pending*. Never flags a scenario under a `done`
//!   spec (a done feature's tasks may have been pruned), and never requires
//!   a pruned spent task to persist (constitution §tasks-phase — `tasks.md`
//!   is ephemeral; see [`pruning_evidence`] for the documented heuristic).
//! - **review-state-drift** (blocking) — reference §"Review state drift
//!   (blocking)": a `done` spec with
//!   `review.last-run` unset, or `review.blocking: true`, drifted. The
//!   grandfather rule applies: a `done` spec with no `review:` block at
//!   all predates `/ductus:review` and is exempt.
//! - **analyze-state-drift** (blocking) — the counterpart to
//!   review-state-drift, and it exists because there was no counterpart: a
//!   `done` spec with `analyze.last-run` unset, or `analyze.blocking: true`,
//!   drifted. Advisory findings are recorded in the block and deliberately
//!   **not** checked here — analyze's advisory tier is made of checks
//!   introduced advisory with their own published promotion criteria, and
//!   gating on them would promote all of them at once. The grandfather rule
//!   applies, and its population is bounded rather than open-ended: a `done`
//!   spec with no `analyze:` block predates the record. `/audit` Family 37
//!   reports exactly that set, so the exemption is countable and shrinking
//!   rather than a silent permanent hiding place — the objection 046 raised
//!   against exemptions, answered by making this one visible instead of
//!   pretending a backfill were possible. It is not: an analyze record
//!   asserts a run happened, and writing one for a run that did not is the
//!   fabrication this whole family exists to prevent.
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
    MarkdownBlock, PrimitiveError, Result, inline_code_spans, label_criteria, list_scenario_files,
    read_spec, read_tasks, read_text, rel_path, scenario_name_cmp, split_blocks,
};
use crate::schema::paths;
use crate::schema::primitives::{
    ArtifactFinding, CheckArtifactsArgs, CheckArtifactsResult, Frontmatter, ReadSpecArgs,
    ReadSpecResult, ReadTasksArgs, SkippedTarget, Task,
};
use crate::schema::status::COMPATIBLE_STATUSES;

/// Execute the `check-artifacts` primitive against the given repo root.
///
/// # Errors
///
/// Returns [`PrimitiveError::FeatureNotFound`] when the feature directory
/// is absent, [`PrimitiveError::MissingFrontmatter`] /
/// [`PrimitiveError::Yaml`] when `spec.md` has no parseable frontmatter
/// (the frontmatter-schema family is `validate-frontmatter`'s job — this
/// primitive needs a readable `status` to classify tiers at all), or
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
    // FeatureNotFound / MissingFrontmatter / Yaml / Io variants on the same
    // file, so the documented error contract above is unchanged.
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
    check_review_drift(&mut findings, frontmatter, &status, &spec_path, repo);
    check_analyze_drift(&mut findings, frontmatter, &status, &spec_path, repo);

    let mut skipped: Vec<SkippedTarget> = Vec::new();
    check_scenario_open_questions(
        &mut findings,
        &mut skipped,
        &feature_dir,
        &status,
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
                severity: "blocking".into(),
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
                    severity: "blocking".into(),
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
                severity: "blocking".into(),
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
            severity: "advisory".into(),
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
    let rel = format!("{root}/{feature}/tasks.md");
    let repository = git2::Repository::discover(repo).ok()?;
    let mut walk = repository.revwalk().ok()?;
    walk.push_head().ok()?;
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for oid in walk.flatten() {
        if seen.len() == slugs.len() {
            break;
        }
        let Ok(tree) = repository.find_commit(oid).and_then(|c| c.tree()) else {
            continue;
        };
        let Ok(entry) = tree.get_path(Path::new(&rel)) else {
            continue;
        };
        let Ok(blob) = repository.find_blob(entry.id()) else {
            continue;
        };
        let Ok(text) = std::str::from_utf8(blob.content()) else {
            continue;
        };
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

/// (d2) Analyze-state drift — the counterpart to review-state drift, added
/// because there was no counterpart at all.
///
/// For a `done` spec, `analyze.last-run` must be set and `analyze.blocking`
/// must be `false`.
///
/// **Grandfather rule**, and it is the honest choice here rather than the
/// convenient one. A `done` spec with no `analyze:` block predates the record
/// and is exempt. 046 refused an exemption for scenario questions on the
/// grounds that a sanctioned hiding place is worse than the gap it papers
/// over, and the criterion-label check backfilled the corpus instead — so the
/// precedent runs against exempting. It does not apply, and the difference is
/// what a backfill would have to assert. A criterion label is derivable from
/// the artifact: the backfill computed a value that was already true. An
/// analyze record asserts *that a run happened*, which is not derivable from
/// anything on disk, so backfilling it would mean writing a claim nobody
/// verified into the field a later gate trusts — the precise failure this
/// family exists to catch, committed by the family itself.
///
/// The exemption is made bounded instead of silent: `/audit` Family 37
/// reports every `done` spec carrying a `review:` block and no `analyze:`
/// one, which is exactly the grandfathered population. It is countable, it
/// shrinks as specs are re-analyzed, and it can never grow — the gate
/// (`check-review-gate`) has no grandfather clause, so nothing new can enter
/// the set.
///
/// **Advisory findings are not checked**, unlike review-state drift's
/// treatment of an outstanding SHOULD. See `AnalyzeBlock::advisory`: analyze's
/// advisory tier is made of checks introduced advisory with published
/// promotion criteria, and gating on them here would promote every one past
/// the criteria it declares.
fn check_analyze_drift(
    findings: &mut Vec<ArtifactFinding>,
    frontmatter: &Frontmatter,
    status: &str,
    spec_path: &Path,
    repo: &Path,
) {
    if status != "done" {
        return;
    }
    let Some(analyze) = &frontmatter.analyze else {
        return; // grandfathered: no analyze block at all
    };
    let spec_rel = rel_path(spec_path, repo);
    if analyze.last_run.is_none() {
        findings.push(ArtifactFinding {
            family: "analyze-state-drift".into(),
            severity: "blocking".into(),
            message: "analyze drift: done spec missing analysis (analyze.last-run unset) — \
                      run the analyze command"
                .into(),
            path: spec_rel.clone(),
        });
    }
    if analyze.blocking {
        findings.push(ArtifactFinding {
            family: "analyze-state-drift".into(),
            severity: "blocking".into(),
            message: format!(
                "analyze drift: done spec has {} hard-fail and {} blocking analyze finding(s) \
                 (analyze.blocking true) — resolve them and re-run the analyze command",
                analyze.hard_fail, analyze.blocking_findings
            ),
            path: spec_rel,
        });
    }
}

/// (d) Review-state drift — reference §"Review state drift (blocking)":
/// for a `done` spec, `review.last-run` must be set and `review.blocking`
/// must be `false`. Grandfather rule: a `done` spec with **no** `review:`
/// block at all predates `/ductus:review` and is exempt. Specs not at `done`
/// are silently exempt (the block populates lazily on first review).
fn check_review_drift(
    findings: &mut Vec<ArtifactFinding>,
    frontmatter: &Frontmatter,
    status: &str,
    spec_path: &Path,
    repo: &Path,
) {
    if status != "done" {
        return;
    }
    let Some(review) = &frontmatter.review else {
        return; // grandfathered: no review block at all
    };
    let spec_rel = rel_path(spec_path, repo);
    if review.last_run.is_none() {
        findings.push(ArtifactFinding {
            family: "review-state-drift".into(),
            severity: "blocking".into(),
            message: "review drift: done spec missing review (review.last-run unset) — \
                      run the review command"
                .into(),
            path: spec_rel.clone(),
        });
    }
    if review.blocking {
        findings.push(ArtifactFinding {
            family: "review-state-drift".into(),
            severity: "blocking".into(),
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
    // 31 compares the frontmatter `review:` block against `review.md`, but
    // one `/{project}:review` run writes both. A fix applied *during* a pass
    // that never re-runs leaves the two consistently stale and that family
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
            severity: "blocking".into(),
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
/// Reads through `read-spec`'s collector so this finding, the
/// `check-review-gate` block, and the count surfaced to the user can never
/// disagree.
fn check_scenario_open_questions(
    findings: &mut Vec<ArtifactFinding>,
    skipped: &mut Vec<SkippedTarget>,
    feature_dir: &Path,
    status: &str,
    spec_path: &Path,
    repo: &Path,
) {
    let scan = read_spec::collect_scenario_open_questions(feature_dir);
    // A scenario that could not be read is reported as a skipped target, not
    // as a finding and not as silence. It is not a defect — nothing can be
    // proven about a file that will not parse — but a zero-finding result
    // over a subject the family never read would be indistinguishable from a
    // fully-examined clean one (QUAL-CLAIM-001).
    for slug in &scan.unreadable {
        skipped.push(SkippedTarget {
            family: "scenario-open-questions".into(),
            reason: "artifact-unreadable".into(),
            path: rel_path(
                &feature_dir.join("scenarios").join(format!("{slug}.md")),
                repo,
            ),
        });
    }
    let questions = scan.questions;
    if questions.is_empty() {
        return;
    }
    let scenarios = read_spec::scenario_names(&questions);
    let severity = if status == "done" {
        "blocking"
    } else {
        "advisory"
    };
    findings.push(ArtifactFinding {
        family: "scenario-open-questions".into(),
        severity: severity.into(),
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
            record_skip(skipped, "artifact-unreadable", &citing);
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
                    Err(reason) => record_skip(skipped, reason, &target_rel),
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
            record_skip(skipped, "no-readable-state", target_rel);
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
        severity: "advisory".into(),
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
/// target reached twice by this family collapses to one entry.
fn record_skip(skipped: &mut Vec<SkippedTarget>, reason: &str, path: &str) {
    if skipped
        .iter()
        .any(|s| s.family == "link-adjacent-drift" && s.reason == reason && s.path == path)
    {
        return;
    }
    skipped.push(SkippedTarget {
        family: "link-adjacent-drift".into(),
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
                let entry = SkippedTarget {
                    family: "criterion-path-existence".into(),
                    reason: "not-a-live-claim".into(),
                    path: candidate.clone(),
                };
                if !skipped.contains(&entry) {
                    skipped.push(entry);
                }
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
                let entry = SkippedTarget {
                    family: "criterion-path-existence".into(),
                    reason: "root-absent".into(),
                    path: candidate.clone(),
                };
                if !skipped.contains(&entry) {
                    skipped.push(entry);
                }
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
                let entry = SkippedTarget {
                    family: "criterion-path-existence".into(),
                    reason: "ships-to-adopter".into(),
                    path: candidate.clone(),
                };
                if !skipped.contains(&entry) {
                    skipped.push(entry);
                }
                continue;
            }
            findings.push(ArtifactFinding {
                family: "criterion-path-existence".into(),
                severity: "advisory".into(),
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
/// adopter's checkout, derived from the **Shared Files** manifest tables in
/// `framework/bootstrap/ductus.md` — the canonical registry of what lands
/// where, per the constitution's canonical-sources map.
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
    let mut out = BTreeSet::new();
    let Ok(text) = std::fs::read_to_string(repo.join("framework/bootstrap/ductus.md")) else {
        return out;
    };
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

/// `true` when `candidate` is one of the adopter destinations, or is a
/// directory containing one. The directory case is what lets a criterion name
/// `specs/templates/` and match the six per-file template rows beneath it.
pub(crate) fn ships_to_adopter(destinations: &BTreeSet<String>, candidate: &str) -> bool {
    if destinations.contains(candidate) {
        return true;
    }
    let prefix = format!("{candidate}/");
    destinations.iter().any(|d| d.starts_with(&prefix))
}

/// `true` when the criterion claims its paths are present — i.e. it carries
/// none of [`NON_ASSERTION_MARKERS`]. Matching is ASCII-case-insensitive over
/// the whole criterion, code spans included: a marker is prose, and a criterion
/// that carries one anywhere is describing a transition throughout.
fn is_live_assertion(text: &str) -> bool {
    let lowered = text.to_ascii_lowercase();
    !NON_ASSERTION_MARKERS
        .iter()
        .any(|marker| lowered.contains(marker))
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
                severity: "advisory".into(),
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
            severity: "advisory".into(),
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
                    severity: "advisory".into(),
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
                severity: "advisory".into(),
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

    fn spec(status: &str, review: Option<&str>) -> String {
        spec_with_analyze(status, review, None)
    }

    /// `spec` plus an explicit `analyze:` block. `None` reproduces the
    /// grandfathered shape — a `done` spec written before the record existed
    /// — which the drift family must leave alone.
    fn spec_with_analyze(status: &str, review: Option<&str>, analyze: Option<&str>) -> String {
        let review_block = review
            .map(|r| format!("review:\n{r}\n"))
            .unwrap_or_default();
        let analyze_block = analyze
            .map(|a| format!("analyze:\n{a}\n"))
            .unwrap_or_default();
        format!(
            "---\nstatus: {status}\ndependencies: []\n{review_block}{analyze_block}---\n\n# Demo\n"
        )
    }

    const CLEAN_ANALYZE: &str = "  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 2\n  unexamined: 1\n  blocking: false";

    /// The ordinary passing case, and the reason `CLEAN_ANALYZE` exists: a
    /// `done` spec carrying a completed, non-blocking analysis is not drift
    /// even though it records advisory findings and an unexamined target.
    #[test]
    fn a_done_spec_with_a_clean_analyze_block_is_not_drift() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/spec.md"),
            &spec_with_analyze("done", Some(CLEAN_REVIEW), Some(CLEAN_ANALYZE)),
        );
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/plan.md"),
            "# Demo Plan\n",
        );
        write(tmp.path(), &format!("specs/{FEATURE}/tasks.md"), GOOD_TASKS);
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            !families(&result)
                .iter()
                .any(|(f, _)| *f == "analyze-state-drift"),
            "{:?}",
            families(&result)
        );
    }

    /// The grandfathered population: a `done` spec with no `analyze:` block
    /// at all predates the record and is exempt. It is exempt rather than
    /// backfilled because a backfill would have to assert a run happened, and
    /// nothing on disk can substantiate that — the one claim this family must
    /// never manufacture. `/audit` Family 37 counts this set so the exemption
    /// is visible and bounded instead of silent.
    #[test]
    fn a_done_spec_with_no_analyze_block_is_grandfathered() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/spec.md"),
            &spec_with_analyze("done", Some(CLEAN_REVIEW), None),
        );
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/plan.md"),
            "# Demo Plan\n",
        );
        write(tmp.path(), &format!("specs/{FEATURE}/tasks.md"), GOOD_TASKS);
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            !families(&result)
                .iter()
                .any(|(f, _)| *f == "analyze-state-drift"),
            "{:?}",
            families(&result)
        );
    }

    #[test]
    fn a_done_spec_with_a_null_analyze_last_run_is_drift() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/spec.md"),
            &spec_with_analyze(
                "done",
                Some(CLEAN_REVIEW),
                Some("  last-run: null\n  blocking: false"),
            ),
        );
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/plan.md"),
            "# Demo Plan\n",
        );
        write(tmp.path(), &format!("specs/{FEATURE}/tasks.md"), GOOD_TASKS);
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            families(&result)
                .iter()
                .any(|(f, sev)| *f == "analyze-state-drift" && *sev == "blocking")
        );
    }

    #[test]
    fn a_done_spec_with_blocking_analyze_findings_is_drift() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/spec.md"),
            &spec_with_analyze(
                "done",
                Some(CLEAN_REVIEW),
                Some(
                    "  last-run: 2026-07-10T00:00:00Z\n  hard-fail: 1\n  blocking-findings: 2\n  blocking: true",
                ),
            ),
        );
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/plan.md"),
            "# Demo Plan\n",
        );
        write(tmp.path(), &format!("specs/{FEATURE}/tasks.md"), GOOD_TASKS);
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            families(&result)
                .iter()
                .any(|(f, sev)| *f == "analyze-state-drift" && *sev == "blocking")
        );
    }

    /// Advisory findings and unexamined targets are recorded in the block and
    /// deliberately never gate — the asymmetry with review-state drift's
    /// treatment of an outstanding SHOULD.
    #[test]
    fn advisory_and_unexamined_counts_are_not_analyze_drift() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/spec.md"),
            &spec_with_analyze(
                "done",
                Some(CLEAN_REVIEW),
                Some(
                    "  last-run: 2026-07-10T00:00:00Z\n  advisory: 9\n  unexamined: 5\n  blocking: false",
                ),
            ),
        );
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/plan.md"),
            "# Demo Plan\n",
        );
        write(tmp.path(), &format!("specs/{FEATURE}/tasks.md"), GOOD_TASKS);
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            !families(&result)
                .iter()
                .any(|(f, _)| *f == "analyze-state-drift")
        );
    }

    /// A spec below `done` is exempt: the block populates lazily on the first
    /// analyze run, exactly as `review:` does.
    #[test]
    fn an_in_progress_spec_without_an_analyze_block_is_not_drift() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/spec.md"),
            &spec_with_analyze("in-progress", Some(CLEAN_REVIEW), None),
        );
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/plan.md"),
            "# Demo Plan\n",
        );
        write(tmp.path(), &format!("specs/{FEATURE}/tasks.md"), GOOD_TASKS);
        let result = run(&args(), tmp.path()).unwrap();
        assert!(
            !families(&result)
                .iter()
                .any(|(f, _)| *f == "analyze-state-drift")
        );
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
        write(tmp.path(), "specs/042-demo/spec.md", &spec("draft", None));
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.clean, "{:?}", result.findings);
        assert_eq!(result.status, "draft");
        assert_eq!(result.path, "specs/042-demo/spec.md");
    }

    #[test]
    fn planned_spec_missing_plan_and_tasks_yields_blocking_findings() {
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/042-demo/spec.md", &spec("planned", None));
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
        write(tmp.path(), "specs/042-demo/spec.md", &spec("planned", None));
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
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            &spec("in-progress", None),
        );
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        write(tmp.path(), "specs/042-demo/tasks.md", GOOD_TASKS);
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.clean, "{:?}", result.findings);
    }

    #[test]
    fn non_increasing_numbering_yields_blocking_finding() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            &spec("in-progress", None),
        );
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
        assert_eq!(numbering[0].severity, "blocking");
        assert_eq!(numbering[0].path, "specs/042-demo/tasks.md");
        assert!(numbering[0].message.contains("task 1 follows task 2"));
    }

    #[test]
    fn missing_done_when_yields_blocking_finding() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            &spec("in-progress", None),
        );
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
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            &spec("clarified", None),
        );
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.clean, "{:?}", result.findings);
    }

    // --- scenario consistency ------------------------------------------------------

    #[test]
    fn unmapped_scenario_yields_advisory_finding() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            &spec("in-progress", None),
        );
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
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            &spec("in-progress", None),
        );
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
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            &spec("in-progress", None),
        );
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
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            &spec("in-progress", None),
        );
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
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            &spec("in-progress", None),
        );
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

    #[test]
    fn done_spec_scenarios_are_never_flagged() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            &spec(
                "done",
                Some("  last-run: 2026-07-01T00:00:00Z\n  blocking: false"),
            ),
        );
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
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            &spec("done", Some("  blocking: false")),
        );
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
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            &spec(
                "done",
                Some("  last-run: 2026-07-01T00:00:00Z\n  blocking: true\n  must-violations: 2"),
            ),
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
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            &spec(
                "done",
                Some(
                    "  last-run: 2026-07-01T00:00:00Z\n  blocking: false\n  must-violations: 0\n  should-violations: 1",
                ),
            ),
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
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            &spec(
                "done",
                Some(
                    "  last-run: 2026-07-01T00:00:00Z\n  blocking: false\n  must-violations: 0\n  should-violations: 0",
                ),
            ),
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
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            &spec(
                "in-progress",
                Some(
                    "  last-run: 2026-07-01T00:00:00Z\n  blocking: false\n  must-violations: 0\n  should-violations: 3",
                ),
            ),
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
        write(tmp.path(), "specs/042-demo/spec.md", &spec("done", None));
        write(tmp.path(), "specs/042-demo/plan.md", "# Plan\n");
        write(tmp.path(), "specs/042-demo/tasks.md", GOOD_TASKS);
        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.clean, "{:?}", result.findings);
    }

    #[test]
    fn non_done_spec_with_empty_review_block_is_exempt() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            &spec("in-progress", Some("  blocking: false")),
        );
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
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            &spec("done", Some("  blocking: true")),
        );
        // done + no plan.md/tasks.md + review drift (last-run unset AND
        // blocking true) → completeness ×2, then review drift ×2. The
        // scenario family is skipped at done.
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
        write(
            repo,
            &format!("specs/{FEATURE}/spec.md"),
            &spec(status, Some(CLEAN_REVIEW)),
        );
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
            &spec("done", None),
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
        // the same rule. Nothing can be proven about a file that will not
        // parse, and an unknown is never escalated into a defect — least
        // of all a blocking one on a `done` spec (spec 046).
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            &format!("specs/{FEATURE}/spec.md"),
            &spec("done", Some(CLEAN_REVIEW)),
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
            &spec("in-progress", Some(CLEAN_REVIEW)),
        );
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
        assert_eq!(found[0].severity, "advisory");
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
                spec("in-progress", Some(CLEAN_REVIEW))
            ),
        );
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
                .all(|f| f.severity == "advisory"),
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
            &format!(
                "{}\n## Acceptance Criteria\n\n{criteria}",
                spec(status, Some(CLEAN_REVIEW))
            ),
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
        assert_eq!(found[0].severity, "advisory");
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
                spec("done", Some(CLEAN_REVIEW))
            ),
        );
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
            &spec("done", Some(CLEAN_REVIEW)),
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
        assert_eq!(found[0].severity, "advisory");
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
