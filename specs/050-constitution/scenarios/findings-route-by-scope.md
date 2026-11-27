---
section: "Follow-on scenarios"
---

# Findings-route-by-scope

## Context

While implementing a spec, an agent finds a bug or an omission in the spec it is currently implementing — a defect in what that spec built, a gap in what it specified, a case its scenarios missed. The reflex the framework had trained was capture. [§brownfield-inbox](../../../framework/constitution.md#brownfield-inbox)'s Automatic issue capture says findings MUST reach `specs/inbox.md` rather than be dropped, and the single exemption it carried covered an issue inside the *current task's* scope, which is fixed as part of the task and not logged.

Nothing covered the tier between those two, and that tier holds the most common case there is. An agent working a spec is looking hardest at exactly the surface that spec owns, so most of what it notices belongs to that spec. Capturing those findings is a loop with no destination: the item is appended, re-read on every intervening `/{project}:groom` pass, walked through the bug decision tree, and routed back to the spec that was open the whole time. The user named the cost directly — *"adding it to the inbox.md creates additional work."*

The rule was first written as an `AGENTS.md` entry, which is the failure this spec exists to correct. Contributor-side practice does not propagate to adopting projects, and this rule governs implementation behavior in every project running the pipeline, so its canonical text belongs in the constitution with `AGENTS.md` keeping only a pointer (`specs/050-constitution/spec.md` §Promotion mechanism).

## Behavior

> **Changed by [058 — Findings route at discovery](../../058-findings-route-at-discovery/spec.md).**
> A finding outside the spec ends in a disposition, never the inbox.
> §brownfield-inbox's Finding dispositions points at `/{project}:implement`'s
> reference, whose *Walk through tasks* step 5 carries the rule.

`/{project}:implement` routes a finding by scope, and the inbox is no destination. `framework/reference/implement.md`'s *Walk through tasks* step 5 names three tiers: a finding inside the current **task** is fixed in the task; a finding inside the current **spec** but outside the task is written to that spec's `tasks.md` as a new unchecked task; a finding outside the spec becomes a disposition task on the targeted spec and ends fixed, deferred (to pipeline work the pre-`done` gate reads, to an external work item, or both) or waived with its reason.

The reason is stated with the rule: an in-progress spec *is* the finding's home, so an item that starts and ends at the same spec would pay a full routing loop for nothing. A task added mid-implementation is surfaced in the `/{project}:implement` completion summary, which lists every disposition task the walk appended.

The rule also states what it does **not** license. `tasks.md` stays a work-tracking artifact and never the durable record, which still lands where [§bug-handling](../../../framework/constitution.md#bug-handling) puts it — a missing requirement becomes a scenario or a spec edit through `/{project}:amend`, with the task as the work item that implements it — and a chore a run finds is fixed in that run, however close to the current work it surfaced.

## Edge Cases

- **The finding is a missing requirement, not a defect.** Still routed to the spec, but through `/{project}:amend` as a scenario or spec edit — the task is what implements it. The durability test in §bug-handling is untouched; a task is a work item, never the record.
- **The finding is a chore.** A chore a run finds is fixed in that run (§bug-handling), so it never reaches the inbox, even when it surfaced in the middle of spec work; only a chore a person logs by hand lives there until it is done. Proximity to the current spec is not the test; ownership is, for where a non-chore finding goes.
- **The finding is about the pipeline's own machinery.** Belongs to no spec, and the standing rule against frontfilling still governs it — it is waived unless it blocks the current work, not relocated into `tasks.md`.
- **No spec is in progress.** There is no in-between tier to route to. No finding a run produces goes to the inbox, whatever the targeted spec's status: it gets a disposition in that run, and the inbox holds only what a person logs with `/{project}:log`.
- **The finding arrives after the spec has closed.** The scope test still applies, but reaching the spec now costs the `done → in-progress` back-edge; that is the intended price and is cheap, not a reason to prefer the inbox.
- **A finding that spans the in-progress spec and another one.** Split it: the part this spec owns becomes a task, and the remainder is fixed, deferred, or waived on its own, so no single item covers work two specs own.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
