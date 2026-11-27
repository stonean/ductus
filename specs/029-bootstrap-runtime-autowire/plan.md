# 029 — Bootstrap Runtime Auto-Detect and Wire Plan

Implements [029 — Bootstrap Runtime Auto-Detect and Wire](spec.md).

## Overview

This feature is a change to the bootstrap **procedure prose** plus the per-agent **permission seeds** — there is no runtime (Rust) code to write. State B (runtime not live) is by definition the runtime-absent case, so every action it performs is host/markdown-side; no MCP primitive is introduced for it.

The spine is `framework/bootstrap/ductus.md`: a **Pre-flight Phase** runs `ductus` detection and the ductus.md self-update check, and sorts each check's restart-requiring writes into the **pending-restart set** (a stale `ductus.md`, which aborts the run) or the **deferred-restart set** (State B's acquisition and wiring, and State A's acquisition or pointer repair, carried to the **Closing restart**). `/ductus` acquires the runtime into a ductus-owned store, materializes a per-project pointer to it, and wires the MCP config to that pointer (`048-govern-acquired-runtime`). Each agent whose `settings_template` seed grants the runtime CLI also seeds the store probe, and an audit family holds that.

## Technical Decisions

### Pre-flight Phase

`framework/bootstrap/ductus.md` §Pre-flight Phase sits **after the §Permission Setup seed** (so the probe is pre-authorized) and **before `framework/bootstrap/ductus-procedure.md` §Pre-run Migrations and the archive fetch**. It runs two checks, `ductus` detection and the self-update byte-compare, and sorts every restart-requiring write into the two sets above. **Pre-flight abort** inspects both: a stale `ductus.md` stops the run at once, carrying State B's file list; State-B writes alone do not stop it, because the binary is on disk and every remaining primitive is reachable as `.ductus/bin/ductus <primitive>` (048's scenario `state-b-continues-in-session`). If neither check needs a restart, the run proceeds to Pre-run Migrations.

Rationale: ordering the two checks cannot fix the compounding restart of a stale + unwired adopter, so both run before the abort is inspected (spec Resolved Questions, **Compounding restarts**).

### State A — tool-inventory introspection, then a version check

The agent inspects its **own** available-tool inventory for any `ductus`-namespaced MCP tool — `mcp__ductus__*` (Claude) and the `ductus__`-prefixed bridge tools (Pi) — treating deferred/lazily-loaded tool names as present. Any match is State A and the deterministic path, with no shell call and no permission. The live runtime is then version-checked against the framework's pin (048's scenario `state-a-version-checks-the-pin`): one reporting the pin is silent; a stale one is re-acquired and the rest of the run calls it through `.ductus/bin/ductus <primitive>`.

### State B — the store probe

Only when introspection finds no `ductus` tool, the store probe `~/.ductus/bin/ductus --version` (`ductus.exe` on Windows) runs; `PATH` is not consulted. A missing binary is acquired, and a probe that cannot run or is denied classifies the run as State B, so detection never hard-fails. The probe is seeded into the `settings_template` of each agent whose seed grants the runtime CLI: Claude (`permissions.allow`) takes `Bash(~/.ductus/bin/ductus --version)` and its `.exe` twin; Pi has no permission gate and seeds nothing (scenario `runtime-probe-parity-audit`). The seed's `Bash(command -v *)` entry stays, and no step runs it.

### MCP wiring — additive, idempotent, in-place JSON merge (host-side)

The target is per-agent (`framework/bootstrap/ductus.md` §MCP registration (per-agent)): `.mcp.json` at the repo root for Claude. The entry names the repo-relative pointer:

```json
{ "mcpServers": { "ductus": { "command": ".ductus/bin/ductus", "args": ["mcp"] } } }
```

Merge rules (update in place, **never** replace or truncate):

- File missing → create with just the `ductus` entry.
- File present, has `mcpServers`, no `ductus` → add `ductus`, preserve all other servers and top-level keys.
- File present, has `ductus` → no-op (byte-unchanged).
- File present, no `mcpServers` key → add the key with just `ductus`, preserve other top-level keys.
- File present, **malformed JSON** → do not touch; skip the registration and warn; the run continues through the CLI.

