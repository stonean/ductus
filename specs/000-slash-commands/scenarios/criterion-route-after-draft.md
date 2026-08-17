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

- **The criterion duplicates an existing one** — a normalized-whitespace, case-insensitive comparison is the same dedup shape `append-question` applies to `## Open Questions

*None — see Resolved Questions.*

## Resolved Questions

- **Which command owns the route?** **`/{project}:amend`, as a third classifier
  route (question / scenario / criterion).** Resolved 2026-08-17.

  The deciding property is which command already performs a back-edge mutation
  on a classified input, and that is amend by its own contract: it documents
  the question route (`clarified` / `planned` / `in-progress` → `draft`) and
  the scenario route (`done` → `in-progress`) as writes it performs itself.
  `clarify.md` disclaims the role in its own body — *"This command is the
  resolver, not the back-edge entry point"* — and its non-`draft` handling is a
  recovery branch for *"a state that should not occur via normal usage"*.
  Routing criteria through clarify would widen a gate its documentation calls
  load-bearing and re-blur the split spec 014 drew deliberately.

  The objection recorded in the question — that amend's Scope Boundaries forbid
  other artifact edits — is real but narrow: amend already writes questions,
  scenarios, tasks, status and the session file, so criteria extend that list
  rather than opening a new class of write.

- **On a `done` spec, back-edge or refuse?** **Back-edge to `in-progress` — and
  no lifecycle change is needed, because the edge already exists.** Resolved
  2026-08-17.

  The question was posed on the belief that *"§spec-lifecycle defines the `done`
  back-edge as scenario-triggered"*. It does not. §spec-lifecycle states that
  **three** back-edges exist, and the third is *"Backward via meaningful body
  edit — `done` → `in-progress` when any artifact under `specs/{feature}/` is
  edited meaningfully… Anything else — **new scope**, changed semantics,
  factual corrections, restructuring — is a meaningful edit and triggers the
  back-edge **via the same `/amend` flow used for scenarios**."*

  A new acceptance criterion is a body edit, it is new scope, and it is none of
  the three enumerated mechanical exemptions. The edge therefore already
  applies; the criterion route uses it rather than adding a second trigger.
  Exemption **(c)** makes the boundary explicit in the other direction: assigning
  an `AC{n}` *label* is mechanical *"because an identifier names a requirement
  without stating one"*, so the constitution had already separated a criterion's
  label from its text.

  Refusing was additionally ruled out by what follows from it: nothing detects
  an unchecked criterion on a `done` spec — `check-artifacts`' criterion
  families cover path existence and labels, not verification state — so a
  refused criterion would be recorded somewhere no gate ever reads. Reopening
  routes it into `/{project}:implement`'s criterion-verification step, which is
  the gate that actually verifies criteria.
