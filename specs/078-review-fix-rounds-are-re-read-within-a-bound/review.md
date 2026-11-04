---
spec: 078-review-fix-rounds-are-re-read-within-a-bound
last-run: 2026-11-04T13:33:22Z
reviewed-against: 008338d61dab59f569823d5a12b19dfae13be896
diff-base: 059b830ee3a367421e14313ed5f67deb915023f5
must-violations: 1
should-violations: 0
low-confidence: 0
examined: 74
scope: 108
skipped-passes: []
fix-rounds: 3
re-reads: 2
diff-checked: 0
not-re-read: 1
budget-stop: true
round-waivers:
  - round: 2
    reason: "The round's four prescribed edits matched exactly (check-prescriptions: no differing file, no failed edit); its other four paths are the 066 session's uncommitted work in progress, which 066's own review reads."
    waived-by: andy@stone.dev
    waived-at: 2026-11-04T13:33:22Z
reviewed-digest:
  data-model.md: bb9f842677234bcb27e0d8a9c5fd518cd8caa77df4484df911445366f3f45b8f
  scenarios/a-reviews-own-record-is-no-fix-round.md: f1aeb3c3fd26d0e17a98ef6bb433babcabe2abb53a3914e2de4f2ddf33be784d
  scenarios/fix-round-results-have-a-bounded-size.md: bd6472ece8d3586d1de656fd1d1689848d455ed79183249ff003e9db24817de7
blocking: true
dispositions:
  fixed: 25
  deferred: 14
  waived: 1
  undispositioned: 0
decisions:
  - key: "bug: a run's own `review.md` write and its step-10 deferral writes land in its first fix round — `runtime/src/primitives/brief_re_read.rs`"
    outcome: deferred
    target: specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md
    task: "11"
    decided-at: 2026-11-04T12:42:29Z
    decided-by: andy@stone.dev
  - key: "bug: a Summary the caller supplies drops the loop's account, so AC2's naming of exempt rounds rests on the host — `runtime/src/primitives/write_review.rs`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "17"
    decided-at: 2026-11-04T12:42:29Z
    decided-by: andy@stone.dev
  - key: "design: `read-re-read` counts a MUST or SHOULD on a path outside the round as a finding, so AC4 rests on the re-read's own labelling — `runtime/src/primitives/read_re_read.rs`"
    outcome: deferred
    target: specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md
    task: "12"
    decided-at: 2026-11-04T12:42:29Z
    decided-by: andy@stone.dev
  - key: "perf: a NOTE's prescribed edit that the operator declines makes `check-prescriptions` re-read the round whole — `runtime/src/primitives/check_prescriptions.rs`"
    outcome: deferred
    target: specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md
    task: "13"
    decided-at: 2026-11-04T12:42:29Z
    decided-by: andy@stone.dev
  - key: "convention: 058's spec body called the gating-finding check the gate's last — `specs/058-findings-route-at-discovery/spec.md`"
    outcome: deferred
    target: specs/058-findings-route-at-discovery/spec.md
    decided-at: 2026-11-04T12:42:29Z
    decided-by: andy@stone.dev
  - key: rule BE-PAYLOAD-001 at runtime/src/primitives/brief_re_read.rs
    outcome: deferred
    target: specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md
    task: "10"
    gating: true
    decided-at: 2026-11-04T12:42:29Z
    decided-by: andy@stone.dev
  - key: "convention: `the_review_record_remains_a_subject`'s doc comment lost its first line in 066 task 9's change — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "18"
    decided-at: 2026-11-04T13:33:22Z
    decided-by: andy@stone.dev
  - key: "other: `every_other_spec_edit_still_moves_its_digest` ticks both checkboxes in one edit, so normalizing only the fenced one still passes it — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "18"
    decided-at: 2026-11-04T13:33:22Z
    decided-by: andy@stone.dev
  - key: "other: the loop's briefs and checks ran on the binary built before this run's runtime fixes — `runtime/src/primitives/brief_re_read.rs`"
    outcome: waived
    reason: By the owner's 2026-11-04 decision this review's runtime calls run on the committed 65ec2d7f binary; the round's runtime changes are verified by their tests and mutations in clones, and reach the binary when the commit's rebuild runs.
    decided-at: 2026-11-04T13:33:22Z
    decided-by: andy@stone.dev
  - key: "bug: 066's data-model example lists two `modified-since` paths with a count of 1 — `specs/066-audit-records-and-the-completion-gate/data-model.md`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "19"
    decided-at: 2026-11-04T13:33:22Z
    decided-by: andy@stone.dev
  - key: "convention: `compute-review-scope`'s file-form test asserts `deleted` on a fixture that deletes nothing, so three mutations survive — `runtime/src/primitives/compute_review_scope.rs`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "19"
    decided-at: 2026-11-04T13:33:22Z
    decided-by: andy@stone.dev
  - key: "convention: `compute-review-scope`'s module doc still defines `scope` as the plain union — `runtime/src/primitives/compute_review_scope.rs`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "19"
    decided-at: 2026-11-04T13:33:22Z
    decided-by: andy@stone.dev
  - key: "convention: 066's `review-scope-union` edge case contradicts its Behavior paragraph on deleted paths — `specs/066-audit-records-and-the-completion-gate/scenarios/review-scope-union.md`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "19"
    decided-at: 2026-11-04T13:33:22Z
    decided-by: andy@stone.dev
  - key: "convention: the command's Inputs Scope bullet contradicts step 1's deleted-file sentence — `framework/commands/review.md`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "19"
    decided-at: 2026-11-04T13:33:22Z
    decided-by: andy@stone.dev
  - key: "convention: 066's spec and the implement reference still describe the review scope as a plain union — `specs/066-audit-records-and-the-completion-gate/spec.md`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "19"
    decided-at: 2026-11-04T13:33:22Z
    decided-by: andy@stone.dev
  - key: "bug: a window holding only deletions under an empty plan reaches the nothing-to-review-yet branch — `framework/commands/review.md`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "19"
    decided-at: 2026-11-04T13:33:22Z
    decided-by: andy@stone.dev
