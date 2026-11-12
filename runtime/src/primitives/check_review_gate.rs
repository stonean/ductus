//! `check-review-gate` — evaluate `/ductus:implement`'s pre-done review gate. Its checks
//! and their order are `schema::descriptions::CHECK_REVIEW_GATE`.
//!
//! The deterministic surface behind the completion gate (spec 022, scenario
//! coverage-expansion-primitives), which the host previously walked by hand on every
//! completion attempt. A `blocked: …` message substitutes the adopter's `[host] project`
//! command namespace into its `/{project}:review` references.
//!
//! A **passing** gate can still carry `guidance`: either record may be *undeterminable*
//! — carrying no digest, so nothing on disk says what its run examined — and saying so
//! is not the same as saying it is current. That notice never changes `passed`; see
//! [`passing_notices`].

use std::collections::BTreeSet;
use std::path::Path;

use crate::host::Host;
use crate::primitives::read_tasks::describe_unchecked;
use crate::primitives::spec_links;
use crate::primitives::{
    PrimitiveError, Result, lint_markdown, read_spec, read_text, split_frontmatter,
};
use crate::schema::paths;
use crate::schema::primitives::{
    AnalyzeBlock, CheckReviewGateArgs, CheckReviewGateResult, CrossSpecImpactEntry,
    CrossSpecImpactState, Frontmatter, LintMarkdownArgs, ListsForm, MarkdownViolation,
    RecordFreshness, ReviewGateBlock,
};

/// Execute the `check-review-gate` primitive against the given repo root.
///
/// # Errors
///
/// Returns [`PrimitiveError::InvalidPath`] when `feature` is empty,
/// absolute, or carries a parent-directory component,
/// or when the feature directory, or a link inside it, resolves outside the
/// project,
/// [`PrimitiveError::FeatureNotFound`] when the feature directory does
/// not exist, [`PrimitiveError::Io`] when `spec.md` is unreadable or
/// `npx` cannot be spawned, or [`PrimitiveError::Yaml`] for a malformed
/// frontmatter block, and what `bounded_lists::place` returns:
/// [`PrimitiveError::InvalidReadSize`] for a malformed `[artifacts]
/// read-size-bytes`, or a list file that cannot be written. Every gate
/// verdict — including every block reason [`ReviewGateBlock`] names — is a
/// domain outcome in the result.
///
/// The result is inline while it fits `[artifacts] read-size-bytes`, and past
/// it carries `violations`, `cross-spec-impact` and `analyze-stale-paths` in
/// the file it names and only their counts inline (`bounded_lists::place`; spec 080, AC7).
pub fn run(args: &CheckReviewGateArgs, repo: &Path) -> Result<CheckReviewGateResult> {
    bounded(args, repo, lint_markdown::compute)
}

/// [`run`] with the lint injected, so a test bounds a verdict without `npx`.
fn bounded(
    args: &CheckReviewGateArgs,
    repo: &Path,
    lint: impl FnOnce(&LintMarkdownArgs, &Path) -> Result<lint_markdown::Lint>,
) -> Result<CheckReviewGateResult> {
    let verdict = run_with_lint(args, repo, lint)?;
    let lists = serde_json::json!({
        "violations": verdict.violations,
        "cross-spec-impact": verdict.cross_spec_impact,
        "analyze-stale-paths": verdict.analyze_stale_paths.as_deref().unwrap_or_default(),
    });
    let impact = verdict.cross_spec_impact;
    let stale = verdict
        .analyze_stale_paths
        .map(Vec::from)
        .unwrap_or_default();
    let mut result = CheckReviewGateResult {
        passed: verdict.passed,
        blocked_by: verdict.blocked_by,
        message: verdict.message,
        guidance: verdict.guidance,
        violations_count: verdict.violations.len(),
        violations: Some(verdict.violations),
        cross_spec_impact_count: impact.len(),
        cross_spec_impact: (!impact.is_empty()).then_some(impact),
        analyze_stale_paths_count: stale.len(),
        analyze_stale_paths: (!stale.is_empty()).then_some(stale),
        form: ListsForm::Inline,
        lists_file: None,
    };
    if let Some(file) =
        super::bounded_lists::place(repo, "check-review-gate", &args.feature, &result, &lists)?
    {
        result.form = ListsForm::File;
        result.violations = None;
        result.cross_spec_impact = None;
        result.analyze_stale_paths = None;
        result.lists_file = Some(file);
    }
    Ok(result)
}

/// A gate verdict, whole: what `check-review-gate` returns, before its result
/// is bounded. Each field is [`CheckReviewGateResult`]'s of the same name;
/// an empty `cross_spec_impact` or `analyze_stale_paths` is the result's absent
/// one.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Verdict {
    pub(crate) passed: bool,
    pub(crate) blocked_by: Option<ReviewGateBlock>,
    pub(crate) message: Option<String>,
    pub(crate) guidance: Option<String>,
    pub(crate) violations: Vec<MarkdownViolation>,
    pub(crate) cross_spec_impact: Vec<CrossSpecImpactEntry>,
    /// Boxed so the verdict, which many checks return as an `Err`, stays small.
    pub(crate) analyze_stale_paths: Option<Box<[String]>>,
}

/// Implementation seam that lets unit tests inject a canned lint outcome
/// instead of spawning `npx markdownlint-cli2`. The MCP and CLI surfaces
/// both call [`run`], which forwards the real `lint-markdown` primitive.
pub(crate) fn run_with_lint(
    args: &CheckReviewGateArgs,
    repo: &Path,
    lint: impl FnOnce(&LintMarkdownArgs, &Path) -> Result<lint_markdown::Lint>,
) -> Result<Verdict> {
    let root = paths::Paths::load(repo)?.specs_root;
    super::validate_feature_argument(repo, &root, &args.feature)?;
    let feature_dir = repo.join(&root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root,
            feature: args.feature.clone(),
        });
    }
    let rel_dir = format!("{root}/{}", args.feature);

    // The spec's frontmatter, read once for every check that consults it.
    // Read **before** the lint because gate check 0 below needs the status,
    // and the status decides whether any other check is worth running.
    //
    // One consequence, stated rather than left to be discovered: a spec whose
    // frontmatter will not parse now surfaces that error ahead of any lint
    // violation it also has. That is the more fundamental defect of the two —
    // a spec whose frontmatter cannot be read cannot be gated at all — but the
    // order did change.
    let spec_path = feature_dir.join("spec.md");
    let content = read_text(&spec_path)?;
    let (fm_text, _body) = split_frontmatter(&content, &spec_path)?;
    let frontmatter: Frontmatter =
        serde_norway::from_str(fm_text).map_err(|source| PrimitiveError::Yaml {
            path: spec_path.clone(),
            source,
        })?;
    let project = Host::load(repo)?.project;

    // Gate check 0: the spec is not already `done`.
    //
    // Every check below presumes a pending transition. On a spec that has
    // already made it, they answer a question nobody asked — and the analyze
    // freshness check answers it *wrongly*, because the completing
    // `set-status` rewrote `spec.md`, which is one of its subjects. Ordered
    // ahead of the lint so a completed spec is never held against a violation
    // introduced long after it closed.
    if let Some(done) = already_done_block(&frontmatter.status, &project) {
        return Ok(done);
    }

    // Gate check 1: every markdown file in the feature directory passes
    // markdownlint.
    if let Some(blocked) = markdown_lint_block(&rel_dir, repo, lint)? {
        return Ok(blocked);
    }

    // Gate check 2: no scenario under this feature carries an unresolved
    // open question (spec 046).
    if let Some(blocked) = scenario_question_block(&feature_dir, repo, &args.feature)? {
        return Ok(blocked);
    }
    // Then no box is left unchecked: a spec with work owed is no candidate for `done`,
    // whatever its review says, and `/{project}:implement`'s count is only a step a host
    // follows, which sees no box outside a numbered task (spec 079).
    if let Some(blocked) = unchecked_task_block(&feature_dir, &rel_dir)? {
        return Ok(blocked);
    }

    // No check reads `folds-into`. A pending fold never holds `done`: the
    // staging spec completes its own pipeline like any other, and fold-back
    // consolidates its content upstream afterwards (spec 051 scenario
    // `a-pending-fold-never-holds-done`). The owed fold is carried past
    // `done` by `check-unfolded-specs` and `/{project}:status`'s
    // `(fold pending)`, not by this gate.

    // Gate check 3: every declared `cross-spec-impact` entry is discharged.
    //
    // Ahead of the `review:` block because a spec carrying an obligation
    // nobody has discharged is not a candidate for `done`, so whether its
    // review is fresh does not yet matter.
    let cross_spec =
        cross_spec_impact_states(&frontmatter.cross_spec_impact, &args.feature, repo, &root);
    if let Some(blocked) = cross_spec_impact_block(&cross_spec, &args.feature, &project) {
        return Ok(blocked);
    }

    // Gate checks 4 and 5: the review record, read from `review.md` — the
    // artifact that owns it (spec 057).
    //
    // Three inputs, not two. An absent artifact means no review ran; an
    // artifact that will not parse means the gate cannot tell. Both block, and
    // they must not share a message: "run /review" is wrong advice for a spec
    // whose review may well have happened and whose record is merely damaged.
    let review = match crate::primitives::load_review_record(&feature_dir) {
        crate::primitives::RecordLoad::Present(review) if review.last_run.is_some() => review,
        crate::primitives::RecordLoad::Unreadable(reason) => {
            return Ok(unreadable_record_block(
                &rel_dir,
                &project,
                RecordKind::Review,
                &reason,
            ));
        }
        // Absent artifact, or a record carrying a null `last-run`: the spec has
        // never completed a review.
        _ => return Ok(never_reviewed_block(&project)),
    };

    if review.blocking {
        return Ok(must_violations_block(
            review.must_violations,
            &feature_dir,
            &rel_dir,
            &project,
        ));
    }

    // Gate check 6, whether the review's digest still described the spec's scenarios
    // and data model, is retired: a review is owed for code, which check 10 reads, and a
    // changed contract owes an analysis, which check 13 reads (spec 081).

    // Gate checks 7 to 10: the rest of what only a review clears, ahead of every
    // analyze check. `review.md` is one of an analysis's subjects, so an analysis run
    // before a review the spec owes is stale when that review writes (spec 079).
    let owed_unknown = match review_half_block(
        &review,
        &args.feature,
        repo,
        &feature_dir,
        &rel_dir,
        &project,
    ) {
        Ok(unknown) => unknown,
        Err(blocked) => return Ok(blocked),
    };

    // Gate checks 11 to 15: the analyze half. The freshness it computes is handed back
    // rather than recomputed for the passing verdict's notice below — the comparison
    // reads and hashes every `.md` under the feature, so doing it twice is real work for
    // one answer.
    let freshness = match analyze_half_block(repo, &rel_dir, &feature_dir, &project) {
        Ok(freshness) => freshness,
        Err(blocked) => return Ok(blocked),
    };

    // The gate passes. Before saying so, name what it could not examine.
    //
    // The analyze staleness check reads the working tree, so an uncommitted
    // artifact is no blind spot. What remains unexaminable is an analysis record
    // that never wrote down what its run read: with no digest it cannot be
    // compared against anything, so the check does not fire at all. It does not
    // block — an undeterminable record clears on the next run — but a bare
    // `passed: true` over it would read as "examined and current" when nothing
    // was compared, which is `QUAL-CLAIM-001` applied to the gate itself. Naming
    // it is what keeps the silence of a clean verdict meaningful, and it is the
    // operator's to weigh at the transition rather than the walker's to drop.
    Ok(Verdict {
        passed: true,
        blocked_by: None,
        message: None,
        guidance: passing_notices(
            &freshness,
            review.not_re_read.is_some(),
            owed_unknown.as_deref(),
        ),
        violations: vec![],
        // Carried onto the pass, not just onto a block: a discharged
        // obligation is something this gate *examined*, and reporting it only
        // when it fails would leave "declared and proven" and "never declared"
        // rendering identically.
        cross_spec_impact: cross_spec,
        analyze_stale_paths: None,
    })
}

/// Gate checks 7 to 10: `review.md` with no `dispositions:` map, an outstanding
/// review finding, a fix-round loop left open or a round unread, and a review the
/// window owes. `Ok` holds why the window could not be read, when it could not: a gate
/// blocking on its own inability to check is one people route around, so that passes
/// and [`passing_notices`] says so.
fn review_half_block(
    review: &crate::schema::primitives::ReviewBlock,
    feature: &str,
    repo: &Path,
    feature_dir: &Path,
    rel_dir: &str,
    project: &str,
) -> std::result::Result<Option<String>, Verdict> {
    let gating = review
        .must_violations
        .saturating_add(review.should_violations);
    if let Some(blocked) = disposition_block(
        (crate::primitives::REVIEW_RECORD_FILE, "review"),
        review.dispositions,
        gating,
        feature_dir,
        rel_dir,
        project,
    ) {
        return Err(blocked);
    }
    if let Some(blocked) = fix_round_block(review, rel_dir, project) {
        return Err(blocked);
    }
    // Read from the record, so a window that cannot be read does not pass it.
    if !review.open_findings.is_empty() {
        return Err(review_owed_block(&review.open_findings, project));
    }
    let window = crate::schema::primitives::ComputeReviewScopeArgs {
        feature: feature.to_string(),
        since: None,
    };
    match crate::primitives::compute_review_scope::compute(&window, repo) {
        Ok(scope) if scope.review_owed => Err(review_owed_block(&scope.owed_by, project)),
        Ok(_) => Ok(None),
        Err(error) => Ok(Some(error.to_string())),
    }
}

/// Gate checks 11 to 15: `analysis.md` unreadable, absent, failing or stale, then with
/// no `dispositions:` map or an outstanding gating finding, after its record checks
/// because a record that is missing, failing, or stale has to be re-run before what it
/// says about its findings means anything.
fn analyze_half_block(
    repo: &Path,
    rel_dir: &str,
    feature_dir: &Path,
    project: &str,
) -> std::result::Result<RecordFreshness, Verdict> {
    let analyze_record = match crate::primitives::load_analyze_record(feature_dir) {
        crate::primitives::RecordLoad::Unreadable(reason) => {
            return Err(unreadable_record_block(
                rel_dir,
                project,
                RecordKind::Analyze,
                &reason,
            ));
        }
        crate::primitives::RecordLoad::Present(record) => Some(record),
        crate::primitives::RecordLoad::Absent => None,
    };
    // `analysis.md` is no subject of its own analysis, so a list damaged after the run
    // leaves the record current, and only this reads it. A hard fail to
    // `validate-frontmatter`, and `write-analysis` refuses to record over it.
    if analyze_record.is_some()
        && let Err(error) = crate::primitives::decisions::read_decisions(
            feature_dir,
            crate::primitives::ANALYSIS_RECORD_FILE,
        )
    {
        return Err(unreadable_decisions_block(rel_dir, &error.to_string()));
    }
    let freshness = analyze_gate_block(repo, rel_dir, analyze_record.as_ref(), project)?;
    if let Some(blocked) = disposition_block(
        (crate::primitives::ANALYSIS_RECORD_FILE, "analyze"),
        analyze_record
            .as_ref()
            .and_then(|record| record.dispositions),
        analyze_record.as_ref().map_or(0, |record| {
            record.hard_fail.saturating_add(record.blocking_findings)
        }),
        feature_dir,
        rel_dir,
        project,
    ) {
        return Err(blocked);
    }
    Ok(freshness)
}

/// Gate check 10: the spec's code changed since the last review, or since its reopen
/// when none has run in it (a changed scenario or data model owes an analysis, spec
/// 081). Its window moves only at the
/// commit holding a finished review's record, so a review just run clears this once
/// that record is committed.
fn review_owed_block(owed_by: &[String], project: &str) -> Verdict {
    Verdict {
        passed: false,
        blocked_by: Some(ReviewGateBlock::ReviewOwed),
        message: Some(format!(
            "blocked: a review is owed for what changed since the last one or what it left \
             open: {} — run /{project}:review and commit its record",
            render_paths(owed_by)
        )),
        guidance: Some(format!(
            "A review's window moves only at the commit holding its record, so a review that \
             has just finished clears this once that record is committed. A broken contract it \
             left open clears when a run finds it fixed, a person waives it, or a deferral to \
             another spec's work releases it. An uncommitted path \
             counts, whoever's it is: commit it, or isolate another session's work with `git \
             worktree`. Run /{project}:analyze after the review, since its record is one of an \
             analysis's subjects."
        )),
        violations: vec![],
        cross_spec_impact: vec![],
        analyze_stale_paths: None,
    }
}

