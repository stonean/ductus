//! Per-primitive args + result shapes.
//!
//! Mirrors the canonical JSON shapes in
//! `specs/022-deterministic-runtime/data-model.md`. Each primitive has an
//! `…Args` struct (also the `clap`-derive shape for the CLI surface) and a
//! `…Result` struct. JSON field names are kebab-case across the surface.

#![allow(clippy::module_name_repetitions, clippy::struct_excessive_bools)]

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// -- read-spec ---------------------------------------------------------------

/// Args for `read-spec`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct ReadSpecArgs {
    /// Feature directory name under `specs/`.
    #[arg(long)]
    pub feature: String,
    /// Whether to populate `sections[].body`.
    #[serde(default)]
    #[arg(long)]
    pub include_body: bool,
}

/// Frontmatter review block (initial-release fields).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ReviewBlock {
    /// ISO-8601 UTC timestamp of the last `/ductus:review`, if any.
    #[serde(default)]
    pub last_run: Option<String>,
    /// Constitution sha the review was run against.
    #[serde(default)]
    pub reviewed_against: Option<String>,
    /// MUST violations from the last review.
    #[serde(default)]
    pub must_violations: u32,
    /// SHOULD violations from the last review.
    #[serde(default)]
    pub should_violations: u32,
    /// Low-confidence findings from the last review.
    #[serde(default)]
    pub low_confidence: u32,
    /// Whether the last review left blocking findings.
    #[serde(default)]
    pub blocking: bool,
}

/// Parsed `analyze:` frontmatter block — the durable record that
/// `/ductus:analyze` ran, and what it found.
///
/// The counterpart to [`ReviewBlock`], and it exists because there was no
/// counterpart. `check-review-gate` read the `review:` block, Family 19
/// checked review freshness, and nothing recorded the *other* gate at all —
/// so a spec that had passed both gates and one that had passed only the
/// review were byte-identical on disk. The pipeline is
/// `implement → review → analyze → done`, and half of it left no trace,
/// which meant the only thing standing between a spec and a skipped audit
/// was whoever remembered — the diligence dependency §design-principles
/// rejects.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct AnalyzeBlock {
    /// ISO-8601 UTC timestamp of the last `/ductus:analyze`, if any.
    #[serde(default)]
    pub last_run: Option<String>,
    /// HEAD sha the analysis ran against.
    #[serde(default)]
    pub analyzed_against: Option<String>,
    /// Hard-fail findings (malformed frontmatter, missing required fields).
    #[serde(default)]
    pub hard_fail: u32,
    /// Blocking findings from the run's blocking tier.
    ///
    /// Named `blocking-findings` rather than `blocking` because this block
    /// also carries the boolean gate flag its sibling `review:` block spells
    /// `blocking`, and two fields differing only in type is the shape a
    /// reader mis-reads.
    #[serde(default)]
    pub blocking_findings: u32,
    /// Advisory findings.
    ///
    /// Recorded but **not** gated on, and the asymmetry with `review:` is
    /// deliberate rather than an oversight. An outstanding SHOULD blocks
    /// `done` because §implement-phase says advisory is not ignorable at the
    /// review gate. Analyze's advisory tier is a different contract: its
    /// members are checks explicitly introduced advisory *with promotion
    /// criteria* — grounding, Applicable-Rules citations, decision drift —
    /// where blocking before the signal is proven is the failure mode each
    /// one names. Gating on them here would promote all of them at once,
    /// past the criteria they each declare.
    #[serde(default)]
    pub advisory: u32,
    /// Targets a family could not examine (the run's `skipped` set).
    ///
    /// The `QUAL-CLAIM-001` field, and the reason this block is not just a
    /// copy of `review:`. A clean analyze is two different states — every
    /// target examined and clean, or some target unexaminable and the rest
    /// clean — and the command's own contract says so: "clean with nothing
    /// skipped is verified-clean, clean with something skipped is partially
    /// examined." A record carrying only the finding counts would collapse
    /// exactly that distinction into the reassuring reading, in the record
    /// a later gate trusts.
    #[serde(default)]
    pub unexamined: u32,
    /// Whether the last analysis left findings that hold the spec out of
    /// `done` — `hard-fail` or `blocking-findings` above zero.
    #[serde(default)]
    pub blocking: bool,
}

// -- discover-rule-files -----------------------------------------------------

/// Args for `discover-rule-files`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct DiscoverRuleFilesArgs {
    /// Surfaces detected by the host's stack detection, consulted ONLY when
    /// `.ductus/config.toml` `[rules] surfaces` is unset. Members are `backend`
    /// and/or `frontend`. When the config key is set it wins; when both are
    /// absent, every recognized surface is loaded.
    #[serde(default)]
    #[arg(long = "detected-surface")]
    pub detected_surfaces: Vec<String>,
}

/// Result for `discover-rule-files`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct DiscoverRuleFilesResult {
    /// Repo-relative rule-file directory that was listed (`framework/rules`
    /// in ductus's own repo, `{specs-root}/rules` in adopters). Empty when
    /// neither exists.
    pub rules_dir: String,
    /// Selected rule-file basenames, sorted, after surface selection and the
    /// disabled-rule-files filter.
    pub selected: Vec<String>,
    /// Ordered stdout notice lines to emit verbatim: unrecognized-suffix
    /// warnings, then disabled-rule-file notices, then the closing
    /// `loading rule files: …` line.
    pub notices: Vec<String>,
}

// -- process-waivers ---------------------------------------------------------

/// A currently-firing `(rule, file)` finding — input to `process-waivers`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct FiredFinding {
    /// Rule ID that fires.
    pub rule: String,
    /// Repo-relative file path where it fires.
    pub file: String,
}

/// A resolved waiver reference in a `process-waivers` result.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct WaiverRef {
    /// Waived rule ID.
    pub rule: String,
    /// Anchored file path.
    pub file: String,
    /// Operator-supplied justification.
    pub reason: String,
}

/// Args for `process-waivers`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct ProcessWaiversArgs {
    /// Feature directory name whose `spec.md` carries `review.waivers`.
    #[arg(long)]
    pub feature: String,
    /// Currently-firing `(rule, file)` findings from the review passes.
    /// Supplied via MCP/interpreter JSON; not a CLI flag.
    #[serde(default)]
    #[arg(skip)]
    pub fired: Vec<FiredFinding>,
    /// Dimensions skipped this run (via `--security` / `--simplicity` /
    /// `--quality`). When non-empty, the run is dimension-restricted and
    /// lacks the full-review picture, so a non-firing waiver is **retained**
    /// (left untouched) rather than expired — a waiver anchored to a skipped
    /// dimension must not be pruned on the strength of a partial run. Only an
    /// unrestricted run (this list empty) expires waivers.
    #[serde(default)]
    #[arg(long = "skipped-pass")]
    pub skipped_passes: Vec<String>,
}

/// Result for `process-waivers`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ProcessWaiversResult {
    /// Waivers that apply this run (anchor exists and the rule still fires).
    pub applied: Vec<WaiverRef>,
    /// Waivers that expired this run (anchor gone or rule no longer fires);
    /// `write-review` drops these on the next frontmatter write. Always empty
    /// on a dimension-restricted run (see `skipped-passes`).
    pub expired: Vec<WaiverRef>,
    /// Waivers left untouched this run because it was dimension-restricted and
    /// they did not fire against the passes that ran — neither applied nor
    /// expired, so `write-review` keeps them in the spec frontmatter.
    pub retained: Vec<WaiverRef>,
    /// Ordered notice lines: `waiver expired: …`, `waiver retained: …`,
    /// `malformed waiver …`, and `duplicate waiver: …`, in entry order.
    pub notices: Vec<String>,
}

// -- compute-review-scope ----------------------------------------------------

/// Args for `compute-review-scope`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct ComputeReviewScopeArgs {
    /// Feature directory name whose review scope is computed.
    #[arg(long)]
    pub feature: String,
    /// Optional diff-base override (a git ref or sha). When omitted, the
    /// commit the spec advanced to `in-progress` at is used.
    #[serde(default)]
    #[arg(long)]
    pub since: Option<String>,
}

/// Result for `compute-review-scope`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ComputeReviewScopeResult {
    /// Resolved diff-base sha (empty when the spec never reached `in-progress`
    /// and no `--since` was given).
    pub diff_base: String,
    /// The review scope: the union of `plan-affected` and `modified-since`.
    pub scope: Vec<String>,
    /// Files changed between `diff-base` and HEAD, sorted.
    pub modified_since: Vec<String>,
    /// Files listed under the plan's `## Affected Files` section.
    pub plan_affected: Vec<String>,
    /// Lines added to `{specs-root}/inbox.md` in the `diff-base..HEAD` window.
    pub captured_issues: Vec<String>,
}

// -- write-review ------------------------------------------------------------

/// One review finding — the record shape a `performReview` pass returns and
/// `write-review` consumes. `rule` / `severity` / `file` / `line-range` /
/// `confidence` are the extension-point contract; the render extras
/// (`summary` / `finding` / `rule-text` / `auto-fixable` / `suggested-fix`)
/// populate the per-finding block in `review.md` and default to empty so a
/// minimal finding still deserializes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ReviewFinding {
    /// Rule ID (e.g., "SEC-BE-014").
    pub rule: String,
    /// Severity tier: `must` or `should`.
    pub severity: String,
    /// Repo-relative file path the finding anchors to.
    pub file: String,
    /// Line range within the file (e.g., "42-55" or "42"); empty means the
    /// whole file (overlaps any range with the same rule + file for dedup).
    #[serde(default)]
    pub line_range: String,
    /// Confidence tier: `high` or `low`. A `low` finding lands in the
    /// Low-confidence section regardless of severity.
    #[serde(default)]
    pub confidence: String,
    /// One-line finding summary (the `### … — <summary>` heading tail).
    #[serde(default)]
    pub summary: String,
    /// One-to-three-sentence explanation.
    #[serde(default)]
    pub finding: String,
    /// Verbatim rule text quoted from the rule file.
    #[serde(default)]
    pub rule_text: String,
    /// Whether a mechanical auto-fix exists.
    #[serde(default)]
    pub auto_fixable: bool,
    /// Suggested fix (code block or prose); omitted from the render when empty.
    #[serde(default)]
    pub suggested_fix: String,
}

/// One review **observation** — something the reviewer judged real that maps
/// to no loaded rule, so it cannot be a [`ReviewFinding`]. Observations never
/// enter the MUST / SHOULD / low-confidence counts and never affect
/// `blocking`; `write-review` renders them in their own report section and
/// appends each one to the inbox in the same call, so recording an observation
/// *is* capturing it (spec 022 scenario `review-observations-write-through`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ReviewObservation {
    /// What was observed, as one line of prose. Leading it with a category
    /// (`security` / `leak` / `convention` / `bug` / `perf` / `other`) matches
    /// the inbox template's auto-capture form, but nothing parses it.
    pub text: String,
    /// Repo-relative path (optionally `file:line`) the observation anchors to;
    /// empty when it is not anchored to one file.
    #[serde(default)]
    pub path: String,
}

/// Args for `write-review`. Findings cross the runtime boundary as a single
/// `findings` array (the content-ingestion convention), never as several
/// large per-section prose params.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct WriteReviewArgs {
    /// Feature directory name whose `review.md` is written.
    #[arg(long)]
    pub feature: String,
    /// ISO-8601 UTC timestamp recorded as `reviewed-at` / `review.last-run`.
    #[arg(long)]
    pub reviewed_at: String,
    /// HEAD sha the review ran against (`reviewed-against`).
    #[arg(long)]
    pub reviewed_against: String,
    /// diff-base sha from `compute-review-scope` (recorded in the report).
    #[arg(long)]
    pub diff_base: String,
    /// Scenario slug, when the run was scenario-targeted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub scenario: Option<String>,
    /// When true, render the "nothing to review yet" empty-scope report.
    #[serde(default)]
    #[arg(long)]
    pub empty_scope: bool,
    /// Optional Summary override; a deterministic count line is generated when
    /// absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub summary: Option<String>,
    /// Dimensions skipped this run (via `--security` / `--simplicity` / …);
    /// echoed to `skipped-passes` and omitted from the counts.
    #[serde(default)]
    #[arg(long = "skipped-pass")]
    pub skipped_passes: Vec<String>,
    /// Pass findings as a single array (the content-ingestion convention).
    /// Supplied via MCP/interpreter JSON; not a CLI flag.
    #[serde(default)]
    #[arg(skip)]
    pub findings: Vec<ReviewFinding>,
    /// Applied waivers from `process-waivers`; matching findings are excluded
    /// from the counts and listed under Waived findings. The `alias` reads
    /// `process-waivers`' `applied` result key directly, so the waiver set
    /// threads through the exec walker's context (which merges primitive
    /// results by their bare key) as well as the MCP/host path.
    #[serde(default, alias = "applied")]
    #[arg(skip)]
    pub applied_waivers: Vec<WaiverRef>,
    /// Expired waivers from `process-waivers`; dropped from the spec
    /// frontmatter `review.waivers` list on this write. The `alias` reads
    /// `process-waivers`' `expired` result key (see `applied_waivers`).
    #[serde(default, alias = "expired")]
    #[arg(skip)]
    pub expired_waivers: Vec<WaiverRef>,
    /// Inbox additions in the review window from `compute-review-scope`;
    /// listed under Captured issues (informational).
    #[serde(default)]
    #[arg(skip)]
    pub captured_issues: Vec<String>,
    /// Reviewer observations that map to no loaded rule. Excluded from every
    /// count and from `blocking`; each is appended to the inbox by this same
    /// call, so the report and the inbox cannot diverge. Supplied via
    /// MCP/interpreter JSON; not a CLI flag.
    #[serde(default)]
    #[arg(skip)]
    pub observations: Vec<ReviewObservation>,
}

/// Result for `write-review`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct WriteReviewResult {
    /// Repo-relative path of the `review.md` written.
    pub path: String,
    /// Repo-relative path of the spec file whose `review:` block was updated.
    pub spec_path: String,
    /// MUST violations counted (waived findings excluded).
    pub must_violations: u32,
    /// SHOULD violations counted (waived findings excluded).
    pub should_violations: u32,
    /// Low-confidence findings counted.
    pub low_confidence: u32,
    /// Findings excluded by an applied waiver.
    pub waived: u32,
    /// Observations rendered in the report's `## Observations` section.
    pub observations: u32,
    /// Observations newly appended to the inbox by this call; the remainder of
    /// `observations` were already there and deduped. Reported so a caller can
    /// tell "nothing to capture" from "capture ran and found duplicates".
    pub observations_captured: u32,
    /// `true` when `must-violations` exceeds zero.
    pub blocking: bool,
    /// Derived exit code: 1 when blocking, else 0.
    pub exit_code: i32,
}

// -- write-analysis ----------------------------------------------------------

/// Args for `write-analysis` — record that `/ductus:analyze` ran, and what it
/// found, in the spec's `analyze:` frontmatter block.
///
/// The narrow, always-on write that makes analyze's own run durable. It is a
/// deliberate change to that command's read-only contract, and the line the
/// contract now draws is between the **subject** and the **observation**:
/// analyze still never mutates an artifact it audits, and `--fix` remains the
/// only path that does. Recording that the audit happened is not mutating the
/// subject — it is the same thing `write-review` does for the other gate, and
/// the reason that gate could be enforced while this one could not.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct WriteAnalysisArgs {
    /// Feature directory whose `spec.md` records the analysis.
    #[arg(long)]
    pub feature: String,
    /// ISO-8601 UTC timestamp recorded as `analyze.last-run`. Host-provided,
    /// as `write-review`'s `reviewed-at` is.
    #[arg(long)]
    pub analyzed_at: String,
    /// HEAD sha the analysis ran against (`analyze.analyzed-against`).
    #[arg(long)]
    pub analyzed_against: String,
    /// Hard-fail findings this run produced.
    #[serde(default)]
    #[arg(long, default_value_t = 0)]
    pub hard_fail: u32,
    /// Blocking-tier findings this run produced.
    #[serde(default)]
    #[arg(long, default_value_t = 0)]
    pub blocking_findings: u32,
    /// Advisory-tier findings this run produced. Recorded, never gated on.
    #[serde(default)]
    #[arg(long, default_value_t = 0)]
    pub advisory: u32,
    /// Targets the run could not examine — the informational `skipped` set.
    /// Required in the record so a clean result cannot be read as a fully
    /// examined one (`QUAL-CLAIM-001`).
    #[serde(default)]
    #[arg(long, default_value_t = 0)]
    pub unexamined: u32,
}

/// Result for `write-analysis`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct WriteAnalysisResult {
    /// Repo-relative path of the spec whose `analyze:` block was written.
    pub spec_path: String,
    /// `true` when `hard-fail` or `blocking-findings` exceeds zero — the
    /// value `check-review-gate` reads.
    pub blocking: bool,
    /// Whether an `analyze:` block already existed and was replaced, as
    /// opposed to being inserted for the first time. Reported so a caller can
    /// tell a re-analysis from a spec leaving the grandfathered population.
    pub replaced: bool,
}

/// Parsed spec frontmatter.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct Frontmatter {
    /// Pipeline status (e.g., "clarified", "planned", "in-progress", "done").
    pub status: String,
    /// Dependency feature names.
    #[serde(default)]
    pub dependencies: Vec<String>,
    /// Topic tags (e.g., `[format, process, pipeline]`).
    #[serde(default)]
    pub tags: Vec<String>,
    /// The upstream spec a branch-scoped spec folds back into, when the
    /// spec declares one (spec 051).
    ///
    /// Declared rather than derived: nothing in the repository can compute
    /// it, and no generator rewrites it. Its presence means the fold has
    /// not happened yet, which is outstanding work — so the pipeline view
    /// reports the spec as pending and the pre-`done` gate blocks on it.
    ///
    /// The named spec routinely does **not** exist in this working tree: a
    /// branch-scoped spec exists because upstream moved, so its target
    /// normally lives on the upstream branch. Absence is the expected
    /// state before a merge, never a defect.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folds_into: Option<String>,
    /// Last-review block, when set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<ReviewBlock>,
    /// Last-analysis block, when set. Absent on a spec that predates the
    /// analyze record, which the drift family grandfathers and Family 37
    /// counts — see [`AnalyzeBlock`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyze: Option<AnalyzeBlock>,
}

/// One parsed body section.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct SpecSection {
    /// Section heading.
    pub heading: String,
    /// Markdown heading level (2 for `##`, etc.).
    pub level: u8,
    /// Section body text (empty unless `include-body` was set).
    pub body: String,
}

/// One acceptance-criterion checkbox.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct AcceptanceCriterion {
    /// Whether the checkbox is checked.
    pub checked: bool,
    /// Criterion text, with any `AC{n}:` label prefix retained — the label
    /// is part of the criterion as authored, and stripping it here would
    /// make the reported text differ from the file.
    pub text: String,
    /// The criterion's stable `AC{n}` label, when it carries one. `None`
    /// for a criterion the labelling pass has not reached (spec 013).
    /// Callers should prefer this over the positional index when holding a
    /// reference across edits.
    pub label: Option<String>,
}

/// One open-question entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct OpenQuestion {
    /// Question text.
    pub text: String,
}

/// One unresolved open question carried by a scenario, tagged with the
/// scenario it came from. Distinct from [`OpenQuestion`] (the spec body's
/// own list) on purpose: spec-level and scenario-level questions are
/// independent concerns for resolution, so the two lists are never merged
/// (spec 046).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ScenarioOpenQuestion {
    /// Slug of the scenario carrying the question (filename without `.md`).
    pub scenario: String,
    /// Question text.
    pub text: String,
}

/// Result for `read-spec`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ReadSpecResult {
    /// Parsed frontmatter.
    pub frontmatter: Frontmatter,
    /// Body sections in document order.
    pub sections: Vec<SpecSection>,
    /// Acceptance-criteria checkboxes.
    pub acceptance_criteria: Vec<AcceptanceCriterion>,
    /// Open Questions list, from the spec body only. Unchanged in meaning
    /// and value by spec 046 — every existing consumer keeps its behavior.
    pub open_questions: Vec<OpenQuestion>,
    /// Unresolved questions carried by this feature's scenarios, in shared
    /// scenario order, each tagged with its source scenario. A sibling
    /// signal to `open-questions`, never merged into it: the two answer
    /// different questions. The body's count is what `clarified` asserts and
    /// what the `draft → clarified` edge turns on; these are remaining work
    /// that gates `done`, so merging them would make a spec's status
    /// contradict its own body (spec 046).
    ///
    /// The earlier rationale here — that merging "would route a feature-level
    /// target to feature-targeted clarify, which does not read scenarios" —
    /// no longer holds: feature-targeted clarify *reports* this field as of
    /// 022's `scenario-open-question-signal`. The conclusion is unchanged;
    /// only the reason is.
    pub scenario_open_questions: Vec<ScenarioOpenQuestion>,
    /// Slugs of scenario files that could not be read while collecting
    /// `scenario-open-questions`. Empty in the ordinary case.
    ///
    /// Present so a caller can tell *"every scenario was examined and none
    /// carries a question"* from *"a scenario could not be examined"* — an
    /// empty `scenario-open-questions` alone means both. An unreadable
    /// scenario is never escalated into a `done`-blocking finding (nothing
    /// can be proven about a file that will not parse), but it is reported
    /// rather than dropped, per `QUAL-CLAIM-001` (spec 046).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scenario_files_unreadable: Vec<String>,
    /// Repo-relative path to the spec file.
    pub path: String,
}

