# 087 — Parts moved out of a project file are read, measured and pruned as that file Tasks

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

## 1. One definition of the part set

Blocked on 084 reaching `done`: 084's Resolved Questions, *Landing order*, has every other open spec's implementation follow it (owner, 2026-11-24). 084's task 17 also rewrites `check_artifact_size.rs`. Start once `check-review-gate` on 084 has passed and its status reads `done`.

- [x] Create `runtime/src/primitives/project_parts.rs`: the four parts directories keyed by `ProjectFileKind`, each directory's parts in `scenario_name_cmp` order, linkage from each project file's links, and the five integrity finding kinds, as the plan's *One definition of the part set* states
- [x] Make `link_targets`, `link_destination` and `lexical_join` in `runtime/src/primitives/check_corpus_links.rs` `pub(crate)`, and call them rather than copy them
- [x] Unit tests: a markdown file directly under each of the four directories is a part whether or not it is linked; a non-markdown file there is neither a part nor a finding; each of the five finding kinds fires on its fixture and not on a clean one; a part that is a link leaving the project is `unreadable` and never opened; a project with no `.ductus/docs/` yields no part and no finding

- **Done when**: the module's unit tests pass, and each is proven to fail with its behavior removed (AC10).

## 2. `check-artifact-size` measures parts and lists the findings

- [x] Add `parts`, `part-findings` and their counts to the project half's result in the bounded set, the `project-part` kind and its split fix, and `examined` counting parts (`runtime/src/primitives/check_artifact_size.rs`, `runtime/src/schema/primitives.rs`)
- [x] Update `CHECK_ARTIFACT_SIZE` in `runtime/src/schema/descriptions.rs`
- [x] Unit tests for AC1 and AC2's runtime half: over a fixture index `system.md` and one part, the result lists the part under its project file with state `measured`, and with the part unreadable, with state `unreadable` and in `skipped`
- [x] Unit tests for AC4 (a part one byte over is reported with its path, size, threshold and fix; one at the threshold is not), AC6 and AC12 and AC20 (each integrity kind listed), AC11 (any markdown file directly under a parts directory is measured), and a stored waiver covering a part by the decided rule
- [x] AC8 fixture: four project files and no `.ductus/docs/`; a feature-only call is byte-identical to before, and a `project-files` call keeps every earlier key and value with `parts` and `part-findings` empty
- [x] Update `runtime/tests/result_sizes.rs` for the new lists
- [x] In `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md`, add the new fields, kind and fix, and correct **Size**

- **Done when**: the tests pass and each is proven to fail without its behavior (AC10); 063's data model states the result a test run writes; the commit records each registry layout's impact (AC18).

## 3. The staged check and the hook refuse a grown part

- [x] Under `staged`, read the part set from the index's entries directly under each parts directory through `ProjectRepository::discover` and `to_git`, with `head-bytes` and `refused` per part; leave `part-findings` absent
- [x] Add the inline `refused-count`; have `runtime/src/main.rs` exit on it and name each refused part on stderr, reading `parts` from the lists file in the file form
- [x] `runtime/tests/check_artifact_size_cli.rs`: a staged part over the limit with no `HEAD` copy, or larger than it, exits 1 naming its path, size, limit and fix, in both forms; a shrunk or unchanged over-limit part exits 0; a deleted part passes
- [x] Create `runtime/tests/size_hook.rs`, which runs the shipped `framework/bootstrap/hooks/ductus-pre-commit` (not `.githooks/`) over a fixture repository, as `runtime/tests/consolidate_hook.rs` does: a commit growing a part past the limit is refused, and one shrinking it passes, which covers AC5 including a project in a repository subdirectory

- **Done when**: AC5's tests pass and are proven to fail without the refusal (AC10); the commit records each registry layout's impact (AC18).

## 4. `/analyze` reads parts and records the findings

