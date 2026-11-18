# 060 — Exec analyze assesses each loaded rule Plan

Implements [060 — Exec analyze assesses each loaded rule](spec.md).

## Overview

The exec walker loads the project's rules once per `/{project}:analyze` walk, through one shared rule-section parser that reads each rule's ID, Statement and Verification and derives its tier from the Statement's RFC 2119 keyword. Each `assessSpecQuality` step then sends one request per loaded rule of the step's tier, so every rule is asked about exactly once, in its own tier. Rules the walk cannot ask about are recorded in `unexamined-by-reason`: a rule file it could not read under a new `rule-file-unreadable` reason, and a rule with no Verification or no keyword under `rule-assessments-not-checked`, which now counts rules rather than requests.

`framework/commands/analyze.md` and 022's data model are updated to describe the per-rule walk and the host's trigger judgment. 022 is reopened to take those edits and a signpost back to this spec, which discharges the `cross-spec-impact:` entry, then re-reviewed back to `done`. The runtime change ships in a patch release, `0.54.2`.

No `data-model.md` is created here. The only vocabulary this spec adds, one unexamined reason, belongs in the reason table 066's data model owns (`specs/066-audit-records-and-the-completion-gate/data-model.md`, `write-analysis`), and the walker's rule type is internal. This repository's `specs/system.md` holds only the network section 022's task 50 added, which this plan does not touch, and there is no `specs/events.md` or `specs/errors.md` to check the plan against.

## Technical Decisions

### One rule-section parser, sharing check-rule-ids' heading grammar

Two parsers read rule files today, and they disagree on what a rule is. `check-rule-ids` recognizes a rule by `heading_id_regex`, `^#{2,4}\s+([A-Z]{2,5}-[A-Z][A-Z0-9]+-\d{3,4})\b` (`runtime/src/primitives/check_rule_ids.rs`). The walker's `extract_rule_verification` and `first_rule_with_verification` treat any level-3 heading's text as a rule ID (`runtime/src/interpreter/payload.rs`). Enumerating every rule would add a third reading, so both walker functions are replaced by one module, and it takes the heading grammar from `check-rule-ids` rather than restating it. The set of rules the walk asks about is then the set of IDs `check-rule-ids` knows.

`runtime/src/primitives/rule_sections.rs`, registered `pub(crate)` in `runtime/src/primitives/mod.rs` beside `spec_links` and `decisions`:

