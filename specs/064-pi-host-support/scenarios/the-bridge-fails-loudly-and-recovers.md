---
section: "Design Decisions"
---

# The-bridge-fails-loudly-and-recovers

## Context

The bridge (D3) is Pi's whole tool surface: it spawns `.ductus/bin/ductus mcp` and registers every tool the runtime lists. The tool list comes from the runtime, so when the runtime cannot start there is nothing to register — and a wired host that lost its binary must stop rather than degrade (§runtime-boundary).

## Behavior

- **The runtime cannot start at load** (the pointer is missing or not executable): no `ductus__*` tool is registered, and session start shows a notice naming the pointer and pointing at `/ductus`. The notice arrives within seconds — a failed spawn is detected at once, not after the request timeout — and pi's own process sees no uncaught exception.
- **The runtime exits mid-session**: the call in flight fails with an error envelope, and the next call spawns a fresh runtime and runs the MCP handshake again before calling.
- **The binary goes missing mid-session**: the call returns an error envelope naming the pointer and pointing at `/ductus`. Once the binary is back, calls succeed again — one failed call never fails the calls after it.

## Edge Cases

- No placeholder tool stands in for a runtime that cannot start: `/ductus` reads any `ductus__*` tool as State A, so a placeholder would stop it from re-acquiring the runtime that is missing.
- The handshake's `notifications/initialized` is a JSON-RPC notification — no `id`, and no response awaited.
- `scripts/tests/pi-bridge-harness.mjs` drives these cases through a mock pi against a runtime build. It does not exercise pi's own extension loader; only a real `pi` run does (AC14).

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
