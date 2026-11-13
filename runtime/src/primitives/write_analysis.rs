//! `write-analysis` — record that `/ductus:analyze` ran, by writing
//! `specs/NNN/analysis.md`: the record in its frontmatter, the findings in a
//! fixed body skeleton. `spec.md` is not written (spec 057).
//!
//! The pipeline is `implement → review → analyze → done`, and until this
//! primitive existed only half of it left a trace. `check-review-gate` read
//! the `review:` block; Family 19 checked its freshness; Family 31 held it
//! against `review.md`. Analyze wrote nothing, so a spec that had passed both
//! gates and a spec that had passed only the first were **byte-identical on
//! disk**. Nothing could tell them apart, which meant nothing could enforce
//! the second gate, which meant the only thing holding it was whoever
//! remembered — the diligence dependency §design-principles rejects outright.
//!
//! That state was not hypothetical. On 2026-09-05 two specs were advanced to
//! `done` on the review gate alone and one of them was published to crates.io
//! before anyone noticed, because there was nothing to notice: every signal
//! the repository had said the spec was complete. The gap was found by being
//! asked, which is the definition of a diligence dependency.
//!
//! **Recording that the audit happened is not mutating the subject** — it is
//! precisely what `write-review` does for the other gate, and precisely why
//! that gate was enforceable and this one was not. Detection stays read-only;
//! the fix-and-route step that follows it writes under the disposition rule, with
//! a gating finding's deferral confirmed by the operator (spec 079), and
//! this record is written after that step and a re-check, so it describes the state
//! after dispositions (spec 058). What the primitive takes and derives is
//! `schema::descriptions::WRITE_ANALYSIS`.
//!
//! The block deliberately is **not** a copy of `review:`:
//!
//! - `advisory` is recorded and never gated on. An outstanding SHOULD blocks
//!   `done` at the review gate because §implement-phase says advisory is not
//!   ignorable there. Analyze's advisory tier is advisory by design: some of
//!   its checks were introduced advisory *with published promotion criteria*
//!   — grounding, Applicable-Rules citations, decision drift — and the rest,
//!   the plan record and un-folded branch specs among them, stay advisory for
//!   good. Gating on them here would promote every one of them at once, past
//!   the criteria some declare and against the design of the rest.
//! - `unexamined` has no counterpart in `review:` at all, and is the field
//!   that makes this record honest. A clean analyze is two states, not one,
//!   and the command's own contract says so: "clean with nothing skipped is
//!   verified-clean, clean with something skipped is partially examined." A
//!   record carrying only finding counts would collapse that into the
//!   reassuring reading — inside the artifact a later gate trusts, which is
//!   the worst possible place for `QUAL-CLAIM-001`.
//!
//! Defined by `specs/047-analyze-findings-durability/scenarios/analyze-run-durability.md`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::Path;

use crate::primitives::decisions;
use crate::primitives::departures::Entry;
use crate::primitives::rule_verdicts::{self, Currency};
use crate::primitives::write_review::{yaml_quoted, yaml_string};
use crate::primitives::{
    PrimitiveError, Result, flatten_line, read_text, rel_path, split_frontmatter, write_atomic,
};
use crate::schema::paths;
use crate::schema::primitives::{
    AnalysisFinding, AnalysisTier, DecisionOutcome, DecisionRef, Disposition, DispositionOutcome,
    Dispositions, FindingTier, ListsForm, RuleContradiction, RuleVerdict, WriteAnalysisArgs,
    WriteAnalysisResult,
};
use crate::schema::severity::RuleSeverity;

/// The `unexamined-by-reason` key for each assessable rule left unjudged,
/// written by this primitive from the loaded rule files and by the exec
/// walker's tally for a rule or step it could not bring to an assessment
/// (`CFG-CONST-001`: one definition for a key two modules write).
pub(crate) const REASON_RULE_ASSESSMENTS_NOT_CHECKED: &str = "rule-assessments-not-checked";

/// The key for each rule file the runtime could not read, written here and by
/// the exec walker's tally.
pub(crate) const REASON_RULE_FILE_UNREADABLE: &str = "rule-file-unreadable";

/// The key a caller's bare `unexamined` total is recorded under when it names
/// no breakdown, so the total stays the breakdown's sum (066's data model:
/// the total and its parts cannot disagree) and one host input writes one
/// record shape.
pub(crate) const REASON_UNCLASSIFIED: &str = "unclassified";

/// Refuse the record while implementation is open, but for the work this run's own deferrals
/// add to its spec (spec 079).
fn refuse_while_implementation_is_open(
    args: &WriteAnalysisArgs,
    root: &str,
    feature_dir: &Path,
) -> Result<()> {
    let deferred_here = super::decisions::tasks_deferred_here(
        root,
        &args.feature,
        args.findings.iter().map(|finding| &finding.disposition),
    );
    super::read_tasks::refuse_while_tasks_are_open("write-analysis", feature_dir, &deferred_here)
}

/// Execute the `write-analysis` primitive against the given repo root.
///
/// # Errors
///
/// - [`PrimitiveError::InvalidPath`] when `feature` is empty, absolute, or
///   carries a parent-directory component, or when the feature directory, a
///   link inside it, or a new deferral's target resolves outside the project.
/// - [`PrimitiveError::FeatureNotFound`] when the feature directory does not
///   exist.
/// - [`PrimitiveError::InvalidArgument`] when `analyzed-at` is blank, when a
///   finding's family or message is blank, when a finding's disposition lacks
///   its companion — a deferral naming neither a target nor a work item, or a
///   waiver stating no reason — or carries one it may not: a work-item ID over
///   more than one line, a task without a `tasks.md` target, a waiver naming
///   work; when a new deferral's target or task does not exist, or its task
///   is in a `tasks.md` that is no feature's; when it waives
///   a `hard-fail` or `blocking` finding; when the
///   findings contradict the tier counts, when two findings sharing a key are
///   given different outcomes, or when a new decision has no `decided-by` —
///   each before any write.
/// - [`PrimitiveError::InvalidArgument`] on `feature` while the spec's `tasks.md` holds an
///   unchecked box outside a task this run's own deferrals name (spec 079).
/// - [`PrimitiveError::Io`] when `spec.md`, the spec's `tasks.md`, the prior `analysis.md` or
///   a new deferral's named `tasks.md` cannot be read, or the record cannot be written.
/// - [`PrimitiveError::MissingFrontmatter`] when `spec.md` has no frontmatter
///   block.
/// - [`PrimitiveError::Yaml`] when `spec.md`'s frontmatter block is malformed —
///   never repaired here. A spec whose frontmatter does not parse is one the
///   analysis itself would have hard-failed on, and writing a record of a
///   clean run into it would be the exact inversion this primitive exists to
///   prevent. Also when the prior `analysis.md`'s frontmatter does not parse,
///   since the decisions it stores cannot be read and would be written over.
/// - [`PrimitiveError::UnclosedFrontmatter`] when the block in `spec.md` or
///   the prior `analysis.md` opens and never closes, for the same reasons. A
///   prior `analysis.md` that opens no block carries no decisions and is
///   overwritten.
pub fn run(args: &WriteAnalysisArgs, repo: &Path) -> Result<WriteAnalysisResult> {
    let root = paths::Paths::load(repo)?.specs_root;
    super::validate_feature_argument(repo, &root, &args.feature)?;
    let feature_dir = repo.join(&root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root,
            feature: args.feature.clone(),
        });
    }

    refuse_while_implementation_is_open(args, &root, &feature_dir)?;

    let spec_path = feature_dir.join("spec.md");
    let content = read_text(&spec_path)?;
    let (fm_text, _body) = split_frontmatter(&content, &spec_path)?;

    // Parse before writing. The value is not used, but a frontmatter block
    // that does not deserialize must not receive a record asserting a clean
    // analysis — see the `Yaml` note above.
    let _: crate::schema::primitives::Frontmatter =
        serde_norway::from_str(fm_text).map_err(|source| PrimitiveError::Yaml {
            path: spec_path.clone(),
            source,
        })?;

    // A blank timestamp would be stamped onto every new decision as a blank
    // `decided-at`, an entry the decisions reader classifies as malformed.
    if single_line(&args.analyzed_at).is_empty() {
        return Err(invalid("analyzed-at", "the run's timestamp is empty"));
    }
    // Before anything is counted or rendered: a rule's waiver of a finding a
    // person already decided carries that person's decision, which `decisions:`
    // keeps, so the record says one thing.
    let settled = settled(args, &feature_dir)?;
    let args = settled.as_ref();
    let gating = gating_counts(args, repo, &root);
    let blocking = gating.hard_fail > 0 || gating.blocking_findings > 0;
    let dispositions = count_dispositions(args)?;
    decisions::one_outcome_per_key(
        args.findings
            .iter()
            .map(|finding| (finding_key(finding), finding.disposition.outcome)),
    )
    .map_err(|refusal| refusal.into_error("write-analysis", "findings", "a finding"))?;
    // The prior record is read only for the decisions it stores. A file that
    // opens no frontmatter block carries none, so it is overwritten, and
    // re-running the command repairs a record damaged that way. One whose
    // frontmatter does not parse, or never closes, is refused: its
    // `decisions:` list cannot be read, and writing over it would drop
    // decisions nobody can see (spec 058).
    // A prior record whose frontmatter parses but whose rule fields alone do
    // not is not refused: `rule_verdicts::standing` reads it as
    // `record-unreadable`, so every rule is judged and the record is
    // overwritten (spec 082).
    let analysis_path = feature_dir.join(crate::primitives::ANALYSIS_RECORD_FILE);
    let replaced = analysis_path.is_file();
    let mut merged = decisions::merge(
        stored_decisions(&feature_dir, args, repo)?,
        &args.expired_decisions,
        &decided_findings(&args.findings),
        &single_line(&args.analyzed_at),
        args.decided_by.as_deref().map(single_line).as_deref(),
    )
    .map_err(|refusal| refusal.into_error("write-analysis", "findings", "a finding"))?;
    decisions::mark_gating(&mut merged, &gating.holding);
    // The breakdown is the authority when supplied: a total a caller can
    // contradict is a total that will eventually be contradicted, which is
    // the same reason `blocking` is derived rather than accepted.
    let mut by_reason: BTreeMap<String, u32> =
        args.unexamined_by_reason
            .iter()
            .fold(BTreeMap::new(), |mut acc, (reason, count)| {
                // Keyed as rendered, so two reasons that flatten to one line
                // are one entry rather than a duplicate YAML key.
                *acc.entry(single_line(reason)).or_default() += *count;
                acc
            });
    // A bare total names no breakdown, so it is recorded under its own
    // reason, before any derived entry joins it: `unexamined` is then always
    // the breakdown's sum, and a total a caller supplied is never dropped
    // when a derived reason appears beside it (`QUAL-CLAIM-001`).
    if by_reason.is_empty() && args.unexamined > 0 {
        by_reason.insert(REASON_UNCLASSIFIED.to_string(), args.unexamined);
    }
    count_constitution_gaps(&mut by_reason, repo);
    // The rule assessments, derived from the loaded rule files, the prior
    // record and what the host judged, for the same reason: a caller that had
    // to name the rules it left unjudged could leave out the ones it skipped
    // (spec 082, AC2, `QUAL-CLAIM-001`). Both counts land in the breakdown,
    // so the total cannot contradict the list.
    let spec_rel = format!("{root}/{}/spec.md", args.feature);
    let rules = derive_rule_fields(args, repo, &feature_dir, &spec_rel, &content)?;
    count_rule_gaps(&mut by_reason, &rules);
    let by_reason = by_reason;

    let unexamined: u32 = by_reason.values().copied().sum();
    // The digest of what this run examined, taken here rather than accepted
    // as an argument. `/{project}:analyze` is read-only, so the subjects are
    // byte-identical to the ones its passes read moments ago — and deriving it
    // means no caller can record a digest it did not take, the same discipline
    // that derives `blocking` and sums `unexamined` from its breakdown. It
    // also guarantees this digest and the one the gate recomputes come from
    // one function, which is what makes the two surfaces agree.
    let subjects = crate::primitives::analyze_subjects::subject_digest(
        &feature_dir,
        crate::primitives::analyze_subjects::is_analyze_subject,
    );
    // The artifact is written on every run it is not refused — clean,
    // empty-scope, blocking alike. A later gate reads its absence as "never
    // analyzed" (spec 057 AC5), so a run that declined to write because it had
    // nothing to report would make "no analysis" and "a clean analysis" the
    // same state on disk. That is the byte-identical failure `write-analysis`
    // was built to end, one level out. The refusals are the ones above: a spec
    // or a prior record whose frontmatter this run cannot read, or a rule directory
    // it cannot list.
    let report = render_analysis(
        args,
        &Derived {
            blocking,
            unexamined,
            dispositions,
            gating,
        },
        &by_reason,
        &subjects,
        &rules.fields,
        &merged,
    );
    // The record is written first: its absence is what a later gate reads as
    // "never analyzed", so a read size that does not parse, which halts the
    // bounding below as it halts every bounded result, must not suppress it.
    write_atomic(&analysis_path, &report)?;
    bounded(
        repo,
        &args.feature,
        WriteAnalysisResult {
            spec_path: rel_path(&spec_path, repo),
            blocking,
            unexamined,
            replaced,
            contradictions: rules.fields.contradictions,
            dispositions,
            ..WriteAnalysisResult::default()
        },
    )
}

/// Bound `result` to one read (`BE-PAYLOAD-001`): `contradictions` inline
/// while the result fits `[artifacts] read-size-bytes`, and past it in the
/// file `lists-file` names, with `contradictions-count` inline in both forms,
/// as `read-re-read` bounds its findings. An empty list stays absent inline,
/// as before the bound, so a result with none reads as it always did.
fn bounded(
    repo: &Path,
    feature: &str,
    mut result: WriteAnalysisResult,
) -> Result<WriteAnalysisResult> {
    result.contradictions_count = result.contradictions.len();
    let lists = serde_json::json!({ "contradictions": &result.contradictions });
    if let Some(file) =
        super::bounded_lists::place(repo, "write-analysis", feature, &result, &lists)?
    {
        result.form = ListsForm::File;
        result.contradictions = Vec::new();
        result.lists_file = Some(file);
    }
    Ok(result)
}

/// Add the shared constitutions this run could not read to the unexamined
/// breakdown.
fn count_constitution_gaps(by_reason: &mut BTreeMap<String, u32>, repo: &Path) {
    // A registered shared constitution this run could not read is an unexamined
    // *input* to the analysis, not an unexamined target, but it lands in the same
    // breakdown because it makes the same claim false: that everything bearing on
    // the verdict was looked at. Derived here rather than accepted as an argument,
    // for the reason `run`'s breakdown comment gives — a caller that had to supply it
    // could omit it, and `QUAL-CLAIM-001` exists precisely to stop a clean result
    // standing in for an unexamined one (spec 055, AC8).
    // Deliberately NOT `?`. This primitive's contract is that it writes a
    // record on every run whose config parses, because the record's
    // *absence* is what a later gate reads as "never analyzed" — so a
    // malformed `[constitutions]` entry must not be able to suppress it. An
    // unreadable registry is recorded as its own reason instead, which is the
    // honest answer: nothing registered was loaded, and the run cannot say
    // how many sources that was. A config that does not parse, or names a
    // `[paths] specs-root` it rejects, halted the run at `Paths::load` above,
    // since the spec root is unknown (spec 022 task 12).
    match crate::primitives::resolve_constitutions::run(repo) {
        Ok(governance) if !governance.skipped.is_empty() => {
            let count = u32::try_from(governance.skipped.len()).unwrap_or(u32::MAX);
            *by_reason
                .entry("constitution-unresolved".to_string())
                .or_default() += count;
        }
        Ok(_) => {}
        Err(_) => {
            *by_reason
                .entry("constitution-registry-unreadable".to_string())
                .or_default() += 1;
        }
    }
}

/// A validation failure naming the offending argument.
fn invalid(argument: impl Into<String>, reason: &str) -> PrimitiveError {
    PrimitiveError::InvalidArgument {
        primitive: "write-analysis".into(),
        argument: argument.into(),
        reason: reason.into(),
    }
}

/// The tier's live count, as the host's re-check stated it.
fn tier_count(args: &WriteAnalysisArgs, tier: AnalysisTier) -> u32 {
    match tier {
        AnalysisTier::HardFail => args.hard_fail,
        AnalysisTier::Blocking => args.blocking_findings,
        AnalysisTier::Advisory => args.advisory,
    }
}

