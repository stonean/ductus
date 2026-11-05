# 079 — Review and analysis work converges Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Commit attribution

The derived write boundary held only `AGENTS.md` and spec directories at `b63e992c`, so `/ductus:implement` may refuse this task's first write under `runtime/`. If it does, seed `write-boundary` in the session, as `framework/reference/plan.md`'s Validation gate describes.

- [x] Create `runtime/src/primitives/commit_attribution.rs`: `claims` per commit, with the first-named spec, resolving through `parse_feature_dir` and `ProjectRepository`
- [x] Fixture histories in `runtime/src/primitives/commit_attribution.rs`'s unit tests: a directory touch, a subject scope, both, a number no spec has, a merge, a root commit, a branch-scoped spec, and a project in a subdirectory (`git_fixture::subdirectory_project`)
- [x] Add the helper's mutations to `scripts/mutation/scope-attribution.py`

- **Done when**: each fixture passes, and each mutation is killed.

## 2. The review window and scope (AC3, AC5, AC6, AC4)

Blocked on task 1.

- [x] `compute`: the last qualifying review commit moves the base, and a skipped pass, budget stop, open mark, unparseable or uncommitted record does not; add `base-source`
- [x] `compute`: AC6's scope, `excluded` in the bounded lists, an unchanged plan entry only on a first review, and `review-owed`
- [x] `write-review`: the denominator follows, frontmatter `excluded`, and the body names each excluded path by its claiming spec
- [x] Tests for each Edge Case: no earlier review, `review-owed` true with no record, a review older than the reopen, one path changed by two commits, `--since`, an unclaimed commit, a multi-spec commit with an unplanned path, its subject naming this spec first and naming none, and uncommitted work
- [x] State the two rules in `framework/commands/review.md`, `framework/reference/review.md` and `framework/reference/implement.md`, and have the command report `review-owed: false`
- [x] Remove the `AGENTS.md` entries AC5 and AC6 retire (the `--since` and window entries), and re-key anything `scripts/audit/run-all.sh` reports

- **Done when**: the tests pass, each fails with its behavior removed, and `compute-review-scope` on 058 returns a scope that AC6's rule explains.

## 3. Fix rounds and the open mark (AC3, AC13, AC14)

Blocked on task 1.

- [x] `round_paths`: drop a path only another spec's commits changed, comparing content at both snapshots with their parents
- [x] `brief-re-read`: exempt paths leave a mixed round's brief and `round.toml`; return `exempt` and `excluded`
- [x] `write-review`: `loop-open`, `fix-rounds[].excluded`; `check-review-gate`: a new block on `fix-rounds-open`
- [x] Step 11 of `framework/commands/review.md` passes `loop-open`, and the reference's Fix-round loop says what changed
- [x] Tests in `runtime/tests/fix_rounds.rs`: another spec's commit between rounds leaves the round, a path both a fix and that commit changed stays, a commit this spec claims stays, and a run cut off between writes blocks while an exec-shaped record does not
- [x] The 078 claims the sweep names, in the same commit, with a signpost back-linking this spec

- **Done when**: the tests pass, each fails with its behavior removed, and a fixture review cut off after step 11 is blocked by `check-review-gate`.

## 4. `diff-cross-spec` (AC7)

Blocked on task 1.

- [x] Start at the transition parent, keep sibling paths only from commits belonging to the spec, keep uncommitted ones, and rename the result field to `window-base`, updating the schema tests and `runtime/tests/golden/*.jsonl`, and reading every bless diff
- [x] Steps 7 and 13 of `framework/commands/implement.md`, `framework/reference/implement.md`, and the doc comment in `runtime/src/primitives/derive_boundary.rs`
- [x] Tests: another spec's commit is absent, a never-`in-progress` spec keeps its first commit, and the no-commit guidance stays

- **Done when**: the tests pass, each fails with its behavior removed, and `diff-cross-spec` on 066 no longer lists another spec's commits.

## 5. Rule-decided dispositions (AC2, AC17)

- [x] `write-review` and `write-analysis`: `by-rule` records `decided-by: rule`; `write-review`: `contract-break` counts in `contract-breaks`
- [x] Steps 10 of `framework/commands/review.md` and 19 of `framework/commands/analyze.md`: only a gating finding's waiver or deferral uses `gate-confirm`
- [x] The rule in `framework/reference/review.md`, `framework/reference/analyze.md` and `framework/reference/groom.md`, and the claims in `README.md`, `docs/analyze.md`, `docs/slash-commands.md` and `framework/commands/help.md`
- [x] The 058 claims the sweep names (16 in `spec.md` and `plan.md`, 2 in its data model and a scenario), with a signpost back-linking this spec, and the 047, 050, 063 and 066 ones
- [x] Tests: a by-rule waiver records `decided-by: rule` and a person's stays, a stored decision wins, a contract break counts, and `ductus exec` still writes nothing

