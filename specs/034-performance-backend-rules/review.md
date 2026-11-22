---
spec: 034-performance-backend-rules
last-run: 2026-11-22T00:14:17Z
reviewed-against: d7de7eb3b4fe90bab1f8763b1687c994f8535f66
diff-base: 2983d2a34c07c501545bb76512b6c7c9555f11a5
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 1
scope: 1
unread: []
excluded: 65
contract-breaks: 0
skipped-passes: []
reviewed-digest: {}
criteria-judged: 9
criteria-met: 9
criteria-verdicts:
  - label: AC1
    digest: b7e3eccddb10f9270db248679439f36b11434fc9a7b96fc4118af248739322db
  - label: AC2
    digest: 5722cc739f50ea2ed66a161d996314e42a1603154f81124a2ac7acea9e536862
  - label: AC3
    digest: ff4464121aa60f3415d4fa168e53320d95890691a67f925de37b2892859e1701
  - label: AC4
    digest: e06954d060a31daeb15b5560025fb8b8bb24f7c26f461dc077d8b22a356ae6eb
  - label: AC5
    digest: 5be8e04f53078c01e2656a42540fcd10440823684f02a20f58f88dfd080ba8a7
  - label: AC6
    digest: 6ba25e7d1e1f1efc21b7da7d307b1e06f19126fc720af73c70e2043d08a80ef4
  - label: AC7
    digest: 2e4eb39f47c51eba8282de3895d892cc8efa81c46bc18a3906275072edac6fc2
  - label: AC8
    digest: 6f84c6bae3c4562488ff3af28dbcdaa3ee707746fa935e04ada6bcb48d84a1f5
  - label: AC9
    digest: e3acc5f117705894f9648cf9b508e7d6d77ac37809fb602c1d0573562926cb74
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 0
  undispositioned: 0
decisions:
  - key: "security: BE-API-013 counts expiring pre-signed URLs as single-use short-lived values; AWS documents them as reusable bearer tokens up to 7 days, and FE-STORAGE-003 forbids them — `framework/rules/security-backend.md`"
    outcome: deferred
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-013 allows a single-use token anywhere in a URL while FE-STORAGE-003 allows it only in a query parameter — `framework/rules/security-frontend.md`"
    outcome: deferred
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "security: BE-AUTHN-001's Verification names only OWASP's headline parameter minimums — `framework/rules/security-backend.md`"
    outcome: deferred
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-015 flags a handshake-URL token that BE-API-013 may permit — `framework/rules/security-backend.md`"
    outcome: deferred
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: QUAL-GROUND-001 cedes configuration to CFG rules that do not cover a required key missing from a file or flag — `framework/rules/quality-cross.md`"
    outcome: deferred
    target: specs/017-derive-dont-ask/scenarios/required-config-values-from-any-source.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md`"
    outcome: deferred
    target: specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: 025's task 10 named only AC4 among the places that describe skip-and-warn — `specs/025-rule-opt-out/tasks.md`"
    outcome: deferred
    target: specs/025-rule-opt-out/tasks.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: 063's task 12 missed AC5, plan.md, README.md, the command sources and the tool description that still describe the fallback — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`"
    outcome: deferred
    target: specs/063-oversized-artifacts-warn-with-a-fix/tasks.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md`"
    outcome: deferred
    target: specs/033-rule-surface-setting/tasks.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: 019's task 1 did not cover a host reading config by hand on the markdown-only path — `specs/019-config-decisions/tasks.md`"
    outcome: deferred
    target: specs/019-config-decisions/tasks.md
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: FE-SEMHTML-001 and FE-ARIA-001 both forbid a div with role=button — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: consistent overlap of two MUSTs with no contradiction; consolidating them changes rule ownership, which 069's clarify owns
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: FE-KBD-002 and FE-CONTRAST-001 both require 3:1 for focus indicators — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: the owner decided FE-KBD-002's MUST carries the 3:1 ratio (069 Resolved Questions, 2026-10-19); the overlap is consistent
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: accessibility-frontend.md's first header paragraph is long — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: style only; the content is accurate, and 069's first open question owns what the header claims about conformance
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md`"
    outcome: waived
    reason: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: BE-REPR-001's Verification names specs/system.md for the API-wide representation — `framework/rules/api-backend.md`"
    outcome: waived
    reason: the Statement's one representation across the API needs a declared one to be checkable; the Verification names where it is declared
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: BE-SCHEMA-001's SDL-versus-introspection split and BE-ERRENV-001's overlap with BE-ERR-002 — `framework/rules/api-backend.md`"
    outcome: waived
    reason: both splits are recorded as intended in 068; low
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: BE-DEPS-002 and BE-DEPS-006 Verifications ask for an update process their Statements do not name — `framework/rules/security-backend.md`"
    outcome: waived
    reason: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: ticked task-1 items in 037, 038 and 039 say one RFC 2119 keyword, and 039's task 4 names an alert — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
  - key: "other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md`"
    outcome: waived
    reason: a resolved question records the reasoning at the time; rewriting it records a fiction
    decided-at: 2026-10-20T00:34:28Z
    decided-by: andy@stone.dev
