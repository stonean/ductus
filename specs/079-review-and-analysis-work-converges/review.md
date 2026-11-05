---
spec: 079-review-and-analysis-work-converges
last-run: 2026-11-05T21:43:57Z
reviewed-against: 5f67ede2b08969a4604ed9eb81b6b5ce67d00eef
diff-base: 3360dea31270eba469a3152fd341d632f1436ad1
must-violations: 0
should-violations: 0
low-confidence: 1
examined: 240
scope: 295
excluded: 11
contract-breaks: 0
skipped-passes: []
fix-rounds-open: true
reviewed-digest:
  data-model.md: 684b13c73bd25e42f2f963caa1fd715e048f5984534d602c21f7ea53c20a5300
blocking: false
dispositions:
  fixed: 60
  deferred: 0
  waived: 6
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
  - key: "perf: the gate checks the open mark after the analyze record, so a cut-off review is sent to /analyze first and then needs a second analyze with no gating finding between them. — `runtime/src/primitives/check_review_gate.rs:206`"
    outcome: waived
    reason: The path needs a cut-off run and is not forced, since the review's own report comes first; moving the check changes the gate's documented order to save one analysis on that rare path.
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
---

# Review — 079-review-and-analysis-work-converges

## Summary

First review of this spec, over the window since its reopen (`3360dea3`), read at `5f67ede2` by four independent read-only agents, each running all five passes over one slice: the attribution, window, fix-round, record and gate code (19 files); the measurement, descriptions, surfaces, other runtime files, mutation lists and workflows (49); shipped prose, governing files and audit scripts (51); and spec artifacts (123). `examined` is 240: 75 files opened whole, and 165 read as their window diff with context, each named with its reason in the agents' results, 118 of them other specs' corrected files, whose unchanged context was clipped and searched rather than read through. Not read, and how the rest of the scope reconciles: `framework/bootstrap/govern.md`, byte-identical to `framework/bootstrap/ductus.md` by `cmp`; `scripts/audit/cross-doc-consistency.sh`, with no window diff, grepped and run; 48 generated mirrors under `.claude/commands/ductus/`, `.ductus/reference/` and `.pi/prompts/`, generated from the sources the passes read and in sync by `scripts/gen-claude-commands.sh --check` at `5f67ede2`; and 6 plan entries that are patterns naming no single file. Rule files were read unevenly, as the results record: `quality-cross.md` by all, the three frontend files by none since nothing in scope is frontend code, and the backend files in places by Statement alone, since a `BE-` rule mostly does not fire on a local process.

The passes found 4 SHOULD violations, all fixed before this record and so outside its counts: `QUAL-TEST-001` on a test in `runtime/tests/fix_rounds.rs` the gate's lint masked, now `a_cut_off_review_leaves_the_mark_its_last_write_clears` with the block proved by the gate's own test; `QUAL-TEST-001` on `a_stored_decision_by_a_person_wins_over_the_rule`, now with outcomes that differ; `QUAL-CLAIM-001` on `measure-reopenings` reading an unreadable specs tree as an empty history, now an error; and `QUAL-TEST-001` on a measurement test that could not fail, removed, since two others cover what it could observe. One low-confidence finding stays recorded below.

Of 66 observations, the rule fixed 60 and waived 6, each with its reason. Ten of the fixed were contract breaks: `review-owed` read false on a deletion and on a stale digest; a merge carrying a run in moved the window's start; a fix round dropped an uncommitted edit to a binary file; `decisions-by-person` did not report per run; step 11's second write dropped the open mark; three references kept a primitive's rules; and the disposition rule's waivers stopped `/prune` proposing plan sections, which only a person's waiver now does. No decision reached the owner. The passes were not re-run on the fixed tree: the fix-round loop re-reads the fixes, and each fix's test was shown to fail with its behavior removed (M23, M24, M44, M63, M68, M69 and M104-M119 in `scripts/mutation/scope-attribution.py`, M89 and M90 in `scripts/mutation/reopen-measure.py`, all killed). This review also corrected claims AC12 had ticked as corrected, in 078, 022, 023 and 050.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: QUAL-TEST-001 — a_record_that_excluded_nothing_says_so only reaches resolve_scope's error fallback

