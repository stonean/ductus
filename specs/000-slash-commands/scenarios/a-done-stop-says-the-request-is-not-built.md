---
section: "Gate Enforcement"
---

# A-done-stop-says-the-request-is-not-built

## Context

When an operator asks for new work on a spec that is already `done`, the commands that stop on it or show it lead with the bare status, and an agent's reply repeating it reads as "your request is already done". In an adopter project (verified 2026-10-26 at HEAD c0952038) a reply opened "Spec … is `done` … recording as a criterion", and the owner believed the requested work finished when none of it was built. The wording matched `/{project}:amend`'s criterion route after its Confirm target step 5 had displayed the status, so `/{project}:amend` is the primary surface: step 5's display, its Impact display for a `done` spec, and its re-open prompt ("Spec is `done` but the feature directory has un-tracked…"). The stops in `/{project}:implement` step 4, `/{project}:plan`'s Gate ("Spec is `done`. Run `/amend` to capture new work as a scenario.") and `/{project}:clarify`'s Gate table ("… to capture this as a scenario instead.") already differ in wording and fire unconditionally; those commands take only a feature (and `--auto`), so the request text comes from the conversation, not the arguments.

## Behavior

- A command that stops on, or displays, a `done` spec while the operator has asked for new work says first whether that work exists yet ("Not built yet: {request}."), then names the next step.
- `done` is never stated bare on those surfaces: it is qualified by what the spec covers (e.g. "`{slug}` is `done` for what it specifies; this request is not part of it.").
- The message has one shape, stated once in a home every command loads, and each command points at it rather than copying it.
- Whether `/{project}:target`'s "confirm complete" row and `/{project}:status`'s "done (spec is complete)" cell take the same treatment is decided with its reason; 023 AC30 records why those two cells differ, so a change supersedes it explicitly.

## Edge Cases

- A command run on a `done` spec with no new work requested (a re-read, a status check): the qualified form still applies, without the "Not built yet" line.
- A `/{project}:status` change touches `status.md`'s table, the runtime dashboard's rendering and its unit tests; the status parity capture is a skipped placeholder whose golden has no `done` row, so there is no parity fixture to update.
- `/{project}:amend` describes its criterion route only in Purpose, with no step that classifies input as a criterion or records one, though this spec's `criterion-route-after-draft` task is ticked.

## Open Questions

- Where does the one message shape live — the constitution's §spec-lifecycle (via 050) or a reference file every command loads — and do `/{project}:target`'s "confirm complete" row and `/{project}:status`'s "done (spec is complete)" cell change with it, superseding 023 AC30?

## Resolved Questions

*None yet.*