- **Done when**: the tests pass, each fails with its behavior removed, and a fixture review's observations reach `gate-confirm` only when gating.

## 6. What reopens (AC4, AC15)

- [x] Create `classify-spec-edit` and register it in the five sites, including `framework/runtime-tools.txt`, with `scripts/gen-configure-mcp.sh` re-run
- [x] `/{project}:amend`'s reconcile pass calls it, and `runtime/src/primitives/check_artifact_size.rs`' `OnDone` hints follow it
- [x] Tests: spec-body prose is a correction, labelling is a correction, and a changed criterion, an added question, an added unchecked task, a scenario edit and a data-model edit each reopen, named by reason
- [x] Add its mutations to `scripts/mutation/reopen-measure.py`

- **Done when**: the tests pass, each fails with its behavior removed, and `classify-spec-edit` on a fixture `done` spec returns `correction` for a prose edit.

## 7. The measurement and `/status` (AC1, AC8, AC17, AC19)

Blocked on tasks 2 and 5: it reads the fields they write. `review.md` now carries `contract-breaks`, `fix-rounds-open` and `excluded`, and each stored decision a `decided-by`, with `rule` for what the rule decided (`decisions::RULE_DECIDER`). `ReviewBlock` in `runtime/src/schema/primitives.rs` has `fix_rounds_open` but no `contract_breaks`, so this task adds that field.

The baseline in the spec's outcome entry came from throwaway scripts, so its method there is what to reproduce: closed reopenings by their closing commit's author date. Two traps when re-deriving: `git log --follow --reverse` returned no commits for 007, 064 and 065, so walk without `--follow`; and a grep for `- [ ]` counts the tasks template's example checkboxes, which 026, 052 and 058 keep inside an HTML comment, so `waiting` reads through `read-tasks`. Another walk gives another count (commits adding `status: done` under `--since` gave 119 against 123), so state which walk a figure used.

- [x] Create `measure-reopenings` and register it in the five sites
- [x] `dashboard` and `framework/commands/status.md`: the counts on every run
- [x] `runtime/tests/measure_reopenings.rs`: `measured`, `shallow-history` and `nothing-to-compare`, within the bound, over it, a second run after a gating finding, a reopening with no recorded run, a record that does not parse, and a review's two writes counting as one run
- [x] Run it at `d62e72f3` over the window since 2026-10-01 and record in the commit whether it reproduces 123 reopenings and 255 runs; explain any difference by method (AC20)
- [x] Time `dashboard` before and after, and narrow `REOPENING_WINDOW_DAYS` if the difference is noticeable; state the times in the commit

- **Done when**: the tests pass, each fails with its behavior removed, the baseline result is recorded, and `/{project}:status` shows the counts in an adopter-shaped fixture project.

## 8. One description per primitive (AC9)

Independent of tasks 1–7, and may run beside them, but `runtime/src/mcp/server.rs` and `runtime/src/main.rs` conflict with tasks 6 and 7, so land it after them.

- [x] Create `runtime/src/schema/descriptions.rs` and point every `#[tool]` and clap subcommand at it, MCP text byte-identical except the claims this spec corrects, with the first sentence as `about` and the whole text as `long_about`, and give the CLI-only primitive one
- [x] A test in `runtime/tests/mcp.rs`: each served tool's description equals its subcommand's `long_about`
- [x] Remove what restates a description from module docs a task already touches, leaving maintainers' notes

- **Done when**: the test passes, fails when one description diverges, and no primitive's description appears as a second string in `runtime/src/`.

## 9. The constitution (AC2, AC14, AC15, AC17, AC21, AC9)

Blocked on tasks 2–8, so it states what exists. Steps 10 and 19 now have a host that cannot ask apply the rule and leave each gating finding undispositioned, with only `ductus exec` writing nothing, so §brownfield-inbox's *with no operator, nothing is written* changes with the rest of that bullet.

- [x] The four edits in the plan, keeping every anchor a `§` reference resolves
- [x] `scripts/audit/cross-doc-consistency.sh` and `framework/commands/*.md`'s host-integration pointers follow them
- [x] `050-constitution`'s scenario `the-constitution-fits-one-read` records the shape

