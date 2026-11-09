//! `write-review` — render `specs/NNN/review.md` for `/ductus:review`. What it does is
//! `schema::descriptions::WRITE_REVIEW`.
//!
//! A cross-pass dedup breaks a severity tie by the higher confidence, and a
//! `low`-confidence finding lands in Low-confidence whatever its severity. `blocking` is
//! true exactly when `must-violations` exceeds zero, and the exit code (0 / 1) is
//! derivable from it. The empty scope is a branch of this primitive, not a prose special
//! case. It wrote a second copy into a `review:` block in `spec.md` until spec 057 left
//! the record one home.
//!
//! Defined by
//! `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md` and
//! `specs/058-findings-route-at-discovery/spec.md`.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use serde::Deserialize;

use crate::primitives::decisions::{self, RawDecision};
use crate::primitives::{
    PrimitiveError, Result, WaiverPaths, analyze_subjects, fix_rounds, read_text, rel_path,
    split_frontmatter, write_atomic,
};
use crate::schema::paths;
use crate::schema::primitives::{
    ConstitutionOutcome, DecisionOutcome, DecisionRef, DispositionOutcome, Dispositions, FixRound,
    FixRoundCheck, ReReadOutcome, RecordFreshness, ReviewFinding, ReviewObservation, RoundWaiver,
    WriteReviewArgs, WriteReviewResult,
};
use crate::schema::severity::ReviewSeverity;

/// Refuse a malformed `fix-rounds` entry before anything is written (spec
/// 078): a `result` anywhere but on a re-read, a re-read without one, a
/// `reason` anywhere but on a round not re-read, a blank or multi-line reason,
/// and a reason with no `decided-by`, since a waiver nobody can attribute is
/// not auditable.
fn validate_fix_rounds(args: &WriteReviewArgs) -> Result<()> {
    let Some(rounds) = args.fix_rounds.as_deref() else {
        return Ok(());
    };
    for (idx, round) in rounds.iter().enumerate() {
        let invalid = |field: &str, reason: &str| PrimitiveError::InvalidArgument {
            primitive: "write-review".into(),
            argument: format!("fix-rounds[{idx}].{field}"),
            reason: reason.into(),
        };
        match (round.check, round.result) {
            (FixRoundCheck::ReRead, None) => {
                return Err(invalid(
                    "result",
                    "a re-read round states what it found: `clean` or `findings`",
                ));
            }
            (FixRoundCheck::ReRead, Some(_)) | (_, None) => {}
            (_, Some(_)) => return Err(invalid("result", "only a re-read round has a result")),
        }
        let Some(reason) = round.reason.as_deref() else {
            continue;
        };
        if round.check != FixRoundCheck::NotReRead {
            return Err(invalid(
                "reason",
                "only a round that was not re-read is waived",
            ));
        }
        if reason.trim().is_empty() {
            return Err(invalid("reason", "a waiver states its reason"));
        }
        super::validate_single_line("write-review", &format!("fix-rounds[{idx}].reason"), reason)?;
        if args
            .decided_by
            .as_deref()
            .is_none_or(|who| who.trim().is_empty())
        {
            return Err(PrimitiveError::InvalidArgument {
                primitive: "write-review".into(),
                argument: "decided-by".into(),
                reason: format!("required when a fix round is waived (fix-rounds[{idx}])"),
            });
        }
    }
    Ok(())
}

/// The fix-round loop's record (spec 078), every field derived from the
/// run's `fix-rounds`, so no caller can state a count its rounds contradict.
struct LoopRecord {
    /// Every round, the four kinds together.
    rounds: usize,
    /// Rounds re-read to completion.
    re_reads: usize,
    /// Rounds the diff check verified.
    diff_checked: usize,
    /// Rounds that needed a re-read and have none.
    not_re_read: usize,
    /// At least [`fix_rounds::RE_READ_BUDGET`] re-reads found a MUST or SHOULD,
    /// and the last one did.
    budget_stop: bool,
    /// The re-reads that found a MUST or SHOULD: the budget's count, which an
    /// operator who lets the loop go on carries past the budget.
    with_findings: usize,
    /// The exempt rounds, from 1.
    exempt: Vec<usize>,
    /// The un-re-read rounds no waiver covers, from 1: each holds `done`.
    unwaived: Vec<usize>,
    /// A waiver per un-re-read round that has one.
    waivers: Vec<RoundWaiver>,
}

/// The run's loop record, or `None` when it passed no `fix-rounds`. The
/// switch is read here, so the host cannot leave its waiver out, and only when
/// the run passed its rounds: it waives rounds, and a run that stated none has
/// none for it to waive.
///
/// # Errors
///
/// [`PrimitiveError::InvalidReReadsOff`] for a malformed switch, and
/// [`PrimitiveError::Toml`] or [`PrimitiveError::Io`] when the config file
/// cannot be read.
fn loop_record_of(args: &WriteReviewArgs, repo: &Path) -> Result<Option<LoopRecord>> {
    let Some(rounds) = args.fix_rounds.as_deref() else {
        return Ok(None);
    };
    let switch = fix_rounds::re_reads_off(repo)?;
    let (_, config) = paths::resolve_config(repo);
    Ok(Some(derive_loop(
        rounds,
        switch.as_deref().map(|reason| (reason, config)),
        args.decided_by.as_deref(),
        &args.reviewed_at,
    )))
}

/// Derive the loop's record from its rounds. An un-re-read round is waived by
/// its own `reason`, attributed to `decided_by`; failing that, by the
/// `[review] re-reads-off` reason in `switch`, attributed to the config file
/// it names, so the switch waives every round the host left without one.
fn derive_loop(
    rounds: &[FixRound],
    switch: Option<(&str, &str)>,
    decided_by: Option<&str>,
    at: &str,
) -> LoopRecord {
    let count = |check: FixRoundCheck| rounds.iter().filter(|r| r.check == check).count();
    let results: Vec<ReReadOutcome> = rounds
        .iter()
        .filter(|r| r.check == FixRoundCheck::ReRead)
        .filter_map(|r| r.result)
        .collect();
    let with_findings = results
        .iter()
        .filter(|&&result| result == ReReadOutcome::Findings)
        .count();
    let numbered = |check: FixRoundCheck| -> Vec<usize> {
        (1..=rounds.len())
            .zip(rounds)
            .filter(|(_, r)| r.check == check)
            .map(|(n, _)| n)
            .collect()
    };
    let mut waivers = Vec::new();
    let mut unwaived = Vec::new();
    for (n, round) in (1..=rounds.len()).zip(rounds) {
        if round.check != FixRoundCheck::NotReRead {
            continue;
        }
        let own = round
            .reason
            .as_deref()
            .map(str::trim)
            .filter(|reason| !reason.is_empty())
            .map(|reason| (reason, decided_by.unwrap_or_default().trim()));
        match own.or(switch) {
            Some((reason, by)) => waivers.push(RoundWaiver {
                round: u32::try_from(n).unwrap_or(u32::MAX),
                reason: reason.to_string(),
                waived_by: by.to_string(),
                waived_at: at.to_string(),
            }),
            None => unwaived.push(n),
        }
    }
    LoopRecord {
        rounds: rounds.len(),
        re_reads: count(FixRoundCheck::ReRead),
        diff_checked: count(FixRoundCheck::DiffChecked),
        not_re_read: count(FixRoundCheck::NotReRead),
        budget_stop: with_findings >= fix_rounds::RE_READ_BUDGET
            && results.last() == Some(&ReReadOutcome::Findings),
        with_findings,
        exempt: numbered(FixRoundCheck::Exempt),
        unwaived,
        waivers,
    }
}

/// Render the loop's fields into the record, after `skipped-passes`.
fn render_loop_record(fm: &mut String, record: &LoopRecord) {
    let _ = writeln!(fm, "fix-rounds: {}", record.rounds);
    let _ = writeln!(fm, "re-reads: {}", record.re_reads);
    let _ = writeln!(fm, "diff-checked: {}", record.diff_checked);
    let _ = writeln!(fm, "not-re-read: {}", record.not_re_read);
    let _ = writeln!(fm, "budget-stop: {}", record.budget_stop);
    if record.waivers.is_empty() {
        return;
    }
    let _ = writeln!(fm, "round-waivers:");
    for waiver in &record.waivers {
        let _ = writeln!(fm, "  - round: {}", waiver.round);
        let _ = writeln!(fm, "    reason: {}", yaml_string(&waiver.reason));
        let _ = writeln!(fm, "    waived-by: {}", yaml_string(&waiver.waived_by));
        let _ = writeln!(fm, "    waived-at: {}", yaml_string(&waiver.waived_at));
    }
}

/// The Summary's account of the loop: the counts, each exempt round, each
/// waived one with its reason, each un-re-read round no waiver covers, and
/// whether the budget stopped the loop.
fn loop_summary(record: &LoopRecord) -> String {
    let mut out = format!(
        "{} fix round(s): {} re-read, {} diff-checked, {} exempt, {} not re-read.",
        record.rounds,
        record.re_reads,
        record.diff_checked,
        record.exempt.len(),
        record.not_re_read
    );
    if !record.exempt.is_empty() {
        let _ = write!(out, " Exempt: {}.", round_list(&record.exempt));
    }
    for waiver in &record.waivers {
        let _ = write!(
            out,
            " Round {} was not re-read, waived: {}.",
            waiver.round,
            crate::primitives::flatten_line(&waiver.reason)
        );
    }
    if !record.unwaived.is_empty() {
        let _ = write!(
            out,
            " Not re-read and not waived: {} — each holds `done`.",
            round_list(&record.unwaived)
        );
    }
    if record.budget_stop {
        let _ = write!(
            out,
            " The budget stopped the loop: {} re-reads found a MUST or SHOULD, against a budget of {}.",
            record.with_findings,
            fix_rounds::RE_READ_BUDGET
        );
    }
    out
}

/// `round 3`, or `rounds 2, 4 and 5`.
fn round_list(rounds: &[usize]) -> String {
    let names: Vec<String> = rounds.iter().map(ToString::to_string).collect();
    match names.as_slice() {
        [one] => format!("round {one}"),
        [rest @ .., last] => format!("rounds {} and {last}", rest.join(", ")),
        [] => String::new(),
    }
}

/// Refuse a violation's disposition that is fixed or waived, or a deferral
/// missing its companions (spec 076). A violation is waived with `--waive`
/// and fixed by removing it, so neither reaches this writer as an outcome.
fn validate_finding_dispositions(args: &WriteReviewArgs) -> Result<()> {
    for (idx, finding) in args.findings.iter().enumerate() {
        let at = format!("findings[{idx}].disposition");
        if matches!(
            finding.disposition.outcome,
            DispositionOutcome::Fixed | DispositionOutcome::Waived
        ) {
            return Err(PrimitiveError::InvalidArgument {
                primitive: "write-review".into(),
                argument: format!("{at}.outcome"),
                reason:
                    "a violation is deferred here, waived with --waive, and fixed by removing it"
                        .into(),
            });
        }
        for (argument, companion) in
            decisions::require_companion("write-review", &at, &finding.disposition)?
        {
            super::validate_single_line("write-review", &argument, companion)?;
        }
        if let Some(key) = finding.decision_key.as_deref() {
            super::validate_single_line(
                "write-review",
                &format!("findings[{idx}].decision-key"),
                key,
            )?;
        }
    }
    Ok(())
}

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
    validate_new_waivers(args)?;
    // An observation renders as one list item and, once decided, becomes a
    // stored decision's key, so an embedded newline would inject structure
    // into the report or the record. Every check of its arguments alone runs here, before any
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
        if observation.contract_break
            && observation.disposition.by_rule
            && observation.disposition.outcome == DispositionOutcome::Waived
        {
            return Err(invalid(
                format!("observations[{idx}].disposition.by-rule"),
                "a broken contract is fixed or deferred to the spec's own task; the rule never waives it",
            ));
        }
        for (argument, companion) in decisions::require_companion(
            "write-review",
            &format!("observations[{idx}].disposition"),
            &observation.disposition,
        )? {
            single_line(&argument, companion)?;
        }
    }
    Ok(())
}

