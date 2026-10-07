---
status: in-progress
dependencies: [012-multi-agent-govern, 022-deterministic-runtime, 028-antigravity-agent, 029-bootstrap-runtime-autowire, 031-agent-mcp-wiring, 056-bootstrap-archive-boundary-split]
next-criterion: 17
---

# 064 — Pi Host Support

Add **Pi** (`pi`, the `@earendil-works/pi-coding-agent` terminal coding agent) as the
fifth agent `ductus` scaffolds into, alongside Claude Code, Auggie, Antigravity, and
OpenCode. The work expresses Pi's command surface (prompt templates), its tool surface
(a zero-dependency extension bridge — Pi has **no built-in MCP**, by design), its native
rules file, and its (absent) permission format as registry-derived values so `/ductus`
produces a working Pi adoption — on **verified** conventions, not assumed ones.

> **Source provenance.** Following the [032-opencode-agent](../032-opencode-agent/spec.md)
> discipline — observed behavior governs — Pi's model here is **verified against the
> live install** (`@earendil-works/pi-coding-agent` as shipped on the maintainer's
> machine, Node 24): `dist/core/prompt-templates.js` (the prompt-template loader — flat,
> non-recursive `.pi/prompts/*.md` discovery; `description` and `argument-hint`
> frontmatter; `$ARGUMENTS` / `$1..$N` / `${N:-default}` / `${@:N}` body expansion;
> unknown frontmatter keys ignored), `dist/modes/interactive/interactive-mode.js`
> (skill commands register as literal `skill:<name>` invocations, gated on
> `enableSkillCommands`), `docs/settings.md` (project `.pi/settings.json`; project-trust
> gates loading of `.pi/` resources; **no permission-gating setting exists in the schema**),
> `docs/security.md` ("No Built-in Sandbox"; trust is an input-loading guard only),
> `docs/extensions.md` (project-local `.pi/extensions/*.ts` auto-discovery,
> `pi.registerTool` custom tools, extension-importable `typebox`), `docs/usage.md`
> ("It intentionally does not include built-in MCP …"), plus a grep over `dist/`
> confirming no MCP client ships. The narrow items settled during planning are
> **design choices** recorded in §Resolved Questions, not unknowns about Pi.

## Motivation

The Agent Registry from [012-multi-agent-govern](../012-multi-agent-govern/spec.md),
generalized by [028-antigravity-agent](../028-antigravity-agent/spec.md), made adding
an agent a registry row plus satellite files. [031-agent-mcp-wiring](../031-agent-mcp-wiring/spec.md)
then split MCP discovery into a per-agent descriptor that
[029-bootstrap-runtime-autowire](../029-bootstrap-runtime-autowire/spec.md)'s State-B
auto-wire consumes. Every registry row so far assumed two things Pi breaks at the root:
that the host can be given a slash-command directory shape under a per-agent config dir,
and that the host can **run an MCP server** — Pi's tool surface is extension-only, and
its command surface is a flat, shared prompt-template directory rather than a
per-agent namespace directory.

Pi adopters driving their work through Pi got none of the `ductus` pipeline: the slash
commands were never scaffolded where Pi scans, and the runtime had no registration path
at all — no MCP, no second route. This spec slots Pi into the same registry-driven
machinery the other four agents use, with the one new mechanism none of them needed.

## Verified Pi Layout

| Dimension | Verified value |
| --- | --- |
| Command surface | **Prompt templates** — `.pi/prompts/*.md`, flat, non-recursive, project-trust-gated. Filename is the command name (`/name`). Frontmatter `description` + `argument-hint` honored; other keys ignored. Body expansion supports `$ARGUMENTS`, `$1..$N`, `${N:-default}`, `${@:N}` — a superset of the syntax the pipeline command files already use |
| Skills (rejected surface) | `.pi/skills/<name>/SKILL.md`, invoked `/skill:<name>` only when `enableSkillCommands: true`; arguments arrive appended as `User: <args>` with **no** `$ARGUMENTS` substitution |
| Runtime tools | **No built-in MCP client** (by design). The only extension point is a TypeScript extension registering custom tools via `pi.registerTool`; project-local `.pi/extensions/*.ts` is auto-discovered once the project is trusted |
| Permissions | **None.** No permission-gating settings surface in `.pi/settings.json` or anywhere else; the model runs tool calls without host prompts. Project trust (`~/.pi/agent/trust.json`, global) controls only whether `.pi/` resources load at all |
| Native rules | `AGENTS.md` is loaded natively, regardless of project trust |
| Settings | `.pi/settings.json` (project) / `~/.pi/agent/settings.json` (global) |
| Trust | Project-local `.pi/` resources (settings, prompts, extensions, skills, themes) load only after the project is trusted; non-interactive runs follow `defaultProjectTrust` or `--approve` |

