---
section: "Parameterization"
---

# Analyze-reads-the-layout-derived-command-set

## Context

[Parameterization](../spec.md#parameterization) states that an installed
command's path is layout-derived and is not spelled out in the command:
`framework/bootstrap/ductus.md` §Derived values carries it per layout.
`framework/commands/analyze.md` did not follow that. Its Scope Boundaries read
list (line 34) and its §Project-level consistency read inputs and **Command
frontmatter completeness** check (lines 428–436) named
`{cli-config-dir}/commands/{project}/help.md`, every `.md` in
`{cli-config-dir}/commands/{project}/`, and `{cli-config-dir}/commands/ductus.md`.

That shape is right for `claude-style` alone. OpenCode installs to
`command/{project}/` (singular), Antigravity to
`skills/{project}-<name>/SKILL.md`, and Pi (064) to flat
`prompts/{project}-<name>.md` with the self-install at `prompts/ductus.md`. For
those adopters the checks read a directory that does not exist, examined
nothing, and passed — a check that could not tell "clean" from "never
looked".

This surfaced on 2026-10-05 reviewing PR #5, when the dogfooded
`.pi/prompts/ductus-analyze.md` came out naming `.pi/commands/ductus/`.

## Behavior

**The steps point at the derived rows rather than restating one layout.**
`analyze.md` names the installed command set by the §Derived values
**Command/skill path** row and the bootstrap installer by the **`ductus` install
path** row, for the adopter's layout, wherever it currently spells the
`claude-style` shape — the read list and the frontmatter check alike. The help
command is the `help` entry of that same set, not a separately spelled path.

**The set is the project's commands, not the directory's contents.** Membership
follows the layout's **Slash-command cleanup glob** row, which already draws
this line: `*.md` in `command/{project}/` for OpenCode, `{project}-*/` skill
directories for Antigravity, and `{project}-*.md` in `prompts/` for Pi.

**The frontmatter expected is the layout's.** `description:` is required in
every layout. `argument-hint:` is checked only where the layout carries the
source frontmatter (`claude-style`, `opencode`, `pi`, which copy it verbatim).
Antigravity's scaffolding writes a skill's frontmatter as `name:` and
`description:` alone, so an `argument-hint:` check there would flag every skill
for a field the layout never writes.

**A path that does not resolve is unexamined, not clean.** When the derived
command set or install path is absent, the check says so and names the path it
looked for, rather than reporting the frontmatter check as passed.

## Edge Cases

- **Pi's prompts directory is shared.** It also holds the adopter's own prompt
  templates. Reading every `.md` there would hold the adopter's prompts to
  ductus's frontmatter rules; the `{project}-` prefix is what scopes the check.
- **Antigravity's install path is a directory form**
  (`skills/ductus/SKILL.md`), so "the bootstrap installer lives outside the
  project namespace" holds by a different shape than a sibling file; the row
  carries that, and the step does not restate it.
- **A `claude-style` adopter sees no change.** The derived row for its layout
  is the path the step spells today, so the fix is invisible where the bug was.
- **Adding a layout costs no edit here.** A fifth layout adds a §Derived values
  column, and `analyze.md` picks it up because it points at the row. Spelling
  each layout in `analyze.md` would be the second copy §drift-prevention rules
  out, and it would go stale at the next agent.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
