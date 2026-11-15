---
next-task: 6
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

Blocked on task 5: it changes two family scripts and three family scenarios, design subjects every stored verdict was judged against, so the review, the criteria's verification and the analysis run after it lands (owner, 2026-11-15).

Not blocked on 067 task 14 or 071 task 22: neither changes a file in 026's review scope or its plan's Affected Files, and under 079's AC6 a commit only 067 or 071 claims stays out of that scope, so 026 closes next (owner, 2026-11-10).

It owes its review recorded, then an analysis and the completion gate, which run once no box in this file is open (spec 079), its 26 ticked criteria verified against the tree before the runs (spec 079 task 26, owner, 2026-11-09).

Handoff (2026-10-29). 026's record (2026-10-16) is review-stale. For the reuse rule: two slices read 026 at `72288af1`; fix rounds `44c0babe` and `d9ac40a3` were re-read and `c16eee54` (prose) was not; the records written at `c1cf51ff` in 002, 017, 050, 051, 052, 062, 064 and 072 cover `af99793c..c1cf51ff`; no record read `70bb4af5`. Carry byte-identically, fired through process-decisions: 10 of the 13 decisions in 071's `review.md` (omit the two on `scripts/gen-claude-commands.sh` and the one on `scripts/audit/marker-list-parity.sh`, outside this scope), the `c1cf51ff` records' in-scope ones, and 026's own `family-19-review-freshness` waiver, whose issue the 2026-11-04 rewrite of that scenario's Behavior removed. The Summary states the fixed findings: QUAL-STUB-001 MUST (Family 6 writes a stderr skip line), QUAL-GROUND-001 twice (Families 1b/1c guard the constitution; Family 5 guards analyze's blocking headings), and that Families 28's and 34's findings are recorded in 067's review, whose scope holds those scripts. Observations (owner decisions 2026-10-26):

- Fixed: Family 16 judged no pointer into an uninstalled reference (`installer-command-parity.sh`), and its "two counts" fix text; AC21's §Scripts agreement was unscoped and one-way; the family-34 scenario's 80 where the family reports 90; placeholder-roundtrip's "fires today"; lib.sh and README §Contract on the python finding shape, the one-file edit and "four families"; 1c said it extracts edges and checks wording (`cross-doc-consistency.sh`); "the audit runs before the build" and the workflow-level `contents: write` (`runtime-release.yml`); the PR paths filter and the "eight checks" count (`framework-checks.yml`); no pinned toolchain in the release audit job, and check-zero hid cargo's output; audit.md's removed `docs/introduction.md` and its Boundary table's missing row; AC10's tense claim; the scaffold never checked the spec was tracked (`adopter-shell-behavior.sh`); round 2 and 3's sentences (Q4, AC7, AC8, AC10, Family 28's three-set wording, Family 6's header).
- Waived, with the same text and reason as 067 task 14's where the path is shared: pair_document restating the pair grammar; a maintainer-only reference allowed from any file; exact lowercase names only; step 7 with `--agents=` omitting the host; 1c reducing bullets to distinct edges.
- Planned as a deferral to 071 task 22 and never stored in a record: a cleanup in `framework/bootstrap/ductus.md`, which it holds outside 026's review scope and 071 task 22 owns.

- **Done when**: 026's review and analysis are recorded and not stale, and every criterion is re-verified against the tree

## 5. Families 35 and 36 canonicalize and contain the paths they read (BE-INPUT-004, from 026's analysis)

- [x] `scripts/audit/manifest-destination-links.sh` canonicalizes each Shared Files source (35a) and each slash-command source (35b) and compares it with the repository root by whole path components (`os.path.realpath` then `os.path.commonpath`, the `contained` helper in `scripts/audit/lib.sh` for 35a) before probing, copying or reading it, and reports one resolving outside the repository rather than copying or reading it. 35a's destinations keep the lexical screen `apply-manifest` shares, since the family builds the tree they land in itself, with `mkdir` and `cp` of regular files, so no link under it can resolve outside it
- [x] `scripts/audit/self-url-resolution.sh` does the same for each URL's path before probing it and for each tracked markdown file before reading it, and reports an outward one as its own finding, never probed or read. The owner decided on 2026-11-15 that 026 fixes Families 35 and 36 and defers Family 26's part, `check-corpus-links`' probes, to 067 task 33
- [x] `scenarios/family-35-manifest-destination-links.md` and `scenarios/family-36-self-url-resolution.md` commit to the containment in their Behavior, and `scenarios/family-26-broken-relative-links.md` says the link resolution it delegates is contained by `check-corpus-links`, 067 task 33
- [x] `scripts/tests/test-audit-path-containment.sh`, run by a `framework-checks.yml` step, builds fixture trees and shows each family reports a symbolic link resolving outside the project and passes on an in-tree file, and it fails with the containment removed

- **Done when**: Neither family probes, copies or reads a path resolving outside the project, proven by a test that fails with the check removed; `framework-checks.yml`'s and `generators.yml`'s steps, `actionlint` and `scripts/audit/run-all.sh` pass; and 026's next review reads the change.