// -- read-tasks --------------------------------------------------------------

/// Args for `read-tasks`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct ReadTasksArgs {
    /// Feature directory name under `specs/`.
    #[arg(long)]
    pub feature: String,
}

/// One sub-item under a top-level task.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct Subtask {
    /// Sub-item text.
    pub text: String,
    /// Whether the checkbox is checked.
    pub checked: bool,
}

/// One top-level task.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct Task {
    /// Top-level task number (e.g., "1", "12").
    pub number: String,
    /// Task heading text.
    pub heading: String,
    /// Subtask list.
    pub subtasks: Vec<Subtask>,
    /// `Done when:` clause, if present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub done_when: Option<String>,
    /// Checked state of the done-when clause when it is authored in
    /// **checkbox form** (`- [ ] Done when: …`), which `/{project}:plan`'s
    /// task breakdown tends to produce. `None` for the canonical bold and
    /// bulletless forms, which carry no checkbox and so can never disagree
    /// with the subtask tally.
    ///
    /// The clause is never an addressable subtask (the read/mark index
    /// contract), so a `Some(false)` here is the one case where every
    /// subtask can be checked while an unchecked box is still visible in
    /// the block — the signal `/{project}:implement` reports rather than
    /// rounding up (scenario unchecked-done-when-clause-tally).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub done_when_checked: Option<bool>,
    /// Phase container heading text, when the task lives under a `## …`
    /// phase (e.g., `Phase A — Refactor`). `None` for flat-structure tasks
    /// declared directly at level 2 (`## N. Title`). Absent from the JSON
    /// output when `None`, so existing consumers that don't know about
    /// phased structure still parse correctly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

/// Result for `read-tasks`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ReadTasksResult {
    /// Tasks in declaration order.
    pub tasks: Vec<Task>,
    /// Repo-relative path to the tasks file.
    pub path: String,
}

// -- mark-task ---------------------------------------------------------------

/// Args for `mark-task`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct MarkTaskArgs {
    /// Feature directory name.
    #[arg(long)]
    pub feature: String,
    /// Top-level task number (e.g., "1").
    #[arg(long)]
    pub task_number: String,
    /// Subtask index within the task (0-based).
    #[arg(long)]
    pub subtask_index: usize,
    /// Desired checkbox state.
    #[arg(long)]
    pub checked: bool,
}

/// Result shape shared by `mark-task` and `mark-criterion`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CheckboxToggleResult {
    /// Previous checkbox state.
    pub previous: bool,
    /// New checkbox state after the write.
    pub current: bool,
    /// Repo-relative path to the file written.
    pub path: String,
}

// -- mark-criterion ----------------------------------------------------------

/// Args for `mark-criterion`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct MarkCriterionArgs {
    /// Feature directory name.
    #[arg(long)]
    pub feature: String,
    /// Acceptance criterion index (0-based, ordered as in the spec).
    /// Mutually exclusive with `label`; supply exactly one.
    #[arg(long)]
    pub criterion_index: Option<usize>,
    /// Acceptance criterion label (e.g. `AC7`), the stable identifier a
    /// criterion carries in its text. Preferred over `criterion-index`:
    /// a label survives criteria being inserted, reordered, or removed,
    /// while an index computed before such an edit silently addresses a
    /// different criterion afterwards (spec 013).
    #[arg(long)]
    pub label: Option<String>,
    /// Desired checkbox state.
    #[arg(long)]
    pub checked: bool,
}

// -- set-status --------------------------------------------------------------

/// Args for `set-status`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct SetStatusArgs {
    /// Feature directory name.
    #[arg(long)]
    pub feature: String,
    /// Expected current status on disk.
    #[arg(long)]
    pub from: String,
    /// Desired status to write.
    #[arg(long)]
    pub to: String,
}

/// Result for `set-status`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct SetStatusResult {
    /// Previous status field value.
    pub previous: String,
    /// New status after the write.
    pub current: String,
    /// Repo-relative path to the spec file.
    pub path: String,
}

// -- derive-dependencies -----------------------------------------------------

/// Args for `derive-dependencies`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct DeriveDependenciesArgs {
    /// Perform the rewrite. **Off by default: the primitive reports without
    /// writing.**
    ///
    /// Read-only is the default because the subprocess interpreter has no
    /// per-step argument binding — a step naming this primitive in a code span
    /// is dispatched with no arguments at all. If writing were the default,
    /// every safety-net step in `/target`, `/clarify`, `/plan`, and
    /// `/implement` would silently rewrite `dependencies:` across the whole
    /// corpus. `run-generator` reaches the same guarantee by hardcoding
    /// `--dry-run` on the scripts it invokes; this is that guarantee, expressed
    /// in the argument.
    #[serde(default)]
    #[arg(long)]
    pub write: bool,
    /// Rewrite only specs staged in the git index for the pending commit,
    /// instead of every tracked spec. The cycle check still spans the full
    /// graph — a staged edge can close a cycle through an unstaged spec. For
    /// pre-commit use, so committing one spec never rewrites another.
    #[serde(default)]
    #[arg(long)]
    pub staged: bool,
}

/// Result for `derive-dependencies`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct DeriveDependenciesResult {
    /// Whether any spec was rewritten (or, under `dry-run`, would be).
    pub drift: bool,
    /// Repo-relative paths of the specs rewritten, sorted.
    pub updated: Vec<String>,
    /// Specs examined and found drifted but deliberately not written —
    /// the `staged` case. Neither "in sync" nor "not examined", so they are
    /// reported separately rather than folded into either.
    pub unwritten: Vec<String>,
    /// Count of tracked specs actually read. Excludes any listed in
    /// `absent`, so it never overstates what the run inspected.
    pub examined: u32,
    /// Untracked specs in the worktree, which are never enumerated or
    /// rewritten (spec 017). Reported so an empty `updated` cannot be read as
    /// "everything is in sync" — these were not examined at all.
    pub untracked_skipped: Vec<String>,
    /// Specs tracked in the git index but absent from the worktree — deleted
    /// without the deletion being staged. Nothing can be derived from them, so
    /// they are named rather than dropped: they are excluded from `examined`,
    /// and reporting neither would let `examined` assert a subject the run
    /// never read (`QUAL-CLAIM-001`, recorded by 022's review on 2026-08-27).
    #[serde(default)]
    pub absent: Vec<String>,
    /// Dependency cycles in the derived graph, each listing its members
    /// sorted, the cycles themselves sorted by least member. A single-member
    /// entry is a self-link. Empty means acyclic. This is a **domain
    /// outcome**: the caller decides whether it blocks.
    pub cycles: Vec<Vec<String>>,
    /// Specs whose frontmatter opens a block it never closes, so nothing could
    /// be derived from them. Reported rather than repaired — `validate-frontmatter`
    /// owns diagnosing a malformed block — and never an error: an unparseable
    /// spec is an unknown, not drift.
    ///
    /// The contract this completes: an empty `updated` means examined-and-clean
    /// only when `unparseable` is also empty (`QUAL-CLAIM-001`), the same
    /// pairing `check-artifacts` has with `skipped`.
    pub unparseable: Vec<String>,
    /// Repo-relative spec-root directory the run enumerated (spec 040), so a
    /// caller can tell a genuinely empty corpus from a misconfigured root.
    pub specs_root: String,
    /// Whether the run actually wrote.
    pub wrote: bool,
}

// -- derive-references -------------------------------------------------------

/// Args for `derive-references`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct DeriveReferencesArgs {
    /// Perform the rewrite. Off by default, for the reason recorded on
    /// [`DeriveDependenciesArgs::write`].
    #[serde(default)]
    #[arg(long)]
    pub write: bool,
    /// Write only specs staged in the git index for the pending commit,
    /// instead of every tracked spec. The enumeration still spans every
    /// tracked spec: a reference derives from the body **and** the
    /// `[services]` registry, so a service rename drifts specs nobody
    /// touched, and an untouched spec is never staged. Those land in
    /// [`DeriveReferencesResult::unwritten`]. For pre-commit use, so
    /// committing one spec never rewrites another.
    #[serde(default)]
    #[arg(long)]
    pub staged: bool,
}

/// Result for `derive-references`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct DeriveReferencesResult {
    /// Whether any spec was rewritten (or, under `dry-run`, would be).
    pub drift: bool,
    /// Repo-relative paths of the specs rewritten, sorted.
    pub updated: Vec<String>,
    /// Specs examined and found drifted but deliberately not written — the
    /// `staged` case. Neither "in sync" nor "not examined", so they are
    /// reported separately rather than folded into either.
    ///
    /// This field carries more weight here than on
    /// [`DeriveDependenciesResult::unwritten`]. A dependency derives from the
    /// spec body alone, so it can only drift when that spec is edited — which
    /// stages it. A reference derives from the body *and* the `[services]`
    /// registry, so renaming a service alias drifts every referencing spec
    /// while leaving all of them untouched, and therefore unstaged. Without
    /// this list the pre-commit hook reports such a tree in sync on every
    /// commit indefinitely.
    pub unwritten: Vec<String>,
    /// Count of tracked specs actually read. Every tracked spec is walked,
    /// `staged` or not — the filter applies to the write, not the walk — but
    /// specs listed in `absent` are excluded, so this never overstates what
    /// the run inspected.
    pub examined: u32,
    /// Untracked specs in the worktree, never enumerated or rewritten
    /// (spec 017). Reported so an empty `updated` cannot be read as
    /// "everything is in sync".
    pub untracked_skipped: Vec<String>,
    /// Number of services resolved from the `[services]` registry. Zero means
    /// every harvested reference records `service: null`, which is a very
    /// different state from "no references found" — an unreadable or absent
    /// config looks identical in `updated` alone.
    pub registered_services: u32,
    /// Specs whose frontmatter opens a block it never closes, so nothing could
    /// be derived from them. Reported rather than repaired — `validate-frontmatter`
    /// owns diagnosing a malformed block — and never an error: an unparseable
    /// spec is an unknown, not drift.
    ///
    /// The contract this completes: an empty `updated` means examined-and-clean
    /// only when `unparseable` is also empty (`QUAL-CLAIM-001`), the same
    /// pairing `check-artifacts` has with `skipped`.
    pub unparseable: Vec<String>,
    /// Specs tracked in the git index but absent from the worktree — deleted
    /// without the deletion being staged. Nothing can be derived from them, so
    /// they are named rather than dropped: they are excluded from `examined`,
    /// and reporting neither would let `examined` assert a subject the run
    /// never read (`QUAL-CLAIM-001`, recorded by 022's review on 2026-08-27).
    #[serde(default)]
    pub absent: Vec<String>,
    /// Spec-root directory name the run enumerated (spec 040).
    pub specs_root: String,
    /// Whether the run actually wrote. A result read out of context cannot
    /// then be mistaken for one that changed the tree.
    pub wrote: bool,
}

// -- derive-boundary ---------------------------------------------------------

/// Args for `derive-boundary`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct DeriveBoundaryArgs {
    /// Feature directory name.
    #[arg(long)]
    pub feature: String,
}

/// Result for `derive-boundary`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct DeriveBoundaryResult {
    /// Boundary entries (glob patterns and concrete paths).
    pub boundary: Vec<String>,
    /// First commit that touched the spec dir. Empty when no commit
    /// touches it yet — the boundary is then unknowable rather than
    /// broken, and `guidance` carries the next step (scenario
    /// derive-boundary-uncommitted-spec-dir).
    pub first_commit: String,
    /// Current `HEAD` sha at derivation time. Empty on an unborn HEAD
    /// (a repo with no commits at all).
    pub current_head: String,
    /// Next-step guidance, present only when the derivation found no
    /// spec history. Its presence is the machine-readable signal that
    /// `/{project}:plan`'s validation gate refuses to advance on and
    /// that `/{project}:implement` surfaces before the walk proceeds
    /// fail-closed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guidance: Option<String>,
}

// -- check-stuck -------------------------------------------------------------

/// Args for `check-stuck`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct CheckStuckArgs {
    /// Feature directory name.
    #[arg(long)]
    pub feature: String,
    /// Commit-count threshold above which the task is considered stuck.
    #[arg(long)]
    pub threshold: u32,
}

/// Result for `check-stuck`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CheckStuckResult {
    /// Number of commits on `tasks.md` since `since-sha`.
    pub commit_count: u32,
    /// Whether `commit-count >= threshold` and the same task is still
    /// incomplete.
    pub stuck: bool,
    /// Sha at which the status entered `in-progress` (origin of the count).
    pub since_sha: String,
    /// Threshold echoed from args.
    pub threshold: u32,
}

// -- validate-frontmatter ---------------------------------------------------

/// One frontmatter validation finding.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct FrontmatterFinding {
    /// Severity tier.
    pub severity: String,
    /// Field path that failed validation (may be empty for cross-field issues).
    pub field: String,
    /// Human-readable description.
    pub message: String,
}

/// Args for `validate-frontmatter`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct ValidateFrontmatterArgs {
    /// Repo-relative path to the spec file.
    #[arg(long)]
    pub path: String,
}

/// Result for `validate-frontmatter`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ValidateFrontmatterResult {
    /// All findings collected (empty when `clean`).
    pub findings: Vec<FrontmatterFinding>,
    /// Whether the frontmatter is clean.
    pub clean: bool,
}

// -- resolve-anchor ----------------------------------------------------------

/// Args for `resolve-anchor`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct ResolveAnchorArgs {
    /// Repo-relative path to the markdown file whose `§<anchor>` references
    /// are scanned.
    #[arg(long)]
    pub path: String,
    /// Optional repo-relative path to the file supplying the
    /// `<!-- §anchor -->` markers to resolve against. Omit to resolve
    /// against markers in `path` itself (the same-file self-consistency
    /// check). Supply a different file — e.g. the constitution — to verify
    /// that a spec's cross-file `§` references still name real sections.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub markers_path: Option<String>,
}

/// One anchor reference.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct AnchorReference {
    /// Anchor name (without `§` prefix).
    pub anchor: String,
    /// 1-based line of the reference.
    pub line: u32,
    /// Whether the anchor resolves to a marker.
    pub resolved: bool,
}

/// Result for `resolve-anchor`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ResolveAnchorResult {
    /// All anchor references found in the file.
    pub references: Vec<AnchorReference>,
    /// Anchor names with no matching marker.
    pub unresolved: Vec<String>,
}

// -- traverse-deps -----------------------------------------------------------

/// Args for `traverse-deps`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct TraverseDepsArgs {
    /// Feature directory name.
    #[arg(long)]
    pub feature: String,
}

/// One dependency edge.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct DependencyEdge {
    /// Dependency feature name.
    pub feature: String,
    /// Whether the dependency directory exists.
    pub exists: bool,
    /// Status of the dependency (empty when `exists` is false).
    #[serde(default)]
    pub status: String,
    /// Whether the dependency status is compatible with this feature.
    pub compatible: bool,
}

/// Result for `traverse-deps`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct TraverseDepsResult {
    /// All dependency edges.
    pub dependencies: Vec<DependencyEdge>,
    /// Overall compatibility (logical AND across direct edges, plus
    /// `cycles.is_empty()`).
    pub compatible: bool,
    /// Strongly-connected components forming cycles in the reachable dep
    /// subgraph rooted at the targeted feature. Each entry is one SCC as
    /// a list of slugs in traversal order — multi-node cycles (size ≥ 2)
    /// and self-cycles (size 1 with a self-edge) both surface here.
    /// Empty when the walked subgraph is acyclic.
    #[serde(default)]
    pub cycles: Vec<Vec<String>>,
}

// -- dashboard ---------------------------------------------------------------

/// Args for `dashboard`. The primitive takes no caller-supplied inputs —
/// the repo root, the project config (`.ductus/config.toml`, committed),
/// and the session file (`.ductus/session.toml`, gitignored per-user
/// session state), each resolved with the pre-042 legacy root fallback,
/// are the only state it reads. The empty args struct preserves
/// clap-derive consistency with every other primitive.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct DashboardArgs {}

/// Per-spec entry in the dashboard payload. The fields mirror the dashboard
/// table 1:1 — `slug` / `status` / `dependencies` / `tags` / `open-question-count`
/// drive the row's identity and labels; `has-plan` / `has-tasks` /
/// `has-data-model` / `scenarios-count` populate the artifact-existence
/// columns; `blocked-by` carries the deterministically-computed list of
/// dependency slugs whose own `status` is below `clarified` (empty when
/// unblocked).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct DashboardSpec {
    /// Directory basename (e.g., "022-deterministic-runtime").
    pub slug: String,
    /// Frontmatter status (one of `draft`, `clarified`, `planned`,
    /// `in-progress`, `done`).
    pub status: String,
    /// Frontmatter `dependencies` array (empty when absent).
    pub dependencies: Vec<String>,
    /// Frontmatter `tags` array (empty when absent).
    pub tags: Vec<String>,
    /// Count of unresolved questions in the spec body's `## Open Questions`
    /// section, matching `read-spec`'s open-question semantics.
    pub open_question_count: u32,
    /// `true` when `specs/{slug}/plan.md` exists on disk.
    pub has_plan: bool,
    /// `true` when `specs/{slug}/tasks.md` exists on disk.
    pub has_tasks: bool,
    /// `true` when `specs/{slug}/data-model.md` exists on disk.
    pub has_data_model: bool,
    /// Count of `*.md` files under `specs/{slug}/scenarios/` (0 when the
    /// directory is absent).
    pub scenarios_count: u32,
    /// Total unresolved questions across this spec's scenarios. Distinct
    /// from `open-question-count`, which stays spec-body-only: the two are
    /// separate signals and are never summed (spec 046).
    pub scenario_open_question_count: u32,
    /// Slugs of the scenarios carrying those questions, in shared scenario
    /// order. Empty when the count is zero. The caller renders the callout
    /// straight from this array — a table cell cannot hold scenario names.
    pub scenarios_with_questions: Vec<String>,
    /// Dependency slugs whose own `status` is below `clarified`; empty when
    /// every dependency is at `clarified` or later. The caller renders the
    /// "blocked specs" callout straight from a non-empty array.
    pub blocked_by: Vec<String>,
    /// The upstream spec this one folds back into, when it declares one
    /// (spec 051). A declared fold is work that has not happened, so the
    /// view reports the spec as carrying it rather than as complete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folds_into: Option<String>,
    /// `true` when [`Self::folds_into`] names a feature absent from this
    /// spec corpus; `false` when it resolves, and `false` whenever no fold
    /// is declared.
    ///
    /// A **report**, never a verdict. A branch-scoped spec exists because
    /// upstream moved, so before the merge its target normally lives on the
    /// upstream branch — absent here is the ordinary case, and this view
    /// cannot tell which tree it is looking at. Surfacing it is what lets an
    /// operator recognize a typo they would otherwise meet as a fold-back
    /// refusal weeks later; treating it as an error would fire on the
    /// feature's normal case.
    #[serde(default)]
    pub fold_target_missing: bool,
}

/// Config review-state summary returned alongside the per-spec
/// inventory, read from the resolved config file (`.ductus/config.toml`;
/// legacy root `.govern.toml` pre-migration). The `present` flag
/// distinguishes "config absent" from "config present but section absent
/// / empty" so callers can drive the callout-suppression rule correctly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct DashboardConfig {
    /// `true` when the resolved config file exists.
    pub present: bool,
    /// Basenames from `[[review.disabled-rule-files]]`. Empty when the
    /// section is absent or its array is empty.
    pub disabled_rule_files: Vec<String>,
}

/// Scenario-level detail returned when the session target names a scenario.
/// Populated so callers render the scenario header line without a separate
/// file read.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct DashboardScenarioDetail {
    /// Scenario frontmatter `section` field (or the legacy `spec-ref` field
    /// for pre-017 scenarios). Empty when neither is present.
    pub section: String,
    /// One-line summary of the scenario's `## Context` section (first
    /// non-blank line, trimmed). Empty when the section is absent.
    pub context_summary: String,
    /// Count of unresolved questions in the scenario body's
    /// `## Open Questions` section.
    pub open_question_count: u32,
}

/// Session-target summary returned when the session file
/// (`.ductus/session.toml`; legacy root `.govern.session.toml`
/// pre-migration) exists and names a target. The `feature` field always names the targeted
/// feature; `scenario` is populated when a scenario is targeted;
/// `scenario-detail` is populated alongside `scenario` to spare callers an
/// extra read.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct DashboardSessionTarget {
    /// Targeted feature slug as recorded in the session file.
    pub feature: String,
    /// Targeted scenario slug, when one is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
    /// Scenario header detail; present when `scenario` is `Some` and the
    /// scenario file is readable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scenario_detail: Option<DashboardScenarioDetail>,
}