## Compatibility Contract

"100% compatibility" for Pi means:

1. Every pipeline command (`specify`, `clarify`, `plan`, `tasks`, `implement`, `review`,
   `analyze`, `target`, `log`, `groom`, `audit`, `amend`, `consolidate`, `fold`,
   `status`, `help`, `configure`, and the `/ductus` bootstrap itself) is invokable from
   Pi and walks the **same** `framework/commands/*.md` procedure — no Pi-specific
   command variants, no forked procedure.
2. Every runtime primitive (the canonical set in `framework/runtime-tools.txt`) is
   callable from Pi with the **same bare name and same JSON schema**, delivered through
   the extension bridge of §Design Decisions D3 — the runtime's own MCP server remains
   the single source of truth for tool names and schemas.
3. `ductus exec` works unchanged from Pi's shell tool: it dispatches primitives
   in-process and never touches the host's tool registry, so the exec path is free
   parity once command-file resolution knows Pi's shape (D4).
4. Gates, session state, constitution, rules, and migrations behave identically — they
   are runtime plus file work, host-agnostic.

Permitted deviations, each an inevitable host difference rather than a capability gap:
the invocation spelling `/{project}-<name>`; tools arriving under `ductus__<name>`
extension tools rather than `mcp__ductus__<name>`; the bootstrap's Permission-Setup step
being a documented no-op for Pi; and Pi's one-time project-trust acceptance. Parity of
host UX is **not** claimed: Pi has no permission prompts and no deferred-tool
mechanism; nothing in the pipeline depends on either.

## Design Decisions

- **D1 — Command surface: prompt templates, flat, project-hyphenated.** Each command
  installs verbatim to `.pi/prompts/{project}-<name>.md`, invoked `/{project}-<name>`
  (the antigravity spelling). The flat project-hyphenated name keeps the shared
  directory collision-free across projects without a per-project subdirectory (which
  would require a `prompts` settings array entry and would strip the namespace from the
  invocation). Enforcement of `.pi/prompts` uses the glob `{project}-*.md` — never the
  default `*.md` — so `enforce-manifest` prunes only ductus's own retired commands and
  can never delete the user's foreign prompt templates. The rejected alternative
  (skills) is rejected on three verified counts: it requires `enableSkillCommands:
  true` in every adopter's settings; its `User: <args>` argument delivery conflicts with
  the `$ARGUMENTS` the command bodies name; and skills are prompt-matched and
  description-loaded into the system prompt, so the model might load a pipeline command
  it should have waited to be invoked for.
- **D2 — Self-install and staleness are verbatim.** The `ductus` bootstrap installs to
  `.pi/prompts/ductus.md` (→ `/ductus`), a verbatim copy of upstream like claude-style
  and opencode: the staleness check is a direct byte-compare, and the stale-write path
  overwrites the installed file in place. No skill-frontmatter wrapping, no body-only
  comparison.
