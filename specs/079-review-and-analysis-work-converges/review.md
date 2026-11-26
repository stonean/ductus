---
spec: 079-review-and-analysis-work-converges
last-run: 2026-11-24T00:26:23Z
reviewed-against: fbdb88c99f99424bee4b3f7f0ef712570018261b
diff-base: 11a208beb44f712b66ad4f6b34dc9a97458db98c
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
scope: 14
unread:
  - .ductus/reference/review.md
  - framework/reference/review.md
  - runtime/CHANGELOG.md
  - runtime/src/primitives/compute_review_scope.rs
  - runtime/src/primitives/write_review.rs
  - runtime/src/schema/descriptions.rs
  - runtime/src/schema/primitives.rs
  - scripts/mutation/scope-attribution.py
  - specs/079-review-and-analysis-work-converges/plan.md
  - specs/066-audit-records-and-the-completion-gate/data-model.md
excluded: 36
contract-breaks: 0
skipped-passes: []
fix-rounds: 2
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 2c8d2c8174acb033015d094acf4db88c5c05efa068077ef11d03a1405433d1c2
  scenarios/the-excluded-section-lists-paths-only.md: 1763a8c3a9dd478a6f39d2813a73d4804c21dc8e9581c401af3fdaf35e6ea8b1
criteria-judged: 30
criteria-met: 30
criteria-verdicts:
  - label: AC1
    digest: fa48392c2dd9cb01f9d12a2a9878af58a556988d6c3a6b66d77add9a2f3672d2
  - label: AC4
    digest: 790dc90c692a82c6506373d8b7155f0de81f9e46cd4ae03cc67eab3af12527b5
  - label: AC2
    digest: 10cb16f6aba96b7e5b279bfa4c58f13c24deaf8cfa27f494acf760823afb2257
  - label: AC3
    digest: 15809584b8010c1f223e1426ad8b8593f80239877302047f96df2093273462b2
  - label: AC5
    digest: 1f8b0aa43132e9e6bfca3ff33c22b1456cd93aa1e77bef6fe0f3fd684ff93832
  - label: AC6
    digest: 73a2742f45acffa7bf01275f9ca2c2bbb5a5a8ef4104fe70f45b5ce14a6186d7
  - label: AC7
    digest: 99dbce01a1c1467bc7fcb89ce108c53bcfe99f4611cd09e958c4bc4d4eb8a9e8
  - label: AC8
    digest: 7a2a570093e2d91c370f1988bcec2f7e5357d33ae6831be1cc995b91ee49cd9a
  - label: AC9
    digest: 5743829473ab0c3bd98da260a0d47fecddcf1899bf0c943925f348b6d388e076
  - label: AC10
    digest: e2d73c37114889600a8086d80a2350546b5a9d48477f2a375d7a822406aa5a99
  - label: AC11
    digest: c16e37158b6a250276e34690844ee2a1150108bd5a65473c02c68e3882217db8
  - label: AC12
    digest: f69f46602828802930ec3a162dcd5f2d7e88a75ce59df0b760ff00cff1372e63
  - label: AC13
    digest: 0c6f34f43ffccd9f94c92f11b19fa9e1fe88a75eeb3732d4f01859cf6170b903
  - label: AC14
    digest: 49a76c57e36802a1d3842ff1c6f51e333425af2014d303d0286ee98c6348612b
  - label: AC15
    digest: 665051f4d8326937f1f99a61b05837f0d9a97cc3865be7766f7645d77ac86845
  - label: AC16
    digest: 06e5b44632a25e7b07398df3a78c4b9ba1be9abbbe4ddd6f8f4f38e36423dccd
  - label: AC17
    digest: 0d7d3f92e8ee5175b16945fa9a872131a3c67e199a6034230c4049fd0801b35e
  - label: AC18
    digest: c13f38b439a3024c5dce7928d6589e227cf5b647c16e7833019be0ffd08a491b
  - label: AC19
    digest: 7259f8699b6deb617af832a9f9544ee0b6598416d752240b41ce845f2f26f052
  - label: AC20
    digest: 193de5ba3110c10af37c5a13c3f87b4ec4a7cd4e935d401b7a7d9bdeae229d5e
  - label: AC22
    digest: 6a66e3d6ca61d1975011b357008d686ae3835ca78c61effcadc6cdce2239dbd7
  - label: AC23
    digest: 5b8b6ebd9a75263371f3bc6b3e6cc97d13af422afd837c2b6aab39fdd1a17506
  - label: AC24
    digest: ab04fef200bc494660e09758f6f21c91fdd22d1aa6163ea236866385158f1012
  - label: AC25
    digest: 6c362511c768e6ca9e0dcf2d15e9aab2c199f7e98562adf69901535d01ef2167
  - label: AC26
    digest: b4e0d39122f65f63d479d3b212c1d3eeb0e05c78f0426a75d568b7571dabf125
  - label: AC21
    digest: 1643ca3f8f61e02fc5fbe6a0fbc757d4866bd49ab8496eb412d936836a8f3c94
  - label: AC27
    digest: efb0320430f95af92ad0e0c635bc42e937b89722eeef613498d44a833d6a2145
  - label: AC28
    digest: e674c242c1783ddb47adafcd834be2b9109ae81922de0507f05c2d47067c50c6
  - label: AC29
    digest: 3c3b93058c3baf2dac1a8265512f32cc4ffb6211caea3a4018cccc995e69cd5a
  - label: AC30
    digest: 74dff4756ec7941dad058dcb549f4b5cdf4cfdbe06bd9ef43735e4c2e5a8e3c9
blocking: false
dispositions:
  fixed: 8
  deferred: 0
  waived: 8
  undispositioned: 0
