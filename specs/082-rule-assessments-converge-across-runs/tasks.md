# 082 — Rule assessments converge across runs Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Rule-file digest and design-subject predicate

- [x] Add a per-file digest over each rule's ID, Statement and Verification in heading order to `runtime/src/primitives/rule_sections.rs`
- [x] Add `is_design_subject` (`spec.md`, `plan.md`, `data-model.md`, `scenarios/*.md`) beside `is_analyze_subject` in `runtime/src/primitives/analyze_subjects.rs`
- [x] Unit tests: a rationale edit leaves the digest, a Statement or Verification edit moves it; a `tasks.md` or `review.md` edit leaves the design-subject digest

- **Done when**: the tests pass in the dev profile.

## 2. The three record fields

Blocked on task 1.

- [x] Add `rule_verdicts`, `unjudged_rules` and `rule_digest` to `AnalyzeBlock` in `runtime/src/schema/primitives.rs`
- [x] Render them in `render_analysis` in `runtime/src/primitives/write_analysis.rs`, flattened and quoted as its other scalars are
- [x] Tests: a written record reads back through `load_analyze_record`; a map or list that does not parse is `RecordLoad::Unreadable` and `validate-frontmatter` reports a hard fail, never an empty verdict set

- **Done when**: the round-trip and malformed-record tests pass.

## 3. The `rule_verdicts` library

Blocked on task 2.

- [x] Create `runtime/src/primitives/rule_verdicts.rs` and declare it in `runtime/src/primitives/mod.rs`
- [x] Currency: design-subject digests, `rule-digest` per file, not in `unjudged-rules`, record carries `rule-digest`; the reason when none is reusable
- [x] Citation check with `heading_lines`: a `path#Heading` naming a design subject and a heading it holds
- [x] Contradiction comparison against the prior verdict
- [x] Unit tests per rule: each of the three conditions failing alone, a removed rule's verdict dropped, a newly loaded rule judged, a design subject added or removed, each bad-citation form

- **Done when**: each currency, citation and contradiction case has a passing test.

## 4. `write-analysis` carries, derives and checks

Blocked on task 3.

- [x] Add `rule-verdicts` and `judged-rules` to `WriteAnalysisArgs` (CLI `--rule-verdicts-json FILE`, `--judged-rule`) and `contradictions` to its result
- [x] Discover the rule universe itself, carry current prior verdicts forward, derive `unjudged-rules` and add its count to `rule-assessments-not-checked` in `by_reason`
- [x] Record an `addressed` verdict with an unresolved or missing citation as unjudged
- [x] Refuse a record whose tier count plus released findings is below its number of `finding` verdicts
- [x] Add the `## Contradictions` section to the body skeleton and update the skeleton test
- [x] Update `WRITE_ANALYSIS` in `runtime/src/schema/descriptions.rs`
- [x] Tests: AC2 both directions (30 of 141 judged, then all), AC3, AC6, AC5 over an edited rule file, the floor refusal, and a reused `finding` not dropped by omission

- **Done when**: those tests pass, and `a_clean_run_still_writes_the_artifact` and the other existing `write_analysis` tests still pass.

## 5. `discover-rule-files` reports what is current

Blocked on task 3.

- [x] Add `feature` to `DiscoverRuleFilesArgs` and the `verdicts` block to its result, with `reusable` and `judge` through `bounded_lists::place`
- [x] Update `DISCOVER_RULE_FILES` in `runtime/src/schema/descriptions.rs` and the `discover-rule-files` case in `runtime/tests/result_sizes.rs`
- [x] Tests: AC1 on the interactive path, a second call over an unchanged fixture lists no rule under `judge` but the unjudged ones; an edited subject lists every rule

- **Done when**: the tests pass and `result_sizes` still measures every registry name.

## 6. The exec walker skips what is current

Blocked on tasks 4 and 5.

- [x] Add `addressed-by` to `AssessSpecQualityResponse` in `runtime/src/schema/extensions.rs`
- [x] `LoadedRules` carries reusable verdicts; `handle_assessments` drops them from `asked` and emits a progress line naming the count reused
- [x] `AnalyzeTally` keeps verdicts and judged rules, counts a reused finding in its tier, and binds `rule-verdicts` and `judged-rules`
- [x] Re-bless `runtime/tests/golden/analyze-basic.jsonl` by itself and read the diff
- [x] Tests: AC1 under exec (a second walk sends no request for a rule with a current verdict and records identical verdicts), AC2 under exec, AC5 and AC6 through the walker

