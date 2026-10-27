---
section: "Classification"
---

# Memory-routing-lives-in-agents-md

## Context

AC20 named four agent-facing surfaces that carry the routing question as a pointer, and the `CLAUDE.md` project template's §Auto-Memory Routing was one of them. That section restated both questions and every tier in full. Since this spec's first cut (`05359e82`) the constitution has said a host's rules file carries the routing as a pointer, and the constitution now loads beside it at session start. The section also reached too few agents: of the bootstrap's five registered agents, only the two `claude-style` ones (Claude Code, Auggie) read `CLAUDE.md`. Antigravity, OpenCode and Pi read `AGENTS.md`, whose template header asks who a learning is true for but not whether it belongs in per-user memory at all.

An inbox item logged 2026-10-26 asked for one home, `AGENTS.md`. Both project templates are installed with the `skip` strategy, so a template edit reaches new adoptions only. On 2026-10-27 the owner chose a framework-managed block in `AGENTS.md`, written on every `/{project}` run as the `ductus:constitutions` block is, so every existing adopter receives it. The owner also chose a migration that removes the copied section from an existing `CLAUDE.md` only when it is a copy the framework wrote, unchanged.

## Behavior

- **`AGENTS.md` is the one home for where a learning goes.** A framework-managed block between `<!-- BEGIN ductus:memory-routing -->` and `<!-- END ductus:memory-routing -->` states both questions, in order, and links the constitution's §drift-prevention. `/{project}` writes it with `merge-managed-block` on every run, whatever agents are selected. A file without the markers gets the block appended at its end, and the run says so.
- **The `AGENTS.md` template ships the empty markers** beneath its header, and the same run fills them. The header keeps what is the file's own: it holds this project's rules, by section.
- **The `CLAUDE.md` template holds its imports and the `ductus:constitutions` block, and nothing else.** A `claude-style` session gets the routing through `@AGENTS.md` and `@.ductus/constitution.md`.
- **The constitution names the block** as the pointer that carries the routing.
- **The migration `memory-routing-leaves-claude-md` removes `CLAUDE.md`'s `## Auto-Memory Routing` section** when the section is byte-identical, after `{project}` is substituted, to one of the versions the template shipped. It leaves an edited section in place and reports one line naming it for the operator to remove. A pinned `CLAUDE.md` gets a warning and no write.
- **This repository's `CLAUDE.md` drops its §Auto-Memory Routing.** `AGENTS.md`'s workflow entry on per-user agent memory already governs, and `CLAUDE.md` imports it.

## Edge Cases

- **A section followed by sections of the adopter's own.** Only the section goes, from its heading to the next second-level heading.
- **An edited section the migration leaves.** The routing then reaches a `claude-style` session twice, once through `@AGENTS.md`, until the operator removes the copy; the warning says so.
- **A pinned `AGENTS.md`.** It still gets the block, as it gets the `ductus:constitutions` block: pinning turns a manifest `update` into `skip`, and the block is a managed-region merge, not a manifest write.
- **A file with CRLF line endings.** The section is compared, and removed, with the file's own line endings.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
