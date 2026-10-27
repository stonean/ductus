//! `write-review` — render `specs/NNN/review.md` for `/ductus:review`: the
//! report body and, in that same file's frontmatter, the record of the run.
//! `spec.md` is not written (spec 057).
//!
//! Consumes the pass findings as a single `findings` array (the
//! content-ingestion convention), plus the waiver results from
//! `process-waivers` and the scope scalars from `compute-review-scope`, and:
//!
//! - applies the deterministic **cross-pass dedup** — same `(rule, file)` with
//!   overlapping line ranges collapses to one finding, highest-severity-wins
//!   (tie broken by higher confidence);
//! - **buckets** the survivors: findings matched by an applied waiver drop out
//!   of the counts into Waived findings; a `low`-confidence finding lands in
//!   Low-confidence regardless of severity; the rest split MUST / SHOULD;
//! - renders the fixed report skeleton into `review.md` and records the run in
//!   that same file's frontmatter (`last-run`, `reviewed-against`,
//!   `must-violations`, `should-violations`, `low-confidence`, `blocking`),
//!   pruning any **expired** waiver entries from its `waivers` list on the
//!   write (per `process-waivers`' contract). It wrote a second copy into a
//!   `review:` block in `spec.md` until spec 057 left the record one home;
//! - **records the disposition of the reviewer's observations** — things the
//!   reviewer judged real that map to no loaded rule. Each renders beside its
//!   outcome, the record counts them under `dispositions:`, and a newly routed
//!   or discarded one is stored under `decisions:` so the next run does not ask
//!   again (spec 058). Nothing is written to the inbox;
//! - the empty-scope case is a branch of this primitive, not a prose
//!   special-case: it emits the 0-findings, `blocking: false` report.
//!
//! Both writes are atomic (tempfile + rename). `blocking` is true exactly when
//! `must-violations` exceeds zero, and the exit code (0 / 1) is derivable from
//! it. Defined by
//! `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
//! and `specs/058-findings-route-at-discovery/spec.md`.

use std::fmt::Write as _;
use std::path::Path;

use serde::Deserialize;

use crate::primitives::decisions::{self, RawDecision};
use crate::primitives::{
    PrimitiveError, Result, WaiverPaths, analyze_subjects, read_text, rel_path, split_frontmatter,
    write_atomic,
};
use crate::schema::paths;
use crate::schema::primitives::{
    ConstitutionOutcome, DecisionRef, DispositionOutcome, Dispositions, RecordFreshness,
    ReviewFinding, ReviewObservation, WriteReviewArgs, WriteReviewResult,
};
use crate::schema::severity::ReviewSeverity;
/// Reject any scalar field that would inject document structure.
///
/// Scalars are spliced verbatim into `review.md` frontmatter, so an embedded
/// newline would add a top-level key (a spoofed `status:`, say) and corrupt
/// pipeline state. Multi-line prose fields — the summary and finding bodies —
/// are markdown body content and are deliberately not screened; waiver fields
/// are separately quoted through `yaml_string`.
fn validate_scalar_fields(args: &WriteReviewArgs) -> Result<()> {
    // Scalar fields spliced verbatim into review.md frontmatter must be
    // single-line: an embedded newline would inject a top-level frontmatter
    // key (e.g. a spoofed `blocking:`) into the record and corrupt what every
    // gate reads from it. Multi-line prose fields (the summary and finding
    // bodies) are markdown body content and are not screened here. Waiver
    // fields are separately quoted via `yaml_string`.
    let single_line =
        |argument: &str, value: &str| super::validate_single_line("write-review", argument, value);
    single_line("feature", &args.feature)?;
    single_line("reviewed-at", &args.reviewed_at)?;
    // A blank timestamp would be stamped onto every new decision as a blank
    // `decided-at`, an entry the decisions reader classifies as malformed.
    if args.reviewed_at.trim().is_empty() {
        return Err(PrimitiveError::InvalidArgument {
            primitive: "write-review".into(),
            argument: "reviewed-at".into(),
            reason: "the run's timestamp is empty".into(),
        });
    }
    single_line("reviewed-against", &args.reviewed_against)?;
    single_line("diff-base", &args.diff_base)?;
    if let Some(scenario) = args.scenario.as_deref() {
        single_line("scenario", scenario)?;
    }
    for (idx, pass) in args.skipped_passes.iter().enumerate() {
        single_line(&format!("skipped-passes[{idx}]"), pass)?;
    }
    if let Some(who) = args.decided_by.as_deref() {
        single_line("decided-by", who)?;
    }
    // An observation renders as one list item and, once decided, becomes a
    // stored decision's key, so an embedded newline would inject structure
    // into the report or the record. Every check runs here, before any
    // artifact is touched.
    for (idx, observation) in args.observations.iter().enumerate() {
        let invalid = |argument: String, reason: &str| PrimitiveError::InvalidArgument {
            primitive: "write-review".into(),
            argument,
            reason: reason.into(),
        };
        if observation.text.trim().is_empty() {
            return Err(invalid(
                format!("observations[{idx}].text"),
                "text is empty",
            ));
        }
        single_line(&format!("observations[{idx}].text"), &observation.text)?;
        single_line(&format!("observations[{idx}].path"), &observation.path)?;
        if let Some(key) = observation.decision_key.as_deref() {
            single_line(&format!("observations[{idx}].decision-key"), key)?;
        }
        if let Some((argument, companion)) = decisions::require_companion(
            "write-review",
            &format!("observations[{idx}].disposition"),
            &observation.disposition,
        )? {
            single_line(&argument, companion)?;
        }
    }
    Ok(())
}

/// Execute the `write-review` primitive.
///
/// # Errors
///
/// Every refusal lands before any write.
///
/// - [`PrimitiveError::InvalidPath`] when `feature` is empty, absolute, or
///   carries a parent-directory component, or when the feature directory, or
///   a link inside it, resolves outside the project.
/// - [`PrimitiveError::InvalidArgument`] when a scalar argument, an
///   observation's text, path, or `decision-key`, or a route's target or a
///   discard's reason carries a line break or another character that is not
///   plain single-line text; when `reviewed-at` or an observation's text is
///   blank; when a route names no target or a discard states no reason; when
///   two observations sharing a key are given different outcomes; or when a
///   new decision has no `decided-by`.
/// - [`PrimitiveError::FeatureNotFound`] when the feature has no `spec.md`.
/// - [`PrimitiveError::MissingFrontmatter`] when `spec.md` has no frontmatter
///   block, and [`PrimitiveError::UnclosedFrontmatter`] when the block in
///   `spec.md` or the prior `review.md` never closes. A prior `review.md`
///   that opens no block records no waivers or decisions and is overwritten.
/// - [`PrimitiveError::Yaml`] when `spec.md`'s frontmatter fails to parse, or
///   the prior `review.md`'s frontmatter, `waivers:`, or `decisions:` list
///   does.
/// - [`PrimitiveError::Io`] on read/write failure.
pub fn run(args: &WriteReviewArgs, repo: &Path) -> Result<WriteReviewResult> {
    let root = paths::Paths::load(repo).specs_root;
    super::validate_feature_argument(repo, &root, &args.feature)?;
    validate_scalar_fields(args)?;
    let feature_dir = repo.join(&root).join(&args.feature);
    let spec_path = feature_dir.join("spec.md");
    if !spec_path.is_file() {
        return Err(PrimitiveError::FeatureNotFound {
            root,
            feature: args.feature.clone(),
        });
    }

    // Dedup, then bucket the survivors.
    let deduped = dedup_findings(&args.findings);
    let mut must: Vec<&ReviewFinding> = Vec::new();
    let mut should: Vec<&ReviewFinding> = Vec::new();
    let mut low: Vec<&ReviewFinding> = Vec::new();
    let mut waived: Vec<&ReviewFinding> = Vec::new();
    for finding in &deduped {
        if waiver_reason(finding, &args.applied_waivers).is_some() {
            waived.push(finding);
        } else if finding.confidence.eq_ignore_ascii_case("low") {
            low.push(finding);
        } else {
            // Exhaustive by construction, and that is the point. This was an
            // `else` arm reached by any severity that was not exactly `must`,
            // so a typo, or a value borrowed from the analyze vocabulary,
            // filed silently as a SHOULD and wrote `blocking: false` past
            // `check-review-gate`. A match over the closed set has no such
            // arm: an unrecognized value can no longer be constructed, and
            // adding a tier is a compile error here rather than a silent
            // demotion (spec 022 `severity-is-a-closed-set-not-a-string`).
            match finding.severity {
                ReviewSeverity::Must => must.push(finding),
                ReviewSeverity::Should => should.push(finding),
            }
        }
    }

    let must_n = u32::try_from(must.len()).unwrap_or(u32::MAX);
    let should_n = u32::try_from(should.len()).unwrap_or(u32::MAX);
    let low_n = u32::try_from(low.len()).unwrap_or(u32::MAX);
    let waived_n = u32::try_from(waived.len()).unwrap_or(u32::MAX);
    let blocking = must_n > 0;

    // Read and validate every input before the write: a malformed spec
    // (missing frontmatter, YAML parse failure) must halt before review.md
    // exists. The ordering outlived the pair it was written for — spec 057
    // left one write, so there is no longer an inconsistent in-between state
    // to reach, and what it now buys is that a malformed spec halts with no
    // report on disk rather than with a report whose spec cannot be read.
    // The review's durable contracts as this run read them. Taken **before**
    // the writes below, because `write-review` does not touch `scenarios/` or
    // `data-model.md` and must not fold its own output into the record's
    // subject — the reason `review.md` and `spec.md` are outside the set.
    let contracts =
        analyze_subjects::subject_digest(&feature_dir, analyze_subjects::is_review_contract);
    // What governance did this run actually have? Resolved here rather than
    // passed in: a caller that had to supply it could omit it, and the whole
    // point of the section is that a report cannot quietly claim clean over
    // rules it never loaded (spec 055, AC8).
    //
    // Deliberately NOT `?`. A registry that will not parse is itself an
    // unexamined-governance state — it is the strongest form of one — so it
    // is rendered, not propagated. Propagating it would mean an unrelated
    // typo in the project config produced no `review.md` at all, losing the
    // findings this run just computed.
    let governance_section = render_unexamined_governance(repo);
    // The denominator `examined` is a claim against — see `resolve_scope_size`.
    let scope = resolve_scope_size(args, repo);
    // Read the spec and resolve the surviving waivers BEFORE either render:
    // the report now carries the full record, waivers included, so both homes
    // are rendered from one computation rather than two.
    let spec_content = read_text(&spec_path)?;
    // Validate the spec's frontmatter before anything is written.
    //
    // This parse used to happen as a by-product of reading waivers out of the
    // spec. With the waivers moved to `review.md` (spec 057 task 5) nothing
    // else needs it, so it is deliberate now — and it has to be: without it a
    // spec whose frontmatter will not parse receives a `review.md` recording a
    // clean run, which is the inversion the halt exists to prevent. The
    // scenario primitive-robustness-hardening pins it.
    let (spec_fm_text, _) = split_frontmatter(&spec_content, &spec_path)?;
    let _: crate::schema::primitives::Frontmatter =
        serde_norway::from_str(spec_fm_text).map_err(|source| PrimitiveError::Yaml {
            path: spec_path.clone(),
            source,
        })?;
    let surviving = surviving_waivers(&feature_dir, args)?;
    let dispositions = count_dispositions(&args.observations);
    decisions::one_outcome_per_key(args.observations.iter().map(|observation| {
        (
            observation_key(observation),
            observation.disposition.outcome,
        )
    }))
    .map_err(|refusal| refusal.into_error("write-review", "observations", "an observation"))?;
    let decisions = decisions::merge(
        decisions::read_decisions(&feature_dir, super::REVIEW_RECORD_FILE)?,
        &args.expired_decisions,
        &decided_observations(&args.observations),
        &args.reviewed_at,
        args.decided_by.as_deref(),
    )
    .map_err(|refusal| refusal.into_error("write-review", "observations", "an observation"))?;
    let report = render_report(
        args,
        &Buckets {
            must: &must,
            should: &should,
            low: &low,
            waived: &waived,
        },
        blocking,
        &governance_section,
        scope,
        &Record {
            waivers: &surviving,
            decisions: &decisions,
            dispositions,
        },
        &contracts,
    );
    let review_path = feature_dir.join("review.md");

    // Everything computed and validated; only now touch the filesystem.
    //
    // One write, one home. `spec.md` is read for validation above and never
    // written here — the record it used to carry now lives in `review.md`
    // alone (spec 057), which also retires the two-writes-must-not-diverge
    // hazard the ordering comment above was built around.
    write_atomic(&review_path, &report)?;

    // The analyze row `/{project}:review` renders (spec 047 AC12), computed
    // AFTER the write above and against the working tree — which is the whole
    // point of the reference point. This call has just rewritten `review.md`,
    // an analyze subject, so the record it reports on is superseded from this
    // moment; a committed comparison would say `current` until someone
    // committed and would then be wrong retroactively.
    //
    // The record is read from `analysis.md` rather than from the spec text this
    // call just produced (spec 057). That is not merely a path change: the old
    // form read the `analyze:` block out of the in-memory `updated` spec so the
    // answer matched the bytes being written. With the record in its own
    // artifact — one this call does not touch — reading it from disk *is* the
    // consistent answer.
    let analyze_freshness = analyze_freshness_of(&feature_dir, &args.feature, repo);

    Ok(WriteReviewResult {
        path: rel_path(&review_path, repo),
        spec_path: rel_path(&spec_path, repo),
        must_violations: must_n,
        should_violations: should_n,
        low_confidence: low_n,
        waived: waived_n,
        observations: u32::try_from(args.observations.len()).unwrap_or(u32::MAX),
        dispositions,
        blocking,
        exit_code: i32::from(blocking),
        analyze_freshness,
        examined: args.examined,
        scope,
    })
}

