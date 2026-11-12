# 082 — Rule assessments converge across runs Plan

Implements [082 — Rule assessments converge across runs](spec.md).

## Overview

`write-analysis` stores each fired rule's verdict beside the loaded rules the run left unjudged, and a digest of the rule files it loaded. A later run reuses a stored verdict while the design subjects and the rule file are unchanged. One shared library, `rule_verdicts`, decides currency for three callers: `discover-rule-files` (the interactive host's list), the exec walker (which skips current rules) and `write-analysis` (which carries current verdicts forward, derives the unjudged list and detects contradictions). The data shapes are in [data-model.md](data-model.md).

## Technical Decisions

### The record holds three fields

`rule-verdicts` maps each rule that fired to its verdict, `unjudged-rules` lists each assessable loaded rule the run neither judged nor reused, and `rule-digest` maps each loaded rule file to a digest of its rules' text. A loaded rule in neither of the first two did not fire, as the spec's Resolved Questions decide. `rule-digest` is a third field the spec did not name. A not-fired verdict leaves no entry to hold a rule-text hash, so without it a rule edited after a not-fired judgment would never be judged again, which is AC5's failure for most rules. Fields are written by `render_analysis` in `runtime/src/primitives/write_analysis.rs` and typed on `AnalyzeBlock` in `runtime/src/schema/primitives.rs`. A map or list that does not parse then makes `load_analyze_record` return `RecordLoad::Unreadable`, which `validate_record` in `validate_frontmatter.rs` reports as a hard fail: the record is undeterminable, never empty (Edge Cases).

### A stored verdict is current while three things match

1. **Design subjects.** The prior record's `analyzed-digest` entries for `spec.md`, `plan.md`, `data-model.md` and `scenarios/*.md` equal a fresh digest of those paths, with the same key set, and neither side lists one under `analyzed-unreadable`. A new `is_design_subject` predicate beside `is_analyze_subject` in `runtime/src/primitives/analyze_subjects.rs` selects them, and `subject_digest` takes it, as it takes `is_analyze_subject` and `is_review_contract` already. `tasks.md` and `review.md` stay in `analyzed-digest` for the gate and are ignored here (Resolved Questions). A first `plan.md` or a deleted scenario changes the key set (Edge Cases), and a design-subject edit makes every stored verdict stale. `spec.md` keeps `without_criterion_ticks`, so a tick moves nothing.
2. **Rule text.** The prior `rule-digest` entry for the rule's file equals the file's digest now: SHA-256 over each rule's ID, Statement and Verification in heading order, from the `RuleSection` fields in `runtime/src/primitives/rule_sections.rs` (`parse_rule_sections`, the one reading of what a rule is, which the exec `load_rules` in `runtime/src/interpreter/payload.rs` already uses). A rationale edit moves nothing. The granularity is the file, not the rule: see Trade-offs, where the per-rule alternative is costed.
3. **The rule was judged.** It is not in the prior `unjudged-rules`, and the prior record carries a `rule-digest` at all. A record without one predates this spec and yields no reusable verdict, as `freshness_of` treats a record with no digest as undeterminable rather than a match.

A rule in a file the prior `rule-digest` lacks is newly loaded, so it is judged. A stored verdict naming a rule no loaded file defines is dropped: `write-analysis` writes the map whole from the current rules, so a removed or disabled rule's verdict is never carried (Edge Cases). A citation into a deleted or renamed subject needs no check of its own, since the digest comparison in (1) already marks every verdict stale.

### `rule_verdicts` is one library

`runtime/src/primitives/rule_verdicts.rs` (new, not a primitive) holds the currency decision, the citation check and the contradiction comparison, so the three callers cannot disagree. This is the arrangement `freshness_of` in `analyze_subjects.rs` gives the gate and `/{project}:review`'s `analyze` row: one function, two surfaces.

### Where the decision is made: a library, reached through an extended `discover-rule-files`

The currency decision is deterministic, so it belongs to the runtime (§runtime-boundary principle 2), not the host. The interactive host learns it from `discover-rule-files`, which step 5 and the Rules section of `framework/reference/analyze.md` already call with `all`, `rules` and `verification`. It gains an optional `feature` argument and, when set, a `verdicts` result block: `reusable` (each current fired verdict with its rule ID, tier, `cite` or `at` and `message`), `judge` (the rule IDs whose verdict is not current) and `reason` when none is reusable (`no-prior-verdicts`, `subjects-changed`). The host judges only the rules in `judge` and re-reports each `reusable` finding as any finding. `reusable` and `judge` grow with the rule library, so they go through `bounded_lists::place` as `process_decisions.rs`' lists do. The extension adds no registration site, where a new primitive costs the seven in `docs/developing-ductus.md` (registry, description, `#[tool]`, `dispatch_primitive` arm, `Command`, `runtime-tools.txt`, `result_sizes` case) and a regenerated allow block for every agent in `framework/bootstrap/configure/`.

