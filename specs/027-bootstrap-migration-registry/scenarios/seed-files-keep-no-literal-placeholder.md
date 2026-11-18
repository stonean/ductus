---
section: "Adopter State"
---

# Seed-files-keep-no-literal-placeholder

## Context

Through `ductus-v0.59.11`, bootstrap step 4 mapped the `skip` rows for `AGENTS.md` and `CLAUDE.md` to `apply-manifest`'s `skip-if-conflict` strategy, which substitutes nothing. So an adoption on the runtime path wrote both files with the literal placeholders `{project-name}`, `{One-line project description.}` and `{project}`. 071's `b0b1f9d0` (2026-10-26) maps them to `create`, which substitutes, but only for a new adoption: `create` never rewrites an existing file, so an earlier adopter keeps the literals through every upgrade. Found by 012 task 12's scratch-adopter upgrade from `ductus-v0.59.0` on 2026-11-16, whose `AGENTS.md` still held all three after the upgrade.

## Behavior

- **A migration fills the substitution keys left literal in the two seed files.** In `AGENTS.md` and `CLAUDE.md`, each remaining `{project-name}` becomes `[project] name`, each `{One-line project description.}` becomes `[project] description`, and each `{project}` becomes `[host] project`, the three keys step 4 substitutes into those templates. It reports each file it changed and how many tokens it filled; an absent file, or one holding none, is left alone with nothing printed.
- **Only those three keys.** The 0.59.0 templates also carried `{name}`, `{slug}` and `{NNN-feature-name}` as documentation (the current `AGENTS.md` template carries `{name}` only), which the bootstrap never substitutes; the migration leaves every other brace-delimited token alone.
- **It runs for every adopter crossing 0.60.0.** Its `introduced_in` is `0.60.0` and its id sorts after `strip-inbox-guidance`, so a `--ref=main` adopter that already applied the other 0.60.0 entries runs it too.

Decided by the owner on 2026-11-16, over a closing-summary report of the literals and over a waiver.

## Edge Cases

- **A key with no value in the config** (no `[project] description`, say): that token is left as it is, and the report names the file and the token it could not fill.
- **A `{project}` the adopter wrote on purpose** is filled too. The migration runs once per repository and git holds the file's earlier content.
- **A pinned `AGENTS.md` or `CLAUDE.md`** is left alone and named with the tokens it still holds, since pinning opts a file out of `ductus`'s writes, as `constitution-relocate` leaves a pinned command body (its step 5).

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
