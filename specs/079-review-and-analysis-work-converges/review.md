---
spec: 079-review-and-analysis-work-converges
last-run: 2026-11-20T00:32:55Z
reviewed-against: b65dcc75d05c4fe18d1eb3ee8c5aa40d2934ef3b
diff-base: ac98cc81ccb20ffc3e928f161c9131ae24c51289
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 11
scope: 16
unread:
  - runtime/CHANGELOG.md
  - runtime/src/schema/descriptions.rs
  - runtime/src/schema/primitives.rs
  - specs/079-review-and-analysis-work-converges/plan.md
  - specs/079-review-and-analysis-work-converges/spec.md
excluded: 86
contract-breaks: 0
skipped-passes: []
fix-rounds: 2
re-reads: 2
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 06298383706eaad589c60381deb6ec991aa75d1ee934006eeb8ff1d792290580
  scenarios/the-excluded-section-lists-paths-only.md: 1763a8c3a9dd478a6f39d2813a73d4804c21dc8e9581c401af3fdaf35e6ea8b1
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
  fixed: 8
  deferred: 0
  waived: 1
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
  - key: "convention: the list header says the next mutation takes the next free number while `M319` follows `M273` in `scripts/mutation/scope-attribution.py` — `scripts/mutation/scope-attribution.py`"
    outcome: waived
    reason: Mutation ids are allocated across every list under `scripts/mutation/`, where `M318` is the highest of another list, so `M319` was the next free id; renumbering it to `M274` would reuse a number range other lists hold and gain nothing.
    decided-at: 2026-11-20T00:32:48Z
    decided-by: rule
---

# Review — 079-review-and-analysis-work-converges

## Summary

