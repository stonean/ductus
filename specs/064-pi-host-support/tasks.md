# 064 — Pi Host Support Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## Phase A — Runtime and the 022 back-edge

## 1. Add the third command-file candidate

- [x] `runtime/src/host.rs`: append `{cli-config-dir}/prompts/{project}-{name}.md` as the **last** entry of `Host::command_file_candidates`, after the plural and singular shapes (plan §D3)
- [x] Unit test: a session fixture recording `cli-config-dir = .pi` yields the candidate list ending in the pi shape; the two pre-existing candidate tests pass unmodified
- [x] `write-session` doc comment: the `cli-config-dir` enumeration gains `.pi` (doc sync, no behavior)

- **Done when**: `cargo test --release --locked` is green with the new fixture test, the candidate order assertion holds, and the pre-existing tests are byte-unchanged.

## 2. Open the 022 back-edge and record the candidate

- [x] `set-status` 022 `done → in-progress`
- [x] Create `specs/022-deterministic-runtime/scenarios/the-pi-command-candidate.md` (via `create-scenario`), back-linking 064 per §cross-spec-impact, stating the third candidate shape and the append-last order rule
- [x] Append the matching task to 022's `tasks.md` (scenario→task mapping family stays clean)
- [x] 022's `data-model.md`: record the third candidate shape in the command-resolution contract

- **Done when**: 022 is `in-progress`, the scenario and task exist and cross-reference each other, and the data-model carries the candidate.

## 3. Probe pi's `registerTool` schema acceptance

