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
//! the fix-and-route step that follows it writes only with the operator's
//! per-write confirmation, and this record is written after that step and a
//! re-check, so it describes the state after dispositions (spec 058).
//!
//! Every finding arrives with its disposition, and the record counts them
//! under `dispositions:`. The tier counts stay host-supplied scalars from the
//! re-check, and `undispositioned` is derived as the live tier total minus the
//! live findings routed or discarded — so a finding the caller does not
//! itemize, including every finding on the exec path, is counted as owed.
//! Nothing is written to the inbox.
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

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use crate::primitives::decisions;
use crate::primitives::write_review::yaml_string;
use crate::primitives::{
    PrimitiveError, Result, flatten_line, read_text, rel_path, split_frontmatter, write_atomic,
};
use crate::schema::paths;
use crate::schema::primitives::{
    AnalysisFinding, AnalysisTier, DecisionRef, DispositionOutcome, Dispositions,
    WriteAnalysisArgs, WriteAnalysisResult,
};

/// Execute the `write-analysis` primitive against the given repo root.
///
/// # Errors
///
/// - [`PrimitiveError::InvalidPath`] when `feature` is empty, absolute, or
///   carries a parent-directory component.
/// - [`PrimitiveError::FeatureNotFound`] when the feature directory does not
///   exist.
/// - [`PrimitiveError::InvalidArgument`] when `analyzed-at` is blank, when a
///   finding's family or message is blank, when a finding's disposition lacks
///   its companion or discards a `hard-fail` or `blocking` finding, when the
///   findings contradict the tier counts, when two findings sharing a key are
///   given different outcomes, or when a new decision has no `decided-by` —
///   each before any write.
/// - [`PrimitiveError::Io`] when `spec.md` or the prior `analysis.md` cannot
///   be read, or the record cannot be written.
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
    super::validate_feature_argument(repo, &args.feature)?;
    let root = paths::Paths::load(repo).specs_root;
    let feature_dir = repo.join(&root).join(&args.feature);
    if !feature_dir.is_dir() {
        return Err(PrimitiveError::FeatureNotFound {
            root,
            feature: args.feature.clone(),
        });
    }

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
    let blocking = args.hard_fail > 0 || args.blocking_findings > 0;
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
    let analysis_path = feature_dir.join(crate::primitives::ANALYSIS_RECORD_FILE);
    let replaced = analysis_path.is_file();
    let stored = decisions::read_decisions(&feature_dir, crate::primitives::ANALYSIS_RECORD_FILE)?;
    let merged = decisions::merge(
        stored,
        &args.expired_decisions,
        &decided_findings(&args.findings),
        &single_line(&args.analyzed_at),
        args.decided_by.as_deref().map(single_line).as_deref(),
    )
    .map_err(|refusal| refusal.into_error("write-analysis", "findings", "a finding"))?;
    // The breakdown is the authority when supplied: a total a caller can
    // contradict is a total that will eventually be contradicted, which is
    // the same reason `blocking` is derived rather than accepted.
    let by_reason: BTreeMap<String, u32> =
        args.unexamined_by_reason
            .iter()
            .fold(BTreeMap::new(), |mut acc, (reason, count)| {
                // Keyed as rendered, so two reasons that flatten to one line
                // are one entry rather than a duplicate YAML key.
                *acc.entry(single_line(reason)).or_default() += *count;
                acc
            });
    // A registered shared constitution this run could not read is an unexamined
    // *input* to the analysis, not an unexamined target, but it lands in the same
    // breakdown because it makes the same claim false: that everything bearing on
    // the verdict was looked at. Derived here rather than accepted as an argument,
    // for the reason the comment above gives — a caller that had to supply it
    // could omit it, and `QUAL-CLAIM-001` exists precisely to stop a clean result
    // standing in for an unexamined one (spec 055, AC8).
    let mut by_reason = by_reason;
    // Deliberately NOT `?`. This primitive's contract is that it writes a
    // record on every run, because the record's *absence* is what a later
    // gate reads as "never analyzed" — so a config that will not parse must
    // not be able to suppress it. An unreadable registry is recorded as its
    // own reason instead, which is the honest answer: nothing registered was
    // loaded, and the run cannot say how many sources that was.
    match crate::primitives::resolve_constitutions::run(
        &crate::schema::primitives::ResolveConstitutionsArgs {},
        repo,
    ) {
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
    let by_reason = by_reason;

    let unexamined = if by_reason.is_empty() {
        args.unexamined
    } else {
        by_reason.values().copied().sum()
    };
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
    // or a prior record this run cannot read.
    let report = render_analysis(
        args,
        &Derived {
            blocking,
            unexamined,
            dispositions,
        },
        &by_reason,
        &subjects,
        &merged,
    );
    write_atomic(&analysis_path, &report)?;

    Ok(WriteAnalysisResult {
        spec_path: rel_path(&spec_path, repo),
        blocking,
        unexamined,
        replaced,
        dispositions,
    })
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
/// `dispositions:` map. Everything that can be refused is refused here, before
/// any write.
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
        if disposition.outcome == DispositionOutcome::Discarded
            && matches!(
                finding.tier,
                AnalysisTier::HardFail | AnalysisTier::Blocking
            )
        {
            return Err(invalid(
                at("disposition.outcome"),
                "a hard-fail or blocking finding already gates done; fix or route it, never discard it",
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
            DispositionOutcome::Routed => counts.routed = counts.routed.saturating_add(1),
            DispositionOutcome::Discarded => counts.discarded = counts.discarded.saturating_add(1),
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
        live_total.saturating_sub(counts.routed.saturating_add(counts.discarded));
    Ok(counts)
}

/// The live findings routed or discarded, as the decisions they store, keyed
/// `{family} — {message}`.
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

/// The counts this call derived, rendered into the record.
struct Derived {
    blocking: bool,
    unexamined: u32,
    dispositions: Dispositions,
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
    let _ = writeln!(out, "hard-fail: {}", args.hard_fail);
    let _ = writeln!(out, "blocking-findings: {}", args.blocking_findings);
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
    let _ = writeln!(out, "blocking: {blocking}");
    decisions::render_dispositions(&mut out, counts);
    decisions::render(&mut out, decisions_list);
    out.push_str("---\n\n");

    let _ = writeln!(out, "# Analysis — {feature}\n");
    let verdict = if blocking { "blocking" } else { "not blocking" };
    let _ = writeln!(out, "## Summary\n");
    let mut summary = format!(
        "{} hard-fail, {} blocking, {} advisory; {verdict}. {} unexamined target(s). \
         Dispositions: {} fixed, {} routed, {} discarded, {} undispositioned.",
        args.hard_fail,
        args.blocking_findings,
        args.advisory,
        unexamined,
        counts.fixed,
        counts.routed,
        counts.discarded,
        counts.undispositioned,
    );
    if counts.undispositioned > 0 {
        summary.push_str(
            " `done` is blocked until each undispositioned finding is fixed, routed, or discarded.",
        );
    }
    let _ = writeln!(out, "{summary}\n");
    for (heading, tier) in [
        ("Hard failures", AnalysisTier::HardFail),
        ("Blocking findings", AnalysisTier::Blocking),
        ("Advisory findings", AnalysisTier::Advisory),
    ] {
        let _ = writeln!(out, "## {heading}\n\n{}\n", render_tier(args, tier));
    }
    let _ = writeln!(
        out,
        "## Unexamined targets\n\n{}\n",
        render_unexamined(unexamined, by_reason)
    );
    let fixed: Vec<&AnalysisFinding> = args.findings.iter().filter(|f| !f.live).collect();
    let _ = write!(out, "## Fixed in this run\n\n{}\n", render_list(&fixed));
    out
}

/// A tier's live findings, each beside its disposition, plus a line for any
/// the caller counted but did not itemize — so the body can never understate
/// what the frontmatter counts. `*None.*` when the tier is empty.
fn render_tier(args: &WriteAnalysisArgs, tier: AnalysisTier) -> String {
    let listed: Vec<&AnalysisFinding> = args
        .findings
        .iter()
        .filter(|finding| finding.live && finding.tier == tier)
        .collect();
    let count = tier_count(args, tier);
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
/// Zero with no breakdown is stated explicitly rather than left blank: a clean
/// run with nothing skipped and a clean run with something skipped are two
/// results, and an empty section would read as the first while meaning either.
fn render_unexamined(unexamined: u32, by_reason: &BTreeMap<String, u32>) -> String {
    if unexamined == 0 && by_reason.is_empty() {
        return "*None — every target was examined.*".to_string();
    }
    if by_reason.is_empty() {
        return format!("{unexamined} target(s) unexamined; no breakdown recorded.");
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
                target: (outcome == DispositionOutcome::Routed)
                    .then(|| companion.unwrap_or_default().to_string()),
                reason: (outcome == DispositionOutcome::Discarded)
                    .then(|| companion.unwrap_or_default().to_string()),
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
        assert!(report.contains("`done` is blocked"), "{report}");
    }

    #[test]
    fn each_disposition_is_counted_and_rendered_in_its_section() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
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
                DispositionOutcome::Routed,
                Some("specs/042-demo/tasks.md"),
            ),
            finding(
                AnalysisTier::Advisory,
                "applicable-rules",
                "BE-AUTHN-001 does not fire",
                DispositionOutcome::Discarded,
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
                routed: 1,
                discarded: 1,
                undispositioned: 1
            }
        );
        let report = analysis_md(&tmp);
        assert!(report.contains("- task-consistency — task 4 has no Done when — `spec.md` — **routed** to `specs/042-demo/tasks.md`"), "{report}");
        assert!(report.contains("- applicable-rules — BE-AUTHN-001 does not fire — `spec.md` — **discarded**: cited for a future endpoint"), "{report}");
        assert!(
            report.contains(
                "## Fixed in this run\n\n- grounding — cites a gone path — `spec.md` — **fixed**"
            ),
            "{report}"
        );
    }

    #[test]
    fn a_discard_on_a_gating_finding_is_refused_with_nothing_written() {
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
                DispositionOutcome::Discarded,
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
        let mut gone_but_routed = finding(
            AnalysisTier::Advisory,
            "f",
            "m",
            DispositionOutcome::Routed,
            Some("t"),
        );
        gone_but_routed.live = false;
        for bad in [still_firing, gone_but_routed] {
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
            DispositionOutcome::Discarded,
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
            outcome: DecisionOutcome::Discarded,
            target: None,
            reason: Some("future endpoint".into()),
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
    fn an_empty_breakdown_omits_the_map_and_keeps_the_supplied_total() {
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let result = run(
            &WriteAnalysisArgs {
                unexamined: 3,
                ..args()
            },
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.unexamined, 3);
        let spec = analysis_md(&tmp);
        assert!(spec.contains("unexamined: 3"));
        assert!(!spec.contains("unexamined-by-reason"));
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
    fn a_malformed_config_still_records_the_run() {
        // Regression: the registry read was `?`-propagated, so an unrelated
        // config typo suppressed the analyze record entirely -- and an absent
        // record is exactly what the pre-done gate reads as "never analyzed".
        // This primitive writes a record on every run, by contract.
        let tmp = spec_repo("status: in-progress\ndependencies: []");
        let cfg = tmp.path().join(".ductus");
        fs::create_dir_all(&cfg).unwrap();
        fs::write(cfg.join("config.toml"), "[constitutions.acme]\nrepo = \n").unwrap();

        let result = run(&args(), tmp.path());
        assert!(result.is_ok(), "a config typo must not suppress the record");

        let spec = analysis_md(&tmp);
        assert!(
            spec.contains("constitution-registry-unreadable: 1"),
            "{spec}"
        );
        assert!(spec.contains("unexamined: 1"), "{spec}");
    }

    // -- spec 058 review: decision-record hardening -------------------------

    /// A control character in a finding's message or a discard's reason is
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
            DispositionOutcome::Discarded,
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
                DispositionOutcome::Discarded,
                Some("why\t#not"),
            ),
            finding(
                AnalysisTier::Advisory,
                "grounding",
                "cites a non\u{ffff}char",
                DispositionOutcome::Discarded,
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
        let classified = crate::primitives::process_decisions::run(
            &crate::schema::primitives::ProcessDecisionsArgs {
                feature: "042-demo".into(),
                record: crate::schema::primitives::DecisionRecord::Analysis,
                fired: keys.iter().map(|k| (*k).to_string()).collect(),
                restricted: false,
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
            DispositionOutcome::Discarded,
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
                       outcome: discarded\n    reason: noise\n\n# Analysis\n";
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
            "---\nspec: 042-demo\ndecisions:\n  - key: \"f — m\"\n    outcome: discarded\n    \
             reason: noise\n    decided-at: 2026-09-01T00:00:00Z\n    decided-by: a@example.com\n---",
        )
        .unwrap();
        run(&args(), tmp.path()).unwrap();
        let stored = decisions::read_decisions(&dir, "analysis.md").unwrap();
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].key.as_deref(), Some("f — m"));
    }

    /// A route names its target and a discard states its reason, refused by
    /// the check `write-review` shares, under this primitive's argument path.
    #[test]
    fn a_disposition_without_its_companion_is_refused() {
        for (outcome, argument, reason) in [
            (
                DispositionOutcome::Routed,
                "findings[0].disposition.target",
                "a route names its target",
            ),
            (
                DispositionOutcome::Discarded,
                "findings[0].disposition.reason",
                "a discard states its reason",
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
        use DispositionOutcome::{Discarded, Fixed, Routed, Undispositioned};
        for (first, second) in [
            (Routed, Undispositioned),
            (Fixed, Undispositioned),
            (Fixed, Discarded),
        ] {
            let tmp = spec_repo("status: in-progress\ndependencies: []");
            let mut a = args();
            a.advisory = 2;
            a.decided_by = Some("dev@example.com".into());
            a.findings = [first, second]
                .into_iter()
                .map(|outcome| {
                    let companion = match outcome {
                        Routed => Some("specs/042-demo/tasks.md"),
                        Discarded => Some("noise"),
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
            DispositionOutcome::Discarded,
            Some("noise"),
        );
        a.findings = vec![twice.clone(), twice];
        let result = run(&a, tmp.path()).unwrap();
        assert_eq!(result.dispositions.discarded, 2);
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

    /// `tier` is required: the discard refusal reads it, so a finding that
    /// omits it must not bind as the permissive advisory tier.
    #[test]
    fn a_finding_without_a_tier_does_not_bind() {
        let parsed = serde_json::from_value::<AnalysisFinding>(serde_json::json!({
            "family": "grounding",
            "message": "m",
            "disposition": { "outcome": "discarded", "reason": "noise" },
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
            DispositionOutcome::Discarded,
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
            DispositionOutcome::Discarded,
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
}
