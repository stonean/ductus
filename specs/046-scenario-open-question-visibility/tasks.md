# 046 — Scenario open-question visibility Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Tasks 2–8 are authored as scenarios under [022 — Deterministic Runtime](../022-deterministic-runtime/spec.md) per the plan's Implementation ownership split; each carries a back-link to this spec. Task 1 blocks everything — no consumer reads the count until the parser is correct.

## 1. Fix the open-question parser

- [x] Switch `parse_open_questions` from the plain section walker to the comment- and fence-aware one, matching the acceptance-criteria parser
- [x] Widen the placeholder guard from an exact match to a set covering both the spec placeholder and the scenario placeholder
- [x] Add a regression test: a spec scaffolded from the shipped template reports zero open questions
- [x] Add a regression test: a question inside a fenced code block and one inside an HTML comment are both uncounted
- [x] Add a regression test: each placeholder is skipped, including when authored as a list bullet
- [x] Confirm `append-question`'s dedup still agrees with the reader after the change

- **Done when**: `read-spec` on a freshly-scaffolded spec returns zero open questions, and the full runtime test suite passes.

## 2. Add the scenario-open-question field to the spec reader

- [x] Read each `scenarios/*.md` through the shared scenario-file listing and parse its Open Questions with the task-1 parser
- [x] Return the entries as a field parallel to `open-questions`, each tagged with its source scenario
- [x] Leave `open-questions` untouched in meaning and value
- [x] Update the `read-spec` schema section in 022's data-model
- [x] Test: a feature whose scenario carries questions reports them in the new field and still reports zero in `open-questions`
- [x] Test: a feature with no `scenarios/` directory reports an empty field

- **Done when**: the new field is populated and documented, and no existing `open-questions` assertion changes.

## 3. Gate `done` on scenario questions

- [x] Add the scenario check to `check-review-gate` as a third ordered variant, between markdown lint and the `review:` block
- [x] Compose the blocked message so it names the scenarios carrying unresolved questions
- [x] Preserve first-failure-wins ordering
- [x] Update the `check-review-gate` schema section in 022's data-model
- [x] Test: a spec with a question-carrying scenario is blocked, with the scenario named
- [x] Test: a markdown-lint failure still wins over the scenario check
- [x] Test: an unreadable scenario file does not block

- **Done when**: the gate blocks the transition with a scenario-naming message, and existing gate tests are unchanged.

## 4. Add the analyze finding family

- [x] Add a `scenario-open-questions` family to `check-artifacts` — blocking at `done`, advisory otherwise
- [x] Wire `--fix` to revert `done → in-progress` with a non-silent notice, matching the review-state-drift revert
- [x] Apply no grandfather exemption
- [x] Update the `check-artifacts` schema section in 022's data-model
- [x] Test: a `done` spec with scenario questions yields a blocking finding and is reverted under `--fix`
- [x] Test: an `in-progress` spec with scenario questions yields an advisory finding and is not reverted

- **Done when**: the family reports at both severities and `--fix` reverts only at `done`.

## 5. Surface outstanding questions in the dashboard

- [x] Suffix the existing Scenarios column with the outstanding count, leaving the cell unchanged at zero
- [x] Override the Next Action cell to scenario-targeted clarify
- [x] Render a callout below the table naming the specs and their question-carrying scenarios, with no cap
- [x] Let recovery state win the Next Action cell when both apply, while rendering both callouts
- [x] Test: the precedence case renders the recovery action and both callouts

- **Done when**: all three surfaces render, and a spec with no scenario questions renders exactly as before.

## 6. Report outstanding questions at feature-level target

- [x] Report the count and name the carrying scenarios when a feature is targeted without a scenario
- [x] List every carrying scenario in case-insensitive filename order, recommending none
- [x] Recommend scenario-targeted clarification rather than implement
- [x] Leave the scenario-targeted path unchanged

- **Done when**: a feature-level target names its question-carrying scenarios and routes to clarification.

## 7. Update the command documentation

- [x] `target.md` — feature-level readout and routing
- [x] `status.md` — column suffix, Next Action override, callout, precedence rule
- [x] `implement.md` — completion-gate step names the third check
- [x] `analyze.md` — new family and its `--fix` behavior in the markdown-only reference
- [x] Verify each markdown-only path matches the runtime behavior it mirrors

- **Done when**: every changed behavior is documented on both the runtime and markdown-only paths.

## 8. Amend the constitution

- [x] §spec-lifecycle — the `done` row states that no scenario may carry unresolved questions
- [x] §readiness-check — "All open questions are resolved" states that scenario questions are included
- [x] Confirm the scenario back-edge description still reads correctly given resolved question 1

- **Done when**: both sections state the rule, and `resolve-anchor` reports every anchor still resolving.

## 9. Verify the feature end to end

- [x] Walk every acceptance criterion in the spec against shipped behavior
- [x] Confirm feature-targeted clarify resolves no scenario question and writes to no scenario file (its *reporting* behavior was revised 2026-08-16 by 022's `scenario-open-question-signal`; the resolution boundary is what this item verifies)
- [x] Confirm the spec-body open-question count is unchanged
- [x] Run the full runtime test suite and the feature directory's markdown lint
- [x] Confirm the 022 scenarios each back-link here and that 022's data-model is current

- **Done when**: every acceptance criterion is verified, and both specs' artifacts are consistent.
