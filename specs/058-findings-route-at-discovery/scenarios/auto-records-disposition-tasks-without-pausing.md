---
section: "`/{project}:implement`"
---

# Auto-records-disposition-tasks-without-pausing

## Context

`/{project}:implement --auto` still pauses for "spec edits, plan edits, or new tasks discovered mid-implement". A disposition task is a new task, so under `--auto` every out-of-spec finding pauses the run. That contradicts walk step 5 and §brownfield-inbox Finding dispositions, which both say to record the finding and keep working. Before 058, an out-of-spec finding was captured without a prompt. The pause gives `--auto` a cost that manual capture never had.

A disposition task is a record, not a change of plan: nothing is decided when it is appended. The decision happens when the task is worked, and each disposition that writes is confirmed then.

Found by the 058 review (2026-09-25).

## Behavior

- Under `--auto`, appending a disposition task (`Disposition out-of-spec finding: …`) does not pause the run. The walk records it and continues with the task in hand, and the per-task summary lists it.
- Working a disposition task still pauses for its confirmation. A fix, a route (including any `done → in-progress` reopen it names), or a discard waits on the operator, because `--auto` never skips a confirmation that writes.
- A task that implements a fix inside the targeted spec is a new task under the existing rule, and still pauses.

`implement.md`'s `--auto` gate list names the exception, so the list and walk step 5 agree.

## Edge Cases

- **A run that appends several disposition tasks** pauses only when it reaches them.
- **A finding that turns out to be a requirement gap in the targeted spec** is not a disposition task. It routes through `/{project}:amend` and pauses as a spec edit.
- **Without `--auto`**, nothing changes: every task is confirmed as it is today.
- **A non-blocking finding met while working a disposition task** is recorded on that task, not appended (scenario `findings-cost-in-proportion-to-risk`), so it adds no task to pause at; its disposition is confirmed with the task's.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
