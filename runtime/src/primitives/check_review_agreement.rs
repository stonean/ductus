//! `check-review-agreement` — a spec's frontmatter `review:` block agrees with
//! its own `review.md` frontmatter.
//!
//! The same review is recorded twice. `spec.md` carries a `review:` block —
//! `last-run`, `reviewed-against`, the three counts, `blocking`, and any
//! `waivers` — and the `review.md` written beside it carries its own
//! frontmatter with `reviewed-at`, `reviewed-against`, and the same three
//! counts. Nothing held the two together.
//!
//! They drifted, and for weeks. 031 and 041 both carried
//! `should-violations: 1` in `spec.md` while their reports recorded `0`. The
//! drift was invisible because every gate reads exactly one of the two files:
//! `check-review-gate` and `/ductus:analyze`'s review-drift check read the
//! block, and Family 19 resolves `reviewed-against` to decide whether a review
//! predates its code but never compares the counts on either side of it.
//!
//! The root cause in the 031 case was a waiver moved into `review.md` by hand
//! with no matching `review.waivers` entry. The waived finding had no
//! structural existence on the gate's side, so the count it was meant to
//! retire never dropped.
//!
//! The cost runs both ways. A stale non-zero count reads as outstanding review
//! work that does not exist — the signal that sent a maintainer back to
//! re-derive two clean specs before a release tag. A stale zero is worse: it
//! would hide real findings from `check-review-gate`, `/ductus:analyze`, and
//! the `in-progress → done` transition they gate, each trusting a number the
//! report beside it contradicts.
//!
//! ## Why this is a primitive and not a shell script
//!
//! Both records are YAML frontmatter, and the runtime already deserializes
//! frontmatter for a living — so this check needs no parser of its own. The
//! first implementation was an `/audit` family with an embedded `python3`
//! heredoc that hand-rolled one, and its `scalar()` helper used `\s*` after the
//! key; because `\s` matches a newline, an empty value walked the match onto
//! the next line and returned *that* line's content, reporting a spec whose
//! waiver was recorded correctly. [§runtime-boundary] names that shape
//! directly: a script parsing frontmatter has already failed principle 3, and
//! eligibility is a default rather than a permission. `scripts/audit/`'s
//! Family 31 is now the shell entry point over this primitive.
//!
//! [§runtime-boundary]: https://github.com/stonean/ductus/blob/main/framework/constitution.md#runtime-boundary
//!
//! ## Fields on only one side are not compared
//!
//! `diff-base`, `captured-issues`, and `skipped-passes` exist only in the
//! report; `blocking` and `waivers` only in the block. They are not duplicated
//! facts, and demanding they match would invent a binding the artifacts never
//! claimed. The timestamp is the one pair spelled differently on each side
//! (`last-run` / `reviewed-at`), so pairs are keyed by meaning, not by name.
//!
//! ## Subject
//!
//! The intersection — specs carrying both records, the only ones that can
//! disagree. A block with no report belongs to Family 19 and
//! `check-review-gate`; a report with no block is what `check-review-gate`
//! reports as "not reviewed". Those are counted in
//! [`single_sided`] rather than dropped, so a shrinking subject is visible
//! rather than silent, and an empty subject sets [`guidance`] — comparing
//! nothing reports agreement, which is the false green this exists to prevent.
//!
//! [`single_sided`]: crate::schema::primitives::CheckReviewAgreementResult::single_sided
//! [`guidance`]: crate::schema::primitives::CheckReviewAgreementResult::guidance

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

use crate::primitives::{
    Result, list_feature_dirs, parse_atx_heading, read_text, split_frontmatter,
};
use crate::schema::paths;
use crate::schema::primitives::{
    CheckReviewAgreementArgs, CheckReviewAgreementResult, ReviewAgreementFinding,
    ReviewAgreementSkip,
};