/// Each `new-waivers` item names a rule once and its reason, on one line each, and a
/// person stands behind it.
fn validate_new_waivers(args: &WriteReviewArgs) -> Result<()> {
    for (idx, waiver) in args.new_waivers.iter().enumerate() {
        let invalid = |argument: String, reason: &str| PrimitiveError::InvalidArgument {
            primitive: "write-review".into(),
            argument,
            reason: reason.into(),
        };
        for (field, value) in [("rule", &waiver.rule), ("reason", &waiver.reason)] {
            let argument = format!("new-waivers[{idx}].{field}");
            if value.trim().is_empty() {
                return Err(invalid(argument, "a waiver names its rule and its reason"));
            }
            super::validate_single_line("write-review", &argument, value)?;
        }
        if args.new_waivers[..idx]
            .iter()
            .any(|earlier| earlier.rule == waiver.rule)
        {
            return Err(invalid(
                format!("new-waivers[{idx}].rule"),
                "one judgment per rule in a run, since its entry lists each file the rule fires at",
            ));
        }
        if args
            .decided_by
            .as_deref()
            .map(str::trim)
            .is_none_or(|who| who.is_empty() || who == decisions::RULE_DECIDER)
        {
            return Err(invalid(
                "decided-by".into(),
                "a waiver names the person who decided it",
            ));
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
///   carries a parent-directory component, or when the feature directory, a
///   link inside it, or a new deferral's target resolves outside the project.
/// - [`PrimitiveError::InvalidArgument`] when a scalar argument, an
///   observation's or finding's `decision-key`, an observation's text or
///   path, or a disposition's target, task or reason carries a line break or
///   another character that is not plain single-line text; when `reviewed-at`
///   or an observation's text is blank; when a violation's disposition is
///   fixed or waived; when a deferral names neither a target nor a work item,
///   spans its work-item ID over more than one line, or numbers a task without
///   a `tasks.md` target; when a new deferral's target or task does not exist,
///   or its task is in a `tasks.md` that is no feature's; when a waiver names
///   work or states no reason; when a finding is both deferred and matched by
///   an applied waiver, or is waived while a stored deferral for its key
///   stands outside `expired-decisions`, or when a broken contract is waived
///   while the rule's stored waiver for its key stands there (argument
///   `expired-decisions`); when a `new-waivers` item is blank or spans lines,
///   repeats a rule, has no person in `decided-by`, or its rule fires at no file
///   a waiver does not already hold and no earlier write of this run stored it; when a
///   broken contract left open names no file, nor does a `decision-key` matching a
///   stored break this run did not expire (argument
///   `observations[idx].path`); when a
///   scope that is empty, by `empty-scope` or by a window holding nothing but the run's own
///   record, would write over a record counting a finding or one it cannot read (argument
///   `empty-scope`); when two observations, or two violations, sharing a key are given different
///   outcomes; when a new decision, or a waived fix round, has no
///   `decided-by`; or when a `fix-rounds` entry carries a `result` on any
///   check but `re-read`, a re-read none, or a `reason` that is blank,
///   spans lines, or sits on any check but `not-re-read`.
/// - [`PrimitiveError::InvalidArgument`] on `feature` while the spec's `tasks.md` holds an
///   unchecked box outside a task this run's own deferral names, and on `diff-base` when no
///   scope can be derived from it (spec 079).
/// - [`PrimitiveError::InvalidReReadsOff`] when `fix-rounds` is given and
///   `[review] re-reads-off` is malformed, and [`PrimitiveError::Toml`] when
///   the config file it is read from does not parse.
/// - [`PrimitiveError::FeatureNotFound`] when the feature has no `spec.md`.
/// - [`PrimitiveError::MissingFrontmatter`] when `spec.md` has no frontmatter
///   block, and [`PrimitiveError::UnclosedFrontmatter`] when the block in
///   `spec.md` or the prior `review.md` never closes. A prior `review.md`
///   that opens no block records no waivers or decisions and is overwritten, unless the
///   scope is empty, which refuses over a record it cannot read.
/// - [`PrimitiveError::Yaml`] when `spec.md`'s frontmatter fails to parse, or
///   the prior `review.md`'s frontmatter, `waivers:`, or `decisions:` list
///   does.
/// - [`PrimitiveError::Io`] on read/write failure.
pub fn run(args: &WriteReviewArgs, repo: &Path) -> Result<WriteReviewResult> {
    let root = paths::Paths::load(repo).specs_root;
    super::validate_feature_argument(repo, &root, &args.feature)?;
    validate_arguments(args)?;
    let feature_dir = repo.join(&root).join(&args.feature);
    let spec_path = feature_dir.join("spec.md");
    if !spec_path.is_file() {
        return Err(PrimitiveError::FeatureNotFound {
            root,
            feature: args.feature.clone(),
        });
    }
    refuse_while_implementation_is_open(args, &root, &feature_dir)?;
    // The denominator `examined` is a claim against — see `resolve_scope`.
    let (scope, excluded) = resolve_scope(args, repo)?;
    refuse_an_empty_scope_over_a_finding(args, &feature_dir, scope)?;
    let (new_waivers, new_anchors) =
        new_waiver_entries(args, &surviving_waivers(&feature_dir, args)?)?;
    let settled = settled(args, new_anchors, &feature_dir)?;
    let args = settled.as_ref();
    an_open_break_names_its_file(args)?;

    // Dedup, then bucket the survivors.
    let deduped = dedup_findings(&args.findings);
    let sorted = sort_findings(&deduped, args, repo, &root)?;
    let (must, should, low, waived) = (&sorted.must, &sorted.should, &sorted.low, &sorted.waived);

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
    let mut surviving = surviving_waivers(&feature_dir, args)?;
    surviving.extend(new_waivers);
    let dispositions = count_dispositions(&args.observations);
    one_outcome_per_key(&args.observations, &deduped)?;
    let mut decisions = merged_decisions(&feature_dir, args, repo, &deduped)?;
    decisions::mark_gating(&mut decisions, &sorted.holding());
    let breaks: Vec<String> = args
        .observations
        .iter()
        .filter(|observation| observation.contract_break)
        .map(observation_key)
        .collect();
    decisions::mark_breaks(&mut decisions, &breaks);
    let loop_record = loop_record_of(args, repo)?;
    let open_findings = open_findings(
        must.iter().chain(should.iter()).map(|f| f.file.as_str()),
        &args.observations,
        (repo, &root, &args.feature),
    );
    let report = render_report(
        args,
        &Buckets {
            must,
            should,
            low,
            waived,
            deferred: &sorted.deferred,
        },
        blocking,
        &governance_section,
        scope,
        &Record {
            waivers: &surviving,
            decisions: &decisions,
            dispositions,
            fix_rounds: loop_record.as_ref(),
            excluded: &excluded,
            open_findings: &open_findings,
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

/// Refuse the record while implementation is open, but for the work this run's own deferrals
/// add to its spec (spec 079).
fn refuse_while_implementation_is_open(
    args: &WriteReviewArgs,
    root: &str,
    feature_dir: &Path,
) -> Result<()> {
    let deferred_here = decisions::tasks_deferred_here(
        root,
        &args.feature,
        args.observations
            .iter()
            .map(|observation| &observation.disposition)
            .chain(args.findings.iter().map(|finding| &finding.disposition)),
    );
    super::read_tasks::refuse_while_tasks_are_open("write-review", feature_dir, &deferred_here)
}

/// Every check of the arguments alone, before anything is read.
fn validate_arguments(args: &WriteReviewArgs) -> Result<()> {
    validate_scalar_fields(args)?;
    validate_finding_dispositions(args)?;
    validate_fix_rounds(args)
}

/// A broken contract left open names the file the next run reads again, or nothing would
/// list it and the gate would pass. Checked after `settled` has applied what is stored,
/// since a stored decision can make one a break, leave it open, or name its file.
fn an_open_break_names_its_file(args: &WriteReviewArgs) -> Result<()> {
    for (idx, observation) in args.observations.iter().enumerate() {
        if observation.contract_break
            && !matches!(
                observation.disposition.outcome,
                DispositionOutcome::Fixed | DispositionOutcome::Waived
            )
            && observation.path.trim().is_empty()
        {
            return Err(PrimitiveError::InvalidArgument {
                primitive: "write-review".into(),
                argument: format!("observations[{idx}].path"),
                reason: "a broken contract left open names the file the next run reads again"
                    .into(),
            });
        }
    }
    Ok(())
}

/// An empty scope reads nothing, so its clean record written over one that counts a
/// finding would clear that finding with its file unread (spec 079). A run's own
/// earlier write carries its timestamp, so it never refuses itself. A record with no
/// `open-findings` list, from before the field, is why this reads the counts too.
fn refuse_an_empty_scope_over_a_finding(
    args: &WriteReviewArgs,
    feature_dir: &Path,
    scope: u32,
) -> Result<()> {
    let counts_a_finding = match crate::primitives::load_review_record(feature_dir) {
        crate::primitives::RecordLoad::Absent => false,
        // Nothing can say a damaged record counts no finding, and overwriting it would
        // collapse undeterminable into a clean run.
        crate::primitives::RecordLoad::Unreadable(_) => true,
        crate::primitives::RecordLoad::Present(record) => {
            record.last_run.as_deref() != Some(args.reviewed_at.as_str())
                && (record
                    .must_violations
                    .saturating_add(record.should_violations)
                    > 0
                    || !record.open_findings.is_empty())
        }
    };
    if !counts_a_finding {
        return Ok(());
    }
    // The flag is the caller's word, as the denominator's doc comment below says, and
    // `ductus exec` never sets it, so an empty window decides too.
    if args.empty_scope || scope == 0 {
        return Err(PrimitiveError::InvalidArgument {
            primitive: "write-review".into(),
            argument: "empty-scope".into(),
            reason: "the record counts a finding that this run would clear with its file unread, \
                     or cannot be read to say it does not; review without an empty scope, naming \
                     an earlier --since if the window holds nothing, or repair the record"
                .into(),
        });
    }
    Ok(())
}

/// This run's own record, which it writes and does not read.
fn own_record_path(args: &WriteReviewArgs, repo: &Path) -> String {
    format!(
        "{}/{}/{}",
        paths::Paths::load(repo).specs_root,
        args.feature,
        super::REVIEW_RECORD_FILE
    )
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
/// base resolves to at some later moment. An empty base, what `compute-review-scope`
/// returns for a spec that never entered `in-progress`, records a scope of zero, though that
/// scope still holds a first review's plan entries. Any other base the derivation fails on refuses the write: a
/// mistyped sha once recorded `scope: 0` beside `examined: 8`, a window no
/// check could tell from an empty one, while the caller still holds its
/// findings and can write them again over the base the scope came from.
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
///
/// The paths the scope left out come with it, each with the specs that claimed
/// it, so the record names what it did not read.
fn resolve_scope(
    args: &WriteReviewArgs,
    repo: &Path,
) -> Result<(u32, BTreeMap<String, Vec<String>>)> {
    if args.diff_base.is_empty() {
        return Ok((0, BTreeMap::new()));
    }
    let scope_args = crate::schema::primitives::ComputeReviewScopeArgs {
        feature: args.feature.clone(),
        since: Some(args.diff_base.clone()),
    };
    let result =
        crate::primitives::compute_review_scope::compute(&scope_args, repo).map_err(|source| {
            PrimitiveError::InvalidArgument {
                primitive: "write-review".into(),
                argument: "diff-base".into(),
                reason: format!(
                    "no scope can be derived from it ({source}); pass the diff-base \
                 compute-review-scope returned"
                ),
            }
        })?;
    // The record is what the run writes, not what it read, so a run's two writes state
    // one denominator.
    let own_record = own_record_path(args, repo);
    let read = result.scope.iter().filter(|path| **path != own_record);
    Ok((
        u32::try_from(read.count()).unwrap_or(u32::MAX),
        result.excluded,
    ))
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

/// The record's stored decisions, read once every new deferral among the
/// observations and violations is shown to name work that exists (spec 076)
/// — before the merge, so a refusal lands before any write.
fn stored_decisions(
    feature_dir: &Path,
    args: &WriteReviewArgs,
    repo: &Path,
) -> Result<Vec<RawDecision>> {
    let stored = decisions::read_decisions(feature_dir, super::REVIEW_RECORD_FILE)?;
    for (idx, observation) in args.observations.iter().enumerate() {
        decisions::refuse_missing_work(
            repo,
            "write-review",
            &format!("observations[{idx}].disposition"),
            &observation.disposition,
            &observation_key(observation),
            &stored,
            &args.expired_decisions,
        )?;
    }
    for (idx, finding) in args.findings.iter().enumerate() {
        decisions::refuse_missing_work(
            repo,
            "write-review",
            &format!("findings[{idx}].disposition"),
            &finding.disposition,
            &violation_key(finding),
            &stored,
            &args.expired_decisions,
        )?;
    }
    let replaced = |key: &str| {
        args.expired_decisions
            .iter()
            .any(|gone| decisions::same_key(&gone.key, key))
    };
    // The rule never waives a broken contract, so its waiver from before the
    // observation was judged one disposes of nothing. A person's waiver replaces
    // it through `expired-decisions`, which `merge` reads as fresh (spec 079).
    for observation in args.observations.iter().filter(|observation| {
        observation.contract_break && observation.disposition.outcome == DispositionOutcome::Waived
    }) {
        let key = observation_key(observation);
        if !replaced(&key)
            && decisions::stored_decision(&stored, &key).is_some_and(|decision| {
                decision.by_rule && decision.outcome == DecisionOutcome::Waived
            })
        {
            return Err(PrimitiveError::InvalidArgument {
                primitive: "write-review".into(),
                argument: "expired-decisions".into(),
                reason: format!(
                    "`{key}` is a broken contract, and the rule's stored waiver does not \
                     dispose of it; pass that waiver in expired-decisions, then fix it, defer \
                     it, or record a person's waiver"
                ),
            });
        }
    }
    // A violation this run waives while its stored deferral stands would be
    // recorded twice, in `waivers:` and in `decisions:`; the deferral the
    // waiver replaces arrives in `expired-decisions` (spec 076, AC13).
    for finding in args
        .findings
        .iter()
        .filter(|finding| waiver_reason(finding, &args.applied_waivers).is_some())
    {
        let key = violation_key(finding);
        if !replaced(&key)
            && decisions::stored_decision(&stored, &key)
                .is_some_and(|decision| decision.outcome == DecisionOutcome::Deferred)
        {
            return Err(PrimitiveError::InvalidArgument {
                primitive: "write-review".into(),
                argument: "expired-decisions".into(),
                reason: format!(
                    "`{}` at `{}` is waived while its stored deferral stands; pass that \
                     deferral in expired-decisions, so the finding holds one disposition",
                    finding.rule, finding.file
                ),
            });
        }
    }
    Ok(stored)
}

/// The `decisions:` list this run records: what is stored, less what it expired, and
/// each decision it made.
fn merged_decisions(
    feature_dir: &Path,
    args: &WriteReviewArgs,
    repo: &Path,
    deduped: &[ReviewFinding],
) -> Result<Vec<RawDecision>> {
    decisions::merge(
        stored_decisions(feature_dir, args, repo)?,
        &args.expired_decisions,
        &[
            decided_observations(&args.observations),
            decided_violations(deduped),
        ]
        .concat(),
        &args.reviewed_at,
        args.decided_by.as_deref(),
    )
    .map_err(|refusal| {
        refusal.into_error(
            "write-review",
            "observations",
            "an observation or violation",
        )
    })
}

/// Refuse two observations, or two violations, that share a key but carry
/// different outcomes, naming the list the conflicting key came from: a
/// violation's key is `rule {rule-id} at {file}`, an observation's its
/// rendered line.
fn one_outcome_per_key(
    observations: &[ReviewObservation],
    deduped: &[ReviewFinding],
) -> Result<()> {
    decisions::one_outcome_per_key(
        observations
            .iter()
            .map(|observation| {
                (
                    observation_key(observation),
                    observation.disposition.outcome,
                )
            })
            .chain(
                deduped
                    .iter()
                    .map(|f| (violation_key(f), f.disposition.outcome)),
            ),
    )
    .map_err(|refusal| {
        let list = match &refusal {
            decisions::DecisionRefusal::ConflictingDecisions(key)
                if deduped
                    .iter()
                    .any(|f| decisions::same_key(&violation_key(f), key)) =>
            {
                "findings"
            }
            _ => "observations",
        };
        refusal.into_error("write-review", list, "an observation or violation")
    })
}

/// The `dispositions:` map for this run's observations. Derived, never
/// accepted: a caller that supplied the counts could report them clean.
fn count_dispositions(observations: &[ReviewObservation]) -> Dispositions {
    let mut counts = Dispositions::default();
    for observation in observations {
        let slot = match observation.disposition.outcome {
            DispositionOutcome::Fixed => &mut counts.fixed,
            DispositionOutcome::Deferred => &mut counts.deferred,
            DispositionOutcome::Waived => &mut counts.waived,
            DispositionOutcome::Undispositioned => &mut counts.undispositioned,
        };
        *slot = slot.saturating_add(1);
    }
    counts
}

/// The observations showing the spec's own code breaks its contract that the
/// run did not fix: what such a run leaves behind, which the measurement reads
/// as a gating finding (spec 079, AC2).
fn contract_breaks(observations: &[ReviewObservation]) -> usize {
    observations
        .iter()
        .filter(|observation| {
            observation.contract_break
                && observation.disposition.outcome != DispositionOutcome::Fixed
        })
        .count()
}

/// Each file the record leaves open, sorted once: every counted violation's, and each
/// contract break's that the run did not fix, a person did not waive and no deferral
/// releases, which is what holds `done` until a review clears it (spec 079).
fn open_findings<'a>(
    violations: impl Iterator<Item = &'a str>,
    observations: &'a [ReviewObservation],
    (repo, root, feature): (&Path, &str, &str),
) -> Vec<String> {
    let open_breaks = observations
        .iter()
        .filter(|observation| {
            observation.contract_break
                && !matches!(
                    observation.disposition.outcome,
                    DispositionOutcome::Fixed | DispositionOutcome::Waived
                )
                && !decisions::releases(repo, root, feature, &observation.disposition)
        })
        .map(|observation| super::path_without_lines(&observation.path));
    violations
        .map(str::trim)
        .chain(open_breaks)
        .filter(|path| !path.is_empty())
        .map(str::to_string)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// The deferred and waived observations, as the decisions they store.
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

/// The five buckets a run's findings fall into, passed as one value so the
/// renderer's signature states a shape rather than five positional slices.
struct Buckets<'a> {
    must: &'a [&'a ReviewFinding],
    should: &'a [&'a ReviewFinding],
    low: &'a [&'a ReviewFinding],
    waived: &'a [&'a ReviewFinding],
    deferred: &'a [&'a ReviewFinding],
}

/// The deduped findings sorted into the report's sections. `deferred` holds
/// the findings their deferral releases ([`decisions::releases`]), which
/// leave this spec's counts (spec 076).
struct Sorted<'a> {
    must: Vec<&'a ReviewFinding>,
    should: Vec<&'a ReviewFinding>,
    low: Vec<&'a ReviewFinding>,
    waived: Vec<&'a ReviewFinding>,
    deferred: Vec<&'a ReviewFinding>,
}

impl Sorted<'_> {
    /// The keys of the counted MUST and SHOULD violations that carry a
    /// deferral — the deferrals that hold `done` rather than release it.
    fn holding(&self) -> Vec<String> {
        self.must
            .iter()
            .chain(&self.should)
            .filter(|finding| finding.disposition.outcome == DispositionOutcome::Deferred)
            .map(|finding| violation_key(finding))
            .collect()
    }
}

