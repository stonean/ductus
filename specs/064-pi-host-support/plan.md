# 064 — Pi Host Support Plan

Implements [064 — Pi Host Support](spec.md).

## Overview

Five surface changes, one runtime change, one release. The bootstrap (`ductus.md` /
`govern.md`) gains the `pi` registry row, the derived-values `pi` column, the
MCP-registration row (mechanism `write-file`, target the bridge file), and a
`### Pi layout` scaffolding section; the new zero-dependency extension bridge
(`framework/bootstrap/pi/ductus-bridge.ts`) ships as a Shared-Files-style row under
the pi layout and lands at `.pi/extensions/ductus.ts`; `install.sh` gains the `pi`
arm; the three agent-aware audit families (14/15/17) learn the pi shape; the command
generator gains a pi output pass (dogfooded and committed in this repo); and
`Host::command_file_candidates` gains the third candidate shape — the one runtime
change, recorded under [022-deterministic-runtime](../022-deterministic-runtime/spec.md)
as its scenario with the `data-model.md` sync, on 022's `done → in-progress` back-edge.
A minor release (`0.59.0`, tag `ductus-v0.59.0`) ships in the same sitting, after
the `done` transition, per the release entry.

## Technical Decisions

### D1 — Command surface: flat project-hyphenated prompt templates

Commands install verbatim to `.pi/prompts/{project}-<name>.md` and invoke as
`/{project}-<name>`. Grounding (live pi install, `dist/core/prompt-templates.js`):
discovery is flat and non-recursive over `.pi/prompts/*.md`; the filename is the
command name; `description` and `argument-hint` frontmatter keys are honored and all
other keys are ignored; body expansion supports `$ARGUMENTS`, `$1..$N`,
`${N:-default}`, `${@:N}` — a superset of the syntax `framework/commands/*.md`
already uses for the Claude path. The flat project-hyphenated name keeps the shared
directory collision-free without a `{project}/` subdirectory, which would need a
`prompts` settings-array entry (non-recursive discovery) and would strip the
namespace from the invocation. `enforce-manifest` runs over `.pi/prompts` with the
glob `{project}-*.md`, never the default `*.md`, so foreign prompt templates are
structurally untouched by slash-command cleanup. The stale/self-install paths are
verbatim byte-compares, identical to the claude-style and opencode cases.

Rejected alternative — skills: verified in
`dist/modes/interactive/interactive-mode.js` that skill commands register as literal
`skill:<name>` behind `enableSkillCommands`; arguments arrive appended as
`User: <args>` with no `$ARGUMENTS` substitution; and skills are described into the
system prompt and prompt-matched. Each is a deviation from the command-file contract
(spec §Design Decisions D1 carries the full rejection).

### D2 — Tool surface: zero-dependency extension bridge over the runtime's MCP server

`framework/bootstrap/pi/ductus-bridge.ts` (new file, no substitutions — installed
byte-identical to `.pi/extensions/ductus.ts`) is a Pi extension that:

- spawns `.ductus/bin/ductus mcp` (resolved relative to the working tree; the
  pointer `/ductus` already materializes) when it loads, keeps it alive, and
  respawns it on the first call after it exits;
- speaks MCP-over-stdio as newline-delimited JSON-RPC 2.0: `initialize` (protocol
  handshake, `clientInfo`) and the `notifications/initialized` notification once per
  child, then `tools/list` and `tools/call` — no npm dependency;
- registers every tool the server lists via `pi.registerTool` under `ductus__<name>`
  with the server's own `inputSchema` passed through;
- when the runtime cannot start at load, registers no tools and shows a
  session-start notice naming the pointer and pointing at `/ductus`
  (re-acquisition); when it fails mid-session, returns an error envelope saying the
  same ([the-bridge-fails-loudly-and-recovers](scenarios/the-bridge-fails-loudly-and-recovers.md)). No
  markdown fallback (spec §Compatibility Contract, deviation list;
  §runtime-boundary principle 3).