---

# Review — 078-review-fix-rounds-are-re-read-within-a-bound

## Summary

1 MUST and 0 SHOULD violations counted, 0 low-confidence findings; blocking. The MUST is BE-PAYLOAD-001: `brief-re-read`, `read-re-read` and `check-prescriptions` return unbounded lists and 078's data model commits to no maximum. By the owner's 2026-11-04 decision it is deferred to this spec's task 10 (scenario `fix-round-results-have-a-bounded-size`), which releases nothing, so it holds `done` until the task lands. Three SHOULDs fired and were fixed in the run, so they leave the counts: QUAL-TEST-001 on `brief-re-read`'s classification (no test round mixed an exempt path with code), QUAL-GROUND-001 on the brief's result-file example (no test parsed it), and QUAL-TEST-001 on `bounded_lists`' prune test (it never called `place`); each new test was shown to fail by a mutation, seven run ad hoc and the 066 list's new M14–M17.

The five passes ran on 2026-11-04 over base `059b830e`, the parent of `6fabd051`, 078's `in-progress` commit, measured and not overridden: 078's first commit opens the window, so no later base keeps all of 078's work, and the window also holds 066's and 077's commits made after it. They read the tree at `65ec2d7f` in four read-only agent slices — 078's four primitives; `write-review`'s record and the gate; the bounded scope results with the runtime's registration and walker; and the framework prose, docs and spec artifacts — and the host re-checked every finding and observation against the tree. Every runtime call ran on `runtime/target/release/ductus` built from `65ec2d7f`, since this session's MCP server had loaded an earlier, uncommitted build. `examined: 74` counts the in-scope files the passes read, whole or by the window's hunks, of the 108 the scope resolves at this write. Not examined (34): the 9 generated files (the `.claude`, `.ductus/reference` and `.pi` mirrors, and `framework/bootstrap/govern.md`), which a generator re-run on a clean clone of `65ec2d7f` reported in sync, `govern.md` byte-identical to `ductus.md` by `cmp`; 16 entries other sessions' commits added after the passes (`94c31465` for 077, `7034459b` for 067, `06f870d4`, `6a08a7ad`, `e8781ccf` and `008338d6` for 066); and this run's 9 writes outside the passes' set, this record among them. Those commits also changed 14 files the passes read, whose later hunks the passes did not read: 066's fall inside the fix rounds below, and 077's `spec.md` and `tasks.md` were committed from edits already in the tree when the rounds' base was taken, so no round read them. Partial reads: `scripts/tests/test-deprecation-removal.sh` at its window hunks and lines 1–209, and, in the prose slice, the six `-backend.md` rule files' IDs and statements alone.

