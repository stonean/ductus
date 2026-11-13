//! The tier counts an exec `/{project}:analyze` walk records (spec 058, AC26).
//!
//! Interactively, the host tallies each detection step's findings into the
//! tier counts `write-analysis` records, by the tier each step assigns. The
//! exec walker has no host to do that, and its `write-analysis` dispatch once
//! bound no counts at all, so every exec record read 0/0/0 and fully examined
//! whatever detection had found: a clean result standing in for an unexamined
//! one, the conflation `QUAL-CLAIM-001` forbids. This is that tally, and the
//! `unexamined-by-reason` breakdown beside it: what the walk did not examine,
//! including the detection it never runs.
//!
//! It is taken at dispatch rather than read back from the walker context,
//! because the context merges results by bare key and several steps answer
//! under the same one (`findings`), each overwriting the last. Nothing is
//! itemized, so `write-analysis` counts every live finding undispositioned —
//! the walker has no operator to decide one.
//!
//! Each arm restates the tier its step in `framework/commands/analyze.md`
//! assigns, and each unexamined reason the class that file's Unexamined
//! targets section gives it. The procedure is the authority: a step whose
//! tiering changes changes here in the same commit.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Map, Value};

use crate::primitives::write_analysis::{
    REASON_RULE_ASSESSMENTS_NOT_CHECKED, REASON_RULE_FILE_UNREADABLE,
};
use crate::schema::primitives::{FindingTier, RuleVerdict};
use crate::schema::severity::{AnalyzeSeverity, RuleSeverity};
use crate::schema::status::{COMPATIBLE_STATUSES, UNBLOCKING_STATUSES};

/// Steps 13–15 — cross-service references, `## Applicable Rules` citations,
/// and the host's reading of the spec and plan, which judges grounding and the
/// three checks of §Spec integrity and §Plan consistency a primitive cannot
/// make, and the reasons recorded for decisions — by the reason each is recorded under (step 15 under three). They are detection
/// the procedure leaves to the host, and the walker no-ops host prose, so an
/// exec run examines none of them. Each is recorded once, on every run: whether
/// a step would have applied to this spec is itself something the walk never
/// looked at.
const HOST_DETECTION: [&str; 5] = [
    "references-not-checked",
    "applicable-rules-not-checked",
    "grounding-not-checked",
    "integrity-not-checked",
    "decision-reasons-not-checked",
];

/// The running tier counts of one exec analyze walk.
#[derive(Debug)]
pub(crate) struct AnalyzeTally {
    hard_fail: u32,
    blocking: u32,
    advisory: u32,
    /// What the walk did not examine, by reason: the host detection it never
    /// runs, the targets its detection steps report they could not reach, and
    /// the loaded rules steps 11 and 12 did not ask the host about.
    unexamined: BTreeMap<String, u32>,
    /// The rules the walk asked the host about, fired or not (spec 082).
    judged: BTreeSet<String>,
    /// The verdict of each asked rule that fired: a finding, or a pass the
    /// host cited as `addressed-by`.
    verdicts: BTreeMap<String, RuleVerdict>,
}

impl AnalyzeTally {
    /// A tally for a walk that has examined nothing yet, carrying the host
    /// detection it will not run.
    pub(crate) fn new() -> Self {
        Self {
            hard_fail: 0,
            blocking: 0,
            advisory: 0,
            unexamined: HOST_DETECTION
                .iter()
                .map(|reason| ((*reason).to_string(), 1))
                .collect(),
            judged: BTreeSet::new(),
            verdicts: BTreeMap::new(),
        }
    }

