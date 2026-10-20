---
spec: 039-reliability-backend-rules
last-run: 2026-10-20T00:54:13Z
reviewed-against: 7d295df2b228f718bb325b704791f9da9389ce1d
diff-base: 37c8219c8730fd23b5fbac7629c11fd2d6bc5b3b
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 90
scope: 94
skipped-passes: []
reviewed-digest: {}
blocking: false
dispositions:
  fixed: 0
  routed: 10
  discarded: 16
  undispositioned: 0
decisions:
  - key: "other: 039's ticked task 5 describes BE-DRAIN-002's propagation delay as counted within the grace period, which 034's review reworded — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: 036's ticked task 2 names the manifest slot it used at the time — `specs/036-quality-cross-rules/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-013 counts expiring pre-signed URLs as single-use short-lived values; AWS documents them as reusable bearer tokens up to 7 days, and FE-STORAGE-003 forbids them — `framework/rules/security-backend.md`"
    outcome: routed
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-013 allows a single-use token anywhere in a URL while FE-STORAGE-003 allows it only in a query parameter — `framework/rules/security-frontend.md`"
    outcome: routed
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "security: BE-AUTHN-001's Verification names only OWASP's headline parameter minimums — `framework/rules/security-backend.md`"
    outcome: routed
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-015 flags a handshake-URL token that BE-API-013 may permit — `framework/rules/security-backend.md`"
    outcome: routed
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: QUAL-GROUND-001 cedes configuration to CFG rules that do not cover a required key missing from a file or flag — `framework/rules/quality-cross.md`"
    outcome: routed
    target: specs/017-derive-dont-ask/scenarios/required-config-values-from-any-source.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md`"
    outcome: routed
    target: specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: 025's task 10 named only AC4 among the places that describe skip-and-warn — `specs/025-rule-opt-out/tasks.md`"
    outcome: routed
    target: specs/025-rule-opt-out/tasks.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: 063's task 12 missed AC5, plan.md, README.md, the command sources and the tool description that still describe the fallback — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`"
    outcome: routed
    target: specs/063-oversized-artifacts-warn-with-a-fix/tasks.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md`"
    outcome: routed
    target: specs/033-rule-surface-setting/tasks.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: 019's task 1 did not cover a host reading config by hand on the markdown-only path — `specs/019-config-decisions/tasks.md`"
    outcome: routed
    target: specs/019-config-decisions/tasks.md
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: FE-SEMHTML-001 and FE-ARIA-001 both forbid a div with role=button — `framework/rules/accessibility-frontend.md`"
    outcome: discarded
    reason: consistent overlap of two MUSTs with no contradiction; consolidating them changes rule ownership, which 069's clarify owns
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: FE-KBD-002 and FE-CONTRAST-001 both require 3:1 for focus indicators — `framework/rules/accessibility-frontend.md`"
    outcome: discarded
    reason: the owner decided FE-KBD-002's MUST carries the 3:1 ratio (069 Resolved Questions, 2026-10-19); the overlap is consistent
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: accessibility-frontend.md's first header paragraph is long — `framework/rules/accessibility-frontend.md`"
    outcome: discarded
    reason: style only; the content is accurate, and 069's first open question owns what the header claims about conformance
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md`"
    outcome: discarded
    reason: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: BE-REPR-001's Verification names specs/system.md for the API-wide representation — `framework/rules/api-backend.md`"
    outcome: discarded
    reason: the Statement's one representation across the API needs a declared one to be checkable; the Verification names where it is declared
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: BE-SCHEMA-001's SDL-versus-introspection split and BE-ERRENV-001's overlap with BE-ERR-002 — `framework/rules/api-backend.md`"
    outcome: discarded
    reason: both splits are recorded as intended in 068; low
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: BE-DEPS-002 and BE-DEPS-006 Verifications ask for an update process their Statements do not name — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md`"
    outcome: discarded
    reason: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: BE-QUERY-004, BE-POOL-005 and BE-TXN-004 repeat PostgreSQL background sentences — `framework/rules/performance-backend.md`"
    outcome: discarded
    reason: the repeated sentences are accurate and not contradictory; each rule stands alone
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md`"
    outcome: discarded
    reason: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: ticked task-1 items in 037, 038 and 039 say one RFC 2119 keyword, and 039's task 4 names an alert — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
  - key: "other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md`"
    outcome: discarded
    reason: a resolved question records the reasoning at the time; rewriting it records a fiction
    decided-at: 2026-10-20T00:54:13Z
    decided-by: andy@stone.dev
---

# Review — 039-reliability-backend-rules

## Summary

