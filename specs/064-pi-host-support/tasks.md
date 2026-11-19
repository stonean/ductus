---
next-task: 20
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

- [x] Implement the behavior described in `scenarios/colon-form-commands-run-on-pi.md`
- [x] `framework/bootstrap/pi/ductus-bridge.ts` registers `/<project>:<name>` for each installed `.pi/prompts/<project>-<name>.md`, forwarding `/<project>-<name> <args>` through `pi.sendUserMessage` with `expandPromptTemplates: true`, and `/<project>:status` and `/<project>:help` with the existing handlers, staying stdio-only and dependency-free; this repository's generated `.pi/extensions/ductus.ts` follows
- [x] `scripts/tests/pi-bridge-harness.mjs` gains cases for an alias per installed template forwarding its arguments, `:status` and `:help` sharing the handlers, no alias for a name outside the grammar, and none when no command registers, each failing against the bridge before this task or under a named mutant of the bridge, recorded in the commit, since `scripts/mutation/run.py` mutates only `runtime/`
- [x] `framework/bootstrap/ductus.md` §Derived values names the colon alias in Pi's Invocation row, with `govern.md` kept byte-identical; 071's Resolved Question on the colon form points at the scenario; `runtime/CHANGELOG.md` records the alias
- [x] A real `pi` run in a pi-selected scratch adopter runs a colon-form command

- **Done when**: the bridge registers a colon-form alias for each installed command and for status and help, each harness case passes and fails under its mutant, and a real pi run in a scratch adopter runs a colon-form command

## 19. Review round: the findings of 064's interrupted review at 8e4dc677, fixed before the review runs again

- [x] `scripts/tests/pi-bridge-harness.mjs`'s grammar case cannot see a dropped `.md` check: `harness-x.txt` slices to `x.`, which `COMMAND_NAME` rejects anyway (`QUAL-TEST-001`, SHOULD, from the review's B pass). Add an extensionless template such as `harness-noext`, which would become the alias `harness:no`, and prove a mutant dropping `.endsWith(".md")` fails it, applied to a copy of the bridge as task 18's nine were
- [x] Contract break against the scenario's "An alias runs as typing" Behavior: typed while Pi is compacting, an alias runs at once, since Pi runs an extension command immediately during compaction, and its `sendUserMessage` throws "Cannot submit a prompt while compaction is in progress", where a typed hyphen command is queued (pi 1.5.2, `interactive-mode.js` "Queue input during compaction" and `agent-session.js` `prompt`). Fix it in the bridge if a command handler can wait out compaction (`ExtensionCommandContext.waitForIdle` awaits `isIdle`; check whether `isIdle` covers compaction), else state it as an edge case of `scenarios/colon-form-commands-run-on-pi.md`
- [x] `framework/bootstrap/ductus-procedure.md` says twice that printing the colon form to a Pi adopter names a command that does not exist on their agent, which task 18 made false; correct both, and sweep `framework/` and `docs/` by meaning for the same claim
- [x] `spec.md` D3 calls the bridge "a ~150-line extension", and it is 531 lines at `8e4dc677`: drop or restate the count. `README.md`'s invocation paragraph names only `/my-project-specify` for Pi: add the colon alias
- [x] Then run 064's review again in full, since the interrupted one wrote no `review.md`, and its analysis. Carry these by rule, each waived with its reason: the `:status` and `:help` handler bodies duplicating each other (080's shape, no third copy added); `AGENTS.md`'s Pi gotcha naming only status and help (incomplete, not false); and a `[host] project` holding a space, which the hyphen form already cannot run (the value comes from the project's own committed config). Of 064's stored decisions, two no longer stand and expire: `ductus.md` step 4b now names both managed-block merges, and Agent Selection now gives dropped hosts' trees a notice

- **Done when**: each item is fixed or stated, the new harness case fails under its mutant, the harness passes, and 064's review and analysis have run again on the fixed tree
