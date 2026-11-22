---
spec: 039-reliability-backend-rules
last-run: 2026-11-22T00:14:30Z
reviewed-against: d4f3b6e78246d9c7d56b4c1d4644d4e57c09432d
diff-base: 2983d2a34c07c501545bb76512b6c7c9555f11a5
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 2
scope: 2
unread: []
excluded: 66
contract-breaks: 0
skipped-passes: []
reviewed-digest:
  scenarios/expendable-messages-may-be-dropped-when-recorded.md: 03e0030d79ae0938b4375c3ec2fc2950e08a41c96817f0a4b54d93536d66d22e
criteria-judged: 10
criteria-met: 10
criteria-verdicts:
  - label: AC1
    digest: 98c2e67118df5e42f5e181eb122e476ef2a825602de7df95026b2cd7f30795eb
  - label: AC2
    digest: 6fb5b8704c0f7729a1a3bbb9a29ecb15e7d104bf164c582f01905f184187f836
  - label: AC3
    digest: c108d2c8556da94ea48264c8cd4d59af50042716137838dec27cba253c3528ea
  - label: AC4
    digest: a4edcacc594f3a6334875b1a3d6c2c86e72fc379f484379f07e50e8bd06e8941
  - label: AC5
    digest: 71079b12058a4e04f86a1742b48193c044a1ca13aca0c7977b8cacda0b8312e2
  - label: AC6
    digest: c0a7264750e08af9db7da4e7542b72482790c324a11aa908b504366902a01412
  - label: AC7
    digest: 95424e2bfc8e2ea79d78345b7d26703a0e3659e41e567d68c9f301e059c4b237
  - label: AC8
    digest: 3af306b6aba2a6a79ee051a56c1bcf3ccbf5b15c602a33caface40dfd4fb9cf6
  - label: AC9
    digest: f4cc4056a544441610048cf509e85c9402df151b828b9b004fff95dcd8fd7bc0
  - label: AC10
    digest: b07de331a98dcace58bd3875b485345f81d5488a80a0ecdb840efba17172a649
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 0
  undispositioned: 0
