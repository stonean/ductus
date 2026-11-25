---
spec: 084-reopen-for-owed-code-review-every-code-change
last-run: 2026-11-25T22:55:14Z
reviewed-against: 8738f690665b6e8cbcb9d2834f94237b78c14e11
diff-base: 157f64fbd9ed5be862962754a53e4e142e0409b1
must-violations: 0
should-violations: 0
low-confidence: 1
examined: 133
scope: 250
unread:
  - .claude/commands/ductus/amend.md
  - .claude/commands/ductus/analyze.md
  - .claude/commands/ductus/configure.md
  - .claude/commands/ductus/consolidate.md
  - .claude/commands/ductus/fold.md
  - .claude/commands/ductus/groom.md
  - .claude/commands/ductus/help.md
  - .claude/commands/ductus/prune.md
  - .claude/commands/ductus/review.md
  - .claude/commands/ductus/status.md
  - .ductus/reference/analyze.md
  - .ductus/reference/audit.md
  - .ductus/reference/fold.md
  - .ductus/reference/groom.md
  - .ductus/reference/implement.md
  - .ductus/reference/prune.md
  - .ductus/reference/review.md
  - .pi/prompts/ductus-amend.md
  - .pi/prompts/ductus-analyze.md
  - .pi/prompts/ductus-consolidate.md
  - .pi/prompts/ductus-fold.md
  - .pi/prompts/ductus-groom.md
  - .pi/prompts/ductus-help.md
  - .pi/prompts/ductus-prune.md
  - .pi/prompts/ductus-review.md
  - .pi/prompts/ductus-status.md
  - 011-*
  - 013-*
  - 014-*
  - 016-*
  - 020-*
  - 022-*
  - 023-*
  - 026-*
  - 030-*
  - 035-*
  - 041-*
  - 046-*
  - 047-*
  - 049-*
  - 050-*
  - 051-*
  - 054-*
  - 057-*
  - 063-*
  - 066-*
  - 067-*
  - 073-*
  - 079-*
  - 080-*
  - 081-*
  - 083-*
  - AGENTS.md
  - analyze.md
  - analyze_subjects.rs
  - brief_re_read.rs
  - check_review_gate.rs
  - decisions.rs
  - framework/bootstrap/ductus-procedure.md
  - framework/bootstrap/ductus.md
  - framework/bootstrap/govern.md
  - framework/constitution.md
  - framework/reference/analyze.md
  - framework/reference/review.md
  - pipeline_view.rs
  - process_decisions.rs
  - process_waivers.rs
  - prune_plan.rs
  - relocate_audit_records.rs
  - review.md
  - runtime/CHANGELOG.md
  - runtime/src/interpreter/mod.rs
  - runtime/src/main.rs
  - runtime/src/mcp/server.rs
  - runtime/src/primitives/analyze_subjects.rs
  - runtime/src/primitives/append_question.rs
  - runtime/src/primitives/brief_re_read.rs
  - runtime/src/primitives/check_artifact_size.rs
  - runtime/src/primitives/check_artifacts.rs
  - runtime/src/primitives/check_review_gate.rs
  - runtime/src/primitives/check_stuck.rs
  - runtime/src/primitives/commit_attribution.rs
  - runtime/src/primitives/compute_review_scope.rs
  - runtime/src/primitives/dashboard.rs
  - runtime/src/primitives/decisions.rs
  - runtime/src/primitives/derive_references.rs
  - runtime/src/primitives/fix_rounds.rs
  - runtime/src/primitives/measure_reopenings.rs
  - runtime/src/primitives/merge_permissions.rs
  - runtime/src/primitives/mod.rs
  - runtime/src/primitives/pipeline_view.rs
  - runtime/src/primitives/process_decisions.rs
  - runtime/src/primitives/process_waivers.rs
  - runtime/src/primitives/prune_plan.rs
  - runtime/src/primitives/relocate_audit_records.rs
  - runtime/src/primitives/resolve_constitutions.rs
  - runtime/src/primitives/write_analysis.rs
  - runtime/src/primitives/write_review.rs
  - runtime/src/primitives/write_review/
  - runtime/src/primitives/write_review/departure_waivers.rs
  - runtime/src/schema/primitives.rs
  - runtime/tests/
  - runtime/tests/mcp.rs
  - runtime/tests/walker.rs
  - scope-attribution.py
  - scripts/audit/README.md
  - scripts/mutation/scope-attribution.py
  - scripts/tests/test-audit-path-containment.sh
  - specs/008-*
  - specs/018-*
  - specs/026-framework-self-audit/scenarios/family-27-done-spec-unchecked-criteria.md
  - specs/026-framework-self-audit/spec.md
  - specs/048-govern-acquired-runtime/spec.md
  - specs/081-reviews-and-checks-follow-what-changed/plan.md
  - specs/082-rule-assessments-converge-across-runs/plan.md
  - specs/083-commits-are-attributed-to-specs-without-a-subject-convention/plan.md
  - write_analysis.rs
