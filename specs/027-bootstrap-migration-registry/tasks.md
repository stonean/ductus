---
next-task: 21
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

## 18. Exercise the shared bootstrap window's review fixes in a scratch-adopter bootstrap

- [x] An upgrade whose project keeps its own root `constitution.md` ends with Post-Scaffolding Output carrying `constitution-relocate`'s declined line and each other line its migrations reported (c061c1f0)
- [x] A first adoption that stopped before step 4 runs no migration on its re-run (8be4f935)

- **Done when**: the batched run 003's matching task names shows each outcome, which the commit ticking this task states; a defect it finds is fixed first

## 19. Finish AC25 and AC27, and correct what 027's pre-done verification found (owner decision, 2026-11-18)

- [x] `scripts/audit/migration-coverage.sh` gains a Family 10 duplicate-id check: each `id` two `[[migrations]]` entries share is a finding. `scripts/tests/test-migration-coverage.sh` gains a case shown to fail without it, also under /bin/bash 3.2. The script's header comment and 027's §Audit Family list it (AC25: no check made a duplicated id fail; a fixture with two same-id entries and one procedure file passed)
- [x] `framework/bootstrap/ductus-procedure.md` §Duplicate-id and reference-integrity guard names its two abort messages: `migration {id}: procedure file missing from archive`, and one for a duplicated id (AC27 named a message the guard never stated)
- [x] 10e's finding for a malformed or non-release `version` carries a remedy for the version file, not the archive-it remedy an overdue entry gets
- [x] spec.md: AC7 annotated superseded, since the loop also reads the pre-run `git status --porcelain -- specs/`, the active config and the guard's procedure-file existence. AC21 narrowed: an absent target emits nothing, while a present, converged target reports one `left as is` line in inbox-guidance-refresh, memory-routing-leaves-claude-md, remove-legacy-constitutions-region, strip-inbox-guidance and constitution-import. AC27's `aborts the batch at that entry` annotated superseded by 367c8817's guard-first placement
- [x] AC30 and `scenarios/migrations-apply-once-per-repo.md` Behavior bullet 5 cite `gitignore-marker-rename` in the past tense, since c061c1f0 archived it

- **Done when**: each item holds against the tree; Family 10 and its tests pass on the tree and the new case fails without the check; the landing follows docs/developing-ductus.md §Landing a runtime change steps 3 to 5; and 027's review of the window since fdf80a2f, its analysis and its gate follow

## 20. Exercise task 19's guard text in a scratch-adopter bootstrap

- [ ] AGENTS.md asks a scratch-adopter bootstrap of every `framework/bootstrap/` change, and 6bb984b0 changed `framework/bootstrap/ductus-procedure.md` §Duplicate-id and reference-integrity guard. The run reads that half from the fetched archive, so it runs after the change is pushed, with `--ref=main`, batched with 029 task 17's run
- [ ] The run's Pre-run Migrations applies the guard at step 1 and passes the real registry, and the run completes as before

- **Done when**: the batched run 029 task 17's matching item names shows the guard passing and the run completing, which the commit ticking this task states; a defect it finds is fixed first