No `merge-mcp-config` primitive exists: State B is runtime-absent by definition, so the write is always host-side.

### ductus tool permissions granted at wiring time (State B)

Alongside the wiring write, State B adds the `ductus` tool permission to the agent's settings file (additively) so the **next** session calls the tools prompt-free: `mcp__ductus__*` for Claude, the minimal bootstrap grant; Pi has no permission entries. The enumerated per-tool set lives in the generated `<!-- generated:mcp-allow -->` block of `framework/bootstrap/configure/claude.md` and is applied by `/{project}:configure`; the wildcard coexists harmlessly with it (exact-match dedup leaves both; `framework/bootstrap/ductus.md` §ductus runtime auto-wiring).

### No confirmation prompt

`framework/bootstrap/ductus.md` §ductus runtime auto-wiring describes the acquire-and-wire behavior. **No confirmation prompt** is introduced — the allowed-prompts list in §Instructions' *Procedural fidelity* preamble is unchanged; disclosure is the **Closing restart** message, which names every file written (the pre-flight abort carries it on a run a stale `ductus.md` stops).

### Version check; no data-model.md

A run compares the runtime with the framework's pin before it wires (048); the runtime itself compares no versions at startup (022's *Versioning enforcement* decision). The feature introduces no domain entities or persisted data structures; the two-state enum and the JSON/permission snippets live in this plan, so **no `data-model.md` exists**.

### Parity enforcement

`scripts/audit/runtime-probe-parity.sh` (Family 15, in `scripts/audit/run-all.sh` and `framework/reference/audit.md`) holds that each agent whose seed grants the runtime CLI also seeds the store probe, scoped to that agent's own §Agent Registry row (scenario `runtime-probe-parity-audit`).

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/bootstrap/ductus.md` | Modify | **Pre-flight Phase** (detection and self-update, two restart sets); State A and State B detection; **MCP wiring**; the State-B tool-perm grant; the store probe in each granting agent's `settings_template`; the **Closing restart** notice |
| `framework/bootstrap/configure/claude.md` | Modify | Canonical allow set carries the `ductus` tool permissions (generated block) |
| `README.md` | Modify | Quick start and Installing (per agent): `/ductus` acquires and wires the runtime |
| `scripts/audit/runtime-probe-parity.sh` | Create | Family 15: each granting agent seeds the store probe |
| `specs/029-bootstrap-runtime-autowire/plan.md` | Create | This plan |
| `specs/029-bootstrap-runtime-autowire/tasks.md` | Create | Task breakdown |

## Trade-offs

- **Merge vs. order the two pre-flight checks.** Chose one phase that runs both before inspecting the sets. Ordering alone leaves a stale + unwired adopter with two restarts. (Resolved Questions, **Compounding restarts**.)
- **Probe in the always-applied seed vs. only at State B.** The *store probe* permission goes in the always-applied seed (it must be authorized before the state is known); the *ductus tool* permissions go in at State-B wiring time (they only matter once wired).
- **No `merge-mcp-config` runtime primitive.** Rejected: State B is runtime-absent by definition, so a primitive could never run there. Revisit only if a future "re-assert wiring while runtime is live" need appears.
- **Wildcard vs. enumerated ductus tool grant at wiring time.** Chose the per-agent wildcard for the bootstrap grant (minimal, robust); the enumerated set stays owned by the generated configure block applied by `/{project}:configure`.
- **Known limitation — mid-session settings reload.** Seeding the probe permission only avoids a prompt if the host re-reads its settings file mid-session (the same assumption `framework/bootstrap/ductus.md` §Permission Setup relies on). On a host that doesn't, the probe prompts once on first run.
- **Known limitation — no automated test for agent-driven detection.** The two-state logic is prose the agent executes; verification is the audit parity check (static parts) plus a manual walk-through of each state. No runtime unit test is added.
