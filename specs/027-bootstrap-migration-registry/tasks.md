---
next-task: 18
---

# 027 — Bootstrap Migration Registry Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 14. Implement scenario: a-projects-own-constitution-is-not-relocated

- [x] Implement the behavior described in `scenarios/a-projects-own-constitution-is-not-relocated.md`

Implemented in `364c330b` (landed on `main` 2026-11-12), and verified by the scratch-adopter bootstrap `fccde48b` records.

- **Done when**: The scenario's open question is resolved, a repository with specs/ and its own constitution.md keeps it through a bootstrap run (verified with a scratch-adopter bootstrap), and a legacy ductus copy still relocates.

## 16. A first adoption runs no migration, and the closing summary names each spec it cannot read

- [x] Implement the behavior described in `scenarios/a-first-adoption-runs-no-migration.md`
- [x] `framework/bootstrap/ductus-procedure.md` §Pre-run Migrations: a run Pre-flight Checks decided is a first adoption applies nothing, asks nothing, and seeds `last_applied` with the newest active entry, as a greenfield run does
- [x] §Post-Scaffolding Output names each feature `spec.md` under the spec root with no frontmatter, one line apiece, as one `/{project}:analyze` reads as a hard fail until it gets frontmatter
- [x] 027's spec.md §Adopter State states the first-adoption case beside the greenfield one
- [x] A scratch-adopter first adoption into a project with specs of its own, lacking frontmatter, runs no migration and names each such spec

- **Done when**: the scenario's behavior is in the procedure, §Adopter State says it, and a scratch-adopter first adoption with frontmatter-less specs of its own ends with no migration run and each such spec named.

## 17. A migration fills the substitution keys left literal in AGENTS.md and CLAUDE.md

- [x] Implement the behavior described in `scenarios/seed-files-keep-no-literal-placeholder.md`
- [x] A `framework/migrations/` entry, `introduced_in` 0.60.0 with an id sorting after `strip-inbox-guidance`, fills each remaining `{project-name}`, `{One-line project description.}` and `{project}` in `AGENTS.md` and `CLAUDE.md` from the config, leaves every other brace token, a pinned file and a key with no value as the scenario says, and reports each file it changed
- [x] `framework/migrations.toml` lists it, and `scripts/audit/run-all.sh`'s migration-coverage family passes
- [x] A scratch-adopter upgrade from `ductus-v0.59.0` ends with both files holding no literal substitution key

- **Done when**: the migration is in the registry, Family 10 passes, and a scratch-adopter upgrade from ductus-v0.59.0 ends with no literal substitution key in AGENTS.md or CLAUDE.md.
