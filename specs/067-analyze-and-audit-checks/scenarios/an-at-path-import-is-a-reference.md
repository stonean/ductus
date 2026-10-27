---
section: "Link and reference integrity"
---

# An-at-path-import-is-a-reference

## Context

`check-orphaned-references` finds a managed path in a referrer by scanning for a managed-root prefix at the start of a token. It treated a prefix preceded by `@` as glued to a word, so `@.ductus/constitution.md` was never a candidate. That cost nothing while the `CLAUDE.md` template wrote its imports as `@import <path>`, where a space comes before the path. Claude Code imports a file only as `@path`, though. [002 — Project Scaffolding](../../002-project-scaffolding/spec.md)'s `claude-md-template-imports-load` scenario moved the template, the bootstrap's shared-constitution block and an adopter migration to that form. A dangling import in that form loads nothing and would have gone unreported, which is the failure this check exists to surface.

## Behavior

**An `@` that starts a token begins a reference.** When the character before a managed-root prefix is `@`, and the `@` is at the start of the line or follows whitespace, the prefix starts a candidate, exactly as one preceded by a space does. So `@.ductus/missing.md` on its own line, or after a space, is reported when the path does not exist.

**An `@` glued to a word does not.** When anything other than whitespace precedes the `@`, as in `x@.ductus/y`, the prefix is inside a token and is excluded, the way `x.ductus/y` is.

## Edge Cases

- **Punctuation before the `@`.** `(@.ductus/x)` and `"@.ductus/x"` are excluded, because only whitespace or the line start begins a token here. Claude Code reads neither as an import written by the template or the bootstrap, which write the import at the start of a line.
- **A byte-order mark.** A U+FEFF before an `@path` on a file's first line is not whitespace, so that line is excluded. The template carries no BOM.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
