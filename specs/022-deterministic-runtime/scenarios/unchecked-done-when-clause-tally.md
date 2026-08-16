---
section: "Follow-on scenarios"
---

# Unchecked-done-when-clause-tally

## Context

[`done-when-authoring-forms`](done-when-authoring-forms.md) settled that a checkbox-nested `Done when` line is a **clause, not an addressable subtask** — `read-tasks` excludes it from its subtask list and `mark-task` excludes it from the subtask index space, one decision made once so the read/mark index contract holds. That decision is correct and this scenario does not reverse it.

What the decision did not consider is the **unchecked** form. Its edge cases are written against `- [x] Done when: …`, where clause-not-subtask is invisible to a reader: the box is already ticked, so excluding it changes nothing anyone sees. When `/ductus:plan` authors `- [ ] Done when: …` and the line is never ticked, the exclusion becomes visible — `/ductus:implement`'s completion tally counts only real subtasks, all of which are checked, and reports the task complete while an unchecked box sits on screen directly beneath it.

Observed on the adopter project `nookwit/magpie`: `/magpie:implement` reported "all checked" against tasks whose `tasks.md` still showed unchecked boxes. Magpie normalized its two specs to the canonical `- **Done when**: …` form (001 + 002, 27 lines), so it no longer trips — the workaround removed the symptom from one repo, not the defect from the runtime. Any adopter whose task breakdown is authored by `/ductus:plan` in the checkbox form reproduces it.

The defect is a reporting one: the runtime's tally and the human's reading of the same file disagree, and the human has no way to tell which is right.

## Behavior

The completion tally must never report a task fully checked while an unchecked checkbox is visible in its block. When a task's `done_when` was parsed from an **unchecked** checkbox-form line, the runtime resolves the disagreement rather than ignoring it:

- **`mark-task` ticks the clause line when the task's real subtasks all complete.** The clause is not addressable by subtask index — that contract is unchanged — but the primitive that completes a task owns leaving the block visually coherent, so the file a human reads agrees with the tally a machine computes. *(Extended by [mark-task-untick-symmetry](mark-task-untick-symmetry.md): the clause mirrors the tally in both directions, unticking when a flip leaves any real subtask unchecked. This scenario specified only the ticking direction, which left a ticked clause above an unchecked subtask — the mirror image of the incoherence it was written to remove.)*
- **The tally never claims more than the file shows.** Until the clause line is ticked, `/ductus:implement`'s per-task report distinguishes "all subtasks checked" from "task block fully checked", so an unticked clause is surfaced rather than rounded up.

Per [§design-principles](../../../AGENTS.md#design-principles) "never depend on human diligence", the fix is not to tell adopters to author the canonical form — the `tasks.md` template and `/ductus:plan` reference already point at it (`done-when-authoring-forms`, Prevention), and the checkbox form is authored anyway.

## Edge Cases

- **Checked clause (`- [x] Done when: …`)** — unchanged from `done-when-authoring-forms`: excluded from the subtask index, nothing to tick, no report difference. This scenario adds behavior only on the unchecked path.
- **Canonical bold form (`- **Done when**: …`)** — carries no checkbox at all, so there is nothing to tick and no visible discrepancy; the tally is unaffected.
- **Bulletless form (`Done when: …`)** — same as the canonical form: no checkbox, no discrepancy.
- **Unchecked clause with unchecked real subtasks** — the task is genuinely incomplete; `mark-task` does not tick the clause, because the condition for ticking it is that every real subtask is complete. Under [mark-task-untick-symmetry](mark-task-untick-symmetry.md) a *ticked* clause in that same state is unticked, which is the case this scenario left unhandled.
- **Subtask index contract** — ticking the clause must not shift the subtask index space. A task with two real subtasks still reports total 2, and `mark-task --subtask-index 2` remains out of range, ticked clause or not.
- **Idempotence** — completing an already-complete task does not rewrite an already-ticked clause line, so re-running `/ductus:implement` produces no diff.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
