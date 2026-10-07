---
section: "Follow-on scenarios"
---

# Per-agent-scaffold-paths-ship-to-adopter

## Context

064's analyze (2026-09-22) flagged its AC1 and AC3 advisory under
`criterion-path-existence`: they name `.pi/prompts/ductus.md` and
`.pi/settings.json`, paths `/ductus` writes into an adopter's checkout. Both
criteria are correct. They failed here for the reason
[criterion-adopter-scope-destinations](criterion-adopter-scope-destinations.md)
already handles — this repo is the framework source, not an adopter — but that
scenario's suppression did not reach them.

It derived the ships-to-adopter set from the **Shared Files** manifest tables
alone, and a per-agent scaffold path is not a Shared Files row. It is computed
from §Agent Registry in `framework/bootstrap/ductus.md`: the agent's
`config_dir` substituted into a layout-derived formula. In the manifest those
formulas appear only as placeholder destinations, which that scenario's edge
cases call inert — they entered the set and could never match.

`.pi/` is dogfooded here, so the `.pi` segment exists and `root-absent` did
not fire, while no generator materializes the self-install or the settings
file. 032's OpenCode criteria escaped the same finding only because `.opencode/`
is absent here and `root-absent` caught them; dogfooding OpenCode would have
surfaced them the same way. The gap was per-agent, not Pi-specific.

## Behavior

`adopter_destinations(repo)` also derives each registered agent's scaffold
destinations from §Agent Registry. For every registry row it substitutes the
row's `config_dir` into the layout-derived **`ductus` install path** and
**Settings file** formulas for the row's `layout`, and takes the agent's **MCP
target** when its scope is project-scoped (`project-committed` or
`project-local`). Every registered agent contributes, not only the ones
dogfooded here: the set describes what this repo ships, not what it happens to
materialize.

The suppression itself is unchanged — same arm, same position after the
resolve and `root-absent` arms, same `skipped { reason: "ships-to-adopter" }`
record. This widens the set the existing arm consults; it adds no arm and no
skip reason.

## Edge Cases

- **Derived from the registry, never listed.** Adding an agent, as 064 added
  Pi, extends the set with no change to the check. A hand-enumerated list of
  `.pi/…` and `.opencode/…` paths would be the second copy §drift-prevention
  rules out, stale at the next agent.
- **A formula still carrying a placeholder after `config_dir` is substituted
  stays inert.** The Command/skill path keeps `{project}` and `<name>`, so it
  cannot match a candidate, exactly as today.
- **A cell with a trailing note contributes its backticked path.** The Pi
  Settings file cell and the OpenCode `opencode.json` cell carry prose beside
  the path; the note is not part of the destination, and a cell-is-exactly-one-
  span rule would silently drop both.
- **A repo-root destination** (`opencode.json`, `.mcp.json`) enters as-is and
  is inert, for the reason
  [criterion-adopter-scope-destinations](criterion-adopter-scope-destinations.md)
  gives for any destination with no `/`: the criterion path grammar requires
  one, so no candidate can match it.
- **A gitignored project-local target** (`.pi/extensions/ductus.ts`) exists in
  a working checkout after `/ductus` but not in CI's clean clone. Including it
  makes the verdict the same in both: it resolves locally and is suppressed in
  CI, rather than flagging in one and passing in the other.
- **User-global and home-level MCP targets are excluded** (`~/.augment/…`,
  `~/.gemini/…`). They live outside any checkout, so no criterion about them is
  a repo-relative path claim.
- **Derivation failure still fails toward reporting, per table.** An
  unreadable registry yields no agent destinations, so findings are emitted
  rather than swallowed; the Shared Files contribution is independent and
  survives it, and the reverse holds.
- **A path no registry row derives still flags.** A `.pi/` path that is
  neither the install path, the settings file, nor the MCP target is reported —
  the suppression is scoped to declared destinations, not a blanket
  `{config_dir}/` exemption.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
