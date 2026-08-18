---
section: "Follow-on scenarios"
---

# Retired-namespace-tools-are-off-limits

## Context

An adopter reaches `/ductus` with a retired server key still in an MCP config
*and* a retired binary still resolvable. Both halves persist by design:
`ductus-rename` explicitly declines to touch an installed binary, and the retired
crate is left published rather than yanked so existing installs keep resolving.
The agent host launches that server at session start, so its tools sit in the
inventory for the whole session.

Detection is unaffected and behaves correctly — tool-inventory introspection is
scoped to `ductus`-namespaced tools, so a retired namespace never classifies a
run as State A. The gap is downstream of detection: the procedure directs the
host to the primitives (or, in State B, to the pointer CLI) but never says the
retired tools are off-limits, and it did not mention the retired namespace at
all. A host that reaches for a live tool is following the general preference for
the deterministic path, not disregarding an instruction.

**Two adopter shapes reach this, and only one of them is a migrating run.** The
first is State B: a pre-rename adopter whose retired key this very run is about
to remove. The second is State A and is the more durable of the two — for a
`surface-instruction` agent the retired key lives in the user's *home* config,
which `ductus-rename` warns about rather than rewriting, so it survives until the
user acts on that warning. Once such a user also registers `ductus`, every
subsequent session has both namespaces live, indefinitely.

## Behavior

For the whole run, in **either** state, no MCP tool outside the `ductus`
namespace may perform a step of the procedure, and none counts as evidence that
the runtime is available. The requirement governs tools that would otherwise
stand in for the runtime — a retired-namespace server above all — and says
nothing about unrelated servers an adopter has registered for their own purposes,
which the procedure never calls either way.

The justification is the one that already makes namespace-scoped detection
correct. A retired-namespace server is a *different runtime at a different
version*, and its primitives resolve paths against the directory layout of the
release that shipped them — a pre-`.ductus/` binary resolves `.govern/` and then
the legacy root, neither of which a converged project has.

How that goes wrong differs by state, which is why the rule is stated once for
both rather than twice:

- **State B** — the resolvers are wrong *by construction*, because this run is
  what migrates the layout. A write lands in the pre-migration location and
  reports success.
- **State A** — the layout already moved, so the retired resolver falls through
  to a path the project no longer has, and the silent default that follows is
  the same failure the shipped `config_path_of` had before it gained a
  `.ductus/` tier.

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
- **An unrelated MCP server the adopter registered for their own work.** Out of
  scope, and deliberately so — the requirement is about what may stand in for the
  runtime, not a blanket prohibition on the host's other tools. Reading it as the
  latter would make the procedure claim authority over the adopter's environment
  that it neither needs nor has.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
