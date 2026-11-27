# 089 — The pipeline's own cost is stated and bounded Tasks

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

## 1. `AGENTS.md` is cut by `/ductus:prune --project`

- [ ] Run `/ductus:prune --project` over `AGENTS.md` and its part, confirming each move with the owner, routed as the plan's *`AGENTS.md` is cut by `/ductus:prune --project`* says; promote no entry into the constitution
- [ ] Where a lesson leaves because a check reports it, the check's message or a comment at its site names the fix in the same commit (072)
- [ ] Record `AGENTS.md`'s size after the cut (`wc -c`) and each part's, and name for the owner any entry true for every adopter that prune drafted an upstream report for

- **Done when**: every `AGENTS.md` entry has been judged by the walk, `AGENTS.md` is at most 49,000 bytes by `wc -c`, each part is at most 50,000 (`check-artifact-size` with `project-files` reports no oversized part and no part finding), and the commit message records `AGENTS.md`'s size and the routes taken.

## 2. The constitution gains the cost rule and is cut

- [ ] Add AC1's bullet to §design-principles, beside *Use what a task needs*
- [ ] Move *In `/{project}:implement`, scope decides the destination* and *Record, do not derail* into `framework/reference/implement.md`, leaving one pointer in §brownfield-inbox; correct the signpost in 050's scenario `findings-route-by-scope` that places the first bullet there, and re-point `AGENTS.md`'s entry on a finding made during implementation
- [ ] Cut the three primitive-rule copies and tighten the host paragraph as the plan's *The constitution keeps every rule* lists, then tighten elsewhere under the as-short-as rule until the file is at most 49,000 bytes: before cutting a passage, search `specs/*/spec.md` and `specs/*/scenarios/*.md` for it, keep any passage another spec quotes or correct that spec through its back-edge, and keep every anchor
- [ ] If 49,000 cannot be reached without dropping required content, stop and ask the owner the ceiling as an open question through `/ductus:amend`
- [ ] `scripts/gen-claude-commands.sh` for the reference edit

- **Done when**: `framework/constitution.md` holds the AC1 bullet and is at most 49,000 bytes by `wc -c` (its size recorded in the commit message), its 31 `<!-- §anchor -->` markers are unchanged (diffed before and after), `scripts/audit/shipped-anchor-resolution.sh` passes, and no ticked criterion or scenario that quotes a cut or moved passage is left false.

## 3. The analyze reference is cut, its primitives' rules moved into their descriptions

- [ ] Cut the copies and duplicates the plan's *The analyze reference moves the primitives' rules into their descriptions* names, keeping every pinned span, heading and Family 18 list verbatim
- [ ] Move the named rules into `CHECK_ARTIFACTS`, `CHECK_ARTIFACT_SIZE` and `WRITE_ANALYSIS` in `runtime/src/schema/descriptions.rs`, each sentence first checked against the code that enforces it, and leave one pointer line per section
- [ ] `runtime/CHANGELOG.md` `[Unreleased]`: the three descriptions state the rules the reference held
- [ ] `scripts/gen-claude-commands.sh`; `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --test mcp` and the `check_step_references` tests; `scripts/audit/marker-list-parity.sh`, `scripts/audit/template-alignment.sh` and `scripts/audit/step-reference-integrity.sh`

- **Done when**: `framework/reference/analyze.md` is at most 50,000 bytes by `wc -c`, every heading a step, audit family or `check_step_references` names still exists, each criterion the plan lists as pinning this file still holds, and the runtime checks above pass.

## 4. The affected specs link back, and 060 is told