- **Done when**: `scripts/lint-governing-size.sh` passes with a smaller constitution than `2abc19aa`'s 49,936 bytes, `scripts/audit/run-all.sh` passes after the commit, and `resolve_anchor.rs`' corpus test passes.

## 10. Correct the remaining false claims (AC12)

Blocked on tasks 2–9: read `research.md`, which is a floor, then grep each behavior's words again, since the code has moved. A statement made false is corrected, including one that a data model is canonical; a copy that is still true waits until next edited. Tasks 3 and 5 already corrected 078, 058 and 047, so `research.md`'s counts for those are spent, and a search of 050, 063 and 066 for a claim that every disposition is confirmed found none, so their counted claims are other behaviors'. Known still false: 066's data model lists `fix-round-not-re-read` as the gate's last check with no `fix-round-loop-open`, and its scenario `review-scope-union` states the old union. Task 6 renamed `check-artifact-size`'s `if-claim-changes` to `if-requirement-changes` and moved a `data-model.md` trim to `always`, so 063's `spec.md` (its behavior line and ticked AC8), `plan.md`, `data-model.md` and scenario `a-plan-trim-does-not-reopen-a-done-spec` state the old hints. Task 9's sweep caught only the exact phrases, so AC21 leaves the no-server wording standing in other forms: the opening lines of `framework/reference/{consolidate,groom,fold}.md`, `framework/reference/clarify.md`'s *Deriving the list*, `framework/reference/review.md`'s empty-section line, `framework/reference/audit.md`, `framework/commands/audit.md`, and the migration `session-file-consolidate`. `framework/bootstrap/ductus.md`'s pointer also says the two paths share a contract, though the bootstrap runs before acquisition, so whether that stays as its own contract is decided here, with 029's scenario `state-a-deterministic-path-forcing` read first.

- [x] In-progress specs: 000, 016, 017, 020, 022, 041, 045, 047, 050, 054, 063, 066, 075
- [x] 011 and 046 (`done`): `classify-spec-edit` returns `correction`, so they stay `done`
- [x] 023 and 076 (`done`): reopen by `set-status`, correct the scenario and the data model, and each owes a review and an analysis
- [x] 059 (`done`): reopen, correct ticked AC3 by the back-edge, and it owes an analysis
- [x] Read AC21's 112 lines in context, and correct those that prove false, reopening any `done` spec whose scenario or data model holds one (at most 7); name the count in the commit
- [x] Signposts back-linking this spec on 020, 022, 026, 050, 066 and 076, so `cross-spec-impact:` is discharged
- [x] Runtime doc comments and MCP text the sweep names, with `scripts/` and the tests and fixtures that assert the old behavior
- [x] `scripts/mutation/run.py` exits non-zero when the ids it is given select no mutation: an unsplit id list once selected none and reported success, which §design-principles rules out

- **Done when**: re-running the sweep's searches finds no false claim, `check-review-gate` reports `cross-spec-impact:` discharged, and the corrected specs' reviews and analyses are recorded or named as owed on their own tasks.

## 11. The prose cuts (AC18, AC21)

Blocked on tasks 2–10: cut what is true after them. Task 9 shortened the `For agent runtimes` pointers, which took `framework/commands/` and `framework/reference/` to 567,487 bytes at `ad4fae5b`, under AC18's total, so what is owed is the two named files and the total staying under while tasks 10 and 11 land; of the two boilerplate blocks, `**Where a named section lives.**` remains. Task 10 left `framework/reference/log.md` restating `framework/commands/log.md`'s step 4 and nothing else; it stays, since 071's AC2 and its entry on which commands are split name `log`, and removing about 500 bytes does not justify reversing them. `framework/reference/review.md`'s §Deferrals calls itself the one full statement of when a deferral releases a finding, and `write-review`'s and `write-analysis`' descriptions cite it, which is a primitive's rule in a reference, the inverse of AC21: decide here whether the descriptions state it and the reference drops it, or it stays as policy. An edit to a command source can move its golden, and `runtime/tests/parity.rs`' failure message says how to re-bless it alone. Take `research.md`'s sections as a starting estimate, drop text rather than move it into a description, and keep every heading's exact text and `§Resolve target and scope`'s item numbers.

- [x] `framework/reference/review.md` under 50,000 bytes
- [x] `framework/reference/analyze.md` under 50,000 bytes, with `scripts/audit/marker-list-parity.sh` following its sentences; if cuts stop short, bring the split to the owner with what it costs
- [x] The two boilerplate blocks across commands and references, keeping `scripts/lint-tool-coverage.sh`'s match
- [x] Restatement in the largest commands
- [x] A finding's message carries what the removed prose explained, wherever a family's rule is no longer stated elsewhere
- [x] Close `020-code-review`'s task 16 by pointing at the result

