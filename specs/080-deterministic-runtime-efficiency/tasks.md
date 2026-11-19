---
next-task: 66
---

# 080 — Deterministic runtime efficiency Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 59. The generator's timeout bound is a named constant (CFG-CONST-004, from 080's analysis)

- [x] `scripts/gen-claude-commands.sh` names the lower bound of `DUCTUS_GEN_TIMEOUT_SECONDS` as a constant beside `DEFAULT_GEN_TIMEOUT_SECONDS` (`MIN_GEN_TIMEOUT_SECONDS=1`); the range check compares against it rather than a bare `0`, and the refusal names the bound. 080's analysis found CFG-CONST-004 (MUST) on 2026-11-16, since neither the plan nor the script named that bound; the owner deferred it here and chose to do it in the same session
- [x] 080's plan, in its `DUCTUS_GEN_TIMEOUT_SECONDS` item, names the constant
- [x] `scripts/tests/test-gen-claude-commands.sh` shows a value below the bound refused with the bound named, and the test fails when the check stops reading the constant

- **Done when**: The bound is a named constant that the check, its refusal and the plan all name, a test that fails without it proves a value below it refused, and 080's next review reads the change.

## 60. A step gates on a bounded list by its count

- [x] Implement the behavior described in `scenarios/a-step-gates-on-a-bounded-list-by-its-count.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested

## 61. A primitive's description states a bounded list's emptiness by its count (found while implementing task 60)

- [x] Reword `DERIVE_DEPENDENCIES` and `DERIVE_REFERENCES` in `runtime/src/schema/descriptions.rs` and the matching doc comments in `runtime/src/schema/primitives.rs`, and report each description's byte change

- **Done when**: the `derive-dependencies` and `derive-references` descriptions and their argument doc comments say examined-and-clean by `updated-count` and the other counts, which both result forms carry, and `runtime/tests/bounded_list_gates.rs` or a sibling test fails on the old wording

## 62. The owner's decisions of 2026-11-19 on the file form are recorded where this spec keeps decisions

- [x] Decision one: the seventeen command steps whose outcome a file-form result can change are fixed, five by gating on the inline count and twelve by the existing clause that reads the list from the file `lists-file` names, which in part reverses the sweep of per-step branches decided on 2026-11-08
- [x] Decision two: `read-tasks` gains no inline unchecked count, since `check-review-gate` blocks on an unchecked task in both forms

- **Done when**: `plan.md`'s section on corpus-growing results and the spec's Resolved Questions state both decisions and no sentence still says every per-step branch is swept

## 63. Every primitive's description states a bounded list's emptiness by its count (found while implementing task 61)

- [x] Reword each served primitive's description and schema doc comment that states examined-and-clean by an empty bounded list, which is absent in the file form, to the list's inline count; the guard test in `runtime/tests/bounded_list_gates.rs` scans every served tool's description against its own bounded lists

- **Done when**: no served primitive's description or schema doc comment states a bounded list's emptiness by the list, no description constant grows, and the test fails on the old wording

## 64. Four more steps read a bounded list from the file when the result is in file form (owner decision, 2026-11-19)

- [ ] `framework/commands/analyze.md` and `framework/commands/review.md`, the steps that invoke `process-decisions`; `framework/commands/prune.md`, the `--project` apply branch; `framework/reference/clarify.md`, the paragraph deriving the scenario question list. `framework/commands/groom.md` step 2 stays, since its list is bounded by the slug set (owner, 2026-11-19)

- **Done when**: each of the four blocks names the file form for the bounded lists it reads or gates on their counts, each file's byte change is measured, and the whole `cargo test` passes

## 65. Disposition out-of-spec finding: spec prose in 022, 041, 045 and 067 states a bounded list's emptiness by the list

- [ ] Found by task 61's sweep: `specs/022-deterministic-runtime/data-model.md` and its scenario `scenario-open-question-signal`; `specs/041-task-pruning/data-model.md`; `specs/067-analyze-and-audit-checks/` (`spec.md`, `data-model.md`, `plan.md`, four scenarios); `specs/045-decision-state-drift-detection/` (`data-model.md`, `plan.md`). Each sentence is judged: true of both result forms, or false of the file form

- **Done when**: every named sentence has one disposition: fixed in its spec, deferred to that spec's work, or waived with its reason
