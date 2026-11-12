---
section: "Pre-flight Checks"
---

# A-first-adoption-with-existing-specs-is-not-an-update

## Context

The spec's Pre-flight Checks section reads an existing spec root as proof of adoption: "If it does, `ductus` is already adopted and the run is an update". `framework/bootstrap/ductus.md` §Pre-flight Checks follows it, reporting "running in update mode", and **Post-Scaffolding Output** in `framework/bootstrap/ductus-procedure.md` then prints the update-mode message. A project that keeps its own `specs/` before it adopts `ductus` is not adopted, so its first run never shows the first-run next steps the spec's Post-Scaffolding Output section requires, `configure` first among them, and the adopter is never told to apply the full permission set. Found by `008-security-rules` task 22's real adopter bootstrap of a first-adoption subject with two feature specs and no prior `ductus`, 2026-10-29.

The missing config file does not mark a first adoption on its own: §Pre-run Migrations step 4 of `framework/bootstrap/ductus-procedure.md` names a legacy adopter that has `specs/` and never wrote a config file.

## Behavior

- A run decides whether the project was adopted before from a signal that only an earlier `ductus` run leaves, not from the spec root's existence: a config file at any tier, the legacy `.governance.toml` included, or any ductus-only artifact — `{spec-root}/templates/spec.md`, `.ductus/constitution.md`, `.govern/constitution.md`, a `{config_dir}/commands/{govern|ductus|<project>}/` directory, or `.githooks/ductus-pre-commit`. Any one of them makes the run an update.
- A first adoption into a project with an existing spec root prints the first-run message and next steps; the update-mode message is for a project adopted before.
- The pre-flight report says which of the two the run is, and names the signal that decided it, so the adopter sees the decision before anything is written.

## Edge Cases

- **A legacy adopter with `specs/` and no config file.** It was adopted before, so it stays an update: `{spec-root}/templates/spec.md` has shipped since `v0.1.0`, so every earlier adoption left it.
- **A project's own root `constitution.md`** is no signal: nothing tells a project's own from a legacy ductus copy by its name (027's scenario `a-projects-own-constitution-is-not-relocated`).
- **A project that built its own `{spec-root}/templates/spec.md`** reads as an update; the pre-flight report names that file as the deciding signal, so the operator sees why.
- **A project whose spec root is renamed.** The decision reads the configured `[paths] specs-root`, as the pre-flight check already does.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Which signal marks an earlier adoption?** Any one of: a config file at any tier, the legacy `.governance.toml` included; `{spec-root}/templates/spec.md`, the marker a no-config legacy adopter still carries; `.ductus/constitution.md` or `.govern/constitution.md`; a `{config_dir}/commands/{govern|ductus|<project>}/` directory; or `.githooks/ductus-pre-commit` (owner, 2026-11-12). The spec templates and `specs/inbox.md` already shipped at `v0.1.0` (`git show v0.1.0:framework/bootstrap/govern.md`), so the templates mark even the oldest adoption. Rejected: a config file alone, which reads a legacy adopter with `specs/` and no config as a first run, against this scenario's first edge case; and the installed agent bootstrap, which `install.sh` places before every run, first adoptions included, so it signals nothing.
