---
spec: 020-code-review
last-run: 2026-11-22T00:23:01Z
reviewed-against: 56b1b2fc6219262bda6b43657cf1cd12c2cc1a5e
diff-base: d5fc51ecea81c9e27952d521a869e42a21de7a48
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 12
scope: 15
unread:
  - .claude/commands/ductus/review.md
  - .ductus/reference/review.md
  - .pi/prompts/ductus-review.md
excluded: 89
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 1625833bc1e1572da569f7c582ededc4ea9b59efe55b3f6d459efb59becd9632
  scenarios/every-loaded-rule-file-has-a-pass.md: 1f684e5b4c4c3d6aa764a4744f27fda185952ca95ae7743de1190e042337b78b
  scenarios/review-flag-parsing-is-specified.md: a19fe0f4f50c954363f7cca4d913a81d96374535d31a986792243be5263631c1
  scenarios/review-reports-a-defect-under-its-owner.md: ef91f01e38d96520853db62f5ee62b9b8893d41582f055f4f01a25d5a796b665
  scenarios/waiver-expiry.md: 0c35cbf8e26a8a6f321c8549b7209c61b37122713bf13270e5b3c972b788f4ed
  scenarios/waiver-file-lists.md: 8b4180d64e4e43ac6a66f57df7c65d6f1f00bb343ae284bb2d7214c88c824f53
criteria-judged: 18
criteria-met: 18
criteria-verdicts:
  - label: AC1
    digest: ce5aa4324d3c5074cc992c77c77fc6544943098ec77457d676bcc21ec6a66c22
  - label: AC2
    digest: 22cf445c79f82fa98f79b09465401f9155f3431b05aed6c19633625eadb29bf3
  - label: AC3
    digest: 438dcd405d7ba95436ed06918be09abda1b2afff3bfa076dfb3be8fbaea3dd96
  - label: AC4
    digest: 907261b65da49141dcbb3021c17ecac2c1adcce25188827f89c2341f00a806ad
  - label: AC5
    digest: 0d8c8bc9088e35ddcf0e4b57f678f1e97831ed39f0821096ab56f9389aacf9a9
  - label: AC6
    digest: fe089c02065e87955465569fdb1a5c91c34ac6507c9c0fa40e17363b81bc4c5d
  - label: AC7
    digest: 4146048a1a2e934fdea9509467d09e41c8e2a23f443ba2b1955055b807d9865b
  - label: AC8
    digest: bbb3164a35920f2fb5ba6e7156315b981715fd2342ff5e815608887c5d41cddb
  - label: AC9
    digest: 5fd3f43cf7b723d5919e1941b4d25b0d78ce13e4b9778b1a3038f4b76b97ee31
  - label: AC10
    digest: c0e2b6fb7c2e4452b23eba54a1f40047298aae8bff0e1f0d917a8460dceccc4a
  - label: AC11
    digest: 5dcb8b369ecbc9ea769332a320af5b9d1d83a98e5ff4ff16814bca49124acf93
  - label: AC12
    digest: 287844dca2caa9884aa8e95b6718c9bdbbcb008945a74cbdcffa23fd9abf8897
  - label: AC13
    digest: ef394ce08f85cbe1ca88bb0248d34c5674eeb4085c413038dcb49a0d5bdf2ffc
  - label: AC14
    digest: ad54c90154031e3864972c5bbd26b7ec784d0c2a25fe1f895f71e6d80b803c73
  - label: AC15
    digest: 1f49d9de76d6223d63204b1ebe837a831e706d6b4a51c87badc7a5da6a1ee3ef
  - label: AC16
    digest: 08d9a0ed5efc065e5ccfca189c8b754468e218e3eefaafa73f2dc44b49b8dca6
  - label: AC17
    digest: c162674202583e492ffdb1b8fe4905bb3fd82ab6ca0cb01c11b2a62716c22c26
  - label: AC18
    digest: de7805675a6786eee063eb29aea997db67f39ab6335f6db645cdf769ecb58429
blocking: false
dispositions:
  fixed: 4
  deferred: 0
  waived: 5
  undispositioned: 0
