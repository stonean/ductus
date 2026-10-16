# 060 — Exec analyze assesses each loaded rule Plan

Implements [060 — Exec analyze assesses each loaded rule](spec.md).

## Overview

The exec walker loads the project's rules once per `/{project}:analyze` walk, through one shared rule-section parser that reads each rule's ID, Statement and Verification and derives its tier from the Statement's RFC 2119 keyword. Each `assessSpecQuality` step then sends one request per loaded rule of the step's tier, so every rule is asked about exactly once, in its own tier. Rules the walk cannot ask about are recorded in `unexamined-by-reason`: a rule file it could not read under a new `rule-file-unreadable` reason, and a rule with no Verification or no keyword under `rule-assessments-not-checked`, which now counts rules rather than requests.

`framework/commands/analyze.md` and 022's data model are updated to describe the per-rule walk and the host's trigger judgment. 022 is reopened to take those edits and a signpost back to this spec, which discharges the `cross-spec-impact:` entry, then re-reviewed back to `done`. The runtime change ships in a patch release, `0.54.2`.

No `data-model.md` is created here. The only vocabulary this spec adds, one unexamined reason, belongs in the reason table 066's data model owns (`specs/066-audit-records-and-the-completion-gate/data-model.md`, `write-analysis`), and the walker's rule type is internal. There is no `specs/system.md`, `specs/events.md` or `specs/errors.md` in this repository to check the plan against.

## Technical Decisions

### One rule-section parser, sharing check-rule-ids' heading grammar

Two parsers read rule files today, and they disagree on what a rule is. `check-rule-ids` recognizes a rule by `heading_id_regex`, `^#{2,4}\s+([A-Z]{2,5}-[A-Z][A-Z0-9]+-\d{3,4})\b` (`runtime/src/primitives/check_rule_ids.rs:92`). The walker's `extract_rule_verification` and `first_rule_with_verification` treat any level-3 heading's text as a rule ID (`runtime/src/interpreter/payload.rs:944`, `:984`). Enumerating every rule would add a third reading, so both walker functions are replaced by one module, and it takes the heading grammar from `check-rule-ids` rather than restating it. The set of rules the walk asks about is then the set of IDs `check-rule-ids` knows.

`runtime/src/primitives/rule_sections.rs`, registered `pub(crate)` in `runtime/src/primitives/mod.rs` beside `spec_links` and `decisions`:

```rust
/// One rule section of a rule file, in heading order.
pub(crate) struct RuleSection {
    pub(crate) id: String,
    /// The section's first block quote, `>` markers stripped, lines joined with spaces.
    pub(crate) statement: String,
    /// The first `**Verification:**` paragraph, wrapped lines joined; `None` when absent or empty.
    pub(crate) verification: Option<String>,
}

impl RuleSection {
    /// MUST-tier when the Statement carries MUST (MUST NOT included), else
    /// SHOULD-tier when it carries SHOULD, else `None`.
    pub(crate) fn tier(&self) -> Option<RuleSeverity>;
}

/// Every rule section in `content`, in heading order.
pub(crate) fn parse_rule_sections(content: &str) -> Vec<RuleSection>;

/// The rule-heading grammar, shared with `check-rule-ids`.
pub(crate) fn heading_id_regex() -> &'static Regex;
```

`heading_id_regex` moves here and `check_rule_ids.rs` imports it. A section runs from its heading to the next heading of the same or higher level, the boundary `section_is_deprecated` already uses (`runtime/src/primitives/check_rule_ids.rs:125`). The Verification paragraph ends at a blank line or the next `**Field:**`, as `extract_rule_verification` ends it today (`runtime/src/interpreter/payload.rs:962`).

The keyword scan splits the Statement on non-alphanumeric characters and matches the whole tokens `MUST` and `SHOULD`, case-sensitively. RFC 2119 keywords are written in capitals in every rule file, and a lowercase "must" in a Statement's prose is not an obligation keyword. Over this repository's 192 rules, 151 Statements carry MUST alone, 25 SHOULD alone, 16 both and none neither (measured during clarification over the Statement block quotes in `framework/rules/*.md`), so the scan yields 167 MUST-tier and 25 SHOULD-tier rules. A deprecated rule parses like any other and is asked about, as the spec's Edge Cases decide.

### The walker loads the rule set once, on the first assessment step