/// Result for `dashboard`. One call returns everything `/ductus:status` needs
/// to render the full pipeline view: the per-spec inventory, the
/// repo-wide `tags-union`, the config review-state summary, and
/// the optional session target.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct DashboardResult {
    /// Session target when the session file (`.ductus/session.toml`;
    /// legacy root fallback) exists and names a target; `None` otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_target: Option<DashboardSessionTarget>,
    /// Per-spec entries in directory-name order.
    pub specs: Vec<DashboardSpec>,
    /// Sorted, deduplicated union of every spec's `tags` array. Empty when
    /// no spec has tags.
    pub tags_union: Vec<String>,
    /// Config review-state summary.
    pub config: DashboardConfig,
    /// The full pipeline view pre-rendered as one markdown fragment —
    /// preamble, dashboard table, counts and callouts, and the
    /// cross-service references readout (the runtime resolves each spec's
    /// `references:` index internally for the readout). Returned data the
    /// host may restyle, never printed by the runtime; the structured
    /// fields above stay authoritative for hosts that render their own
    /// view.
    pub rendered_markdown: String,
}

// -- check-rule-ids ----------------------------------------------------------

/// Args for `check-rule-ids`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct CheckRuleIdsArgs {
    /// Repo-relative path to the file scanned for citations.
    #[arg(long)]
    pub path: String,
    /// Repo-relative paths to rule files defining the known rule IDs.
    #[arg(long = "rule-file")]
    pub rule_files: Vec<String>,
}

/// One rule-ID citation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct RuleCitation {
    /// Rule ID as cited (e.g., "SEC-AUTH-001").
    pub rule_id: String,
    /// Whether the ID exists in any rule file.
    pub found: bool,
    /// Whether the ID is deprecated.
    pub deprecated: bool,
}

/// Result for `check-rule-ids`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CheckRuleIdsResult {
    /// All citations parsed from the file.
    pub citations: Vec<RuleCitation>,
    /// Cited rule IDs that don't exist.
    pub missing: Vec<String>,
    /// Cited rule IDs that exist but are deprecated.
    pub deprecated: Vec<String>,
}

// -- run-generator -----------------------------------------------------------

/// Args for `run-generator`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct RunGeneratorArgs {
    /// Repo-relative path to the bash script.
    #[arg(long)]
    pub script: String,
}

/// Result for `run-generator`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct RunGeneratorResult {
    /// Whether the script reported drift (non-zero exit treated as drift).
    pub drift: bool,
    /// Captured stdout.
    pub stdout: String,
    /// Captured stderr.
    pub stderr: String,
    /// Script's exit code.
    pub exit_code: i32,
}

// -- lint-markdown -----------------------------------------------------------

/// Args for `lint-markdown`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct LintMarkdownArgs {
    /// Paths or globs to lint.
    #[arg(long = "path")]
    pub paths: Vec<String>,
    /// Whether to invoke `markdownlint-cli2` in fix mode.
    #[serde(default)]
    #[arg(long)]
    pub fix: bool,
}

/// One markdown-lint violation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct MarkdownViolation {
    /// Repo-relative file path.
    pub path: String,
    /// 1-based line.
    pub line: u32,
    /// `markdownlint` rule name (e.g., "MD013").
    pub rule: String,
    /// Description of the violation.
    pub message: String,
}

/// Result for `lint-markdown`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct LintMarkdownResult {
    /// All violations.
    pub violations: Vec<MarkdownViolation>,
    /// Whether the lint produced no violations and exited zero.
    pub clean: bool,
    /// `markdownlint-cli2` exit code.
    pub exit_code: i32,
}

// -- apply-manifest ----------------------------------------------------------

/// One entry in an `apply-manifest` request.
///
/// `source` is a path relative to the args' `source-root`; `dest` is a
/// path relative to the args' `target-root`. Both use forward slashes;
/// the primitive normalizes to the host OS when joining.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ManifestEntry {
    /// Path under `source-root` to read.
    pub source: String,
    /// Path under `target-root` to write.
    pub dest: String,
    /// Per-entry strategy: `update` / `create` / `skip-if-conflict`.
    pub strategy: String,
    /// Substitution keys (without braces) to exclude for this entry only.
    /// Unlisted keys are substituted normally; unknown keys are no-ops.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_literals: Option<Vec<String>>,
}

/// Args for `apply-manifest`.
///
/// `entries`, `pinned`, and `substitutions` are arrays and maps of objects,
/// which clap cannot express as flags — they arrive through the JSON context on
/// the MCP and interpreter paths. The CLI reaches them through the sibling
/// `--*-json` flags below, each a path to a file holding that field's value.
/// That surface exists because a State-B `/{project}` run drives the whole
/// bootstrap through the CLI (spec 048 scenario `state-b-continues-in-session`);
/// without it, `apply-manifest` would receive an empty manifest and copy
/// nothing — silently, since an empty manifest is a legal one.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct ApplyManifestArgs {
    /// Local path to the source tree (typically a prior `extract-archive`
    /// staging directory).
    #[arg(long)]
    pub source_root: String,
    /// Local path to the destination tree; created on demand for each entry.
    #[arg(long)]
    pub target_root: String,
    /// Per-entry manifest. Set via JSON context — not exposed as CLI flags.
    #[serde(default)]
    #[arg(skip)]
    pub entries: Vec<ManifestEntry>,
    /// Destination paths the primitive must never touch, regardless of strategy.
    /// Forward-slash form, relative to `target-root`. Set via JSON context.
    #[serde(default)]
    #[arg(skip)]
    pub pinned: Vec<String>,
    /// Substitution map applied to text files. **Keys are bare** — `project`,
    /// not `{project}`: the primitive wraps each key in braces itself to build
    /// the placeholder it searches for, so a key that already carries them
    /// yields `{{project}}` and matches nothing. A placeholder-shaped or empty
    /// key is rejected before any file is touched. Per-entry `keep-literals`
    /// masks specific keys for individual entries, and is keyed the same way.
    /// Set via JSON context.
    #[serde(default)]
    #[arg(skip)]
    pub substitutions: std::collections::BTreeMap<String, String>,
    /// CLI-only: path to a JSON file holding the `entries` array.
    /// `#[serde(skip)]` keeps it out of the MCP schema, so the tool contract
    /// is unchanged and an MCP caller passing it is still rejected as unknown.
    #[serde(skip)]
    #[arg(long, value_name = "PATH")]
    pub entries_json: Option<String>,
    /// CLI-only: path to a JSON file holding the `pinned` array.
    #[serde(skip)]
    #[arg(long, value_name = "PATH")]
    pub pinned_json: Option<String>,
    /// CLI-only: path to a JSON file holding the `substitutions` map.
    #[serde(skip)]
    #[arg(long, value_name = "PATH")]
    pub substitutions_json: Option<String>,
}

/// One per-entry outcome from `apply-manifest`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ManifestEntryResult {
    /// Echo of the entry's `source` field.
    pub source: String,
    /// Echo of the entry's `dest` field.
    pub dest: String,
    /// One of `created` / `updated` / `unchanged` / `skipped-exists` /
    /// `skipped-pinned` / `source-missing`.
    pub action: String,
    /// Placeholder replacements applied to this entry, or `null` when
    /// substitution never ran for it.
    ///
    /// The distinction is the point, and it is `QUAL-CLAIM-001`: `0` means the
    /// file was read, decoded as UTF-8, and matched no placeholder — which is
    /// correct for a file that carries none and is the signature of a
    /// malformed substitution map for a file that does. `null` means the
    /// question was never asked, because the entry was pinned, skipped, its
    /// source was missing, it used `skip-if-conflict` (which never
    /// substitutes), or its bytes are not UTF-8. Collapsing the two into a
    /// bare `0` would let "examined and found nothing" and "never examined"
    /// arrive as the same value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub substitutions_applied: Option<u32>,
}

/// Result for `apply-manifest`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ApplyManifestResult {
    /// Per-entry outcomes in declaration order.
    pub entries: Vec<ManifestEntryResult>,
    /// Count of `created` actions across all entries.
    pub created: u32,
    /// Count of `updated` actions across all entries.
    pub updated: u32,
    /// Count of `unchanged` actions across all entries.
    pub unchanged: u32,
    /// Count of `skipped-exists` actions across all entries.
    pub skipped_exists: u32,
    /// Count of `skipped-pinned` actions across all entries.
    pub skipped_pinned: u32,
    /// Count of `source-missing` actions across all entries.
    pub source_missing: u32,
    /// Total placeholder replacements applied across every entry that ran
    /// substitution.
    ///
    /// Surfaced because the walk was previously silent about it: the count
    /// was computed and discarded, so a caller who passed a malformed
    /// substitution map got `{"created":1,"updated":25,"unchanged":11}` — a
    /// result indistinguishable from a correct run — while every placeholder
    /// in every copied file survived literally. A malformed *key* is now
    /// rejected outright, but a map that is merely incomplete (a key the
    /// caller forgot) still cannot be, so the number a caller can sanity-check
    /// against its own expectation belongs in the result.
    pub substitutions_applied: u32,
    /// How many entries actually ran substitution — the denominator without
    /// which `substitutions-applied` cannot be read.
    ///
    /// Zero replacements across twenty entries that were substituted is a
    /// defect; zero across zero entries that were is a manifest of pinned and
    /// skipped files behaving correctly. One number cannot say which.
    pub entries_substituted: u32,
}

// -- enforce-manifest --------------------------------------------------------

/// Args for `enforce-manifest`.
///
/// Walks `directory`, removes files matching `glob-include` that are not
/// in `expected` and not in `pinned`, and returns the per-file outcome.
/// The primitive does not create `directory` when missing — that's
/// `apply-manifest`'s job.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct EnforceManifestArgs {
    /// Local path to the directory to enforce.
    #[arg(long)]
    pub directory: String,
    /// Files relative to `directory` that must remain (basenames for
    /// top-level, slash-delimited relative paths for recursive). Set via
    /// JSON context.
    #[serde(default)]
    #[arg(skip)]
    pub expected: Vec<String>,
    /// Files relative to `directory` that must remain regardless of
    /// `expected`. Reported under `pinned-kept` so callers can surface
    /// the count in completion messages. Set via JSON context.
    #[serde(default)]
    #[arg(skip)]
    pub pinned: Vec<String>,
    /// CLI-only: path to a JSON file holding the `expected` array. See
    /// [`ApplyManifestArgs::entries_json`] for why this surface exists.
    #[serde(skip)]
    #[arg(long, value_name = "PATH")]
    pub expected_json: Option<String>,
    /// CLI-only: path to a JSON file holding the `pinned` array.
    #[serde(skip)]
    #[arg(long, value_name = "PATH")]
    pub pinned_json: Option<String>,
    /// When `true`, walk subdirectories recursively. Default `false` —
    /// the bootstrap's slash-command cleanup is top-level only.
    #[serde(default)]
    #[arg(long)]
    pub recursive: bool,
    /// Glob applied to each file's basename. Default `*.md`. Files whose
    /// basename does not match the glob are left untouched (not even
    /// considered for removal).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub glob_include: Option<String>,
}

/// Result for `enforce-manifest`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct EnforceManifestResult {
    /// Forward-slash relative paths of files removed during the walk.
    pub removed: Vec<String>,
    /// Forward-slash relative paths of files kept because they were in
    /// `expected`.
    pub kept: Vec<String>,
    /// Forward-slash relative paths of files kept because they were in
    /// `pinned`.
    pub pinned_kept: Vec<String>,
}

// -- merge-managed-block -----------------------------------------------------

/// Args for `merge-managed-block`.
///
/// Generalization of [`MergeClaudeMdArgs`] that handles configurable
/// marker shapes. `marker-style: "html-comment"` (default) reproduces
/// `merge-claude-md`'s exact behavior; `marker-style: "line-prefix"`
/// uses a single `# {marker}` preamble line followed by the block,
/// terminated by a blank line or EOF — matching `.gitignore` and
/// `.gitattributes` conventions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct MergeManagedBlockArgs {
    /// Local path to the file to merge into (relative paths resolve
    /// against the runtime's `repo`).
    #[arg(long)]
    pub path: String,
    /// Markdown / plain-text block the framework wants to install.
    /// Trailing whitespace is normalized to a single newline before
    /// write.
    #[arg(long)]
    pub block: String,
    /// Marker name used to delimit the framework-managed region.
    /// Defaults to `ductus-managed`. Multiple frameworks can coexist in
    /// the same file by using different marker names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub marker: Option<String>,
    /// One of `html-comment` (default) or `line-prefix`. The former
    /// uses `<!-- BEGIN/END {marker} -->` pairs; the latter uses a
    /// single `# {marker}` preamble line followed by the block,
    /// terminated by a blank line or EOF.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub marker_style: Option<String>,
}

/// Result for `merge-managed-block`. Extends the retired
/// `merge-claude-md` shim's result shape (`path` / `action` / `marker`)
/// with two `line-prefix`-only fields for the cross-boundary dedup pass
/// (`dedup-removed` count, `dedup-removed-lines` listing). Both fields
/// are absent for `html-comment` invocations.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct MergeManagedBlockResult {
    /// Repo-relative or absolute path of the merged file.
    pub path: String,
    /// One of `created`, `inserted`, `updated`, `unchanged`.
    pub action: String,
    /// Marker name actually applied (echoes the arg's value or the default).
    pub marker: String,
    /// Marker style actually applied (echoes the arg's value or the default).
    pub marker_style: String,
    /// Count of adopter-area lines removed by the cross-boundary dedup
    /// pass. `Some(n)` only on `line-prefix` invocations; `None` for
    /// `html-comment` callsites (the dedup contract is line-list-shaped
    /// and doesn't apply to prose blocks).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dedup_removed: Option<u32>,
    /// Verbatim content of the adopter-area lines removed by the
    /// cross-boundary dedup pass, in source order. `Some(vec)` only on
    /// `line-prefix` invocations; `None` for `html-comment` callsites.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dedup_removed_lines: Option<Vec<String>>,
}

// -- merge-permissions -------------------------------------------------------

/// Args for `merge-permissions` — idempotently merge a canonical
/// permission allow/deny set into a JSON file, removing exact-match
/// duplicates from each array. The primitive is the deterministic surface
/// `/configure` calls; see spec 022's `framework-list-dedup` scenario for
/// the contract.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct MergePermissionsArgs {
    /// Repo-relative path of the JSON file to merge into (e.g.,
    /// `.claude/settings.local.json` on Claude Code,
    /// `.augment/settings.json` on Auggie). Host-supplied from the
    /// bootstrap-substituted `{cli-config-dir}/settings.local.json`
    /// template — no default, so a missing path fails loudly instead of
    /// silently writing to a Claude-shaped location on a non-Claude host.
    #[arg(long)]
    pub path: String,
    /// Canonical entries to ensure under `permissions.allow`.
    #[serde(default)]
    #[arg(long, value_delimiter = ',')]
    pub allow: Vec<String>,
    /// Canonical entries to ensure under `permissions.deny`.
    #[serde(default)]
    #[arg(long, value_delimiter = ',')]
    pub deny: Vec<String>,
    /// Formerly-canonical entries to remove from `permissions.allow`.
    ///
    /// Retirement is **allow-side only, by construction**. An over-broad
    /// entry in a deny set refuses more rather than approving more, so the
    /// same shape that is a hole in an allow entry is a stronger guard
    /// there; a primitive that could sweep both arrays would let a caller
    /// "finish the job" by narrowing the deny set into holes. Re-granting
    /// something a deny entry refuses is a deliberate operation with its
    /// own design, not a side effect of retiring a grant.
    ///
    /// Entries are removed by exact string match, so a pattern an adopter
    /// authored themselves is never touched — only what the framework
    /// itself once shipped and has since retired.
    ///
    /// An entry appearing in both `allow` and `revoke` is rejected
    /// ([`crate::primitives::PrimitiveError::ConflictingRevoke`]) rather
    /// than resolved by pass order: the canonical-presence pass would
    /// re-add whatever the revoke pass removed, so the merge would never
    /// reach a fixed point and `unchanged` would never be emitted.
    #[serde(default)]
    #[arg(long, value_delimiter = ',')]
    pub revoke: Vec<String>,
}

/// Result for `merge-permissions`. Reports the action taken plus
/// per-array counts of entries added (canonical members that were
/// not present) vs. duplicates removed (exact-match entries that
/// were redundant).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct MergePermissionsResult {
    /// Repo-relative or absolute path of the merged file.
    pub path: String,
    /// One of `created`, `updated`, `unchanged`.
    pub action: String,
    /// Count of canonical `allow` entries appended (not already present).
    pub allow_added: u32,
    /// Count of duplicate `allow` entries removed.
    pub allow_deduped: u32,
    /// Count of retired `allow` entries removed — every copy of a
    /// `revoke` member that was present. A doubled retired entry counts
    /// twice here and not at all under `allow-deduped`: the retirement
    /// is why both copies went, so attributing one of them to dedup
    /// would credit that pass with work it did not do.
    pub allow_revoked: u32,
    /// Count of canonical `deny` entries appended (not already present).
    pub deny_added: u32,
    /// Count of duplicate `deny` entries removed.
    pub deny_deduped: u32,
}

// -- extract-archive ---------------------------------------------------------

/// Args for `extract-archive`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct ExtractArchiveArgs {
    /// Local path to the archive (`.tar.gz`, `.tgz`, `.zip`).
    #[arg(long)]
    pub archive: String,
    /// Destination directory; created if missing.
    #[arg(long)]
    pub dest: String,
    /// Explicit format override (`tar-gz` / `zip`). Auto-detected from the
    /// archive's extension when absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub format: Option<String>,
}

/// Result for `extract-archive`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ExtractArchiveResult {
    /// Repo-relative or absolute path of the destination directory.
    pub dest: String,
    /// Repo-relative paths of every regular file extracted, in archive order.
    pub files: Vec<String>,
    /// Count of regular files extracted (directories are not counted).
    pub count: u32,
    /// Detected or override format echoed back (`tar-gz` or `zip`).
    pub format: String,
}

// -- fetch-archive -----------------------------------------------------------

/// Args for `fetch-archive`.
///
/// The local destination uses the `archive` field name (not `dest`) so
/// it shares a context key with [`ExtractArchiveArgs::archive`] when both
/// primitives appear in the same procedure walk — fetch writes the
/// downloaded archive to that path; extract then reads it from the same
/// path without the host having to thread two keys.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct FetchArchiveArgs {
    /// URL of the archive (`.tar.gz`, `.zip`, etc.).
    #[arg(long)]
    pub url: String,
    /// URL of the sha256 sidecar file (matching the `shasum -a 256` format —
    /// one or more lines of `<hex>  <filename>`). **Optional**: when
    /// absent the primitive downloads without verifying but still
    /// returns the computed sha256 in the result, so callers can verify
    /// out-of-band against a known-good digest if desired.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub sha256_url: Option<String>,
    /// Local path where the downloaded archive is written. Used as the
    /// `archive` input by a subsequent `extract-archive` step in the
    /// bootstrap procedure.
    #[arg(long)]
    pub archive: String,
}

/// Result for `fetch-archive`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct FetchArchiveResult {
    /// Repo-relative or absolute path where the archive was written.
    pub path: String,
    /// Lowercase hex sha256 of the downloaded archive. When the args
    /// included `sha256_url`, this value also matched the sidecar's
    /// digest (verification succeeded). When the sidecar URL was
    /// absent, this is the computed digest only — the host can
    /// compare it against a known-good value out-of-band.
    pub sha256: String,
    /// Whether the sha256 was verified against a sidecar URL provided
    /// in the args. `false` when no sidecar URL was supplied.
    pub verified: bool,
    /// Size of the downloaded archive in bytes.
    pub bytes: u64,
}

// -- gate-confirm ------------------------------------------------------------

/// Args for `gate-confirm`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct GateConfirmArgs {
    /// Named gate (e.g., "plan-finalize-status").
    #[arg(long)]
    pub gate: String,
    /// Prompt shown to the user.
    #[arg(long)]
    pub prompt: String,
}

/// Result for `gate-confirm`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct GateConfirmResult {
    /// Whether the user confirmed.
    pub confirmed: bool,
}

// -- create-scenario ---------------------------------------------------------

/// Args for `create-scenario`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct CreateScenarioArgs {
    /// Repo-relative feature directory (e.g., `specs/042-foo`).
    #[arg(long)]
    pub feature_path: String,
    /// Scenario slug (no extension; the filename becomes `{slug}.md`).
    #[arg(long)]
    pub slug: String,
    /// Parent-spec section name written into the scenario frontmatter.
    #[arg(long)]
    pub section: String,
    /// Assembled scenario body — the `## Context` … `## Edge Cases` markdown
    /// the LLM authored, crossing the runtime boundary as one payload (the
    /// content-ingestion convention). The primitive frames it with the
    /// `section:` frontmatter, the H1-from-slug, and the auto-appended
    /// Open / Resolved Questions scaffolding.
    #[arg(long)]
    pub body: String,
}

/// Result for `create-scenario`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CreateScenarioResult {
    /// Repo-relative path of the newly-created scenario file.
    pub created: String,
}

// -- label-criteria ----------------------------------------------------------

/// Args for `label-criteria`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct LabelCriteriaArgs {
    /// Feature directory name under the configured spec root.
    #[arg(long)]
    pub feature: String,
}

