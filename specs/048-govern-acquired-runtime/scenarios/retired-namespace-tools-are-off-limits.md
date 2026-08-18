---
section: "Follow-on scenarios"
---

# Retired-namespace-tools-are-off-limits

## Context

A pre-rename adopter reaches `/ductus` with the retired server key still in
their MCP config *and* a retired binary still resolvable. Both halves persist by
design: `ductus-rename` explicitly declines to touch an installed binary, and the
retired crate is left published rather than yanked so existing installs keep
resolving. The agent host launches that server at session start, so the very run
that is about to retire the key executes with the retired runtime's tools live in
its inventory for the whole session.

Detection is unaffected and behaves correctly. Tool-inventory introspection is
scoped to `ductus`-namespaced tools, so a retired-namespace inventory yields no
match, the store probe runs, and the run classifies **State B** — acquire, wire,
continue. The gap is downstream of detection: §State B directs the host to invoke
every remaining primitive as `{pointer-path} <primitive>`, but never says the
retired tools are off-limits, and the bootstrap procedure does not mention the
retired namespace at all. A host that reaches for a live tool over a CLI call is
following the general preference for the deterministic path, not disregarding an
instruction.

## Behavior

For the remainder of a State-B run, the host MUST ignore every MCP tool whose
namespace is not `ductus`, and reach the runtime only through the pointer CLI. A
retired-namespace tool is treated as absent: not called, not preferred over the
CLI, and never read as evidence that a runtime is available.

The justification is the one that already makes namespace-scoped detection
correct. A retired-namespace server is a *different runtime at a different
version*, and its primitives resolve paths against the directory layout of the
release that shipped them. During the single run that migrates that layout, those
resolvers are wrong by construction — the migration is precisely what makes them
wrong — so any write they perform lands in the pre-migration location, silently
and with a success result.

## Edge Cases

- **The retired server is live but never called.** No effect. The requirement
  binds the host's choice of surface, not the adopter's configuration, so it
  costs nothing on a run that already uses the CLI.
- **The retired key is removed mid-run, but its server stays live.** An MCP
  server is spawned at session start and is not torn down when its registration
  is deleted, so the tools remain in the inventory after the rename step has
  removed the key. The rule therefore binds for the whole run, not until that
  step.
- **A permission entry auto-added for a retired tool after the rename's
  permission rewrite has run.** That rewrite converges the entries present when
  it executes, so a later addition is unreachable by it and survives. Declining
  to call the retired tools removes the cause; a second sweep at end of run would
  only treat the symptom.
- **A retired-namespace tool that resolves no paths** — a pure parse, say — is
  still off-limits. The rule is namespace-scoped rather than per-tool, so it
  needs no per-primitive risk assessment and cannot be eroded one exception at a
  time.
- **`surface-instruction` agents.** Their MCP config lives at home level, so a
  retired server can be live in a project whose MCP file this run never writes.
  The rule binds identically: it governs which tools the host calls, not which
  file registered them.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