The rule set is loaded lazily, when the walk reaches its first `assessSpecQuality` step, from the walker context's `rule-files` list. That list is seeded by the session, or derived by `derive_analyze_seeds` from the rule-file directory in the sorted order `list_rule_files` returns (`runtime/src/interpreter/mod.rs:907`, `runtime/src/primitives/discover_rule_files.rs:151`). Each file is read with `read_repo_file`'s containment check (`runtime/src/interpreter/payload.rs:853`) and parsed with `parse_rule_sections`. The walker keeps the result in a new `Walker` field, beside `analyze_tally` (`runtime/src/interpreter/mod.rs:97`):

```rust
/// The rules an `/analyze` walk assesses, loaded at its first assessment step.
struct LoadedRules {
    /// Rules carrying a tier and a Verification, in file then heading order.
    assessable: Vec<AssessableRule>,   // id, verification, tier
    /// Rule sections with no Verification or no keyword.
    unassessable: u32,
    /// Listed rule files that could not be read.
    unreadable_files: u32,
}
```

Loading records its own gaps in the tally once per walk rather than once per step, so a rule that belongs to neither tier is not counted twice: each unreadable file under `rule-file-unreadable`, each unassessable rule under `rule-assessments-not-checked`. Loading lazily rather than in `Walker::new` keeps a walk that never reaches steps 11 and 12 from recording rules it was never going to assess.

### One request per rule of the step's tier

`handle_extension` (`runtime/src/interpreter/mod.rs:364`) builds one request per step today. For `assessSpecQuality` it builds a list: one `AssessSpecQualityRequest` for each assessable rule whose tier matches the step's, each carrying that rule's ID, Verification and tier in the unchanged typed shape (`runtime/src/schema/extensions.rs:28`). It then emits, awaits, validates and records each in turn. Every other extension point builds its single request exactly as before. The step's tier still comes from `severity_from_step_prose` (`runtime/src/interpreter/payload.rs:867`), but it now selects which rules a step asks about, and never assigns a tier to a rule. That is AC1's distinction.

Requests go out in rule-file order, then heading order, with `fresh_request_id` numbering them as it does today, so request ids and goldens are reproducible. Each response is validated as any extension response is. A failure halts the walk with `error: schema-mismatch` before `write-analysis` at step 20, so no record is written (`runtime/src/interpreter/mod.rs:600`). The `llm:assessSpecQuality` context key keeps the last response, as before. No step reads it: `llm:*` keys are read only by `verifyCriteria`'s gate (`runtime/src/interpreter/mod.rs:772`) and filtered out of every outbound request (`runtime/src/interpreter/payload.rs:193`).

A step that asks about no rule emits a progress envelope saying so, rather than silence in the stream. When the rule set is empty, whatever the cause, the step also records one `rule-assessments-not-checked` target: this is AC7's per-step record, the count today's no-rule-directory parity test already asserts (`runtime/tests/parity.rs:458`). A step whose own tier is empty while the other tier has rules records nothing, since every loaded rule was asked about. A step whose prose names no MUST or SHOULD tier cannot know which rules it covers. It records one `rule-assessments-not-checked` target rather than asking about nothing. `framework/commands/analyze.md` never writes such a step.

The per-rule builder replaces `build_assess_spec_quality_request`, `resolve_assessed_rule`, `extract_rule_verification` and `first_rule_with_verification` in `runtime/src/interpreter/payload.rs`. `build_extension_request`'s `assessSpecQuality` arm has no remaining caller once `handle_extension` builds the list itself: its only caller is `handle_extension` (`runtime/src/interpreter/mod.rs:371`). The arm is removed, and the function's doc comment (`runtime/src/interpreter/payload.rs:122`) is updated to match. The rule's citation no longer steers which rule is asked about, because every rule is asked about.

### The tally takes the tier from the rule it asked about

`AnalyzeTally::assessed` reads a request back to learn what it asked, and returns `Assessed::Nothing` for a request carrying no Verification (`runtime/src/interpreter/analyze_tally.rs:153`). No request is built without a rule now, so `record_assessment` takes the asked rule's tier directly. `Assessed` and its `Nothing` arm are removed, along with the test for a verdict on no rule (`:509`). That case is replaced by the per-step record above. A failure still counts in the finding's own tier first, and in the asked rule's tier when the host returned no finding (`:189`). A mixed rule's SHOULD-only failure therefore still lands in advisory when the host reports it so.

Two tally methods are added: one records a loaded rule set's gaps, and one records a step that asked about no rule because none is loaded.

