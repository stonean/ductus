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

- [x] Run `/ductus:prune --project` over `AGENTS.md` and its part, confirming each move with the owner, routed as the plan's *`AGENTS.md` is cut by `/ductus:prune --project`* says; promote no entry into the constitution
- [x] Where a lesson leaves because a check reports it, the check's message or a comment at its site names the fix in the same commit (072)
- [x] Record `AGENTS.md`'s size after the cut (`wc -c`) and each part's, and name for the owner any entry true for every adopter that prune drafted an upstream report for

- **Done when**: every `AGENTS.md` entry has been judged by the walk, `AGENTS.md` is at most 49,000 bytes by `wc -c`, each part is at most 50,000 (`check-artifact-size` with `project-files` reports no oversized part and no part finding), and the commit message records `AGENTS.md`'s size and the routes taken.

## 2. The constitution gains the cost rule and is cut

- [x] Add AC1's bullet to §design-principles, beside *Use what a task needs*
- [x] Move *In `/{project}:implement`, scope decides the destination* and *Record, do not derail* into `framework/reference/implement.md`, leaving one pointer in §brownfield-inbox; correct the signpost in 050's scenario `findings-route-by-scope` that places the first bullet there, and re-point `AGENTS.md`'s entry on a finding made during implementation
- [x] Cut the three primitive-rule copies and tighten the host paragraph as the plan's *The constitution keeps every rule* lists, then tighten elsewhere under the as-short-as rule until the file is at most 49,000 bytes: before cutting a passage, search `specs/*/spec.md` and `specs/*/scenarios/*.md` for it, keep any passage another spec quotes or correct that spec through its back-edge, and keep every anchor
- [x] If 49,000 cannot be reached without dropping required content, stop and ask the owner the ceiling as an open question through `/ductus:amend`
- [x] `scripts/gen-claude-commands.sh` for the reference edit

- **Done when**: `framework/constitution.md` holds the AC1 bullet and is at most 49,000 bytes by `wc -c` (its size recorded in the commit message), its 31 `<!-- §anchor -->` markers are unchanged (diffed before and after), `scripts/audit/shipped-anchor-resolution.sh` passes, and no ticked criterion or scenario that quotes a cut or moved passage is left false.

## 3. The analyze reference is cut, its primitives' rules moved into their descriptions

**Working notes.** These come from the inventory of `framework/reference/analyze.md` at `d963a791`, which read all 386 lines. Bytes are removable, by section. Routes: R1 means a description already states the text; R4 means the text duplicates a `framework/commands/analyze.md` step; *move* means the text goes into the named description, leaving a pointer. Before cutting each span, re-read it against its description.

