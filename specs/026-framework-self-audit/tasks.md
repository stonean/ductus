---
next-task: 8
---

# 026 — Framework self-audit (`/audit`) Tasks

Tasks derived from the [plan](plan.md). Complete in order.

<!-- Each task should be small enough to implement and verify independently.
     Mark subtasks as they are completed. Every task MUST close with a
     `- **Done when**: …` line stating its completion condition — the
     tooling reads this exact form to confirm the task is fully specified.

     A task body may also carry working notes — what the next session needs
     to resume it: the mechanics, the order, what is left on disk — placed
     as §tasks-phase says. Write them as prose on the pending task they
     concern, so they go when the task is pruned. Put an ordering constraint
     on the task that must wait, never above the first task or under a
     heading of its own, which outlive every task; and never write a note as
     a checkbox, which would count toward the task's completion. Anything
     that must outlast the task belongs in its durable home first. Example:

## 1. Create sessions table migration

- [ ] Write SQL migration for `sessions` table
- [ ] Run migration and verify schema

- **Done when**: the migration applies cleanly and `sessions` matches the data model.

## 2. Implement session store

Blocked on task 1: the store tests run against the migrated schema.

- [ ] Create `shared/auth/session.go` with Create, Get, Delete methods
- [ ] Write store integration tests against real PostgreSQL

- **Done when**: all store methods are covered by passing integration tests.

## 3. Update README link to migration guide

- [ ] Edit `README.md` to point at the new path

- **Done when**: the README link resolves to the new path.

-->

## 2. Record the review and analysis of the 071 reopen's fixes

Handoff (2026-11-15). Tasks 5, 6 and 7 have landed; 5 and 6 were reviewed at `713c1ca7` and `368249a9`. Owed, in order:

1. 026's third review, over the window since `368249a9`: task 7's ten family scripts and `scripts/audit/manifest-destination-links.sh`, whose task-5 containment call now tells a containment that could not run from a path resolving outside; `scripts/audit/lib.sh`'s three-way `contained`, `containment_unchecked` and `specs_root`; `scripts/tests/test-audit-path-containment.sh` (cases I to S) and `scripts/tests/test-deprecation-removal.sh`; `scripts/audit/README.md`; the plan's new decision *Every family contains the paths it did not write as a literal*; and the eight scenarios it touched. Fix two pre-existing defects there as chores, since task 7's implementer found them outside its task: `scripts/audit/migration-coverage.sh` Family 10b expands an empty `paths` array under `set -u`, an unbound-variable error on bash 3.2 for an entry with no `target_paths`; and in `scripts/audit/sibling-coupling.sh` the `emit` inside `extract_affected_files` runs within `$(...)`, so its finding is captured as data and never printed.
2. The 26 ticked criteria verified against the tree.
3. The analysis, all 310 rules judged, since `plan.md` and the scenarios changed. BE-PAYLOAD-001 is now met by the design: `check-step-references` by task 6, `resolve-anchor` and `read-deprecation-labels` by 067 task 35 (`7f40a52e`), so the owner's 2026-11-15 deferral of that part to 067 task 35 is discharged and needs no stored decision. BE-INPUT-004 is met by task 7's plan decision. BE-DEPS-007 keeps its stored waiver.
4. The completion gate, then the owner's `done` decision.

- **Done when**: 026's review and analysis are recorded and not stale, and every criterion is re-verified against the tree

## 5. Families 35 and 36 canonicalize and contain the paths they read (BE-INPUT-004, from 026's analysis)

- [x] `scripts/audit/manifest-destination-links.sh` canonicalizes each Shared Files source (35a) and each slash-command source (35b) and compares it with the repository root by whole path components (`os.path.realpath` then `os.path.commonpath`, the `contained` helper in `scripts/audit/lib.sh` for 35a) before probing, copying or reading it, and reports one resolving outside the repository rather than copying or reading it. 35a's destinations keep the lexical screen `apply-manifest` shares, since the family builds the tree they land in itself, with `mkdir` and `cp` of regular files, so no link under it can resolve outside it
- [x] `scripts/audit/self-url-resolution.sh` does the same for each URL's path before probing it and for each tracked markdown file before reading it, and reports an outward one as its own finding, never probed or read. The owner decided on 2026-11-15 that 026 fixes Families 35 and 36 and defers Family 26's part, `check-corpus-links`' probes, to 067 task 33
- [x] `scenarios/family-35-manifest-destination-links.md` and `scenarios/family-36-self-url-resolution.md` commit to the containment in their Behavior, and `scenarios/family-26-broken-relative-links.md` says the link resolution it delegates is contained by `check-corpus-links`, 067 task 33
- [x] `scripts/tests/test-audit-path-containment.sh`, run by a `framework-checks.yml` step, builds fixture trees and shows each family reports a symbolic link resolving outside the project and passes on an in-tree file, and it fails with the containment removed

