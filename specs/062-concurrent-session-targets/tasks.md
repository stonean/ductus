---
next-task: 31
---

# 062 — Concurrent session targets Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 29. Analyze the corrections spec 079 made

`check-review-gate` holds only on the analysis, so no review is owed. The pre-done verification ran first (2026-11-14): every criterion holds or is annotated, and no `spec.md` edit is needed. It owes `/ductus:analyze` and the completion gate. `## Applicable Rules` cites `BE-RACE-001` and `BE-RACE-002`, so step 14 reads their verdicts; both fired as `addressed` in the 2026-11-14 judging.

Waiting on a task that session `3fd29c6a` adds on another spec (the owner routes it): the session core opens `.ductus/sessions/{key}.toml` through canonical containment (`validate_contained`), or refuses a link out of the project. The owner deferred 062's `BE-INPUT-004` finding to it (2026-11-14): the design sanitizes the identity to a slug and documents no canonicalization or containment, the gap the owner ruled a finding on 045, and 022's `plan.md` §Path arguments omits the session files from the sites that open a path. Once the task exists, in one sitting:

- add to the plan's Technical Decision "Command files" the rationale the owner approved (2026-11-14): a host read would keep every command on the shared default, since only the runtime can read a process's identity (the first decision; the spec's §Process identity), and `fold` and `consolidate` remove a spec directory, so each must re-target or clear every session naming it, which only `retarget-sessions` reaches (AC21; §Removal strands no process);
- add a plan sentence naming the deferral's task;
- judge all 310 rules and run the step-15 pass after both edits, since a fix to a design subject is judged again (owner, 2026-11-14), briefing judge A that `BE-INPUT-004` is a MUST finding deferred to that task;
- record "Command files" fixed and `BE-INPUT-004` deferred, `decided-by` the owner.

A design change beyond what the code does, a blocking deferral, `done` and a push carrying another session's commits are the owner's to decide (2026-11-10).

- **Done when**: an analysis is recorded on this spec, and `/ductus:implement` has returned it to `done`.
