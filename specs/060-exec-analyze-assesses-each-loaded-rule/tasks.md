# 060 — Exec analyze assesses each loaded rule Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Rule-section parser (AC6)

- [x] Create `runtime/src/primitives/rule_sections.rs` with `RuleSection`, `parse_rule_sections`, and `RuleSection::tier`; register it `pub(crate)` in `runtime/src/primitives/mod.rs`
- [x] Move `heading_id_regex` from `runtime/src/primitives/check_rule_ids.rs` into the new module and import it back, leaving `check-rule-ids`' behavior unchanged
- [x] Unit tests: MUST alone, MUST NOT alone, SHOULD alone, SHOULD NOT alone, both keywords (MUST-tier), neither (no tier), lowercase "must" only (no tier), no Verification, an empty Verification, a Verification wrapped across lines, a `**Statement:**`-prefixed block quote, a deprecated section (parsed like any other), a category `##` heading that is not a rule, and a section ending at the next same-or-higher heading
- [x] Run the parser over `framework/rules/*.md` in a test asserting that every rule heading parses into a section carrying a Verification and a tier; confirm once that it reproduces the clarification's measurement (192 sections, 167 MUST-tier, 25 SHOULD-tier), without pinning those counts in the test, since they change whenever a rule is added or split

- **Done when**: the parser tests pass, `check_rule_ids` tests pass unchanged, and the parser reproduces the clarification's 167 and 25.

## 2. Walker asks about each rule once, in its own tier (AC1, AC2, AC5, AC7)

- [x] Write the tests first and see them fail against the current walker: rewrite `an_exec_analyze_records_the_assessments_it_receives` (`runtime/src/interpreter/mod.rs:1397`) as the multi-rule test the plan describes (MUST, SHOULD, mixed, keyword-less, and Verification-less rules plus one unreadable `rule-files` path), and add a test that a malformed response partway through the loop writes no `analysis.md`
- [x] Add the lazy rule-set load to `Walker`, recording unreadable files under `rule-file-unreadable` and unassessable rules under `rule-assessments-not-checked` once per walk
- [x] In `handle_extension`, build and send one `assessSpecQuality` request per assessable rule of the step's tier, in file then heading order; emit a progress envelope for a step that asks about no rule; record one `rule-assessments-not-checked` for a step when no rule is loaded, and for a step whose prose names no MUST or SHOULD tier
- [x] In `runtime/src/interpreter/payload.rs`, replace `build_assess_spec_quality_request`, `resolve_assessed_rule`, `extract_rule_verification` and `first_rule_with_verification` with the per-rule builder; remove `build_extension_request`'s `assessSpecQuality` arm and update its doc comment; move or delete their tests
- [x] In `runtime/src/interpreter/analyze_tally.rs`, take the asked rule's tier directly, remove `Assessed` and its `Nothing` arm with that arm's test, and add the rule-set and empty-step records with tests

- **Done when**: the new tests pass, having failed before the change; `cargo test --release --locked` passes for the `interpreter` and `primitives` modules; and no request is built without a rule.

## 3. Fixture, golden and parity (AC4)

- [x] Give the `analyze-basic` fixture's `CFG-CONST-001` Statement a MUST keyword
- [x] Drop the `req-2` response from `runtime/tests/fixtures/analyze-basic/stdin.jsonl`
- [x] Update `analyze_completes_on_the_session_write_session_writes` to assert one request, `blocking-findings: 1` and `advisory: 0`; leave `analyze_with_no_rule_directory_records_its_citations_unexamined`'s `rule-assessments-not-checked: 2` as it is, and confirm it still passes
- [x] Re-bless with `BLESS=1 cargo test --release --locked analyze_basic_stream_matches_golden` only, and confirm the golden's diff is the dropped `req-2` request, its progress line, and the step-12 no-rule progress line, and nothing else

- **Done when**: every parity test passes and the golden diff contains only the lines named above.

## 4. Document the per-rule walk and the reasons (AC3, AC8)

- [x] `framework/commands/analyze.md` steps 11 and 12: add the exec-path sentence (one request per loaded rule of the tier, tier from the Statement's keyword, host judges the trigger and answers `passed: true` when it does not fire), keeping the `MUST-tier` and `SHOULD-tier` phrases
- [x] `framework/commands/analyze.md` §Unexamined targets: classify `rule-file-unreadable` as could-not-be-read, redefine `rule-assessments-not-checked` as counting rules, and rewrite the counted phrases ("The last six reasons") without a count where the sentence allows
- [x] `runtime/src/schema/primitives.rs` `unexamined_by_reason` doc comment: the same two reasons
- [x] Commit, letting the pre-commit hook regenerate `.claude/commands/ductus/analyze.md`, then run `cargo test --release --locked` in full, since a command-source edit can move a parity golden

- **Done when**: the three documents name the same reason set with the same classes, the generated command matches its source, and the full test suite passes.

## 5. Prose-claim sweep

- [x] Grep live artifacts for claims of the old behavior by meaning, not only by identifier: one rule per step, a tier taken from the step's prose, the first cited rule, `resolve_assessed_rule`, and `rule-assessments-not-checked` counting requests. The live artifacts are `framework/`, `specs/` (022's scenarios included), `README.md`, `AGENTS.md`, and runtime doc comments
- [x] Grep for the count words of the reason set ("six reasons", "other six", "all six", "first two", "last four") across the same artifacts
- [x] Classify each hit: corrected here, historical (a changelog entry, a review record, a past-tense account), or corrected in task 6 as part of 022

