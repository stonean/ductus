---
section: "Follow-on scenarios"
---

# Findings-route-by-scope

## Context

While implementing a spec, an agent finds a bug or an omission in the spec it is currently implementing — a defect in what that spec built, a gap in what it specified, a case its scenarios missed. The reflex the framework had trained was capture. [§brownfield-inbox](../../../framework/constitution.md#brownfield-inbox)'s Automatic issue capture says findings MUST reach `specs/inbox.md` rather than be dropped, and the single exemption it carried covered an issue inside the *current task's* scope, which is fixed as part of the task and not logged.

Nothing covered the tier between those two, and that tier holds the most common case there is. An agent working a spec is looking hardest at exactly the surface that spec owns, so most of what it notices belongs to that spec. Capturing those findings is a loop with no destination: the item is appended, re-read on every intervening `/{project}:groom` pass, walked through the bug decision tree, and routed back to the spec that was open the whole time. The user named the cost directly — *"adding it to the inbox.md creates additional work."*

The rule was first written as an `AGENTS.md` entry, which is the failure this spec exists to correct. Contributor-side practice does not propagate to adopting projects, and this rule governs implementation behavior in every project running the pipeline, so its canonical text belongs in the constitution with `AGENTS.md` keeping only a pointer (§Promotion mechanism).

## Behavior

§brownfield-inbox's Automatic issue capture carries a bullet stating that scope decides the destination and that the inbox is only for findings with no home. It names three tiers explicitly: a finding inside the current **task** is fixed in the task; a finding inside the current **spec** but outside the task is written to that spec's `tasks.md` as a new unchecked task; a finding outside the spec is captured to the inbox.

The reason is stated with the rule, because the rule reads as an exception to capture unless the reader can see why it is not one: an in-progress spec *is* the home the inbox exists to find, so an item that starts and ends at the same spec pays a full routing loop for nothing. Visibility is unchanged either way — a task added mid-implementation is surfaced in the `/{project}:implement` completion summary the same way a capture is, which is the backstop the capture rule relies on.

The bullet also states what it does **not** license. `tasks.md` does not become a second capture queue: the durable record still lands where [§bug-handling](../../../framework/constitution.md#bug-handling) puts it — a missing requirement becomes a scenario or a spec edit through `/{project}:amend`, with the task as the work item that implements it — and a chore with no feature home stays an inbox item however close to the current work it surfaced.

The section's closing sentence, which promised that discoveries reach the inbox, now names both destinations, so the section does not contradict its own new bullet.

## Edge Cases

- **The finding is a missing requirement, not a defect.** Still routed to the spec, but through `/{project}:amend` as a scenario or spec edit — the task is what implements it. The durability test in §bug-handling is untouched; a task is a work item, never the record.
- **The finding is a chore.** Project maintenance with no feature home goes to the inbox even when it surfaced in the middle of spec work. Proximity to the current spec is not the test; ownership is.
- **The finding is about the pipeline's own machinery.** Belongs to no spec, and the standing rule against frontfilling still governs it — it is discarded unless it blocks the current work, not relocated into `tasks.md`.
- **No spec is in progress.** There is no in-between tier to route to, so the inbox is the destination and capture behaves exactly as before.
- **The finding arrives after the spec has closed.** The scope test still applies, but reaching the spec now costs the `done → in-progress` back-edge; that is the intended price and is cheap, not a reason to prefer the inbox.
- **A finding that spans the in-progress spec and another one.** Split it: the part this spec owns becomes a task, the remainder is captured. A single item covering both would be routed by `/{project}:groom` to the spec that is already handling half of it.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
