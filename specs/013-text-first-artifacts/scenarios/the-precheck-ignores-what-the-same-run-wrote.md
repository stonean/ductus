---
section: "Migration via `/ductus`"
---

# The-precheck-ignores-what-the-same-run-wrote

## Context

On 2026-10-27 the real adopter bootstrap `050-constitution` ran for its task 35 adopted a scratch project at `ductus-v0.28.0` and then ran `/ductus --ref=main` at `ad595280`. Pre-run Migrations applied `inbox-guidance-refresh`, which rewrote the guidance comment in `specs/inbox.md`. Moments later in the same run, the Frontmatter Migration's precheck (`framework/bootstrap/ductus-procedure.md` §Frontmatter Migration) ran `git status --porcelain -- specs/`, saw `specs/inbox.md` modified, and refused: "Migration requires a clean working tree under `specs/`. Commit or stash your changes, then re-run." Every adopter whose inbox still carries the pre-0.53 guidance meets that refusal on its first run past 0.53. The precheck's next line, "Exit before any modifications", does not say whether the section or the whole run exits, so one host skips the migration and continues while another halts before Shared Files.

AC14 makes the precheck refuse a dirty tree so that the migration never rewrites work an adopter has not committed. A change the same `/ductus` run made is not that work.

## Behavior

- A path under `specs/` that this run's Pre-run Migrations wrote does not make the tree dirty for the Frontmatter Migration's precheck. An uncommitted change the adopter made there still does, and the precheck refuses exactly as AC14 requires.
- The refusal says what it stops: the Frontmatter Migration alone, with the rest of the run continuing, or the run. The procedure names one, so every host walks it the same way.
- The run's closing report lists what Pre-run Migrations changed under `specs/`, so the adopter commits those changes with the rest of the run's.

## Edge Cases

- **The adopter's change and a migration's change in one file.** The file was dirty before the run started, so the precheck refuses.
- **A migration that writes a file the Frontmatter Migration would also rewrite.** The Frontmatter Migration reads the file as the earlier migration left it, so both changes land.
- **A refusal that stops the run leaves the new imports pointing at the old files.** Pre-run Migrations run before Shared Files (`framework/bootstrap/ductus.md`). On an upgrade across 0.60.0, `claude-md-import-syntax` has already rewritten `CLAUDE.md`'s imports to the `@path` form Claude Code loads by the time the precheck refuses, while `.ductus/constitution.md` is still the pre-upgrade copy, so every session loads that copy at startup until a re-run completes. A refusal that stops only the Frontmatter Migration lets Shared Files replace the copy in the same run.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
