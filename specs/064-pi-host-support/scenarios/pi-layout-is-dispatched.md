---
section: "Design Decisions"
---

# Pi-layout-is-dispatched

## Context

Pi's command surface is prompt templates installed by the bootstrap's Per-Agent Scaffolding. The install lives in the `### Pi layout` section, which is reached only through the dispatch prose at the top of that section — the lines that route each registry `layout` to its section.

## Behavior

When a `pi`-layout adoption runs Per-Agent Scaffolding, the dispatch prose must route `pi` to `### Pi layout`, so the sixteen pipeline commands plus `configure` install to `.pi/prompts/{project}-{name}.md`. A missing `pi` dispatch line leaves that section unreachable: the adoption falls through to the claude-style `### Slash commands` path and installs no `{project}-*.md` command set.

## Edge Cases

- The `ductus.md` self-install is a separate, always-run step with its own inline `pi` branch, so it lands even when the dispatch line is missing — leaving a pi adoption with only `ductus.md`, which reads as complete.
- The sibling sections (self-install, self-update check, post-write integrity, placeholder substitution) each carry their own `pi` branch independent of the dispatch, so only the command-file scaffold is affected by a missing dispatch.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
