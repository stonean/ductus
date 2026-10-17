---
section: "Gate Enforcement"
---

# Implement Stops Below Planned

## Context

The constitution's Readiness Check requires a spec to be `planned` before implementation begins, and AC3 requires `implement` to enforce gates matching the spec lifecycle. As delivered, `implement` opened with a gate that stopped a spec below `planned` and named the command to run instead. The gate was dropped when 022 rewrote `implement` under the parseable conventions (`feaf7899`), and nothing restored it. A spec can reach `implement` below `planned` with a `tasks.md` on disk — `/{project}:amend` reverts an in-progress spec to `draft` when it records a question — and without the gate `implement` would walk its tasks while the questions stand.

## Behavior

- Before walking any task, `implement` reads the spec's status. When it is `draft`, `clarified` or `done`, it stops, changes nothing, and reports:
  - `draft` → "Spec has unresolved open questions. Run `/{project}:clarify` first."
  - `clarified` → "No plan exists. Run `/{project}:plan` first."
  - `done` → "Spec is `done`. Run `/{project}:amend` to capture new work as a scenario."
- A `planned` spec moves to `in-progress` with no prompt, as before, and an `in-progress` spec continues; the walk proceeds for both.
- The stop sits in step 4 of `framework/commands/implement.md`, beside the transition it guards, so no step number moves.

## Edge Cases

- **A `draft` spec with every task checked** still stops: the open questions, not the task list, decide readiness.
- **A spec whose frontmatter carries no `status`** is not this gate's case: it fails the spec-file schema, which `/{project}:analyze` reports as a hard failure.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