Under `ductus exec` the walker calls `rule_verdicts` as a library, as `load_current_task` in `runtime/src/interpreter/payload.rs` already calls `read_tasks::compute`. `Walker::handle_assessments` in `runtime/src/interpreter/mod.rs` drops each rule whose verdict is current from `asked`, so AC1 holds on both paths. `LoadedRules` gains the reusable verdicts, and the walker emits a progress line naming how many it reused, so a skipped rule is visible in the stream. A reused finding still counts in the tier counts: `AnalyzeTally` adds a `record_reused` that counts it by the rule's tier.

### The exec response says whether a fired rule was addressed

`AssessSpecQualityResponse` in `runtime/src/schema/extensions.rs` gains an optional `addressed-by`, a `path#Heading` citation. `passed: true` with it is `addressed`, `passed: true` without it is not fired, and `passed: false` is a finding whose `message` and `location` the tally stores. The field is additive, so a host that sends none records every `passed: true` as not fired, which is the walk before this spec. `AnalyzeTally` keeps `verdicts` and `judged` and `bind`s them as `write-analysis`' `rule-verdicts` and `judged-rules`.

### `write-analysis` derives the rest

`WriteAnalysisArgs` gains `rule-verdicts` (this run's newly judged fired rules, CLI `--rule-verdicts-json FILE` as `findings` is) and `judged-rules` (every rule the run judged, not-fired ones included, repeatable `--judged-rule`). From them and the rule files it discovers itself, `write_analysis::run` derives:

- **The universe.** Every assessable rule in the directory `resolve_rules_dir` and `list_rule_files` give, under `all`, the set the exec walker's `derive_analyze_seeds` binds. A host that treats a file as unloadable still leaves its rules in the universe, so they are named unjudged: the failure is visible, never a shorter list.
- **Carried-forward verdicts.** Each prior verdict still current under the rule above is carried into the new map unless this run judged that rule, so a host cannot drop a reusable `finding` by not passing it.
- **`unjudged-rules`.** The universe less `judged-rules` less the carried-forward rules, plus each rule this run's `addressed` verdict cited badly. Its count is added to `rule-assessments-not-checked` in `by_reason`, as the unresolved-constitution count is today, so a caller cannot supply a total that contradicts the list. The count's meaning widens as the spec decides: it keeps 060's rules the walker could not ask about, and also counts the rules not judged.
- **A tier-count floor.** The run is refused when a tier's supplied count plus its released findings is below its number of `finding` verdicts, since each stands for at least one live finding. The tier is the rule's own, read from the current rule text. Reused findings are the host's to count, and the exec tally does; the floor keeps an omission from dropping a MUST failure from `blocking-findings` (`QUAL-CLAIM-001`).

The write stays whole and atomic through `write_atomic`, so under two concurrent runs the later record stands (Edge Cases).

### An `addressed` citation is checked by `write-analysis`

A citation is `path#Heading`: a design subject of the feature and a heading it contains. `rule_verdicts` resolves it with `heading_lines` in `runtime/src/primitives/resolve_anchor.rs`, the heading reader `resolve-anchor` and `read-section` share, compared by equality as `section_range` compares, since an explicit pair marks where the name ends. A citation naming a non-design subject, a file that is missing or unreadable, a heading the file lacks, or no citation at all is not stored as `addressed`: the rule joins `unjudged-rules`. The closed unexamined-reason set gains nothing (Resolved Questions). The check runs against the files as they are at write time, which `write-analysis` already treats as what the run read (`analyze_subjects::subject_digest` is taken there for the same reason).

### A contradiction is derived from the prior record

When this run judges a rule whose prior verdict is current under the three-part rule, `write_analysis::run` compares them. A new `finding` against a prior not-fired or `addressed` is a contradiction. The finding is recorded in its tier as any finding is, and the result gains `contradictions` (rule ID and earlier verdict) and the body a `## Contradictions` section, `*None.*` when empty, so step 22 reports it. It holds nothing and appears in no count (AC3, Resolved Questions). It arises only when a rule is judged although its verdict was current, which needs a host that ignores `judge`, so no flag forces a re-judge here.

### The constitution pays about 110 bytes

`framework/constitution.md` is 49,616 bytes against `scripts/lint-governing-size.sh`'s 50,000 ceiling (`wc -c`, 2026-11-12, 384 bytes of headroom). The three fields add about 113 bytes to the Analyze record row, leaving about 270. No cut is needed. A fourth field would need one.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/primitives/rule_verdicts.rs` | Create | Currency decision, citation check, contradiction comparison |
| `runtime/src/primitives/rule_sections.rs` | Modify | Per-file digest over each rule's ID, Statement and Verification |
| `runtime/src/primitives/analyze_subjects.rs` | Modify | `is_design_subject` predicate |
| `runtime/src/primitives/mod.rs` | Modify | Declare the `rule_verdicts` module |
| `runtime/src/primitives/discover_rule_files.rs` | Modify | `feature` argument and `verdicts` block |
| `runtime/src/primitives/write_analysis.rs` | Modify | New args, carry-forward, derived unjudged list, citation check, floor, contradictions, three fields and body section |
| `runtime/src/schema/primitives.rs` | Modify | `AnalyzeBlock`, both args and both results |
| `runtime/src/schema/descriptions.rs` | Modify | `DISCOVER_RULE_FILES` and `WRITE_ANALYSIS` descriptions |
| `runtime/src/schema/extensions.rs` | Modify | `addressed-by` on `AssessSpecQualityResponse` |
| `runtime/src/interpreter/payload.rs` | Modify | `LoadedRules` carries reusable verdicts and file digests |
| `runtime/src/interpreter/mod.rs` | Modify | `handle_assessments` skips current rules |
| `runtime/src/interpreter/analyze_tally.rs` | Modify | Verdicts, judged set, reused findings, bindings |
| `runtime/tests/rule_verdicts.rs` | Create | AC1 to AC6 over both paths, each proved red |
| `runtime/tests/result_sizes.rs` | Modify | `discover-rule-files` case with a `feature` |
| `runtime/tests/golden/analyze-basic.jsonl` | Modify | Re-blessed exec stream, diff read |
| `scripts/mutation/rule-verdicts-converge.py` | Create | One mutant per behavior, each killed by a named test |
| `framework/commands/analyze.md` | Modify | Steps 11, 12, 21 and 22 |
| `framework/reference/analyze.md` | Modify | Rules, Unexamined targets, Record fields, and a rule-verdicts section |
| `framework/constitution.md` | Modify | Analyze record row |
| `specs/066-audit-records-and-the-completion-gate/data-model.md` | Modify | Record fields, `write-analysis` args and result |
| `specs/022-deterministic-runtime/data-model.md` | Modify | `assessSpecQuality` response and `discover-rule-files` |
| `specs/050-constitution/spec.md` | Modify | Back-link signpost |
| `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md` | Modify | Back-link signpost |
| `specs/066-audit-records-and-the-completion-gate/spec.md` | Modify | Back-link signpost |
| `runtime/CHANGELOG.md` | Modify | Unreleased entry |

## Data Model

[data-model.md](data-model.md) defines `rule-verdicts`, `unjudged-rules` and `rule-digest`, and the shape of the new `discover-rule-files` block. The analysis record's existing schema stays in [066's data model](../066-audit-records-and-the-completion-gate/data-model.md).

