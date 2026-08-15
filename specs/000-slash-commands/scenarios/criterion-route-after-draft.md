---
section: "Command Set"
---

# Criterion-route-after-draft

## Context

Acceptance criteria are the surface `/{project}:implement`'s completion gate verifies semantically — the gate walks each one and marks it at verification time. A criterion that does not exist is a behavior the gate never checks, so a spec can pass its gate on a list that omits work the spec actually shipped.

Criteria are authored at `/{project}:specify` and refined at `/{project}:clarify`. After that, no command writes one:

- `/{project}:clarify` updates criteria (it verifies each is concrete and testable, rewrites vague ones, and flags missing ones) but gates on `draft`. On an `in-progress` spec with zero open questions it stops with "already `in-progress`".
- `/{project}:amend`'s two routes write a question or a scenario plus its task, and its Scope Boundaries state that no other artifact contents are modified.
- `/{project}:plan` gates on `clarified`, so a spec on the reopen cycle fails it.

The state that needs the write is ordinary: a follow-on scenario lands on a reopened spec, its behavior is implemented, and nothing at criteria level verifies it. The workaround is a direct `spec.md` body edit — the same shape as the `append-task`-by-hand workaround that `023-govern-refinement`'s `extend-existing-scenario-task` scenario exists to remove, and the same shape as the gap `046-scenario-open-question-visibility` closed: a reachable pipeline state with no command routing out of it.

The direct edit is legitimate on a non-`done` spec (no back-edge is owed), so this is not a correctness hole. It is a surface hole: the pipeline's own answer to "how do I record this?" is "edit the file yourself", which is the answer the command surface exists to replace.

## Behavior

- A command route exists for adding an acceptance criterion to a spec past `draft`, without reverting the spec to `draft` to reach it.
- The route does not require the spec to carry an open question, and does not manufacture one to unlock a back-edge.
- A criterion added this way lands unchecked. Marking it is the completion gate's job, at verification time — a route that wrote it pre-checked would assert a verification that never ran.
- On a `done` spec the route either takes the documented back-edge or refuses and names the command that does; it does not silently edit a `done` spec's body.

## Edge Cases

- **The criterion duplicates an existing one** — a normalized-whitespace, case-insensitive comparison is the same dedup shape `append-question` applies to `## Open Questions`; a match reports the existing entry rather than appending a near-twin.
- **The spec is `draft`** — `/{project}:clarify` already owns this and the route should not compete with it.
- **The spec is `done`** — see the Behavior note above; the choice between back-edge and refusal is part of the open question.
- **A missing `## Acceptance Criteria` section** — created in template order, the way `append-question` creates a missing `## Open Questions` section, rather than refusing.
- **The criterion describes behavior with no implementation yet** — that is the normal case on a reopened spec, and is exactly why it lands unchecked.

## Open Questions

- Which command owns the route — `/{project}:amend` gaining a third classifier route (question / scenario / criterion), or `/{project}:clarify` accepting a non-`draft` spec for a criteria-only pass? Amend is already the "add to this spec" verb and its classifier surface would absorb a third input shape, but its Scope Boundaries currently forbid other artifact edits; clarify already owns criteria authoring and its verification prose, but its `draft` gate is load-bearing (it is the resolver, not the back-edge entry point, per spec 014) and widening it risks reopening the recovery-path ambiguity that gate closed.
- On a `done` spec, does the route take a back-edge to `in-progress` or refuse and redirect? A new criterion is unverified behavior, which argues for the reopen the scenario route already performs; but §spec-lifecycle defines the `done` back-edge as scenario-triggered, and a criterion is not a scenario, so adding a second trigger to that edge is a lifecycle change rather than a command addition.

## Resolved Questions

*None yet.*
