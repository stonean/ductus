---
spec: 020-code-review
last-run: 2026-11-19T02:29:42Z
reviewed-against: 59d02adcf4fff32d4988d43b77fb5df859ca7582
diff-base: 7dbd960507100f7d0bf34b17626405604e9ea012
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 2
scope: 6
unread:
  - framework/reference/review.md
  - runtime/src/primitives/write_review.rs
  - runtime/src/schema/primitives.rs
  - specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md
excluded: 98
contract-breaks: 0
skipped-passes: []
fix-rounds: 2
re-reads: 2
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 61bef581d1ee7851e3fa31b2f17c91151a1ff47ed62295c9f6cf885d17624893
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
  fixed: 7
  deferred: 0
  waived: 1
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
---

# Review — 020-code-review

## Summary

Window 7dbd9605 (base-source reopen, 066 task 27a) to 59d02adc. The window held two artifacts, spec.md (27,002 bytes) and data-model.md (15,522), each read whole by the passes; the other four scope entries are this run's own fixes (framework/reference/review.md, runtime/src/primitives/write_review.rs, runtime/src/schema/primitives.rs, 022's scenario review-runtime-acceleration), uncommitted when written, which the passes did not read and the fix rounds re-read, so examined is 2 of 6 and unread names those four. The nine backend and cross rule files were read whole (285,795 bytes) and all five passes ran: no finding. The criteria step judged all 16 criteria one by one against the tree and history. AC8, AC9, AC10 and AC16 were judged not met first: AC8, AC10 and AC16 were true when written and superseded (two of AC8's three flatten parsers went with check-review-agreement; 1ffcef35 made the README's Commands section a list; 066 task 27 added the criteria step to the subagent), and their annotations were corrected by rule; AC9 never held as a process exit status, and the owner chose correcting the claim, with framework/reference/review.md §Output corrected to match. All four were judged met on their new text. Seven observations were fixed, round 1's NOTE among them, and one waived by rule; three stored decisions are retained, each keyed to a file outside the scope. Unexamined: the decisions schema field by field against 058 and 076, the --all Resolved Question's quote of the old drift predicate, and AC7's --fix, which edits code only in a host.

2 fix round(s): 2 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