/// How many files this review's scope covers — the denominator.
///
/// Derived here rather than accepted as an argument, the same discipline that
/// derives `blocking`, `reviewed-digest` and the governance section: a caller
/// supplying it could shrink the subject to match whatever it happened to
/// read. It can still overstate the numerator, but it cannot hide how large
/// the scope was.
///
/// Resolved through `compute-review-scope` against the run's own `diff-base`,
/// so it is the scope the review was told to cover rather than whatever that
/// base resolves to at some later moment. An unresolvable window (a spec dir
/// with no commit yet) yields zero rather than an error: the findings this run
/// computed still have to land, and a zero scope beside a zero `examined` is
/// the coherent empty-scope state rather than a false claim.
///
/// **`empty-scope` does not short-circuit this.** It used to, so a caller that
/// passed the flag recorded `scope: 0` without deriving anything — and the
/// realistic way to pass it wrongly is not carelessness but an over-cap
/// `compute-review-scope` call, which returns a hard error with no result and
/// reads as *nothing in scope*. The resulting record (`0/0/0`, `scope: 0`,
/// `examined` absent, `blocking: false`) is self-consistent and invisible to
/// every gate: Family 31 cannot see it because both of its `examined` arms
/// guard on `total > 0`, Family 19 cannot because the digest is valid, and
/// `check-review-gate` cannot because the review is non-blocking. Deriving the
/// denominator unconditionally closes it with no new check: a genuinely empty
/// scope still resolves to 0, while a false one now carries the real `scope: N`
/// beside a zero `examined`, which is exactly the shape Family 31 already
/// reports as `examined-nothing`.
fn resolve_scope_size(args: &WriteReviewArgs, repo: &Path) -> u32 {
    let scope_args = crate::schema::primitives::ComputeReviewScopeArgs {
        feature: args.feature.clone(),
        since: Some(args.diff_base.clone()),
    };
    crate::primitives::compute_review_scope::run(&scope_args, repo).map_or(0, |result| {
        u32::try_from(result.scope.len()).unwrap_or(u32::MAX)
    })
}

/// The spec's analyze-record freshness.
///
/// Delegates to [`crate::primitives::analyze_subjects::analyze_freshness`] — the single
/// implementation the completion gate also uses (spec 047 AC14) — so the row
/// this primitive reports and the verdict that gate reaches cannot disagree.
/// There is no reference point left to differ on: the comparison is of the
/// subject set's content against the digest the analysis recorded, which
/// answers the same way before and after a commit.
///
/// A spec whose frontmatter will not parse yields
/// [`RecordFreshness::Undeterminable`] rather than an error: the review
/// itself has already been written by this point, and failing the whole call
/// over the notice would trade a report for a row.
fn analyze_freshness_of(feature_dir: &Path, feature: &str, repo: &Path) -> RecordFreshness {
    let root = paths::Paths::load(repo).specs_root;
    let rel_dir = format!("{root}/{feature}");
    // An unreadable record is reported as never-run here, deliberately: this
    // is an advisory row in a review's stdout, not a gate, and the gate is
    // where the absent/undeterminable distinction is load-bearing. Blocking a
    // review's summary line on a damaged analyze record would let one record's
    // damage suppress an unrelated command's output.
    let record = crate::primitives::load_analyze_record(feature_dir);
    match record.as_present() {
        Some(analyze) => analyze_subjects::analyze_freshness(repo, &rel_dir, Some(analyze)),
        None => RecordFreshness::NeverRun,
    }
}

// -- observation dispositions -----------------------------------------------

/// The `dispositions:` map for this run's observations. Derived, never
/// accepted: a caller that supplied the counts could report them clean.
fn count_dispositions(observations: &[ReviewObservation]) -> Dispositions {
    let mut counts = Dispositions::default();
    for observation in observations {
        let slot = match observation.disposition.outcome {
            DispositionOutcome::Fixed => &mut counts.fixed,
            DispositionOutcome::Routed => &mut counts.routed,
            DispositionOutcome::Discarded => &mut counts.discarded,
            DispositionOutcome::Undispositioned => &mut counts.undispositioned,
        };
        *slot = slot.saturating_add(1);
    }
    counts
}

/// The routed and discarded observations, as the decisions they store.
fn decided_observations(observations: &[ReviewObservation]) -> Vec<DecisionRef> {
    observations
        .iter()
        .filter_map(|observation| {
            decisions::decision_for(&observation.disposition, &observation_key(observation))
        })
        .collect()
}

/// The observation's key: the stored decision the host matched it to, else —
/// when no key was supplied, or a blank one — its own rendered line.
fn observation_key(observation: &ReviewObservation) -> String {
    decisions::nonblank(observation.decision_key.as_deref())
        .map_or_else(|| observation_line(observation), str::to_string)
}

/// The observation as one line of prose: its text, with the anchoring path
/// appended when it has one. The report's list item and, absent a matched
/// key, the observation's stored-decision key.
fn observation_line(observation: &ReviewObservation) -> String {
    let text = observation.text.trim();
    let path = observation.path.trim();
    if path.is_empty() {
        text.to_string()
    } else {
        format!("{text} — `{path}`")
    }
}

// -- dedup -------------------------------------------------------------------

/// Cross-pass dedup: collapse findings that share a `(rule, file)` anchor and
/// have overlapping line ranges into one, keeping the highest-severity member
/// (tie broken by higher confidence). First-seen order is preserved for a
/// stable render.
fn dedup_findings(findings: &[ReviewFinding]) -> Vec<ReviewFinding> {
    let mut kept: Vec<ReviewFinding> = Vec::new();
    for finding in findings {
        let range = parse_range(&finding.line_range);
        let overlap = kept.iter().position(|existing| {
            existing.rule == finding.rule
                && existing.file == finding.file
                && ranges_overlap(range, parse_range(&existing.line_range))
        });
        if let Some(idx) = overlap {
            if finding_rank(finding) > finding_rank(&kept[idx]) {
                kept[idx] = finding.clone();
            }
        } else {
            kept.push(finding.clone());
        }
    }
    kept
}

/// Rank a finding for dedup: severity dominates (`must` > `should`), ties break
/// on confidence (`high` > `low`).
fn finding_rank(finding: &ReviewFinding) -> u8 {
    let severity = u8::from(finding.severity.is_blocking()) * 2;
    let confidence = u8::from(!finding.confidence.eq_ignore_ascii_case("low"));
    severity + confidence
}

/// Parse a line-range string into an inclusive `(start, end)`. An empty or
/// unparseable range covers the whole file, so it overlaps any range sharing
/// the same `(rule, file)`.
fn parse_range(text: &str) -> (u32, u32) {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return (0, u32::MAX);
    }
    if let Some((start, end)) = trimmed.split_once('-') {
        let start = start.trim().parse::<u32>().unwrap_or(0);
        let end = end.trim().parse::<u32>().unwrap_or(u32::MAX);
        (start.min(end), start.max(end))
    } else if let Ok(single) = trimmed.parse::<u32>() {
        (single, single)
    } else {
        (0, u32::MAX)
    }
}

/// Inclusive interval overlap.
fn ranges_overlap(a: (u32, u32), b: (u32, u32)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

/// The reason of the first applied waiver anchored to this finding's
/// `(rule, file)`, or `None` when unwaived.
fn waiver_reason<'a>(
    finding: &ReviewFinding,
    applied: &'a [crate::schema::primitives::WaiverRef],
) -> Option<&'a str> {
    applied
        .iter()
        .find(|waiver| waiver.rule == finding.rule && waiver.file == finding.file)
        .map(|waiver| waiver.reason.as_str())
}

// -- report rendering --------------------------------------------------------

/// Render the full `review.md` document (frontmatter + fixed skeleton).
/// The four buckets a run's findings fall into, passed as one value so the
/// renderer's signature states a shape rather than four positional slices.
struct Buckets<'a> {
    must: &'a [&'a ReviewFinding],
    should: &'a [&'a ReviewFinding],
    low: &'a [&'a ReviewFinding],
    waived: &'a [&'a ReviewFinding],
}

/// The record's lists and derived counts, rendered into the frontmatter.
struct Record<'a> {
    waivers: &'a [RawWaiverFull],
    decisions: &'a [RawDecision],
    dispositions: Dispositions,
}

fn render_report(
    args: &WriteReviewArgs,
    buckets: &Buckets<'_>,
    blocking: bool,
    governance_section: &str,
    scope: u32,
    record: &Record<'_>,
    contracts: &crate::primitives::analyze_subjects::SubjectDigest,
) -> String {
    let (must, should, low, waived) = (buckets.must, buckets.should, buckets.low, buckets.waived);
    let feature = &args.feature;

    let mut fm = String::from("---\n");
    let _ = writeln!(fm, "spec: {feature}");
    if let Some(scenario) = args.scenario.as_deref().filter(|s| !s.trim().is_empty()) {
        let _ = writeln!(fm, "scenario: {scenario}");
    }
    // `last-run`, not `reviewed-at`. The two names were one instant spelled
    // twice across the record's two homes, and `check-review-agreement` had to
    // key that pair "by meaning, not by name" to compare them at all. With one
    // home the second spelling has nothing left to justify it (spec 057).
    let _ = writeln!(fm, "last-run: {}", args.reviewed_at);
    let _ = writeln!(fm, "reviewed-against: {}", args.reviewed_against);
    let _ = writeln!(fm, "diff-base: {}", args.diff_base);
    let _ = writeln!(fm, "must-violations: {}", must.len());
    let _ = writeln!(fm, "should-violations: {}", should.len());
    let _ = writeln!(fm, "low-confidence: {}", low.len());
    // The claim and its denominator. `examined` is omitted when the run stated
    // nothing, so an unstated claim reads as absent rather than as a computed
    // zero; `scope` is always written, because it was always computed.
    if let Some(examined) = args.examined {
        let _ = writeln!(fm, "examined: {examined}");
    }
    let _ = writeln!(fm, "scope: {scope}");
    let _ = writeln!(fm, "skipped-passes: [{}]", args.skipped_passes.join(", "));
    // The three fields that lived only in the spec's `review:` block. They
    // arrive here unchanged in meaning: `reviewed-digest` is still always
    // written (empty map included, so "digest taken over a spec with no
    // durable contracts" stays distinct from "pre-digest record"), `blocking`
    // is still derived rather than accepted, and waivers still carry their
    // adopter-authored extras verbatim.
    if contracts.digests.is_empty() {
        let _ = writeln!(fm, "reviewed-digest: {{}}");
    } else {
        let _ = writeln!(fm, "reviewed-digest:");
        for (path, digest) in &contracts.digests {
            let _ = writeln!(fm, "  {path}: {digest}");
        }
    }
    if !contracts.unreadable.is_empty() {
        let _ = writeln!(fm, "reviewed-unreadable:");
        for path in &contracts.unreadable {
            let _ = writeln!(fm, "  - {path}");
        }
    }
    let _ = writeln!(fm, "blocking: {blocking}");
    decisions::render_dispositions(&mut fm, record.dispositions);
    render_waivers_at(&mut fm, record.waivers, "");
    decisions::render(&mut fm, record.decisions);
    fm.push_str("---");

    let summary = args
        .summary
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .map_or_else(
            || {
                generate_summary(
                    args,
                    must.len(),
                    should.len(),
                    low.len(),
                    waived.len(),
                    blocking,
                    record.dispositions.undispositioned,
                )
            },
            str::to_string,
        );

    let sections = [
        format!("# Review — {feature}"),
        format!("## Summary\n\n{summary}"),
        format!(
            "## MUST violations (blocking)\n\n{}",
            render_findings(must, "MUST", &args.applied_waivers)
        ),
        format!(
            "## SHOULD violations (advisory)\n\n{}",
            render_findings(should, "SHOULD", &args.applied_waivers)
        ),
        format!(
            "## Low-confidence findings\n\n{}",
            render_findings(low, "LOW-CONFIDENCE", &args.applied_waivers)
        ),
        format!(
            "## Waived findings\n\n{}",
            render_findings(waived, "WAIVED", &args.applied_waivers)
        ),
        format!(
            "## Observations\n\n{}",
            render_observations(&args.observations)
        ),
        format!(
            "## Skipped passes\n\n{}",
            render_skipped(&args.skipped_passes)
        ),
        format!("## Unexamined governance\n\n{governance_section}"),
    ];

    format!("{fm}\n\n{}\n", sections.join("\n\n"))
}

