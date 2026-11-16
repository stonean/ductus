---
next-task: 18
---

# 064 — Pi Host Support Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 17. The bridge closes its server at session_shutdown and never holds pi's process open

- [ ] Implement the behavior described in `scenarios/the-bridge-closes-its-server-at-shutdown.md`
- [ ] `framework/bootstrap/pi/ductus-bridge.ts` registers an idempotent `session_shutdown` handler that stops the child, and unreferences the child and both its pipes, staying stdio-only and dependency-free; this repository's generated `.pi/extensions/ductus.ts` follows
- [ ] `scripts/tests/pi-bridge-harness.mjs` gains a case where `session_shutdown` stops the runtime and a later call respawns it, and one where a Node process that loaded the bridge and made a call exits by itself, which fails against the bridge before this task
- [ ] A real `pi -p` run in a pi-selected scratch adopter prints its answer and exits

- **Done when**: the bridge stops its server at session_shutdown and holds no process open, both harness cases pass and the second failed before the fix, and a real `pi -p` exits after its turn.
