---
spec: 057-analyze-artifact-and-record-relocation
last-run: 2026-11-19T22:47:57Z
reviewed-against: 2ca97542a952327d12a72140e9e783ba702443c0
diff-base: 5b20aaa487539159c92dd11c148e577488af2838
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 6
scope: 10
unread:
  - runtime/CHANGELOG.md
  - runtime/src/primitives/analyze_subjects.rs
  - runtime/src/primitives/check_artifacts.rs
  - runtime/src/primitives/check_review_gate.rs
excluded: 46
contract-breaks: 0
skipped-passes: []
fix-rounds: 2
re-reads: 2
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 9d6dc21acae6e3e128006a20b2c379d5f9987020d2c21b9e0c885f9fc94dd365
  scenarios/a-merge-carries-each-keys-whole-value.md: c0cc9145d9f13e8deb06d0befe848a21bcd48b3c6d6ad24cba2ae2e648fc4138
criteria-judged: 26
criteria-met: 26
criteria-verdicts:
  - label: AC1
    digest: 449de845ca94e6263cd40c8d8d72f5371c959c0685ce896cf1e0e5ac77a6bb75
  - label: AC2
    digest: 65f62db04a3f0551a62593e327998214612d1f8da39b3b0c06039a411245d5ca
  - label: AC3
    digest: 6946c3ea048bb34a3cfe4a52a1414884179a66653420d47ff2c5c07bcea9ac01
  - label: AC4
    digest: 31d6c3c434aa500dc0fa01bfc49ebf858cbc7cdec90256326491977a374f2daa
  - label: AC5
    digest: e3a88f96637537fae7e87005fd5ac454bf80b9c31b1355528f3864d80ee5cad2
  - label: AC6
    digest: 0563f9a860d5f93adcee3d85ee46c44475cf28b1387642fe8ad05ce19ccb32e1
  - label: AC7
    digest: e96e430c3708e79e2f25becaf5b42dba9aa09d6bc9ce54a735f31e13e4c193d2
  - label: AC8
    digest: f0529f6b0bea6f99c1bc8f7ec230dd00436c2731be9ba9530007f16f454f72b2
  - label: AC9
    digest: 089b0eeae01640476d05a43325a25603acdb7e71536c6d3990f083ea0b73e90d
  - label: AC10
    digest: d3c0da76dfcaef1a6bed754f665ec36bcfbbe21e2c5bfca987a57c1340960a90
  - label: AC11
    digest: a6977b19d268b8e931f035f05f3ea13f0643af19de37f8c74fb524f2f342811a
  - label: AC12
    digest: bf8c28721b5dfdbd9a7957076f76f19ed46984557ce6c68871312b4172a7b5a1
  - label: AC13
    digest: c1ccefa2a8af199840da2320d2214b1710ee5eeb1a6a7d590c3b8b56e9348ae5
  - label: AC14
    digest: eeba2fba6fa97212be24b1c76ca3ec61ce378e344853080fd54da7fdaac0949e
  - label: AC15
    digest: a62195b3fc970b747b6b2036ddb753826d46d6db9ec17c5369f8946b2fb40a2c
  - label: AC16
    digest: b738b53b64de4e5b2c54ade8ecded4920b9c4d247d3ceca681869b99ff1d6f14
  - label: AC17
    digest: 77bb458d7da5889160d15bc6857f88faf5e1941b5752127977a36dca60e20a88
  - label: AC18
    digest: a2048a1b34e777bf73b0235b28d9eaf6a1b080083cc1befb98c8e7e810e69e87
  - label: AC19
    digest: a267f951b302198eae96345d195e07ee86a2fb3ec1d37929a9ac4f7f82adb95e
  - label: AC20
    digest: 90a866420f43768a60e90fa75f631b4bd8758a18c4b0869cea5b21ec6037f490
  - label: AC21
    digest: 840ca0221760387893a5f89f8c33d4140fd3257879ebb800f6934c80a31e1a5c
  - label: AC22
    digest: 2c524fbd88d898cda3b9facbd28a2bd8b54fb368039d2b5265431da55f4b6f27
  - label: AC23
    digest: e33287c641fc019abc67f504cdeb8e58b773c5b17e427576b3b328d1bb5a1ceb
  - label: AC24
    digest: f7415b810cdef186f0d56d9a18499180d3f2f0e0cd5193fb44c2a4a55a27d842
  - label: AC25
    digest: d32547205f39c8f4e2c92f2ba123b1635297539326b5932fe29fbf2874bda6e6
  - label: AC26
    digest: 11c5d160c1b0fb3513590759b8346e96a0bb0022b5fb1ae08305507e7e20e4f4