excluded: 102
contract-breaks: 0
skipped-passes: []
fix-rounds: 2
re-reads: 2
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 7d45e955836d4a240ff8ce6c92c49af33f77428c34950dc8fb4232e92f9b13b2
criteria-judged: 23
criteria-met: 23
criteria-verdicts:
  - label: AC1
    digest: fd86aed91f61a8973a1d90ca6b7bb59c80306766b9138a4c59567b4315188f6d
  - label: AC2
    digest: ce1d0dc7699fad02962337e2cad8c3810de4663de5629e9c87128175abfcbc6e
  - label: AC3
    digest: 6d91a28115a4f57c664da545ebf96aa1dabe18df2ab54766be296b866963e416
  - label: AC4
    digest: 493e30d6130a2ae56e2f3720a53938699a91126e42e1cd80e239586a1904ae71
  - label: AC5
    digest: 3f4de126fcb8a464b14a22f808fac97e734f53051d1899a78fcc2eeab9d073df
  - label: AC6
    digest: 0419595a1023f3aede7091ba720dbc6ed0fbe68d4567a2cc667a1cff88d021f0
  - label: AC14
    digest: 28ae177c20239ba55d9439b876dde890878f46daa8e391382c47ecb09ee02844
  - label: AC7
    digest: df4eeb41d531f3639d9f74f2a4512a96fb6f810889194bb2a608f87c4af23f1a
  - label: AC8
    digest: c3406bf611c2b0f8b211e4f8aab1d135dfaaa0d4da812fa9450419cf5fa941e3
  - label: AC9
    digest: 25c2f246b0a2b085325604e503bd98e47a891d9b21c4f2f83475bbcb58b07ee7
  - label: AC15
    digest: a169366f6b370723c9ff32b8528a6eb5ca01a8b7f2df55ad5c247814795af263
  - label: AC16
    digest: 58e168bb7fe676632485e452e0c864c484ee4d0b229553fdeb1da987f280088c
  - label: AC17
    digest: b90acd4eb438e69c36928c1665500662487ac1536d7bca471a35ebbc48cbd17c
  - label: AC10
    digest: 93627e8165c0b53a73a02f01069e906d2fba836ea967097f240763fae92b4cc3
  - label: AC11
    digest: f2107ccd8f9543c5676c37bc131383f11541190a416c19a185c673444b079609
  - label: AC12
    digest: 9c56c7e4e48a813945b024e93d28866fd6d85071a0432298b2625e63de40aa78
  - label: AC18
    digest: 887304e4b683c27660ba10f294a993a55423bd3b3d0573b52c8a10eae5b868c1
  - label: AC13
    digest: 5f185f7ab2b64c5ac8dd8a2e1cc3163a784e5d4f85632e2314514a6593595536
  - label: AC19
    digest: b239bb3b17baf637bfdb27fbaab034691e97f82eaf32dde5ca199ed9cfac9a4d
  - label: AC20
    digest: 619624285f8a9177cf91d551ae8afecd8e10a044e9f4f79b00c4da800537faa5
  - label: AC21
    digest: 2ecbc95047bac2fef559fdd86cf528f16c4fcb6671f1fae36e6276fd7762f1d1
  - label: AC22
    digest: 688f59da56f8130ee6d9cc3081adb90fd89106776747278adcf6ceecff744dac
  - label: AC23
    digest: 7100afe55833fd28679814fac2cd68884253a7acae98a571946c47245dc58c04
blocking: false
dispositions:
  fixed: 33
  deferred: 0
  waived: 17
  undispositioned: 0
decisions:
  - key: "convention: `framework/commands/fold.md` step 10 resets an upstream spec already `in-progress` that the fold did not reopen — `framework/commands/fold.md`"
    outcome: waived
    reason: AC19 governs a `done` upstream spec; resetting an `in-progress` one's review on a fold is the behavior the plan keeps from before 084
    decided-at: 2026-11-25T22:55:14Z
    decided-by: rule
  - key: "other: a groom spec edit adding an open question to a `done` spec leaves it `done` — `framework/commands/groom.md`"
    outcome: waived
    reason: "not 084's change: §spec-lifecycle's edge to `draft` is `/amend`'s, and `/ductus:analyze` reports a `done` spec's open question"
    decided-at: 2026-11-25T22:55:14Z
    decided-by: rule
  - key: "bug: a `done` spec with no review recorded loses the code committed while `done` from its window once reopened — `runtime/src/primitives/compute_review_scope.rs`"
    outcome: waived
    reason: "`framework/reference/review.md` §Window and coverage names this case and its override, and `check-unreviewed-code` reports that code, holding its push, while the spec is `done`"
    decided-at: 2026-11-25T22:55:14Z
    decided-by: rule
  - key: "convention: AC23's text omits that an uncommitted path stays when a commit of the spec's own changed it — `specs/084-reopen-for-owed-code-review-every-code-change/spec.md`"
    outcome: waived
    reason: AC23 states a necessary condition the code meets, and `COMPUTE_REVIEW_SCOPE` states the own-commit clause; rewording a criterion is not a mechanical fix
    decided-at: 2026-11-25T22:55:14Z
    decided-by: rule
  - key: "other: `project`, the project record's `spec`, is not refused as a feature directory name — `runtime/src/primitives/review_subject.rs`"
    outcome: waived
    reason: §numbering gives every feature directory a number, and `ReviewSubject::resolve` accepting an unnumbered one predates 084; refusing it changes how such directories resolve, not a mechanical fix
    decided-at: 2026-11-25T22:55:14Z
    decided-by: rule
  - key: "other: `check_unreviewed_code::unreviewed_push` repeats `fix_rounds::re_reads_off`'s config-read steps — `runtime/src/primitives/check_unreviewed_code.rs`"
    outcome: waived
    reason: a shared reader would refactor 078's `re_reads_off`, outside 084's change, and is not a mechanical fix
    decided-at: 2026-11-25T22:55:14Z
    decided-by: rule
  - key: "other: `framework/bootstrap/hooks/ductus-pre-push` copies `ductus-pre-commit`'s `probe` and `installed_version` — `framework/bootstrap/hooks/ductus-pre-push`"
    outcome: waived
    reason: sharing them needs a third shipped file and its manifest row, not a mechanical fix
    decided-at: 2026-11-25T22:55:14Z
    decided-by: rule
  - key: "other: no test covers the adopter CI template's step on a subject it cannot examine — `scripts/tests/test-done-spec-owed-work.sh`"
    outcome: waived
    reason: a shallow-clone fixture is new authoring, not a mechanical fix; the step's `sed` was checked against a real shallow report
    decided-at: 2026-11-25T22:55:14Z
    decided-by: rule
  - key: "other: the shipped pre-push hook's too-old-runtime halt, version echo and watchdog have no fixture — `scripts/audit/adopter-shell-behavior.sh`"
    outcome: waived
    reason: new fixtures are authoring, not a mechanical fix; AC8's cases each have one
    decided-at: 2026-11-25T22:55:14Z
    decided-by: rule
  - key: "other: no unit test pins that `measure-reopenings` reads a reopening's start and closing commits for owed work — `runtime/src/primitives/measure_reopenings.rs`"
    outcome: waived
    reason: new fixtures are authoring, not a mechanical fix; the code reads both commits and AC16's fixtures prove both directions
    decided-at: 2026-11-25T22:55:14Z
    decided-by: rule
  - key: "other: the status count's test holds one not-examined subject, so joining several is unproved — `runtime/src/primitives/check_unreviewed_code.rs`"
    outcome: waived
    reason: a new fixture is authoring, not a mechanical fix
    decided-at: 2026-11-25T22:55:14Z
    decided-by: rule
  - key: "perf: `dashboard` walks history twice, once for reopenings and once for unreviewed code — `runtime/src/primitives/dashboard.rs`"
    outcome: waived
    reason: AC9 requires the count on every `/ductus:status` run; measured at 3.8 s, an upper bound, over 5,115 commits
    decided-at: 2026-11-25T22:55:14Z
    decided-by: rule
  - key: "other: `COMPUTE_REVIEW_SCOPE`'s summary in `ductus -h` is cut inside its opening parenthetical — `runtime/src/schema/descriptions.rs`"
    outcome: waived
    reason: the cut predates 084, and restructuring a 6,300-byte opening sentence is not a mechanical fix
    decided-at: 2026-11-25T22:55:14Z
    decided-by: rule
  - key: "other: `framework/reference/analyze.md`'s suggested fix for a dead criterion path unticks or corrects with no owner question — `framework/reference/analyze.md`"
    outcome: waived
    reason: analyze's rule fixes only mechanical observations, so it unticks no criterion itself, and a renamed path's correction is §drift-prevention's mechanical sweep, allowed on a ticked criterion
    decided-at: 2026-11-25T22:55:14Z
    decided-by: rule
  - key: "other: 077's, 042's and 043's plans cite the retired mechanical class — `specs/077-security-cross-rules/plan.md`"
    outcome: waived
    reason: each records how its own sweep was classified under the rule then, and a decision record keeps the retired name (§drift-prevention)
    decided-at: 2026-11-25T22:55:14Z
    decided-by: rule
  - key: "other: `dashboard`'s `unreviewed-code.reason` stays inline whole, so on a shallow checkout with a small `[artifacts] read-size-bytes` the payload passes the bound — `runtime/src/primitives/dashboard.rs`"
    outcome: waived
    reason: "`DASHBOARD` moves the lists it names past the bound and promises no total size, and 084's `data-model.md` specifies `reason` naming each subject; the view, which promises a bound, now cuts the row"
    decided-at: 2026-11-25T22:55:14Z
    decided-by: rule
  - key: "other: `framework/templates/ci/adopter-generators.yml` pins `DUCTUS_VERSION: 0.64.0`, whose runtime serves neither new primitive — `framework/templates/ci/adopter-generators.yml`"
    outcome: waived
    reason: "release-gated: the release commit moves the pin with the version, and the test header says so"
    decided-at: 2026-11-25T22:55:14Z
    decided-by: rule