/// One `AC{n}` label the pass assigned.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct LabelAssignment {
    /// The assigned label (e.g., `AC7`).
    pub label: String,
    /// 0-based index of the criterion in body order — the same index
    /// `read-spec` lists it at and `mark-criterion` addresses it by.
    pub criterion_index: usize,
}

/// Result for `label-criteria`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct LabelCriteriaResult {
    /// Labels assigned by this run, in body order. Empty when every
    /// criterion was already labelled.
    pub assigned: Vec<LabelAssignment>,
    /// The counter written to frontmatter — the label the next criterion
    /// will receive. Always greater than every label present in the body.
    pub next_criterion: u32,
    /// Repo-relative path of the spec examined.
    pub path: String,
    /// Whether the file was written. `false` distinguishes "already fully
    /// labelled" from "labelled just now", so a caller can report the
    /// no-op rather than an assignment it did not make.
    pub changed: bool,
}

// -- append-task -------------------------------------------------------------

/// Args for `append-task`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct AppendTaskArgs {
    /// Repo-relative feature directory (e.g., `specs/042-foo`).
    #[arg(long)]
    pub feature_path: String,
    /// Task title (the text after the `## N. ` heading prefix).
    #[arg(long)]
    pub title: String,
    /// Body content for the task's `Done when:` clause.
    #[arg(long)]
    pub done_when: String,
    /// Optional checkbox sub-items to render inside the task block. Pass the
    /// item *content* only — the primitive renders the `- [ ] ` marker, and a
    /// caller-supplied leading marker (`- `, `- [ ] `, `- [x] `) is stripped
    /// so it cannot double. When omitted, the primitive emits a single default
    /// `- [ ] Implement the behavior described in scenarios/{slug}.md`
    /// line using the explicit `slug` argument below.
    #[arg(long)]
    pub body: Option<Vec<String>>,
    /// Scenario slug rendered as the `scenarios/{slug}.md` pointer line.
    /// Required when `body` is omitted (there would otherwise be no body at
    /// all). When `body` **is** supplied the pointer is still rendered, above
    /// the caller's items — a supplied slug is never silently discarded.
    ///
    /// It used to be "ignored when `body` is supplied", which quietly broke
    /// the task-references-its-scenario promise `/ductus:groom` and
    /// `/ductus:amend` both make; a caller who wanted a bare custom body says
    /// so by omitting `slug`. Pairs with the slug previously passed to
    /// `create-scenario` when both primitives are invoked together by the
    /// scenario branch of `/ductus:amend`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub slug: Option<String>,
    /// Heading of an existing `## …` phase container under which the new
    /// task should be appended (e.g., `Phase B — Implementation`). Only
    /// consulted when the target `tasks.md` is phased — i.e., contains
    /// at least one `### N.` heading. In a flat file the argument is
    /// ignored and the task is appended at file bottom as `## N. …`.
    ///
    /// When phased and `parent-heading` is omitted, the primitive
    /// creates a default follow-on phase using the auto-computed letter:
    /// `## Phase {next-letter} — Follow-on scenarios`, where
    /// `{next-letter}` is the next alphabetical letter after existing
    /// `Phase X` labels (defaulting to `A` when none are present).
    ///
    /// When phased and the supplied heading does not match any existing
    /// phase, the primitive refuses with
    /// `PrimitiveError::ParentHeadingNotFound` rather than silently
    /// creating a new phase or appending at file bottom.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub parent_heading: Option<String>,
}

/// Result for `append-task`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct AppendTaskResult {
    /// Number assigned to the newly-appended task (`max(existing) + 1`).
    pub task_number: u32,
    /// Repo-relative path of the tasks file written.
    pub path: String,
    /// Whether `tasks.md` was created by this invocation. `false` when an
    /// existing file was extended.
    pub created: bool,
    /// Whether a task block was actually written. `false` is the dedup
    /// domain outcome: a `slug` was supplied and an existing task already
    /// points at `scenarios/{slug}.md`, so `task_number` names that task
    /// rather than a new one and `tasks.md` is unchanged.
    ///
    /// Reported rather than folded into `created` because the two answer
    /// different questions — `created` is about the *file*, this is about
    /// the *task* — and a caller re-running an interrupted fold needs to
    /// distinguish "recorded now" from "already recorded" (spec 051, AC29).
    pub appended: bool,
}

// -- prune-tasks -------------------------------------------------------------

/// Args for `prune-tasks`. Reduces the target feature's `tasks.md` by
/// dropping spent (fully-checked) task sections, or resetting the file to
/// its template initial state. See
/// `specs/041-task-pruning/data-model.md`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct PruneTasksArgs {
    /// Feature directory name under `specs/`.
    #[arg(long)]
    pub feature: String,
    /// Full reset to the template's initial state, rather than the default
    /// keep-pending prune.
    #[serde(default)]
    #[arg(long)]
    pub reset: bool,
    /// Override the `--reset` status gate on a non-`done` spec.
    #[serde(default)]
    #[arg(long)]
    pub force: bool,
    /// Write the reduced file. When false (the default) the primitive is a
    /// pure preview: it computes and classifies but does not write, and the
    /// file body never leaves the runtime.
    #[serde(default)]
    #[arg(long)]
    pub apply: bool,
}

/// Which reduction the primitive performed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum PruneMode {
    /// Drop spent sections; keep every pending section.
    KeepPending,
    /// Reset to the template's initial state.
    Reset,
}

/// Completion state of a task section.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Classification {
    /// >= 1 checkbox and every one is checked. Removable.
    Spent,
    /// >= 1 checkbox and at least one is unchecked. Preserved.
    Pending,
    /// Zero checkboxes. Preserved; never classified spent.
    NoCheckbox,
}

/// Outcome of the `--reset` status gate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum PruneGate {
    /// Keep-pending mode — the gate does not apply.
    NotApplicable,
    /// Reset is permitted (status is `done`, or `force` was supplied).
    Allowed,
    /// Reset refused: status is not `done` and `force` was absent.
    BlockedNeedsForce,
}

/// What prune did with a task section.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum PruneAction {
    /// The section was dropped from the output.
    Removed,
    /// The section was kept verbatim.
    Kept,
}

/// Line and byte size of a `tasks.md`, before or after pruning.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct SizeSummary {
    /// Line count.
    pub lines: usize,
    /// Byte count.
    pub bytes: usize,
}

/// One compact per-section record. Carries the section's identity,
/// classification, and checkbox counts — never its body.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct PruneSection {
    /// Task number (e.g., "1", "12").
    pub number: String,
    /// Task heading text.
    pub heading: String,
    /// Containing phase heading, when phased. Absent for flat structure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// Completion classification.
    pub classification: Classification,
    /// Task-list checkboxes in the section.
    pub checkbox_total: u32,
    /// Of which are checked.
    pub checkbox_checked: u32,
    /// What prune did with the section.
    pub action: PruneAction,
}

/// Result for `prune-tasks`. A compact summary; the file body is never
/// included — the token-reduction contract that motivates the primitive
/// performing its own write.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct PruneTasksResult {
    /// The reduction performed.
    pub mode: PruneMode,
    /// Whether a write happened. `false` on preview, on `nothing-to-prune`,
    /// and on a blocked reset.
    pub applied: bool,
    /// `--reset` status-gate outcome.
    pub gate: PruneGate,
    /// Spec status, read only when `reset` is set (otherwise `null`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Output equals input — nothing spent to prune (keep-pending), or
    /// already template-state (reset).
    pub nothing_to_prune: bool,
    /// Task sections removed.
    pub removed_count: u32,
    /// Task sections kept.
    pub kept_count: u32,
    /// Size before pruning.
    pub size_before: SizeSummary,
    /// Size the tasks file would have after pruning (reported even on a
    /// preview, which computes but does not write it); equal to
    /// `size_before` only on a no-op (nothing to prune).
    pub size_after: SizeSummary,
    /// Per-section classification records.
    pub sections: Vec<PruneSection>,
    /// Repo-relative path to the tasks file.
    pub path: String,
}

// -- migrate-session-file ----------------------------------------------------

/// Args for `migrate-session-file`. Translates a pre-0.10.0 legacy
/// session JSON at `legacy-path` into the consolidated
/// `<repo>/.ductus/session.toml` and deletes the legacy file. The
/// destination is hardcoded (it's the runtime's `SESSION_FILE`
/// constant) so the migration cannot drift from the runtime's read
/// path.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct MigrateSessionFileArgs {
    /// Repo-relative path of the legacy session JSON, host-supplied
    /// from the bootstrap-substituted `{cli-config-dir}/{project}-session.json`
    /// template (e.g., `.claude/gov-session.json`,
    /// `.claude/anvil-session.json`, `.augment/anvil-session.json`).
    /// Validated as relative-and-no-`..`.
    #[arg(long)]
    pub legacy_path: String,
}

/// Result for `migrate-session-file`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct MigrateSessionFileResult {
    /// Repo-relative path of the legacy file the primitive operated on
    /// (echoes the input `legacy-path`).
    pub source: String,
    /// Repo-relative path of the consolidated session file. Always
    /// `.ductus/session.toml` (the runtime's `SESSION_FILE` constant).
    pub dest: String,
    /// `"migrated"` — legacy file translated into a fresh
    /// `.ductus/session.toml` and deleted.
    /// `"kept-existing"` — `.ductus/session.toml` already existed; the
    /// new file was left untouched and the legacy file was deleted.
    /// `"no-legacy"` — no legacy file present at `legacy-path`; no-op.
    pub action: String,
    /// `true` when the legacy file was removed from disk; `false` only
    /// when `action == "no-legacy"`.
    pub legacy_deleted: bool,
}

// -- write-session -----------------------------------------------------------

/// Args for `write-session`. Sets the session state at the active
/// session file — `.ductus/session.toml`, or the legacy root
/// `.govern.session.toml` pre-migration (the `/ductus` migration is the
/// sole cutover); gitignored either way. The `scenario` and `scenario-path` fields
/// are paired — both must be supplied together or both omitted; omitting
/// both clears any previously set scenario.
///
/// Three write shapes, in precedence order:
///
/// 1. **Clear write** (`clear: true`) — removes the target block
///    (feature / path / scenario / scenario-path / set-at) while
///    preserving `cli-config-dir`. Mutually exclusive with every target
///    field; `cli-config-dir` may still be supplied and overrides the
///    preserved value.
/// 2. **Target write** (`feature` + `path`) — sets the target and a
///    fresh `set-at`, preserving `cli-config-dir`.
/// 3. **Host-config write** (only `cli-config-dir`) — sets the agent
///    identity, preserving the existing target.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct WriteSessionArgs {
    /// Feature slug (e.g., `022-deterministic-runtime`). Supplying it makes
    /// this a *target write* — feature, path, scenario, and a fresh `set-at`
    /// are written, preserving the per-contributor `cli-config-dir`. Omit it
    /// (supplying only `cli-config-dir`) for a *host-config write* that sets
    /// the agent identity while preserving the existing target. Must be
    /// supplied together with `path`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub feature: Option<String>,
    /// Repo-relative spec directory (e.g., `specs/022-deterministic-runtime`).
    /// The TOML key in the written session file is `path`, matching the
    /// convention used by `dashboard`'s reader and by host-written
    /// sessions in adopter repos pre-consolidation. Must be supplied
    /// together with `feature`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub path: Option<String>,
    /// Optional scenario slug. Must be supplied iff `scenario-path` is set,
    /// and only on a target write (with `feature`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub scenario: Option<String>,
    /// Optional repo-relative scenario file path. Must be supplied iff
    /// `scenario` is set. Stored as `scenario-path` (kebab-case) in the
    /// written session TOML.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub scenario_path: Option<String>,
    /// Optional per-contributor agent config-dir name (`.claude`, `.augment`,
    /// `.opencode`, `.agents`). Written to the gitignored session file by
    /// `/ductus` so a teammate's agent choice never lands in committed
    /// config. Read back by `crate::host::Host`. On a target write it is
    /// preserved from the existing file unless supplied here.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cli-config-dir"
    )]
    #[arg(long = "cli-config-dir")]
    pub cli_config_dir: Option<String>,
    /// Clear mode: remove the target block (feature / path / scenario /
    /// scenario-path / set-at) while preserving the per-contributor
    /// `cli-config-dir`. Mutually exclusive with a target write —
    /// supplying `clear` together with any of `feature`, `path`,
    /// `scenario`, or `scenario-path` is rejected. A `cli-config-dir`
    /// supplied alongside `clear` still applies (the supplied value
    /// overrides the preserved one).
    #[serde(default)]
    #[arg(long)]
    pub clear: bool,
}

/// Result for `write-session`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct WriteSessionResult {
    /// Repo-relative path of the written session file — the active file
    /// the write resolved (`.ductus/session.toml`, or the legacy root
    /// `.govern.session.toml` pre-migration); kept on the result for
    /// symmetry with other write primitives' return shapes.
    pub path: String,
    /// `true` when the file did not exist before this call, `false` when
    /// an existing file was overwritten in place.
    pub created: bool,
}

// -- resolve-references ------------------------------------------------------

/// Args for `resolve-references`. Resolves the consumer feature's derived
/// `references:` index (see spec 030) against the `.ductus/config.toml`
/// `[services]` registry, reading each linked spec's live `status` from its local
/// checkout. Takes only the consumer feature; the repo root is supplied by
/// the runtime.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct ResolveReferencesArgs {
    /// Consumer feature directory name under `specs/` whose `references:`
    /// index is resolved.
    #[arg(long)]
    pub feature: String,
}

/// Closed outcome enum for one resolved cross-service reference. Decided by
/// deterministic predicates — no prose is read for intent. Canonical in
/// `specs/030-cross-service-references/data-model.md`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ReferenceOutcome {
    /// Registered, checkout reachable, target spec resolves, `status` present
    /// and in the allowed set. `status` carries the linked lifecycle value.
    Ok,
    /// The reference's service is null (the href repo matched no `[services]`
    /// entry at harvest time, or the alias is no longer registered) — a plain
    /// navigational link; status not attempted.
    Unregistered,
    /// Registered, but the service's local `path` is missing or not a usable
    /// checkout. Informational unknown, never reported as broken.
    NotCheckedOut,
    /// Registered and reachable, but the target spec does not resolve
    /// (renamed / moved / deleted / mistyped upstream, or a malformed URL that
    /// yielded no such spec). A provable defect — an analyze finding.
    Broken,
    /// The target file exists but its `status` cannot be read (no frontmatter,
    /// malformed YAML, missing or out-of-set `status`). Surfaced, never silent;
    /// the defect is upstream's.
    StatusUnreadable,
}

/// One resolution record: the input reference plus its classified outcome
/// and, on `ok`, the linked lifecycle status.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ResolutionRecord {
    /// Matched registry alias, or `null` when the reference is `unregistered`.
    pub service: Option<String>,
    /// Target `NNN-slug` (the stable reference identity).
    pub spec: String,
    /// Classified outcome.
    pub outcome: ReferenceOutcome,
    /// Linked lifecycle status; non-null only when `outcome` is `ok`.
    pub status: Option<String>,
}

/// Result for `resolve-references`: one record per entry in the consumer's
/// `references:` index, in index order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ResolveReferencesResult {
    /// Resolution records in the consumer spec's `references:` order.
    pub references: Vec<ResolutionRecord>,
    /// Repo-relative path to the consumer spec file.
    pub path: String,
}

// -- resolve-feature ----------------------------------------------------------

/// Args for `resolve-feature`. Scans the configured spec root and resolves
/// a user-supplied identifier to a feature directory — the deterministic
/// core of `/ductus:target`'s specs-dir scan.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct ResolveFeatureArgs {
    /// Identifier to resolve: an exact feature directory name
    /// (`022-deterministic-runtime`), a feature number (`22` or `022` —
    /// both match the zero-padded `022-` prefix), or a partial slug
    /// substring (`deterministic`, matched case-insensitively).
    #[arg(long)]
    pub identifier: String,
    /// Optional scenario slug. When supplied and the feature resolves, the
    /// result's `scenario` block reports the scenario file's existence and
    /// its `section` frontmatter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub scenario: Option<String>,
}

/// Closed outcome enum for `resolve-feature`. Ambiguity and no-match are
/// **domain outcomes** (the host mediates the follow-up prompt), never
/// operational errors — per the scaffolding-primitives scenario's edge
/// cases ("choosing stays with the user through the host").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ResolveFeatureOutcome {
    /// Exactly one feature matched the identifier.
    Resolved,
    /// A partial identifier matched more than one feature; `candidates`
    /// carries the sorted matches for the host's disambiguation prompt.
    Ambiguous,
    /// No feature matched the identifier.
    NotFound,
}

/// Scenario detail attached to a `resolve-feature` result when the args
/// named a scenario slug and the feature resolved.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ResolvedScenario {
    /// Scenario slug echoed from the args.
    pub slug: String,
    /// Repo-relative path of the scenario file (reported whether or not
    /// the file exists, so the host can offer to create it).
    pub path: String,
    /// Whether the scenario file exists on disk.
    pub exists: bool,
    /// Scenario frontmatter `section` field (falling back to the legacy
    /// pre-017 `spec-ref` field). Empty when the file is absent,
    /// unreadable, or carries neither — mirroring `dashboard`'s
    /// scenario-detail degradation.
    pub section: String,
}

/// Result for `resolve-feature`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ResolveFeatureResult {
    /// How the identifier resolved.
    pub outcome: ResolveFeatureOutcome,
    /// Resolved feature directory name; present only on `resolved`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    /// Repo-relative feature directory path; present only on `resolved`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Spec frontmatter `status`; present on `resolved` when the feature's
    /// `spec.md` is readable (best-effort — a malformed spec degrades to
    /// an absent status rather than failing the resolution).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Sorted candidate directory names; populated on `ambiguous`, empty
    /// on the other outcomes.
    #[serde(default)]
    pub candidates: Vec<String>,
    /// Scenario detail; present when the args named a scenario slug and
    /// the outcome is `resolved`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scenario: Option<ResolvedScenario>,
}

// -- create-feature -----------------------------------------------------------

/// Args for `create-feature`. Computes the next feature number, derives
/// the kebab-case slug from `title`, creates `{specs-root}/{NNN-slug}/`,
/// and copies the spec template into it — the deterministic scaffold step
/// of `/ductus:specify`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct CreateFeatureArgs {
    /// Feature title. The directory slug is derived from it: lowercased,
    /// every run of non-alphanumeric characters collapsed to a single
    /// hyphen, leading/trailing hyphens trimmed.
    #[arg(long)]
    pub title: String,
    /// Branch identifier, for branch-scoped creation (spec 051).
    ///
    /// Absent is the default and unchanged path: the feature takes the
    /// next sequential number. Present switches to the branch-scoped
    /// form, `<identifier>.<n>-<slug>`, where the counter runs within
    /// this identifier alone.
    ///
    /// (Written with angle brackets rather than braces because clap
    /// renders this doc comment as `--help` text and expands a
    /// brace-wrapped `n` there as a newline escape.)
    ///
    /// The value is an opaque operator-supplied token, not a number —
    /// trackers disagree (`PROJ-1111`, `1111-PROJ`) — and is sanitized by
    /// the same rule that derives a slug from a title, so the caller must
    /// echo the `identifier` this primitive returns rather than assume
    /// its input survived verbatim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub branch_id: Option<String>,
    /// The upstream spec this branch-scoped spec folds back into, written
    /// to its `folds-into:` frontmatter.
    ///
    /// Required with `branch_id` and meaningless without it. A
    /// branch-scoped spec exists in order to be folded, so there is no
    /// way to create one that names no target: the number exists to keep
    /// the merge clean, and the target is what makes the spec actionable
    /// once it lands.
    ///
    /// Validated for *shape* only. The target normally lives on the
    /// upstream branch and is absent from the tree creating this spec, so
    /// requiring it to resolve would refuse the feature's normal case.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub fold_into: Option<String>,
}

/// Result for `create-feature`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CreateFeatureResult {
    /// Whether the feature directory was created. `false` is the refusal
    /// domain outcome: the derived directory already existed and nothing
    /// was written (no overwrite path).
    pub created: bool,
    /// Feature directory name (`{NNN}-{slug}`).
    pub feature: String,
    /// Repo-relative feature directory path.
    pub path: String,
    /// Repo-relative path of the spec template copied into the new
    /// directory; absent on the refusal outcome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    /// The **sanitized** branch identifier actually used; absent on a
    /// sequential creation.
    ///
    /// Returned because sanitization can change what the operator typed
    /// (`PROJ-1111` becomes `proj-1111`), and a transformation the
    /// operator never sees is one they discover from a directory name
    /// later. The calling command echoes this at its confirmation prompt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

// -- create-plan-artifacts -------------------------------------------------

