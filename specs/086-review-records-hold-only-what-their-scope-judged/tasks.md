# 086 — Review records hold only what their scope judged Tasks

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
     that must outlast the task belongs in its durable home first.

     Schedule no review, analysis or completion gate as a task: they run
     once every box here is ticked, and refuse to record before then, so
     what a spec owes them is prose on its pending task. -->

## 1. The files a spec owns

Blocked on 084 reaching `done`: 084's Resolved Questions, *Landing order*, has every other open spec's implementation follow it (owner, 2026-11-24), and its tasks 8 to 10 rewrite `write_review.rs`, `compute_review_scope.rs` and `process_decisions.rs`' arguments. Start once `check-review-gate` on 084 has passed and its status reads `done`, and re-read the symbols the plan cites at the signatures 084 left.

- [ ] Create `runtime/src/primitives/spec_ownership.rs`: for a set of features, one walk of every commit reachable from `HEAD`, each attributed once by `Attributor::claims` and divided by `SpecShare::split` for each asked spec that claims it, plus each spec's plan entries by `PlanEntries::names`; for one spec, each commit tested first by `Attributor::belongs_to` and only an admitted one diffed; `examined` or `not-examined` with a reason, as the plan's *One definition of "belongs"* states
- [ ] Unit tests: a file a plan entry names belongs; a file a commit attributed to the spec changed belongs; a file only an unclaimed commit, or only another spec's share, changed does not; a merge touching the spec's directory gives it nothing; a shallow repository (`shallow_at`'s fixture shape) and an unrecognized directory name are not examined; one walk over two specs gives each the answer a walk over it alone gives

- **Done when**: the tests pass and each is proven to fail with its behavior removed (`QUAL-TEST-001`), and the commit records the single-spec walk's time on 022 and on a spec of fewer than 50 commits, and the two-spec walk's on all of this repository's specs, release build.

## 2. `write-review` refuses an outcome outside the scope

- [ ] `RunScope` gains a constructor from a resolved scope, which `RunScope::resolve` calls; `resolve_scope` builds one from the scope it resolved
- [ ] Refuse, before any write, each `findings[].file` and each `observations[].path` without its `:line` suffix that the scope does not hold, an empty `path` exempt, naming the argument and the file; with an empty `diff-base` the reason says to pass the diff base `compute-review-scope` returned
- [ ] Tests: a finding and an observation on a file outside the scope are refused and nothing is written; an observation at `file:line` inside the scope, one with no path and a finding on a file a first review's plan pattern names are accepted; a re-read's finding on a file a fix round touched is accepted at the run's last write; the project record's write refuses the same way
- [ ] `WRITE_REVIEW` in `runtime/src/schema/descriptions.rs`, and `specs/066-audit-records-and-the-completion-gate/data-model.md`'s `write-review` section
- [ ] `framework/commands/review.md` step 10 and §Passes in a subagent in `framework/reference/review.md`: an outcome on a file outside the scope is marked by the passes and taken out before step 11, a gating one fixed when mechanical or deferred to the home §Groom decision tree in `.ductus/reference/groom.md` chooses, one at a time, any other named in the report and recorded nowhere

- **Done when**: AC1 holds, its tests are proven to fail with the refusal removed, `cargo test` passes, the two command files state the spec's Edge Case on an outcome outside the scope, and the commit records each registry layout's impact (`claude-style`, `pi`).

## 3. Review drops what does not belong

- [ ] `process-decisions`: in today's `NotRead` case, on a run neither restricted nor given `since` and on the record of a spec, ask `spec_ownership`; a file that does not belong joins `dropped` with its notice, one not examined is retained under a new class naming the reason; add `dropped`, `dropped-count`, `ownership` and `ownership-reason`, bounded with the other lists
- [ ] `process-waivers`: the same in `Pass::place`'s `why_unread` branch, for a run with no `skipped-passes` and no `since`
- [ ] `framework/commands/review.md` steps 8 and 9 pass each `dropped` entry with the expired ones and show its notice; `framework/reference/review.md` §Per-run waiver processing states the rule
- [ ] `PROCESS_DECISIONS` and `PROCESS_WAIVERS`; 066's data model; `runtime/tests/result_sizes.rs`' two cases
- [ ] Fixtures: a stored decision and a waiver on a file that does not belong are dropped and named, and absent from the next `write-review`'s record; one on a belonging file unchanged since the last review is retained; a decision on a deleted file is expired, not dropped (AC4); a restricted run and a `since` run drop nothing; a shallow history reports `not-examined` and drops nothing; a decision on code no spec claims, stored before 084's scope change, is dropped and `{specs-root}/review.md` does not gain it (AC3)

- **Done when**: AC2, AC4 and the review half of AC3 hold, each fixture proven to fail with its behavior removed, and the commit records each registry layout's impact (`claude-style`, `pi`).

## 4. A first review reads what the spec's commits changed

- [ ] In `compute`'s branch for a record naming no run, add each owned path on disk, not leaving the project and not deleted, and remove it from `excluded`; add `ownership` and `ownership-reason` to the result on a first review
- [ ] `COMPUTE_REVIEW_SCOPE`; `framework/reference/review.md` §Window and coverage; 066's data model
- [ ] Fixtures: a file a commit attributed to the spec changed, absent from the plan, joins a first review's scope; a cut history gives the plan entries alone with `ownership: not-examined`; a later review's result carries no `ownership`

- **Done when**: AC6 holds, each fixture proven to fail with its behavior removed, and `runtime/tests/golden/` changes only where a re-bless the commit reads moves it.

## 5. `prune-review`

- [ ] Create `runtime/src/primitives/prune_review.rs`, with `feature` or `all`, preview and `apply` with the previewed digest, as the plan's `prune-review` decision states: `drops`, `rewrites`, skips and refusals, the text splice and its re-parse check, `ReadSnapshot::write_if_unchanged`
- [ ] Register it at the seven sites **§Adding or removing a runtime primitive** in `docs/developing-ductus.md` names, with its argument type, `PRUNE_REVIEW`, `framework/runtime-tools.txt`'s line, and `scripts/gen-configure-mcp.sh` re-run
- [ ] Tests: a record's entries on files that do not belong are dropped and claimant lines rewritten, every other byte unchanged; a record with nothing to drop or rewrite is not written; a fix-round line is not rewritten; `fix-rounds-open: true` is skipped; an unparseable record or `decisions:` list is refused naming the file, and skipped in the `all` walk; a record changed since the read is refused; `all` makes one ownership walk; `{specs-root}/review.md` is not walked
- [ ] `specs/041-task-pruning/data-model.md`: a `prune-review` section, and its write guard naming four primitives

- **Done when**: AC5 and the prune half of AC3 hold, each test proven to fail with its behavior removed, and the commit records each registry layout's impact (`claude-style`, `pi`).

## 6. `/prune` runs it

- [ ] `framework/commands/prune.md`: `prune-review` in the preview, render, its own confirm and apply step after the plan sections, "nothing to prune" needing no drop or rewrite, the `review.md` row, Scope Boundaries, description and `ToolSearch` line; `--project` does not run it
- [ ] `scripts/gen-help-tables.sh`'s prune entry if its description changes, then `scripts/gen-help-tables.sh` and `scripts/gen-claude-commands.sh` re-run, and any golden the edit moves re-blessed alone and read
- [ ] Read `docs/slash-commands.md`'s prune entry whole and correct any sentence saying prune never writes `review.md`

- **Done when**: a `/ductus:prune` run over a fixture spec whose record holds an entry on a file that does not belong shows the drop, writes it on confirmation and nothing when declined, and the commit records each registry layout's impact (`claude-style`, `pi`).

## 7. The owners' specs

- [ ] Ask the owner once whether the notes on 079's AC24 and AC6, 058's AC32 and 041's AC9 may each read "Superseded in part (spec 086)" with the spec staying `done` (084's correction rule), and record the answer in the commit
- [ ] 079: AC24 and AC6 notes, and the edge case *A stored finding the run did not read*; 058: AC32 note; 041: AC9 note, its body sentence that prune never writes `review.md`, and the `review.md` row of its scenario `prune-reduces-the-spec-directory`
- [ ] Signposts, each a blockquote linking this spec: 020, 041, 058, 066, 079 and 084
- [ ] `runtime/CHANGELOG.md` `[Unreleased]`: the refusal, `dropped` and `ownership`, the first review's scope, and `prune-review`

- **Done when**: `check-artifacts` reports no undischarged `cross-spec-impact:` entry on 086 and no new finding on the six, each swept spec's status is unchanged, and `scripts/audit/run-all.sh` exits 0 after the commit.