Window `ac98cc81`..HEAD, base-source reopen, as `compute-review-scope` resolved it (14 entries before this run's fixes, 16 with the two the fixes touched). Four readers each read their part: A the three `security-*.md` files whole (194,749 bytes) and `slice-code.diff` and `slice-docs.diff` whole; B the nine other rule files whole (213,777 bytes), `AGENTS.md` lines 123 to 215 and `specs/system.md`, all four diff slices whole (`slice-code.diff`, `slice-docs.diff`, the `descriptions.rs` word diff, the `runtime/CHANGELOG.md` diff) and a verdict on each of the 38 stored decisions; C1 and C2 the criteria step over all 29 criteria. No MUST or SHOULD violation; 9 observations, 8 fixed in the run and 1 waived by the rule. The fix round re-read twice, both clean. Unread, named in `unread`: `runtime/CHANGELOG.md` (the window diff read whole, other specs' hunks not checked beyond it), `runtime/src/schema/descriptions.rs` (word diff whole and the two constants this spec owns), `runtime/src/schema/primitives.rs` (the changed doc read as a diff), and `plan.md` and `spec.md` of this spec (`spec.md` read whole to the end of its acceptance criteria and in the changed lines; its Resolved Questions beyond the cited ones were not read). `compute_review_scope.rs` and `write_review.rs` were read as the whole window diff plus the changed functions, not whole files, and are counted examined on that basis. Stored decisions: 7 hold and carry (the ones naming `compute_review_scope.rs`, `write_review.rs`, `descriptions.rs` and this spec's `spec.md`), 31 name a path outside the scope and are retained. AC6 was edited this run and judged met with its current text; criteria judged are all 29 of 29.

2 fix round(s): 2 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

- AC1 — met: `measure_reopenings.rs` `over_bound`/`review_fields`/`analysis_fields` implement the bound (second run over unless the previous left a gating count, open fix-round mark, budget stop or skipped pass) and `descriptions.rs` MEASURE_REOPENINGS states not-counted (no-recorded-run, record-unparseable, no-in-progress-commit) and waiting-count; tests a_second_clean_run_of_either_kind_is_over_the_bound, a_reopening_with_no_run_is_not_counted; the 10-reopening sample and per-miss tasks were not re-measured here.
- AC4 — met: `compute_review_scope.rs` `owes_a_review`/`paths_owing_a_review` (code only via `is_project_artifact`; no run, `holds_done_until_reviewed`, left-open paths and `criteria_owing` owe one) match the criterion and all three Superseded sentences; tests a_reopening_that_changed_only_text_owes_no_review, a_stale_digest_puts_its_contract_in_scope_and_owes_no_review, a_record_the_gate_holds_owes_a_review_whatever_the_window_holds, a_review_is_owed_with_no_record_whatever_the_window_holds.
- AC2 — met: `framework/commands/review.md` step 10 and `framework/reference/review.md` Observations: rule decides, chore fixed, else waived with reason, only a `contract-break` observation is deferred (gate-confirm), person's deferral overrules; matches the Superseded-in-part sentence.
- AC3 — met: `compute_review_scope.rs` `split_changes` excludes a path only when `window.others` holds it and neither `window.own` nor uncommitted does, `fix_rounds.rs` `round_paths`/`changed_only_by_other_specs` does the same per round, and `write_review.rs` `render_excluded` lists every excluded path and round-excluded path (tests another_specs_commit_leaves_the_scope_and_is_listed_in_excluded, the_record_names_each_excluded_path, an_excluded_path_is_listed_without_the_specs_that_claimed_it, fix-round `left out of fix round 2` test); the paths-only refactor still names every path.
- AC5 — met: `compute_review_scope.rs` `resolve_base`/`last_review_commit` start at the later of the reopen parent and the last finished review commit, `recorded_run.stopped_early` (budget stop, skipped pass, `holds_done_until_reviewed`) keeps the start, `--since` is used verbatim; tests a_run_that_stopped_early_or_does_not_parse_does_not_move_the_start, a_record_the_gate_holds_does_not_move_the_start, since_is_used_verbatim_with_no_parent_walk.
- AC6 — met: `compute_review_scope.rs` `attribute_window` (alone/no-spec take all, several-claimant rule, 081 named-spec rule), `compute` (uncommitted, `open-findings`, digest contracts, `folded_in`, plan files only with no run at HEAD), `commit_attribution.rs` merge handling, and the 081 sentence via `take_unclaimed`/`ReadRecord::read`/`read_records` (window holds commit, `unread` lacks path, no skipped pass/budget stop/open loop) all hold post-refactor; test a_path_a_complete_record_read_is_credited_and_owes_no_review asserts not in scope, in `excluded`, `!review_owed`; the COMPUTE_REVIEW_SCOPE description agrees.
- AC7 — met: `diff_cross_spec.rs` `window_base` uses the parent of the in-progress commit via `transition_parent` and `compute` keeps only paths of commits where `claims.is_claimed_by(form)` (or uncommitted); tests the_window_starts_at_the_transition_parent, a_reopen_leaves_an_earlier_windows_sibling_changes_out, another_specs_commit_is_absent.
- AC8 — met: The behavior lives in runtime primitives (`measure-reopenings` in `framework/runtime-tools.txt`, spec root read via `Paths::load` so it measures an adopter's project), `framework/commands/review.md` and `framework/reference/review.md`; `runtime/CHANGELOG.md` carries 079's entries (44 mentions); the `AGENTS.md`-only rules (AC10, AC11, AC16) are the criterion's own exceptions.
- AC9 — met: `runtime/src/main.rs` builds `about`/`long_about` and `runtime/src/mcp/server.rs` the MCP `description` from the same `descriptions::` constant (MEASURE_REOPENINGS checked), `framework/commands/review.md` step 1 cites `compute-review-scope`'s description, and `framework/constitution.md` Drift Prevention table names `runtime/src/schema/descriptions.rs`.
- AC10 — met: `AGENTS.md` bullet 'Weigh what an option starts against what it delivers' says each option states its result and the reopens and review runs it starts and that rejecting one says which result is given up.
- AC11 — met: `AGENTS.md` 'Local checks follow what a change touches' scales checks by blast radius with 'Before a tag: every push workflow's steps', and 'After pushing...Never wait on it' reads CI without waiting; `scripts/run-workflow-steps.py` exists.
- AC12 — met: 079 has `plan.md`, `tasks.md` (tasks 1-40) and a 12-entry `cross-spec-impact:`; the paths-only change's cross-spec claim was corrected in 081's AC13 (now 'listed in `excluded`', 081 `done`), and a grep of 066, 058 and 079 artifacts finds no remaining claim that `excluded` names claimants.
- AC13 — met: `brief_re_read.rs` `split_exempt` removes exempt paths before `classify` and the brief renders and records only `paths`; test an_exempt_path_leaves_a_mixed_rounds_brief_and_record.
- AC14 — met: `write_review.rs` writes `fix-rounds-open: true` only when `args.loop_open` (command steps 11 and 12 pass it on the first write and omit it on the last), `check_review_gate.rs` `holds_done_until_reviewed` and `fix_round_block` block on `fix_rounds_open == Some(true)`; tests the_open_mark_follows_the_loop_open_argument, a_record_with_no_mark_is_unaffected.
- AC15 — met: `classify_spec_edit.rs` reads spec.md/plan.md edits that add or change no criterion, question or unchecked task as a correction (tests spec_body_prose_is_a_correction, a_plan_edit_is_a_correction, a_task_edit_adding_no_unchecked_task_is_a_correction) and `framework/constitution.md` Spec lifecycle says a correction takes no back-edge.
- AC16 — met: AGENTS.md Workflow bullet 'A change to a shared runtime surface lands in the spec that requires it, and `cross-spec-impact:` holds that spec's `done` until the surface's owner links back' (022/066/067 owners) is present verbatim in substance.
- AC17 — met: Only a gating deferral reaches the operator via gate-confirm (framework/commands/review.md step 10, analyze.md step 19, constitution 'Finding dispositions'); measure_reopenings.rs `drop_carried` takes same-key-and-outcome decisions held by another spec's record at the first write's parents, proven by `a_decision_carried_from_another_specs_record_counts_where_it_began` (asserts [(FEATURE,1),(013-b,1)]) and the changed-outcome and keyless tests.
- AC18 — met: wc -c at HEAD: commands+reference (30 files, git ls-tree and cat agree) 465,917 < 567,705 (93f63a1c: 276,548+291,157=567,705 exactly); AGENTS.md 49,793 and constitution 49,021 < 50,000 (scripts/lint-governing-size.sh silent); review.md 46,394 and analyze.md 49,777 at 9064b7be (re-measured with git show|wc -c, 2026-11-11 close) under read-size 50,000; now 55,547 and 63,924, which RQ 'Does AC18 hold each reference...' allows.
- AC19 — met: pipeline_view.rs renders 'Reopenings since ... closed, N over the bound, N not counted · N in-progress spec(s) with no unchecked task' and the shallow/none-closed/not-measured states (tests ~1722-1745), render_status.rs `the_reopenings_row_renders_on_every_run_just_before_the_inbox_row`; `waiting_features` in measure_reopenings.rs counts in-progress specs with no unchecked line.
- AC20 — met: Tasks: tasks.md holds no unchecked box at HEAD (tasks 37-40 all [x]); baseline: RQ 'What outcome...' records 126 closings/261 runs at d62e72f3 and later re-measures at fc20a939 and d6594496 (126 closings); sample: spec went done at 9064b7be (2026-11-11) after per-closing records (git log 'tasks(079): ...closings in the closing order's record', misses as tasks 29/31/33/34) and RQ 'What does this spec's own done require?' scopes AC1 to that sample with the owner's 2026-11-16 decision on nine later misses; I did not re-run measure-reopenings (no cargo/MCP) so the count of 10 rests on those records.
- AC22 — met: check_review_gate.rs `run_with_lint` calls `review_half_block` (disposition, fix-round, open-findings, `review-owed`) before `analyze_half_block`; tests `a_missing_review_is_reported_before_a_missing_analysis`, `a_contract_change_since_the_review_holds_done_only_on_the_analysis` (AnalyzeStale then ReviewOwed naming src/code.rs) and compute_review_scope `a_path_in_another_specs_directory_owes_no_review`/`another_specs_deletion_owes_this_spec_no_review`.
- AC23 — met: write_review.rs `open_findings` lists each counted violation's file plus unfixed/unwaived/unreleased contract breaks (path_without_lines); compute_review_scope.rs reads `at_head.open_findings` into `hold_in_scope` wherever the window starts (tests `a_path_left_open_comes_back_from_excluded`, `..._comes_from_head_and_is_named_as_owing_a_review`); check_review_gate `review_half_block` blocks ReviewOwed naming it (`a_finding_left_open_blocks_and_names_its_file`); `refuse_an_empty_scope_over_a_finding` refuses over counted findings or open-findings.
- AC24 — met: process_decisions.rs and process_waivers.rs retain a silent decision/waiver on a file still on disk the scope does not hold and expire it when the scope holds it or it is gone (tests `a_silent_decision_on_a_file_the_run_did_not_read_is_retained`, `..._the_run_read_expires`, `..._gone_from_disk_expires`, `a_scope_that_cannot_be_read_retains_and_says_so`, process_waivers `a_waiver_on_a_file_the_run_did_not_read_is_retained`).
- AC25 — met: classify_spec_edit.rs `contract_reasons`/`only_references_differ` return Mechanical for a scenario or data-model edit that is a `SweepIndex` (mechanical_sweep) sweep or only a cross-service reference edit; tests `a_token_swept_across_files_is_mechanical` (scenario+data-model), `a_token_changed_in_one_file_reopens`, `a_rewritten_reference_is_mechanical`, `a_reference_added_or_removed_as_a_list_item_is_mechanical`; kind Mechanical is the non-reopening class in constitution 'Spec lifecycle'.
- AC26 — met: check_artifacts.rs `check_unticked_criteria` emits blocking review-state-drift for a done spec (test `a_done_spec_with_an_unticked_criterion_has_drifted`, names 'AC2, three'); classify_spec_edit tests `an_untick_reopens`, `an_untick_is_read_across_a_label` (Reopens/Criterion) and `a_tick_is_a_correction`.
- AC21 — met: framework/constitution.md Runtime Boundary 'Host integration' states 'The binary is the source of a primitive's rules', requires the server ('without it, stop and say to restart or re-run `/ductus`'), keeps in the reference only what a step does with the result, host judgment and policy; Drift Prevention canonical-source table names the compiled description / `ductus <name> --help`.
- AC27 — met: One test `unchecked_lines`/`adds_unchecked_box`/`describe_unchecked` (read_tasks.rs, any heading, outside fences/comments) feeds check_review_gate `unchecked_task_block`, check_artifacts blocking drift (`a_done_spec_holding_an_unchecked_box_is_review_state_drift`, three heading shapes), classify_spec_edit (`a_box_moved_or_under_a_renamed_task_is_a_correction`, `a_box_no_numbered_task_holds_reopens`), prune_plan (reopen tests ~1204-1285) and measure_reopenings `waiting_features`.
- AC28 — met: framework/commands/review.md step 1 and analyze.md step 1 stop naming boxes (done spec goes on to step 22 drift/--fix, step 21 records nothing); `refuse_while_tasks_are_open` called by write_review.rs and write_analysis.rs with the run's `deferred_here` exemption (tests write_review ~5144-5173, write_analysis ~1372-1407 incl. the 'Other' task still refusing); templates/spec/tasks.md line 19 'Schedule no review, analysis or completion gate as a task'; `git log 29ec6cc2` is 'feat(079): a review or an analysis runs once implementation is complete'.
- AC29 — met: framework/reference/review.md 'Two things a pass notices are no observation' (a no-claim edit such as a stray comma or hedge is not reported; an unre-checkable claim for want of a fetched source is named in the Summary's account of what went unexamined, not under Observations); review.md command criteria step 7.2 consistent.

## Observations

- convention: the doc comment of `ComputeReviewScopeResult.excluded` still said `write-review`'s record names the spec that claimed each, which the paths-only change made false — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- other: the scenario's 88.8% claimant share did not reproduce at `b7b07500` (88.6% by removed bytes), and the Resolved Question said 88.6% — `specs/079-review-and-analysis-work-converges/scenarios/the-excluded-section-lists-paths-only.md` — **fixed** (decided by the rule)
- other: `runtime/CHANGELOG.md`'s 079 entry repeated the 88.8% figure, a claimant-text count that was not stated, a corpus total 16 bytes off and description sizes that matched no commit of `runtime/src/schema/descriptions.rs` (7,961 to 7,932 and 4,359 to 4,323 are the constants' bytes), and an `[Unreleased]` entry still said the section names the claiming spec — `runtime/CHANGELOG.md` — **fixed** (decided by the rule)
- other: the Resolved Question's section sizes left the heading line out without saying so (9,467,774 against 9,469,246 with it), and its baseline sizes of the two references read as a target the owner later relaxed — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
- bug: mutation M14 still replaced its anchor with a `window.others.entry(path)` line that no longer compiled against the set, so it errored instead of being killed; re-anchored to `window.others.extend(claims.paths);` and run both ways — `scripts/mutation/reviews-follow-what-changed.py` — **fixed** (decided by the rule)
- convention: the `split_changes` doc comment said a path the walk never met 'has no claimant to name', wording from when `excluded` named claimants — `runtime/src/primitives/compute_review_scope.rs` — **fixed** (decided by the rule)
- convention: the `resolve_scope` doc paragraph wrapped oddly and `yaml_string`'s doc named only waiver and decision lists and said paths stay unquoted, though it quotes every path the record holds — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)
- other: AC6 and its Resolved Question carried no note of 081's read credit, under which a path a committed review read leaves the scope — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
- convention: the list header says the next mutation takes the next free number while `M319` follows `M273` in `scripts/mutation/scope-attribution.py` — `scripts/mutation/scope-attribution.py` — **waived**: Mutation ids are allocated across every list under `scripts/mutation/`, where `M318` is the highest of another list, so `M319` was the next free id; renumbering it to `M274` would reuse a number range other lists hold and gain nothing. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/review.md`
- `.ductus/reference/analyze.md`
- `.ductus/reference/audit.md`
- `.ductus/reference/clarify.md`
- `.ductus/reference/plan.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-review.md`
- `framework/commands/analyze.md`
- `framework/commands/review.md`
- `framework/reference/analyze.md`
- `framework/reference/audit.md`
- `framework/reference/clarify.md`
- `framework/reference/plan.md`
- `runtime/src/interpreter/payload.rs`
- `runtime/src/main.rs`
- `runtime/src/parser/mod.rs`
- `runtime/src/primitives/analyze_subjects.rs`
- `runtime/src/primitives/check_artifacts.rs`
- `runtime/src/primitives/check_command_flags.rs`
- `runtime/src/primitives/check_corpus_links.rs`
- `runtime/src/primitives/check_orphaned_references.rs`
- `runtime/src/primitives/check_review_gate.rs`
- `runtime/src/primitives/check_step_references.rs`
- `runtime/src/primitives/derive_routing_candidates.rs`
- `runtime/src/primitives/relocate_audit_records.rs`
- `runtime/src/primitives/spec_links.rs`
- `runtime/src/primitives/tool_loading.rs`
- `runtime/tests/bounded_list_gates.rs`
- `runtime/tests/check_step_references_cli.rs`
- `runtime/tests/derive_dependencies_golden.rs`
- `runtime/tests/derive_references_golden.rs`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/mcp.rs`
- `scripts/audit/README.md`
- `scripts/audit/step-reference-integrity.sh`
- `scripts/mutation/relocate-merge-children-057.py`
- `scripts/mutation/reopen-measure.py`
- `scripts/mutation/step-references-bounded.py`
- `scripts/mutation/step-references-reach.py`
- `scripts/tests/test-step-reference-integrity.sh`
- `specs/020-code-review/analysis.md`
- `specs/020-code-review/data-model.md`
- `specs/020-code-review/review.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/scenarios/dashboard-primitive.md`
- `specs/022-deterministic-runtime/scenarios/scenario-open-question-signal.md`
- `specs/022-deterministic-runtime/tasks.md`
- `specs/026-framework-self-audit/scenarios/families-follow-the-command-reference-split.md`
- `specs/026-framework-self-audit/scenarios/family-34-reference-section-reachability.md`
- `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/026-framework-self-audit/tasks.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/tasks.md`
- `specs/051-branch-scoped-spec-numbering/spec.md`
- `specs/051-branch-scoped-spec-numbering/tasks.md`
- `specs/057-analyze-artifact-and-record-relocation/analysis.md`
- `specs/057-analyze-artifact-and-record-relocation/data-model.md`
- `specs/057-analyze-artifact-and-record-relocation/review.md`
- `specs/057-analyze-artifact-and-record-relocation/scenarios/a-merge-carries-each-keys-whole-value.md`
- `specs/057-analyze-artifact-and-record-relocation/spec.md`
- `specs/057-analyze-artifact-and-record-relocation/tasks.md`
- `specs/066-audit-records-and-the-completion-gate/analysis.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/plan.md`
- `specs/066-audit-records-and-the-completion-gate/review.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/066-audit-records-and-the-completion-gate/tasks.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-bare-port-span-names-its-remedy.md`
- `specs/067-analyze-and-audit-checks/scenarios/check-artifacts-skipped-targets.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md`
- `specs/067-analyze-and-audit-checks/scenarios/orphaned-reference-check.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/071-command-reference-split/scenarios/reference-sections-are-reached-by-a-step.md`
- `specs/071-command-reference-split/spec.md`
- `specs/071-command-reference-split/tasks.md`
- `specs/080-deterministic-runtime-efficiency/plan.md`
- `specs/080-deterministic-runtime-efficiency/spec.md`
- `specs/080-deterministic-runtime-efficiency/tasks.md`
- `specs/081-reviews-and-checks-follow-what-changed/analysis.md`
- `specs/081-reviews-and-checks-follow-what-changed/plan.md`
- `specs/081-reviews-and-checks-follow-what-changed/review.md`
- `specs/081-reviews-and-checks-follow-what-changed/spec.md`

## Unexamined governance

*None.*
