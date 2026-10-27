---
section: "Follow-on scenarios"
---

# An-answer-is-applied-as-chosen

## Context

On 2026-10-20 an owner question about `BE-AUTHN-018` against saved recovery codes existed only because the text applied for `008-security-rules`' `BE-AUTHN-022` answer departed from the option the owner chose, and a review caught a second departure: NIST's code-plus-authenticator route narrowed to the password. The item was logged to `specs/inbox.md` that day and routed here. §recommendations says how a decision is put to the owner and says nothing about applying the answer, so nothing stopped the applied text from drifting from the choice.

## Behavior

- An owner's answer is applied as the chosen option reads.
- A refinement the sources suggest, narrower, broader or reshaped, goes back to the owner as its own question, worked out per §recommendations, before it lands.
- The rule is not part of the one-at-a-time default a project MAY replace in its `AGENTS.md`; it binds every project.

## Edge Cases

- **Wording the destination needs.** Fitting the chosen option to its destination's grammar or size budget is not a refinement while what the option requires is unchanged.
- **A refinement found after the answer landed.** It is asked the same way, and the landed text stands until the owner answers.
- **A change the chosen option reaches through a shared surface.** When applying an option to one consumer widens what a second consumer does, the widening is a refinement and is asked; on 2026-10-27 extending a fold's link rewrite reached `/{project}:consolidate` through their shared primitive, and was asked before it landed.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
