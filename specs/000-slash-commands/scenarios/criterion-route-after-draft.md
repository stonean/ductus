---
section: "Command Set"
---

# Criterion-route-after-draft

## Context

Acceptance criteria are the surface `/{project}:review`'s criteria step verifies semantically — it judges each one and marks it at verification time, and `/{project}:implement`'s completion gate holds `done` while one stands unticked. A criterion that does not exist is a behavior no review checks, so a spec can pass its gate on a list that omits work the spec actually shipped.

Criteria are authored at `/{project}:specify` and refined at `/{project}:clarify`. Before `/{project}:amend` gained the criterion route this scenario specifies, no command wrote one after that:

- `/{project}:clarify` updates criteria (it verifies each is concrete and testable, rewrites vague ones, and flags missing ones) but gates on `draft`. On an `in-progress` spec with zero open questions it stops with "already `in-progress`".
- `/{project}:amend`'s two routes wrote a question or a scenario plus its task, and its Scope Boundaries stated that no other artifact contents were modified.
- `/{project}:plan` gates on `clarified`, so a spec on the reopen cycle fails it.

The state that needs the write is ordinary: a follow-on scenario lands on a reopened spec, its behavior is implemented, and nothing at criteria level verifies it. The workaround is a direct `spec.md` body edit — the same shape as the `append-task`-by-hand workaround that `023-govern-refinement`'s `extend-existing-scenario-task` scenario exists to remove, and the same shape as the gap `046-scenario-open-question-visibility` closed: a reachable pipeline state with no command routing out of it.

The direct edit is legitimate on a non-`done` spec (no back-edge is owed), so this is not a correctness hole. It is a surface hole: the pipeline's own answer to "how do I record this?" is "edit the file yourself", which is the answer the command surface exists to replace.

## Behavior

- A command route exists for adding an acceptance criterion to a spec past `draft`, without reverting the spec to `draft` to reach it.
- The route does not require the spec to carry an open question, and does not manufacture one to unlock a back-edge.
- A criterion added this way lands unchecked. Marking it is `/{project}:review`'s criteria step's job, at verification time — a route that wrote it pre-checked would assert a verification that never ran.
- On a `done` spec the route takes the owed-code back-edge, reopening the spec once `check-owed-work` reports the new criterion; or, when the input corrects a ticked criterion the owner confirms the code bears out, it corrects that criterion's text in place and reopens nothing (spec 084). It does not silently edit a `done` spec's body.

## Edge Cases

- **The criterion duplicates an existing one** — a normalized-whitespace, case-insensitive comparison is the same dedup shape `append-question` applies to `## Open Questions`; a match reports the existing entry rather than appending a near-twin.
- **The spec is `draft`** — `/{project}:clarify` already owns this and the route should not compete with it.
- **The spec is `done`** — the route takes the `done → in-progress` back-edge for a new criterion, which spec 084 states as owed code; the second Resolved Question below settled that, and this bullet said "part of the open question" while the question was still open.
- **A missing `## Acceptance Criteria` section** — created in template order, the way `append-question` creates a missing `## Open Questions` section, rather than refusing.
- **The criterion describes behavior with no implementation yet** — that is the normal case on a reopened spec, and is exactly why it lands unchecked.

## Open Questions

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

  Refusing was additionally ruled out by what followed from it then: nothing
  detected an unchecked criterion on a `done` spec — `check-artifacts`'
  criterion families covered path existence and labels, not verification
  state — so a refused criterion would have been recorded somewhere no gate
  read. Reopening routed it into `/{project}:implement`'s criterion-verification
  step, then the gate that verified criteria. Since 066's
  `criteria-are-verified-in-review`, `/{project}:review`'s criteria step judges
  every criterion and `check-artifacts` reports a `done` spec holding an
  unticked criterion as drifted, so the reopen still routes the criterion to
  its judge.

  Superseded in part (spec 084): §spec-lifecycle now has two back-edges, to `draft` on a new open question and to `in-progress` while code is owed. The derivation above from a third back-edge, *Backward via meaningful body edit*, no longer holds; a new criterion still reopens a `done` spec, because it is an unticked criterion, and the criterion route reopens it once `check-owed-work` reports it.