- **Frontmatter schema:** R1 559 (`validate-frontmatter`'s row selection).
- **Spec integrity:**
  - R1 269, the items `check-artifacts` lists.
  - R5 214, the clarify and amend paths.
  - Move the placeholder tokens (105) to `CHECK_ARTIFACTS`.
- **Plan consistency:**
  - R1 782.
  - R4 251, step 15's parts procedure.
  - Move the plan-title link rule (90).
  - Keep the sentence about an unreadable part being unexamined (087 AC2).
- **Plan record:** R1 102.
- **Artifact size:**
  - R1 479.
  - R4 571, step 17.
  - Move "bytes are a proxy … no value switches it off" (185) to `CHECK_ARTIFACT_SIZE`.
- **Task consistency:** R1 234.
- **Grounding:** R4 201.
- **Decision reasons:** R4 119.
- **Scenario consistency:** R1 327.
- **Acceptance-criterion path existence:**
  - R1 295.
  - Move to `CHECK_ARTIFACTS`, about 1,650 net: the scope remainder, the candidate grammar, `root-absent`, `ships-to-adopter`, `gitignored`, and the unbackticked-path note.
  - Keep the live-claim phrase bullet and the negated-creation bullet (Family 18), the heading, and each reason name (045 AC16).
- **Unexamined targets:**
  - R1 388, `write-analysis`' `unclassified` and `unjudged-rules`.
  - R1 about 400 more: `check-artifacts` states the artifact-unreadable-at-`done` exception and the `outward-link` / `target-unparseable` parenthetical.
  - R4 512, steps 5 and 9.
  - Move `constitution-unresolved` (200) to `WRITE_ANALYSIS`.
  - Keep the closed reason classes, which step 18 reads.
- **Body path existence:**
  - R1 197.
  - Move to `CHECK_ARTIFACTS`, about 580 net: the scope remainder, path reading, and the history rule.
  - Keep the body-phrase bullet (Family 18).
- **Line-number citations:**
  - R1 551.
  - Move the scope sentence (80).
  - Keep "prose reference not detected".
- **Acceptance-criterion labels:** R1 about 200.
- **Cross-service references:**
  - R1 747, `resolve-references`' class definitions.
  - Keep the severity mapping.
- **Review state drift:** R1 429, R4 214.
- **Analyze state drift:** R4 116.
- **Disposition drift:** R4 114, R1 about 140.
- **Link-adjacent decision drift:** move "Not covered", the scan exemptions and the message shape to `CHECK_ARTIFACTS`, about 720 net.
- **Rules:**
  - R1 699, `discover-rule-files`' listing and `other-headings`.
  - R4 341, step 5.
  - R2 503: the per-prefix enumeration in "Which ID format governs a file" becomes a pointer to the canonical-sources row.
  - Keep the ladder itself (068, 069, `developing-ductus.md`).
- **Applicable Rules citation consistency:** R4 702, steps 11, 12 and 14.
- **Departure entries:** R4 317 (step 5.1), R1 452.
- **Rule judging in a subagent:** R4 142. 060 AC10 pins the rest.
- **Stored rule verdicts:**
  - R1 2,351: `discover-rule-files`' judge, rejudge, reusable, `reason` and `rejudged-files`; `write-analysis`' `section-digest` and `unjudged-rules`.
  - R4 539, step 11.
  - Keep "a Verification can yield to another rule where that rule's file is also loaded".
- **Project-level consistency:**
  - R1 411, `check-orphaned-references`' roots.
  - R4 502, steps 9 and 10.
- **Finding dispositions:**
  - R4 2,595, steps 18 to 22.
  - R1 622, `write-analysis`' refusals.
  - Keep the decisions' statelessness and key format, and one line stating the order, which the constitution's *Detect, decide, re-check, record* points at.
- **Record fields:** R1 441, the `rules-judged`/`rules-carried` and waived fill. Keep the rest.
- **Untouched:** the intro, Artifact completeness, Scenario open questions, the Promotion criterion, Cross-spec references, Severity tiers.

The headings `scripts/audit/template-alignment.sh` enumerates must stay verbatim. Totals: about 18,300 removable net, plus about 3,530 moved, so the file is projected near 47,500.

- [x] Cut the copies and duplicates the plan's *The analyze reference moves the primitives' rules into their descriptions* names, keeping every pinned span, heading and Family 18 list verbatim
- [x] Move the named rules into `CHECK_ARTIFACTS`, `CHECK_ARTIFACT_SIZE` and `WRITE_ANALYSIS` in `runtime/src/schema/descriptions.rs`, each sentence first checked against the code that enforces it, and leave one pointer line per section
- [x] `runtime/CHANGELOG.md` `[Unreleased]`: the three descriptions state the rules the reference held
- [x] `scripts/gen-claude-commands.sh`; `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --test mcp` and the `check_step_references` tests; `scripts/audit/marker-list-parity.sh`, `scripts/audit/template-alignment.sh` and `scripts/audit/step-reference-integrity.sh`

- **Done when**: `framework/reference/analyze.md` is at most 50,000 bytes by `wc -c`, every heading a step, audit family or `check_step_references` names still exists, each criterion the plan lists as pinning this file still holds, and the runtime checks above pass.

## 4. The affected specs link back, and 060 is told

Lands after 086 task 7 reaches `origin/main`. The other lane (`ductus-2-f9`) messages when it does. The owner chose the form for 086 task 7's notes (2026-11-27): each affected criterion stays ticked and gains an appended **Superseded in part (spec 086).** with one sentence saying what changed, as 084's notes do. So 079's AC18 gains, at its end: "**Superseded in part (spec 089).** Neither reference may grow past the read size any longer: since 089, `scripts/lint-governing-size.sh` holds each `framework/reference/*.md` file to it, and 089 cut both to fit." Rebase onto 086 task 7 first, re-read AC18 as it then stands, tell the other lane before and after the edit, and run 079's review afterwards, which the reworded criterion owes.

- [x] 089's `cross-spec-impact:` gains 063, 066, 067 and 079
- [x] A blockquoted signpost to 089 in 050, 063, 067, 072 and 075, each naming what 089 changed there, as the plan's *Cross-spec impact* says
- [ ] Tell the other lane through `SendMessage`, before editing 066's or 079's spec and again once landed, and tell it that once task 6 lands the hook refuses any commit leaving a command or reference file over 50,000
- [x] 066's signpost; 079's signpost beside its Resolved Question on AC18, and its AC18 note, worded per the answer 086 task 7 recorded (asked of the owner if none is recorded) and landed with or after 086 task 7's notes, so one review of 079 covers AC6, AC18 and AC24
- [x] Append to 060's task 15 a checkbox: state in 060's `spec.md` what a run of the scenario judgment costs and what it retires, or why none, beside a blockquoted signpost to 089 (089's AC1)
- [x] `derive-dependencies` reports no drift and no cycle

- **Done when**: every spec in 089's `cross-spec-impact:` links back to 089, 060's task 15 carries the checkbox, and `derive-dependencies` reports every affected spec's `dependencies:` unchanged and no cycle. The review of 079 the note owes runs before the release tag. So does the review of 050 that task 2's AC15 note owes (`compute-review-scope` reads 050 `review-owed`).

## 5. The review reference is cut

Lands after 086 task 4's edit to §Window and coverage reaches `origin/main`. The other lane (`ductus-2-f9`) asked for that order and messages when it lands; tell it through `SendMessage` when the cut starts and when it lands. Task 4 adds about 820 bytes to three of that section's bullets: "A review reads its own spec's change, and the record names the rest", the empty-diff-base bullet, and "`scope` counts entries, not files". That puts the projection near 49,350. The last two bullets are ones the plan keeps verbatim, so re-read them after the rebase. `COMPUTE_REVIEW_SCOPE` states task 4's new rule, so its added text may reduce to a pointer.

**Working notes.** These come from the inventory of `framework/reference/review.md` at `d963a791`, which read all 838 lines. Bytes are removable, by section. R1 means a description already states the text, R2 the constitution, R4 `framework/commands/review.md` or this file elsewhere, R5 another command. Before cutting each span, re-read it against its description.

- **Resolve target and scope:**
  - R1 530: `discover-rule-files`' header test and unknown-key notices; `write-review`'s empty scope.
  - R4 91, the notices exit code, which Output repeats.
  - Keep item 5 (071 AC13, and the installer's pointer).
- **Load rules:**
  - R1 320, the header paragraph.
  - Keep the rule-file-to-pass lines (`review_reference_names_the_rule_file_mapping`).
- **Run review passes:**
  - R1 340.
  - Keep the false-positive sentence (020 AC18).
- **Criteria step:** R1 503.
- **Write `review.md`:**
  - R1 1,292: the examined and scope paragraph, leaving about a 110-byte pointer; the open-findings sentences; the finding-shape code block.
  - R4 434: "A SHOULD kept as-is is waived", which Waivers and Deferrals restate.
- **Apply `--fix`:** R4 161 (Auto-fix scope).
- **Fix-round loop:**
  - R1 971: snapshot and next base, exempt paths, the `read-re-read` states, the `check-prescriptions` verdicts, the elapsed figures, re-reads-off, the entry shapes.
  - R2 137, the worktree sentence.
  - Keep the first paragraph (067's plan).
- **Record the run:**
  - R1 404, the blocks `check-review-gate` states.
  - Keep the field definitions.
- **Window and coverage:**
  - R1 2,585: the window start and base sources; attribution, keeping one line (081 AC11); the held-record sentence; the `reviewed-against` bullet.
  - R2 554, the two blast-radius bullets.
  - Keep the bullets `AGENTS.md` points at: `examined`, pre-fields unexamined, empty diff base, denominator at write time, and `scope` counts entries.
- **Blocking semantics:** R2 2,011 out, leaving the heading and one pointer line.
- **Waivers:**
  - R1 1,924: the list is one judgment; covers, anchor and expiry; the departure paragraph, keeping the host's reading of `malformed-departures` and `departure-notices`.
  - Keep the heading (073 AC14, 020 AC16).
- **Per-run waiver processing:**
  - R1 2,298, what `process-waivers` states.
  - Keep the host steps.
- **Malformed and duplicate waivers:**
  - R1 297.
  - Keep the open-schema paragraph (076 AC6).
- **Deferrals:**
  - R1 803 (`process-decisions`).
  - Keep "When a deferral releases a gating finding" (076 AC16).
- **The `observations` row:** R5 77.
- **The `analyze` row:** R1 442.
- **Notes for adopters:**
  - R4 197, the header-test sentence.
  - Keep the rule-directory bullet (024 AC8).

The provenance tags inside these spans come to about 335 bytes. The fallback spans that need a description sentence first total about 2,218:

- item 5's suffix mapping, surface selection and disabled-file filter (1,337), into `discover-rule-files`;
- the waiver YAML example (456), into `process-waivers`;
- the example frontmatter block (425).

The file is projected near 48,500, plus task 4's 820.

- [x] Cut the copies the plan's *The review reference cuts copies only* names, keeping §Blocking semantics' heading with one pointer line, one line on attribution and the artifact set in §Window and coverage, and every pinned span and heading verbatim
- [x] If the result is over 50,000 bytes for any reason, move the four spans the plan names into `discover-rule-files`' and `process-waivers`' descriptions, with a `runtime/CHANGELOG.md` line, and run task 3's runtime checks
- [x] `scripts/gen-claude-commands.sh`; `cargo test` for `review_reference_names_the_rule_file_mapping` and `check_step_references`

- **Done when**: `framework/reference/review.md` is at most 50,000 bytes by `wc -c`, every heading a command, `framework/constitution.md`, `AGENTS.md` or another reference points at still exists, and each criterion the plan lists as pinning this file still holds.

## 6. The lint lowers the two ceilings and governs every command and reference

Blocked on tasks 1, 2, 3 and 5: this is the commit AC3 names, so every governed file fits before it.

- [x] `scripts/lint-governing-size.sh`: write `AGENTS.md`'s and the constitution's ceilings into `CEILINGS` as literals, each ⌈size ÷ 1,000⌉ × 1,000 of the file as this commit stages it, never derived when the script runs; add the `framework/commands/*.md` and `framework/reference/*.md` rows at 50,000, enumerated with `':(glob)'` pathspecs, and a glob matching no indexed file fails with its own message
- [x] `scripts/tests/test-lint-governing-size.sh`: seed every case's tree with one small command and reference file and case A with every indexed one; for each row, one byte over fails naming file, size, ceiling and fix, and exactly at passes; a command file no row names is governed; an empty glob fails; H re-aimed as the plan says; each case shown to fail with its row removed
- [x] The header and table comments in the script, `.githooks/pre-commit`'s and `framework-checks.yml` step (o)'s name the governed set and 089
- [x] Prose-claim sweep for "both ceilings", "50,000" beside `AGENTS.md` or the constitution, and the lint's governed set, in the live-artifact set (`scripts/stale-literals.sh`, and a meaning search of `.ductus/docs/`, `docs/`, `README.md`, `framework/`)

- **Done when**: both scripts pass at the commit, each new case fails with its row removed, `framework-checks.yml`'s steps pass via `scripts/run-workflow-steps.py`, `actionlint` passes, and `scripts/audit/run-all.sh` passes after the commit (AC2, AC3).

## 7. 089 states its own cost

- [x] Measure the lint's run at task 6's commit: git processes spawned per run and wall time over 10 runs (`time`), stating the method
- [x] 089's `spec.md`: a `## Cost` section with those figures, the bytes each `/analyze` run now reads from the grown descriptions (their `wc -c` growth from `d963a791`), and that it retires no check, field or primitive, since it widens an existing check's subject

- **Done when**: 089's `spec.md` carries the `## Cost` section with its method and units.

## 8. AC4: 085 to 088 state their cost

Blocked on 086 task 8, the other lane's: it writes 086's `## Cost` section.

- [ ] Confirm each of 085, 086, 087 and 088 carries a `## Cost` section with a measurable unit and its method, what it retires or why none, and a blockquoted signpost to 089, and that each `## Cost` commit precedes its spec's `status(…): … → done` commit

- **Done when**: all four confirmations hold by `git log` and each spec's text.

## 9. Disposition out-of-spec finding: 060's plan says its review window starts after the reopen's first commit

- [x] `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — its line on reviewing 022 says the window "is measured after the reopen's first commit, as the review reference's (`framework/reference/review.md`) §Window and coverage requires"; since 084's AC7 a review window starts where the spec's last review ended (`compute-review-scope`), so the claim has been stale since 084, found by task 5's draft — fixed: the sentence is dropped from 060's plan, a chore that adds no requirement (the agent's recommendation, adopted by the operator, 2026-11-27)

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task