/// Bucket the deduped findings. A waived finding drops out of the counts into
/// Waived findings; a finding its deferral releases drops out into Deferred
/// findings; a `low`-confidence one lands in Low-confidence regardless of
/// severity; the rest split MUST / SHOULD.
///
/// # Errors
///
/// [`PrimitiveError::InvalidArgument`] naming `findings` when one finding is
/// both deferred and matched by an applied waiver: it holds one disposition.
fn sort_findings<'a>(
    deduped: &'a [ReviewFinding],
    args: &WriteReviewArgs,
    repo: &Path,
    root: &str,
) -> Result<Sorted<'a>> {
    let mut sorted = Sorted {
        must: Vec::new(),
        should: Vec::new(),
        low: Vec::new(),
        waived: Vec::new(),
        deferred: Vec::new(),
    };
    for finding in deduped {
        let waived = waiver_reason(finding, &args.applied_waivers).is_some();
        let deferred = finding.disposition.outcome == DispositionOutcome::Deferred;
        if waived && deferred {
            return Err(PrimitiveError::InvalidArgument {
                primitive: "write-review".into(),
                argument: "findings".into(),
                reason: format!(
                    "`{}` at `{}` is both deferred and waived; a finding holds one disposition",
                    finding.rule, finding.file
                ),
            });
        }
        if waived {
            sorted.waived.push(finding);
        } else if deferred && decisions::releases(repo, root, &args.feature, &finding.disposition) {
            sorted.deferred.push(finding);
        } else if finding.confidence.eq_ignore_ascii_case("low") {
            sorted.low.push(finding);
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
                ReviewSeverity::Must => sorted.must.push(finding),
                ReviewSeverity::Should => sorted.should.push(finding),
            }
        }
    }
    Ok(sorted)
}

/// The decisions the deferred violations store, keyed by [`violation_key`].
fn decided_violations(findings: &[ReviewFinding]) -> Vec<DecisionRef> {
    findings
        .iter()
        .filter_map(|finding| {
            decisions::decision_for(&finding.disposition, &violation_key(finding))
        })
        .collect()
}

/// The stored-decision key for a violation: the stored decision the host
/// matched it to, when it supplied one, else `rule {rule-id} at {file}` — one
/// entry per file the rule fires at, the anchor a waiver keeps (spec 076).
fn violation_key(finding: &ReviewFinding) -> String {
    decisions::nonblank(finding.decision_key.as_deref()).map_or_else(
        || format!("rule {} at {}", finding.rule.trim(), finding.file.trim()),
        str::to_string,
    )
}

/// The record's lists and derived counts, rendered into the frontmatter.
struct Record<'a> {
    waivers: &'a [RawWaiverFull],
    decisions: &'a [RawDecision],
    dispositions: Dispositions,
    /// The fix-round loop's fields; `None` when the run passed no
    /// `fix-rounds`, which writes none of them.
    fix_rounds: Option<&'a LoopRecord>,
    /// Each path the scope left out, with the specs that claimed it.
    excluded: &'a BTreeMap<String, Vec<String>>,
    /// Each file the record leaves open; see [`open_findings`].
    open_findings: &'a [String],
}

/// The `open-findings` list, absent when the record leaves nothing open.
fn render_open_findings(fm: &mut String, open_findings: &[String]) {
    if !open_findings.is_empty() {
        let _ = writeln!(fm, "open-findings:");
        for path in open_findings {
            let _ = writeln!(fm, "  - {}", yaml_string(path));
        }
    }
}

/// Render the full `review.md` document (frontmatter + fixed skeleton).
fn render_report(
    args: &WriteReviewArgs,
    buckets: &Buckets<'_>,
    blocking: bool,
    governance_section: &str,
    scope: u32,
    record: &Record<'_>,
    contracts: &crate::primitives::analyze_subjects::SubjectDigest,
) -> String {
    let (must, should, low, waived, deferred) = (
        buckets.must,
        buckets.should,
        buckets.low,
        buckets.waived,
        buckets.deferred,
    );
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
    let _ = writeln!(fm, "excluded: {}", record.excluded.len());
    let _ = writeln!(
        fm,
        "contract-breaks: {}",
        contract_breaks(&args.observations)
    );
    render_open_findings(&mut fm, record.open_findings);
    let _ = writeln!(fm, "skipped-passes: [{}]", args.skipped_passes.join(", "));
    if args.loop_open {
        let _ = writeln!(fm, "fix-rounds-open: true");
    }
    if let Some(loop_record) = record.fix_rounds {
        render_loop_record(&mut fm, loop_record);
    }
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

    let summary = report_summary(args, buckets, blocking, record);

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
            "## Deferred findings\n\n{}",
            render_findings(deferred, "DEFERRED", &args.applied_waivers)
        ),
        format!(
            "## Observations\n\n{}",
            render_observations(&args.observations, record.decisions)
        ),
        format!(
            "## Skipped passes\n\n{}",
            render_skipped(&args.skipped_passes)
        ),
        format!(
            "## Excluded from scope\n\n{}",
            render_excluded(
                record.excluded,
                args.fix_rounds.as_deref().unwrap_or_default()
            )
        ),
        format!("## Unexamined governance\n\n{governance_section}"),
    ];

    format!("{fm}\n\n{}\n", sections.join("\n\n"))
}

/// The report's Summary: the caller's, when it supplied one, else the
/// generated count line. Either is followed by the loop's account when the run
/// passed its rounds, so the record names each exempt and each waived round
/// whoever wrote the Summary (spec 078 AC2; 066's scenario
/// `a-supplied-summary-carries-the-loops-account`): after a space on the
/// generated line, and as a paragraph of its own after a supplied Summary,
/// whose last line may be a code fence, a table row or a list item that text
/// joined to it would break. A supplied Summary is otherwise rendered exactly
/// as given.
fn report_summary(
    args: &WriteReviewArgs,
    buckets: &Buckets<'_>,
    blocking: bool,
    record: &Record<'_>,
) -> String {
    if let Some(summary) = args.summary.as_deref().filter(|s| !s.trim().is_empty()) {
        return match record.fix_rounds {
            Some(loop_record) => format!("{}\n\n{}", summary.trim_end(), loop_summary(loop_record)),
            None => summary.to_string(),
        };
    }
    let mut summary = generate_summary(
        args,
        buckets.must.len(),
        buckets.should.len(),
        buckets.low.len(),
        (buckets.waived.len(), buckets.deferred.len()),
        blocking,
        record.dispositions.undispositioned,
    );
    if let Some(loop_record) = record.fix_rounds {
        summary.push(' ');
        summary.push_str(&loop_summary(loop_record));
    }
    if !record.excluded.is_empty() {
        let _ = write!(
            summary,
            " {} path(s) only another spec's commits changed are excluded from scope.",
            record.excluded.len()
        );
    }
    summary
}

