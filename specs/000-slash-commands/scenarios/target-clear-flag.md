---
section: "Command Set"
---

# Target-clear-flag

## Context

`/ductus:target` (Behavior → Command Set → Utility commands) sets the working feature for the session by writing `.ductus/session.toml`. After a spec advances to `done`, the session still points at the now-completed feature (and optionally scenario), producing awkward `/ductus:status` output ("Target: 000-slash-commands / done / next: done (spec is complete)") and trapping `/ductus:amend` and `/ductus:implement` on stale state until the user manually picks a new target.

The only reset path today is `/ductus:target <other-feature>`, which mutates the session toward a different target rather than clearing it. There is no first-class "no target" state reachable through the command — even though the `dashboard` primitive already reports no `session-target` when there is none and the pipeline view already opens with "No session target." when that's the case and closes with "Run /ductus:target <feature> to select one." (`render-status`, spec 080).

The user-visible gap: closing out a spec leaves the session pointer dangling, and clearing it requires hand-editing `.ductus/session.toml`.

## Behavior

`/ductus:target` accepts a `--clear` flag (mutually exclusive with a feature argument). When set:

- Clear the target block from `.ductus/session.toml`. **Superseded by `022-deterministic-runtime`'s `cli-config-dir-per-contributor` scenario**, which made the clear preserve a recorded `cli-config-dir`: the file is rewritten to hold only that key when one is present, and left empty when none is. As delivered this bullet said "delete the file" unconditionally. The reset state is the same either way — no `feature` remains, so the `dashboard` primitive's documented no-target state, a result with no `session-target`, is what the clear reaches, and there's no separate empty-session format to invent.
- Emit a one-line confirmation: `Session cleared. Run /ductus:target to set a new target.`
- Exit 0.

Mutually exclusive with positional arguments and other flags. Invoking `--clear` alongside a feature argument halts with `/ductus:target: --clear cannot be combined with a feature argument`; alongside a scenario flag (when scenario-targeting is supported), halt analogously.

## Edge Cases

- **Session file already absent.** `--clear` writes an empty session file, still emits the confirmation line and exits 0. Idempotent.
- **Session file present but malformed.** `--clear` resets it cleanly; do not error on stale state. As delivered this read "malformed JSON", naming the pre-consolidation `{cli-config-dir}/{project}-session.json`; the session file has been TOML at `.ductus/session.toml` since `022-deterministic-runtime` moved it (see `framework/migrations/session-file-consolidate.md`).
- **Permission denied on write.** Surface the OS error and exit non-zero — same envelope shape other session-file writes use today.
- **`--clear` combined with feature argument or scenario flag.** Halt with the mutex-violation message above; no session mutation.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
