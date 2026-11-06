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

Blocked on tasks 1–11, and subtask 3 on tasks 14–16, since a change after the review reads the tree would stale it. Subtask 3's review ran and is open: `review.md` is committed with `fix-rounds-open: true`, and `/ductus:analyze` has not run. Subtask 4's count starts at `e7fcde37`, the commit landing AC24, by the owner's choice (2026-11-06), so other specs close while the rest of task 16 lands.

- [x] Walk each criterion against the tree, and check each shipped change is in the runtime, `framework/commands/`, `framework/reference/` or the constitution (AC8)
- [x] A real adopter bootstrap on a subject far enough behind to cross the chain, since this changes `framework/commands/` and adds primitives (`AGENTS.md`)
- [ ] Run `/ductus:review` and `/ductus:analyze` on this spec
- [ ] Note the landing commit, then run `measure-reopenings --since` on it as later specs close: 058 (its task 5), 016, 029, 034, 037, 039, 045, 055, and others, until 10 reopenings holding a run have closed
- [ ] A miss gets a task on this spec before it closes, and the count carries on
- [ ] Propose `done` with the completion gate

Subtask 4 cannot finish in one session: once AC24 lands, record its commit here and hand off. Task 10 reopened 010, 014, 019, 023, 030, 032, 059, 062 and 076, each holding a task naming the runs it owes, so each counts toward the 10 as it closes. Handoff: 078's task 15 waits for this spec to be `done`, and 066's session resumes once tasks 2–5 land, rebasing its held work onto them. AC20's baseline comparison is in `aaa4287f`'s message: 126 closed and 261 runs against the recorded 123 and 255, the difference by method, and a plain per-spec `git log` walk reproduces 123 and 255 exactly.

- **Done when**: the 10 closings are measured within the bound or each miss has a finished task, `check-review-gate` passes, and the owner approved `done`.

## 13. Disposition out-of-spec finding: claims stale before this spec, found checking the cuts

- [x] `specs/054-remove-supersession/spec.md` — AC10's annotation counted nine families against the ten the description lists — fixed
- [x] `specs/023-govern-refinement/spec.md` — AC21 cited an `AGENTS.md` account that `4eebe91a` removed — fixed
- [x] `specs/016-cross-cutting-rules/spec.md` and `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md` — AC3 each named `framework/commands/analyze.md` for a section 071 moved to its reference — fixed
- [x] `specs/024-rule-loader/spec.md` — AC8 names `framework/commands/review.md` for §Notes for adopters, which 071 moved — waived: the claim holds at the reference, and a criterion edit would reopen a `done` spec for a location alone
- [x] `specs/071-command-reference-split/spec.md` — AC17, unticked, names `AGENTS.md`'s entry on a new command, now in `docs/developing-ductus.md` — waived: 071's own open criterion, which its implementation settles
- [x] `specs/000-slash-commands/scenarios/dashboard-dependencies-column.md`, `specs/014-reclarify-backedge/spec.md`, `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`, `specs/066-audit-records-and-the-completion-gate/scenarios/an-empty-scope-flag-does-not-decide-the-denominator.md` and `specs/067-analyze-and-audit-checks/scenarios/frontmatter-severity-tiers-on-both-sides.md` — each Context names command text that has since changed — waived: each describes the state its scenario or spec set out to fix, an account rather than a claim about the file today

- **Done when**: each finding is fixed, deferred, or waived, with a waiver's reason written on the task.

## 14. What the adopter bootstrap found in this spec's code

- [x] `review-owed` is true while the record holds `done` until a review runs: a gating count, no `dispositions:` map, the open mark or an unwaived round, read through one predicate the gate shares. In the subject, a text-only reopening read `review-owed: false` and was then blocked at `record-predates-dispositions`, with `/review` stopping on the first and the gate asking for it
- [x] A commit that only relocates a `review:` block out of `spec.md` finishes no run, so `last_review_commit` leaves the window at the reopen. In the subject, the relocation commit read as `base-source: last-review`, which would leave commits between the real review and the upgrade unread
- [x] Tests for both, each failing with its behavior removed, and their mutations in `scripts/mutation/scope-attribution.py`
- [x] The Edge Case, `data-model.md`, the plan, `framework/reference/review.md`, `runtime/CHANGELOG.md` and the compiled description state them
- [x] Re-run the text-only reopening in the subject: a record with the map owes no review and reaches `done` on an analysis alone, and one without the map owes a review

