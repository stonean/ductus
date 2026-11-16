---
section: "Design Decisions"
---

# The-bridge-closes-its-server-at-shutdown

## Context

The bridge `.pi/extensions/ductus.ts` (source `framework/bootstrap/pi/ductus-bridge.ts`) spawns `ductus mcp` as a child process with piped stdin and stdout when pi loads it, to list the tools it registers, and keeps that child for every later call. It registers a `session_start` handler and nothing else. pi's extension documentation, read from pi 1.5.2's installed `docs/extensions.md` (§Long-lived resources and shutdown), requires an extension to "register an idempotent `session_shutdown` handler to close any session-scoped resources you start", and says print mode "exits automatically when all prompts are processed".

Found by 012 task 12's real pi run on 2026-11-16, in a scratch adopter upgraded to `main` with pi selected: `pi -p --approve --no-session "Reply with the single word ok."` printed nothing and was still running at 90 s. With `--no-extensions` it printed `ok` and exited 0 in 1.2 s, and with `--no-extensions -e .pi/extensions/ductus.ts`, the bridge alone, it printed `ok` and was still running at 60 s. The live child and its open pipes keep pi's Node process from exiting, so every print-mode run in a pi adopter hangs once its turn is done.

## Behavior

- **The bridge stops its server at `session_shutdown`.** Its handler kills the child, if one runs, and fails any call still waiting on it, as a death of the runtime does. It is idempotent, and a later call respawns the server as after a death.
- **The child never holds pi's process open.** The bridge unreferences the child and both its pipes, so a print-mode run exits once its prompts are done, and so does an invocation that never starts a session. When pi exits the child's stdin closes, and `ductus mcp` stops.
- **Interactive and RPC use are unchanged.** pi's own event loop keeps the process alive there, so a call made at any point still reaches the child.

Decided by the owner on 2026-11-16, over a waiver: no 064 criterion names print mode, but every pi adopter's `pi -p`, in a script or CI, hung.

## Edge Cases

- **A call in flight at shutdown** settles with the shutdown's error rather than waiting out its timeout.
- **A shutdown with no child running** (the runtime never started, or died and no call has respawned it) finds nothing to stop and does nothing; one before any tool call still stops the child load spawned.
- **A call queued behind the one in flight** fails with the same error, and respawns nothing; a call issued after the shutdown respawns the runtime.
- **pi exiting without `session_shutdown`**: the unreferenced child no longer holds the process, and it stops at its stdin's end of file.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