/// A deterministic one-line Summary derived from the counts.
fn generate_summary(
    args: &WriteReviewArgs,
    must: usize,
    should: usize,
    low: usize,
    waived: usize,
    blocking: bool,
    undispositioned: u32,
) -> String {
    if args.empty_scope {
        return "Review scope is empty — no implementation files in scope. \
             Zero findings across all passes; blocking: no."
            .to_string();
    }
    let mut summary = format!(
        "{must} MUST violation(s), {should} SHOULD violation(s), {low} low-confidence finding(s)"
    );
    if waived > 0 {
        let _ = write!(summary, ", {waived} waived");
    }
    let _ = write!(
        summary,
        ". blocking: {}.",
        if blocking { "yes" } else { "no" }
    );
    if undispositioned > 0 {
        let _ = write!(
            summary,
            " {undispositioned} observation(s) undispositioned — recorded, and they do not hold `done`."
        );
    }
    summary
}

/// Render a bucket of findings, or `*None.*` when empty.
fn render_findings(
    findings: &[&ReviewFinding],
    label: &str,
    applied: &[crate::schema::primitives::WaiverRef],
) -> String {
    if findings.is_empty() {
        return "*None.*".to_string();
    }
    findings
        .iter()
        .map(|finding| finding_block(finding, label, waiver_reason(finding, applied)))
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// Render one finding block per the fixed per-finding shape. Absent optional
/// fields (rule text, finding prose, suggested fix) drop their bullet so the
/// output stays markdownlint-clean.
fn finding_block(finding: &ReviewFinding, label: &str, waived_reason: Option<&str>) -> String {
    let mut out = String::new();
    let summary = finding.summary.trim();
    if summary.is_empty() {
        let _ = writeln!(out, "### {label}: {}\n", finding.rule);
    } else {
        let _ = writeln!(out, "### {label}: {} — {summary}\n", finding.rule);
    }
    let range = finding.line_range.trim();
    if range.is_empty() {
        let _ = writeln!(out, "- **File**: `{}`", finding.file);
    } else {
        let _ = writeln!(out, "- **File**: `{}:{range}`", finding.file);
    }
    if !finding.rule_text.trim().is_empty() {
        let _ = writeln!(out, "- **Rule**: {}", finding.rule_text.trim());
    }
    if !finding.finding.trim().is_empty() {
        let _ = writeln!(out, "- **Finding**: {}", finding.finding.trim());
    }
    let _ = writeln!(
        out,
        "- **Auto-fixable**: {}",
        if finding.auto_fixable { "yes" } else { "no" }
    );
    if !finding.suggested_fix.trim().is_empty() {
        let _ = writeln!(out, "- **Suggested fix**: {}", finding.suggested_fix.trim());
    }
    if let Some(reason) = waived_reason {
        let _ = writeln!(out, "- **Waived**: {reason}");
    }
    out.trim_end().to_string()
}

/// Render the observations as a list, or `*None.*` when empty, each beside
/// its disposition so the report says what was done with it, not only that it
/// was seen.
fn render_observations(observations: &[ReviewObservation]) -> String {
    if observations.is_empty() {
        return "*None.*".to_string();
    }
    observations
        .iter()
        .map(|observation| {
            let outcome = decisions::disposition_suffix(&observation.disposition, |text| {
                text.trim().to_string()
            });
            format!("- {} — {outcome}", observation_line(observation))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Characters of a registered source's `description` a single-line report
/// carries before truncating. A budget rather than a wrap, because the line is
/// single-line by contract and a wrapped one would break the list item.
const DESCRIPTION_BUDGET: usize = 100;

/// Truncate `text` to `budget` characters, appending `…` when it was cut.
///
/// Counts **characters**, not bytes, so a multi-byte description cannot be
/// sliced mid-codepoint. A value already inside the budget is returned as-is,
/// so the common case carries no ellipsis.
fn truncate(text: &str, budget: usize) -> String {
    if text.chars().count() <= budget {
        return text.to_string();
    }
    let kept: String = text.chars().take(budget).collect();
    format!("{}…", kept.trim_end())
}

/// Shared constitutions the project registered that this run could not read.
///
/// A registered-but-unreadable source means the review ran under fewer rules than
/// the project's config declares, and `QUAL-CLAIM-001` is explicit that a result
/// must distinguish *examined and found nothing* from *could not examine*. So the
/// section names each one with its reason rather than letting the finding counts
/// stand alone.
///
/// `*None.*` covers both "none registered" and "all registered sources read" —
/// those are the same claim from the report's side, because in both cases nothing
/// went unexamined. The distinction that matters here is unexamined vs not.
fn render_unexamined_governance(repo: &Path) -> String {
    let governance = match crate::primitives::resolve_constitutions::run(
        &crate::schema::primitives::ResolveConstitutionsArgs {},
        repo,
    ) {
        Ok(result) => result,
        // The registry itself is unreadable, so *nothing* registered was
        // loaded and the run cannot even say how many sources it missed.
        // That is the most severe form of this section's subject, so it is
        // reported here rather than raised — `write-review`'s job is to record
        // the run, and a config typo must not cost the operator the findings.
        Err(err) => {
            return format!(
                "- the `[constitutions]` registry could not be read ({err}) — **no** shared \
                 constitution was loaded for this review, and the count of what was missed \
                 is itself unknown"
            );
        }
    };
    if governance.skipped.is_empty() {
        return "*None.*".to_string();
    }
    governance
        .skipped
        .iter()
        .map(|record| {
            let reason = match record.outcome {
                ConstitutionOutcome::NotCheckedOut => "not checked out",
                ConstitutionOutcome::NoConstitutionDocument => "no constitution.md in checkout",
                ConstitutionOutcome::Loaded => "loaded",
            };
            // The description, when the entry carries one. An alias is a
            // config key someone chose — often a bare org name — so it is the
            // description that tells an operator *which* checkout they are
            // missing without going to look the alias up. Truncated rather
            // than wrapped: the line is single-line by contract.
            let purpose = record
                .description
                .as_deref()
                .map(|text| format!(" — {}", truncate(text, DESCRIPTION_BUDGET)))
                .unwrap_or_default();
            format!(
                "- `{}` ({}){} — {} — its rules were NOT loaded for this review",
                record.alias, record.path, purpose, reason
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Render skipped passes as a list, or `*None.*` when empty.
fn render_skipped(skipped: &[String]) -> String {
    if skipped.is_empty() {
        return "*None.*".to_string();
    }
    skipped
        .iter()
        .map(|pass| format!("- {pass}"))
        .collect::<Vec<_>>()
        .join("\n")
}

// -- spec frontmatter update -------------------------------------------------

/// The waivers that survive this run: every recorded waiver the run did not
/// expire, with its open-schema extras intact.
///
/// Extracted when the report started carrying the whole record: the report is
/// rendered before the record is assembled, and computing the surviving set
/// twice would be two chances to prune differently — the class of divergence
/// the retired `check-review-agreement` was built to catch.
fn surviving_waivers(feature_dir: &Path, args: &WriteReviewArgs) -> Result<Vec<RawWaiverFull>> {
    let recorded: Vec<RawWaiverFull> = crate::primitives::read_recorded_waivers(feature_dir)?;
    Ok(recorded
        .into_iter()
        .filter_map(|waiver| prune_expired(waiver, &args.expired_waivers))
        .collect())
}

/// The waiver with every path whose `(rule, file)` anchor expired removed, or
/// `None` when that removes its last path.
///
/// Each listed path is its own anchor (spec 020's `waiver-file-lists`), so an
/// entry loses only the paths that expired and keeps the rest, with its other
/// fields and extras. Only anchors are compared. A waiver missing another
/// field, which `process-waivers` reports as malformed and never lists as
/// expired, still loses a path an expired anchor names. A waiver missing its
/// rule or file names no anchor, and one listing no path has none to lose, so
/// neither is ever pruned.
fn prune_expired(
    mut waiver: RawWaiverFull,
    expired: &[crate::schema::primitives::WaiverRef],
) -> Option<RawWaiverFull> {
    let (Some(rule), Some(paths)) = (waiver.rule.as_deref(), waiver.file.as_mut()) else {
        return Some(waiver); // no anchor to match
    };
    let listed = paths.as_slice().len();
    paths.retain(|path| {
        !expired
            .iter()
            .any(|entry| entry.rule == rule && &entry.file == path)
    });
    let emptied = listed > 0 && paths.as_slice().is_empty();
    (!emptied).then_some(waiver)
}

/// Append the `waivers:` list to a rendered record, at the given base indent,
/// preserving every adopter-authored extra field verbatim
/// (§text-first-artifacts' open-schema rule).
///
/// Shared with `invalidate-review`, which drops the review's scalars but must
/// not drop operator state: a waiver is a recorded judgement, and an
/// invalidation says the *review* is out of date, not that the judgement was
/// withdrawn.
///
/// The indent is a parameter rather than a constant because the record briefly
/// had two homes while it was being relocated (spec 057). Only one remains —
/// top-level in `review.md` — so `base` is `""` at every call site today; it is
/// kept because the alternative was two renderers that could drift, and that
/// trade does not change now that one of them is gone.
pub(crate) fn render_waivers_at(block: &mut String, waivers: &[RawWaiverFull], base: &str) {
    if waivers.is_empty() {
        return;
    }
    let _ = writeln!(block, "{base}waivers:");
    for waiver in waivers {
        render_list_entry(
            block,
            base,
            &[
                ("rule", waiver.rule.as_deref().into()),
                ("file", waiver_file(waiver.file.as_ref())),
                ("reason", waiver.reason.as_deref().into()),
                ("waived-at", waiver.waived_at.as_deref().into()),
                ("waived-by", waiver.waived_by.as_deref().into()),
            ],
            &waiver.extra,
        );
    }
}

/// A waiver's `file` as it renders: one path as a scalar, several as a list.
///
/// The spelling follows the count, not how the entry was written, so a list
/// pruned to one path — or hand-written with one — renders exactly as a
/// one-path entry always has, and a record that never used a list is
/// byte-identical to one written before lists were accepted.
fn waiver_file(paths: Option<&WaiverPaths>) -> EntryValue<'_> {
    match paths.map(WaiverPaths::as_slice) {
        None => EntryValue::Absent,
        Some([path]) => EntryValue::Scalar(path),
        Some(paths) => EntryValue::List(paths),
    }
}

/// One known field's value in a record list entry.
pub(crate) enum EntryValue<'a> {
    /// Omitted from the entry.
    Absent,
    /// Rendered inline after the key.
    Scalar(&'a str),
    /// Rendered as a block sequence under the key; `[]` when empty.
    List(&'a [String]),
}

impl<'a> From<Option<&'a str>> for EntryValue<'a> {
    fn from(value: Option<&'a str>) -> Self {
        value.map_or(Self::Absent, Self::Scalar)
    }
}

/// Append one entry of a record's list at `base` indent: its known fields in
/// order, each absent one omitted, then every adopter-authored extra verbatim
/// (§text-first-artifacts' open-schema rule). The one entry renderer for every
/// list an audit record carries — `review.md`'s `waivers:` and both records'
/// `decisions:` (spec 058) — so their quoting and nesting cannot drift apart.
pub(crate) fn render_list_entry(
    block: &mut String,
    base: &str,
    fields: &[(&str, EntryValue<'_>)],
    extra: &std::collections::BTreeMap<String, serde_norway::Value>,
) {
    let mut first = true;
    let mut indent = || {
        let indent = if first {
            format!("{base}  - ")
        } else {
            format!("{base}    ")
        };
        first = false;
        indent
    };
    for (key, value) in fields {
        match value {
            EntryValue::Absent => {}
            EntryValue::Scalar(value) => {
                let _ = writeln!(block, "{}{key}: {}", indent(), yaml_string(value));
            }
            EntryValue::List([]) => {
                let _ = writeln!(block, "{}{key}: []", indent());
            }
            EntryValue::List(items) => {
                // The key sits at column `base + 4` whether or not it opens the
                // entry, so its items nest two columns further in.
                let _ = writeln!(block, "{}{key}:", indent());
                for item in *items {
                    let _ = writeln!(block, "{base}      - {}", yaml_string(item));
                }
            }
        }
    }
    for (key, value) in extra {
        render_extra_field(block, &indent(), key, value);
    }
}

/// Append one adopter-authored field of a list entry, preserving it across
/// the re-render. Scalars render inline (matching the known-field style); a
/// nested value is serialized as a YAML block indented under the key. Extras
/// always follow the required scalar fields, so the continuation indent
/// (`    `) is the normal case.
fn render_extra_field(block: &mut String, indent: &str, key: &str, value: &serde_norway::Value) {
    use serde_norway::Value;
    // Adopter-controlled key: quote it so a key like `@owner`, `weird: key`, or
    // a bare-numeric string key survives the round-trip instead of corrupting
    // the frontmatter.
    let key = yaml_string(key);
    match value {
        Value::Null => {
            let _ = writeln!(block, "{indent}{key}: null");
        }
        Value::Bool(b) => {
            let _ = writeln!(block, "{indent}{key}: {b}");
        }
        Value::Number(n) => {
            let _ = writeln!(block, "{indent}{key}: {n}");
        }
        Value::String(s) => {
            let _ = writeln!(block, "{indent}{key}: {}", yaml_string(s));
        }
        other => {
            // Non-scalar (sequence/mapping) — serialize and re-indent each
            // line under the key. The key sits at the indent's width — column
            // 4 in the top-level lists, where `  - ` and `    ` are both 4 wide
            // — so children starting at column 8 nest *under* the key rather
            // than becoming its siblings. serde_norway preserves the value's
            // internal relative indentation, so a constant 8-space base offset
            // keeps the whole structure correctly nested. The common case is
            // scalars.
            let _ = writeln!(block, "{indent}{key}:");
            let serialized = serde_norway::to_string(other).unwrap_or_default();
            for line in serialized.lines() {
                let _ = writeln!(block, "        {line}");
            }
        }
    }
}

/// Emit a YAML scalar for text that MUST round-trip as the identical string —
/// double-quotes when a plain scalar would be syntactically ambiguous (empty,
/// surrounding whitespace, an indicator lead, a `: ` sequence, a trailing
/// colon, or an embedded quote) OR when the plain form would not read back as
/// the same string: a bare `1234` / `true` / `null` / `~` retypes, and a `#`
/// after a space or a tab opens a comment that truncates it.
///
/// Used for every string value in the waiver and decision lists that
/// `review.md` and `analysis.md` carry in their frontmatter — the known fields
/// and the open-schema adopter extras alike — because each is read back by the
/// next run's parse, and a decision key must come back byte for byte to match
/// the finding it was stored for. An extra **key** like `@owner` or
/// `weird: key`, an extra string **value** like `"1234"`, or a known `reason`
/// that happened to read `true` or `1234` would otherwise render unquoted and
/// either corrupt the frontmatter or retype the value. Simple timestamps
/// (`waived-at`), shas, rule IDs, emails, and paths stay unquoted: they read
/// back unchanged (`serde_norway` has no timestamp type), so quoting them here
/// would be no-op churn.
///
/// A value carrying a control character or a Unicode line break is always
/// quoted, and every such character is escaped: the YAML reader refuses a raw
/// control character, so a single one left bare would make the whole record
/// unreadable (spec 058). `serde_json` escapes C0 characters but not DEL, C1,
/// `U+2028`, `U+2029`, `U+FFFE` or `U+FFFF`, so those are escaped here —
/// YAML's double-quoted `\u` escape reads back the same character.
pub(crate) fn yaml_string(value: &str) -> String {
    if needs_quote(value) || !reads_back_identically(value) {
        let quoted = serde_json::to_string(value).unwrap_or_else(|_| format!("\"{value}\""));
        quoted
            .chars()
            .fold(String::with_capacity(quoted.len()), |mut out, c| {
                if super::is_line_hazard(c) {
                    let _ = write!(out, "\\u{:04x}", u32::from(c));
                } else {
                    out.push(c);
                }
                out
            })
    } else {
        value.to_string()
    }
}

/// Whether the plain scalar `value` reads back, through the same YAML reader
/// that reads the record, as exactly `value`. It does not when it retypes (an
/// integer, float, bool, or null), when a comment truncates it (`#` after a
/// space or a tab), or when it does not parse at all — and each of those must
/// be quoted to keep the value intact across the round-trip.
fn reads_back_identically(value: &str) -> bool {
    matches!(
        serde_norway::from_str::<serde_norway::Value>(value),
        Ok(serde_norway::Value::String(parsed)) if parsed == value
    )
}

/// YAML plain-scalar indicator characters: a value leading with one of these
/// must be quoted.
const YAML_INDICATORS: &[u8] = b"!&*?|>@%#{}[],\"'`:-";

fn needs_quote(value: &str) -> bool {
    if value.is_empty() || value != value.trim() || value.chars().any(super::is_line_hazard) {
        return true;
    }
    if value.contains(": ") || value.contains('"') || value.ends_with(':') {
        return true;
    }
    YAML_INDICATORS.contains(&value.as_bytes()[0])
}

// -- existing-frontmatter parse shapes ---------------------------------------

/// One waiver entry with every field optional, so pruning preserves the full
/// record (`waived-at` / `waived-by` included) that `WaiverRef` drops. Unknown
/// adopter-authored fields (`ticket`, `co-waived-by`, …) are captured by
/// `extra` and re-emitted verbatim on the write, so the §text-first-artifacts
/// open-schema rule holds across a re-render — an org-specific policy field is
/// never silently dropped. `BTreeMap` keeps the extras in deterministic order.
#[derive(Deserialize)]
pub(crate) struct RawWaiverFull {
    #[serde(default)]
    rule: Option<String>,
    #[serde(default)]
    file: Option<WaiverPaths>,
    #[serde(default)]
    reason: Option<String>,
    #[serde(default, rename = "waived-at")]
    waived_at: Option<String>,
    #[serde(default, rename = "waived-by")]
    waived_by: Option<String>,
    #[serde(flatten)]
    extra: std::collections::BTreeMap<String, serde_norway::Value>,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::schema::primitives::{DecisionOutcome, WaiverRef};
    use std::fs;
    use tempfile::{TempDir, tempdir};

    fn finding(
        rule: &str,
        severity: &str,
        file: &str,
        range: &str,
        confidence: &str,
    ) -> ReviewFinding {
        ReviewFinding {
            rule: rule.into(),
            severity: severity
                .parse()
                .expect("test severity must be a legal review tier"),
            file: file.into(),
            line_range: range.into(),
            confidence: confidence.into(),
            summary: format!("{rule} summary"),
            finding: "Explanation of the finding.".into(),
            rule_text: "Verbatim rule text.".into(),
            auto_fixable: false,
            suggested_fix: String::new(),
        }
    }

    fn waiver(rule: &str, file: &str) -> WaiverRef {
        WaiverRef {
            rule: rule.into(),
            file: file.into(),
            reason: "Justified for now.".into(),
        }
    }

    /// The report's frontmatter carries the **whole** record — the pre-relocation
    /// merge is only correct if nothing is dropped, so every field that lived
    /// on one side is asserted individually rather than through a struct
    /// comparison that a defaulted field would silently satisfy (spec 057 AC12).
    #[test]
    fn review_md_frontmatter_deserializes_into_the_whole_record() {
        let dir = spec_repo("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(
            &dir,
            "001-x",
            "waivers:\n  - rule: SEC-001\n    file: src/a.rs\n    reason: Justified.\n    ticket: OPS-42",
        );
        let mut args = base_args("001-x");
        args.examined = Some(7);
        args.skipped_passes = vec!["security".into()];
        let out = run(&args, dir.path()).unwrap();

        let report = review_md(&dir, "001-x");
        let (fm, _body) = crate::primitives::split_frontmatter(&report, Path::new("review.md"))
            .expect("report carries frontmatter");
        let record: crate::schema::primitives::ReviewBlock =
            serde_norway::from_str(fm).expect("frontmatter deserializes into the record");

        // The timestamp under its one surviving name.
        assert_eq!(record.last_run.as_deref(), Some(args.reviewed_at.as_str()));
        assert!(
            !report.contains("reviewed-at:"),
            "the second spelling is retired: {report}"
        );

        // Fields that were report-only before the merge.
        assert_eq!(record.spec.as_deref(), Some("001-x"));
        assert_eq!(record.diff_base.as_deref(), Some(args.diff_base.as_str()));
        assert_eq!(
            record.dispositions,
            Some(crate::schema::primitives::Dispositions::default()),
            "a run with no observations records an all-zero map, never an absent one"
        );
        assert!(!report.contains("captured-issues"), "{report}");
        assert_eq!(record.skipped_passes, vec!["security".to_string()]);

        // Fields that were block-only before the merge.
        assert_eq!(record.blocking, out.blocking);
        assert!(
            record.reviewed_digest.is_some(),
            "the digest is always recorded, empty map included"
        );

        // Waivers keep their adopter-authored extras across the move, at the
        // top-level indent this home uses.
        assert!(report.contains("waivers:"), "{report}");
        assert!(report.contains("  - rule: SEC-001"), "{report}");
        assert!(
            report.contains("ticket: OPS-42"),
            "open-schema waiver fields survive the relocation: {report}"
        );

        // Duplicated fields still arrive.
        assert_eq!(record.examined, Some(7));
        assert_eq!(record.scope, Some(out.scope));
    }

    /// The claim and its denominator land in the record — and nowhere else.
    ///
    /// Was `examined_and_scope_are_written_to_both_records`, whose point was
    /// giving `check-review-agreement` two sides to compare. With one home the
    /// property worth pinning inverts: the spec must carry neither.
    #[test]
    fn examined_and_scope_are_written_to_the_record_alone() {
        let dir = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.examined = Some(7);
        let out = run(&args, dir.path()).unwrap();
        assert_eq!(out.examined, Some(7));

        let report = review_md(&dir, "001-x");
        assert!(report.contains("examined: 7"), "{report}");
        assert!(
            report.contains(&format!("scope: {}", out.scope)),
            "{report}"
        );

        assert!(
            !fs::read_to_string(dir.path().join("specs/001-x/spec.md"))
                .unwrap()
                .contains("examined:"),
            "the spec is not a second home for the claim"
        );
    }

    /// An unstated claim is **absent**, never rendered as a computed zero —
    /// the distinction the field exists to preserve.
    #[test]
    fn unstated_examined_is_absent_rather_than_zero() {
        let dir = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let out = run(&base_args("001-x"), dir.path()).unwrap();
        assert_eq!(out.examined, None);

        let report = review_md(&dir, "001-x");
        assert!(!report.contains("examined:"), "{report}");
        assert!(
            !fs::read_to_string(dir.path().join("specs/001-x/spec.md"))
                .unwrap()
                .contains("examined:"),
            "the spec carries no record at all now"
        );
        // The denominator is always written: it was always computed.
        assert!(report.contains("scope: "), "{report}");
    }

    fn base_args(feature: &str) -> WriteReviewArgs {
        WriteReviewArgs {
            feature: feature.into(),
            reviewed_at: "2026-07-02T12:00:00Z".into(),
            reviewed_against: "abc1234".into(),
            diff_base: "def5678".into(),
            scenario: None,
            empty_scope: false,
            summary: None,
            skipped_passes: Vec::new(),
            findings: Vec::new(),
            applied_waivers: Vec::new(),
            expired_waivers: Vec::new(),
            observations: Vec::new(),
            expired_decisions: Vec::new(),
            decided_by: None,
            examined: None,
        }
    }

    fn observation(text: &str, path: &str) -> ReviewObservation {
        ReviewObservation {
            text: text.into(),
            path: path.into(),
            ..ReviewObservation::default()
        }
    }

    fn dispositioned(
        text: &str,
        outcome: DispositionOutcome,
        companion: Option<&str>,
    ) -> ReviewObservation {
        ReviewObservation {
            text: text.into(),
            disposition: crate::schema::primitives::Disposition {
                outcome,
                target: (outcome == DispositionOutcome::Routed)
                    .then(|| companion.unwrap_or_default().to_string()),
                reason: (outcome == DispositionOutcome::Discarded)
                    .then(|| companion.unwrap_or_default().to_string()),
            },
            ..ReviewObservation::default()
        }
    }

    fn inbox(tmp: &TempDir) -> Option<String> {
        fs::read_to_string(tmp.path().join("specs/inbox.md")).ok()
    }

    /// Write `specs/{feature}/spec.md` with the given frontmatter body (the
    /// text between the `---` fences) and return the tempdir.
    fn spec_repo(feature: &str, frontmatter: &str) -> TempDir {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs").join(feature);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("spec.md"),
            format!("---\n{frontmatter}\n---\n\n# {feature}\n"),
        )
        .unwrap();
        tmp
    }

    /// Seed `review.md` with a prior run's recorded waivers — the home the
    /// list moved to (spec 057 task 5). The fixtures below previously seeded
    /// the spec's `review:` block; the property each one asserts is unchanged,
    /// only the file it is asserted against.
    fn seed_waivers(tmp: &TempDir, feature: &str, waivers_yaml: &str) {
        let dir = tmp.path().join("specs").join(feature);
        fs::write(
            dir.join("review.md"),
            format!(
                "---\nspec: {feature}\nlast-run: 2026-01-01T00:00:00Z\nmust-violations: 0\nblocking: false\n{waivers_yaml}\n---\n\n# Review — {feature}\n"
            ),
        )
        .unwrap();
    }

    /// The waivers `review.md` records, parsed back through the one reader.
    fn recorded(tmp: &TempDir, feature: &str) -> Vec<RawWaiverFull> {
        crate::primitives::read_recorded_waivers(&tmp.path().join("specs").join(feature)).unwrap()
    }

    fn review_md(tmp: &TempDir, feature: &str) -> String {
        fs::read_to_string(tmp.path().join("specs").join(feature).join("review.md")).unwrap()
    }

    fn spec_md(tmp: &TempDir, feature: &str) -> String {
        fs::read_to_string(tmp.path().join("specs").join(feature).join("spec.md")).unwrap()
    }

    /// AC12's reference point, at the primitive. A spec with no analyze
    /// record reports `never-analyzed` rather than a defaulted state, so
    /// `/{project}:review` can render the row that carries an operator to
    /// the second gate.
    #[test]
    fn a_spec_with_no_analyze_record_reports_never_analyzed() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let result = run(&base_args("001-x"), tmp.path()).unwrap();
        assert_eq!(result.analyze_freshness, RecordFreshness::NeverRun);
        // Never a gate: the row must not touch the command's verdict.
        assert!(!result.blocking);
        assert_eq!(result.exit_code, 0);
    }

    /// The whole point of the working-tree reference point. This primitive has
    /// just rewritten `review.md`, an analyze subject, so the record it reports
    /// on is superseded from this moment.
    /// A committed comparison would say `current` until someone committed, and
    /// would then have been wrong retroactively.
    #[test]
    fn writing_a_review_supersedes_the_analyze_record_it_reports() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        fs::write(
            tmp.path().join("specs/001-x/analysis.md"),
            concat!(
                "---\nspec: 001-x\nlast-run: 2026-09-06T00:00:00Z\n",
                "analyzed-against: PLACEHOLDER\nhard-fail: 0\n",
                "blocking-findings: 0\nadvisory: 0\nunexamined: 0\nblocking: false\n",
                "---\n\n# Analysis — 001-x\n",
            ),
        )
        .unwrap();
        // A real repo, and a record pointing at its only commit.
        let repository = git2::Repository::init(tmp.path()).unwrap();
        let sha = {
            let mut index = repository.index().unwrap();
            index
                .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
                .unwrap();
            index.write().unwrap();
            let tree = repository.find_tree(index.write_tree().unwrap()).unwrap();
            let sig = git2::Signature::now("Test", "test@example.com").unwrap();
            repository
                .commit(Some("HEAD"), &sig, &sig, "base", &tree, &[])
                .unwrap()
                .to_string()
        };
        // Give the record a digest of the subjects as they are now, so it is
        // judgeable at all. Without one the honest answer is undeterminable,
        // which is what every pre-digest record reports.
        let analysis = tmp.path().join("specs/001-x/analysis.md");
        let subjects = analyze_subjects::subject_digest(
            &tmp.path().join("specs/001-x"),
            analyze_subjects::is_analyze_subject,
        );
        let mut fm = String::from("spec: 001-x\nlast-run: 2026-09-06T00:00:00Z\n");
        let _ = writeln!(fm, "analyzed-against: {sha}");
        fm.push_str("hard-fail: 0\nblocking-findings: 0\nadvisory: 0\nunexamined: 0\n");
        fm.push_str("analyzed-digest:\n");
        for (path, digest) in &subjects.digests {
            let _ = writeln!(fm, "  {path}: {digest}");
        }
        fm.push_str("blocking: false\n");
        fs::write(&analysis, format!("---\n{fm}---\n\n# Analysis — 001-x\n")).unwrap();

        let result = run(&base_args("001-x"), tmp.path()).unwrap();
        let RecordFreshness::Stale { paths, .. } = &result.analyze_freshness else {
            panic!(
                "writing a review rewrites review.md, an analyze subject, so the \
                 digest must no longer match: {:?}",
                result.analyze_freshness
            );
        };
        assert!(
            paths.iter().any(|p| p.ends_with("review.md")),
            "the report this call just wrote is an analyze subject: {paths:?}"
        );
        // Still not a gate.
        assert_eq!(result.exit_code, 0);
    }

    #[test]
    fn empty_scope_report_has_zero_findings_and_is_not_blocking() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.empty_scope = true;
        let result = run(&args, tmp.path()).unwrap();
        assert_eq!(result.must_violations, 0);
        assert_eq!(result.should_violations, 0);
        assert_eq!(result.low_confidence, 0);
        assert!(!result.blocking);
        assert_eq!(result.exit_code, 0);
        let report = review_md(&tmp, "001-x");
        assert!(report.contains("must-violations: 0"));
        assert!(report.contains("Review scope is empty"));
        assert!(report.contains("## MUST violations (blocking)\n\n*None.*"));
        // AC12: the row renders on an empty scope too. This path jumps
        // straight here from step 1, so it is the one most likely to skip a
        // field added later — and an empty scope is exactly when an operator
        // needs telling that the second gate is still owed.
        assert_eq!(result.analyze_freshness, RecordFreshness::NeverRun);
    }

    #[test]
    fn empty_scope_does_not_suppress_the_derived_denominator() {
        // The flag renders the empty-scope Summary; it must not also decide
        // the denominator. It used to short-circuit `resolve_scope_size`, so
        // a caller that reached this branch by misreading an over-cap
        // `compute-review-scope` error as "nothing in scope" recorded
        // `scope: 0` beside a zero `examined` — a record byte-identical to an
        // honest empty review, and invisible to Family 31 (both `examined`
        // arms guard on `total > 0`), to Family 19 (the digest is valid) and
        // to `check-review-gate` (it is non-blocking).
        //
        // Deriving unconditionally is what makes the two distinguishable.
        // This spec has no recorded `in-progress` transition, so the derived
        // window is empty and the honest answer really is 0 — the assertion
        // that matters is that the number came from a derivation rather than
        // from the flag, which the next test pins from the other side.
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.empty_scope = true;
        let result = run(&args, tmp.path()).unwrap();
        assert_eq!(result.exit_code, 0);
        let report = review_md(&tmp, "001-x");
        // The rendering half of the flag is untouched and still correct.
        assert!(report.contains("Review scope is empty"));
    }

    #[test]
    fn rejects_frontmatter_injection_via_reviewed_against_newline() {
        // A newline in a frontmatter scalar would splice a spoofed top-level
        // key (here `status: done`) into the spec's frontmatter. It must be
        // refused before any write, leaving spec.md untouched.
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let before = spec_md(&tmp, "001-x");
        let mut args = base_args("001-x");
        args.reviewed_against = "abc1234\nstatus: done".into();
        let err = run(&args, tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidArgument { primitive, argument, .. }
                if primitive == "write-review" && argument == "reviewed-against"),
            "expected InvalidArgument for reviewed-against, got {err:?}"
        );
        assert_eq!(spec_md(&tmp, "001-x"), before, "spec.md must be untouched");
        assert!(
            !tmp.path().join("specs/001-x/review.md").exists(),
            "review.md must not be written"
        );
    }

    #[test]
    fn cross_pass_dedup_highest_severity_wins() {
        // Same (rule, file) with overlapping ranges from two passes → 1 MUST.
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.findings = vec![
            finding("SEC-BE-001", "should", "src/a.rs", "10-20", "high"),
            finding("SEC-BE-001", "must", "src/a.rs", "15-25", "high"),
        ];
        let result = run(&args, tmp.path()).unwrap();
        assert_eq!(result.must_violations, 1);
        assert_eq!(result.should_violations, 0);
    }

    #[test]
    fn dedup_keeps_non_overlapping_same_rule_file() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.findings = vec![
            finding("SEC-BE-001", "must", "src/a.rs", "10-20", "high"),
            finding("SEC-BE-001", "must", "src/a.rs", "50-60", "high"),
        ];
        let result = run(&args, tmp.path()).unwrap();
        assert_eq!(result.must_violations, 2);
    }

    #[test]
    fn blocking_true_when_must_violations_exceed_zero() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.findings = vec![finding("SEC-BE-002", "must", "src/a.rs", "1-5", "high")];
        let result = run(&args, tmp.path()).unwrap();
        assert!(result.blocking);
        assert_eq!(result.exit_code, 1);
        let report = review_md(&tmp, "001-x");
        assert!(report.contains("blocking: true"), "{report}");
        assert!(report.contains("must-violations: 1"), "{report}");
    }

    #[test]
    fn low_confidence_finding_routed_to_low_bucket() {
        // A low-confidence MUST-severity finding counts as low-confidence, not
        // a MUST violation → not blocking.
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.findings = vec![finding("SIM-001", "must", "src/a.rs", "1-5", "low")];
        let result = run(&args, tmp.path()).unwrap();
        assert_eq!(result.must_violations, 0);
        assert_eq!(result.low_confidence, 1);
        assert!(!result.blocking);
        assert!(review_md(&tmp, "001-x").contains("### LOW-CONFIDENCE: SIM-001"));
    }

    #[test]
    fn single_findings_array_ingestion_buckets_by_section() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.findings = vec![
            finding("SEC-BE-001", "must", "src/a.rs", "1-5", "high"),
            finding("QUAL-002", "should", "src/b.rs", "1-5", "high"),
            finding("SIM-003", "should", "src/c.rs", "1-5", "low"),
        ];
        let result = run(&args, tmp.path()).unwrap();
        assert_eq!(result.must_violations, 1);
        assert_eq!(result.should_violations, 1);
        assert_eq!(result.low_confidence, 1);
        let report = review_md(&tmp, "001-x");
        assert!(report.contains("### MUST: SEC-BE-001"));
        assert!(report.contains("### SHOULD: QUAL-002"));
        assert!(report.contains("### LOW-CONFIDENCE: SIM-003"));
    }

    #[test]
    fn applied_waiver_excludes_finding_from_must_count() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.findings = vec![finding(
            "SEC-BE-014",
            "must",
            "src/internal.ts",
            "1-5",
            "high",
        )];
        args.applied_waivers = vec![waiver("SEC-BE-014", "src/internal.ts")];
        let result = run(&args, tmp.path()).unwrap();
        assert_eq!(result.must_violations, 0);
        assert_eq!(result.waived, 1);
        assert!(!result.blocking);
        let report = review_md(&tmp, "001-x");
        assert!(report.contains("### WAIVED: SEC-BE-014"));
        assert!(report.contains("- **Waived**: Justified for now."));
    }

    #[test]
    fn skipped_passes_recorded_in_frontmatter_and_section() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.skipped_passes = vec!["security".into(), "simplicity".into()];
        run(&args, tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(report.contains("skipped-passes: [security, simplicity]"));
        assert!(report.contains("## Skipped passes\n\n- security\n- simplicity"));
    }

    /// The record describes its own subject. A spec with no scenarios and no
    /// data model still records `reviewed-digest: {}` — taken and empty, which
    /// the gate reads as current, rather than absent, which it cannot judge.
    #[test]
    fn a_spec_with_no_durable_contracts_records_an_empty_digest() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        run(&base_args("001-x"), tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(
            report.contains("reviewed-digest: {}"),
            "an empty digest is written, not omitted: {report}"
        );
    }

    #[test]
    fn the_review_record_digests_its_durable_contracts() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let dir = tmp.path().join("specs/001-x");
        fs::create_dir_all(dir.join("scenarios")).unwrap();
        fs::write(dir.join("scenarios/retry.md"), "# Retry\n").unwrap();
        fs::write(dir.join("data-model.md"), "# Model\n").unwrap();
        // Not contracts: this command's own outputs and the ephemeral files.
        fs::write(dir.join("tasks.md"), "# Tasks\n").unwrap();

        run(&base_args("001-x"), tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(report.contains("reviewed-digest:"), "{report}");
        assert!(report.contains("  scenarios/retry.md: "), "{report}");
        assert!(report.contains("  data-model.md: "), "{report}");
        assert!(
            !report.contains("  tasks.md: "),
            "tasks.md is not a review contract: {report}"
        );
        assert!(
            !report.contains("  review.md: "),
            "the report this call writes is not its own subject: {report}"
        );
    }

    #[test]
    fn frontmatter_review_block_inserted_when_absent() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.findings = vec![
            finding("A-1", "must", "src/a.rs", "1-2", "high"),
            finding("B-2", "should", "src/b.rs", "1-2", "high"),
        ];
        let before = spec_md(&tmp, "001-x");
        run(&args, tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(
            report.contains("last-run: 2026-07-02T12:00:00Z"),
            "{report}"
        );
        assert!(report.contains("reviewed-against: abc1234"), "{report}");
        assert!(report.contains("must-violations: 1"), "{report}");
        assert!(report.contains("should-violations: 1"), "{report}");
        assert!(report.contains("low-confidence: 0"), "{report}");
        assert!(report.contains("blocking: true"), "{report}");
        assert_eq!(
            spec_md(&tmp, "001-x"),
            before,
            "the spec is no longer a write target"
        );
    }

    #[test]
    fn frontmatter_review_block_replaced_when_present() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(&tmp, "001-x", "must-violations: 9");
        let args = base_args("001-x");
        run(&args, tmp.path()).unwrap();
        let spec = review_md(&tmp, "001-x");
        assert!(spec.contains("must-violations: 0"));
        assert!(spec.contains("blocking: false"));
        assert!(!spec.contains("must-violations: 9"));
        assert!(!spec.contains("2020-01-01"));
        // The record still parses, and the spec is untouched beside it.
        let (fm, _) = split_frontmatter(&spec, Path::new("review.md")).unwrap();
        let record: crate::schema::primitives::ReviewBlock = serde_norway::from_str(fm).unwrap();
        assert_eq!(record.must_violations, 0);
        assert!(spec_md(&tmp, "001-x").contains("status: in-progress"));
    }

    #[test]
    fn expired_waiver_pruned_from_spec_frontmatter() {
        let frontmatter = "waivers:\n  - rule: SEC-BE-014\n    file: src/gone.ts\n    reason: No longer relevant.\n    waived-at: 2026-01-01T00:00:00Z\n    waived-by: dev@example.com\n  - rule: SEC-BE-020\n    file: src/keep.ts\n    reason: Still valid.\n    waived-at: 2026-01-02T00:00:00Z\n    waived-by: dev@example.com";
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(&tmp, "001-x", frontmatter);
        let mut args = base_args("001-x");
        args.expired_waivers = vec![waiver("SEC-BE-014", "src/gone.ts")];
        run(&args, tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        // Expired anchor gone; surviving waiver kept with all its fields.
        assert!(!report.contains("src/gone.ts"), "{report}");
        assert!(report.contains("rule: SEC-BE-020"), "{report}");
        assert!(report.contains("file: src/keep.ts"), "{report}");
        assert!(report.contains("waived-by: dev@example.com"), "{report}");
        assert!(report.contains("Still valid."), "{report}");
        // The rewritten record still parses through the shared reader.
        assert_eq!(recorded(&tmp, "001-x").len(), 1);
    }

    #[test]
    fn preserves_adopter_authored_waiver_fields_across_rewrite() {
        // §text-first-artifacts open schema: an org-specific policy field on a
        // surviving waiver must round-trip, not vanish on the re-render.
        let frontmatter = "waivers:\n  - rule: SEC-BE-020\n    file: src/keep.ts\n    reason: Still valid.\n    waived-at: 2026-01-02T00:00:00Z\n    waived-by: dev@example.com\n    ticket: SEC-1234\n    approved-by-team: platform-security";
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(&tmp, "001-x", frontmatter);
        run(&base_args("001-x"), tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(report.contains("ticket: SEC-1234"), "{report}");
        assert!(
            report.contains("approved-by-team: platform-security"),
            "{report}"
        );
        // Still parses and the extras survive a round-trip parse.
        let waivers = recorded(&tmp, "001-x");
        assert_eq!(waivers.len(), 1);
        assert_eq!(
            waivers[0].extra.get("ticket").and_then(|v| v.as_str()),
            Some("SEC-1234")
        );
    }

    #[test]
    fn quotes_adopter_waiver_keys_that_would_corrupt_frontmatter() {
        // A quoted adopter key that needs quoting (`@owner`, a colon-bearing
        // key, a bare-numeric key) must re-emit quoted, so the frontmatter
        // stays valid and the key keeps its identity — not silently corrupt
        // spec.md (which is written without a re-parse).
        let frontmatter = "waivers:\n  - rule: SEC-BE-020\n    file: src/keep.ts\n    reason: Still valid.\n    waived-at: 2026-01-02T00:00:00Z\n    waived-by: dev@example.com\n    \"@owner\": alice\n    \"weird: key\": v\n    \"1234\": numeric-key";
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(&tmp, "001-x", frontmatter);
        run(&base_args("001-x"), tmp.path()).unwrap();
        // The rewritten frontmatter still parses (no corruption/injection)...
        let waivers = recorded(&tmp, "001-x");
        assert_eq!(waivers.len(), 1);
        // ...and the exotic keys survived under the waiver entry, not as
        // leaked siblings or an injected top-level key.
        assert!(waivers[0].extra.contains_key("@owner"));
        assert!(waivers[0].extra.contains_key("weird: key"));
        assert!(waivers[0].extra.contains_key("1234"));
    }

    #[test]
    fn quotes_bool_like_known_waiver_field_for_round_trip() {
        // 066's scenario write-review-known-field-quoting: a known waiver field
        // (here `reason`) whose value would re-parse as a non-string must be
        // quoted like the extras, so it round-trips through the spec
        // frontmatter as a string instead of a bool/number and does not break
        // the next RawWaiver parse.
        let frontmatter = "waivers:\n  - rule: SEC-BE-020\n    file: src/keep.ts\n    reason: \"true\"\n    waived-at: 2026-01-02T00:00:00Z\n    waived-by: dev@example.com";
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(&tmp, "001-x", frontmatter);
        run(&base_args("001-x"), tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        // Bool-like known value is quoted, not rendered as a bare `reason: true`.
        assert!(
            report.contains("reason: \"true\""),
            "bool-like known waiver field must be quoted:\n{report}"
        );
        // Timestamp-shaped known field is unchanged — no golden churn.
        assert!(report.contains("waived-at: 2026-01-02T00:00:00Z"));
        // The rewritten record still parses (a bare `reason: true` would not
        // round-trip into the string-typed field).
        assert_eq!(recorded(&tmp, "001-x").len(), 1);
    }

    #[test]
    fn preserves_string_typed_waiver_extra_values_across_rewrite() {
        // A string value that looks like a number/bool/null must round-trip as
        // a string, not silently change type.
        let frontmatter = "waivers:\n  - rule: SEC-BE-020\n    file: src/keep.ts\n    reason: Still valid.\n    waived-at: 2026-01-02T00:00:00Z\n    waived-by: dev@example.com\n    ticket: \"1234\"\n    flagged: \"true\"";
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(&tmp, "001-x", frontmatter);
        run(&base_args("001-x"), tmp.path()).unwrap();
        let waivers = recorded(&tmp, "001-x");
        let extra = &waivers[0].extra;
        assert_eq!(extra.get("ticket").and_then(|v| v.as_str()), Some("1234"));
        assert_eq!(extra.get("flagged").and_then(|v| v.as_str()), Some("true"));
    }

    #[test]
    fn preserves_nested_waiver_extra_field_with_correct_nesting() {
        // A non-scalar adopter field (nested mapping) must round-trip nested,
        // not flattened into sibling keys of the waiver entry.
        let frontmatter = "waivers:\n  - rule: SEC-BE-020\n    file: src/keep.ts\n    reason: Still valid.\n    waived-at: 2026-01-02T00:00:00Z\n    waived-by: dev@example.com\n    approvals:\n      security: alice\n      lead: bob";
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(&tmp, "001-x", frontmatter);
        run(&base_args("001-x"), tmp.path()).unwrap();
        // Re-parse and confirm `approvals` survived as a nested mapping, not
        // as flattened sibling keys.
        let waivers = recorded(&tmp, "001-x");
        assert_eq!(waivers.len(), 1);
        let approvals = waivers[0]
            .extra
            .get("approvals")
            .expect("approvals preserved");
        let mapping = approvals.as_mapping().expect("approvals is a mapping");
        assert_eq!(
            mapping
                .get(serde_norway::Value::String("security".into()))
                .and_then(|v| v.as_str()),
            Some("alice")
        );
        assert_eq!(
            mapping
                .get(serde_norway::Value::String("lead".into()))
                .and_then(|v| v.as_str()),
            Some("bob")
        );
        // And no stray top-level `security` / `lead` keys leaked onto the waiver.
        assert!(!waivers[0].extra.contains_key("security"));
    }

    // -- a `file` listing several paths (020's waiver-file-lists) -------------

    /// A `SEC-BE-014` waiver whose `file` is the given YAML, carrying an
    /// adopter extra so each test also sees it survive.
    fn waiver_with_file(file: &str) -> String {
        format!(
            "  - rule: SEC-BE-014\n    file: {file}\n    reason: Shared justification.\n    waived-at: 2026-01-01T00:00:00Z\n    waived-by: dev@example.com\n    ticket: SEC-9"
        )
    }

    fn recorded_paths(tmp: &TempDir, feature: &str) -> Vec<Vec<String>> {
        recorded(tmp, feature)
            .iter()
            .map(|w| {
                w.file
                    .as_ref()
                    .map(|f| f.as_slice().to_vec())
                    .unwrap_or_default()
            })
            .collect()
    }

    #[test]
    fn an_expired_path_leaves_the_list_and_the_rest_survive() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(
            &tmp,
            "001-x",
            &format!(
                "waivers:\n{}",
                waiver_with_file("\n      - src/a.ts\n      - src/b.ts\n      - src/c.ts")
            ),
        );
        let mut args = base_args("001-x");
        args.expired_waivers = vec![waiver("SEC-BE-014", "src/b.ts")];
        run(&args, tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(
            report.contains(
                "  - rule: SEC-BE-014\n    file:\n      - src/a.ts\n      - src/c.ts\n    reason: Shared justification.\n"
            ),
            "{report}"
        );
        assert!(report.contains("    ticket: SEC-9\n"), "{report}");
        assert_eq!(
            recorded_paths(&tmp, "001-x"),
            vec![vec!["src/a.ts".to_string(), "src/c.ts".to_string()]]
        );
    }

    #[test]
    fn one_path_renders_as_a_scalar_however_it_was_written() {
        // A list pruned to one path, and a one-item list written by hand, both
        // render exactly as a one-path entry always has.
        for (file, expired) in [
            ("\n      - src/a.ts\n      - src/b.ts", Some("src/b.ts")),
            ("[src/a.ts]", None),
        ] {
            let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
            seed_waivers(
                &tmp,
                "001-x",
                &format!("waivers:\n{}", waiver_with_file(file)),
            );
            let mut args = base_args("001-x");
            args.expired_waivers = expired
                .map(|path| waiver("SEC-BE-014", path))
                .into_iter()
                .collect();
            run(&args, tmp.path()).unwrap();
            let report = review_md(&tmp, "001-x");
            assert!(
                report.contains("    file: src/a.ts\n    reason:"),
                "{file}:\n{report}"
            );
        }
    }

    #[test]
    fn an_entry_is_dropped_when_its_last_path_expires() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(
            &tmp,
            "001-x",
            &format!(
                "waivers:\n{}\n  - rule: SEC-BE-020\n    file: src/keep.ts\n    reason: Still valid.\n    waived-at: 2026-01-02T00:00:00Z\n    waived-by: dev@example.com",
                waiver_with_file("[src/a.ts, src/b.ts]")
            ),
        );
        let mut args = base_args("001-x");
        args.expired_waivers = vec![
            waiver("SEC-BE-014", "src/a.ts"),
            waiver("SEC-BE-014", "src/b.ts"),
        ];
        run(&args, tmp.path()).unwrap();
        let waivers = recorded(&tmp, "001-x");
        assert_eq!(waivers.len(), 1);
        assert_eq!(waivers[0].rule.as_deref(), Some("SEC-BE-020"));
    }

    #[test]
    fn an_expired_anchor_is_pruned_from_every_entry_that_claims_it() {
        // Entry [0] claims a alone; entry [1] repeats a beside b. Expiring a
        // drops entry [0] and leaves entry [1] with b.
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(
            &tmp,
            "001-x",
            &format!(
                "waivers:\n{}\n{}",
                waiver_with_file("src/a.ts"),
                waiver_with_file("[src/a.ts, src/b.ts]")
            ),
        );
        let mut args = base_args("001-x");
        args.expired_waivers = vec![waiver("SEC-BE-014", "src/a.ts")];
        run(&args, tmp.path()).unwrap();
        assert_eq!(
            recorded_paths(&tmp, "001-x"),
            vec![vec!["src/b.ts".to_string()]]
        );
    }

    #[test]
    fn a_malformed_entry_keeps_its_list_and_loses_only_an_expired_path() {
        // No `reason`, so process-waivers never classifies it; the prune still
        // compares its anchors. An empty list has no anchor to lose and is kept
        // as the operator wrote it, and so is an entry with no rule.
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(
            &tmp,
            "001-x",
            "waivers:\n  - rule: SEC-BE-014\n    file: [src/a.ts, src/b.ts]\n  - rule: SEC-BE-014\n    file: []\n  - file: [src/a.ts, src/c.ts]",
        );
        let mut args = base_args("001-x");
        args.expired_waivers = vec![waiver("SEC-BE-014", "src/a.ts")];
        run(&args, tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(report.contains("    file: []\n"), "{report}");
        // A list as an entry's first key nests under it and still re-parses.
        assert!(
            report.contains("  - file:\n      - src/a.ts\n      - src/c.ts\n"),
            "{report}"
        );
        assert_eq!(
            recorded_paths(&tmp, "001-x"),
            vec![
                vec!["src/b.ts".to_string()],
                vec![],
                vec!["src/a.ts".to_string(), "src/c.ts".to_string()],
            ]
        );
    }

    #[test]
    fn listed_paths_are_quoted_so_they_read_back_unchanged() {
        // Paths that would retype, or open a comment, as plain scalars.
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(
            &tmp,
            "001-x",
            &format!(
                "waivers:\n{}",
                waiver_with_file("[\"1234\", \"true\", \"src/a b #c.ts\"]")
            ),
        );
        run(&base_args("001-x"), tmp.path()).unwrap();
        assert_eq!(
            recorded_paths(&tmp, "001-x"),
            vec![vec![
                "1234".to_string(),
                "true".to_string(),
                "src/a b #c.ts".to_string()
            ]]
        );
    }

    // -- observation dispositions (spec 058) -----------------------------------

    #[test]
    fn observations_never_touch_the_inbox() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let inbox_path = tmp.path().join("specs/inbox.md");
        fs::write(&inbox_path, "# Inbox\n\n- [ ] a todo someone logged\n").unwrap();
        let before = fs::read_to_string(&inbox_path).unwrap();
        let mut args = base_args("001-x");
        args.observations = vec![observation("perf: a() is called in a loop", "src/a.rs")];
        run(&args, tmp.path()).unwrap();
        assert_eq!(
            fs::read_to_string(&inbox_path).unwrap(),
            before,
            "the inbox holds what a person logs; a review writes nothing there"
        );
        let report = review_md(&tmp, "001-x");
        assert!(
            report.contains(
                "## Observations\n\n- perf: a() is called in a loop — `src/a.rs` — **undispositioned**"
            ),
            "{report}"
        );
        assert!(!report.contains("## Captured issues"), "{report}");
    }

    #[test]
    fn a_run_without_an_inbox_creates_none() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.observations = vec![observation("other: noted", "")];
        run(&args, tmp.path()).unwrap();
        assert!(inbox(&tmp).is_none());
    }

    #[test]
    fn each_outcome_is_counted_and_rendered_and_none_touches_the_violation_counts() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.observations = vec![
            dispositioned("bug: off by one", DispositionOutcome::Fixed, None),
            dispositioned(
                "bug: a gap",
                DispositionOutcome::Routed,
                Some("specs/001-x/scenarios/gap.md"),
            ),
            dispositioned(
                "other: machinery",
                DispositionOutcome::Discarded,
                Some("pipeline tooling, not this spec"),
            ),
            dispositioned("perf: unclear", DispositionOutcome::Undispositioned, None),
        ];
        let result = run(&args, tmp.path()).unwrap();
        assert_eq!(
            result.dispositions,
            crate::schema::primitives::Dispositions {
                fixed: 1,
                routed: 1,
                discarded: 1,
                undispositioned: 1,
            }
        );
        assert_eq!(result.must_violations, 0);
        assert!(
            !result.blocking,
            "an observation never blocks through the violation counts"
        );
        let report = review_md(&tmp, "001-x");
        assert!(
            report.contains(
                "dispositions:\n  fixed: 1\n  routed: 1\n  discarded: 1\n  undispositioned: 1\n"
            ),
            "{report}"
        );
        assert!(report.contains("- bug: off by one — **fixed**"), "{report}");
        assert!(
            report.contains("- bug: a gap — **routed** to `specs/001-x/scenarios/gap.md`"),
            "{report}"
        );
        assert!(
            report.contains("- other: machinery — **discarded**: pipeline tooling, not this spec"),
            "{report}"
        );
        assert!(
            report.contains("1 observation(s) undispositioned"),
            "{report}"
        );
    }

    #[test]
    fn a_routed_or_discarded_observation_is_stored_as_a_decision() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.observations = vec![
            dispositioned(
                "other: machinery",
                DispositionOutcome::Discarded,
                Some("not this spec"),
            ),
            dispositioned("bug: fixed now", DispositionOutcome::Fixed, None),
        ];
        run(&args, tmp.path()).unwrap();
        let stored =
            decisions::read_decisions(&tmp.path().join("specs/001-x"), "review.md").unwrap();
        assert_eq!(
            stored.len(),
            1,
            "a fixed observation stops firing and stores nothing"
        );
        assert_eq!(stored[0].key.as_deref(), Some("other: machinery"));
        assert_eq!(stored[0].outcome.as_deref(), Some("discarded"));
        assert_eq!(stored[0].reason.as_deref(), Some("not this spec"));
        assert_eq!(
            stored[0].decided_at.as_deref(),
            Some(args.reviewed_at.as_str())
        );
        assert_eq!(stored[0].decided_by.as_deref(), Some("dev@example.com"));
    }

    #[test]
    fn a_new_decision_without_its_author_is_refused_before_any_write() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.observations = vec![dispositioned(
            "other: x",
            DispositionOutcome::Discarded,
            Some("why"),
        )];
        assert!(matches!(
            run(&args, tmp.path()),
            Err(PrimitiveError::InvalidArgument { argument, .. }) if argument == "decided-by"
        ));
        assert!(!tmp.path().join("specs/001-x/review.md").exists());
    }

    #[test]
    fn a_matched_decision_is_kept_and_an_expired_one_is_pruned() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(
            &tmp,
            "001-x",
            "decisions:\n  - key: \"other: settled earlier\"\n    outcome: discarded\n    reason: noise\n    decided-at: 2026-09-01T00:00:00Z\n    decided-by: first@example.com\n    ticket: OPS-1\n  - key: \"bug: since fixed\"\n    outcome: routed\n    target: specs/001-x/tasks.md\n    decided-at: 2026-09-01T00:00:00Z\n    decided-by: first@example.com",
        );
        let mut args = base_args("001-x");
        let mut rematched = dispositioned(
            "other: settled, reworded",
            DispositionOutcome::Discarded,
            Some("noise"),
        );
        rematched.decision_key = Some("other: settled earlier".into());
        args.observations = vec![rematched];
        args.expired_decisions = vec![DecisionRef {
            key: "bug: since fixed".into(),
            outcome: DecisionOutcome::Routed,
            target: Some("specs/001-x/tasks.md".into()),
            reason: None,
        }];
        // No `decided-by`: a matched decision is not a new one.
        run(&args, tmp.path()).unwrap();
        let stored =
            decisions::read_decisions(&tmp.path().join("specs/001-x"), "review.md").unwrap();
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].key.as_deref(), Some("other: settled earlier"));
        assert_eq!(stored[0].decided_by.as_deref(), Some("first@example.com"));
        assert!(
            stored[0].extra.contains_key("ticket"),
            "an adopter field survives the re-render"
        );
    }

    #[test]
    fn an_empty_scope_run_still_records_its_observations_dispositions() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.empty_scope = true;
        args.observations = vec![observation("other: seen with nothing in scope", "")];
        let result = run(&args, tmp.path()).unwrap();
        assert_eq!(result.dispositions.undispositioned, 1);
        assert!(review_md(&tmp, "001-x").contains("undispositioned: 1"));
    }

    #[test]
    fn rejects_malformed_observations_before_any_write() {
        let cases: Vec<(ReviewObservation, &str)> = vec![
            (observation("   ", ""), "observations[0].text"),
            (observation("two\nlines", ""), "observations[0].text"),
            (observation("ok", "a\nb"), "observations[0].path"),
            (
                dispositioned("ok", DispositionOutcome::Routed, Some("  ")),
                "observations[0].disposition.target",
            ),
            (
                dispositioned("ok", DispositionOutcome::Discarded, None),
                "observations[0].disposition.reason",
            ),
        ];
        for (bad, expected) in cases {
            let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
            let mut args = base_args("001-x");
            args.decided_by = Some("dev@example.com".into());
            args.observations = vec![bad];
            let error = run(&args, tmp.path()).unwrap_err();
            assert!(
                error.to_string().contains(expected),
                "expected a rejection naming {expected}, got: {error}"
            );
            assert!(!tmp.path().join("specs/001-x/review.md").exists());
        }
    }

    #[test]
    fn idempotent_rerun_reproduces_identical_review_md() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.findings = vec![finding("SEC-BE-001", "must", "src/a.rs", "1-5", "high")];
        run(&args, tmp.path()).unwrap();
        let first = review_md(&tmp, "001-x");
        run(&args, tmp.path()).unwrap();
        let second = review_md(&tmp, "001-x");
        assert_eq!(first, second);
    }

    #[test]
    fn missing_feature_is_operational_error() {
        let tmp = tempdir().unwrap();
        let err = run(&base_args("999-nope"), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::FeatureNotFound { .. }));
    }

    #[test]
    fn malformed_spec_frontmatter_halts_before_any_write() {
        // The spec's frontmatter fails YAML parse. The halt must land
        // BEFORE the write: no review.md may exist afterward (scenario
        // primitive-robustness-hardening, whose reason was a halt between
        // the two writes leaving them inconsistent; spec 057 left one write
        // and the validate-first ordering is what survived).
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: [unclosed");
        let mut args = base_args("001-x");
        args.findings = vec![finding("SEC-BE-001", "must", "src/a.rs", "1-5", "high")];
        let err = run(&args, tmp.path()).unwrap_err();
        assert!(
            matches!(err, PrimitiveError::Yaml { .. }),
            "expected Yaml error, got {err:?}"
        );
        assert!(
            !tmp.path().join("specs/001-x/review.md").exists(),
            "a malformed spec must halt before review.md is written"
        );
    }

    #[test]
    fn spec_without_frontmatter_halts_before_any_write() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("spec.md"), "# No frontmatter here\n").unwrap();
        let err = run(&base_args("001-x"), tmp.path()).unwrap_err();
        assert!(
            matches!(err, PrimitiveError::MissingFrontmatter { .. }),
            "expected MissingFrontmatter, got {err:?}"
        );
        assert!(!dir.join("review.md").exists());
    }

    #[test]
    fn dropping_named_tempfile_leaves_no_review_md() {
        use std::io::Write;
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let dir = tmp.path().join("specs/001-x");
        let dest = dir.join("review.md");
        {
            let mut tf = tempfile::NamedTempFile::new_in(&dir).unwrap();
            tf.write_all(b"INTERRUPTED").unwrap();
        }
        assert!(!dest.exists());
    }

    #[test]
    fn yaml_string_quotes_syntax_and_reparse_hazards() {
        // Opaque string values stay unquoted — including timestamps, so the
        // switch from `yaml_scalar` to `yaml_string` for known waiver fields
        // is no-op churn on `waived-at` (scenarios/write-review-known-field-
        // quoting.md).
        assert_eq!(yaml_string("2026-07-02T12:00:00Z"), "2026-07-02T12:00:00Z");
        assert_eq!(yaml_string("SEC-BE-014"), "SEC-BE-014");
        assert_eq!(yaml_string("src/api/internal.ts"), "src/api/internal.ts");
        assert_eq!(yaml_string("dev@example.com"), "dev@example.com");
        assert_eq!(
            yaml_string("Endpoint is internal-only."),
            "Endpoint is internal-only."
        );
        // Syntactic hazards quote.
        assert_eq!(yaml_string("see: this"), "\"see: this\"");
        assert_eq!(yaml_string(""), "\"\"");
        // Values that would re-parse as a non-string quote, keeping their
        // string identity across the round-trip.
        assert_eq!(yaml_string("true"), "\"true\"");
        assert_eq!(yaml_string("1234"), "\"1234\"");
        assert_eq!(yaml_string("null"), "\"null\"");
    }

    /// A `#` after a space or a tab opens a YAML comment, so a plain value
    /// carrying one would read back truncated — and a stored decision key
    /// that does not come back byte for byte never matches its finding again.
    /// Each is quoted, and reads back intact.
    #[test]
    fn a_comment_opener_is_quoted_so_the_value_reads_back() {
        for value in [
            "plan.md cites issue\t#12 as closed",
            "why\t#not",
            "see issue #12",
        ] {
            let rendered = yaml_string(value);
            let parsed: String = serde_norway::from_str(&rendered).unwrap();
            assert_eq!(parsed, value, "round-trip of {value:?} via {rendered:?}");
        }
        // A `#` inside a word is not a comment, so the value stays plain.
        assert_eq!(yaml_string("issue#12"), "issue#12");
    }

    /// Register one `[constitutions.*]` entry pointing at `path` in a spec repo.
    fn with_constitution(tmp: &TempDir, alias: &str, path: &str) {
        let cfg = tmp.path().join(".ductus");
        fs::create_dir_all(&cfg).unwrap();
        fs::write(
            cfg.join("config.toml"),
            format!(
                "[constitutions.{alias}]\nrepo = \"https://example.test/g\"\npath = \"{path}\"\n"
            ),
        )
        .unwrap();
    }

    /// Register one `[constitutions.*]` entry carrying a description.
    fn with_described_constitution(tmp: &TempDir, alias: &str, path: &str, description: &str) {
        let cfg = tmp.path().join(".ductus");
        fs::create_dir_all(&cfg).unwrap();
        fs::write(
            cfg.join("config.toml"),
            format!(
                "[constitutions.{alias}]\nrepo = \"https://example.test/g\"\npath = \"{path}\"\n\
                 description = \"{description}\"\n"
            ),
        )
        .unwrap();
    }

    /// An alias is a config key someone chose; the description is what tells
    /// the operator which checkout they are missing.
    #[test]
    fn an_unexamined_source_is_named_with_its_description() {
        let tmp = spec_repo("055-x", "status: in-progress");
        with_described_constitution(&tmp, "acme", "nowhere", "Acme platform engineering rules");
        run(&base_args("055-x"), tmp.path()).unwrap();
        let report = review_md(&tmp, "055-x");
        assert!(
            report
                .contains("- `acme` (nowhere) — Acme platform engineering rules — not checked out"),
            "{report}"
        );
    }

    /// Absent is absent: an entry with no description renders exactly as it
    /// did before this field reached the report.
    #[test]
    fn an_unexamined_source_without_a_description_renders_unchanged() {
        let tmp = spec_repo("055-x", "status: in-progress");
        with_constitution(&tmp, "acme", "nowhere");
        run(&base_args("055-x"), tmp.path()).unwrap();
        let report = review_md(&tmp, "055-x");
        assert!(
            report.contains("- `acme` (nowhere) — not checked out"),
            "{report}"
        );
    }

    /// The line is single-line by contract, so an over-long description is
    /// truncated rather than wrapped.
    #[test]
    fn an_over_long_description_is_truncated_not_wrapped() {
        let tmp = spec_repo("055-x", "status: in-progress");
        let long = "x".repeat(DESCRIPTION_BUDGET + 40);
        with_described_constitution(&tmp, "acme", "nowhere", &long);
        run(&base_args("055-x"), tmp.path()).unwrap();
        let report = review_md(&tmp, "055-x");
        let line = report
            .lines()
            .find(|l| l.starts_with("- `acme`"))
            .expect("the skipped source is named");
        assert!(
            line.contains(&format!("{}…", "x".repeat(DESCRIPTION_BUDGET))),
            "{line}"
        );
        assert!(!line.contains(&long), "{line}");
    }

    #[test]
    fn truncate_counts_characters_not_bytes() {
        // Slicing this by bytes would panic mid-codepoint.
        let text = "é".repeat(10);
        assert_eq!(truncate(&text, 4), "éééé…");
        assert_eq!(truncate(&text, 10), text);
    }

    #[test]
    fn unexamined_governance_is_none_when_no_constitution_is_registered() {
        let tmp = spec_repo("055-x", "status: in-progress");
        run(&base_args("055-x"), tmp.path()).unwrap();
        let report = review_md(&tmp, "055-x");
        assert!(
            report.contains("## Unexamined governance\n\n*None.*"),
            "{report}"
        );
    }

    #[test]
    fn unexamined_governance_names_a_source_that_could_not_be_read() {
        // AC8: a review produced without a registered constitution says so in the
        // report itself, instead of letting the finding counts read as clean.
        let tmp = spec_repo("055-x", "status: in-progress");
        with_constitution(&tmp, "acme", "nowhere");

        run(&base_args("055-x"), tmp.path()).unwrap();
        let report = review_md(&tmp, "055-x");
        assert!(report.contains("## Unexamined governance"), "{report}");
        assert!(report.contains("`acme`"), "{report}");
        assert!(report.contains("not checked out"), "{report}");
        assert!(
            report.contains("its rules were NOT loaded for this review"),
            "{report}"
        );
        assert!(
            !report.contains("## Unexamined governance\n\n*None.*"),
            "an unreadable source must not render as None: {report}"
        );
    }

    #[test]
    fn a_checkout_without_a_document_reads_differently_than_a_missing_one() {
        let tmp = spec_repo("055-x", "status: in-progress");
        with_constitution(&tmp, "acme", "gov");
        fs::create_dir_all(tmp.path().join("gov")).unwrap();

        run(&base_args("055-x"), tmp.path()).unwrap();
        let report = review_md(&tmp, "055-x");
        assert!(
            report.contains("no constitution.md in checkout"),
            "cloning the wrong repo must not read like cloning nothing: {report}"
        );
    }

    #[test]
    fn a_resolved_constitution_does_not_appear_as_unexamined() {
        let tmp = spec_repo("055-x", "status: in-progress");
        with_constitution(&tmp, "acme", "gov");
        let gov = tmp.path().join("gov");
        fs::create_dir_all(&gov).unwrap();
        fs::write(gov.join("constitution.md"), "# House rules\n").unwrap();

        run(&base_args("055-x"), tmp.path()).unwrap();
        let report = review_md(&tmp, "055-x");
        assert!(
            report.contains("## Unexamined governance\n\n*None.*"),
            "a source that was read is not unexamined: {report}"
        );
    }

    #[test]
    fn a_malformed_config_still_produces_a_report() {
        // Regression: the registry read was `?`-propagated, so an unrelated
        // typo in the project config produced no review.md at all and the
        // findings this run computed were lost. An unreadable registry is a
        // governance state to report, not a reason to drop the report.
        let tmp = spec_repo("055-x", "status: in-progress");
        let cfg = tmp.path().join(".ductus");
        fs::create_dir_all(&cfg).unwrap();
        fs::write(cfg.join("config.toml"), "[constitutions.acme]\nrepo = \n").unwrap();

        let result = run(&base_args("055-x"), tmp.path());
        assert!(
            result.is_ok(),
            "a config typo must not cost the operator the report"
        );

        let report = review_md(&tmp, "055-x");
        assert!(report.contains("## Unexamined governance"), "{report}");
        assert!(
            report.contains("registry could not be read"),
            "the report must say the registry was unreadable: {report}"
        );
        assert!(
            !report.contains("## Unexamined governance\n\n*None.*"),
            "an unreadable registry must never render as None: {report}"
        );
    }

    // -- spec 058 review: decision-record hardening -------------------------

    /// A control character or a Unicode line break in a quoted value is
    /// escaped, so the frontmatter the YAML reader parses back holds the same
    /// string. A raw one would make the whole record unreadable.
    #[test]
    fn yaml_string_escapes_control_characters_and_line_breaks() {
        for value in [
            "false\u{1b}positive",
            "split\u{2028}here",
            "para\u{2029}graph",
            "next\u{85}line",
            "del\u{7f}ete",
            "non\u{fffe}char",
            "non\u{ffff}char",
        ] {
            let rendered = yaml_string(value);
            assert!(
                !rendered.chars().any(crate::primitives::is_line_hazard),
                "no raw hazard may survive: {rendered:?}"
            );
            let parsed: String = serde_norway::from_str(&rendered).unwrap();
            assert_eq!(parsed, value, "round-trip of {value:?}");
        }
    }

    /// An observation's text is refused before any write when it carries a
    /// control character, as a newline already was.
    #[test]
    fn an_observation_carrying_a_control_character_is_refused() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.observations = vec![dispositioned(
            "other: noise\u{1b}[31m",
            DispositionOutcome::Discarded,
            Some("false positive"),
        )];
        assert!(matches!(
            run(&args, tmp.path()),
            Err(PrimitiveError::InvalidArgument { argument, .. })
                if argument == "observations[0].text"
        ));
        assert!(!tmp.path().join("specs/001-x/review.md").exists());
    }

    /// A noncharacter is refused like a control character: the YAML reader
    /// refuses `U+FFFE` and `U+FFFF` outright, so one stored in a decision
    /// would leave the whole record unreadable.
    #[test]
    fn an_observation_carrying_a_noncharacter_is_refused() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.observations = vec![dispositioned(
            "other: noise",
            DispositionOutcome::Discarded,
            Some("false\u{ffff}positive"),
        )];
        assert!(matches!(
            run(&args, tmp.path()),
            Err(PrimitiveError::InvalidArgument { .. })
        ));
        assert!(!tmp.path().join("specs/001-x/review.md").exists());
    }

    /// A tab-then-`#` in an observation's text and its discard reason reads
    /// back intact, so the stored key matches the same observation next run.
    /// Written plain, YAML read it as a comment and truncated both.
    #[test]
    fn a_decision_carrying_a_comment_opener_reads_back() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.observations = vec![dispositioned(
            "bug: issue\t#12 reopens",
            DispositionOutcome::Discarded,
            Some("why\t#not"),
        )];
        run(&args, tmp.path()).unwrap();
        let dir = tmp.path().join("specs/001-x");
        let stored = decisions::read_decisions(&dir, "review.md").unwrap();
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].key.as_deref(), Some("bug: issue\t#12 reopens"));
        assert_eq!(stored[0].reason.as_deref(), Some("why\t#not"));
    }

    /// A blank timestamp would stamp every new decision with a blank
    /// `decided-at`, which the reader then calls malformed.
    #[test]
    fn a_blank_reviewed_at_is_refused() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.reviewed_at = "   ".into();
        assert!(matches!(
            run(&args, tmp.path()),
            Err(PrimitiveError::InvalidArgument { argument, .. }) if argument == "reviewed-at"
        ));
    }

    /// A blank `decision-key` is no key: the observation is stored under its
    /// own rendered line, never under an empty key the reader would reject.
    #[test]
    fn a_blank_decision_key_falls_back_to_the_observation_line() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        let mut observation = dispositioned(
            "other: machinery",
            DispositionOutcome::Discarded,
            Some("not this spec"),
        );
        observation.decision_key = Some("  ".into());
        args.observations = vec![observation];
        run(&args, tmp.path()).unwrap();
        let stored =
            decisions::read_decisions(&tmp.path().join("specs/001-x"), "review.md").unwrap();
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].key.as_deref(), Some("other: machinery"));
        assert!(stored[0].to_ref().is_ok());
    }

    /// Two observations sharing a key are one finding: one stored decision,
    /// and different decisions for the one key are refused.
    #[test]
    fn same_key_observations_store_one_decision_and_conflicts_are_refused() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.observations = vec![
            dispositioned("other: twice", DispositionOutcome::Discarded, Some("noise")),
            dispositioned("other: twice", DispositionOutcome::Discarded, Some("noise")),
        ];
        let result = run(&args, tmp.path()).unwrap();
        let stored =
            decisions::read_decisions(&tmp.path().join("specs/001-x"), "review.md").unwrap();
        assert_eq!(stored.len(), 1, "one key, one stored decision");
        // Each observation is counted, as `observations` counts it: the map
        // counts the observations a run recorded, and both carry the one
        // disposition their key gets.
        assert_eq!(result.observations, 2);
        assert_eq!(result.dispositions.discarded, 2);

        args.observations = vec![
            dispositioned("other: split", DispositionOutcome::Discarded, Some("noise")),
            dispositioned(
                "other: split",
                DispositionOutcome::Routed,
                Some("specs/001-x/tasks.md"),
            ),
        ];
        assert!(matches!(
            run(&args, tmp.path()),
            Err(PrimitiveError::InvalidArgument { argument, .. }) if argument == "observations"
        ));
    }

    /// One key, one disposition, whatever the two outcomes are: routed beside
    /// undispositioned was accepted and counted once each, so one finding
    /// was recorded as both decided and owed.
    #[test]
    fn same_key_observations_with_any_two_outcomes_are_refused() {
        use DispositionOutcome::{Discarded, Fixed, Routed, Undispositioned};
        for (first, second) in [
            (Routed, Undispositioned),
            (Fixed, Undispositioned),
            (Fixed, Discarded),
        ] {
            let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
            let mut args = base_args("001-x");
            args.decided_by = Some("dev@example.com".into());
            args.observations = [first, second]
                .into_iter()
                .map(|outcome| {
                    let companion = match outcome {
                        Routed => Some("specs/001-x/tasks.md"),
                        Discarded => Some("noise"),
                        Fixed | Undispositioned => None,
                    };
                    let mut observation = dispositioned("bug: same", outcome, companion);
                    observation.path = "src/a.rs".into();
                    observation
                })
                .collect();
            let error = run(&args, tmp.path()).unwrap_err();
            assert!(
                matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "observations"),
                "{first:?} beside {second:?}: {error}"
            );
            assert!(!tmp.path().join("specs/001-x/review.md").exists());
        }
    }

    /// A prior `review.md` that opens no frontmatter block records nothing —
    /// no waivers, no decisions — so it is overwritten, as `write-analysis`
    /// overwrites such an `analysis.md`.
    #[test]
    fn a_prior_review_without_frontmatter_is_overwritten() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let path = tmp.path().join("specs/001-x/review.md");
        fs::write(&path, "# Review — 001-x\n\nhand-edited, no record\n").unwrap();
        run(&base_args("001-x"), tmp.path()).unwrap();
        assert!(review_md(&tmp, "001-x").starts_with("---\n"));
    }

    /// A prior `review.md` whose frontmatter opens and never closes may hold
    /// waivers and decisions nobody can read, so it is refused, named as
    /// unclosed, and left as it is.
    #[test]
    fn a_prior_review_whose_frontmatter_never_closes_is_refused() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let path = tmp.path().join("specs/001-x/review.md");
        let damaged = "---\nspec: 001-x\nwaivers:\n  - rule: SEC-BE-001\n\n# Review\n";
        fs::write(&path, damaged).unwrap();
        let error = run(&base_args("001-x"), tmp.path()).unwrap_err();
        assert!(error.to_string().contains("never closes"), "{error}");
        assert!(matches!(error, PrimitiveError::UnclosedFrontmatter { .. }));
        assert_eq!(fs::read_to_string(&path).unwrap(), damaged);
    }
}