    /// Count one detection primitive's result. `context` is the walker context
    /// the step ran against, where step 3 finds the spec's own status in step
    /// 1's `read-spec` result.
    pub(crate) fn record_primitive(
        &mut self,
        name: &str,
        result: &Value,
        context: &Map<String, Value>,
    ) {
        let items = |key: &str| {
            result
                .get(key)
                .and_then(Value::as_array)
                .map_or(&[][..], Vec::as_slice)
        };
        let count = |key: &str| u32::try_from(items(key).len()).unwrap_or(u32::MAX);
        match name {
            // Step 2: each finding in the tier its own condition carries.
            "validate-frontmatter" => self.by_severity(items("findings")),
            // Step 3: a missing dependency and every cycle are blocking; an
            // incompatible dependency is blocking once this spec is at
            // `clarified` or later.
            "traverse-deps" => {
                let status = context
                    .get("frontmatter")
                    .and_then(|frontmatter| frontmatter.get("status"))
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                let past_draft = UNBLOCKING_STATUSES.contains(&status);
                for edge in items("dependencies") {
                    let holds = |key: &str| edge.get(key) == Some(&Value::Bool(true));
                    if !holds("exists") || (past_draft && !holds("compatible")) {
                        self.blocking += 1;
                    }
                }
                self.blocking += count("cycles");
            }
            // Step 4: an unresolved anchor is advisory, and a reference the
            // primitive could not judge is unexamined under its reason: one
            // naming only absent or non-markdown documents excluded by
            // construction, one naming an unreadable document or a skipped
            // registered constitution could not be read (spec 067 scenario
            // `shared-constitution-references-are-judged`).
            "resolve-anchor" => {
                self.advisory += count("unresolved");
                for reason in [
                    crate::primitives::resolve_anchor::UNJUDGEABLE,
                    crate::primitives::resolve_anchor::UNREADABLE,
                ] {
                    let unjudged = result
                        .get(reason)
                        .and_then(Value::as_u64)
                        .map_or(0, |n| u32::try_from(n).unwrap_or(u32::MAX));
                    self.record_unexamined(reason, unjudged);
                }
            }
            // Step 5: a missing citation is blocking and a deprecated one
            // advisory — but `missing` means something only against rule files
            // actually read. With none examined, nothing was checked: a
            // blocking finding raised from that would be against a correct
            // spec, so each citation is recorded unexamined instead.
            "check-rule-ids" => {
                if result.get("examined").and_then(Value::as_u64).unwrap_or(0) > 0 {
                    self.blocking += count("missing");
                    // A shipped rule whose file was never installed is known
                    // and not missing, but its text was not available to
                    // judge the spec against: unexamined, never clean (spec
                    // 033 scenario `an-out-of-surface-citation-resolves`).
                    self.record_unexamined("rule-not-installed", count("not-installed"));
                } else {
                    self.record_unexamined("rule-citations-not-checked", count("citations"));
                }
                self.advisory += count("deprecated");
            }
            // Step 6: drift between body links and `dependencies:` is one
            // advisory finding, and so is each of a non-empty `unwritable`
            // and `unparseable`, which `drift` stays false over. Those two
            // are read from their counts, inline in both forms, an absent
            // `unwritable-count` being zero.
            "derive-dependencies" => {
                if count("updated") > 0 {
                    self.advisory += 1;
                }
                for counted in ["unwritable-count", "unparseable-count"] {
                    if result.get(counted).and_then(Value::as_u64).unwrap_or(0) > 0 {
                        self.advisory += 1;
                    }
                }
            }
            // Step 7: each lint violation is advisory.
            "lint-markdown" => self.advisory += count("violations"),
            // Step 8: each finding in its tier; each skipped target under its
            // reason, never as a finding.
            "check-artifacts" => {
                self.by_severity(items("findings"));
                for skipped in items("skipped") {
                    if let Some(reason) = skipped.get("reason").and_then(Value::as_str) {
                        self.record_unexamined(reason, 1);
                    }
                }
            }
            // Steps 9 and 10: project-level checks, advisory. A referrer
            // step 9 could not read is unexamined, never clean.
            "check-orphaned-references" => {
                self.advisory += count("findings");
                self.record_unexamined("referrer-unreadable", count("skipped"));
            }
            "check-unfolded-specs" => self.advisory += count("unfolded"),
            // Step 16: on a spec at `planned` or later, every plan section
            // outside the design record is advisory, decided or not. Exec
            // matches no stored decision (step 18 is a host step), so a
            // decided section is counted live here, as every finding is.
            "prune-plan" => {
                let status = result.get("status").and_then(Value::as_str);
                if status.is_some_and(|status| COMPATIBLE_STATUSES.contains(&status)) {
                    self.advisory += count("sections");
                }
            }
            // Step 17: every oversized artifact is advisory at every status,
            // decided or not, for the reason step 16's are; a subject the size
            // check could not measure is unexamined, never clean.
            "check-artifact-size" => {
                self.advisory += count("oversized");
                self.record_unexamined("artifact-unreadable", count("skipped"));
            }
            _ => {}
        }
    }