---

# Review — 084-reopen-for-owed-code-review-every-code-change

## Summary

First review of 084, over the window `157f64fb..8738f690` (`base-source: reopen`), run from a clean worktree of `8738f690`. Fourteen pass parts ran all five passes, each reading all five loaded rule files whole: four parts were sent back to finish their rule files, and one to finish its whole-file reads. Five criteria parts judged all 23 criteria, against the code and the tests that prove them.

Reading depth was the operator's call: the agent's recommendation, adopted by the operator, 2026-11-25. Files up to 40 KB, added files, and files the window rewrote were read whole. The 40 larger modified files were read by their hunks, with the context each needed, so they are listed in `unread`. `examined` is 133 of 250 scope entries. Of the 117 unread:

- 27 are generated copies under `.claude/`, `.pi/` and `.ductus/reference/`, plus `framework/bootstrap/govern.md`. The generators were re-run in the worktree before and after the fixes, and the tree came back in sync. `govern.md` is byte-identical to `ductus.md` by design.
- 28 are plan patterns naming other specs' directories. Their edited files are artifacts `compute-review-scope` excludes; the criteria step read AC11's sites there.
- The others are the hunk-read files; `runtime/tests/` and `AGENTS.md`, read in part; and six other specs' files the fixes changed, read at their changed paragraphs.

`compute-review-scope` counts 251 entries once this record is on disk, the record itself among them; the record leaves its own `review.md` out of its 250. Scope entries are not files. Two are directories, and 51 are bare plan names, each counted read when every path it denotes was read whole. `runtime/src/primitives/classify_spec_edit.rs` is deleted in the window: P2 and C5 found no caller, registration, CLI subcommand or MCP tool left.

Three findings were fixed in the run:

- `BE-INPUT-011`: `unreviewed_code_lines` in `runtime/src/main.rs` now escapes every value it writes to stderr.
- `BE-INPUT-004`: `ReviewSubject::resolve` now contains the project record path. P6 raised this as an observation; the host recorded it under the loaded rule, whose Verification covers committed text and symlinks.
- `CFG-ENV-012`: `unreviewed_push` now warns on an unknown `[review]` key. It was raised from low confidence once read.

One finding was dropped. `QUAL-CLAIM-001` on `check_at` did not hold, because every status is examined; the part that raised it agrees. One low-confidence `QUAL-CLAIM-001` stands.

Decisions of 2026-11-25. Each is the agent's recommendation, adopted by the operator, unless marked:

- **AC11:** correct 048's spec and 081's, 082's and 083's plans, which still stated the old reopen rule. Then C4 re-judged AC11 met.
- **AC20:** correct its text, twice, until it names only what the migration writes.
- **026's AC20 and its Family 27 scenario:** they now name a finding's file and label, since a line number drifts. This was the operator's own decision. 026 is in `cross-spec-impact:` with a signpost back.
- **The `Unreviewed code:` row:** fix its size overrun, now the view's last cut.
- **The fold commit count:** fix it in the code.

Each fix has a test that failed before it, apart from the `runtime/tests/mcp.rs` ETXTBSY helper, whose race no single run can show.

2 fix round(s): 2 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

1 of 78 commits in the window no spec claimed.

23 of 23 acceptance criteria judged met.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: QUAL-CLAIM-001 — A version of the spec the walk could not read is recorded as held-owed-work false, so it counts toward never-owed-count

- **File**: `runtime/src/primitives/measure_reopenings.rs`
- **Site**: History::owes_work
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish *examined the subject and found nothing* from *could not examine the subject*.
- **Finding**: owes_work and blob_holds return false for a tasks.md or spec.md that is not UTF-8 or whose spec.md frontmatter does not split, so a reopening whose readable versions never owed work is counted never owed and nothing in the result says a version went unread; the MEASURE_REOPENINGS description and a unit test state the choice, but a caller reading never-owed-count cannot tell an examined clean reopening from one with an unread version, unlike decisions_held, which counts records-unread.
- **Auto-fixable**: no
- **Suggested fix**: Count the reopenings that had an unreadable version (as records-unread does for decisions) beside never-owed-count, or exclude them from it; or record in the data model why a count of the bar's defect may take them.

## Waived findings

*None.*

## Deferred findings

*None.*

## Acceptance criteria