/// The five fields both files record, as `(spec-side key, report-side key)`.
/// Keyed by meaning: the timestamp is spelled differently on each side, and
/// pairing by name would silently drop it from the comparison.
const PAIRS: [(&str, &str); 5] = [
    ("last-run", "reviewed-at"),
    ("reviewed-against", "reviewed-against"),
    ("must-violations", "must-violations"),
    ("should-violations", "should-violations"),
    ("low-confidence", "low-confidence"),
];

/// Spec frontmatter, narrowed to the `review:` block.
#[derive(Deserialize, Default)]
struct SpecFm {
    #[serde(default)]
    review: Option<SpecReviewBlock>,
}

/// The block as written, every field optional so a partial block is
/// comparable rather than a parse failure.
#[derive(Deserialize, Default)]
struct SpecReviewBlock {
    #[serde(flatten)]
    fields: BTreeMap<String, serde_norway::Value>,
    #[serde(default)]
    waivers: Vec<RawWaiver>,
}

#[derive(Deserialize)]
struct RawWaiver {
    #[serde(default)]
    rule: Option<String>,
}

/// `review.md` frontmatter, kept open so unknown fields do not fail the parse.
#[derive(Deserialize, Default)]
struct ReportFm {
    #[serde(flatten)]
    fields: BTreeMap<String, serde_norway::Value>,
}

/// Render a YAML scalar the way the frontmatter wrote it, so a finding can
/// quote both sides. `null` and absent both render empty — they mean the same
/// thing to every gate that reads the block.
fn render(value: Option<&serde_norway::Value>) -> String {
    match value {
        None | Some(serde_norway::Value::Null) => String::new(),
        Some(serde_norway::Value::String(s)) => s.clone(),
        Some(serde_norway::Value::Bool(b)) => b.to_string(),
        Some(serde_norway::Value::Number(n)) => n.to_string(),
        Some(other) => serde_norway::to_string(other)
            .unwrap_or_default()
            .trim()
            .to_string(),
    }
}

/// Rule ids under `## Waived findings`, one per `### WAIVED: <rule> — …`.
///
/// Headings come from [`parse_atx_heading`] rather than a line-shape regex, so
/// the section walk is the same one every other primitive uses.
fn waived_rules_in_report(body: &str) -> Vec<String> {
    let mut rules = Vec::new();
    let mut in_section = false;
    for line in body.lines() {
        let Some((level, text)) = parse_atx_heading(line) else {
            continue;
        };
        if level <= 2 {
            in_section = text.eq_ignore_ascii_case("Waived findings");
            continue;
        }
        if !in_section || level != 3 {
            continue;
        }
        let Some(rest) = text.strip_prefix("WAIVED:") else {
            continue;
        };
        // `SIMPLICITY — <summary>` / `BE-AUTHN-001 — <summary>`: the rule is
        // the run before the em-dash separator. Split on the separator rather
        // than on whitespace so a rule id keeps its internal hyphens.
        let rule = rest
            .split(" — ")
            .next()
            .unwrap_or(rest)
            .trim()
            .trim_matches('`')
            .to_string();
        if !rule.is_empty() {
            rules.push(rule);
        }
    }
    rules
}