    /// Steps 11 and 12: a failed `assessSpecQuality` assessment joins the
    /// blocking tier for a MUST rule and the advisory tier for a SHOULD rule.
    /// A rule whose trigger does not fire is answered `passed: true` and
    /// counts in no tier.
    ///
    /// The tier is the finding's own when it names MUST or SHOULD and `mixed`
    /// holds (the rule's Statement carries both MUST and SHOULD), otherwise
    /// `asked`, the tier of the rule the walker asked about, which
    /// its Statement states (spec 060): a failure the host reported without a
    /// finding, or with a finding naming `info` or the empty tier, still
    /// failed, and dropping it would record the rule as passed. The finding's
    /// tier comes first so a rule carrying both keywords, asked as MUST-tier,
    /// can still fail advisory on its SHOULD clause alone. It is read through
    /// [`RuleSeverity`], case-insensitively, as validation accepts it.
    ///
    /// The verdict is kept for `write-analysis` (spec 082): a pass carrying
    /// `addressed-by` is `addressed` with that citation, a plain pass is a rule
    /// that did not fire and leaves no verdict, and a failure is a `finding`
    /// at the section its finding names.
    ///
    /// `departed` is whether the spec holds a departure entry for the rule
    /// (spec 073): any fired answer, a failure or a pass citing
    /// `addressed-by`, records the bare `waived` verdict and counts into no
    /// tier, and a plain pass records nothing, as for any rule.
    /// `write-analysis` fills the verdict from the entry.
    pub(crate) fn record_assessment(
        &mut self,
        rule: &str,
        asked: RuleSeverity,
        mixed: bool,
        departed: bool,
        response: &Value,
    ) {
        self.judged.insert(rule.to_string());
        let passed = response.get("passed") == Some(&Value::Bool(true));
        let cited = response
            .get("addressed-by")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|cite| !cite.is_empty());
        if departed {
            if !passed || cited.is_some() {
                self.verdicts
                    .insert(rule.to_string(), RuleVerdict::bare_waived());
            }
            return;
        }
        if passed {
            if let Some(cite) = cited {
                self.verdicts.insert(
                    rule.to_string(),
                    RuleVerdict::Addressed {
                        cite: cite.to_string(),
                    },
                );
            }
            return;
        }
        // The tier the finding is counted in: its own severity when it names
        // MUST or SHOULD and the rule's Statement carries both, else the asked
        // rule's (spec 060), stored on the verdict so a later run reports and
        // floors it in that tier. A single-tier rule can fail only in its own
        // tier: a label cannot move a MUST failure to advisory, and
        // `write-analysis` refuses a finding in a tier its rule cannot fail in.
        let tier = match rule_tier(response, "/finding/severity") {
            Some(tier @ (RuleSeverity::Must | RuleSeverity::Should)) if mixed => tier,
            _ => asked,
        };
        let counted = match tier {
            RuleSeverity::Must => FindingTier::Must,
            _ => FindingTier::Should,
        };
        let section = response
            .pointer("/finding/location/section")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|section| !section.is_empty());
        let message = response
            .pointer("/finding/message")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|message| !message.is_empty());
        self.verdicts.insert(
            rule.to_string(),
            RuleVerdict::Finding {
                tier: counted,
                at: section.map_or_else(|| "spec.md".to_string(), |s| format!("spec.md#{s}")),
                message: message
                    .unwrap_or("the host reported the rule failed and gave no finding")
                    .to_string(),
            },
        );
        match tier {
            RuleSeverity::Must => self.blocking += 1,
            RuleSeverity::Should => self.advisory += 1,
            RuleSeverity::Info | RuleSeverity::Unspecified => {}
        }
    }

    /// Count one stored finding a step reused instead of asking, in the tier
    /// its verdict stored: the walk reports it as it would any finding, and
    /// `write-analysis` carries it forward itself (spec 082).
    pub(crate) fn record_reused(&mut self, tier: FindingTier) {
        match tier {
            FindingTier::Must => self.blocking += 1,
            FindingTier::Should => self.advisory += 1,
        }
    }

    /// Record what loading the walk's rule set could not bring to an
    /// assessment, once per walk rather than once per step, so a rule that
    /// belongs to neither step's tier is not counted twice: each rule file it
    /// could not read under `rule-file-unreadable`, and each rule with no
    /// Verification or no RFC 2119 keyword under
    /// `rule-assessments-not-checked`.
    pub(crate) fn record_rule_set(&mut self, unreadable_files: u32, unassessable: u32) {
        self.record_unexamined(REASON_RULE_FILE_UNREADABLE, unreadable_files);
        self.record_unexamined(REASON_RULE_ASSESSMENTS_NOT_CHECKED, unassessable);
    }

    /// Record an assessment step that asked the host about no rule because it
    /// had none it could: the walk loaded no rule at all, or the step's prose
    /// names no MUST or SHOULD tier to select rules by. Either way the step
    /// examined nothing, and its record must not read as clean.
    pub(crate) fn record_step_asking_nothing(&mut self) {
        self.record_unexamined(REASON_RULE_ASSESSMENTS_NOT_CHECKED, 1);
    }

    /// Bind the tally as `write-analysis`' tier arguments, over any the context
    /// carries: the walk's own detection is the only authority on them.
    pub(crate) fn bind(&self, bindings: &mut Map<String, Value>) {
        bindings.insert("hard-fail".into(), self.hard_fail.into());
        bindings.insert("blocking-findings".into(), self.blocking.into());
        bindings.insert("advisory".into(), self.advisory.into());
        bindings.insert(
            "unexamined-by-reason".into(),
            Value::Array(
                self.unexamined
                    .iter()
                    .map(|(reason, count)| serde_json::json!([reason, count]))
                    .collect(),
            ),
        );
        bindings.insert(
            "rule-verdicts".into(),
            serde_json::to_value(&self.verdicts).unwrap_or(Value::Null),
        );
        bindings.insert(
            "judged-rules".into(),
            Value::Array(self.judged.iter().cloned().map(Value::String).collect()),
        );
    }

    /// Record `count` targets unexamined under `reason`; nothing when zero, so
    /// a step that reached everything adds no entry.
    fn record_unexamined(&mut self, reason: &str, count: u32) {
        if count > 0 {
            *self.unexamined.entry(reason.to_string()).or_default() += count;
        }
    }

    /// Count findings by the analyze severity each carries.
    fn by_severity(&mut self, findings: &[Value]) {
        for finding in findings {
            match finding
                .get("severity")
                .and_then(Value::as_str)
                .and_then(|tier| tier.parse().ok())
            {
                Some(AnalyzeSeverity::HardFail) => self.hard_fail += 1,
                Some(AnalyzeSeverity::Blocking) => self.blocking += 1,
                Some(AnalyzeSeverity::Advisory) => self.advisory += 1,
                Some(AnalyzeSeverity::Informational) | None => {}
            }
        }
    }
}

