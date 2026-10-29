---
section: "Bootstrap Loop"
---

# A-projects-own-constitution-is-not-relocated

## Context

The `constitution-relocate` migration's step 1 treats any root `constitution.md` as a legacy ductus copy, with no content check, and the migration stays active indefinitely. Read from the migration and the bootstrap procedure on 2026-10-26 at HEAD c0952038: it moves the file to `.govern/`, `ductus-rename` moves it on to `.ductus/`, and the Shared Files update pass overwrites it with ductus's constitution; its step 4 also re-points the project's own `CLAUDE.md` import and its README and `AGENTS.md` links at the result. Since b0b1f9d0 (AC6) a greenfield run — no config file at any tier and no spec root — applies no migration, so the path remains for a repository with no ductus config but an existing `specs/` directory beside its own `constitution.md`. Consent is one batch prompt naming the migration, with no per-file prompt; the move uses `git mv` when the file is tracked, so committed content is recoverable from git, but uncommitted edits and an untracked document are lost.

## Behavior

- A root `constitution.md` that is not a ductus constitution is never moved, overwritten or re-linked by a migration, whatever else the repository contains.
- A root `constitution.md` that is a legacy ductus copy is relocated as today.
- When the migration declines a root `constitution.md`, the run says so, naming the file, so the operator knows ductus left it in place.

## Edge Cases

- A repository with `specs/` and its own `constitution.md`, no ductus config: the case above; not relocated.
- A legacy adopter whose root copy carries local edits: still a ductus copy by whichever recognition is chosen.
- A pre-0.24 ductus copy: recognition must be checked against those copies, which may lack the current section markers.
- Repositories that already ran the migration are unaffected by a change to its gate; the change reaches exactly those that have not.

## Open Questions

- How does the migration recognize a ductus constitution: gate on a legacy ductus artifact (a pre-existing `.govern.toml` or `.govern/config.toml`), recognize the file by content (the shipped copy's section markers, checked against pre-0.24 copies), or both — and is this an edit to the existing migration's gate or a new registry entry ordered after it?

## Resolved Questions

*None yet.*
