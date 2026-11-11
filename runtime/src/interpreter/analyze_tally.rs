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

use std::collections::BTreeMap;

use serde_json::{Map, Value};

use crate::schema::severity::{AnalyzeSeverity, RuleSeverity};
use crate::schema::status::{COMPATIBLE_STATUSES, UNBLOCKING_STATUSES};

/// Steps 13–15 — cross-service references, `## Applicable Rules` citations,
/// and grounding — by the reason each is recorded under. They are detection
/// the procedure leaves to the host, and the walker no-ops host prose, so an
/// exec run examines none of them. Each is recorded once, on every run: whether
/// a step would have applied to this spec is itself something the walk never
/// looked at.
const HOST_DETECTION: [&str; 3] = [
    "references-not-checked",
    "applicable-rules-not-checked",
    "grounding-not-checked",
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
            // Step 4: an unresolved anchor is advisory.
            "resolve-anchor" => self.advisory += count("unresolved"),
            // Step 5: a missing citation is blocking and a deprecated one
            // advisory — but `missing` means something only against rule files
            // actually read. With none examined, nothing was checked: a
            // blocking finding raised from that would be against a correct
            // spec, so each citation is recorded unexamined instead.
            "check-rule-ids" => {
                if result.get("examined").and_then(Value::as_u64).unwrap_or(0) > 0 {
                    self.blocking += count("missing");
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
    /// The tier is the finding's own when it names MUST or SHOULD, and
    /// otherwise `asked`, the tier of the rule the walker asked about, which
    /// its Statement states (spec 060): a failure the host reported without a
    /// finding, or with a finding naming `info` or the empty tier, still
    /// failed, and dropping it would record the rule as passed. The finding's
    /// tier comes first so a rule carrying both keywords, asked as MUST-tier,
    /// can still fail advisory on its SHOULD clause alone. It is read through
    /// [`RuleSeverity`], case-insensitively, as validation accepts it.
    pub(crate) fn record_assessment(&mut self, asked: RuleSeverity, response: &Value) {
        if response.get("passed") == Some(&Value::Bool(true)) {
            return;
        }
        let tier = match rule_tier(response, "/finding/severity") {
            Some(tier @ (RuleSeverity::Must | RuleSeverity::Should)) => tier,
            _ => asked,
        };
        match tier {
            RuleSeverity::Must => self.blocking += 1,
            RuleSeverity::Should => self.advisory += 1,
            RuleSeverity::Info | RuleSeverity::Unspecified => {}
        }
    }

    /// Record what loading the walk's rule set could not bring to an
    /// assessment, once per walk rather than once per step, so a rule that
    /// belongs to neither step's tier is not counted twice: each rule file it
    /// could not read under `rule-file-unreadable`, and each rule with no
    /// Verification or no RFC 2119 keyword under
    /// `rule-assessments-not-checked`.
    pub(crate) fn record_rule_set(&mut self, unreadable_files: u32, unassessable: u32) {
        self.record_unexamined("rule-file-unreadable", unreadable_files);
        self.record_unexamined("rule-assessments-not-checked", unassessable);
    }

    /// Record an assessment step that asked the host about no rule because it
    /// had none it could: the walk loaded no rule at all, or the step's prose
    /// names no MUST or SHOULD tier to select rules by. Either way the step
    /// examined nothing, and its record must not read as clean.
    pub(crate) fn record_step_asking_nothing(&mut self) {
        self.record_unexamined("rule-assessments-not-checked", 1);
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
            ["grounding-not-checked", 1],
            ["references-not-checked", 1]
        ])
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
            RuleSeverity::Must,
            &json!({ "passed": false, "finding": { "severity": "must" } }),
        );
        tally.record_assessment(
            RuleSeverity::Should,
            &json!({ "passed": false, "finding": { "severity": "should" } }),
        );
        tally.record_assessment(RuleSeverity::Must, &json!({ "passed": true }));

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
                ["grounding-not-checked", 1],
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
                ["grounding-not-checked", 1],
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
                ["grounding-not-checked", 1],
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
            RuleSeverity::Should,
            &json!({ "passed": false, "finding": { "severity": "MUST" } }),
        );
        tally.record_assessment(
            RuleSeverity::Must,
            &json!({ "passed": false, "finding": { "severity": "Should" } }),
        );
        let failed = json!({ "passed": false });
        tally.record_assessment(RuleSeverity::Must, &failed);
        tally.record_assessment(RuleSeverity::Should, &failed);
        tally.record_assessment(RuleSeverity::Info, &failed);
        let bindings = bound(&tally);
        assert_eq!(bindings["blocking-findings"], 2);
        assert_eq!(bindings["advisory"], 2);
    }

    /// A failure whose finding names no analyze tier — `info`, or the empty
    /// tier, both of which validation accepts — still failed, so it counts in
    /// the asked rule's tier rather than in none, which would record a failed
    /// rule as passed.
    #[test]
    fn a_failure_whose_finding_names_no_analyze_tier_counts_in_the_asked_tier() {
        let mut tally = AnalyzeTally::new();
        tally.record_assessment(
            RuleSeverity::Must,
            &json!({ "passed": false, "finding": { "severity": "info" } }),
        );
        tally.record_assessment(
            RuleSeverity::Should,
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
        tally.record_assessment(RuleSeverity::Must, &json!({ "passed": true }));
        tally.record_assessment(RuleSeverity::Should, &json!({ "passed": true }));
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
                ["grounding-not-checked", 1],
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
                ["grounding-not-checked", 1],
                ["references-not-checked", 1],
                ["rule-assessments-not-checked", 2]
            ])
        );
    }
}
