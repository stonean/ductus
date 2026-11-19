---
spec: 079-review-and-analysis-work-converges
last-run: 2026-11-19T02:54:34Z
reviewed-against: 92914fe917c04c25847aa280a39e55ce62474caf
diff-base: 24d3a8f4ca50e3c6d0e483bce82dbaafa157a171
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 1
scope: 3
unread:
  - runtime/src/primitives/derive_routing_candidates.rs
  - specs/079-review-and-analysis-work-converges/plan.md
excluded: 103
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 1413afb3cec6f554723b56bcc6e190fc2140fe7ab52407e09e6f27984a75ba7f
criteria-judged: 29
criteria-met: 29
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
    digest: aacbf00568e5751c5cd4f592d97ccad745b23a85546e78bfde974a7abfb881f7
  - label: AC6
    digest: 542430983be9de71f136431c8959d9a408093015a5b755257d0ff330185658dd
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
    digest: 69a092756cf6297061bbb8cb35aa39439c8a907098bbcfd3d867547e2cbfb545
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
blocking: false
dispositions:
  fixed: 5
  deferred: 0
  waived: 2
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
  - key: "convention — two AGENTS.md rules added in this window fail §drift-prevention's routing test or its measurement rule. 'Run a spec's pre-done verification after its review and before its analysis' (4d681fad) holds for every project running the pipeline, since a criterion correction edits spec.md and stales an analysis wherever the pipeline runs, yet framework/reference/implement.md steps 11 to 14 say nothing of the order, so adopters never get it. 'A docs/ commit naming no spec owes every open spec a review' (2604a6f9) is something check-review-gate's review-owed message already reports. The first also gives '687K subagent tokens' with no method; its commit message holds it (the five agents' token totals). Checked: AGENTS.md is 49,932 bytes and lint-governing-size passes; the other added rules (symlinked node_modules, files for write-review arguments, the owner's recommendation and closing preferences, the subagent Write trap) name this project's or the owner's practice, and --observations-json and the other --{name}-json flags exist on write-review. — `AGENTS.md:53`"
    outcome: waived
    reason: Moving the ordering rule into framework/reference/implement.md ships new behavior guidance to adopters, which is a requirement of its own and not a chore; the review-owed line is a pointer to a check and costs the file nothing against its ceiling.
    decided-at: 2026-11-16T16:32:26Z
    decided-by: rule
  - key: "other — the runner now reads a step whose env: holds a ${{ }} expression as SKIP, so framework-checks.yml's and generators.yml's Build the runtime step, whose env: carries CACHE_HIT, no longer runs locally, and a local run checks the binary already built — `scripts/run-workflow-steps.py`"
    outcome: waived
    reason: "docs/developing-ductus.md step 2 runs cargo build --release --locked before the workflows' steps, and the SKIP is reported with its reason, never as a pass; running a step without a value CI gives its env: is what the fix removed."
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
---

# Review — 079-review-and-analysis-work-converges

## Summary

