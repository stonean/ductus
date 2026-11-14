---
next-task: 25
---

# 023 — `ductus` Refinement Tasks

Tasks derived from the [plan](plan.md). Complete in order. Phase A (invariant + `ductus` primitives) must ship as a tagged `ductus` release before any phase B work begins. Within Phase A, the configure MCP allow-list generator lands first so the `runtime-tools.txt` → configure invariant holds at every commit.

## Phase C — Follow-on scenarios

### 24. Review and analyze the corrections spec 079 made

Spec 079 task 10 corrected scenarios `living-specs`, `configure-dedup-permissions` and `configure-retires-formerly-canonical-entries`, and annotated AC19 and AC36; a scenario edit stales the review. It owes `/ductus:review`, then `/ductus:analyze`, then the completion gate, which run once no box in this file is open (spec 079).

- **Done when**: a review and an analysis are recorded on this spec, and `/ductus:implement` has returned it to `done`.