- AC1 — met: `framework/constitution.md` §spec-lifecycle holds exactly two back-edge bullets, `Backward via new questions` (to `draft`) and `Backward via owed code` (`done` to `in-progress` exactly while `tasks.md` holds an unchecked box or `spec.md` an unticked criterion, never an edit's diff, `/review` writing no other status than on an untick or a deferral to its own spec's work), with no correction or mechanical class left; checked by reading the section and grepping `correction`, `mechanical` and `Three back` in the file.
- AC2 — met: `framework/commands/amend.md` steps 4 to 6 and its scenario and criterion routes, `groom.md` steps 6 and 7, `fold.md` step 9, `prune.md` step 9 and `review.md` steps 7.3 and 10 (and `analyze.md` step 19, `reference/implement.md` disposition task) each reopen through `check-owed-work` (`runtime/src/primitives/check_owed_work.rs`, over `owed_work.rs`), `analyze --fix` through `check-artifacts`' review-state drift that reads the same `owed_work` helper; test `check_owed_work::tests::only_an_added_criterion_or_an_appended_task_makes_a_done_spec_owe_work` passes and failed in a clone when `OwedWork::owes` ignored criteria (assertion `criterion.owes`) and when it returned true (the no-edit assertion).
- AC3 — met: `framework/commands/review.md` step 7.3 (gate-confirm per not-met ticked criterion on a spec `done` at start, corrects text and keeps the tick when code is right, decides and names it for the commit under an explicit instruction), `amend.md` criterion route step 3 and `groom.md` step 2 `spec` route say the same; the exec walker's `mark_each_verdict` in `runtime/src/interpreter/mod.rs` leaves such a criterion ticked, proved by `interpreter::tests::the_review_walk_leaves_a_done_specs_ticked_criterion_judged_not_met`, which passes.
- AC4 — met: `check_unreviewed_code::compute`/`check_at`/`DoneSubject::code` in `runtime/src/primitives/check_unreviewed_code.rs` report a done spec's and unclaimed code, drop paths an in-progress spec's share holds, and say `UnreviewedState::NotExamined` apart from `None`; proved by `a_done_specs_code_and_unclaimed_code_are_reported_and_an_open_specs_is_not`, `a_path_an_in_progress_spec_also_claims_passes`, `a_shallow_history_names_each_subject_it_cannot_place`, `a_tree_holding_no_project_record_is_not_examined_for_unclaimed_code` (all 28 `check_unreviewed_code::` tests pass).
- AC5 — met: `framework/commands/review.md` step 1 admits a `done` spec and runs steps 3 to 7.3 whole, step 7.3 asks the owner before an untick and runs `check-owed-work` then `set-status` only after one, step 10 puts every MUST or SHOULD violation on a spec `done` at start to the operator one at a time and reopens through `check-owed-work` after a deferral to its own task, `write-review` writes `specs/NNN/review.md`; the exec-path half is proved by `interpreter::tests::the_review_walk_leaves_a_done_specs_ticked_criterion_judged_not_met`, which passes; no test drives the host-side command text, which was read, not run.
- AC6 — met: `write_review::run` with no feature writes `{specs-root}/review.md` via `ReviewSubject::Project` (whole render, no criteria/digest, `refuse_what_only_a_spec_has`), and `SpecShare::split` gives a spec nothing of an unclaimed commit (`project_share` is its inverse); proved by `write_review::project_record::a_project_review_records_the_unclaimed_commit_in_the_spec_roots_record`, `an_unclaimed_commit_leaves_every_window` in `runtime/src/primitives/compute_review_scope.rs` and `the_projects_share_is_the_commits_no_spec_claims` in `runtime/src/primitives/commit_attribution.rs` (10 project_record tests and 159 compute_review_scope tests pass).
- AC14 — met: `fix_rounds::round_paths` takes `ReviewSubject::of(subject.feature).record_rel(specs_root)` as the record it leaves out, so the project's `specs/review.md` is never a round path; proved by `a_project_reviews_rounds_leave_out_its_record_and_what_a_spec_claims` in `runtime/tests/fix_rounds.rs`, which writes only the record and asserts `RoundKind::Empty`, then asserts the unclaimed and spec-claimed landings divide as the inverted share says (4 tests pass).
- AC7 — met: `compute_review_scope::base_at`/`unreviewed_base` start the window at `last_review_commit` or, for an unreviewed done spec, `find_done_commit`; proved by `a_reopen_keeps_the_code_committed_while_done_in_the_window` (code while done, reopen, scope holds it, base-source last-review) and `an_unreviewed_done_spec_starts_at_the_commit_that_set_it_done` in `runtime/src/primitives/compute_review_scope.rs`, and the check's `an_unreviewed_done_spec_reports_its_code_after_the_commit_that_set_it_done` in `runtime/src/primitives/check_unreviewed_code.rs` (reports `src/after.rs` only, window-start the done commit).
- AC8 — met: `framework/bootstrap/hooks/ductus-pre-push` (loop `|| rc=$?`, missing-runtime halt, deleted-ref `continue`) with `framework/bootstrap/hooks/pre-push` is installed by `framework/bootstrap/ductus.md` §Shared Files rows and `ductus-procedure.md` §Hook Installation; `check_pre_push` in `scripts/audit/adopter-shell-behavior.sh` proves block, warn, absent key, bad value, unparsable config, deleted ref, shallow clone under both keys and no runtime, passes at 8738f690, and three hook mutants (exit status swallowed, runtime halt removed, deleted ref checked) each make it fail.
- AC9 — met: CI half: `framework/templates/ci/adopter-generators.yml` has `fetch-depth: 0` and the step "No code is left unreviewed" (exit status of `ductus check-unreviewed-code`, `::warning::` per path and per subject not examined, checked against a real report), proven by `scripts/tests/test-done-spec-owed-work.sh` G and H (pass; mutants swallowing the status or the annotation fail it); status half: `unreviewed_line` in `runtime/src/primitives/pipeline_view.rs` renders `Unreviewed code:` in each state on every run, proven by `the_unreviewed_code_block_is_the_check_at_head_in_each_state` in `runtime/src/primitives/dashboard.rs` (passes) and a read-only `render-status` run at 8738f690 showing `Unreviewed code: none`.
- AC15 — met: `framework/templates/ci/adopter-generators.yml` step "Done specs owe no work" and `scripts/audit/done-spec-criteria.sh` both read `check-owed-work --all` and name each unchecked box and unticked criterion; `scripts/tests/test-done-spec-owed-work.sh` cases A to F pass, and mutants dropping the box loop, the criterion loop or the exit status in the template or the script each fail the matching case.
- AC16 — met: `OpenPass::observe`/`History::owes_work` and `Measurement::counts` in `runtime/src/primitives/measure_reopenings.rs` set `held-owed-work` per commit in each closed reopening with a start (counted or `not-counted`), leave it `None` for `no-in-progress-commit`, and count `never_owed_count`; `never_owed_counts_each_closed_reopening_that_held_no_work`, `a_box_open_at_a_middle_commit_is_owed_work_though_ticked_by_the_end`, `a_criterion_unticked_at_a_middle_commit_is_owed_work_though_ticked_by_the_end` and `a_closing_with_no_pass_is_not_counted` pass (58 of 58), and a mutant turning `held_owed ||` into `&&` fails three of them.
- AC17 — met: `check_unreviewed_code::compute` takes `at` and `since` and walks `simplify_first_parent` counting `commits-walked`/`commits-reporting`/`commits-not-examined`; proved by `since_counts_the_first_parent_commits_reporting_code` (a merge's branch commits left out, walked 3 reporting 2, tip-to-tip walks 0, a record-less commit counts apart) and `since_counts_on_the_cli` in `runtime/tests/check_unreviewed_code_cli.rs`.
- AC10 — met: `framework/constitution.md` §bug-handling Bug Decision Tree item 4 reads "fix the code, adding a scenario only when the requirement needs more precision; a fix to code an existing criterion describes is reviewed without one."
- AC11 — met: The four notes in 079 (AC5, AC15) and 081 (AC10, AC13) hold and the corrected `specs/048-govern-acquired-runtime/spec.md` paragraph and the 081, 082 and 083 plan bullets now say the rule then in force counted it, spec 084 replaced it; a re-run of the census over 1,397 files found no remaining statement that an edit's diff reopens a `done` spec, but files were read by hit line and suspect paragraph, never whole, and `review.md`, `analysis.md`, `tasks.md`, `research.md` and generic runtime identifiers went unsearched.
- AC12 — met: Worktree binary: `ductus --help` and `ductus classify-spec-edit --help` report an unrecognized subcommand, MCP `tools/list` holds no classify-spec-edit (it lists `check-unreviewed-code`); `framework/runtime-tools.txt` drops it; `RETIRED_ALLOW` in `runtime/src/primitives/merge_permissions.rs` and step 4 of `framework/bootstrap/configure/claude.md` carry `mcp__ductus__classify-spec-edit`, held equal by `the_retired_set_is_configure_claudes_step_4` and `the_sets_hold_the_counts_task_43_left` (both pass, 51 merge_permissions tests pass); `runtime/src/primitives/mechanical_sweep.rs` module doc names the two readers, `analyze_subjects::freshness_of` and `review_freshness`; no source or test reads it. No test names the classify entry alone: the generic canonical revoke is proved by `a_canonical_call_retires_names_and_keeps_the_adopters_entries` over another retired entry.
- AC18 — met: `runtime/src/primitives/check_artifact_size.rs` `oversized_artifact` and `fix_plan` name no per-fix reopen (`on_done` and `OnDone` are gone from `runtime/src` and `runtime/tests`) and append `DONE_RULE` once, for a `done` spec only, stating a fix reopens it only if it leaves an unchecked box or an unticked criterion; `a_done_specs_warning_states_the_owed_work_rule_once` passes, but see the observation that it does not pin the sentence's wording.
- AC13 — met: Of the 78 commits in the window, every behavior change by a 084-named implementation commit lands in the runtime, `framework/commands/`, `framework/reference/`, `framework/constitution.md`, `framework/bootstrap/`, `framework/migrations/` or `framework/templates/` (task 11 hook in `framework/bootstrap/hooks/`, task 12 seed migration, task 13 `framework/templates/ci/adopter-generators.yml`, task 14 status count, tasks 1 to 10, 24 to 27, 29, 32, 33 in `runtime/src` plus references); the only changes outside those are this repository's dogfood copies and records (`.githooks/pre-push`, `.ductus/config.toml`, `scripts/audit/`, `.github/`, `AGENTS.md` rewordings), each with a shipped counterpart, and `AGENTS.md` changes in 6069f56a, 45b6020d and 1b3bb4ac are rewordings pointing at the constitution, adding no behavior.
- AC19 — met: `framework/commands/fold.md` step 9 sets `from: done` to `in-progress` only when `check-owed-work` reports `owes: true`, step 10 invalidates the review only when the upstream is `in-progress` after step 9 (the `done`-stays case is stated in step 14 and `framework/reference/fold.md` Reopening a `done` upstream spec); the report test is `code_a_staging_spec_folded_below_done_brought_is_reported_under_its_upstream` in `runtime/src/primitives/check_unreviewed_code.rs` (passes; below-done fold reports `src/t.rs` under `002-b`, from-`done` fold reports nothing), with the compute-review-scope fold tests (18 `a_fold_` tests pass); `specs/051-branch-scoped-spec-numbering/spec.md` AC24 and AC25 each carry "Superseded in part (spec 084)."
- AC20 — met: `framework/migrations/project-review-record.md` step 2 runs `check-unreviewed-code` once, reports each `done` spec with its path count and `[review] unreviewed-push` with `"block"` and `"warn"`, and says "Write no configuration"; the migration's two primitives left a scratch adopter's `.ductus/config.toml` byte-identical, and the only write the runner adds is `[migrations].last_applied` (`framework/bootstrap/ductus-procedure.md` step 7, item 3), matching the recorded run in `b31cf5b2` (the config differed also by `[source] ref`, which `--ref=main` records, not the migration).
- AC21 — met: `framework/reference/analyze.md` Scenario open questions says `--fix` reverts nothing and the finding stays blocking with its two exits, `framework/commands/analyze.md` step 22 never reverts it, `check_scenario_open_questions` in `runtime/src/primitives/check_artifacts.rs` stays blocking at `done` (`scenario_questions_are_blocking_on_a_done_spec` passes) and `scenario_question_block` in `runtime/src/primitives/check_review_gate.rs` still blocks (`blocks_on_unresolved_scenario_questions_naming_the_scenario` passes); deferrals reopen only on `check-owed-work`'s report in `framework/commands/analyze.md` step 19, `framework/reference/review.md` Deferrals (Released) and `framework/reference/implement.md` Working a disposition task; `specs/046-scenario-open-question-visibility/spec.md` AC23 and AC30 each carry "Superseded in part (spec 084)."
- AC22 — met: `check_task_consistency` in `runtime/src/primitives/check_artifacts.rs` blocks a repeated `TaskNumber` and reads no order; `a_repeated_task_number_yields_blocking_finding`, `task_numbers_out_of_order_yield_no_finding` and `a_repeated_suffixed_task_number_is_reported` pass (154 of 154 `check_artifacts` tests), and `check-artifacts` on 084 reports no `task-consistency` finding.
- AC23 — met: `held_elsewhere` and `split_changes` in `runtime/src/primitives/compute_review_scope.rs` move an uncommitted path to `excluded` only when a planned or in-progress spec's plan names it (name or glob) and this spec's plan and own commits do not; proved in both directions by `an_uncommitted_path_another_open_plan_holds_leaves_the_scope_and_owes_no_review` and `an_uncommitted_path_no_other_open_plan_holds_alone_joins_and_owes_a_review` (plus `a_planned_specs_plan_holds_an_uncommitted_path_too`); the own-commit stay condition is extra to the text, see the observation.

## Observations

- bug: `CHECK_OWED_WORK` and `OwedWorkOfSpec::unchecked_boxes` said `unchecked-boxes` come in line order, but `owed_work` returns them sorted by the line's text; the description and doc now say sorted — `runtime/src/schema/descriptions.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: `RENDER_STATUS` and `pipeline_view`'s `summary` doc listed the summary without the `Unreviewed code:` line 084 renders on every run; both now name it — `runtime/src/schema/descriptions.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: `BRIEF_RE_READ` and `CHECK_PRESCRIPTIONS` said a feature round leaves out only what another spec's commits changed, though since 084 a commit no spec claims leaves it too; both now say so — `runtime/src/schema/descriptions.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: `WRITE_REVIEW` and `write_review::resolve_scope`'s doc said an empty diff-base comes from a spec never in-progress, though one added as `done` now starts at `reached-done`; both now say never in-progress or done — `runtime/src/schema/descriptions.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: in the not-examined state the `Unreviewed code:` row named every subject on one row the cut ladder never shortened, so `render-status` could exceed `[artifacts] read-size-bytes`; the ladder now collapses it last (operator's decision, 2026-11-25) — `runtime/src/primitives/pipeline_view.rs` (breaks the spec's contract) — **fixed**
- bug: `check-unreviewed-code` counted `commits` only from the spec's own commits, so a `done` spec whose paths a staging spec folded below `done` brought read 0 commits; the fold's commits now count (operator's decision, 2026-11-25) — `runtime/src/primitives/check_unreviewed_code.rs` (breaks the spec's contract) — **fixed**
- bug: `framework/commands/fold.md` step 5, `framework/reference/fold.md` and 084's Behavior said a body-edit fold leaves owed work only when it adds a criterion, though step 8 appends a task per carried scenario on either route; all three now say so — `framework/commands/fold.md` — **fixed** (decided by the rule)
- bug: the project review's step 1 tested an absent `unclaimed` before saying the file form moves it to `lists-file`, so a large window read as nothing owed; the file-form read now comes first — `framework/commands/review.md` — **fixed** (decided by the rule)
- convention: step 10's sentence on a task of the spec's own lacked its relative pronoun and did not parse; it now reads with `which` — `framework/commands/review.md` — **fixed** (decided by the rule)
- convention: Scope Boundaries' step 10 clause stated the reopen of a `done` spec a deferral goes to with no condition, which fix round 1's re-read noted; it now carries the `check-owed-work` condition — `framework/commands/review.md` — **fixed** (decided by the rule)
- convention: Scope Boundaries stated `--defer`'s reopen of a `done` spec with no condition; it now carries step 10's `check-owed-work` condition — `framework/commands/review.md` — **fixed** (decided by the rule)
- convention: `framework/reference/review.md` §Window and coverage said another open spec's plan, where AC23 names a spec at `planned` or `in-progress`; it now says so — `framework/reference/review.md` — **fixed** (decided by the rule)
- convention: `PRUNE_PLAN` said `reopen-required` reports whether the tree takes the back-edge; it now calls it a prediction and names `check-owed-work` as deciding the reopen — `runtime/src/schema/descriptions.rs` — **fixed** (decided by the rule)
- convention: `APPEND_QUESTION` said a `done` spec reopens via the scenario route and `INVALIDATE_REVIEW` did not say fold calls it only on an upstream spec left `in-progress`; both now state 084's rule — `runtime/src/schema/descriptions.rs` — **fixed** (decided by the rule)
- convention: `CHECK_PRESCRIPTIONS` named no list-file directory for the project round, where its four siblings name theirs; it now names `.ductus/results/check-prescriptions/project/` — `runtime/src/schema/descriptions.rs` — **fixed** (decided by the rule)
- convention: `append_question.rs`' module doc and a test comment said a `done` spec reopens via the scenario route; both now state the owed-work rule — `runtime/src/primitives/append_question.rs` — **fixed** (decided by the rule)
- convention: `criterion_verdicts::text`'s doc named an edit's classification (079 AC15), which `classify-spec-edit`'s retirement removed; the clause is gone — `runtime/src/primitives/criterion_verdicts.rs` — **fixed** (decided by the rule)
- other: `framework/reference/audit.md` item 27 still described Family 27 as parsing criteria itself; it now describes `check-owed-work --all` and the unchecked box — `framework/reference/audit.md` — **fixed** (decided by the rule)
- other: `scripts/audit/run-all.sh` labelled Family 27 done-spec unchecked criteria though it now also reports an unchecked box; it reads done-spec owed work — `scripts/audit/run-all.sh` — **fixed** (decided by the rule)
- convention: `README.md`'s Every bug bullet said a bug in specified behavior becomes a scenario, against §bug-handling's fourth route (AC10); it now says a scenario only where more precision is needed — `README.md` — **fixed** (decided by the rule)
- convention: `README.md`'s `/amend` entry and the `docs/slash-commands.md` heading omitted the criterion route; both name it, the anchor moved with its one referrer — `docs/slash-commands.md` — **fixed** (decided by the rule)
- convention: the `/status` paragraph of `docs/slash-commands.md` described the unreviewed-code figures loosely and omitted the not-examined state; it now names what the line renders — `docs/slash-commands.md` — **fixed** (decided by the rule)
- convention: `framework/commands/amend.md` and two `scripts/audit/cross-doc-consistency.sh` comments said `/amend` owns the back-edges, though since 084 any command whose write leaves work owed reopens; they now say it takes both and owns the edge into draft alone — `framework/commands/amend.md` — **fixed** (decided by the rule)
- convention: a comment in `process_waivers::compute` said the function had found the spec to exist, which it no longer checks; it now says what the code does — `runtime/src/primitives/process_waivers.rs` — **fixed** (decided by the rule)
- convention: the doc comment of `elapsed_is_the_clock_less_the_recorded_start` in `fix_rounds.rs`' tests sat on the next test; it is back on its own — `runtime/src/primitives/fix_rounds.rs` — **fixed** (decided by the rule)
- convention: `scripts/mutation/loop-inputs-screened.py`'s docstring named `validate_feature`, renamed by 084 to `validate_subject` — `scripts/mutation/loop-inputs-screened.py` — **fixed** (decided by the rule)
- convention: `dispatch_blocking`'s doc in `runtime/src/mcp/server.rs` left out `check-unreviewed-code` and the dashboard's unreviewed-code walk; it names both — `runtime/src/mcp/server.rs` — **fixed** (decided by the rule)
- convention: 084's `data-model.md` grouped `unclaimed-path-count` with the list counts as entries, though it counts `unclaimed`'s paths and deleted paths; the row says so — `specs/084-reopen-for-owed-code-review-every-code-change/data-model.md` — **fixed** (decided by the rule)
- other: `a_done_specs_warning_states_the_owed_work_rule_once` compared the warning to `DONE_RULE` and never its words, so reversing the rule passed; it now asserts them — `runtime/src/primitives/check_artifact_size.rs` — **fixed** (decided by the rule)
- bug: `runtime/tests/mcp.rs`' `vendor_markdownlint` and failing-linter stub wrote a script and exec'd it at once, the ETXTBSY race task 21 fixed elsewhere; both now write through a child shell — `runtime/tests/mcp.rs` — **fixed** (decided by the rule)
- other: no test failed if `fetch-depth: 0` left the adopter CI template, since the runner skips `uses` steps; `scripts/tests/test-done-spec-owed-work.sh` case I now reads it — `scripts/tests/test-done-spec-owed-work.sh` — **fixed** (decided by the rule)
- convention: the AC11 sweep left the old reopen rule as current behavior in `specs/048-govern-acquired-runtime/spec.md` and in 081's, 082's and 083's plans; each now states the owed-work rule or records the old one in the past tense (operator's decision, 2026-11-25) — `specs/048-govern-acquired-runtime/spec.md` — **fixed**
- convention: task 13 left Family 27 anchoring a finding to its file, against 026's ticked AC20 and its scenario saying file:line; both now say the file and label, since a line number drifts (operator's decision, 2026-11-25), with 026 in 084's cross-spec-impact — `specs/026-framework-self-audit/spec.md` — **fixed**
- convention: `framework/commands/fold.md` step 10 resets an upstream spec already `in-progress` that the fold did not reopen — `framework/commands/fold.md` — **waived**: AC19 governs a `done` upstream spec; resetting an `in-progress` one's review on a fold is the behavior the plan keeps from before 084 (decided by the rule)
- other: a groom spec edit adding an open question to a `done` spec leaves it `done` — `framework/commands/groom.md` — **waived**: not 084's change: §spec-lifecycle's edge to `draft` is `/amend`'s, and `/ductus:analyze` reports a `done` spec's open question (decided by the rule)
- bug: a `done` spec with no review recorded loses the code committed while `done` from its window once reopened — `runtime/src/primitives/compute_review_scope.rs` — **waived**: `framework/reference/review.md` §Window and coverage names this case and its override, and `check-unreviewed-code` reports that code, holding its push, while the spec is `done` (decided by the rule)
- convention: AC23's text omits that an uncommitted path stays when a commit of the spec's own changed it — `specs/084-reopen-for-owed-code-review-every-code-change/spec.md` — **waived**: AC23 states a necessary condition the code meets, and `COMPUTE_REVIEW_SCOPE` states the own-commit clause; rewording a criterion is not a mechanical fix (decided by the rule)
- other: `project`, the project record's `spec`, is not refused as a feature directory name — `runtime/src/primitives/review_subject.rs` — **waived**: §numbering gives every feature directory a number, and `ReviewSubject::resolve` accepting an unnumbered one predates 084; refusing it changes how such directories resolve, not a mechanical fix (decided by the rule)
- other: `check_unreviewed_code::unreviewed_push` repeats `fix_rounds::re_reads_off`'s config-read steps — `runtime/src/primitives/check_unreviewed_code.rs` — **waived**: a shared reader would refactor 078's `re_reads_off`, outside 084's change, and is not a mechanical fix (decided by the rule)
- other: `framework/bootstrap/hooks/ductus-pre-push` copies `ductus-pre-commit`'s `probe` and `installed_version` — `framework/bootstrap/hooks/ductus-pre-push` — **waived**: sharing them needs a third shipped file and its manifest row, not a mechanical fix (decided by the rule)
- other: no test covers the adopter CI template's step on a subject it cannot examine — `scripts/tests/test-done-spec-owed-work.sh` — **waived**: a shallow-clone fixture is new authoring, not a mechanical fix; the step's `sed` was checked against a real shallow report (decided by the rule)
- other: the shipped pre-push hook's too-old-runtime halt, version echo and watchdog have no fixture — `scripts/audit/adopter-shell-behavior.sh` — **waived**: new fixtures are authoring, not a mechanical fix; AC8's cases each have one (decided by the rule)
- other: no unit test pins that `measure-reopenings` reads a reopening's start and closing commits for owed work — `runtime/src/primitives/measure_reopenings.rs` — **waived**: new fixtures are authoring, not a mechanical fix; the code reads both commits and AC16's fixtures prove both directions (decided by the rule)
- other: the status count's test holds one not-examined subject, so joining several is unproved — `runtime/src/primitives/check_unreviewed_code.rs` — **waived**: a new fixture is authoring, not a mechanical fix (decided by the rule)
- perf: `dashboard` walks history twice, once for reopenings and once for unreviewed code — `runtime/src/primitives/dashboard.rs` — **waived**: AC9 requires the count on every `/ductus:status` run; measured at 3.8 s, an upper bound, over 5,115 commits (decided by the rule)
- other: `COMPUTE_REVIEW_SCOPE`'s summary in `ductus -h` is cut inside its opening parenthetical — `runtime/src/schema/descriptions.rs` — **waived**: the cut predates 084, and restructuring a 6,300-byte opening sentence is not a mechanical fix (decided by the rule)
- other: `framework/reference/analyze.md`'s suggested fix for a dead criterion path unticks or corrects with no owner question — `framework/reference/analyze.md` — **waived**: analyze's rule fixes only mechanical observations, so it unticks no criterion itself, and a renamed path's correction is §drift-prevention's mechanical sweep, allowed on a ticked criterion (decided by the rule)
- other: 077's, 042's and 043's plans cite the retired mechanical class — `specs/077-security-cross-rules/plan.md` — **waived**: each records how its own sweep was classified under the rule then, and a decision record keeps the retired name (§drift-prevention) (decided by the rule)
- other: `dashboard`'s `unreviewed-code.reason` stays inline whole, so on a shallow checkout with a small `[artifacts] read-size-bytes` the payload passes the bound — `runtime/src/primitives/dashboard.rs` — **waived**: `DASHBOARD` moves the lists it names past the bound and promises no total size, and 084's `data-model.md` specifies `reason` naming each subject; the view, which promises a bound, now cuts the row (decided by the rule)
- other: `framework/templates/ci/adopter-generators.yml` pins `DUCTUS_VERSION: 0.64.0`, whose runtime serves neither new primitive — `framework/templates/ci/adopter-generators.yml` — **waived**: release-gated: the release commit moves the pin with the version, and the test header says so (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `scripts/lint-tool-coverage.sh`
- `specs/000-slash-commands/review.md`
- `specs/000-slash-commands/scenarios/criterion-route-after-draft.md`
- `specs/000-slash-commands/scenarios/scenario-without-task-visibility.md`
- `specs/008-security-rules/scenarios/a-statement-carries-one-obligation-keyword.md`
- `specs/008-security-rules/scenarios/rules-are-as-short-as-their-required-content.md`
- `specs/011-brownfield-process/spec.md`
- `specs/012-multi-agent-govern/review.md`
- `specs/012-multi-agent-govern/spec.md`
- `specs/013-text-first-artifacts/scenarios/criterion-identifiers.md`
- `specs/014-reclarify-backedge/plan.md`
- `specs/014-reclarify-backedge/scenarios/reopen-after-informal-edits.md`
- `specs/014-reclarify-backedge/spec.md`
- `specs/016-cross-cutting-rules/spec.md`
- `specs/018-adopter-owned-pre-commit/spec.md`
- `specs/020-code-review/data-model.md`
- `specs/020-code-review/spec.md`
- `specs/021-runtime-boundary/review.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/plan.md`
- `specs/022-deterministic-runtime/scenarios/ask-consolidation.md`
- `specs/022-deterministic-runtime/scenarios/groom-command-acceleration.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/scenarios/scenario-open-question-signal.md`
- `specs/022-deterministic-runtime/scenarios/spec-side-parser-hardening.md`
- `specs/022-deterministic-runtime/scenarios/specify-routes-before-scaffolding.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/023-govern-refinement/review.md`
- `specs/023-govern-refinement/scenarios/living-specs.md`
- `specs/025-rule-opt-out/review.md`
- `specs/026-framework-self-audit/scenarios/family-23-sweep-target-manifest-parity.md`
- `specs/030-cross-service-references/plan.md`
- `specs/030-cross-service-references/spec.md`
- `specs/035-groom-session-target/scenarios/confirmation-names-reopen.md`
- `specs/035-groom-session-target/scenarios/reopen-done-spec-on-scenario.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/plan.md`
- `specs/041-task-pruning/scenarios/a-plan-edit-does-not-reopen-a-done-spec.md`
- `specs/041-task-pruning/scenarios/a-suffixed-task-number-is-a-task.md`
- `specs/041-task-pruning/scenarios/prune-all-walks-every-spec.md`
- `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md`
- `specs/041-task-pruning/scenarios/prune-walks-the-project-files.md`
- `specs/041-task-pruning/spec.md`
- `specs/046-scenario-open-question-visibility/plan.md`
- `specs/046-scenario-open-question-visibility/spec.md`
- `specs/047-analyze-findings-durability/scenarios/analyze-record-freshness.md`
- `specs/047-analyze-findings-durability/spec.md`
- `specs/048-govern-acquired-runtime/review.md`
- `specs/049-rename-govern-to-ductus/plan.md`
- `specs/049-rename-govern-to-ductus/review.md`
- `specs/049-rename-govern-to-ductus/spec.md`
- `specs/050-constitution/plan.md`
- `specs/050-constitution/scenarios/a-canonical-source-is-pointed-at-not-copied.md`
- `specs/050-constitution/scenarios/a-declared-cross-spec-impact-gates-done.md`
- `specs/050-constitution/scenarios/a-primitive-is-probed-in-a-local-clone.md`
- `specs/050-constitution/scenarios/a-retired-filename-leaves-a-decision-record.md`
- `specs/050-constitution/spec.md`
- `specs/051-branch-scoped-spec-numbering/plan.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-pending-fold-never-holds-done.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/fold-target-checked-before-the-rewrite.md`
- `specs/051-branch-scoped-spec-numbering/spec.md`
- `specs/054-remove-supersession/plan.md`
- `specs/054-remove-supersession/spec.md`
- `specs/057-analyze-artifact-and-record-relocation/spec.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/review.md`
- `specs/062-concurrent-session-targets/review.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/plan.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/a-plan-trim-does-not-reopen-a-done-spec.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/project-files-are-measured.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/plan.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-commit-after-the-scope-was-resolved-refuses-the-write.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/criteria-are-verified-in-review.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-scope-includes-uncommitted-work.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-staleness-on-done-specs.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-none-list-item-is-no-open-question.md`
- `specs/067-analyze-and-audit-checks/scenarios/disposition-drift-is-advisory.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/069-accessibility-frontend-rules/review.md`
- `specs/073-decided-rule-departures-are-recorded/plan.md`
- `specs/075-agents-md-holds-what-every-session-needs/plan.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/080-deterministic-runtime-efficiency/plan.md`
- `specs/081-reviews-and-checks-follow-what-changed/review.md`
- `specs/081-reviews-and-checks-follow-what-changed/spec.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/spec.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/plan.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/spec.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/tasks.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/tasks.md`

## Unexamined governance

*None.*
