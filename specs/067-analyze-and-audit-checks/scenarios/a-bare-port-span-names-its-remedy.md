---
section: "Artifact checks"
---

# A-bare-port-span-names-its-remedy

## Context

A spec says a service listens on port 8080 and writes the port alone in a code span, a colon and the number. The `line-number-citations` family reads a code span holding only `:N` as a line number set apart from its file (owner decision, 2026-11-17) and cannot tell it from a port.

## Behavior

- The finding for a set-apart colon span names the remedy: a port is written with its host, as `localhost:8080`, which is no citation. Detection is unchanged.

## Edge Cases

- A `#LN` span carries no hint, since it can never be a port; a citation attached to its path carries none either.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