- [x] Throwaway two-tool probe in `/tmp`: a minimal pi extension registering one tool whose `parameters` is a plain JSON-Schema object (no typebox), run under `pi -e` + non-interactive run; observe whether pi accepts the schema and exposes the tool to the model
- [x] If the plain object is rejected: confirm the typebox wrap path (import from pi's extension-available `typebox`) accepts the same server-sourced schema
- [x] Record the outcome in plan §D2a (the probe-result placeholder), with the observed pi version

- **Done when**: plan §D2a states which of the two paths the bridge takes, grounded in the observed probe output — not in a guess.

## Phase B — Framework surface

## 4. Author the bridge extension

- [x] Create `framework/bootstrap/pi/ductus-bridge.ts` per plan §D2: lazy spawn of `.ductus/bin/ductus mcp`, hand-rolled MCP-over-stdio JSON-RPC (`initialize` / `tools/list` / `tools/call`), `pi.registerTool` per server-listed tool under `ductus__<name>` with the server's `inputSchema` passed through (via the path §D2a settled), respawn on exit, missing-pointer/binary error envelope naming the file and pointing at `/ductus`, one-time load notice — zero npm dependencies
- [x] Lint/parse check: the file is loadable TypeScript (no build step; `node --check`-equivalent or the smoke test's load itself is the proof)

- **Done when**: the file exists, imports nothing beyond pi's extension API surface, and every §D2 behavior is present in the source.

## 5. Author `configure/pi.md` and state the generator bound

- [x] Create `framework/bootstrap/configure/pi.md` per plan §D4: verify `.pi/extensions/ductus.ts` against the upstream bridge (overwrite on divergence), report the no-permission-settings fact and the trust requirement
- [x] `scripts/gen-configure-mcp.sh`: header names the pi exclusion with its reason (MCP-permission blocks only; pi has none)

- **Done when**: `configure/pi.md` is a well-formed command source (frontmatter + procedure), the generator still passes its existing tests, and its header states the pi exclusion.

## 6. Bootstrap prose: registry, derived values, MCP registration, Pi layout

- [x] `framework/bootstrap/ductus.md`: Agent Registry `pi` row (plan §D6); §Derived values `pi` column; §MCP registration `pi` row (target `.pi/extensions/ductus.ts`, scope `project-local` (gitignored), mechanism `write-file`) + the mechanism-note prose
- [x] `### Pi layout` scaffolding section: the sixteen command rows + configure row → `.pi/prompts/{project}-<name>.md` (standard substitutions), the bridge row → `.pi/extensions/ductus.ts` (no substitution), the `.pi/` gitignore-block line, the completion-message trust reminder
- [x] §Permission Setup pi no-op branch; State B / §MCP wiring pi target naming; `ductus` self-install / Self-update / Post-Write Integrity / Placeholder Substitution / step-1 / step-7 enumerations gain the pi paths (all verbatim byte-compare)
- [x] `framework/bootstrap/ductus-procedure.md` (archive half): next-steps invocation list, pinned-divergent install-path row, Directory Creation gain the pi shapes
- [x] `cp framework/bootstrap/ductus.md framework/bootstrap/govern.md` (Family 21 byte-identity)

- **Done when**: every layout-keyed section in both bootstrap files carries a `pi` branch consistent with §Derived values, `cmp framework/bootstrap/ductus.md framework/bootstrap/govern.md` is silent, and a derived-value walk (command/skill path, invocation, install path, settings file, permission shape, native rules file, cleanup glob) returns the plan §D6 values for `pi`.

## 7. `install.sh` pi arm

- [x] `pi)` arm: `dest=".pi/prompts/ductus.md"`, verbatim copy, **no settings seed** (stated in-arm with the rationale: pi has no permission-gating settings)
- [x] Usage comment and the unknown-agent error line name `pi`
- [x] Smoke-run the arm in a `/tmp` scratch repo: it installs the bootstrap file and writes nothing else

- **Done when**: `sh install.sh -s pi` in the scratch repo yields exactly `.pi/prompts/ductus.md` byte-identical to the payload and no settings file.

## Phase C — Audits and generator

## 8. Audit families 14 / 15 / 17 learn the pi shape

- [x] Family 14 (`installer-registry-parity.sh`): `pi → {config_dir}/prompts/ductus.md` layout branch; the settings-seed parity check asserts the pi arm writes no settings file (absence assertion, per plan §D5)
- [x] Family 15 (`runtime-probe-parity.sh`): pi skipped by name on stderr; header states the bound
- [x] Family 17 (`host-namespace-parity.sh`): `.pi/prompts/{ns}-*.md` recognized as an installed namespace
- [x] `scripts/audit/README.md`: the three family descriptions updated to carry the pi shapes

- **Done when**: all three families pass over this repo's pi surface (task 10's dogfood), each family's stderr/behavior matches its README description, and the families' own test suite (if any) is green.

## 9. Generator second pass and the dogfooded pi surface

- [x] `scripts/gen-claude-commands.sh`: second output pass — flat `.pi/prompts/ductus-<name>.md` (`{project}` → `ductus`, `{cli-config-dir}` → `.pi`) for every `framework/commands/*.md` + the `configure/pi.md` row, plus the verbatim bridge copy to `.pi/extensions/ductus.ts`; `--check` covers both (the prune loop's `expected` list gains the pi files — no hand-maintained orphans)
- [x] `.gitignore`: `.pi/*` with negations for `.pi/prompts/` and `.pi/extensions/ductus.ts`, mirroring the `.claude/*` / `!.claude/commands/` convention
- [x] Run the generator and commit the generated `.pi/` surface

- **Done when**: `scripts/gen-claude-commands.sh --check` passes, `.pi/prompts/` carries the full command set byte-consistent with the sources, and `git status` shows the committed dogfood surface with nothing pi-related untracked.

## Phase D — Prose sweep

## 10. Constitution, README, AGENTS.md, docs

- [x] `framework/constitution.md` §runtime-host-integration: the pi spelling in full (plan §D7) — `ductus__<verb>-<noun>` via the project's `.pi/extensions/ductus.ts` bridge; no built-in MCP; the server stays the single source for names and schemas
- [x] `README.md`: agent list gains Pi; §Installing (per agent) accepts `pi`; the invocation paragraph names `/{project}-<name>` for Pi
- [x] `AGENTS.md`: the live registry enumeration updates to five agents across four layouts; one new entry records the bridge architecture (never "fix" pi by registering an MCP server; keep the bridge dependency-free)
- [x] `docs/slash-commands.md` (and any `framework/commands/help.md` agent-shape claim): audited; pi added where agents or invocation forms are enumerated
- [x] Anti-claim sweep: grep the changed set for "registers MCP"-shaped and namespace-directory claims about pi (the spec §D7 anti-claim)

- **Done when**: a grep for the pi shapes over `README.md`, `docs/`, `AGENTS.md`, `framework/constitution.md` shows the new spellings present and no stale four-agent enumeration survives in a live-claim position.

## Phase E — Verification and release

## 11. In-repo smoke test with the real pi binary

- [x] With the task-9 surface committed: run `pi` non-interactively in this repo (`--approve`), invoking a `/{project}-…` prompt-template command and observing: the command resolves from `.pi/prompts/`, the `ductus__*` tools are registered from `.pi/extensions/ductus.ts`, a live tool call round-trips (e.g., a read-only primitive), and `ductus exec <command>` resolves the `.pi/prompts/` candidate through the session's `cli-config-dir = .pi`
- [x] Record the result in plan §D9 (the smoke-test-record placeholder), deviations included with their resolution
- [ ] Re-run the smoke test after `991956e2`, which rewrote the bridge's lifecycle (spawn at load, handshake per child, respawn, failure notice), and record it in plan §D9 beside the first run — `scripts/tests/pi-bridge-harness.mjs` covers the transport, not pi's own loader

- **Done when**: plan §D9 carries the recorded result with no unresolved deviation — AC14's evidence.

## 12. Reviews and analysis: 022 first, then 064

- [x] 022: `/{project}:review` (five passes over its changed-since window; the standing truthful-`examined` disposition if the window is too large) and `/{project}:analyze`; all task blocks checked; `done` transition
- [ ] 064: every AC verified against the tree (AC14 via §D9's record), `/{project}:review` and `/{project}:analyze`; `done` transition

- **Done when**: both specs are `done` with current, non-blocking reviews and current analyses, and `check-review-gate` passes for both.

## 13. Release: the next upstream minor

Reopened 2026-10-05: the `0.53.0` bump and the `ductus-v0.53.0` tag these items recorded were made on the contributor's fork (greghaygood/ductus) only. This repository's `0.53.0` is a different, earlier release, and the pi change has not been released here.

- [ ] Bump all three version sites to the next minor (root `version`, `runtime/Cargo.toml`, `runtime/CHANGELOG.md`, moving the `[Unreleased]` pi entries under the new heading)
- [ ] One `cargo build --release --offline` to refresh `Cargo.lock`, then confirm `--locked` succeeds
- [ ] Commit the version bump; `/{project}:review` 064 at that HEAD (the review covers the release commit); commit the review
- [ ] `scripts/audit/run-all.sh` **after** the commit; on green, `git tag ductus-v<minor>` at the release commit and push the tag
- [ ] Watch the run: `gh run list --json workflowName,status,conclusion,headSha`; `in_progress` + `release not found` is wait, not outage; a `failure` conclusion is the outage (read the failed job, never delete-and-re-tag an in-flight run)

- **Done when**: `ductus-v<minor>` is a published release (assets + crates.io), the three version sites agree, and every workflow row for the sha is read and green.

## 14. Implement scenario: pi-layout-is-dispatched

- [x] Implement the behavior described in `scenarios/pi-layout-is-dispatched.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 15. Implement scenario: [the-bridge-fails-loudly-and-recovers](scenarios/the-bridge-fails-loudly-and-recovers.md)

- [x] Implement the behavior described in `scenarios/the-bridge-fails-loudly-and-recovers.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.