## Trade-offs

- **Rule text is hashed per file, not per rule.** The Resolved Questions name a hash of the rule's own text, and a not-fired rule has no entry to hold one. A per-rule hash for every loaded rule costs `(ID + 12 hex)` each: about 8.8 KB per record at this repository's 310 rules, about 640 KB across its 73 records, against 1,074 bytes and 78 KB for 12 file digests (measured 2026-11-12 over `framework/rules/` and `specs/*/analysis.md`). That is the cost the Resolved Questions rejected for a verdict-per-rule map. The price of the file digest is that editing one rule judges that file's rules again, not only it. A rule edit is rare beside a spec edit, which already judges every rule.
- **`finding` verdicts store their message and location.** AC1 forbids asking about a rule with a current `finding`, and the next run must still report it in its tier, so the message stays in the record. The spec's 26 to 61 bytes per fired rule held for `addressed`; a `finding` entry carries about 100 more.
- **A fix re-judges every rule.** A chore fix edits a design subject, so the re-check (step 20) finds every verdict stale. Skipping the first pass's verdicts there would judge a fix's own effects against stale text, so the re-check pays one full pass.
- **The `rule-assessments-not-checked` count can overlap.** A step whose prose names no tier still records its one target under 060's rule, beside the unjudged rules `write-analysis` now names. The state is a defect, and removing the walker's call would change 060 for no reader.
- **Fired-rule fraction is unmeasured.** The map approaches a full one if most rules fire. Task 11 measures it.
- Rejected: a new `assess-rules` primitive, for the registration sites above; letting the host assemble the list from `discover-rule-files` and a stored map, which leaves the currency rule to every host and every session.

## Open Questions Resolved

The five Resolved Questions are applied as written. The first names no storage for the rule-text hash, which `rule-digest` supplies, and the fourth's size estimate gains the `finding` message and the digest (both under Trade-offs).

## Cross-spec impact

- [050](../050-constitution/spec.md) owns the constitution's contents; the audit-record row gains the three fields. Discharged by a signpost in its `spec.md` linking back here.
- [060](../060-exec-analyze-assesses-each-loaded-rule/spec.md) owns the exec walker's per-rule requests, which now skip current rules and send `addressed-by`. Discharged by a signpost in its `spec.md`.
- [066](../066-audit-records-and-the-completion-gate/spec.md) owns the record's schema and `write-analysis`' contract; its data model gains the fields and arguments. Discharged by a signpost in its `spec.md`.
- [022](../022-deterministic-runtime/spec.md) owns the `assessSpecQuality` response and `discover-rule-files` shapes in its data model, which this plan changes. 082's `cross-spec-impact:` does not list it, so task 10 adds the data-model sync and a back-link there, and the key needs 022 added when the plan is approved.
