# 029 — Bootstrap Runtime Auto-Detect and Wire Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Add the Pre-flight Phase to `ductus.md`

- [x] Generalize the §ductus.md Self-Update Check section into a **Pre-flight Phase** positioned after the §Permission Setup seed and before §Pre-run Migrations / the archive fetch.
- [x] Describe the phase running both checks (ductus detection + self-update), accumulating restart-requiring writes, and emitting a single combined abort if anything was written.
- [x] Preserve the existing self-update byte-compare / stale-write behavior inside the phase.
- Done when: ductus.md describes one pre-flight phase whose worst case is a single restart, and the prior standalone self-update section no longer exists as a sibling.

## 2. Add the three-state detection mechanism

- [x] Document **State A** = tool-inventory introspection (`mcp__ductus__*` / `mcp:ductus:*`, lazy names count as present); no shell, no permission.
- [x] Document **State B** = binary probe succeeds, no tools → wire + grant perms + contribute to the pre-flight abort (names every file written).
- [x] Document **State C** = probe fails *or* cannot run *or* is denied → markdown path + one tip line pointing at README Runtime.
- Done when: all three states and the "deny/unavailable ⇒ State C" degradation are specified in ductus.md.

## 3. Add the MCP Wiring subsection

- [x] Specify the per-layout target (`.mcp.json` / `{config_dir}/mcp_config.json`) and the `ductus` entry shape.
- [x] Specify the additive/idempotent in-place merge rules: missing file, missing `ductus`, existing `ductus` (no-op), missing `mcpServers` key, malformed JSON (skip + warn + degrade).
- [x] State explicitly that the write updates and never replaces/truncates the file.
- Done when: ductus.md's MCP Wiring subsection covers all five merge cases.

## 4. Update §Permission Setup and reverse the "not scaffolded" decision

- [x] Add the binary-probe permission to the always-applied seed description.
- [x] Describe the State-B ductus tool-permission grant (per-layout wildcard, additive).
- [x] Rewrite the paragraph that says the runtime is "wired separately and not scaffolded" into the auto-wire-on-detect behavior.
- [x] Confirm the §Procedural-fidelity allowed-prompts list is unchanged (no new prompt).
- Done when: the "not scaffolded" language is gone and no new confirmation prompt was added.

## 5. Add the probe to the Agent Registry `settings_template` seeds

- [x] Claude row: add `Bash(command -v *)`.
- [x] Auggie row: add `{ "toolName": "launch-process", "shellInputRegex": "^command -v ", "permission": { "type": "allow" } }`.
- [x] Antigravity row: add `command(command -v)` (or the validated fallback) and keep it identical to the configure file's form.
- Done when: all three registry seeds grant the probe.

## 6. Add the probe to the three configure files

- [x] `configure/claude.md`: add `Bash(command -v *)` to the canonical allow set.
- [x] `configure/auggie.md`: add the `"^command -v "` `toolPermissions` entry.
- [x] `configure/antigravity.md`: add the probe to `permissions.allow`, matching the registry-seed form; resolve the token-prefix grammar (`command(command -v)` vs `command(which)`).
- Done when: each configure file grants the probe in the same form as its registry seed.

## 7. Reframe the README Runtime section

- [x] Change the framing from "manual MCP wiring" to "`/ductus` auto-wires ductus when the binary is detected."
- [x] Keep the binary download/install instructions (the binary is still installed out of band).
- Done when: README states auto-wiring and no longer instructs the user to hand-edit the MCP config.

## 8. Add new edge cases and the State-C tip to ductus.md output

- [x] §Edge Cases: malformed wiring file (no clobber), denied/unavailable probe ⇒ State C, compounding restart collapsed to one.
- [x] §Post-Scaffolding Output: the State-C tip line; ensure the State-B abort message lists every file written.
- Done when: the new edge cases and output lines are present.

## 9. Run the parity audit (new family deferred — Option B)