/// Compare one spec's `review:` block against its report's frontmatter.
///
/// Split out of [`run`] so the enumeration loop stays readable: the three
/// checks below are the whole contract, and reading them should not mean
/// scrolling past the file-reading and skip-classification above.
fn compare_records(
    feature: &str,
    rel_spec: &str,
    rel_report: &str,
    block: &SpecReviewBlock,
    report_fm: &ReportFm,
    report_body: &str,
    findings: &mut Vec<ReviewAgreementFinding>,
) {
    // The five paired fields.
    for (spec_key, report_key) in PAIRS {
        let spec_value = render(block.fields.get(spec_key));
        let report_value = render(report_fm.fields.get(report_key));
        if spec_value == report_value {
            continue;
        }
        findings.push(ReviewAgreementFinding {
            feature: feature.to_string(),
            kind: "field-mismatch".into(),
            field: spec_key.into(),
            spec_value: spec_value.clone(),
            report_value: report_value.clone(),
            location: rel_spec.to_string(),
            message: format!(
                "review.{spec_key} is {spec_value:?} but {rel_report}'s {report_key} is \
                 {report_value:?} — the same review recorded twice, disagreeing"
            ),
            fix: format!(
                "re-derive the spec's `review:` block from {rel_report}, which is the \
                 source of record; re-run /ductus:review if the report itself is out of date"
            ),
        });
    }

    // `blocking` agrees with `must-violations`. Only this direction is
    // asserted: the reverse is a stuck gate, which surfaces on its own
    // rather than silently letting a spec through.
    let blocking = block
        .fields
        .get("blocking")
        .and_then(serde_norway::Value::as_bool)
        .unwrap_or(false);
    let must = block
        .fields
        .get("must-violations")
        .and_then(serde_norway::Value::as_u64)
        .unwrap_or(0);
    if !blocking && must > 0 {
        findings.push(ReviewAgreementFinding {
            feature: feature.to_string(),
            kind: "blocking-mismatch".into(),
            field: String::new(),
            spec_value: format!("blocking=false must-violations={must}"),
            report_value: String::new(),
            location: rel_spec.to_string(),
            message: format!(
                "review.blocking is false while review.must-violations is {must} — the \
                 block says the spec may advance and may not at once"
            ),
            fix: "set `blocking: true`, or correct the count if the violations are \
                  resolved or waived, then re-run /ductus:review"
                .into(),
        });
    }

    // Every waived finding has a waiver entry. Matched on rule id alone:
    // the report renders a file with a line range while the waiver anchors
    // a bare path, and a spurious finding here would send a maintainer
    // editing a waiver that is already correct.
    let recorded: Vec<String> = block
        .waivers
        .iter()
        .filter_map(|w| w.rule.clone())
        .collect();
    for rule in waived_rules_in_report(report_body) {
        if recorded.iter().any(|r| r == &rule) {
            continue;
        }
        findings.push(ReviewAgreementFinding {
            feature: feature.to_string(),
            kind: "orphan-waiver".into(),
            field: rule.clone(),
            spec_value: String::new(),
            report_value: rule.clone(),
            location: rel_spec.to_string(),
            message: format!(
                "{rel_report} waives {rule} but no `review.waivers` entry records it — \
                 the waiver has no structural existence, so no gate can see it"
            ),
            fix: format!(
                "add a `review.waivers` entry for {rule} (rule, file, reason), or run \
                 /ductus:review --waive {rule} --reason \"...\""
            ),
        });
    }
}

/// What one feature directory yielded — the classification the enumeration
/// loop acts on.
///
/// An enum rather than a ladder of `continue`s because the four non-comparing
/// outcomes are the interesting part of this check's contract: three of them
/// are deliberately *not* findings, and which is which is the thing a reader
/// needs to see at a glance.
enum Load {
    /// No `spec.md` — not a spec directory.
    NoSpec,
    /// Exactly one of the two records exists. Not this check's defect: a block
    /// with no report belongs to Family 19 and `check-review-gate`, and a
    /// report with no block is what `check-review-gate` calls "not reviewed".
    SingleSided,
    /// The spec side could not be read or parsed at all.
    Skip(ReviewAgreementSkip),
    /// The report exists but its frontmatter does not parse. A finding rather
    /// than a skip: the record is present and unreadable, which is not the
    /// same as agreeing. `feature` is filled in by the caller.
    Unparseable(Box<ReviewAgreementFinding>),
    /// Both records parsed; the body is carried for the waiver scan.
    Both(Box<SpecReviewBlock>, Box<ReportFm>, String),
}

