---
spec: 038-concurrency-backend-rules
last-run: 2026-11-20T04:14:30Z
reviewed-against: 84e090b9e535cd14f2b94dfcec5530cc53caa70d
diff-base: fa4ae5c75b7d68e907f18cee94eb8f8157f83f52
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 6
unread:
  - runtime/CHANGELOG.md
excluded: 34
contract-breaks: 0
skipped-passes: []
fix-rounds: 0
re-reads: 0
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  scenarios/a-declared-loss-tolerant-message-owes-no-outbox.md: 1d82638ef829a2b345d14a253f1bea673ce1aa730d0b44110fd17799ed1c929d
  scenarios/lock-the-parent-before-rewriting-its-children.md: 81701adaf3813bbd84be93d692c6f65e309c58822c3229991be923461aef601b
criteria-judged: 8
criteria-met: 8
criteria-verdicts:
  - label: AC1
    digest: 0b7a9077b1696aa02aedcc97b4f2ad1dcd691638165b9de7b67d2a47fe5f1b1d
  - label: AC2
    digest: a8a8cc603665eee387c32154dc458143928a8b8aae0bb4a1f418d70996eb8e48
  - label: AC3
    digest: 2bae2ae73f8f3aa65b9bc21a43ff00d34f96859a1a5bd7b8cf2429abd1b118eb
  - label: AC4
    digest: 7f8079d3edebc306f20a5903b6319b5ed3990e291ab1eb2e74a59c049f961872
  - label: AC5
    digest: 1fb0425ab1e428305107c43c8e68ac6d99f65b1caed0084849b433a706e950b9
  - label: AC6
    digest: baed24384dfb610c59b7610b74e3ea55d914bb9343e8a032fc0188003a34047b
  - label: AC7
    digest: e0a1e0aafb04cb8a160f0d103b807c954aed5b5173c72629a26d5ed87f732526
  - label: AC8
    digest: 381356efb8942defe8343e1d973398f04b11534f1ad8e0670fef733da80af850
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 6
  undispositioned: 0
