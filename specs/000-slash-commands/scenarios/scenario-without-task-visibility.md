---
section: "Command Set"
---

# Scenario-without-task-visibility

## Context

`/{project}:amend`'s scenario route is the sanctioned way to add a scenario: it writes `scenarios/{slug}.md`, appends a linked task to `tasks.md`, and takes the `done → in-progress` back-edge. A scenario added by hand — by an agent writing the file directly, or by a contributor — skips all three, and two separate mechanisms then fail to notice.

`/{project}:amend`'s re-open precondition detects only *uncommitted* deltas (`git status --porcelain`, untracked `??` scenario files — `framework/commands/amend.md:51-53`) and fires only on `done` specs (`:68`). Once the hand-added scenario is committed, that signal is gone permanently.

`/{project}:analyze`'s scenario→task mapping family explicitly does not flag a scenario under a `done` spec that has no task. That is a deliberate decision from [041-task-pruning](../../041-task-pruning/spec.md): the scenario→task linkage is not a durable index, because an implemented scenario's tasks are expected to have been pruned.

[046-scenario-open-question-visibility](../../046-scenario-open-question-visibility/spec.md) closes the case where the hand-added scenario carries open questions — those block `done` and produce a finding. A committed, **question-free** scenario with no task falls through every check: the spec stays `done` while carrying behavior that was never implemented, and nothing surfaces it.

## Behavior

- The signal does not depend on working-tree state. Whatever surfaces a scenario with no task fires on a committed scenario as well as an untracked one; `git status --porcelain` alone is insufficient as the sole trigger.
- 041's pruning decision is preserved. A `done` spec whose implemented scenario tasks were pruned remains a non-finding. This scenario does not reintroduce the scenario→task linkage as a durable index.
- The two states are therefore distinguished by positive evidence, not by the absence of a task alone. Which evidence is the open question below.
- When the unimplemented case is detected, the remedy matches what `/{project}:amend`'s scenario route would have written: the spec reopens `done → in-progress` and a task referencing the scenario is appended.

## Edge Cases

- **A scenario documenting already-shipped behavior, written after the fact** — §scenarios says an implemented scenario stays as documentation. This must not be reported as unimplemented work; it is the discriminator the open question turns on.
- **A `done` spec whose scenario tasks were pruned** — unchanged non-finding, per 041.
- **A hand-added scenario carrying open questions** — already covered by 046's `scenario-open-questions` family. This signal must not double-report the same spec.
- **A scenario under a non-`done` spec with no task** — the spec is already actionable and needs no reopen; existing scenario-consistency coverage applies unchanged.
- **A `tasks.md` reset to template state** (041's reset mode) — every scenario under the spec loses its task at once, which must not turn each one into a finding.

## Open Questions

- What positive evidence distinguishes a scenario documenting already-shipped behavior from one describing unimplemented work? Candidates: git history (the scenario file postdates the spec's last transition to `done`), an explicit marker written by whatever authored the scenario, or reading the code. Whichever answer is chosen must leave 041's pruned-task non-finding intact and must not double-report a spec already flagged by 046's `scenario-open-questions` family.

## Resolved Questions

*None yet.*
