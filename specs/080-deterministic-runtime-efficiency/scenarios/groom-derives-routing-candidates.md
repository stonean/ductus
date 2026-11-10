---
section: "Behavior"
---

# Groom-derives-routing-candidates

## Context

`derive-routing-candidates` derives the existing homes a piece of work could belong to (a runtime-owning spec, a rule file, a spec sharing vocabulary), and its description leaves the semantic call at groom's `routeInboxItem` extension point. Only specify calls it. In the cost round, groom searched spec bodies with three `grep` calls, one denied, before its routing round trip (`research.md`, R6).

## Behavior

- groom derives each inbox item's candidate homes with `derive-routing-candidates` before its routing judgment, as specify does, rather than searching specs through the shell.
- The candidates are advisory. The routing call stays the model's and the decision the operator's, at the same gate.

## Edge Cases

- An item the primitive matches to nothing: the routing round trip runs as before, with no candidate shown, and whether it also searches spec bodies is the model's call. A slug list that matches nothing says nothing of what the bodies cover (`research.md`, *Routing candidates on the main path*; owner, 2026-11-10).
- A source the primitive could not examine is shown as skipped, never as a source that matched nothing.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