Window 24d3a8f4 (base-source reopen, 066 task 27b) to 92914fe9. The window held one artifact, spec.md (92,854 bytes), read whole by the passes; the other two scope entries are this run's own fixes (plan.md and runtime/src/primitives/derive_routing_candidates.rs), uncommitted when written, which the passes did not read and the fix round re-reads, so examined is 1 of 3 and unread names those two. The nine backend and cross rule files were read whole (285,795 bytes) and all five passes ran: no finding. The criteria step judged all 29 criteria one by one against the tree and history, in two halves by body order, with the named tests run (cargo test --lib 3372 passed, and the integration suites the criteria cite) and measure-reopenings, render-status and classify-spec-edit driven on fixtures: all 29 met. Code that 048 changed after the judging (extensions.rs doc comments, release workflows, the bootstrap's Windows rows) touches no criterion. Five observations were fixed by rule: four stale claims in spec.md (the owes-no-review Edge Case before 081 and 066, the pipeline view's home before 080, a pruned task 36 pointer, mechanical_sweep.rs and a retired Family 19 named where classify-spec-edit and analyze_subjects.rs decide), and the Resolved Question's promised correction of derive-routing-candidates' doc comment, which the plan never made and this run made. Two stored decisions on spec.md still stand and are matched; 36 are retained, each keyed to a file outside the scope. Unexamined: the spec's dated measurements other than byte counts (commit, run, reopening, token and CI-run counts), which were not re-derived.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

29 of 29 acceptance criteria judged met.

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

- AC1 — met: `measure-reopenings` (`runtime/src/primitives/measure_reopenings.rs`; `--help`) reports over-bound reopenings, `not-counted` and `waiting`; run on HEAD it returned state measured and on a scratch adopter repo; `tests/measure_reopenings.rs` 4 passed; the first 10 closings after `e7fcde37` end at `9857b89b`, 2 over the bound (032, 030) each with a task here (21, 23, `3fb79512`).
- AC4 — met: Historical shas hold (`b23b020c` review_owed with no record, `5f67ede2` `holds_done_until_reviewed`, `6aef0472` stale digest, `68b643c8` `open_findings`); current `owes_a_review` in `compute_review_scope.rs` reads gating record or owing paths or `criteria_owing`, tests `a_reopening_that_changed_only_text_owes_no_review`, `a_stale_digest_puts_its_contract_in_scope_and_owes_no_review`, `an_artifact_only_window_owes_a_review_while_a_criterion_is_unjudged` pass.
- AC2 — met: `framework/commands/review.md` step 10 and `analyze.md` step 19 apply the by-rule disposition (chore fixed, else waived, only a contract-break or hard-fail/blocking deferral reaches `gate-confirm`); `b307b7aa` added the person-overrules-by-deferring text to `framework/reference/review.md`.
- AC3 — met: `compute_review_scope.rs` moves paths only another spec's commit changed to `excluded` and `write_review.rs` renders them under `## Excluded from scope` (`the_record_names_each_excluded_path_by_the_spec_that_claimed_it`); `fix_rounds.rs` `changed_only_by_other_specs` with `brief_re_read` tests `only_another_specs_commit_leaves_the_round_with_nothing_to_re_read` pass; `cargo test --lib` 3372 passed, `--test fix_rounds` 3 passed.
- AC5 — met: `compute_review_scope.rs` tests `a_review_in_the_reopening_moves_the_start_to_its_commit`, `a_stopped_run_falls_back_to_the_last_finished_one`, `a_record_the_gate_holds_does_not_move_the_start`, `a_run_that_stopped_early_or_does_not_parse_does_not_move_the_start`, `since_moves_only_the_start_and_the_scope_rule_still_applies`, `a_review_older_than_the_reopen_leaves_the_start_at_the_reopen` all pass.
- AC6 — met: `commit_attribution.rs` (`7daae060`) claims by directory or subject scope, a merge claims nothing and contributes only its own paths (`a_merge_contributes_only_what_it_changed_itself`); scope tests for several-claimant paths, left-open paths, changed contracts, folds below done, first-review-only plan entries and 081's `a_spec_claiming_a_named_commit_only_by_its_directory_reviews_only_that_directory` pass.
- AC7 — met: `diff_cross_spec.rs` starts at the reopening's parent (`window-base`, `--help`) and keeps only commits belonging to the spec: tests `the_window_starts_at_the_transition_parent`, `another_specs_commit_is_absent`, `a_commit_belonging_to_the_spec_counts_as_its_impact`, `a_reopen_leaves_an_earlier_windows_sibling_changes_out` pass.
- AC8 — met: The runtime primitives `measure-reopenings` and `classify-spec-edit` are in `framework/runtime-tools.txt`, changes sit in `framework/commands/`, `framework/reference/`, `framework/constitution.md`; `measure-reopenings` run on a scratch non-ductus git repo with only `specs/` and `.ductus/config.toml` returned a measured reopening, and a subdirectory-project test passes; AGENTS.md-only entries (AC10, AC11, AC16) are maintainer workflow, not pipeline behavior.
- AC9 — met: `runtime/src/schema/descriptions.rs` holds 74 consts (73 served tools plus the CLI-only one), `mcp/server.rs` uses `description = descriptions::X` 73 times and `main.rs` sets `long_about` from them; `tests/mcp.rs` `each_served_tools_description_is_its_subcommands_long_about` and `no_tool_spells_its_description_as_a_literal` pass (mcp 55 passed); constitution table row 'A primitive's behavior' names the compiled description.
- AC10 — met: `AGENTS.md` bullet 'Weigh what an option starts against what it delivers' states each option's result and the reopens and review runs it starts, and that rejecting one for that cost says which result is given up.
- AC11 — met: `AGENTS.md` bullet 'Local checks follow what a change touches' scales checks by blast radius and ends 'Before a tag: every push workflow's steps'; bullet 'After pushing, read every workflow's run' says never wait on it.
- AC12 — met: `spec.md` frontmatter `cross-spec-impact:` lists the 12 specs equal to the table in `plan.md` Cross-spec impact, each of which holds a back-link to 079 (grep of spec.md, plan.md and scenarios); corrected claims in 000, 014, 021, 023, 030 etc. carry signposts back to 079.
- AC13 — met: `brief_re_read.rs` test `an_exempt_path_leaves_a_mixed_rounds_brief_and_record` and `a_spec_body_edit_with_code_needs_a_re_read` pass; `classify` drops exempt paths per path so the re-read reads only non-exempt paths.
- AC14 — met: `write_review` takes `loop-open` (`framework/commands/review.md` step 11 passes it, step 12's last write omits it); tests `the_open_mark_follows_the_loop_open_argument`, `the_write_clearing_the_open_mark_finishes_the_run`, `an_open_fix_round_loop_holds_done`, `a_record_with_no_mark_is_unaffected`, `a_record_without_the_loop_fields_passes_and_says_so` pass.
- AC15 — met: Fixture (done spec, committed): a `plan.md` or `spec.md` prose edit returned `{"kind":"correction"}`, a reworded criterion `reopens` [criterion], an added `data-model.md` `reopens`; `framework/constitution.md` §spec-lifecycle says a correction 'edits `spec.md`, `plan.md` or `tasks.md` and changes none of those'.
- AC16 — met: `AGENTS.md` bullet 'A change to a shared runtime surface lands in the spec that requires it, and `cross-spec-impact:` holds that spec's `done` until the surface's owner links back' present; enforcement is `cross_spec_impact_block` in `runtime/src/primitives/check_review_gate.rs`.
- AC17 — met: `framework/constitution.md` §brownfield-inbox says only a gating finding's waiver or deferral reaches the operator, one at a time, other observations rule-decided and recorded as decided by the rule (`framework/reference/analyze.md` §Finding dispositions); `measure-reopenings` returns `decisions-by-person` per reopening and per run, counted only where a key is first held, proven by `a_decision_carried_from_another_specs_record_counts_where_it_began` (`cargo test --release --lib measure_reopenings`: 54 passed).
- AC18 — met: `git cat-file -s` sums over `framework/commands/` plus `framework/reference/`: 431,103 at close `9064b7be` and 477,574 now, both under 567,705 (93f63a1c = 567,705); `AGENTS.md` 49,793 and the constitution 48,763 pass `scripts/lint-governing-size.sh` (exit 0); `review.md` 46,394 and `analyze.md` 49,777 at `9064b7be` fit read size 50,000, and the later growth is the annotated allowance (Resolved Question on AC18).
- AC19 — met: `reopenings_line` in `runtime/src/primitives/pipeline_view.rs`; `ductus render-status` prints 'Reopenings since 2026-10-20: 167 closed, 38 over the review and analysis bound, 0 not counted · 7 in-progress spec(s) with no unchecked task'.
- AC20 — met: Tasks 0 unchecked (`specs/079-review-and-analysis-work-converges/tasks.md`); `measure-reopenings --since 2026-10-01 --at d62e72f3` reads closed-count 126, over-bound-count 28, runs 261, the recorded baseline; at `9064b7be` the over-bound closings from 2026-11-05 on (015, 029, 030, 032, 045, 046, 061, 071, 078, 079, 080, 081) have miss tasks 21 to 33 in tasks.md at that commit, 071, 046 and 080 included, and the later misses without a task are the owner's call per the Resolved Question on `done` (owner, 2026-11-16).
- AC22 — met: `check_review_gate::run` calls `review_half_block` (checks 4 to 10, incl. `review-owed` via `compute_review_scope::compute`) before `analyze_half_block`; tests `what_only_a_review_clears_outranks_a_stale_analysis` and `a_path_in_another_specs_directory_owes_no_review` pass (check_review_gate 96 passed, compute_review_scope 134 passed).
- AC23 — met: `open_findings` in `runtime/src/primitives/write_review.rs` lists each counted violation's file and each break not fixed, waived or released; passing tests `a_path_left_open_comes_back_from_excluded`, `a_path_left_open_comes_from_head_and_is_named_as_owing_a_review`, `a_finding_left_open_blocks_and_names_its_file` and `an_empty_scope_write_refuses_over_a_record_counting_a_finding` (11 passed).
- AC24 — met: `process_decisions` and `process_waivers` retain on a file on disk the scope does not hold and expire on one the scope holds or one gone: `a_silent_decision_on_a_file_the_run_did_not_read_is_retained`, `a_silent_decision_on_a_file_the_run_read_expires`, `a_silent_decision_on_a_file_gone_from_disk_expires`, `a_waiver_on_a_file_the_run_did_not_read_is_retained` (32 and 24 passed); review step 8 and 9 pass `since`.
- AC25 — met: `classify_spec_edit` tests `a_token_swept_across_files_is_mechanical`, `a_rewritten_reference_is_mechanical`, `a_reference_added_or_removed_as_a_list_item_is_mechanical` pass (65 passed, `cargo test --release --lib classify_spec_edit`); `a_real_contract_edit_beside_a_sweep_reopens` covers the converse.
- AC26 — met: `check_artifacts` tests `a_done_spec_with_an_unticked_criterion_has_drifted` (family `review-state-drift`, Blocking) and `a_done_spec_with_every_criterion_ticked_has_not` pass; `classify_spec_edit` tests `an_untick_reopens` and `a_tick_is_a_correction` pass.
- AC21 — met: `framework/constitution.md` §runtime-boundary Host integration: 'The binary is the source of a primitive's rules', the server is required and without it 'stop and say to restart or re-run `/ductus`', the reference keeps what a step does with the result, host judgment and policy; the canonical-source table row names `ductus <name> --help`; `framework/commands/review.md` pointer repeats it (`bbff7a53`).
- AC27 — met: One text test `unchecked_lines`/`describe_unchecked`/`adds_unchecked_box` in `runtime/src/primitives/read_tasks.rs` is used by `check_review_gate` (`an_unchecked_box_blocks_wherever_it_sits`), `check_artifacts` (`a_done_spec_holding_an_unchecked_box_is_review_state_drift`), `classify_spec_edit`, `prune_plan` (`moving_or_reindenting_an_unchecked_box_does_not_reopen`) and `measure_reopenings` (`waiting_is_the_in_progress_specs_with_no_unchecked_task`); those suites pass.
- AC28 — met: `refuse_while_tasks_are_open` in `runtime/src/primitives/read_tasks.rs` is called by `write_review` and `write_analysis` with `tasks_deferred_here` (test `a_runs_own_deferral_opens_no_refusal_but_other_work_does` passes); `framework/commands/review.md` and `analyze.md` step 1 stop naming the boxes, analyze going on for a `done` spec (step 22 report/`--fix`); the exception text landed in `29ec6cc2` (`git show` has `deferred_here` and the review.md deferral clause).
- AC29 — met: `framework/reference/review.md` Observations section 'Two things a pass notices are no observation': a no-claim edit is not reported and an unre-checkable claim for want of a fetched source is named in the Summary's unexamined account, not under Observations; delivered by `80f6012d` (task 24).

## Observations

- convention — the Edge Case 'A reopening that owes no review' states the test as it stood before 081 and 066's task 27: a path outside a spec's directory or an edit to its scenarios or data model, whose digest no gate compares now — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
- convention — the Resolved Question on /status's reference says `dashboard` renders the pipeline view and its description names `rendered-markdown`, though 080 moved the view to `render-status` — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
- convention — the Behavior bullet 'A behavior is stated once' and plan.md's Sweeping restatements entry point at task 36, which landed and was pruned from tasks.md — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
- convention — a Resolved Question says the plan corrects `derive-routing-candidates`' doc comment, which described the old rule; the plan never did, and the comment still called routing runtime work to the runtime-owning spec a rule — `runtime/src/primitives/derive_routing_candidates.rs` — **fixed** (decided by the rule)
- convention — the Resolved Question on what reopens a `done` spec says `mechanical_sweep.rs` is where the gate and Family 19 classify an edit, though `classify-spec-edit` classifies it and Family 19 is retired — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
- `cross-spec-impact:` names neither 000 nor 035, though the round edited their scenarios and reopened 035 — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Each is a correction of a claim, which owes no back-link and names this spec in its commit, as `plan.md`'s Cross-spec impact says; AC12's list is of specs whose surface this changes. (decided by the rule)
- other — the Behavior bullet 'A behavior is stated once' now requires existing restatements in a data model or scenario to be swept to pointers (8596f472), but no acceptance criterion tests it: AC9 covers the description, commands and references only, AC12 corrects false claims. A criterion would be one nothing can check, since the spec's own Resolved Question records that nothing detects a new copy. — `specs/079-review-and-analysis-work-converges/spec.md:64` — **waived**: The Resolved Question on the canonical surface states that nothing detects a new copy (§drift-prevention) and that this part relies on review, so no testable criterion exists to add; the sweep was task 36's, and a criterion is a requirement of its own. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/amend.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `.claude/commands/ductus/implement.md` — 066-audit-records-and-the-completion-gate
- `.claude/commands/ductus/review.md` — 066-audit-records-and-the-completion-gate
- `.ductus/reference/audit.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `.ductus/reference/implement.md` — 066-audit-records-and-the-completion-gate
- `.ductus/reference/review.md` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `.github/workflows/runtime-acquisition.yml` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `.github/workflows/runtime-release.yml` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `.github/workflows/runtime.yml` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `.pi/prompts/ductus-amend.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `.pi/prompts/ductus-implement.md` — 066-audit-records-and-the-completion-gate
- `.pi/prompts/ductus-review.md` — 066-audit-records-and-the-completion-gate
- `AGENTS.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `README.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `docs/developing-ductus.md` — 062-concurrent-session-targets, 066-audit-records-and-the-completion-gate
- `docs/runtime.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `docs/slash-commands.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/bootstrap/ductus-procedure.md` — 027-bootstrap-migration-registry
- `framework/bootstrap/ductus.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime, 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `framework/bootstrap/govern.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime, 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `framework/commands/amend.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `framework/commands/implement.md` — 066-audit-records-and-the-completion-gate
- `framework/commands/review.md` — 066-audit-records-and-the-completion-gate
- `framework/constitution.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/migrations.toml` — 027-bootstrap-migration-registry
- `framework/reference/audit.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/reference/implement.md` — 066-audit-records-and-the-completion-gate
- `framework/reference/review.md` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `runtime/CHANGELOG.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 073-decided-rule-departures-are-recorded
- `runtime/README.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `runtime/src/interpreter/mod.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/interpreter/payload.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/check_review_gate.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `runtime/src/primitives/compute_review_scope.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `runtime/src/primitives/criterion_verdicts.rs` — 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `runtime/src/primitives/line_citations.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/write_review.rs` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `runtime/src/schema/descriptions.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `runtime/src/schema/extensions.rs` — 022-deterministic-runtime, 048-govern-acquired-runtime, 066-audit-records-and-the-completion-gate
- `runtime/src/schema/primitives.rs` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `runtime/tests/fixtures/implement-basic/.govern.session.toml` — 066-audit-records-and-the-completion-gate
- `runtime/tests/fixtures/implement-basic/stdin.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/fixtures/review-basic/stdin.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/golden/implement-basic.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/golden/review-basic.jsonl` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `runtime/tests/mcp.rs` — 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `runtime/tests/review_command.rs` — 066-audit-records-and-the-completion-gate
- `scripts/audit/README.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `scripts/audit/done-spec-criteria.sh` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `scripts/audit/migration-coverage.sh` — 027-bootstrap-migration-registry
- `scripts/mutation/criteria-in-review-066.py` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `scripts/mutation/fix-round-record.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/line-citation-strip-066.py` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `scripts/mutation/reopen-measure.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/reviews-follow-what-changed.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/scope-attribution.py` — 066-audit-records-and-the-completion-gate
- `scripts/tests/test-migration-coverage.sh` — 027-bootstrap-migration-registry
- `specs/000-slash-commands/analysis.md` — 000-slash-commands
- `specs/000-slash-commands/plan.md` — 000-slash-commands
- `specs/000-slash-commands/review.md` — 000-slash-commands
- `specs/000-slash-commands/scenarios/criterion-route-after-draft.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/000-slash-commands/scenarios/validation-gates.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/000-slash-commands/spec.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/015-tarball-fetch/plan.md` — 015-tarball-fetch
- `specs/020-code-review/analysis.md` — 020-code-review
- `specs/020-code-review/data-model.md` — 020-code-review, 022-deterministic-runtime
- `specs/020-code-review/plan.md` — 020-code-review
- `specs/020-code-review/review.md` — 020-code-review, 022-deterministic-runtime
- `specs/020-code-review/spec.md` — 020-code-review, 022-deterministic-runtime
- `specs/022-deterministic-runtime/data-model.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/022-deterministic-runtime/plan.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/022-deterministic-runtime/scenarios/govern-bootstrap.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/022-deterministic-runtime/scenarios/implement-completion-gate.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md` — 020-code-review, 022-deterministic-runtime
- `specs/022-deterministic-runtime/spec.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/027-bootstrap-migration-registry/analysis.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/plan.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/review.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/a-projects-own-constitution-is-not-relocated.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/migration-chain-reference-integrity.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/migrations-apply-once-per-repo.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/spec.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/tasks.md` — 027-bootstrap-migration-registry, 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/analysis.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/plan.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/review.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/spec.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/tasks.md` — 027-bootstrap-migration-registry, 029-bootstrap-runtime-autowire
- `specs/048-govern-acquired-runtime/data-model.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/plan.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/scenarios/state-b-continues-in-session.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/spec.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/tasks.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/analysis.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `specs/066-audit-records-and-the-completion-gate/plan.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `specs/066-audit-records-and-the-completion-gate/review.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/scenarios/criteria-are-verified-in-review.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/spec.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `specs/073-decided-rule-departures-are-recorded/plan.md` — 073-decided-rule-departures-are-recorded
- `specs/081-reviews-and-checks-follow-what-changed/spec.md` — 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed

## Unexamined governance

*None.*
