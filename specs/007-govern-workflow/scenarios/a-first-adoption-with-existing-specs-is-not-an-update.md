---
section: "Pre-flight Checks"
---

# A-first-adoption-with-existing-specs-is-not-an-update

## Context

The spec's Pre-flight Checks section reads an existing spec root as proof of adoption: "If it does, `ductus` is already adopted and the run is an update". `framework/bootstrap/ductus.md` §Pre-flight Checks follows it, reporting "running in update mode", and **Post-Scaffolding Output** in `framework/bootstrap/ductus-procedure.md` then prints the update-mode message. A project that keeps its own `specs/` before it adopts `ductus` is not adopted, so its first run never shows the first-run next steps the spec's Post-Scaffolding Output section requires, `configure` first among them, and the adopter is never told to apply the full permission set. Found by `008-security-rules` task 22's real adopter bootstrap of a first-adoption subject with two feature specs and no prior `ductus`, 2026-10-29.

The missing config file does not mark a first adoption on its own: §Pre-run Migrations step 4 of `framework/bootstrap/ductus-procedure.md` names a legacy adopter that has `specs/` and never wrote a config file.

## Behavior

- A run decides whether the project was adopted before from a signal that only an earlier `ductus` run leaves, not from the spec root's existence.
- A first adoption into a project with an existing spec root prints the first-run message and next steps; the update-mode message is for a project adopted before.
- The pre-flight report says which of the two the run is, so the adopter sees the decision before anything is written.

## Edge Cases

- **A legacy adopter with `specs/` and no config file.** It was adopted before, so it stays an update.
- **A project whose spec root is renamed.** The decision reads the configured `[paths] specs-root`, as the pre-flight check already does.

## Open Questions

- Which signal marks an earlier adoption — a file only a ductus run writes, such as `.ductus/constitution.md` or its `.govern/` predecessor, an installed agent bootstrap or command directory, a config file at any tier, or a combination — so that a legacy adopter with `specs/` and no config file still reads as an update?

## Resolved Questions

*None yet.*