The `ductus__` prefix is the Pi spelling of the host-namespace wrap
(§runtime-host-integration: match the prefix, not a spelling) — it makes State-A
detection a `ductus__*` inventory match and prevents collision with Pi built-ins
(`read`, `bash`, …) or foreign extensions.

**Schema-acceptance probe (implementation-time verification, recorded outcome below).**
Pi types `registerTool` parameters as typebox `TSchema`; the bridge holds plain JSON
Schema objects from `tools/list`. Before the bridge is written, a two-tool probe in
`/tmp` (throwaway extension registering one tool with a plain JSON-Schema parameters
object, run under `pi -e`) settles whether the plain object passes through or a
typebox wrap is required. Either path keeps the schema data server-sourced.

> **D2a — Probe outcome (2026-09-20, task 3):** resolved by reading the living source
> rather than a model round-trip — pi 0.86.1's `registerTool`
> (`dist/core/extensions/loader.js:220-230`) validates only that `parameters` is a
> non-array object, and `dist/core/tools/tool-definition-wrapper.js` passes it
> through verbatim into the model-bound AgentTool. **Plain JSON-Schema objects are
> accepted directly — no typebox wrap needed**; the bridge passes the server's
> `inputSchema` through as-is (server-sourced schema data, the D2 property).

### D3 — One runtime change: the third command-file candidate

`Host::command_file_candidates` (runtime/src/host.rs) appends
`{cli-config-dir}/prompts/{project}-{name}.md` **after** the existing
`commands/{project}/` (plural, claude-style/auggie) and `command/{project}/`
(singular, opencode) candidates — order preserved, `cli-config-dir` from the session
file remains the real selector. A unit test records `cli-config-dir = .pi` in a
session fixture and asserts the candidate list ends with the pi shape; the two
pre-existing tests (`command_file_candidates_cover_both_layouts_plural_first` and the
`.augment`/`.opencode` fixtures) pass unmodified. Nothing else in `runtime/` moves:
no new primitive, no schema change, the MCP server is byte-identical. The
`write-session` doc comment's config-dir enumeration gains `.pi` in the same commit
(doc sync, no behavior).

Record routing (standing AGENTS.md rule): the primitive-level change lands as a
scenario under 022 back-linking 064 per §cross-spec-impact, with 022's
`data-model.md` recording the candidate in the command-resolution contract; 022
takes the `done → in-progress` back-edge and 064 cannot reach `done` before the
022 scenario does. 064's AC8/AC9 verify against the shipped behavior.

### D4 — `configure/pi.md`: verify-and-repair

`framework/bootstrap/configure/pi.md` (new command source, no substitutions beyond
the standard `{project}` / `{cli-config-dir}`): (1) byte-compare
`.pi/extensions/ductus.ts` against the upstream `framework/bootstrap/pi/ductus-bridge.ts`
and overwrite on divergence; (2) report that Pi has no permission-gating settings
(every tool call runs without a host prompt; project trust is the only gate) and
state the trust requirement for a freshly cloned pi-adopted repo.
`gen-configure-mcp.sh`'s header names the pi exclusion (it generates MCP-permission
blocks; pi has none) — stated, not silent, per the check-that-cannot-run rule.

### D5 — Audit and generator fan-out (mechanical, per AC10/AC11/AC6)

- **Family 14** (`installer-registry-parity.sh`): layout branch
  `pi → {config_dir}/prompts/ductus.md`; the settings-seed parity check treats pi as
  "the arm writes no settings file" — it asserts the *absence* of a settings heredoc
  in the pi arm, the parity shape for the no-permission agent.
- **Family 15** (`runtime-probe-parity.sh`): skips pi by name on stderr; the header
  states the bound (pi has no settings template and no permission shape to probe).
- **Family 17** (`host-namespace-parity.sh`): third installed-namespace shape —
  `.pi/prompts/{ns}-*.md` flat files; a `{ns}-`-prefixed file counts the namespace as
  installed.
