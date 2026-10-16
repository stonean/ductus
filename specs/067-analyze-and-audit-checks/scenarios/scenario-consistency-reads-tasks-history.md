---
section: "Artifact checks"
---

# Scenario-consistency-reads-tasks-history

## Context

`scenario-consistency` decided whether a scenario with no task in `tasks.md` was a finding from the shape of the file. Pruning leaves a fingerprint — a reset file has no task sections, and a keep-pending prune leaves numbering that no longer starts at 1 or has a gap — and the family read either as evidence that every unmapped scenario's task had been pruned. On a `done` spec it asked `tasks.md` history instead (000's `scenario-without-task-visibility`), but only after the fingerprint test, which ran first at every status.

That order failed in both directions, both seen on 022 on 2026-10-16:

- **A false finding for every implemented scenario.** After a reset, `append-task` numbers the next task 1, so the file shows no fingerprint. With 022 reopened and five new tasks numbered 1 to 5, the family flagged all 68 implemented scenarios, whose tasks had been pruned.
- **A real finding hidden.** At `done` with a reset `tasks.md` — the shape a pruned `done` spec is in — the fingerprint returned before history was asked, so a scenario no revision of `tasks.md` ever named went unreported, which is the case 000's scenario exists to report. 022 had two, and they surfaced only when the restart at 1 removed the fingerprint.

The fingerprint answers *was this file pruned*, which is not the question. The question is the one the `done` rule already asks of each scenario: did a task for it ever exist.

## Behavior

- At every status, a scenario that no task in the current `tasks.md` references is a finding **only when no revision of `tasks.md` in the history from HEAD ever named its slug**. A scenario whose task was pruned, by keep-pending or by reset, had a task in some revision and is never flagged. How the file is numbered today plays no part.
- The pruning fingerprint is removed. Nothing in the family reads the file's shape.
- **A history that cannot be consulted flags nothing, at every status**: no repository, a shallow clone, an unreadable revision, a non-UTF-8 blob. §tasks-phase mandates that direction — a pruned spent task never produces a finding — so the family does not manufacture one out of what it cannot see.
- **Nor is it silent.** Each unmapped scenario it could not judge is recorded in `skipped` with reason `no-readable-state`, so a clean result does not read as examined and found mapped (`QUAL-CLAIM-001`). A spec whose every scenario is mapped needs no history and records nothing.
- Below `done`, the message says the scenario has no task and never had one, and names appending its task, as `/{project}:amend`'s scenario route does, as the remedy. The `done` message is unchanged, so a decision stored against it still matches.
- `/{project}:analyze`'s markdown-only reference states the same rule, so the two paths agree.

## Edge Cases

- **A task appended and pruned before any commit.** The family reads the working `tasks.md` and the committed history, so a task that lived only between two commits is in neither, and its scenario reads as never tasked. The finding is advisory. The pipeline commits a scenario's task when it records the scenario, so the window is a task appended and pruned by hand within one uncommitted session.
- **A scenario renamed after its task was pruned.** No revision names the new slug, so it reads as never tasked. The finding is advisory, and a stored discard settles it.
- **No `tasks.md`.** Unchanged: the family does not run, and artifact completeness owns the missing file at `planned` and later.
- **Cost.** The walk runs only when some scenario is unmapped, stops once every unmapped slug has been found, and reads each distinct `tasks.md` blob once. A spec with a reset `tasks.md` and a never-tasked scenario walks the whole history, which is the one case that pays for it.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