- **Done when**: `wc -c` shows `framework/commands/` and `framework/reference/` under 567,705 bytes together and each named file under 50,000, `check-step-references` and `resolve_anchor.rs`' corpus test pass, and `scripts/audit/run-all.sh` passes after the commit.

## 12. Verify, land and measure (AC8, AC20)

Blocked on tasks 1–11.

- [ ] Walk each criterion against the tree, and check each shipped change is in the runtime, `framework/commands/`, `framework/reference/` or the constitution (AC8)
- [ ] A real adopter bootstrap on a subject far enough behind to cross the chain, since this changes `framework/commands/` and adds primitives (`AGENTS.md`)
- [ ] Run `/ductus:review` and `/ductus:analyze` on this spec
- [ ] Note the landing commit, then run `measure-reopenings --since` on it as later specs close: 058 (its task 5), 016, 029, 034, 037, 039, 045, 055, and others, until 10 reopenings holding a run have closed
- [ ] A miss gets a task on this spec before it closes, and the count carries on
- [ ] Propose `done` with the completion gate

Starting point: tasks 1–11 and 13 are done, and no criterion of this spec is ticked yet, so subtask 1 ticks them with `mark-criterion` as each is verified. AC18 was met at `fd1054dc` (`wc -c`: `framework/reference/review.md` 43,198, `framework/reference/analyze.md` 47,694, the two directories 426,983) and is re-measured here. This spec has no review or analysis record yet.

Assumption to check before subtask 2, not yet tried: `version` pins 0.59.11, whose released binary has neither `classify-spec-edit` nor `measure-reopenings`, and the owner will not tag before every spec is `done`, so a `--ref=main` bootstrap likely needs `[runtime] path` naming a local build. Read `framework/bootstrap/ductus.md` §Pre-flight Phase and `docs/developing-ductus.md` first.

Subtask 4 cannot finish in one session: once 1–3 are done, record the landing commit here and hand off. Task 10 reopened 010, 014, 019, 023, 030, 032, 059, 062 and 076, each holding a task naming the runs it owes, so each counts toward the 10 as it closes. Handoff: 078's task 15 waits for this spec to be `done`, and 066's session resumes once tasks 2–5 land, rebasing its held work onto them. AC20's baseline comparison is in `aaa4287f`'s message: 126 closed and 261 runs against the recorded 123 and 255, the difference by method, and a plain per-spec `git log` walk reproduces 123 and 255 exactly. Ask the owner at this task whether the outcome entry in Resolved Questions gains the primitive's figure beside the recorded one, or AC20 reads as met by that re-derivation; `spec.md` had 523 bytes of room by `wc -c`, so a longer entry needs a cut, which is also the owner's.

- **Done when**: the 10 closings are measured within the bound or each miss has a finished task, `check-review-gate` passes, and the owner approved `done`.

## 13. Disposition out-of-spec finding: claims stale before this spec, found checking the cuts

- [x] `specs/054-remove-supersession/spec.md` — AC10's annotation counted nine families against the ten the description lists — fixed
- [x] `specs/023-govern-refinement/spec.md` — AC21 cited an `AGENTS.md` account that `4eebe91a` removed — fixed
- [x] `specs/016-cross-cutting-rules/spec.md` and `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md` — AC3 each named `framework/commands/analyze.md` for a section 071 moved to its reference — fixed
- [x] `specs/024-rule-loader/spec.md` — AC8 names `framework/commands/review.md` for §Notes for adopters, which 071 moved — waived: the claim holds at the reference, and a criterion edit would reopen a `done` spec for a location alone
- [x] `specs/071-command-reference-split/spec.md` — AC17, unticked, names `AGENTS.md`'s entry on a new command, now in `docs/developing-ductus.md` — waived: 071's own open criterion, which its implementation settles
- [x] `specs/000-slash-commands/scenarios/dashboard-dependencies-column.md`, `specs/014-reclarify-backedge/spec.md`, `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`, `specs/066-audit-records-and-the-completion-gate/scenarios/an-empty-scope-flag-does-not-decide-the-denominator.md` and `specs/067-analyze-and-audit-checks/scenarios/frontmatter-severity-tiers-on-both-sides.md` — each Context names command text that has since changed — waived: each describes the state its scenario or spec set out to fix, an account rather than a claim about the file today

- **Done when**: each finding is fixed, deferred, or waived, with a waiver's reason written on the task.
