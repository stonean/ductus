---
spec: 020-code-review
last-run: 2026-11-19T22:35:55Z
reviewed-against: 5de7c1f98d3545515a679f09b569df3260be2644
diff-base: ba3c205e68dd43e4ae6c746c62ca52161d905fb0
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 5
unread: []
excluded: 44
contract-breaks: 0
skipped-passes: []
reviewed-digest:
  data-model.md: 1625833bc1e1572da569f7c582ededc4ea9b59efe55b3f6d459efb59becd9632
  scenarios/every-loaded-rule-file-has-a-pass.md: 4e31f3bd59ad2c2145b8cb24ceaf8561580ffab2ad3143de1087f65b5109a0f3
  scenarios/review-flag-parsing-is-specified.md: a19fe0f4f50c954363f7cca4d913a81d96374535d31a986792243be5263631c1
  scenarios/review-reports-a-defect-under-its-owner.md: ef91f01e38d96520853db62f5ee62b9b8893d41582f055f4f01a25d5a796b665
  scenarios/waiver-expiry.md: 0c35cbf8e26a8a6f321c8549b7209c61b37122713bf13270e5b3c972b788f4ed
  scenarios/waiver-file-lists.md: 8b4180d64e4e43ac6a66f57df7c65d6f1f00bb343ae284bb2d7214c88c824f53
criteria-judged: 16
criteria-met: 16
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
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 2
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
  - key: "other — the `[review] tech-stack-verified` halt is carried by step 1's tech-stack branch, so the `review-owed: false` stop and the empty-scope branch never read the key, while the Config bullet and the data model say the halt comes at step 1 before any pass; the claim is wider than the branch that carries it. — ``framework/commands/review.md``"
    outcome: waived
    reason: The empty branches write no alignment decision and read no flag, so no malformed value is read as `false` there and CFG-ENV-008's no-silent-fallback holds; step 1's own sentence says the halt is where the flag is read, and rewording the claim is not mechanical enough to spend a fix round on a low-confidence note.
    decided-at: 2026-11-19T22:35:21Z
    decided-by: rule
  - key: other — the sentence added to the `[review] tech-stack-verified` Config bullet leaves its first line longer than the neighbouring bullets' wrap. — ``framework/commands/review.md``
    outcome: waived
    reason: Cosmetic only, the markdown lint passes, and a rewrap would start a fix round and regenerate two copies for no requirement.
    decided-at: 2026-11-19T22:35:21Z
    decided-by: rule
---

# Review — 020-code-review

## Summary