/// Args for `create-plan-artifacts`. Copies the plan/tasks (and, on
/// request, data-model) templates into an existing feature directory —
/// the deterministic template-copy and existing-artifact-detection step
/// of `/ductus:plan` (the plan-side mirror of `create-feature`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct CreatePlanArtifactsArgs {
    /// Feature directory name under the configured spec root.
    #[arg(long)]
    pub feature: String,
    /// Also copy the data-model template. Whether the feature introduces
    /// or modifies domain entities is the host's judgment, so
    /// `data-model.md` joins the copy set only on request. A pre-existing
    /// `data-model.md` is reported (`kept`) regardless, so the
    /// existing-artifact prompt always sees the full set.
    #[serde(default)]
    #[arg(long)]
    pub include_data_model: bool,
    /// Copy fresh templates over pre-existing artifacts — the "replace"
    /// branch of the existing-artifact prompt, passed only after the user
    /// confirms. Default `false`: pre-existing artifacts are never
    /// touched (`kept`).
    #[serde(default)]
    #[arg(long)]
    pub overwrite: bool,
}

/// Outcome for one plan artifact.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum PlanArtifactAction {
    /// The artifact was missing; the template was copied in.
    Created,
    /// The artifact pre-existed and was left untouched.
    Kept,
    /// The artifact pre-existed and the template was copied over it
    /// (`overwrite: true`).
    Replaced,
}

/// Per-artifact report entry for `create-plan-artifacts`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct PlanArtifact {
    /// Artifact file name: `plan.md`, `tasks.md`, or `data-model.md`.
    pub file: String,
    /// Repo-relative artifact path.
    pub path: String,
    /// What happened to the artifact this call.
    pub action: PlanArtifactAction,
    /// Repo-relative path of the template copied in; absent on `kept`.
    /// No last-modified stamp accompanies `kept` entries — primitive
    /// results carry no wall-clock data (same rule as `write-session`,
    /// whose `set-at` goes into the file, never the result), so the
    /// envelope stream stays deterministic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

/// Result for `create-plan-artifacts`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CreatePlanArtifactsResult {
    /// Repo-relative feature directory path.
    pub path: String,
    /// Per-artifact outcomes in canonical order (`plan.md`, `tasks.md`,
    /// `data-model.md`). A `data-model.md` that is neither requested nor
    /// on disk is omitted.
    pub artifacts: Vec<PlanArtifact>,
}

// -- check-review-gate -------------------------------------------------------

/// Args for `check-review-gate`. Evaluates `/ductus:implement`'s pre-done
/// review gate for one feature: the feature directory's markdown lint,
/// then the spec frontmatter `review:` block, in the completion gate's
/// documented order (first failing check wins).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct CheckReviewGateArgs {
    /// Feature directory name under the configured spec root.
    #[arg(long)]
    pub feature: String,
}

/// First failing check of the pre-done review gate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ReviewGateBlock {
    /// The feature directory's markdown files failed `markdownlint-cli2`
    /// (violations, or a non-zero exit the parser could not attribute).
    MarkdownLint,
    /// One or more of the feature's scenarios carry unresolved open
    /// questions. Ordered before the `review:` checks: an unresolved design
    /// question is more upstream than a missing review, so surfacing it
    /// first avoids sending a contributor to review a design that is about
    /// to change (spec 046).
    ScenarioOpenQuestions,
    /// The spec declares `folds-into`: it is a branch-scoped staging spec
    /// whose content has not yet been folded into its upstream home
    /// (spec 051).
    ///
    /// The same category as an unresolved scenario question, and ordered
    /// beside it for the same reason: both say the spec carries an
    /// undischarged obligation, which makes asking whether its review is
    /// fresh beside the point. The consequence is that the branch-scoped
    /// form has no `done` state at all — it is retired by fold-back, not
    /// completed.
    PendingFold,
    /// The spec has no completed review: the `review:` block is absent or
    /// its `last-run` is null.
    NotReviewed,
    /// The last review left blocking MUST violations
    /// (`review.blocking: true`).
    MustViolations,
    /// The review is **stale**: a file the spec's plan declares as its own
    /// surface changed after `review.reviewed-against`, so the recorded
    /// verdict describes a diff that no longer exists.
    ///
    /// Ordered last because it is the weakest claim — the other four say a
    /// review is missing or failing, this one says a passing review is out
    /// of date. Without it a review that predates the code it nominally
    /// covers satisfies every other check, which is how `gvrn-v0.26.2`
    /// shipped three commits of unreviewed runtime change (spec 022 review,
    /// 2026-08-03).
    ReviewStale,
    /// The spec has no completed analysis: the `analyze:` block is absent or
    /// its `last-run` is null.
    ///
    /// Ordered after every `review:` check because the pipeline is
    /// `review → analyze → done`: a spec that has not been reviewed has not
    /// reached the point where analysis is the next thing owed, and sending
    /// a contributor to analyze a spec whose review is missing or failing
    /// would name the later gate for an earlier defect.
    ///
    /// This check exists because its absence was reached in practice: on
    /// 2026-09-05 two specs were advanced to `done` on the review gate
    /// alone, and one of them was released to crates.io — irreversibly —
    /// before anything noticed, because nothing could. Analyze left no
    /// trace, so a spec that had passed both gates and one that had passed
    /// only the first were identical on disk.
    NotAnalyzed,
    /// The last analysis left findings in the hard-fail or blocking tier
    /// (`analyze.blocking: true`).
    ///
    /// Advisory findings are deliberately not a gate here — see
    /// [`AnalyzeBlock::advisory`] for why this does not mirror the review
    /// gate's treatment of an outstanding SHOULD.
    AnalyzeFindings,
}

/// Result for `check-review-gate`. A blocked gate is a domain outcome —
/// the host halts with `message` and does not propose the in-progress →
/// done transition; it is never an operational error.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CheckReviewGateResult {
    /// Whether the gate passes and the transition may be proposed.
    pub passed: bool,
    /// First failing check, in gate order; absent on pass.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_by: Option<ReviewGateBlock>,
    /// The canonical blocked message for the failing check (the
    /// `blocked: …` texts documented in `/ductus:implement`'s completion
    /// gate, with the adopter's `[host] project` command namespace
    /// substituted); absent on pass.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Follow-up guidance accompanying the message — the
    /// resolve-or-waive options on `must-violations`; absent otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guidance: Option<String>,
    /// Markdown-lint violations backing a `markdown-lint` block; empty
    /// otherwise.
    pub violations: Vec<MarkdownViolation>,
}

// -- append-question ---------------------------------------------------------

/// Args for `append-question`. Appends one question bullet to the target
/// artifact's `## Open Questions` section — `/ductus:amend`'s question-route
/// write, including the same-write status back-edge on spec targets.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct AppendQuestionArgs {
    /// Feature directory name under the configured spec root.
    #[arg(long)]
    pub feature: String,
    /// Refined question text, appended as a `- {question}` bullet. Pass the
    /// question text only — the primitive renders the `- ` marker, and a
    /// caller-supplied leading marker is stripped before both the dedup
    /// comparison and the write, so a marked and an unmarked form of the same
    /// question are one entry. Single-line; embedded newlines are rejected
    /// (structure injection).
    #[arg(long)]
    pub question: String,
    /// Optional scenario slug: the target artifact becomes
    /// `scenarios/{slug}.md` instead of `spec.md`, and no status
    /// back-edge applies (scenarios have no status field).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub scenario: Option<String>,
}

/// Result for `append-question`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct AppendQuestionResult {
    /// Repo-relative path of the target artifact.
    pub path: String,
    /// Whether the question was appended. `false` is the dedup domain
    /// outcome: an equivalent entry already exists and nothing was
    /// written.
    pub appended: bool,
    /// The existing entry that suppressed the append (normalized-
    /// whitespace, case-insensitive match); present only when deduped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duplicate_of: Option<String>,
    /// Whether the `## Open Questions` section had to be created.
    pub section_created: bool,
    /// Whether the same-write status back-edge fired: a spec target
    /// whose status was `clarified`, `planned`, `in-progress`, or `done`
    /// reverts to `draft` in the same atomic write as the append.
    pub status_reverted: bool,
    /// The status the back-edge reverted from; present only when
    /// `status-reverted` is `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_status: Option<String>,
}

// -- diff-cross-spec ---------------------------------------------------------

/// Args for `diff-cross-spec`. Computes `/ductus:implement`'s cross-spec
/// impact surface: the diff from the feature's first spec-dir commit to
/// the working tree, scoped to the spec root and filtered to paths
/// outside the feature's own directory, plus the lines added to
/// `{specs-root}/inbox.md` in the same window.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct DiffCrossSpecArgs {
    /// Feature directory name under the configured spec root.
    #[arg(long)]
    pub feature: String,
}

/// Result for `diff-cross-spec`. Read-only; both lists empty is the
/// no-impact domain outcome.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct DiffCrossSpecResult {
    /// First commit touching the feature's spec dir (the diff base;
    /// shared derivation with `derive-boundary`).
    pub first_commit: String,
    /// Current HEAD commit.
    pub current_head: String,
    /// Changed paths under the spec root but outside the feature's own
    /// directory (sorted; `{specs-root}/inbox.md` is excluded — its
    /// additions report separately below). The diff runs against the
    /// working tree (index + untracked included), so uncommitted sibling
    /// edits surface at the per-task summary; on a clean tree this equals
    /// the documented `git diff <first-commit>..HEAD -- {specs-root}/`.
    pub cross_spec_paths: Vec<String>,
    /// Bullet lines added to `{specs-root}/inbox.md` in the window — the
    /// issues captured during the feature's work (§brownfield-inbox).
    /// Filtered through the shared bullet grammar, so structural
    /// additions (the heading, blanks when the whole file is new) never
    /// report as captured items.
    pub inbox_additions: Vec<String>,
    /// Next-step guidance, present only when no commit touches the spec
    /// dir. Without it the empty lists above would read as *"no cross-spec
    /// impact"* — a positive claim — when the truth is that there is no
    /// window to diff and the impact is **unknowable**. Its presence tells
    /// the caller to report the difference rather than the reassurance
    /// (scenario derive-boundary-uncommitted-spec-dir).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guidance: Option<String>,
}

// -- append-inbox --------------------------------------------------------------

/// Args for `append-inbox`. Appends one `- {text}` bullet to
/// `{specs-root}/inbox.md`, creating the file when missing. The optional
/// `dedup-prefix` makes the append idempotent for auto-capture callers
/// (the bootstrap audit's dedup-by-prefix contract).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct AppendInboxArgs {
    /// Single-line bullet text, appended as `- [ ] {text}`. Pass the item
    /// content only — the primitive renders the marker, and a caller-supplied
    /// leading marker is stripped so it cannot double. Embedded newlines are
    /// rejected (structure injection into inbox.md).
    #[arg(long)]
    pub text: String,
    /// Optional dedup guard: when an existing inbox bullet's text starts
    /// with this prefix, nothing is written and the result reports
    /// `deduped: true`. Compared against marker-stripped bullet text, so a
    /// leading marker on the prefix is stripped too — otherwise the prefix
    /// would match nothing and the guard would silently no-op.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub dedup_prefix: Option<String>,
}

/// Result for `append-inbox`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct AppendInboxResult {
    /// Repo-relative path of the inbox file.
    pub path: String,
    /// Whether the inbox file was created by this invocation.
    pub created: bool,
    /// `true` when `dedup-prefix` matched an existing bullet and no write
    /// happened.
    pub deduped: bool,
    /// Total real (comment/fence-aware) inbox bullets after this call — the
    /// count `/ductus:log` reports without hand-counting. On a `deduped` no-op
    /// this is the pre-existing total.
    pub item_count: u32,
}

// -- remove-inbox-item ---------------------------------------------------------

/// Args for `remove-inbox-item`. Removes the first bullet from
/// `{specs-root}/inbox.md` whose text matches `item`. The complement of
/// `append-inbox`; the deterministic surface behind `/ductus:groom`'s per-item
/// inbox removal (step 8), which previously edited the file by hand.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct RemoveInboxItemArgs {
    /// The bullet text to remove: the first inbox bullet whose text (after
    /// the `- ` marker and an optional `[ ]`/`[x]` checkbox are stripped),
    /// trimmed, equals this value is removed. Single-line; embedded newlines
    /// are rejected.
    #[arg(long)]
    pub item: String,
}

/// Result for `remove-inbox-item`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct RemoveInboxItemResult {
    /// Repo-relative path of the inbox file.
    pub path: String,
    /// Whether a matching bullet was found and removed. A no-match (or a
    /// missing inbox file) is a clean domain outcome, not an error.
    pub removed: bool,
    /// Number of bullet items remaining in the inbox after the operation.
    pub remaining_count: u32,
}

// -- check-artifacts -----------------------------------------------------------

/// Args for `check-artifacts`. Runs the residual deterministic check
/// families from `/ductus:analyze`'s markdown-only reference against one
/// feature (`--all` stays with the caller looping).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct CheckArtifactsArgs {
    /// Feature directory name under the configured spec root.
    #[arg(long)]
    pub feature: String,
}

/// One deterministic artifact finding. Family names and severity tiers
/// mirror `framework/commands/analyze.md`'s markdown-only reference —
/// the primitive mechanizes the documented policy, it introduces none.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ArtifactFinding {
    /// Check family: `artifact-completeness`, `task-consistency`,
    /// `scenario-consistency`, `review-state-drift`,
    /// `scenario-open-questions`, `link-adjacent-drift`,
    /// `criterion-path-existence`, `criterion-labels`, or
    pub family: String,
    /// Severity tier per the reference's assignments: `blocking`
    /// (artifact completeness, task consistency, review state drift, and
    /// scenario open questions at `done`) or `advisory` (scenario
    /// consistency, scenario open questions below `done`, link-adjacent
    /// drift, criterion path existence, and criterion labels
    /// reciprocity).
    pub severity: String,
    /// Human-readable description of the finding.
    pub message: String,
    /// Repo-relative path of the artifact the finding anchors to.
    pub path: String,
}

/// One target a family could not examine.
///
/// Distinguishes *"examined the subject and found nothing"* from *"could not
/// examine the subject"* (`QUAL-CLAIM-001`). The families added by spec 045
/// read targets that may be unreadable — a link's destination, a criterion's
/// path — and 045 forbids escalating an unreadable one into a finding. Without
/// this list, a family that examined every target and found nothing would
/// return exactly what a family that could examine nothing returns, and a
/// caller would read the reassuring one.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct SkippedTarget {
    /// Family that skipped it, matching [`ArtifactFinding::family`].
    pub family: String,
    /// Why it could not be examined — a closed set, so repeat runs over
    /// unchanged inputs are byte-identical: `target-missing`,
    /// `target-unparseable`, `no-readable-state`, `root-absent`,
    /// `ships-to-adopter`, `artifact-unreadable`, or `not-a-live-claim`.
    pub reason: String,
    /// Repo-relative path of the target that was not examined.
    pub path: String,
}

/// Result for `check-artifacts`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CheckArtifactsResult {
    /// Feature directory name echoed from the args.
    pub feature: String,
    /// Spec frontmatter `status` the tier classification ran against.
    pub status: String,
    /// Findings across the eight families, in family order (completeness →
    /// task consistency → scenario consistency → review drift → scenario
    /// open questions → link-adjacent drift → criterion path existence →
    /// criterion labels).
    pub findings: Vec<ArtifactFinding>,
    /// `true` when no family produced a finding.
    ///
    /// Deliberately **not** widened to account for [`Self::skipped`]:
    /// redefining it would silently change the verdict the families
    /// predating spec 045 produce. The assurance lives in the pair —
    /// `clean` with an empty `skipped` is verified-clean, `clean` with a
    /// non-empty `skipped` is partially examined.
    pub clean: bool,
    /// Targets no family could examine. Empty for the five families
    /// predating spec 045 and for `criterion-labels`, whose subjects are
    /// fully examinable by construction — only the two families that read
    /// an artifact's *targets* can fail to look.
    #[serde(default)]
    pub skipped: Vec<SkippedTarget>,
    /// Repo-relative path to the spec file.
    pub path: String,
}

// -- derive-routing-candidates -------------------------------------------------

/// Args for `derive-routing-candidates`. Derives the homes new work could
/// belong to, so `/ductus:specify` can run the routing decision *before*
/// `create-feature` writes anything (spec 022 scenario
/// `specify-routes-before-scaffolding`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct DeriveRoutingCandidatesArgs {
    /// The proposed work, in the requester's words — `/ductus:specify`'s
    /// feature description. Matching is lexical over this text.
    ///
    /// The `title` alias reads the key `create-feature` already consumes, so
    /// the exec walker binds both primitives from one context value instead of
    /// the operator supplying the same string under two names.
    #[serde(alias = "title")]
    #[arg(long)]
    pub description: String,
    /// The command whose routing tree already ran, when one did (e.g.
    /// `groom`). Present means the decision has been made already, so
    /// `gate-required` comes back false and the caller does not ask twice.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub routed_by: Option<String>,
}

/// One derived home the proposed work could belong to instead of a new spec.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct RoutingCandidate {
    /// Route from the `routeInboxItem` vocabulary this candidate implies:
    /// `rule` (amend an existing rule surface) or `scenario` (add a scenario
    /// to an existing spec). `spec` — create a new one — is the absence of
    /// candidates, never a candidate itself.
    pub route: String,
    /// Which derivation produced it: `runtime-work`, `rule-surface`, or
    /// `spec-corpus`. Ordered by that precedence in the result.
    pub source: String,
    /// The home: a rule-file basename, or a feature slug.
    pub target: String,
    /// Repo-relative path of the target.
    pub path: String,
    /// Target spec's frontmatter `status`; empty for a rule-file candidate
    /// and for a spec whose frontmatter could not be read.
    #[serde(default)]
    pub status: String,
    /// `true` when accepting this candidate implies the `done → in-progress`
    /// back-edge, so the confirmation can name the reopen before it happens
    /// exactly as `/ductus:groom`'s does.
    pub reopens: bool,
    /// Why it matched — the shared tokens, or the runtime artifact named.
    /// Surfaced in the gate so the operator can judge the match rather than
    /// take it on trust.
    pub reason: String,
}

/// One derivation source that could not be examined. The reason a
/// zero-candidate result is not automatically *no candidate found*.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct RoutingSkip {
    /// Source that could not run: `rule-surface`, `spec-corpus`, or
    /// `runtime-work`.
    pub source: String,
    /// What stopped it, in the operator's terms.
    pub reason: String,
    /// Repo-relative path the source would have read; empty when the source
    /// has no single path.
    #[serde(default)]
    pub path: String,
}

/// Result for `derive-routing-candidates`.
///
/// `candidates` empty with `skipped` empty is **no candidate found** — every
/// source ran and matched nothing, so a new spec is the right answer.
/// `candidates` empty with `skipped` non-empty is **could not derive
/// candidates** — a different answer that must not be reported as the first
/// (`QUAL-CLAIM-001`). `sources-examined` is what makes the distinction
/// checkable rather than inferred.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct DeriveRoutingCandidatesResult {
    /// The description the derivation ran against, echoed so the routing
    /// extension point downstream reads one key whichever name the caller
    /// supplied it under.
    pub description: String,
    /// Derived homes, ordered by source precedence (`runtime-work`,
    /// `rule-surface`, `spec-corpus`), then by descending match strength,
    /// then by target for a stable render.
    pub candidates: Vec<RoutingCandidate>,
    /// Sources that ran to completion. A source appears here or in
    /// `skipped`, never both and never neither.
    pub sources_examined: Vec<String>,
    /// Sources that could not be examined.
    #[serde(default)]
    pub skipped: Vec<RoutingSkip>,
    /// `false` when `routed-by` named a command whose tree already ran — the
    /// caller skips the routing decision and its confirmation rather than
    /// asking a question that has been answered.
    pub gate_required: bool,
    /// `true` when at least one source could not be examined, so the caller
    /// reports *could not derive candidates* rather than *none found*.
    pub derivation_incomplete: bool,
}

// -- check-orphaned-references -------------------------------------------------

/// Args for `check-orphaned-references`. Reports adopter-owned files whose
/// references to ductus-managed paths no longer resolve — the runtime half of
/// spec 027's `migration-chain-reference-integrity`, invoked from both
/// `/ductus:analyze` §Project-level consistency and the bootstrap's
/// migration-batch end (spec 022 scenario `orphaned-reference-check`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct CheckOrphanedReferencesArgs {}

/// One adopter-owned reference that does not resolve.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct OrphanedReference {
    /// Repo-relative path of the adopter-owned file carrying the reference.
    pub referrer: String,
    /// The ductus-managed path it names, which does not exist.
    pub target: String,
    /// 1-based line number within `referrer`.
    pub line: u32,
    /// Registry entry whose `target_paths` covers `target`, when the registry
    /// was readable. Empty under `watermark` attribution — an empty string is
    /// *not* a migration named "", and the `attribution` field is what tells
    /// a caller which reading applies.
    #[serde(default)]
    pub migration: String,
}

/// A referrer the check could not read, recorded so an empty `findings` is
/// never mistaken for a verified-clean tree.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct OrphanedReferenceSkip {
    /// Repo-relative path that went unexamined.
    pub path: String,
    /// Why, in the operator's terms.
    pub reason: String,
}