decisions:
  - key: "other: 036's ticked task 2 names the manifest slot it used at the time — `specs/036-quality-cross-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-013 counts expiring pre-signed URLs as single-use short-lived values; AWS documents them as reusable bearer tokens up to 7 days, and FE-STORAGE-003 forbids them — `framework/rules/security-backend.md`"
    outcome: deferred
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-013 allows a single-use token anywhere in a URL while FE-STORAGE-003 allows it only in a query parameter — `framework/rules/security-frontend.md`"
    outcome: deferred
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "security: BE-AUTHN-001's Verification names only OWASP's headline parameter minimums — `framework/rules/security-backend.md`"
    outcome: deferred
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-015 flags a handshake-URL token that BE-API-013 may permit — `framework/rules/security-backend.md`"
    outcome: deferred
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: QUAL-GROUND-001 cedes configuration to CFG rules that do not cover a required key missing from a file or flag — `framework/rules/quality-cross.md`"
    outcome: deferred
    target: specs/017-derive-dont-ask/scenarios/required-config-values-from-any-source.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md`"
    outcome: deferred
    target: specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: 025's task 10 named only AC4 among the places that describe skip-and-warn — `specs/025-rule-opt-out/tasks.md`"
    outcome: deferred
    target: specs/025-rule-opt-out/tasks.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: 063's task 12 missed AC5, plan.md, README.md, the command sources and the tool description that still describe the fallback — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`"
    outcome: deferred
    target: specs/063-oversized-artifacts-warn-with-a-fix/tasks.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md`"
    outcome: deferred
    target: specs/033-rule-surface-setting/tasks.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: 019's task 1 did not cover a host reading config by hand on the markdown-only path — `specs/019-config-decisions/tasks.md`"
    outcome: deferred
    target: specs/019-config-decisions/tasks.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: FE-SEMHTML-001 and FE-ARIA-001 both forbid a div with role=button — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: consistent overlap of two MUSTs with no contradiction; consolidating them changes rule ownership, which 069's clarify owns
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: FE-KBD-002 and FE-CONTRAST-001 both require 3:1 for focus indicators — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: the owner decided FE-KBD-002's MUST carries the 3:1 ratio (069 Resolved Questions, 2026-10-19); the overlap is consistent
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: accessibility-frontend.md's first header paragraph is long — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: style only; the content is accurate, and 069's first open question owns what the header claims about conformance
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md`"
    outcome: waived
    reason: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: BE-REPR-001's Verification names specs/system.md for the API-wide representation — `framework/rules/api-backend.md`"
    outcome: waived
    reason: the Statement's one representation across the API needs a declared one to be checkable; the Verification names where it is declared
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: BE-SCHEMA-001's SDL-versus-introspection split and BE-ERRENV-001's overlap with BE-ERR-002 — `framework/rules/api-backend.md`"
    outcome: waived
    reason: both splits are recorded as intended in 068; low
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: BE-DEPS-002 and BE-DEPS-006 Verifications ask for an update process their Statements do not name — `framework/rules/security-backend.md`"
    outcome: waived
    reason: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: BE-QUERY-004, BE-POOL-005 and BE-TXN-004 repeat PostgreSQL background sentences — `framework/rules/performance-backend.md`"
    outcome: waived
    reason: the repeated sentences are accurate and not contradictory; each rule stands alone
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md`"
    outcome: waived
    reason: a resolved question records the reasoning at the time; rewriting it records a fiction
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
---

# Review — 039-reliability-backend-rules

## Summary

Five-pass review of 039's reopening by 925c6905 (020's task 18, AC17), diff-base 2983d2a3, base-source reopen, at HEAD ba69ef5d. The scope is `specs/039-reliability-backend-rules/spec.md` and `plan.md`, which 925c6905 edited to say a review leaves out a rule file whose header gives it to `/ductus:analyze`. Passes (part P1, one reviewer for 020 and the eight specs 925c6905 reopened) read those hunks and each scope file whole against the five rule files `discover-rule-files` selected (232,805 bytes, each read whole against `wc -c`): no finding and no observation. The seven other rule files are judged by analysis alone (020's AC17), so the efficiency pass ran on its own definition. The criteria step (part C6) judged every criterion individually: 10 of 10 met, with the rule file read whole, exactly the ten rules AC6 lists citing `CFG-CONST-003`, and 034's spec naming `reliability-backend.md` (AC8). Examined 2 of 2; nothing unread.

10 of 10 acceptance criteria judged met.

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

- AC1 — met: `framework/rules/reliability-backend.md` exists, `-backend.md` suffix, ### ID headings with `Statement/Verification/Source`, no Rationale (grep count 0)
- AC2 — met: `scripts/lint-rule-ids.sh` run: exit 0, 12 files, 308 IDs; categories BE-TIMEOUT, BE-RETRY, BE-BREAKER, BE-DRAIN, BE-BULK are disjoint from every other `-backend.md` file's (listed from their ### headings)
- AC3 — met: header paragraph lists Categories: TIMEOUT, RETRY, BREAKER, DRAIN, BULK abbreviations
- AC4 — met: TIMEOUT-001..003, RETRY-001 (bound, backoff+jitter, idempotent only), BREAKER-001, `DRAIN-001/002` with design-time Verification
- AC5 — met: MUSTs: TIMEOUT-001, RETRY-001, RETRY-005, DRAIN-001 (availability risks); the rest SHOULD
- AC6 — met: BE-POOL-002 (`TIMEOUT-001/003`, BULK-001), BE-IDEMP (RETRY-001), BE-ASYNC-001 (BULK-001) cited; CFG-CONST-003 appears in exactly ten rules (`TIMEOUT-001/002/003`, `RETRY-001/002/005`, BREAKER-001, `DRAIN-001/002`, BULK-001), matching the criterion's list and its 'ten rules in all'
- AC7 — met: `framework/bootstrap/ductus.md` Shared Files row lists the file; ductus discover-rule-files (release binary) leaves it out with a 'judged by analysis alone' notice and selects the other five files; its header names analyze, not review
- AC8 — met: `specs/034-performance-backend-rules/spec.md` names `reliability-backend.md` (spec 039) for the deferred deadlines, timeouts, retries and breakers; the file carries `TIMEOUT/RETRY/BREAKER` rules
- AC9 — met: BE-RETRY-003 SHOULD, BE-RETRY-004 SHOULD, BE-RETRY-005 MUST with a MAY for expendable messages and a Not-flagged clause on monitoring, BE-BULK-002 SHOULD; each has `Statement/Verification/Source`
- AC10 — met: BE-TIMEOUT-003 SHOULD; Verification states PostgreSQL `statement_timeout`, MySQL `max_execution_time` (SELECT only), MongoDB maxTimeMS and SQL Server's driver-side timeout; cites BE-POOL-002, BE-POOL-005 (exists in `performance-backend.md`), CFG-CONST-003

## Observations

*None.*

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/review.md`
- `.ductus/reference/analyze.md`
- `.ductus/reference/review.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-review.md`
- `docs/developing-ductus.md`
- `docs/rules.md`
- `framework/commands/analyze.md`
- `framework/commands/review.md`
- `framework/reference/analyze.md`
- `framework/reference/review.md`
- `runtime/CHANGELOG.md`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/primitives/brief_re_read.rs`
- `runtime/src/primitives/discover_rule_files.rs`
- `runtime/src/primitives/fix_rounds.rs`
- `runtime/src/primitives/resolve_anchor.rs`
- `runtime/src/primitives/start_fix_rounds.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/schema/descriptions.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/line_ending_discipline.rs`
- `scripts/analyze/check.py`
- `scripts/analyze/detect.py`
- `scripts/analyze/step15-brief.md`
- `scripts/mutation/analysis-design-fix-067.py`
- `scripts/mutation/command-reference-split.py`
- `scripts/mutation/exec-scenario-anchors-067.py`
- `scripts/mutation/owner-reviews-067.py`
- `scripts/tests/test-analyze-scripts.py`
- `specs/020-code-review/plan.md`
- `specs/020-code-review/spec.md`
- `specs/020-code-review/tasks.md`
- `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/024-rule-loader/review.md`
- `specs/024-rule-loader/spec.md`
- `specs/034-performance-backend-rules/review.md`
- `specs/034-performance-backend-rules/spec.md`
- `specs/037-observability-backend-rules/plan.md`
- `specs/037-observability-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/plan.md`
- `specs/038-concurrency-backend-rules/review.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/068-api-backend-rules/spec.md`
- `specs/069-accessibility-frontend-rules/spec.md`
- `specs/070-performance-frontend-rules/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md`
- `specs/079-review-and-analysis-work-converges/analysis.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/inbox.md`

## Unexamined governance

*None.*