- **Done when**: the tests pass and each mutation is killed, and the subject's reopening reaches `done` with one analysis and no review.

## 15. Disposition out-of-spec finding: what the adopter bootstrap surfaced outside this spec

- [x] `framework/bootstrap/ductus.md` — §Collect Project Inputs says a re-run prompts for nothing, but a project adopted before `[paths]` existed is asked for it, and only a first scaffold is told to persist the answer — fixed
- [x] `framework/bootstrap/ductus.md` — §Shared Files' `resolve-constitutions --unmeasured` is refused by the pinned 0.59.11, and a State B run has no shell specification to fall back to — waived: the owner discarded this on 2026-10-30 (058 task 3, option a), since the next tag moves the pin past it
- [x] `framework/bootstrap/ductus.md` — §Detection mechanism read literally made the test's host State A, since its inventory holds another project's `mcp__ductus__*` names — waived: this is the subagent test's shape, and the brief set State B
- [x] `framework/bootstrap/ductus.md` and `framework/bootstrap/ductus-procedure.md` — the numbered walker does not place the archive-half sections, step 10 asks for more than the Update-mode text, and `generator-primitives` and `audit-record-relocate` define no report line — waived: pre-existing wording this spec did not change, which blocked nothing and made no run's result wrong, per `AGENTS.md`'s rule on the pipeline's own machinery
- [x] `framework/bootstrap/ductus.md` — the store's binary mode, and `.ductus/scripts/` left empty after `generator-primitives` — waived: harmless, and git tracks neither
- [x] The pinned 0.59.11 and the latest `install.sh` — group-writable installed files, and 13 retired permission entries the seed then removes — waived: each is fixed on `main` and clears at the next tag
- [x] `relocate-audit-records` lands `review.md` at `0600` on disk — waived: every `write_atomic_bytes` record does (`specs/078-review-fix-rounds-are-re-read-within-a-bound/review.md` is `0600` too), and git records `100644`
- [x] Two builds both report `ductus 0.59.11`, so Branch 1's mismatch check passes while behavior differs — waived: until a version bump a development build reports the pin, as Branch 1 expects
- [x] A review record relocated inside an open reopening counts as a run of it, so `measure-reopenings` can read that one reopening over the bound — waived: one-time, in the direction of over-reporting, and no gate reads it

- **Done when**: each finding is fixed, deferred, or waived, with a waiver's reason written on the task.

## 16. What the flow pass found

`research.md`'s *What the flow pass found* holds each item's input, result and fix options, in this order. Work the first item, since the gaps after it follow from it, then ask the owner each item marked as their choice, one at a time, with its numbers worked out, then the remaining contract breaks and gaps. Add a test shown to fail without each fix, as `docs/developing-ductus.md` says. A review run's Observations in `review.md` are this run's record; `write-review` regenerates the file whole, so its last write must pass them again.

`research.md`'s first, third and fourth items landed in `fc20a939`, `9129d914` and `3b5093da`, which added AC22. The owner's choices on what item 1's fix leaves (`68b643c8`, which added AC23 and `open-findings`), on what the measurement counts (`0e5ff030`), on rules left in prose (`46d0dd6e`), on running without the server (`bbff7a53`), on overriding the rule (`b307b7aa`) and on code committed between a run and its record (`e03558a0`, waived) landed after them. The gate now reads `review-owed` and `open-findings`, and a command stops without the server, so re-run each remaining item's reproduction before fixing it: `research.md`'s item 21, "the gate passes", was written against the old gate and has not been re-run. Next is the first unticked item marked as the owner's choice, in task order, then the rest of them, then the remaining contract breaks and gaps, each with its reproduction re-run first. An item's `research.md` entry holds its options; the spec's Resolved Questions hold how each earlier one was weighed.

The last item closes the run. Two ways, the second cheaper: write it as the item says, passing the 66 observations and these findings; or, once the fixes land, run a fresh `/ductus:review --since=6aef0472`, naming the base and why in its Summary, since the passes at `5f67ede2` and the flow pass read what came before it. A fresh run replaces the record whole, keeps stored `decisions:`, and clears the mark on its last write. Its agents follow behaviors, not file kinds (**§Run review passes** in `framework/reference/review.md`).