/// Validate the itemized findings against the tier counts and derive the
/// `dispositions:` map. Every refusal of the itemization itself lands here;
/// the key, author and missing-work refusals follow in `run`, all before any
/// write.
///
/// The tier counts are the authority on how many live findings exist, so the
/// itemization may fall short of them — the shortfall is undispositioned — but
/// may never exceed them, and may never contradict itself: a finding the
/// re-check still produces was not fixed, and a finding gone from the re-check
/// was.
fn count_dispositions(args: &WriteAnalysisArgs) -> Result<Dispositions> {
    let mut counts = Dispositions::default();
    let mut itemized = [0u32; 3];
    for (idx, finding) in args.findings.iter().enumerate() {
        let at = |field: &str| format!("findings[{idx}].{field}");
        if finding.family.trim().is_empty() {
            return Err(invalid(at("family"), "family is empty"));
        }
        if finding.message.trim().is_empty() {
            return Err(invalid(at("message"), "message is empty"));
        }
        let disposition = &finding.disposition;
        decisions::require_companion("write-analysis", &at("disposition"), disposition)?;
        if disposition.outcome == DispositionOutcome::Waived
            && matches!(
                finding.tier,
                AnalysisTier::HardFail | AnalysisTier::Blocking
            )
        {
            return Err(invalid(
                at("disposition.outcome"),
                "a hard-fail or blocking finding already gates done; fix or defer it, never waive it",
            ));
        }
        if !finding.live {
            if disposition.outcome != DispositionOutcome::Fixed {
                return Err(invalid(
                    at("disposition.outcome"),
                    "a finding gone from the re-check was fixed; record it as fixed",
                ));
            }
            counts.fixed = counts.fixed.saturating_add(1);
            continue;
        }
        let slot = match finding.tier {
            AnalysisTier::HardFail => 0,
            AnalysisTier::Blocking => 1,
            AnalysisTier::Advisory => 2,
        };
        itemized[slot] = itemized[slot].saturating_add(1);
        match disposition.outcome {
            DispositionOutcome::Fixed => {
                return Err(invalid(
                    at("disposition.outcome"),
                    "the re-check still produces this finding, so it was not fixed",
                ));
            }
            DispositionOutcome::Deferred => counts.deferred = counts.deferred.saturating_add(1),
            DispositionOutcome::Waived => counts.waived = counts.waived.saturating_add(1),
            DispositionOutcome::Undispositioned => {}
        }
    }
    for (slot, tier) in [
        AnalysisTier::HardFail,
        AnalysisTier::Blocking,
        AnalysisTier::Advisory,
    ]
    .into_iter()
    .enumerate()
    {
        if itemized[slot] > tier_count(args, tier) {
            return Err(invalid(
                "findings",
                "more live findings are itemized in a tier than its count states",
            ));
        }
    }
    let live_total = args
        .hard_fail
        .saturating_add(args.blocking_findings)
        .saturating_add(args.advisory);
    counts.undispositioned =
        live_total.saturating_sub(counts.deferred.saturating_add(counts.waived));
    Ok(counts)
}

/// The arguments as this run records them: a finding the rule waived again
/// carrying the stored decision of the person it cannot replace, as
/// `write-review` settles an observation (spec 079).
fn settled<'a>(
    args: &'a WriteAnalysisArgs,
    feature_dir: &Path,
) -> Result<std::borrow::Cow<'a, WriteAnalysisArgs>> {
    let stored = decisions::read_decisions(feature_dir, crate::primitives::ANALYSIS_RECORD_FILE)?;
    let overruled: Vec<(usize, Disposition)> = args
        .findings
        .iter()
        .enumerate()
        .filter_map(|(idx, finding)| {
            standing_decision(finding, &stored, &args.expired_decisions)
                .map(|decision| (idx, decision))
        })
        .collect();
    if overruled.is_empty() {
        return Ok(std::borrow::Cow::Borrowed(args));
    }
    let mut args = args.clone();
    for (idx, decision) in overruled {
        args.findings[idx].disposition = decision;
    }
    Ok(std::borrow::Cow::Owned(args))
}