- **Done when**: Neither family probes, copies or reads a path resolving outside the project, proven by a test that fails with the check removed; `framework-checks.yml`'s and `generators.yml`'s steps, `actionlint` and `scripts/audit/run-all.sh` pass; and 026's next review reads the change.

## 6. check-step-references' result is bounded (BE-PAYLOAD-001, from 026's analysis)

- [x] `check-step-references`' result lists (`findings`, `examined`, `with-steps`, `not-a-procedure`, `skipped`) go through `bounded_lists::place`: inline while the result fits `[artifacts] read-size-bytes`, and past it in the file `lists-file` names, each list's count inline. Today the result has no file form, and each list grows with the command files it reads. The owner decided on 2026-11-15 that 026 bounds it, since the primitive's contract is `scenarios/family-34-step-reference-integrity.md`; the finding's other part, `resolve-anchor` and `read-deprecation-labels`, is deferred to 067 task 35
- [x] `CHECK_STEP_REFERENCES`' description and the family-34 scenario state the bound, and `scripts/audit/step-reference-integrity.sh` reads the lists from `lists-file` in the file form, reporting a list file it cannot read, or one missing a list, as a finding, never as nothing found
- [x] A test proves the inline form under the read size and the file form past it, failing under its mutant, and a fixture run of Family 34 with a read size small enough to force the file form reports the same findings as the inline form

- **Done when**: The result returns inline, or in a lists-file past the read size, with Family 34 reading the file form, proven by tests their mutants turn red; the whole `cargo test` passes; `framework-checks.yml` step (u) passes; and 026's next review reads the change.

## 7. Every audit family contains the paths it did not write as a literal (BE-INPUT-004, from 026's analysis)

- [x] `plan.md` commits once, for every family: a path the family did not write as a literal — read from committed text (a manifest row, a registry field, a table cell, a link target, configured `[paths] specs-root`), listed by `git ls-files` or a walk, or enumerated by a primitive — is canonicalized and contained by whole path components to the repository root before it is probed, globbed or opened, through `contained` in `scripts/audit/lib.sh` or the inline `os.path.commonpath` form, and one resolving outside is reported as such, never touched. A literal path, a primitive's own `lists-file`, and a tree the family builds itself with `mkdir` and regular-file `cp` are outside the rule, each with its reason. The owner decided on 2026-11-15 to close the class rather than fix the two families the third judge run named
- [x] The families the 2026-11-15 inventory found uncontained come under it: 7 (`sibling-coupling.sh`) and 8 (`introducing-drift.sh`), feature directories the dashboard enumerated; 10 (`migration-coverage.sh`), `procedure_file` and `target_paths` from `framework/migrations.toml`, globbed and probed; 12 (`fixture-session-shape.sh`), fixture files `find` lists; 17 (`host-namespace-parity.sh`), the Agent Registry's `config_dir` cells; 24 (`rename-sweep-residue.sh`), 27 (`done-spec-criteria.sh`), 39 (`shipped-anchor-resolution.sh`) and 40 (`deprecation-removal.sh`), files `git ls-files` lists; 37 (`analyze-record-backlog.sh`), records under the configured spec root
- [x] Each family's scenario, where it has one, points at the plan's commitment rather than restating it, and `scripts/audit/README.md` names the rule once
- [x] `scripts/tests/test-audit-path-containment.sh` gains a case per changed family showing a symbolic link resolving outside the repository is reported and not touched, each failing with its family's check removed, proven by hand and recorded

- **Done when**: No audit family probes, globs or opens a path it did not write as a literal without containing it first, proven by a test case per family that fails with the check removed; `framework-checks.yml`'s and `generators.yml`'s steps and `scripts/audit/run-all.sh` pass; and 026's next review reads the change.