blocking: false
dispositions:
  fixed: 10
  deferred: 0
  waived: 8
  undispositioned: 0
decisions:
  - key: "bug — a write or lock failure after an earlier artifact was written (the lock wait timing out, or the rename failing) returns a plain Io error that does not name the files already written, though the relocation is then half done; only a changed-since-read refusal becomes relocation-incomplete. Judged against the contract: the data model and task 27 state the half-done report for a refused (changed) file only, the Io error names its own path, and a re-run is idempotent (AC20), so no stated requirement is broken — `runtime/src/primitives/relocate_audit_records.rs:205-214`"
    outcome: waived
    reason: 057's contract (data model Errors table, task 27) states the half-done report for a changed-since-read refusal; an Io or lock-timeout failure after a write has no stated requirement, names its failing path, and the merge is idempotent so a re-run finishes the relocation
    decided-at: 2026-11-16T13:32:11Z
    decided-by: rule
  - key: convention — relocate-audit-records creates a new review.md or analysis.md at mode 0600 beside a spec.md at 0644; the code is unchanged by the window — `runtime/src/primitives/relocate_audit_records.rs`
    outcome: waived
    reason: No requirement of 057 states a mode, the artifacts are committed and not secret, and the creation is not in this window's change.
    decided-at: 2026-11-19T22:47:48Z
    decided-by: rule
  - key: other — load_record reads a dangling-symlink review.md or analysis.md as absent rather than unreadable; unreachable in normal use — `runtime/src/primitives/mod.rs`
    outcome: waived
    reason: Outside this window and 057's own code; a dangling link in a spec directory is not a state any pipeline step creates, and the gate already reports an unreadable record.
    decided-at: 2026-11-19T22:47:48Z
    decided-by: rule
  - key: "convention — a blank line on each side of a review: or analyze: block leaves two consecutive blank lines in the rewritten spec.md frontmatter — `runtime/src/primitives/relocate_audit_records.rs`"
    outcome: waived
    reason: "Untidy only: the frontmatter parses, the body is byte-identical, and the pre-window code did the same."
    decided-at: 2026-11-19T22:47:48Z
    decided-by: rule
  - key: "convention — relocate-audit-records relocates a block whose every field is null into an artifact carrying last-run: null, which the CI gate and check-review-gate read as never run — `runtime/src/primitives/relocate_audit_records.rs`"
    outcome: waived
    reason: "No record asserting a run is invented: a null last-run reads as never run at every gate, no spec in the corpus carried a null-only block when the sweep ran, and skipping all-null blocks is a behavior decision outside this window."
    decided-at: 2026-11-19T22:47:48Z
    decided-by: rule
  - key: "convention — the migration procedure's detection of a pre-relocation CI file keys on the literal status: done, which appears in the old template only in comments — `framework/migrations/audit-record-relocate.md`"
    outcome: waived
    reason: A heuristic prose detector whose miss costs one unshown re-copy notice; AC18 holds, and editing a migration an adopter may have run reaches no one who ran it.
    decided-at: 2026-11-19T22:47:48Z
    decided-by: rule
  - key: "convention — specs/046-scenario-open-question-visibility spec.md and plan.md say before the review: block and an absent review: block marks a spec as predating its feature — `specs/046-scenario-open-question-visibility/spec.md`"
    outcome: waived
    reason: The fix lies in 046's criterion AC20, which reopens 046 alone; the claim names a gate ordering and no spec.md placement, and 046 is reported to the owner rather than reopened in this run.
    decided-at: 2026-11-19T22:47:48Z
    decided-by: rule
  - key: "convention — the 047 scenario analyze-run-durability.md row write-review to the review: block is the original problem statement, unannotated at the row — `specs/047-analyze-findings-durability/scenarios/analyze-run-durability.md`"
    outcome: waived
    reason: A scenario edit reopens 047; the file's own 057 reversal note states the move, and the row names the block, not spec.md.
    decided-at: 2026-11-19T22:47:48Z
    decided-by: rule