Five-pass review of 039 over `37c8219c..7d295df2` plus the working tree. Its 94 scope entries are the shared window of the 2026-10-19 rule-file campaign that 034's review read this session — less 045's restored `plan.md` — plus the new `review.md` and `analysis.md` of 034, 036, 037 and 038. **Method.** The shared entries were read by 034's review (eight parallel reviewer passes, every observation re-verified, the whole fixed diff re-read), and every later change to them is this session's own fix in the 034–038 commits, re-read as diffs; this pass read in full `framework/rules/reliability-backend.md` (including the `BE-DRAIN-002` grace-period fix 034's review made), 039's spec, plan and tasks, and the eight new records. **Examined: 90 of 94.** The four generated command mirrors (`.claude/commands/ductus/analyze.md`, `.claude/commands/ductus/review.md`, `.pi/prompts/ductus-analyze.md`, `.pi/prompts/ductus-review.md`) were not read; `scripts/gen-claude-commands.sh --check` reports them in sync. **Findings: none** — no loaded rule's trigger fires on rule prose or these spec artifacts. **Observations: 26.** One new, discarded: 039's ticked task 5 still describes `BE-DRAIN-002`'s delay as counted within the grace period, spent history of the wording 034's review aligned with `BE-DRAIN-001`. The 10 routed and 15 discarded observations standing from 034's and 036's passes still hold and are recorded with the same dispositions and keys. **039's acceptance criteria:** all ten verified against `reliability-backend.md` — the canonical schema and `-backend.md` suffix; `TIMEOUT`, `RETRY`, `BREAKER`, `DRAIN` and `BULK` declared and disjoint (`scripts/lint-rule-ids.sh` passes over 297 IDs); timeouts, bounded retries, breakers and shutdown covered with design-time Verification clauses; four MUSTs (`BE-TIMEOUT-001`, `BE-RETRY-001`, `BE-RETRY-005`, `BE-DRAIN-001`) and nine SHOULDs; `BE-POOL-002`, `BE-IDEMP`, `BE-ASYNC` cited, and `CFG-CONST-003` cited from exactly the ten rules AC6 names; the manifest row between `quality-cross.md` and `security-backend.md`; 034's forward reference resolving here; AC9's four added rules with `BE-RETRY-005` carrying no monitoring or alerting requirement; and `BE-TIMEOUT-003` naming PostgreSQL `statement_timeout`, MySQL `max_execution_time` for read-only `SELECT` only, MongoDB `maxTimeMS` and SQL Server's client-driver timeout, and citing `BE-POOL-002`, `BE-POOL-005` and `CFG-CONST-003`.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- other: 039's ticked task 5 describes BE-DRAIN-002's propagation delay as counted within the grace period, which 034's review reworded — `specs/039-reliability-backend-rules/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction
- other: 036's ticked task 2 names the manifest slot it used at the time — `specs/036-quality-cross-rules/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction
- security: BE-API-013 counts expiring pre-signed URLs as single-use short-lived values; AWS documents them as reusable bearer tokens up to 7 days, and FE-STORAGE-003 forbids them — `framework/rules/security-backend.md` — **routed** to `specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md`
- security: BE-API-013 allows a single-use token anywhere in a URL while FE-STORAGE-003 allows it only in a query parameter — `framework/rules/security-frontend.md` — **routed** to `specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md`
- security: BE-AUTHN-001's Verification names only OWASP's headline parameter minimums — `framework/rules/security-backend.md` — **routed** to `specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md`
- security: BE-API-015 flags a handshake-URL token that BE-API-013 may permit — `framework/rules/security-backend.md` — **routed** to `specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md`
- other: QUAL-GROUND-001 cedes configuration to CFG rules that do not cover a required key missing from a file or flag — `framework/rules/quality-cross.md` — **routed** to `specs/017-derive-dont-ask/scenarios/required-config-values-from-any-source.md`
- other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md` — **routed** to `specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md`
- other: 025's task 10 named only AC4 among the places that describe skip-and-warn — `specs/025-rule-opt-out/tasks.md` — **routed** to `specs/025-rule-opt-out/tasks.md`
- other: 063's task 12 missed AC5, plan.md, README.md, the command sources and the tool description that still describe the fallback — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md` — **routed** to `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`
- other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md` — **routed** to `specs/033-rule-surface-setting/tasks.md`
- other: 019's task 1 did not cover a host reading config by hand on the markdown-only path — `specs/019-config-decisions/tasks.md` — **routed** to `specs/019-config-decisions/tasks.md`
- other: FE-SEMHTML-001 and FE-ARIA-001 both forbid a div with role=button — `framework/rules/accessibility-frontend.md` — **discarded**: consistent overlap of two MUSTs with no contradiction; consolidating them changes rule ownership, which 069's clarify owns
- other: FE-KBD-002 and FE-CONTRAST-001 both require 3:1 for focus indicators — `framework/rules/accessibility-frontend.md` — **discarded**: the owner decided FE-KBD-002's MUST carries the 3:1 ratio (069 Resolved Questions, 2026-10-19); the overlap is consistent
- other: accessibility-frontend.md's first header paragraph is long — `framework/rules/accessibility-frontend.md` — **discarded**: style only; the content is accurate, and 069's first open question owns what the header claims about conformance
- other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md` — **discarded**: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
- other: BE-REPR-001's Verification names specs/system.md for the API-wide representation — `framework/rules/api-backend.md` — **discarded**: the Statement's one representation across the API needs a declared one to be checkable; the Verification names where it is declared
- other: BE-SCHEMA-001's SDL-versus-introspection split and BE-ERRENV-001's overlap with BE-ERR-002 — `framework/rules/api-backend.md` — **discarded**: both splits are recorded as intended in 068; low
- other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md` — **discarded**: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
- other: BE-DEPS-002 and BE-DEPS-006 Verifications ask for an update process their Statements do not name — `framework/rules/security-backend.md` — **discarded**: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
- other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md` — **discarded**: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
- other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md` — **discarded**: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
- other: BE-QUERY-004, BE-POOL-005 and BE-TXN-004 repeat PostgreSQL background sentences — `framework/rules/performance-backend.md` — **discarded**: the repeated sentences are accurate and not contradictory; each rule stands alone
- other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md` — **discarded**: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
- other: ticked task-1 items in 037, 038 and 039 say one RFC 2119 keyword, and 039's task 4 names an alert — `specs/039-reliability-backend-rules/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction
- other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md` — **discarded**: a resolved question records the reasoning at the time; rewriting it records a fiction

## Skipped passes

*None.*

## Unexamined governance

*None.*