decisions:
  - key: "other: 036's ticked task 2 names the manifest slot it used at the time — `specs/036-quality-cross-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-013 counts expiring pre-signed URLs as single-use short-lived values; AWS documents them as reusable bearer tokens up to 7 days, and FE-STORAGE-003 forbids them — `framework/rules/security-backend.md`"
    outcome: deferred
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-013 allows a single-use token anywhere in a URL while FE-STORAGE-003 allows it only in a query parameter — `framework/rules/security-frontend.md`"
    outcome: deferred
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "security: BE-AUTHN-001's Verification names only OWASP's headline parameter minimums — `framework/rules/security-backend.md`"
    outcome: deferred
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-015 flags a handshake-URL token that BE-API-013 may permit — `framework/rules/security-backend.md`"
    outcome: deferred
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: QUAL-GROUND-001 cedes configuration to CFG rules that do not cover a required key missing from a file or flag — `framework/rules/quality-cross.md`"
    outcome: deferred
    target: specs/017-derive-dont-ask/scenarios/required-config-values-from-any-source.md
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md`"
    outcome: deferred
    target: specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: 025's task 10 named only AC4 among the places that describe skip-and-warn — `specs/025-rule-opt-out/tasks.md`"
    outcome: deferred
    target: specs/025-rule-opt-out/tasks.md
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: 063's task 12 missed AC5, plan.md, README.md, the command sources and the tool description that still describe the fallback — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`"
    outcome: deferred
    target: specs/063-oversized-artifacts-warn-with-a-fix/tasks.md
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md`"
    outcome: deferred
    target: specs/033-rule-surface-setting/tasks.md
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: 019's task 1 did not cover a host reading config by hand on the markdown-only path — `specs/019-config-decisions/tasks.md`"
    outcome: deferred
    target: specs/019-config-decisions/tasks.md
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: FE-SEMHTML-001 and FE-ARIA-001 both forbid a div with role=button — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: consistent overlap of two MUSTs with no contradiction; consolidating them changes rule ownership, which 069's clarify owns
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: FE-KBD-002 and FE-CONTRAST-001 both require 3:1 for focus indicators — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: the owner decided FE-KBD-002's MUST carries the 3:1 ratio (069 Resolved Questions, 2026-10-19); the overlap is consistent
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: accessibility-frontend.md's first header paragraph is long — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: style only; the content is accurate, and 069's first open question owns what the header claims about conformance
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md`"
    outcome: waived
    reason: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: BE-REPR-001's Verification names specs/system.md for the API-wide representation — `framework/rules/api-backend.md`"
    outcome: waived
    reason: the Statement's one representation across the API needs a declared one to be checkable; the Verification names where it is declared
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: BE-SCHEMA-001's SDL-versus-introspection split and BE-ERRENV-001's overlap with BE-ERR-002 — `framework/rules/api-backend.md`"
    outcome: waived
    reason: both splits are recorded as intended in 068; low
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: BE-DEPS-002 and BE-DEPS-006 Verifications ask for an update process their Statements do not name — `framework/rules/security-backend.md`"
    outcome: waived
    reason: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: BE-QUERY-004, BE-POOL-005 and BE-TXN-004 repeat PostgreSQL background sentences — `framework/rules/performance-backend.md`"
    outcome: waived
    reason: the repeated sentences are accurate and not contradictory; each rule stands alone
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: ticked task-1 items in 037, 038 and 039 say one RFC 2119 keyword, and 039's task 4 names an alert — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md`"
    outcome: waived
    reason: a resolved question records the reasoning at the time; rewriting it records a fiction
    decided-at: 2026-10-20T00:49:42Z
    decided-by: andy@stone.dev
  - key: "other: `BE-COORD-003`'s Verification exempts a declared loss-tolerant message from both orderings, commit-then-publish and publish-then-commit, and nothing says whether a spurious publish before a rolled-back commit was meant to be covered — `framework/rules/concurrency-backend.md` — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: the owner's decision of 2026-11-19 names loss-tolerant and the Statement is unchanged; a narrower or wider scope is the owner's, not an accuracy defect (low confidence)
    decided-at: 2026-11-20T04:14:30Z
    decided-by: rule
  - key: "other: task 8's done-when item that bytes before and after are measured has no record in the tree — `specs/038-concurrency-backend-rules/tasks.md` — `specs/038-concurrency-backend-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: a ticked task's value is spent, git keeps the diff (19,388 to 19,512 bytes), and no gate reads the figure"
    decided-at: 2026-11-20T04:14:30Z
    decided-by: rule
  - key: "other: the COORD row of `specs/038-concurrency-backend-rules/spec.md`'s category table and the `BE-COORD-003` row of `specs/038-concurrency-backend-rules/plan.md` describe the Statement without the loss-tolerant scoping — `specs/038-concurrency-backend-rules/plan.md` — `specs/038-concurrency-backend-rules/plan.md`"
    outcome: waived
    reason: the Statement is unchanged and both rows describe it, so neither claim is false; the scoping is in the Verification, the scenario and the Resolved Question
    decided-at: 2026-11-20T04:14:30Z
    decided-by: rule
  - key: "other: `specs/034-performance-backend-rules/spec.md`'s Boundaries bullet and `framework/rules/performance-backend.md` `BE-POOL-005` describe `BE-COORD-003` without the loss-tolerant scoping — `specs/034-performance-backend-rules/spec.md` — `specs/034-performance-backend-rules/spec.md`"
    outcome: waived
    reason: another spec's files, not this spec's to edit; each states its own scope and the claim stays true for a message that must be published
    decided-at: 2026-11-20T04:14:30Z
    decided-by: rule
---

# Review — 038-concurrency-backend-rules

## Summary

0 MUST, 0 SHOULD, not blocking. Window: diff base `fa4ae5c7` (base-source reopen), 6 scope entries; the change is `BE-COORD-003`'s Verification in `framework/rules/concurrency-backend.md`. Three independent readers: A read the three `security-*.md` files whole (193,813 bytes) and the in-scope files; B read the nine other rule files whole (216,167 bytes), `AGENTS.md` Gotchas to Design Principles and `specs/system.md`, and judged both stored decisions naming the rule file (both still hold); C judged AC1 to AC8 (all met, `scripts/lint-rule-ids.sh` run, the backend selection driven through the release binary). Unread: `runtime/CHANGELOG.md` (610,176 bytes; only the window diff and the 038 bullet were read, so it is counted unread); `specs/008-security-rules/data-model.md` was not opened by C (AC1 judged from the lint run and the file structure). Not re-run by the readers: `cargo test` (taken from the commit message of `84e090b9`). Unexamined for want of a fetched copy: the PostgreSQL documentation quotes in `specs/038-concurrency-backend-rules/scenarios/lock-the-parent-before-rewriting-its-children.md` (recorded as fetched 2026-11-08), and the 017 and 036 data-model basis of one stored waiver. Four observations, all waived by the rule with reasons; no fix was written, so no fix round.

