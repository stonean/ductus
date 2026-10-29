---
section: "Command Set"
---

# Target-names-the-step-the-gate-would-name

## Context

When `/{project}:target` selects a spec whose status is `in-progress`, its next-action guidance prompts `/{project}:implement` whatever the spec's tasks say. With no unchecked task left, `/{project}:implement` has nothing to walk and goes straight to its completion gate, whose first failure names the real next step — typically `/{project}:review`. The operator is sent through a command whose only output is to name a different one.

## Behavior

- For an `in-progress` spec with at least one unchecked task, `/{project}:target` names `/{project}:implement`, as today.
- For an `in-progress` spec with no unchecked task, `/{project}:target` names the step `/{project}:implement`'s completion gate would name: `/{project}:review` when the review record is missing or stale, `/{project}:analyze` when the analysis is, the step the gate names for any other first failure, and the `done` transition through `/{project}:implement` when the gate passes.
- The step is derived from the same gate evaluation `/{project}:implement` uses (`check-review-gate`), not from a second copy of its ordering, so the two commands cannot disagree.

## Edge Cases

- A `tasks.md` that is absent or in template state on an `in-progress` spec: no unchecked task, so the gate decides.
- A scenario under the spec with unresolved open questions: the gate's own scenario-question check names `/{project}:clarify` or `/{project}:amend`, and `/{project}:target` repeats it.
- `/{project}:target` stays read-only: evaluating the gate writes nothing.
- The markdown-only path: the host walks the same gate order from `/{project}:implement`'s reference.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