- AC1 — met: framework/commands/review.md exists with frontmatter description/argument-hint like analyze.md; .claude/commands/ductus/review.md is tracked and scripts/gen-claude-commands.sh --check exits 0 (in sync)
- AC2 — met: driven: runtime/target/release/ductus write-review on a fixture wrote specs/001-demo/review.md with ## MUST violations (blocking), ## SHOULD violations (advisory), ## Low-confidence findings holding a MUST, a SHOULD and a low-confidence finding respectively
- AC3 — met: discover-rule-files on this repo (no [rules] surfaces) selects security-backend.md and security-frontend.md (plus security-cross.md) as authoritative; selection is now suffix/[rules] surfaces/stack-driven so a backend-only project loads only the backend file, as the spec's own Resolved Questions stack-detection answer says; five passes are steps 3-7 of framework/commands/review.md
- AC4 — met: relocation holds: write-review writes last-run, must-violations, should-violations, blocking, waivers into review.md frontmatter only (fixture driven); framework/templates/spec/spec.md ships status+dependencies only; validate-frontmatter --path on a spec.md with a residual review: block returns severity blocking
- AC5 — met: driven: check-review-gate gives not-reviewed (absent review.md), record-unreadable (unparseable) and must-violations (blocking true); check-artifacts review-state-drift blocking on done spec with blocking true or null last-run, none when review.md absent, validate-frontmatter hard-fail on unreadable record; the CI step extracted from framework/templates/ci/adopter-generators.yml exits 1 on blocking:true and on the high-water-mark bound (057 commit 099f4c23, predicate was a review: block grep before d2bfac5a)
- AC6 — met: driven: two write-review runs on identical input with different --reviewed-at/--reviewed-against produced review.md files differing only in last-run and reviewed-against (diff)
- AC7 — met: framework/reference/review.md sections Apply --fix and Auto-fix scope (auto-fixable yes, no signature/schema/status-code/message change, no test deletion, within scope) which the Flags table in framework/commands/review.md points at; behavior-changing fixes excluded by those rules (prose contract, not driven: --fix edits code in a host)
- AC8 — met: `RawWaiverFull` in `runtime/src/primitives/write_review.rs` keeps `#[serde(flatten)]` extras, and a fixture's custom `co-waived-by` field survived a `write-review` re-render; waivers live in `review.md` (057), an applied one kept and an expired one dropped, `process-waivers` expiring one whose rule stopped firing; the clause corrected from 'the three raw parsers', two of which went with `check-review-agreement` (by rule)
- AC9 — met: `write-review`'s result carries `exit-code` from `i32::from(blocking)`, driven 1 with a MUST and 0 without; `main.rs` routes `WriteReview` through the ungated `emit_result` and `run_exec` returns success on a complete walk, while `framework/templates/ci/adopter-generators.yml`'s audit-record gate reads the record's `blocking`, as the corrected annotation states (owner decision)
- AC10 — met: `README.md`'s Commands list names `/review` under **Pipeline — advance state** with a one-line purpose; `30e1fbaa`'s README held the `/review` table row under `### Pipeline (advance state)` and `1ffcef35` (2026-08-30) made the section a list, as the annotation states (by rule)
- AC11 — met: framework/commands/review.md step 1 and framework/reference/review.md Resolve target and scope item 4 halt on a missing/empty/inconsistent AGENTS.md Tech Stack before the passes (steps 3-7), skip when [review] tech-stack-verified = true, persist via gate-confirm, no auto-reset; the key is a recognized config key (discover-rule-files on this repo's config emits no unknown-key notice)
- AC12 — met: driven: write-review --empty-scope on a fresh fixture wrote 0/0/0 counts, blocking:false, result exit-code 0 and no examined key; with a not-met criterion the same call blocks (exit-code 1), the 066 qualification stated in framework/reference/review.md Resolve target and scope item 3
- AC13 — met: driven: same rule BE-X-001 on src/a.py as should 1-2 and must 2-3 yielded one MUST (src/a.py:2-3) and the non-overlapping 10-12 SHOULD only; must-violations 1, should-violations 1, the lower-severity duplicate absent from counts and report
- AC14 — met: framework/commands/review.md section Parsing $ARGUMENTS covers every Flags-table flag (--since with no value and unrecognized flags are errors), argument-hint names all twelve; check-command-flags returns findings 0 over 17 files and scripts/audit/command-flag-hint-parity.sh exits 0; proven to fail: on a scratch copy with [--since=<ref>] removed from the hint the primitive returns a flag-unsurfaced finding for --since
- AC15 — met: examined/scope half holds in review.md alone: write-review omits examined when unstated (fixture empty-scope record has no examined key), scope derived by the primitive; test examined_and_scope_are_written_to_the_record_alone passes; Family 31 is gone (no Family 31 in scripts/audit, check-review-agreement retired per framework/bootstrap/configure/claude.md) and no gate reads examined, matching the annotation
- AC16 — met: `framework/commands/review.md` step 3 and `framework/reference/review.md` Passes in a subagent run steps 3 to 7.2 in one subagent and continue at 7.3 with findings, observations, criterion verdicts and coverage; `cargo test --test review_command` passes with steps 3 to 7 `performReview` and 7.2 `verifyCriteria` under `ductus exec` (annotation added by rule, from 066 task 27)

## Observations

- convention — spec.md's Blocking message note quotes the gate's and the drift check's texts as emitted today and points at `framework/commands/implement.md` step 5 and `framework/commands/analyze.md` for them, though step 5 is the write-code step, the drift section is in `framework/reference/analyze.md`, and the emitted texts differ from the quotes — `specs/020-code-review/spec.md` — **fixed** (decided by the rule)
- convention — data-model.md's `reviewed-against` row says it is read for one thing, the mechanical-sweep rename exemption, though `compute-review-scope` also reads it as the end of a committed record's window to credit what the record read (081) — `specs/020-code-review/data-model.md` — **fixed** (decided by the rule)
- convention — data-model.md says its frontmatter example is written in that order on every run, but the example omits fields the table says every run writes and `write-review` orders them differently — `specs/020-code-review/data-model.md` — **fixed** (decided by the rule)
- convention — spec.md's Resolved Question on the quality-pass confidence threshold says it is fixed at 80, while the quality pass records a `high` or `low` tier — `specs/020-code-review/spec.md` — **fixed** (decided by the rule)
- other — spec.md's Tasks list still directs edits to a `review:` block in spec frontmatter and the spec template, which 057 removed — `specs/020-code-review/spec.md` — **waived**: The list records what 020 delivered, and the relocation is stated beside it, in the 057 note and the Frontmatter schema section; rewording a delivered task list changes no requirement and makes no claim about the tree. (decided by the rule)
- convention — framework/reference/review.md §Output says CI can gate on review's exit code without parsing the report, but `ductus write-review` and `ductus exec` exit 0 once the record is written and CI gates on the record's `blocking` — `framework/reference/review.md` — **fixed** (decided by the rule)
- convention — `write_review.rs`'s module doc, `WriteReviewResult.blocking`'s doc and 022's scenario `review-runtime-acceleration` say `blocking` is true exactly when `must-violations` exceeds zero, though since 066's task 27 a criterion judged not met sets it too — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- convention — round 1's re-read: the module doc of `runtime/src/primitives/write_review.rs`, edited this run, ran one line far past its wrap width — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/amend.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `.claude/commands/ductus/implement.md` — 066-audit-records-and-the-completion-gate
- `.claude/commands/ductus/review.md` — 066-audit-records-and-the-completion-gate
- `.ductus/reference/audit.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `.ductus/reference/implement.md` — 066-audit-records-and-the-completion-gate
- `.ductus/reference/review.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `.github/workflows/runtime-acquisition.yml` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `.github/workflows/runtime-release.yml` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `.github/workflows/runtime.yml` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `.pi/prompts/ductus-amend.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `.pi/prompts/ductus-implement.md` — 066-audit-records-and-the-completion-gate
- `.pi/prompts/ductus-review.md` — 066-audit-records-and-the-completion-gate
- `AGENTS.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `docs/developing-ductus.md` — 062-concurrent-session-targets, 066-audit-records-and-the-completion-gate
- `docs/slash-commands.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/bootstrap/ductus-procedure.md` — 027-bootstrap-migration-registry
- `framework/bootstrap/ductus.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `framework/bootstrap/govern.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `framework/commands/amend.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `framework/commands/implement.md` — 066-audit-records-and-the-completion-gate
- `framework/commands/review.md` — 066-audit-records-and-the-completion-gate
- `framework/constitution.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/migrations.toml` — 027-bootstrap-migration-registry
- `framework/reference/audit.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/reference/implement.md` — 066-audit-records-and-the-completion-gate
- `runtime/CHANGELOG.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 073-decided-rule-departures-are-recorded
- `runtime/src/interpreter/mod.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/interpreter/payload.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `runtime/src/main.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/check_review_gate.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/src/primitives/classify_spec_edit.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/compute_review_scope.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/src/primitives/criterion_verdicts.rs` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/src/primitives/invalidate_review.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/line_citations.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/mod.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/schema/descriptions.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/src/schema/extensions.rs` — 066-audit-records-and-the-completion-gate
- `runtime/tests/fixtures/implement-basic/.govern.session.toml` — 066-audit-records-and-the-completion-gate
- `runtime/tests/fixtures/implement-basic/stdin.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/fixtures/review-basic/stdin.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/golden/implement-basic.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/golden/review-basic.jsonl` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/tests/mcp.rs` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/tests/review_command.rs` — 066-audit-records-and-the-completion-gate
- `scripts/audit/README.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `scripts/audit/done-spec-criteria.sh` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `scripts/audit/migration-coverage.sh` — 027-bootstrap-migration-registry
- `scripts/mutation/criteria-in-review-066.py` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
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
- `specs/007-govern-workflow/plan.md` — 007-govern-workflow
- `specs/015-tarball-fetch/plan.md` — 015-tarball-fetch
- `specs/022-deterministic-runtime/data-model.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/022-deterministic-runtime/plan.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/022-deterministic-runtime/scenarios/implement-completion-gate.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/026-framework-self-audit/analysis.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/plan.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/spec.md` — 026-framework-self-audit
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
- `specs/029-bootstrap-runtime-autowire/scenarios/state-a-deterministic-path-forcing.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/spec.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/tasks.md` — 027-bootstrap-migration-registry, 029-bootstrap-runtime-autowire
- `specs/048-govern-acquired-runtime/data-model.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/plan.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/scenarios/state-b-continues-in-session.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/spec.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/tasks.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/analysis.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/066-audit-records-and-the-completion-gate/plan.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/066-audit-records-and-the-completion-gate/review.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/scenarios/criteria-are-verified-in-review.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/spec.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/073-decided-rule-departures-are-recorded/plan.md` — 073-decided-rule-departures-are-recorded
- `specs/079-review-and-analysis-work-converges/spec.md` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/081-reviews-and-checks-follow-what-changed/spec.md` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed

## Unexamined governance

*None.*