```rust
/// One rule section of a rule file, in heading order.
pub(crate) struct RuleSection {
    pub(crate) id: String,
    /// The section's first block quote, `>` markers stripped, lines joined with spaces.
    pub(crate) statement: String,
    /// The first `**Verification:**` field to the next `**Field:**` line or heading, every block
    /// between included: a paragraph's wrapped lines joined, a table or list keeping its line
    /// breaks (scenario `an-assessment-sees-the-rule-it-asks-about`); `None` when absent or empty.
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

`heading_id_regex` moves here and `check_rule_ids.rs` imports it. A section runs from its heading to the next heading of the same or higher level, the boundary `section_is_deprecated` already uses (`runtime/src/primitives/check_rule_ids.rs`). The Verification runs from its marker to the next `**Field:**` line or heading, every block between included (scenario `an-assessment-sees-the-rule-it-asks-about`), and a line inside a fenced block is neither a field nor a heading.

The keyword scan splits the Statement on non-alphanumeric characters and matches the whole tokens `MUST` and `SHOULD`, case-sensitively. RFC 2119 keywords are written in capitals in every rule file, and a lowercase "must" in a Statement's prose is not an obligation keyword. Over this repository's 192 rules, 151 Statements carry MUST alone, 25 SHOULD alone, 16 both and none neither (measured during clarification over the Statement block quotes in `framework/rules/*.md`), so the scan yields 167 MUST-tier and 25 SHOULD-tier rules. A deprecated rule parses like any other and is asked about, as the spec's Edge Cases decide.

### The walker loads the rule set once, on the first assessment step

The rule set is loaded lazily, when the walk reaches its first `assessSpecQuality` step, from the walker context's `rule-files` list. That list is seeded by the session, or derived by `derive_analyze_seeds` from the rule-file directory in the sorted order `list_rule_files` returns (`runtime/src/interpreter/mod.rs`, `runtime/src/primitives/discover_rule_files.rs`). Each file is read with `read_repo_file`'s containment check (`runtime/src/interpreter/payload.rs`) and parsed with `parse_rule_sections`. The walker keeps the result in a new `Walker` field, beside `analyze_tally` (`runtime/src/interpreter/mod.rs`):

```rust
/// The rules an `/analyze` walk assesses, loaded at its first assessment step.
struct LoadedRules {
    /// Rules carrying a tier and a Verification, in file then heading order.
    assessable: Vec<AssessSpecQualityRule>,   // id, statement, verification, severity
    /// Rule sections with no Verification or no keyword.
    unassessable: u32,
    /// Listed rule files that could not be read.
    unreadable_files: u32,
}
```

Loading records its own gaps in the tally once per walk rather than once per step, so a rule that belongs to neither tier is not counted twice: each unreadable file under `rule-file-unreadable`, each unassessable rule under `rule-assessments-not-checked`. Loading lazily rather than in `Walker::new` keeps a walk that never reaches steps 11 and 12 from recording rules it was never going to assess.

### One request per rule of the step's tier

`handle_extension` (`runtime/src/interpreter/mod.rs`) builds one request per step today. For `assessSpecQuality` it builds a list: one `AssessSpecQualityRequest` for each assessable rule whose tier matches the step's and, since spec 082, whose stored verdict is not current, each carrying that rule's ID, Verification and tier in its `rule` object (`AssessSpecQualityRule` in `runtime/src/schema/extensions.rs`), beside `loaded-rule-files`, the basenames of the rule files the walk read, against which the host evaluates a Verification's "where `{file}` is also loaded" yield (scenario `an-assessment-sees-the-loaded-rule-files`). It then emits, awaits, validates and records each in turn. Every other extension point builds its single request exactly as before. The step's tier still comes from `severity_from_step_prose` (`runtime/src/interpreter/payload.rs`), but it now selects which rules a step asks about, and never assigns a tier to a rule. That is AC1's distinction.

Requests go out in rule-file order, then heading order, with `fresh_request_id` numbering them as it does today, so request ids and goldens are reproducible. Each request's size is bounded (`BE-PAYLOAD-001`): it carries `spec-content` inline only while it fits `[artifacts] read-size-bytes`, and past it a path the host reads, as `specs/022-deterministic-runtime/data-model.md` §Request bodies past the read size states (022's task 36). Each response is validated as any extension response is. A failure halts the walk with `error: schema-mismatch` before `write-analysis` at step 20, so no record is written (`Walker::validate_llm_response` in `runtime/src/interpreter/mod.rs`). The `llm:assessSpecQuality` context key keeps the last response, as before. No step reads it: `llm:*` keys are read only by `verifyCriteria`'s gate (`dispatch_primitive`'s `mark-criterion` branch in `runtime/src/interpreter/mod.rs`) and filtered out of every outbound request (`is_walker_internal_key` in `runtime/src/interpreter/payload.rs`).

A step that asks about no rule emits a progress envelope saying so, rather than silence in the stream. When the rule set is empty, whatever the cause, the step also records one `rule-assessments-not-checked` target: this is AC7's per-step record, the count today's no-rule-directory parity test already asserts (`analyze_with_no_rule_directory_records_its_citations_unexamined` in `runtime/tests/parity.rs`). A step whose own tier is empty while the other tier has rules records nothing, since every loaded rule was asked about. A step whose prose names no MUST or SHOULD tier cannot know which rules it covers. It records one `rule-assessments-not-checked` target rather than asking about nothing. `framework/commands/analyze.md` never writes such a step.

`load_rules` and the shared parser replace `resolve_assessed_rule`, `extract_rule_verification` and `first_rule_with_verification` in `runtime/src/interpreter/payload.rs`. `build_assess_spec_quality_request` stays, building each request from the one rule and the loaded rule files the walker seeds under `ASSESSED_RULE_KEY` and `LOADED_RULE_FILES_KEY`, and `build_extension_request`'s `assessSpecQuality` arm stays with it, since the walker's `exchange` builds every request through it. The rule's citation no longer steers which rule is asked about, because every rule is asked about.

### The tally takes the tier from the rule it asked about

`AnalyzeTally::assessed` reads a request back to learn what it asked, and returns `Assessed::Nothing` for a request carrying no Verification (`runtime/src/interpreter/analyze_tally.rs`). No request is built without a rule now, so `record_assessment` takes the asked rule's tier directly. `Assessed` and its `Nothing` arm are removed, along with the test for a verdict on no rule (`an_assessment_of_no_rule_is_unexamined_whatever_the_host_answers`). That case is replaced by the per-step record above. A failure still counts in the finding's own tier first, and in the asked rule's tier when the host returned no finding (`record_assessment`). A mixed rule's SHOULD-only failure therefore still lands in advisory when the host reports it so.

Two tally methods are added: one records a loaded rule set's gaps, and one records a step that asked about no rule because none is loaded.

### The unexamined vocabulary: one new reason, one redefined

- **`rule-file-unreadable`** (new; could not be read) counts the listed rule files the walk could not read. A file that cannot be read cannot be enumerated, so counting its rules is impossible. Counting the file is the only honest unit. It is kept separate from `artifact-unreadable`, which names the spec's own artifacts and escalates to blocking at `done` (`specs/022-deterministic-runtime/data-model.md`). A rule file is not the spec's artifact, and its unreadability must not block the spec.
- **`rule-assessments-not-checked`** (existing; could not be read) now counts the loaded rules the walk did not ask about, those with no Verification or whose Statement carries no keyword. When no rule is loaded, it counts each of steps 11 and 12 once. Today it counts requests that carried no rule (`specs/022-deterministic-runtime/data-model.md`'s `write-analysis` section).

The reason set is not enforced in code: `write-analysis` records whatever map it is bound (`runtime/src/primitives/write_analysis.rs`'s `run`). It is documented in three places, all updated together: `framework/reference/analyze.md` §Unexamined targets (informational), the table and prose in `specs/022-deterministic-runtime/data-model.md`'s `write-analysis` section, and the `unexamined_by_reason` doc comment at `runtime/src/schema/primitives.rs`. Each states a count of its reasons ("the last six", "the other six", "all six", "the first two", "the last four"), and each count is rewritten without a number where the sentence allows. §grounding prefers a set stated without a count, since the count is what goes stale.

On the interactive path the host reads rule files itself. A host that cannot read one records `rule-file-unreadable` too, so the reason is not exec-only.

### The analyze-basic fixture's rule gains a keyword

The fixture's `CFG-CONST-001` Statement carries no RFC 2119 keyword (`runtime/tests/fixtures/analyze-basic/framework/rules/configuration.md`), so it would be recorded unexamined, and AC4's single count could not hold. Its Statement gains MUST. The golden then carries one request (`req-1`, step 11) and no request at step 12. `runtime/tests/fixtures/analyze-basic/stdin.jsonl` drops its `req-2` response, and `analyze_completes_on_the_session_write_session_writes` asserts one request, `blocking-findings: 1` and `advisory: 0` (`runtime/tests/parity.rs`). The golden is re-blessed filtered to that one test (`BLESS=1 cargo test --release --locked analyze_basic_stream_matches_golden`). The diff is confirmed to carry only the dropped request and the step-12 progress line.

`an_exec_analyze_records_the_assessments_it_receives` (`runtime/src/interpreter/mod.rs`) uses a keyword-less rule, "A rule.". It is rewritten as AC4's multi-rule test:

- the rule files hold a MUST rule, a SHOULD rule, a rule carrying both keywords, a keyword-less rule and a rule with no Verification;
- a seeded `rule-files` path names a file that does not exist;
- the test asserts the requests step 11 and step 12 send, the rule and tier in each, the tier counts from scripted verdicts, and `unexamined-by-reason`, which holds `rule-assessments-not-checked: 2` and `rule-file-unreadable: 1` beside the host-detection reasons.

A second test scripts a malformed response partway through the loop and asserts that no `analysis.md` is written. Both tests are written before the walker changes and seen failing against the current walker.

### Documentation of the per-rule walk

`framework/commands/analyze.md` steps 11 and 12 gain an exec-path sentence each (AC8):

- `ductus exec` asks about every loaded rule of the step's tier that has no current stored verdict (spec 082), one request per rule;
- the rule's tier comes from its Statement's keyword;
- the host judges whether the rule's Verification trigger fires, and answers `passed: true` when it does not.

The steps keep their `MUST-tier` and `SHOULD-tier` phrases, which `severity_from_step_prose` reads. The step list is unchanged, so no other golden's step numbers move. The generated `.claude/commands/ductus/analyze.md` is rewritten by the pre-commit hook.

022's `assessSpecQuality` section (`specs/022-deterministic-runtime/data-model.md`) gains the same statement: the exec walker sends one request per loaded rule carrying a tier and a Verification, since spec 082 only for one whose stored verdict is not current, and a trigger that does not fire is answered `passed: true`.

### 022 is reopened, corrected, and re-reviewed

The edits to 022's data model change a durable contract, and the signpost AC9 requires is a body edit. Neither is a mechanical edit under §spec-lifecycle. The canonical-sources map names no data model for the unexamined-reason table, so the sync exemption does not reach it. 022 is therefore reopened `done → in-progress` through the status primitive, following 059's discharge of its impact on 018 (`specs/059-project-in-a-repository-subdirectory/tasks.md`, task 14).

The signpost goes in 022's body beside the extension-point inventory (`specs/022-deterministic-runtime/spec.md`'s `## LLM extension points`), as a **block-quoted** line linking to this spec. The block quote matters. 060 already depends on 022 through its Motivation's link, and `derive-dependencies` induces no edge from a block-quoted line while `check-review-gate`'s discharge test still reads it (066's AC5). An unquoted link would add a `022 → 060` edge and close a cycle.

022 is then reviewed and analyzed with 022 targeted. Its review window is measured after the reopen's first commit, as the review reference's (`framework/reference/review.md`) §Window and coverage requires. `/ductus:implement`'s completion gate returns it to `done`, and 060 is targeted again.

### This repository's host runs its rule judging from `scripts/analyze/`

The scenario `rule-judging-runs-from-repo-scripts` puts the host's orchestration around AC10's subagents in `scripts/analyze/`. Rebuilt in a temp directory, it was lost with the session and refined by nobody (owner, 2026-11-18). There is one script for each step. A cells manifest, `manifest.json`, is written by the split and read by every later step, so no share, group, feature or address is fixed in code:

- `detect.py` runs the deterministic steps.
- `split.py` cuts `verdicts.judge` into shares and the scenarios into groups.
- `briefs.py` renders `judge-brief.md` and `step15-brief.md`.
- `check.py` checks each hand-back.
- `combine.py` merges the cells.
- `record.py` calls `process-decisions` and `write-analysis`.

They stay in this repository, since the Shared Files manifest in `framework/bootstrap/ductus.md` names no `scripts/` path. An adopter's host follows `framework/reference/analyze.md` §Rule judging in a subagent itself.

- **Staging in the system temp directory.** The scripts stage under `$TMPDIR/ductus-analyze/<feature>`, or under `ANALYZE_WORK`, which must resolve inside the system temp directory, because the CLI reads a `--*-json` file only from there. A feature argument must name one directory directly under the spec root. They write no repository file. `analysis.md` is written by `write-analysis` alone.
- **Settings are read once, at startup, and every call is bounded.** `DUCTUS_BIN`, `ANALYZE_WORK` and `ANALYZE_TIMEOUT_SECONDS` are optional, each with its default a named constant in `scripts/analyze/analyze_lib.py`, an empty value counting as unset. `docs/runtime.md`'s environment-variable table lists them (`CFG-ENV-001`, `CFG-ENV-002`). Each runtime and git call is stopped after `ANALYZE_TIMEOUT_SECONDS`, by default `DEFAULT_TIMEOUT_SECONDS`, 600 seconds, and at most `MAX_TIMEOUT_SECONDS`, a day (`BE-TIMEOUT-001`). Each call passes its command as an argument vector, never a shell string, so no feature name, path or setting reaches a shell (`BE-INPUT-003`).
- **Each setting has one source beside its default, and a bad value fails before any work.** The source, when given, wins over the default constant, the standard order (`CFG-ENV-007`). For `--share-bytes` and `--cell-bytes` it is the flag; for the three variables, the environment. The types and checks (`CFG-ENV-005`, `CFG-ENV-008`):
  - `ANALYZE_TIMEOUT_SECONDS` is a whole number of seconds from `MIN_TIMEOUT_SECONDS` to `MAX_TIMEOUT_SECONDS`.
  - `ANALYZE_WORK` is a path that must resolve inside the system temp directory.
  - `DUCTUS_BIN` is the path, or a name on `PATH`, of an executable file.
  - Each flag is a whole number of bytes of at least `MIN_BUDGET_BYTES`.

  Each is parsed and checked when the script starts, never coerced to its default. A value that fails exits 2, naming the setting, the value and the bound.
- **Shares keep each rule file whole and balance by rendered bytes.** A cell is one share beside the spec, plan, data model and a scenario group. Scenarios are grouped only when one cell would pass `--cell-bytes`, into the fewest groups that keep each cell within it. A share or cell that cannot fit, because a rule file or a scenario is larger than its budget, is named in a note. The defaults are `DEFAULT_SHARE_BYTES` and `DEFAULT_CELL_BYTES`:
  - 135,000 bytes a share, just over the largest of the three rendered shares the 2026-11-18 analyses judged whole (114,213 to 132,784 bytes, by `wc -c`);
  - 480,000 a cell, the top of the 450 to 480 KB cells that `docs/developing-ductus.md` §Sizing an analysis's judges records finishing whole reads.
- **The briefs point at the canonical text.** A judge reads steps 11, 12 or 15 and the reference sections they name, so a template restates none of them. It carries only the run's facts and the JSON `check.py` reads back.
- **A cite is checked with `read-section`.** Its exact-text heading match uses the same `heading_lines` parse that `write-analysis`'s `cite_resolves` does, so a cite `check.py` passes is one the record keeps.
- **A disagreement goes to the host, never to a default.** Cells agreeing on a finding give a finding. A finding beside a cell that read other scenarios and found none is unsettled, since that cell may have read what addresses it. `record.py` refuses any rule its plan file does not settle.
- **What is recorded is what the judges wrote.** `record.py` refuses:
  - detection older than a commit, or than any uncommitted edit in the working tree: a changed file of the feature, a rule file or the constitution is named, and an edit anywhere else is reported as the working tree;
  - a design subject edited, added or removed since the split;
  - a malformed rule heading a judge reported that the plan file does not name.

  It checks and merges the hand-backs again itself rather than trusting `combine.py`'s output. It records an integrity item step 15 did not check under `integrity-not-checked`, and a broken cross-service reference as an Advisory finding. Any target the run could not read passes `restricted` to `process-decisions`, so a stored decision whose source went unseen is retained.
- **A failure is never an exit 0.** Each script exits 1 when a check fails, and 2 when an input or setting cannot be read, a call runs out of time, or the script itself fails. `decided-by` is `git config user.email`, and an empty one refuses.
- **The test stubs the runtime, and one case runs the real one.** `scripts/tests/test-analyze-scripts.py` runs each script in a fixture repository against a stub runtime. Its last case runs the whole chain against the release binary. The scripts read each list the runtime always writes strictly, so a field the runtime renames fails that case rather than reading as empty. `check-rule-ids`' `not-installed`, a list the runtime omits when empty, the case makes non-empty. `framework-checks.yml` step (ab) runs it after the job builds the runtime.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/primitives/rule_sections.rs` | Create | Rule-section parser, tier derivation, shared heading grammar |
| `runtime/src/primitives/mod.rs` | Modify | Register `rule_sections` |
| `runtime/src/primitives/check_rule_ids.rs` | Modify | Import `heading_id_regex` from `rule_sections` |
| `runtime/src/interpreter/mod.rs` | Modify | Lazy rule-set load, per-rule request loop, empty-step progress, tests |
| `runtime/src/interpreter/payload.rs` | Modify | Per-rule request builder carrying the loaded rule files; remove the single-rule resolver and its parsers; doc comment |
| `runtime/src/schema/extensions.rs` | Modify | `AssessSpecQualityRequest` gains `loaded-rule-files`; `AssessSpecQualityRule` gains `statement` |
| `runtime/src/interpreter/analyze_tally.rs` | Modify | Tier from the asked rule; rule-set and empty-step records; tests |
| `runtime/src/schema/primitives.rs` | Modify | `unexamined_by_reason` doc: new and redefined reasons |
| `runtime/tests/fixtures/analyze-basic/framework/rules/configuration.md` | Modify | Statement gains MUST |
| `runtime/tests/fixtures/analyze-basic/stdin.jsonl` | Modify | Drop the `req-2` response |
| `runtime/tests/golden/analyze-basic.jsonl` | Modify | Re-blessed: one request, no step-12 request |
| `runtime/tests/parity.rs` | Modify | One request, one blocking, no advisory |
| `framework/commands/analyze.md` | Modify | Steps 11–12 exec note and load-conditioned yield; `framework/reference/analyze.md` §Unexamined targets (informational) reasons |
| `.claude/commands/ductus/analyze.md` | Generated | Rewritten by the pre-commit hook |
| `specs/022-deterministic-runtime/data-model.md` | Modify | Reason table and prose; `assessSpecQuality` exec note and `loaded-rule-files` |
| `specs/022-deterministic-runtime/spec.md` | Modify | Status flips; block-quoted signpost linking 060 |
| `version`, `runtime/Cargo.toml`, `runtime/Cargo.lock`, `runtime/CHANGELOG.md` | Modify | Release `0.54.2` |
| `scripts/analyze/*.py`, `scripts/analyze/*.md` | Create | The host's rule-judging orchestration and its two brief templates |
| `scripts/tests/test-analyze-scripts.py` | Create | Each script against a stub runtime |
| `.github/workflows/framework-checks.yml` | Modify | Step (ab) runs the test |
| `docs/developing-ductus.md` | Modify | §Sizing an analysis's judges names the scripts and the disagreement rule |
| `docs/runtime.md` | Modify | Its environment-variable table lists `ANALYZE_WORK` and `ANALYZE_TIMEOUT_SECONDS`, and names `scripts/analyze/` in `DUCTUS_BIN`'s row |
| `AGENTS.md` | Modify | Points every session at `scripts/analyze/` |

## Trade-offs

- **Round trips and payload.** Measured at `33a5c302`, an exec analyze in this repository sends up to 192 requests carrying about 3.4 MB of spec text (192 × the 17.5 KB median tracked `spec.md`), against 2 before this spec; the request count grows with the shipped rule set. That cost was accepted at clarification as the price of examining every rule. Each request puts `spec-content` before `rule`, since `AssessSpecQualityRequest` in `runtime/src/schema/extensions.rs` declares them in that order and serde writes a struct's fields in declaration order, so a host that caches prompts could reuse that prefix across a walk. Nothing specifies this for `assessSpecQuality` and it is unmeasured.
- **Rejected: a per-item loop in the procedure grammar.** A general marker such as "repeat this step per item" would let any extension step fan out, including clarify's per-question round trip. It changes the procedure parser and every command's parse to serve one extension point today. The fan-out stays a walker behavior for `assessSpecQuality`, and generalizing it waits for a second consumer.
- **Rejected: counting a rule file's rules when it cannot be read.** An unreadable file has no enumerable rules. Guessing a count from the file's name or size would state a number nobody measured.
- **Rejected: reusing `artifact-unreadable` for rule files.** That reason escalates to blocking at `done` because it names the spec's own artifact. A shared rule file is not one.
- **Kept: per-section assessment when two files define the same ID.** Each section is asked about once, so neither section's text goes unexamined. Duplicate IDs are the rule format's integrity concern, not the walker's.
- **Out of scope: exec clarify's per-question round trip.** `/ductus:clarify` step 6's one `askClarifyQuestion` round trip per open question is spec 022's scenario `exec-clarify-asks-each-open-question` (`Walker::handle_clarify_questions`), which this spec leaves as it is.