decisions:
  - key: "other — the Context of scenarios every-loaded-rule-file-has-a-pass and review-reports-a-defect-under-its-owner states in the present tense what task 14 and task 15 changed: that the reference names a rule file in only one pass, that the quality pass's definition does not say it enforces `quality-cross.md`, that `/review` applies no Not-flagged clause, and that 18 yields sit in files no pass applies today. — `specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md:9`"
    outcome: waived
    reason: Each Context records the defect as found on a stated date (2026-10-19, 2026-11-02, anchored to ab2ad085 in the Resolved Questions), the Resolved Questions and Behavior state the requirement, and rewording the found-state changes no requirement.
    decided-at: 2026-11-15T15:51:01Z
    decided-by: rule
  - key: other — plan.md's first Technical Decision says `/ductus:review` has no new code and no new runtime, and its Overview says the same, while the plan's later decisions record the runtime half (`process-waivers`, `review_pass_for_rule_file`) and §Plan records the design as it stands. — `specs/020-code-review/plan.md:9`
    outcome: waived
    reason: Rewriting the Overview and the first decision into the runtime-backed design is a design-record edit, not a mechanical chore, and the plan's later decisions state the runtime half.
    decided-at: 2026-11-15T15:51:01Z
    decided-by: rule
  - key: "other — the unapplied-file naming in the Summary on a restricted run rests on the reviewer writing it: `write-review`'s `## Skipped passes` lists passes only, and nothing derives the rule files the skipped passes would have applied, although `review_pass_for_rule_file` maps them. — `framework/commands/review.md:125`"
    outcome: waived
    reason: The record's `skipped-passes` plus the §Load rules mapping let a reader derive the unapplied files, so the record is not indistinguishable from a clean one; deriving them in write-review is new runtime requirement, not a chore.
    decided-at: 2026-11-15T15:51:01Z
    decided-by: rule
  - key: other — spec.md's Tasks list still directs edits to a `review:` block in spec frontmatter and the spec template, which 057 removed — `specs/020-code-review/spec.md`
    outcome: waived
    reason: The list records what 020 delivered, and the relocation is stated beside it, in the 057 note and the Frontmatter schema section; rewording a delivered task list changes no requirement and makes no claim about the tree.
    decided-at: 2026-11-19T02:29:42Z
    decided-by: rule
  - key: "other: discover-rule-files' header test runs only over the rule-file directory's selection; a file an `AGENTS.md` reference adds under `extra-files` (outside the directory) is never header-tested, so one whose header names analyze alone still gets the quality pass, which AC17's stated purpose (the two audits never judge one rule differently) would not predict. The spec's AC17 speaks of the review's selection only, so no criterion is broken; low confidence that anyone intended it. — `runtime/src/primitives/discover_rule_files.rs`"
    outcome: waived
    reason: An extra-files entry is a rule file the project's AGENTS.md names by path, outside the rule-file directory AC17 governs; the header test would only drop a file a review now reads, so leaving it in costs one more pass and loses nothing.
    decided-at: 2026-11-22T00:22:28Z
    decided-by: rule
---

# Review — 020-code-review

## Summary

Five-pass review of 020's reopening (diff-base d5fc51ec, base-source reopen) at HEAD 56b1b2fc. Its code commits are 925c6905 (task 18, AC17: a review leaves out a rule file whose header names `/ductus:analyze` alone) and ec080d78 (task 19, AC18: a finding needs a reachable trigger), with the spec-only cdee8ab0, 91dd60ea and f84932e8. Passes (part P1, one reviewer for 020 and the eight specs 925c6905 reopened) read those commits' hunks in each scope file, and the files whole where a behavior needed context, against the five rule files `discover-rule-files` selected (232,805 bytes, each read whole against `wc -c`): no finding, five observations. Fixed in this run: two tests in `runtime/src/primitives/discover_rule_files.rs` prove that a disabled file left to analysis is named disabled and that a file the header test cannot read stays selected, behaviors the description and plan stated untested, with six mutants in `scripts/mutation/rule-files-left-to-analysis-020.py`, each killed; the review reference's Notes for adopters and `docs/rules.md` now say which command judges each rule file; the plan's Affected Files gains `docs/rules.md`, the CHANGELOG and the review golden. Waived by the rule: an `extra-files` entry is never header-tested. The four stored waivers still apply. P1 re-ran its passes over the fix. The criteria step (part C1) judged every criterion individually: 18 of 18 met, ten driven with the release binary on fixtures. Examined 12 of 15, the new mutation list included; unread: the three generated copies (`.claude/commands/ductus/review.md`, `.ductus/reference/review.md`, `.pi/prompts/ductus-review.md`), confirmed in sync by `scripts/gen-claude-commands.sh --check`. The deleted path `specs/083-review-and-analysis-check-each-other/spec.md` is the retired 083's, removed by c5da762d, and no pass read it.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

18 of 18 acceptance criteria judged met.

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

