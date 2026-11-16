---
spec: 079-review-and-analysis-work-converges
last-run: 2026-11-16T17:52:49Z
reviewed-against: 8636bc1ba8d3622e32da1e050303ab0f6dc25dbe
diff-base: 7c1c1530f6b142075cbef24a737302af2132dc36
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
scope: 4
unread: []
excluded: 0
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 1413afb3cec6f554723b56bcc6e190fc2140fe7ab52407e09e6f27984a75ba7f
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
---

# Review — 079-review-and-analysis-work-converges

## Summary

079's review of the window since its last review, from `7c1c1530` to HEAD: the two spec edits from its pre-done verification and analysis (`0b02d5d9`, AC1 and AC18 by the owner's decisions; `36b4bf35`, the plan naming the writers' guards) and the workflow-step runner's containment fix (`68f6bc67`), the window's only code. One agent ran all five passes, read all nine loaded backend and cross rule files and 079's spec, plan and data model whole, and read 4 of the 4 scope entries whole. No finding: `contained()` resolves links and compares whole path components, every step's own and job's directory is checked before any step runs, the refusal names the workflow and the value, and case K failed under each of the fix's three mutants. Four observations: two chores fixed in `8636bc1b` (case K gains a link under the root that points out of it, which a runner using `abspath` had passed; the AC18 Resolved Question names the growth by its commits instead of an incomplete list), and two waived by the rule with their reasons. The four stored decisions on files the run read still stand and are carried. The fix round was re-read by an independent agent against the whole contract, spec and data model, read whole: clean, and the loop's closing brief was empty.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

## Observations

- security — BE-INPUT-004's Verification counts an operator-set value as user input, and the runner opens the workflow files and the --log-dir directory named on its own command line with no canonicalization or containment. The rule needs an expected base directory to compare against, and none exists for these arguments: the operator names the workflow whose commands run and where its logs go, and the runner executes that workflow's text by design. The path the rule reaches, a working directory the committed workflow text names, is the one now checked, and it is checked by realpath and commonpath (verified by importing parse and contained from the runner without running a step: all 68 run steps of the repository's own workflows resolve inside the root, and '../x' and '/etc' are refused). — `scripts/run-workflow-steps.py:262-279` — **waived**: No base directory exists for the workflow-file and log-directory arguments, since the operator's own invocation names what runs and where its logs land, so the containment test has nothing to bind; the committed-text working directory BE-INPUT-004 reaches is checked. (decided by the rule)
- other — case K of the runner's test names three escapes (a relative one, a job's absolute path, a sibling sharing the root's name) but no symbolic link under the root that points outside it, though the runner's docstring and the plan state that links are followed and BE-INPUT-004's Verification names exactly that case as the one a lexical check misses. On a copy of the runner with realpath swapped for abspath, the whole suite passes (all of A to K), and a working-directory that is a link out of the repository then runs its step outside it; the unmodified runner refuses the same fixture with exit 2 (both driven under the fixture directory, not the checkout). — `scripts/tests/test-run-workflow-steps.sh:346-367` — **fixed** (decided by the rule)
- other — the Resolved Question 'Does AC18 hold each reference to the read size after the close it was first met at?' says later specs grew the two references past the read size 'through 073, 082, 020 and 022 from 2026-11-12'. Per commit between 9064b7be (18:49 on 2026-11-11) and 7c1c1530, by git cat-file -s before and after each, the growth began that same evening (041 task 24 at 19:57, 063 task 15 at 19:58, 020 task 14 at 20:37, 033 task 10 at 21:26, 067 task 23 at 21:46) and the contributors also include 050 (+259 in review.md), 066 (+273), 081 (+717) and nine commits naming 067 (+3,457 in analyze.md); 082 task 8 (+3,355) and 073 task 7 (+2,532 in analyze.md) are the two largest single steps. The list and the date are both incomplete, and an enumeration is a claim of its own (constitution §grounding). — `specs/079-review-and-analysis-work-converges/spec.md:163` — **fixed** (decided by the rule)
- other — a working-directory that stays inside the root but does not exist (a typo) passes contained() and then raises FileNotFoundError out of subprocess.Popen in run_bounded, so the run ends with a traceback and exit 1 after the earlier steps ran, with no summary line and the later steps unreported; reproduced on a fixture workflow of three steps. The run is loud, not silent, and the behavior is the same as before the window (the directory was used unresolved before). — `scripts/run-workflow-steps.py:233-246` — **waived**: The failure is loud (traceback, non-zero exit) and unchanged by this fix; refusing a missing directory before any step runs is a new exit-2 rule that task 38's criteria do not state, while this fix was asked to bound where a step may run, not whether the directory exists. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

*None.*

## Unexamined governance

*None.*
