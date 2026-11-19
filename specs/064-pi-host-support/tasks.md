---
next-task: 19
---

# 064 — Pi Host Support Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 17. The bridge closes its server at session_shutdown and never holds pi's process open

- [x] Implement the behavior described in `scenarios/the-bridge-closes-its-server-at-shutdown.md`
- [x] `framework/bootstrap/pi/ductus-bridge.ts` registers an idempotent `session_shutdown` handler that stops the child, and unreferences the child and both its pipes, staying stdio-only and dependency-free; this repository's generated `.pi/extensions/ductus.ts` follows
- [x] `scripts/tests/pi-bridge-harness.mjs` gains a case where `session_shutdown` stops the runtime and a later call respawns it, and one where a Node process that loaded the bridge and made a call exits by itself, which fails against the bridge before this task
- [x] A real `pi -p` run in a pi-selected scratch adopter prints its answer and exits

- **Done when**: the bridge stops its server at session_shutdown and holds no process open, both harness cases pass and the second failed before the fix, and a real `pi -p` exits after its turn.

## 18. Colon-form command references run on Pi through bridge aliases

- [ ] Implement the behavior described in `scenarios/colon-form-commands-run-on-pi.md`
- [ ] `framework/bootstrap/pi/ductus-bridge.ts` registers `/<project>:<name>` for each installed `.pi/prompts/<project>-<name>.md`, forwarding `/<project>-<name> <args>` through `pi.sendUserMessage` with `expandPromptTemplates: true`, and `/<project>:status` and `/<project>:help` with the existing handlers, staying stdio-only and dependency-free; this repository's generated `.pi/extensions/ductus.ts` follows
- [ ] `scripts/tests/pi-bridge-harness.mjs` gains cases for an alias per installed template forwarding its arguments, `:status` and `:help` sharing the handlers, no alias for a name outside the grammar, and none when no command registers, each failing under its mutant in a list under `scripts/mutation/`
- [ ] `framework/bootstrap/ductus.md` §Derived values names the colon alias in Pi's Invocation row, with `govern.md` kept byte-identical; 071's Resolved Question on the colon form points at the scenario; `runtime/CHANGELOG.md` records the alias
- [ ] A real `pi` run in a pi-selected scratch adopter runs a colon-form command

- **Done when**: the bridge registers a colon-form alias for each installed command and for status and help, each harness case passes and fails under its mutant, and a real pi run in a scratch adopter runs a colon-form command