- **Done when**: those tests and `runtime/tests/parity.rs` pass.

## 7. Prove each criterion's test fails without its behavior

Blocked on tasks 4 to 6.

- [x] Create `scripts/mutation/rule-verdicts-converge.py` with a mutant per behavior: the design-subject comparison dropped, the rule-digest comparison dropped, a `tasks.md` edit counted as a subject, the citation check skipped, the derived unjudged list emptied, the contradiction comparison removed, the carry-forward removed, the floor removed, the walker's skip removed
- [x] Run each mutant by id and name the killing test beside it; run `run.py --check`; rebuild the release binary after any parity mutant

- **Done when**: every mutant is killed by a named test, and AC1 to AC6 each have one.

## 8. The command and its reference

Blocked on tasks 4 to 6.

- [x] `framework/commands/analyze.md` steps 11 and 12: judge only the rules `discover-rule-files` lists under `judge`, re-report each `reusable` finding, pass `feature` and `judged-rules` to the recording step; steps 21 and 22: `rule-verdicts`, `judged-rules`, report each contradiction. Keep step numbers. Check how step 11's `llm:` marker combines with `<!-- audit:ignore-promotion -->` and the walker's argument rule in `AGENTS.md`, rather than assuming it
- [x] `framework/reference/analyze.md`: the Rules section's call, the `rule-assessments-not-checked` bullet under Unexamined targets, Record fields, and a section on stored verdicts and contradictions
- [x] Run `scripts/gen-claude-commands.sh`, then the sweep for prose that says every rule is judged on every run (`docs/analyze.md`, the two files above, 060 and 022 prose)

- **Done when**: the pre-commit hook's generators and the command, step-reference and promotion checks pass and the sweep greps are empty.

## 9. Constitution, 066 and 022

Blocked on task 4.

- [x] Add the three fields to the Analyze record row of `framework/constitution.md`, then confirm `scripts/lint-governing-size.sh` passes and record the new byte count in the commit
- [x] `specs/066-audit-records-and-the-completion-gate/data-model.md`: the three fields, the `write-analysis` arguments and result, and the `rule-assessments-not-checked` description
- [x] `specs/022-deterministic-runtime/data-model.md`: `addressed-by` in the `assessSpecQuality` response and the `verdicts` block of `discover-rule-files`

- **Done when**: the constitution is at or under 50,000 bytes and the three documents describe the shapes the code writes.

## 10. Back-links

Blocked on tasks 8 and 9.

- [x] A signpost in each of `specs/050-constitution/spec.md`, `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md` and `specs/066-audit-records-and-the-completion-gate/spec.md` linking to this spec, as a blockquote and with no criterion edited
- [x] `specs/022-deterministic-runtime/spec.md`: the same, once 022 is in this spec's `cross-spec-impact:`

- **Done when**: each named spec links back here; an edit that reopens its spec takes the back-edge, a cost, never a reason to leave the link out.

## 11. Measure the fired-rule fraction

Blocked on task 6.

- [x] Build the release binary and run `ductus exec analyze` over a few specs from `./runtime/target/release/ductus`
- [x] Count each record's `rule-verdicts` entries against its loaded rules and its bytes, state the method and denominator in the commit message, and edit the plan's Trade-offs if the map nears a full one

- **Done when**: the counts are in the commit message and the plan matches them.

## 12. Changelog

- [x] `runtime/CHANGELOG.md` Unreleased: the three record fields, the `discover-rule-files` `feature` argument, the `write-analysis` arguments and refusals, `addressed-by`, and the widened meaning of `rule-assessments-not-checked`

- **Done when**: the entry names each changed surface.

## 13. The completion gate holds done while a loaded rule is unjudged

- [ ] The owner's answer (a), 2026-11-12: an unjudged MUST rule at done is an unverified obligation, and since this spec the next analysis judges only those rules, so closing the gap is cheap. Raised by the independent verification: a spec edit that makes every verdict stale, followed by a run judging nothing, moves a prior MUST finding out of blocking-findings into unjudged-rules, which nothing gated.
- [ ] The gate surface is 066's (cross-spec-impact already declared on 066): update 066's data model and spec text where the gate's checks are listed, and 082's plan.

- **Done when**: check-review-gate blocks the in-progress to done transition while the spec's analysis record names any rule in unjudged-rules, naming them, with a test in both directions that fails when the check is removed, a mutant, the compiled CHECK_REVIEW_GATE description and 066's gate text updated
