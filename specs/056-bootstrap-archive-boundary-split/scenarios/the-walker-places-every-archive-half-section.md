---
section: "What the run reads, and when"
---

# The-walker-places-every-archive-half-section

## Context

`framework/bootstrap/ductus.md` §Instructions is the run's whole execution order (the spec's It is execution order, not reading order section), and its steps 1–10 never name four sections of the archive half: Pre-run Migrations, Frontmatter Migration, Security Audit (brownfield) and Hook Installation. Their own text in `framework/bootstrap/ductus-procedure.md` places three of them only relative to another section: the Security Audit triggers "after the **Shared Files** manifest pass has completed", Hook Installation runs "After **Per-Agent Scaffolding** completes", and the Frontmatter Migration runs "after the Pre-flight Phase". Pre-run Migrations state no position; reading `framework/migrations.toml` from the fetched archive puts them after step 3 and nothing places them further. The list of sections the stale-`ductus.md` branch skips names them in a sequence and says that sequence is not the order they run in. So each host places them by its own reading. Found by `008-security-rules` task 22's real adopter bootstrap, 2026-10-29.

`013-text-first-artifacts` scenario `the-precheck-ignores-what-the-same-run-wrote` depends on this order: its Edge Cases take Pre-run Migrations to run before Shared Files.

## Behavior

- `framework/bootstrap/ductus.md` §Instructions names, for each of Pre-run Migrations, Frontmatter Migration, Security Audit (brownfield) and Hook Installation, the numbered step it runs at or the two steps it runs between, so every host walks the run in one order.
- Pre-run Migrations run after step 3 extracts the archive and before step 4's first `apply-manifest` pass: they read their registry from the archive, and they convert what an earlier release left behind before this release's files are written over it.
- The Security Audit runs after step 4's Shared Files pass, and Hook Installation after step 4's Per-Agent Scaffolding passes, as their own sections already require.
- The stale-branch list's disclaimer points at those placements.

## Edge Cases

- **A migration that moves the config file.** `govern-dir-consolidate` makes `.ductus/config.toml` the active config file for every later step, so the placement keeps Pre-run Migrations ahead of every walker step that reads or writes the active config file, step 6's host block included.

## Open Questions

- Does the Frontmatter Migration run before step 4's Shared Files pass, as the stale-branch list's sequence reads, or after step 4?
- Where do the Security Audit and Hook Installation fall relative to steps 5–9: directly after step 4, or after step 9's self-install?

## Resolved Questions

*None yet.*
