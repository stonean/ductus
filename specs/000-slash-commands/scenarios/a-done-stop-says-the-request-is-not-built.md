---
section: "Gate Enforcement"
---

# A-done-stop-says-the-request-is-not-built

## Context

When an operator asks for new work on a spec that is already `done`, the commands that stop on it or show it lead with the bare status, and an agent's reply repeating it reads as "your request is already done". In an adopter project (verified 2026-10-26 at HEAD c0952038) a reply opened "Spec … is `done` … recording as a criterion", and the owner believed the requested work finished when none of it was built. The wording matched `/{project}:amend`'s criterion route after its Confirm target step 5 had displayed the status, so `/{project}:amend` is the primary surface: step 5's display, its Impact display for a `done` spec, and its re-open prompt ("Spec is `done` but the feature directory has un-tracked…"). The stops in `/{project}:implement` step 4 and `/{project}:plan`'s Gate ("Spec is `done`. Run `/{project}:amend` to capture new work as a scenario."), which match word for word, and `/{project}:clarify`'s Gate table ("… to capture this as a scenario instead.") fire unconditionally; those commands take only a feature (and `--auto`), so the request text comes from the conversation, not the arguments.

## Behavior

- A command that stops on, or displays, a `done` spec while the operator has asked for new work says first whether that work exists yet ("Not built yet: {request}."), then names the next step.
- `done` is never stated bare on those surfaces: it is qualified by what the spec covers (e.g. "`{slug}` is `done` for what it specifies; this request is not part of it.").
- The message has one shape, stated once in `framework/constitution.md` §spec-lifecycle, which every command already loads, and each command that stops on or displays a `done` spec points at it rather than copying it: `/{project}:amend`, `/{project}:clarify`, `/{project}:plan`, `/{project}:implement` and `/{project}:target`.
- `/{project}:target`'s "confirm complete" row takes the qualified form ("complete for what it specifies; run `/{project}:amend` to record new work"), superseding 023 AC30 for that row; `/{project}:status`'s "done (spec is complete)" cell keeps its wording, as AC30 decided, because the status view names no request.

## Edge Cases

- A command run on a `done` spec with no new work requested (a re-read): the qualified form still applies, without the "Not built yet" line. `/{project}:status` is the exception, since it is a view of every spec with no request in hand.
- `/{project}:status`'s cell is the runtime constant `COMPLETE` in `runtime/src/primitives/pipeline_view.rs`, quoted by done spec 080's data model, AC24 and plan, so changing it would reopen 080; it does not change.
- `/{project}:amend` describes its criterion route only in Purpose, with no step that classifies input as a criterion or records one, though this spec's `criterion-route-after-draft` task is ticked.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Where does the one message shape live, and do `/{project}:target`'s row and `/{project}:status`'s cell change with it?** In `framework/constitution.md` §spec-lifecycle, the document every command already loads, as one bullet the five commands point at; `/{project}:target`'s "confirm complete" row takes the qualified form, superseding 023 AC30 for that row, and `/{project}:status`'s cell does not change (owner, 2026-11-12). The constitution is at 49,990 of its 50,000 bytes at `ab2ad085`, so the bullet lands with a compensating cut, measured by `scripts/lint-governing-size.sh`. Rejected: changing the status cell too, which changes the runtime constant `COMPLETE` and the data model, AC24 and plan of done spec 080 that quote it, reopening 080; and a new reference file every command reads, which needs a manifest row and a real adopter bootstrap, and is a file no other step loads, where the constitution is one every command holds.
