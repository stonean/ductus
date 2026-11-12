---
section: "Follow-on scenarios"
---

# Scaffolding-primitives

## Context

The 2026-07-11 coverage review found four clusters of deterministic scaffold work hand-rolled in prose across commands, each with either a shipped-but-unused primitive or no primitive at all:

- **Session writes**: specify.md's session step claims "the runtime exposes no session-shaped primitive for this step" — stale since `write-session` shipped in 0.10.0 — and hand-rolls the cli-config-dir-preserving write; target.md's `--clear` hand-rolls a clearing variant that `write-session` cannot express (it has no clear mode).
- **Feature resolution**: target.md's specs-dir scan with NNN/partial-name matching and its scenario frontmatter read have no primitive ("no runtime primitive iterates the specs directory" per its own prose), so every session's first command starts with LLM-walked directory iteration.
- **Feature creation**: specify.md's step 1 has the host pre-compute the next NNN number, derive the slug, create the directory, and copy the template — all deterministic, no primitive.
- **Inbox appends**: log.md, implement.md's auto-capture rule, and the bootstrap security audit each hand-roll the same atomic append-to-`specs/inbox.md` (the bootstrap variant with its own dedup-by-prefix). Additionally, plan.md and specify.md carry plain ask-for-approval steps while `gate-confirm` exists and prune.md proves the pattern; and `gen-spec-deps.sh` is named raw in several command files while only analyze.md routes it through `run-generator`.

## Behavior

- `write-session` gains a `clear` mode that removes the target while preserving `cli-config-dir`; target.md `--clear` and specify.md's session step invoke `write-session` and the stale no-primitive claim is deleted.
- A new `resolve-feature` primitive resolves a feature identifier for target.md (behavior: `ductus resolve-feature --help`); ambiguity and no-match remain host-mediated prompts.
- A new `create-feature` primitive scaffolds the feature directory (behavior: `ductus create-feature --help`); specify.md invokes it and the LLM fills the body via `writeSpecBody` as today.
- A new `append-inbox` primitive appends an inbox bullet (behavior: `ductus append-inbox --help`); `/{project}:log` is its only caller. Since 058, `/{project}:implement` records an out-of-spec finding as a disposition task, and the bootstrap security audit writes nothing.
- plan.md and specify.md approval gates invoke `gate-confirm`. `gen-spec-deps.sh` is retired into the `derive-dependencies` primitive, which the command files invoke directly (067's `adopter-generator-promotion`); `run-generator` remains for generators that stay scripts.
- All new primitives are wired at every site (schema Args/Result, primitive module, mod.rs, MCP server, PRIMITIVE_NAMES, interpreter dispatch, CLI enum, `runtime-tools.txt`, data-model entries, regenerated configure permission blocks) per the AGENTS.md six-site rule.

## Edge Cases

- Ambiguity and no-match are `resolve-feature` domain outcomes (`ductus resolve-feature --help`); choosing stays with the user through the host.
- The already-exists refusal is in `ductus create-feature --help`; there is no overwrite path.
- `append-inbox` carries no dedup guard: its dedup-by-prefix served implement's auto-capture and the bootstrap audit, and 058 removed it with them.
- `cli-config-dir` preservation is `write-session`'s (`ductus write-session --help`).

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