/// Result for `check-orphaned-references`.
///
/// `findings` empty with `skipped` empty is **examined and clean**. `findings`
/// empty with `skipped` non-empty is **could not examine**, and the caller must
/// not render it as assurance (`QUAL-CLAIM-001`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CheckOrphanedReferencesResult {
    /// Unresolved references, in referrer then line order.
    #[serde(default)]
    pub findings: Vec<OrphanedReference>,
    /// Referrers examined, repo-relative — the subject the counts describe.
    /// A caller that reports "clean" quantifies it from this rather than
    /// asserting a property of files nobody enumerated.
    #[serde(default)]
    pub examined: Vec<String>,
    /// Referrers that could not be read.
    #[serde(default)]
    pub skipped: Vec<OrphanedReferenceSkip>,
    /// The path prefixes a reference had to carry to be examined at all —
    /// current *and* historical framework-managed roots.
    ///
    /// [`Self::examined`] bounds the claim by **subject** (which referrers were
    /// read); this bounds it by **scope** (which reference forms were
    /// recognized). Without it a clean result reads as "no orphans" when it
    /// means "no orphans among paths carrying one of these prefixes", and a
    /// reference outside the list cannot even be reported as skipped, because
    /// nothing recognized it as a reference. A caller quantifying a clean
    /// verdict states this alongside `examined` (spec 048 AC10 review).
    #[serde(default)]
    pub matched_prefixes: Vec<String>,
    /// `registry` when `framework/migrations.toml` was readable and findings
    /// carry a `migration`; `watermark` when it was not, and `last-applied`
    /// is the only migration context available.
    pub attribution: String,
    /// `[migrations].last_applied` from the active config file — the adopter's
    /// migration watermark. Empty when no `[migrations]` section exists, which
    /// means *no migration has been applied*, not *a migration named ""*.
    #[serde(default)]
    pub last_applied: String,
}

// -- check-corpus-links --------------------------------------------------------

/// Which markdown files `check-corpus-links` examines.
///
/// One resolver, two subjects. The link grammar, the code-span stripping,
/// the shape filter, and the lexical resolution are identical either way —
/// what differs is the enumeration, and it differs for a stated reason
/// rather than by accident (spec 026 scenario `link-check-consolidation`).
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::ValueEnum,
)]
#[serde(rename_all = "kebab-case")]
pub enum LinkScope {
    /// Every `.md` file under the configured spec root. The default, and
    /// the adopter-facing subject: it is what the shipped pre-commit hook
    /// checks, and it needs no git index.
    #[default]
    SpecCorpus,
    /// Every tracked `.md` file in the repository. The maintainer subject —
    /// `/{project}:audit` Family 26 — which reaches framework sources,
    /// scripts, and the audit's own documentation, none of which an adopter
    /// has any business being told about.
    ///
    /// Enumerated from the **git index** rather than a worktree walk, so an
    /// untracked draft is never reported and `runtime/target` is never
    /// descended into.
    Repository,
}

/// Args for `check-corpus-links`. Reports relative markdown links that
/// resolve to nothing, over the subject `scope` names.
///
/// Three checks sit near this ground and none of them covers it:
/// `check-orphaned-references` scopes to the adopter-owned bootstrap referrers
/// pointing into ductus-managed roots, `/{project}:analyze` is bounded to one
/// feature plus its declared dependencies, and `/{project}:audit` Family 26
/// performs exactly the right check but is maintainer-only — adopters never
/// invoke it. So an adopter who deletes or renames a spec directory dangles
/// every inbound pointer and nothing reports it (spec 022 scenario
/// `adopter-corpus-link-integrity`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct CheckCorpusLinksArgs {
    /// The subject to examine. Defaults to the spec corpus, so an adopter's
    /// hook and every existing caller are unaffected by the widening.
    #[serde(default)]
    #[arg(long, value_enum, default_value_t)]
    pub scope: LinkScope,
}

/// One relative link whose target does not exist.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct BrokenCorpusLink {
    /// Repo-relative path of the file carrying the link.
    pub path: String,
    /// 1-based line number, counting from the top of the file (frontmatter
    /// included), so the citation matches what an editor shows.
    pub line: usize,
    /// The link target exactly as written, fragment included.
    pub target: String,
    /// What to do about it. A target that resolves one directory up is the
    /// dominant class — a scenario lives one tier deeper than its spec, so
    /// one `../` too few renders fine and resolves to nothing — and naming
    /// that case turns a report into a fix.
    pub guidance: String,
}

/// A file the scan could not read.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CorpusLinkSkip {
    /// Repo-relative path of the file.
    pub path: String,
    /// Why it could not be examined.
    pub reason: String,
}

/// Result for `check-corpus-links`.
///
/// `broken` empty with `skipped` empty **and** a non-zero `examined` is
/// examined-and-clean. `broken` empty with anything in `skipped`, or with
/// `examined` at zero, is **could not examine** — a caller must not render
/// either as assurance (`QUAL-CLAIM-001`). That distinction is the whole
/// reason the counts are returned rather than a bare list.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CheckCorpusLinksResult {
    /// Broken links, in file then line order.
    #[serde(default)]
    pub broken: Vec<BrokenCorpusLink>,
    /// Markdown files actually read, repo-relative — the subject the verdict
    /// describes. Zero examined with zero broken is not a clean corpus.
    #[serde(default)]
    pub examined: Vec<String>,
    /// Files that could not be read. Never silent: an unreadable file is a
    /// file whose links were never checked.
    #[serde(default)]
    pub skipped: Vec<CorpusLinkSkip>,
    /// Files excluded **by construction**, because a link that does not
    /// resolve here is their correct state: the adopter-facing templates
    /// under `{specs-root}/templates/`, whose links resolve in a scaffolded
    /// feature directory rather than in the template's own.
    ///
    /// Counted rather than dropped, so the scope of the verdict is legible.
    ///
    /// Generated command copies (a host's commands directory) are **not**
    /// counted here and are not an exclusion: they sit outside the spec root
    /// entirely, so they were never part of this primitive's subject. The
    /// distinction matters for reading the number — this counts what was in
    /// scope and deliberately not examined, never everything unexamined.
    pub excluded_by_construction: u32,
    /// Link targets skipped as documentation **shapes** rather than
    /// references — a target carrying `NNN`, a `{placeholder}`, a `*`, or a
    /// bare `...`. Prose names link syntax as often as it names files, and
    /// testing a shape against the filesystem manufactures findings out of
    /// documentation.
    pub shapes_skipped: u32,
    /// The spec root the scan walked, repo-relative — the configured
    /// `[paths] specs-root`, not an assumed `specs`. Reported on both
    /// scopes, since a repository scan still resolves it.
    pub specs_root: String,
    /// The subject actually examined, echoed so a reader of the result is
    /// never left inferring it from the counts. A repository scan and a
    /// spec-corpus scan of the same tree differ by hundreds of files, and
    /// a bare `examined` list does not say which one ran.
    pub scope: LinkScope,
    /// Set when the scan could not establish a subject at all: the spec root
    /// is absent or unreadable, so zero examined means *nothing was looked
    /// at* rather than *nothing is broken*. Empty otherwise.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub guidance: String,
}

// -- check-step-references -----------------------------------------------------

/// Args for `check-step-references`. Reports `step N` prose references in a
/// command file that name a step the file does not have. Maintainer scope: the
/// subject is `framework/commands/` plus the two bootstrap procedures, where
/// the drift originates, not the generated copies an adopter cannot repair.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct CheckStepReferencesArgs {}

/// One step-reference defect.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct StepReferenceFinding {
    /// Repo-relative path of the file the finding is in.
    pub file: String,
    /// 1-based line the reference sits on; `0` for a whole-file finding
    /// (`discontinuous`, `no-steps-extracted`), which names no single line.
    pub line: usize,
    /// `unresolved`, `self-reference`, or `discontinuous` — a closed set,
    /// because the repairs differ.
    pub kind: String,
    /// The step number referenced; `0` for a whole-file finding.
    pub reference: u32,
    /// What is wrong, in the maintainer's terms.
    pub message: String,
}

/// A file the check could not read, recorded so an empty `findings` is never
/// mistaken for a verified-clean corpus.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct StepReferenceSkip {
    /// Repo-relative path that could not be examined.
    pub path: String,
    /// Why it could not be.
    pub reason: String,
}

/// Result of `check-step-references`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CheckStepReferencesResult {
    /// Every defect found, in file then line order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub findings: Vec<StepReferenceFinding>,
    /// Files read. A file with no `## Instructions` section is examined and
    /// contributes nothing, which is correct rather than a gap.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examined: Vec<String>,
    /// The subset carrying numbered Instructions steps — the files this
    /// check can actually say something about.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub with_steps: Vec<String>,
    /// Files whose Instructions section holds several numbered lists rather
    /// than one procedure — `amend.md` restarts at 1 under each subsection,
    /// `status.md` uses three separate one-item lists. Their references
    /// cannot be resolved against a single set without inventing findings,
    /// so they are named here rather than examined.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub not_a_procedure: Vec<String>,
    /// Files that could not be read.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skipped: Vec<StepReferenceSkip>,
    /// `step N` mentions outside the Instructions section, counted but never
    /// resolved: the markdown-only sub-procedures restart at 1 and are a
    /// different list. Present so an empty `findings` is not read as *every
    /// reference in the file resolves* (`QUAL-CLAIM-001`).
    #[serde(default)]
    pub references_out_of_subject: u32,
    /// Set when the check could not establish a subject at all.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub guidance: String,
}

// -- check-command-flags -------------------------------------------------------

/// Args for `check-command-flags`. Reports flags a command's Flags table
/// documents but its `argument-hint:` frontmatter omits — the surface a host
/// renders, so an omitted flag is one the operator is never shown. Maintainer
/// scope: the subject is `framework/commands/`, where the divergence
/// originates, not the generated copies an adopter cannot repair.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct CheckCommandFlagsArgs {}

/// One documented-but-unsurfaced flag.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CommandFlagFinding {
    /// Repo-relative path of the command source.
    pub command: String,
    /// The flag the table documents, e.g. `--since`. Empty for the
    /// whole-file case — a Flags table with no `argument-hint` at all — which
    /// names no single flag because every one of them is unsurfaced.
    #[serde(default)]
    pub flag: String,
    /// What is wrong, in the maintainer's terms.
    pub reason: String,
}

/// A command file the check could not read, recorded so an empty `findings`
/// is never mistaken for a verified-clean surface.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CommandFlagSkip {
    /// Repo-relative path that went unexamined.
    pub path: String,
    /// Why, in the maintainer's terms.
    pub reason: String,
}

/// Result for `check-command-flags`.
///
/// `findings` empty with `skipped` empty is **examined and clean** — but only
/// over the subject `with-flags-table` names. A command documenting flags in
/// prose rather than a table is examined and contributes nothing, so the two
/// counts are what let a caller state which claim it is making
/// (`QUAL-CLAIM-001`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CheckCommandFlagsResult {
    /// Unsurfaced flags, in command then table order.
    #[serde(default)]
    pub findings: Vec<CommandFlagFinding>,
    /// Command sources read, repo-relative and sorted.
    #[serde(default)]
    pub examined: Vec<String>,
    /// The subset of `examined` that carries a Flags table — the actual
    /// subject. A clean result quantifies itself from this, not from
    /// `examined`, since a command with no table cannot produce a finding.
    #[serde(default)]
    pub with_flags_table: Vec<String>,
    /// Command sources that could not be read.
    #[serde(default)]
    pub skipped: Vec<CommandFlagSkip>,
    /// Directory the run enumerated, so a caller can tell a genuinely
    /// table-free corpus from a wrong working directory.
    pub commands_dir: String,
    /// Set when the run examined command files but found no Flags table at
    /// all. Two empty sets compare equal, so without this an extraction
    /// failure returns the payload of a clean run. Empty otherwise — the
    /// common case stays quiet, so its silence means "examined and current".
    #[serde(default)]
    pub guidance: String,
}

// -- check-review-agreement ----------------------------------------------------

/// Args for `check-review-agreement`. Compares each spec's frontmatter
/// `review:` block against its own `review.md` frontmatter — the same review
/// recorded twice, with nothing holding the two together until this check.
/// Maintainer scope: the subject is every spec under the configured spec root
/// that carries both records.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct CheckReviewAgreementArgs {}

/// One disagreement between a spec's `review:` block and its `review.md`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ReviewAgreementFinding {
    /// Feature directory name the finding anchors to.
    pub feature: String,
    /// Which check produced it: `field-mismatch`, `blocking-mismatch`, or
    /// `orphan-waiver`. Kept distinct because the repairs differ.
    pub kind: String,
    /// The disagreeing field, keyed by the spec-side name (`last-run`,
    /// `reviewed-against`, `must-violations`, …). For `orphan-waiver` this is
    /// the waived rule id; empty for `blocking-mismatch`, which names no
    /// single field.
    #[serde(default)]
    pub field: String,
    /// The spec-side value, rendered. Empty when the key is absent.
    #[serde(default)]
    pub spec_value: String,
    /// The report-side value, rendered. Empty when the key is absent.
    #[serde(default)]
    pub report_value: String,
    /// Repo-relative path the finding is reported against.
    pub location: String,
    /// What is wrong, in the maintainer's terms.
    pub message: String,
    /// The repair, in the maintainer's terms.
    pub fix: String,
}

/// A spec the check could not compare, recorded so an empty `findings` is
/// never mistaken for a verified-clean corpus.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct ReviewAgreementSkip {
    /// Repo-relative path that went unexamined.
    pub path: String,
    /// Why, in the maintainer's terms.
    pub reason: String,
}

/// Result for `check-review-agreement`.
///
/// `findings` empty with `skipped` empty is **examined and clean**, but only
/// over the subject `examined` names: the intersection of specs carrying both
/// a `review:` block and a `review.md`, since those are the only ones that can
/// disagree. A spec with one record and not the other is a different defect
/// with a different owner (Family 19 and `check-review-gate`), so it is
/// counted in `single-sided` rather than silently dropped — two empty sets
/// compare equal, and a caller needs to tell a small subject from a clean one
/// (`QUAL-CLAIM-001`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CheckReviewAgreementResult {
    /// Disagreements, in feature then field order.
    #[serde(default)]
    pub findings: Vec<ReviewAgreementFinding>,
    /// Feature slugs compared — those carrying both records. Sorted.
    #[serde(default)]
    pub examined: Vec<String>,
    /// Feature slugs carrying exactly one of the two records, which this
    /// check cannot compare and deliberately does not own. Sorted.
    #[serde(default)]
    pub single_sided: Vec<String>,
    /// Specs whose frontmatter could not be parsed on either side.
    #[serde(default)]
    pub skipped: Vec<ReviewAgreementSkip>,
    /// Repo-relative spec root the run enumerated, so a caller can tell an
    /// genuinely review-free corpus from a wrong working directory.
    pub specs_root: String,
    /// Set when the run enumerated specs but found no spec carrying both
    /// records. Comparing nothing reports agreement, so without this an
    /// enumeration failure returns the payload of a clean run. Empty
    /// otherwise — the common case stays quiet, so its silence means
    /// "examined and current".
    #[serde(default)]
    pub guidance: String,
}

// -- check-unfolded-specs ------------------------------------------------------

/// Args for `check-unfolded-specs`. Takes none: the subject is the whole
/// spec corpus, which the primitive resolves through `[paths] specs-root`
/// exactly as every other corpus reader does.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct CheckUnfoldedSpecsArgs {}

/// One branch-scoped spec still present in the working tree.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct UnfoldedSpec {
    /// Directory basename (e.g., `1234.1-widget-cache`).
    pub feature: String,
    /// The branch identifier the directory is numbered under — the `1234`
    /// of `1234.1-widget-cache`. Reported separately so a caller can group
    /// a branch's specs without re-parsing the name.
    pub identifier: String,
    /// The upstream spec this one folds into, or `None` when the spec
    /// declares none.
    ///
    /// `None` is a real state rather than an error: `create-feature`
    /// refuses branch-scoped creation without a target, so a spec reaching
    /// here without one was hand-edited to stand on its own — the
    /// supported way to keep such a directory. A caller reports it, and
    /// does not infer a target for it.
    #[serde(default)]
    pub folds_into: Option<String>,
    /// The spec's frontmatter `status`, so a caller can say where in the
    /// pipeline the un-folded work sits.
    pub status: String,
}

/// Result for `check-unfolded-specs`.
///
/// Empty `unfolded` with a non-zero `examined` is **examined and clean**;
/// empty with `examined` zero means the corpus held no feature directories
/// at all, which is not the same claim (`QUAL-CLAIM-001`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CheckUnfoldedSpecsResult {
    /// Surviving branch-scoped specs, in the corpus order
    /// `list_feature_dirs` establishes.
    #[serde(default)]
    pub unfolded: Vec<UnfoldedSpec>,
    /// Feature directories scanned — **every** form, not just the
    /// branch-scoped ones. The count bounds the claim by subject: it is what
    /// separates "no branch-scoped specs survive" from "nothing was looked
    /// at", which an empty list alone cannot express.
    pub examined: u32,
}

// -- rewrite-spec-links --------------------------------------------------------

/// Args for `rewrite-spec-links`. Re-points every inbound pointer to a
/// retiring or renamed feature directory at its fold target — body links
/// across the corpus and the `folds-into` frontmatter field alike.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct RewriteSpecLinksArgs {
    /// The retiring or renamed feature directory name, e.g.
    /// `1234.1-widget-cache`. Matched as a whole path segment, so a
    /// directory whose name merely starts with this one is left alone.
    #[arg(long)]
    pub from: String,
    /// The fold target: a feature directory name, optionally with a
    /// scenario slug after a `/` — `050-alpha` or `050-alpha/eviction`.
    ///
    /// Without a scenario the rewrite is a rename and preserves each
    /// link's tail, so `../<from>/plan.md` becomes `../050-alpha/plan.md`.
    /// With one, the retiring directory's files did not survive
    /// individually — their content landed in that single scenario — so
    /// every inbound link collapses onto
    /// `<feature>/scenarios/<slug>.md`.
    #[arg(long)]
    pub to: String,
}

/// One file whose inbound pointers were re-pointed.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct RewrittenFile {
    /// Repo-relative path of the rewritten file.
    pub path: String,
    /// Pointers re-pointed within it — body links plus any `folds-into`
    /// field. A file appears here only when this is non-zero.
    pub count: u32,
}

/// Result for `rewrite-spec-links`.
///
/// An empty `rewritten` with a non-zero `examined` is **examined and
/// nothing pointed here**; empty with `examined` zero means no markdown was
/// scanned at all, which is not the same claim (`QUAL-CLAIM-001`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct RewriteSpecLinksResult {
    /// Files changed, in spec-root path order.
    #[serde(default)]
    pub rewritten: Vec<RewrittenFile>,
    /// Markdown files scanned under the spec root — the subject the empty
    /// case describes.
    pub examined: u32,
}

// -- retire-feature ------------------------------------------------------------

/// Args for `retire-feature`. Removes a branch-scoped feature directory once
/// its content has been folded into the upstream spec that receives it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct RetireFeatureArgs {
    /// The branch-scoped feature directory to remove, e.g.
    /// `1234.1-widget-cache`.
    ///
    /// Must parse as the branch-scoped form. A sequential `NNN-slug`
    /// feature is refused **unless** [`Self::allow_sequential`] is set: the
    /// sequential form is permanent, and a primitive that could delete one
    /// would make an irreversible operation reachable from a typo.
    #[arg(long)]
    pub feature: String,
    /// The upstream feature the content was folded into. It must exist,
    /// and that check is the whole point of the argument — it is what
    /// stops a retirement from stranding content nothing else holds.
    ///
    /// A fold routed into a scenario still names the **feature** here: the
    /// scenario lives inside it, so the feature's existence is what the
    /// check needs to establish.
    #[arg(long)]
    pub fold_target: String,
    /// Permit removing a **sequential** feature directory (spec 052).
    ///
    /// Off by default, which keeps the refusal exactly as it was for every
    /// existing caller. `/{project}:fold` never sets it, so a mistyped
    /// feature name during a fold still meets the refusal unchanged — which
    /// is the protection the refusal exists for, and the reason it is gated
    /// here rather than deleted.
    ///
    /// Consolidation sets it deliberately, having already named both specs
    /// and confirmed the removal with the operator. The flag is not a
    /// weaker refusal; it is the record that a second, explicit decision was
    /// made. The anti-stranding guard below is untouched and applies to both
    /// callers.
    #[arg(long, default_value_t = false)]
    #[serde(default)]
    pub allow_sequential: bool,
}

/// Result for `retire-feature`.
///
/// `retired: false` is the domain outcome for a directory that was already
/// gone — a re-run of an interrupted fold, not an error. Both refusals (a
/// sequential feature, an absent fold target) are operational errors
/// instead: each means the call should not have been made, and reporting
/// them as an outcome would let a walker continue past a destructive step
/// it had just declined to take.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct RetireFeatureResult {
    /// Whether this call removed the directory. `false` means it was
    /// already absent.
    pub retired: bool,
    /// Repo-relative path of the directory, present either way so a caller
    /// can name what it retired — or what it found already gone.
    pub path: String,
}

// -- invalidate-review ---------------------------------------------------------