- [x] Investigate the existing audit families for a seed↔configure permission-parity home: none fits (Family 14 is install.sh↔registry; Family 3 is the workflow registry; Family 1 is pipeline-status wording). Log a follow-up to `specs/inbox.md` to add a dedicated `runtime-probe-parity.sh` family as its own spec rather than expand the audit surface mid-feature.
- [x] Run `scripts/audit/run-all.sh` and confirm no regressions from tasks 1–8.
- Done when: the audit passes clean with no regressions, and the deferred seed/configure parity family is captured as an inbox follow-up.

## 10. Verification pass

- [x] `npx markdownlint-cli2` clean on the changed markdown (ductus.md, configure files, README, this spec dir).
- [x] Manually walk each state's prose to confirm coherence: State A (silent deterministic), State B (wire + single abort + file list), State C (markdown + tip), and the merged stale-`ductus.md` + unwired case (one restart).
- [x] Confirm `ductus`-absent CI (`markdown-only-pipeline.yml`) reasoning still holds — the new probe is a detection step, not a runtime dependency.
- Done when: lint is clean and all state walk-throughs are coherent.

## 11. Implement scenario: runtime-probe-parity-audit

- [x] Implement the behavior described in [`scenarios/runtime-probe-parity-audit.md`](scenarios/runtime-probe-parity-audit.md).