- **File**: `runtime/src/primitives/write_review.rs:1898-1909`
- **Rule**: A test SHOULD fail when the behavior it names is removed or broken. A test that passes whether or not the code under test behaves as named SHOULD NOT be added or counted as coverage.
- **Finding**: spec_repo builds no git repository and base_args names diff-base def5678, so compute errors and resolve_scope returns its fallback (0, empty) at :611-612. The test never reaches the primary path with nothing to exclude. It does pin render_excluded's *None.* branch, and the_record_names_each_excluded_path_by_the_spec_that_claimed_it covers the primary path with an exclusion, hence low confidence.
- **Auto-fixable**: no
- **Suggested fix**: Build the fixture from a real repository with a committed transition and no sibling commit, as repo_with_another_specs_commit does, set args.diff_base to the transition's parent, and assert `scope` is above 0 beside `excluded: 0`.

## Waived findings

*None.*

## Deferred findings

*None.*

## Observations

- bug: review-owed is false when a reopening's only code or contract change is a deletion, because the test reads a scope that has deleted paths removed. — `runtime/src/primitives/compute_review_scope.rs:215` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: review-owed is false while the gate holds the record for a stale digest, because holds_done_until_reviewed omits the one state whose guidance is 'Re-run /{project}:review' that the window cannot see. — `runtime/src/primitives/check_review_gate.rs:460` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: a merge carrying a review record into a branch without it reads as the commit that finished the run, so the window starts at the merge and commits made after the review leave it unread. — `runtime/src/primitives/compute_review_scope.rs:411` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: a fix round drops an uncommitted edit to a non-UTF-8 file sitting on another spec's commit, because blob_text reads every non-UTF-8 blob as None and None equals None. — `runtime/src/primitives/fix_rounds.rs:565` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: a first review that stopped early still drops unchanged plan entries from the next scope, so what its skipped pass never read stays unread, against the module doc's 'stopping early leaves nothing permanently unread'. — `runtime/src/primitives/compute_review_scope.rs:201` — **waived**: AC6 keeps an unchanged plan entry only on the first review, and that run was the first; what its skipped pass did not read is named in the record's skipped-passes, and restricting a run is the operator's choice. (decided by the rule)
- perf: check-prescriptions counts an exempt path as unprescribed, so a round that applied exactly the prescribed edits and also touched spec.md, plan.md or tasks.md differs and costs another re-read of paths the check had just matched. — `runtime/src/primitives/check_prescriptions.rs:82` — **waived**: No criterion covers it: AC13 covers the brief, and this costs one re-read on a rare round that also edits the spec's own text. (decided by the rule)
- perf: the gate checks the open mark after the analyze record, so a cut-off review is sent to /analyze first and then needs a second analyze with no gating finding between them. — `runtime/src/primitives/check_review_gate.rs:206` — **waived**: The path needs a cut-off run and is not forced, since the review's own report comes first; moving the check changes the gate's documented order to save one analysis on that rare path. (decided by the rule)
- convention: the open-mark block's message does not name the command, though the Edge Case says it does; only its guidance does, and /implement says to 'halt with that message'. — `runtime/src/primitives/check_review_gate.rs:497` — **fixed** (decided by the rule)
- convention: fix_round_block's doc comment is attached to holds_done_until_reviewed, which inserted its own lines beneath it, so that function's doc opens 'Gate check 12' and fix_round_block has none. — `runtime/src/primitives/check_review_gate.rs:451` — **fixed** (decided by the rule)
- convention: render_skipped's doc comment now opens render_excluded's, describing skipped passes on the function that renders exclusions, and render_skipped has none. — `runtime/src/primitives/write_review.rs:1395` — **fixed** (decided by the rule)
- convention: a test comment still names resolve_scope_size, which this window renamed to resolve_scope. — `runtime/src/primitives/write_review.rs:2409` — **fixed** (decided by the rule)
- convention: write-analysis's module doc says a gating finding's waiver is confirmed by the operator, while the same file refuses any waiver of one. — `runtime/src/primitives/write_analysis.rs:24` — **fixed** (decided by the rule)
- convention: two test-fixture comments in compute_review_scope.rs describe something else: one opens with a sentence about another test, and one names a parameter that does not exist. — `runtime/src/primitives/compute_review_scope.rs:1531` — **fixed** (decided by the rule)
- bug: decisions-by-person is summed across a reopening's runs, so the measurement does not report how many decisions each run put to a person, as AC17 says it does. — `runtime/src/primitives/measure_reopenings.rs:614` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: a spec directory renamed while in-progress counts a record carried from before the rename as a run of this reopening, which can report a false over-bound. — `runtime/src/primitives/measure_reopenings.rs:708` — **waived**: A rename mid-reopening is rare, it over-reports rather than hides, no gate reads the count, and the module doc states that a renamed spec's history starts at the rename. (decided by the rule)
- perf: the module doc says /status does not pay for the whole history, but the sorted revwalk parses every reachable commit header before its first yield; the early stop saves only tree and record reads. — `runtime/src/primitives/measure_reopenings.rs:11` — **fixed** (decided by the rule)
- convention: relocate-audit-records' compiled description still says the mechanical-edit rule applies, where its module doc and the constitution now call a spec.md frontmatter edit a correction. — `runtime/src/schema/descriptions.rs:80` — **fixed** (decided by the rule)
- convention: review-owed's schema doc omits the record the gate holds, and the description says a record 'exists' where the code and data model say it names a run. — `runtime/src/schema/primitives.rs:758` — **fixed** (decided by the rule)
- convention: a sweep left a garbled sentence in registry.rs's module doc. — `runtime/src/schema/registry.rs:23` — **fixed** (decided by the rule)
- convention: a new test was inserted inside another test's doc comment, so that comment now sits on the wrong test and its own test has none. — `runtime/src/main.rs:937` — **fixed** (decided by the rule)
- convention: the same displacement in tests/mcp.rs: the manifest test's doc comment now opens the doc of the long_about helper. — `runtime/tests/mcp.rs:148` — **fixed** (decided by the rule)
- convention: dispatch_blocking's enumeration of history walkers omits measure-reopenings, and neither it nor the dashboard handler's comment names dashboard's new history walk. — `runtime/src/mcp/server.rs:238` — **fixed** (decided by the rule)
- convention: schema/mod.rs's module doc lists its submodules and registries but not the new descriptions module. — `runtime/src/schema/mod.rs:8` — **fixed** (decided by the rule)
- other: the changelog's exact figure for framework/reference/review.md is out of date since the last commit. — `runtime/CHANGELOG.md:70` — **fixed** (decided by the rule)
- other: cross_service.rs's second assertion and tests/parity/cross-service/expected.txt are now a copy of the golden whose only effect is requiring a re-bless in two places, as the file's own doc says. — `runtime/tests/cross_service.rs:60` — **waived**: plan.md's Affected Files keeps this assertion on purpose, and 030's plan and tasks name the capture, so removing it is a design choice no criterion here asks for. (decided by the rule)
- bug: analyze's new disposition rule waives every plan-record finding, and prune-plan counts any stored waiver as decided, so /prune stops proposing the sections §plan-phase says it moves home. — `framework/commands/analyze.md:85` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: analyze step 5 passes 'the files the rule-file discovery step selected', but no step of the 22 discovers rule files and none points at §Rules, so the step cannot be followed from the command, and the window removed the old 'the project's rule files'. — `framework/commands/analyze.md:52` — **fixed** (decided by the rule)
- bug: no step points at the host's scenario-frontmatter check or at §Cross-spec references, so a hard-fail check 079's text assigns to the host is never reached. — `framework/commands/analyze.md:46` — **fixed** (decided by the rule)
- convention: analyze's Scope Boundaries says a host that cannot ask writes nothing at step 19, and step 19 says that host still applies the rule, which fixes and waives. — `framework/commands/analyze.md:33` — **fixed** (decided by the rule)
- bug: review step 10 says a defect making the review's own result wrong is never waived and gives it no disposition, since its one deferral is for a broken contract. — `framework/commands/review.md:130` — **fixed** (decided by the rule)
- bug: review step 1's 'stop' on review-owed: false reads as ending the whole run, which under --all leaves every later feature unreviewed. — `framework/commands/review.md:118` — **fixed** (decided by the rule)
- bug: configure step 1 lost its no-server path to the markdown-only cut and gained no CLI pointer, unlike every framework/commands file, and the file says no constitution section applies, so §runtime-host-integration is not loaded. — `framework/bootstrap/configure/claude.md:18` — **fixed** (decided by the rule)
- bug: /amend's reopen check runs classify-spec-edit only on a delta that excludes a modified scenario and a data model, the two edits §spec-lifecycle says always reopen, and its line then says no scenario edit exists. — `framework/commands/amend.md:58` — **waived**: The delta predates this spec, AC15 governs what classify-spec-edit returns and not which edits reach it, and widening the delta changes what the reconcile pass offers, which is a requirement of its own. (decided by the rule)
- convention: implement step 14 lists what a passing gate's guidance names and omits the contract-break notice this spec added, which is the one notice meant for whoever approves done. — `framework/commands/implement.md:87` — **fixed** (decided by the rule)
- convention: AGENTS.md still lets a review defer a non-gating observation that concerns the code under review, which AC2 no longer allows. — `AGENTS.md:97` — **fixed** (decided by the rule)
- convention: groom step 1 still says the host resolves the inbox path 'on **both** paths', residue of the two-paths text the constitution retired. — `framework/commands/groom.md:32` — **fixed** (decided by the rule)
- convention: framework/reference/status.md keeps a 'host-tools procedure' stating resolve-references' rules, though status step 1 says the runtime resolves them and the path it served is retired. — `framework/reference/status.md:21-32` (breaks the spec's contract) — **fixed** (decided by the rule)
- convention: framework/reference/specify.md still states create-feature's numbering and copy rules and says so itself. — `framework/reference/specify.md:58-66` (breaks the spec's contract) — **fixed** (decided by the rule)
- convention: consolidate step 4 restates rewrite-spec-links' description clause by clause, which is what AC9 retires. — `framework/commands/consolidate.md:83` (breaks the spec's contract) — **fixed** (decided by the rule)
- convention: the review reference says a changed contract obliges 'a full re-review', which AC5's window makes false. — `framework/reference/review.md:341` — **fixed** (decided by the rule)
- convention: docs/developing-ductus.md says the next two sites name a primitive's description, but the dispatch arm does not and main.rs's Command, the one after it, does. — `docs/developing-ductus.md:10` — **fixed** (decided by the rule)
- convention: outside slice C, found checking framework/commands/analyze.md:54's claim, which is right: derive-dependencies' compiled description, now the canonical source, names a dry-run flag that does not exist. — `runtime/src/schema/descriptions.rs:51` — **fixed** (decided by the rule)
- bug: review step 11's second write under --fix does not say it carries loop-open, and data-model.md says only the first write passes it, so the mark clears before step 12's loop runs. — `framework/commands/review.md:133` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: only framework/reference/review.md's §Resolve target and scope item 2 halts a review of a spec not in-progress or done; no command step carries it, and step 1 points at that section only for the tech-stack check. — `framework/commands/review.md:118` — **fixed** (decided by the rule)
- bug: 078's Edge Case says a commit made between rounds changes no round's scope, which 079's round filter falsified; the bullet directly below it, which 079 edited, says the opposite. — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md:74` — **fixed** (decided by the rule)
- bug: 058's ticked AC31, edited in this window to drop `confirmed`, now says every chore fix that fails is undispositioned, against both commands and 058's own Edge Case, which 079 also edited. — `specs/058-findings-route-at-discovery/spec.md:509` — **fixed** (decided by the rule)
- convention: 026's signpost says 079 changed what `scripts/audit/cross-doc-consistency.sh` and `scripts/audit/marker-list-parity.sh` pin; the first has no commit in the window and the second changed one word of a comment. — `specs/026-framework-self-audit/spec.md:24` — **fixed** (decided by the rule)
- convention: 022's plan, corrected in this window, names `/{project}:prune` as the slash command that runs `git diff`; task 11's cut then removed the only such line, so no command does. — `specs/022-deterministic-runtime/plan.md:135` — **fixed** (decided by the rule)
- convention: 022's scenario `review-scope-parse-fidelity`, corrected in this window, still says the plan-affected set is never dropped for a larger one; AC6 drops an unchanged plan entry on every later review. — `specs/022-deterministic-runtime/scenarios/review-scope-parse-fidelity.md:31` — **fixed** (decided by the rule)
- convention: 023's scenario `living-specs` says `classify-spec-edit` decides the mechanical-versus-meaningful boundary from the diff alone; the primitive reads a uniform sweep into a scenario as `reopens`, and `mechanical_sweep` exempts it. — `specs/023-govern-refinement/scenarios/living-specs.md:22` — **fixed** (decided by the rule)
- convention: 022's spec still lists a markdown-only case among the extension-point surfaces and says a host never does a primitive's work by hand, beside corrected text saying no such path remains; the bootstrap's per-step fallback does exactly that. — `specs/022-deterministic-runtime/spec.md:164` — **fixed** (decided by the rule)
- convention: 050's plan keeps "W14 and W15 ... are cases (b) and (c)" of the mechanical-edit rule beside a row this window changed to call W15 a correction, and the window's row removals left two header-only tables and prose pointing at a removed row. — `specs/050-constitution/plan.md:383` — **fixed** (decided by the rule)
- other: reopened 059's task 22 says `compute-review-scope` reports `review-owed: false`; at HEAD it reports true. — `specs/059-project-in-a-repository-subdirectory/tasks.md:181` — **fixed** (decided by the rule)
- other: 079's task 12 handoff prose is stale: it says no criterion is ticked, calls subtask 2's assumption untried, and asks the owner a question already answered. — `specs/079-review-and-analysis-work-converges/tasks.md:143` — **fixed** (decided by the rule)
- convention: 066's data model gained the prose for `excluded`, five lists and `base-source`, but its two result examples still show the old shape, and its own 354-byte figure holds only with the fields they omit. — `specs/066-audit-records-and-the-completion-gate/data-model.md:16` — **fixed** (decided by the rule)
- convention: 030's plan keeps a heading and a table row naming a markdown-only path whose text this window removed. — `specs/030-cross-service-references/plan.md:49` — **fixed** (decided by the rule)
- convention: 063's plan keeps a heading and a sentence naming a markdown-only path, beside a section this window rewrote to say the host runs the primitive. — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md:91` — **fixed** (decided by the rule)
- convention: a 022 scenario keeps the heading `### Markdown-only path` over the one sentence this window left beneath it. — `specs/022-deterministic-runtime/scenarios/commands-dir-parameterization.md:52` — **fixed** (decided by the rule)
- convention: 032's data model keeps "markdown-only path faithful" on the bullet after the one this window corrected to say the run continues through the CLI. — `specs/032-opencode-agent/data-model.md:134` — **fixed** (decided by the rule)
- convention: 029's spec keeps "falls back to the markdown path" in the paragraph beside a bullet this window corrected. — `specs/029-bootstrap-runtime-autowire/spec.md:83` — **fixed** (decided by the rule)
- other: 050's scenario records the constitution's size with no commit, and the size has since moved. — `specs/050-constitution/scenarios/the-constitution-fits-one-read.md:18` — **fixed** (decided by the rule)
- other: 020's task 16 states the reference's current size in the present tense with no commit, and the size has since moved. — `specs/020-code-review/tasks.md:130` — **fixed** (decided by the rule)
- convention: 022's scenario gives implement's second `diff-cross-spec` step as 12, in a sentence this window edited. — `specs/022-deterministic-runtime/scenarios/coverage-expansion-primitives.md:23` — **fixed** (decided by the rule)
- convention: 079's own cross-spec table gives 076's status as `done`, and says it reopens, though 079 already reopened it. — `specs/079-review-and-analysis-work-converges/plan.md:186` — **fixed** (decided by the rule)
- convention: 079's own plan says `scripts/audit/cross-doc-consistency.sh` changes with the back-edge text and lists both audit scripts as modified for their logic; neither script's logic changed. — `specs/079-review-and-analysis-work-converges/plan.md:49` — **fixed** (decided by the rule)
- other: reopened 062's task 29 gives one reason for `review-owed: true`, and that reason is not the whole of it. — `specs/062-concurrent-session-targets/tasks.md:237` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `scripts/mutation/fix-round-results-bounded.py` — 078-review-fix-rounds-are-re-read-within-a-bound
- `scripts/mutation/own-record-is-no-round.py` — 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/058-findings-route-at-discovery/scenarios/analyze-findings-match-decisions-by-host-judgment.md` — 000-slash-commands, 011-brownfield-process, 016-cross-cutting-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 035-groom-session-target, 041-task-pruning, 045-decision-state-drift-detection, 046-scenario-open-question-visibility, 047-analyze-findings-durability, 048-govern-acquired-runtime, 049-rename-govern-to-ductus, 050-constitution, 051-branch-scoped-spec-numbering, 054-remove-supersession, 055-shared-constitution, 057-analyze-artifact-and-record-relocation, 058-findings-route-at-discovery, 060-exec-analyze-assesses-each-loaded-rule, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 071-command-reference-split, 072-governing-documents-stay-within-one-read, 075-agents-md-holds-what-every-session-needs, 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-base-includes-the-transition-commit.md` — 000-slash-commands, 011-brownfield-process, 016-cross-cutting-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 035-groom-session-target, 041-task-pruning, 045-decision-state-drift-detection, 046-scenario-open-question-visibility, 047-analyze-findings-durability, 048-govern-acquired-runtime, 049-rename-govern-to-ductus, 050-constitution, 051-branch-scoped-spec-numbering, 054-remove-supersession, 055-shared-constitution, 057-analyze-artifact-and-record-relocation, 058-findings-route-at-discovery, 060-exec-analyze-assesses-each-loaded-rule, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 071-command-reference-split, 072-governing-documents-stay-within-one-read, 075-agents-md-holds-what-every-session-needs, 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-scope-includes-uncommitted-work.md` — 000-slash-commands, 011-brownfield-process, 016-cross-cutting-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 035-groom-session-target, 041-task-pruning, 045-decision-state-drift-detection, 046-scenario-open-question-visibility, 047-analyze-findings-durability, 048-govern-acquired-runtime, 049-rename-govern-to-ductus, 050-constitution, 051-branch-scoped-spec-numbering, 054-remove-supersession, 055-shared-constitution, 057-analyze-artifact-and-record-relocation, 058-findings-route-at-discovery, 060-exec-analyze-assesses-each-loaded-rule, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 071-command-reference-split, 072-governing-documents-stay-within-one-read, 075-agents-md-holds-what-every-session-needs, 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-scope-plan-affected-table-format.md` — 000-slash-commands, 011-brownfield-process, 016-cross-cutting-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 035-groom-session-target, 041-task-pruning, 045-decision-state-drift-detection, 046-scenario-open-question-visibility, 047-analyze-findings-durability, 048-govern-acquired-runtime, 049-rename-govern-to-ductus, 050-constitution, 051-branch-scoped-spec-numbering, 054-remove-supersession, 055-shared-constitution, 057-analyze-artifact-and-record-relocation, 058-findings-route-at-discovery, 060-exec-analyze-assesses-each-loaded-rule, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 071-command-reference-split, 072-governing-documents-stay-within-one-read, 075-agents-md-holds-what-every-session-needs, 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-scope-union.md` — 000-slash-commands, 011-brownfield-process, 016-cross-cutting-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 035-groom-session-target, 041-task-pruning, 045-decision-state-drift-detection, 046-scenario-open-question-visibility, 047-analyze-findings-durability, 048-govern-acquired-runtime, 049-rename-govern-to-ductus, 050-constitution, 051-branch-scoped-spec-numbering, 054-remove-supersession, 055-shared-constitution, 057-analyze-artifact-and-record-relocation, 058-findings-route-at-discovery, 060-exec-analyze-assesses-each-loaded-rule, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 071-command-reference-split, 072-governing-documents-stay-within-one-read, 075-agents-md-holds-what-every-session-needs, 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/066-audit-records-and-the-completion-gate/scenarios/the-committed-tree-horizon.md` — 000-slash-commands, 011-brownfield-process, 016-cross-cutting-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 035-groom-session-target, 041-task-pruning, 045-decision-state-drift-detection, 046-scenario-open-question-visibility, 047-analyze-findings-durability, 048-govern-acquired-runtime, 049-rename-govern-to-ductus, 050-constitution, 051-branch-scoped-spec-numbering, 054-remove-supersession, 055-shared-constitution, 057-analyze-artifact-and-record-relocation, 058-findings-route-at-discovery, 060-exec-analyze-assesses-each-loaded-rule, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 071-command-reference-split, 072-governing-documents-stay-within-one-read, 075-agents-md-holds-what-every-session-needs, 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/066-audit-records-and-the-completion-gate/scenarios/the-fix-round-loop-is-recorded-and-gated.md` — 000-slash-commands, 011-brownfield-process, 016-cross-cutting-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 035-groom-session-target, 041-task-pruning, 045-decision-state-drift-detection, 046-scenario-open-question-visibility, 047-analyze-findings-durability, 048-govern-acquired-runtime, 049-rename-govern-to-ductus, 050-constitution, 051-branch-scoped-spec-numbering, 054-remove-supersession, 055-shared-constitution, 057-analyze-artifact-and-record-relocation, 058-findings-route-at-discovery, 060-exec-analyze-assesses-each-loaded-rule, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 071-command-reference-split, 072-governing-documents-stay-within-one-read, 075-agents-md-holds-what-every-session-needs, 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/066-audit-records-and-the-completion-gate/scenarios/write-analysis-and-the-second-gate.md` — 000-slash-commands, 011-brownfield-process, 016-cross-cutting-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 035-groom-session-target, 041-task-pruning, 045-decision-state-drift-detection, 046-scenario-open-question-visibility, 047-analyze-findings-durability, 048-govern-acquired-runtime, 049-rename-govern-to-ductus, 050-constitution, 051-branch-scoped-spec-numbering, 054-remove-supersession, 055-shared-constitution, 057-analyze-artifact-and-record-relocation, 058-findings-route-at-discovery, 060-exec-analyze-assesses-each-loaded-rule, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 071-command-reference-split, 072-governing-documents-stay-within-one-read, 075-agents-md-holds-what-every-session-needs, 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/066-audit-records-and-the-completion-gate/spec.md` — 000-slash-commands, 011-brownfield-process, 016-cross-cutting-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 035-groom-session-target, 041-task-pruning, 045-decision-state-drift-detection, 046-scenario-open-question-visibility, 047-analyze-findings-durability, 048-govern-acquired-runtime, 049-rename-govern-to-ductus, 050-constitution, 051-branch-scoped-spec-numbering, 054-remove-supersession, 055-shared-constitution, 057-analyze-artifact-and-record-relocation, 058-findings-route-at-discovery, 060-exec-analyze-assesses-each-loaded-rule, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 071-command-reference-split, 072-governing-documents-stay-within-one-read, 075-agents-md-holds-what-every-session-needs, 078-review-fix-rounds-are-re-read-within-a-bound

## Unexamined governance

*None.*