/// A deterministic one-line Summary derived from the counts.
fn generate_summary(
    args: &WriteReviewArgs,
    must: usize,
    should: usize,
    low: usize,
    (waived, deferred): (usize, usize),
    blocking: bool,
    undispositioned: u32,
) -> String {
    if args.empty_scope {
        let held = if contract_breaks(&args.observations) > 0 {
            " A broken contract among its observations holds `done` unless fixed, waived by a \
             person or released by a deferral."
        } else {
            ""
        };
        return format!(
            "Review scope is empty — no file changed in this review's window. \
             Zero findings across all passes; blocking: no.{held}"
        );
    }
    let mut summary = format!(
        "{must} MUST violation(s), {should} SHOULD violation(s), {low} low-confidence finding(s)"
    );
    if waived > 0 {
        let _ = write!(summary, ", {waived} waived");
    }
    // A released violation leaves the counts, so without this a run whose
    // only violation its deferral released reads exactly like a clean one
    // (spec 076).
    if deferred > 0 {
        let _ = write!(summary, ", {deferred} released by deferral");
    }
    let _ = write!(
        summary,
        ". blocking: {}.",
        if blocking { "yes" } else { "no" }
    );
    if undispositioned > 0 {
        let _ = write!(
            summary,
            " {undispositioned} observation(s) undispositioned — recorded, and only a broken contract among them holds `done`."
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
    let disposition = &finding.disposition;
    if disposition.outcome == DispositionOutcome::Deferred {
        let _ = writeln!(
            out,
            "- **Deferred**: to {}",
            decisions::work_reference(
                decisions::nonblank(disposition.target.as_deref()),
                decisions::nonblank(disposition.task.as_deref()),
                decisions::nonblank(disposition.work_item.as_deref()),
                // The ID is stored as given, but the body shows it flattened
                // like every other piece of caller text, so a control
                // character never reaches the report raw.
                crate::primitives::flatten_line,
            )
        );
    }
    out.trim_end().to_string()
}

/// Render the observations as a list, or `*None.*` when empty, each beside
/// its disposition so the report says what was done with it, not only that it
/// was seen.
fn render_observations(observations: &[ReviewObservation], stored: &[RawDecision]) -> String {
    if observations.is_empty() {
        return "*None.*".to_string();
    }
    // A security issue or a leak is surfaced first (§brownfield-inbox, Finding
    // dispositions). The sort is stable, so the rest keep the run's order.
    let mut ordered: Vec<&ReviewObservation> = observations.iter().collect();
    ordered.sort_by_key(|observation| !is_security_first(&observation.text));
    ordered
        .into_iter()
        .map(|observation| {
            // Flattened like every other piece of caller text, so a
            // work-item ID, stored as given, never reaches the body raw.
            let standing = standing_rule_waiver(observation, stored);
            let outcome = decisions::disposition_suffix(
                standing.as_ref().unwrap_or(&observation.disposition),
                crate::primitives::flatten_line,
            );
            let mark = if observation.contract_break {
                " (breaks the spec's contract)"
            } else {
                ""
            };
            format!("- {}{mark} — {outcome}", observation_line(observation))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// The rule's stored waiver, when this observation is waived again. A waiver agreeing
/// with the rule's makes no new decision, so the record keeps the rule's reason and
/// label, and the body says what the record holds rather than contradict it (spec 079).
fn standing_rule_waiver(
    observation: &ReviewObservation,
    stored: &[RawDecision],
) -> Option<crate::schema::primitives::Disposition> {
    if observation.disposition.outcome != DispositionOutcome::Waived {
        return None;
    }
    let decision = decisions::stored_decision(stored, &observation_key(observation))?;
    decision
        .by_rule
        .then(|| crate::schema::primitives::Disposition {
            outcome: DispositionOutcome::Waived,
            reason: decision.reason,
            by_rule: true,
            ..crate::schema::primitives::Disposition::default()
        })
}

/// Whether an observation leads with the category a reader must meet first.
fn is_security_first(text: &str) -> bool {
    let lead = text.trim_start().to_ascii_lowercase();
    lead.starts_with("security") || lead.starts_with("leak")
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
        &crate::schema::primitives::ResolveConstitutionsArgs::default(),
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

/// Each path the scope left out, by the specs that claimed it, and each path a
/// fix round left out, by the round.
fn render_excluded(excluded: &BTreeMap<String, Vec<String>>, rounds: &[FixRound]) -> String {
    let flat = crate::primitives::flatten_line;
    let from_scope = excluded
        .iter()
        .map(|(path, claimants)| format!("- `{}` — {}", flat(path), claimants.join(", ")));
    let from_rounds = (1..=rounds.len()).zip(rounds).flat_map(|(n, round)| {
        round
            .excluded
            .iter()
            .map(move |path| format!("- `{}` — left out of fix round {n}", flat(path)))
    });
    let lines: Vec<String> = from_scope.chain(from_rounds).collect();
    if lines.is_empty() {
        return "*None.*".to_string();
    }
    lines.join("\n")
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

/// What this run's `new-waivers` add: the `waivers:` entries to store, each listing the
/// files its rule fires at in this run that no waiver holds, and every anchor to apply. A
/// run writes its record twice with one set of arguments, so an anchor its earlier write
/// stored applies and is not stored again, and a rule its fix has since cleared asks
/// nothing of this write (spec 079).
///
/// # Errors
///
/// [`PrimitiveError::InvalidArgument`] when a rule fires at no such file and no earlier
/// write of this run stored it.
fn new_waiver_entries(
    args: &WriteReviewArgs,
    surviving: &[RawWaiverFull],
) -> Result<(
    Vec<RawWaiverFull>,
    Vec<crate::schema::primitives::WaiverRef>,
)> {
    let mut entries = Vec::new();
    let mut anchors = Vec::new();
    for (idx, waiver) in args.new_waivers.iter().enumerate() {
        let held = |file: &str| {
            surviving.iter().any(|entry| {
                entry.rule.as_deref() == Some(waiver.rule.as_str())
                    && entry
                        .file
                        .as_ref()
                        .is_some_and(|paths| paths.as_slice().iter().any(|path| path == file))
            })
        };
        let firing: std::collections::BTreeSet<&str> = args
            .findings
            .iter()
            .filter(|finding| {
                finding.rule == waiver.rule
                    && waiver_reason(finding, &args.applied_waivers).is_none()
            })
            .map(|finding| finding.file.as_str())
            .collect();
        let stored_by_this_run = surviving.iter().any(|entry| {
            entry.rule.as_deref() == Some(waiver.rule.as_str())
                && entry.waived_at.as_deref() == Some(args.reviewed_at.as_str())
        });
        if firing.is_empty() && !stored_by_this_run {
            return Err(PrimitiveError::InvalidArgument {
                primitive: "write-review".into(),
                argument: format!("new-waivers[{idx}]"),
                reason: format!(
                    "`{}` fires at no file in this run that a waiver does not already hold",
                    waiver.rule
                ),
            });
        }
        anchors.extend(
            firing
                .iter()
                .map(|file| crate::schema::primitives::WaiverRef {
                    rule: waiver.rule.clone(),
                    file: (*file).to_string(),
                    reason: waiver.reason.clone(),
                }),
        );
        let fresh: Vec<String> = firing
            .iter()
            .filter(|file| !held(file))
            .map(|file| (*file).to_string())
            .collect();
        if !fresh.is_empty() {
            entries.push(RawWaiverFull {
                rule: Some(waiver.rule.clone()),
                file: Some(WaiverPaths::of(fresh)),
                reason: Some(waiver.reason.clone()),
                waived_at: Some(args.reviewed_at.clone()),
                waived_by: args.decided_by.clone(),
                extra: std::collections::BTreeMap::new(),
            });
        }
    }
    Ok((entries, anchors))
}

/// The arguments as this run records them: each new waiver's anchors applied, so its
/// findings leave the counts as an applied waiver's do; an observation the rule waived
/// carrying the decision of the person it cannot replace; and an observation whose stored
/// decision says it is a broken contract staying one, since the hold must not depend on a
/// host marking it again (spec 079).
fn settled<'a>(
    args: &'a WriteReviewArgs,
    new_anchors: Vec<crate::schema::primitives::WaiverRef>,
    feature_dir: &Path,
) -> Result<std::borrow::Cow<'a, WriteReviewArgs>> {
    let stored = decisions::read_decisions(feature_dir, super::REVIEW_RECORD_FILE)?;
    let overruled: Vec<(usize, crate::schema::primitives::Disposition)> = args
        .observations
        .iter()
        .enumerate()
        .filter_map(|(idx, observation)| {
            standing_decision(observation, &stored, &args.expired_decisions)
                .map(|decision| (idx, decision))
        })
        .collect();
    let still_broken: Vec<usize> = args
        .observations
        .iter()
        .enumerate()
        .filter(|(_, observation)| {
            let key = observation_key(observation);
            !args
                .expired_decisions
                .iter()
                .any(|gone| decisions::same_key(&gone.key, &key))
                && decisions::stored_as_a_break(&stored, &key)
        })
        .map(|(idx, _)| idx)
        .collect();
    if new_anchors.is_empty() && overruled.is_empty() && still_broken.is_empty() {
        return Ok(std::borrow::Cow::Borrowed(args));
    }
    let mut args = args.clone();
    args.applied_waivers.extend(new_anchors);
    for (idx, decision) in overruled {
        args.observations[idx].disposition = decision;
    }
    for idx in still_broken {
        let observation = &mut args.observations[idx];
        observation.contract_break = true;
        // The matched key names the file the break was listed under, so the list does not
        // depend on a host passing the path again, marked or not.
        if observation.path.trim().is_empty()
            && let Some(file) = observation
                .decision_key
                .as_deref()
                .and_then(decisions::key_anchor)
        {
            observation.path = file.to_string();
        }
    }
    Ok(std::borrow::Cow::Owned(args))
}

/// The stored decision, when the rule waives this observation again: the rule never
/// replaces a person's unless this run expired it, so the count and the body follow it
/// rather than contradict what the record stores (spec 079).
fn standing_decision(
    observation: &ReviewObservation,
    stored: &[RawDecision],
    expired: &[DecisionRef],
) -> Option<crate::schema::primitives::Disposition> {
    let disposition = &observation.disposition;
    if !disposition.by_rule || disposition.outcome != DispositionOutcome::Waived {
        return None;
    }
    let key = observation_key(observation);
    if expired
        .iter()
        .any(|gone| decisions::same_key(&gone.key, &key))
    {
        return None;
    }
    let decision = decisions::stored_decision(stored, &key)?;
    Some(crate::schema::primitives::Disposition {
        outcome: match decision.outcome {
            DecisionOutcome::Deferred => DispositionOutcome::Deferred,
            DecisionOutcome::Waived => DispositionOutcome::Waived,
        },
        target: decision.target,
        task: decision.task,
        work_item: decision.work_item,
        reason: decision.reason,
        by_rule: decision.by_rule,
    })
}

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
    /// Rendered `key: true` when set and omitted when not — a derived flag
    /// whose absence is its false.
    Flag(bool),
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
            EntryValue::Absent | EntryValue::Flag(false) => {}
            EntryValue::Flag(true) => {
                let _ = writeln!(block, "{}{key}: true", indent());
            }
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
/// adopter-authored fields (`co-waived-by`, `approved-by-team`, …) are captured by
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
            disposition: crate::schema::primitives::Disposition::default(),
            decision_key: None,
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
        let (dir, base) = spec_repo_with_a_change("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(
            &dir,
            "001-x",
            "waivers:\n  - rule: SEC-001\n    file: src/a.rs\n    reason: Justified.\n    ticket: OPS-42",
        );
        let mut args = base_args("001-x");
        args.diff_base = base;
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

    /// A spec with a committed transition, then a commit another spec claims
    /// and one this spec claims. Returns the transition's parent.
    fn repo_with_another_specs_commit(tmp: &TempDir) -> String {
        use crate::primitives::git_fixture::{commit_all, write};
        let repository = git2::Repository::init(tmp.path()).unwrap();
        let spec = |status: &str| format!("---\nstatus: {status}\ndependencies: []\n---\n\n# X\n");
        write(&tmp.path().join("specs/001-x/spec.md"), &spec("planned"));
        write(&tmp.path().join("specs/002-y/spec.md"), &spec("planned"));
        let parent = commit_all(&repository, "chore: seed");
        write(
            &tmp.path().join("specs/001-x/spec.md"),
            &spec("in-progress"),
        );
        commit_all(&repository, "status(001): start");
        write(&tmp.path().join("src/other.rs"), "fn o() {}\n");
        commit_all(&repository, "feat(002): its work");
        write(&tmp.path().join("src/mine.rs"), "fn m() {}\n");
        commit_all(&repository, "feat(001): this spec's work");
        parent.to_string()
    }

    #[test]
    fn the_record_names_each_excluded_path_by_the_spec_that_claimed_it() {
        let tmp = tempdir().unwrap();
        let parent = repo_with_another_specs_commit(&tmp);
        let mut args = base_args("001-x");
        args.diff_base = parent;
        let out = run(&args, tmp.path()).unwrap();

        // The transition's spec.md and this spec's one file: the other's left.
        assert_eq!(out.scope, 2);
        let report = review_md(&tmp, "001-x");
        assert!(report.contains("\nscope: 2\n"), "{report}");
        assert!(report.contains("\nexcluded: 1\n"), "{report}");
        assert!(
            report.contains("## Excluded from scope\n\n- `src/other.rs` — 002-y\n"),
            "{report}"
        );
        assert!(
            report.contains("1 path(s) only another spec's commits changed"),
            "{report}"
        );
    }

    #[test]
    fn a_record_that_excluded_nothing_says_so() {
        let dir = spec_repo("001-x", "status: in-progress\ndependencies: []");
        run(&base_args("001-x"), dir.path()).unwrap();
        let report = review_md(&dir, "001-x");
        assert!(report.contains("\nexcluded: 0\n"), "{report}");
        assert!(
            report.contains("## Excluded from scope\n\n*None.*\n"),
            "{report}"
        );
        assert!(!report.contains("excluded from scope."), "{report}");
    }

    /// A run's own record is what it writes, not what it read, so a run's two writes
    /// state one denominator and a committed earlier record is no file in it (spec 079).
    #[test]
    fn a_runs_own_record_is_not_in_its_denominator() {
        let tmp = tempdir().unwrap();
        let repo = git2::Repository::init(tmp.path()).unwrap();
        let dir = tmp.path().join("specs/001-x");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# x\n",
        )
        .unwrap();
        fs::create_dir_all(tmp.path().join("src")).unwrap();
        fs::write(tmp.path().join("src/a.rs"), "a\n").unwrap();
        let commit = |message: &str| {
            let mut index = repo.index().unwrap();
            index
                .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
                .unwrap();
            index.write().unwrap();
            let tree = repo.find_tree(index.write_tree().unwrap()).unwrap();
            let sig = git2::Signature::now("t", "t@example.com").unwrap();
            let parent = repo.head().ok().and_then(|h| h.peel_to_commit().ok());
            let parents: Vec<&git2::Commit<'_>> = parent.iter().collect();
            repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)
                .unwrap()
                .to_string()
        };
        let base = commit("seed");
        fs::write(tmp.path().join("src/a.rs"), "a2\n").unwrap();
        commit("feat(001): code");
        let mut args = base_args("001-x");
        args.diff_base = base;
        run(&args, tmp.path()).unwrap();
        let first = review_md(&tmp, "001-x");
        assert!(first.contains("\nscope: 1\n"), "{first}");
        run(&args, tmp.path()).unwrap();
        let second = review_md(&tmp, "001-x");
        assert!(second.contains("\nscope: 1\n"), "{second}");
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

    fn deferred(
        mut finding: ReviewFinding,
        target: Option<&str>,
        work_item: Option<&str>,
    ) -> ReviewFinding {
        finding.disposition = crate::schema::primitives::Disposition {
            outcome: DispositionOutcome::Deferred,
            target: target.map(Into::into),
            work_item: work_item.map(Into::into),
            ..crate::schema::primitives::Disposition::default()
        };
        finding
    }

    /// Spec 076: a violation its deferral releases — here, to an in-progress
    /// spec — leaves this spec's counts for Deferred findings; one deferred
    /// to its own spec's task, or to a work item alone, stays counted and
    /// marks its decision gating.
    #[test]
    fn a_deferral_releases_a_violation_only_to_another_spec() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let other = tmp.path().join("specs/002-y");
        std::fs::create_dir_all(other.join("scenarios")).unwrap();
        std::fs::write(
            other.join("spec.md"),
            "---\nstatus: in-progress\n---\n# 002\n",
        )
        .unwrap();
        std::fs::write(other.join("scenarios/fix.md"), "# Fix\n").unwrap();
        std::fs::write(
            tmp.path().join("specs/001-x/tasks.md"),
            "# Tasks\n\n## 1. Fix\n\n- [ ] a\n\n- **Done when**: a.\n",
        )
        .unwrap();
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        let mut own = deferred(
            finding("SEC-1", "must", "src/a.rs", "1-2", "high"),
            Some("specs/001-x/tasks.md"),
            None,
        );
        own.disposition.task = Some("1".into());
        args.findings = vec![
            deferred(
                finding("SEC-2", "must", "src/b.rs", "1-2", "high"),
                Some("specs/002-y/scenarios/fix.md"),
                None,
            ),
            own,
            deferred(
                finding("SEC-3", "should", "src/c.rs", "1-2", "high"),
                None,
                Some("PROJ-9"),
            ),
        ];
        let result = run(&args, tmp.path()).unwrap();
        assert_eq!(
            result.must_violations, 1,
            "only the own-spec deferral is counted"
        );
        assert_eq!(result.should_violations, 1);
        assert!(result.blocking);
        let report = review_md(&tmp, "001-x");
        assert!(report.contains("### DEFERRED: SEC-2"), "{report}");
        assert!(
            report.contains("- **Deferred**: to `specs/001-x/tasks.md` task 1"),
            "{report}"
        );
        assert!(
            report.contains("- **Deferred**: to work item `PROJ-9`"),
            "{report}"
        );
        let stored =
            decisions::read_decisions(&tmp.path().join("specs/001-x"), "review.md").unwrap();
        let gating = |key: &str| {
            stored
                .iter()
                .find(|d| d.key.as_deref() == Some(key))
                .map(|d| d.gating)
        };
        assert_eq!(gating("rule SEC-2 at src/b.rs"), Some(None));
        assert_eq!(gating("rule SEC-1 at src/a.rs"), Some(Some(true)));
        assert_eq!(gating("rule SEC-3 at src/c.rs"), Some(Some(true)));
    }

    const STANDING_DEFERRAL: &str = "decisions:\n  - key: \"rule SEC-1 at src/a.rs\"\n    \
         outcome: deferred\n    work-item: PROJ-1\n    gating: true\n    \
         decided-at: 2026-09-01T00:00:00Z\n    decided-by: first@example.com";

    /// Spec 076, AC4 and AC11: re-deferring a violation whose stored
    /// deferral stands, to other work, is a new deferral. Its work is checked
    /// against the tree, and the record then names the work the counts follow
    /// rather than keeping the old entry.
    #[test]
    fn a_re_deferral_to_other_work_is_checked_and_stored() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(&tmp, "001-x", STANDING_DEFERRAL);
        let other = tmp.path().join("specs/002-y");
        std::fs::create_dir_all(&other).unwrap();
        std::fs::write(
            other.join("spec.md"),
            "---\nstatus: in-progress\n---\n# 002\n",
        )
        .unwrap();
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        let mut missing = deferred(
            finding("SEC-1", "must", "src/a.rs", "1-2", "high"),
            Some("specs/002-y/tasks.md"),
            None,
        );
        missing.disposition.task = Some("4".into());
        args.findings = vec![missing];
        let err = run(&args, tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidArgument { argument, .. }
                if argument == "findings[0].disposition.target"),
            "{err:?}"
        );

        args.findings = vec![deferred(
            finding("SEC-1", "must", "src/a.rs", "1-2", "high"),
            Some("specs/002-y/spec.md"),
            None,
        )];
        let result = run(&args, tmp.path()).unwrap();
        assert_eq!(result.must_violations, 0);
        let stored =
            decisions::read_decisions(&tmp.path().join("specs/001-x"), "review.md").unwrap();
        let entry = stored
            .iter()
            .find(|d| d.key.as_deref() == Some("rule SEC-1 at src/a.rs"))
            .unwrap();
        assert_eq!(entry.target.as_deref(), Some("specs/002-y/spec.md"));
        assert_eq!(entry.work_item, None);
        assert_eq!(entry.gating, None);
        // A released violation leaves the counts, so the Summary says so.
        let report = std::fs::read_to_string(tmp.path().join("specs/001-x/review.md")).unwrap();
        assert!(report.contains(", 1 released by deferral"), "{report}");
    }

    /// Spec 076: a work-item ID is stored as given, but the report shows it
    /// flattened like every other piece of caller text, so a control character
    /// never reaches `review.md`'s body raw.
    #[test]
    fn a_work_item_is_flattened_in_the_report_and_kept_in_the_record() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.findings = vec![deferred(
            finding("SEC-1", "must", "src/a.rs", "1-2", "high"),
            None,
            Some("PROJ-1\u{1b}[31m"),
        )];
        let mut observed = dispositioned("perf: noise", DispositionOutcome::Deferred, None);
        observed.disposition.target = None;
        observed.disposition.work_item = Some("PROJ-2\u{1b}[31m".into());
        args.observations = vec![observed];
        run(&args, tmp.path()).unwrap();
        let report = std::fs::read_to_string(tmp.path().join("specs/001-x/review.md")).unwrap();
        let body = report.splitn(3, "---\n").nth(2).unwrap();
        assert!(!body.contains('\u{1b}'), "{body}");
        assert!(
            body.contains("- **Deferred**: to work item `PROJ-1 [31m`"),
            "{body}"
        );
        assert!(
            body.contains("**deferred** to work item `PROJ-2 [31m`"),
            "{body}"
        );
        let stored =
            decisions::read_decisions(&tmp.path().join("specs/001-x"), "review.md").unwrap();
        let item = |key: &str| {
            stored
                .iter()
                .find(|d| d.key.as_deref() == Some(key))
                .and_then(|d| d.work_item.clone())
        };
        assert_eq!(
            item("rule SEC-1 at src/a.rs").as_deref(),
            Some("PROJ-1\u{1b}[31m")
        );
        assert_eq!(item("perf: noise").as_deref(), Some("PROJ-2\u{1b}[31m"));
    }

    /// Spec 076, AC13: a violation waived while its stored deferral stands
    /// is refused unless that deferral arrives in `expired-decisions`, so the
    /// record never holds a waiver and a deferral for one finding.
    #[test]
    fn a_waiver_replaces_a_standing_deferral_only_through_expired_decisions() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(&tmp, "001-x", STANDING_DEFERRAL);
        let mut args = base_args("001-x");
        args.findings = vec![finding("SEC-1", "must", "src/a.rs", "1-2", "high")];
        args.applied_waivers = vec![waiver("SEC-1", "src/a.rs")];
        let err = run(&args, tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidArgument { argument, .. }
                if argument == "expired-decisions"),
            "{err:?}"
        );
        args.expired_decisions = vec![DecisionRef {
            key: "rule SEC-1 at src/a.rs".into(),
            outcome: DecisionOutcome::Deferred,
            target: None,
            task: None,
            work_item: Some("PROJ-1".into()),
            reason: None,
            by_rule: false,
        }];
        run(&args, tmp.path()).unwrap();
        let stored =
            decisions::read_decisions(&tmp.path().join("specs/001-x"), "review.md").unwrap();
        assert!(
            stored
                .iter()
                .all(|d| d.key.as_deref() != Some("rule SEC-1 at src/a.rs")),
            "{stored:?}"
        );
    }

    /// Spec 076: two violations of one rule at one file share a key, so
    /// deferring one and not the other is refused — naming `findings`, the
    /// list the key came from, not `observations`.
    #[test]
    fn conflicting_violation_dispositions_name_the_findings_list() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.findings = vec![
            deferred(
                finding("SEC-1", "must", "src/a.rs", "1-2", "high"),
                None,
                Some("PROJ-1"),
            ),
            finding("SEC-1", "must", "src/a.rs", "40-41", "high"),
        ];
        let err = run(&args, tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidArgument { argument, .. } if argument == "findings"),
            "{err:?}"
        );
    }

    /// Spec 076: a finding holds one disposition, so one both deferred and
    /// waived is refused, and a violation's disposition is only a deferral.
    #[test]
    fn a_violation_is_deferred_or_waived_never_both() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.findings = vec![deferred(
            finding("SEC-BE-014", "must", "src/internal.ts", "1-2", "high"),
            None,
            Some("PROJ-1"),
        )];
        args.applied_waivers = vec![waiver("SEC-BE-014", "src/internal.ts")];
        let err = run(&args, tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidArgument { argument, .. } if argument == "findings"),
            "{err:?}"
        );
        let mut waived = finding("SEC-1", "must", "src/a.rs", "1-2", "high");
        waived.disposition.outcome = DispositionOutcome::Waived;
        waived.disposition.reason = Some("noise".into());
        args.applied_waivers = vec![];
        args.findings = vec![waived];
        let err = run(&args, tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidArgument { argument, .. }
                if argument == "findings[0].disposition.outcome"),
            "{err:?}"
        );
    }

    fn base_args(feature: &str) -> WriteReviewArgs {
        WriteReviewArgs {
            feature: feature.into(),
            reviewed_at: "2026-07-02T12:00:00Z".into(),
            reviewed_against: "abc1234".into(),
            diff_base: String::new(),
            scenario: None,
            empty_scope: false,
            summary: None,
            skipped_passes: Vec::new(),
            findings: Vec::new(),
            applied_waivers: Vec::new(),
            expired_waivers: Vec::new(),
            new_waivers: Vec::new(),
            observations: Vec::new(),
            expired_decisions: Vec::new(),
            decided_by: None,
            examined: None,
            fix_rounds: None,
            loop_open: false,
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
                target: (outcome == DispositionOutcome::Deferred)
                    .then(|| companion.unwrap_or_default().to_string()),
                task: None,
                work_item: None,
                reason: (outcome == DispositionOutcome::Waived)
                    .then(|| companion.unwrap_or_default().to_string()),
                by_rule: false,
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

    /// `spec_repo`'s spec committed, with one file changed since the returned base, so
    /// the run's window holds something and its base names a commit.
    fn spec_repo_with_a_change(feature: &str, frontmatter: &str) -> (TempDir, String) {
        use crate::primitives::git_fixture::{commit_all, write};
        let tmp = spec_repo(feature, frontmatter);
        let repository = git2::Repository::init(tmp.path()).unwrap();
        let base = commit_all(&repository, "chore: seed").to_string();
        write(&tmp.path().join("src/changed.rs"), "fn changed() {}\n");
        (tmp, base)
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
        // the denominator. It used to short-circuit `resolve_scope`, so
        // a caller that reached this branch by misreading an over-cap
        // `compute-review-scope` error as "nothing in scope" recorded
        // `scope: 0` beside a zero `examined` — a record byte-identical to an
        // honest empty review, and invisible to Family 31 (both `examined`
        // arms guard on `total > 0`), to Family 19 (the digest is valid) and
        // to `check-review-gate` (it is non-blocking).
        //
        // Deriving unconditionally is what makes the two distinguishable: the
        // window holds one changed file, so a flag that decided the number
        // would record 0, and the derivation records 1.
        let (tmp, base) = spec_repo_with_a_change("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.diff_base = base;
        args.empty_scope = true;
        let result = run(&args, tmp.path()).unwrap();
        assert_eq!(result.exit_code, 0);
        assert_eq!(result.scope, 1);
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
        let (tmp, base) = spec_repo_with_a_change("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(&tmp, "001-x", "must-violations: 9");
        let mut args = base_args("001-x");
        args.diff_base = base;
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
        // A new deferral names work that exists (spec 076).
        let scenarios = tmp.path().join("specs/001-x/scenarios");
        std::fs::create_dir_all(&scenarios).unwrap();
        std::fs::write(scenarios.join("gap.md"), "# Gap\n").unwrap();
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.observations = vec![
            dispositioned("bug: off by one", DispositionOutcome::Fixed, None),
            dispositioned(
                "bug: a gap",
                DispositionOutcome::Deferred,
                Some("specs/001-x/scenarios/gap.md"),
            ),
            dispositioned(
                "other: machinery",
                DispositionOutcome::Waived,
                Some("pipeline tooling, not this spec"),
            ),
            dispositioned("perf: unclear", DispositionOutcome::Undispositioned, None),
        ];
        let result = run(&args, tmp.path()).unwrap();
        assert_eq!(
            result.dispositions,
            crate::schema::primitives::Dispositions {
                fixed: 1,
                deferred: 1,
                waived: 1,
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
                "dispositions:\n  fixed: 1\n  deferred: 1\n  waived: 1\n  undispositioned: 1\n"
            ),
            "{report}"
        );
        assert!(report.contains("- bug: off by one — **fixed**"), "{report}");
        assert!(
            report.contains("- bug: a gap — **deferred** to `specs/001-x/scenarios/gap.md`"),
            "{report}"
        );
        assert!(
            report.contains("- other: machinery — **waived**: pipeline tooling, not this spec"),
            "{report}"
        );
        assert!(
            report.contains("1 observation(s) undispositioned"),
            "{report}"
        );
    }

    #[test]
    fn a_deferred_or_waived_observation_is_stored_as_a_decision() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.observations = vec![
            dispositioned(
                "other: machinery",
                DispositionOutcome::Waived,
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
        assert_eq!(stored[0].outcome.as_deref(), Some("waived"));
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
            DispositionOutcome::Waived,
            Some("why"),
        )];
        assert!(matches!(
            run(&args, tmp.path()),
            Err(PrimitiveError::InvalidArgument { argument, .. }) if argument == "decided-by"
        ));
        assert!(!tmp.path().join("specs/001-x/review.md").exists());
    }

    // -- the fix-round loop (spec 078) ---------------------------------------

    fn round(
        check: FixRoundCheck,
        result: Option<ReReadOutcome>,
        reason: Option<&str>,
    ) -> FixRound {
        FixRound {
            check,
            result,
            reason: reason.map(str::to_string),
            excluded: Vec::new(),
        }
    }

    fn rule_waived(text: &str) -> ReviewObservation {
        let mut waived = dispositioned(text, DispositionOutcome::Waived, Some("out of scope"));
        waived.disposition.by_rule = true;
        waived
    }

    /// With no operator asked, the rule's waiver records itself as the rule's,
    /// and needs no `decided-by`.
    #[test]
    fn a_by_rule_waiver_records_decided_by_rule() {
        let dir = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = None;
        args.observations = vec![rule_waived("perf: a loop")];
        run(&args, dir.path()).unwrap();
        let report = review_md(&dir, "001-x");
        assert!(report.contains("decided-by: rule"), "{report}");
        assert!(report.contains("(decided by the rule)"), "{report}");
    }

    #[test]
    fn a_persons_waiver_records_the_person() {
        let dir = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.observations = vec![dispositioned(
            "perf: a loop",
            DispositionOutcome::Waived,
            Some("out of scope"),
        )];
        run(&args, dir.path()).unwrap();
        let report = review_md(&dir, "001-x");
        assert!(report.contains("decided-by: dev@example.com"), "{report}");
        assert!(!report.contains("decided by the rule"), "{report}");
    }

    /// A person's standing decision wins: the next run's rule-decided waiver of
    /// the same observation changes nothing in the record.
    #[test]
    fn a_stored_decision_by_a_person_wins_over_the_rule() {
        let dir = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.observations = vec![dispositioned(
            "perf: a loop",
            DispositionOutcome::Deferred,
            Some("specs/001-x/spec.md"),
        )];
        run(&args, dir.path()).unwrap();

        let mut next = base_args("001-x");
        next.decided_by = None;
        let mut again = dispositioned(
            "perf: a loop",
            DispositionOutcome::Waived,
            Some("the rule's reason"),
        );
        again.disposition.by_rule = true;
        next.observations = vec![again];
        run(&next, dir.path()).unwrap();
        let report = review_md(&dir, "001-x");
        assert!(report.contains("decided-by: dev@example.com"), "{report}");
        assert!(!report.contains("decided-by: rule"), "{report}");
        assert!(report.contains("outcome: deferred"), "{report}");
        // The count and the body say what the record holds rather than contradict it.
        assert!(report.contains("  deferred: 1\n  waived: 0\n"), "{report}");
        assert!(!report.contains("decided by the rule"), "{report}");
        assert!(!report.contains("the rule's reason"), "{report}");

        // Unless this run expired it: then the rule decides afresh.
        next.expired_decisions = vec![DecisionRef {
            key: "perf: a loop".into(),
            outcome: DecisionOutcome::Deferred,
            target: Some("specs/001-x/spec.md".into()),
            task: None,
            work_item: None,
            reason: None,
            by_rule: false,
        }];
        run(&next, dir.path()).unwrap();
        let report = review_md(&dir, "001-x");
        assert!(report.contains("decided-by: rule"), "{report}");
        assert!(report.contains("  deferred: 0\n  waived: 1\n"), "{report}");
    }

    /// A broken contract left for later counts, and one the run fixed left
    /// nothing, so it does not. An ordinary observation never counts.
    #[test]
    fn a_contract_break_the_run_did_not_fix_counts() {
        let dir = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        let mut left = dispositioned(
            "bug: a contract broken",
            DispositionOutcome::Undispositioned,
            None,
        );
        left.contract_break = true;
        left.path = "src/a.rs".into();
        let mut fixed = dispositioned("bug: another", DispositionOutcome::Fixed, None);
        fixed.contract_break = true;
        args.observations = vec![
            left,
            fixed,
            dispositioned("perf: a loop", DispositionOutcome::Undispositioned, None),
        ];
        run(&args, dir.path()).unwrap();
        let report = review_md(&dir, "001-x");
        assert!(report.contains("\ncontract-breaks: 1\n"), "{report}");
        assert!(
            report.contains("bug: a contract broken — `src/a.rs` (breaks the spec's contract)"),
            "{report}"
        );
    }

    /// The frontmatter list the record leaves, read back as the gate reads it.
    fn listed_open(tmp: &TempDir, feature: &str) -> Vec<String> {
        match crate::primitives::load_review_record(&tmp.path().join("specs").join(feature)) {
            crate::primitives::RecordLoad::Present(record) => record.open_findings,
            other => panic!("{other:?}"),
        }
    }

    /// A contract break with its file, in the disposition given.
    fn broken(
        path: &str,
        outcome: DispositionOutcome,
        companion: Option<&str>,
    ) -> ReviewObservation {
        let mut observation = dispositioned(&format!("bug: {path}"), outcome, companion);
        observation.path = path.into();
        observation.contract_break = true;
        observation
    }

    fn must_at(rule: &str, file: &str) -> ReviewFinding {
        serde_json::from_value(serde_json::json!({
            "rule": rule, "severity": "must", "file": file, "line-range": "1", "confidence": "high",
        }))
        .unwrap()
    }

    fn waive(rule: &str, reason: &str) -> crate::schema::primitives::NewWaiver {
        crate::schema::primitives::NewWaiver {
            rule: rule.into(),
            reason: reason.into(),
        }
    }

    /// `--waive`'s write route: the entry lists each file the rule fires at this run,
    /// stamped from the run, and the next run's `process-waivers` reads it, so the
    /// finding stays out of the count rather than coming back (spec 079).
    #[test]
    fn a_new_waiver_is_stored_and_the_next_run_applies_it() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.findings = vec![must_at("SEC-1", "src/b.rs"), must_at("SEC-1", "src/a.rs")];
        args.new_waivers = vec![waive("SEC-1", "both are internal")];
        let result = run(&args, tmp.path()).unwrap();
        assert_eq!(result.must_violations, 0);
        assert_eq!(result.waived, 2);
        assert!(!result.blocking);
        let report = review_md(&tmp, "001-x");
        assert!(
            report.contains(
                "waivers:\n  - rule: SEC-1\n    file:\n      - src/a.rs\n      - src/b.rs\n    reason: both are internal\n    waived-at: 2026-07-02T12:00:00Z\n    waived-by: dev@example.com\n"
            ),
            "{report}"
        );
        assert!(listed_open(&tmp, "001-x").is_empty(), "{report}");
        for path in ["src/a.rs", "src/b.rs"] {
            let file = tmp.path().join(path);
            fs::create_dir_all(file.parent().unwrap()).unwrap();
            fs::write(file, "x\n").unwrap();
        }
        let next = crate::primitives::process_waivers::run(
            &crate::schema::primitives::ProcessWaiversArgs {
                feature: "001-x".into(),
                fired: vec![
                    crate::schema::primitives::FiredFinding {
                        rule: "SEC-1".into(),
                        file: "src/a.rs".into(),
                    },
                    crate::schema::primitives::FiredFinding {
                        rule: "SEC-1".into(),
                        file: "src/b.rs".into(),
                    },
                ],
                ..Default::default()
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(next.applied.len(), 2, "{next:?}");
    }

    /// A file a stored waiver already holds is not listed again, since only the first
    /// claim on a `(rule, file)` applies.
    #[test]
    fn a_new_waiver_lists_only_what_no_applied_waiver_holds() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.findings = vec![must_at("SEC-1", "src/a.rs"), must_at("SEC-1", "src/b.rs")];
        args.applied_waivers = vec![WaiverRef {
            rule: "SEC-1".into(),
            file: "src/a.rs".into(),
            reason: "earlier".into(),
        }];
        args.new_waivers = vec![waive("SEC-1", "this one too")];
        let result = run(&args, tmp.path()).unwrap();
        assert_eq!(result.must_violations, 0);
        let report = review_md(&tmp, "001-x");
        assert!(
            report.contains("  - rule: SEC-1\n    file: src/b.rs\n    reason: this one too\n"),
            "{report}"
        );
    }

    /// Each refused `--waive` writes nothing: a waiver with nothing to waive, or nobody
    /// to attribute it to, is the operator's to correct.
    #[test]
    fn a_new_waiver_with_nothing_to_waive_or_nobody_behind_it_is_refused() {
        let cases: Vec<(&str, Option<&str>, &str, &str, &str)> = vec![
            (
                "fires nowhere",
                Some("dev@example.com"),
                "SEC-2",
                "why",
                "new-waivers[0]",
            ),
            (
                "no reason",
                Some("dev@example.com"),
                "SEC-1",
                "  ",
                "new-waivers[0].reason",
            ),
            (
                "no rule",
                Some("dev@example.com"),
                " ",
                "why",
                "new-waivers[0].rule",
            ),
            (
                "two lines",
                Some("dev@example.com"),
                "SEC-1",
                "a\nb",
                "new-waivers[0].reason",
            ),
            ("nobody", None, "SEC-1", "why", "decided-by"),
            ("the rule", Some("rule"), "SEC-1", "why", "decided-by"),
        ];
        for (name, by, rule, reason, argument) in cases {
            let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
            let mut args = base_args("001-x");
            args.decided_by = by.map(str::to_string);
            args.findings = vec![must_at("SEC-1", "src/a.rs")];
            args.new_waivers = vec![waive(rule, reason)];
            let error = run(&args, tmp.path()).unwrap_err();
            assert!(
                matches!(&error, PrimitiveError::InvalidArgument { argument: named, .. } if named == argument),
                "{name}: {error}"
            );
            assert!(!tmp.path().join("specs/001-x/review.md").exists(), "{name}");
        }

        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.findings = vec![must_at("SEC-1", "src/a.rs")];
        args.new_waivers = vec![waive("SEC-1", "why"), waive("SEC-1", "again")];
        let error = run(&args, tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "new-waivers[1].rule"),
            "{error}"
        );
    }

    /// A run writes its record twice with one set of arguments, so its second write
    /// stores nothing new, and the finding stays waived (spec 079).
    #[test]
    fn a_new_waiver_through_both_writes_is_stored_once() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.findings = vec![must_at("SEC-1", "src/a.rs")];
        args.new_waivers = vec![waive("SEC-1", "internal")];
        args.loop_open = true;
        run(&args, tmp.path()).unwrap();
        args.loop_open = false;
        let result = run(&args, tmp.path()).unwrap();
        assert_eq!(result.must_violations, 0);
        let report = review_md(&tmp, "001-x");
        assert_eq!(report.matches("rule: SEC-1").count(), 1, "{report}");
    }

    /// A fix between a run's writes that clears the finding asks nothing of the last,
    /// or a run could not finish; a later run's has no such standing.
    #[test]
    fn a_new_waiver_whose_finding_a_fix_cleared_lets_the_last_write_finish() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.findings = vec![must_at("SEC-1", "src/a.rs")];
        args.new_waivers = vec![waive("SEC-1", "internal")];
        run(&args, tmp.path()).unwrap();
        args.findings = Vec::new();
        run(&args, tmp.path()).unwrap();
        assert_eq!(review_md(&tmp, "001-x").matches("rule: SEC-1").count(), 1);

        args.reviewed_at = "2026-07-03T12:00:00Z".into();
        let error = run(&args, tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "new-waivers[0]"),
            "{error}"
        );
    }

    /// A stored decision remembers it is a broken contract's, so a run that matches it
    /// keeps the hold without a host marking it again (spec 079).
    #[test]
    fn a_stored_break_stays_one_when_a_later_run_matches_it() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut first = base_args("001-x");
        first.decided_by = Some("dev@example.com".into());
        let mut deferred = broken("src/a.rs", DispositionOutcome::Deferred, None);
        deferred.disposition.work_item = Some("JIRA-1".into());
        deferred.disposition.target = None;
        first.observations = vec![deferred.clone()];
        run(&first, tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(report.contains("contract-break: true"), "{report}");
        assert_eq!(listed_open(&tmp, "001-x"), vec!["src/a.rs"]);

        let mut next = base_args("001-x");
        let mut matched = deferred;
        matched.contract_break = false;
        next.observations = vec![matched.clone()];
        run(&next, tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(report.contains("\ncontract-breaks: 1\n"), "{report}");
        assert_eq!(listed_open(&tmp, "001-x"), vec!["src/a.rs"]);

        // Expired, the decision stands for nothing, and neither does its mark.
        let mut after = base_args("001-x");
        after.decided_by = Some("dev@example.com".into());
        after.expired_decisions = vec![DecisionRef {
            key: "bug: src/a.rs — `src/a.rs`".into(),
            outcome: DecisionOutcome::Deferred,
            target: None,
            task: None,
            work_item: Some("JIRA-1".into()),
            reason: None,
            by_rule: false,
        }];
        after.observations = vec![matched];
        run(&after, tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(report.contains("\ncontract-breaks: 0\n"), "{report}");
        assert!(listed_open(&tmp, "001-x").is_empty(), "{report}");
    }

    /// A later run matching a stored break by its key, with no path, still lists the file
    /// that key names, or the gate would pass an open break.
    #[test]
    fn a_stored_break_matched_with_no_path_lists_the_file_its_key_names() {
        for marked in [false, true] {
            a_stored_break_matched_with_no_path(marked);
        }
    }

    fn a_stored_break_matched_with_no_path(marked: bool) {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut first = base_args("001-x");
        first.decided_by = Some("dev@example.com".into());
        let mut deferred = broken("src/a.rs", DispositionOutcome::Deferred, None);
        deferred.disposition.work_item = Some("JIRA-1".into());
        deferred.disposition.target = None;
        first.observations = vec![deferred.clone()];
        run(&first, tmp.path()).unwrap();

        let mut next = base_args("001-x");
        let mut matched = deferred;
        matched.decision_key = Some("bug: src/a.rs — `src/a.rs`".into());
        matched.path = String::new();
        matched.contract_break = marked;
        next.observations = vec![matched];
        run(&next, tmp.path()).unwrap();
        assert_eq!(listed_open(&tmp, "001-x"), vec!["src/a.rs"], "{marked}");
    }

    /// Only a matched key names the file: a stored key passed as the observation's text,
    /// with no path, refuses rather than storing a second key with the path doubled.
    #[test]
    fn a_stored_key_passed_as_text_fills_no_path() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut first = base_args("001-x");
        first.decided_by = Some("dev@example.com".into());
        let mut deferred = broken("src/a.rs", DispositionOutcome::Deferred, None);
        deferred.disposition.work_item = Some("JIRA-1".into());
        deferred.disposition.target = None;
        first.observations = vec![deferred.clone()];
        run(&first, tmp.path()).unwrap();

        let mut next = base_args("001-x");
        next.decided_by = Some("dev@example.com".into());
        let mut matched = deferred;
        matched.text = "bug: src/a.rs — `src/a.rs`".into();
        matched.path = String::new();
        matched.contract_break = false;
        next.observations = vec![matched];
        let error = run(&next, tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "observations[0].path"),
            "{error}"
        );
    }

    /// A stored decision can leave a break open with no file anywhere, which refuses as a
    /// host's own open break with no path does.
    #[test]
    fn a_stored_break_left_open_with_no_file_refuses() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut first = base_args("001-x");
        first.decided_by = Some("dev@example.com".into());
        let mut waived = broken(
            "src/a.rs",
            DispositionOutcome::Waived,
            Some("not this spec"),
        );
        waived.path = String::new();
        first.observations = vec![waived.clone()];
        run(&first, tmp.path()).unwrap();

        let mut next = base_args("001-x");
        next.decided_by = Some("dev@example.com".into());
        let mut reopened = broken("src/a.rs", DispositionOutcome::Deferred, None);
        reopened.path = String::new();
        reopened.contract_break = false;
        reopened.disposition.work_item = Some("JIRA-1".into());
        reopened.disposition.target = None;
        next.observations = vec![reopened];
        let error = run(&next, tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "observations[0].path"),
            "{error}"
        );
    }

    /// A `:line:column` reads as the file, in a list and through a key alike.
    #[test]
    fn a_line_and_column_leave_the_file_a_break_names() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.observations = vec![broken(
            "src/b.rs:3:9",
            DispositionOutcome::Undispositioned,
            None,
        )];
        run(&args, tmp.path()).unwrap();
        assert_eq!(listed_open(&tmp, "001-x"), vec!["src/b.rs"]);
        assert_eq!(
            decisions::key_anchor("bug: a thing — `src/b.rs:3:9`"),
            Some("src/b.rs")
        );
    }

    /// With a record counting a MUST and listing nothing, as one from before the list does,
    /// a window holding nothing refuses whether or not the caller said so, as `ductus exec`
    /// never does, and one holding a file does not.
    #[test]
    fn an_empty_window_refuses_over_a_counted_finding_without_the_flag() {
        let tmp = tempdir().unwrap();
        repo_with_another_specs_commit(&tmp);
        let repository = git2::Repository::open(tmp.path()).unwrap();
        fs::write(
            tmp.path().join("specs/001-x/review.md"),
            "---\nspec: 001-x\nlast-run: 2026-07-01T00:00:00Z\nmust-violations: 1\nblocking: true\n---\n\n# Review\n",
        )
        .unwrap();
        crate::primitives::git_fixture::commit_all(&repository, "review(001): the run");
        let head = repository
            .head()
            .unwrap()
            .peel_to_commit()
            .unwrap()
            .id()
            .to_string();

        let mut empty = base_args("001-x");
        empty.diff_base = head;
        let before = review_md(&tmp, "001-x");
        let error = run(&empty, tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "empty-scope"),
            "{error}"
        );
        assert_eq!(review_md(&tmp, "001-x"), before);

        std::fs::write(tmp.path().join("src/mine.rs"), "changed\n").unwrap();
        run(&empty, tmp.path()).unwrap();
    }

    /// A record the last run left uncommitted is what this run writes over, not a file it
    /// read, so it keeps no window from holding nothing.
    #[test]
    fn an_uncommitted_record_keeps_no_window_from_being_empty() {
        let tmp = tempdir().unwrap();
        repo_with_another_specs_commit(&tmp);
        let head = git2::Repository::open(tmp.path())
            .unwrap()
            .head()
            .unwrap()
            .peel_to_commit()
            .unwrap()
            .id()
            .to_string();
        fs::write(
            tmp.path().join("specs/001-x/review.md"),
            "---\nspec: 001-x\nlast-run: 2026-07-01T00:00:00Z\nmust-violations: 1\nblocking: true\n---\n\n# Review\n",
        )
        .unwrap();
        let mut empty = base_args("001-x");
        empty.diff_base = head;
        let error = run(&empty, tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "empty-scope"),
            "{error}"
        );
    }

    /// A base no scope can be derived from refuses the write, naming it, and leaves the
    /// record as it was: a mistyped sha once recorded `scope: 0` beside `examined: 8`
    /// (spec 079, task 20).
    #[test]
    fn a_base_that_names_no_commit_refuses_the_write() {
        let (tmp, _) = spec_repo_with_a_change("001-x", "status: in-progress\ndependencies: []");
        let path = tmp.path().join("specs/001-x/review.md");
        let prior = "---\nspec: 001-x\nlast-run: 2026-07-01T00:00:00Z\nmust-violations: 0\nblocking: false\n---\n\n# Review\n";
        fs::write(&path, prior).unwrap();
        let mut args = base_args("001-x");
        args.diff_base = "0123456789abcdef0123456789abcdef01234567".into();
        args.examined = Some(8);
        let error = run(&args, tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "diff-base"),
            "{error:?}"
        );
        assert_eq!(fs::read_to_string(&path).unwrap(), prior);
    }

    /// An empty base, what `compute-review-scope` returns for a spec that never entered
    /// `in-progress`, records a scope of zero, so it is an empty one and refuses over a record
    /// counting a finding.
    #[test]
    fn an_empty_base_is_an_empty_scope() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let path = tmp.path().join("specs/001-x/review.md");
        fs::write(&path, "---\nspec: 001-x\nlast-run: 2026-07-01T00:00:00Z\nmust-violations: 1\nblocking: true\n---\n\n# Review\n").unwrap();
        let error = run(&base_args("001-x"), tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "empty-scope"),
            "{error:?}"
        );
        fs::remove_file(&path).unwrap();
        let out = run(&base_args("001-x"), tmp.path()).unwrap();
        assert_eq!(out.scope, 0);
    }

    /// Nothing can say a damaged record counts no finding, so an empty scope does not
    /// write over it as a clean run.
    #[test]
    fn an_empty_scope_over_an_unreadable_record_refuses() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let file = tmp.path().join("specs/001-x/review.md");
        std::fs::write(
            &file,
            "---\nspec: 001-x\nlast-run: 2026-01-01T00:00:00Z\nmust-violations: many\nblocking: true\n---\n\n# Review\n",
        )
        .unwrap();
        let mut args = base_args("001-x");
        args.empty_scope = true;
        let error = run(&args, tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "empty-scope"),
            "{error}"
        );
    }

    /// A review records once implementation is complete, so a box still open in
    /// `tasks.md`, under any heading, refuses the write and leaves the record as it was.
    #[test]
    fn a_review_records_only_once_every_task_is_done() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let tasks = tmp.path().join("specs/001-x/tasks.md");
        std::fs::write(&tasks, "# T\n\n## Notes\n\n- [ ] still owed\n").unwrap();
        let error = run(&base_args("001-x"), tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "feature"),
            "{error}"
        );
        assert!(!tmp.path().join("specs/001-x/review.md").exists());
        std::fs::write(&tasks, "# T\n\n## Notes\n\n- [x] done\n").unwrap();
        run(&base_args("001-x"), tmp.path()).unwrap();
    }

    /// The run's own deferral opens the box it records, so that box does not refuse the
    /// write, while a box under any other task still does.
    #[test]
    fn a_runs_own_deferral_opens_no_refusal_but_other_work_does() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let tasks = tmp.path().join("specs/001-x/tasks.md");
        let deferred = "# T\n\n## 1. A\n\n- [x] a\n\n- **Done when**: a.\n\n## 2. Fix it\n\n- [ ] fix\n\n- **Done when**: fixed.\n";
        std::fs::write(&tasks, deferred).unwrap();
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        let mut own = broken("src/a.rs", DispositionOutcome::Deferred, None);
        own.disposition.target = Some("specs/001-x/tasks.md".into());
        own.disposition.task = Some("2".into());
        args.observations = vec![own];
        run(&args, tmp.path()).unwrap();

        std::fs::write(
            &tasks,
            format!("{deferred}\n## 3. Other\n\n- [ ] other\n\n- **Done when**: done.\n"),
        )
        .unwrap();
        let error = run(&args, tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, reason, .. } if argument == "feature" && reason.contains("other") && !reason.contains("fix")),
            "{error}"
        );
    }

    /// An empty run's summary says what its record holds.
    #[test]
    fn an_empty_scope_summary_names_a_break_it_holds() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.empty_scope = true;
        args.observations = vec![broken(
            "src/a.rs",
            DispositionOutcome::Undispositioned,
            None,
        )];
        run(&args, tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(
            report.contains("A broken contract among its observations"),
            "{report}"
        );
    }

    /// The refusal over a rule's stored waiver, with the record left as it was.
    fn refused_over_the_rules_waiver(args: &WriteReviewArgs, tmp: &TempDir, before: &str) {
        let error = run(args, tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, reason, .. }
                if argument == "expired-decisions" && reason.contains("does not dispose")),
            "{error}"
        );
        assert_eq!(review_md(tmp, "001-x"), before);
    }

    /// The rule's stored waiver, then the same observation judged a broken contract.
    /// Carrying that waiver is refused, since nobody decided it, and only a person's
    /// waiver with the rule's in `expired-decisions` closes it (spec 079, AC2).
    #[test]
    fn a_rules_stored_waiver_does_not_dispose_of_a_broken_contract() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut first = base_args("001-x");
        first.decided_by = None;
        let mut waived = rule_waived("bug: src/a.rs");
        waived.path = "src/a.rs".into();
        first.observations = vec![waived];
        run(&first, tmp.path()).unwrap();
        let before = review_md(&tmp, "001-x");
        assert!(before.contains("decided-by: rule"), "{before}");
        let key = "bug: src/a.rs — `src/a.rs`";

        let mut carried = base_args("001-x");
        carried.decided_by = Some("dev@example.com".into());
        carried.observations = vec![broken(
            "src/a.rs",
            DispositionOutcome::Waived,
            Some("the rule says so"),
        )];
        refused_over_the_rules_waiver(&carried, &tmp, &before);

        carried.expired_decisions = vec![DecisionRef {
            key: key.into(),
            outcome: DecisionOutcome::Waived,
            target: None,
            task: None,
            work_item: None,
            reason: Some("the rule says so".into()),
            by_rule: true,
        }];
        run(&carried, tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(report.contains("decided-by: dev@example.com"), "{report}");
        assert!(!report.contains("decided-by: rule"), "{report}");
        assert!(report.contains("\ncontract-breaks: 1\n"), "{report}");
        assert!(listed_open(&tmp, "001-x").is_empty(), "{report}");
    }

    /// What the refusal leaves alone: a person's stored waiver closes the break with
    /// nothing expired, and a break fixed or left open over the rule's waiver is not
    /// disposed of by it.
    #[test]
    fn only_a_waived_break_over_the_rules_waiver_is_refused() {
        let person = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut first = base_args("001-x");
        first.decided_by = Some("dev@example.com".into());
        first.observations = vec![broken(
            "src/a.rs",
            DispositionOutcome::Waived,
            Some("a person's reason"),
        )];
        run(&first, person.path()).unwrap();
        run(&first, person.path()).unwrap();
        let report = review_md(&person, "001-x");
        assert!(report.contains("decided-by: dev@example.com"), "{report}");

        for outcome in [
            DispositionOutcome::Fixed,
            DispositionOutcome::Undispositioned,
        ] {
            let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
            let mut first = base_args("001-x");
            first.decided_by = None;
            let mut waived = rule_waived("bug: src/a.rs");
            waived.path = "src/a.rs".into();
            first.observations = vec![waived];
            run(&first, tmp.path()).unwrap();
            let mut next = base_args("001-x");
            next.decided_by = None;
            next.observations = vec![broken("src/a.rs", outcome, None)];
            run(&next, tmp.path()).unwrap();
        }
    }

    /// Counted violations and open breaks are listed once each, without a line range;
    /// what a run fixed or a person waived, and a deferral that releases, are not.
    #[test]
    fn the_record_lists_what_it_leaves_open() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let other = tmp.path().join("specs/002-y");
        std::fs::create_dir_all(other.join("scenarios")).unwrap();
        std::fs::write(
            other.join("spec.md"),
            "---\nstatus: in-progress\n---\n# 002\n",
        )
        .unwrap();
        std::fs::write(other.join("scenarios/fix.md"), "# Fix\n").unwrap();
        std::fs::write(
            tmp.path().join("specs/001-x/tasks.md"),
            "# Tasks\n\n## 1. Fix\n\n- [ ] a\n\n- **Done when**: a.\n",
        )
        .unwrap();
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.findings = vec![
            finding("SEC-1", "must", "src/must.rs", "1-2", "high"),
            finding("SEC-2", "should", "src/should.rs", "1-2", "high"),
            finding("SEC-3", "should", "src/must.rs", "9-9", "high"),
        ];
        let mut own_task = broken(
            "src/deferred.rs",
            DispositionOutcome::Deferred,
            Some("specs/001-x/tasks.md"),
        );
        own_task.disposition.task = Some("1".into());
        args.observations = vec![
            broken(
                "src/undecided.rs:12-14",
                DispositionOutcome::Undispositioned,
                None,
            ),
            broken("src/line.rs:7", DispositionOutcome::Undispositioned, None),
            own_task,
            broken("src/fixed.rs", DispositionOutcome::Fixed, None),
            broken(
                "src/waived.rs",
                DispositionOutcome::Waived,
                Some("a person chose"),
            ),
            broken(
                "src/released.rs",
                DispositionOutcome::Deferred,
                Some("specs/002-y/scenarios/fix.md"),
            ),
            observation("perf: not a break", "src/perf.rs"),
        ];
        run(&args, tmp.path()).unwrap();
        assert_eq!(
            listed_open(&tmp, "001-x"),
            [
                "src/deferred.rs",
                "src/line.rs",
                "src/must.rs",
                "src/should.rs",
                "src/undecided.rs",
            ]
        );
    }

    #[test]
    fn a_record_leaving_nothing_open_writes_no_list() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        run(&base_args("001-x"), tmp.path()).unwrap();
        assert!(!review_md(&tmp, "001-x").contains("open-findings"));
    }

    /// With no file the next run could not read it again, so the hold would have no
    /// way to be cleared by reading. A break a person waived needs none.
    #[test]
    fn a_broken_contract_left_open_names_its_file() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.observations = vec![broken("  ", DispositionOutcome::Undispositioned, None)];
        let err = run(&args, tmp.path()).unwrap_err();
        assert!(err.to_string().contains("observations[0].path"), "{err}");

        args.decided_by = Some("dev@example.com".into());
        args.observations = vec![broken(
            "",
            DispositionOutcome::Waived,
            Some("a person chose"),
        )];
        run(&args, tmp.path()).unwrap();
        args.observations = vec![broken("", DispositionOutcome::Fixed, None)];
        run(&args, tmp.path()).unwrap();
    }

    /// An empty scope read nothing, so it cannot clear what the record counts: neither
    /// a count, for a record from before the list, nor a listed break. A clean record,
    /// no record, and the run's own earlier write are all fine.
    #[test]
    fn an_empty_scope_write_refuses_over_a_record_counting_a_finding() {
        let mut empty = base_args("001-x");
        empty.empty_scope = true;
        for held in [
            "must-violations: 1\nshould-violations: 0\nlow-confidence: 0\nblocking: true\n",
            "must-violations: 0\nshould-violations: 1\nlow-confidence: 0\nblocking: false\n",
            "must-violations: 0\nshould-violations: 0\nlow-confidence: 0\nblocking: false\nopen-findings:\n  - src/a.rs\n",
        ] {
            // A window holding a file, so the flag is what refuses.
            let (tmp, base) =
                spec_repo_with_a_change("001-x", "status: in-progress\ndependencies: []");
            empty.diff_base = base;
            let record = format!("---\nspec: 001-x\nlast-run: 2020-01-01T00:00:00Z\n{held}---\n");
            std::fs::write(tmp.path().join("specs/001-x/review.md"), &record).unwrap();
            let err = run(&empty, tmp.path()).unwrap_err();
            assert!(err.to_string().contains("empty-scope"), "{held}: {err}");
            assert_eq!(
                std::fs::read_to_string(tmp.path().join("specs/001-x/review.md")).unwrap(),
                record,
                "a refusal writes nothing"
            );
        }

        empty.diff_base = String::new();
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        run(&empty, tmp.path()).unwrap();
        run(&empty, tmp.path()).unwrap();

        let mut own = empty.clone();
        own.observations = vec![broken(
            "src/a.rs",
            DispositionOutcome::Undispositioned,
            None,
        )];
        run(&own, tmp.path()).unwrap();
        run(&own, tmp.path()).unwrap();
    }

    /// A person agreeing with the rule's waiver makes no new decision, so the record
    /// and the body both keep the rule's; deferring it does change the decision. A
    /// person's own stored waiver is never relabelled the rule's.
    #[test]
    fn a_waiver_agreeing_with_the_rules_stays_the_rules_in_the_record_and_the_body() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.observations = vec![rule_waived("perf: a loop")];
        run(&args, tmp.path()).unwrap();
        let rule_reason = args.observations[0].disposition.reason.clone().unwrap();

        args.reviewed_at = "2026-11-02T00:00:00Z".into();
        args.observations = vec![dispositioned(
            "perf: a loop",
            DispositionOutcome::Waived,
            Some("a person agrees"),
        )];
        run(&args, tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(report.contains("decided-by: rule"), "{report}");
        assert!(!report.contains("a person agrees"), "{report}");
        assert!(report.contains(&format!("**waived**: {rule_reason} (decided by the rule)")));

        // Fixed since, its stored waiver standing: the body says what this run did.
        let fixed = {
            let mut once = args.clone();
            once.reviewed_at = "2026-11-02T12:00:00Z".into();
            once.observations = vec![dispositioned(
                "perf: a loop",
                DispositionOutcome::Fixed,
                None,
            )];
            once
        };
        run(&fixed, tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(report.contains("decided-by: rule"), "{report}");
        assert!(report.contains("— **fixed**"), "{report}");

        args.reviewed_at = "2026-11-03T00:00:00Z".into();
        args.observations = vec![dispositioned(
            "perf: a loop",
            DispositionOutcome::Deferred,
            Some("specs/001-x/spec.md"),
        )];
        run(&args, tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(report.contains("decided-by: dev@example.com"), "{report}");
        assert!(!report.contains("(decided by the rule)"), "{report}");

        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.observations = vec![dispositioned(
            "perf: a loop",
            DispositionOutcome::Waived,
            Some("a person chose"),
        )];
        run(&args, tmp.path()).unwrap();
        args.reviewed_at = "2026-11-02T00:00:00Z".into();
        run(&args, tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(report.contains("**waived**: a person chose"), "{report}");
        assert!(!report.contains("(decided by the rule)"), "{report}");
    }

    #[test]
    fn a_record_with_no_contract_break_says_zero() {
        let dir = spec_repo("001-x", "status: in-progress\ndependencies: []");
        run(&base_args("001-x"), dir.path()).unwrap();
        assert!(review_md(&dir, "001-x").contains("\ncontract-breaks: 0\n"));
    }

    #[test]
    fn the_rule_never_waives_a_broken_contract() {
        let dir = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        let mut waived = rule_waived("bug: a contract broken");
        waived.contract_break = true;
        args.observations = vec![waived];
        let err = run(&args, dir.path()).unwrap_err();
        assert!(
            err.to_string()
                .contains("observations[0].disposition.by-rule"),
            "{err}"
        );
    }

    #[test]
    fn a_security_observation_is_listed_first() {
        let dir = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.observations = vec![
            observation("perf: a loop", "src/a.rs"),
            observation("Security: a token logged", "src/b.rs"),
            observation("other: noted", ""),
            observation("leak: a handle kept", "src/c.rs"),
        ];
        run(&args, dir.path()).unwrap();
        let report = review_md(&dir, "001-x");
        let at = |needle: &str| report.find(needle).unwrap();
        assert!(at("Security: a token") < at("leak: a handle"), "{report}");
        assert!(at("leak: a handle") < at("perf: a loop"), "{report}");
        assert!(at("perf: a loop") < at("other: noted"), "{report}");
    }

    /// The first write marks the loop open and the last clears it; a run that
    /// never passes the argument, as under `ductus exec`, writes no mark.
    #[test]
    fn the_open_mark_follows_the_loop_open_argument() {
        let dir = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        run(&args, dir.path()).unwrap();
        assert!(!review_md(&dir, "001-x").contains("fix-rounds-open"));

        args.loop_open = true;
        run(&args, dir.path()).unwrap();
        assert_eq!(record_of(&dir).fix_rounds_open, Some(true));

        args.loop_open = false;
        run(&args, dir.path()).unwrap();
        assert_eq!(record_of(&dir).fix_rounds_open, None);
    }

    #[test]
    fn the_record_names_each_path_a_fix_round_left_out() {
        let dir = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        let mut second = round(FixRoundCheck::Exempt, None, None);
        second.excluded = vec!["src/other.rs".to_string()];
        args.fix_rounds = Some(vec![round(FixRoundCheck::Exempt, None, None), second]);
        run(&args, dir.path()).unwrap();
        assert!(
            review_md(&dir, "001-x")
                .contains("## Excluded from scope\n\n- `src/other.rs` — left out of fix round 2\n"),
            "{}",
            review_md(&dir, "001-x")
        );
    }

    fn record_of(tmp: &TempDir) -> crate::schema::primitives::ReviewBlock {
        let report = review_md(tmp, "001-x");
        let (fm, _) =
            crate::primitives::split_frontmatter(&report, Path::new("review.md")).unwrap();
        serde_norway::from_str(fm).unwrap()
    }

    /// Every count comes from the rounds, the waived round names its reason,
    /// its author and the run's time, and the Summary names the exempt and
    /// the waived round; the record reads back into the struct the gate reads.
    #[test]
    fn each_loop_field_is_derived_from_the_rounds() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.fix_rounds = Some(vec![
            round(FixRoundCheck::ReRead, Some(ReReadOutcome::Findings), None),
            round(FixRoundCheck::DiffChecked, None, None),
            round(FixRoundCheck::Exempt, None, None),
            round(
                FixRoundCheck::NotReRead,
                None,
                Some("the change reverts round 1"),
            ),
            round(FixRoundCheck::ReRead, Some(ReReadOutcome::Clean), None),
        ]);
        run(&args, tmp.path()).unwrap();

        let record = record_of(&tmp);
        assert_eq!(
            (
                record.fix_rounds,
                record.re_reads,
                record.diff_checked,
                record.not_re_read,
                record.budget_stop
            ),
            (Some(5), Some(2), Some(1), Some(1), Some(false))
        );
        assert_eq!(
            record.round_waivers,
            [RoundWaiver {
                round: 4,
                reason: "the change reverts round 1".into(),
                waived_by: "dev@example.com".into(),
                waived_at: args.reviewed_at.clone(),
            }]
        );
        let report = review_md(&tmp, "001-x");
        assert!(
            report.contains(
                "5 fix round(s): 2 re-read, 1 diff-checked, 1 exempt, 1 not re-read. \
                 Exempt: round 3. Round 4 was not re-read, waived: the change reverts round 1."
            ),
            "{report}"
        );
        assert!(!report.contains("not waived"), "{report}");
    }

    /// No list writes no loop field: a run that stated nothing about its
    /// fixes is not one with no rounds. An empty list writes every field at
    /// zero, and no `round-waivers`.
    #[test]
    fn an_absent_list_writes_no_loop_field_and_an_empty_one_writes_zeros() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        run(&base_args("001-x"), tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        for field in [
            "fix-rounds:",
            "re-reads:",
            "diff-checked:",
            "not-re-read:",
            "budget-stop:",
        ] {
            assert!(!report.contains(field), "{field}: {report}");
        }
        assert!(!report.contains("fix round(s)"), "{report}");
        let record = record_of(&tmp);
        assert_eq!((record.fix_rounds, record.not_re_read), (None, None));

        let mut args = base_args("001-x");
        args.fix_rounds = Some(Vec::new());
        run(&args, tmp.path()).unwrap();
        let record = record_of(&tmp);
        assert_eq!(
            (
                record.fix_rounds,
                record.re_reads,
                record.diff_checked,
                record.not_re_read,
                record.budget_stop
            ),
            (Some(0), Some(0), Some(0), Some(0), Some(false))
        );
        assert_eq!(record.round_waivers, [] as [RoundWaiver; 0]);
        let report = review_md(&tmp, "001-x");
        assert!(!report.contains("round-waivers"), "{report}");
        assert!(report.contains("0 fix round(s)"), "{report}");
    }

    /// A Summary the caller supplies is rendered exactly as given when the
    /// run passed no rounds, and carries the loop's account as a paragraph of
    /// its own when it did, so an exempt round is named whoever wrote the
    /// Summary, and a Summary ending in a code fence keeps the fence closed and
    /// every later section a heading (spec 078 AC2; 066's scenario
    /// `a-supplied-summary-carries-the-loops-account`).
    #[test]
    fn a_supplied_summary_carries_the_loops_account() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.summary = Some("Read the scope whole".into());
        run(&args, tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(
            report.contains("## Summary\n\nRead the scope whole\n\n## MUST"),
            "{report}"
        );

        args.summary = Some("Read the scope whole\n".into());
        args.fix_rounds = Some(vec![
            round(FixRoundCheck::Exempt, None, None),
            round(FixRoundCheck::ReRead, Some(ReReadOutcome::Clean), None),
        ]);
        run(&args, tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(
            report.contains(
                "## Summary\n\nRead the scope whole\n\n2 fix round(s): 1 re-read, 0 diff-checked, \
                 1 exempt, 0 not re-read. Exempt: round 1.\n\n## MUST"
            ),
            "{report}"
        );

        args.summary = Some("Read the scope whole:\n\n```text\nsrc/a.rs\n```\n".into());
        run(&args, tmp.path()).unwrap();
        let report = review_md(&tmp, "001-x");
        assert!(
            report.contains("src/a.rs\n```\n\n2 fix round(s): 1 re-read"),
            "{report}"
        );
        assert!(!report.contains("``` 2 fix round(s)"), "{report}");
    }

    /// One round is named alone, several in order with the last joined by
    /// "and".
    #[test]
    fn rounds_are_listed_in_order() {
        assert_eq!(round_list(&[3]), "round 3");
        assert_eq!(round_list(&[1, 2]), "rounds 1 and 2");
        assert_eq!(round_list(&[2, 4, 5]), "rounds 2, 4 and 5");
    }

    /// The budget stopped the loop when at least two re-reads found a MUST or
    /// SHOULD and the last re-read did; a diff-checked round between them is
    /// no re-read.
    #[test]
    fn the_budget_stop_needs_two_finding_re_reads_and_a_finding_last() {
        use ReReadOutcome::{Clean, Findings};
        let re_read = |result| round(FixRoundCheck::ReRead, Some(result), None);
        let checked = round(FixRoundCheck::DiffChecked, None, None);
        for (rounds, expect) in [
            (vec![re_read(Findings), re_read(Findings)], true),
            (
                vec![re_read(Findings), re_read(Clean), re_read(Findings)],
                true,
            ),
            (
                vec![re_read(Findings), checked.clone(), re_read(Findings)],
                true,
            ),
            (
                vec![re_read(Findings), re_read(Findings), re_read(Clean)],
                false,
            ),
            // The last re-read, not the last round: a diff-checked round after
            // it is no re-read.
            (
                vec![re_read(Findings), re_read(Findings), checked.clone()],
                true,
            ),
            (vec![re_read(Findings), checked], false),
            (vec![re_read(Findings)], false),
            (Vec::new(), false),
        ] {
            let record = derive_loop(&rounds, None, None, "2026-07-02T12:00:00Z");
            assert_eq!(record.budget_stop, expect, "{rounds:?}");
        }
        let stopped = derive_loop(
            &[re_read(Findings), re_read(Findings)],
            None,
            None,
            "2026-07-02T12:00:00Z",
        );
        assert!(loop_summary(&stopped).contains("The budget stopped the loop"));
        // The sentence counts the run's finding re-reads, not the budget: an
        // operator who let the loop go on past the budget has more.
        let continued = derive_loop(
            &[re_read(Findings), re_read(Findings), re_read(Findings)],
            None,
            None,
            "2026-07-02T12:00:00Z",
        );
        let expected = format!(
            "3 re-reads found a MUST or SHOULD, against a budget of {}.",
            fix_rounds::RE_READ_BUDGET
        );
        assert!(
            loop_summary(&continued).contains(&expected),
            "{}",
            loop_summary(&continued)
        );
    }

    /// With `[review] re-reads-off` set, every un-re-read round without its
    /// own reason is waived with the switch's, attributed to the config file;
    /// a round with its own reason keeps it. Without the switch, such a round
    /// is named in the Summary as holding `done`.
    #[test]
    fn the_switch_waives_each_round_left_without_a_reason() {
        const REASON: &str = "Reviews here run on a host without subagents.";
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.fix_rounds = Some(vec![
            round(FixRoundCheck::NotReRead, None, None),
            round(
                FixRoundCheck::NotReRead,
                None,
                Some("the operator declined"),
            ),
        ]);
        run(&args, tmp.path()).unwrap();
        assert!(
            review_md(&tmp, "001-x")
                .contains("Not re-read and not waived: round 1 — each holds `done`."),
            "without the switch the round is open"
        );
        assert_eq!(record_of(&tmp).round_waivers.len(), 1);

        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/config.toml"),
            format!("[review]\nre-reads-off = \"{REASON}\"\n"),
        )
        .unwrap();
        run(&args, tmp.path()).unwrap();
        let waivers = record_of(&tmp).round_waivers;
        assert_eq!(
            waivers
                .iter()
                .map(|w| (w.round, w.reason.as_str(), w.waived_by.as_str()))
                .collect::<Vec<_>>(),
            [
                (1, REASON, ".ductus/config.toml"),
                (2, "the operator declined", "dev@example.com"),
            ]
        );
        assert!(!review_md(&tmp, "001-x").contains("not waived"));
    }

    /// A malformed switch halts a run that passed its rounds before anything
    /// is written; a run that passed none does not read the switch, since it
    /// has no round for it to waive.
    #[test]
    fn a_malformed_switch_halts_only_a_run_with_rounds() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[review]\nre-reads-off = \"short\"\n",
        )
        .unwrap();
        let mut args = base_args("001-x");
        args.fix_rounds = Some(Vec::new());
        let err = run(&args, tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidReReadsOff { path, .. } if path.ends_with(".ductus/config.toml")),
            "{err}"
        );
        assert!(!tmp.path().join("specs/001-x/review.md").exists());

        run(&base_args("001-x"), tmp.path()).unwrap();
        assert!(tmp.path().join("specs/001-x/review.md").exists());
    }

    /// Each malformed entry is refused, naming its field, before anything is
    /// written, and an unknown key in an entry does not deserialize.
    #[test]
    fn each_malformed_round_is_refused_before_any_write() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        for (entry, author, argument) in [
            (
                round(FixRoundCheck::ReRead, None, None),
                true,
                "fix-rounds[0].result",
            ),
            (
                round(FixRoundCheck::Exempt, Some(ReReadOutcome::Clean), None),
                true,
                "fix-rounds[0].result",
            ),
            (
                round(
                    FixRoundCheck::NotReRead,
                    Some(ReReadOutcome::Findings),
                    None,
                ),
                true,
                "fix-rounds[0].result",
            ),
            (
                round(
                    FixRoundCheck::ReRead,
                    Some(ReReadOutcome::Clean),
                    Some("why"),
                ),
                true,
                "fix-rounds[0].reason",
            ),
            (
                round(FixRoundCheck::DiffChecked, None, Some("why")),
                true,
                "fix-rounds[0].reason",
            ),
            (
                round(FixRoundCheck::NotReRead, None, Some("  ")),
                true,
                "fix-rounds[0].reason",
            ),
            (
                round(FixRoundCheck::NotReRead, None, Some("one\ntwo")),
                true,
                "fix-rounds[0].reason",
            ),
            (
                round(FixRoundCheck::NotReRead, None, Some("why")),
                false,
                "decided-by",
            ),
        ] {
            let mut args = base_args("001-x");
            args.decided_by = author.then(|| "dev@example.com".to_string());
            args.fix_rounds = Some(vec![entry.clone()]);
            match run(&args, tmp.path()) {
                Err(PrimitiveError::InvalidArgument { argument: got, .. }) => {
                    assert_eq!(got, argument, "{entry:?}");
                }
                other => panic!("{entry:?} was not refused: {other:?}"),
            }
            assert!(!tmp.path().join("specs/001-x/review.md").exists());
        }

        let misspelled = serde_json::json!({
            "feature": "001-x", "reviewed-at": "t", "reviewed-against": "a", "diff-base": "b",
            "fix-rounds": [{ "check": "not-re-read", "reasn": "why" }]
        });
        assert!(serde_json::from_value::<WriteReviewArgs>(misspelled).is_err());
    }

    #[test]
    fn a_matched_decision_is_kept_and_an_expired_one_is_pruned() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        seed_waivers(
            &tmp,
            "001-x",
            "decisions:\n  - key: \"other: settled earlier\"\n    outcome: waived\n    reason: noise\n    decided-at: 2026-09-01T00:00:00Z\n    decided-by: first@example.com\n    ticket: OPS-1\n  - key: \"bug: since fixed\"\n    outcome: deferred\n    target: specs/001-x/tasks.md\n    decided-at: 2026-09-01T00:00:00Z\n    decided-by: first@example.com",
        );
        let mut args = base_args("001-x");
        let mut rematched = dispositioned(
            "other: settled, reworded",
            DispositionOutcome::Waived,
            Some("noise"),
        );
        rematched.decision_key = Some("other: settled earlier".into());
        args.observations = vec![rematched];
        args.expired_decisions = vec![DecisionRef {
            key: "bug: since fixed".into(),
            outcome: DecisionOutcome::Deferred,
            target: Some("specs/001-x/tasks.md".into()),
            task: None,
            work_item: None,
            reason: None,
            by_rule: false,
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
                dispositioned("ok", DispositionOutcome::Deferred, Some("  ")),
                "observations[0].disposition.target",
            ),
            (
                dispositioned("ok", DispositionOutcome::Waived, None),
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
            DispositionOutcome::Waived,
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
            DispositionOutcome::Waived,
            Some("false\u{ffff}positive"),
        )];
        assert!(matches!(
            run(&args, tmp.path()),
            Err(PrimitiveError::InvalidArgument { .. })
        ));
        assert!(!tmp.path().join("specs/001-x/review.md").exists());
    }

    /// A tab-then-`#` in an observation's text and its waiver reason reads
    /// back intact, so the stored key matches the same observation next run.
    /// Written plain, YAML read it as a comment and truncated both.
    #[test]
    fn a_decision_carrying_a_comment_opener_reads_back() {
        let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
        let mut args = base_args("001-x");
        args.decided_by = Some("dev@example.com".into());
        args.observations = vec![dispositioned(
            "bug: issue\t#12 reopens",
            DispositionOutcome::Waived,
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
            DispositionOutcome::Waived,
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
            dispositioned("other: twice", DispositionOutcome::Waived, Some("noise")),
            dispositioned("other: twice", DispositionOutcome::Waived, Some("noise")),
        ];
        let result = run(&args, tmp.path()).unwrap();
        let stored =
            decisions::read_decisions(&tmp.path().join("specs/001-x"), "review.md").unwrap();
        assert_eq!(stored.len(), 1, "one key, one stored decision");
        // Each observation is counted, as `observations` counts it: the map
        // counts the observations a run recorded, and both carry the one
        // disposition their key gets.
        assert_eq!(result.observations, 2);
        assert_eq!(result.dispositions.waived, 2);

        args.observations = vec![
            dispositioned("other: split", DispositionOutcome::Waived, Some("noise")),
            dispositioned(
                "other: split",
                DispositionOutcome::Deferred,
                Some("specs/001-x/tasks.md"),
            ),
        ];
        assert!(matches!(
            run(&args, tmp.path()),
            Err(PrimitiveError::InvalidArgument { argument, .. }) if argument == "observations"
        ));
    }

    /// One key, one disposition, whatever the two outcomes are: deferred beside
    /// undispositioned was accepted and counted once each, so one finding
    /// was recorded as both decided and owed.
    #[test]
    fn same_key_observations_with_any_two_outcomes_are_refused() {
        use DispositionOutcome::{Deferred, Fixed, Undispositioned, Waived};
        for (first, second) in [
            (Deferred, Undispositioned),
            (Fixed, Undispositioned),
            (Fixed, Waived),
        ] {
            let tmp = spec_repo("001-x", "status: in-progress\ndependencies: []");
            let mut args = base_args("001-x");
            args.decided_by = Some("dev@example.com".into());
            args.observations = [first, second]
                .into_iter()
                .map(|outcome| {
                    let companion = match outcome {
                        Deferred => Some("specs/001-x/tasks.md"),
                        Waived => Some("noise"),
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
        let (tmp, base) = spec_repo_with_a_change("001-x", "status: in-progress\ndependencies: []");
        let path = tmp.path().join("specs/001-x/review.md");
        fs::write(&path, "# Review — 001-x\n\nhand-edited, no record\n").unwrap();
        let mut args = base_args("001-x");
        args.diff_base = base;
        run(&args, tmp.path()).unwrap();
        assert!(review_md(&tmp, "001-x").starts_with("---\n"));
    }

    /// A prior `review.md` whose frontmatter opens and never closes may hold
    /// waivers and decisions nobody can read, so it is refused, named as
    /// unclosed, and left as it is.
    #[test]
    fn a_prior_review_whose_frontmatter_never_closes_is_refused() {
        let (tmp, base) = spec_repo_with_a_change("001-x", "status: in-progress\ndependencies: []");
        let path = tmp.path().join("specs/001-x/review.md");
        let damaged = "---\nspec: 001-x\nwaivers:\n  - rule: SEC-BE-001\n\n# Review\n";
        fs::write(&path, damaged).unwrap();
        let mut args = base_args("001-x");
        args.diff_base = base;
        let error = run(&args, tmp.path()).unwrap_err();
        assert!(error.to_string().contains("never closes"), "{error}");
        assert!(matches!(error, PrimitiveError::UnclosedFrontmatter { .. }));
        assert_eq!(fs::read_to_string(&path).unwrap(), damaged);
    }
}