decisions:
  - key: "bug: a first review that stopped early still drops unchanged plan entries from the next scope, so what its skipped pass never read stays unread, against the module doc's 'stopping early leaves nothing permanently unread'. — `runtime/src/primitives/compute_review_scope.rs:201`"
    outcome: waived
    reason: AC6 keeps an unchanged plan entry only on the first review, and that run was the first; what its skipped pass did not read is named in the record's skipped-passes, and restricting a run is the operator's choice.
    decided-at: 2026-11-05T21:43:57Z
    decided-by: rule
  - key: "perf: check-prescriptions counts an exempt path as unprescribed, so a round that applied exactly the prescribed edits and also touched spec.md, plan.md or tasks.md differs and costs another re-read of paths the check had just matched. — `runtime/src/primitives/check_prescriptions.rs:82`"
    outcome: waived
    reason: "No criterion covers it: AC13 covers the brief, and this costs one re-read on a rare round that also edits the spec's own text."
    decided-at: 2026-11-05T21:43:57Z
    decided-by: rule
  - key: "bug: a spec directory renamed while in-progress counts a record carried from before the rename as a run of this reopening, which can report a false over-bound. — `runtime/src/primitives/measure_reopenings.rs:708`"
    outcome: waived
    reason: A rename mid-reopening is rare, it over-reports rather than hides, no gate reads the count, and the module doc states that a renamed spec's history starts at the rename.
    decided-at: 2026-11-05T21:43:57Z
    decided-by: rule
  - key: "other: cross_service.rs's second assertion and tests/parity/cross-service/expected.txt are now a copy of the golden whose only effect is requiring a re-bless in two places, as the file's own doc says. — `runtime/tests/cross_service.rs:60`"
    outcome: waived
    reason: plan.md's Affected Files keeps this assertion on purpose, and 030's plan and tasks name the capture, so removing it is a design choice no criterion here asks for.
    decided-at: 2026-11-05T21:43:57Z
    decided-by: rule
  - key: "bug: /amend's reopen check runs classify-spec-edit only on a delta that excludes a modified scenario and a data model, the two edits §spec-lifecycle says always reopen, and its line then says no scenario edit exists. — `framework/commands/amend.md:58`"
    outcome: waived
    reason: The delta predates this spec, AC15 governs what classify-spec-edit returns and not which edits reach it, and widening the delta changes what the reconcile pass offers, which is a requirement of its own.
    decided-at: 2026-11-05T21:43:57Z
    decided-by: rule
  - key: step 8 names no family — `framework/commands/analyze.md`
    outcome: waived
    reason: It points at the description, as AC9 asks.
    decided-at: 2026-11-06T21:57:41Z
    decided-by: rule
  - key: "`1. [ ]`, `> - [ ]`, `* [ ]` and `+ [ ]` are no boxes to `find_checkbox_line` — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: One documented grammar, `mark-task`'s too, every surface alike, and 0 such lines here.
    decided-at: 2026-11-06T21:57:41Z
    decided-by: rule
  - key: a claim carried only in a reference's label reads `mechanical` — `runtime/src/primitives/derive_references.rs`
    outcome: waived
    reason: The scrub removes label and target by its stated design, and `plan.md`'s known limitations say so.
    decided-at: 2026-11-06T21:57:41Z
    decided-by: rule
  - key: a reference added as an ordered item or a table row reads `reopens` — `runtime/src/primitives/classify_spec_edit.rs`
    outcome: waived
    reason: It errs toward reopening.
    decided-at: 2026-11-06T21:57:41Z
    decided-by: rule
  - key: a reference-only edit reads `mechanical` while staleness reads stale — `runtime/src/primitives/classify_spec_edit.rs`
    outcome: waived
    reason: Exempting references from the digest is a change of its own, with 0 occurrences since no spec here carries `references:`.
    decided-at: 2026-11-06T21:57:41Z
    decided-by: rule
  - key: "`cross-spec-impact:` names neither 000 nor 035, though the round edited their scenarios and reopened 035 — `specs/079-review-and-analysis-work-converges/spec.md`"
    outcome: waived
    reason: Each is a correction of a claim, which owes no back-link and names this spec in its commit, as `plan.md`'s Cross-spec impact says; AC12's list is of specs whose surface this changes.
    decided-at: 2026-11-06T21:57:41Z
    decided-by: rule
  - key: "`an_open_break_names_its_file` refuses a blank-path break whose deferral releases it, though only a break no deferral releases must name its path — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: It fails closed, the host passes the path it has, and the condition predates this run's fixes.
    decided-at: 2026-11-06T21:57:41Z
    decided-by: rule
  - key: "bug: an empty diff-base records scope 0 without consulting compute-review-scope, so a spec never in-progress records a first review's examined beside scope 0 — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: The data model, write-review's description and the owner's task-20 choice (2026-11-07) define an empty diff-base as scope 0, and deriving it instead is a requirement of its own.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other: write-review reports every compute-review-scope error as an invalid diff-base, an unreadable plan included — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: The message carries the underlying error, and telling a base's errors from the scope's is a change to the error contract no criterion asks for.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other: process-decisions' expired notice does not mark the decisions naming a file the scope holds, which step 9 re-checks — `runtime/src/primitives/process_decisions.rs`"
    outcome: waived
    reason: The owner chose the host's re-check against the file (task 22, 2026-11-07), and marking those decisions in the notice is a requirement of its own.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "bug: deferring an observation to an existing open task exempts that task's earlier boxes from the open-box refusal — `runtime/src/primitives/decisions.rs`"
    outcome: waived
    reason: "No command reaches it: step 1 stops before a run's deferral exists, and ductus exec writes no deferral, so only a hand-driven write does."
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "convention: owed-run prose on a task whose boxes are all ticked is dropped by prune-tasks while the runs are owed, and on a task with no box it is never dropped — `runtime/src/primitives/prune_tasks.rs`"
    outcome: waived
    reason: What a spec owes is derived by check-review-gate, which the prose only names, so pruning it loses no gate, and where owed runs live is a requirement of its own.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "bug: 026's, 058's and 078's ordering constraints lost the box /implement and the gate read, so no step holds a review to them — `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md`"
    outcome: waived
    reason: "No gate orders one spec's runs after another's: the owner sets the closing order (task 12, 2026-11-09), and a gate for it is a requirement of its own."
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other: an analysis whose own chore fix appends a task cannot record, since only a deferral's task is exempt — `runtime/src/primitives/write_analysis.rs`"
    outcome: waived
    reason: AC28's exception covers a deferral alone, as written, and a fix that appends a task is no chore.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other: no test pins REOPENING_WINDOW_DAYS's 30 days, both default-window tests deriving the window from the constant — `runtime/src/primitives/measure_reopenings.rs`"
    outcome: waived
    reason: It is outside this scope and in another session's active file (spec 080, 01b97a87), and the constant is defined once and named in the data model.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other: 041's prune scenario rests a plan section's removal on its changing no claim, the removed test — `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md`"
    outcome: waived
    reason: It records a decision taken under the old rule, and its conclusion holds, since §spec-lifecycle gives a plan.md edit no back-edge.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other: Family 24's guidance says a repair in a scenario or data model reopens, while the same repair made across files reads as a sweep — `scripts/audit/rename-sweep-residue.sh`"
    outcome: waived
    reason: Family 24 reports each site on its own, and §spec-lifecycle says a typo fixed in one file reopens.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other: 063's split Resolved Question calls a split a meaningful edit because it moves a requirement, which holds only for a criterion or scenario — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`"
    outcome: waived
    reason: A split moves requirements, which are criteria or scenarios, so its conclusion holds.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other: 063's plan justifies 058's step-number sync as mechanical because it changes no claim — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md`"
    outcome: waived
    reason: It records a decision, and its conclusion holds under §spec-lifecycle's case (a).
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other: 050's plan says AC13 settles whether an edit that changes no claim is mechanical — `specs/050-constitution/plan.md`"
    outcome: waived
    reason: It states the question AC13 settled, which the plan's following sentences now answer with the default and classify-spec-edit's test.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other — drop_carried matches a carried decision on key and outcome only, so two cases read as carried that decisions::merge would call new or that were never counted as a person's: a deferral re-made with different work (merge treats changed work as a fresh decision), and a person's decision matching one a rule made in another spec's record (decisions_held takes decisions whoever made them, so the person's decision then counts in neither record). Both follow the owner's stated rule, same key and outcome already held (2026-11-12), and neither was observed. — `runtime/src/primitives/measure_reopenings.rs:719`"
    outcome: waived
    reason: The Resolved Question on what counts as a person's decision states the rule as same key and outcome already held, and comparing work or the author is a requirement of its own that no recorded case asks for.
    decided-at: 2026-11-16T16:32:26Z
    decided-by: rule
  - key: other — the two new carried-decision tests build the same two-spec fixture (the decision closure, the 013-b writer, the status flip, the first spec's record) line for line and differ only in the outcome of the second decision. — `runtime/src/primitives/measure_reopenings.rs:1422`
    outcome: waived
    reason: Two uses of a short fixture inside a test module; a shared helper would hide which outcome each test varies, which is the one thing each test is named for.
    decided-at: 2026-11-16T16:32:26Z
    decided-by: rule
  - key: "other — the Behavior bullet 'A behavior is stated once' now requires existing restatements in a data model or scenario to be swept to pointers (8596f472), but no acceptance criterion tests it: AC9 covers the description, commands and references only, AC12 corrects false claims. A criterion would be one nothing can check, since the spec's own Resolved Question records that nothing detects a new copy. — `specs/079-review-and-analysis-work-converges/spec.md:64`"
    outcome: waived
    reason: The Resolved Question on the canonical surface states that nothing detects a new copy (§drift-prevention) and that this part relies on review, so no testable criterion exists to add; the sweep was task 36's, and a criterion is a requirement of its own.
    decided-at: 2026-11-16T16:32:26Z
    decided-by: rule
  - key: other — the eight descriptions_hold tests in mcp.rs assert that each description contains a sentence, so they pass when the code stops behaving as the sentence says; the truth-check of each claim was a one-time reading, and the test that pins the process-waivers plan claim pins a sentence the code does not bear out. — `runtime/tests/mcp.rs:337`
    outcome: waived
    reason: Each test fails when its sentence is removed, as the commits record, and AC9 makes the description the canonical text; a test tying each sentence to the code's behavior is a requirement no criterion of 079 states, and the specific false sentence is the first observation.
    decided-at: 2026-11-16T16:32:26Z
    decided-by: rule
  - key: other — the eight descriptions_hold tests carry the same assertion loop in two shapes (assert per item in the first three, collect missing in the later five) and could share a helper. — `runtime/tests/mcp.rs:337`
    outcome: waived
    reason: Behavior-neutral test style with no criterion asking for a helper, and the file is shared with other sessions.
    decided-at: 2026-11-16T16:32:26Z
    decided-by: rule
  - key: "perf — the seven commits add 8,176 characters to the compiled descriptions (sum over each changed constant's text between the quotes, before and after each commit: 2,718 + 830 + 1,103 + 1,023 + 742 + 989 + 771), which every schema fetch serves; WRITE_REVIEW alone is 7,169 characters and CHECK_REVIEW_GATE 4,262 at HEAD. — `runtime/src/schema/descriptions.rs`"
    outcome: waived
    reason: The owner chose moving restatements into the descriptions on 2026-11-12 (spec Resolved Questions), the byte limit of AC18 covers commands and references and not runtime code, and the growth is recorded here with its method.
    decided-at: 2026-11-16T16:32:26Z
    decided-by: rule
  - key: "other — 18c24c78's check_artifact_size doc comment says an unreadable index makes each project file unreadable and exits 1, which staged_bytes and size_check_blocks bear out, but no test opens a corrupt index: the CLI test covers a conflicted entry (Staged::Unreadable by the conflict branch) and the unit test a file unreadable on disk. — `runtime/src/primitives/check_artifact_size.rs:96`"
    outcome: waived
    reason: The change is a doc comment, the claim is true by reading staged_bytes, and a test for a corrupt index is a requirement no criterion of 079 states.
    decided-at: 2026-11-16T16:32:26Z
    decided-by: rule
  - key: other — python3 -I scripts/run-workflow-steps.py --help is read as a workflow path and exits 2 with 'cannot read --help'; only a bare run prints the usage line, so the docstring's usage text has no way to be asked for. — `scripts/run-workflow-steps.py:127`
    outcome: waived
    reason: Task 38 specifies no help flag and the bare invocation prints the usage line with exit 2; adding one is a new interface, which would need its own test, not a chore.
    decided-at: 2026-11-16T16:32:26Z
    decided-by: rule
  - key: "other — twelve done specs still hold a task with no checkbox whose prose says a review, an analysis or the completion gate is owed: 010 task 18, 014 task 9, 019 task 2, 023 tasks 24 and 25, 030 task 14, 035 task 7, 045 task 17, 051 task 36, 054 task 13, 059 task 22, 062 task 29 and 076 task 15. Each spec is done, so the work is spent, and 023's task 25 also quotes in full a paragraph that its plan.md now holds. The prune keeps a task with no box as pending, and AC27 and AC28 make a box what a spec owes, so no gate reads these tasks; they cost a read and describe owed work in a done spec. — `specs/023-govern-refinement/tasks.md:11`"
    outcome: waived
    reason: tasks.md holds no durable requirement and no gate reads a task without a box, so a spent one costs a read, not a verdict; dropping one is a per-spec judgment and which tasks a prune keeps is 041's design, so the fix lands in other specs' directories.
    decided-at: 2026-11-16T16:32:26Z
    decided-by: rule
  - key: "other — the runner now reads a step whose env: holds a ${{ }} expression as SKIP, so framework-checks.yml's and generators.yml's Build the runtime step, whose env: carries CACHE_HIT, no longer runs locally, and a local run checks the binary already built — `scripts/run-workflow-steps.py`"
    outcome: waived
    reason: ".ductus/docs/agents/developing-ductus.md step 2 runs cargo build --release --locked before the workflows' steps, and the SKIP is reported with its reason, never as a pass; running a step without a value CI gives its env: is what the fix removed."
    decided-at: 2026-11-16T16:32:26Z
    decided-by: rule
  - key: "security — BE-INPUT-004's Verification counts an operator-set value as user input, and the runner opens the workflow files and the --log-dir directory named on its own command line with no canonicalization or containment. The rule needs an expected base directory to compare against, and none exists for these arguments: the operator names the workflow whose commands run and where its logs go, and the runner executes that workflow's text by design. The path the rule reaches, a working directory the committed workflow text names, is the one now checked, and it is checked by realpath and commonpath (verified by importing parse and contained from the runner without running a step: all 68 run steps of the repository's own workflows resolve inside the root, and '../x' and '/etc' are refused). — `scripts/run-workflow-steps.py:262-279`"
    outcome: waived
    reason: No base directory exists for the workflow-file and log-directory arguments, since the operator's own invocation names what runs and where its logs land, so the containment test has nothing to bind; the committed-text working directory BE-INPUT-004 reaches is checked.
    decided-at: 2026-11-16T17:52:48Z
    decided-by: rule
  - key: other — a working-directory that stays inside the root but does not exist (a typo) passes contained() and then raises FileNotFoundError out of subprocess.Popen in run_bounded, so the run ends with a traceback and exit 1 after the earlier steps ran, with no summary line and the later steps unreported; reproduced on a fixture workflow of three steps. The run is loud, not silent, and the behavior is the same as before the window (the directory was used unresolved before). — `scripts/run-workflow-steps.py:233-246`
    outcome: waived
    reason: The failure is loud (traceback, non-zero exit) and unchanged by this fix; refusing a missing directory before any step runs is a new exit-2 rule that task 38's criteria do not state, while this fix was asked to bound where a step may run, not whether the directory exists.
    decided-at: 2026-11-16T17:52:48Z
    decided-by: rule
  - key: "convention: the list header says the next mutation takes the next free number while `M319` follows `M273` in `scripts/mutation/scope-attribution.py` — `scripts/mutation/scope-attribution.py`"
    outcome: waived
    reason: Mutation ids are allocated across every list under `scripts/mutation/`, where `M318` is the highest of another list, so `M319` was the next free id; renumbering it to `M274` would reuse a number range other lists hold and gain nothing.
    decided-at: 2026-11-20T00:32:48Z
    decided-by: rule
---

# Review — 079-review-and-analysis-work-converges

## Summary

Five-pass review of 079's reopening (diff-base `11a208be`, base-source `reopen`) at `fbdb88c9`, run in a clean worktree because other sessions' uncommitted edits shared the main checkout (the agent's recommendation, adopted by the operator, 2026-11-23). The scope was 13 entries when the passes began, 14 with this run's fix to 066's data model, and 079's change is its four commits there, `a91bed0d`, `c55f7222`, `8b554e89` and `4ace1d51`, about 189 KB of diff. Three parts read it: A the security pass against `security-backend.md`, `security-frontend.md` and `security-cross.md` (192,970 bytes, each whole); B the reuse, quality, efficiency and simplicity passes against `quality-cross.md` and `configuration-cross.md` (38,992 bytes, each whole), `AGENTS.md`'s sections and `specs/system.md`, with a verdict on each of the nine stored decisions keyed on an in-scope file; C the criteria step. No finding. Eight observations, each fixed in the run by the rule: `join_unread` let a gone plain-path plan entry join, and after that fix a gone `dir/` one (a broken contract of AC30, both fixed); M154's filter named a test task 41 renamed; 066's data-model size sentence and 079's data-model accounting of `modified-since` stated what this change made false; the `scope` and `excluded` doc comments restated the scope rule; and this run's own fix misplaced `entry_pattern`'s doc comment. The criteria step first judged AC9 and AC12 not met on the fixed tree, for the `since` and `review_owed` doc comments restating the window and `review-owed` rules (`--help` renders `since`) and for 066's data-model paragraph restating the scope's contents; each was fixed in the run, and C judged them met against the fixed tree. A and B re-ran their passes over each fix set; the last two fixes, the doc comment moved and the `excluded` comment pointed at the description, were not re-run by a pass and are read by the fix-round re-read. Eight stored decisions still fire and keep their waivers; the one on `AGENTS.md` no longer fires and expires; thirty on files this run did not read are retained. The criteria step judged every criterion individually against the code it names: 30 of 30 met.

Coverage: `examined` counts the four entries read whole, `specs/079-review-and-analysis-work-converges/spec.md`, `data-model.md` and `tasks.md`, and `AGENTS.md`, which part B held whole from a copy identical to the worktree's (no commit touched it after `bcc439f9`). The ten `unread` entries had 079's window hunks read with the code around them: `runtime/CHANGELOG.md` (its `[Unreleased]` entry whole), `runtime/src/schema/primitives.rs`, `runtime/src/schema/descriptions.rs` (the whole `COMPUTE_REVIEW_SCOPE` constant), `runtime/src/primitives/write_review.rs`, `runtime/src/primitives/compute_review_scope.rs`, `scripts/mutation/scope-attribution.py` (each entry 079 touched), `framework/reference/review.md` (the sections the passes apply), `specs/079-review-and-analysis-work-converges/plan.md` (its window hunks and headings) and `specs/066-audit-records-and-the-completion-gate/data-model.md`, which joins the scope as this run's uncommitted fix (its two corrected sentences); `.ductus/reference/review.md` is generated from `framework/reference/review.md` by `scripts/gen-claude-commands.sh`, re-run in sync, and was not opened. Other specs' hunks in these files, 073's and 083's, are theirs and were read only where a 079 hunk depends on them.

2 fix round(s): 1 re-read, 0 diff-checked, 1 exempt, 0 not re-read. Exempt: round 2.

30 of 30 acceptance criteria judged met.

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

- AC1 — met: `runtime/src/primitives/measure_reopenings.rs` `over_bound` (second run of a kind needs the prior run non-gating), `review_fields` (blocking, MUST, SHOULD, contract-breaks, open mark, budget stop, skipped pass) and `analysis_fields`, `OpenPass::close` (not-counted reasons), `waiting_features` (no unchecked task via `unchecked_lines`); the release binary ran it. The 'or has a task disposing the miss' clause is a spec-side disposition, outside the primitive.
- AC4 — met: `runtime/src/primitives/compute_review_scope.rs` `owes_a_review` (no run, `holds_done_until_reviewed`, code paths, criteria owing), `paths_owing_a_review` (filters `is_project_artifact` in `runtime/src/primitives/mod.rs`, so scenarios, data model and other specs owe none) and `runtime/src/primitives/criterion_verdicts.rs` `owing_in` (unticked or unjudged-text criterion owes one); `runtime/src/primitives/check_review_gate.rs` `holds_done_until_reviewed` reads the gating count, missing dispositions, open mark, unwaived rounds and open findings, and a stale digest is read by none of them.
- AC2 — met: `framework/reference/review.md` Observations ('The rule decides, and nobody is asked, except for a broken contract'; Fixed/Deferred/Waived list) restricts a task-adding deferral to a broken contract and waives the rest; `runtime/src/primitives/write_review.rs` `validate_finding_dispositions` refuses fixed or waived outcomes on a violation and `open_findings` keeps only counted violations and unreleased breaks open; the person-overrule exception is the reference's 'A person overrules the rule on an observation'.
- AC3 — met: `runtime/src/primitives/compute_review_scope.rs` `split_changes` puts a path only another spec's commit changed (and no uncommitted edit) in `excluded`; `runtime/src/primitives/write_review.rs` `resolve_scope` passes it through and `render_excluded` lists each path (test `the_record_names_each_excluded_path` asserts the `## Excluded from scope` section and `excluded: 1`); `runtime/src/primitives/fix_rounds.rs` `round_paths` and `changed_only_by_other_specs` exclude the same for fix rounds and `render_excluded` names them per round.
- AC5 — met: `runtime/src/primitives/compute_review_scope.rs` `resolve_base` and `held_base` start the window at the finishing commit (last-review) or a held record's `reviewed-against` no earlier than the reopen (held-review), `recorded_run` makes a skipped pass, budget stop, or absent or non-empty `unread` a stopped-early run that leaves the start at the reopen (`reopen-past-partial-review`, else `reopen`), and `--since` is verbatim; tests `each_hold_on_a_record_that_read_its_scope_starts_at_its_reviewed_against`, `a_held_record_that_did_not_read_its_scope_leaves_the_window_at_the_reopen`, `a_held_start_is_no_earlier_than_the_reopen` pass.
- AC6 — met: `runtime/src/primitives/compute_review_scope.rs` `attribute_window` (own vs others by claimants, `named`, plan entries and the unplanned rule, uncommitted whole via `split_changes`), `WindowClaims::take_unclaimed` with `read_records` (081 credit), `FoldTest::brought` (folded-in below `done`), plan entries only on a first review in `compute`; `runtime/src/primitives/commit_attribution.rs` `Attributor::claims` (directory touch or subject scope; a merge's own paths only) and `own_paths`.
- AC7 — met: `runtime/src/primitives/diff_cross_spec.rs` `compute` takes `window_base` (the transition parent via `transition_parent`, HEAD while uncommitted) and keeps a sibling path only when a commit `is_claimed_by` this spec changed it or it is uncommitted; tests `a_reopen_leaves_an_earlier_windows_sibling_changes_out` and `another_specs_commit_is_absent` pass.
- AC8 — met: The behavior is in the runtime (`runtime/src/primitives/compute_review_scope.rs`, `runtime/src/primitives/measure_reopenings.rs`, `runtime/src/primitives/check_review_gate.rs`), `framework/reference/review.md` and `framework/constitution.md`, all shipped by a tag; `measure_reopenings` reads only git history of the project it runs in.
- AC9 — met: `runtime/src/main.rs` (`long_about`) and `runtime/src/mcp/server.rs` (`description`) take the one `COMPUTE_REVIEW_SCOPE` constant in `runtime/src/schema/descriptions.rs` (and siblings), the constitution's canonical-source table names it, and every doc comment on `ComputeReviewScopeArgs` and `ComputeReviewScopeResult` in `runtime/src/schema/primitives.rs` was read: `since`, `review_owed` and `scope` now point at the description and the rest (`diff_base`, `base_source`, `head`, counts, `modified_since`, `plan_affected`, `deleted`, `lists_file`) state field shape without a conflicting rule; a clone built from fix2-all.diff renders `--since` as 'the base is the default `compute-review-scope`'s description states'.
- AC10 — met: `AGENTS.md` Workflow bullet 'Weigh what an option starts against what it delivers' (line 49) has each option state its result and the reopens and review runs it starts, and says rejecting one for that cost says which result is given up.
- AC11 — met: `AGENTS.md` bullet 'Local checks follow what a change touches' (line 84) scales checks by blast radius (artifacts only, docs, framework/scripts/CI, runtime) and ends 'Before a tag: every push workflow's steps'; the 'After pushing, read every workflow's run' bullet says 'Never wait on it'.
- AC12 — met: `specs/079-review-and-analysis-work-converges/tasks.md` carries tasks 41 to 43 and `plan.md`/`data-model.md` state the window's changes; `cross-spec-impact:` lists 12 specs; 066's data-model compute-review-scope paragraph now says 'what can move it' and `scope` is what the rule keeps 'with what that rule adds wherever the window starts' and its size sentence is re-measured (450 bytes, `reopen-past-partial-review`); my repeat search of every other spec's data models, scenarios and plans for base-source values, the window-start rule, scope contents, left-open, unread and digest-changed claims found no remaining false claim (020's signpost 'default window starts at the last review of the reopening' stays true in gist, 022's coverage-expansion scenario is a dated rationale).
- AC13 — met: `runtime/src/primitives/brief_re_read.rs` `split_exempt` partitions the round's paths, `run` writes only the non-exempt `paths` to the brief and `round.toml` and `classify` returns `re-read` whenever any non-exempt path remains; test `an_exempt_path_leaves_a_mixed_rounds_brief_and_record` asserts the brief's JSON list and `RoundRecord.paths` omit `spec.md`.
- AC14 — met: `runtime/src/primitives/write_review.rs` renders `fix-rounds-open: true` only when `args.loop_open` and omits it otherwise (test `the_open_mark_follows_the_loop_open_argument` asserts set then cleared); `runtime/src/primitives/check_review_gate.rs` `fix_round_block` blocks on `fix_rounds_open == Some(true)` and a record with no mark falls to the `not_re_read` check or passes; `framework/commands/review.md` step 11 passes `loop-open` and the reference says step 12's last write omits it; `ductus exec` never sets it.
- AC15 — met: `runtime/src/primitives/classify_spec_edit.rs` `run` and `spec_reasons` make a `plan.md` edit a correction and a `spec.md`/`tasks.md` edit reopen only for a changed criterion, open question or added unchecked box (`tasks_reasons`); `framework/constitution.md` Spec lifecycle defines **correction** and lists these; tests `a_task_edit_adding_no_unchecked_task_is_a_correction`, `a_tick_is_a_correction` pass.
- AC16 — met: `AGENTS.md` Workflow bullet 'A change to a shared runtime surface lands in the spec that requires it' (line 80) states landing in the requiring spec with `cross-spec-impact:` holding its `done`; `check_review_gate.rs` `cross_spec_impact_block` enforces the hold.
- AC17 — met: `framework/reference/review.md` ('The rule decides, and nobody is asked, except for a broken contract') and constitution Brownfield Inbox ('Only a gating finding's waiver, deferral or design edit reaches the operator, one at a time') state the routing; `runtime/src/primitives/measure_reopenings.rs` `StoredDecisions::by_person_at` (excludes `RULE_DECIDER` and config waivers) and `OpenPass::drop_carried` (drops a decision another spec's record held with the same key and outcome) implement the count; tests `a_decision_carried_from_another_specs_record_counts_where_it_began` and `a_carried_key_decided_again_with_another_outcome_counts_as_new` assert it.
- AC18 — met: `cat framework/commands/*.md framework/reference/*.md | wc -c` is 482,650 (under 567,705); `wc -c` of `AGENTS.md` is 49,905 and `framework/constitution.md` is 49,990 (under 50,000, the ceilings in `scripts/lint-governing-size.sh`); the references' later growth past the close-time sizes is the owner's recorded decision in the spec's Resolved Questions.
- AC19 — met: `runtime/src/primitives/pipeline_view.rs` `reopenings_line` renders a `Reopenings` item (closed, over the bound, not counted, in-progress specs with no unchecked task) or its own not-measured, shallow, nothing-to-compare words, pushed with `whole(...)` on every run; `runtime/src/primitives/dashboard.rs` carries `reopenings` from `measure_reopenings::counts`; tests `the_reopenings_row_renders_on_every_run_just_before_the_inbox_row` and `the_reopenings_line_says_each_state_in_its_own_words` pass.
- AC20 — met: All boxes in `specs/079-review-and-analysis-work-converges/tasks.md` are ticked (0 unchecked), `measure-reopenings` runs on the release binary and the spec's Resolved Questions record the baseline, the 10-reopening sample and the owner's disposition of the nine later misses (owner, 2026-11-16); the 'next 10 reopenings' outcome is the spec's own recorded decision, not a code path.
- AC22 — met: `runtime/src/primitives/check_review_gate.rs` `run_with_lint` calls `review_half_block` (dispositions, `fix_round_block`, open findings, `compute_review_scope::compute` `review_owed`) before `analyze_half_block`; `paths_owing_a_review` filters `is_project_artifact`, so a path in another spec's directory owes none; test `what_the_gate_holds_until_a_review_runs_is_what_review_owed_reads` passes.
- AC23 — met: `runtime/src/primitives/write_review.rs` `open_findings` lists every counted violation's file and each contract break not fixed, not waived and not `decisions::releases`d, `refuse_an_empty_scope_over_a_finding` refuses an empty-scope write over a record counting a finding (test `an_empty_scope_write_refuses_over_a_record_counting_a_finding` asserts three held cases and no write); `runtime/src/primitives/compute_review_scope.rs` `compute` holds each at-HEAD `open_findings` path via `hold_in_scope`; `runtime/src/primitives/check_review_gate.rs` `review_half_block` blocks via `review_owed_block(&review.open_findings)`, naming them.
- AC24 — met: `runtime/src/primitives/process_waivers.rs` `Pass::place` retains a non-firing waiver when `RunScope::why_unread` says the file is on disk and outside the scope and expires it when the file is gone or in scope; `runtime/src/primitives/process_decisions.rs` `compute` does the same for stored decisions; `runtime/src/primitives/compute_review_scope.rs` `RunScope::why_unread` and `holds`; tests `a_waiver_on_a_file_the_run_did_not_read_is_retained`, `expires_when_file_is_gone`, `a_silent_decision_on_a_file_the_run_read_expires` pass.
- AC25 — met: `runtime/src/primitives/classify_spec_edit.rs` `contract_reasons` reads a scenario or data-model edit as `mechanical` when `only_references_differ` or `SweepIndex::changed_beyond_spelling` is false (`mechanical_sweep.rs` rule: one-for-one lines, same token count, uniform across files), else a reopen reason; `run` returns `Mechanical` when only such edits exist.
- AC26 — met: `runtime/src/primitives/check_artifacts.rs` `check_unticked_criteria` files blocking `review-state-drift` for a `done` spec with an unticked criterion; `runtime/src/primitives/classify_spec_edit.rs` `unticks_a_criterion` makes a taken-back tick a `Criterion` reopen while a tick adds none (tests `an_untick_reopens`, `an_untick_is_read_across_a_label`, `a_tick_is_a_correction`).
- AC21 — met: `framework/constitution.md` Host integration says 'The binary is the source of a primitive's rules' and that without the MCP server a command stops and says to restart or re-run `/ductus`, with a command's reference keeping only what a step does with the result, host judgment and policy; `framework/commands/analyze.md` repeats the stop notice.
- AC27 — met: One test, `runtime/src/primitives/read_tasks.rs` `unchecked_lines` (any heading, outside fences and comments); `runtime/src/primitives/check_review_gate.rs` `unchecked_task_block` blocks `done`; `runtime/src/primitives/check_artifacts.rs` `check_unchecked_boxes` files blocking `review-state-drift` on a `done` spec; `adds_unchecked_box` is used by `classify_spec_edit.rs` `tasks_reasons` and `prune_plan.rs` (added, reworded or unticked reopens, moved or renamed does not; tests `a_box_moved_or_under_a_renamed_task_is_a_correction`); `measure_reopenings.rs` `waiting_features` counts with the same `unchecked_lines`.
- AC28 — met: `runtime/src/primitives/read_tasks.rs` `refuse_while_tasks_are_open` and `without_tasks` (exempts a box under a task the run's deferral names) are called from `runtime/src/primitives/write_review.rs` `refuse_while_implementation_is_open` and `runtime/src/primitives/write_analysis.rs`; `framework/commands/review.md` step 1 and `analyze.md` step 1 stop on an unchecked box (analyze goes on for a `done` spec to report drift); `framework/templates/spec/tasks.md` says to schedule no review or analysis as a task and keep what is owed as prose on the pending task; `check_artifacts.rs` `check_unchecked_boxes` reports the `done` spec's drift.
- AC29 — met: `framework/reference/review.md` Observations paragraph 'Two things a pass notices are no observation' says an edit that would change no claim, rule or behavior is not reported and an unchecked claim is named in the Summary's account of what went unexamined, not under Observations; this is a rule of the review's reading with no code path, so the reference text is the whole implementation.
- AC30 — met: `join_unread` in `runtime/src/primitives/compute_review_scope.rs` joins an unread plan entry only when `is_glob(entry)` (`*`, `?`, `{`) and it does not leave the project, and any other entry, a `dir/` one included, joins only through `hold_in_scope` when it exists on disk; it runs after `paths_owing_a_review` so no entry owes a review, `Carried.unread` and `ReviewEnd::Finished` carry the start record's entries, `write_review.rs` `resolve_scope` carries them into the denominator, and the description reads 'a plan entry that is a glob joining as the entry it is'; with fix2-all.diff applied in a clone `cargo test --lib compute_review_scope` passed 150/150, the updated test asserting `src/planned.rs` and `src/gone/` (never created) stay out and `src/*.rs` joins.

## Observations

- bug: a first review that stopped early still drops unchanged plan entries from the next scope, so what its skipped pass never read stays unread, against the module doc's 'stopping early leaves nothing permanently unread'. — `runtime/src/primitives/compute_review_scope.rs:201` — **waived**: AC6 keeps an unchanged plan entry only on the first review, and that run was the first; what its skipped pass did not read is named in the record's skipped-passes, and restricting a run is the operator's choice. (decided by the rule)
- `cross-spec-impact:` names neither 000 nor 035, though the round edited their scenarios and reopened 035 — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Each is a correction of a claim, which owes no back-link and names this spec in its commit, as `plan.md`'s Cross-spec impact says; AC12's list is of specs whose surface this changes. (decided by the rule)
- `an_open_break_names_its_file` refuses a blank-path break whose deferral releases it, though only a break no deferral releases must name its path — `runtime/src/primitives/write_review.rs` — **waived**: It fails closed, the host passes the path it has, and the condition predates this run's fixes. (decided by the rule)
- bug: an empty diff-base records scope 0 without consulting compute-review-scope, so a spec never in-progress records a first review's examined beside scope 0 — `runtime/src/primitives/write_review.rs` — **waived**: The data model, write-review's description and the owner's task-20 choice (2026-11-07) define an empty diff-base as scope 0, and deriving it instead is a requirement of its own. (decided by the rule)
- other: write-review reports every compute-review-scope error as an invalid diff-base, an unreadable plan included — `runtime/src/primitives/write_review.rs` — **waived**: The message carries the underlying error, and telling a base's errors from the scope's is a change to the error contract no criterion asks for. (decided by the rule)
- other — the Behavior bullet 'A behavior is stated once' now requires existing restatements in a data model or scenario to be swept to pointers (8596f472), but no acceptance criterion tests it: AC9 covers the description, commands and references only, AC12 corrects false claims. A criterion would be one nothing can check, since the spec's own Resolved Question records that nothing detects a new copy. — `specs/079-review-and-analysis-work-converges/spec.md:64` — **waived**: The Resolved Question on the canonical surface states that nothing detects a new copy (§drift-prevention) and that this part relies on review, so no testable criterion exists to add; the sweep was task 36's, and a criterion is a requirement of its own. (decided by the rule)
- perf — the seven commits add 8,176 characters to the compiled descriptions (sum over each changed constant's text between the quotes, before and after each commit: 2,718 + 830 + 1,103 + 1,023 + 742 + 989 + 771), which every schema fetch serves; WRITE_REVIEW alone is 7,169 characters and CHECK_REVIEW_GATE 4,262 at HEAD. — `runtime/src/schema/descriptions.rs` — **waived**: The owner chose moving restatements into the descriptions on 2026-11-12 (spec Resolved Questions), the byte limit of AC18 covers commands and references and not runtime code, and the growth is recorded here with its method. (decided by the rule)
- convention: the list header says the next mutation takes the next free number while `M319` follows `M273` in `scripts/mutation/scope-attribution.py` — `scripts/mutation/scope-attribution.py` — **waived**: Mutation ids are allocated across every list under `scripts/mutation/`, where `M318` is the highest of another list, so `M319` was the next free id; renumbering it to `M274` would reuse a number range other lists hold and gain nothing. (decided by the rule)
- bug: `join_unread` adds an unread plan entry to the scope without checking it exists, so a plan entry naming a plain file that is gone (a planned file never created, one deleted) joins every later review's scope while AC30 says an entry joins 'unless it is gone or leaves the project'; the first loop's `hold_in_scope` skips it, the second loop inserts it. `ductus compute-review-scope` on a fixture (plan naming `src/planned_never_created.rs`, a clean record listing it in `unread`, a later commit) returns scope `[src/b.rs, src/planned_never_created.rs]` at base-source `last-review`. A pattern entry is exempt by design (`a pattern names no path on disk`), a literal path is not a pattern. — `runtime/src/primitives/compute_review_scope.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: mutation M154 in `scripts/mutation/scope-attribution.py` still names the test `compute_review_scope::tests::a_record_the_gate_holds_does_not_move_the_start`, which c55f7222 renamed to `a_record_the_gate_holds_that_states_no_empty_unread_does_not_move_the_start` when it updated M121; `run.py` passes the name to `cargo test` as a filter, a filter matching no test exits 0, so M154 reads SURVIVED against an empty EXPECTED_SURVIVORS. `run.py --check` validates anchors only and reports 0 invalid; a scan of all 275 entries' test names against `fn` definitions finds this one missing. M154's mutant (open-findings no longer held by the gate) by reading, would be killed by `each_hold_on_a_record_that_read_its_scope_starts_at_its_reviewed_against`'s open-findings case (not run). — `scripts/mutation/scope-attribution.py` — **fixed** (decided by the rule)
- convention: `specs/066-audit-records-and-the-completion-gate/data-model.md` states the file form's maximum size as 403 bytes, measured with `base-source` `uncommitted-transition`, 'the longest'; 079 added `reopen-past-partial-review` (26 characters against 22), so the longest value is now another and the figure is stale. Measured at fbdb88c9 with the release binary (`read-size-bytes = 500`, five-character feature, no criterion owing): the compact file-form result is 455 bytes for `reopen-past-partial-review`, against 408 stated; 43 of the 47 bytes are 083's `commits-examined` and `commits-unclaimed`, 4 are 079's longer value. 066 was `done` at `fbdb88c9` and is reopened since by `191954e3`, so correcting it reopens nothing. — `specs/066-audit-records-and-the-completion-gate/data-model.md` — **fixed** (decided by the rule)
- convention: the closing paragraph of `specs/079-review-and-analysis-work-converges/data-model.md` says `scope` plus `excluded` plus `deleted` accounts for `modified-since` 'apart from a plan entry on a first review, a path the last record lists as open, a contract its digest says changed and what a fold brings', and omits the entries a record lists in `unread`, which AC30 (task 42) now joins to the scope wherever the window starts, so a path unchanged in the window can be in `scope` for that reason; `COMPUTE_REVIEW_SCOPE` states it, and task 42's prose sweep did not reach this paragraph. — `specs/079-review-and-analysis-work-converges/data-model.md` — **fixed** (decided by the rule)
- convention: the doc comment on `ComputeReviewScopeResult.scope` in `runtime/src/schema/primitives.rs` restates what the scope holds ('each path the record at `HEAD` leaves open; and what a spec folded into this one below `done` brought') and omits what the held record the window starts at leaves open (AC5) and each `unread` entry of the HEAD record and the start record (AC30); it feeds the schema and `--help`, whose canonical text is the `COMPUTE_REVIEW_SCOPE` description (AC9), so it is a stale copy. Tasks 41 and 42 edited the description, the module doc, the reference and the changelog but not this comment. — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- bug: the first fix of `join_unread` still let a `dir/` plan entry gone from disk join, since `entry_pattern` reads a trailing `/` as a pattern, against AC30's 'unless it is gone'; a shared `is_glob` now admits only a glob, and a `dir/` entry joins only through `hold_in_scope`'s existence check — `runtime/src/primitives/compute_review_scope.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- convention: the run's own fix put `is_glob` between `entry_pattern`'s doc comment and its function, so that comment described `is_glob` and `entry_pattern` had none — `runtime/src/primitives/compute_review_scope.rs` — **fixed** (decided by the rule)
- convention: the `excluded` field's doc comment on `ComputeReviewScopeResult` said the scope leaves out only paths other specs' commits changed, omitting the paths a committed review record credits (spec 081), a narrower copy of the `COMPUTE_REVIEW_SCOPE` description; it now points at the description — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/implement.md`
- `.claude/commands/ductus/review.md`
- `.ductus/reference/implement.md`
- `.pi/prompts/ductus-implement.md`
- `.pi/prompts/ductus-review.md`
- `framework/commands/implement.md`
- `framework/commands/review.md`
- `framework/constitution.md`
- `framework/reference/implement.md`
- `runtime/src/primitives/check_departures.rs`
- `runtime/src/primitives/departures.rs`
- `runtime/src/primitives/mod.rs`
- `runtime/src/primitives/process_waivers.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/primitives/write_review/departure_waivers.rs`
- `runtime/tests/golden/implement-basic.jsonl`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/mcp.rs`
- `scripts/mutation/commit-counts.py`
- `scripts/mutation/departure-waivers.py`
- `scripts/mutation/departures.py`
- `scripts/mutation/freshness-results-have-a-bounded-size.py`
- `scripts/mutation/review-departures-bounded.py`
- `scripts/mutation/waiver-notices-flatten.py`
- `specs/015-tarball-fetch/spec.md`
- `specs/050-constitution/analysis.md`
- `specs/050-constitution/plan.md`
- `specs/050-constitution/review.md`
- `specs/050-constitution/spec.md`
- `specs/050-constitution/tasks.md`
- `specs/073-decided-rule-departures-are-recorded/plan.md`
- `specs/073-decided-rule-departures-are-recorded/spec.md`
- `specs/073-decided-rule-departures-are-recorded/tasks.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/plan.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/spec.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/tasks.md`

## Unexamined governance

*None.*
