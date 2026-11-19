---
next-task: 61
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