0 fix round(s): 0 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

8 of 8 acceptance criteria judged met.

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

- AC1 — met: framework/rules/concurrency-backend.md ends -backend.md; 13 `### BE-*` headings with Statement/Verification/Source, no Rationale; scripts/lint-rule-ids.sh rc=0 (12 files, 308 IDs)
- AC2 — met: IDs BE-{RACE,LOCK,TXN,COORD}-NNN share no category with the security, api, performance, reliability or observability backend files; scripts/lint-rule-ids.sh rc=0
- AC3 — met: header of framework/rules/concurrency-backend.md declares Categories: RACE, LOCK, TXN, COORD with the BE-{CATEGORY}-{NNN} format
- AC4 — met: RACE-001..003, LOCK-001/002, TXN-001..004 each carry a design-time-commitment Verification enforced by /analyze; the changed COORD-003 Verification keeps that form
- AC5 — met: 8 MUST (RACE-001, RACE-003, TXN-002, TXN-003, COORD-001..004), the rest SHOULD; no Statement changed in the window
- AC6 — met: BE-IDEMP, BE-COND-001, BE-RETRY-001/005, CFG-CONST-003 and BE-CACHE-003 exist as cited; grep -i 'pool|connection' over the concurrency file returns nothing
- AC7 — met: framework/bootstrap/ductus.md shared-files row for concurrency-backend.md; discover_rule_files.rs classify maps -backend.md to Backend; release ductus discover-rule-files with surfaces=[backend] selects it, [frontend] does not
- AC8 — met: spec.md category table cites RACE-001/002/003, LOCK-001/002, TXN-001..004, COORD-001..004 and all 13 exist as headings in framework/rules/concurrency-backend.md

## Observations

- other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md` — `framework/rules/concurrency-backend.md` — **waived**: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
- other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md` — `framework/rules/concurrency-backend.md` — **waived**: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
- other: `BE-COORD-003`'s Verification exempts a declared loss-tolerant message from both orderings, commit-then-publish and publish-then-commit, and nothing says whether a spurious publish before a rolled-back commit was meant to be covered — `framework/rules/concurrency-backend.md` — `framework/rules/concurrency-backend.md` — **waived**: the owner's decision of 2026-11-19 names loss-tolerant and the Statement is unchanged; a narrower or wider scope is the owner's, not an accuracy defect (low confidence) (decided by the rule)
- other: task 8's done-when item that bytes before and after are measured has no record in the tree — `specs/038-concurrency-backend-rules/tasks.md` — `specs/038-concurrency-backend-rules/tasks.md` — **waived**: spent task history: a ticked task's value is spent, git keeps the diff (19,388 to 19,512 bytes), and no gate reads the figure (decided by the rule)
- other: the COORD row of `specs/038-concurrency-backend-rules/spec.md`'s category table and the `BE-COORD-003` row of `specs/038-concurrency-backend-rules/plan.md` describe the Statement without the loss-tolerant scoping — `specs/038-concurrency-backend-rules/plan.md` — `specs/038-concurrency-backend-rules/plan.md` — **waived**: the Statement is unchanged and both rows describe it, so neither claim is false; the scoping is in the Verification, the scenario and the Resolved Question (decided by the rule)
- other: `specs/034-performance-backend-rules/spec.md`'s Boundaries bullet and `framework/rules/performance-backend.md` `BE-POOL-005` describe `BE-COORD-003` without the loss-tolerant scoping — `specs/034-performance-backend-rules/spec.md` — `specs/034-performance-backend-rules/spec.md` — **waived**: another spec's files, not this spec's to edit; each states its own scope and the claim stays true for a message that must be published (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `framework/rules/accessibility-frontend.md`
- `framework/rules/api-backend.md`
- `framework/rules/configuration-cross.md`
- `framework/rules/reliability-backend.md`
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
- `specs/039-reliability-backend-rules/plan.md`
- `specs/039-reliability-backend-rules/scenarios/expendable-messages-may-be-dropped-when-recorded.md`
- `specs/039-reliability-backend-rules/spec.md`
- `specs/039-reliability-backend-rules/tasks.md`
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