- AC1 — met: `framework/commands/review.md` is the verb-named command (argument-hint, Instructions steps 1-12); generated copies `.claude/commands/ductus/review.md` and `.pi/prompts/ductus-review.md` exist; `runtime/tests/review_command.rs` `review_command_parses_to_the_expected_procedure` parses it as analyze's shape is parsed.
- AC2 — met: Drove write-review on a fixture: `specs/001-demo/review.md` rendered '## MUST violations (blocking)', '## SHOULD violations (advisory)' and '## Low-confidence findings' (a low-confidence MUST landed under Low-confidence, counts must `0/should` `1/low` 1); render in `runtime/src/primitives/write_review.rs.`
- AC3 — met: discover-rule-files on the real tree selects `security-backend.md`, `security-frontend.md`, `security-cross.md` plus `configuration-cross.md` and `quality-cross.md`; command steps 3-7 run the five passes (security, reuse, quality, efficiency, simplicity); `review_pass_for_rule_file` in `runtime/src/interpreter/payload.rs` maps security-* to the security pass. `Stack/surface` filtering is the Resolved Question's, not a defect.
- AC4 — met: write-review wrote last-run, must-violations, should-violations, blocking and waivers into `review.md`'s own frontmatter and left `spec.md` untouched; `framework/templates/spec/spec.md` carries status and dependencies only; validate-frontmatter on a spec with a residual review: block returned a blocking finding 'review: no longer belongs in spec frontmatter'.
- AC5 — met: All three driven on a fixture: check-review-gate blocked on must-violations, on no `review.md` (not-reviewed) and on an unparseable record (record-unreadable, undeterminable); check-artifacts reported review-state-drift (blocking) on a done spec with blocking true and validate-frontmatter a hard-fail on the unparseable record; the CI template's audit-record gate step, extracted and run, failed on blocking:true, on an unparseable `review.md`, and on an exempt set (1) above its baseline (0).
- AC6 — met: Two write-review calls with identical arguments and hours-apart reviewed-at produced `review.md` files whose only diff was last-run (reviewed-against and diff-base were passed identical).
- AC7 — met: `framework/reference/review.md` 'Apply --fix' and 'Auto-fix scope' (never auto-apply behavior, status-code, message or schema changes; contained to scope), applied by command step 11; a host-judgment contract with no runtime path to drive, so judged by reading the command and reference, no test.
- AC8 — met: Drove write-review new-waivers (waiver entry with rule, file, reason, waived-at, waived-by landed in `review.md` frontmatter), process-waivers (expired with a 'waiver expired:' notice when the rule no longer fired at the file; applied when it fired), and a hand-added co-waived-by field survived a re-render (RawWaiverFull #[serde(flatten)] in `write_review.rs`).
- AC9 — met: write-review's result carried exit-code 1 with blocking true and 0 otherwise while the CLI process exited 0 both times, as the criterion's parenthetical says; CI gates on the record (AC5's gate step).
- AC10 — met: `README.md` Commands section lists /review under '- **Pipeline — advance state**' with a one-line purpose and an anchor that resolves to `docs/slash-commands.md` '### /review — `review.md`, and a hold on done while violations stand'; commits 30e1fbaa (2026-05-10) and 1ffcef35 (2026-08-30) both exist as the parenthetical says.
- AC11 — met: `framework/commands/review.md` step 1 and `framework/reference/review.md` 'Resolve target and scope' item 4 halt on a `missing/empty` or inconsistent Tech Stack with the tech-stack-misalignment message, offer to persist [review] tech-stack-verified, and skip the check when it is true; the key is registered in `runtime/src/schema/config_keys.rs.` Host-judgment step, so no driven fixture.
- AC12 — met: write-review --empty-scope on a fixture spec wrote a `review.md` with 0 findings, blocking false and result exit-code 0 ('Review scope is empty ... blocking: no'); a refusal guards an earlier record counting a finding.
- AC13 — met: `dedup_findings` in `runtime/src/primitives/write_review.rs`: a SHOULD (lines 1-5) and a MUST (3-8) on the same rule and file collapsed to the single MUST (must-violations 1, should-violations 0 in the record and report); the (rule, file, overlapping range) enumeration matches `ranges_overlap`.
- AC14 — met: `framework/commands/review.md` 'Parsing $ARGUMENTS' specifies every flag, a bare --since and an unrecognized --flag are reported and stop; its argument-hint names --all --fix --security --simplicity --quality --since --waive --defer (--target --task --work-item --reason); check-command-flags returned findings 0 (13 commands with a hint, 3 with a Flags table) and `scripts/audit/command-flag-hint-parity.sh` exited 0.
- AC15 — met: write-review recorded examined and a self-derived scope in `review.md` (examined 1 / scope 1); with --examined omitted the field was absent from the record, with --examined 0 it read 'examined: 0', so absent is not zero; spec's superseded half (no check reads it) is stated in the criterion itself.
- AC16 — met: `framework/commands/review.md` step 3 and `framework/reference/review.md` 'Passes in a subagent' state the fresh-subagent run of steps 3 to 7.2 with the hand-back and the short-read rule (kept out of examined); `review_command_parses_to_the_expected_procedure` fixes steps 3-7 as performReview and 7.2 as verifyCriteria, and `interpreter/mod.rs` `perform_review_emits_one_llm_request_per_pass_step` and `the_review_walk_judges_every_criterion_and_writes_its_tick` show exec asks each pass in turn, then the criteria.
- AC17 — met: Drove discover-rule-files on a fixture (analyze-only header, header naming both, header naming neither): the analyze-only file was left out of a review with the notice 'rule file `concurrency-backend.md` is judged by analysis alone: its header names the analyze command and not review, so review does not load it', the other two kept, --all loaded all three; on the real tree seven analyze-only files are left out; `leave_to_analysis/header_leaves_to_analysis` in `discover_rule_files.rs` with test `a_review_leaves_out_each_rule_file_whose_header_names_analyze_alone` holding a fixture of each kind.
- AC18 — met: `framework/reference/review.md` '## Run review passes' states a finding, observation or claim is recorded only when its trigger is shown possible from the project's code, configuration, history or deployment shape, plus mechanism checked against the tree, and that a case shown only by building it is a false positive recorded nowhere and fixed in no round (spec 020, AC18); 'Passes in a subagent' requires the hand-back to carry that evidence and drops one without it. Prose contract, no test.

## Observations

- other: discover-rule-files' header test runs only over the rule-file directory's selection; a file an `AGENTS.md` reference adds under `extra-files` (outside the directory) is never header-tested, so one whose header names analyze alone still gets the quality pass, which AC17's stated purpose (the two audits never judge one rule differently) would not predict. The spec's AC17 speaks of the review's selection only, so no criterion is broken; low confidence that anyone intended it. — `runtime/src/primitives/discover_rule_files.rs` — **waived**: An extra-files entry is a rule file the project's AGENTS.md names by path, outside the rule-file directory AC17 governs; the header test would only drop a file a review now reads, so leaving it in costs one more pass and loses nothing. (decided by the rule)
- other: `leave_to_analysis`' doc comment, the `DISCOVER_RULE_FILES` description and `plan.md` all state that a file that cannot be read stays selected, and the disabled-filter-before-header ordering, but the three new tests cover neither (they read only well-formed files); QUAL-TEST-001 is not met by a missing test, so recorded as a coverage gap in a claim the description makes. — `runtime/src/primitives/discover_rule_files.rs` — **fixed** (decided by the rule)
- convention: `framework/reference/review.md`'s Notes for adopters still says adding a file at `specs/rules/<domain>-{backend,frontend,cross}.md` is the only step and the suffix selects which stacks load it, and still offers disabling an accessibility file (an internal admin UI not yet at WCAG AA) as the example of `[[review.disabled-rule-files]]`; since task 18 a file whose header names analyze alone is not loaded at all, so the first sentence omits the header test and the example's file is never loaded by a review. — `framework/reference/review.md` — **fixed** (decided by the rule)
- convention: `docs/rules.md`'s opening ("the rules /review holds code to", "/review audits the implementation against them") and its What ships table present all twelve shipped files as held by /review, though seven are analysis-only since task 18; only the one sentence under Which rules load states the exception. — `docs/rules.md` — **fixed** (decided by the rule)
- convention: `plan.md`'s Affected Files listed `discover_rule_files.rs` and `descriptions.rs` but not `docs/rules.md`, `runtime/CHANGELOG.md` or `runtime/tests/golden/review-basic.jsonl`, which 925c6905 also edited — `specs/020-code-review/plan.md` — **fixed** (decided by the rule)
- other — the Context of scenarios every-loaded-rule-file-has-a-pass and review-reports-a-defect-under-its-owner states in the present tense what task 14 and task 15 changed: that the reference names a rule file in only one pass, that the quality pass's definition does not say it enforces `quality-cross.md`, that `/review` applies no Not-flagged clause, and that 18 yields sit in files no pass applies today. — `specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md:9` — **waived**: Each Context records the defect as found on a stated date (2026-10-19, 2026-11-02, anchored to ab2ad085 in the Resolved Questions), the Resolved Questions and Behavior state the requirement, and rewording the found-state changes no requirement. (decided by the rule)
- other — plan.md's first Technical Decision says `/ductus:review` has no new code and no new runtime, and its Overview says the same, while the plan's later decisions record the runtime half (`process-waivers`, `review_pass_for_rule_file`) and §Plan records the design as it stands. — `specs/020-code-review/plan.md:9` — **waived**: Rewriting the Overview and the first decision into the runtime-backed design is a design-record edit, not a mechanical chore, and the plan's later decisions state the runtime half. (decided by the rule)
- other — the unapplied-file naming in the Summary on a restricted run rests on the reviewer writing it: `write-review`'s `## Skipped passes` lists passes only, and nothing derives the rule files the skipped passes would have applied, although `review_pass_for_rule_file` maps them. — `framework/commands/review.md:125` — **waived**: The record's `skipped-passes` plus the §Load rules mapping let a reader derive the unapplied files, so the record is not indistinguishable from a clean one; deriving them in write-review is new runtime requirement, not a chore. (decided by the rule)
- other — spec.md's Tasks list still directs edits to a `review:` block in spec frontmatter and the spec template, which 057 removed — `specs/020-code-review/spec.md` — **waived**: The list records what 020 delivered, and the relocation is stated beside it, in the 057 note and the Frontmatter schema section; rewording a delivered task list changes no requirement and makes no claim about the tree. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md`
- `.ductus/reference/analyze.md`
- `.pi/prompts/ductus-analyze.md`
- `docs/analyze.md`
- `docs/developing-ductus.md`
- `docs/shared-constitution.md`
- `framework/bootstrap/ductus.md`
- `framework/bootstrap/govern.md`
- `framework/commands/analyze.md`
- `framework/constitution.md`
- `framework/migrations/remove-legacy-constitutions-region.md`
- `framework/reference/analyze.md`
- `runtime/src/interpreter/analyze_tally.rs`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/primitives/analyze_subjects.rs`
- `runtime/src/primitives/brief_re_read.rs`
- `runtime/src/primitives/fix_rounds.rs`
- `runtime/src/primitives/resolve_anchor.rs`
- `runtime/src/primitives/rule_verdicts.rs`
- `runtime/src/primitives/start_fix_rounds.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/tests/line_ending_discipline.rs`
- `scripts/analyze/briefs.py`
- `scripts/analyze/check.py`
- `scripts/analyze/detect.py`
- `scripts/analyze/judge-brief.md`
- `scripts/analyze/record.py`
- `scripts/analyze/split.py`
- `scripts/analyze/step15-brief.md`
- `scripts/mutation/analysis-design-fix-067.py`
- `scripts/mutation/command-reference-split.py`
- `scripts/mutation/criterion-ticks-outside-the-digest.py`
- `scripts/mutation/exec-scenario-anchors-067.py`
- `scripts/mutation/owner-reviews-067.py`
- `scripts/mutation/rule-verdicts-converge.py`
- `scripts/tests/test-analyze-scripts.py`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/024-rule-loader/review.md`
- `specs/024-rule-loader/spec.md`
- `specs/034-performance-backend-rules/review.md`
- `specs/034-performance-backend-rules/spec.md`
- `specs/037-observability-backend-rules/plan.md`
- `specs/037-observability-backend-rules/review.md`
- `specs/037-observability-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/plan.md`
- `specs/038-concurrency-backend-rules/review.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/039-reliability-backend-rules/plan.md`
- `specs/039-reliability-backend-rules/review.md`
- `specs/039-reliability-backend-rules/spec.md`
- `specs/050-constitution/spec.md`
- `specs/055-shared-constitution/plan.md`
- `specs/055-shared-constitution/spec.md`
- `specs/055-shared-constitution/tasks.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/068-api-backend-rules/review.md`
- `specs/068-api-backend-rules/spec.md`
- `specs/069-accessibility-frontend-rules/review.md`
- `specs/069-accessibility-frontend-rules/spec.md`
- `specs/070-performance-frontend-rules/review.md`
- `specs/070-performance-frontend-rules/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md`
- `specs/079-review-and-analysis-work-converges/analysis.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/research.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/082-rule-assessments-converge-across-runs/analysis.md`
- `specs/082-rule-assessments-converge-across-runs/data-model.md`
- `specs/082-rule-assessments-converge-across-runs/plan.md`
- `specs/082-rule-assessments-converge-across-runs/review.md`
- `specs/082-rule-assessments-converge-across-runs/spec.md`
- `specs/082-rule-assessments-converge-across-runs/tasks.md`
- `specs/inbox.md`

## Unexamined governance

*None.*