/// Gate checks 7 and 8 for `review.md`, and 14 and 15 for `analysis.md`: the record
/// has no `dispositions:` map, then it counts an outstanding gating finding (spec 058).
///
/// **A record without the map predates dispositions, and absence is not
/// zero.** Before 058 a run's findings were captured to the inbox, which no
/// gate read, so reading a map-less record as "nothing undispositioned" would
/// pass exactly the case the change exists for: an old run whose findings sit
/// in the inbox unseen. That departs from the notice-only precedent for a
/// record predating a *digest*, deliberately — a digest-less record says only
/// that freshness is unknown, while a map-less one may be hiding owed work.
///
/// Then the gating count: a review MUST or SHOULD, or an analyze hard-fail
/// or blocking finding, each neither waived nor released by its deferral
/// ([`crate::primitives::decisions::releases`], spec 076). Only those hold `done` (spec 058,
/// `findings-cost-in-proportion-to-risk`). The map's `undispositioned` count
/// is not read: an undecided advisory finding or review observation is
/// recorded and visible, and holds nothing. For a record the runtime writes,
/// every gating count but the SHOULD one is already caught by an earlier
/// check, so in practice this is where an outstanding SHOULD blocks, as
/// §implement-phase requires; the other counts stay as a guard against a
/// hand-edited record whose `blocking` disagrees with them.
fn disposition_block(
    (file, command): (&str, &str),
    map: Option<crate::schema::primitives::Dispositions>,
    gating: u32,
    feature_dir: &Path,
    rel_dir: &str,
    project: &str,
) -> Option<Verdict> {
    let blocked = |by, message: String, guidance: String| Verdict {
        passed: false,
        blocked_by: Some(by),
        message: Some(message),
        guidance: Some(guidance),
        violations: vec![],
        cross_spec_impact: vec![],
        analyze_stale_paths: None,
    };

    if map.is_none() {
        return Some(blocked(
            ReviewGateBlock::RecordPredatesDispositions,
            format!(
                "blocked: {rel_dir}/{file} predates finding dispositions — re-run \
                 /{project}:{command}"
            ),
            format!(
                "A record without a `dispositions:` map was written before findings were \
                 dispositioned, so its run's findings may sit in the inbox unrouted. Re-running \
                 /{project}:{command} writes the map; the gate does not read its absence as zero."
            ),
        ));
    }

    if gating == 0 {
        return None;
    }
    let mut message =
        format!("blocked: {gating} outstanding gating finding(s) — {rel_dir}/{file}: {gating}");
    message.push_str(&held_by_deferral(feature_dir, rel_dir, &[file]));
    Some(blocked(
        ReviewGateBlock::UndispositionedFindings,
        message,
        format!(
            "Fix each outstanding finding; waive a review violation with its rationale \
             (/{project}:review --waive <rule-id> --reason \"...\"); or defer it to pipeline \
             work on another spec (/{project}:review --defer <rule-id> --target <path>), whose \
             gate then holds it — a deferral to this spec's own task, or to an external work \
             item alone, releases nothing. Then re-run /{project}:{command}. Undispositioned \
             advisory findings and observations do not hold `done`, a broken contract left \
             open apart."
        ),
    ))
}

/// Gate check 5: the review record carries MUST violations. Names each
/// deferral that holds `done` in `review.md` too, since a MUST whose deferral
/// releases nothing stops the gate here and never reaches the gating-finding
/// check (spec 076).
fn must_violations_block(
    must_violations: u32,
    feature_dir: &Path,
    rel_dir: &str,
    project: &str,
) -> Verdict {
    Verdict {
        passed: false,
        blocked_by: Some(ReviewGateBlock::MustViolations),
        message: Some(format!(
            "blocked: spec has {must_violations} MUST violation(s) — see {rel_dir}/review.md{}",
            held_by_deferral(
                feature_dir,
                rel_dir,
                &[crate::primitives::REVIEW_RECORD_FILE]
            ),
        )),
        guidance: Some(format!(
            "Resolve the violations and re-run /{project}:review, run \
             /{project}:review --waive <rule-id> --reason \"...\" for each waivable finding, \
             or defer each to pipeline work on another spec \
             (/{project}:review --defer <rule-id> --target <path>), whose gate then holds it."
        )),
        violations: vec![],
        cross_spec_impact: vec![],
        analyze_stale_paths: None,
    }
}

/// The suffix a blocked message carries for the deferrals that hold `done`
/// in `files`: each one its record marks `gating: true`, as its finding and
/// the work it names (spec 076) — a gating finding whose deferral releases
/// nothing ([`crate::primitives::decisions::releases`]). Empty when none
/// does. The MUST, analyze-findings and outstanding-findings blocks all
/// carry it, since a MUST or a gating analyze finding stops the gate before
/// the gating-finding check. A record whose decisions cannot be read, and an entry
/// marked gating that does not read as a deferral, are each said so, rather
/// than named as none (`QUAL-CLAIM-001`).
fn held_by_deferral(feature_dir: &Path, rel_dir: &str, files: &[&str]) -> String {
    use crate::schema::primitives::DecisionOutcome;
    let mut held = Vec::new();
    for file in files {
        let Ok(stored) = crate::primitives::decisions::read_decisions(feature_dir, file) else {
            held.push(format!(
                "the decisions in {rel_dir}/{file}, which could not be read"
            ));
            continue;
        };
        let mut unreadable = 0_usize;
        for entry in stored.iter().filter(|entry| entry.gating == Some(true)) {
            match entry.to_ref() {
                Ok(decision) if decision.outcome == DecisionOutcome::Deferred => {
                    held.push(format!(
                        "{} — to {}",
                        crate::primitives::flatten_line(&decision.key),
                        crate::primitives::decisions::work_reference(
                            decision.target.as_deref(),
                            decision.task.as_deref(),
                            decision.work_item.as_deref(),
                            crate::primitives::flatten_line,
                        )
                    ));
                }
                _ => unreadable += 1,
            }
        }
        if unreadable > 0 {
            held.push(format!(
                "{unreadable} entr(ies) in {rel_dir}/{file} marked gating that do not read as a deferral"
            ));
        }
    }
    if held.is_empty() {
        String::new()
    } else {
        format!("; held by deferral: {}", held.join("; "))
    }
}

/// Whether this record holds `done` until `/review` runs again, whatever the
/// window holds: a gating count, no `dispositions:` map, the open mark, a fix
/// round with neither a re-read nor a waiver, or a finding it leaves open, a broken
/// contract nobody fixed or waived among them. `compute-review-scope` reads it, with
/// the digest's freshness that takes files to judge, so `review-owed` never says no
/// review is owed while this gate asks for one (spec 079).
pub(crate) fn holds_done_until_reviewed(review: &crate::schema::primitives::ReviewBlock) -> bool {
    review.blocking
        || review
            .must_violations
            .saturating_add(review.should_violations)
            > 0
        || review.dispositions.is_none()
        || review.fix_rounds_open == Some(true)
        || review
            .not_re_read
            .is_some_and(|rounds| rounds > waived_rounds(review))
        || !review.open_findings.is_empty()
}

/// The fix rounds a record waives with a reason.
fn waived_rounds(review: &crate::schema::primitives::ReviewBlock) -> u32 {
    let waived = review
        .round_waivers
        .iter()
        .filter(|waiver| !waiver.reason.trim().is_empty())
        .count();
    u32::try_from(waived).unwrap_or(u32::MAX)
}

/// Gate check 9 — `review.md` still carries `fix-rounds-open` (spec 079), or
/// counts more fix rounds that needed a re-read and have none (`not-re-read`)
/// than waivers with a reason (spec 078).
///
/// Counted rather than matched by round, as the record states it, so a
/// hand-edited record with fewer waivers than un-re-read rounds blocks
/// whatever its other counts say, and a waiver whose reason is blank waives
/// nothing. A record with neither field does not say whether its fixes
/// were re-read: it passes here, and [`passing_notices`] says so.
fn fix_round_block(
    review: &crate::schema::primitives::ReviewBlock,
    rel_dir: &str,
    project: &str,
) -> Option<Verdict> {
    if review.fix_rounds_open == Some(true) {
        return Some(Verdict {
            passed: false,
            blocked_by: Some(ReviewGateBlock::FixRoundLoopOpen),
            message: Some(format!(
                "blocked: the fix-round loop in {rel_dir}/review.md is still open, so the review \
                 stopped before its last write and its fixes may be unread — re-run \
                 /{project}:review"
            )),
            guidance: Some(format!(
                "Re-run /{project}:review: its last write records the rounds and clears the mark."
            )),
            violations: vec![],
            cross_spec_impact: vec![],
            analyze_stale_paths: None,
        });
    }
    let not_re_read = review.not_re_read?;
    let waived = waived_rounds(review);
    if not_re_read <= waived {
        return None;
    }
    Some(Verdict {
        passed: false,
        blocked_by: Some(ReviewGateBlock::FixRoundNotReRead),
        message: Some(format!(
            "blocked: {} fix round(s) in {rel_dir}/review.md needed a re-read and have none \
             ({not_re_read} not re-read, {waived} waived)",
            not_re_read - waived
        )),
        guidance: Some(format!(
            "Re-run /{project}:review: its fix-round loop re-reads each round, or records the \
             operator's reason for declining one as that round's waiver."
        )),
        violations: vec![],
        cross_spec_impact: vec![],
        analyze_stale_paths: None,
    })
}

/// What a **passing** gate could not examine, as one guidance line.
///
/// Joined rather than ranked because an operator weighing the transition needs
/// all of it. One arm fires for an analysis record whose freshness could not be
/// determined at all, because it carries no digest of what its run read; a review
/// record's digest no longer decides anything at the gate (spec 081). Another fires
/// for a review record without the fix-round fields (`fix_rounds_recorded` false),
/// which does not say whether the fixes made during that review were re-read (spec
/// 078). A third fires when the window could not be read, so nothing says whether
/// a review is owed (spec 079).
///
/// `None` — real silence — only when everything examined cleanly, which is
/// what makes that silence mean something.
fn passing_notices(
    analyze: &RecordFreshness,
    fix_rounds_recorded: bool,
    owed_unknown: Option<&str>,
) -> Option<String> {
    let mut notices: Vec<String> = Vec::new();
    if let Some(reason) = owed_unknown {
        notices.push(format!(
            "Whether a review is owed could not be determined ({reason}), so what changed since \
             the last review was not examined."
        ));
    }
    if let RecordFreshness::Undeterminable { reason } = analyze {
        notices.push(format!(
            "Analyze-record freshness could not be determined ({reason}), so whether the \
             recorded analysis still describes these artifacts was not examined."
        ));
    }
    if !fix_rounds_recorded {
        notices.push(
            "The review record has no `not-re-read` field, so it does not say whether the \
             fixes made during that review were re-read: it predates the fix-round loop, or its \
             run stated nothing about its fixes."
                .to_string(),
        );
    }
    if notices.is_empty() {
        None
    } else {
        Some(notices.join(" "))
    }
}

/// Gate checks 11, 12 and 13 — the analyze record in `analysis.md`: a completed analysis,
/// whose findings do not hold the spec out of `done`, still describing the
/// current artifacts.
///
/// Ordered after every `review:` check because the pipeline is
/// `review → analyze → done`. A spec whose review is missing or failing has
/// not reached the point where analysis is the next thing owed, and naming the
/// later gate for an earlier defect sends a contributor to the wrong command.
///
/// **No grandfather clause, and there must not be one.** The CI template's
/// record check and `/{project}:audit` Family 37 exempt a `done` spec that
/// predates the record, because they audit a corpus written before the field
/// existed. This gate runs at the moment a
/// spec is being completed *now*, so the record is always writable — an
/// exemption here would be a permanent hole rather than a bounded
/// transitional one.
///
/// Advisory findings never block; see `AnalyzeBlock::advisory` for why this
/// does not mirror the review gate's treatment of an outstanding SHOULD. They
/// ride the guidance line instead, together with the unexamined count, so a
/// blocked gate still says what the analysis did not look at.
fn analyze_gate_block(
    repo: &Path,
    rel_dir: &str,
    analyze: Option<&AnalyzeBlock>,
    project: &str,
) -> std::result::Result<RecordFreshness, Verdict> {
    let analyze = match analyze {
        Some(analyze) if analyze.last_run.is_some() => analyze,
        // Absent block or null `last-run`: the spec has never completed an
        // analysis. This is the state every spec was in before the record
        // existed, which is how two specs reached `done` on 2026-09-05 with
        // only the review gate run — one of them published to crates.io
        // before anything noticed, because nothing could.
        _ => {
            return Err(Verdict {
                passed: false,
                blocked_by: Some(ReviewGateBlock::NotAnalyzed),
                message: Some(format!(
                    "blocked: spec has not been analyzed — run /{project}:analyze before completing"
                )),
                guidance: None,
                violations: vec![],
                cross_spec_impact: vec![],
                analyze_stale_paths: None,
            });
        }
    };

    if !analyze.blocking {
        // Check 13 last: it is the weakest of the three claims. The others say the analysis is missing or failing;
        // this one says a passing analysis is out of date.
        let freshness =
            crate::primitives::analyze_subjects::analyze_freshness(repo, rel_dir, Some(analyze));
        return match stale_analyze_block(&freshness, project)
            .or_else(|| unjudged_rules_block(analyze, project))
        {
            Some(blocked) => Err(blocked),
            None => Ok(freshness),
        };
    }

    Err(Verdict {
        passed: false,
        blocked_by: Some(ReviewGateBlock::AnalyzeFindings),
        message: Some(format!(
            "blocked: spec has {} hard-fail and {} blocking analyze finding(s) — \
             run /{project}:analyze to see them{}",
            analyze.hard_fail,
            analyze.blocking_findings,
            held_by_deferral(
                &repo.join(rel_dir),
                rel_dir,
                &[crate::primitives::ANALYSIS_RECORD_FILE]
            ),
        )),
        guidance: Some(format!(
            "Resolve them and re-run /{project}:analyze, or defer each to pipeline work on \
             another spec, whose gate then holds it. Advisory findings do not block; \
             the recorded run carries {} of them and {} unexamined target(s).",
            analyze.advisory, analyze.unexamined
        )),
        violations: vec![],
        cross_spec_impact: vec![],
        analyze_stale_paths: None,
    })
}

/// Render a path list for an operator-facing message: the first three, comma
/// separated, plus `(+N more)` when the set is larger.
///
/// One implementation for the two messages that name paths — the review-owed
/// block and the analyze-staleness block. The two staleness blocks it first
/// served had drifted into copies of the same six lines, which is how the cap
/// silently becomes three in one message and five in another.
fn render_paths<'a>(paths: impl IntoIterator<Item = &'a String>) -> String {
    const SHOWN: usize = 3;
    let paths: Vec<&str> = paths.into_iter().map(String::as_str).collect();
    let more = paths.len().saturating_sub(SHOWN);
    let head = paths
        .iter()
        .take(SHOWN)
        .copied()
        .collect::<Vec<&str>>()
        .join(", ");
    if more > 0 {
        format!("{head} (+{more} more)")
    } else {
        head
    }
}

/// Gate check 13, after staleness: `Some(blocked)` when the analysis record names a
/// loaded rule in `unjudged-rules` (spec 082). An unjudged rule is an obligation
/// nobody has verified, and the next analysis judges only those rules, since a
/// current verdict is reused. A record with no such field lists none.
fn unjudged_rules_block(analyze: &AnalyzeBlock, project: &str) -> Option<Verdict> {
    if analyze.unjudged_rules.is_empty() {
        return None;
    }
    Some(Verdict {
        passed: false,
        blocked_by: Some(ReviewGateBlock::UnjudgedRules),
        message: Some(format!(
            "blocked: {} loaded rule(s) no analysis has judged: {}",
            analyze.unjudged_rules.len(),
            render_paths(&analyze.unjudged_rules)
        )),
        guidance: Some(format!(
            "Run /{project}:analyze, which judges only these rules now that current verdicts are reused."
        )),
        violations: vec![],
        cross_spec_impact: vec![],
        analyze_stale_paths: None,
    })
}

/// The staleness gate check over the `analyze:` record: `Some(blocked)` when
/// the recorded analysis no longer describes the spec's artifacts.
///
/// Ordered after [`analyze_gate_block`]'s two checks because this is the weakest
/// of the three claims.
/// The others say the analysis is missing or failing; this one says a passing
/// analysis is out of date.
///
/// The comparison lives in [`crate::primitives::analyze_subjects`], which
/// `/{project}:review`'s row also calls. It compares **content**, not commits,
/// so this gate and that row cannot return different answers, and neither
/// blocks on content an analysis genuinely examined merely because it has
/// since been committed.
///
/// [`RecordFreshness::Undeterminable`] does **not** block — it rides a guidance
/// line on the passing verdict instead. A gate that blocks on its own inability
/// to check is one people route around; the difference from failing open is
/// that this one says so.
fn stale_analyze_block(freshness: &RecordFreshness, project: &str) -> Option<Verdict> {
    let RecordFreshness::Stale { paths, .. } = freshness else {
        return None;
    };
    Some(Verdict {
        passed: false,
        blocked_by: Some(ReviewGateBlock::AnalyzeStale),
        message: Some(format!(
            "blocked: analysis is stale — {} artifact(s) changed since it ran: {}",
            paths.len(),
            render_paths(paths)
        )),
        guidance: Some(format!(
            "Re-run /{project}:analyze so the recorded findings describe the current artifacts."
        )),
        violations: vec![],
        cross_spec_impact: vec![],
        analyze_stale_paths: Some(paths.clone().into_boxed_slice()),
    })
}

