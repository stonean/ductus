---
section: "Adopter State"
---

# A-first-adoption-runs-no-migration

## Context

Pre-run Migrations skips only a **greenfield** run, one with no config file at any tier and no spec root (`framework/bootstrap/ductus-procedure.md` §Pre-run Migrations, step 4). A first adoption into a project that keeps its own spec root is not greenfield, so it runs every active entry, though no earlier `ductus` run left anything to convert: Pre-flight Checks has already decided it is a first adoption from signals only an earlier run leaves (007's scenario `a-first-adoption-with-existing-specs-is-not-an-update`).

Found by 007 task 7's scratch-adopter bootstrap on 2026-11-16: a first adoption into a project with two specs of its own, `specs/001-greeting/spec.md` and `specs/002-farewell/spec.md`, neither with frontmatter. All 14 pending entries ran. `criterion-label-backfill`'s `label-criteria` and `audit-record-relocate`'s `relocate-audit-records` each exited 1 on both specs (`frontmatter missing … (no leading --- block)`), and the procedure gives no case for a migration's primitive error, so the run went on. The Frontmatter Migration then skipped both (`no metadata to migrate`), and the run ended reporting success with two specs that `/{project}:analyze` reads as a hard fail, which nothing told the operator.

## Behavior

- **A first adoption runs no migration.** When Pre-flight Checks found no earlier-adoption signal, Pre-run Migrations applies nothing, asks nothing, and seeds `[migrations] last_applied` with the newest active entry's id, as a greenfield run does.
- **An update is unchanged.** It runs the pending entries as before. A legacy adopter with a spec root and no config file is an update, since its `{spec-root}/templates/spec.md` is a signal, so it still migrates.
- **The closing summary names each spec it cannot read.** Each `spec.md` directly under a feature directory of the spec root that has no frontmatter (no leading `---` block) is named, one line apiece, as one `/{project}:analyze` reads as a hard fail until it is given frontmatter.

Decided by the owner on 2026-11-16, over the same change plus default frontmatter from the Frontmatter Migration, and over a waiver.

## Edge Cases

- **A first adoption with no spec root** is greenfield already; nothing changes for it.
- **A project's own root `constitution.md` on a first adoption:** `constitution-relocate` no longer runs, so the file is untouched without relying on that migration's content gate, which still guards an update (scenario `a-projects-own-constitution-is-not-relocated`).
- **A spec with frontmatter that does not parse** is not one without frontmatter: it is not named here, and `/{project}:analyze` reports it.
- **An update whose spec has no frontmatter** still runs its migrations, and the closing summary names the spec the same way.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