- **D3 — Tool surface: a shipped, zero-dependency extension bridge.**
  `framework/bootstrap/pi/ductus-bridge.ts` (Shared Files row, strategy `update`) lands
  as `.pi/extensions/ductus.ts` in adopter repos. The extension spawns
  `.ductus/bin/ductus mcp` (the per-project pointer `/ductus` already materializes) as
  a stdio child when it loads, because the tool list comes from it, speaks MCP-over-stdio as newline-delimited JSON-RPC
  2.0 (`initialize` → `tools/list` → `tools/call`) with no npm dependency — the README
  promises nothing enters the adopter's dependency manifest, and the stdio transport
  needs nothing a ~150-line extension cannot carry. Every tool the server lists is
  registered via `pi.registerTool` under `ductus__<name>` with the **server's own
  `inputSchema` passed through** — names and schemas are live-from-server on every
  start, so a future release that adds or changes a primitive reaches Pi with zero
  framework edits, exactly as it reaches the MCP hosts. The `ductus__` prefix is the
  Pi spelling of the host-namespace wrap §runtime-host-integration prescribes (match
  the prefix, not a spelling); it makes State-A detection a `ductus__*` inventory
  match and rules out collision with Pi built-ins or foreign extensions. The process
  is kept alive and respawned on the first call after it exits. When it cannot start
  at load, no `ductus__*` tool is registered — a placeholder would read as State A —
  and session start shows a notice naming the pointer and pointing at `/ductus`
  (re-acquisition); when it fails mid-session, the call returns an error envelope
  saying the same
  ([the-bridge-fails-loudly-and-recovers](scenarios/the-bridge-fails-loudly-and-recovers.md))
  — no silent markdown fallback: §runtime-boundary principle 3 makes acquisition failure halt, and the
  two-paths guarantee covers *unwired* hosts, not a wired host that lost its binary.
  The one implementation risk — Pi types `registerTool` parameters as typebox
  `TSchema` while the bridge holds plain JSON Schema — is settled by a throwaway
  two-tool probe against the live install before the bridge is written, and the
  recorded outcome (plain-object accepted, or typebox-wrapped) lands in this spec's
  plan. Either way the schema data is server-sourced.
- **D4 — One runtime change: the third command-file candidate.**
  `Host::command_file_candidates` gains `{cli-config-dir}/prompts/{project}-{name}.md`,
  appended **after** the existing plural `commands/{project}/` and singular
  `command/{project}/` shapes, so every existing layout's resolution order is
  untouched and `cli-config-dir` in the session file remains the real selector. This is
  what makes `ductus exec` and the `writeCode` payload find Pi's installed command
  files. No other runtime surface moves: no new primitive, no schema change, the MCP
  server is byte-identical. Per the standing routing rule the primitive-level change
  lands as a scenario under [022-deterministic-runtime](../022-deterministic-runtime/spec.md)
  (back-linking this spec per §cross-spec-impact) with `022`'s `data-model.md`
  recording the candidate; this spec owns the requirement and the
  [056-bootstrap-archive-boundary-split](../056-bootstrap-archive-boundary-split/spec.md)-shaped
  bootstrap work on both sides of the archive/installer boundary.
- **D5 — `/{project}:configure` for Pi is verify-and-repair.** Pi has no
  permission-gating settings to write, so `framework/bootstrap/configure/pi.md` is:
  (1) byte-compare `.pi/extensions/ductus.ts` against the upstream
  `framework/bootstrap/pi/ductus-bridge.ts` (the same comparison the bootstrap's
  self-update check makes for installed `ductus` files) and overwrite on divergence;
  (2) report that Pi has no permission settings to configure — every tool call runs
  without a host prompt, and the trust decision is the only gate — and state the
  trust requirement for a freshly cloned pi-adopted repo (accept the trust prompt or
  pass `--approve` once). `gen-configure-mcp.sh` excludes `pi.md` **by name** in its
  header, stating the bound (Pi's tools arrive through the bridge; it has no
  MCP-permission shape) rather than silently skipping it.
- **D6 — Audit and generator fan-out.** (1) `install.sh` gains a `pi` arm —
  `.pi/prompts/ductus.md`, verbatim, **no settings seed** (stated in-arm: Pi has no
  permission-gating settings) — and the unknown-agent error names `pi`. (2) Family 14
  (`installer-registry-parity`) gains the `pi → {config_dir}/prompts/ductus.md`
  layout branch, and its settings-seed parity check treats Pi as "the arm writes no
  settings file" (asserts absence — the parity shape for the no-permission agent).
  (3) Family 15 (`runtime-probe-parity`) skips Pi by name on stderr, bound stated in
  the header. (4) Family 17 (`host-namespace-parity`) recognizes a third
  installed-namespace shape: `.pi/prompts/{ns}-*.md` flat files. (5)
  `gen-claude-commands.sh` gains a second output pass to `.pi/prompts/ductus-*.md`
  (flat, `{project}` → `ductus`, `{cli-config-dir}` → `.pi`), included in `--check`
  so the pre-commit check-zero covers the pi surface; the framework repo's dogfooded
  pi copy is **committed**, mirroring the `.claude/commands/` convention. (6)
  `scripts/audit/README.md` family descriptions update accordingly.