/// Gate check 1 — every markdown file in the feature directory passes
/// `markdownlint-cli2`, by [`lint_markdown::feature_markdown_glob`].
///
/// A non-zero exit the parser could not attribute to specific violations gets
/// its own message rather than being reported as zero violations: a lint that
/// could not run is not a lint that found nothing.
fn markdown_lint_block(
    rel_dir: &str,
    repo: &Path,
    lint: impl FnOnce(&LintMarkdownArgs, &Path) -> Result<lint_markdown::Lint>,
) -> Result<Option<Verdict>> {
    let lint_result = lint(
        &LintMarkdownArgs {
            paths: vec![lint_markdown::feature_markdown_glob(rel_dir)],
            fix: false,
        },
        repo,
    )?;
    if lint_result.clean {
        return Ok(None);
    }
    let message = if lint_result.violations.is_empty() && lint_result.exit_code == 0 {
        format!(
            "blocked: markdownlint-cli2 linted no file in {rel_dir} — nothing was examined, so the lint is not a pass"
        )
    } else if lint_result.violations.is_empty() {
        format!(
            "blocked: markdownlint-cli2 exited {} for {rel_dir} — resolve the lint failure before completing",
            lint_result.exit_code
        )
    } else {
        format!(
            "blocked: {} markdownlint violation(s) in {rel_dir} — resolve them before completing",
            lint_result.violations.len()
        )
    };
    Ok(Some(Verdict {
        passed: false,
        blocked_by: Some(ReviewGateBlock::MarkdownLint),
        message: Some(message),
        guidance: None,
        violations: lint_result.violations,
        cross_spec_impact: vec![],
        analyze_stale_paths: None,
    }))
}

/// The blocked verdict for a spec with no completed review.
///
/// Absence of the artifact and a record carrying a null `last-run` are the same
/// answer — no review has finished — so they share one message. What they must
/// not share is [`unreadable_record_block`]'s, which is a different claim.
fn never_reviewed_block(project: &str) -> Verdict {
    Verdict {
        passed: false,
        blocked_by: Some(ReviewGateBlock::NotReviewed),
        message: Some(format!(
            "blocked: spec has not been reviewed — run /{project}:review before completing"
        )),
        guidance: None,
        violations: vec![],
        cross_spec_impact: vec![],
        analyze_stale_paths: None,
    }
}

/// Which record a blocked verdict is about — the two halves differ only in
/// their nouns.
#[derive(Clone, Copy)]
enum RecordKind {
    Review,
    Analyze,
}

/// The blocked verdict for an `analysis.md` whose record reads and whose `decisions:` list
/// does not: `write-analysis` refuses to record over it, so a re-run is no repair.
fn unreadable_decisions_block(rel_dir: &str, reason: &str) -> Verdict {
    Verdict {
        passed: false,
        blocked_by: Some(ReviewGateBlock::AnalyzeRecordUnreadable),
        message: Some(format!(
            "blocked: {rel_dir}/analysis.md's decisions: list could not be read — which findings it decided is undeterminable"
        )),
        guidance: Some(format!(
            "Repair the list in {rel_dir}/analysis.md by hand, since an analysis refuses to record over it. Parse error: {reason}"
        )),
        violations: vec![],
        cross_spec_impact: vec![],
        analyze_stale_paths: None,
    }
}

/// The blocked verdict for a record artifact that exists but will not parse.
///
/// Deliberately not the never-run message. Absence is an answer; a damaged file
/// is not, and telling an operator to "run /review" for a spec that may well
/// have been reviewed sends them to redo work on the strength of something the
/// gate could not read. The parse error travels with it, so the repair is
/// actionable rather than a guess.
fn unreadable_record_block(
    rel_dir: &str,
    project: &str,
    kind: RecordKind,
    reason: &str,
) -> Verdict {
    let (file, noun, verb, blocked_by) = match kind {
        RecordKind::Review => (
            "review.md",
            "a review",
            "review",
            ReviewGateBlock::RecordUnreadable,
        ),
        RecordKind::Analyze => (
            "analysis.md",
            "an analysis",
            "analyze",
            ReviewGateBlock::AnalyzeRecordUnreadable,
        ),
    };
    Verdict {
        passed: false,
        blocked_by: Some(blocked_by),
        message: Some(format!(
            "blocked: {rel_dir}/{file} exists but its record could not be read — whether {noun} ran is undeterminable"
        )),
        guidance: Some(format!(
            "Repair the frontmatter in {rel_dir}/{file}, or re-run /{project}:{verb} to rewrite it. Parse error: {reason}"
        )),
        violations: vec![],
        cross_spec_impact: vec![],
        analyze_stale_paths: None,
    }
}

/// Gate check 0 — the spec is already at `status: done`, so there is no
/// `in-progress → done` transition for this gate to authorize.
///
/// **Not `passed: true`.** A gate that reports "passed" for a spec it did not
/// examine is the `QUAL-CLAIM-001` conflation every other check here is built
/// to avoid, and a caller could read it as authorization to transition a spec
/// that is already transitioned. It is its own variant, naming the state.
///
/// The concrete defect this closes: the completing `set-status` rewrites
/// `spec.md`, which is one of the analyze record's subjects, so a spec's
/// analysis is stale the instant it reaches `done`. Verified on spec 047
/// seconds after its own transition. Re-running `/{project}:implement` against
/// finished work therefore reported `analysis is stale — spec.md` and told the
/// operator to re-run the analysis; following that advice wrote a fresh record
/// and appeared to work, which is worse than advice that plainly fails.
///
/// Only the exact value `done` short-circuits. `status` is a required field,
/// so an absent one already fails deserialization before this runs; a value
/// *outside* the lifecycle set falls through and the gate runs its checks,
/// because `validate-frontmatter` owns reporting a bad value and inferring
/// "probably finished" from an unrecognized status would be the same unearned
/// conclusion in the other direction.
fn already_done_block(status: &str, project: &str) -> Option<Verdict> {
    if status != "done" {
        return None;
    }
    Some(Verdict {
        passed: false,
        blocked_by: Some(ReviewGateBlock::AlreadyDone),
        message: Some(
            "not gated: spec is already at status done — the in-progress → done transition \
             this gate authorizes has already happened"
                .to_string(),
        ),
        guidance: Some(format!(
            "Nothing is owed. To change what the spec requires, reopen it first: /{project}:amend records a \
             scenario and takes the done → in-progress back-edge."
        )),
        violations: vec![],
        cross_spec_impact: vec![],
        analyze_stale_paths: None,
    })
}

/// Classify every `cross-spec-impact:` entry the declaring spec carries.
///
/// An entry is **discharged** when the named spec links back to `feature` —
/// from its own `spec.md` body or from any scenario under it. The reciprocal
/// link is the signpost §cross-spec-impact already requires, so the check
/// proves the obligation was met rather than trusting that a key was removed
/// honestly: a declaration an author deletes to unblock themselves enforces
/// nothing, while a link they have to add lands the change where the affected
/// spec's reader meets it.
///
/// A scenario counts because a reader of the affected spec reads its
/// scenarios; that is the whole test the signpost has to pass.
///
/// Link reading is `derive-dependencies`' matcher
/// ([`super::derive_dependencies::scan_line`]) rather than a second parser,
/// over [`spec_links::LinkScope::Pointers`] rather than the edge scope that matcher is
/// normally paired with. The scope is the difference that matters and it is
/// deliberate: the edge scope drops blockquote-prefixed lines, and a signpost
/// on a `done` spec is written as a blockquote in this corpus, so reusing it
/// whole would make the canonical discharge artifact invisible here.
///
/// An entry naming a **branch-scoped** directory (`{id}.{n}-slug`, spec 051)
/// resolves as `target-missing` when no such directory exists and otherwise
/// reports `undischarged` however the target links back, because the shared
/// matcher recognizes the sequential `NNN-slug` link form only. That bound is
/// inherited rather than introduced, and it is outside what a declaration is
/// for: a branch-scoped spec's content is folded into its upstream spec, so an
/// impact on one is declared against the upstream spec it folds into.
pub(crate) fn cross_spec_impact_states(
    declared: &[String],
    feature: &str,
    repo: &Path,
    specs_root: &str,
) -> Vec<CrossSpecImpactEntry> {
    declared
        .iter()
        .map(|target| CrossSpecImpactEntry {
            spec: target.clone(),
            state: classify_cross_spec_entry(target, feature, repo, specs_root),
        })
        .collect()
}

/// Classify one declared entry. Split out so the ordering of the
/// disqualifying cases is readable: self-reference, a name that is not a
/// feature directory, and a directory that is not there are all decided
/// **before** anything is read, and only a real other spec is searched for the
/// back-link.
///
/// The name is screened through [`super::parse_feature_dir`] — the single
/// place either directory form is recognized — rather than by joining it to a
/// path and asking the filesystem. That ordering is load-bearing, not tidiness:
/// an entry is a *slug*, and joining an unscreened one would let
/// `cross-spec-impact: ["../../elsewhere"]` walk out of the spec root and read
/// a file there. Unlike the `[constitutions]` and `[services]` paths described
/// under **The config-sourced path boundary** on
/// [`super::validate_no_traversal`], nothing about this key's design needs
/// `..`: a slug that is not a feature directory name can name no feature
/// directory, which is exactly `target-missing`.
fn classify_cross_spec_entry(
    target: &str,
    feature: &str,
    repo: &Path,
    specs_root: &str,
) -> CrossSpecImpactState {
    if target == feature {
        return CrossSpecImpactState::SelfReference;
    }
    if super::parse_feature_dir(target).is_none() {
        return CrossSpecImpactState::TargetMissing;
    }
    let target_dir = repo.join(specs_root).join(target);
    if !target_dir.is_dir() {
        return CrossSpecImpactState::TargetMissing;
    }
    if links_back(&target_dir.join("spec.md"), specs_root, feature, 1) {
        return CrossSpecImpactState::Discharged;
    }
    let scenarios = target_dir.join("scenarios");
    let Ok(entries) = std::fs::read_dir(&scenarios) else {
        // No scenarios directory, or one that cannot be read. Neither proves
        // a back-link is absent from a file, so this is not a discharge — it
        // is the same `undischarged` the operator resolves by adding one.
        return CrossSpecImpactState::Undischarged;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
            && links_back(&path, specs_root, feature, 2)
        {
            return CrossSpecImpactState::Discharged;
        }
    }
    CrossSpecImpactState::Undischarged
}

/// Whether the markdown file at `path` carries a pointer to `feature`.
///
/// `depth` is how many levels below the spec root the file sits, which decides
/// how many `../` segments its relative links carry — 1 for a `spec.md`, 2 for
/// a scenario. An unreadable file yields `false`: nothing can be proven about a
/// file that will not open, and the honest reading of "no proof of discharge"
/// is that the obligation still stands.
fn links_back(path: &Path, specs_root: &str, feature: &str, depth: usize) -> bool {
    let Ok(content) = std::fs::read_to_string(path) else {
        return false;
    };
    let mut slugs = BTreeSet::new();
    for line in spec_links::pointer_lines(&content) {
        super::derive_dependencies::scan_line(line.text, specs_root, depth, &mut slugs);
    }
    slugs.contains(feature)
}

/// The cross-spec-impact gate check: `Some(blocked)` when any declared entry
/// is not discharged, `None` when every entry is (including the empty case).
///
/// Ordered ahead of the `review:` block because a spec carrying an obligation
/// nobody has discharged is not a candidate for `done`, so whether its review
/// is fresh does not yet matter.
///
/// **Nothing but this check carries an undischarged impact past `done`**, and
/// that is why it blocks where a pending fold does not. A fold owed by a `done`
/// staging spec stays reported by `check-unfolded-specs` and the pipeline
/// view's `(fold pending)` until fold-back discharges it (spec 051 scenario
/// `a-pending-fold-never-holds-done`); an impact has no such surface, so
/// letting it through would let the obligation disappear the moment the spec
/// closed. The two keys stay separate for the same reason they always were:
/// they point in opposite directions — a fold moves the declaring spec's own
/// content into its home, an impact asks for a change to another spec's.
///
/// There is no `--fix`. Which section of the affected spec should carry the
/// signpost is the routing judgment `/{project}:amend` puts to the operator,
/// and a wrong auto-write into another spec is worse than a precise refusal.
fn cross_spec_impact_block(
    entries: &[CrossSpecImpactEntry],
    feature: &str,
    project: &str,
) -> Option<Verdict> {
    let name = |wanted: CrossSpecImpactState| -> Vec<&str> {
        entries
            .iter()
            .filter(|e| e.state == wanted)
            .map(|e| e.spec.as_str())
            .collect()
    };
    let undischarged = name(CrossSpecImpactState::Undischarged);
    let missing = name(CrossSpecImpactState::TargetMissing);
    let self_ref = name(CrossSpecImpactState::SelfReference);
    if undischarged.is_empty() && missing.is_empty() && self_ref.is_empty() {
        return None;
    }

    let mut parts = Vec::new();
    if !undischarged.is_empty() {
        parts.push(format!("{} does not link back", undischarged.join(", ")));
    }
    if !missing.is_empty() {
        parts.push(format!("{} names no spec directory", missing.join(", ")));
    }
    if !self_ref.is_empty() {
        parts.push(format!("{} is this spec itself", self_ref.join(", ")));
    }

    let mut guidance = Vec::new();
    if !undischarged.is_empty() {
        guidance.push(format!(
            "Record the change in each affected spec with a signpost linking back to {feature}, then re-run this gate — discharge is the reciprocal link, not the key's removal. A signpost on a done spec is a correction and leaves it done; a criterion or scenario there reopens it through /{project}:amend."
        ));
    }
    if !missing.is_empty() {
        guidance.push(
            "An entry naming no spec directory is a typo — correct the slug, or remove the entry if the impact turned out not to exist."
                .to_string(),
        );
    }
    if !self_ref.is_empty() {
        guidance.push(
            "A spec cannot owe itself a cross-spec change; remove the self-entry.".to_string(),
        );
    }

    Some(Verdict {
        passed: false,
        blocked_by: Some(ReviewGateBlock::UndischargedCrossSpecImpact),
        message: Some(format!(
            "blocked: undischarged cross-spec impact — {}",
            parts.join("; ")
        )),
        guidance: Some(guidance.join(" ")),
        violations: vec![],
        // The full per-entry list, discharged rows included: the operator
        // needs to see which half of a partial declaration is already met.
        cross_spec_impact: entries.to_vec(),
        analyze_stale_paths: None,
    })
}

/// Gate check 2's second half: `tasks.md` holds an unchecked box under any heading
/// (spec 079). A file that cannot be read is an error, never a pass.
fn unchecked_task_block(feature_dir: &Path, rel_dir: &str) -> Result<Option<Verdict>> {
    let tasks = feature_dir.join("tasks.md");
    if !tasks.is_file() {
        return Ok(None);
    }
    Ok(describe_unchecked(&read_text(&tasks)?).map(|owed| Verdict {
        passed: false,
        blocked_by: Some(ReviewGateBlock::UncheckedTask),
        message: Some(format!(
            "blocked: {rel_dir}/tasks.md holds {owed} — finish each before completing"
        )),
        guidance: Some(
            "Tick a box once its work is done, and remove one no longer owed. A step that can \
             only happen after done, such as a release, is not this spec's task."
                .to_string(),
        ),
        violations: vec![],
        cross_spec_impact: vec![],
        analyze_stale_paths: None,
    }))
}