---

# Review — 034-performance-backend-rules

## Summary

Five-pass review of 034's reopening by 925c6905 (020's task 18, AC17), diff-base 2983d2a3, base-source reopen, at HEAD ba69ef5d. The scope is `specs/034-performance-backend-rules/spec.md`, which 925c6905 edited to say a review leaves out a rule file whose header gives it to `/ductus:analyze`. Passes (part P1, one reviewer for 020 and the eight specs 925c6905 reopened) read those hunks and each scope file whole against the five rule files `discover-rule-files` selected (232,805 bytes, each read whole against `wc -c`): no finding and no observation. The seven other rule files are judged by analysis alone (020's AC17), so the efficiency pass ran on its own definition. The criteria step (part C5) judged every criterion individually: 9 of 9 met, with each of the 18 rules' tier checked against the Severity posture, the five categories against the other rule files' headers, AC6's citation enumeration rule by rule, and the manifest row in `framework/bootstrap/ductus.md`. Examined 1 of 1; nothing unread.

9 of 9 acceptance criteria judged met.

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

- AC1 — met: `framework/rules/performance-backend.md` exists, -backend suffix, 18 '### BE-' rules each with Statement and Verification, no Rationale (0 hits), RFC 2119 keywords; `lint-rule-ids.sh` passes.
- AC2 — met: IDs use `QUERY/CACHE/POOL/PAYLOAD/ASYNC` only; disjoint from security-backend (AUTHN AUTHZ INPUT DATA API LOG DEPS ERR), api-backend and the other backend headers; `scripts/lint-rule-ids.sh` rc=0.
- AC3 — met: File header line 3 declares the five categories with abbreviations.
- AC4 — met: BE-QUERY-001..004, BE-CACHE-001..004, BE-POOL-001..005, BE-PAYLOAD-001..004 cover N+1, indexes, bounds, TTL, keys, stampede, pooling, size cap, field selection, compression; every Verification is a design-time commitment judged by analyze.
- AC5 — met: MUST: QUERY-001, QUERY-003, CACHE-001, `POOL-001/002/003`, PAYLOAD-001, ASYNC-001 (exhaustion risks); SHOULD: `QUERY-002/004`, `CACHE-002/003/004`, `POOL-004/005`, `PAYLOAD-002/003/004`, matching the Severity posture.
- AC6 — met: Enumeration checked rule by rule: BE-PAGE in QUERY-003 and PAYLOAD-001; BE-STATUS-001, BE-LRO-001, BE-TIMEOUT-001, BE-BREAKER-001 in ASYNC-001; CFG-CONST-003 in `QUERY-003/004`, `POOL-001/002`, PAYLOAD-004; `BE-AUTHZ-002/005/003` in CACHE-002; BE-API-001 and FE-LOAD-007 in CACHE-004; BE-INPUT-006 in PAYLOAD-004; BE-TIMEOUT-001 and BE-COORD-003 in POOL-005; no BE-IDEMP present. All cited rules exist in the rule files.
- AC7 — met: `framework/bootstrap/ductus.md` Shared Files row maps `framework/rules/performance-backend.md` to `specs/rules`/; its header names analyze alone and driven discover-rule-files (backend surface) left it out with the 'judged by analysis alone' notice while --all loads it, as the narrowing states.
- AC8 — met: BE-QUERY-004, BE-CACHE-004, BE-POOL-004, BE-PAYLOAD-004 present, each SHOULD with a design-time Verification.
- AC9 — met: BE-POOL-005 is SHOULD NOT hold a transaction or connection across an outside call; Verification asks for the call outside the transaction or the named BE-COORD-003 outbox.

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
- `specs/037-observability-backend-rules/plan.md`
- `specs/037-observability-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/plan.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/039-reliability-backend-rules/plan.md`
- `specs/039-reliability-backend-rules/spec.md`
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