- [x] Contract break, the owner chose (2026-11-05) that a record the gate holds does not move the start, so add `holds_done_until_reviewed` to `recorded_run`'s `stopped_early`, and AC5's text already says so: a committed record with an unfixed MUST or SHOULD moves the window past it, so a re-run sees `scope-count: 0`, takes the empty-scope branch and writes `must-violations: 0` with the file unchanged. Also clears an unread round and a contract break, and leaves `--waive` no finding to act on. Reproduced by hand in this session; absent at `3360dea3`
- [x] Contract break, the owner chose (2026-11-06) that the record lists what it leaves open, the next scope holds it, and a break left open holds `done`: what that fix leaves, on inputs where what a finding names lies outside the next window (`research.md`, *What item 1's fix leaves*). An unfixed contract break the gate does not hold, a held record from before the reopening, a spec with no transition, `--since=HEAD`, and a first review's MUST on a plan file the window left unchanged each still end at a count of 0 with the file unread. An unread fix round from before the reopening stays out of reach, waived: it needs a record past the gate, 0 of 120 at the closings since 2026-10-01
- [x] Contract break: a narrowed window expires each stored waiver and decision on a file the run did not read, a person's included, with the notice that the finding no longer fires. Retain an anchor outside the scope
- [x] Contract break, the owner chose (2026-11-06) the gate blocking on an owed review, with a path in another spec's directory owing none: a code-changing reopening following the gate's own message runs analyze, review, analyze and ends over the bound, or stops where the gate first passes with `review-owed: true` and no review, since the gate names `/analyze` first and never reads `review-owed`
- [x] Contract break, the owner chose (2026-11-06) that a pass through draft is one reopening: after the draft back-edge the reopening's earlier code leaves the window and the measure, and `review-owed` reads false
- [ ] Contract break: a spec renamed while `in-progress` starts its window at the rename, its earlier code in no list, and `diff-cross-spec` reads its own old directory as another spec's. Compare status by `FeatureForm`
- [ ] Contract break: on a shallow clone `compute-review-scope` and `diff-cross-spec` answer as if history were whole, with `review-owed: false` and zero impact, while `measure-reopenings` detects the same clone. The fold's walk through all of history takes the same check
- [ ] Contract break: reverting a review's commit, or restoring an older record, moves the window's start to that commit
- [x] Contract break, the owner chose (2026-11-06) that AC17 counts what a record stores: `decisions-by-person` reads only `decisions:`, so a person's `--waive` and round waiver count as none, and a declined deferral leaves no trace, which stays so
- [ ] Contract break: `ductus exec` cannot record `contract-break`, missing from `PassObservation`, so the gate's hold on a break left open never fires for it
- [ ] Contract break: a stored rule waiver matched to an observation now judged a contract break stands, with nobody asked
- [x] Contract break, the owner chose the cut (2026-11-06): `framework/reference/status.md` keeps `dashboard`'s render rules, and `derive-boundary`, `mark-task` and `check-stuck` keep theirs in prose, each `--help` being one line
- [x] Gap, the owner chose (2026-11-06) that the server is required, so a command stops without it: `/review` steps 8, 9, 11 and `/analyze` steps 18, 21 cannot pass what they must through the CLI; CLI `write-review` records a clean run over a MUST and `process-*` read everything as expired
- [ ] Gap, follows the window: `disposition-drift` says to re-run `/review`, which stops on `review-owed: false`, and a forced run erases the findings undecided
- [ ] Gap: `--waive` has no write route, predating this spec; `applied-waivers` clears the run and persists nothing
- [x] Gap, the owner chose (2026-11-06) to narrow the sentence to violations: the spec says `--waive` and `--defer` override any decision, and both take only a rule ID, so a person cannot replace the rule's waiver of an observation
- [ ] Gap: `framework/reference/groom.md` and `framework/reference/analyze.md` tell a host to waive a gating finding `write-analysis` refuses
- [ ] Gap: the gate passes an `analysis.md` whose `decisions:` list does not parse, a hard fail to `validate-frontmatter`
- [ ] Gap: a review owed because a contract changed does not read that contract when its edit lies outside the window, and the digest then refreshes
- [x] Gap, the owner waived (2026-11-06, the spec's Resolved Questions): a record's commit moves the start past code committed after the run, though `reviewed-against` shows where it read
- [x] Gap, the owner chose (2026-11-06) that a fold below `done` passes its commits to its target wherever the window starts: a retired or folded spec's commits stay excluded by a claimant that cannot review them, against `invalidate-review`'s stated purpose, and a fold into a `done` upstream leaves them before the window
- [ ] Gap: a run that dies in step 10, after its fix and before step 11, leaves no mark
- [ ] Gap, the owner's choice: the re-run the gate demands after a committed cut-off reads over the bound
- [ ] Gap: `implement`'s `write-boundary` seed has no route for an interactive host
- [ ] Gap: a review cut off on a `done` spec leaves the open mark where nothing reads it
- [ ] Gap: `prune-plan`'s `reopen-required` and `classify-spec-edit` disagree in both directions on the same tree
- [ ] Gap, the owner's choice: the constitution says `classify-spec-edit` decides and two kinds of edit take no back-edge, and it reads both as `reopens`
- [ ] Gap: `framework/reference/implement.md` sends a signpost on a `done` spec through `/amend`, whose route there reopens it
- [ ] Gap, the owner's choice: the constitution says a criterion that no longer holds reopens its spec, and the classifier reads the untick as a correction
- [ ] Gap: `/amend` advertises a criterion route its classifier, record steps and status table lack, predating this spec
- [ ] Gap: a `cross-spec-impact:` entry added to a `done` spec is a correction nothing then reads
- [ ] The wording items the flows name, fixed or waived by the rule: a record contradicting itself on a person's deferral, the record's `scope` counting its own `review.md`, literal namespaces in result strings, and `/groom`'s spec-edit route
- [ ] Close this run: the last `write-review` lists rounds 1 to 4 as re-read, diff-checked, re-read, diff-checked and round 5, the fold edits, `not-re-read` with the owner's reason; `budget-stop` then derives true, so the next review starts at the reopen. Reconcile `examined` and `scope` in the Summary

- **Done when**: each contract break is fixed with a test shown to fail without it, each gap is fixed, deferred or waived with its reason, the owner's choices are recorded, and AC5 and AC17 are ticked again against the tree.

## 17. Disposition out-of-spec finding: a suffixed task number is invisible to every `tasks.md` consumer

Reported by an adopter on 0.59.11 and reproduced there and at `35a2ed11`, on a `tasks.md` with `### 11. Spent task` and `### 11a. Spent task with a suffixed number` under `## Phase 1`, each fully ticked. `prune-tasks` returns `removed-count: 1`, `kept-count: 0`; `apply` removes `## Phase 1` and task 11, leaving `### 11a.` directly under the `#` title, which fails MD001. With an unticked box under `11a` on a `done` spec, `read-tasks` lists no such task, so `/implement`'s tally misses pending work; `classify-spec-edit` and `measure-reopenings`' `waiting` use the same parser (`parse_tasks`), which was read and not run. `check-artifacts` stays silent, since its numbering check sees only parsed tasks. The cause is `split_numbered_heading` in `runtime/src/primitives/mod.rs`, which takes only digits before the dot. This repository has the form too: `specs/023-govern-refinement/tasks.md` holds `### 1b.`, `### 7b.` and `### 12b.`, and 6 other level-3 headings in `tasks.md` files are unnumbered.

The bug decision tree's route 3: the spec is ambiguous on the form. The decision is the owner's, and is whether a suffixed number is valid. If valid, widen `split_numbered_heading` and every consumer, including `check-artifacts`' `u32` parse and `next_task_number`. If not, `check-artifacts` reports the section, `prune-tasks` accounts for every section at the task level rather than reading zero kept and keeps a `##` phase while a section remains under it, and 023 renumbers. Either way the shared parser is 022's surface and the requirement lands in 041, which is `in-progress`, by a scenario and a task, with a mutation proving the regression test fails before the fix, as the reporter asked.

- [ ] A suffixed number is valid or not, the owner's choice, then a scenario and task on `041-task-pruning` naming this spec's finding — deferred or waived here

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task.