### The unexamined vocabulary: one new reason, one redefined

- **`rule-file-unreadable`** (new; could not be read) counts the listed rule files the walk could not read. A file that cannot be read cannot be enumerated, so counting its rules is impossible. Counting the file is the only honest unit. It is kept separate from `artifact-unreadable`, which names the spec's own artifacts and escalates to blocking at `done` (`specs/022-deterministic-runtime/data-model.md:478`). A rule file is not the spec's artifact, and its unreadability must not block the spec.
- **`rule-assessments-not-checked`** (existing; could not be read) now counts the loaded rules the walk did not ask about, those with no Verification or whose Statement carries no keyword. When no rule is loaded, it counts each of steps 11 and 12 once. Today it counts requests that carried no rule (`specs/022-deterministic-runtime/data-model.md:476`).

The reason set is not enforced in code: `write-analysis` records whatever map it is bound (`runtime/src/primitives/write_analysis.rs:157`). It is documented in three places, all updated together: `framework/commands/analyze.md` §Unexamined targets (`:231`, `:233`), the table and prose at `specs/022-deterministic-runtime/data-model.md:473` and `:476`, and the `unexamined_by_reason` doc comment at `runtime/src/schema/primitives.rs:312`. Each states a count of its reasons ("the last six", "the other six", "all six", "the first two", "the last four"), and each count is rewritten without a number where the sentence allows. §grounding prefers a set stated without a count, since the count is what goes stale.

On the interactive path the host reads rule files itself. A host that cannot read one records `rule-file-unreadable` too, so the reason is not exec-only.

### The analyze-basic fixture's rule gains a keyword

The fixture's `CFG-CONST-001` Statement carries no RFC 2119 keyword (`runtime/tests/fixtures/analyze-basic/framework/rules/configuration.md`), so it would be recorded unexamined, and AC4's single count could not hold. Its Statement gains MUST. The golden then carries one request (`req-1`, step 11) and no request at step 12. `runtime/tests/fixtures/analyze-basic/stdin.jsonl` drops its `req-2` response, and `analyze_completes_on_the_session_write_session_writes` asserts one request, `blocking-findings: 1` and `advisory: 0` (`runtime/tests/parity.rs:426`, `:436`, `:437`). The golden is re-blessed filtered to that one test (`BLESS=1 cargo test --release --locked analyze_basic_stream_matches_golden`). The diff is confirmed to carry only the dropped request and the step-12 progress line.

`an_exec_analyze_records_the_assessments_it_receives` (`runtime/src/interpreter/mod.rs:1397`) uses a keyword-less rule, "A rule.". It is rewritten as AC4's multi-rule test:

- the rule files hold a MUST rule, a SHOULD rule, a rule carrying both keywords, a keyword-less rule and a rule with no Verification;
- a seeded `rule-files` path names a file that does not exist;
- the test asserts the requests step 11 and step 12 send, the rule and tier in each, the tier counts from scripted verdicts, and `unexamined-by-reason`, which holds `rule-assessments-not-checked: 2` and `rule-file-unreadable: 1` beside the host-detection reasons.

A second test scripts a malformed response partway through the loop and asserts that no `analysis.md` is written. Both tests are written before the walker changes and seen failing against the current walker.

### Documentation of the per-rule walk

`framework/commands/analyze.md` steps 11 and 12 gain an exec-path sentence each (AC8):

- `ductus exec` asks about every loaded rule of the step's tier, one request per rule;
- the rule's tier comes from its Statement's keyword;
- the host judges whether the rule's Verification trigger fires, and answers `passed: true` when it does not.

The steps keep their `MUST-tier` and `SHOULD-tier` phrases, which `severity_from_step_prose` reads. The step list is unchanged, so no other golden's step numbers move. The generated `.claude/commands/ductus/analyze.md` is rewritten by the pre-commit hook.

022's `assessSpecQuality` section (`specs/022-deterministic-runtime/data-model.md:1273`) gains the same statement: the exec walker sends one request per loaded rule carrying a tier and a Verification, and a trigger that does not fire is answered `passed: true`.

### 022 is reopened, corrected, and re-reviewed

The edits to 022's data model change a durable contract, and the signpost AC9 requires is a body edit. Neither is a mechanical edit under §spec-lifecycle. The canonical-sources map names 022's data model only for the scenario→task rule, so the sync exemption does not reach the unexamined-reason table. 022 is therefore reopened `done → in-progress` through the status primitive, following 059's discharge of its impact on 018 (`specs/059-project-in-a-repository-subdirectory/tasks.md`, task 14).