- **`gen-claude-commands.sh`**: second output pass to `.pi/prompts/` — flat
  `ductus-<name>.md` files (`{project}` → `ductus`, `{cli-config-dir}` → `.pi`) plus
  the verbatim bridge copy to `.pi/extensions/ductus.ts`; `--check` covers both, so
  the pre-commit check-zero protects the pi surface. This repo's `.gitignore` gains
  `.pi/*` with negations for `.pi/prompts/` and `.pi/extensions/ductus.ts`, mirroring
  the `.claude/*` / `!.claude/commands/` convention; the dogfooded copy is committed.
- `scripts/audit/README.md` updates the three family descriptions to carry the pi
  shapes.

### D6 — Bootstrap prose (ductus.md, mirrored to govern.md byte-for-byte)

- Agent Registry: the `pi` row — `Pi` / `.pi` / layout `pi` / empty
  `settings_template` (no permission-gating surface) / "Pi reads `AGENTS.md`
  natively — no second rules file is needed."
- §Derived values: new `pi` column — command/skill path
  `.pi/prompts/{project}-<name>.md`; invocation `/{project}-<name>`; `ductus` install
  path `.pi/prompts/ductus.md`; settings file `.pi/settings.json` (Permission Setup:
  documented no-op — no permission-gating entries exist to seed); native rules file
  `AGENTS.md`; no native rules dir; cleanup glob `{project}-*.md` in `.pi/prompts`.
- §MCP registration: the `pi` row — target `.pi/extensions/ductus.ts` (the bridge
  from the staging archive), scope `project-local` (gitignored; re-scaffolded by
  every `/ductus` and repairable by `/{project}:configure`), mechanism `write-file`
  (the additive write operation is unchanged; only the target's commit status differs
  from the other two `write-file` rows).
- `### Pi layout` scaffolding section beside the Antigravity/OpenCode sections: the
  sixteen `framework/commands/*.md` rows plus the `configure` row copy verbatim to
  `.pi/prompts/{project}-<name>.md` with the standard substitutions; the bridge row
  copies `framework/bootstrap/pi/ductus-bridge.ts` to `.pi/extensions/ductus.ts`
  with **no** substitution; the `.pi/` line joins the framework-managed `.gitignore`
  block; the completion message carries the Pi trust reminder (project-local `.pi/`
  resources load only after trust — accept the prompt or pass `--approve` once).
- State B / §MCP wiring: the `write-file` branch names the pi target (bridge file,
  not an MCP config); the `surface-instruction` prose is unchanged. §Permission
  Setup's per-layout resolution gains the pi no-op branch. §Self-update check and
  the `ductus` self-install / Placeholder Substitution / Post-Write Integrity
  enumerations name the pi paths (all verbatim byte-compares). `ductus-procedure.md`
  (the archive half): next-steps invocation list, pinned-divergent install-path row,
  and Directory Creation gain the pi shapes.

### D7 — Constitution, README, AGENTS.md, docs (prose sweep, per AC12/AC13)

- `framework/constitution.md` §runtime-host-integration: Pi's spelling in full —
  `ductus__<verb>-<noun>`, supplied by the project's `.pi/extensions/ductus.ts`
  bridge; Pi has no built-in MCP, so the bridge wraps the runtime's own MCP server
  over stdio and the server remains the single source for names and schemas.
- `README.md`: agent list gains Pi; §Installing (per agent) accepts `pi`
  (`sh -s -- pi`); the invocation paragraph names `/{project}-<name>` for Pi.