/// The scenario-open-questions gate check: `Some(blocked)` when any
/// scenario under `feature_dir` carries an unresolved open question,
/// `None` when the check passes.
///
/// A scenario is an organizational split of the spec — it exists to keep
/// `spec.md` from becoming one huge document — so its questions are the
/// spec's questions for the purpose of completeness, and the spec is not
/// `done` while any remain (spec 046). Resolution stays independent:
/// the guidance points at scenario-targeted clarify, the only command that
/// can act on them.
///
/// Only `## Open Questions` counts. Per §spec-requirements an open
/// question is an *undecided blocker*; one deferred pending a condition is
/// resolved-with-a-condition and belongs in `## Resolved Questions`, which
/// the parser does not read. That is a convention rather than a skip
/// marker on purpose — an exemptible section would let anything blocking
/// be relabelled to ship past this gate.
///
/// Ordered ahead of the `review:` checks because an unresolved design
/// question is the more upstream defect — reviewing a design that is about
/// to change wastes the review.
///
/// Reads through `read-spec`'s collector rather than re-deriving the list,
/// so the gate blocks on exactly the questions the user was shown.
fn scenario_question_block(
    feature_dir: &Path,
    repo: &Path,
    feature: &str,
) -> Result<Option<Verdict>> {
    let scan = read_spec::collect_scenario_open_questions(feature_dir);
    let questions = scan.questions;
    if questions.is_empty() {
        // An unreadable scenario does not block — nothing can be proven about
        // a file that will not parse, and this gate must not fail closed on
        // its own inability to read. It is reported by `read-spec` and by
        // `check-artifacts`' skipped list instead, so the pass is still
        // distinguishable from a fully-examined one.
        return Ok(None);
    }
    let scenarios = read_spec::scenario_names(&questions);
    let project = Host::load(repo)?.project;
    Ok(Some(Verdict {
        passed: false,
        blocked_by: Some(ReviewGateBlock::ScenarioOpenQuestions),
        message: Some(format!(
            "blocked: {} unresolved open question(s) in scenario(s) {} — resolve them before completing",
            questions.len(),
            scenarios.join(", ")
        )),
        guidance: Some(format!(
            "Run /{project}:target {feature}/<scenario> then /{project}:clarify to resolve each \
             scenario's questions in place. A question that is deferred rather than undecided \
             (\"not now; revisit when X lands\") is resolved with a condition — move it to the \
             scenario's Resolved Questions with its trigger recorded."
        )),
        violations: vec![],
        cross_spec_impact: vec![],
        analyze_stale_paths: None,
    }))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fmt::Write as _;
    use std::fs;
    use tempfile::tempdir;

    /// Reviewed clean, but the analyze record is absent entirely — the state
    /// every spec was in before this block existed, and the one that let two
    /// specs reach `done` (and one of them crates.io) on a single gate.
    const NEVER_ANALYZED: &str = "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false\n---\n\n# 007 — Gate\n";

    /// The block is present but `last-run` is null — the same "never ran"
    /// state spelled differently, and the review gate treats its own
    /// equivalent identically.
    const ANALYZE_NULL_RUN: &str = "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false\nanalyze:\n  last-run: null\n  blocking: false\n---\n\n# 007 — Gate\n";

    const ANALYZE_BLOCKING: &str = "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false\nanalyze:\n  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc123\n  hard-fail: 1\n  blocking-findings: 2\n  advisory: 0\n  unexamined: 0\n  blocking: true\n---\n\n# 007 — Gate\n";

    /// Advisory findings and unexamined targets recorded, nothing gating.
    const ANALYZE_ADVISORY_ONLY: &str = "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false\nanalyze:\n  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc123\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 7\n  unexamined: 4\n  blocking: false\n---\n\n# 007 — Gate\n";

    /// A `done` spec, otherwise clean — the state gate check 0 short-circuits.
    const ALREADY_DONE: &str = "---\nstatus: done\ndependencies: []\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false\nanalyze:\n  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc123\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 0\n  unexamined: 0\n  blocking: false\n---\n\n# 007 — Gate\n";
    const REVIEWED_CLEAN: &str = "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false\nanalyze:\n  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc123\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 2\n  unexamined: 0\n  blocking: false\n---\n\n# 007 — Gate\n";
    const REVIEWED_BLOCKING: &str = "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 3\n  should-violations: 0\n  low-confidence: 0\n  blocking: true\nanalyze:\n  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc123\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 2\n  unexamined: 0\n  blocking: false\n---\n\n# 007 — Gate\n";
    const NEVER_REVIEWED: &str = "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: null\n  reviewed-against: null\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false\nanalyze:\n  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc123\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 2\n  unexamined: 0\n  blocking: false\n---\n\n# 007 — Gate\n";
    const NO_REVIEW_BLOCK: &str = "---\nstatus: in-progress\ndependencies: []\nanalyze:\n  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc123\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 2\n  unexamined: 0\n  blocking: false\n---\n\n# 007 — Gate\n";

    /// Split a fixture's `review:` / `analyze:` blocks out of the spec and
    /// write each to the artifact that now owns it (spec 057).
    ///
    /// The constants above describe *states* — never reviewed, blocking,
    /// advisory-only — and those states are what the tests assert on. Moving
    /// them by hand across twenty-odd literals would be twenty-odd chances to
    /// alter a case while relocating it, so the fixture is relocated
    /// mechanically instead and each constant keeps saying what it said.
    fn relocate_records(repo: &Path, spec: &str) -> String {
        relocate_records_in(&repo.join("specs/007-gate"), spec)
    }

    fn relocate_records_in(dir: &Path, spec: &str) -> String {
        let mut spec_out = String::new();
        let mut blocks: std::collections::BTreeMap<&str, String> =
            std::collections::BTreeMap::new();
        let mut current: Option<&str> = None;

        for line in spec.lines() {
            let indented = line.starts_with([' ', '\t']);
            if !indented {
                current = match line.trim_end() {
                    "review:" => Some("review"),
                    "analyze:" => Some("analyze"),
                    _ => None,
                };
                if current.is_some() {
                    continue;
                }
            }
            if let Some(key) = current {
                if indented {
                    let entry = blocks.entry(key).or_default();
                    entry.push_str(line.strip_prefix("  ").unwrap_or(line));
                    entry.push('\n');
                    continue;
                }
                current = None;
            }
            spec_out.push_str(line);
            spec_out.push('\n');
        }

        for (key, body) in &blocks {
            let file = if *key == "review" {
                "review.md"
            } else {
                "analysis.md"
            };
            // Every fixture record is written as a post-058 run would write
            // it, with a `dispositions:` map, unless it states its own. The
            // records that deliberately predate the map are written directly
            // by the tests that exercise that block.
            let map = if body.contains("dispositions:") {
                ""
            } else {
                "dispositions:\n  fixed: 0\n  deferred: 0\n  waived: 0\n  undispositioned: 0\n"
            };
            fs::write(
                dir.join(file),
                format!("---\nspec: 007-gate\n{body}{map}---\n\n# {key} — 007-gate\n"),
            )
            .unwrap();
        }
        spec_out
    }

    fn seed(repo: &Path, spec: &str) {
        fs::create_dir_all(repo.join("specs/007-gate")).unwrap();
        let spec = relocate_records(repo, spec);
        fs::write(repo.join("specs/007-gate/spec.md"), &spec).unwrap();
        // Pin the slash-command namespace so canonical messages are
        // deterministic (the default is the tempdir's random basename).
        fs::write(repo.join(".govern.toml"), "[host]\nproject = \"ductus\"\n").unwrap();
    }

    fn args() -> CheckReviewGateArgs {
        CheckReviewGateArgs {
            feature: "007-gate".into(),
        }
    }

    // The wrapper matches the `run_with_lint` seam signature.
    #[allow(clippy::unnecessary_wraps)]
    fn clean_lint(_: &LintMarkdownArgs, _: &Path) -> Result<lint_markdown::Lint> {
        Ok(lint_markdown::Lint {
            violations: vec![],
            clean: true,
            exit_code: 0,
        })
    }

    /// A branch-scoped spec, reviewed clean and analyzed clean, declaring the
    /// upstream spec it folds into. Nothing in it holds `done`: the fold is
    /// owed after the transition, not before it.
    const DECLARES_FOLD: &str = "---\nstatus: in-progress\ndependencies: []\nfolds-into: 050-upstream\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false\nanalyze:\n  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc123\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 2\n  unexamined: 0\n  blocking: false\n---\n\n# 007 — Gate\n";

    /// Reviewed clean and analyzed clean, declaring one cross-spec impact —
    /// the declaration is the only thing holding it short of `done`.
    const DECLARES_IMPACT: &str = "---\nstatus: in-progress\ndependencies: []\ncross-spec-impact: [050-upstream]\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123\n  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n  blocking: false\nanalyze:\n  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: abc123\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 0\n  unexamined: 0\n  blocking: false\n---\n\n# 007 — Gate\n";

    /// Write `specs/050-upstream/spec.md` with `body` after its frontmatter.
    fn seed_target(repo: &Path, body: &str) {
        fs::create_dir_all(repo.join("specs/050-upstream")).unwrap();
        fs::write(
            repo.join("specs/050-upstream/spec.md"),
            format!("---\nstatus: done\ndependencies: []\n---\n\n# 050 — Upstream\n\n{body}\n"),
        )
        .unwrap();
    }

    #[test]
    fn a_declared_impact_blocks_while_the_target_does_not_link_back() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), DECLARES_IMPACT);
        seed_target(tmp.path(), "Body text that names nothing.");

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();

        assert!(!result.passed);
        assert_eq!(
            result.blocked_by,
            Some(ReviewGateBlock::UndischargedCrossSpecImpact)
        );
        let message = result.message.expect("the block carries a message");
        assert!(
            message.contains("050-upstream does not link back"),
            "{message}"
        );
        assert_eq!(
            result.cross_spec_impact,
            vec![CrossSpecImpactEntry {
                spec: "050-upstream".into(),
                state: CrossSpecImpactState::Undischarged,
            }]
        );
        let guidance = result.guidance.expect("the block carries guidance");
        assert!(guidance.contains("signpost linking back"), "{guidance}");
        assert!(guidance.contains("/ductus:amend"), "{guidance}");
    }

    #[test]
    fn a_reciprocal_link_in_the_target_body_discharges_the_obligation() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), DECLARES_IMPACT);
        seed_target(
            tmp.path(),
            "Recorded for [007](../007-gate/spec.md), which required it.",
        );

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();

        assert!(result.passed, "{:?}", result.message);
        assert_eq!(
            result.cross_spec_impact,
            vec![CrossSpecImpactEntry {
                spec: "050-upstream".into(),
                state: CrossSpecImpactState::Discharged,
            }],
            "a passing gate still reports what it examined"
        );
    }

    /// The canonical discharge artifact in this corpus is a blockquoted
    /// signpost, and the dependency harvest drops blockquote-prefixed lines by
    /// policy. Reusing that scope whole would make the gate blind to exactly
    /// the artifact §cross-spec-impact prescribes — this is the test that
    /// holds `LinkScope::Pointers` in place.
    #[test]
    fn a_blockquoted_signpost_discharges_the_obligation() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), DECLARES_IMPACT);
        seed_target(
            tmp.path(),
            "> **Signpost:** required by [007](../007-gate/spec.md).",
        );

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();

        assert!(result.passed, "{:?}", result.message);
    }

    #[test]
    fn a_link_from_a_scenario_under_the_target_discharges_the_obligation() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), DECLARES_IMPACT);
        seed_target(tmp.path(), "Body text that names nothing.");
        fs::create_dir_all(tmp.path().join("specs/050-upstream/scenarios")).unwrap();
        fs::write(
            tmp.path().join("specs/050-upstream/scenarios/a-case.md"),
            "---\nsection: \"Body\"\n---\n\n# A-case\n\nRequired by [007](../../007-gate/spec.md).\n",
        )
        .unwrap();

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();

        assert!(result.passed, "{:?}", result.message);
    }

    /// A scenario's links carry two `../` segments rather than one. Reading
    /// them at the spec-body depth finds nothing, so the depth parameter is
    /// what makes the case above work — assert the wrong depth really is
    /// wrong, or the test above would pass for the wrong reason.
    #[test]
    fn a_scenario_link_read_at_spec_depth_resolves_to_nothing() {
        let line = "Required by [007](../../007-gate/spec.md).";
        let mut at_spec_depth = BTreeSet::new();
        super::super::derive_dependencies::scan_line(line, "specs", 1, &mut at_spec_depth);
        assert!(at_spec_depth.is_empty());

        let mut at_scenario_depth = BTreeSet::new();
        super::super::derive_dependencies::scan_line(line, "specs", 2, &mut at_scenario_depth);
        assert_eq!(
            at_scenario_depth
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["007-gate"]
        );
    }

    #[test]
    fn an_entry_naming_no_spec_directory_is_a_finding_not_a_pass() {
        let tmp = tempdir().unwrap();
        seed(
            tmp.path(),
            &DECLARES_IMPACT.replace("050-upstream", "050-upstreem"),
        );
        seed_target(tmp.path(), "Required by [007](../007-gate/spec.md).");

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();

        assert!(!result.passed, "one letter must not disable the gate");
        assert_eq!(
            result.blocked_by,
            Some(ReviewGateBlock::UndischargedCrossSpecImpact)
        );
        assert_eq!(
            result.cross_spec_impact,
            vec![CrossSpecImpactEntry {
                spec: "050-upstreem".into(),
                state: CrossSpecImpactState::TargetMissing,
            }]
        );
        let guidance = result.guidance.expect("the block carries guidance");
        assert!(guidance.contains("typo"), "{guidance}");
    }

    /// An entry is a slug, not a path. Joining an unscreened one to the spec
    /// root would read a file outside it; the membership screen runs first.
    #[test]
    fn a_traversing_entry_is_target_missing_and_reads_nothing() {
        let tmp = tempdir().unwrap();
        seed(
            tmp.path(),
            &DECLARES_IMPACT.replace("050-upstream", "../../elsewhere"),
        );
        // A real directory at the traversal target, holding a back-link that
        // *would* discharge if the path were followed.
        let outside = tmp.path().parent().unwrap().join("elsewhere");
        fs::create_dir_all(&outside).unwrap();
        fs::write(
            outside.join("spec.md"),
            "---\nstatus: done\ndependencies: []\n---\n\n# X\n\n[007](../007-gate/spec.md)\n",
        )
        .unwrap();

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();

        assert!(!result.passed);
        assert_eq!(
            result.cross_spec_impact,
            vec![CrossSpecImpactEntry {
                spec: "../../elsewhere".into(),
                state: CrossSpecImpactState::TargetMissing,
            }],
            "a traversing entry must never resolve, discharged or otherwise"
        );
        fs::remove_dir_all(&outside).ok();
    }

    #[test]
    fn an_entry_naming_the_declaring_spec_is_reported_not_discharged() {
        let tmp = tempdir().unwrap();
        seed(
            tmp.path(),
            &DECLARES_IMPACT.replace("050-upstream", "007-gate"),
        );

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();

        assert!(!result.passed);
        assert_eq!(
            result.cross_spec_impact,
            vec![CrossSpecImpactEntry {
                spec: "007-gate".into(),
                state: CrossSpecImpactState::SelfReference,
            }]
        );
        let guidance = result.guidance.expect("the block carries guidance");
        assert!(guidance.contains("cannot owe itself"), "{guidance}");
    }

    /// Partial discharge is the normal state of a multi-entry declaration, so
    /// the result reports per entry rather than rounding to blocked-or-clear.
    #[test]
    fn a_partially_discharged_declaration_reports_both_halves() {
        let tmp = tempdir().unwrap();
        seed(
            tmp.path(),
            &DECLARES_IMPACT.replace(
                "cross-spec-impact: [050-upstream]",
                "cross-spec-impact: [050-upstream, 051-other]",
            ),
        );
        seed_target(tmp.path(), "Required by [007](../007-gate/spec.md).");
        fs::create_dir_all(tmp.path().join("specs/051-other")).unwrap();
        fs::write(
            tmp.path().join("specs/051-other/spec.md"),
            "---\nstatus: done\ndependencies: []\n---\n\n# 051 — Other\n",
        )
        .unwrap();

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();

        assert!(!result.passed);
        assert_eq!(
            result.cross_spec_impact,
            vec![
                CrossSpecImpactEntry {
                    spec: "050-upstream".into(),
                    state: CrossSpecImpactState::Discharged,
                },
                CrossSpecImpactEntry {
                    spec: "051-other".into(),
                    state: CrossSpecImpactState::Undischarged,
                },
            ]
        );
        let message = result.message.expect("the block carries a message");
        assert!(
            message.contains("051-other does not link back"),
            "{message}"
        );
        assert!(!message.contains("050-upstream"), "{message}");
    }

    #[test]
    fn an_absent_declaration_reports_nothing_and_blocks_nothing() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();

        assert!(result.passed);
        assert_eq!(result.cross_spec_impact, [] as [CrossSpecImpactEntry; 0]);
    }

    /// An empty list is the same state as an absent key — the posture
    /// `dependencies: []` already takes.
    #[test]
    fn an_empty_declaration_is_indistinguishable_from_an_absent_one() {
        let tmp = tempdir().unwrap();
        seed(
            tmp.path(),
            &DECLARES_IMPACT.replace("cross-spec-impact: [050-upstream]", "cross-spec-impact: []"),
        );

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();

        assert!(result.passed);
        assert_eq!(result.cross_spec_impact, [] as [CrossSpecImpactEntry; 0]);
    }

    /// The check sits ahead of the `review:` block, so an undischarged
    /// obligation is reported even on a spec that has never been reviewed —
    /// whether the review is fresh does not yet matter.
    #[test]
    fn an_undischarged_impact_outranks_a_missing_review() {
        let tmp = tempdir().unwrap();
        seed(
            tmp.path(),
            &DECLARES_IMPACT.replace(
                "  last-run: 2026-07-10T00:00:00Z\n  reviewed-against: abc123",
                "  last-run: null\n  reviewed-against: null",
            ),
        );
        seed_target(tmp.path(), "Body text that names nothing.");

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();

        assert_eq!(
            result.blocked_by,
            Some(ReviewGateBlock::UndischargedCrossSpecImpact)
        );
    }

    /// A pending fold never holds `done`, wherever its target lives (spec 051
    /// scenario `a-pending-fold-never-holds-done`). With the target in this
    /// tree — the case the gate used to block, since the fold could be done
    /// here — the spec passes when every other check does, exactly as it does
    /// with the target on another line. The fold is owed after `done`.
    #[test]
    fn a_pending_fold_never_holds_done_wherever_its_target_lives() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), DECLARES_FOLD);
        // Another line of development: nothing named `050-upstream` here.
        assert!(!tmp.path().join("specs/050-upstream").exists());
        let elsewhere = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(elsewhere.passed, "{:?}", elsewhere.message);
        assert_eq!(elsewhere.blocked_by, None);

        // This tree: the fold could be done here, and still does not block.
        seed_target(tmp.path(), "The upstream home.");
        let here = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(here.passed, "{:?}", here.message);
        assert_eq!(here.blocked_by, None);
    }

    /// The fold is not a pass either: a spec declaring one meets every other
    /// check exactly as a spec without one does — here a review recording a
    /// MUST violation, with the fold target in this tree.
    #[test]
    fn a_declared_fold_leaves_every_other_check_to_decide() {
        let tmp = tempdir().unwrap();
        seed(
            tmp.path(),
            &DECLARES_FOLD
                .replacen("  must-violations: 0\n", "  must-violations: 1\n", 1)
                .replacen(
                    "  blocking: false\nanalyze:",
                    "  blocking: true\nanalyze:",
                    1,
                ),
        );
        seed_target(tmp.path(), "The upstream home.");

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();

        assert_eq!(result.blocked_by, Some(ReviewGateBlock::MustViolations));
    }

    /// The gap this block closes. A spec with a clean review and no analyze
    /// record used to pass the gate — which is how two specs reached `done`
    /// on 2026-09-05 with only half the pipeline run, one of them published
    /// irreversibly before anything noticed, because nothing could.
    #[test]
    fn a_reviewed_but_unanalyzed_spec_is_blocked() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), NEVER_ANALYZED);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::NotAnalyzed));
        assert!(
            result
                .message
                .as_deref()
                .unwrap()
                .contains("has not been analyzed")
        );
    }

    #[test]
    fn a_null_analyze_last_run_blocks_like_an_absent_block() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), ANALYZE_NULL_RUN);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::NotAnalyzed));
    }

    /// Spec 082: a passing, current analysis that names a loaded rule it never
    /// judged holds `done`, and the block names each rule.
    #[test]
    fn an_analysis_naming_unjudged_rules_blocks_and_names_them() {
        let tmp = tempdir().unwrap();
        seed(
            tmp.path(),
            &ANALYZE_ADVISORY_ONLY.replace(
                "  unexamined: 4\n",
                "  unexamined: 4\n  unjudged-rules:\n    - TS-MUST-001\n    - TS-MUST-002\n",
            ),
        );
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::UnjudgedRules));
        let message = result.message.as_deref().unwrap();
        assert!(message.contains("TS-MUST-001, TS-MUST-002"), "{message}");
        let guidance = result.guidance.as_deref().unwrap();
        assert!(guidance.contains("/ductus:analyze"), "{guidance}");
    }

    /// The other direction: an analysis with no `unjudged-rules` field, one from
    /// before spec 082 included, or an empty list blocks on nothing here.
    #[test]
    fn an_analysis_with_no_unjudged_rules_passes() {
        for record in [
            ANALYZE_ADVISORY_ONLY.to_string(),
            ANALYZE_ADVISORY_ONLY.replace(
                "  unexamined: 4\n",
                "  unexamined: 4\n  unjudged-rules: []\n",
            ),
        ] {
            let tmp = tempdir().unwrap();
            seed(tmp.path(), &record);
            let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
            assert!(result.passed, "{:?}", result.message);
            assert_eq!(result.blocked_by, None);
        }
    }

    #[test]
    fn blocking_analyze_findings_block_and_name_both_tiers() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), ANALYZE_BLOCKING);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::AnalyzeFindings));
        let message = result.message.as_deref().unwrap();
        assert!(message.contains("1 hard-fail"), "{message}");
        assert!(message.contains("2 blocking"), "{message}");
        // Spec 076: a deferral can release a gating analyze finding, so the
        // guidance names deferral beside fixing.
        let guidance = result.guidance.as_deref().unwrap();
        assert!(
            guidance.contains("defer each to pipeline work on another spec"),
            "{guidance}"
        );
    }

    /// Advisory findings are recorded and never gate — the deliberate
    /// asymmetry with the review gate's treatment of an outstanding SHOULD.
    /// Neither they nor the unexamined targets block, so a record carrying
    /// only those passes; their counts ride an `analyze-findings` block's
    /// guidance, never a passing verdict's.
    #[test]
    fn advisory_findings_and_unexamined_targets_do_not_block() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), ANALYZE_ADVISORY_ONLY);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed);
        assert!(result.blocked_by.is_none());
    }

    /// Gate order: a spec failing both gates is told about the review, not
    /// the analysis. Naming the later gate for an earlier defect sends a
    /// contributor to the wrong command.
    #[test]
    fn a_missing_review_is_reported_before_a_missing_analysis() {
        let tmp = tempdir().unwrap();
        seed(
            tmp.path(),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# 007 — Gate\n",
        );
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::NotReviewed));
    }

    #[test]
    fn passes_when_lint_clean_and_review_current() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed);
        assert!(result.blocked_by.is_none());
        assert!(result.message.is_none());
        assert_eq!(result.violations, [] as [MarkdownViolation; 0]);
        // This fixture is a bare tempdir with no git repository, so the gate
        // genuinely cannot inspect the working tree and says so. The
        // assertion used to read `guidance.is_none()`, which is precisely the
        // conflation this scenario removes: silence has to mean "examined and
        // clean", and it cannot mean that here. The genuinely-clean case is
        // `a_clean_tree_emits_no_unexaminable_guidance`, which commits first.
        // This fixture's analysis record carries no digest, so its freshness is
        // not judgeable — and the gate says so rather than passing in silence. A
        // review record's digest decides nothing at the gate (spec 081).
        let guidance = result
            .guidance
            .expect("an unjudgeable record must not read as a current one");
        assert!(!guidance.contains("Review-record freshness"), "{guidance}");
        assert!(guidance.contains("Analyze-record freshness"), "{guidance}");
    }

    // --- fixtures for the record checks --------------------------------------

    /// Init a repo, stage everything, commit; returns the commit sha.
    fn git_commit_all(repo: &Path, message: &str) -> String {
        let repository =
            git2::Repository::open(repo).unwrap_or_else(|_| git2::Repository::init(repo).unwrap());
        let mut index = repository.index().unwrap();
        index
            .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
            .unwrap();
        index.write().unwrap();
        let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
        let sig = git2::Signature::now("Test", "test@example.com").unwrap();
        let parent = repository
            .head()
            .ok()
            .and_then(|h| h.target())
            .and_then(|oid| repository.find_commit(oid).ok());
        let parents: Vec<&git2::Commit> = parent.as_ref().into_iter().collect();
        repository
            .commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)
            .unwrap()
            .to_string()
    }

    /// A spec whose review points at `review_sha` and whose analysis points at
    /// `analyze_sha`, so a test can move one without the other.
    fn seed_records_at(repo: &Path, review_sha: &str, analyze_sha: &str) {
        fs::write(
            repo.join("specs/007-gate/spec.md"),
            format!(
                "---\nstatus: in-progress\ndependencies: []\nreview:\n  last-run: 2026-07-10T00:00:00Z\n  \
                 reviewed-against: {review_sha}\n  must-violations: 0\n  should-violations: 0\n  \
                 low-confidence: 0\n  blocking: false\nanalyze:\n  last-run: 2026-07-10T00:00:00Z\n  analyzed-against: {analyze_sha}\n  hard-fail: 0\n  blocking-findings: 0\n  advisory: 2\n  unexamined: 0\n  blocking: false\n---\n\n# 007 — Gate\n"
            ),
        )
        .unwrap();
    }

    #[test]
    fn a_contract_change_since_the_review_holds_done_only_on_the_analysis() {
        // A review reads code and an analysis the artifacts (spec 081): a changed
        // scenario stales the analysis, which holds `done`, and blocks on no review
        // check; a code change after it owes the review.
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        fs::create_dir_all(tmp.path().join("specs/007-gate/scenarios")).unwrap();
        fs::write(
            tmp.path().join("specs/007-gate/scenarios/retry.md"),
            "# Retry\n\n## Open Questions\n\n*None.*\n",
        )
        .unwrap();
        let base = git_commit_all(tmp.path(), "base");

        seed_with_current_digest(tmp.path(), &base);
        fs::write(
            tmp.path().join("specs/007-gate/scenarios/retry.md"),
            "# Retry\n\nNew contract text.\n\n## Open Questions\n\n*None.*\n",
        )
        .unwrap();
        git_commit_all(tmp.path(), "docs(007): change a durable contract");

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed, "{result:?}");
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::AnalyzeStale));

        fs::create_dir_all(tmp.path().join("src")).unwrap();
        fs::write(tmp.path().join("src/code.rs"), "fn code() {}\n").unwrap();
        git_commit_all(tmp.path(), "fix(007): change the code");
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(
            result.blocked_by,
            Some(ReviewGateBlock::ReviewOwed),
            "{result:?}"
        );
        assert!(
            result.message.unwrap().contains("src/code.rs"),
            "the owed review names the code that changed"
        );
    }

    #[test]
    fn lint_violations_block_before_review_state_is_consulted() {
        let tmp = tempdir().unwrap();
        // Even a never-reviewed spec reports the lint block first — the
        // gate's documented order.
        seed(tmp.path(), NEVER_REVIEWED);
        let violation = MarkdownViolation {
            path: "specs/007-gate/plan.md".into(),
            line: 12,
            rule: "MD012".into(),
            message: "Multiple consecutive blank lines".into(),
        };
        let canned = violation.clone();
        let result = run_with_lint(&args(), tmp.path(), move |_, _| {
            Ok(lint_markdown::Lint {
                violations: vec![canned],
                clean: false,
                exit_code: 1,
            })
        })
        .unwrap();
        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::MarkdownLint));
        assert_eq!(
            result.message.as_deref(),
            Some(
                "blocked: 1 markdownlint violation(s) in specs/007-gate — resolve them before completing"
            )
        );
        assert_eq!(result.violations, vec![violation]);
    }

    #[test]
    fn unparseable_lint_failure_blocks_with_exit_code() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let result = run_with_lint(&args(), tmp.path(), |_, _| {
            Ok(lint_markdown::Lint {
                violations: vec![],
                clean: false,
                exit_code: 2,
            })
        })
        .unwrap();
        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::MarkdownLint));
        assert_eq!(
            result.message.as_deref(),
            Some(
                "blocked: markdownlint-cli2 exited 2 for specs/007-gate — resolve the lint failure before completing"
            )
        );
        assert_eq!(result.violations, [] as [MarkdownViolation; 0]);
    }

    #[test]
    fn a_lint_that_examined_no_file_blocks_and_says_so() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let result = run_with_lint(&args(), tmp.path(), |_, _| {
            Ok(lint_markdown::Lint {
                violations: vec![],
                clean: false,
                exit_code: 0,
            })
        })
        .unwrap();
        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::MarkdownLint));
        assert!(
            result
                .message
                .as_deref()
                .is_some_and(|m| m.contains("linted no file in specs/007-gate")),
            "{:?}",
            result.message
        );
    }

    #[test]
    fn lint_receives_the_recursive_feature_dir_glob() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let mut seen: Option<LintMarkdownArgs> = None;
        run_with_lint(&args(), tmp.path(), |lint_args, _| {
            seen = Some(lint_args.clone());
            clean_lint(lint_args, Path::new(""))
        })
        .unwrap();
        let seen = seen.unwrap();
        assert_eq!(seen.paths, vec!["specs/007-gate/**/*.md".to_string()]);
        assert!(!seen.fix, "the gate never lints in fix mode");
    }

    #[test]
    fn null_last_run_blocks_not_reviewed() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), NEVER_REVIEWED);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::NotReviewed));
        assert_eq!(
            result.message.as_deref(),
            Some("blocked: spec has not been reviewed — run /ductus:review before completing")
        );
        assert!(result.guidance.is_none());
    }

    #[test]
    fn absent_review_block_blocks_not_reviewed() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), NO_REVIEW_BLOCK);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::NotReviewed));
    }

    #[test]
    fn blocking_review_blocks_with_must_count_and_waive_guidance() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_BLOCKING);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::MustViolations));
        assert_eq!(
            result.message.as_deref(),
            Some("blocked: spec has 3 MUST violation(s) — see specs/007-gate/review.md")
        );
        let guidance = result.guidance.unwrap();
        assert!(guidance.contains("re-run /ductus:review"), "{guidance}");
        assert!(guidance.contains("--waive <rule-id>"), "{guidance}");
        // Spec 076: a violation to be fixed later is deferred, not waived, so
        // the guidance names the deferral that releases the gate.
        assert!(
            guidance.contains("--defer <rule-id> --target <path>")
                && guidance.contains("another spec"),
            "{guidance}"
        );
    }

    #[test]
    fn honors_host_project_in_messages() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), NEVER_REVIEWED);
        fs::write(
            tmp.path().join(".govern.toml"),
            "[host]\nproject = \"acme\"\n",
        )
        .unwrap();
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(
            result.message.as_deref(),
            Some("blocked: spec has not been reviewed — run /acme:review before completing")
        );
    }

    #[test]
    fn honors_configured_specs_root() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("governance/007-gate")).unwrap();
        let spec = relocate_records_in(&tmp.path().join("governance/007-gate"), REVIEWED_CLEAN);
        fs::write(tmp.path().join("governance/007-gate/spec.md"), &spec).unwrap();
        fs::write(
            tmp.path().join(".govern.toml"),
            "[host]\nproject = \"ductus\"\n\n[paths]\nspecs-root = \"governance\"\n",
        )
        .unwrap();
        let mut seen_glob = String::new();
        let result = run_with_lint(&args(), tmp.path(), |lint_args, _| {
            seen_glob = lint_args.paths[0].clone();
            Ok(lint_markdown::Lint {
                violations: vec![],
                clean: true,
                exit_code: 0,
            })
        })
        .unwrap();
        assert!(result.passed);
        assert_eq!(seen_glob, "governance/007-gate/**/*.md");
    }

    #[test]
    fn missing_feature_directory_errors() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let err = run_with_lint(
            &CheckReviewGateArgs {
                feature: "099-absent".into(),
            },
            tmp.path(),
            clean_lint,
        )
        .unwrap_err();
        assert!(matches!(err, PrimitiveError::FeatureNotFound { .. }));
    }

    /// Write a scenario file under the seeded feature.
    fn seed_scenario(repo: &Path, name: &str, body: &str) {
        let dir = repo.join("specs/007-gate/scenarios");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(name), body).unwrap();
    }

    const SCENARIO_WITH_QUESTIONS: &str = "---\nsection: Behavior\n---\n\n# Wire contract\n\n## Open Questions\n\n- Bracket operator or empty operand?\n";

    #[test]
    fn blocks_on_unresolved_scenario_questions_naming_the_scenario() {
        let tmp = tempdir().unwrap();
        // Review is current and lint is clean — only the scenario question
        // stands between this spec and `done`.
        seed(tmp.path(), REVIEWED_CLEAN);
        seed_scenario(tmp.path(), "wire-contract.md", SCENARIO_WITH_QUESTIONS);

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed);
        assert_eq!(
            result.blocked_by,
            Some(ReviewGateBlock::ScenarioOpenQuestions)
        );
        let message = result.message.unwrap();
        assert!(
            message.contains("wire-contract"),
            "the blocked message must name the scenario, got: {message}"
        );
        assert!(message.contains('1'), "and the count, got: {message}");
        // The guidance points at the command that can actually resolve
        // them — scenario-targeted clarify, not feature-targeted.
        assert!(result.guidance.unwrap().contains("/ductus:clarify"));
    }

    #[test]
    fn scenario_questions_do_not_block_when_all_are_resolved() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        seed_scenario(
            tmp.path(),
            "settled.md",
            "---\nsection: Behavior\n---\n\n## Open Questions\n\n*None — captured during scenario authoring.*\n\n## Resolved Questions\n\n- **Answered** — yes.\n",
        );
        assert!(
            run_with_lint(&args(), tmp.path(), clean_lint)
                .unwrap()
                .passed
        );
    }

    #[test]
    fn markdown_lint_still_wins_over_the_scenario_check() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        seed_scenario(tmp.path(), "wire-contract.md", SCENARIO_WITH_QUESTIONS);

        // Both checks would block; gate order must report the first.
        let dirty = |_: &LintMarkdownArgs, _: &Path| {
            Ok(lint_markdown::Lint {
                violations: vec![MarkdownViolation {
                    path: "specs/007-gate/spec.md".into(),
                    line: 3,
                    rule: "MD012".into(),
                    message: "Multiple consecutive blank lines".into(),
                }],
                clean: false,
                exit_code: 1,
            })
        };
        let result = run_with_lint(&args(), tmp.path(), dirty).unwrap();
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::MarkdownLint));
    }

    #[test]
    fn scenario_questions_are_reported_before_a_missing_review() {
        let tmp = tempdir().unwrap();
        // Never reviewed AND carrying a scenario question: the scenario
        // check is ordered first, so the contributor resolves the design
        // question before being sent to review code its answer may change.
        seed(tmp.path(), NEVER_REVIEWED);
        seed_scenario(tmp.path(), "wire-contract.md", SCENARIO_WITH_QUESTIONS);

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(
            result.blocked_by,
            Some(ReviewGateBlock::ScenarioOpenQuestions)
        );
    }

    #[test]
    fn a_question_deferred_into_resolved_questions_does_not_block() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        // The convention spec 046 settles on: a question that is deferred
        // rather than undecided is resolved *with a condition* and lives in
        // Resolved Questions. There is deliberately no exemptible section
        // under Open Questions — that would let anything blocking be
        // relabelled to ship past this gate.
        seed_scenario(
            tmp.path(),
            "deferred.md",
            "---\nsection: Behavior\n---\n\n## Open Questions\n\n*None — all resolved.*\n\n## Resolved Questions\n\n- **Should Family 10 enforce uniqueness?** Deferred — revisit when the first sunset commit lands.\n",
        );
        assert!(
            run_with_lint(&args(), tmp.path(), clean_lint)
                .unwrap()
                .passed
        );
    }

    #[test]
    fn the_gate_guidance_offers_both_exits() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        seed_scenario(tmp.path(), "wire-contract.md", SCENARIO_WITH_QUESTIONS);
        let guidance = run_with_lint(&args(), tmp.path(), clean_lint)
            .unwrap()
            .guidance
            .unwrap();
        assert!(guidance.contains("clarify"), "resolve exit: {guidance}");
        assert!(
            guidance.contains("Resolved Questions"),
            "defer exit: {guidance}"
        );
    }

    #[test]
    fn an_unparseable_scenario_never_blocks_the_gate() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        // Truncated frontmatter and no questions section: nothing can be
        // proven about it, and an unknown is never escalated into a
        // done-blocking finding.
        seed_scenario(tmp.path(), "broken.md", "---\nsection: Behavior\n");
        assert!(
            run_with_lint(&args(), tmp.path(), clean_lint)
                .unwrap()
                .passed
        );
    }

    #[test]
    fn feature_without_scenarios_is_unaffected() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        assert!(
            run_with_lint(&args(), tmp.path(), clean_lint)
                .unwrap()
                .passed
        );
    }

    #[test]
    fn rejects_traversal_and_absolute_feature() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        for bad in ["../007-gate", "/etc", ""] {
            let err = run_with_lint(
                &CheckReviewGateArgs {
                    feature: bad.into(),
                },
                tmp.path(),
                clean_lint,
            )
            .unwrap_err();
            assert!(
                matches!(err, PrimitiveError::InvalidPath { .. }),
                "expected InvalidPath for {bad:?}"
            );
        }
    }

    // --- uncommitted contracts: now a comparison, not a notice --------------
    //
    // These replace the `unexaminable_contracts_guidance` tests. That notice
    // existed because the staleness check compared committed trees and so
    // could not see a contract living only in the working tree. The digest
    // comparison reads the tree directly, so the state the notice described is
    // now *answered* rather than reported — and a notice that can no longer
    // fire is indistinguishable from one that passed, which is why it went
    // rather than staying alongside.

    /// A pre-digest `review:` block raises no freshness notice: a review record's
    /// digest decides nothing at the gate (spec 081).
    #[test]
    fn a_review_record_with_no_digest_raises_no_review_notice() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let base = git_commit_all(tmp.path(), "base");
        seed_records_at(tmp.path(), &base, &base);
        git_commit_all(tmp.path(), "a pre-digest record");

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "{result:?}");
        let guidance = result.guidance.unwrap_or_default();
        assert!(!guidance.contains("Review-record freshness"), "{guidance}");
        assert!(!guidance.contains("no reviewed-digest"), "{guidance}");
    }

    // --- gate check 13: analyze staleness -----------------------------------
    //
    // The *comparison* is tested in `crate::primitives::analyze_subjects` —
    // subject-set membership, the `analyze:`-block exclusion, deleted and
    // unreadable subjects, and the property that the answer does not depend on
    // commit status. What belongs here is the gate wiring: that the block is
    // reachable, ordered correctly among the other checks, and that an
    // unjudgeable record reports rather than blocks.

    /// Seed a spec whose review points at `review_sha` and whose analyze
    /// record carries a digest of the feature's subjects **as they are now**.
    fn seed_with_current_digest(repo: &Path, review_sha: &str) {
        seed_with_current_digest_over(repo, review_sha, "");
    }

    /// [`seed_with_current_digest`] with `body` appended to `spec.md` before
    /// either digest is taken.
    fn seed_with_current_digest_over(repo: &Path, review_sha: &str, body: &str) {
        let dir = repo.join("specs/007-gate");
        seed_records_at(repo, review_sha, review_sha);
        let spec = dir.join("spec.md");
        let mut text = fs::read_to_string(&spec).unwrap();
        text.push_str(body);
        fs::write(&spec, text).unwrap();

        // In pipeline order: the review's digest, then its block, and only
        // then the analysis — which digests the `spec.md` the review just
        // rewrote. Taking the analyze digest first would leave the fixture
        // permanently stale, because the `review:` block is one of its
        // subjects. That is the real ordering constraint, not an artefact of
        // the fixture.
        let contracts = crate::primitives::analyze_subjects::subject_digest(
            &dir,
            crate::primitives::analyze_subjects::is_review_contract,
        );
        let mut review = String::from("review:\n  last-run: 2026-07-10T00:00:00Z\n");
        let _ = writeln!(review, "  reviewed-against: {review_sha}");
        review.push_str("  must-violations: 0\n  should-violations: 0\n  low-confidence: 0\n");
        if contracts.digests.is_empty() {
            review.push_str("  reviewed-digest: {}\n");
        } else {
            review.push_str("  reviewed-digest:\n");
            for (path, digest) in &contracts.digests {
                let _ = writeln!(review, "    {path}: {digest}");
            }
        }
        review.push_str("  blocking: false\n");
        splice_block_into(&spec, "review", &review);

        let subjects = crate::primitives::analyze_subjects::subject_digest(
            &dir,
            crate::primitives::analyze_subjects::is_analyze_subject,
        );
        let mut analyze = String::from("analyze:\n  last-run: 2026-07-10T00:00:00Z\n");
        let _ = writeln!(analyze, "  analyzed-against: {review_sha}");
        analyze
            .push_str("  hard-fail: 0\n  blocking-findings: 0\n  advisory: 2\n  unexamined: 0\n");
        analyze.push_str("  analyzed-digest:\n");
        for (path, digest) in &subjects.digests {
            let _ = writeln!(analyze, "    {path}: {digest}");
        }
        analyze.push_str("  blocking: false\n");
        splice_block_into(&spec, "analyze", &analyze);
    }

    /// Write one record into the artifact that owns it, de-indenting the
    /// fixture's nested block form.
    ///
    /// Named for what it did before the relocation — the callers still hand it
    /// a `key:\n  field: …` block, because that is the shape the digest
    /// builders produce and reshaping them would be a second edit to fixtures
    /// whose point is the digest, not its indentation.
    fn splice_block_into(spec: &Path, key: &str, block: &str) {
        let dir = spec.parent().unwrap();
        let file = if key == "review" {
            "review.md"
        } else {
            "analysis.md"
        };
        let mut body = String::new();
        for line in block.lines().skip(1) {
            body.push_str(line.strip_prefix("  ").unwrap_or(line));
            body.push('\n');
        }
        // A post-058 record, as `relocate_records_in` writes one.
        if !body.contains("dispositions:") {
            body.push_str(
                "dispositions:\n  fixed: 0\n  deferred: 0\n  waived: 0\n  undispositioned: 0\n",
            );
        }
        fs::write(
            dir.join(file),
            format!("---\nspec: 007-gate\n{body}---\n\n# {key} — 007-gate\n"),
        )
        .unwrap();
    }

    /// A spec with work owed is no candidate for `done`, wherever the box sits, and the
    /// message names it. An unresolved scenario question is more upstream, so it reads
    /// first (owner, 2026-11-06).
    #[test]
    fn an_unchecked_box_blocks_wherever_it_sits() {
        for tasks in [
            "# T\n\n## 1. A\n\n- [x] a\n- [ ] owed\n",
            "# T\n\n## 1. A\n\n- [x] a\n\n## Later\n\n- [ ] owed\n",
            "# T\n\n## 1. A\n\n- [x] a\n\n## 1a. B\n\n- [ ] owed\n",
            "# T\n\n- [ ] owed\n\n## 1. A\n\n- [x] a\n",
        ] {
            let tmp = tempdir().unwrap();
            seed(tmp.path(), REVIEWED_CLEAN);
            fs::write(tmp.path().join("specs/007-gate/tasks.md"), tasks).unwrap();
            let blocked = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
            assert!(!blocked.passed, "{tasks}: {blocked:?}");
            assert_eq!(blocked.blocked_by, Some(ReviewGateBlock::UncheckedTask));
            let message = blocked.message.unwrap();
            assert!(message.contains("1 unchecked box"), "{message}");
            assert!(message.contains("owed"), "{message}");
        }

        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        fs::write(
            tmp.path().join("specs/007-gate/tasks.md"),
            "# T\n\n## 1. A\n\n- [ ] owed\n",
        )
        .unwrap();
        seed_scenario(
            tmp.path(),
            "retry.md",
            "---\nsection: x\n---\n\n# R\n\n## Open Questions\n\n- Which?\n",
        );
        let blocked = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(
            blocked.blocked_by,
            Some(ReviewGateBlock::ScenarioOpenQuestions)
        );
    }

    /// A comment's example box is none, and a fully ticked list or no list owes nothing.
    #[test]
    fn a_comment_s_box_or_a_ticked_list_passes_the_task_check() {
        for tasks in [
            Some("# T\n\n<!--\n- [ ] an example\n-->\n\n## 1. A\n\n- [x] a\n"),
            Some("# T\n\n## 1. A\n\n- [x] a\n"),
            None,
        ] {
            let tmp = tempdir().unwrap();
            seed(tmp.path(), REVIEWED_CLEAN);
            if let Some(tasks) = tasks {
                fs::write(tmp.path().join("specs/007-gate/tasks.md"), tasks).unwrap();
            }
            let base = git_commit_all(tmp.path(), "base");
            seed_with_current_digest(tmp.path(), &base);
            git_commit_all(tmp.path(), "the records");
            let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
            assert!(result.passed, "{tasks:?}: {result:?}");
        }
    }

    /// `analysis.md` is no subject of its own analysis, so a `decisions:` list damaged
    /// after the run reads as current. `validate-frontmatter` calls it a hard fail, and
    /// `write-analysis` refuses to record over it, so the message says to repair it and
    /// does not name a re-run (spec 079).
    #[test]
    fn an_analysis_whose_decisions_do_not_parse_blocks() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let base = git_commit_all(tmp.path(), "base");
        seed_with_current_digest(tmp.path(), &base);
        git_commit_all(tmp.path(), "the records");
        let clean = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(clean.passed, "{clean:?}");
        let file = tmp.path().join("specs/007-gate/analysis.md");
        let text = fs::read_to_string(&file).unwrap();
        fs::write(
            &file,
            text.replacen(
                "spec: 007-gate\n",
                "spec: 007-gate\ndecisions: not-a-list\n",
                1,
            ),
        )
        .unwrap();
        let blocked = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!blocked.passed, "{blocked:?}");
        assert_eq!(
            blocked.blocked_by,
            Some(ReviewGateBlock::AnalyzeRecordUnreadable)
        );
        let message = blocked.message.unwrap();
        assert!(message.contains("decisions:"), "{message}");
        let guidance = blocked.guidance.unwrap();
        assert!(guidance.contains("Repair"), "{guidance}");
        assert!(!guidance.contains("re-run"), "{guidance}");
    }

    /// The hole this check closes: `review → fix → done` used to pass the gate
    /// on an analysis recorded before the fixes, because the presence check
    /// asks only whether `last-run` is set.
    #[test]
    fn a_stale_analyze_record_blocks_and_names_the_changed_artifacts() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let base = git_commit_all(tmp.path(), "base");
        seed_with_current_digest(tmp.path(), &base);

        // A subject changes after the record was taken.
        fs::write(
            tmp.path().join("specs/007-gate/tasks.md"),
            "# Tasks\n\n## 1. Work\n\n- [x] Do it\n\n- **Done when**: done.\n",
        )
        .unwrap();

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed, "{result:?}");
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::AnalyzeStale));
        let message = result.message.unwrap();
        assert!(message.contains("analysis is stale"), "{message}");
        assert!(message.contains("tasks.md"), "{message}");
        assert!(
            result.guidance.unwrap().contains("/ductus:analyze"),
            "guidance must name the command that clears it"
        );
    }

    /// A record whose digest still matches passes, and says nothing — the
    /// silence is what makes the notice below meaningful.
    #[test]
    fn a_current_analyze_record_passes_quietly() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let base = git_commit_all(tmp.path(), "base");
        seed_with_current_digest(tmp.path(), &base);
        git_commit_all(tmp.path(), "commit the record");

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "{result:?}");
        assert!(result.blocked_by.is_none());
        assert!(
            !result
                .guidance
                .as_deref()
                .unwrap_or_default()
                .contains("freshness could not be determined"),
            "a matching digest is determinable: {:?}",
            result.guidance
        );
    }

    /// A criterion ticked and another unticked through `mark-criterion` after
    /// the analysis leave it current, so the gate passes; rewording a
    /// criterion is any other `spec.md` edit, and stales it (066 scenario
    /// `checkbox-state-is-outside-the-analysis-digest`).
    #[test]
    fn a_criterion_tick_leaves_the_analysis_current_and_another_spec_edit_stales_it() {
        use crate::primitives::mark_criterion;
        use crate::schema::primitives::MarkCriterionArgs;
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let spec = tmp.path().join("specs/007-gate/spec.md");
        let base = git_commit_all(tmp.path(), "base");
        seed_with_current_digest_over(
            tmp.path(),
            &base,
            "\n## Acceptance Criteria\n\n- [ ] AC1: First.\n- [x] AC2: Second.\n",
        );
        git_commit_all(tmp.path(), "commit the record");

        for (label, checked) in [("AC1", true), ("AC2", false)] {
            let flipped = mark_criterion::run(
                &MarkCriterionArgs {
                    feature: "007-gate".into(),
                    criterion_index: None,
                    label: Some(label.into()),
                    checked,
                },
                tmp.path(),
            )
            .unwrap();
            assert_ne!(flipped.previous, checked, "{label} was not flipped");
        }
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "a tick staled the analysis: {result:?}");

        let ticked = fs::read_to_string(&spec).unwrap();
        fs::write(
            &spec,
            ticked.replace("AC1: First.", "AC1: First, reworded."),
        )
        .unwrap();
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::AnalyzeStale));
        assert!(result.message.unwrap().contains("spec.md"));
    }

    /// Every record written before the digest existed. It does not block — a
    /// gate that blocked on its own inability to check is one people route
    /// around — and it does not pass in silence either.
    #[test]
    fn a_record_with_no_digest_passes_with_a_notice() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let base = git_commit_all(tmp.path(), "base");
        seed_records_at(tmp.path(), &base, &base);
        git_commit_all(tmp.path(), "a pre-digest record");

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "{result:?}");
        assert!(result.blocked_by.is_none());
        let guidance = result.guidance.unwrap();
        assert!(
            guidance.contains("freshness could not be determined"),
            "{guidance}"
        );
        assert!(guidance.contains("no analyzed-digest"), "{guidance}");
    }

    // --- gate check 0: the spec is already done ------------------------------

    /// The transition this gate authorizes has already happened, so it does
    /// not run the checks — and does not claim to have passed them.
    #[test]
    fn a_done_spec_short_circuits_without_claiming_to_have_passed() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), ALREADY_DONE);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed, "a gate that did not examine must not pass");
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::AlreadyDone));
        let message = result.message.unwrap();
        assert!(message.contains("already at status done"), "{message}");
        assert!(
            result.guidance.unwrap().contains("/ductus:amend"),
            "the guidance names the reopen path"
        );
    }

    /// Ordered ahead of the lint, so a completed spec is never held against a
    /// violation introduced long after it closed.
    #[test]
    fn a_done_spec_is_not_held_against_a_later_lint_violation() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), ALREADY_DONE);
        let dirty_lint = |_: &LintMarkdownArgs, _: &Path| {
            Ok(lint_markdown::Lint {
                violations: vec![MarkdownViolation {
                    path: "specs/007-gate/spec.md".into(),
                    line: 1,
                    rule: "MD041".into(),
                    message: "first line in a file should be a top-level heading".into(),
                }],
                clean: false,
                exit_code: 1,
            })
        };
        let result = run_with_lint(&args(), tmp.path(), dirty_lint).unwrap();
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::AlreadyDone));
        assert!(
            result.violations.is_empty(),
            "the lint never ran, so there are no violations to report"
        );
    }

    /// The defect this closes, as a test: a `done` spec whose `spec.md` was
    /// rewritten by its own completing `set-status` used to report a stale
    /// analysis and send the operator to re-run it.
    #[test]
    fn a_done_spec_is_not_reported_as_having_a_stale_analysis() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let base = git_commit_all(tmp.path(), "base");
        seed_with_current_digest(tmp.path(), &base);
        // The completing flip: status moves, which rewrites spec.md — one of
        // the digest's own subjects.
        let spec = tmp.path().join("specs/007-gate/spec.md");
        let text = fs::read_to_string(&spec).unwrap();
        fs::write(&spec, text.replace("status: in-progress", "status: done")).unwrap();

        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(
            result.blocked_by,
            Some(ReviewGateBlock::AlreadyDone),
            "a finished spec must not be told its analysis is stale: {result:?}"
        );
    }

    #[test]
    fn an_in_progress_spec_still_runs_every_check() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), NEVER_ANALYZED);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::NotAnalyzed));
    }

    /// An unrecognized status falls through rather than being read as
    /// finished. `validate-frontmatter` owns reporting the bad value.
    #[test]
    fn an_unrecognized_status_is_not_treated_as_done() {
        let tmp = tempdir().unwrap();
        seed(
            tmp.path(),
            "---\nstatus: finished\ndependencies: []\n---\n\n# 007 — Gate\n",
        );
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::NotReviewed));
    }

    // -- dispositions (spec 058) ------------------------------------------------

    const ZERO_MAP: &str =
        "dispositions:\n  fixed: 0\n  deferred: 0\n  waived: 0\n  undispositioned: 0\n";

    /// Rewrite a seeded record as a pre-058 run left it: no `dispositions:`.
    fn strip_dispositions(repo: &Path, file: &str) {
        let path = repo.join("specs/007-gate").join(file);
        let text = fs::read_to_string(&path).unwrap();
        assert!(
            text.contains(ZERO_MAP),
            "fixture record carries the map: {text}"
        );
        fs::write(&path, text.replace(ZERO_MAP, "")).unwrap();
    }

    fn set_undispositioned(repo: &Path, file: &str, count: u32) {
        let path = repo.join("specs/007-gate").join(file);
        let text = fs::read_to_string(&path).unwrap();
        fs::write(
            &path,
            text.replace(
                "  undispositioned: 0\n",
                &format!("  undispositioned: {count}\n"),
            ),
        )
        .unwrap();
    }

    #[test]
    fn a_review_that_predates_dispositions_blocks_and_names_its_command() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), ANALYZE_ADVISORY_ONLY);
        strip_dispositions(tmp.path(), "review.md");
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed);
        assert_eq!(
            result.blocked_by,
            Some(ReviewGateBlock::RecordPredatesDispositions)
        );
        let message = result.message.unwrap();
        assert_eq!(
            message,
            "blocked: specs/007-gate/review.md predates finding dispositions — re-run /ductus:review"
        );
        assert!(
            result
                .guidance
                .unwrap()
                .contains("does not read its absence as zero")
        );
    }

    /// Review first: its run stales the analysis, and the analysis then writes its map.
    #[test]
    fn a_review_with_no_map_is_named_before_an_analysis_with_none() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), ANALYZE_ADVISORY_ONLY);
        strip_dispositions(tmp.path(), "review.md");
        strip_dispositions(tmp.path(), "analysis.md");
        let message = run_with_lint(&args(), tmp.path(), clean_lint)
            .unwrap()
            .message
            .unwrap();
        assert_eq!(
            message,
            "blocked: specs/007-gate/review.md predates finding dispositions — re-run /ductus:review"
        );
    }

    // --- gate check 9: a fix round not re-read (spec 078) ------------------

    /// Add `yaml` to the end of the seeded `review.md`'s frontmatter.
    fn with_review_fields(repo: &Path, yaml: &str) {
        let path = repo.join("specs/007-gate/review.md");
        let text = fs::read_to_string(&path).unwrap();
        let close = "---\n\n# review";
        assert!(text.contains(close), "{text}");
        fs::write(&path, text.replacen(close, &format!("{yaml}{close}"), 1)).unwrap();
    }

    const LOOP_FIELDS: &str = "fix-rounds: 2\nre-reads: 1\ndiff-checked: 0\nbudget-stop: false\n";

    fn round_waiver(round: u32, reason: &str) -> String {
        format!(
            "  - round: {round}\n    reason: \"{reason}\"\n    waived-by: dev@example.com\n    \
             waived-at: 2026-07-10T00:00:00Z\n"
        )
    }

    /// An un-re-read round with no waiver holds `done`, naming the count,
    /// with guidance to re-run the review.
    #[test]
    fn an_un_re_read_round_without_a_waiver_holds_done() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        with_review_fields(tmp.path(), &format!("{LOOP_FIELDS}not-re-read: 1\n"));
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::FixRoundNotReRead));
        assert_eq!(
            result.message.as_deref(),
            Some(
                "blocked: 1 fix round(s) in specs/007-gate/review.md needed a re-read and have \
                 none (1 not re-read, 0 waived)"
            )
        );
        assert!(result.guidance.unwrap().contains("Re-run /ductus:review"));
    }

    /// A review cut off between its two writes leaves the mark, which holds
    /// `done` and names the command that clears it (spec 079, AC14).
    #[test]
    fn an_open_fix_round_loop_holds_done() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        with_review_fields(tmp.path(), "fix-rounds-open: true\n");
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::FixRoundLoopOpen));
        assert!(
            result
                .message
                .as_deref()
                .unwrap()
                .contains("specs/007-gate/review.md is still open"),
            "{:?}",
            result.message
        );
        assert!(
            result
                .message
                .as_deref()
                .unwrap()
                .contains("re-run /ductus:review"),
            "{:?}",
            result.message
        );
        assert!(result.guidance.unwrap().contains("Re-run /ductus:review"));
    }

    /// Each state only a review run clears holds `done` at the gate and reads as
    /// held to `review-owed`, and a record that passes reads as neither, so the
    /// two cannot disagree (spec 079, AC4).
    #[test]
    fn what_the_gate_holds_until_a_review_runs_is_what_review_owed_reads() {
        for (fields, blocked_by) in [
            ("fix-rounds-open: true\n", ReviewGateBlock::FixRoundLoopOpen),
            ("not-re-read: 1\n", ReviewGateBlock::FixRoundNotReRead),
        ] {
            let tmp = tempdir().unwrap();
            seed(tmp.path(), REVIEWED_CLEAN);
            with_review_fields(tmp.path(), fields);
            let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
            assert_eq!(result.blocked_by, Some(blocked_by), "{fields}");
            let review = crate::primitives::load_review_record(&tmp.path().join("specs/007-gate"));
            assert!(
                holds_done_until_reviewed(review.as_present().unwrap()),
                "{fields}"
            );
        }

        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "{:?}", result.message);
        let review = crate::primitives::load_review_record(&tmp.path().join("specs/007-gate"));
        assert!(!holds_done_until_reviewed(review.as_present().unwrap()));
    }

    /// Each term alone holds the record, so none can be dropped unnoticed.
    #[test]
    fn each_state_only_a_review_clears_holds_the_record_alone() {
        use crate::schema::primitives::{Dispositions, ReviewBlock, RoundWaiver};
        let clean = ReviewBlock {
            dispositions: Some(Dispositions::default()),
            ..ReviewBlock::default()
        };
        assert!(!holds_done_until_reviewed(&clean));
        for (name, held) in [
            (
                "blocking",
                ReviewBlock {
                    blocking: true,
                    ..clean.clone()
                },
            ),
            (
                "a MUST",
                ReviewBlock {
                    must_violations: 1,
                    ..clean.clone()
                },
            ),
            (
                "a SHOULD",
                ReviewBlock {
                    should_violations: 1,
                    ..clean.clone()
                },
            ),
            (
                "no map",
                ReviewBlock {
                    dispositions: None,
                    ..clean.clone()
                },
            ),
            (
                "the open mark",
                ReviewBlock {
                    fix_rounds_open: Some(true),
                    ..clean.clone()
                },
            ),
            (
                "an unwaived round",
                ReviewBlock {
                    not_re_read: Some(1),
                    ..clean.clone()
                },
            ),
        ] {
            assert!(holds_done_until_reviewed(&held), "{name}");
        }
        let waived = ReviewBlock {
            not_re_read: Some(1),
            round_waivers: vec![RoundWaiver {
                round: 1,
                reason: "a reason".to_string(),
                waived_by: "dev@example.invalid".to_string(),
                waived_at: "2026-07-10T00:00:00Z".to_string(),
            }],
            ..clean.clone()
        };
        assert!(!holds_done_until_reviewed(&waived));
    }

    /// A record with no mark, older or from `ductus exec`, is unaffected: it
    /// passes with the notice that it does not say whether fixes were re-read.
    #[test]
    fn a_record_with_no_mark_is_unaffected() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "{:?}", result.message);
        assert!(result.guidance.unwrap().contains("`not-re-read` field"));
    }

    /// A waiver with a reason releases its round, and the passing gate says
    /// nothing about fix rounds; a waiver whose reason is blank waives nothing.
    #[test]
    fn a_waived_round_passes_and_a_blank_waiver_waives_nothing() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        with_review_fields(
            tmp.path(),
            &format!(
                "{LOOP_FIELDS}not-re-read: 1\nround-waivers:\n{}",
                round_waiver(2, "the change reverts round 1")
            ),
        );
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "{result:?}");
        assert!(
            !result.guidance.unwrap_or_default().contains("not-re-read"),
            "a record with the loop fields says nothing about them on a pass"
        );

        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        with_review_fields(
            tmp.path(),
            &format!(
                "{LOOP_FIELDS}not-re-read: 1\nround-waivers:\n{}",
                round_waiver(2, "  ")
            ),
        );
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::FixRoundNotReRead));
    }

    /// The check counts what the record states: fewer waivers than un-re-read
    /// rounds block, whatever the record's other counts say.
    #[test]
    fn fewer_waivers_than_un_re_read_rounds_block_whatever_else_the_record_says() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        with_review_fields(
            tmp.path(),
            &format!(
                "fix-rounds: 0\nre-reads: 5\nnot-re-read: 2\nround-waivers:\n{}",
                round_waiver(1, "declined")
            ),
        );
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::FixRoundNotReRead));
        assert!(
            result
                .message
                .unwrap()
                .contains("1 fix round(s) in specs/007-gate/review.md needed a re-read and have none (2 not re-read, 1 waived)")
        );
    }

    /// A record without the loop fields passes, and the guidance says it does
    /// not record whether its fixes were re-read; one with them at zero does
    /// not say so.
    #[test]
    fn a_record_without_the_loop_fields_passes_and_says_so() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "{result:?}");
        assert!(
            result
                .guidance
                .unwrap()
                .contains("does not say whether the fixes made during that review were re-read")
        );

        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        with_review_fields(tmp.path(), &format!("{LOOP_FIELDS}not-re-read: 0\n"));
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "{result:?}");
        assert!(!result.guidance.unwrap_or_default().contains("re-read"));
    }

    /// A finding the review left open holds `done` from the record itself, naming its
    /// file, so a window that cannot be read, as here, does not pass it (spec 079).
    #[test]
    fn a_finding_left_open_blocks_and_names_its_file() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        with_review_fields(
            tmp.path(),
            &format!(
                "{LOOP_FIELDS}not-re-read: 0\ncontract-breaks: 1\nopen-findings:\n  - src/a.rs\n"
            ),
        );
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(!result.passed, "{result:?}");
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::ReviewOwed));
        assert!(result.message.unwrap().contains("src/a.rs"));
    }

    /// A break a person waived, or one fixed since, is decided: it may still count, it
    /// lists nothing, and neither the gate nor its guidance has anything to say.
    #[test]
    fn a_contract_break_nothing_lists_as_open_passes_and_says_nothing() {
        for fields in ["contract-breaks: 1\n", "contract-breaks: 0\n", ""] {
            let tmp = tempdir().unwrap();
            seed(tmp.path(), REVIEWED_CLEAN);
            with_review_fields(
                tmp.path(),
                &format!("{LOOP_FIELDS}not-re-read: 0\n{fields}"),
            );
            let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
            assert!(result.passed, "{fields}: {result:?}");
            assert!(
                !result
                    .guidance
                    .unwrap_or_default()
                    .contains("contract break"),
                "{fields}"
            );
        }
    }

    /// Check 9 follows check 8: an outstanding SHOULD is reported first.
    #[test]
    fn an_outstanding_should_is_reported_before_an_un_re_read_round() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_CLEAN);
        with_review_fields(tmp.path(), &format!("{LOOP_FIELDS}not-re-read: 1\n"));
        let path = tmp.path().join("specs/007-gate/review.md");
        let text = fs::read_to_string(&path).unwrap();
        fs::write(
            &path,
            text.replace("should-violations: 0", "should-violations: 1"),
        )
        .unwrap();
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(
            result.blocked_by,
            Some(ReviewGateBlock::UndispositionedFindings)
        );
    }

    /// Spec 058's `findings-cost-in-proportion-to-risk`: a spec whose only
    /// undecided findings are advisory passes, in both records.
    #[test]
    fn undispositioned_advisory_findings_do_not_hold_done() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), ANALYZE_ADVISORY_ONLY);
        set_undispositioned(tmp.path(), "analysis.md", 2);
        set_undispositioned(tmp.path(), "review.md", 1);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "{:?}", result.message);
        assert_eq!(result.blocked_by, None);
    }

    /// The other direction: an outstanding SHOULD, a gating finding no
    /// earlier check covers, still holds `done`, whatever the map counts.
    #[test]
    fn an_outstanding_should_holds_done() {
        let tmp = tempdir().unwrap();
        seed(
            tmp.path(),
            &ANALYZE_ADVISORY_ONLY.replace("should-violations: 0", "should-violations: 2"),
        );
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(
            result.blocked_by,
            Some(ReviewGateBlock::UndispositionedFindings)
        );
        assert_eq!(
            result.message.as_deref(),
            Some("blocked: 2 outstanding gating finding(s) — specs/007-gate/review.md: 2")
        );
        let guidance = result.guidance.unwrap();
        assert!(guidance.contains("/ductus:review --waive"), "{guidance}");
        assert!(guidance.contains("do not hold `done`"), "{guidance}");
    }

    /// Insert a top-level `decisions:` list into the seeded record `file`.
    fn add_decisions(repo: &Path, file: &str, block: &str) {
        let path = repo.join("specs/007-gate").join(file);
        let text = fs::read_to_string(&path).unwrap();
        assert!(text.contains("  undispositioned: 0\n"), "{text}");
        fs::write(
            &path,
            text.replacen(
                "  undispositioned: 0\n",
                &format!("  undispositioned: 0\n{block}"),
                1,
            ),
        )
        .unwrap();
    }

    /// Spec 076: a gating finding whose deferral releases nothing — to this
    /// spec's own task, or to an external work item alone — is named with
    /// its work reference where the gate blocks on it; a deferral that is
    /// not gating is not.
    #[test]
    fn a_holding_deferral_is_named_where_the_gate_blocks() {
        let tmp = tempdir().unwrap();
        seed(
            tmp.path(),
            &ANALYZE_ADVISORY_ONLY.replace("should-violations: 0", "should-violations: 2"),
        );
        let stamp = "    decided-at: 2026-07-10T00:00:00Z\n    decided-by: dev@example.com\n";
        add_decisions(
            tmp.path(),
            "review.md",
            &format!(
                "decisions:\n  - key: \"rule SEC-1 at src/a.rs\"\n    outcome: deferred\n    \
                 work-item: PROJ-1\n    gating: true\n{stamp}  - key: \"rule SEC-2 at src/b.rs\"\n    \
                 outcome: deferred\n    target: specs/007-gate/tasks.md\n    task: \"3\"\n    \
                 gating: true\n{stamp}  - key: \"perf: noise\"\n    outcome: deferred\n    \
                 work-item: PROJ-2\n{stamp}"
            ),
        );
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(
            result.blocked_by,
            Some(ReviewGateBlock::UndispositionedFindings)
        );
        let message = result.message.unwrap();
        assert!(
            message.ends_with(
                "; held by deferral: rule SEC-1 at src/a.rs — to work item `PROJ-1`; \
                 rule SEC-2 at src/b.rs — to `specs/007-gate/tasks.md` task 3"
            ),
            "{message}"
        );
        let guidance = result.guidance.unwrap();
        assert!(guidance.contains("/ductus:review --defer"), "{guidance}");
    }

    /// Spec 076, AC16: a MUST violation deferred to this spec's own task, or
    /// to a work item alone, blocks at the MUST check, which runs before the
    /// last one, so that check names the deferral with its work too.
    #[test]
    fn a_must_held_by_a_deferral_is_named_where_it_blocks() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_BLOCKING);
        add_decisions(
            tmp.path(),
            "review.md",
            "decisions:\n  - key: \"rule SEC-1 at src/a.rs\"\n    outcome: deferred\n    \
             work-item: PROJ-1\n    gating: true\n    decided-at: 2026-07-10T00:00:00Z\n    \
             decided-by: dev@example.com\n",
        );
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::MustViolations));
        assert_eq!(
            result.message.as_deref(),
            Some(
                "blocked: spec has 3 MUST violation(s) — see specs/007-gate/review.md; \
                 held by deferral: rule SEC-1 at src/a.rs — to work item `PROJ-1`"
            )
        );
    }

    /// Spec 076: a work-item ID is stored as given, and a hand-edited key may
    /// hold anything, so the gate flattens both, as every report does, before
    /// naming them: an escape in the record never reaches the message.
    #[test]
    fn a_held_deferral_is_named_flattened() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_BLOCKING);
        add_decisions(
            tmp.path(),
            "review.md",
            "decisions:\n  - key: \"rule SEC-1 at src/a\\u001b.rs\"\n    outcome: deferred\n    \
             work-item: \"PROJ-1\\u001b[31m\"\n    gating: true\n    \
             decided-at: 2026-07-10T00:00:00Z\n    decided-by: dev@example.com\n",
        );
        let message = run_with_lint(&args(), tmp.path(), clean_lint)
            .unwrap()
            .message
            .unwrap();
        assert!(
            message.ends_with(
                "; held by deferral: rule SEC-1 at src/a .rs — to work item `PROJ-1 [31m`"
            ),
            "{message:?}"
        );
    }

    /// `QUAL-CLAIM-001`: where the gate names holding deferrals, a decisions
    /// list it cannot read, and an entry marked gating that is no deferral,
    /// are each said so — never rendered as no deferral at all.
    #[test]
    fn held_deferrals_the_gate_cannot_read_are_said_so() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_BLOCKING);
        add_decisions(tmp.path(), "review.md", "decisions:\n  - key: [a, b]\n");
        let message = run_with_lint(&args(), tmp.path(), clean_lint)
            .unwrap()
            .message
            .unwrap();
        assert!(
            message.ends_with(
                "; held by deferral: the decisions in specs/007-gate/review.md, \
                 which could not be read"
            ),
            "{message}"
        );

        let tmp = tempdir().unwrap();
        seed(tmp.path(), REVIEWED_BLOCKING);
        add_decisions(
            tmp.path(),
            "review.md",
            "decisions:\n  - key: \"rule SEC-1 at src/a.rs\"\n    outcome: waived\n    \
             reason: noise\n    gating: true\n    decided-at: 2026-07-10T00:00:00Z\n    \
             decided-by: dev@example.com\n",
        );
        let message = run_with_lint(&args(), tmp.path(), clean_lint)
            .unwrap()
            .message
            .unwrap();
        assert!(
            message.ends_with(
                "; held by deferral: 1 entr(ies) in specs/007-gate/review.md marked gating \
                 that do not read as a deferral"
            ),
            "{message}"
        );
    }

    /// Spec 076, AC16: the same for a blocking analyze finding deferred to
    /// this spec's own task, which blocks at the analyze-findings check.
    #[test]
    fn an_analyze_finding_held_by_a_deferral_is_named_where_it_blocks() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), ANALYZE_BLOCKING);
        add_decisions(
            tmp.path(),
            "analysis.md",
            "decisions:\n  - key: \"grounding — cites a gone path\"\n    outcome: deferred\n    \
             target: specs/007-gate/tasks.md\n    task: \"2\"\n    gating: true\n    \
             decided-at: 2026-07-10T00:00:00Z\n    decided-by: dev@example.com\n",
        );
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::AnalyzeFindings));
        let message = result.message.unwrap();
        assert!(
            message.ends_with(
                "; held by deferral: grounding — cites a gone path — to \
                 `specs/007-gate/tasks.md` task 2"
            ),
            "{message}"
        );
    }

    #[test]
    fn a_gating_count_beside_a_false_blocking_flag_still_holds_done() {
        // A hand-edited record whose `blocking` disagrees with its counts
        // passes the MUST and analyze-findings checks, which read the flag;
        // the gating-finding check's sum is what still holds it.
        let tmp = tempdir().unwrap();
        seed(
            tmp.path(),
            &ANALYZE_ADVISORY_ONLY
                .replace("must-violations: 0", "must-violations: 1")
                .replace("hard-fail: 0", "hard-fail: 1")
                .replace("blocking-findings: 0", "blocking-findings: 2"),
        );
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(
            result.blocked_by,
            Some(ReviewGateBlock::UndispositionedFindings)
        );
        assert_eq!(
            result.message.as_deref(),
            Some("blocked: 1 outstanding gating finding(s) — specs/007-gate/review.md: 1")
        );

        // The review's count cleared, the analysis's holds in its turn.
        let tmp = tempdir().unwrap();
        seed(
            tmp.path(),
            &ANALYZE_ADVISORY_ONLY
                .replace("hard-fail: 0", "hard-fail: 1")
                .replace("blocking-findings: 0", "blocking-findings: 2"),
        );
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(
            result.blocked_by,
            Some(ReviewGateBlock::UndispositionedFindings)
        );
        assert_eq!(
            result.message.as_deref(),
            Some("blocked: 3 outstanding gating finding(s) — specs/007-gate/analysis.md: 3")
        );
    }

    #[test]
    fn a_predating_record_outranks_an_outstanding_should() {
        let tmp = tempdir().unwrap();
        seed(
            tmp.path(),
            &ANALYZE_ADVISORY_ONLY.replace("should-violations: 0", "should-violations: 4"),
        );
        strip_dispositions(tmp.path(), "review.md");
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(
            result.blocked_by,
            Some(ReviewGateBlock::RecordPredatesDispositions),
            "a count beside a record that predates the field means nothing yet"
        );
    }

    /// A review's map is the review's to write, so it comes before a stale analysis,
    /// which that review would stale again. An analysis's own map comes after it: a
    /// stale record has to be re-run before what it says about its findings means
    /// anything.
    #[test]
    fn a_stale_analysis_outranks_only_its_own_missing_map() {
        let seeded = || {
            let tmp = tempdir().unwrap();
            seed(tmp.path(), REVIEWED_CLEAN);
            let base = git_commit_all(tmp.path(), "base");
            seed_with_current_digest(tmp.path(), &base);
            tmp
        };

        // Stripping review.md's map edits an analyze subject, so the analysis is stale too.
        let tmp = seeded();
        strip_dispositions(tmp.path(), "review.md");
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(
            result.blocked_by,
            Some(ReviewGateBlock::RecordPredatesDispositions)
        );

        let tmp = seeded();
        strip_dispositions(tmp.path(), "analysis.md");
        fs::write(tmp.path().join("specs/007-gate/plan.md"), "# Plan\n").unwrap();
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::AnalyzeStale));
    }

    // --- gate check 10: a review owed for what changed (spec 079) -----------

    fn seed_reopened(repo: &Path) {
        fs::create_dir_all(repo.join("specs/007-gate")).unwrap();
        fs::write(repo.join(".govern.toml"), "[host]\nproject = \"ductus\"\n").unwrap();
        fs::write(
            repo.join("specs/007-gate/spec.md"),
            "---\nstatus: done\ndependencies: []\n---\n\n# 007 — Gate\n",
        )
        .unwrap();
        write_records(repo, "2026-10-01T00:00:00Z");
        git_commit_all(repo, "review(007): and its analysis");
        set_gate_status(repo, "in-progress");
    }

    fn set_gate_status(repo: &Path, status: &str) {
        let path = repo.join("specs/007-gate/spec.md");
        let text = fs::read_to_string(&path).unwrap();
        let reset = text
            .replace("status: done", "status: X")
            .replace("status: in-progress", "status: X");
        fs::write(
            &path,
            reset.replace("status: X", &format!("status: {status}")),
        )
        .unwrap();
    }

    fn write_gate_review(repo: &Path, at: &str) {
        crate::primitives::write_review::run(
            &crate::schema::primitives::WriteReviewArgs {
                feature: "007-gate".into(),
                reviewed_at: at.into(),
                reviewed_against: "abc123".into(),
                examined: Some(1),
                ..Default::default()
            },
            repo,
        )
        .unwrap();
    }

    fn write_gate_analysis(repo: &Path, at: &str) {
        crate::primitives::write_analysis::run(
            &crate::schema::primitives::WriteAnalysisArgs {
                feature: "007-gate".into(),
                analyzed_at: at.into(),
                analyzed_against: "abc123".into(),
                ..Default::default()
            },
            repo,
        )
        .unwrap();
    }

    fn write_records(repo: &Path, at: &str) {
        write_gate_review(repo, at);
        write_gate_analysis(repo, at);
    }

    fn gate(repo: &Path) -> Verdict {
        run_with_lint(&args(), repo, clean_lint).unwrap()
    }

    /// The outcome this check exists for: following the gate's own messages, a
    /// reopening that changed code takes one review and one analysis, and neither
    /// message sends it to `/analyze` before the review.
    #[test]
    fn a_reopening_that_changed_code_takes_one_review_then_one_analysis() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        seed_reopened(repo);
        fs::create_dir_all(repo.join("src")).unwrap();
        fs::write(repo.join("src/a.rs"), "fn a() {}\n").unwrap();
        git_commit_all(repo, "feat(007): reopen with its code");

        let first = gate(repo);
        assert_eq!(first.blocked_by, Some(ReviewGateBlock::ReviewOwed));
        let message = first.message.unwrap();
        assert!(message.contains("src/a.rs"), "{message}");
        assert!(message.contains("/ductus:review"), "{message}");

        // Its window moves only at the commit holding the record.
        write_gate_review(repo, "2026-11-01T00:00:00Z");
        assert_eq!(gate(repo).blocked_by, Some(ReviewGateBlock::ReviewOwed));
        git_commit_all(repo, "review(007)");
        assert_eq!(gate(repo).blocked_by, Some(ReviewGateBlock::AnalyzeStale));

        write_gate_analysis(repo, "2026-11-01T01:00:00Z");
        git_commit_all(repo, "analyze(007)");
        let last = gate(repo);
        assert!(last.passed, "{last:?}");

        set_gate_status(repo, "done");
        git_commit_all(repo, "status(007): done");
        let measured = crate::primitives::measure_reopenings::run(
            &crate::schema::primitives::MeasureReopeningsArgs {
                since: Some("2000-01-01".into()),
                at: None,
            },
            repo,
        )
        .unwrap();
        let reopening = measured.reopenings.unwrap();
        assert_eq!(reopening.len(), 1, "{reopening:?}");
        assert_eq!((reopening[0].reviews, reopening[0].analyses), (1, 1));
        assert!(!reopening[0].over_bound);
    }

    /// A stale analysis names every changed artifact: the message shows three
    /// and counts the rest, and `analyze-stale-paths` holds them all inline, or
    /// past the read size in the file the result names, its count inline in
    /// both forms (066 scenario `freshness-results-have-a-bounded-size`).
    #[test]
    fn a_stale_analysis_past_the_read_size_names_a_file_holding_every_path() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        seed_reopened(repo);
        fs::create_dir_all(repo.join("specs/007-gate/scenarios")).unwrap();
        let mut expected: Vec<String> = Vec::new();
        for n in 0..40 {
            let rel = format!("specs/007-gate/scenarios/s{n:02}.md");
            fs::write(repo.join(&rel), "# S\n").unwrap();
            expected.push(rel);
        }
        git_commit_all(repo, "docs(007): scenarios");
        let read_size = |bytes: u32| {
            fs::create_dir_all(repo.join(".ductus")).unwrap();
            fs::write(
                repo.join(".ductus/config.toml"),
                format!("[artifacts]\nread-size-bytes = {bytes}\n"),
            )
            .unwrap();
        };

        read_size(1_000_000);
        let inline = bounded(&args(), repo, clean_lint).unwrap();
        assert_eq!(inline.blocked_by, Some(ReviewGateBlock::AnalyzeStale));
        assert_eq!(inline.form, ListsForm::Inline);
        let whole = inline.analyze_stale_paths.clone().unwrap();
        for path in &expected {
            assert!(whole.contains(path), "{path} missing from {whole:?}");
        }
        assert_eq!(inline.analyze_stale_paths_count, whole.len());
        assert!(inline.message.as_deref().unwrap().contains("(+"));

        read_size(10);
        let file = bounded(&args(), repo, clean_lint).unwrap();
        assert_eq!(file.form, ListsForm::File);
        assert_eq!(file.analyze_stale_paths, None);
        assert_eq!(file.analyze_stale_paths_count, whole.len());
        let listed: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(repo.join(file.lists_file.unwrap())).unwrap())
                .unwrap();
        assert_eq!(listed["analyze-stale-paths"], serde_json::json!(whole));
    }

    /// Nothing but text, this spec's own or another's, and the earlier record stands:
    /// the reopening takes one analysis and no review (AC4).
    #[test]
    fn a_reopening_that_changed_only_text_passes_on_one_analysis() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        seed_reopened(repo);
        fs::write(repo.join("specs/007-gate/plan.md"), "# Plan\n").unwrap();
        fs::create_dir_all(repo.join("specs/008-other")).unwrap();
        fs::write(repo.join("specs/008-other/notes.md"), "notes\n").unwrap();
        git_commit_all(repo, "docs(007): the reopening's text");

        assert_eq!(gate(repo).blocked_by, Some(ReviewGateBlock::AnalyzeStale));
        write_gate_analysis(repo, "2026-11-01T00:00:00Z");
        git_commit_all(repo, "analyze(007)");
        let result = gate(repo);
        assert!(result.passed, "{result:?}");
    }

    #[test]
    fn an_uncommitted_change_makes_a_review_owed_and_is_named() {
        let tmp = tempdir().unwrap();
        let repo = tmp.path();
        seed_reopened(repo);
        git_commit_all(repo, "status(007): reopen");
        fs::create_dir_all(repo.join("src")).unwrap();
        fs::write(repo.join("src/uncommitted.rs"), "fn u() {}\n").unwrap();
        let result = gate(repo);
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::ReviewOwed));
        assert!(result.message.unwrap().contains("src/uncommitted.rs"));
        assert!(result.guidance.unwrap().contains("commit"));
    }

    /// Every review check comes before the analyze checks, so none of them names an
    /// analysis the review would then stale.
    #[test]
    fn what_only_a_review_clears_outranks_a_stale_analysis() {
        for (old, new, expected) in [
            (
                "should-violations: 0\n",
                "should-violations: 1\n",
                ReviewGateBlock::UndispositionedFindings,
            ),
            (
                "blocking: false\n",
                "blocking: false\nfix-rounds-open: true\n",
                ReviewGateBlock::FixRoundLoopOpen,
            ),
            (
                "blocking: false\n",
                "blocking: false\nnot-re-read: 1\n",
                ReviewGateBlock::FixRoundNotReRead,
            ),
        ] {
            let tmp = tempdir().unwrap();
            seed(tmp.path(), REVIEWED_CLEAN);
            let base = git_commit_all(tmp.path(), "base");
            seed_with_current_digest(tmp.path(), &base);
            let path = tmp.path().join("specs/007-gate/review.md");
            let record = fs::read_to_string(&path).unwrap();
            assert!(record.contains(old), "{record}");
            fs::write(&path, record.replacen(old, new, 1)).unwrap();
            let result = gate(tmp.path());
            assert_eq!(result.blocked_by, Some(expected), "{new}");
        }
    }

    /// Outside a repository nothing says what changed: the gate says so, rather than
    /// reading as a window it examined, and it does not block on it.
    #[test]
    fn a_passing_gate_says_when_it_could_not_tell_whether_a_review_is_owed() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), ANALYZE_ADVISORY_ONLY);
        let result = gate(tmp.path());
        assert!(result.passed, "{result:?}");
        let guidance = result.guidance.unwrap();
        assert!(
            guidance.contains("Whether a review is owed could not be determined"),
            "{guidance}"
        );
    }

    #[test]
    fn a_done_spec_short_circuits_before_any_disposition_check() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), ALREADY_DONE);
        let review = tmp.path().join("specs/007-gate/review.md");
        if review.exists() {
            let text = fs::read_to_string(&review).unwrap();
            fs::write(&review, text.replace(ZERO_MAP, "")).unwrap();
        }
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert_eq!(result.blocked_by, Some(ReviewGateBlock::AlreadyDone));
    }

    #[test]
    fn an_all_zero_map_passes() {
        let tmp = tempdir().unwrap();
        seed(tmp.path(), ANALYZE_ADVISORY_ONLY);
        let result = run_with_lint(&args(), tmp.path(), clean_lint).unwrap();
        assert!(result.passed, "{result:?}");
    }
}
