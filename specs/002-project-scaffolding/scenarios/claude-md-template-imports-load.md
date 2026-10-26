---
section: "CLAUDE.md template"
---

# Claude-md-template-imports-load

## Context

On 2026-10-26, `072-governing-documents-stay-within-one-read` found that this template wrote its imports as `@import .ductus/constitution.md` and `@import AGENTS.md`. Claude Code imports a file only as `@path`, so neither line loaded anything. This repository's own `CLAUDE.md` used the same form, and a `claude -p` run with no file tools could not answer from `AGENTS.md` until the line became `@AGENTS.md`. An adopter's Claude Code sessions therefore started without their own guide; the pipeline commands still read the constitution explicitly. For `claude-style` agents the bootstrap also rendered registered shared constitutions into the template's managed block in the same form. Converting both template lines literally would have begun loading the 119,262-byte constitution, more than one read, into every session.

## Behavior

- The template imports the project's guide as `@AGENTS.md`.
- The template does not import the constitution while it exceeds the 50,000-byte read size, and the pipeline commands keep reading it when they need it. Removing the line changes nothing an adopter had, because it never loaded. Importing the constitution becomes a choice for the change that lands `050-constitution`'s cut to one read.
- For `claude-style` agents, the bootstrap renders each registered shared constitution into the managed block as `@{document-path}`.
- A migration rewrites an existing adopter's `CLAUDE.md`, which the bootstrap never overwrites because the adopter owns it. Each `@import <path>` line outside code becomes `@<path>`, and the line importing `.ductus/constitution.md` is removed.
- This spec's §CLAUDE.md template and AC3 state the form the template now writes, and every framework document that describes the old form is corrected with them.

## Edge Cases

- **A line already fixed by hand.** An adopter whose `CLAUDE.md` already reads `@AGENTS.md` is left as it is, because the migration rewrites only `@import` lines.
- **`@import` inside code.** An `@import` inside a code span or fence is text, not an import, and stays.
- **No `@import` line.** A `CLAUDE.md` with none is not touched.
- **Other agents.** Agents that read `AGENTS.md` natively, without `CLAUDE.md`, are unaffected.
- **The orphaned-reference check.** `check-orphaned-references` reports a dangling import path. It must still recognise a reference written as `@path`, and that is confirmed against the primitive before the task closes.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