- **D7 — Prose sweep and anti-claim.** README agent list + per-agent install section +
  invocation paragraph; constitution §runtime-host-integration gains Pi's spelling in
  full (the canonical source other copies point at); `AGENTS.md`'s live registry
  enumeration updates (five agents, four layouts) plus one entry recording the
  bridge architecture (never "fix" Pi by registering an MCP server for it; keep the
  bridge dependency-free — the README promise and the live-schema property both
  depend on it). No artifact may say Pi "registers MCP" or that its commands live in
  a namespace directory — Pi's shape is the exact outlier (bridge-not-MCP,
  flat-not-directory) a careless parallel with antigravity would misstate.
- **D8 — Release and process.** A new-feature runtime change ships as a minor bump:
  all three version sites to the next minor, CHANGELOG entry, commit,
  `ductus-v<minor>` tag pushed the same sitting as the `done` transition. The
  pipeline is walked from Pi via the runtime CLI surface (the State-B continuation
  path the bootstrap itself sanctions), the bridge is probe-verified against the live
  install before the tag, and a real `pi` run in the framework repo is the
  pre-tag composition check.

## Acceptance Criteria

- [x] AC1: `install.sh` carries a `pi` arm that writes `.pi/prompts/ductus.md` verbatim, writes **no** settings file (stated in-arm), and the unknown-agent error line names `pi`
- [x] AC2: The Agent Registry row `pi` / `Pi` / `.pi` / layout `pi` is present, with an empty `settings_template` (no permission-gating surface) and a `rules_file_note` stating Pi reads `AGENTS.md` natively
- [x] AC3: The §Derived values table carries a `pi` column: command/skill path `.pi/prompts/{project}-<name>.md`, invocation `/{project}-<name>`, `ductus` install path `.pi/prompts/ductus.md`, settings file `.pi/settings.json` (Permission Setup a documented no-op), native rules file `AGENTS.md`, slash-command cleanup glob `{project}-*.md` in `.pi/prompts`
- [x] AC4: The MCP registration table carries the `pi` row — target `.pi/extensions/ductus.ts` (the bridge from the staging archive), scope `project-local` (gitignored), mechanism `write-file` — and the State-B / MCP-wiring prose branches on Pi without writing any MCP config file
- [x] AC5: The bootstrap's Pi-layout scaffolding section installs the sixteen command rows plus the configure row verbatim to `.pi/prompts/{project}-<name>.md` (with `{project}` / `{cli-config-dir}` substitution), installs `framework/bootstrap/pi/ductus-bridge.ts` to `.pi/extensions/ductus.ts` with no substitution, adds `.pi/` to the framework-managed `.gitignore` block, and the completion message carries the Pi trust reminder
- [x] AC6: `framework/bootstrap/pi/ductus-bridge.ts` exists with zero npm dependencies: spawn of `.ductus/bin/ductus mcp` at load, hand-rolled MCP-over-stdio JSON-RPC (`initialize` / `notifications/initialized` / `tools/list` / `tools/call`), `pi.registerTool` for every server-listed tool under `ductus__<name>` with the server's `inputSchema` passed through, and the failure paths of [the-bridge-fails-loudly-and-recovers](scenarios/the-bridge-fails-loudly-and-recovers.md): no tools and a session-start notice naming the pointer and `/ductus` when the runtime cannot start at load, an error envelope naming both when it fails mid-session, a timeout that retires a runtime that stops answering, and a respawn on the next call (no markdown fallback); the runtime's stdout is decoded as a UTF-8 stream
- [x] AC7: `framework/bootstrap/configure/pi.md` verifies `.pi/extensions/ductus.ts` against the bridge at the project's own source — the ref `/ductus` resolves — (overwrite on divergence) and reports the no-permission-settings and trust facts; `gen-configure-mcp.sh`'s header names the Pi exclusion
- [x] AC8: `Host::command_file_candidates` yields `{cli-config-dir}/prompts/{project}-{name}.md` as the **last** candidate, with a runtime test whose session file records `cli-config-dir = .pi`, and the two pre-existing candidate shapes keep their order (pre-existing host tests pass unmodified)
- [x] AC9: [022-deterministic-runtime](../022-deterministic-runtime/spec.md) carries the scenario for the third candidate shape back-linking this spec, and its `data-model.md` records the candidate in the result-shape registry
- [x] AC10: Family 14 passes with the Pi row: the `pi → {config_dir}/prompts/ductus.md` branch resolves, and the settings-seed parity check asserts the Pi arm writes no settings file; Family 15 skips Pi by name on stderr; Family 17 recognizes `.pi/prompts/{ns}-*.md` as an installed namespace
- [x] AC11: `gen-claude-commands.sh` emits `.pi/prompts/ductus-*.md` and `--check` covers it; the framework repo's `.pi/prompts/` dogfooded copy is committed with the matching `.gitignore` line
- [x] AC12: Constitution §runtime-host-integration names Pi's tool spelling in full — `ductus__<verb>-<noun>` via the project's `.pi/extensions/ductus.ts` bridge, with the no-built-in-MCP note keeping the runtime's MCP server the single source for names and schemas
- [x] AC13: `README.md` names Pi in the agent list, accepts `pi` in the per-agent install section, and the invocation-paragraph names `/{project}-<name>` for Pi
- [x] AC14: A real `pi` run in a trusted project resolves a `/{project}-…` command from `.pi/prompts/`, registers the `ductus__*` tools, round-trips a live tool call, and `ductus exec` resolves the `.pi/prompts/` candidate — result recorded in this spec's plan as verification evidence
- [x] AC15: `framework/bootstrap/govern.md` remains byte-identical to `framework/bootstrap/ductus.md` (Family 21)