---

# Review — 057-analyze-artifact-and-record-relocation

## Summary

057's review of the reopening that added scenario `a-merge-carries-each-keys-whole-value` (task 28, delivered by `ac98cc81`), window `5b20aaa4..HEAD` of the base `5b20aaa4` (base-source reopen), run in a clean worktree of `origin/main` at `b2421e14`. Six readers ran, each reading its rule files and files whole against measured sizes: A, the security pass (`security-backend.md` 121,432 bytes, `security-cross.md` 23,779 and `security-frontend.md` 49,538, all whole); B, the other four passes (the nine other rule files, 213,825 bytes, `AGENTS.md` Gotchas to the end 23,775 bytes and `specs/system.md`, all whole, plus the verdict on the stored decision, which still holds); and C1 to C4, the criteria step over all 26 criteria, which read the large runtime files by symbol and tests rather than whole and drove the named tests, the migration, the CI gate fixtures, markdown lint and the audits. Read whole of the scope: `runtime/src/primitives/relocate_audit_records.rs`, `scripts/mutation/relocate-merge-children-057.py`, and 057's `spec.md`, `data-model.md`, `tasks.md` and the scenario. `runtime/CHANGELOG.md` (574,001 bytes) was read in the window's hunks (16,272 bytes) and the spec 057 entry, so it is unread and named, as are `runtime/src/primitives/analyze_subjects.rs`, `runtime/src/primitives/check_artifacts.rs` and `runtime/src/primitives/check_review_gate.rs`, which the fix rounds added to the scope and which were read at their changed hunks by the re-reads, comment edits only. No MUST or SHOULD violation was found. Four criteria were first judged not met, each a drifted claim in 057's own spec and not a code defect: AC14 (a count of sections and a missing 073 and 082 attribution), AC19 (spec 079's refusal), AC25 and AC26 (wording the tree could not satisfy); each was corrected in `fix(057)` commits and judged again met, and four stale runtime comments AC26 found were reworded. One bug in the spec's own code was fixed with a test and mutant M9 (an artifact holding both `reviewed-at` and `last-run` was written with `last-run` twice). The two fix rounds were each re-read by an independent agent, clean, with a security and four-pass re-run and criteria readers re-judging the edited criteria. Findings in other specs' files (`046` spec and plan, the `047` scenario row) are waived here with their reason and reported to the owner. The stored waiver on `Relocation::write` is carried unchanged.

2 fix round(s): 2 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

26 of 26 acceptance criteria judged met.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Deferred findings

*None.*

## Acceptance criteria

- AC1 — met: write_analysis.rs render_analysis writes the record fields and a Summary body into analysis.md; tests a_first_run_writes_the_artifact_and_leaves_the_spec_untouched and a_clean_run_still_writes_the_artifact pass.
- AC2 — met: No specs/*/spec.md carries a top-level review: or analyze: block and framework/templates/spec/spec.md frontmatter is status and dependencies alone.
- AC3 — met: write_review.rs writes review.md only; tests frontmatter_review_block_inserted_when_absent and frontmatter_review_block_replaced_when_present assert spec.md byte-identical.
- AC4 — met: check_review_gate.rs reads the review record from review.md and the analyze record from analysis.md and keeps every blocking branch under passing tests; the one-time pre-relocation differential is not re-derivable now.
- AC5 — met: mod.rs load_record gives Absent, Unreadable or Present and the gate maps them to never-run and undeterminable; tests absent_review_block_blocks_not_reviewed and a_review_whose_frontmatter_does_not_parse_says_to_repair_it_by_hand pass.
- AC6 — met: validate_no_residual_records reports a residual review or analyze key in spec.md as Blocking and validate_record_artifacts validates each artifact; tests a_residual_record_block_in_the_spec_is_reported and an_unreadable_record_artifact_is_a_hard_fail pass.
- AC7 — met: analyze_subjects.rs is_analyze_subject excludes analysis.md outright and keeps review.md; tests the_record_is_not_a_subject_of_its_own_digest and the_review_record_remains_a_subject pass.
- AC8 — met: read_spec composes the review record from review.md and the dashboard never rendered the records; status_basic_stream_matches_golden, the 59 dashboard tests and 11 drift tests pass.
- AC9 — met: scripts/audit/analyze-record-backlog.sh counts done specs with a review.md and no analysis.md against a baseline of 0; run: 64 done specs examined, backlog 0, exit 0.
- AC10 — met: relocate_audit_records.rs relocates both blocks; at the sweep commit 110 block pairs compared by YAML parse with 0 mismatches, 76 of 76 spec.md now clean; 22 unit tests pass.
- AC11 — met: npx markdownlint-cli2 with the project config over 884 files: 0 issues.
- AC12 — met: relocate_audit_records.rs render_artifact carries blocking, waivers, diff-base and skipped-passes and folds reviewed-at into one last-run, also when an artifact holds both spellings; tests an_existing_artifact_is_merged_not_refused and a_legacy_key_beside_its_current_spelling_is_dropped_not_duplicated pass.
- AC13 — met: write_analysis.rs plain() strips a leading dash and checkbox marker from every caller text; test the_report_carries_no_checkbox_even_when_finding_text_arrives_with_one passes.
- AC14 — met: Overwrite and fixed skeleton hold (a_rerun_overwrites_its_own_record_and_leaves_the_review_record_alone); the 076, 073 and 082 attributions and the section order match render_analysis and docs/analyze.md.
- AC15 — met: No check_review_agreement.rs, review-block-agreement.sh or Family 31 remains; remaining hits are the retired-tool deny list and records of the removal, and never-reviewed and never-analyzed stay distinct through AC5.
- AC16 — met: framework/migrations.toml carries one audit-record-relocate entry with its procedure file and scripts/audit/migration-coverage.sh (Family 10) exits 0 over 15 entries.
- AC17 — met: The CI template Audit-record gate reads review.md and analysis.md and bounds absent records by a committed mark; eight driven fixtures behave as stated and none passes every spec.
- AC18 — met: framework/migrations/audit-record-relocate.md step 5 detects the old status: done and review: test and prints the re-copy warning, as a notice for the agent.
- AC19 — met: Every recording analyze run writes analysis.md, clean and empty scope included (a_clean_run_still_writes_the_artifact), and the 079 refusal is annotated in the criterion and matches refuse_while_implementation_is_open.
- AC20 — met: relocate_audit_records.rs returns Converged for a block-less spec; driven over a partially migrated fixture the second pass left every file byte-identical.
- AC21 — met: framework/constitution.md Audit records tables declare each record's owning artifact, fields and types, matching ReviewBlock and AnalyzeBlock and what write_review and write_analysis emit.
- AC22 — met: docs/analyze.md The record gives the field table and The report body the skeleton in render order; docs/slash-commands.md, docs/shared-constitution.md and README.md place neither record in spec.md frontmatter.
- AC23 — met: Every mention of the records in framework/commands/{review,analyze,implement,audit,consolidate}.md and their references names review.md or analysis.md frontmatter; no review: block mention remains in framework/.
- AC24 — met: scripts/gen-claude-commands.sh --check exits 0 and the Family 16, flag-hint, readme-command and check-zero parity scripts exit 0.
- AC25 — met: AGENTS.md and framework/templates/spec/spec.md carry no statement placing either record in spec.md frontmatter (searched, not read line by line for AGENTS.md).
- AC26 — met: At 2ca97542 no hit states a review: or analyze: block as spec.md frontmatter as current outside the named relocation records and annotations; the reworded runtime comments are clean.

## Observations

- bug — a write or lock failure after an earlier artifact was written (the lock wait timing out, or the rename failing) returns a plain Io error that does not name the files already written, though the relocation is then half done; only a changed-since-read refusal becomes relocation-incomplete. Judged against the contract: the data model and task 27 state the half-done report for a refused (changed) file only, the Io error names its own path, and a re-run is idempotent (AC20), so no stated requirement is broken — `runtime/src/primitives/relocate_audit_records.rs:205-214` — **waived**: 057's contract (data model Errors table, task 27) states the half-done report for a changed-since-read refusal; an Io or lock-timeout failure after a write has no stated requirement, names its failing path, and the merge is idempotent so a re-run finishes the relocation (decided by the rule)
- convention — relocate-audit-records creates a new review.md or analysis.md at mode 0600 beside a spec.md at 0644; the code is unchanged by the window — `runtime/src/primitives/relocate_audit_records.rs` — **waived**: No requirement of 057 states a mode, the artifacts are committed and not secret, and the creation is not in this window's change. (decided by the rule)
- convention — a renamed legacy key with an empty value was written with a trailing space — `runtime/src/primitives/relocate_audit_records.rs` — **fixed** (decided by the rule)
- other — load_record reads a dangling-symlink review.md or analysis.md as absent rather than unreadable; unreachable in normal use — `runtime/src/primitives/mod.rs` — **waived**: Outside this window and 057's own code; a dangling link in a spec directory is not a state any pipeline step creates, and the gate already reports an unreadable record. (decided by the rule)
- bug — an existing artifact holding both reviewed-at and last-run, with the block carrying neither, was merged with last-run written twice, a duplicate key validate-frontmatter reports as a hard fail — `runtime/src/primitives/relocate_audit_records.rs` — **fixed** (decided by the rule)
- convention — a blank line on each side of a review: or analyze: block leaves two consecutive blank lines in the rewritten spec.md frontmatter — `runtime/src/primitives/relocate_audit_records.rs` — **waived**: Untidy only: the frontmatter parses, the body is byte-identical, and the pre-window code did the same. (decided by the rule)
- other — the scenario's multi-line flow collection edge case did not match entries, which groups an indented continuation under its key — `specs/057-analyze-artifact-and-record-relocation/scenarios/a-merge-carries-each-keys-whole-value.md` — **fixed** (decided by the rule)
- other — the module doc comment of relocate_audit_records.rs read as a run-on after the window's splice and stated a stale corpus count — `runtime/src/primitives/relocate_audit_records.rs` — **fixed** (decided by the rule)
- convention — relocate-audit-records relocates a block whose every field is null into an artifact carrying last-run: null, which the CI gate and check-review-gate read as never run — `runtime/src/primitives/relocate_audit_records.rs` — **waived**: No record asserting a run is invented: a null last-run reads as never run at every gate, no spec in the corpus carried a null-only block when the sweep ran, and skipping all-null blocks is a behavior decision outside this window. (decided by the rule)
- convention — the migration procedure's detection of a pre-relocation CI file keys on the literal status: done, which appears in the old template only in comments — `framework/migrations/audit-record-relocate.md` — **waived**: A heuristic prose detector whose miss costs one unshown re-copy notice; AC18 holds, and editing a migration an adopter may have run reaches no one who ran it. (decided by the rule)
- convention — AC25 said AGENTS.md and the template describe the new location of the records, which neither did — `specs/057-analyze-artifact-and-record-relocation/spec.md` — **fixed** (decided by the rule)
- convention — AC14's annotation counted the sections, credited ## Waived rules to the wrong spec and named a description that does not list the order, and data-model.md's skeleton omitted three sections — `specs/057-analyze-artifact-and-record-relocation/spec.md` — **fixed** (decided by the rule)
- convention — AC19 said every analyze run writes analysis.md, which spec 079's refusal makes untrue for a run stopped on an unchecked task — `specs/057-analyze-artifact-and-record-relocation/spec.md` — **fixed** (decided by the rule)
- convention — runtime doc and test comments in analyze_subjects.rs, check_review_gate.rs and check_artifacts.rs placed the review: block in spec.md as current — `runtime/src/primitives/analyze_subjects.rs` — **fixed** (decided by the rule)
- other — AC26's carve-out named only 057's Motivation and Resolved Questions, so the criterion could not be satisfied as worded — `specs/057-analyze-artifact-and-record-relocation/spec.md` — **fixed** (decided by the rule)
- convention — the fix round's reworded comments left long lines, a grandfather comment naming review.md for a case read-spec leaves empty too, an analyze:-block exclusion comment, and a local named for the function it calls — `runtime/src/primitives/check_artifacts.rs` — **fixed** (decided by the rule)
- convention — specs/046-scenario-open-question-visibility spec.md and plan.md say before the review: block and an absent review: block marks a spec as predating its feature — `specs/046-scenario-open-question-visibility/spec.md` — **waived**: The fix lies in 046's criterion AC20, which reopens 046 alone; the claim names a gate ordering and no spec.md placement, and 046 is reported to the owner rather than reopened in this run. (decided by the rule)
- convention — the 047 scenario analyze-run-durability.md row write-review to the review: block is the original problem statement, unannotated at the row — `specs/047-analyze-findings-durability/scenarios/analyze-run-durability.md` — **waived**: A scenario edit reopens 047; the file's own 057 reversal note states the move, and the row names the block, not spec.md. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.ductus/reference/analyze.md`
- `.ductus/reference/audit.md`
- `.ductus/reference/plan.md`
- `.ductus/reference/review.md`
- `framework/reference/analyze.md`
- `framework/reference/audit.md`
- `framework/reference/plan.md`
- `framework/reference/review.md`
- `runtime/src/main.rs`
- `runtime/src/primitives/check_step_references.rs`
- `runtime/src/primitives/commit_attribution.rs`
- `runtime/src/primitives/compute_review_scope.rs`
- `runtime/src/primitives/tool_loading.rs`
- `runtime/src/primitives/write_review.rs`
- `runtime/src/schema/descriptions.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/tests/check_step_references_cli.rs`
- `scripts/audit/README.md`
- `scripts/audit/step-reference-integrity.sh`
- `scripts/mutation/review-read-credit.py`
- `scripts/mutation/scope-attribution.py`
- `scripts/mutation/step-references-bounded.py`
- `scripts/mutation/step-references-reach.py`
- `scripts/tests/test-step-reference-integrity.sh`
- `specs/020-code-review/data-model.md`
- `specs/020-code-review/spec.md`
- `specs/022-deterministic-runtime/tasks.md`
- `specs/026-framework-self-audit/scenarios/families-follow-the-command-reference-split.md`
- `specs/026-framework-self-audit/scenarios/family-34-reference-section-reachability.md`
- `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/026-framework-self-audit/tasks.md`
- `specs/041-task-pruning/tasks.md`
- `specs/051-branch-scoped-spec-numbering/tasks.md`
- `specs/066-audit-records-and-the-completion-gate/tasks.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-bare-port-span-names-its-remedy.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/071-command-reference-split/scenarios/reference-sections-are-reached-by-a-step.md`
- `specs/071-command-reference-split/tasks.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/scenarios/the-excluded-section-lists-paths-only.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/079-review-and-analysis-work-converges/tasks.md`
- `specs/080-deterministic-runtime-efficiency/tasks.md`
- `specs/081-reviews-and-checks-follow-what-changed/spec.md`

## Unexamined governance

*None.*
