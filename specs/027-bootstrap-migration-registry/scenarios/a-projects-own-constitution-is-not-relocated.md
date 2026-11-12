---
section: "Bootstrap Loop"
---

# A-projects-own-constitution-is-not-relocated

## Context

The `constitution-relocate` migration's step 1 treats any root `constitution.md` as a legacy ductus copy, with no content check, and the migration stays active indefinitely. Read from the migration and the bootstrap procedure on 2026-10-26 at HEAD c0952038: it moves the file to `.govern/`, `ductus-rename` moves it on to `.ductus/`, and the Shared Files update pass overwrites it with ductus's constitution; its step 4 also re-points the project's own `CLAUDE.md` import and its README and `AGENTS.md` links at the result. Since b0b1f9d0 (AC6) a greenfield run — no config file at any tier and no spec root — applies no migration, so the path remains for a repository with no ductus config but an existing `specs/` directory beside its own `constitution.md`. Consent is one batch prompt naming the migration, with no per-file prompt; the move uses `git mv` when the file is tracked, so committed content is recoverable from git, but uncommitted edits and an untracked document are lost.

## Behavior

- A root `constitution.md` that is not a ductus constitution is never moved, overwritten or re-linked by a migration, whatever else the repository contains.
- `constitution-relocate` recognizes a ductus copy by its content: its first line is `# Constitution` and it carries the `<!-- §principles -->` marker. Only a root file carrying both is relocated, as today.
- When the migration declines a root `constitution.md`, the run says so, naming the file, so the operator knows ductus left it in place.

## Edge Cases

- A repository with `specs/` and its own `constitution.md`, no ductus config: the case above; not relocated.
- A legacy adopter whose root copy carries local edits: still a ductus copy, since an edit keeps the title and the marker.
- A pre-0.24 ductus copy: every shipped copy checked carries both, at `v0.1.0`, `gvrn-v0.20.0`, `gvrn-v0.22.0`, `gvrn-v0.23.0` and `main`; copies older than `8558ac8b` (2026-04-28) predate every tag and are unchecked.
- A project's own constitution that opens `# Constitution` and carries a `<!-- §principles -->` marker reads as a ductus copy and is relocated: the marker is ductus's own anchor grammar, so the content gate cannot tell such a file apart, and git holds its content.
- Repositories that already ran the migration are unaffected by a change to its gate; the change reaches exactly those that have not. A project that already lost its own file can recover it from git, which no registry entry can reach.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **How does the migration recognize a ductus constitution, and is this an edit to its gate or a new entry after it?** By content — the first line `# Constitution` and the `<!-- §principles -->` marker — as an edit to `framework/migrations/constitution-relocate.md`'s step 1, with a "declined, left in place" line naming the file (owner, 2026-11-12). A new entry ordered after it cannot protect the file: Pre-run Migrations runs every entry after `[migrations] last_applied`, which is unset on a first adoption, so the existing entry moves the file before any later one runs; an edit to its gate reaches exactly the adopters who have not run it, which AGENTS.md's add-a-new-entry rule exists for reaching the others. Rejected: gating on a legacy `.govern.toml` or `.govern/config.toml`, which misses a legacy adopter that never wrote a config, leaving its stale root copy; and requiring both gates, which adds that gate's false negatives and gains nothing.