/// The stored decision, when the rule waives this finding again: the rule never
/// replaces a person's unless this run expired it, so the count and the body
/// follow it rather than contradict what the record stores (spec 079).
fn standing_decision(
    finding: &AnalysisFinding,
    stored: &[decisions::RawDecision],
    expired: &[DecisionRef],
) -> Option<Disposition> {
    let disposition = &finding.disposition;
    if !disposition.by_rule || disposition.outcome != DispositionOutcome::Waived {
        return None;
    }
    let key = finding_key(finding);
    if expired
        .iter()
        .any(|gone| decisions::same_key(&gone.key, &key))
    {
        return None;
    }
    let decision = decisions::stored_decision(stored, &key)?;
    Some(Disposition {
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

/// The record's stored decisions, read once every new deferral among the
/// live findings is shown to name work that exists (spec 076) — before the
/// merge, so a refusal lands before any write.
fn stored_decisions(
    feature_dir: &Path,
    args: &WriteAnalysisArgs,
    repo: &Path,
) -> Result<Vec<decisions::RawDecision>> {
    let stored = decisions::read_decisions(feature_dir, crate::primitives::ANALYSIS_RECORD_FILE)?;
    for (idx, finding) in args.findings.iter().enumerate().filter(|(_, f)| f.live) {
        decisions::refuse_missing_work(
            repo,
            "write-analysis",
            &format!("findings[{idx}].disposition"),
            &finding.disposition,
            &finding_key(finding),
            &stored,
            &args.expired_decisions,
        )?;
    }
    Ok(stored)
}

/// The live findings deferred or waived, as the decisions they store, keyed
/// by [`finding_key`].
fn decided_findings(findings: &[AnalysisFinding]) -> Vec<DecisionRef> {
    findings
        .iter()
        .filter(|finding| finding.live)
        .filter_map(|finding| decisions::decision_for(&finding.disposition, &finding_key(finding)))
        .collect()
}

/// The stored-decision key for an analyze finding: the stored decision the
/// host matched it to, when it supplied one, else its own
/// `{family} — {message}` — the deterministic half of the old capture key,
/// whose leading `{category}` was assigned by the host when it wrote the
/// bullet and did not reproduce across runs.
fn finding_key(finding: &AnalysisFinding) -> String {
    if let Some(key) = finding
        .decision_key
        .as_deref()
        .map(single_line)
        .filter(|key| !key.is_empty())
    {
        return key;
    }
    finding_key_of(&finding.family, &finding.message)
}

/// The stored-decision key of the analyze finding with this family and
/// message, `{family} — {message}`. `prune-plan` looks a plan section's stored
/// decision up by it, so the key prune reads and the key this primitive writes
/// for a finding the host matched to no earlier decision come from one
/// function; a finding re-matched by judgment keeps its stored key instead.
pub(crate) fn finding_key_of(family: &str, message: &str) -> String {
    format!("{} — {}", single_line(family), single_line(message))
}

/// The three rule-assessment fields the record carries beside the counts
/// (spec 082): each fired rule's verdict, each assessable rule left unjudged,
/// and the digest of each loaded rule file.
#[derive(Default)]
struct RuleFields {
    verdicts: BTreeMap<String, RuleVerdict>,
    unjudged: Vec<String>,
    digest: BTreeMap<String, String>,
    /// Reported in the result and the body, recorded in no field.
    contradictions: Vec<RuleContradiction>,
}

/// Add the gaps [`derive_rule_fields`] found to the unexamined breakdown: each
/// unjudged rule under `rule-assessments-not-checked`, and the rule files the
/// runtime could not read under `rule-file-unreadable`, where a walk counts
/// the files it could not read too, and the run's two reads of the one
/// directory are one gap, not two.
fn count_rule_gaps(by_reason: &mut BTreeMap<String, u32>, rules: &DerivedRules) {
    if !rules.fields.unjudged.is_empty() {
        *by_reason
            .entry(REASON_RULE_ASSESSMENTS_NOT_CHECKED.to_string())
            .or_default() += u32::try_from(rules.fields.unjudged.len()).unwrap_or(u32::MAX);
    }
    if rules.unreadable_files > 0 {
        let counted = by_reason
            .entry(REASON_RULE_FILE_UNREADABLE.to_string())
            .or_default();
        *counted = (*counted).max(rules.unreadable_files);
    }
}

/// What [`derive_rule_fields`] found.
struct DerivedRules {
    fields: RuleFields,
    /// Rule files listed that could not be read, whose rules cannot be named.
    unreadable_files: u32,
}

/// Refuse a new verdict that names no assessable loaded rule, a `finding`
/// with no message, a `finding` whose tier its rule cannot fail in, a
/// `waived` for a rule the spec holds no departure entry for, and a `finding`
/// or `addressed` for a rule it does (spec 073): the entry is the decision, so
/// a host can neither invent a waiver nor turn one back into a finding.
fn check_new_verdicts(
    args: &WriteAnalysisArgs,
    standing: &rule_verdicts::Standing,
    entries: &BTreeMap<String, Entry>,
) -> Result<()> {
    for (id, verdict) in &args.rule_verdicts {
        if standing.currency(id).is_none() {
            return Err(invalid(
                format!("rule-verdicts.{id}"),
                "names no assessable rule of the loaded rule files",
            ));
        }
        match (verdict, entries.contains_key(id)) {
            (RuleVerdict::Waived { .. }, false) => {
                return Err(invalid(
                    format!("rule-verdicts.{id}"),
                    "a waived verdict names a rule the spec's Applicable Rules holds no well-formed departure entry for",
                ));
            }
            (RuleVerdict::Finding { .. } | RuleVerdict::Addressed { .. }, true) => {
                return Err(invalid(
                    format!("rule-verdicts.{id}"),
                    "the spec's Applicable Rules holds a departure entry for this rule, so a fired answer is waived, never a finding or addressed",
                ));
            }
            _ => {}
        }
        if let RuleVerdict::Finding { message, tier, .. } = verdict {
            if single_line(message).is_empty() {
                return Err(invalid(
                    format!("rule-verdicts.{id}.message"),
                    "a finding verdict's message is empty",
                ));
            }
            // A finding is counted in its own tier, but a rule whose
            // Statement carries one keyword can only fail in that tier: a
            // MUST failure labelled `should` would leave blocking-findings
            // and let the spec reach `done`. A rule carrying both may fail on
            // either clause.
            let rule = standing
                .rules()
                .iter()
                .map(|(rule, _)| rule)
                .find(|rule| rule.id == *id);
            if let Some(rule) = rule
                && !rule.mixed
                && (rule.tier == RuleSeverity::Should) != (*tier == FindingTier::Should)
            {
                let name = |should: bool| if should { "should" } else { "must" };
                return Err(invalid(
                    format!("rule-verdicts.{id}.tier"),
                    &format!(
                        "the finding's tier is {} but {id}'s Statement carries only {}",
                        name(*tier == FindingTier::Should),
                        name(rule.tier == RuleSeverity::Should),
                    ),
                ));
            }
        }
    }
    Ok(())
}

/// Derive the record's rule fields (spec 082).
///
/// The universe is every assessable rule of every file in the rule-file
/// directory, as `/{project}:analyze` loads them, so a host that treated a
/// file as unloadable still has its rules named unjudged. Each rule is then
/// one of: judged by this run (a verdict stored when it fired, its `addressed`
/// citation checked), reused from a prior verdict that is still current
/// (carried forward, so a host cannot drop a reusable finding by not passing
/// it), or left unjudged. A rule the prior record held that no loaded file
/// defines is in none of them, so its verdict is dropped.
///
/// A new `finding` over a rule whose prior verdict was current and quiet or
/// addressed is a contradiction, reported and held against nothing.
///
/// # Errors
///
/// [`PrimitiveError::InvalidArgument`] when a verdict names a rule no loaded
/// file defines as assessable, when a `finding` verdict has no message, when a
/// `finding`'s `tier` is one its rule's current Statement cannot fail in (a
/// rule carrying both MUST and SHOULD can fail in either), or when a tier's
/// supplied count is below the number of `finding` verdicts the record holds
/// in that tier: each stands for at least one live finding, and a count under
/// that would drop a MUST failure from `blocking-findings`. That floor reads
/// the tier each verdict stored, the one the finding was counted in, not the
/// rule's current text. Reused findings are the host's to count, and the exec
/// tally does. Also the errors of [`rule_verdicts::load_universe`]: a project
/// config that does not parse, and a rule directory that cannot be listed,
/// which writes no record. A prior record whose rule fields alone do not
/// parse is no error: it reads as `record-unreadable`, every rule is judged,
/// and the record is overwritten.
fn derive_rule_fields(
    args: &WriteAnalysisArgs,
    repo: &Path,
    feature_dir: &Path,
    spec_rel: &str,
    spec_content: &str,
) -> Result<DerivedRules> {
    let universe = rule_verdicts::load_universe(repo)?;
    let standing = rule_verdicts::standing(feature_dir, &universe.files);
    let entries = rule_verdicts::departure_entries(spec_content);
    check_new_verdicts(args, &standing, &entries)?;
    let judged: BTreeSet<&str> = args
        .judged_rules
        .iter()
        .map(String::as_str)
        .chain(args.rule_verdicts.keys().map(String::as_str))
        .collect();
    let mut fields = RuleFields {
        digest: universe
            .files
            .iter()
            .map(|file| (file.name.clone(), file.digest.clone()))
            .collect(),
        ..RuleFields::default()
    };
    for (rule, current) in standing.rules() {
        let id = rule.id.as_str();
        if !judged.contains(id) {
            match current {
                // A carried waiver whose entry is gone is not carried: it
                // cannot be refilled, and the rule is judged again.
                Currency::Fired(RuleVerdict::Waived { .. }) if !entries.contains_key(id) => {
                    fields.unjudged.push(rule.id.clone());
                }
                Currency::Fired(verdict) => {
                    fields.verdicts.insert(rule.id.clone(), verdict.clone());
                }
                Currency::NotFired => {}
                Currency::Stale => fields.unjudged.push(rule.id.clone()),
            }
            continue;
        }
        match args.rule_verdicts.get(id) {
            // A judgment no later run can test is no judgment.
            Some(RuleVerdict::Addressed { cite })
                if !rule_verdicts::cite_resolves(feature_dir, cite) =>
            {
                fields.unjudged.push(rule.id.clone());
            }
            Some(verdict) => {
                if let Some(earlier) = rule_verdicts::contradiction(current, verdict) {
                    fields.contradictions.push(RuleContradiction {
                        rule: rule.id.clone(),
                        earlier: earlier.to_string(),
                    });
                }
                fields.verdicts.insert(rule.id.clone(), verdict.clone());
            }
            None => {}
        }
    }
    // Every `waived` verdict, supplied or carried, takes its fields from the
    // entry and its history now: attribution depends on commits, not on a
    // digest, so a stored one may be stale though its subjects are current.
    rule_verdicts::fill_waived(repo, spec_rel, &entries, &mut fields.verdicts);
    // The tier a finding was counted in, which its verdict stored, is the one
    // it floors, not the rule's current text tier: a rule carrying MUST and
    // SHOULD can fail on its SHOULD clause alone, which counts advisory
    // whatever its Statement's tier says.
    let mut owed = [0u32; 2];
    for verdict in fields.verdicts.values() {
        if let RuleVerdict::Finding { tier, .. } = verdict {
            owed[usize::from(*tier == FindingTier::Should)] += 1;
        }
    }
    for (count, argument, supplied, tier) in [
        (owed[0], "blocking-findings", args.blocking_findings, "MUST"),
        (owed[1], "advisory", args.advisory, "SHOULD"),
    ] {
        if supplied < count {
            return Err(invalid(
                argument,
                &format!(
                    "the record holds {count} {tier}-tier finding verdict(s), each at least one live finding, and the count is {supplied}"
                ),
            ));
        }
    }
    Ok(DerivedRules {
        fields,
        unreadable_files: universe.unreadable,
    })
}

/// The counts this call derived, rendered into the record.
struct Derived {
    blocking: bool,
    unexamined: u32,
    dispositions: Dispositions,
    gating: Gating,
}

/// The gating tiers as the record states them (spec 076): the re-check's
/// counts less each live finding whose deferral releases it to another spec's
/// gate, with the indices of those released findings and the keys of the
/// gating deferrals that release nothing.
struct Gating {
    hard_fail: u32,
    blocking_findings: u32,
    released: Vec<usize>,
    holding: Vec<String>,
}

fn gating_counts(args: &WriteAnalysisArgs, repo: &Path, root: &str) -> Gating {
    let gates = |finding: &AnalysisFinding| {
        finding.live
            && matches!(
                finding.tier,
                AnalysisTier::HardFail | AnalysisTier::Blocking
            )
    };
    let mut gating = Gating {
        hard_fail: args.hard_fail,
        blocking_findings: args.blocking_findings,
        released: Vec::new(),
        holding: Vec::new(),
    };
    for (idx, finding) in args.findings.iter().enumerate().filter(|(_, f)| gates(f)) {
        if decisions::releases(repo, root, &args.feature, &finding.disposition) {
            gating.released.push(idx);
            let count = match finding.tier {
                AnalysisTier::HardFail => &mut gating.hard_fail,
                _ => &mut gating.blocking_findings,
            };
            *count = count.saturating_sub(1);
        } else if finding.disposition.outcome == DispositionOutcome::Deferred {
            gating.holding.push(finding_key(finding));
        }
    }
    gating
}

/// Render `analysis.md` — the record's frontmatter plus the fixed skeleton.
///
/// The counterpart to `write-review`'s `render_report`, and deliberately the
/// same shape: one artifact per command, carrying its own record and a report
/// a person can read. Overwritten whole on every run, so the file always
/// describes the current analysis and git carries the history.
fn render_analysis(
    args: &WriteAnalysisArgs,
    derived: &Derived,
    by_reason: &BTreeMap<String, u32>,
    subjects: &crate::primitives::analyze_subjects::SubjectDigest,
    rules: &RuleFields,
    decisions_list: &[decisions::RawDecision],
) -> String {
    let (blocking, unexamined, counts) =
        (derived.blocking, derived.unexamined, derived.dispositions);
    let feature = &args.feature;
    let mut out = String::from("---\n");
    // Every caller-supplied scalar and key is flattened to one line and then
    // quoted when a plain scalar would not read back — a `: ` or a leading
    // `#` in one would otherwise leave a record the next run refuses.
    let _ = writeln!(out, "spec: {}", yaml_string(&single_line(feature)));
    let _ = writeln!(
        out,
        "last-run: {}",
        yaml_string(&single_line(&args.analyzed_at))
    );
    let _ = writeln!(
        out,
        "analyzed-against: {}",
        yaml_string(&single_line(&args.analyzed_against))
    );
    let _ = writeln!(out, "hard-fail: {}", derived.gating.hard_fail);
    let _ = writeln!(
        out,
        "blocking-findings: {}",
        derived.gating.blocking_findings
    );
    let _ = writeln!(out, "advisory: {}", args.advisory);
    let _ = writeln!(out, "unexamined: {unexamined}");
    if !subjects.digests.is_empty() {
        let _ = writeln!(out, "analyzed-digest:");
        for (path, digest) in &subjects.digests {
            let _ = writeln!(out, "  {path}: {digest}");
        }
    }
    if !subjects.unreadable.is_empty() {
        let _ = writeln!(out, "analyzed-unreadable:");
        for path in &subjects.unreadable {
            let _ = writeln!(out, "  - {path}");
        }
    }
    if !by_reason.is_empty() {
        let _ = writeln!(out, "unexamined-by-reason:");
        for (reason, count) in by_reason {
            let _ = writeln!(out, "  {}: {count}", yaml_string(reason));
        }
    }
    render_rule_fields(&mut out, rules);
    let _ = writeln!(out, "blocking: {blocking}");
    decisions::render_dispositions(&mut out, counts);
    decisions::render(&mut out, decisions_list);
    out.push_str("---\n\n");

    let _ = writeln!(out, "# Analysis — {feature}\n");
    let verdict = if blocking { "blocking" } else { "not blocking" };
    let _ = writeln!(out, "## Summary\n");
    let mut summary = format!(
        "{} hard-fail, {} blocking, {} advisory; {verdict}. {} unexamined target(s). \
         Dispositions: {} fixed, {} deferred, {} waived, {} undispositioned.",
        derived.gating.hard_fail,
        derived.gating.blocking_findings,
        args.advisory,
        unexamined,
        counts.fixed,
        counts.deferred,
        counts.waived,
        counts.undispositioned,
    );
    if counts.undispositioned > 0 {
        summary.push_str(
            " An undispositioned advisory finding does not hold `done`; a hard-fail or blocking one holds it until the re-check no longer produces it.",
        );
    }
    let _ = writeln!(out, "{summary}\n");
    for (heading, tier) in [
        ("Hard failures", AnalysisTier::HardFail),
        ("Blocking findings", AnalysisTier::Blocking),
        ("Advisory findings", AnalysisTier::Advisory),
    ] {
        let tier_section = render_tier(args, tier, &derived.gating.released);
        let _ = writeln!(out, "## {heading}\n\n{tier_section}\n");
    }
    let released: Vec<&AnalysisFinding> = derived
        .gating
        .released
        .iter()
        .map(|&idx| &args.findings[idx])
        .collect();
    let _ = writeln!(
        out,
        "## Deferred to other specs\n\n{}\n",
        render_list(&released)
    );
    out.push_str(&render_rule_sections(by_reason, rules));
    let fixed: Vec<&AnalysisFinding> = args.findings.iter().filter(|f| !f.live).collect();
    let _ = write!(out, "## Fixed in this run\n\n{}\n", render_list(&fixed));
    out
}

/// The `## Contradictions` and `## Unexamined targets` sections: the second
/// names each rule left unjudged beside the breakdown that counts them.
fn render_rule_sections(by_reason: &BTreeMap<String, u32>, rules: &RuleFields) -> String {
    let mut unexamined_section = render_unexamined(by_reason);
    if !rules.unjudged.is_empty() {
        let _ = write!(
            unexamined_section,
            "\n\nRules left unjudged: {}.",
            rules
                .unjudged
                .iter()
                .map(|id| plain(id))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    format!(
        "## Contradictions\n\n{}\n\n## Unexamined targets\n\n{unexamined_section}\n\n",
        render_contradictions(&rules.contradictions)
    )
}

/// Each rule whose verdict turned from quiet or addressed to a finding over
/// unchanged subjects, or `*None.*`. A note, never a finding: the finding is
/// recorded in its own tier (spec 082, AC3).
fn render_contradictions(contradictions: &[RuleContradiction]) -> String {
    if contradictions.is_empty() {
        return "*None.*".to_string();
    }
    contradictions
        .iter()
        .map(|c| {
            format!(
                "- {} — judged a finding over unchanged subjects, against an earlier verdict of {}",
                plain(&c.rule),
                plain(&c.earlier)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// The rule-assessment fields, each omitted when empty. A verdict is a flow
/// mapping whose text values are always double-quoted, since a plain scalar
/// that block context accepts would end at a `,` or `}` here (spec 082).
fn render_rule_fields(out: &mut String, rules: &RuleFields) {
    if !rules.digest.is_empty() {
        out.push_str("rule-digest:\n");
        for (file, digest) in &rules.digest {
            let _ = writeln!(out, "  {}: {digest}", yaml_string(&single_line(file)));
        }
    }
    if !rules.verdicts.is_empty() {
        out.push_str("rule-verdicts:\n");
        for (id, verdict) in &rules.verdicts {
            let flow = match verdict {
                RuleVerdict::Addressed { cite } => format!(
                    "{{verdict: addressed, cite: {}}}",
                    yaml_quoted(&single_line(cite))
                ),
                RuleVerdict::Waived {
                    reason,
                    decided_by,
                    decided_at,
                    cited,
                } => {
                    let mut flow = format!(
                        "{{verdict: waived, reason: {}, decided-by: {}",
                        yaml_quoted(&single_line(reason)),
                        yaml_quoted(&single_line(decided_by))
                    );
                    if let Some(at) = decided_at {
                        let _ = write!(flow, ", decided-at: {}", yaml_quoted(&single_line(at)));
                    }
                    if let Some(cite) = cited {
                        let _ = write!(flow, ", cited: {}", yaml_quoted(&single_line(cite)));
                    }
                    flow.push('}');
                    flow
                }
                RuleVerdict::Finding { tier, at, message } => format!(
                    "{{verdict: finding, tier: {}, at: {}, message: {}}}",
                    match tier {
                        FindingTier::Must => "must",
                        FindingTier::Should => "should",
                    },
                    yaml_quoted(&single_line(at)),
                    yaml_quoted(&single_line(message))
                ),
            };
            let _ = writeln!(out, "  {}: {flow}", yaml_string(&single_line(id)));
        }
    }
    if !rules.unjudged.is_empty() {
        out.push_str("unjudged-rules:\n");
        for id in &rules.unjudged {
            let _ = writeln!(out, "  - {}", yaml_string(&single_line(id)));
        }
    }
}

/// A tier's section: its live findings, less those `released` to another
/// spec's gate, which render under Deferred to other specs, each beside its
/// disposition, and a line for any the host counted but did not itemize — so
/// the body never understates the frontmatter. `*None.*` when the tier is
/// empty.
fn render_tier(args: &WriteAnalysisArgs, tier: AnalysisTier, released: &[usize]) -> String {
    let in_tier = |finding: &AnalysisFinding| finding.live && finding.tier == tier;
    let listed: Vec<&AnalysisFinding> = args
        .findings
        .iter()
        .enumerate()
        .filter(|(idx, finding)| in_tier(finding) && !released.contains(idx))
        .map(|(_, finding)| finding)
        .collect();
    let moved = released
        .iter()
        .filter(|&&idx| in_tier(&args.findings[idx]))
        .count();
    let count = tier_count(args, tier).saturating_sub(u32::try_from(moved).unwrap_or(u32::MAX));
    if count == 0 && listed.is_empty() {
        return "*None.*".to_string();
    }
    let mut lines: Vec<String> = Vec::new();
    if !listed.is_empty() {
        lines.push(render_list(&listed));
    }
    let missing = count.saturating_sub(u32::try_from(listed.len()).unwrap_or(u32::MAX));
    if missing > 0 {
        lines.push(format!(
            "{missing} finding(s) not itemized — counted as undispositioned."
        ));
    }
    lines.join("\n\n")
}

/// Findings as plain bullets, each beside its disposition, or `*None.*`.
///
/// Every piece of caller text passes through [`plain`], which flattens line
/// breaks and strips a leading checkbox marker: the mechanical half of spec 057
/// AC13, so this report can never become a second triage queue.
fn render_list(findings: &[&AnalysisFinding]) -> String {
    if findings.is_empty() {
        return "*None.*".to_string();
    }
    findings
        .iter()
        .map(|finding| {
            let outcome = decisions::disposition_suffix(&finding.disposition, plain);
            let path = plain(&finding.path);
            let location = if path.is_empty() {
                String::new()
            } else {
                format!(" — `{path}`")
            };
            format!(
                "- {} — {}{location} — {outcome}",
                plain(&finding.family),
                plain(&finding.message)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Caller text flattened to one line with any leading checkbox marker removed.
fn plain(text: &str) -> String {
    let text = single_line(text);
    let text = text.strip_prefix("- ").unwrap_or(&text);
    text.strip_prefix("[ ] ")
        .or_else(|| text.strip_prefix("[x] "))
        .or_else(|| text.strip_prefix("[X] "))
        .unwrap_or(text)
        .trim()
        .to_string()
}

/// The unexamined breakdown, or `*None — every target was examined.*`
///
/// An empty breakdown is stated explicitly rather than left blank: a clean
/// run with nothing skipped and a clean run with something skipped are two
/// results, and an empty section would read as the first while meaning either.
/// A nonzero total always has a breakdown (a bare one is recorded under
/// `unclassified`), so no "total with no breakdown" state is rendered.
fn render_unexamined(by_reason: &BTreeMap<String, u32>) -> String {
    if by_reason.is_empty() {
        return "*None — every target was examined.*".to_string();
    }
    by_reason
        .iter()
        .map(|(reason, count)| format!("- {}: {count}", plain(reason)))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Collapse any line break or control character in a host-supplied scalar to
/// a space — the shared [`flatten_line`] normalization.
///
/// `write-review` rejects such a value outright; this one flattens instead,
/// because the record is written on every run and must not be refused over a
/// stray character: the timestamp and sha are machine-generated, and a
/// finding's family and message become a stored decision's key, where a raw
/// control character would leave the whole record unreadable (spec 058).
/// There is no user intent to preserve, only an injection to defuse. The same
/// normalization keys a fired finding in `process-decisions`, so a stored key
/// and the key a host fires for the same finding agree.
fn single_line(value: &str) -> String {
    flatten_line(value)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::schema::primitives::DecisionOutcome;
    use std::fs;
    use tempfile::TempDir;

    fn spec_repo(frontmatter: &str) -> TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("specs/042-demo");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("spec.md"),
            format!("---\n{frontmatter}\n---\n\n# 042 — Demo\n\n## Behavior\n\nText.\n"),
        )
        .unwrap();
        tmp
    }

    /// An analysis records once implementation is complete, so a box still open in
    /// `tasks.md` refuses the write, and a file with every box ticked does not.
    #[test]
    fn an_analysis_records_only_once_every_task_is_done() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let tasks = tmp.path().join("specs/042-demo/tasks.md");
        fs::write(&tasks, "# T\n\n## 1. A\n\n- [ ] a\n\n- **Done when**: a.\n").unwrap();
        let error = run(&args(), tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "feature"),
            "{error}"
        );
        assert!(!tmp.path().join("specs/042-demo/analysis.md").exists());
        fs::write(&tasks, "# T\n\n## 1. A\n\n- [x] a\n\n- **Done when**: a.\n").unwrap();
        run(&args(), tmp.path()).unwrap();
    }

    /// A box under the task this run's own deferral names is the work the record holds, while
    /// any other open box still refuses: write-analysis's half of AC28's exception (spec 079).
    #[test]
    fn an_analysis_records_over_its_own_deferrals_task_and_no_other() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let tasks = tmp.path().join("specs/042-demo/tasks.md");
        let deferred = "# T\n\n## 1. A\n\n- [x] a\n\n- **Done when**: a.\n\n## 2. Fix it\n\n- [ ] fix\n\n- **Done when**: fixed.\n";
        fs::write(&tasks, deferred).unwrap();
        let mut a = args();
        a.blocking_findings = 1;
        a.decided_by = Some("dev@example.com".into());
        let mut own = finding(
            AnalysisTier::Blocking,
            "task-consistency",
            "task 2 is owed",
            DispositionOutcome::Deferred,
            Some("specs/042-demo/tasks.md"),
        );
        own.disposition.task = Some("2".into());
        a.findings = vec![own];
        run(&a, tmp.path()).unwrap();

        fs::write(
            &tasks,
            format!("{deferred}\n## 3. Other\n\n- [ ] other\n\n- **Done when**: done.\n"),
        )
        .unwrap();
        let error = run(&a, tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, reason, .. } if argument == "feature" && reason.contains("other") && !reason.contains("fix")),
            "{error}"
        );
    }

    /// `analysis.md`'s text, after a run.
    fn analysis_md(tmp: &TempDir) -> String {
        fs::read_to_string(tmp.path().join("specs/042-demo/analysis.md")).unwrap()
    }

    fn analysis_exists(tmp: &TempDir) -> bool {
        tmp.path().join("specs/042-demo/analysis.md").exists()
    }

    /// The artifact is written on a run with nothing whatsoever to report.
    ///
    /// This is the load-bearing case, not the trivial one: a later gate reads
    /// the file's *absence* as never-analyzed, so a clean run that skipped the
    /// write would make "no analysis" and "a clean analysis" identical on disk
    /// (spec 057 AC19).
    #[test]
    fn a_clean_run_still_writes_the_artifact() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        run(&args(), tmp.path()).unwrap();

        let report = analysis_md(&tmp);
        let (fm, body) =
            crate::primitives::split_frontmatter(&report, Path::new("analysis.md")).unwrap();
        let record: crate::schema::primitives::AnalyzeBlock =
            serde_norway::from_str(fm).expect("frontmatter deserializes into the record");

        assert_eq!(record.last_run.as_deref(), Some("2026-09-05T18:00:00Z"));
        assert!(!record.blocking);
        assert!(
            body.contains("*None — every target was examined.*"),
            "a clean run says so rather than leaving the section blank: {body}"
        );
    }

    /// Every section of the skeleton is present on every run, in order.
    #[test]
    fn the_skeleton_is_fixed_and_complete() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        run(&args(), tmp.path()).unwrap();
        let report = analysis_md(&tmp);

        let expected = [
            "# Analysis — 042-demo",
            "## Summary",
            "## Hard failures",
            "## Blocking findings",
            "## Advisory findings",
            "## Contradictions",
            "## Unexamined targets",
            "## Fixed in this run",
        ];
        let mut cursor = 0;
        for heading in expected {
            let found = report[cursor..]
                .find(heading)
                .unwrap_or_else(|| panic!("{heading} missing or out of order in:\n{report}"));
            cursor += found + heading.len();
        }
    }

    /// No section carries a checkbox — the mechanical half of AC13 — even when
    /// a caller's finding text arrives in the inbox's old `- [ ] …` form.
    #[test]
    fn the_report_carries_no_checkbox_even_when_finding_text_arrives_with_one() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.advisory = 1;
        a.findings = vec![finding(
            AnalysisTier::Advisory,
            "- [ ] criterion-path-existence",
            "specs/system.md missing",
            DispositionOutcome::Undispositioned,
            None,
        )];
        run(&a, tmp.path()).unwrap();

        let report = analysis_md(&tmp);
        assert!(
            !report.contains("- [ ]") && !report.contains("- [x]"),
            "analysis.md must never become a second triage queue:\n{report}"
        );
        assert!(
            report.contains("- criterion-path-existence — specs/system.md missing"),
            "the finding's text still lands, just without the marker:\n{report}"
        );
    }

    /// An unexamined breakdown reaches the report, not just the record.
    #[test]
    fn unexamined_reasons_are_rendered() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.unexamined_by_reason = vec![("no-readable-state".into(), 3)];
        run(&a, tmp.path()).unwrap();

        let report = analysis_md(&tmp);
        assert!(report.contains("- no-readable-state: 3"), "{report}");
    }

    /// Spec 076: a work-item ID round-trips through `analysis.md` exactly as
    /// given, with no target beside it.
    #[test]
    fn a_work_item_id_round_trips_byte_identical() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.advisory = 1;
        let mut deferred = finding(
            AnalysisTier::Advisory,
            "grounding",
            "cites a gone path",
            DispositionOutcome::Deferred,
            None,
        );
        deferred.disposition.work_item = Some(" PROJ-12\t(x) ".into());
        a.findings = vec![deferred];
        a.decided_by = Some("dev@example.com".into());
        run(&a, tmp.path()).unwrap();
        let stored =
            decisions::read_decisions(&tmp.path().join("specs/042-demo"), "analysis.md").unwrap();
        assert_eq!(stored[0].work_item.as_deref(), Some(" PROJ-12\t(x) "));
        assert_eq!(stored[0].target, None);
    }

    /// Spec 076: a deferral to work that does not exist is refused before
    /// anything is written.
    #[test]
    fn a_deferral_to_missing_work_is_refused_before_any_write() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.advisory = 1;
        a.findings = vec![finding(
            AnalysisTier::Advisory,
            "grounding",
            "cites a gone path",
            DispositionOutcome::Deferred,
            Some("specs/042-demo/scenarios/gone.md"),
        )];
        a.decided_by = Some("dev@example.com".into());
        let err = run(&a, tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidArgument { argument, .. }
                if argument == "findings[0].disposition.target"),
            "{err:?}"
        );
        assert!(!analysis_exists(&tmp));
    }

    /// Spec 076: a re-deferral after a lapse is a new deferral, so its work is
    /// checked against the tree like any other, not waved through because a
    /// deferral for the key is still on disk until this write drops it.
    #[test]
    fn a_re_deferral_after_a_lapse_is_checked_against_the_tree() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.advisory = 1;
        a.decided_by = Some("dev@example.com".into());
        a.findings = vec![finding(
            AnalysisTier::Advisory,
            "grounding",
            "cites a gone path",
            DispositionOutcome::Deferred,
            Some("specs/042-demo/spec.md"),
        )];
        run(&a, tmp.path()).unwrap();

        let mut next = a.clone();
        next.expired_decisions = vec![DecisionRef {
            key: "grounding — cites a gone path".into(),
            outcome: DecisionOutcome::Deferred,
            target: Some("specs/042-demo/spec.md".into()),
            task: None,
            work_item: None,
            reason: None,
            by_rule: false,
        }];
        next.findings[0].disposition.target = Some("specs/042-demo/scenarios/gone.md".into());
        let err = run(&next, tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidArgument { argument, .. }
                if argument == "findings[0].disposition.target"),
            "{err:?}"
        );
    }

    /// Spec 076: a blocking finding its deferral releases — here, to an
    /// in-progress spec — leaves the gating count and renders under Deferred
    /// to other specs; one deferred to a work item alone stays counted and
    /// marks its decision gating.
    #[test]
    fn a_deferral_releases_a_gating_finding_only_to_another_spec() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let other = tmp.path().join("specs/043-other");
        fs::create_dir_all(&other).unwrap();
        fs::write(
            other.join("spec.md"),
            "---\nstatus: in-progress\n---\n# 043\n",
        )
        .unwrap();
        let mut a = args();
        a.blocking_findings = 2;
        a.decided_by = Some("dev@example.com".into());
        let mut held = finding(
            AnalysisTier::Blocking,
            "applicable-rules",
            "BE-X fails",
            DispositionOutcome::Deferred,
            None,
        );
        held.disposition.work_item = Some("PROJ-3".into());
        a.findings = vec![
            finding(
                AnalysisTier::Blocking,
                "grounding",
                "cites a gone path",
                DispositionOutcome::Deferred,
                Some("specs/043-other/spec.md"),
            ),
            held,
        ];
        assert!(
            run(&a, tmp.path()).unwrap().blocking,
            "the work-item deferral still blocks"
        );
        let report = analysis_md(&tmp);
        assert!(report.contains("blocking-findings: 1\n"), "{report}");
        let deferred_section = report.split("## Deferred to other specs").nth(1).unwrap();
        assert!(
            deferred_section
                .trim_start()
                .starts_with("- grounding — cites a gone path"),
            "{report}"
        );
        let stored =
            decisions::read_decisions(&tmp.path().join("specs/042-demo"), "analysis.md").unwrap();
        let gating = |key: &str| {
            stored
                .iter()
                .find(|d| d.key.as_deref() == Some(key))
                .map(|d| d.gating)
        };
        assert_eq!(gating("grounding — cites a gone path"), Some(None));
        assert_eq!(gating("applicable-rules — BE-X fails"), Some(Some(true)));
        a.findings.truncate(1);
        a.blocking_findings = 1;
        assert!(
            !run(&a, tmp.path()).unwrap().blocking,
            "the release alone does not block"
        );
    }

    /// Spec 076: a deferral releases a gating finding only to work still to
    /// do on another spec, whose gate then holds it. A `done` spec's gate holds
    /// nothing, and a rule file has no gate, so a deferral to either stays
    /// counted and gating.
    #[test]
    fn a_deferral_to_complete_or_non_spec_work_releases_nothing() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let done = tmp.path().join("specs/043-done");
        fs::create_dir_all(&done).unwrap();
        fs::write(done.join("spec.md"), "---\nstatus: done\n---\n# 043\n").unwrap();
        let rules = tmp.path().join("specs/rules");
        fs::create_dir_all(&rules).unwrap();
        fs::write(rules.join("x-cross.md"), "# X\n").unwrap();
        let mut a = args();
        a.blocking_findings = 2;
        a.decided_by = Some("dev@example.com".into());
        a.findings = vec![
            finding(
                AnalysisTier::Blocking,
                "grounding",
                "cites a gone path",
                DispositionOutcome::Deferred,
                Some("specs/043-done/spec.md"),
            ),
            finding(
                AnalysisTier::Blocking,
                "applicable-rules",
                "BE-X fails",
                DispositionOutcome::Deferred,
                Some("specs/rules/x-cross.md"),
            ),
        ];
        assert!(run(&a, tmp.path()).unwrap().blocking);
        let report = analysis_md(&tmp);
        assert!(report.contains("blocking-findings: 2\n"), "{report}");
        let stored =
            decisions::read_decisions(&tmp.path().join("specs/042-demo"), "analysis.md").unwrap();
        assert!(stored.iter().all(|d| d.gating == Some(true)), "{stored:?}");
    }

    fn args() -> WriteAnalysisArgs {
        WriteAnalysisArgs {
            feature: "042-demo".into(),
            analyzed_at: "2026-09-05T18:00:00Z".into(),
            analyzed_against: "abc123".into(),
            hard_fail: 0,
            blocking_findings: 0,
            advisory: 0,
            unexamined: 0,
            unexamined_by_reason: vec![],
            findings: vec![],
            expired_decisions: vec![],
            decided_by: None,
            ..WriteAnalysisArgs::default()
        }
    }

    fn finding(
        tier: AnalysisTier,
        family: &str,
        message: &str,
        outcome: DispositionOutcome,
        companion: Option<&str>,
    ) -> AnalysisFinding {
        AnalysisFinding {
            tier,
            family: family.into(),
            message: message.into(),
            path: "spec.md".into(),
            live: outcome != DispositionOutcome::Fixed,
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
            decision_key: None,
        }
    }

    #[test]
    fn a_call_with_counts_and_no_findings_records_every_live_finding_as_owed() {
        // The exec path's shape: tier counts from the re-check, nothing
        // itemized, because there was no operator to disposition anything.
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.blocking_findings = 1;
        a.advisory = 3;
        let result = run(&a, tmp.path()).unwrap();
        assert_eq!(result.dispositions.undispositioned, 4);
        let report = analysis_md(&tmp);
        assert!(report.contains("  undispositioned: 4\n"), "{report}");
        assert!(
            report.contains("3 finding(s) not itemized — counted as undispositioned."),
            "the body must not understate what the frontmatter counts:\n{report}"
        );
        assert!(!report.contains("captured-issues"), "{report}");
        assert!(
            report.contains("An undispositioned advisory finding does not hold `done`"),
            "{report}"
        );
    }

    #[test]
    fn each_disposition_is_counted_and_rendered_in_its_section() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        // A new deferral names work that exists (spec 076).
        fs::write(tmp.path().join("specs/042-demo/tasks.md"), "# Tasks\n").unwrap();
        let mut a = args();
        a.blocking_findings = 1;
        a.advisory = 2;
        a.decided_by = Some("dev@example.com".into());
        a.findings = vec![
            finding(
                AnalysisTier::Advisory,
                "grounding",
                "cites a gone path",
                DispositionOutcome::Fixed,
                None,
            ),
            finding(
                AnalysisTier::Blocking,
                "task-consistency",
                "task 4 has no Done when",
                DispositionOutcome::Deferred,
                Some("specs/042-demo/tasks.md"),
            ),
            finding(
                AnalysisTier::Advisory,
                "applicable-rules",
                "BE-AUTHN-001 does not fire",
                DispositionOutcome::Waived,
                Some("cited for a future endpoint"),
            ),
            finding(
                AnalysisTier::Advisory,
                "decision-drift",
                "prose asserts an open state",
                DispositionOutcome::Undispositioned,
                None,
            ),
        ];
        let result = run(&a, tmp.path()).unwrap();
        assert_eq!(
            result.dispositions,
            Dispositions {
                fixed: 1,
                deferred: 1,
                waived: 1,
                undispositioned: 1
            }
        );
        let report = analysis_md(&tmp);
        assert!(report.contains("- task-consistency — task 4 has no Done when — `spec.md` — **deferred** to `specs/042-demo/tasks.md`"), "{report}");
        assert!(report.contains("- applicable-rules — BE-AUTHN-001 does not fire — `spec.md` — **waived**: cited for a future endpoint"), "{report}");
        assert!(
            report.contains(
                "## Fixed in this run\n\n- grounding — cites a gone path — `spec.md` — **fixed**"
            ),
            "{report}"
        );
    }

    #[test]
    fn a_waiver_on_a_gating_finding_is_refused_with_nothing_written() {
        for tier in [AnalysisTier::HardFail, AnalysisTier::Blocking] {
            let tmp = spec_repo("status: in-progress\ndependencies: []");
            let mut a = args();
            a.hard_fail = 1;
            a.blocking_findings = 1;
            a.decided_by = Some("dev@example.com".into());
            a.findings = vec![finding(
                tier,
                "f",
                "m",
                DispositionOutcome::Waived,
                Some("why"),
            )];
            let error = run(&a, tmp.path()).unwrap_err();
            assert!(
                error
                    .to_string()
                    .contains("findings[0].disposition.outcome"),
                "{error}"
            );
            assert!(!analysis_exists(&tmp), "nothing is written on a refusal");
        }
    }

    #[test]
    fn a_self_contradicting_itemization_is_refused() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut still_firing = finding(
            AnalysisTier::Advisory,
            "f",
            "m",
            DispositionOutcome::Fixed,
            None,
        );
        still_firing.live = true;
        let mut gone_but_deferred = finding(
            AnalysisTier::Advisory,
            "f",
            "m",
            DispositionOutcome::Deferred,
            Some("t"),
        );
        gone_but_deferred.live = false;
        for bad in [still_firing, gone_but_deferred] {
            let mut a = args();
            a.advisory = 1;
            a.findings = vec![bad];
            assert!(run(&a, tmp.path()).is_err());
        }
        let mut over = args();
        over.advisory = 0;
        over.findings = vec![finding(
            AnalysisTier::Advisory,
            "f",
            "m",
            DispositionOutcome::Undispositioned,
            None,
        )];
        assert!(
            run(&over, tmp.path()).is_err(),
            "more itemized than counted"
        );
        assert!(!analysis_exists(&tmp));
    }

    #[test]
    fn a_decision_is_stored_keyed_on_family_and_message_and_pruned_once_expired() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.advisory = 1;
        a.decided_by = Some("dev@example.com".into());
        a.findings = vec![finding(
            AnalysisTier::Advisory,
            "applicable-rules",
            "BE-AUTHN-001 does not fire",
            DispositionOutcome::Waived,
            Some("future endpoint"),
        )];
        run(&a, tmp.path()).unwrap();
        let dir = tmp.path().join("specs/042-demo");
        let stored = decisions::read_decisions(&dir, "analysis.md").unwrap();
        assert_eq!(stored.len(), 1);
        assert_eq!(
            stored[0].key.as_deref(),
            Some("applicable-rules — BE-AUTHN-001 does not fire")
        );

        // The finding stops firing: process-decisions reports the entry
        // expired, and the next write drops it.
        let mut next = args();
        next.expired_decisions = vec![DecisionRef {
            key: "applicable-rules — BE-AUTHN-001 does not fire".into(),
            outcome: DecisionOutcome::Waived,
            target: None,
            task: None,
            work_item: None,
            reason: Some("future endpoint".into()),
            by_rule: false,
        }];
        run(&next, tmp.path()).unwrap();
        assert_eq!(
            decisions::read_decisions(&dir, "analysis.md").unwrap(),
            [] as [crate::primitives::decisions::RawDecision; 0]
        );
    }

    /// A first run creates the artifact, and leaves `spec.md` alone.
    ///
    /// Was `inserts_the_block_when_absent`: the record used to be spliced into
    /// spec frontmatter, so the contract was "insert without disturbing the
    /// neighbours". With one home per record there are no neighbours, and the
    /// contract worth pinning is that the spec is not touched at all.
    #[test]
    fn a_first_run_writes_the_artifact_and_leaves_the_spec_untouched() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let before = fs::read_to_string(tmp.path().join("specs/042-demo/spec.md")).unwrap();

        let result = run(&args(), tmp.path()).unwrap();
        assert!(!result.replaced);
        assert!(!result.blocking);

        let report = analysis_md(&tmp);
        assert!(
            report.contains("last-run: 2026-09-05T18:00:00Z"),
            "{report}"
        );
        assert!(report.contains("blocking: false"), "{report}");

        let after = fs::read_to_string(tmp.path().join("specs/042-demo/spec.md")).unwrap();
        assert_eq!(before, after, "the spec is not a write target any more");
    }

    /// A re-run overwrites its own artifact whole, and the review record beside
    /// it is untouched.
    ///
    /// The sibling-preservation contract survives the relocation; what changed
    /// is that the sibling is a *file* rather than an adjacent frontmatter
    /// block, which makes it harder to damage rather than easier.
    #[test]
    fn a_rerun_overwrites_its_own_record_and_leaves_the_review_record_alone() {
        let tmp = spec_repo("status: done\ndependencies: []\nnext-criterion: 7");
        let dir = tmp.path().join("specs/042-demo");
        fs::write(
            dir.join("analysis.md"),
            "---\nspec: 042-demo\nlast-run: 2019-01-01T00:00:00Z\nblocking: true\n---\n\n# Analysis\n",
        )
        .unwrap();
        let review = "---\nspec: 042-demo\nlast-run: 2020-01-01T00:00:00Z\nblocking: false\n---\n\n# Review\n";
        fs::write(dir.join("review.md"), review).unwrap();

        let result = run(&args(), tmp.path()).unwrap();
        assert!(result.replaced, "an analysis.md already existed");

        let report = analysis_md(&tmp);
        assert!(!report.contains("2019-01-01T00:00:00Z"), "{report}");
        assert!(
            report.contains("last-run: 2026-09-05T18:00:00Z"),
            "{report}"
        );
        assert_eq!(
            fs::read_to_string(dir.join("review.md")).unwrap(),
            review,
            "the review record is not this command's to write"
        );
        assert!(
            fs::read_to_string(dir.join("spec.md"))
                .unwrap()
                .contains("next-criterion: 7")
        );
    }

    #[test]
    fn a_reason_or_scalar_that_would_not_read_back_is_quoted() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut hostile = args();
        hostile.analyzed_against = "abc status: done".into();
        hostile.unexamined_by_reason = vec![
            ("root-absent\nblocking: false".into(), 1),
            ("a: b".into(), 2),
            ("# not a comment".into(), 1),
        ];
        run(&hostile, tmp.path()).unwrap();
        let report = analysis_md(&tmp);
        let (fm, _) = split_frontmatter(&report, Path::new("analysis.md")).unwrap();
        let parsed: serde_norway::Value =
            serde_norway::from_str(fm).expect("the record reads back");
        assert_eq!(parsed["analyzed-against"], "abc status: done");
        assert_eq!(parsed["blocking"], false);
        let reasons = &parsed["unexamined-by-reason"];
        assert_eq!(reasons["root-absent blocking: false"], 1, "{fm}");
        assert_eq!(reasons["a: b"], 2, "{fm}");
        assert_eq!(reasons["# not a comment"], 1, "{fm}");
        // The newline injected no key of its own.
        assert_eq!(fm.matches("blocking: false").count(), 2, "{fm}");
        // And the next run reads the record it left.
        run(&args(), tmp.path()).unwrap();
    }

    #[test]
    fn blocking_is_set_by_either_gating_tier() {
        for (hard, blocking_findings) in [(1, 0), (0, 1), (2, 3)] {
            let tmp = spec_repo("status: in-progress\ndependencies: []");
            let result = run(
                &WriteAnalysisArgs {
                    hard_fail: hard,
                    blocking_findings,
                    ..args()
                },
                tmp.path(),
            )
            .unwrap();
            assert!(result.blocking, "hard={hard} blocking={blocking_findings}");
        }
    }

    /// Advisory findings are recorded and never gate — the asymmetry with the
    /// review block is the design, not an omission.
    #[test]
    fn advisory_findings_are_recorded_but_do_not_block() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(
            &WriteAnalysisArgs {
                advisory: 9,
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        assert!(!result.blocking);
        let spec = analysis_md(&tmp);
        assert!(spec.contains("advisory: 9"));
        assert!(spec.contains("blocking: false"));
    }

    /// A bare total answers *that* something was unexamined and nothing
    /// about what — and the reasons are not equivalent. The breakdown is what
    /// makes the number actionable.
    #[test]
    fn the_breakdown_is_written_and_sorted() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(
            &WriteAnalysisArgs {
                unexamined_by_reason: vec![
                    ("ships-to-adopter".into(), 10),
                    ("not-a-live-claim".into(), 81),
                    ("artifact-unreadable".into(), 1),
                ],
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.unexamined, 92);
        let spec = analysis_md(&tmp);
        assert!(spec.contains("unexamined: 92"));
        // BTreeMap order, so the rendering is byte-stable across runs.
        let block = spec.split("unexamined-by-reason:").nth(1).unwrap();
        let order: Vec<&str> = block
            .lines()
            .skip(1) // the remainder of the `unexamined-by-reason:` line itself
            .take_while(|l| l.starts_with("  "))
            .map(|l| l.trim().split(':').next().unwrap())
            .collect();
        assert_eq!(
            order,
            vec![
                "artifact-unreadable",
                "not-a-live-claim",
                "ships-to-adopter"
            ]
        );
    }

    /// Reasons are folded by the key they render as, so two that flatten to
    /// one line are one entry, never a duplicate key the next reader refuses.
    #[test]
    fn reasons_that_flatten_alike_are_one_entry() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(
            &WriteAnalysisArgs {
                unexamined_by_reason: vec![("a\nb".into(), 1), ("a b".into(), 2)],
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.unexamined, 3);
        let spec = analysis_md(&tmp);
        assert_eq!(spec.matches("  a b:").count(), 1, "{spec}");
        assert!(spec.contains("  a b: 3"), "{spec}");
    }

    /// The breakdown is the authority: a total a caller can contradict is a
    /// total that will eventually be contradicted, which is why `blocking` is
    /// derived too.
    #[test]
    fn a_supplied_total_cannot_contradict_its_breakdown() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(
            &WriteAnalysisArgs {
                unexamined: 999,
                unexamined_by_reason: vec![("root-absent".into(), 4)],
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.unexamined, 4);
        let spec = analysis_md(&tmp);
        assert!(spec.contains("unexamined: 4"));
        // Anchored to the field, not to the bare digits: the block now also
        // carries hex digests, and a bare `999` match can land inside one.
        assert!(!spec.contains("unexamined: 999"));
    }

    /// A fully-examined run carries no map rather than a map of zeroes.
    #[test]
    fn a_fully_examined_run_omits_the_map() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(result.unexamined, 0);
        let spec = analysis_md(&tmp);
        assert!(spec.contains("unexamined: 0"));
        assert!(!spec.contains("unexamined-by-reason"));
        assert!(
            spec.contains("*None — every target was examined.*"),
            "{spec}"
        );
    }

    /// A caller's bare total is recorded under `unclassified`, so `unexamined`
    /// stays the breakdown's sum and the total is never lost (`QUAL-CLAIM-001`).
    #[test]
    fn a_bare_total_is_recorded_as_unclassified() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(
            &WriteAnalysisArgs {
                unexamined: 5,
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.unexamined, 5);
        let record = written_record(&tmp);
        assert_eq!(record.unexamined, 5);
        assert_eq!(
            record.unexamined_by_reason,
            BTreeMap::from([("unclassified".to_string(), 5)])
        );
        assert!(analysis_md(&tmp).contains("- unclassified: 5"));
    }

    /// With derived entries beside it, the bare total is still recorded: 5
    /// bare plus 2 unjudged rules is 7, in two reasons.
    #[test]
    fn a_bare_total_survives_a_derived_reason_beside_it() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_rules(&tmp, 2, 0);
        let result = run(
            &WriteAnalysisArgs {
                unexamined: 5,
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.unexamined, 7);
        let record = written_record(&tmp);
        assert_eq!(record.unexamined, 7);
        assert_eq!(
            record.unexamined_by_reason,
            BTreeMap::from([
                ("rule-assessments-not-checked".to_string(), 2),
                ("unclassified".to_string(), 5),
            ])
        );
    }

    /// The `QUAL-CLAIM-001` field: a clean run that could not examine
    /// everything must not record the same thing as one that could.
    #[test]
    fn unexamined_count_survives_a_clean_run() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        run(
            &WriteAnalysisArgs {
                unexamined: 3,
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        let spec = analysis_md(&tmp);
        assert!(spec.contains("unexamined: 3"));
        assert!(spec.contains("blocking: false"));
    }

    #[test]
    fn newline_in_a_scalar_cannot_inject_frontmatter_keys() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        run(
            &WriteAnalysisArgs {
                analyzed_against: "abc\nstatus: done".into(),
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        let report = analysis_md(&tmp);
        // Flattened to one line, and quoted, so the record reads back with the
        // value intact rather than as a scalar the YAML reader rejects.
        let (fm, _) = split_frontmatter(&report, Path::new("analysis.md")).unwrap();
        let parsed: serde_norway::Value =
            serde_norway::from_str(fm).expect("the record reads back");
        assert_eq!(parsed["analyzed-against"], "abc status: done", "{report}");
        assert!(
            !report.contains("\nstatus: done"),
            "an embedded newline must not become a frontmatter key: {report}"
        );
        // And the spec it was aimed at is untouched.
        assert!(
            fs::read_to_string(tmp.path().join("specs/042-demo/spec.md"))
                .unwrap()
                .contains("status: in-progress")
        );
    }

    #[test]
    fn malformed_frontmatter_is_never_given_a_clean_record() {
        let tmp = spec_repo("status: in-progress\ndependencies: [oops");
        assert!(matches!(
            run(&args(), tmp.path()).unwrap_err(),
            PrimitiveError::Yaml { .. }
        ));
        assert!(
            !analysis_exists(&tmp),
            "a spec that will not parse receives no record at all — the artifact's \
             absence is what a later gate reads as never-analyzed"
        );
    }

    #[test]
    fn missing_feature_is_an_error() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        assert!(matches!(
            run(
                &WriteAnalysisArgs {
                    feature: "999-absent".into(),
                    ..args()
                },
                tmp.path()
            )
            .unwrap_err(),
            PrimitiveError::FeatureNotFound { .. }
        ));
    }

    /// Register one `[constitutions.*]` entry pointing at `path`.
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

    #[test]
    fn an_unresolved_constitution_is_recorded_as_unexamined() {
        // AC8: an analysis that ran without a registered source must not record a
        // clean, fully-examined run. The reason lands in the breakdown the record
        // already carries, so the pre-done gate and any reader see it.
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_constitution(&tmp, "acme", "nowhere");

        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(result.unexamined, 1);
        let spec = analysis_md(&tmp);
        assert!(spec.contains("unexamined: 1"), "{spec}");
        assert!(spec.contains("constitution-unresolved: 1"), "{spec}");
    }

    #[test]
    fn a_resolved_constitution_records_nothing_unexamined() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_constitution(&tmp, "acme", "gov");
        let gov = tmp.path().join("gov");
        fs::create_dir_all(&gov).unwrap();
        fs::write(gov.join("constitution.md"), "# House rules\n").unwrap();

        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(
            result.unexamined, 0,
            "a source that was read is not unexamined"
        );
    }

    #[test]
    fn no_registered_constitution_leaves_the_count_untouched() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(&args(), tmp.path()).unwrap();
        assert_eq!(result.unexamined, 0);
        let spec = analysis_md(&tmp);
        assert!(
            !spec.contains("constitution-unresolved"),
            "a project with none registered reads exactly as before: {spec}"
        );
    }

    #[test]
    fn a_malformed_registry_entry_still_records_the_run() {
        // Regression: the registry read was `?`-propagated, so a malformed
        // `[constitutions]` entry suppressed the analyze record entirely -- and
        // an absent record is exactly what the pre-done gate reads as "never
        // analyzed". A config that parses records the run (spec 055's data
        // model); one that does not halts it, below.
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let cfg = tmp.path().join(".ductus");
        fs::create_dir_all(&cfg).unwrap();
        fs::write(
            cfg.join("config.toml"),
            "[constitutions.acme]\nrepo = \"\"\n",
        )
        .unwrap();

        let result = run(&args(), tmp.path());
        assert!(
            result.is_ok(),
            "a malformed entry must not suppress the record"
        );

        let spec = analysis_md(&tmp);
        assert!(
            spec.contains("constitution-registry-unreadable: 1"),
            "{spec}"
        );
        assert!(spec.contains("unexamined: 1"), "{spec}");
    }

    /// A config that does not parse halts the run naming the file, writing no
    /// record: the spec root it would be written under is unknown (spec 022
    /// task 12, option b; spec 055's data model scopes the recorded run to a
    /// config that parses).
    #[test]
    fn an_unparseable_config_halts_the_run_naming_the_file() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let cfg = tmp.path().join(".ductus");
        fs::create_dir_all(&cfg).unwrap();
        fs::write(cfg.join("config.toml"), "[constitutions.acme]\nrepo = \n").unwrap();

        let err = run(&args(), tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::Toml { path, .. } if path.ends_with(".ductus/config.toml")),
            "{err}"
        );
        assert!(
            !analysis_exists(&tmp),
            "no record is written under a guessed spec root"
        );
    }

    // -- spec 058 review: decision-record hardening -------------------------

    /// A control character in a finding's message or a waiver's reason is
    /// flattened before it is stored, so the record reads back and the stored
    /// key is the one `process-decisions` fires for the same finding. A raw one
    /// would have left the whole record unreadable.
    #[test]
    fn line_hazards_are_flattened_so_the_record_reads_back() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.advisory = 1;
        a.decided_by = Some("dev@example.com".into());
        a.findings = vec![finding(
            AnalysisTier::Advisory,
            "grounding",
            "cites\u{1b}[31m a path\u{2028}twice",
            DispositionOutcome::Waived,
            Some("false\u{1b}positive"),
        )];
        run(&a, tmp.path()).unwrap();
        let dir = tmp.path().join("specs/042-demo");
        let stored = decisions::read_decisions(&dir, "analysis.md").unwrap();
        assert_eq!(stored.len(), 1);
        assert_eq!(
            stored[0].key.as_deref(),
            Some("grounding — cites [31m a path twice")
        );
        assert_eq!(stored[0].reason.as_deref(), Some("false positive"));
        assert!(matches!(
            crate::primitives::load_analyze_record(&dir),
            crate::primitives::RecordLoad::Present(_)
        ));
    }

    /// A decision whose key carries a comment opener or a noncharacter comes
    /// back so the next run can match it: the tab-then-`#` key reads back byte
    /// for byte, and the noncharacter is flattened before it is stored. Either
    /// one written raw left a key that expired the run after it was decided,
    /// or a record the reader refused outright.
    #[test]
    fn a_decision_carrying_a_comment_opener_or_noncharacter_reads_back() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.advisory = 2;
        a.decided_by = Some("dev@example.com".into());
        a.findings = vec![
            finding(
                AnalysisTier::Advisory,
                "grounding",
                "plan.md cites issue\t#12 as closed",
                DispositionOutcome::Waived,
                Some("why\t#not"),
            ),
            finding(
                AnalysisTier::Advisory,
                "grounding",
                "cites a non\u{ffff}char",
                DispositionOutcome::Waived,
                Some("noise"),
            ),
        ];
        run(&a, tmp.path()).unwrap();
        let dir = tmp.path().join("specs/042-demo");
        let stored = decisions::read_decisions(&dir, "analysis.md").unwrap();
        let keys: Vec<_> = stored.iter().filter_map(|d| d.key.as_deref()).collect();
        assert_eq!(
            keys,
            [
                "grounding — plan.md cites issue\t#12 as closed",
                "grounding — cites a non char"
            ]
        );
        assert_eq!(stored[0].reason.as_deref(), Some("why\t#not"));
        let classified = crate::primitives::process_decisions::compute(
            &crate::schema::primitives::ProcessDecisionsArgs {
                feature: "042-demo".into(),
                record: crate::schema::primitives::DecisionRecord::Analysis,
                fired: keys.iter().map(|k| (*k).to_string()).collect(),
                restricted: false,
                since: None,
                fired_json: None,
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(classified.matched.len(), 2);
        assert_eq!(classified.expired, [] as [DecisionRef; 0]);
    }

    /// A `decision-key` the host matched to a decision that is not stored — a
    /// stale match, or one from another record — is not a match: the finding
    /// is a new decision, stored under that key, and needs its author like
    /// any other.
    #[test]
    fn a_decision_key_naming_no_stored_decision_is_a_new_decision() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.advisory = 1;
        let mut matched = finding(
            AnalysisTier::Advisory,
            "grounding",
            "plan.md cites a path, reworded",
            DispositionOutcome::Waived,
            Some("noise"),
        );
        matched.decision_key = Some("grounding — plan.md cites a path".into());
        a.findings = vec![matched];
        assert!(matches!(
            run(&a, tmp.path()),
            Err(PrimitiveError::InvalidArgument { argument, .. }) if argument == "decided-by"
        ));

        a.decided_by = Some("dev@example.com".into());
        run(&a, tmp.path()).unwrap();
        let stored =
            decisions::read_decisions(&tmp.path().join("specs/042-demo"), "analysis.md").unwrap();
        assert_eq!(stored.len(), 1);
        assert_eq!(
            stored[0].key.as_deref(),
            Some("grounding — plan.md cites a path")
        );
        assert_eq!(stored[0].decided_by.as_deref(), Some("dev@example.com"));
    }

    /// A prior `analysis.md` that opens no frontmatter block stores no
    /// decisions, so it is overwritten: re-running the command repairs it.
    #[test]
    fn a_prior_record_with_no_frontmatter_is_overwritten() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let path = tmp.path().join("specs/042-demo/analysis.md");
        fs::write(&path, "# Analysis — 042-demo\n\nhand-edited, no record\n").unwrap();
        run(&args(), tmp.path()).unwrap();
        assert!(fs::read_to_string(&path).unwrap().starts_with("---\n"));
    }

    /// A prior `analysis.md` whose frontmatter does not parse is refused and
    /// left as it is: the decisions it stores cannot be read, and writing over
    /// it would drop them unseen.
    #[test]
    fn a_prior_record_whose_frontmatter_does_not_parse_is_refused() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let path = tmp.path().join("specs/042-demo/analysis.md");
        let damaged = "---\nspec: \"042-demo\nlast-run: x\n---\n\n# Analysis\n";
        fs::write(&path, damaged).unwrap();
        assert!(matches!(
            run(&args(), tmp.path()),
            Err(PrimitiveError::Yaml { .. })
        ));
        assert_eq!(fs::read_to_string(&path).unwrap(), damaged);
    }

    /// A prior `analysis.md` whose frontmatter opens and never closes is
    /// refused, named as unclosed, and left as it is: the decisions it may
    /// store cannot be read, and writing over it would drop them unseen.
    #[test]
    fn a_prior_record_whose_frontmatter_never_closes_is_refused() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let path = tmp.path().join("specs/042-demo/analysis.md");
        let damaged = "---\nspec: 042-demo\ndecisions:\n  - key: \"f — m\"\n    \
                       outcome: waived\n    reason: noise\n\n# Analysis\n";
        fs::write(&path, damaged).unwrap();
        let error = run(&args(), tmp.path()).unwrap_err();
        assert!(error.to_string().contains("never closes"), "{error}");
        assert!(matches!(error, PrimitiveError::UnclosedFrontmatter { .. }));
        assert_eq!(fs::read_to_string(&path).unwrap(), damaged);
    }

    /// A closing `---` on the file's last line, with no newline after it,
    /// closes the block: the prior record's decisions are read and kept.
    #[test]
    fn a_prior_record_closed_on_its_last_line_keeps_its_decisions() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let dir = tmp.path().join("specs/042-demo");
        fs::write(
            dir.join("analysis.md"),
            "---\nspec: 042-demo\ndecisions:\n  - key: \"f — m\"\n    outcome: waived\n    \
             reason: noise\n    decided-at: 2026-09-01T00:00:00Z\n    decided-by: a@example.com\n---",
        )
        .unwrap();
        run(&args(), tmp.path()).unwrap();
        let stored = decisions::read_decisions(&dir, "analysis.md").unwrap();
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].key.as_deref(), Some("f — m"));
    }

    /// A deferral names its work and a waiver states its reason, refused by
    /// the check `write-review` shares, under this primitive's argument path.
    #[test]
    fn a_disposition_without_its_companion_is_refused() {
        for (outcome, argument, reason) in [
            (
                DispositionOutcome::Deferred,
                "findings[0].disposition.target",
                "a deferral names its work: a target, a work item, or both",
            ),
            (
                DispositionOutcome::Waived,
                "findings[0].disposition.reason",
                "a waiver states its reason",
            ),
        ] {
            let tmp = spec_repo("status: in-progress\ndependencies: []");
            let mut a = args();
            a.advisory = 1;
            a.decided_by = Some("dev@example.com".into());
            a.findings = vec![finding(
                AnalysisTier::Advisory,
                "f",
                "m",
                outcome,
                Some("  "),
            )];
            let error = run(&a, tmp.path()).unwrap_err();
            assert!(
                matches!(
                    &error,
                    PrimitiveError::InvalidArgument { primitive, argument: at, reason: why }
                        if primitive == "write-analysis" && at == argument && why == reason
                ),
                "{error:?}"
            );
            assert!(!analysis_exists(&tmp));
        }
    }

    /// One key, one disposition, whatever the two outcomes are — a finding
    /// fixed beside one still firing under the same key included.
    #[test]
    fn same_key_findings_with_any_two_outcomes_are_refused() {
        use DispositionOutcome::{Deferred, Fixed, Undispositioned, Waived};
        for (first, second) in [
            (Deferred, Undispositioned),
            (Fixed, Undispositioned),
            (Fixed, Waived),
        ] {
            let tmp = spec_repo("status: in-progress\ndependencies: []");
            let mut a = args();
            a.advisory = 2;
            a.decided_by = Some("dev@example.com".into());
            a.findings = [first, second]
                .into_iter()
                .map(|outcome| {
                    let companion = match outcome {
                        Deferred => Some("specs/042-demo/tasks.md"),
                        Waived => Some("noise"),
                        Fixed | Undispositioned => None,
                    };
                    finding(
                        AnalysisTier::Advisory,
                        "grounding",
                        "same",
                        outcome,
                        companion,
                    )
                })
                .collect();
            let error = run(&a, tmp.path()).unwrap_err();
            assert!(
                matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "findings"),
                "{first:?} beside {second:?}: {error}"
            );
            assert!(!analysis_exists(&tmp));
        }
    }

    /// Two findings sharing a key and an outcome are one decision, stored
    /// once, and two findings counted: the tier counts count each, and
    /// `undispositioned` is derived from them, so counting the pair once
    /// would record one of them as owed.
    #[test]
    fn same_key_findings_sharing_an_outcome_are_each_counted_and_stored_once() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.advisory = 2;
        a.decided_by = Some("dev@example.com".into());
        let twice = finding(
            AnalysisTier::Advisory,
            "grounding",
            "same",
            DispositionOutcome::Waived,
            Some("noise"),
        );
        a.findings = vec![twice.clone(), twice];
        let result = run(&a, tmp.path()).unwrap();
        assert_eq!(result.dispositions.waived, 2);
        assert_eq!(result.dispositions.undispositioned, 0);
        let stored =
            decisions::read_decisions(&tmp.path().join("specs/042-demo"), "analysis.md").unwrap();
        assert_eq!(stored.len(), 1);
    }

    /// A blank timestamp would stamp every new decision with a blank
    /// `decided-at`, which the reader then calls malformed.
    #[test]
    fn a_blank_analyzed_at_is_refused() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.analyzed_at = " \n ".into();
        assert!(matches!(
            run(&a, tmp.path()),
            Err(PrimitiveError::InvalidArgument { argument, .. }) if argument == "analyzed-at"
        ));
        assert!(!analysis_exists(&tmp));
    }

    /// `tier` is required: the waiver refusal reads it, so a finding that
    /// omits it must not bind as the permissive advisory tier.
    #[test]
    fn a_finding_without_a_tier_does_not_bind() {
        let parsed = serde_json::from_value::<AnalysisFinding>(serde_json::json!({
            "family": "grounding",
            "message": "m",
            "disposition": { "outcome": "waived", "reason": "noise" },
        }));
        assert!(parsed.is_err(), "an untagged finding must be refused");
    }

    /// A host-worded message drifts between runs, so the host matches a new
    /// finding to the stored decision describing the same issue and passes its
    /// key. The finding is stored — and re-matched — under that key, keeping
    /// the original stamp, rather than asked about again under new wording
    /// (scenario `analyze-findings-match-decisions-by-host-judgment`).
    #[test]
    fn a_matched_decision_key_keys_the_finding() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut first = args();
        first.advisory = 1;
        first.decided_by = Some("first@example.com".into());
        first.findings = vec![finding(
            AnalysisTier::Advisory,
            "grounding",
            "plan.md asserts the cache is warm without a source",
            DispositionOutcome::Waived,
            Some("the plan names the source two lines down"),
        )];
        run(&first, tmp.path()).unwrap();
        let dir = tmp.path().join("specs/042-demo");
        let stored = decisions::read_decisions(&dir, "analysis.md").unwrap();
        let original = stored[0].key.clone().unwrap();

        let mut second = args();
        second.analyzed_at = "2026-09-26T00:00:00Z".into();
        second.advisory = 1;
        let mut reworded = finding(
            AnalysisTier::Advisory,
            "grounding",
            "plan.md claims a warm cache with no citation",
            DispositionOutcome::Waived,
            Some("the plan names the source two lines down"),
        );
        reworded.decision_key = Some(original.clone());
        second.findings = vec![reworded];
        run(&second, tmp.path()).unwrap();
        let stored = decisions::read_decisions(&dir, "analysis.md").unwrap();
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].key.as_deref(), Some(original.as_str()));
        assert_eq!(
            stored[0].decided_by.as_deref(),
            Some("first@example.com"),
            "a re-matched decision keeps its original stamp"
        );
    }

    /// Spec 079: analyze's rule-decided waiver needs no author and says whose it
    /// is; a person's stays theirs.
    #[test]
    fn a_by_rule_waiver_of_an_advisory_finding_records_decided_by_rule() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.advisory = 2;
        let mut ruled = finding(
            AnalysisTier::Advisory,
            "grounding",
            "cites a gone path",
            DispositionOutcome::Waived,
            Some("out of scope"),
        );
        ruled.disposition.by_rule = true;
        let person = finding(
            AnalysisTier::Advisory,
            "grounding",
            "a person looked",
            DispositionOutcome::Waived,
            Some("not ours"),
        );
        a.findings = vec![ruled];
        a.decided_by = None;
        run(&a, tmp.path()).unwrap();
        let stored =
            decisions::read_decisions(&tmp.path().join("specs/042-demo"), "analysis.md").unwrap();
        assert_eq!(stored[0].decided_by.as_deref(), Some("rule"));

        let mut next = args();
        next.advisory = 1;
        next.findings = vec![person];
        next.decided_by = Some("dev@example.com".into());
        run(&next, tmp.path()).unwrap();
        let stored =
            decisions::read_decisions(&tmp.path().join("specs/042-demo"), "analysis.md").unwrap();
        assert!(
            stored
                .iter()
                .any(|d| d.decided_by.as_deref() == Some("dev@example.com")),
            "{stored:?}"
        );
    }

    /// A person's standing decision wins: the next run's rule-decided waiver of
    /// the same finding changes nothing in the record, its counts and body
    /// included, as in `write-review` (spec 058, §Decisions persist across runs).
    #[test]
    fn a_stored_decision_by_a_person_wins_over_the_rule() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.advisory = 1;
        a.decided_by = Some("dev@example.com".into());
        a.findings = vec![finding(
            AnalysisTier::Advisory,
            "grounding",
            "cites a gone path",
            DispositionOutcome::Deferred,
            Some("specs/042-demo/spec.md"),
        )];
        run(&a, tmp.path()).unwrap();

        let mut next = args();
        next.advisory = 1;
        next.decided_by = None;
        let mut again = finding(
            AnalysisTier::Advisory,
            "grounding",
            "cites a gone path",
            DispositionOutcome::Waived,
            Some("the rule's reason"),
        );
        again.disposition.by_rule = true;
        next.findings = vec![again];
        let result = run(&next, tmp.path()).unwrap();
        assert_eq!(result.dispositions.deferred, 1);
        assert_eq!(result.dispositions.waived, 0);
        let report = analysis_md(&tmp);
        assert!(report.contains("decided-by: dev@example.com"), "{report}");
        assert!(!report.contains("decided-by: rule"), "{report}");
        assert!(report.contains("outcome: deferred"), "{report}");
        // The count and the body say what the record holds rather than contradict it.
        assert!(report.contains("  deferred: 1\n  waived: 0\n"), "{report}");
        assert!(!report.contains("decided by the rule"), "{report}");
        assert!(!report.contains("the rule's reason"), "{report}");

        // Unless this run expired it: then the rule decides afresh.
        next.expired_decisions = vec![DecisionRef {
            key: "grounding — cites a gone path".into(),
            outcome: DecisionOutcome::Deferred,
            target: Some("specs/042-demo/spec.md".into()),
            task: None,
            work_item: None,
            reason: None,
            by_rule: false,
        }];
        run(&next, tmp.path()).unwrap();
        let report = analysis_md(&tmp);
        assert!(report.contains("decided-by: rule"), "{report}");
        assert!(report.contains("  deferred: 0\n  waived: 1\n"), "{report}");
    }

    /// The rule never waives or defers what gates `done`.
    #[test]
    fn the_rule_cannot_waive_or_defer_a_blocking_finding() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let mut a = args();
        a.blocking_findings = 1;
        let mut waived = finding(
            AnalysisTier::Blocking,
            "grounding",
            "a blocking finding",
            DispositionOutcome::Waived,
            Some("out of scope"),
        );
        waived.disposition.by_rule = true;
        a.findings = vec![waived];
        assert!(run(&a, tmp.path()).is_err());

        let mut deferred = finding(
            AnalysisTier::Blocking,
            "grounding",
            "a blocking finding",
            DispositionOutcome::Deferred,
            Some("specs/042-demo/spec.md"),
        );
        deferred.disposition.by_rule = true;
        a.findings = vec![deferred];
        let err = run(&a, tmp.path()).unwrap_err();
        assert!(err.to_string().contains("by-rule"), "{err}");
    }

    fn rendered_with(rules: &RuleFields) -> String {
        render_analysis(
            &args(),
            &Derived {
                blocking: false,
                unexamined: 0,
                dispositions: Dispositions::default(),
                gating: Gating {
                    hard_fail: 0,
                    blocking_findings: 0,
                    released: Vec::new(),
                    holding: Vec::new(),
                },
            },
            &BTreeMap::new(),
            &crate::primitives::analyze_subjects::SubjectDigest::default(),
            rules,
            &[],
        )
    }

    /// Spec 082, AC4: the three fields a record carries read back through the
    /// record's own loader, text values holding the characters a flow mapping
    /// ends on included, and an empty field is omitted.
    #[test]
    fn the_rule_fields_read_back_through_the_loader() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let hostile = "one, two} three: four # five \"six\" \u{e9}\u{2028}seven";
        let rules = RuleFields {
            verdicts: BTreeMap::from([
                (
                    "BE-LOG-002".to_string(),
                    RuleVerdict::Addressed {
                        cite: "plan.md#Observability, and}".into(),
                    },
                ),
                (
                    "QUAL-CLAIM-001".to_string(),
                    RuleVerdict::Finding {
                        tier: FindingTier::Must,
                        at: "spec.md#Acceptance Criteria".into(),
                        message: hostile.into(),
                    },
                ),
                // Plain in block context, but ends a flow mapping value.
                (
                    "CFG-CONST-001".to_string(),
                    RuleVerdict::Finding {
                        tier: FindingTier::Should,
                        at: "plan.md#Constants, and more".into(),
                        message: "two parts, one} rule".into(),
                    },
                ),
            ]),
            unjudged: vec!["BE-AUTHN-004".into(), "BE-AUTHN-005".into()],
            digest: BTreeMap::from([("quality-cross.md".to_string(), "ab".repeat(32))]),
            contradictions: Vec::new(),
        };
        let dir = tmp.path().join("specs/042-demo");
        fs::write(dir.join("analysis.md"), rendered_with(&rules)).unwrap();
        let crate::primitives::RecordLoad::Present(record) =
            crate::primitives::load_analyze_record(&dir)
        else {
            panic!("the record did not load");
        };
        assert_eq!(record.rule_digest, rules.digest);
        assert_eq!(record.unjudged_rules, rules.unjudged);
        assert_eq!(record.rule_verdicts.len(), 3);
        assert_eq!(
            record.rule_verdicts["CFG-CONST-001"],
            rules.verdicts["CFG-CONST-001"]
        );
        assert_eq!(
            record.rule_verdicts["BE-LOG-002"],
            rules.verdicts["BE-LOG-002"]
        );
        match &record.rule_verdicts["QUAL-CLAIM-001"] {
            RuleVerdict::Finding { at, message, .. } => {
                assert_eq!(at, "spec.md#Acceptance Criteria");
                // A Unicode line break flattens to a space, as every scalar does.
                assert_eq!(message, &single_line(hostile));
            }
            other @ (RuleVerdict::Addressed { .. } | RuleVerdict::Waived { .. }) => {
                panic!("{other:?}")
            }
        }

        fs::write(
            dir.join("analysis.md"),
            rendered_with(&RuleFields::default()),
        )
        .unwrap();
        let report = analysis_md(&tmp);
        for key in ["rule-verdicts", "unjudged-rules", "rule-digest"] {
            assert!(
                !report.contains(key),
                "{key} is omitted when empty: {report}"
            );
        }
    }

    /// Spec 082, Edge Cases: a verdict map or list that does not parse makes
    /// the record unreadable, never an empty verdict set.
    #[test]
    fn a_rule_field_that_does_not_parse_makes_the_record_unreadable() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let dir = tmp.path().join("specs/042-demo");
        for bad in [
            "rule-verdicts: [BE-LOG-002]\n",
            "rule-verdicts:\n  BE-LOG-002: {verdict: maybe}\n",
            "rule-verdicts:\n  BE-LOG-002: {verdict: addressed}\n",
            "unjudged-rules: {BE-LOG-002: x}\n",
            "rule-digest: [quality-cross.md]\n",
        ] {
            fs::write(
                dir.join("analysis.md"),
                format!("---\nspec: 042-demo\nlast-run: 2026-09-05T18:00:00Z\n{bad}---\n"),
            )
            .unwrap();
            assert!(
                matches!(
                    crate::primitives::load_analyze_record(&dir),
                    crate::primitives::RecordLoad::Unreadable(_)
                ),
                "{bad}"
            );
        }
    }

    // -- rule assessments (spec 082) --------------------------------------

    /// A project holding `must` MUST-tier rules in `a-cross.md` and `should`
    /// SHOULD-tier rules in `b-cross.md`, IDs `TS-RULE-001…` and
    /// `TS-SHLD-001…`, each with a Verification.
    fn with_rules(tmp: &TempDir, must: usize, should: usize) {
        let dir = tmp.path().join("specs/rules");
        fs::create_dir_all(&dir).unwrap();
        let body = |prefix: &str, keyword: &str, n: usize| {
            (1..=n).fold(String::new(), |mut out, i| {
                let _ = write!(
                    out,
                    "### TS-{prefix}-{i:03}\n\n> Specs {keyword} do {i}.\n\n**Verification:** Check {i}.\n\n"
                );
                out
            })
        };
        fs::write(dir.join("a-cross.md"), body("RULE", "MUST", must)).unwrap();
        fs::write(dir.join("b-cross.md"), body("SHLD", "SHOULD", should)).unwrap();
    }

    fn rule_ids(must: usize, should: usize) -> Vec<String> {
        (1..=must)
            .map(|i| format!("TS-RULE-{i:03}"))
            .chain((1..=should).map(|i| format!("TS-SHLD-{i:03}")))
            .collect()
    }

    fn written_record(tmp: &TempDir) -> crate::schema::primitives::AnalyzeBlock {
        let crate::primitives::RecordLoad::Present(record) =
            crate::primitives::load_analyze_record(&tmp.path().join("specs/042-demo"))
        else {
            panic!("the record did not load");
        };
        record
    }

    fn finding_verdict(message: &str) -> RuleVerdict {
        finding_in(FindingTier::Must, message)
    }

    fn finding_in(tier: FindingTier, message: &str) -> RuleVerdict {
        RuleVerdict::Finding {
            tier,
            at: "spec.md#Behavior".into(),
            message: message.into(),
        }
    }

    fn addressed(cite: &str) -> RuleVerdict {
        RuleVerdict::Addressed { cite: cite.into() }
    }

    /// Spec 082, AC2: a run that judges fewer rules than were loaded records
    /// each rule left unjudged and a non-zero `rule-assessments-not-checked`,
    /// and one that judges them all records neither.
    #[test]
    fn a_run_judging_fewer_rules_than_were_loaded_names_each_one_unjudged() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_rules(&tmp, 100, 41);
        let all = rule_ids(100, 41);
        assert_eq!(all.len(), 141);

        let mut a = args();
        a.judged_rules = all[..30].to_vec();
        let result = run(&a, tmp.path()).unwrap();
        let record = written_record(&tmp);
        assert_eq!(record.unjudged_rules, all[30..].to_vec());
        assert_eq!(
            record.unexamined_by_reason["rule-assessments-not-checked"],
            111
        );
        assert_eq!(result.unexamined, 111);
        assert_eq!(record.rule_digest.len(), 2);
        assert!(
            analysis_md(&tmp).contains("Rules left unjudged: TS-RULE-031, "),
            "{}",
            analysis_md(&tmp)
        );

        a.judged_rules = all.clone();
        let result = run(&a, tmp.path()).unwrap();
        let record = written_record(&tmp);
        assert_eq!(record.unjudged_rules.len(), 0);
        assert!(
            !record
                .unexamined_by_reason
                .contains_key("rule-assessments-not-checked")
        );
        assert_eq!(result.unexamined, 0);
        assert!(analysis_md(&tmp).contains("*None — every target was examined.*"));
    }

    /// Spec 082, AC1: a second run over an unchanged spec, told nothing,
    /// records the verdicts the first did: a fired finding and an addressed
    /// verdict are carried, a quiet rule stays quiet, and nothing is
    /// unjudged.
    #[test]
    fn a_second_run_over_an_unchanged_spec_carries_every_verdict_and_judges_nothing() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_rules(&tmp, 3, 2);
        let mut first = args();
        first.judged_rules = rule_ids(3, 2);
        first.blocking_findings = 1;
        first.rule_verdicts = BTreeMap::from([
            (
                "TS-RULE-001".to_string(),
                finding_verdict("names no constant"),
            ),
            ("TS-SHLD-002".to_string(), addressed("spec.md#Behavior")),
        ]);
        run(&first, tmp.path()).unwrap();
        let before = written_record(&tmp);
        assert_eq!(before.rule_verdicts.len(), 2);

        // Nothing judged, nothing passed: only the count the host re-reports.
        let mut second = args();
        second.blocking_findings = 1;
        run(&second, tmp.path()).unwrap();
        let after = written_record(&tmp);
        assert_eq!(after.rule_verdicts, before.rule_verdicts);
        assert_eq!(after.rule_digest, before.rule_digest);
        assert_eq!(after.unjudged_rules, before.unjudged_rules);
        assert_eq!(after.unjudged_rules.len(), 0);
        assert!(after.unexamined_by_reason.is_empty());
    }

    /// Spec 082, AC1 and the Resolved Questions: a `tasks.md` tick between the
    /// runs changes no design subject, so the verdicts still carry.
    #[test]
    fn a_task_tick_between_runs_leaves_the_verdicts_carried() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_rules(&tmp, 2, 0);
        let tasks = tmp.path().join("specs/042-demo/tasks.md");
        fs::write(&tasks, "# T\n\n## 1. A\n\n- [x] a\n\n- **Done when**: a.\n").unwrap();
        let mut first = args();
        first.judged_rules = rule_ids(2, 0);
        run(&first, tmp.path()).unwrap();
        fs::write(
            &tasks,
            "# T\n\n## 1. A\n\n- [x] a\n- [x] b\n\n- **Done when**: a.\n",
        )
        .unwrap();
        run(&args(), tmp.path()).unwrap();
        assert_eq!(written_record(&tmp).unjudged_rules.len(), 0);

        fs::write(
            tmp.path().join("specs/042-demo/spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# 042\n\n## Behavior\n\nEdited.\n",
        )
        .unwrap();
        run(&args(), tmp.path()).unwrap();
        assert_eq!(written_record(&tmp).unjudged_rules, rule_ids(2, 0));
    }

    /// Spec 082, AC3: a finding over unchanged subjects, against a rule that
    /// did not fire or was addressed, is recorded as a finding in its tier and
    /// reported as a contradiction naming the earlier verdict; a finding
    /// against a finding is none; the contradiction holds nothing.
    #[test]
    fn a_finding_after_a_quiet_or_addressed_verdict_is_a_recorded_contradiction() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_rules(&tmp, 3, 0);
        let mut first = args();
        first.judged_rules = rule_ids(3, 0);
        first.blocking_findings = 1;
        first.rule_verdicts = BTreeMap::from([
            ("TS-RULE-002".to_string(), addressed("spec.md#Behavior")),
            ("TS-RULE-003".to_string(), finding_verdict("old")),
        ]);
        assert_eq!(run(&first, tmp.path()).unwrap().contradictions.len(), 0);

        let mut second = args();
        second.blocking_findings = 3;
        second.judged_rules = rule_ids(3, 0);
        second.rule_verdicts = BTreeMap::from([
            ("TS-RULE-001".to_string(), finding_verdict("was quiet")),
            ("TS-RULE-002".to_string(), finding_verdict("was addressed")),
            ("TS-RULE-003".to_string(), finding_verdict("still bad")),
        ]);
        let result = run(&second, tmp.path()).unwrap();
        assert_eq!(
            result.contradictions,
            [
                RuleContradiction {
                    rule: "TS-RULE-001".into(),
                    earlier: "not-fired".into()
                },
                RuleContradiction {
                    rule: "TS-RULE-002".into(),
                    earlier: "addressed".into()
                },
            ]
        );
        // The finding is recorded in its tier; the contradiction is no count.
        let record = written_record(&tmp);
        assert_eq!(record.rule_verdicts.len(), 3);
        assert_eq!(record.blocking_findings, 3);
        assert!(record.blocking);
        let report = analysis_md(&tmp);
        assert!(
            report.contains("- TS-RULE-002 — judged a finding over unchanged subjects, against an earlier verdict of addressed"),
            "{report}"
        );

        // Contradictions alone hold nothing: with the finding counted nowhere
        // else, a run whose only news is one is still not blocking.
        let mut third = args();
        third.advisory = 0;
        third.judged_rules = rule_ids(3, 0);
        third.blocking_findings = 3;
        let result = run(&third, tmp.path()).unwrap();
        assert_eq!(result.contradictions.len(), 0);
    }

    /// Two contradictions: rule 1 was quiet, rule 2 was addressed, and both
    /// are judged a finding over the same subjects (`BE-PAYLOAD-001`).
    fn contradicting_run(tmp: &TempDir) -> WriteAnalysisArgs {
        with_rules(tmp, 2, 0);
        let mut first = args();
        first.judged_rules = rule_ids(2, 0);
        first.rule_verdicts =
            BTreeMap::from([("TS-RULE-002".to_string(), addressed("spec.md#Behavior"))]);
        run(&first, tmp.path()).unwrap();
        let mut second = args();
        second.blocking_findings = 2;
        second.judged_rules = rule_ids(2, 0);
        second.rule_verdicts = BTreeMap::from([
            ("TS-RULE-001".to_string(), finding_verdict("was quiet")),
            ("TS-RULE-002".to_string(), finding_verdict("was addressed")),
        ]);
        second
    }

    /// `contradictions` is inline while the result fits the read size, with
    /// its count, and absent with the count zero when there are none.
    #[test]
    fn contradictions_within_the_read_size_are_inline_with_their_count() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let second = contradicting_run(&tmp);
        let result = run(&second, tmp.path()).unwrap();
        assert_eq!(result.form, ListsForm::Inline);
        assert_eq!(result.contradictions.len(), 2);
        assert_eq!(result.contradictions_count, 2);
        assert_eq!(result.lists_file, None);

        // Rerun with nothing new judged: the verdicts carry, none contradicts.
        let mut again = args();
        again.blocking_findings = 2;
        let quiet = run(&again, tmp.path()).unwrap();
        assert_eq!(
            (
                quiet.form,
                quiet.contradictions_count,
                quiet.contradictions.len()
            ),
            (ListsForm::Inline, 0, 0)
        );
        let json = serde_json::to_value(&quiet).unwrap();
        assert!(json.get("contradictions").is_none(), "{json}");
    }

    /// Past `[artifacts] read-size-bytes` the list is in the file `lists-file`
    /// names, whole, with its count inline; the record is written all the same.
    #[test]
    fn contradictions_past_the_read_size_are_in_a_file() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let second = contradicting_run(&tmp);
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[artifacts]\nread-size-bytes = 1\n",
        )
        .unwrap();
        let result = run(&second, tmp.path()).unwrap();
        assert_eq!(result.form, ListsForm::File);
        assert_eq!(result.contradictions_count, 2);
        assert_eq!(result.contradictions.len(), 0);
        let file = result
            .lists_file
            .expect("a file-form result names its file");
        let lists: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(tmp.path().join(file)).unwrap()).unwrap();
        assert_eq!(lists["contradictions"].as_array().unwrap().len(), 2);
        assert_eq!(lists["contradictions"][1]["earlier"], "addressed");
        assert_eq!(written_record(&tmp).blocking_findings, 2);
    }

    /// A rule given a verdict is judged by that alone, listed or not.
    #[test]
    fn a_rule_given_a_verdict_is_judged_without_being_listed() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_rules(&tmp, 2, 0);
        let mut a = args();
        a.blocking_findings = 1;
        a.rule_verdicts = BTreeMap::from([("TS-RULE-001".to_string(), finding_verdict("m"))]);
        a.judged_rules = vec!["TS-RULE-002".into()];
        run(&a, tmp.path()).unwrap();
        let record = written_record(&tmp);
        assert_eq!(record.rule_verdicts.len(), 1);
        assert_eq!(record.unjudged_rules.len(), 0);
    }

    /// A contradiction over a SHOULD-tier rule is an advisory finding, so the
    /// record it lands in does not block: the note holds nothing and the
    /// finding holds what its tier does.
    #[test]
    fn a_contradiction_over_a_should_rule_holds_nothing() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_rules(&tmp, 1, 1);
        let mut first = args();
        first.judged_rules = rule_ids(1, 1);
        run(&first, tmp.path()).unwrap();
        let mut second = args();
        second.advisory = 1;
        second.judged_rules = rule_ids(1, 1);
        second.rule_verdicts = BTreeMap::from([(
            "TS-SHLD-001".to_string(),
            finding_in(FindingTier::Should, "m"),
        )]);
        let result = run(&second, tmp.path()).unwrap();
        assert_eq!(result.contradictions.len(), 1);
        assert!(!result.blocking);
        let record = written_record(&tmp);
        assert_eq!((record.blocking_findings, record.advisory), (0, 1));
    }

    /// Spec 082, AC5: a rule edited between two runs over an unchanged spec is
    /// judged again, and its file's neighbours are not.
    #[test]
    fn an_edited_rule_file_is_judged_again_over_an_unchanged_spec() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_rules(&tmp, 2, 2);
        let mut first = args();
        first.judged_rules = rule_ids(2, 2);
        run(&first, tmp.path()).unwrap();

        let rules = tmp.path().join("specs/rules/a-cross.md");
        let text = fs::read_to_string(&rules).unwrap();
        fs::write(&rules, text.replace("Check 2.", "Check 2, and more.")).unwrap();
        run(&args(), tmp.path()).unwrap();
        let record = written_record(&tmp);
        assert_eq!(record.unjudged_rules, rule_ids(2, 0));
        assert_eq!(
            record.unexamined_by_reason["rule-assessments-not-checked"],
            2
        );

        // A rationale edit is no rule edit.
        let mut first = args();
        first.judged_rules = rule_ids(2, 2);
        run(&first, tmp.path()).unwrap();
        let text = fs::read_to_string(&rules).unwrap();
        fs::write(&rules, format!("{text}\n**Rationale:** Prose.\n")).unwrap();
        run(&args(), tmp.path()).unwrap();
        assert_eq!(written_record(&tmp).unjudged_rules.len(), 0);
    }

    /// Spec 082, AC6: an `addressed` verdict whose citation does not resolve
    /// is recorded among the rules left unjudged, never as `addressed`.
    #[test]
    fn an_addressed_verdict_whose_citation_does_not_resolve_is_unjudged() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_rules(&tmp, 5, 0);
        let mut a = args();
        a.judged_rules = rule_ids(5, 0);
        a.rule_verdicts = BTreeMap::from([
            ("TS-RULE-001".to_string(), addressed("spec.md#Behavior")),
            (
                "TS-RULE-002".to_string(),
                addressed("spec.md#No such heading"),
            ),
            ("TS-RULE-003".to_string(), addressed("")),
            ("TS-RULE-004".to_string(), addressed("tasks.md#Behavior")),
            ("TS-RULE-005".to_string(), addressed("gone.md#Behavior")),
        ]);
        run(&a, tmp.path()).unwrap();
        let record = written_record(&tmp);
        assert_eq!(
            record.rule_verdicts.keys().collect::<Vec<_>>(),
            ["TS-RULE-001"]
        );
        assert_eq!(
            record.unjudged_rules,
            ["TS-RULE-002", "TS-RULE-003", "TS-RULE-004", "TS-RULE-005"]
        );
        assert_eq!(
            record.unexamined_by_reason["rule-assessments-not-checked"],
            4
        );
    }

    /// A tier count under the number of `finding` verdicts is refused with
    /// nothing written, and a carried finding the host forgets to count is
    /// refused too: the count a record gates on cannot drop a rule failure.
    #[test]
    fn a_finding_verdict_the_tier_counts_do_not_cover_is_refused() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_rules(&tmp, 2, 2);
        let mut a = args();
        a.judged_rules = rule_ids(2, 2);
        a.rule_verdicts = BTreeMap::from([
            ("TS-RULE-001".to_string(), finding_verdict("must")),
            (
                "TS-SHLD-001".to_string(),
                finding_in(FindingTier::Should, "should"),
            ),
        ]);
        a.blocking_findings = 0;
        a.advisory = 1;
        let error = run(&a, tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "blocking-findings"),
            "{error}"
        );
        assert!(!analysis_exists(&tmp));
        a.blocking_findings = 1;
        a.advisory = 0;
        let error = run(&a, tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "advisory"),
            "{error}"
        );
        assert!(!analysis_exists(&tmp));
        a.advisory = 1;
        run(&a, tmp.path()).unwrap();

        // Carried forward, the finding still owes its count.
        let mut again = args();
        again.advisory = 1;
        assert!(run(&again, tmp.path()).is_err());
        again.blocking_findings = 1;
        run(&again, tmp.path()).unwrap();
        assert_eq!(written_record(&tmp).rule_verdicts.len(), 2);
    }

    /// A rule carrying MUST and SHOULD is MUST-tier by its Statement, and can
    /// fail on its SHOULD clause alone. The tier a finding was counted in is
    /// the one it floors: a `should` verdict is covered by `advisory` and
    /// refused by a zero there however blocking-findings reads, and a `must`
    /// one is the converse.
    #[test]
    fn a_finding_floors_in_the_tier_it_was_counted_in_not_its_rules_text_tier() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let dir = tmp.path().join("specs/rules");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("a-cross.md"),
            "### TS-BOTH-001\n\n> Lists MUST paginate and SHOULD use cursors.\n\n**Verification:** Look.\n",
        )
        .unwrap();
        let verdict = |tier| BTreeMap::from([("TS-BOTH-001".to_string(), finding_in(tier, "m"))]);
        let mut a = args();
        a.judged_rules = vec!["TS-BOTH-001".into()];

        // Failed on the SHOULD clause: advisory covers it, blocking need not.
        a.rule_verdicts = verdict(FindingTier::Should);
        a.advisory = 1;
        run(&a, tmp.path()).unwrap();
        a.advisory = 0;
        a.blocking_findings = 1;
        let error = run(&a, tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "advisory"),
            "{error}"
        );

        // Failed on the MUST clause: blocking covers it, advisory need not.
        a.rule_verdicts = verdict(FindingTier::Must);
        a.advisory = 1;
        a.blocking_findings = 0;
        let error = run(&a, tmp.path()).unwrap_err();
        assert!(
            matches!(&error, PrimitiveError::InvalidArgument { argument, .. } if argument == "blocking-findings"),
            "{error}"
        );
        a.blocking_findings = 1;
        a.advisory = 0;
        run(&a, tmp.path()).unwrap();
        let record = written_record(&tmp);
        assert_eq!(record.blocking_findings, 1);
        assert_eq!(record.advisory, 0);
    }

    /// A finding's stored tier must be one its rule can fail in: a MUST-only
    /// rule labelled `should` would move a blocking failure to advisory, and a
    /// SHOULD-only rule labelled `must` the converse, so both are refused at
    /// write naming the rule and both tiers; a rule carrying both keywords
    /// accepts either (spec 082).
    #[test]
    fn a_finding_labelled_in_a_tier_its_rule_cannot_fail_in_is_refused() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let dir = tmp.path().join("specs/rules");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("a-cross.md"),
            "### TS-ONLY-001\n\n> Specs MUST do it.\n\n**Verification:** Look.\n\n\
             ### TS-SOFT-001\n\n> Specs SHOULD do it.\n\n**Verification:** Look.\n\n\
             ### TS-BOTH-001\n\n> Lists MUST paginate and SHOULD use cursors.\n\n**Verification:** Look.\n",
        )
        .unwrap();
        let run_with = |id: &str, tier: FindingTier| {
            // A fresh record each time: a prior finding would be carried forward.
            let _ = fs::remove_file(tmp.path().join("specs/042-demo/analysis.md"));
            let mut a = args();
            a.judged_rules = vec![id.into()];
            a.blocking_findings = 1;
            a.advisory = 1;
            a.rule_verdicts = BTreeMap::from([(id.to_string(), finding_in(tier, "m"))]);
            run(&a, tmp.path())
        };
        for (id, wrong, right) in [
            ("TS-ONLY-001", FindingTier::Should, FindingTier::Must),
            ("TS-SOFT-001", FindingTier::Must, FindingTier::Should),
        ] {
            let error = run_with(id, wrong).unwrap_err().to_string();
            assert!(error.contains(id), "{error}");
            assert!(
                error.contains("tier is") && error.contains("carries only"),
                "{error}"
            );
            assert!(!analysis_exists(&tmp), "nothing is written on a refusal");
            run_with(id, right).unwrap();
        }
        run_with("TS-BOTH-001", FindingTier::Must).unwrap();
        run_with("TS-BOTH-001", FindingTier::Should).unwrap();
    }

    /// A verdict for a rule no loaded file defines, or a finding with no
    /// message, is refused before any write.
    #[test]
    fn a_verdict_naming_no_loaded_rule_or_no_message_is_refused() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_rules(&tmp, 1, 0);
        let mut a = args();
        a.blocking_findings = 1;
        a.rule_verdicts = BTreeMap::from([("TS-NONE-001".to_string(), finding_verdict("m"))]);
        assert!(run(&a, tmp.path()).is_err());
        a.rule_verdicts = BTreeMap::from([("TS-RULE-001".to_string(), finding_verdict(" "))]);
        assert!(run(&a, tmp.path()).is_err());
        assert!(!analysis_exists(&tmp));
    }

    /// A stored verdict naming a rule no loaded file defines any longer is
    /// dropped rather than carried as a judgment of a rule nobody loaded.
    #[test]
    fn a_rule_removed_between_runs_loses_its_verdict() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_rules(&tmp, 2, 1);
        let mut first = args();
        first.judged_rules = rule_ids(2, 1);
        first.advisory = 1;
        first.rule_verdicts = BTreeMap::from([(
            "TS-SHLD-001".to_string(),
            finding_in(FindingTier::Should, "m"),
        )]);
        run(&first, tmp.path()).unwrap();
        fs::remove_file(tmp.path().join("specs/rules/b-cross.md")).unwrap();
        run(&args(), tmp.path()).unwrap();
        let record = written_record(&tmp);
        assert!(record.rule_verdicts.is_empty());
        assert_eq!(
            record.rule_digest.keys().collect::<Vec<_>>(),
            ["a-cross.md"]
        );
    }

    /// A prior record whose verdict map does not parse reuses nothing and is
    /// repaired by the write: every rule is named unjudged.
    #[test]
    fn a_prior_verdict_map_that_does_not_parse_reuses_nothing() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_rules(&tmp, 2, 0);
        fs::write(
            tmp.path().join("specs/042-demo/analysis.md"),
            "---\nspec: 042-demo\nlast-run: 2026-09-05T18:00:00Z\nrule-verdicts: [TS-RULE-001]\n---\n",
        )
        .unwrap();
        run(&args(), tmp.path()).unwrap();
        assert_eq!(written_record(&tmp).unjudged_rules, rule_ids(2, 0));
    }

    /// A rule file the run cannot read is counted under `rule-file-unreadable`,
    /// without doubling what a walk already counted.
    #[test]
    fn a_rule_file_the_runtime_cannot_read_is_counted_once() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_rules(&tmp, 1, 0);
        fs::write(tmp.path().join("specs/rules/c-cross.md"), [0xff, 0xfe]).unwrap();
        let mut a = args();
        a.judged_rules = rule_ids(1, 0);
        run(&a, tmp.path()).unwrap();
        assert_eq!(
            written_record(&tmp).unexamined_by_reason["rule-file-unreadable"],
            1
        );
        a.unexamined_by_reason = vec![("rule-file-unreadable".into(), 1)];
        run(&a, tmp.path()).unwrap();
        assert_eq!(
            written_record(&tmp).unexamined_by_reason["rule-file-unreadable"],
            1
        );
    }

    // -- waived verdicts (spec 073) ---------------------------------------

    const DEPARTURE: &str = "\n## Applicable Rules\n\n- departs: `TS-RULE-001`\n  - reason: removed rather than versioned\n  - cited: §shared-exceptions\n";

    /// A git repository over a demo spec holding the departure entry for
    /// `TS-RULE-001` and three MUST rules, with the work tree committed as
    /// `test@example.com` when `committed`.
    fn departing_repo(committed: bool) -> (TempDir, git2::Repository) {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_rules(&tmp, 3, 0);
        set_spec_tail(&tmp, DEPARTURE);
        let repository = git2::Repository::init(tmp.path()).unwrap();
        if committed {
            crate::primitives::git_fixture::commit_all(&repository, "c");
        }
        (tmp, repository)
    }

    /// Replace the demo spec's `## Applicable Rules` tail with `tail`.
    fn set_spec_tail(tmp: &TempDir, tail: &str) {
        fs::write(
            tmp.path().join("specs/042-demo/spec.md"),
            format!(
                "---\nstatus: in-progress\ndependencies: []\n---\n\n# 042 — Demo\n\n## Behavior\n\nText.\n{tail}"
            ),
        )
        .unwrap();
    }

    fn waived_args() -> WriteAnalysisArgs {
        let mut a = args();
        a.judged_rules = rule_ids(3, 0);
        a.rule_verdicts = BTreeMap::from([("TS-RULE-001".to_string(), RuleVerdict::bare_waived())]);
        a
    }

    fn waived(by: &str, at: Option<&str>) -> RuleVerdict {
        RuleVerdict::Waived {
            reason: "removed rather than versioned".into(),
            decided_by: by.into(),
            decided_at: at.map(str::to_owned),
            cited: Some("§shared-exceptions".into()),
        }
    }

    /// AC2: a MUST rule whose trigger fires and whose spec departs from it is
    /// `waived`, with the entry's reason and citation and the committing
    /// author, and `blocking-findings` stays 0; the host's own values for the
    /// derived fields are not kept.
    #[test]
    fn a_fired_must_rule_with_an_entry_is_waived_and_blocks_nothing() {
        let (tmp, _repository) = departing_repo(true);
        let mut a = waived_args();
        a.rule_verdicts = BTreeMap::from([(
            "TS-RULE-001".to_string(),
            RuleVerdict::Waived {
                reason: "fabricated".into(),
                decided_by: "mallory@example.com".into(),
                decided_at: Some("1999-01-01T00:00:00Z".into()),
                cited: Some("§nowhere".into()),
            },
        )]);
        let result = run(&a, tmp.path()).unwrap();
        assert!(!result.blocking);
        let record = written_record(&tmp);
        assert_eq!(record.blocking_findings, 0);
        assert!(!record.blocking);
        let RuleVerdict::Waived {
            reason,
            decided_by,
            decided_at,
            cited,
        } = &record.rule_verdicts["TS-RULE-001"]
        else {
            panic!("{:?}", record.rule_verdicts);
        };
        assert_eq!(reason, "removed rather than versioned");
        assert_eq!(decided_by, "test@example.com");
        assert!(decided_at.as_deref().is_some_and(|at| at.ends_with('Z')));
        assert_eq!(cited.as_deref(), Some("§shared-exceptions"));
        assert_eq!(record.rule_verdicts.len(), 1);
        assert_eq!(record.dispositions.map_or(0, |d| d.waived), 0);
    }

    /// A rule with an entry whose trigger does not fire has no verdict, as
    /// any rule that does not fire has none.
    #[test]
    fn a_rule_that_does_not_fire_has_no_verdict_however_its_spec_departs() {
        let (tmp, _repository) = departing_repo(true);
        let mut a = waived_args();
        a.rule_verdicts = BTreeMap::new();
        run(&a, tmp.path()).unwrap();
        let record = written_record(&tmp);
        assert_eq!(record.rule_verdicts.len(), 0);
        assert_eq!(record.unjudged_rules.len(), 0);
    }

    /// AC4: a waiver carries forward while its rule fires, and is gone when
    /// the rule no longer fires.
    #[test]
    fn a_waiver_carries_forward_while_its_rule_fires_and_is_gone_when_it_does_not() {
        let (tmp, _repository) = departing_repo(true);
        run(&waived_args(), tmp.path()).unwrap();
        let first = written_record(&tmp).rule_verdicts;
        assert_eq!(first.len(), 1);

        run(&args(), tmp.path()).unwrap();
        let carried = written_record(&tmp);
        assert_eq!(carried.rule_verdicts, first);
        assert_eq!(carried.unjudged_rules.len(), 0);

        let mut quiet = waived_args();
        quiet.rule_verdicts = BTreeMap::new();
        run(&quiet, tmp.path()).unwrap();
        assert_eq!(written_record(&tmp).rule_verdicts.len(), 0);
    }

    /// A removed entry makes the rule a blocking finding again: the spec
    /// edit makes the stored waiver stale, so it is not carried, and the
    /// finding is then accepted and counted.
    #[test]
    fn a_removed_entry_makes_the_rule_a_blocking_finding_again() {
        let (tmp, _repository) = departing_repo(true);
        run(&waived_args(), tmp.path()).unwrap();
        set_spec_tail(&tmp, "");

        run(&args(), tmp.path()).unwrap();
        let unjudged = written_record(&tmp);
        assert_eq!(unjudged.rule_verdicts.len(), 0);
        assert_eq!(unjudged.unjudged_rules, rule_ids(3, 0));

        let mut a = args();
        a.judged_rules = rule_ids(3, 0);
        a.blocking_findings = 1;
        a.rule_verdicts = BTreeMap::from([(
            "TS-RULE-001".to_string(),
            finding_verdict("removed the field"),
        )]);
        let result = run(&a, tmp.path()).unwrap();
        assert!(result.blocking);
        let record = written_record(&tmp);
        assert_eq!(record.blocking_findings, 1);
        assert!(matches!(
            record.rule_verdicts["TS-RULE-001"],
            RuleVerdict::Finding { .. }
        ));
    }

    /// A carried `waived` whose entry is gone is never kept, even where the
    /// record's digests still read current: it cannot be refilled, so the
    /// rule is judged again.
    #[test]
    fn a_carried_waiver_with_no_entry_is_recorded_unjudged() {
        let (tmp, _repository) = departing_repo(true);
        run(&waived_args(), tmp.path()).unwrap();
        set_spec_tail(&tmp, "");
        let mut a = args();
        a.judged_rules = rule_ids(3, 0);
        run(&a, tmp.path()).unwrap();
        let record = tmp.path().join("specs/042-demo/analysis.md");
        let text = fs::read_to_string(&record).unwrap();
        fs::write(
            &record,
            text.replacen(
                "rule-digest:",
                "rule-verdicts:\n  TS-RULE-001: {verdict: waived, reason: \"r\", decided-by: \"x\"}\nrule-digest:",
                1,
            ),
        )
        .unwrap();
        run(&args(), tmp.path()).unwrap();
        let after = written_record(&tmp);
        assert_eq!(after.rule_verdicts.len(), 0, "{:?}", after.rule_verdicts);
        assert_eq!(after.unjudged_rules, vec!["TS-RULE-001".to_string()]);
    }

    /// A commit after an `unattributed` run attributes the next write, though
    /// the carried verdict's subjects never changed: attribution follows
    /// history, not a digest.
    #[test]
    fn a_commit_after_an_unattributed_run_attributes_the_next_write() {
        let (tmp, repository) = departing_repo(false);
        run(&waived_args(), tmp.path()).unwrap();
        assert_eq!(
            written_record(&tmp).rule_verdicts["TS-RULE-001"],
            waived("unattributed", None)
        );
        let text = analysis_md(&tmp);
        assert!(text.contains("decided-by: \"unattributed\""), "{text}");
        assert!(!text.contains("decided-at"), "{text}");

        crate::primitives::git_fixture::commit_all(&repository, "decide");
        run(&args(), tmp.path()).unwrap();
        let RuleVerdict::Waived {
            decided_by,
            decided_at,
            ..
        } = &written_record(&tmp).rule_verdicts["TS-RULE-001"]
        else {
            panic!("not waived");
        };
        assert_eq!(decided_by, "test@example.com");
        assert!(decided_at.is_some());
    }

    /// The refusals, each leaving no record: `waived` for a rule with no
    /// entry or only a malformed one, and `finding` or `addressed` for a rule
    /// with one.
    #[test]
    fn a_waiver_without_an_entry_and_a_finding_against_one_are_refused_with_no_record() {
        let (tmp, _repository) = departing_repo(true);
        let refused = |a: &WriteAnalysisArgs, tmp: &TempDir, needle: &str| {
            let err = run(a, tmp.path()).unwrap_err().to_string();
            assert!(err.contains(needle), "{err}");
            assert!(!analysis_exists(tmp));
        };

        let mut a = waived_args();
        a.rule_verdicts = BTreeMap::from([("TS-RULE-002".to_string(), RuleVerdict::bare_waived())]);
        refused(&a, &tmp, "no well-formed departure entry");

        a.rule_verdicts =
            BTreeMap::from([("TS-RULE-001".to_string(), finding_verdict("it breaks"))]);
        a.blocking_findings = 1;
        refused(&a, &tmp, "a fired answer is waived");

        a.rule_verdicts =
            BTreeMap::from([("TS-RULE-001".to_string(), addressed("spec.md#Behavior"))]);
        refused(&a, &tmp, "a fired answer is waived");

        set_spec_tail(
            &tmp,
            "\n## Applicable Rules\n\n- departs: `TS-RULE-001`\n  - cited: §shared-exceptions\n",
        );
        refused(&waived_args(), &tmp, "no well-formed departure entry");
    }

    /// A spec with no entry records what it always did: no `waived` verdict,
    /// and a finding and an addressed verdict are accepted.
    #[test]
    fn a_spec_with_no_entry_records_findings_and_addressed_verdicts_as_before() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        with_rules(&tmp, 3, 0);
        let mut a = args();
        a.judged_rules = rule_ids(3, 0);
        a.blocking_findings = 1;
        a.rule_verdicts = BTreeMap::from([
            ("TS-RULE-001".to_string(), finding_verdict("it breaks")),
            ("TS-RULE-002".to_string(), addressed("spec.md#Behavior")),
        ]);
        run(&a, tmp.path()).unwrap();
        let record = written_record(&tmp);
        assert_eq!(record.rule_verdicts.len(), 2);
        assert_eq!(record.blocking_findings, 1);
        assert!(record.blocking);
    }
}