/// Read and classify one feature's two review records.
///
/// Split out of [`run`] so the enumeration loop reads as the classification it
/// is, rather than as file-reading interleaved with policy.
fn load_records(dir: &Path, rel_spec: &str, rel_report: &str) -> Load {
    let spec_path = dir.join("spec.md");
    let report_path = dir.join("review.md");

    if !spec_path.is_file() {
        return Load::NoSpec;
    }
    let report_exists = report_path.is_file();

    let Ok(spec_text) = read_text(&spec_path) else {
        return Load::Skip(ReviewAgreementSkip {
            path: rel_spec.to_string(),
            reason: "spec.md could not be read".into(),
        });
    };
    let Ok((spec_fm_text, _)) = split_frontmatter(&spec_text, &spec_path) else {
        return Load::Skip(ReviewAgreementSkip {
            path: rel_spec.to_string(),
            reason: "spec.md has no parseable frontmatter block".into(),
        });
    };
    let spec_fm: SpecFm = match serde_norway::from_str(spec_fm_text) {
        Ok(parsed) => parsed,
        Err(err) => {
            return Load::Skip(ReviewAgreementSkip {
                path: rel_spec.to_string(),
                reason: format!("spec.md frontmatter is not valid YAML: {err}"),
            });
        }
    };

    let Some(block) = spec_fm.review else {
        return if report_exists {
            Load::SingleSided
        } else {
            Load::NoSpec
        };
    };
    if !report_exists {
        return Load::SingleSided;
    }

    let Ok(report_text) = read_text(&report_path) else {
        return Load::Skip(ReviewAgreementSkip {
            path: rel_report.to_string(),
            reason: "review.md could not be read".into(),
        });
    };
    let Ok((report_fm_text, report_body)) = split_frontmatter(&report_text, &report_path) else {
        return Load::Unparseable(Box::new(unparseable_finding(
            rel_report,
            "review.md has no parseable frontmatter — its record cannot be compared, which is \
             not the same as agreeing"
                .into(),
            "restore the `---` frontmatter block, or re-run /ductus:review to regenerate the \
             report",
        )));
    };
    let report_fm: ReportFm = match serde_norway::from_str(report_fm_text) {
        Ok(parsed) => parsed,
        Err(err) => {
            return Load::Unparseable(Box::new(unparseable_finding(
                rel_report,
                format!(
                    "review.md frontmatter is not valid YAML ({err}) — its record cannot be \
                     compared, which is not the same as agreeing"
                ),
                "repair the frontmatter, or re-run /ductus:review to regenerate the report",
            )));
        }
    };

    Load::Both(
        Box::new(block),
        Box::new(report_fm),
        report_body.to_string(),
    )
}

/// An `unparseable` finding with the feature left for the caller to fill in.
fn unparseable_finding(rel_report: &str, message: String, fix: &str) -> ReviewAgreementFinding {
    ReviewAgreementFinding {
        feature: String::new(),
        kind: "unparseable".into(),
        field: String::new(),
        spec_value: String::new(),
        report_value: String::new(),
        location: rel_report.to_string(),
        message,
        fix: fix.into(),
    }
}