- [ ] 089's `cross-spec-impact:` gains 063, 066, 067 and 079
- [ ] A blockquoted signpost to 089 in 050, 063, 067, 072 and 075, each naming what 089 changed there, as the plan's *Cross-spec impact* says
- [ ] Tell the other lane through `SendMessage`, before editing 066's or 079's spec and again once landed, and tell it that once task 6 lands the hook refuses any commit leaving a command or reference file over 50,000
- [ ] 066's signpost; 079's signpost beside its Resolved Question on AC18, and its AC18 note, worded per the answer 086 task 7 recorded (asked of the owner if none is recorded) and landed with or after 086 task 7's notes, so one review of 079 covers AC6, AC18 and AC24
- [ ] Append to 060's task 15 a checkbox: state in 060's `spec.md` what a run of the scenario judgment costs and what it retires, or why none, beside a blockquoted signpost to 089 (089's AC1)
- [ ] `derive-dependencies` reports no drift and no cycle

- **Done when**: every spec in 089's `cross-spec-impact:` links back to 089, 060's task 15 carries the checkbox, and `derive-dependencies` reports every affected spec's `dependencies:` unchanged and no cycle. The review of 079 the note owes runs before the release tag.

## 5. The review reference is cut

Lands after 086 task 4's edit to §Window and coverage reaches `origin/main`. The other lane (`ductus-2-f9`) asked for that order and messages when it lands; tell it through `SendMessage` when the cut starts and when it lands. Task 4 adds about 820 bytes to three of that section's bullets: "A review reads its own spec's change, and the record names the rest", the empty-diff-base bullet, and "`scope` counts entries, not files". That puts the projection near 49,350. The last two bullets are ones the plan keeps verbatim, so re-read them after the rebase. `COMPUTE_REVIEW_SCOPE` states task 4's new rule, so its added text may reduce to a pointer.

- [ ] Cut the copies the plan's *The review reference cuts copies only* names, keeping §Blocking semantics' heading with one pointer line, one line on attribution and the artifact set in §Window and coverage, and every pinned span and heading verbatim
- [ ] If the result is over 50,000 bytes for any reason, move the four spans the plan names into `discover-rule-files`' and `process-waivers`' descriptions, with a `runtime/CHANGELOG.md` line, and run task 3's runtime checks
- [ ] `scripts/gen-claude-commands.sh`; `cargo test` for `review_reference_names_the_rule_file_mapping` and `check_step_references`

- **Done when**: `framework/reference/review.md` is at most 50,000 bytes by `wc -c`, every heading a command, `framework/constitution.md`, `AGENTS.md` or another reference points at still exists, and each criterion the plan lists as pinning this file still holds.

## 6. The lint lowers the two ceilings and governs every command and reference

Blocked on tasks 1, 2, 3 and 5: this is the commit AC3 names, so every governed file fits before it.

- [ ] `scripts/lint-governing-size.sh`: write `AGENTS.md`'s and the constitution's ceilings into `CEILINGS` as literals, each ⌈size ÷ 1,000⌉ × 1,000 of the file as this commit stages it, never derived when the script runs; add the `framework/commands/*.md` and `framework/reference/*.md` rows at 50,000, enumerated with `':(glob)'` pathspecs, and a glob matching no indexed file fails with its own message
- [ ] `scripts/tests/test-lint-governing-size.sh`: seed every case's tree with one small command and reference file and case A with every indexed one; for each row, one byte over fails naming file, size, ceiling and fix, and exactly at passes; a command file no row names is governed; an empty glob fails; H re-aimed as the plan says; each case shown to fail with its row removed
- [ ] The header and table comments in the script, `.githooks/pre-commit`'s and `framework-checks.yml` step (o)'s name the governed set and 089
- [ ] Prose-claim sweep for "both ceilings", "50,000" beside `AGENTS.md` or the constitution, and the lint's governed set, in the live-artifact set (`scripts/stale-literals.sh`, and a meaning search of `.ductus/docs/`, `docs/`, `README.md`, `framework/`)

- **Done when**: both scripts pass at the commit, each new case fails with its row removed, `framework-checks.yml`'s steps pass via `scripts/run-workflow-steps.py`, `actionlint` passes, and `scripts/audit/run-all.sh` passes after the commit (AC2, AC3).

## 7. 089 states its own cost

- [ ] Measure the lint's run at task 6's commit: git processes spawned per run and wall time over 10 runs (`time`), stating the method
- [ ] 089's `spec.md`: a `## Cost` section with those figures, the bytes each `/analyze` run now reads from the grown descriptions (their `wc -c` growth from `d963a791`), and that it retires no check, field or primitive, since it widens an existing check's subject

- **Done when**: 089's `spec.md` carries the `## Cost` section with its method and units.

## 8. AC4: 085 to 088 state their cost

Blocked on 086 task 8, the other lane's: it writes 086's `## Cost` section.

- [ ] Confirm each of 085, 086, 087 and 088 carries a `## Cost` section with a measurable unit and its method, what it retires or why none, and a blockquoted signpost to 089, and that each `## Cost` commit precedes its spec's `status(…): … → done` commit

- **Done when**: all four confirmations hold by `git log` and each spec's text.