- **Done when**: every hit is corrected or classified, and no present-tense claim of the one-rule-per-step walk remains outside 022.

## 6. Discharge the impact on 022 (AC8, AC9)

Operator-approved (2026-09-27, with the `cross-spec-impact:` declaration): this task writes to `specs/022-deterministic-runtime/` and reopens 022 once.

- [x] Reopen `022-deterministic-runtime` (`done → in-progress`) through the status primitive, committed as its own step
- [x] `specs/022-deterministic-runtime/data-model.md`: add `rule-file-unreadable` to the could-not-be-read row (`:473`); rewrite the `rule-assessments-not-checked` sentence and the reason counts in the prose (`:476`); add the exec per-rule and trigger-judgment note to the `assessSpecQuality` section (`:1273`)
- [x] `specs/022-deterministic-runtime/spec.md`: add a **block-quoted** signpost beside the extension-point inventory (`:170`) linking to this spec, so `check-review-gate` reads the discharge and `derive-dependencies` induces no `022 → 060` edge; after committing, read the derivation's cycle report as well as its drift report
- [x] Read 022's ticked criteria naming `assessSpecQuality` or the unexamined reasons against the new behavior, and correct or annotate any claim 060 supersedes
- [x] With 022 targeted, run `/ductus:review` and `/ductus:analyze`, then `/ductus:implement`'s completion gate, which returns 022 to `done`; target 060 again afterwards

- **Done when**: 022 is `done`, its body links to 060 in a block quote, `derive-dependencies` reports no cycle, and `check-review-gate` on 060 reports the impact discharged.

## 7. Disposition out-of-spec finding: exec clarify asks only the first open question

`/ductus:clarify` step 6 promises one `askClarifyQuestion` round trip per open question (`framework/commands/clarify.md:78`), and 022's `clarify-command-acceleration` scenario says the same. The exec walker sends one request per step, and `resolve_clarify_question` falls back to the first question (`runtime/src/interpreter/payload.rs:516`). Found while planning 060, outside its scope.

- [x] Decide with the operator: fix, route (the bug decision tree's clear-spec, wrong-implementation branch points at a scenario on 022), or discard with its reason written here — routed to `specs/022-deterministic-runtime/scenarios/exec-clarify-asks-each-open-question.md` (operator decision 2026-09-27, implemented in this effort before `0.54.2`; 022's task 125 carries it)

- **Done when**: the finding is fixed, routed to an artifact the pre-`done` gate reads, or discarded with its reason recorded on this task.

## 8. Local gate

- [x] `cargo fmt --check`, `cargo clippy --release --all-targets --locked -- -D warnings`, and `cargo test --release --locked` under `runtime/`, as CI runs them
- [x] `bash scripts/audit/run-all.sh`, the generators, and `npx markdownlint-cli2` over every changed markdown file
- [x] Commit, then run the checks that read git history again against the commit

- **Done when**: every check passes against the committed tree, with each result read on the line that ran it.

## 9. Release `0.54.2`

- [x] Bump `version` and `runtime/Cargo.toml` to `0.54.2`; refresh `runtime/Cargo.lock` with one build without `--locked`
- [x] Add a `runtime/CHANGELOG.md` section covering tasks 1–3
- [x] Run audit Family 20

- **Done when**: Family 20 passes and the changelog section names every runtime change. The tag is cut after 060 and 022 are `done`, in the same session.

## 10. Rule-file review 2026-10-19: replace the stale rule count

- [x] spec.md's edge case states the property (no shipped rule lacks a Verification or a tier, held by every_shipped_rule_is_assessable) instead of the 192-rule count the corpus has outgrown

- **Done when**: spec.md makes no count claim about the shipped rule corpus that the tree contradicts, and 060 is re-reviewed back to done.

## 11. An assessment sees the loaded rule files

- [x] Implement the behavior described in `scenarios/an-assessment-sees-the-loaded-rule-files.md`

- **Done when**: every `assessSpecQuality` request carries the loaded rule files, a test proved to fail without the field covers it, the 022 data-model section and `framework/commands/analyze.md` steps 11–12 describe the load-conditioned yield, AC5's "existing typed shape" is reworded to say the rule stays in the `rule` object beside the new field, and the parity goldens are re-blessed with each diff read
