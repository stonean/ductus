---
section: "Behavior"
---

# Family-17-contract-binding

## Context

Family 17 (host-namespace parity) reproduces `Host::load`'s namespace resolution in shell so it can compare the resolved slash-command namespace against what is installed on disk. That means it depends on four contracts it does not own: the `[host] project` key, the `commands` / `command` subdirectory pair, the set of agent config dirs, and the new-wins config resolution order.

All four were bare literals — no generated binding, no reference to the source, no test exercising the real resolver. 026's own review flagged it under `QUAL-GROUND-001` on 2026-08-02: rename the key, add a fifth agent, or introduce a third layout, and the family keeps exiting 0 while silently checking the wrong thing. A drift-detector that has itself drifted is worse than no check, because a passing run reads as assurance.

The review offered three resolutions. Registering the literals in Family 6's tracked list was rejected on inspection: Family 6 emits no findings at all, so that would be documentation guarding a check that does nothing — the same discipline-dependence the finding objects to. Accepting as-is was rejected because the acceptance would stay implicit, which is the specific thing the finding asks to change.

## Behavior

**The agent config dirs are derived, not listed.** Family 17 reads them from the `config_dir` column of the **Agent Registry** table in `framework/bootstrap/ductus.md` — the canonical source per the constitution's canonical-sources map. Adding a fifth agent to the registry extends the family automatically; nothing has to be remembered.

**A failed derivation is a finding, not a fallback.** When the table yields no config dirs — renamed, restructured, or removed — the family emits a finding naming that and exits non-zero, rather than falling back to a built-in list and passing. This is the distinction the family exists to enforce, applied to itself: *could not check* is reported, never rendered as *checked and clean*.

**The three single-site contracts are asserted against their source.** The `commands` / `command` pair and the `[host] project` key are asserted present in `runtime/src/host.rs`; the new-wins config order is asserted present in `runtime/src/schema/paths.rs`. A failed assertion emits a finding and stops the family, because every comparison downstream of a stale mirror is meaningless.

**The preferred long-term fix is recorded, not taken.** Exposing the resolved namespace from the runtime — so the shell consumes it rather than reproducing it — removes the duplication entirely instead of guarding it. That is a new runtime surface and is deliberately out of scope here; the suggested-fix text in each assertion names it so the next reader sees the better answer.

## Edge Cases

- A registry table whose column order changes: the derivation reads `config_dir` by position, so a reordering yields wrong values rather than none. The assertions do not cover this; it is the residual the runtime-exposed namespace would close.
- An agent registered with a duplicate `config_dir`: the derived list is de-duplicated, so two agents sharing a directory are compared once.
- A config dir present in the registry but absent from the repo is skipped, unchanged from before — the family compares what is installed, and nothing installed is not a defect.
- The assertions are substring checks against source files, not a parse. They catch a rename or a deletion, which is the failure mode observed; they do not catch a semantic change that preserves the literal.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