Tasks 1–3 landed the result this task reads: `check-artifact-size` with `project-files` carries `parts`, `parts-count`, `part-findings` and `part-findings-count` in its project half, and, staged only, `refused` and `refused-count` (063's `data-model.md`, *The project half*). An unreadable part, and a directory under `.ductus/docs/` that could not be listed, are in `skipped` under `artifact-unreadable`.

- [x] `framework/commands/analyze.md`: Scope Boundaries name the parts; step 15 invokes `check-artifact-size` with `project-files` and no feature before judging the plan against `system.md`, reads every part listed and names them; step 17 records each `part-findings` entry as Advisory, family `project-parts`
- [x] `framework/reference/analyze.md`: in §Plan consistency (blocking if plan exists), *How each runs*, the judgment covers the file and its parts, and an `unreadable` part is recorded unexamined once under `artifact-unreadable` (AC2); §Artifact size (advisory) states the findings and that none holds `done`
- [x] `runtime/src/interpreter/analyze_tally.rs`: the `check-artifact-size` arm adds `part-findings-count` to advisory; an interpreter test shows an exec `/analyze` counting an unlinked part
- [x] Add the signpost in `specs/022-deterministic-runtime/spec.md`, a blockquote linking this spec, for the tally and the scanner's visibility
- [x] In `specs/067-analyze-and-audit-checks/data-model.md`, make the `plan-consistency` entry's "a contradiction of `system.md`" cover its parts, and add a signpost in `specs/067-analyze-and-audit-checks/spec.md`

- **Done when**: the interpreter test passes and is proven to fail without the arm (AC10); a fixture `/analyze` run over a project with an unlinked part and a stray directory records each as an Advisory `project-parts` finding, and with the part unreadable records the plan-against-`system.md` judgment unexamined (AC1, AC2, AC6, AC12, AC20); the commit records each registry layout's impact (AC18).

## 5. `/plan` reads parts

- [x] `framework/commands/plan.md`: Scope Boundaries name `specs/system.md` and its parts
- [x] `framework/reference/plan.md`: in §Cross-spec context checklist, *Always read* takes `system.md` and every part `check-artifact-size` with `project-files` lists under it, `events.md` and `errors.md` with their parts under their conditions, and the run names the parts it read; in §Create the plan step 3 and §Validation gate, the check covers the parts, and an `unreadable` part is reported unexamined in the gate prompt
- [x] Correct `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/project-files-are-measured.md`'s sentence that `/plan` keeps the feature-only call: step 8 does, and the read call is a second, `project-files`-only one
- [x] Add the signpost in `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`

- **Done when**: the three command files (`plan.md`, its reference, `analyze.md`) direct the run to read every listed part before judging and to name them (AC1); the commit records each registry layout's impact (AC18).

## 6. `/implement` and `/review` read the parts a pointer names

Blocked on 084's tasks 8 and 10, which edit `framework/reference/review.md`; task 1's wait covers them.

- [x] `framework/reference/implement.md` step 4 and `framework/reference/review.md`'s cross-reference with `specs/system.md`: the parts those files' pointers name for the work in hand, and the run says which parts it read

- **Done when**: both references state it (AC3); the commit records each registry layout's impact (AC18).

## 7. `prune-project` walks parts

This task's mutations, and every later runtime task's, continue in `scripts/mutation/project-parts-087.py` from M40.

- [ ] Add each `measured` part as a subject after the four files, segmented by its project file's grammar, with its record in the new `part-files` list (`files` stays the four project files); have `check_args` accept a part's path; add `part-findings`; bound `part-files` and `part-findings` with `entries` (`runtime/src/primitives/prune_project.rs`, `runtime/src/schema/primitives.rs`, `PRUNE_PROJECT` in `runtime/src/schema/descriptions.rs`)
- [ ] Unit tests: a fixture index and part yield the part's entries, and an apply removes a part's entry; a part of `events.md` joins the literal search; past the read size the new lists are in the lists file with their counts inline; the AC8 fixture keeps every earlier key and value
- [ ] Update `prune-project`'s case in `runtime/tests/result_sizes.rs`
- [ ] `framework/commands/prune.md` *Project files*: walk the parts, show `part-findings` in the preview, and route an architecture entry to `system.md` or the part it belongs in (AC7)
- [ ] Update `specs/041-task-pruning/data-model.md` §`prune-project`

- **Done when**: the tests pass and are proven to fail without their behavior (AC10); a `/prune --project` run over the fixture proposes the part as the home of an architecture entry that belongs in it (AC7); the commit records each registry layout's impact (AC18).

## 8. `prune-project` moves a pointer's target

- [ ] Preview: `move-candidates`, and with `move` the literal, boundary-bounded referrer search with occurrence counts over every tracked file, the spec root included and `from` excepted; both lists bounded with `entries`
- [ ] Apply with `move` and the confirmed referrers: write `to` create-only, rewrite each confirmed referrer keeping its prefix, rewrite the moved file's own relative links, remove `from`, each under the write guard; refuse an unfound referrer and keep an unconfirmed one
- [ ] The CLI's `move` argument (`runtime/src/main.rs`), as `--replace-json` carries a list, through a file in the system temp directory
- [ ] Unit tests: AC14's fixture (an `AGENTS.md` pointer to a markdown file under `docs/` that a second file, under the spec root, also links) moves and re-points both links, and rewrites the moved file's own relative links; a declined move writes nothing; a referrer changed since the read is refused; a longer path ending in `from` is not matched
- [ ] Update the `result_sizes.rs` case for the move's lists
- [ ] `framework/commands/prune.md`: the procedure route to `.ductus/docs/agents/`, a README or a named skill (AC13); the move proposals before the entry walk, showing the search and confirming each referrer, after which the operator may end the run with each unjudged entry reported as not examined; Scope Boundaries naming the referrer writes
- [ ] Update `specs/041-task-pruning/data-model.md` and add the signpost in `specs/041-task-pruning/spec.md`

- **Done when**: the tests pass and are proven to fail without their behavior (AC10); a `/prune --project` run over AC14's fixture shows the search, moves the file and re-points both links on confirmation, and writes nothing when declined (AC14); the same run ended after the move proposals reports each entry it did not judge as not examined, never as kept (AC21); the commit records each registry layout's impact (AC18).

## 9. The routing text and the constitution

Task 3 made the shipped hook refuse a grown part, so the constitution's §text-first-artifacts sentence on the hook's refusal (*Write each artifact as short as its required content allows*) and `README.md`'s `[artifacts]` entry, which name a project file alone, change with this task's constitution edits.

- [ ] `specs/075-agents-md-holds-what-every-session-needs/spec.md`: the routing row *A procedure for one kind of task* names `.ductus/docs/agents/` in place of `docs/`, plus a signpost
- [ ] `framework/bootstrap/ductus.md`'s `ductus:memory-routing` line, and its §Project Configuration `read-size-bytes` comment, copied byte-identically to `framework/bootstrap/govern.md` (AC19)
- [ ] `README.md`'s `[artifacts]` entry: `read-size-bytes` limits parts too
- [ ] `framework/constitution.md`: the §spec-phase tree line, §drift-prevention's routing, *No dead references*' live artifacts, and the canonical-sources row (AC9, AC13); re-measure with `wc -c`, and cut a restatement in the same commit if it passes 50,000 bytes
- [ ] Add the signpost in `specs/050-constitution/spec.md`

- **Done when**: `scripts/lint-governing-size.sh` and audit Family 21 pass; the constitution and 075's table name the parts and `.ductus/docs/agents/` (AC9, AC13, AC19); the commit records each registry layout's impact (AC18).

## 10. A shared constitution's parts

- [ ] `resolve-constitutions`' measured entry: each `loaded` record's `parts` and `parts-count`, measured and worded as the document is, bounded through the lists file; the unmeasured function reads none (`runtime/src/primitives/resolve_constitutions.rs`, `runtime/src/schema/primitives.rs`, `RESOLVE_CONSTITUTIONS` in `runtime/src/schema/descriptions.rs`)
- [ ] `framework/commands/target.md` step 4: each part's bytes beside its document, one over the limit named with its owning repository as the fix, and a skipped entry's parts as not measured
- [ ] Unit tests: a fixture part one byte over the limit carries its warning naming the `repo`; a not-checked-out entry carries no parts; an exec `/analyze` over a registered constitution with an over-limit part records no finding for it (AC17)
- [ ] `specs/055-shared-constitution/data-model.md` (a loaded record's parts, **Bound**, and its sentence that nothing else in the checkout is read), and in `specs/055-shared-constitution/spec.md` the departure reason and a signpost

- **Done when**: the tests pass and are proven to fail without their behavior (AC10); `/target` over the fixture reports the part's bytes, names the over-limit part with its repository, reports a not-checked-out entry's parts as not measured, and continues (AC16, AC17); the commit records each registry layout's impact (AC18).

## 11. The migration entry

Task 3 edited the shipped hook's step-4 comment and `framework/bootstrap/ductus-procedure.md` without a scratch-adopter run, which this task's three bootstrap runs, installing that hook, stand in for (the agent's recommendation, adopted by the operator, 2026-11-25): confirm in each run that the installed `.githooks/ductus-pre-commit` carries the parts wording.

- [ ] `framework/migrations/project-file-links-into-docs.md`: run `prune-project` as a preview through the CLI at the pointer path Pre-flight acquired, never the session's MCP server, and report each `move-candidates` target under `docs/` with its project file and `/{project}:prune --project`, writing nothing; a result without `move-candidates` is reported as not checked, naming the command
- [ ] Register it in `framework/migrations.toml`, with `introduced_in` read from `version` when this task runs
- [ ] Three scratch-adopter bootstrap runs, per **§Exercising an installer change without a push** in `docs/developing-ductus.md`: one adopter whose `AGENTS.md` links a markdown file under `docs/`, one with no such link, and one whose pointer binary predates `move-candidates`

- **Done when**: the first run reports the link and names `/{project}:prune --project`, moving and rewriting nothing, the second reports nothing, and the third reports not checked (AC15); the commit records each registry layout's impact (AC18).

## 12. This repository's procedure file moves

Runs after every other task, from a session started after task 8's build is installed: `mcp__ductus__*` serves the binary the session started with, so an earlier session's server lacks `move`.

- [ ] Run `/ductus:prune --project` here, confirm the move `docs/developing-ductus.md` → `.ductus/docs/agents/developing-ductus.md` with every referrer the search finds, this spec's own files included, and end the run after the move, before the entry walk
- [ ] Commit the rename and substitution alone, as a mechanical sweep
- [ ] In a commit of its own: add `.ductus/docs/` to `AGENTS.md`'s sweep-targets list; re-measure `AGENTS.md` against its 50,000-byte ceiling; restore each line of this spec's `spec.md` and `plan.md` that the substitution made `X → X`, or that records a measurement of the old path, to the old path in the `` ` → ` `` form or to non-path form
- [ ] Run `scripts/audit/run-all.sh` after the commits

- **Done when**: the file is at its new path; every line `git grep -n 'docs/developing-ductus.md'` lists records the move in the `` ` → ` `` form; the two commits' diffs hold no `X → X`; `check-corpus-links`, `scripts/lint-governing-size.sh` and the audit run pass; the sweep commit names each file the move wrote.

## 13. `scripts/analyze/` follows step 15's and step 17's parts

- [ ] `scripts/analyze/split.py` and `scripts/analyze/briefs.py`: step 15's subjects take each `measured` part that `check-artifact-size` with `project-files` and no feature lists under `specs/system.md`, and `scripts/analyze/step15-brief.md` judges the plan against the file and those parts, naming an `unreadable` part as unexamined, never as no contradiction
- [ ] `scripts/analyze/record.py` generates an Advisory `project-parts` finding from each `part-findings` entry of detection's `check-artifact-size` result, as `reference_findings` generates step 13's, and `scripts/analyze/detect.py`'s summary of that result shows `part-findings`

- **Done when**: a run of `scripts/analyze/` over a fixture project with an index `system.md`, a linked part and an unlinked part lists the linked part among step 15's subjects and records the unlinked part as an Advisory `project-parts` finding; `framework-checks.yml`'s steps pass.
