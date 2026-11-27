---
status: done
dependencies: [003-bootstrap-automation, 021-runtime-boundary, 022-deterministic-runtime]
next-criterion: 22
---

# 029 — Bootstrap Runtime Auto-Detect and Wire

`/ductus` should detect the `ductus` runtime at the start of a bootstrap run, wire its MCP server into the project when the binary is present but unregistered, and tell the user that doing so reduces token use. The goal is to stop first-time adoptions from exhausting their token budget on the markdown-only reference path when a deterministic path was actually available.

> **Signpost (post-048):** `framework/bootstrap/ductus.md` §MCP registration (per-agent) is the canonical record of each agent's MCP registration; this spec points at it rather than restating it.
>
> - MCP discovery is a **per-agent** property rather than a layout-derived one, and State-B registration branches on a `mechanism` column (**MCP Wiring** states each agent's descriptor): a `write-file` agent gets its MCP file written. Both agents in the registry, Claude and, since `064-pi-host-support`, Pi, are `write-file`; Pi's file is an extension bridge rather than an MCP config.
> - [048-govern-acquired-runtime](../048-govern-acquired-runtime/spec.md) made State B continue rather than abort; its scenario `state-b-continues-in-session` is where that decision lives. The **Post-completion note** at the foot of this spec carries the rest of 048's amendment.
>
> The binary probe is the store probe, and State A version-checks the live runtime (Post-completion note).

## Motivation

On a first-time adoption, the bootstrap walks the markdown-only reference path — `curl`, `tar`, byte-compares, and a hand-authored scaffold script — because the [ductus runtime](../022-deterministic-runtime/spec.md) is not registered for the agent session. The model interprets a procedure of about 20k tokens step by step, an estimate made when 029 was written with no recorded method, and can run out of tokens before the install finishes.

The failure is not that `ductus` is missing — the binary may already be installed globally and on `PATH`. The failure is that [`/ductus`](../003-bootstrap-automation/spec.md) deliberately does **not** scaffold the MCP wiring (it is documented as a separate, manual install), so the agent session starts with no `ductus` MCP tools and has no deterministic path to take even though one was one config file away.

MCP servers are loaded at session start: a running session keeps the server it started with (`.ductus/docs/agents/developing-ductus.md` §Running a second session). Writing the wiring file mid-run therefore cannot help the running session — only the next one. So the fix is not "use the runtime now"; it is "wire the runtime, tell the user, and let the next session run cheaply." This is the same shape as the existing **ductus.md Self-Update Check**, which already writes a fresh file and aborts with a "start a new session and re-run" instruction. Both are restart-requiring pre-flight concerns, so this feature unifies them into a single pre-flight phase (see **Pre-flight Phase**) — the user restarts at most once even when both fire.

The runtime is required per [§runtime-boundary](../021-runtime-boundary/spec.md), so a missing binary is acquired rather than routed around (Post-completion note).

## Pre-flight Phase

`ductus` detection and the existing **ductus.md Self-Update Check** run as a single pre-flight phase — after the cheap **Permission Setup** seed (so the binary probe is pre-authorized) and before pre-run migrations and the archive fetch. Both checks can independently require the session to restart: the self-update check to load a fresh `ductus.md`, ductus detection to load the MCP server. The pre-flight phase runs both checks and sorts every restart-requiring write (a fresh `ductus.md` and/or the ductus wiring and permission entries) into a pending-restart set or a deferred one. The two contributors are **non-equivalent** (`048-govern-acquired-runtime`) and are inspected separately: a stale `ductus.md` aborts immediately, because the run must not proceed on instructions it has just replaced, while State-B wiring does **not** abort — the binary is on disk and every remaining primitive is reachable through the CLI, so the run continues and the wiring notice rides the **Closing restart** at the end. If neither check requires a restart, the run proceeds past pre-flight normally. The user still restarts at most once regardless of how many pre-flight concerns fired, except a never-wired Pi adopter whose run aborts on a stale `ductus.md`: Pi's bridge is written only after the archive is extracted, which the aborted run never reaches, so that adopter restarts once more after the run that writes it (`064-pi-host-support`).

Within pre-flight, `ductus` detection resolves to exactly one of two states.

### Detection mechanism

Two independent probes resolve the state:

- **Tool-inventory introspection (State A).** The agent inspects its own available-tool inventory for any `ductus`-namespaced MCP tool — `mcp__ductus__*` on Claude Code and a `ductus__*`-prefixed extension-bridge tool on Pi (`064-pi-host-support`) — counting deferred or lazily-loaded tool names as present. This needs no shell and no permission; the agent always knows its own tools. Any match is State A.
- **Store probe (State B).** Only when introspection finds no `ductus` tool, a shell probe runs: the store probe `~/.ductus/bin/ductus --version` (`ductus.exe` on Windows), a filesystem check on the ductus-owned store rather than a `PATH` lookup. A missing binary is acquired (Post-completion note). There is no non-shell way to detect this — a tool that could answer "is ductus installed" would itself put the session in State A. The probe is seeded into the **Permission Setup** pre-grant so routine runs do not prompt for it, on every agent whose seed grants the runtime CLI (Claude; Pi has no seed).

A probe the host cannot run, or the user denies, classifies the run as State B and acquires; acquisition is idempotent, so a false negative costs a version comparison rather than a redundant download, and the gate never hard-fails on a locked-down host.

### State A — runtime live this session

The `ductus` MCP tools are callable in the current session. The bootstrap takes the deterministic primitive path for the rest of the run, and emits no detection message when the version check reports the framework's pin (AC1).

A host that lists MCP tool schemas lazily (e.g., Claude Code surfaces tool names in a deferred-tool reminder before exposing their schemas, and the constitution's §runtime-host-integration has the host fetch a lazy schema before the call) is still in State A — the runtime is registered. The gate must treat lazily-listed `ductus` tools as live, not absent, and fetch the schema through the host's mechanism rather than falling through to a lower state.

### State B — runtime not live: acquire and wire

No `ductus` MCP tools are available to the session. The bootstrap acquires the binary when the store lacks the pin, then, in order:

1. Registers the runtime per the agent's MCP `mechanism`, writing a `write-file` agent's file additively (see **MCP Wiring**), after first acquiring the binary and materializing its pointer (`framework/bootstrap/ductus.md` §State B — runtime not live: acquire, wire, restart).
2. Adds the permission entries the agent needs to call the `ductus` tools (see **Permission Setup**).
3. Contributes a notice that **names every file it wrote** (the wiring file and the settings file), telling the user that `ductus` was wired and that it reduces token use. The notice rides the **Closing restart** emitted after the run's work completes, and the restart it asks for is for the MCP tool surface rather than for unfinished work. On a run a stale `ductus.md` aborts, which never reaches the Closing restart, the abort carries the notice instead.
4. The run **continues**: the binary is on disk and the CLI is permission-seeded, so every remaining primitive is reachable as `.ductus/bin/ductus <primitive>`, and stopping here would defer the archive fetch, the migrations and the scaffolding to another session for nothing (`048-govern-acquired-runtime`'s scenario `state-b-continues-in-session`). A stale `ductus.md` is the only pre-flight contributor that aborts.

The only writes State B performs are the additive MCP-wiring entry and the additive permission entries, alongside the store acquisition and the pointer. State B never triggers a restart mid-run: when a stale `ductus.md` aborts in the same phase, its notice carries State B's file list too, and otherwise the wiring notice waits for the Closing restart.

State B issues **no separate consent prompt** before writing. The writes are additive and idempotent, and silent like the **Permission Setup** writes, matching the procedure's no-extra-prompts rule; the notice's file list is the disclosure, and the user reverses any change through git (for tracked files) or by deleting the additive `ductus` entry. There is no opt-out flag for auto-wiring — it is deliberately out of scope for this spec.

## MCP Wiring

The wiring target is **per-agent, not layout-derived** — `framework/bootstrap/ductus.md` §MCP registration is the canonical table of each agent's registration descriptor (target, scope, `mechanism`), and §Derived values deliberately carries no MCP row. Each agent is `write-file`: `.mcp.json` at the repo root for Claude, and for Pi the `.pi/extensions/ductus.ts` extension bridge, copied from the framework archive once it is extracted (`064-pi-host-support`). Where an MCP config is written, the entry registers the server as `ductus` running the runtime's MCP subcommand; Pi's bridge spawns that subcommand itself.

The write **updates an existing configuration in place — it never replaces or truncates the file.** It is additive and idempotent:

- An existing wiring file with other `mcpServers` entries keeps every one of them, and every other top-level key; only a missing `ductus` entry is added under `mcpServers`.
- An existing wiring file that already contains a `ductus` entry is left unchanged — re-running does not duplicate or overwrite it.
- An existing wiring file that has no `mcpServers` key gets the key added with just the `ductus` entry, preserving all other top-level keys.
- A missing wiring file is created containing just the `ductus` entry.
- An existing wiring file that is **not valid JSON is left untouched** — the gate skips wiring, warns the user to repair it, and the run continues through the CLI; the next run wires it once the file parses. A hand-maintained config is never clobbered.

Reversal of the wiring follows the written file's tracking status. Claude's `project-committed` repo-root `.mcp.json` is a committed repo file, so `git restore` applies to it. Pi's bridge is `project-local` and gitignored, so deleting it reverses it, and the next `/ductus` run writes it again.

Because the write is idempotent, State B is self-limiting: once it has run, the next session is either State A (the entry took effect) or — if the binary was removed between runs — State B again, which re-acquires it.

Wiring is not gated on a version-compatibility check of an installed binary: a run compares the runtime with the framework's pin before it wires. Acquisition replaces a store binary at another version, State A re-acquires a live runtime that is stale, and a `[runtime] path` at another version is wired with a warning, so a too-old store binary no longer reaches a session. [Spec 022's **Versioning enforcement** decision](../022-deterministic-runtime/spec.md) governs the runtime itself, which compares no versions at startup: it rejects startup version comparison and refuse-on-mismatch in favor of lockstep tagged releases plus the runtime's own parse-error path, and any primitive that cannot run falls back to that step's shell specification, except `extract-archive`, `apply-manifest` and `enforce-manifest`, which halt the run (AC20).

## Permission Setup

In State B the agent's settings must also grant permission to call the `ductus` MCP tools, whether or not a wiring file was written (`framework/bootstrap/ductus.md` §State B — runtime not live: acquire, wire, restart, step 4 makes the write the same for every `mechanism`; Pi has no permission entries), so the next session can use the deterministic path without a permission prompt per tool. These entries are added to the same per-layout settings file the existing **Permission Setup** section manages, using the additive rules of its merge step: add only missing entries, never remove, reorder, or overwrite entries the user or `/{project}:configure` previously added.

Separately, the store probe `~/.ductus/bin/ductus --version` used by pre-flight detection is added to the bootstrap's existing pre-grant seed of bootstrap shell commands, so the probe does not prompt on routine runs, on each agent whose seed grants the runtime CLI. This seed runs before pre-flight (see **Pre-flight Phase**).

## Documentation

The README states that `/ductus` acquires and wires the runtime (AC17).

## Acceptance Criteria

- [x] AC1: When `ductus` MCP tools are callable in the session (State A), `/ductus` takes the deterministic primitive path and version-checks the live runtime against the framework's pin (048's scenario `state-a-version-checks-the-pin`), emitting no detection message when it reports the pin. A project-supplied `[runtime] path` at another version gets a warning, and a live runtime that is stale is re-acquired, the rest of the run calling it through `.ductus/bin/ductus <primitive>` and the Closing restart saying so.
- [x] AC2: `ductus` tools exposed lazily by the host (deferred schemas, not-yet-loaded) are classified as State A, not as absent.
- [x] AC3: State A is reached by introspecting the agent's own tool inventory (per-host `ductus` tool prefix) — no shell probe and no added permission are required to detect State A.
- [x] AC4: When the store probe cannot run (no shell granted) or is denied, the run is classified as State B and acquisition proceeds — detection never hard-fails.
- [x] AC5: The store probe `~/.ductus/bin/ductus --version` is part of the pre-granted shell-command seed of each agent whose seed grants the runtime CLI (Claude), so it does not prompt on routine runs; Pi has no permission prompts and no seed. The `command -v` entry stays in the seeds, and no step runs it (`framework/bootstrap/ductus.md` §Permission Setup).
- [x] AC6: When no `ductus` MCP tools are available to the session (State B), `/ductus` registers the runtime per the agent's MCP `mechanism` — writing the MCP file (`.mcp.json` for Claude, and for Pi the `.pi/extensions/ductus.ts` bridge, written once the archive is extracted). The target is per-agent (`framework/bootstrap/ductus.md` §MCP registration (per-agent)).
- [x] AC7: In State B, the write preserves every pre-existing `mcpServers` entry and is idempotent — a wiring file that already contains a `ductus` entry is left byte-unchanged.
- [x] AC8: A run compares the runtime with the framework's pin before it wires, in State B's acquisition (048's AC6 and AC7) and in State A's version check, re-acquiring on a mismatch; only a `[runtime] path` at another version is wired as it stands, with a warning (048's AC18). The runtime itself compares no versions at startup (022's *Versioning enforcement* decision).
- [x] AC9: An existing wiring file with no `mcpServers` key gains the key with just the `ductus` entry, preserving all other top-level keys.
- [x] AC10: An existing wiring file that is not valid JSON is left untouched — the gate skips only the registration, warns the user, and the run continues through the CLI (no clobber).
- [x] AC11: In State B, `/ductus` tells the user that `ductus` was wired, that it reduces token use, and names every file it wrote, then asks for one restart. The notice rides the **Closing restart** after the run's work completes — the archive is fetched and scaffolding runs in the same session, because the acquired binary is reachable through the CLI (048's scenario `state-b-continues-in-session`) — where it says the restart is for the MCP tool surface, not for unfinished work; on a run a stale `ductus.md` aborts, the abort carries it.
- [x] AC12: State B performs no separate consent prompt before writing the wiring and permission entries (additive, idempotent); disclosure is the file list in State B's notice, which rides the **Closing restart**, or the abort on a run a stale `ductus.md` stops.
- [x] AC13: In State B, the agent's per-layout settings file gains the permission entries needed to call `ductus` tools, added additively (no existing entry removed or reordered).
- [x] AC14: When the `ductus` binary is not in the store, `/ductus` acquires it rather than proceeding on the markdown-only path, and a failed acquisition halts the run (`framework/bootstrap/ductus.md` §ductus runtime detection). No state of "binary absent" exists.
- [x] AC15: `ductus` detection and the ductus.md self-update check run in one pre-flight phase before the archive fetch, and the user restarts at most once even when both fire. Only a stale `ductus.md` aborts, and State-B wiring defers its notice to the Closing restart, so when both fire the stale abort carries State B's file list, and when only State B fires there is no abort to batch into. The guarantee does not reach Pi: its bridge is written only after the archive is extracted, which an aborted run never reaches, so a never-wired Pi adopter whose run aborts on a stale `ductus.md` restarts once more (`064-pi-host-support`).
- [x] AC16: Every agent in `framework/bootstrap/ductus.md` §Agent Registry is covered by the above. The registry holds two agents across two layouts (`claude-style`, `pi`, the latter since `064-pi-host-support`), and the coverage question is answered per agent by its MCP `mechanism`: Pi is a `write-file` agent whose file is its extension bridge, and has no permission entries to add, having no permission-gating settings.
- [x] AC17: The README states that `/ductus` acquires and wires the runtime: its Quick start and Installing (per agent) sections say so, and its runtime section describes the store and the pointer.
- [x] AC18: Project inputs (`name`, `description`, `languages`) are persisted in `.ductus/config.toml`'s `[project]` table and resolved at §Collect Project Inputs after the Pre-flight Phase — reading existing values back and prompting only for what is missing — so they are asked at most once across the State B / stale-`ductus.md` restart and never re-asked on update runs. (`host.project` is the derived runtime namespace, written from `project.name`.) *(scenario: [project-inputs-asked-once](scenarios/project-inputs-asked-once.md))*
- [x] AC19: The markdown-path archive fetch uses the direct `codeload.github.com` endpoint (no 302 redirect), so one `curl` fetches it. No seed pre-grants `curl` (it can write anywhere with `-o`; 022's task 18 question 2), so on Claude in Manual mode the fetch is one prompt, which the direct endpoint keeps from being two. *(scenario: [archive-fetch-direct-codeload](scenarios/archive-fetch-direct-codeload.md))*
- [x] AC20: State A (ductus live) is a binding execution contract: every primitive-backed step is performed by calling its `ductus` MCP tool — or as `.ductus/bin/ductus <primitive>` when the version check found the live runtime stale, the live server being the old binary (`framework/bootstrap/ductus.md` §State A — runtime live this session) — and the shell commands under those steps are not to be executed in either state. Non-primitive steps (gitignore curl, git config, input prompts) run as shown in every state. A primitive call that cannot run falls back to that one step's shell spec, except a call to `extract-archive`, `apply-manifest` or `enforce-manifest`, whose shell specs carry none of their path screens (`BE-INPUT-004`, deferred from 007's analysis): it halts the run, naming the primitive and its error. A primitive that refuses its input, such as an archive entry `extract-archive` refuses, halts the run naming it, since the shell spec has none of its screens. *(scenario: [state-a-deterministic-path-forcing](scenarios/state-a-deterministic-path-forcing.md))*
- [x] AC21: A `/audit` family (`scripts/audit/runtime-probe-parity.sh`, Family 15) guards that each agent whose §Agent Registry `settings_template` seed grants the runtime CLI also seeds the store probe, `~/.ductus/bin/ductus --version` and its `.exe` twin, as a fixed-string check in each agent's native grammar scoped to its own registry row, and skips Pi by name, which has no seed (owner decision, 2026-11-10). *(scenario: [runtime-probe-parity-audit](scenarios/runtime-probe-parity-audit.md))*

## Open Questions

*None — all resolved.*

## Resolved Questions

These record the decisions as they now stand; `048-govern-acquired-runtime`'s amendment (see the **Post-completion note** at the foot of this spec) is folded into each.

- **Compounding restarts.** *Resolved:* unify `ductus` detection and the ductus.md self-update check into a single **pre-flight phase** that sorts every restart-requiring write. Ordering alone cannot fix the compounding — a stale + unwired adopter hits two independent write-and-abort points regardless of order — so the paths are merged, not reordered. Only a stale `ductus.md` aborts the run; State-B wiring defers its notice to the **Closing restart** (AC15), so the worst case is one restart, plus a stale abort that carries State B's file list. On a genuine first run the just-curled `ductus.md` is already current, so self-update does not abort and only the ductus restart fires.
- **Consent to modify agent config.** *Resolved:* no separate consent prompt. State B writes the wiring and permission entries additively and idempotently — silent like the **Permission Setup** writes, and consistent with ductus.md's "no extra prompts" procedural-fidelity rule — and the **Closing restart** message (or the abort a stale `ductus.md` raises) names every file written as the disclosure. The user reverses any change via git or by deleting the additive `ductus` entry. An opt-out flag for auto-wiring is explicitly out of scope for this spec.
- **claude-style `.mcp.json` location.** *Resolved:* repo-root `.mcp.json` is correct — it is the project-scoped MCP config Claude Code reads, Claude's `project-committed` target in `framework/bootstrap/ductus.md` §MCP registration (per-agent). Coexistence with an existing config is the additive/idempotent in-place update: preserve all existing servers and top-level keys, add only a missing `ductus` entry, add the `mcpServers` key if absent, leave an existing `ductus` entry unchanged, and never clobber a malformed-JSON file (skip the registration + warn; the run continues through the CLI). Reversal is per written file — `git restore` for the tracked `.mcp.json`. The JSON-merge is **always host-side**, because State B is by definition the case where the runtime is not live, so no primitive is available to perform it — ductus.md §MCP wiring states this.
- **Version compatibility.** *Resolved:* the bootstrap compares the runtime with the framework's pin and re-acquires on a mismatch (see **MCP Wiring**); the runtime itself compares no versions at startup, since §runtime-boundary's Versioning subsection rejected startup version comparison and refuse-on-mismatch in favor of lockstep tagged releases plus the runtime's parse-error path. `runtime-tools.txt` carries only tool names (no version floor).
- **Detection mechanism across hosts.** *Resolved:* two independent probes. State A is reached by the agent introspecting its own tool inventory for a `ductus`-namespaced MCP tool (`mcp__ductus__*` on Claude, a `ductus__*`-prefixed extension-bridge tool on Pi), counting deferred/lazy names as present — no shell, no permission. State B is reached by the store probe `~/.ductus/bin/ductus --version`, run only when introspection finds no tool and seeded into the existing Permission Setup pre-grant so it does not prompt. If the probe cannot run or is denied, the run is classified as State B and acquires; acquisition is idempotent, so detection never hard-fails on a locked-down host.

## Post-completion note — two detection states

> Amended by [048 — Ductus-Acquired Runtime](../048-govern-acquired-runtime/spec.md).

Spec 048 makes acquisition `/{project}`'s job, so detection has two states, A and B, and none for an absent binary.

- **A missing binary is work to perform, not a state to report.** It is acquired, then wired and permissioned exactly as State B does, joining the same pending-restart set. An adopter with no runtime reaches the deterministic path in one run plus one restart, with no manual install in between.
- **State B does not abort.** 048's scenario `state-b-continues-in-session` is the record. After acquisition the binary is on disk and the CLI is permission-seeded for each subcommand a remaining step runs, so each of those primitives is reachable as `.ductus/bin/ductus <primitive>` (the seed names each such subcommand rather than the whole CLI since [022](../022-deterministic-runtime/spec.md)'s task 18, without a prompt on Claude; Pi has no permission prompts), and the abort bought nothing the CLI could not already give that run. The pending-restart set's two contributors are inspected separately: a stale `ductus.md` still aborts, because the run must not continue on instructions it has just replaced, while State-B wiring carries its notice to the **Closing restart** after the work completes. That restart is for the MCP tool surface, and the message says so, to stop an operator re-running `/ductus` expecting it to do more.
- **The binary probe** is a filesystem check on the ductus-owned store; `PATH` is not consulted at all — an adopter's own `ductus` on `PATH` is ignored, not warned about, not removed.
- **A probe that cannot run** classifies the run as State B and acquisition proceeds; acquisition is idempotent, so the false negative costs a version comparison rather than a redundant download.
- **No post-scaffolding tip suggests installing `ductus`**: an acquisition that fails halts the run with a message naming the store path, the `[runtime]` key and the release URL (048's Acquisition).

**State A keeps its binding execution contract**: detecting the runtime and then walking the prose path anyway remains the failure this spec exists to prevent. It version-checks the live runtime (048's scenario `state-a-version-checks-the-pin`): a live runtime at another version than the pin is re-acquired, and the rest of the run calls it through `.ductus/bin/ductus <primitive>` rather than through the stale server's tools.