/// Execute the `check-review-agreement` primitive.
///
/// # Errors
///
/// Never returns an error for an unreadable or unparseable spec — that is a
/// [`ReviewAgreementSkip`], so an empty `findings` is not mistaken for a
/// verified corpus.
pub fn run(_args: &CheckReviewAgreementArgs, repo: &Path) -> Result<CheckReviewAgreementResult> {
    let specs_root = paths::Paths::load(repo).specs_root;
    let specs_dir = repo.join(&specs_root);

    let mut examined = Vec::new();
    let mut single_sided = Vec::new();
    let mut skipped = Vec::new();
    let mut findings = Vec::new();

    for feature in list_feature_dirs(&specs_dir) {
        let dir = specs_dir.join(&feature);
        let rel_spec = format!("{specs_root}/{feature}/spec.md");
        let rel_report = format!("{specs_root}/{feature}/review.md");

        match load_records(&dir, &rel_spec, &rel_report) {
            Load::NoSpec => {}
            Load::SingleSided => single_sided.push(feature),
            Load::Skip(skip) => skipped.push(skip),
            Load::Unparseable(finding) => {
                let mut finding = *finding;
                finding.feature.clone_from(&feature);
                findings.push(finding);
            }
            Load::Both(block, report_fm, report_body) => {
                examined.push(feature.clone());
                compare_records(
                    &feature,
                    &rel_spec,
                    &rel_report,
                    &block,
                    &report_fm,
                    &report_body,
                    &mut findings,
                );
            }
        }
    }

    let guidance = if examined.is_empty() {
        format!(
            "no spec under {specs_root}/ carries both a `review:` block and a review.md — \
             the enumeration or the frontmatter parse broke, and comparing nothing reports \
             agreement"
        )
    } else {
        String::new()
    };

    Ok(CheckReviewAgreementResult {
        findings,
        examined,
        single_sided,
        skipped,
        specs_root,
        guidance,
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use tempfile::{TempDir, tempdir};

    /// A spec dir with the given frontmatter bodies. `report` of `None` writes
    /// no `review.md` at all.
    fn seed(repo: &Path, feature: &str, spec_review: &str, report: Option<&str>) {
        let dir = repo.join("specs").join(feature);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("spec.md"),
            format!("---\nstatus: done\n{spec_review}---\n\n# {feature}\n"),
        )
        .unwrap();
        if let Some(body) = report {
            fs::write(dir.join("review.md"), body).unwrap();
        }
    }

    fn repo() -> TempDir {
        tempdir().unwrap()
    }

    const CLEAN_BLOCK: &str = "review:\n  last-run: 2026-01-01T00:00:00Z\n  \
                               reviewed-against: abc123\n  must-violations: 0\n  \
                               should-violations: 0\n  low-confidence: 0\n  blocking: false\n";
    const CLEAN_REPORT: &str = "---\nspec: 001-x\nreviewed-at: 2026-01-01T00:00:00Z\n\
                                reviewed-against: abc123\nmust-violations: 0\n\
                                should-violations: 0\nlow-confidence: 0\n---\n\n# Review\n";

    fn run_in(dir: &TempDir) -> CheckReviewAgreementResult {
        run(&CheckReviewAgreementArgs {}, dir.path()).unwrap()
    }

    #[test]
    fn agreeing_records_produce_no_finding() {
        let dir = repo();
        seed(dir.path(), "001-x", CLEAN_BLOCK, Some(CLEAN_REPORT));
        let out = run_in(&dir);
        assert!(out.findings.is_empty(), "{:?}", out.findings);
        assert_eq!(out.examined, vec!["001-x".to_string()]);
        assert!(out.guidance.is_empty());
    }

    #[test]
    fn count_divergence_is_reported_in_either_direction() {
        for (block_count, report_count) in [("1", "0"), ("0", "7")] {
            let dir = repo();
            let block = CLEAN_BLOCK.replace(
                "should-violations: 0",
                &format!("should-violations: {block_count}"),
            );
            let report = CLEAN_REPORT.replace(
                "should-violations: 0",
                &format!("should-violations: {report_count}"),
            );
            seed(dir.path(), "001-x", &block, Some(&report));
            let out = run_in(&dir);
            assert_eq!(out.findings.len(), 1, "{:?}", out.findings);
            assert_eq!(out.findings[0].kind, "field-mismatch");
            assert_eq!(out.findings[0].field, "should-violations");
            assert_eq!(out.findings[0].spec_value, block_count);
            assert_eq!(out.findings[0].report_value, report_count);
        }
    }

    #[test]
    fn timestamp_pairs_across_the_two_spellings() {
        let dir = repo();
        let report = CLEAN_REPORT.replace(
            "reviewed-at: 2026-01-01T00:00:00Z",
            "reviewed-at: 2026-02-02T00:00:00Z",
        );
        seed(dir.path(), "001-x", CLEAN_BLOCK, Some(&report));
        let out = run_in(&dir);
        assert_eq!(out.findings.len(), 1);
        assert_eq!(out.findings[0].field, "last-run");
    }

    #[test]
    fn an_empty_value_never_reads_the_next_line() {
        // The defect that motivated moving this out of a shell script: a
        // regex whose whitespace run matched a newline returned the *next*
        // key's line as this key's value. serde cannot express that bug, and
        // this test is what says so.
        let dir = repo();
        let block = "review:\n  last-run:\n  must-violations: 7\n  blocking: true\n";
        let report = "---\nreviewed-at:\nmust-violations: 7\n---\n\n# Review\n";
        seed(dir.path(), "001-x", block, Some(report));
        let out = run_in(&dir);
        let mismatches: Vec<_> = out
            .findings
            .iter()
            .filter(|f| f.kind == "field-mismatch")
            .collect();
        assert!(mismatches.is_empty(), "{mismatches:?}");
    }

    #[test]
    fn blocking_false_with_must_violations_is_reported() {
        let dir = repo();
        let block = CLEAN_BLOCK.replace("must-violations: 0", "must-violations: 2");
        let report = CLEAN_REPORT.replace("must-violations: 0", "must-violations: 2");
        seed(dir.path(), "001-x", &block, Some(&report));
        let out = run_in(&dir);
        assert_eq!(out.findings.len(), 1, "{:?}", out.findings);
        assert_eq!(out.findings[0].kind, "blocking-mismatch");
    }

    #[test]
    fn waived_finding_without_a_waiver_entry_is_reported() {
        let dir = repo();
        let report = format!(
            "{}\n## Waived findings\n\n### WAIVED: SIMPLICITY — something\n\n## Skipped passes\n",
            CLEAN_REPORT.trim_end()
        );
        seed(dir.path(), "001-x", CLEAN_BLOCK, Some(&report));
        let out = run_in(&dir);
        assert_eq!(out.findings.len(), 1, "{:?}", out.findings);
        assert_eq!(out.findings[0].kind, "orphan-waiver");
        assert_eq!(out.findings[0].field, "SIMPLICITY");
    }

    #[test]
    fn a_recorded_waiver_silences_the_orphan_finding() {
        let dir = repo();
        let block = format!(
            "{CLEAN_BLOCK}  waivers:\n    - rule: SIMPLICITY\n      file: a.md\n      reason: because\n"
        );
        let report = format!(
            "{}\n## Waived findings\n\n### WAIVED: SIMPLICITY — something\n",
            CLEAN_REPORT.trim_end()
        );
        seed(dir.path(), "001-x", &block, Some(&report));
        let out = run_in(&dir);
        assert!(out.findings.is_empty(), "{:?}", out.findings);
    }

    #[test]
    fn empty_subject_sets_guidance_rather_than_reporting_clean() {
        let dir = repo();
        fs::create_dir_all(dir.path().join("specs")).unwrap();
        let out = run_in(&dir);
        assert!(out.findings.is_empty());
        assert!(out.examined.is_empty());
        assert!(
            !out.guidance.is_empty(),
            "an empty subject must not read as clean"
        );
    }

    #[test]
    fn single_sided_specs_are_counted_not_dropped() {
        let dir = repo();
        seed(dir.path(), "001-x", CLEAN_BLOCK, Some(CLEAN_REPORT));
        seed(dir.path(), "002-block-only", CLEAN_BLOCK, None);
        seed(dir.path(), "003-report-only", "", Some(CLEAN_REPORT));
        let out = run_in(&dir);
        assert!(out.findings.is_empty(), "{:?}", out.findings);
        assert_eq!(out.examined, vec!["001-x".to_string()]);
        assert_eq!(
            out.single_sided,
            vec!["002-block-only".to_string(), "003-report-only".to_string()]
        );
    }

    #[test]
    fn unparseable_report_frontmatter_is_a_finding_not_a_skip() {
        let dir = repo();
        seed(
            dir.path(),
            "001-x",
            CLEAN_BLOCK,
            Some("# Review\n\nno frontmatter\n"),
        );
        let out = run_in(&dir);
        assert_eq!(out.findings.len(), 1, "{:?}", out.findings);
        assert_eq!(out.findings[0].kind, "unparseable");
    }
}
