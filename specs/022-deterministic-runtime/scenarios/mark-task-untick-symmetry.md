---
section: "Follow-on scenarios"
---

# Mark-task-untick-symmetry

## Context

[unchecked-done-when-clause-tally](unchecked-done-when-clause-tally.md) taught `mark-task` to tick a checkbox-form `Done when` clause once the flip completes every real subtask of its task, so the block a reader sees stops showing an unchecked box under a task the tally calls complete.

It specified only that direction. Unchecking a subtask on a previously-complete task therefore leaves the clause ticked — a ticked clause sitting above an unchecked subtask, which is the mirror image of the incoherence that scenario exists to remove. The asymmetry was deliberate at implementation time: the scenario named one direction, and inventing the other would have been unspecified behavior. This scenario names the other.

Impact is low today because the `/{project}:implement` walk only ever marks *checked*; unchecking is a manual edit. That makes this a coherence fix rather than a bug fix, which is why it waited for a decision rather than being assumed.

## Behavior

**The clause tracks its task's subtasks in both directions.** A checkbox-form `- [ ] Done when: …` clause is ticked when every real subtask of its task is checked, and unticked when any of them is not. The clause is a live mirror of the tally, not a one-way completion marker.

Rejected alternative, recorded because it was the other half of the decision: leave the behavior tick-only and document it in the originating scenario's Edge Cases as intended. That reading is coherent on its own terms — a completion marker legitimately records that a task *was* complete — but it loses the property the originating scenario was written to establish, that the file a human reads agrees with the tally a machine computes. A ticked clause above an unchecked subtask breaks that agreement in exactly the way an unticked clause above complete subtasks did.

**Everything else is unchanged.** The clause stays outside the subtask index space, so a two-subtask task still reports a total of 2 and `--subtask-index 2` stays out of range. The canonical bold form and the bulletless form carry no checkbox and are untouched. The write guard still compares rebuilt content against the file as read, so a flip that leaves the block already coherent produces no write.

## Edge Cases

- A task with no real subtasks: nothing to tally against, so the clause is left exactly as authored in either direction.
- A task whose clause is already in the correct state: no diff, per the existing write guard.
- Unticking is triggered by the same atomic write as the subtask flip — the clause and the subtask never land in separate writes, so an interrupted run cannot leave the two disagreeing.
- The non-checkbox clause forms remain out of scope: they carry no state that can disagree with the tally.
- A hand-edited clause that disagrees with its subtasks is corrected on the next `mark-task` call against that task, not proactively — the primitive reconciles what it writes, and does not sweep the file.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
