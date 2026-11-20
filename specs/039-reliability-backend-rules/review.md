---
spec: 039-reliability-backend-rules
last-run: 2026-11-20T04:27:30Z
reviewed-against: 07fa02c3bd31a2b5cedd4aabfece9738d693b6bb
diff-base: fa4ae5c75b7d68e907f18cee94eb8f8157f83f52
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 6
scope: 6
unread: []
excluded: 33
contract-breaks: 0
skipped-passes: []
fix-rounds: 0
re-reads: 0
diff-checked: 0
not-re-read: 0
budget-stop: false
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
    digest: e0a1e0aafb04cb8a160f0d103b807c954aed5b5173c72629a26d5ed87f732526
  - label: AC8
    digest: 3af306b6aba2a6a79ee051a56c1bcf3ccbf5b15c602a33caface40dfd4fb9cf6
  - label: AC9
    digest: f4cc4056a544441610048cf509e85c9402df151b828b9b004fff95dcd8fd7bc0
  - label: AC10
    digest: b07de331a98dcace58bd3875b485345f81d5488a80a0ecdb840efba17172a649
blocking: false
dispositions:
  fixed: 1
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

Reopened by task 8 (BE-RETRY-005 may drop a declared-expendable message once recorded). Base fa4ae5c7 (reopen), window of 6 entries, all read by three readers: a security reader over the three security rule files (whole), a reader for the other four passes over the nine other loaded rule files, AGENTS.md Gotchas to the end and specs/system.md (whole), and a criteria reader. runtime/CHANGELOG.md was read as its window diff (11,021 bytes) and its 039 entry, not as the whole 610 KB file. No finding; one observation (the plan Overview narrated history and cited pruned tasks 5 and 7) fixed by commit specs(039) plan Overview, before the fix-round base, and read whole by the criteria reader. Two stored decisions on tasks.md expired because the pruned tasks no longer exist. Criteria AC1 to AC10 judged met. The stated sizes of reliability-backend.md (19,029 bytes) and the spec files are unchanged by this review.

0 fix round(s): 0 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

- AC1 — met: `framework/rules/reliability-backend.md` exists; 13 `### BE-` headings, 13 Verification, 13 Source, 0 Rationale; every Statement is a block quote with RFC 2119 keywords.
- AC2 — met: All IDs are `BE-{TIMEOUT|RETRY|BREAKER|DRAIN|BULK}-NNN`, none of these categories in the other backend files; `bash scripts/lint-rule-ids.sh` exits 0 (12 files, 308 IDs).
- AC3 — met: The header paragraph of `framework/rules/reliability-backend.md` declares `Categories:` TIMEOUT, RETRY, BREAKER, DRAIN, BULK with their concerns.
- AC4 — met: `BE-TIMEOUT-001`, `BE-RETRY-001`, `BE-BREAKER-001`, `BE-DRAIN-001` each carry a design-time-commitment Verification flagged by `/{project}:analyze`, no code-pattern grep.
- AC5 — met: Four MUSTs (`BE-TIMEOUT-001`, `BE-RETRY-001`, `BE-RETRY-005`, `BE-DRAIN-001`), each an availability or cascading-failure risk; the other nine are SHOULD; `BE-RETRY-005`'s added sentence is a MAY.
- AC6 — met: `CFG-CONST-003` appears in exactly the ten rules the criterion lists; `BE-POOL-002`, `BE-IDEMP`, `BE-ASYNC-001` and `BE-POOL-005` are cited from `BE-TIMEOUT-001`, `BE-RETRY-001`, `BE-BULK-001` and `BE-TIMEOUT-003` and exist in their files.
- AC7 — met: `framework/bootstrap/ductus.md` carries the manifest row between `quality-cross.md` and `security-backend.md`; the `-backend.md` suffix selects it under the backend surface; `discover-rule-files` selects it in this run.
- AC8 — met: `specs/034-performance-backend-rules/spec.md` forward-references `reliability-backend.md` as landed by spec 039, and the file holds the TIMEOUT, RETRY and BREAKER rules.
- AC9 — met: `BE-RETRY-003`, `-004` and `BE-BULK-002` are SHOULD, and `BE-RETRY-005` is MUST with dead-letter or recorded halt, no alerting requirement, and a MAY for a recorded drop of a spec-declared-expendable message; each has Statement, Verification and Source.
- AC10 — met: `BE-TIMEOUT-003` is SHOULD, its Verification states PostgreSQL `statement_timeout`, MySQL `max_execution_time` for read-only `SELECT`, MongoDB `maxTimeMS` and SQL Server's driver-side timeout, and cites `BE-POOL-002`, `BE-POOL-005` and `CFG-CONST-003`.

## Observations

- convention: `plan.md`'s Overview cited tasks 5 and 7, which `tasks.md` no longer holds, and narrated how the rule set got here, which the plan's design record does not carry; the Overview now states the rule set as it stands (commit `specs(039): the plan's Overview states the rule set as it stands, without the history and the pruned task numbers`). — `specs/039-reliability-backend-rules/plan.md` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `framework/rules/accessibility-frontend.md`
- `framework/rules/api-backend.md`
- `framework/rules/concurrency-backend.md`
- `framework/rules/configuration-cross.md`
- `framework/rules/security-backend.md`
- `framework/rules/security-cross.md`
- `framework/rules/security-frontend.md`
- `runtime/src/primitives/shipped_rule_ids.txt`
- `scripts/audit/README.md`
- `specs/008-security-rules/scenarios/security-rules-flag-what-their-sources-support.md`
- `specs/008-security-rules/spec.md`
- `specs/008-security-rules/tasks.md`
- `specs/017-derive-dont-ask/scenarios/a-throwaway-credential-is-not-a-secret.md`
- `specs/017-derive-dont-ask/spec.md`
- `specs/017-derive-dont-ask/tasks.md`
- `specs/026-framework-self-audit/scenarios/family-40-deprecated-rules-are-removed-on-time.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/038-concurrency-backend-rules/scenarios/a-declared-loss-tolerant-message-owes-no-outbox.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/tasks.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/068-api-backend-rules/scenarios/repr-003-bounded-numbers-and-patch-001-format-home.md`
- `specs/068-api-backend-rules/spec.md`
- `specs/068-api-backend-rules/tasks.md`
- `specs/069-accessibility-frontend-rules/scenarios/verifications-flag-only-what-the-statement-covers.md`
- `specs/069-accessibility-frontend-rules/spec.md`
- `specs/069-accessibility-frontend-rules/tasks.md`
- `specs/077-security-cross-rules/data-model.md`
- `specs/077-security-cross-rules/plan.md`
- `specs/077-security-cross-rules/scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md`
- `specs/077-security-cross-rules/spec.md`
- `specs/077-security-cross-rules/tasks.md`

## Unexamined governance

*None.*