- `AGENTS.md` (this repo's contributor home): the live registry enumeration updates to
  five agents across four layouts; one new entry records the bridge architecture
  (never "fix" Pi by registering an MCP server for it; keep the bridge
  dependency-free — the README promise and the live-schema property both depend on
  it).
- `docs/slash-commands.md`: audited at implementation for agent-shape claims;
  anything enumerating agents or invocation forms gains Pi.
- Anti-claim discipline (spec §D7): no artifact may say Pi "registers MCP" or that
  its commands live in a namespace directory.

### D8 — 022 back-edge and release sequencing

022 reopens (`done → in-progress`) before any 064 runtime work; the scenario
(`the-pi-command-candidate` slug, or as plan lands it) back-links 064; 022's
`data-model.md` records the third candidate shape; a matching task lands in 022's
`tasks.md`. 022's review/analyze refresh follows its standing disposition
(`compute-review-scope` decides the window; a truthful small-`examined` record with
every unread path named when the window is too large). Release, once 064 is `done`
and in the same sitting: all three version sites to `0.59.0` + CHANGELOG heading,
one `cargo build --release --offline` lock refresh, commit, post-commit
`scripts/audit/run-all.sh`, `git tag ductus-v0.59.0`, push, then read every
workflow's run for the sha (`in_progress` + `release not found` is wait, not outage).
The fork's own `0.53.0` bump and tag were never this repository's release.

### D9 — In-repo smoke test (verification record, AC14)

Before the tag, a real `pi` run in the framework repo (trusted; `--approve` for the
non-interactive leg): a `/{project}-…` command resolves from `.pi/prompts/`, the
`ductus__*` tools register from `.pi/extensions/ductus.ts`, a live tool call
round-trips against `runtime/target/release/ductus`, and `ductus exec` resolves the
`.pi/prompts/` candidate via the session's `cli-config-dir = .pi`.

> **Smoke-test record (2026-09-20, task 11):** all four evidence points held.
> (1) The bridge load is clean: `pi --approve -p … --no-session` in the framework repo
> (pointer at `.ductus/bin/ductus` → `runtime/target/release/ductus`, session
> `cli-config-dir = .pi`) completes normally — the extension registers at load without
> crashing pi. (2) Tool registration proven by live call: prompting the model to call
> `ductus__read-spec` returned the real spec JSON (frontmatter status `planned`, all 16
> ACs) — the tool was on the model's menu, the call crossed the bridge to the runtime's
> MCP server, and the server's own schema produced the result. (3) `ductus exec`
> resolves the pi candidate: in an adopter-shaped scratch repo (`.pi/prompts/{project}-status.md`,
> session `cli-config-dir = .pi`), `ductus exec status` dispatched the `dashboard`
> primitive instead of failing with `command file not found`. (4) The framework repo's
> own exec uses the framework-local fallback first (correct dogfood behavior — the
> interpreter's `locate_command_file` tries `framework/commands/…` first). One
> implementation fix fell out of the probe: the bridge's repo-root resolution must walk
> up from the working directory to the `.ductus/` marker rather than trust `__dirname`
> under jiti (pi's loader), which resolved the root one level short in the first attempt.
>
> **After `991956e2` (2026-10-07, task 11):** not re-run in this repository. The
> rewritten lifecycle — spawn at load, handshake per child, respawn, failure notice —
> is accepted on the contributor's testing of it against pi, by operator decision.
> `scripts/tests/pi-bridge-harness.mjs` covers the transport under node; nothing here
> exercised pi's own loader on the rewritten bridge. An attempt with pi 1.0.4 stopped
> at the model request, refused by the account's usage limit, before any tool call.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/host.rs` | Modify | Third candidate shape + `.pi` session fixture test; `write-session` doc enumeration sync |
| `framework/bootstrap/pi/ductus-bridge.ts` | Create | The zero-dependency MCP-over-stdio bridge extension |
| `framework/bootstrap/configure/pi.md` | Create | Pi's `/{project}:configure` — bridge verify/repair + trust/permissions report |
| `framework/bootstrap/ductus.md` | Modify | Registry row, derived-values pi column, MCP-registration row, `### Pi layout`, State B / Permission / self-install / staleness prose, gitignore block |
| `framework/bootstrap/govern.md` | Modify | Byte-identical mirror (Family 21) |
| `framework/bootstrap/ductus-procedure.md` | Modify | Archive-half prose: invocation list, pinned-divergent row, Directory Creation |
| `framework/constitution.md` | Modify | §runtime-host-integration pi spelling |
| `install.sh` | Modify | `pi` arm (no settings seed) + usage/error lines |
| `scripts/audit/installer-registry-parity.sh` | Modify | Family 14 pi branch + no-settings assertion |
| `scripts/audit/runtime-probe-parity.sh` | Modify | Family 15 pi skip by name |
| `scripts/audit/host-namespace-parity.sh` | Modify | Family 17 pi namespace shape |
| `scripts/audit/README.md` | Modify | Family 14/15/17 descriptions |
| `scripts/gen-claude-commands.sh` | Modify | Second output pass (`.pi/prompts/` + bridge copy) in generate and `--check` |
| `scripts/gen-configure-mcp.sh` | Modify | Header names the pi exclusion |
| `README.md` | Modify | Agent list, install section, invocation paragraph |
| `AGENTS.md` | Modify | Registry count + bridge-architecture entry |
| `docs/slash-commands.md` | Modify (if claims agent shapes) | Pi in any agent enumeration |
| `.gitignore` | Modify | Dogfooded `.pi/*` + negations |
| `.pi/prompts/ductus-*.md`, `.pi/extensions/ductus.ts` | Create (generated) | Dogfooded pi surface, committed |
| `specs/022-deterministic-runtime/scenarios/the-pi-command-candidate.md` | Create | 022 scenario, back-linking 064 |
| `specs/022-deterministic-runtime/{spec.md, data-model.md, tasks.md}` | Modify | Back-edge, candidate recording, matching task |
| `version`, `runtime/Cargo.toml`, `runtime/CHANGELOG.md` | Modify | `0.59.0` release, after `done` |

## Trade-offs

- **Skills rejected for the command surface** — `/skill:<name>` invocation, the
  `enableSkillCommands` settings dependency, `User: <args>` argument delivery against
  the `$ARGUMENTS` the command bodies name, and system-prompt description loading.
  Prompt templates are the native superset-compatible match (D1).
- **Flat hyphenated names over a `{project}/` subdirectory** — the subdirectory would
  need a `prompts` settings-array entry (discovery is non-recursive) and would strip
  the namespace from the invocation; the flat spelling keeps the antigravity
  invocation form with zero settings (D1).
- **MCP bridge over a CLI bridge** — a CLI bridge makes the CLI the second contract,
  re-encodes every primitive's typed arguments by hand, and silently diverges when
  CLI and MCP stop agreeing; the bridge keeps the runtime's MCP server the single
  source of truth for names and schemas (D2).
- **Hand-rolled JSON-RPC over an npm MCP SDK** — the stdio transport needs exactly
  three method families; a dependency would break the README's
  "nothing is added to your project's dependency manifest" promise in a committed,
  team-visible file (D2).
- **`ductus__` prefix over bare tool names** — bare names would make State-A
  detection a per-name inventory scan and leave a collision surface against Pi
  built-ins and foreign extensions (D2).
- **Verify/repair `configure/pi.md` over a no-op** — the bridge file gets a repair
  path between full `/ductus` runs, and the trust/permission facts have a home in the
  per-agent command set (D4).
- **Spawn at load, not lazily** — recorded first as "lazy spawn over eager", which
  the bridge never was: the tool list comes from the runtime, so registration has to
  start it, and a lazily started bridge would register nothing. A respawn on the
  first call after an exit is what self-heals (D2; corrected 2026-10-05).
- **No `cross-spec-impact:` on 012/028/029/031/032** — the registry *consumes* the
  new row; layout branches are additive; State-B wiring branches on the existing
  `mechanism` vocabulary. 032's provenance citation stays blockquoted so no
  dependency edge is induced (it is a citation, not a dependency — 064 does not build
  on opencode's behavior). The one modified spec is 022 (D3), which is the
  back-link partner.
- **Known limitation — no permission gating in Pi.** Irreducible host property, not a
  gap this spec can close: the bootstrap's Permission-Setup no-op, `configure/pi.md`'
  report, and the completion message's trust reminder are the whole of the handling.
- **Known limitation — the smoke test is a manual verification step, not a CI check.**
  Project trust + a live host make an interactive pi run non-reproducible in CI; a
  skipped conformance test is the check-that-cannot-run failure, so the step records
  its result in this plan (§D8 smoke-test record, under AC14) instead of pretending CI
  covers it.