Fix rounds: 3, from the base `start-fix-rounds` took at 03:57:16Z (`6a846854`), none exempt. Round 1 — this run's fixes and deferrals, its record, and 066 task 9's change, which landed in the shared tree after the base — was re-read by an independent agent with findings: 3 SHOULDs, fixed by their prescribed edits, and 6 NOTEs. Round 2, those edits and four of the 066 session's files still in progress, had `check-prescriptions` report `differs` only for the 066 files, and was not re-read: the owner waived it. Round 3 — the NOTE fixes, 066 task 18's write and 066 task 10's change — was re-read with findings, all seven on 066 task 10's change, and the budget stopped the loop there, two re-reads having found a SHOULD; the owner deferred the seven to 066 task 19. Task 19's own write lands after round 3's snapshot, the deferral-write case this spec's task 11 makes a round. 40 observations, each dispositioned by the owner on 2026-11-04: 25 fixed in the run, 14 deferred — to this spec's tasks 11, 12 and 13, 066's tasks 17, 18 and 19, and 058's spec body, reopened `done` → `in-progress` — and 1 waived.

## MUST violations (blocking)

### MUST: BE-PAYLOAD-001 — brief-re-read, read-re-read and check-prescriptions return unbounded lists, and 078's data model commits to no maximum

- **File**: `runtime/src/primitives/brief_re_read.rs:84-89`
- **Rule**: Response payloads MUST have a bounded maximum size. Endpoints that return collections able to exceed a small fixed maximum MUST paginate per `BE-PAGE` (api-backend.md) rather than return an unbounded list in a single response.
- **Finding**: `brief-re-read`'s `paths`, `read-re-read`'s `findings` and `check-prescriptions`' `differing`, `unprescribed` and `failed` are unbounded lists in a tool result, and `data-model.md` states no maximum size or pagination for any of them, as 066 found of `compute-review-scope` and `diff-cross-spec`.
- **Auto-fixable**: no
- **Suggested fix**: State `[artifacts] read-size-bytes` as each result's maximum in `data-model.md`, and route the three results through `bounded_lists::place`, as 066 task 7 does.
- **Deferred**: to `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md` task 10

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Deferred findings

*None.*

## Observations

