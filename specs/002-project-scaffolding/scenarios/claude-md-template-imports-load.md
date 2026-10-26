---
section: "CLAUDE.md template"
---

# Claude-md-template-imports-load

## Context

On 2026-10-26, `072-governing-documents-stay-within-one-read` found that this template wrote its imports as `@import .ductus/constitution.md` and `@import AGENTS.md`. Claude Code imports a file only as `@path`, so neither line loaded anything. This repository's own `CLAUDE.md` used the same form, and a `claude -p` run with no file tools could not answer from `AGENTS.md` until the line became `@AGENTS.md`. An adopter's Claude Code sessions therefore started without their own guide; the pipeline commands still read the constitution explicitly. For `claude-style` agents the bootstrap also rendered registered shared constitutions into the template's managed block in the same form. Converting both template lines literally would have begun loading the 119,262-byte constitution, more than one read, into every session.

## Behavior

- The template imports the project's guide as `@AGENTS.md`.
- The template does not import the constitution while it exceeds the 50,000-byte read size, and the pipeline commands keep reading it when they need it. Removing the line changes nothing an adopter had, because it never loaded. Importing the constitution becomes a choice for the change that lands `050-constitution`'s cut to one read.
- For `claude-style` agents, the bootstrap renders each registered shared constitution into the managed block as `@{document-path}`. It then loads at startup with `AGENTS.md`, so a session that starts from a pasted prompt and runs no command still has its rules.
- `/{project}:target` loads `.ductus/constitution.md` and each shared constitution only when it is not already in context. An imported document counts as loaded, so none is read twice.
- A migration rewrites an existing adopter's `CLAUDE.md`, which the bootstrap never overwrites because the adopter owns it. Each `@import <path>` line outside code becomes `@<path>`, and the line importing `.ductus/constitution.md` is removed.
- This spec's §CLAUDE.md template and AC3 state the form the template now writes, and every framework document that describes the old form is corrected with them.

## Edge Cases

- **A line already fixed by hand.** An adopter whose `CLAUDE.md` already reads `@AGENTS.md` is left as it is, because the migration rewrites only `@import` lines.
- **`@import` inside code.** An `@import` inside a code span or fence is text, not an import, and stays.
- **No `@import` line.** A `CLAUDE.md` with none is not touched.
- **Other agents.** Agents that read `AGENTS.md` natively, without `CLAUDE.md`, are unaffected.
- **A host that does not resolve `@path`.** Auggie reads `CLAUDE.md` natively, and whether it resolves `@AGENTS.md` and `@path` imports is verified before the task closes. A host that does not resolve them falls back to `/{project}:target`'s read for the constitutions, which is the reason that read stays.
- **The orphaned-reference check.** `check-orphaned-references` reports a dangling import path. It must still recognise a reference written as `@path`, and that is confirmed against the primitive before the task closes.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Do the constitutions load at startup or when a command runs?** At startup, each held to one read. Resolved 2026-10-26 by the owner. A session the owner starts from a handoff prompt often runs no pipeline command, so loading only on demand would leave it working with no constitution in context. Startup imports cost the sum of the documents, so each is bounded instead: shared constitutions are imported here, and the shipped constitution once `050-constitution`'s cut brings it to one read. Until then, `AGENTS.md` asks that a handoff prompt start with `/ductus:target`.