Window ba3c205e (base-source reopen) to HEAD. Scope 5 entries: framework/commands/review.md, specs/020-code-review/data-model.md and specs/020-code-review/spec.md, each read whole after its window diff, and the two generated copies .claude/commands/ductus/review.md and .pi/prompts/ductus-review.md, read as their window diffs (each equal to the source's change), not whole, so examined is 5 of 5 on that reading. The window's change: this run's own fix(020) commit making a present non-boolean [review] tech-stack-verified halt step 1 (CFG-ENV-008, found by analyze), the waiver example's rule IDs, the Excluded from scope row's pointer to write-review's description, and the status flip. The three security rule files (194,749 bytes) were read whole by the security pass and the nine other rule files, AGENTS.md's Gotchas to Design Principles and system.md by the other four passes: no finding. The criteria step judged all 16 criteria one by one: 16 met (AC9's exit status taken from the criterion's own annotation and write_review.rs tests, not driven). Two NOTE observations, both waived by rule; two stored decisions retained as still holding, two retained as naming files the scope does not hold. No fix round: this run wrote no fix after start-fix-rounds' base. Unexamined: framework/reference/review.md section 'Resolve target and scope' item 4 was not edited and does not mention the new halt.

16 of 16 acceptance criteria judged met.

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

- AC1 — met: framework/commands/review.md (frontmatter, Purpose/Instructions in /analyze shape); generated copies .claude/commands/ductus/review.md and .pi/prompts/ductus-review.md carry the same diff.
- AC2 — met: write-review (runtime/src/primitives/write_review.rs) writes specs/NNN/review.md with MUST/SHOULD/low-confidence buckets (low-confidence field; step 11 of command).
- AC3 — met: command step 2 discover-rule-files + step 3 security pass over loaded security-*.md; five passes steps 3-7; framework/reference/review.md §Load rules (surface selection, the cross files always).
- AC4 — met: record fields last-run, must-violations, should-violations, blocking, waivers live in review.md frontmatter (write_review.rs), as the relocation annotation states.
- AC5 — met: check-review-gate (implement step 11), analyze review-state-drift, framework/templates/ci/adopter-generators.yml audit-record gate (comments lines 15-29, 152-167 describe review.md reads and the baseline).
- AC6 — met: runtime test idempotent_rerun_reproduces_identical_review_md in write_review.rs.
- AC7 — met: framework/reference/review.md "## Auto-fix scope"; command Flags row --fix.
- AC8 — met: process-waivers + process_waivers.rs tests expires_when_file_is_gone, expires_when_rule_no_longer_fires, code_moving_within_file_does_not_expire; write_review.rs prune_expired and test expired_waiver_pruned_from_spec_frontmatter; scenarios/waiver-expiry.md exists.
- AC9 — met: write_review.rs result exit_code = i32::from(blocking), tests assert 0 and 1 (e.g. result.exit_code == 1); CI gates on the record. I did not drive the CLI to confirm it exits 0 on a blocking record; the annotation states it and I take it as stated.
- AC10 — met: README.md Commands list, "**Pipeline - advance state**" at line 93 names /review with one-line purpose (line 98).
- AC11 — met: reference §Resolve target and scope item 4 (tech-stack check, missing/empty/inconsistent halts, exit 1, gate-confirm persist, no auto-reset); command text (framework/commands/review.md Config bullet and step 1) skips when [review] tech-stack-verified is true and now halts on a present non-boolean value before any pass naming key, file and value; data-model.md states the same (CFG-ENV-008); both generated copies carry the change (2 matching diff lines each). Note: the reference's item 4 does not mention the non-boolean halt, only the command and data-model do; not a criterion failure.
- AC12 — met: write_review.rs empty-scope branch (module doc; AC12 test "the row renders on an empty scope too"); command step 1 scope-count zero path; blocking only through a criterion not met.
- AC13 — met: write_review.rs dedup_findings, tests cross_pass_dedup_highest_severity_wins and dedup_keeps_non_overlapping_same_rule_file.
- AC14 — met: command "### Parsing $ARGUMENTS" section, Flags table, argument-hint lists every flag (--all --fix --security|--simplicity|--quality --since --waive --defer ...); scripts/audit/command-flag-hint-parity.sh holds hint and table in agreement; scenario review-flag-parsing-is-specified.md exists.
- AC15 — met: write_review.rs records examined (args.examined) against a self-derived scope; annotation on supersession matches.
- AC16 — met: command step 3 (llm:performReview) states the subagent split and step 7.3 continuation; framework/reference/review.md "### Passes in a subagent" says short reads are named in Summary and kept out of examined; Pi runs it itself.

## Observations

- other — the `[review] tech-stack-verified` halt is carried by step 1's tech-stack branch, so the `review-owed: false` stop and the empty-scope branch never read the key, while the Config bullet and the data model say the halt comes at step 1 before any pass; the claim is wider than the branch that carries it. — ``framework/commands/review.md`` — **waived**: The empty branches write no alignment decision and read no flag, so no malformed value is read as `false` there and CFG-ENV-008's no-silent-fallback holds; step 1's own sentence says the halt is where the flag is read, and rewording the claim is not mechanical enough to spend a fix round on a low-confidence note. (decided by the rule)
- other — the sentence added to the `[review] tech-stack-verified` Config bullet leaves its first line longer than the neighbouring bullets' wrap. — ``framework/commands/review.md`` — **waived**: Cosmetic only, the markdown lint passes, and a rewrap would start a fix round and regenerate two copies for no requirement. (decided by the rule)

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
- `runtime/CHANGELOG.md`
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
- `specs/022-deterministic-runtime/tasks.md`
- `specs/026-framework-self-audit/scenarios/families-follow-the-command-reference-split.md`
- `specs/026-framework-self-audit/scenarios/family-34-reference-section-reachability.md`
- `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/026-framework-self-audit/tasks.md`
- `specs/041-task-pruning/tasks.md`
- `specs/051-branch-scoped-spec-numbering/tasks.md`
- `specs/057-analyze-artifact-and-record-relocation/tasks.md`
- `specs/066-audit-records-and-the-completion-gate/tasks.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-bare-port-span-names-its-remedy.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/071-command-reference-split/scenarios/reference-sections-are-reached-by-a-step.md`
- `specs/071-command-reference-split/tasks.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/tasks.md`
- `specs/080-deterministic-runtime-efficiency/tasks.md`
- `specs/081-reviews-and-checks-follow-what-changed/spec.md`

## Unexamined governance

*None.*
