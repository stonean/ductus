---
spec: 079-review-and-analysis-work-converges
last-run: 2026-11-22T01:30:28Z
reviewed-against: fc5bbfea0aa6237613ebc82256e8a12101953068
diff-base: db227015b55be230e5e6a55f3c9baeeac7fb89e0
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 1
scope: 2
unread: []
excluded: 22
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 0
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
blocking: false
dispositions:
  fixed: 2
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
  - key: "convention: the list header says the next mutation takes the next free number while `M319` follows `M273` in `scripts/mutation/scope-attribution.py` — `scripts/mutation/scope-attribution.py`"
    outcome: waived
    reason: Mutation ids are allocated across every list under `scripts/mutation/`, where `M318` is the highest of another list, so `M319` was the next free id; renumbering it to `M274` would reuse a number range other lists hold and gain nothing.
    decided-at: 2026-11-20T00:32:48Z
    decided-by: rule
---

# Review — 079-review-and-analysis-work-converges

## Summary

Five-pass review of 079's reopening (diff-base db227015, base-source reopen) at HEAD fc5bbfea. Its window is fc5bbfea, which reopened 079 as the owner chose on 2026-11-21: AC17 reworded to admit an analysis's operator-confirmed design edit (067's AC6), with a signpost back to 067, a Resolved Question and a Why-split-from-067 entry. Passes (part P5) read that hunk and `specs/079-review-and-analysis-work-converges/spec.md` whole against the rule files the change touches, `quality-cross.md` and `configuration-cross.md` (38,992 bytes, each whole against `wc -c`); the three `security-*.md` files were not read, since the one-file scope holds no code for the security pass, which is named here as not run. No finding; one observation, and the criteria part's note on the plan, both fixed in this run: the Resolved Question "What may one review run cost?" and `plan.md`'s Dispositions and Finding-dispositions bullet now admit the design edit. P5 re-ran over the fix. Two stored waivers still apply. The criteria step (part C10) judged every criterion individually: 29 of 29 met, AC17 against its reworded text, many driven with the release binary on a scratch clone. Examined 1 of 1; nothing unread.

1 fix round(s): 0 re-read, 0 diff-checked, 1 exempt, 0 not re-read. Exempt: round 1.

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

- AC1 — met: `runtime/src/primitives/measure_reopenings.rs` (`REOPENING_RUN_BOUND`, `over_bound`; gating = `blocking/MUST/SHOULD/contract-breaks/open` `mark/budget` `stop/skipped` pass); release binary `measure-reopenings --at d62e72f3 --since 2026-10-01` reproduces 126 closed, 261 runs, 28 over, 0 not counted, waiting 11; lists not-counted and waiting; tests in `measure_reopenings.rs` (`a_second_clean_run_`..._is_over_the_bound, `a_second_run_after_a_gating_finding_`..., `a_reopening_with_no_run_is_not_counted`). The 10-reopening sample's per-reopening tasks were not re-counted here; the Resolved Questions scope it (owner, 2026-11-09 and 2026-11-16).
- AC4 — met: `compute_review_scope.rs` `owes_a_review` and `check_review_gate.rs` `holds_done_until_reviewed` (gating count, no dispositions map, open mark, unwaived round, open-findings); tests `a_reopening_that_changed_only_text_owes_no_review`, `a_record_the_gate_holds_owes_a_review_whatever_the_window_holds`, `a_review_is_owed_with_no_record_whatever_the_window_holds`, `an_artifact_only_window_owes_a_review_while_a_criterion_is_unjudged`; binary compute-review-scope on 079 reports review-owed true with criteria-owing [AC17].
- AC2 — met: `framework/commands/review.md` step 10 and `analyze.md` step 19 (chore fixed, else waived by rule, only a gating deferral or design edit reaches gate-confirm; contract-break observations treated as gating); `decisions.rs` `RULE_DECIDER` 'by-rule'; `runtime/tests/review_command.rs` `a_fix_and_route_step_asks_a_person_only_about_a_gating_deferral`.
- AC3 — met: `compute_review_scope.rs` excluded is a list of path strings (binary output on 079: 26 plain strings); `fix_rounds.rs` `round_paths` drops another spec's commits, `tests/fix_rounds.rs` `another_specs_commit_between_rounds_leaves_the_round` and `brief_re_read.rs` `another_specs_commit_leaves_a_round_that_also_holds_a_fix`; `write_review.rs` renders the Excluded from scope section.
- AC5 — met: `compute_review_scope.rs` window start; binary base-source 'reopen' on 079; tests `a_review_in_the_reopening_moves_the_start_to_its_commit`, `a_run_that_stopped_early_or_does_not_parse_does_not_move_the_start`, `a_record_the_gate_holds_does_not_move_the_start`, `since_override_replaces_the_derived_base`.
- AC6 — met: `commit_attribution.rs` (CommitClaims, merge commit claims nothing) with `compute_review_scope.rs`; tests `a_named_commits_code_goes_to_every_spec_it_names`, `a_planned_path_of_an_unnamed_multi_spec_commit_goes_to_the_plan_naming_it`, `an_unplanned_path_`..._goes_to_every_claimant, `what_the_record_leaves_open_joins_the_scope_wherever_the_window_starts`, `a_contract_changed_before_the_window_joins_the_scope`, `a_fold_below_done_brings_its_code_into_a_window_`..., `an_unchanged_plan_entry_stays_only_on_the_first_review`, `uncommitted_work_joins_whole_even_on_another_specs_path`; the 081 supersessions are in the same file's tests.
- AC7 — met: `diff_cross_spec.rs`; binary on 079 reports window-base db227015 (the reopen's parent) and 6 sibling paths, all 067 files that fc5bbfea touched with 079's directory; tests `the_window_starts_at_the_transition_parent`, `a_reopen_leaves_an_earlier_windows_sibling_changes_out`.
- AC8 — met: `framework/runtime-tools.txt` lists measure-reopenings and classify-spec-edit as MCP tools; commands, references and constitution carry the behavior; `measure_reopenings.rs` test `a_project_in_a_subdirectory_reads_its_own_paths` and `--help` show it reads a project's own history.
- AC9 — met: `ductus mcp` `tools/list` against `ductus <tool> --help` for all 73 tools: 73 descriptions equal their --help text, 0 mismatches; `runtime/src/schema/descriptions.rs`; constitution canonical-source table row 'A primitive's behavior'.
- AC10 — met: `AGENTS.md` Workflow bullet 'Weigh what an option starts against what it delivers' states each option's result, the reopens and review runs, and what is given up.
- AC11 — met: `AGENTS.md` Workflow bullet 'Local checks follow what a change touches' (artifacts-only, docs, `framework/scripts/CI`, runtime tiers; every push workflow's steps before a tag) and 'After pushing, read every workflow's run' with 'Never wait on it'.
- AC12 — met: spec cross-spec-impact lists the 12 specs; each of 020, 022, 026, 041, 050, 051, 058, 063, 066, 067, 076, 078 links back to 079 (grep of `spec/plan/scenarios`), 067 and 079 have reciprocal signposts; `plan.md` and `tasks.md` exist with all boxes ticked. Not a verdict change: `plan.md`'s Dispositions section (line on step `10/19`) still says only a gating waiver or deferral reaches the operator, and its constitution list does not name the 2026-11-21 design-edit amendment, so it lags AC17's reword.
- AC13 — met: `brief_re_read.rs` classify keeps the per-path exemption; test `an_exempt_path_leaves_a_mixed_rounds_brief_and_record` and `the_brief_carries_exactly_the_derived_paths`.
- AC14 — met: `write_review.rs` `loop_open` writes fix-rounds-open: true (test `the_open_mark_follows_the_loop_open_argument`); `review.md` step 11 passes loop-open and step 12 omits it; `check_review_gate.rs` gate check 9 blocks on the mark (tests `what_the_gate_holds_until_a_review_runs_is_what_review_owed_reads`, `a_done_spec_holding_the_open_mark_is_review_state_drift`).
- AC15 — met: classify-spec-edit on a done fixture in a scratch clone: spec prose edit and plan edit read correction, a criterion edit reads reopens; constitution §spec-lifecycle states it; tests `spec_body_prose_is_a_correction`, `a_plan_edit_is_a_correction`.
- AC16 — met: `AGENTS.md` Workflow bullet 'A change to a shared runtime surface lands in the spec that requires it, and cross-spec-impact: holds that spec's done until the surface's owner links back'.
- AC17 — met: Re-judged against the reworded text: it matches 067 AC6 (an analysis fixes a gating design-subject finding once the operator confirms) and every surface that carries it: constitution §brownfield-inbox Finding dispositions ('fixed ... or an analysis's edit to a design subject ... once the operator confirms it'; 'Only a gating finding's waiver, deferral or design edit reaches the operator, one at a time'), `framework/commands/analyze.md` description, Purpose and step 19, `review.md` step 10 (only a contract-break deferral reaches gate-confirm); measure-reopenings reports decisions-by-person per run with carried decisions counted in the first record (`measure_reopenings.rs` `drop_carried`; test `a_decision_carried_from_another_specs_record_counts_where_it_began`); gate-confirm is one decision per call. The criterion is ticked but compute-review-scope names it in criteria-owing until this review's record is written.
- AC18 — met: wc -c at HEAD: `framework/commands` plus `framework/reference` 474,248 under 567,705; `AGENTS.md` 49,793 and constitution 49,199 under 50,000; `review.md` 57,533 and `analyze.md` 66,965 exceed the 2026-11-11 read size, which the criterion and its Resolved Question allow for later specs.
- AC19 — met: binary render-status prints 'Reopenings since 2026-10-23: 186 closed, 42 over the review and analysis bound, 0 not counted · 15 in-progress spec(s) with no unchecked task'; dashboard carries the reopenings object; `dashboard.rs` and `render_status.rs`.
- AC20 — met: `tasks.md` holds 0 unchecked boxes; measurement reproduces the recorded baseline at d62e72f3 (126 closings, 261 runs, 28 over, run by the release binary in a scratch clone). The next-10-closings sample is owner-scoped by the Resolved Questions of 2026-11-04, 2026-11-09 and 2026-11-16; its per-reopening tasks were not re-counted.
- AC22 — met: binary check-review-gate on 079 returns blocked-by review-owed naming AC17 ahead of every analyze check; tests `a_reopening_that_changed_code_takes_one_review_then_one_analysis`, `a_review_owed_for_paths_and_criteria_names_both`, `a_path_in_another_specs_directory_owes_no_review`.
- AC23 — met: `write_review.rs` `open_findings` and `render_open_findings`; `compute_review_scope.rs` holds each listed path (tests `what_the_record_leaves_open_joins_the_scope_wherever_the_window_starts`, `a_path_left_open_that_is_gone_or_leaves_the_project_is_not_read`); gate holds on `open_findings` via `holds_done_until_reviewed`; `write_review.rs` tests `a_contract_break_the_run_did_not_fix_counts`, `a_stored_break_left_open_with_no_file_refuses`.
- AC24 — met: `process_waivers.rs` and `process_decisions.rs` tests `a_waiver_on_a_file_the_run_did_not_read_is_retained`, `a_silent_decision_on_a_file_the_run_read_expires`, `a_silent_decision_on_a_file_gone_from_disk_expires`, `a_scope_that_cannot_be_read_retains_and_says_so`, `an_analysis_expires_a_silent_decision_wherever_its_file_lies`.
- AC25 — met: `classify_spec_edit.rs` SweepIndex and ReferenceScrub; tests `a_token_swept_across_files_is_mechanical`, `a_sweep_through_another_file_is_mechanical`, `a_rewritten_reference_is_mechanical`, `a_token_changed_in_one_file_reopens` (the kinds the criterion names).
- AC26 — met: binary on a done fixture: check-artifacts reports blocking review-state-drift 'done spec has unticked acceptance criteria'; classify-spec-edit reads an untick as reopens (reason criterion) and a tick as correction.
- AC21 — met: `framework/constitution.md` §runtime-boundary Host integration ('The binary is the source of a primitive's rules'; without the server, stop and say to restart or re-run /ductus; marker §runtime-host-integration resolves); the commands' pointer lines (log, target and review checked) say the same.
- AC27 — met: `read_tasks.rs` `describe_unchecked/unchecked_lines/adds_unchecked_box` used by `check_review_gate.rs`, `check_artifacts.rs`, `classify_spec_edit.rs`, `prune_plan.rs` and `measure_reopenings.rs`; binary: gate blocked-by unchecked-task for a box under an unnumbered heading, check-artifacts review-state-drift on a done spec with a box, comment and fence boxes ignored; tests `a_box_moved_or_under_a_renamed_task_is_a_correction`, `a_box_no_numbered_task_holds_reopens`, `moving_or_reindenting_an_unchecked_box_does_not_reopen`, `waiting_is_the_in_progress_specs_with_no_unchecked_task`.
- AC28 — met: binary on a fixture: write-review and write-analysis both refuse with '`tasks.md` holds 1 unchecked box(es)' for a box under a numbered and under an unnumbered heading, and write-analysis accepts a list whose only boxes are ticked, in a comment or in a fence; `review.md` and `analyze.md` step 1 stop on it; the deferral-named exception is in `refuse_while_tasks_are_open` (`read_tasks.rs`), not driven here.
- AC29 — met: `framework/reference/review.md` Observations: 'Two things a pass notices are no observation' (a no-claim edit is not reported; an unchecked claim with no fetched source goes in the Summary's unexamined account).

## Observations

- convention: the Resolved Question "What may one review run cost?" still said only a gating finding's waiver or deferral reaches the owner, after AC17's reword admitted an analysis's design edit — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
- convention: `plan.md`'s Dispositions said only a gating waiver or deferral reaches the operator, and its Finding-dispositions bullet did not record 067's AC6 amendment — `specs/079-review-and-analysis-work-converges/plan.md` — **fixed** (decided by the rule)
- `cross-spec-impact:` names neither 000 nor 035, though the round edited their scenarios and reopened 035 — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Each is a correction of a claim, which owes no back-link and names this spec in its commit, as `plan.md`'s Cross-spec impact says; AC12's list is of specs whose surface this changes. (decided by the rule)
- other — the Behavior bullet 'A behavior is stated once' now requires existing restatements in a data model or scenario to be swept to pointers (8596f472), but no acceptance criterion tests it: AC9 covers the description, commands and references only, AC12 corrects false claims. A criterion would be one nothing can check, since the spec's own Resolved Question records that nothing detects a new copy. — `specs/079-review-and-analysis-work-converges/spec.md:64` — **waived**: The Resolved Question on the canonical surface states that nothing detects a new copy (§drift-prevention) and that this part relies on review, so no testable criterion exists to add; the sweep was task 36's, and a criterion is a requirement of its own. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/help.md`
- `.ductus/reference/analyze.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-help.md`
- `framework/commands/analyze.md`
- `framework/commands/help.md`
- `framework/constitution.md`
- `framework/reference/analyze.md`
- `runtime/CHANGELOG.md`
- `runtime/src/primitives/resolve_anchor.rs`
- `runtime/src/schema/descriptions.rs`
- `scripts/analyze/detect.py`
- `scripts/analyze/record.py`
- `scripts/mutation/quoted-heading-forms-067.py`
- `scripts/tests/test-analyze-scripts.py`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/review.md`
- `specs/067-analyze-and-audit-checks/scenarios/anchor-reference-kinds.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`

## Unexamined governance

*None.*