## Resolved Questions

- **The release follows `done`; it is not part of this spec's completion gate** — resolved 2026-10-07 by operator decision. AC16 required the `ductus-v<minor>` tag to exist before this spec could reach `done`, which inverts `AGENTS.md`'s release entry: a release is cut from completed work, every affected spec `done` first and the tag pushed in the same sitting — the order whose reverse that entry records as circular. AC16 is retired and its label is not reissued; D8's same-sitting release is the commitment that remains, so `ductus-v0.59.0` is cut in the session that marks this spec `done`.
- **Prompt templates over skills** — settled D1. Skills' `/skill:<name>` invocation, `enableSkillCommands` dependency, `User: <args>` argument delivery (against the `$ARGUMENTS` the command bodies name), and system-prompt description loading are each a verified deviation from the command-file contract; prompt templates are a superset-compatible argument engine with the same frontmatter keys the command files already carry.
- **Flat project-hyphenated names over a `{project}/` subdirectory** — settled D1. A subdirectory would require a `prompts` settings array entry (non-recursive discovery) and would strip the namespace from the invocation (`/specify` un-namespaced); the flat `{project}-<name>` spelling keeps the antigravity invocation form, needs no settings, and stays collision-free.
- **`ductus__` prefix over bare tool names** — settled D3. Every other host wraps bare names in a server namespace; the prefix makes State-A detection an inventory glob, matches the constitution's match-the-prefix rule, and rules out built-in/foreign-extension collisions.
- **Bridge over CLI bridge** — settled D3. A CLI bridge would make the CLI surface the second contract, re-encode every primitive's typed arguments by hand, and silently diverge the moment CLI and MCP stop agreeing — a second registry copy, the exact drift §runtime-boundary prevents.
- **Pi's `configure` is verify/repair, not a no-op** — settled D5. The bridge file gets a repair path between full `/ductus` runs, and the trust/permission facts have a home in the per-agent command set instead of living only in README prose.
- **Dogfooded pi surface committed in the framework repo** — settled D6. Uncommitted would make the framework repo's own pi adoption invisible to the audits that exist to protect it; the `.claude/commands/` convention already commits the dogfooded copy.
- **No `cross-spec-impact:` on 012 / 028 / 029 / 031 / 032** — those specs' documented behavior is untouched: the registry *consumes* the new row, the layout branches are additive, and the State-B wiring branches on the existing `mechanism` vocabulary. The one spec this change modifies is 022 (the runtime), which is the back-link partner. 056 is a body-linked dependency (the archive/installer boundary the bootstrap edits land on both sides of), not an impact: neither side's contract moves.
- **Typebox/JSON-Schema acceptance** — not a design open question but an implementation-time verification: the two-tool probe outcome (plain JSON-Schema object accepted by `registerTool`, or typebox wrap required) is recorded in the plan at implementation time; either path keeps the schema data server-sourced.