/// Args for `invalidate-review`. Resets a spec's `review:` block to the
/// un-reviewed state, so the pre-`done` gate demands a fresh review.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, clap::Args)]
#[serde(rename_all = "kebab-case")]
pub struct InvalidateReviewArgs {
    /// The feature whose recorded review no longer describes it — the
    /// **upstream** spec on a fold, not the branch-scoped one being
    /// retired.
    #[arg(long)]
    pub feature: String,
}

/// Result for `invalidate-review`.
///
/// `invalidated: false` is the domain outcome for a spec that records no
/// current review: it is already in the state this primitive produces, so a
/// re-run converges rather than halting.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct InvalidateReviewResult {
    /// Whether this call reset a recorded review.
    pub invalidated: bool,
    /// Repo-relative path of the spec file, present either way.
    pub path: String,
    /// The `last-run` value that was cleared, so the caller can say what it
    /// invalidated rather than only that it did. Absent when nothing was.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_last_run: Option<String>,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::{
        AcceptanceCriterion, AnalyzeBlock, AnchorReference, CheckRuleIdsArgs, CheckRuleIdsResult,
        CheckStuckArgs, CheckStuckResult, CheckboxToggleResult, Classification, DependencyEdge,
        DeriveBoundaryArgs, DeriveBoundaryResult, Frontmatter, FrontmatterFinding, GateConfirmArgs,
        GateConfirmResult, LintMarkdownArgs, LintMarkdownResult, MarkCriterionArgs, MarkTaskArgs,
        MarkdownViolation, MigrateSessionFileArgs, MigrateSessionFileResult, OpenQuestion,
        PruneAction, PruneGate, PruneMode, PruneSection, PruneTasksArgs, PruneTasksResult,
        ReadSpecArgs, ReadSpecResult, ReadTasksArgs, ReadTasksResult, ResolveAnchorArgs,
        ResolveAnchorResult, ReviewBlock, RuleCitation, RunGeneratorArgs, RunGeneratorResult,
        ScenarioOpenQuestion, SetStatusArgs, SetStatusResult, SizeSummary, SpecSection, Subtask,
        Task, TraverseDepsArgs, TraverseDepsResult, ValidateFrontmatterArgs,
        ValidateFrontmatterResult, WriteSessionArgs, WriteSessionResult,
    };

    fn round_trip<T>(value: &T) -> T
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let text = serde_json::to_string(value).unwrap();
        serde_json::from_str(&text).unwrap()
    }

    #[test]
    fn read_spec_args_use_kebab_case() {
        let args = ReadSpecArgs {
            feature: "022-deterministic-runtime".into(),
            include_body: true,
        };
        let value: serde_json::Value = serde_json::to_value(&args).unwrap();
        assert_eq!(value["feature"], "022-deterministic-runtime");
        assert_eq!(value["include-body"], true);
        assert_eq!(round_trip(&args), args);
    }

    #[test]
    fn read_spec_result_round_trip() {
        let result = ReadSpecResult {
            frontmatter: Frontmatter {
                status: "clarified".into(),
                dependencies: vec!["021-runtime-boundary".into()],
                tags: vec![],
                folds_into: None,
                review: Some(ReviewBlock::default()),
                analyze: Some(AnalyzeBlock {
                    last_run: Some("2026-09-05T18:00:00Z".into()),
                    analyzed_against: Some("abc123".into()),
                    hard_fail: 0,
                    blocking_findings: 0,
                    advisory: 2,
                    unexamined: 1,
                    blocking: false,
                }),
            },
            sections: vec![SpecSection {
                heading: "Motivation".into(),
                level: 2,
                body: "…".into(),
            }],
            acceptance_criteria: vec![AcceptanceCriterion {
                checked: false,
                text: "A single binary builds…".into(),
                label: None,
            }],
            open_questions: vec![OpenQuestion { text: "?".into() }],
            scenario_open_questions: vec![ScenarioOpenQuestion {
                scenario: "framework-list-dedup".into(),
                text: "Format argument or separate primitive?".into(),
            }],
            scenario_files_unreadable: vec![],
            path: "specs/022-deterministic-runtime/spec.md".into(),
        };
        let value: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert!(value.get("acceptance-criteria").is_some());
        assert!(value.get("open-questions").is_some());
        // The scenario signal is a sibling key, never folded into
        // `open-questions` (spec 046).
        assert_eq!(
            value["scenario-open-questions"][0]["scenario"],
            "framework-list-dedup"
        );
        let fm = &value["frontmatter"];
        assert!(fm.get("dependencies").is_some());
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn read_tasks_round_trip() {
        let result = ReadTasksResult {
            tasks: vec![Task {
                number: "1".into(),
                heading: "Bootstrap".into(),
                subtasks: vec![Subtask {
                    text: "Create Cargo.toml".into(),
                    checked: true,
                }],
                done_when: Some("cargo build succeeds".into()),
                done_when_checked: None,
                phase: None,
            }],
            path: "specs/022-deterministic-runtime/tasks.md".into(),
        };
        let value: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(value["tasks"][0]["done-when"], "cargo build succeeds");
        // `phase: None` must not surface in the JSON — backward-compat for
        // existing consumers that pre-date the phased read-tasks fix.
        assert!(
            !value["tasks"][0].as_object().unwrap().contains_key("phase"),
            "phase: None should serialize as absent, not null"
        );
        assert_eq!(round_trip(&result), result);
        let args = ReadTasksArgs {
            feature: "022-deterministic-runtime".into(),
        };
        assert_eq!(round_trip(&args), args);
    }

    #[test]
    fn read_tasks_phased_task_carries_phase_metadata() {
        let result = ReadTasksResult {
            tasks: vec![Task {
                number: "1".into(),
                heading: "Wire up".into(),
                subtasks: vec![],
                done_when: None,
                done_when_checked: None,
                phase: Some("Phase A — Bootstrap".into()),
            }],
            path: "specs/022-deterministic-runtime/tasks.md".into(),
        };
        let value: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(value["tasks"][0]["phase"], "Phase A — Bootstrap");
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn mark_task_round_trip() {
        let args = MarkTaskArgs {
            feature: "022-deterministic-runtime".into(),
            task_number: "2".into(),
            subtask_index: 0,
            checked: true,
        };
        let value: serde_json::Value = serde_json::to_value(&args).unwrap();
        assert_eq!(value["task-number"], "2");
        assert_eq!(value["subtask-index"], 0);
        assert_eq!(round_trip(&args), args);

        let result = CheckboxToggleResult {
            previous: false,
            current: true,
            path: "specs/022-deterministic-runtime/tasks.md".into(),
        };
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn prune_tasks_round_trip() {
        let args = PruneTasksArgs {
            feature: "041-task-pruning".into(),
            reset: false,
            force: false,
            apply: true,
        };
        let value: serde_json::Value = serde_json::to_value(&args).unwrap();
        assert_eq!(value["feature"], "041-task-pruning");
        assert_eq!(value["reset"], false);
        assert_eq!(value["apply"], true);
        assert_eq!(round_trip(&args), args);

        let result = PruneTasksResult {
            mode: PruneMode::KeepPending,
            applied: false,
            gate: PruneGate::NotApplicable,
            status: None,
            nothing_to_prune: false,
            removed_count: 1,
            kept_count: 1,
            size_before: SizeSummary {
                lines: 40,
                bytes: 900,
            },
            size_after: SizeSummary {
                lines: 20,
                bytes: 450,
            },
            sections: vec![
                PruneSection {
                    number: "1".into(),
                    heading: "Schema types".into(),
                    phase: Some("Phase A".into()),
                    classification: Classification::Spent,
                    checkbox_total: 2,
                    checkbox_checked: 2,
                    action: PruneAction::Removed,
                },
                PruneSection {
                    number: "2".into(),
                    heading: "Segmentation".into(),
                    phase: Some("Phase A".into()),
                    classification: Classification::Pending,
                    checkbox_total: 3,
                    checkbox_checked: 1,
                    action: PruneAction::Kept,
                },
            ],
            path: "specs/041-task-pruning/tasks.md".into(),
        };
        let value: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(value["mode"], "keep-pending");
        assert_eq!(value["gate"], "not-applicable");
        assert_eq!(value["nothing-to-prune"], false);
        assert_eq!(value["removed-count"], 1);
        assert_eq!(value["size-before"]["lines"], 40);
        assert_eq!(value["sections"][0]["classification"], "spent");
        assert_eq!(value["sections"][0]["action"], "removed");
        assert_eq!(value["sections"][1]["classification"], "pending");
        // `status: None` must serialize as absent, not null.
        assert!(!value.as_object().unwrap().contains_key("status"));
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn mark_criterion_round_trip() {
        let args = MarkCriterionArgs {
            feature: "022-deterministic-runtime".into(),
            criterion_index: Some(3),
            label: None,
            checked: true,
        };
        let value: serde_json::Value = serde_json::to_value(&args).unwrap();
        assert_eq!(value["criterion-index"], 3);
        assert_eq!(round_trip(&args), args);
    }

    #[test]
    fn set_status_round_trip() {
        let args = SetStatusArgs {
            feature: "022-deterministic-runtime".into(),
            from: "clarified".into(),
            to: "planned".into(),
        };
        assert_eq!(round_trip(&args), args);
        let result = SetStatusResult {
            previous: "clarified".into(),
            current: "planned".into(),
            path: "specs/022-deterministic-runtime/spec.md".into(),
        };
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn derive_boundary_round_trip() {
        let args = DeriveBoundaryArgs {
            feature: "022-deterministic-runtime".into(),
        };
        assert_eq!(round_trip(&args), args);
        let result = DeriveBoundaryResult {
            boundary: vec![
                "specs/022-deterministic-runtime/**".into(),
                "runtime/**".into(),
            ],
            first_commit: "d398083".into(),
            current_head: "6f0f54e".into(),
            guidance: None,
        };
        let value: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(value["first-commit"], "d398083");
        assert_eq!(value["current-head"], "6f0f54e");
        // Absent on an ordinary derivation, so no existing consumer or
        // golden sees a new key (scenario derive-boundary-uncommitted-spec-dir).
        assert!(value.get("guidance").is_none());
        assert_eq!(round_trip(&result), result);

        let uncommitted = DeriveBoundaryResult {
            boundary: vec!["specs/022-deterministic-runtime/**".into()],
            first_commit: String::new(),
            current_head: "6f0f54e".into(),
            guidance: Some("commit the spec directory".into()),
        };
        let value: serde_json::Value = serde_json::to_value(&uncommitted).unwrap();
        assert_eq!(value["guidance"], "commit the spec directory");
        assert_eq!(round_trip(&uncommitted), uncommitted);
    }

    #[test]
    fn check_stuck_round_trip() {
        let args = CheckStuckArgs {
            feature: "022-deterministic-runtime".into(),
            threshold: 10,
        };
        assert_eq!(round_trip(&args), args);
        let result = CheckStuckResult {
            commit_count: 3,
            stuck: false,
            since_sha: "abcdef0".into(),
            threshold: 10,
        };
        let value: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(value["commit-count"], 3);
        assert_eq!(value["since-sha"], "abcdef0");
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn validate_frontmatter_round_trip() {
        let args = ValidateFrontmatterArgs {
            path: "specs/022-deterministic-runtime/spec.md".into(),
        };
        assert_eq!(round_trip(&args), args);
        let result = ValidateFrontmatterResult {
            findings: vec![FrontmatterFinding {
                severity: "blocking".into(),
                field: "status".into(),
                message: "unknown status".into(),
            }],
            clean: false,
        };
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn resolve_anchor_round_trip() {
        let args = ResolveAnchorArgs {
            path: "framework/constitution.md".into(),
            markers_path: None,
        };
        assert_eq!(round_trip(&args), args);
        let result = ResolveAnchorResult {
            references: vec![AnchorReference {
                anchor: "runtime-boundary".into(),
                line: 459,
                resolved: true,
            }],
            unresolved: vec![],
        };
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn traverse_deps_round_trip() {
        let args = TraverseDepsArgs {
            feature: "022-deterministic-runtime".into(),
        };
        assert_eq!(round_trip(&args), args);
        let result = TraverseDepsResult {
            dependencies: vec![DependencyEdge {
                feature: "021-runtime-boundary".into(),
                exists: true,
                status: "done".into(),
                compatible: true,
            }],
            compatible: true,
            cycles: Vec::new(),
        };
        assert_eq!(round_trip(&result), result);
        // Cycle-bearing payload round-trips with the new field populated.
        let with_cycles = TraverseDepsResult {
            dependencies: vec![DependencyEdge {
                feature: "100-a".into(),
                exists: true,
                status: "planned".into(),
                compatible: true,
            }],
            compatible: false,
            cycles: vec![vec!["100-a".into(), "101-b".into()]],
        };
        assert_eq!(round_trip(&with_cycles), with_cycles);
    }

    #[test]
    fn check_rule_ids_round_trip() {
        let args = CheckRuleIdsArgs {
            path: "specs/022-deterministic-runtime/spec.md".into(),
            rule_files: vec!["framework/rules/security-backend.md".into()],
        };
        let value: serde_json::Value = serde_json::to_value(&args).unwrap();
        assert_eq!(
            value["rule-files"][0],
            "framework/rules/security-backend.md"
        );
        assert_eq!(round_trip(&args), args);
        let result = CheckRuleIdsResult {
            citations: vec![RuleCitation {
                rule_id: "SEC-AUTH-001".into(),
                found: true,
                deprecated: false,
            }],
            missing: vec![],
            deprecated: vec![],
        };
        let value: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(value["citations"][0]["rule-id"], "SEC-AUTH-001");
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn run_generator_round_trip() {
        let args = RunGeneratorArgs {
            script: "scripts/gen-spec-deps.sh".into(),
        };
        assert_eq!(round_trip(&args), args);
        let result = RunGeneratorResult {
            drift: false,
            stdout: "ok\n".into(),
            stderr: String::new(),
            exit_code: 0,
        };
        let value: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(value["exit-code"], 0);
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn lint_markdown_round_trip() {
        let args = LintMarkdownArgs {
            paths: vec!["framework/constitution.md".into()],
            fix: false,
        };
        assert_eq!(round_trip(&args), args);
        let result = LintMarkdownResult {
            violations: vec![MarkdownViolation {
                path: "README.md".into(),
                line: 17,
                rule: "MD013".into(),
                message: "Line length".into(),
            }],
            clean: false,
            exit_code: 1,
        };
        let value: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(value["exit-code"], 1);
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn gate_confirm_round_trip() {
        let args = GateConfirmArgs {
            gate: "plan-finalize-status".into(),
            prompt: "Advance status from clarified to planned?".into(),
        };
        assert_eq!(round_trip(&args), args);
        let result = GateConfirmResult { confirmed: true };
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn extract_archive_round_trip() {
        use super::{ExtractArchiveArgs, ExtractArchiveResult};
        let args = ExtractArchiveArgs {
            archive: "/tmp/ductus.tar.gz".into(),
            dest: "/tmp/out".into(),
            format: Some("tar-gz".into()),
        };
        let value: serde_json::Value = serde_json::to_value(&args).unwrap();
        assert_eq!(value["archive"], "/tmp/ductus.tar.gz");
        assert_eq!(value["dest"], "/tmp/out");
        assert_eq!(value["format"], "tar-gz");
        assert_eq!(round_trip(&args), args);

        let result = ExtractArchiveResult {
            dest: "/tmp/out".into(),
            files: vec!["a.txt".into(), "dir/b.txt".into()],
            count: 2,
            format: "tar-gz".into(),
        };
        let r_value: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(r_value["count"], 2);
        assert_eq!(r_value["files"][1], "dir/b.txt");
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn merge_managed_block_round_trip() {
        use super::{MergeManagedBlockArgs, MergeManagedBlockResult};
        let args = MergeManagedBlockArgs {
            path: ".gitignore".into(),
            block: ".claude/\nspecs/.cache/".into(),
            marker: Some("ductus-managed".into()),
            marker_style: Some("line-prefix".into()),
        };
        let value: serde_json::Value = serde_json::to_value(&args).unwrap();
        assert_eq!(value["marker-style"], "line-prefix");
        assert_eq!(value["marker"], "ductus-managed");
        assert_eq!(round_trip(&args), args);

        // marker_style omitted serializes without the field.
        let args_default_style = MergeManagedBlockArgs {
            path: "CLAUDE.md".into(),
            block: "x".into(),
            marker: None,
            marker_style: None,
        };
        let v: serde_json::Value = serde_json::to_value(&args_default_style).unwrap();
        assert!(!v.as_object().unwrap().contains_key("marker-style"));
        assert!(!v.as_object().unwrap().contains_key("marker"));

        let result = MergeManagedBlockResult {
            path: ".gitignore".into(),
            action: "inserted".into(),
            marker: "ductus-managed".into(),
            marker_style: "line-prefix".into(),
            dedup_removed: Some(2),
            dedup_removed_lines: Some(vec![".claude/".into(), "*.sqlite".into()]),
        };
        let r_value: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(r_value["marker-style"], "line-prefix");
        assert_eq!(r_value["dedup-removed"], 2);
        assert_eq!(round_trip(&result), result);

        // html-comment shape: dedup fields are absent from JSON when None.
        let html_result = MergeManagedBlockResult {
            path: "CLAUDE.md".into(),
            action: "updated".into(),
            marker: "ductus-managed".into(),
            marker_style: "html-comment".into(),
            dedup_removed: None,
            dedup_removed_lines: None,
        };
        let v: serde_json::Value = serde_json::to_value(&html_result).unwrap();
        assert!(!v.as_object().unwrap().contains_key("dedup-removed"));
        assert!(!v.as_object().unwrap().contains_key("dedup-removed-lines"));
    }

    #[test]
    fn merge_permissions_round_trip() {
        use super::{MergePermissionsArgs, MergePermissionsResult};
        let args = MergePermissionsArgs {
            path: ".claude/settings.local.json".into(),
            allow: vec!["Bash(ls *)".into(), "Edit".into()],
            deny: vec!["Bash(rm -rf *)".into()],
            revoke: vec!["Write(.ductus/session.toml)".into()],
        };
        let value: serde_json::Value = serde_json::to_value(&args).unwrap();
        assert_eq!(value["path"], ".claude/settings.local.json");
        assert_eq!(value["allow"][0], "Bash(ls *)");
        assert_eq!(value["revoke"][0], "Write(.ductus/session.toml)");
        assert_eq!(round_trip(&args), args);

        // A non-Claude host supplies its own settings path; the runtime
        // does not hardcode `.claude/`.
        let auggie_args = MergePermissionsArgs {
            path: ".augment/settings.json".into(),
            allow: vec![],
            deny: vec![],
            revoke: vec![],
        };
        let v: serde_json::Value = serde_json::to_value(&auggie_args).unwrap();
        assert_eq!(v["path"], ".augment/settings.json");

        let result = MergePermissionsResult {
            path: ".claude/settings.local.json".into(),
            action: "updated".into(),
            allow_added: 2,
            allow_deduped: 1,
            allow_revoked: 3,
            deny_added: 0,
            deny_deduped: 0,
        };
        let r_value: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(r_value["allow-added"], 2);
        assert_eq!(r_value["allow-deduped"], 1);
        // Kebab-case on the wire, matching every sibling count.
        assert_eq!(r_value["allow-revoked"], 3);
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn enforce_manifest_round_trip() {
        use super::{EnforceManifestArgs, EnforceManifestResult};
        let args = EnforceManifestArgs {
            expected_json: None,
            pinned_json: None,
            directory: ".claude/commands/anvil".into(),
            expected: vec!["status.md".into(), "target.md".into()],
            pinned: vec!["adopter-custom.md".into()],
            recursive: false,
            glob_include: Some("*.md".into()),
        };
        let value: serde_json::Value = serde_json::to_value(&args).unwrap();
        assert_eq!(value["directory"], ".claude/commands/anvil");
        assert_eq!(value["expected"][0], "status.md");
        assert_eq!(value["glob-include"], "*.md");
        assert_eq!(round_trip(&args), args);

        // glob_include omitted serializes without the field.
        let args_default_glob = EnforceManifestArgs {
            expected_json: None,
            pinned_json: None,
            directory: "x".into(),
            expected: vec![],
            pinned: vec![],
            recursive: true,
            glob_include: None,
        };
        let v: serde_json::Value = serde_json::to_value(&args_default_glob).unwrap();
        assert!(!v.as_object().unwrap().contains_key("glob-include"));
        assert_eq!(v["recursive"], true);

        let result = EnforceManifestResult {
            removed: vec!["legacy.md".into()],
            kept: vec!["status.md".into()],
            pinned_kept: vec!["adopter-custom.md".into()],
        };
        let r_value: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(r_value["pinned-kept"][0], "adopter-custom.md");
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn apply_manifest_round_trip() {
        use super::{ApplyManifestArgs, ApplyManifestResult, ManifestEntry, ManifestEntryResult};
        use std::collections::BTreeMap;
        let mut subs = BTreeMap::new();
        subs.insert("project".into(), "anvil".into());
        let args = ApplyManifestArgs {
            entries_json: None,
            pinned_json: None,
            substitutions_json: None,
            source_root: "/tmp/staging".into(),
            target_root: "/tmp/project".into(),
            entries: vec![
                ManifestEntry {
                    source: "framework/commands/status.md".into(),
                    dest: "framework/commands/status.md".into(),
                    strategy: "update".into(),
                    keep_literals: None,
                },
                ManifestEntry {
                    source: "ductus.md".into(),
                    dest: ".claude/commands/anvil/ductus.md".into(),
                    strategy: "update".into(),
                    keep_literals: Some(vec!["project".into(), "cli-config-dir".into()]),
                },
            ],
            pinned: vec!["AGENTS.md".into()],
            substitutions: subs,
        };
        let value: serde_json::Value = serde_json::to_value(&args).unwrap();
        assert_eq!(value["source-root"], "/tmp/staging");
        assert_eq!(value["target-root"], "/tmp/project");
        assert_eq!(value["entries"][0]["strategy"], "update");
        assert_eq!(value["entries"][1]["keep-literals"][0], "project");
        // keep-literals omitted on the first entry should not serialize.
        assert!(
            value["entries"][0]
                .as_object()
                .unwrap()
                .get("keep-literals")
                .is_none()
        );
        assert_eq!(round_trip(&args), args);

        let result = ApplyManifestResult {
            entries: vec![
                ManifestEntryResult {
                    source: "framework/commands/status.md".into(),
                    dest: "framework/commands/status.md".into(),
                    action: "created".into(),
                    substitutions_applied: Some(3),
                },
                ManifestEntryResult {
                    source: "framework/templates/spec/spec.md".into(),
                    dest: "specs/templates/spec.md".into(),
                    action: "skipped-pinned".into(),
                    substitutions_applied: None,
                },
            ],
            created: 1,
            updated: 0,
            unchanged: 0,
            skipped_exists: 0,
            skipped_pinned: 1,
            source_missing: 0,
            substitutions_applied: 3,
            entries_substituted: 1,
        };
        let r_value: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(r_value["skipped-pinned"], 1);
        assert_eq!(r_value["source-missing"], 0);
        assert_eq!(r_value["entries"][0]["action"], "created");
        assert_eq!(r_value["substitutions-applied"], 3);
        assert_eq!(r_value["entries-substituted"], 1);
        assert_eq!(r_value["entries"][0]["substitutions-applied"], 3);
        // A `None` count must be absent rather than serialized as 0 — the
        // whole point of the field is that the two do not read alike.
        assert!(
            r_value["entries"][1]
                .as_object()
                .unwrap()
                .get("substitutions-applied")
                .is_none()
        );
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn fetch_archive_round_trip() {
        use super::{FetchArchiveArgs, FetchArchiveResult};
        let args = FetchArchiveArgs {
            url: "https://example.test/ductus-0.2.0.tar.gz".into(),
            sha256_url: Some("https://example.test/ductus-0.2.0.tar.gz.sha256".into()),
            archive: "/tmp/ductus.tar.gz".into(),
        };
        let value: serde_json::Value = serde_json::to_value(&args).unwrap();
        assert_eq!(
            value["sha256-url"],
            "https://example.test/ductus-0.2.0.tar.gz.sha256"
        );
        assert_eq!(value["archive"], "/tmp/ductus.tar.gz");
        assert_eq!(round_trip(&args), args);

        // Absent sha256_url omits the field entirely.
        let args_no_sidecar = FetchArchiveArgs {
            url: "https://example.test/main.tar.gz".into(),
            sha256_url: None,
            archive: "/tmp/main.tar.gz".into(),
        };
        let v: serde_json::Value = serde_json::to_value(&args_no_sidecar).unwrap();
        assert!(!v.as_object().unwrap().contains_key("sha256-url"));
        assert_eq!(round_trip(&args_no_sidecar), args_no_sidecar);

        let result = FetchArchiveResult {
            path: "/tmp/ductus.tar.gz".into(),
            sha256: "abc123".into(),
            verified: true,
            bytes: 12345,
        };
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn migrate_session_file_round_trip() {
        let args = MigrateSessionFileArgs {
            legacy_path: ".claude/gov-session.json".into(),
        };
        let value: serde_json::Value = serde_json::to_value(&args).unwrap();
        assert_eq!(value["legacy-path"], ".claude/gov-session.json");
        assert_eq!(round_trip(&args), args);

        // Adopter on a non-Claude host or non-`gov` project name:
        let auggie_args = MigrateSessionFileArgs {
            legacy_path: ".augment/anvil-session.json".into(),
        };
        let v: serde_json::Value = serde_json::to_value(&auggie_args).unwrap();
        assert_eq!(v["legacy-path"], ".augment/anvil-session.json");

        let result = MigrateSessionFileResult {
            source: ".claude/gov-session.json".into(),
            dest: ".ductus/session.toml".into(),
            action: "migrated".into(),
            legacy_deleted: true,
        };
        let r_value: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(r_value["source"], ".claude/gov-session.json");
        assert_eq!(r_value["dest"], ".ductus/session.toml");
        assert_eq!(r_value["action"], "migrated");
        assert_eq!(r_value["legacy-deleted"], true);
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn write_session_round_trip() {
        let args = WriteSessionArgs {
            feature: Some("022-deterministic-runtime".into()),
            path: Some("specs/022-deterministic-runtime".into()),
            scenario: Some("write-session-primitive".into()),
            scenario_path: Some(
                "specs/022-deterministic-runtime/scenarios/write-session-primitive.md".into(),
            ),
            cli_config_dir: None,
            clear: false,
        };
        let value: serde_json::Value = serde_json::to_value(&args).unwrap();
        // CLI/MCP args remain kebab-case to match every other primitive.
        assert_eq!(value["feature"], "022-deterministic-runtime");
        assert_eq!(value["path"], "specs/022-deterministic-runtime");
        assert_eq!(value["scenario"], "write-session-primitive");
        assert_eq!(
            value["scenario-path"],
            "specs/022-deterministic-runtime/scenarios/write-session-primitive.md"
        );
        assert_eq!(round_trip(&args), args);

        // Host-config write: only `cli-config-dir` set; the target fields
        // are absent.
        let args_host = WriteSessionArgs {
            feature: None,
            path: None,
            scenario: None,
            scenario_path: None,
            cli_config_dir: Some(".opencode".into()),
            clear: false,
        };
        let vh: serde_json::Value = serde_json::to_value(&args_host).unwrap();
        let objh = vh.as_object().unwrap();
        assert!(!objh.contains_key("feature"));
        assert_eq!(vh["cli-config-dir"], ".opencode");
        assert_eq!(round_trip(&args_host), args_host);

        // Absent scenario + scenario-path omit both fields.
        let args_no_scenario = WriteSessionArgs {
            feature: Some("002-target".into()),
            path: Some("specs/002-target".into()),
            scenario: None,
            scenario_path: None,
            cli_config_dir: None,
            clear: false,
        };
        let v: serde_json::Value = serde_json::to_value(&args_no_scenario).unwrap();
        let obj = v.as_object().unwrap();
        assert!(!obj.contains_key("scenario"));
        assert!(!obj.contains_key("scenario-path"));
        assert_eq!(round_trip(&args_no_scenario), args_no_scenario);

        // Clear write: only `clear` set; `clear` serializes as a plain
        // boolean and an absent `clear` key deserializes to `false`
        // (backward compatibility for pre-clear callers).
        let args_clear = WriteSessionArgs {
            feature: None,
            path: None,
            scenario: None,
            scenario_path: None,
            cli_config_dir: None,
            clear: true,
        };
        let vc: serde_json::Value = serde_json::to_value(&args_clear).unwrap();
        assert_eq!(vc["clear"], true);
        assert_eq!(round_trip(&args_clear), args_clear);
        let legacy: WriteSessionArgs = serde_json::from_str("{}").unwrap();
        assert!(!legacy.clear, "absent `clear` defaults to false");

        let result = WriteSessionResult {
            path: ".ductus/session.toml".into(),
            created: true,
        };
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn resolve_feature_round_trip() {
        use super::{
            ResolveFeatureArgs, ResolveFeatureOutcome, ResolveFeatureResult, ResolvedScenario,
        };
        let args = ResolveFeatureArgs {
            identifier: "22".into(),
            scenario: Some("scaffolding-primitives".into()),
        };
        let value: serde_json::Value = serde_json::to_value(&args).unwrap();
        assert_eq!(value["identifier"], "22");
        assert_eq!(value["scenario"], "scaffolding-primitives");
        assert_eq!(round_trip(&args), args);

        // Absent scenario omits the field.
        let bare = ResolveFeatureArgs {
            identifier: "runtime".into(),
            scenario: None,
        };
        let v: serde_json::Value = serde_json::to_value(&bare).unwrap();
        assert!(!v.as_object().unwrap().contains_key("scenario"));

        let resolved = ResolveFeatureResult {
            outcome: ResolveFeatureOutcome::Resolved,
            feature: Some("022-deterministic-runtime".into()),
            path: Some("specs/022-deterministic-runtime".into()),
            status: Some("in-progress".into()),
            candidates: vec![],
            scenario: Some(ResolvedScenario {
                slug: "scaffolding-primitives".into(),
                path: "specs/022-deterministic-runtime/scenarios/scaffolding-primitives.md".into(),
                exists: true,
                section: "Follow-on scenarios".into(),
            }),
        };
        let rv: serde_json::Value = serde_json::to_value(&resolved).unwrap();
        assert_eq!(rv["outcome"], "resolved");
        assert_eq!(rv["scenario"]["exists"], true);
        assert_eq!(rv["scenario"]["section"], "Follow-on scenarios");
        assert_eq!(round_trip(&resolved), resolved);

        // Ambiguous carries the sorted candidate list; the resolved-only
        // fields serialize as absent, not null.
        let ambiguous = ResolveFeatureResult {
            outcome: ResolveFeatureOutcome::Ambiguous,
            feature: None,
            path: None,
            status: None,
            candidates: vec!["001-a-runtime".into(), "002-b-runtime".into()],
            scenario: None,
        };
        let av: serde_json::Value = serde_json::to_value(&ambiguous).unwrap();
        assert_eq!(av["outcome"], "ambiguous");
        assert_eq!(av["candidates"][0], "001-a-runtime");
        let obj = av.as_object().unwrap();
        assert!(!obj.contains_key("feature"));
        assert!(!obj.contains_key("status"));
        assert_eq!(round_trip(&ambiguous), ambiguous);

        let not_found = ResolveFeatureResult {
            outcome: ResolveFeatureOutcome::NotFound,
            feature: None,
            path: None,
            status: None,
            candidates: vec![],
            scenario: None,
        };
        let nv: serde_json::Value = serde_json::to_value(&not_found).unwrap();
        assert_eq!(nv["outcome"], "not-found");
        assert_eq!(round_trip(&not_found), not_found);
    }

    #[test]
    fn create_feature_round_trip() {
        use super::{CreateFeatureArgs, CreateFeatureResult};
        let args = CreateFeatureArgs {
            title: "Deterministic Runtime!".into(),
            branch_id: None,
            fold_into: None,
        };
        let value: serde_json::Value = serde_json::to_value(&args).unwrap();
        assert_eq!(value["title"], "Deterministic Runtime!");
        // The branch-scoped arguments are absent from a sequential
        // request, not null: an adopter's stored payload predates them.
        assert!(!value.as_object().unwrap().contains_key("branch-id"));
        assert!(!value.as_object().unwrap().contains_key("fold-into"));
        assert_eq!(round_trip(&args), args);

        let branch_args = CreateFeatureArgs {
            title: "Staged Change".into(),
            branch_id: Some("proj-1111".into()),
            fold_into: Some("022-deterministic-runtime".into()),
        };
        let bv: serde_json::Value = serde_json::to_value(&branch_args).unwrap();
        assert_eq!(bv["branch-id"], "proj-1111");
        assert_eq!(bv["fold-into"], "022-deterministic-runtime");
        assert_eq!(round_trip(&branch_args), branch_args);

        let result = CreateFeatureResult {
            created: true,
            feature: "043-deterministic-runtime".into(),
            path: "specs/043-deterministic-runtime".into(),
            template: Some("framework/templates/spec/spec.md".into()),
            identifier: None,
        };
        let rv: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(rv["created"], true);
        assert_eq!(rv["template"], "framework/templates/spec/spec.md");
        assert_eq!(round_trip(&result), result);

        // Refusal outcome: template absent from the JSON, not null.
        let refused = CreateFeatureResult {
            created: false,
            feature: "043-deterministic-runtime".into(),
            path: "specs/043-deterministic-runtime".into(),
            template: None,
            identifier: None,
        };
        let fv: serde_json::Value = serde_json::to_value(&refused).unwrap();
        assert!(!fv.as_object().unwrap().contains_key("template"));
        assert_eq!(round_trip(&refused), refused);
    }

    #[test]
    fn create_plan_artifacts_round_trip() {
        use super::{
            CreatePlanArtifactsArgs, CreatePlanArtifactsResult, PlanArtifact, PlanArtifactAction,
        };
        let args = CreatePlanArtifactsArgs {
            feature: "042-widget".into(),
            include_data_model: true,
            overwrite: false,
        };
        let value: serde_json::Value = serde_json::to_value(&args).unwrap();
        assert_eq!(value["feature"], "042-widget");
        assert_eq!(value["include-data-model"], true);
        assert_eq!(value["overwrite"], false);
        assert_eq!(round_trip(&args), args);

        // Booleans default false when omitted (host sends only `feature`).
        let minimal: CreatePlanArtifactsArgs =
            serde_json::from_value(serde_json::json!({"feature": "042-widget"})).unwrap();
        assert!(!minimal.include_data_model);
        assert!(!minimal.overwrite);

        let result = CreatePlanArtifactsResult {
            path: "specs/042-widget".into(),
            artifacts: vec![
                PlanArtifact {
                    file: "plan.md".into(),
                    path: "specs/042-widget/plan.md".into(),
                    action: PlanArtifactAction::Created,
                    template: Some("specs/templates/plan.md".into()),
                },
                PlanArtifact {
                    file: "tasks.md".into(),
                    path: "specs/042-widget/tasks.md".into(),
                    action: PlanArtifactAction::Kept,
                    template: None,
                },
            ],
        };
        let rv: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(rv["artifacts"][0]["action"], "created");
        assert_eq!(rv["artifacts"][1]["action"], "kept");
        // `template` is absent from the JSON on kept, not null.
        let kept = rv["artifacts"][1].as_object().unwrap();
        assert!(!kept.contains_key("template"));
        assert_eq!(round_trip(&result), result);

        let replaced: serde_json::Value =
            serde_json::to_value(PlanArtifactAction::Replaced).unwrap();
        assert_eq!(replaced, "replaced");
    }

    #[test]
    fn check_review_gate_round_trip() {
        use super::{CheckReviewGateArgs, CheckReviewGateResult, ReviewGateBlock};
        let args = CheckReviewGateArgs {
            feature: "042-widget".into(),
        };
        assert_eq!(round_trip(&args), args);

        let passed = CheckReviewGateResult {
            passed: true,
            blocked_by: None,
            message: None,
            guidance: None,
            violations: vec![],
        };
        let pv: serde_json::Value = serde_json::to_value(&passed).unwrap();
        // Options are absent from the JSON on pass, not null.
        let obj = pv.as_object().unwrap();
        assert!(!obj.contains_key("blocked-by"));
        assert!(!obj.contains_key("message"));
        assert!(!obj.contains_key("guidance"));
        assert_eq!(round_trip(&passed), passed);

        let blocked = CheckReviewGateResult {
            passed: false,
            blocked_by: Some(ReviewGateBlock::MustViolations),
            message: Some(
                "blocked: spec has 3 MUST violation(s) — see specs/042-widget/review.md".into(),
            ),
            guidance: Some("Resolve the violations and re-run /ductus:review".into()),
            violations: vec![],
        };
        let bv: serde_json::Value = serde_json::to_value(&blocked).unwrap();
        assert_eq!(bv["blocked-by"], "must-violations");
        assert_eq!(round_trip(&blocked), blocked);

        let lint: serde_json::Value = serde_json::to_value(ReviewGateBlock::MarkdownLint).unwrap();
        assert_eq!(lint, "markdown-lint");
        let unreviewed: serde_json::Value =
            serde_json::to_value(ReviewGateBlock::NotReviewed).unwrap();
        assert_eq!(unreviewed, "not-reviewed");
    }

    #[test]
    fn append_question_round_trip() {
        use super::{AppendQuestionArgs, AppendQuestionResult};
        let args = AppendQuestionArgs {
            feature: "042-widget".into(),
            question: "Should rate limits be configurable per tenant?".into(),
            scenario: None,
        };
        let av: serde_json::Value = serde_json::to_value(&args).unwrap();
        // Absent scenario is omitted from the JSON, not null.
        assert!(!av.as_object().unwrap().contains_key("scenario"));
        assert_eq!(round_trip(&args), args);

        let scenario_target = AppendQuestionArgs {
            scenario: Some("retry-on-timeout".into()),
            ..args
        };
        let sv: serde_json::Value = serde_json::to_value(&scenario_target).unwrap();
        assert_eq!(sv["scenario"], "retry-on-timeout");
        assert_eq!(round_trip(&scenario_target), scenario_target);

        let appended = AppendQuestionResult {
            path: "specs/042-widget/spec.md".into(),
            appended: true,
            duplicate_of: None,
            section_created: false,
            status_reverted: true,
            previous_status: Some("planned".into()),
        };
        let rv: serde_json::Value = serde_json::to_value(&appended).unwrap();
        assert_eq!(rv["status-reverted"], true);
        assert_eq!(rv["previous-status"], "planned");
        assert!(!rv.as_object().unwrap().contains_key("duplicate-of"));
        assert_eq!(round_trip(&appended), appended);

        let deduped = AppendQuestionResult {
            path: "specs/042-widget/spec.md".into(),
            appended: false,
            duplicate_of: Some("Should rate limits be configurable per tenant?".into()),
            section_created: false,
            status_reverted: false,
            previous_status: None,
        };
        let dv: serde_json::Value = serde_json::to_value(&deduped).unwrap();
        assert_eq!(
            dv["duplicate-of"],
            "Should rate limits be configurable per tenant?"
        );
        assert!(!dv.as_object().unwrap().contains_key("previous-status"));
        assert_eq!(round_trip(&deduped), deduped);
    }

    #[test]
    fn diff_cross_spec_round_trip() {
        use super::{DiffCrossSpecArgs, DiffCrossSpecResult};
        let args = DiffCrossSpecArgs {
            feature: "042-widget".into(),
        };
        assert_eq!(round_trip(&args), args);

        let result = DiffCrossSpecResult {
            first_commit: "abc123".into(),
            current_head: "def456".into(),
            cross_spec_paths: vec!["specs/007-sibling/spec.md".into()],
            inbox_additions: vec!["- security: token logged in plaintext".into()],
            guidance: None,
        };
        let rv: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(rv["cross-spec-paths"][0], "specs/007-sibling/spec.md");
        // Absent on an ordinary window, so no existing consumer sees a new key.
        assert!(rv.get("guidance").is_none());
        assert_eq!(
            rv["inbox-additions"][0],
            "- security: token logged in plaintext"
        );
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn append_inbox_round_trip() {
        use super::{AppendInboxArgs, AppendInboxResult};
        let args = AppendInboxArgs {
            text: "security: token logged in plaintext — src/auth.rs (captured during 022)".into(),
            dedup_prefix: Some("security: token logged".into()),
        };
        let value: serde_json::Value = serde_json::to_value(&args).unwrap();
        assert_eq!(value["dedup-prefix"], "security: token logged");
        assert_eq!(round_trip(&args), args);

        // Absent dedup-prefix omits the field.
        let bare = AppendInboxArgs {
            text: "x".into(),
            dedup_prefix: None,
        };
        let v: serde_json::Value = serde_json::to_value(&bare).unwrap();
        assert!(!v.as_object().unwrap().contains_key("dedup-prefix"));

        let result = AppendInboxResult {
            path: "specs/inbox.md".into(),
            created: false,
            deduped: true,
            item_count: 3,
        };
        let rv: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(rv["deduped"], true);
        assert_eq!(rv["item-count"], 3);
        assert_eq!(round_trip(&result), result);
    }

    #[test]
    fn check_artifacts_round_trip() {
        use super::{ArtifactFinding, CheckArtifactsArgs, CheckArtifactsResult, SkippedTarget};
        let args = CheckArtifactsArgs {
            feature: "022-deterministic-runtime".into(),
        };
        assert_eq!(round_trip(&args), args);

        let result = CheckArtifactsResult {
            feature: "022-deterministic-runtime".into(),
            status: "planned".into(),
            findings: vec![ArtifactFinding {
                family: "artifact-completeness".into(),
                severity: "blocking".into(),
                message: "plan.md is required at status 'planned' but does not exist".into(),
                path: "specs/022-deterministic-runtime/plan.md".into(),
            }],
            clean: false,
            skipped: vec![SkippedTarget {
                family: "link-adjacent-drift".into(),
                reason: "target-missing".into(),
                path: "specs/022-deterministic-runtime/scenarios/renamed.md".into(),
            }],
            path: "specs/022-deterministic-runtime/spec.md".into(),
        };
        let rv: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(rv["findings"][0]["family"], "artifact-completeness");
        assert_eq!(rv["findings"][0]["severity"], "blocking");
        assert_eq!(rv["clean"], false);
        assert_eq!(round_trip(&result), result);
    }
}