The signpost goes in 022's body beside the extension-point inventory (`specs/022-deterministic-runtime/spec.md:170`), as a **block-quoted** line linking to this spec. The block quote matters. 060 already depends on 022 through its Motivation's link, and `derive-dependencies` induces no edge from a block-quoted line while `check-review-gate`'s discharge test still reads it (066's AC5). An unquoted link would add a `022 → 060` edge and close a cycle.

022 is then reviewed and analyzed with 022 targeted. Its review window is measured after the reopen's first commit, as §implement-phase requires. `/ductus:implement`'s completion gate returns it to `done`, and 060 is targeted again.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/primitives/rule_sections.rs` | Create | Rule-section parser, tier derivation, shared heading grammar |
| `runtime/src/primitives/mod.rs` | Modify | Register `rule_sections` |
| `runtime/src/primitives/check_rule_ids.rs` | Modify | Import `heading_id_regex` from `rule_sections` |
| `runtime/src/interpreter/mod.rs` | Modify | Lazy rule-set load, per-rule request loop, empty-step progress, tests |
| `runtime/src/interpreter/payload.rs` | Modify | Per-rule request builder; remove the single-rule resolver and its parsers; doc comment |
| `runtime/src/interpreter/analyze_tally.rs` | Modify | Tier from the asked rule; rule-set and empty-step records; tests |
| `runtime/src/schema/primitives.rs` | Modify | `unexamined_by_reason` doc: new and redefined reasons |
| `runtime/tests/fixtures/analyze-basic/framework/rules/configuration.md` | Modify | Statement gains MUST |
| `runtime/tests/fixtures/analyze-basic/stdin.jsonl` | Modify | Drop the `req-2` response |
| `runtime/tests/golden/analyze-basic.jsonl` | Modify | Re-blessed: one request, no step-12 request |
| `runtime/tests/parity.rs` | Modify | One request, one blocking, no advisory |
| `framework/commands/analyze.md` | Modify | Steps 11–12 exec note; §Unexamined targets reasons |
| `.claude/commands/ductus/analyze.md` | Generated | Rewritten by the pre-commit hook |
| `specs/022-deterministic-runtime/data-model.md` | Modify | Reason table and prose; `assessSpecQuality` exec note |
| `specs/022-deterministic-runtime/spec.md` | Modify | Status flips; block-quoted signpost linking 060 |
| `version`, `runtime/Cargo.toml`, `runtime/Cargo.lock`, `runtime/CHANGELOG.md` | Modify | Release `0.54.2` |

## Trade-offs

- **Round trips and payload.** An exec analyze in this repository now sends up to 192 requests carrying about 3.4 MB of spec text (192 × the 17.5 KB median tracked `spec.md`), against 2 today. That cost was accepted at clarification as the price of examining every rule. Each request puts `spec-content` before `rule`, so a host that caches prompts could reuse that prefix across a walk. Nothing specifies this for `assessSpecQuality` and it is unmeasured.
- **Rejected: a per-item loop in the procedure grammar.** A general marker such as "repeat this step per item" would let any extension step fan out, including clarify's per-question round trip. It changes the procedure parser and every command's parse to serve one extension point today. The fan-out stays a walker behavior for `assessSpecQuality`, and generalizing it waits for a second consumer.
- **Rejected: counting a rule file's rules when it cannot be read.** An unreadable file has no enumerable rules. Guessing a count from the file's name or size would state a number nobody measured.
- **Rejected: reusing `artifact-unreadable` for rule files.** That reason escalates to blocking at `done` because it names the spec's own artifact. A shared rule file is not one.
- **Kept: per-section assessment when two files define the same ID.** Each section is asked about once, so neither section's text goes unexamined. Duplicate IDs are the rule format's integrity concern, not the walker's.
- **Limitation: exec clarify has the same one-request-per-step shape.** `/ductus:clarify` step 6 promises one `askClarifyQuestion` round trip per open question (`framework/commands/clarify.md:78`). The walker sends one per step, and `resolve_clarify_question` falls back to the first question (`runtime/src/interpreter/payload.rs:516`). This spec does not fix it. It is recorded as a disposition task in `tasks.md`, so 060 cannot reach `done` until it is fixed, routed or discarded.