- convention: the review reference called the two disposition checks the gate's last; the fix-round check now follows them — `framework/reference/review.md` — **fixed**
- convention: the review reference said every fix-round field is written whenever rounds are passed; `round-waivers` is written only when a round is waived — `framework/reference/review.md` — **fixed**
- convention: the command's Writes line said no other file is modified, missing `compute-review-scope`'s lists file and `start-fix-rounds`' step-10 write — `framework/commands/review.md` — **fixed**
- convention: step 12's last `write-review` call named no home for the loop's NOTEs and open findings — `framework/commands/review.md` — **fixed**
- convention: the data model said only an empty round returns no snapshot, though `re-reads-off` returns none either — `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md` — **fixed**
- convention: the data model and plan put the rounds `.gitignore` in the feature's directory; the code writes `.ductus/review-rounds/.gitignore` — `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md` — **fixed**
- other: the plan quoted two byte counts that no longer held — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — **fixed**
- convention: the installer's `re-reads-off` example said to uncomment it alone, which would put it under the wrong table — `framework/bootstrap/ductus.md` — **fixed**
- convention: the slash-command docs and the changelog said every fix round is re-read, though an exempt round is not — `docs/slash-commands.md` — **fixed**
- other: `brief-re-read`'s tool description said nothing is written under `re-reads-off`, though the rounds directory and the snapshot's objects are — `runtime/src/mcp/server.rs` — **fixed**
- other: `compute-review-scope`'s tool description left out the `HEAD` and empty diff bases — `runtime/src/mcp/server.rs` — **fixed**
- convention: the brief and `FailedEdit.occurrences` said `old` is matched as the round left the file, not after the earlier edits to it — `runtime/src/primitives/brief_re_read.rs` — **fixed**
- other: the brief did not say the result file's path is from the project root — `runtime/src/primitives/brief_re_read.rs` — **fixed**
- convention: `review_command.rs`'s module doc ended the procedure at `write-review` — `runtime/tests/review_command.rs` — **fixed**
- bug: `check-prescriptions` returned a snapshot on `differs`, which is no valid next base — `runtime/src/primitives/check_prescriptions.rs` — **fixed**
- other: the budget-stop test had no diff-checked round after the last re-read with findings, and `round_list`'s plural form was untested — `runtime/src/primitives/write_review.rs` — **fixed**
- convention: no subdirectory-project test reached `brief-re-read` or `check-prescriptions`, so `compare`'s `to_git` was unexercised — `runtime/src/primitives/check_prescriptions.rs` — **fixed**
- convention: the base resolution was copied in three primitives and the unprescribed-finding predicate in two — `runtime/src/primitives/fix_rounds.rs` — **fixed**
- other: AC2 and its Behavior bullet promised exemption for `analysis.md` and non-markdown files that `classify` re-reads — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **fixed**
- convention: `bounded_lists::place`'s `InvalidPath` and `Io` branches had no test — `runtime/src/primitives/bounded_lists.rs` — **fixed**
- convention: `fixture_tests` left out `a_pure_rename_reads_alike_in_both_halves`, which builds its own repository — `runtime/tests/mechanical_sweep_parity.rs` — **fixed**
- convention: 066's spec and scenario and four runtime doc comments called the gating-finding check the gate's last — `specs/066-audit-records-and-the-completion-gate/spec.md` — **fixed**
- bug: a run's own `review.md` write and its step-10 deferral writes land in its first fix round — `runtime/src/primitives/brief_re_read.rs` — **deferred** to `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md` task 11
- bug: a Summary the caller supplies drops the loop's account, so AC2's naming of exempt rounds rests on the host — `runtime/src/primitives/write_review.rs` — **deferred** to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 17
- design: `read-re-read` counts a MUST or SHOULD on a path outside the round as a finding, so AC4 rests on the re-read's own labelling — `runtime/src/primitives/read_re_read.rs` — **deferred** to `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md` task 12
- perf: a NOTE's prescribed edit that the operator declines makes `check-prescriptions` re-read the round whole — `runtime/src/primitives/check_prescriptions.rs` — **deferred** to `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md` task 13
- convention: 058's spec body called the gating-finding check the gate's last — `specs/058-findings-route-at-discovery/spec.md` — **deferred** to `specs/058-findings-route-at-discovery/spec.md`
- convention: `the_review_record_remains_a_subject`'s doc comment lost its first line in 066 task 9's change — `runtime/src/primitives/analyze_subjects.rs` — **deferred** to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 18
- other: `every_other_spec_edit_still_moves_its_digest` ticks both checkboxes in one edit, so normalizing only the fenced one still passes it — `runtime/src/primitives/analyze_subjects.rs` — **deferred** to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 18
- convention: 058's scenario `findings-cost-in-proportion-to-risk` called the gating-finding check the gate's last — `specs/058-findings-route-at-discovery/scenarios/findings-cost-in-proportion-to-risk.md` — **fixed**
- convention: the 058 mutation list's docstring called the gating-finding check the gate's last — `scripts/mutation/findings-cost-in-proportion-to-risk.py` — **fixed**
- convention: the slash-command docs and the changelog named a narrower exempt round than AC2 does — `runtime/CHANGELOG.md` — **fixed**
- other: the loop's briefs and checks ran on the binary built before this run's runtime fixes — `runtime/src/primitives/brief_re_read.rs` — **waived**: By the owner's 2026-11-04 decision this review's runtime calls run on the committed 65ec2d7f binary; the round's runtime changes are verified by their tests and mutations in clones, and reach the binary when the commit's rebuild runs.
- bug: 066's data-model example lists two `modified-since` paths with a count of 1 — `specs/066-audit-records-and-the-completion-gate/data-model.md` — **deferred** to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 19
- convention: `compute-review-scope`'s file-form test asserts `deleted` on a fixture that deletes nothing, so three mutations survive — `runtime/src/primitives/compute_review_scope.rs` — **deferred** to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 19
- convention: `compute-review-scope`'s module doc still defines `scope` as the plain union — `runtime/src/primitives/compute_review_scope.rs` — **deferred** to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 19
- convention: 066's `review-scope-union` edge case contradicts its Behavior paragraph on deleted paths — `specs/066-audit-records-and-the-completion-gate/scenarios/review-scope-union.md` — **deferred** to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 19
- convention: the command's Inputs Scope bullet contradicts step 1's deleted-file sentence — `framework/commands/review.md` — **deferred** to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 19
- convention: 066's spec and the implement reference still describe the review scope as a plain union — `specs/066-audit-records-and-the-completion-gate/spec.md` — **deferred** to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 19
- bug: a window holding only deletions under an empty plan reaches the nothing-to-review-yet branch — `framework/commands/review.md` — **deferred** to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 19

## Skipped passes

*None.*

## Unexamined governance

*None.*