/// The rule tier at `pointer` in an `assessSpecQuality` payload.
fn rule_tier(payload: &Value, pointer: &str) -> Option<RuleSeverity> {
    payload
        .pointer(pointer)
        .and_then(Value::as_str)
        .and_then(|tier| tier.parse().ok())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use serde_json::json;

    fn bound(tally: &AnalyzeTally) -> Map<String, Value> {
        let mut bindings = Map::new();
        tally.bind(&mut bindings);
        bindings
    }

    /// The host detection an exec walk never runs, as the record lists it.
    fn host_detection() -> Value {
        json!([
            ["applicable-rules-not-checked", 1],
            ["decision-reasons-not-checked", 1],
            ["grounding-not-checked", 1],
            ["integrity-not-checked", 1],
            ["references-not-checked", 1]
        ])
    }

    #[test]
    fn a_walk_records_the_decision_reasons_check_unexamined_once() {
        // Step 15's judgment of the reasons behind recorded decisions is host
        // prose the walker no-ops, so an exec run says it did not examine
        // them, in the class that is a real gap (reference Unexamined
        // targets), however many findings the detection steps tally.
        let mut tally = AnalyzeTally::new();
        let mut context = Map::new();
        context.insert("frontmatter".into(), json!({ "status": "planned" }));
        tally.record_primitive(
            "validate-frontmatter",
            &json!({ "findings": [{ "severity": "blocking" }] }),
            &context,
        );
        let reasons = bound(&tally)["unexamined-by-reason"].clone();
        let recorded = reasons
            .as_array()
            .unwrap()
            .iter()
            .filter(|entry| entry[0] == "decision-reasons-not-checked")
            .collect::<Vec<_>>();
        assert_eq!(recorded, vec![&json!(["decision-reasons-not-checked", 1])]);
    }

    #[test]
    fn each_step_counts_in_the_tier_it_assigns() {
        let mut tally = AnalyzeTally::new();
        let mut context = Map::new();
        context.insert("frontmatter".into(), json!({ "status": "planned" }));
        tally.record_primitive(
            "validate-frontmatter",
            &json!({ "findings": [{ "severity": "hard-fail" }, { "severity": "blocking" }] }),
            &context,
        );
        tally.record_primitive(
            "traverse-deps",
            &json!({
                "dependencies": [
                    { "exists": false, "compatible": false },
                    { "exists": true, "compatible": false },
                    { "exists": true, "compatible": true }
                ],
                "cycles": [["a", "b"]]
            }),
            &context,
        );
        tally.record_primitive(
            "resolve-anchor",
            &json!({ "unresolved": [{}, {}] }),
            &context,
        );
        tally.record_primitive(
            "check-rule-ids",
            &json!({ "missing": ["X-1"], "deprecated": ["Y-1"], "examined": 3 }),
            &context,
        );
        tally.record_primitive(
            "derive-dependencies",
            &json!({ "updated": ["a", "b"] }),
            &context,
        );
        tally.record_primitive("lint-markdown", &json!({ "violations": [{}] }), &context);
        tally.record_primitive(
            "check-artifacts",
            &json!({
                "findings": [{ "severity": "blocking" }, { "severity": "advisory" }],
                "skipped": [{ "reason": "root-absent" }, { "reason": "root-absent" }]
            }),
            &context,
        );
        tally.record_primitive(
            "check-orphaned-references",
            &json!({ "findings": [{}], "skipped": [] }),
            &context,
        );
        tally.record_primitive(
            "check-unfolded-specs",
            &json!({ "unfolded": [{}] }),
            &context,
        );
        tally.record_assessment(
            "TS-RULE-001",
            RuleSeverity::Must,
            true,
            false,
            &json!({ "passed": false, "finding": { "severity": "must" } }),
        );
        tally.record_assessment(
            "TS-RULE-001",
            RuleSeverity::Should,
            true,
            false,
            &json!({ "passed": false, "finding": { "severity": "should" } }),
        );
        tally.record_assessment(
            "TS-RULE-001",
            RuleSeverity::Must,
            true,
            false,
            &json!({ "passed": true }),
        );

        let bindings = bound(&tally);
        assert_eq!(bindings["hard-fail"], 1);
        // frontmatter 1 + two edges + a cycle + a missing citation + an
        // artifact finding + a MUST assessment.
        assert_eq!(bindings["blocking-findings"], 7);
        // anchors 2 + deprecated 1 + drift 1 + lint 1 + artifact 1 + orphan 1
        // + unfolded 1 + a SHOULD assessment.
        assert_eq!(bindings["advisory"], 9);
        assert_eq!(
            bindings["unexamined-by-reason"],
            json!([
                ["applicable-rules-not-checked", 1],
                ["decision-reasons-not-checked", 1],
                ["grounding-not-checked", 1],
                ["integrity-not-checked", 1],
                ["references-not-checked", 1],
                ["root-absent", 2]
            ])
        );
    }

    /// Step 6 counts a non-empty `unwritable` and a non-empty `unparseable`
    /// as one advisory finding each, read from their counts, beside drift,
    /// and nothing when both counts are zero or absent.
    #[test]
    fn unwritable_and_unparseable_dependencies_are_advisory() {
        let cases = [
            (json!({ "updated": [], "unparseable-count": 0 }), 0),
            (
                json!({ "updated": [], "unwritable-count": 2, "unparseable-count": 0 }),
                1,
            ),
            (json!({ "updated": [], "unparseable-count": 3 }), 1),
            (
                json!({ "updated": ["a"], "unwritable-count": 1, "unparseable-count": 1 }),
                3,
            ),
        ];
        for (result, advisory) in cases {
            let mut tally = AnalyzeTally::new();
            tally.record_primitive("derive-dependencies", &result, &Map::new());
            let bindings = bound(&tally);
            assert_eq!(bindings["advisory"], advisory, "{result}");
            assert_eq!(bindings["blocking-findings"], 0, "{result}");
        }
    }

    /// Step 16 counts a plan's sections outside the design record on a
    /// `planned`-or-later spec, decided ones included, and none below it.
    #[test]
    fn plan_sections_outside_the_record_are_advisory_from_planned_on() {
        let sections = json!([{ "decided": true }, { "decided": false }]);
        for status in ["planned", "in-progress", "done"] {
            let mut tally = AnalyzeTally::new();
            tally.record_primitive(
                "prune-plan",
                &json!({ "status": status, "sections": sections }),
                &Map::new(),
            );
            assert_eq!(bound(&tally)["advisory"], 2, "{status}");
        }
        let mut clarified = AnalyzeTally::new();
        clarified.record_primitive(
            "prune-plan",
            &json!({ "status": "clarified", "sections": sections }),
            &Map::new(),
        );
        assert_eq!(bound(&clarified)["advisory"], 0);
    }

    /// Step 17 counts every oversized artifact as advisory at every status,
    /// decided ones included, and a subject it could not measure as
    /// unexamined under `artifact-unreadable`.
    #[test]
    fn oversized_artifacts_are_advisory_and_unmeasured_ones_unexamined() {
        let mut tally = AnalyzeTally::new();
        tally.record_primitive(
            "check-artifact-size",
            &json!({
                "status": "draft",
                "oversized": [{ "decided": true }, { "decided": false }],
                "skipped": [{ "family": "artifact-size", "reason": "artifact-unreadable", "path": "p" }],
            }),
            &Map::new(),
        );
        let bindings = bound(&tally);
        assert_eq!(bindings["advisory"], 2);
        assert_eq!(bindings["blocking-findings"], 0);
        let unexamined = bindings["unexamined-by-reason"].as_array().unwrap();
        assert!(
            unexamined.contains(&json!(["artifact-unreadable", 1])),
            "{unexamined:?}"
        );
    }

    /// Steps 13–15 are host prose the walker never runs, so a walk that
    /// detected nothing still records them unexamined rather than reading as
    /// fully examined.
    #[test]
    fn an_exec_walk_records_the_host_detection_it_never_runs() {
        let bindings = bound(&AnalyzeTally::new());
        assert_eq!(bindings["blocking-findings"], 0);
        assert_eq!(bindings["unexamined-by-reason"], host_detection());
    }

    /// Below `clarified` an incompatible dependency is allowed, and with no
    /// rule file read a citation is unexamined, not missing: nothing was
    /// checked, so nothing is blocking and the record says what went unchecked.
    #[test]
    fn a_draft_spec_raises_nothing_and_an_unread_rule_set_records_its_citations() {
        let mut tally = AnalyzeTally::new();
        let mut context = Map::new();
        context.insert("frontmatter".into(), json!({ "status": "draft" }));
        tally.record_primitive(
            "traverse-deps",
            &json!({ "dependencies": [{ "exists": true, "compatible": false }], "cycles": [] }),
            &context,
        );
        tally.record_primitive(
            "check-rule-ids",
            &json!({
                "citations": [
                    { "rule-id": "X-1", "found": false, "deprecated": false },
                    { "rule-id": "X-2", "found": false, "deprecated": false }
                ],
                "missing": ["X-1", "X-2"],
                "deprecated": [],
                "examined": 0
            }),
            &context,
        );
        let bindings = bound(&tally);
        assert_eq!(bindings["blocking-findings"], 0);
        assert_eq!(bindings["advisory"], 0);
        assert_eq!(
            bindings["unexamined-by-reason"],
            json!([
                ["applicable-rules-not-checked", 1],
                ["decision-reasons-not-checked", 1],
                ["grounding-not-checked", 1],
                ["integrity-not-checked", 1],
                ["references-not-checked", 1],
                ["rule-citations-not-checked", 2]
            ])
        );
    }

    /// An unread rule set is a gap only when the spec cites a rule: with no
    /// citation there was nothing to check against it.
    #[test]
    fn an_unread_rule_set_with_no_citation_records_nothing() {
        let mut tally = AnalyzeTally::new();
        tally.record_primitive(
            "check-rule-ids",
            &json!({ "citations": [], "missing": [], "deprecated": [], "examined": 0 }),
            &Map::new(),
        );
        assert_eq!(bound(&tally)["unexamined-by-reason"], host_detection());
    }

    /// A citation of a shipped rule whose file is not installed is no finding,
    /// and is recorded unexamined, one per citation, beside the blocking
    /// `missing` ones.
    #[test]
    fn a_rule_that_is_not_installed_is_recorded_unexamined_not_blocking() {
        let mut tally = AnalyzeTally::new();
        tally.record_primitive(
            "check-rule-ids",
            &json!({
                "citations": [],
                "missing": ["X-9"],
                "deprecated": [],
                "not-installed": ["FE-XSS-001", "FE-XSS-002"],
                "examined": 1
            }),
            &Map::new(),
        );
        let bindings = bound(&tally);
        assert_eq!(bindings["blocking-findings"], 1);
        assert_eq!(
            bindings["unexamined-by-reason"],
            json!([
                ["applicable-rules-not-checked", 1],
                ["decision-reasons-not-checked", 1],
                ["grounding-not-checked", 1],
                ["integrity-not-checked", 1],
                ["references-not-checked", 1],
                ["rule-not-installed", 2]
            ])
        );
    }

    /// A reference `resolve-anchor` could not judge is unexamined under the
    /// reason its count names, beside the unresolved one counted advisory.
    #[test]
    fn an_unjudged_anchor_reference_is_recorded_unexamined() {
        let mut tally = AnalyzeTally::new();
        tally.record_primitive(
            "resolve-anchor",
            &json!({
                "unresolved": ["gone"],
                "anchor-target-unjudgeable": 3,
                "anchor-target-unreadable": 1
            }),
            &Map::new(),
        );
        let bindings = bound(&tally);
        assert_eq!(bindings["advisory"], 1);
        assert_eq!(
            bindings["unexamined-by-reason"],
            json!([
                ["anchor-target-unjudgeable", 3],
                ["anchor-target-unreadable", 1],
                ["applicable-rules-not-checked", 1],
                ["decision-reasons-not-checked", 1],
                ["grounding-not-checked", 1],
                ["integrity-not-checked", 1],
                ["references-not-checked", 1]
            ])
        );
    }

    /// A referrer `check-orphaned-references` could not read is unexamined:
    /// its empty `findings` for that file is not a clean one.
    #[test]
    fn an_unreadable_referrer_is_recorded_unexamined() {
        let mut tally = AnalyzeTally::new();
        tally.record_primitive(
            "check-orphaned-references",
            &json!({
                "findings": [],
                "skipped": [
                    { "path": "CLAUDE.md", "reason": "file exists but could not be read as UTF-8 text" },
                    { "path": "AGENTS.md", "reason": "file exists but could not be read as UTF-8 text" }
                ]
            }),
            &Map::new(),
        );
        let bindings = bound(&tally);
        assert_eq!(bindings["advisory"], 0);
        assert_eq!(
            bindings["unexamined-by-reason"],
            json!([
                ["applicable-rules-not-checked", 1],
                ["decision-reasons-not-checked", 1],
                ["grounding-not-checked", 1],
                ["integrity-not-checked", 1],
                ["references-not-checked", 1],
                ["referrer-unreadable", 2]
            ])
        );
    }

    /// Validation accepts a capitalized tier, so the tally does too; a
    /// finding counts in its own tier before the asked rule's, so a rule
    /// carrying both keywords, asked as MUST-tier, fails advisory on its
    /// SHOULD clause alone; and a failed assessment with no finding counts in
    /// the asked rule's tier rather than vanishing.
    #[test]
    fn an_assessment_counts_whatever_its_case_and_with_or_without_a_finding() {
        let mut tally = AnalyzeTally::new();
        tally.record_assessment(
            "TS-RULE-001",
            RuleSeverity::Should,
            true,
            false,
            &json!({ "passed": false, "finding": { "severity": "MUST" } }),
        );
        tally.record_assessment(
            "TS-RULE-001",
            RuleSeverity::Must,
            true,
            false,
            &json!({ "passed": false, "finding": { "severity": "Should" } }),
        );
        let failed = json!({ "passed": false });
        tally.record_assessment("TS-RULE-001", RuleSeverity::Must, true, false, &failed);
        tally.record_assessment("TS-RULE-001", RuleSeverity::Should, true, false, &failed);
        tally.record_assessment("TS-RULE-001", RuleSeverity::Info, true, false, &failed);
        let bindings = bound(&tally);
        assert_eq!(bindings["blocking-findings"], 2);
        assert_eq!(bindings["advisory"], 2);
    }

    /// A single-tier rule (`mixed` false) counts a finding in the rule's own
    /// tier whatever the host labelled it; a mixed rule keeps the host's label.
    #[test]
    fn a_single_tier_rule_counts_in_its_own_tier_and_a_mixed_rule_keeps_the_label() {
        let mut tally = AnalyzeTally::new();
        let should = json!({ "passed": false, "finding": { "severity": "should" } });
        let must = json!({ "passed": false, "finding": { "severity": "must" } });
        tally.record_assessment("TS-MUST-001", RuleSeverity::Must, false, false, &should);
        tally.record_assessment("TS-SHOULD-001", RuleSeverity::Should, false, false, &must);
        let bindings = bound(&tally);
        assert_eq!(bindings["blocking-findings"], 1);
        assert_eq!(bindings["advisory"], 1);
        assert_eq!(
            tally.verdicts["TS-MUST-001"],
            RuleVerdict::Finding {
                tier: FindingTier::Must,
                at: "spec.md".into(),
                message: "the host reported the rule failed and gave no finding".into(),
            }
        );
        tally.record_assessment("TS-BOTH-001", RuleSeverity::Must, true, false, &should);
        assert_eq!(bound(&tally)["advisory"], 2);
    }

    /// A failure whose finding names no analyze tier — `info`, or the empty
    /// tier, both of which validation accepts — still failed, so it counts in
    /// the asked rule's tier rather than in none, which would record a failed
    /// rule as passed.
    #[test]
    fn a_failure_whose_finding_names_no_analyze_tier_counts_in_the_asked_tier() {
        let mut tally = AnalyzeTally::new();
        tally.record_assessment(
            "TS-RULE-001",
            RuleSeverity::Must,
            true,
            false,
            &json!({ "passed": false, "finding": { "severity": "info" } }),
        );
        tally.record_assessment(
            "TS-RULE-001",
            RuleSeverity::Should,
            true,
            false,
            &json!({ "passed": false, "finding": { "severity": "" } }),
        );
        let bindings = bound(&tally);
        assert_eq!(bindings["blocking-findings"], 1);
        assert_eq!(bindings["advisory"], 1);
    }

    /// A rule whose trigger does not fire is answered `passed: true`: it was
    /// examined and nothing was found, so it counts in no tier and is not
    /// unexamined either.
    #[test]
    fn a_passed_assessment_counts_nowhere() {
        let mut tally = AnalyzeTally::new();
        tally.record_assessment(
            "TS-RULE-001",
            RuleSeverity::Must,
            true,
            false,
            &json!({ "passed": true }),
        );
        tally.record_assessment(
            "TS-RULE-001",
            RuleSeverity::Should,
            true,
            false,
            &json!({ "passed": true }),
        );
        let bindings = bound(&tally);
        assert_eq!(bindings["blocking-findings"], 0);
        assert_eq!(bindings["advisory"], 0);
        assert_eq!(bindings["unexamined-by-reason"], host_detection());
    }

    /// What loading the rule set could not bring to an assessment is recorded
    /// under its reason, and a rule set with nothing missing adds no entry.
    #[test]
    fn a_rule_set_records_its_unreadable_files_and_unassessable_rules() {
        let mut tally = AnalyzeTally::new();
        tally.record_rule_set(0, 0);
        assert_eq!(bound(&tally)["unexamined-by-reason"], host_detection());
        tally.record_rule_set(1, 2);
        assert_eq!(
            bound(&tally)["unexamined-by-reason"],
            json!([
                ["applicable-rules-not-checked", 1],
                ["decision-reasons-not-checked", 1],
                ["grounding-not-checked", 1],
                ["integrity-not-checked", 1],
                ["references-not-checked", 1],
                ["rule-assessments-not-checked", 2],
                ["rule-file-unreadable", 1]
            ])
        );
    }

    /// A step that asked about no rule examined nothing: it is recorded once,
    /// and raises no finding.
    #[test]
    fn a_step_asking_nothing_is_recorded_unexamined() {
        let mut tally = AnalyzeTally::new();
        tally.record_step_asking_nothing();
        tally.record_step_asking_nothing();
        let bindings = bound(&tally);
        assert_eq!(bindings["blocking-findings"], 0);
        assert_eq!(bindings["advisory"], 0);
        assert_eq!(
            bindings["unexamined-by-reason"],
            json!([
                ["applicable-rules-not-checked", 1],
                ["decision-reasons-not-checked", 1],
                ["grounding-not-checked", 1],
                ["integrity-not-checked", 1],
                ["references-not-checked", 1],
                ["rule-assessments-not-checked", 2]
            ])
        );
    }

    /// Spec 082: each asked rule is judged, a pass citing `addressed-by` is an
    /// `addressed` verdict, a plain pass leaves none, and a failure is a
    /// `finding` at its section with its message; the tally binds both.
    #[test]
    fn each_assessment_leaves_its_verdict_and_judges_its_rule() {
        let mut tally = AnalyzeTally::new();
        tally.record_assessment(
            "TS-A-001",
            RuleSeverity::Must,
            true,
            false,
            &json!({ "passed": true }),
        );
        tally.record_assessment(
            "TS-A-002",
            RuleSeverity::Must,
            true,
            false,
            &json!({ "passed": true, "addressed-by": " plan.md#Observability " }),
        );
        tally.record_assessment(
            "TS-A-003",
            RuleSeverity::Must,
            true,
            false,
            &json!({ "passed": true, "addressed-by": "  " }),
        );
        tally.record_assessment(
            "TS-A-004",
            RuleSeverity::Must,
            true,
            false,
            &json!({ "passed": false, "finding": {
                "severity": "must", "rule-id": "TS-A-004",
                "location": { "section": "Goal", "line": 3 }, "message": "no owner" } }),
        );
        tally.record_assessment(
            "TS-A-005",
            RuleSeverity::Should,
            true,
            false,
            &json!({ "passed": false }),
        );
        let bindings = bound(&tally);
        assert_eq!(
            bindings["judged-rules"],
            json!(["TS-A-001", "TS-A-002", "TS-A-003", "TS-A-004", "TS-A-005"])
        );
        assert_eq!(
            bindings["rule-verdicts"],
            json!({
                "TS-A-002": { "verdict": "addressed", "cite": "plan.md#Observability" },
                "TS-A-004": { "verdict": "finding", "tier": "must", "at": "spec.md#Goal", "message": "no owner" },
                "TS-A-005": { "verdict": "finding", "tier": "should", "at": "spec.md",
                    "message": "the host reported the rule failed and gave no finding" },
            })
        );
        assert_eq!(bindings["blocking-findings"], 1);
        assert_eq!(bindings["advisory"], 1);
    }

    /// A walk that asked about no rule still binds empty verdicts, so nothing
    /// the context holds can stand in for what the walk judged.
    #[test]
    fn a_walk_that_judged_nothing_binds_empty_verdicts() {
        let bindings = bound(&AnalyzeTally::new());
        assert_eq!(bindings["rule-verdicts"], json!({}));
        assert_eq!(bindings["judged-rules"], json!([]));
    }

    /// A reused finding counts in its rule's tier, and no other kind of
    /// reuse counts.
    #[test]
    fn a_reused_finding_counts_in_its_tier() {
        let mut tally = AnalyzeTally::new();
        tally.record_reused(FindingTier::Must);
        tally.record_reused(FindingTier::Should);
        tally.record_reused(FindingTier::Should);
        let bindings = bound(&tally);
        assert_eq!(bindings["blocking-findings"], 1);
        assert_eq!(bindings["advisory"], 2);
    }

    /// Spec 073: a rule the spec departs from records the bare `waived`
    /// verdict on any fired answer, a failure or a pass citing a heading, and
    /// counts into no tier; a plain pass records nothing, and a rule with no
    /// entry fails as it always did.
    #[test]
    fn a_departed_rule_that_fires_is_waived_and_counts_into_no_tier() {
        let mut tally = AnalyzeTally::new();
        let failed = json!({ "passed": false, "finding": { "severity": "must" } });
        tally.record_assessment("TS-RULE-001", RuleSeverity::Must, false, true, &failed);
        tally.record_assessment(
            "TS-RULE-002",
            RuleSeverity::Must,
            false,
            true,
            &json!({ "passed": true, "addressed-by": "spec.md#Behavior" }),
        );
        tally.record_assessment(
            "TS-RULE-003",
            RuleSeverity::Must,
            false,
            true,
            &json!({ "passed": true }),
        );
        tally.record_assessment("TS-RULE-004", RuleSeverity::Should, false, true, &failed);
        let bindings = bound(&tally);
        assert_eq!(bindings["blocking-findings"], 0);
        assert_eq!(bindings["advisory"], 0);
        assert_eq!(
            bindings["rule-verdicts"],
            json!({
                "TS-RULE-001": { "verdict": "waived", "reason": "", "decided-by": "" },
                "TS-RULE-002": { "verdict": "waived", "reason": "", "decided-by": "" },
                "TS-RULE-004": { "verdict": "waived", "reason": "", "decided-by": "" },
            })
        );
        assert_eq!(bindings["judged-rules"].as_array().unwrap().len(), 4);

        tally.record_assessment("TS-RULE-005", RuleSeverity::Must, false, false, &failed);
        assert_eq!(bound(&tally)["blocking-findings"], 1);
    }
}
