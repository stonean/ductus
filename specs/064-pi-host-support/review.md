---
spec: 064-pi-host-support
last-run: 2026-10-07T01:59:40Z
reviewed-against: fed95a442b626a81faf232e9ff131ba6176edf20
diff-base: ad476381c1966403ebaac595b09beaca8e910fdd
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 11
scope: 129
skipped-passes: []
reviewed-digest:
  scenarios/pi-layout-is-dispatched.md: 0d0d2a6867c610e4d55252aa82fdf7a449ab0735ec16f47484a8c8dc332595fc
  scenarios/the-bridge-fails-loudly-and-recovers.md: 67570df65ceffa494936a5282eb62c100f14c125065562038aaa771077b1bf45
blocking: false
dispositions:
  fixed: 2
  routed: 0
  discarded: 1
  undispositioned: 0
decisions:
  - key: "other: pi 1.0.4 ships a native MCP client (.pi/mcp.json), so the premise that pi has none — and the bridge built on it — is out of date — `framework/bootstrap/ductus.md`"
    outcome: discarded
    reason: "set aside by operator decision 2026-10-07: the bridge still serves pi as an extension, and moving pi to native MCP registration through .pi/mcp.json is an optional later change, not a defect in this release"
    decided-at: 2026-10-07T01:59:40Z
    decided-by: andy@stone.dev
---

# Review — 064-pi-host-support

## Summary

All five passes ran over 064's code since its previous review (reviewed-against ad476381), stamped against fed95a44. Reviewed with `--since=ad476381`: the default window opens at 6c2edf81 and holds none of the bridge rewrite (991956e2) or the configure repair (a1e42693), which landed after that review and before the window's base. 0 MUST, 0 SHOULD, 0 low-confidence; not blocking. Read: `framework/bootstrap/pi/ductus-bridge.ts`, `scripts/tests/pi-bridge-harness.mjs`, the in-window diffs of `runtime/src/host.rs`, `framework/bootstrap/configure/pi.md`, `scripts/audit/host-namespace-parity.sh`, `scripts/audit/installer-registry-parity.sh` and `scripts/audit/README.md`, and 064's `spec.md`, `plan.md`, `tasks.md` and `scenarios/the-bridge-fails-loudly-and-recovers.md`. Not counted though in scope: `.pi/extensions/ductus.ts`, byte-identical to the bridge source (cmp); the window's 022, 000 and 065 changes, each reviewed under its own spec; and the paths it carries from the merge of main into PR #5. Three observations: the bridge's per-chunk stdout decoding corrupted a multi-byte character split across chunks (reproduced against a 250 KB spec) and a timed-out request left a hung runtime serving every later call — both fixed in fed95a44 with operator confirmation, each pinned by a harness case shown failing against the previous bridge; and pi 1.0.4's native MCP client, which dates the bridge's premise, discarded by operator decision.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- bug: the bridge decoded the runtime's stdout chunk by chunk, so a multi-byte character split across pipe chunks was corrupted — `framework/bootstrap/pi/ductus-bridge.ts` — **fixed**
- bug: a request that timed out left the hung runtime as the connection, so every later call waited out the full 120 s cap — `framework/bootstrap/pi/ductus-bridge.ts` — **fixed**
- other: pi 1.0.4 ships a native MCP client (.pi/mcp.json), so the premise that pi has none — and the bridge built on it — is out of date — `framework/bootstrap/ductus.md` — **discarded**: set aside by operator decision 2026-10-07: the bridge still serves pi as an extension, and moving pi to native MCP registration through .pi/mcp.json is an optional later change, not a defect in this release

## Skipped passes

*None.*

## Unexamined governance

*None.*