- **Done when**: `scripts/audit/runtime-probe-parity.sh` asserts, per agent, that the ductus binary probe is in parity between the §Agent Registry `settings_template` seed and that agent's `framework/bootstrap/configure/{key}.md` (present in both or neither — present in one only is a finding), as a fixed-string check in each agent's native grammar; the family is wired into `scripts/audit/run-all.sh` (family-count comment bumped) and enumerated in `framework/commands/audit.md`; `scripts/audit/run-all.sh` passes clean. (Scope narrowed from the original "every seed entry" framing during implementation — the seed and configure sets legitimately diverge; only the probe is a real cross-artifact invariant. See the scenario's Resolved Questions.)

> Tasks 12–14 are backfilled records of scenarios that were added and implemented together via the direct reopen-implement path (a `set-status` reopen plus a single `feat` commit), tracked at the time by their acceptance criteria rather than a `tasks.md` entry. They are recorded here for scenario→task traceability; the work landed in the commits noted on each and predates Task 11.

## 12. Implement scenario: project-inputs-asked-once

- [x] Implement the behavior described in [`scenarios/project-inputs-asked-once.md`](scenarios/project-inputs-asked-once.md).

- **Done when**: project inputs (`name`, `description`, `languages`) persist in `.ductus/config.toml`'s `[project]` table and resolve at §Collect Project Inputs after the Pre-flight Phase — existing values read back, only missing ones prompted — so they are asked at most once across a State-B / stale-`ductus.md` restart and never re-asked on update runs. (Landed in commit `6f75041`.)

## 13. Implement scenario: archive-fetch-direct-codeload

- [x] Implement the behavior described in [`scenarios/archive-fetch-direct-codeload.md`](scenarios/archive-fetch-direct-codeload.md).

- **Done when**: the markdown-path archive fetch uses the direct `codeload.github.com` endpoint (no 302 redirect), so the bootstrap's pre-granted `curl` permission covers it without a prompt; the tarball is byte-equivalent and extraction is unchanged. (Landed in commit `6f75041`.)

## 14. Implement scenario: state-a-deterministic-path-forcing

- [x] Implement the behavior described in [`scenarios/state-a-deterministic-path-forcing.md`](scenarios/state-a-deterministic-path-forcing.md).

- **Done when**: State A (ductus live) is a binding execution contract — every primitive-backed step calls its `ductus` MCP tool, the shell commands under those steps are the State-B/C fallback spec (not executed), non-primitive steps (gitignore `curl`, `git config`, input prompts) run in every state, and a primitive call that errors falls back to that one step's shell spec. (Landed in commit `2d14949`, refined in `42965fe`.)

## 15. Disposition: the criteria and Family 15 the completion gate found false at aa91cdf2

- [x] AC1, AC8 and AC20 predate 048's State A version check (`171d5024`; `framework/bootstrap/ductus.md` lines 330-334 at aa91cdf2): State A warns on a `[runtime] path` mismatch and, live but stale, re-acquires and runs the rest of the session through `{pointer-path} <primitive>`, which contradicts line 340's "MUST call the MCP tool"; AC8's parenthetical misreads the constitution's Versioning text. Annotate each as superseded in part, and reconcile the two `ductus.md` passages, `govern.md` following byte for byte
- [x] AC5 and AC21 name the retired `command -v` seed entry, which no step runs (`ductus.md` line 163); the probe is `~/.ductus/bin/ductus --version` (line 294), seeded for Claude, Auggie and OpenCode and in no `framework/bootstrap/configure/*.md`, Antigravity's seed carrying no CLI entry. Family 15 (`scripts/audit/runtime-probe-parity.sh`) checks the retired literal, so it passes without testing the probe. Owner's call (2026-11-10): change the family's both-or-neither rule, not the configure files — Family 15 checks that each agent whose Permission Setup seed carries CLI entries seeds the store probe, which the seed grants before the probe runs on every `/ductus` run (`framework/bootstrap/ductus.md` §Permission Setup, §ductus runtime detection), and skips Antigravity, whose seed carries no CLI entry, and Pi by name; AC21 is amended to state that rule. Then fix the family and its header, correct the `runtime-probe-parity-audit` scenario (it calls `command -v` the probe and names `framework/commands/audit.md`, where the entry is `framework/reference/audit.md`), annotate AC5 and AC21, and show the family fails on a one-sided probe
- [x] AC15 and AC16 predate Pi (`064-pi-host-support`): Pi restarts once more after the run that writes its bridge (`ductus.md` lines 190 and 552), the registry holds five agents across four layouts, and no text delivers AC15's "the stale abort carries State B's file list" (line 568). Annotate both, and correct or remove the unsupported clause. Owner's call (2026-11-10): fix the bootstrap rather than the clause — the stale abort (`ductus.md` §Pre-flight abort) carries State B's notice, as that file's §Self-update check and `framework/bootstrap/ductus-procedure.md`'s Edge Cases already said it did, so AC11, AC12, AC15 and the State B body hold as written
- [x] AC17: the README's Runtime section no longer says `/ductus` wires the runtime; `cb5b330c` moved that to its Installing and Quick start sections. Annotate it, dropping "when the binary is detected"
- [x] Correct the false sentences in criteria that otherwise hold, and the stale body text: AC6 omits Pi's `write-file` bridge; AC12's "the abort message's file list" is the Closing restart's since 048; `spec.md`'s "State A is unchanged", its two no-version-check claims (MCP Wiring and the Resolved Question), its signpost's "write-file is Claude and OpenCode", and the Documentation section's restatement of AC17
- [x] Criteria verified after the items above, against `b7d8a95f` with this task's edits, by one read-only agent reading the source and running Family 15, with no test run (163 s), its claims checked by the operator: all 21 hold, 13 with their annotations; AC17's annotation no longer dates the Installing (per agent) sentence to `cb5b330c`, which predates it. Body, scenarios and plan, corrected by fact or tense (owner's delegation, 2026-11-10): Pi's bridge in §MCP Wiring and its reversal, Pi's prefix in State A's detection, State B's per-agent registration and the permission write for every `mechanism`, the Resolved Questions preamble, and the plan signpost's count; `state-a-deterministic-path-forcing`'s quote of a §runtime-boundary sentence `ad4fae5b` removed; `archive-fetch-direct-codeload`'s `DUCTUS_REPO` URL, its State C and its tool permission; and `project-inputs-asked-once`'s `AGENTS.md` citation, never true

- **Done when**: every criterion holds or is annotated true, Family 15 fails on a one-sided probe and passes at HEAD, and 029 holds its second review and analysis
