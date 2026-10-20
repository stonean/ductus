---
spec: 063-oversized-artifacts-warn-with-a-fix
last-run: 2026-10-20T01:51:34Z
reviewed-against: a226ba3d1b6cb439327b350145eb0a02e59c55b0
diff-base: 37c8219c8730fd23b5fbac7629c11fd2d6bc5b3b
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 34
scope: 122
skipped-passes: []
reviewed-digest:
  data-model.md: 0bd537b2f16e9aa7114a70a1e21549711827954dcdb028809ddfe8d92c2285a5
  scenarios/an-invalid-threshold-fails-fast.md: fb341859bc463f2c51d84856d76f158208a4b8180f9d94bc5537fc9f1bb62c71
blocking: false
dispositions:
  fixed: 6
  routed: 9
  discarded: 17
  undispositioned: 0
decisions:
  - key: "other: 063's ticked tasks 1 and 6 describe the invalid-threshold notice and default fallback that task 12 replaced — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: 039's ticked task 5 describes BE-DRAIN-002's propagation delay as counted within the grace period, which 034's review reworded — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: 036's ticked task 2 names the manifest slot it used at the time — `specs/036-quality-cross-rules/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-013 counts expiring pre-signed URLs as single-use short-lived values; AWS documents them as reusable bearer tokens up to 7 days, and FE-STORAGE-003 forbids them — `framework/rules/security-backend.md`"
    outcome: routed
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-013 allows a single-use token anywhere in a URL while FE-STORAGE-003 allows it only in a query parameter — `framework/rules/security-frontend.md`"
    outcome: routed
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "security: BE-AUTHN-001's Verification names only OWASP's headline parameter minimums — `framework/rules/security-backend.md`"
    outcome: routed
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "security: BE-API-015 flags a handshake-URL token that BE-API-013 may permit — `framework/rules/security-backend.md`"
    outcome: routed
    target: specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: QUAL-GROUND-001 cedes configuration to CFG rules that do not cover a required key missing from a file or flag — `framework/rules/quality-cross.md`"
    outcome: routed
    target: specs/017-derive-dont-ask/scenarios/required-config-values-from-any-source.md
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md`"
    outcome: routed
    target: specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: 025's task 10 named only AC4 among the places that describe skip-and-warn — `specs/025-rule-opt-out/tasks.md`"
    outcome: routed
    target: specs/025-rule-opt-out/tasks.md
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md`"
    outcome: routed
    target: specs/033-rule-surface-setting/tasks.md
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: 019's task 1 did not cover a host reading config by hand on the markdown-only path — `specs/019-config-decisions/tasks.md`"
    outcome: routed
    target: specs/019-config-decisions/tasks.md
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: FE-SEMHTML-001 and FE-ARIA-001 both forbid a div with role=button — `framework/rules/accessibility-frontend.md`"
    outcome: discarded
    reason: consistent overlap of two MUSTs with no contradiction; consolidating them changes rule ownership, which 069's clarify owns
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: FE-KBD-002 and FE-CONTRAST-001 both require 3:1 for focus indicators — `framework/rules/accessibility-frontend.md`"
    outcome: discarded
    reason: the owner decided FE-KBD-002's MUST carries the 3:1 ratio (069 Resolved Questions, 2026-10-19); the overlap is consistent
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: accessibility-frontend.md's first header paragraph is long — `framework/rules/accessibility-frontend.md`"
    outcome: discarded
    reason: style only; the content is accurate, and 069's first open question owns what the header claims about conformance
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md`"
    outcome: discarded
    reason: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: BE-REPR-001's Verification names specs/system.md for the API-wide representation — `framework/rules/api-backend.md`"
    outcome: discarded
    reason: the Statement's one representation across the API needs a declared one to be checkable; the Verification names where it is declared
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: BE-SCHEMA-001's SDL-versus-introspection split and BE-ERRENV-001's overlap with BE-ERR-002 — `framework/rules/api-backend.md`"
    outcome: discarded
    reason: both splits are recorded as intended in 068; low
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: BE-DEPS-002 and BE-DEPS-006 Verifications ask for an update process their Statements do not name — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: no flag condition fires on its absence; adding it to the MUST or removing it changes scope, and no accuracy defect requires either
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md`"
    outcome: discarded
    reason: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md`"
    outcome: discarded
    reason: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: BE-QUERY-004, BE-POOL-005 and BE-TXN-004 repeat PostgreSQL background sentences — `framework/rules/performance-backend.md`"
    outcome: discarded
    reason: the repeated sentences are accurate and not contradictory; each rule stands alone
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md`"
    outcome: discarded
    reason: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: ticked task-1 items in 037, 038 and 039 say one RFC 2119 keyword, and 039's task 4 names an alert — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: discarded
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
  - key: "other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md`"
    outcome: discarded
    reason: a resolved question records the reasoning at the time; rewriting it records a fiction
    decided-at: 2026-10-20T01:51:34Z
    decided-by: andy@stone.dev
---

# Review — 063-oversized-artifacts-warn-with-a-fix

## Summary

Five-pass review of 063 over `37c8219c..a226ba3d` plus the working tree, after task 12's fail-fast fix (`a226ba3d`). The 122 scope entries are the shared window of the 2026-10-19 rule-file campaign that 034's review read in an earlier session, plus 063's own fix. **What this session read** (`examined: 34`): 063's `spec.md`, `plan.md`, `data-model.md`, `tasks.md` and scenario in full; `runtime/src/primitives/check_artifact_size.rs` in full; the fix's diffs to `runtime/src/primitives/mod.rs`, `runtime/src/schema/primitives.rs`, `runtime/src/mcp/server.rs`, `README.md`, `framework/bootstrap/ductus.md` and `framework/commands/{analyze,clarify,plan}.md`; all eleven `framework/rules/*.md` in full (`configuration-cross.md` and `quality-cross.md` by this pass, the other nine by two parallel reviewer passes reading the commit through git); `AGENTS.md` and `CLAUDE.md`; the 019 and 025 scenarios, specs and tasks; and 039's `review.md`. **Not read** (88 entries): `framework/bootstrap/govern.md` (`cmp` identical to `ductus.md`), the eight generated mirrors under `.claude/commands/ductus/` and `.pi/prompts/` (`scripts/gen-claude-commands.sh --check` in sync), `framework/commands/review.md` beyond its Instructions and Waivers, the plan-affected runtime files 063 did not change in this window, `docs/`, and the other specs' artifacts. **Findings: none remain.** One fired and was fixed in the run: `BE-COMPAT-001`, since the fix removes `threshold.rejected` and the `invalid` source from a result shipped since 0.58.0 and turns a successful response into an error; the plan now classifies the change as breaking and commits its release to a minor bump, the pre-1.0 compatibility boundary. `CFG-ENV-012` (no report of an unknown `[artifacts]` key) is a design-time rule `/{project}:analyze` enforces, and which review pass applies `configuration-cross.md` is 020's open question, so it is left to the analysis. **The fix:** a present `read-size-bytes` that is not a positive whole number, or a non-table `artifacts`, fails with `PrimitiveError::InvalidReadSize` naming the setting, the value, the file and the accepted form, verified through the CLI binary in a `/tmp` fixture; the threshold tests fail when the halt reverts to the default and when the value's flattening is removed (mutations run this session). **Observations: 32.** Six fixed in the run (the non-table halt documented everywhere, the value described as TOML renders it, a newline case added to the one-line test, `config_path` for a discarded tuple field, the message's setting derived from `SECTION` and `KEY`, the bootstrap comment rewrapped). One new discard: 063's spent tasks 1 and 6. The 9 routed and 16 discarded observations standing from 034's and 036's passes still hold and keep their text and dispositions; 039's routed note on 063's task 12 is discharged by this fix and is not carried.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- other: 063's ticked tasks 1 and 6 describe the invalid-threshold notice and default fallback that task 12 replaced — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction
- other: 039's ticked task 5 describes BE-DRAIN-002's propagation delay as counted within the grace period, which 034's review reworded — `specs/039-reliability-backend-rules/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction
- other: 036's ticked task 2 names the manifest slot it used at the time — `specs/036-quality-cross-rules/tasks.md` — **discarded**: spent task history: each records what the task said when it was done, and rewriting it records a fiction
- security: BE-API-013 counts expiring pre-signed URLs as single-use short-lived values; AWS documents them as reusable bearer tokens up to 7 days, and FE-STORAGE-003 forbids them — `framework/rules/security-backend.md` — **routed** to `specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md`
- security: BE-API-013 allows a single-use token anywhere in a URL while FE-STORAGE-003 allows it only in a query parameter — `framework/rules/security-frontend.md` — **routed** to `specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md`
- security: BE-AUTHN-001's Verification names only OWASP's headline parameter minimums — `framework/rules/security-backend.md` — **routed** to `specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md`
- security: BE-API-015 flags a handshake-URL token that BE-API-013 may permit — `framework/rules/security-backend.md` — **routed** to `specs/008-security-rules/scenarios/independent-review-findings-await-the-owner.md`
- other: QUAL-GROUND-001 cedes configuration to CFG rules that do not cover a required key missing from a file or flag — `framework/rules/quality-cross.md` — **routed** to `specs/017-derive-dont-ask/scenarios/required-config-values-from-any-source.md`
- other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md` — **routed** to `specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md`
- other: 025's task 10 named only AC4 among the places that describe skip-and-warn — `specs/025-rule-opt-out/tasks.md` — **routed** to `specs/025-rule-opt-out/tasks.md`
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
- quality: the halt on an `artifacts` that is not a table was missing from spec.md's edge case and AC5, the scenario, clarify.md, plan.md, README.md and the bootstrap comment — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **fixed**
- quality: InvalidReadSize's value was documented as written in the config file, when it is TOML's re-rendering — `runtime/src/primitives/mod.rs` — **fixed**
- quality: an_invalid_value_is_named_on_one_line exercised U+2028 but not the newline its comment names — `runtime/src/primitives/check_artifact_size.rs` — **fixed**
- simplicity: resolve_threshold discarded resolve_config's display name where config_path returns the path alone — `runtime/src/primitives/check_artifact_size.rs` — **fixed**
- simplicity: InvalidReadSize's message spelled the setting literally rather than from SECTION and KEY — `runtime/src/primitives/mod.rs` — **fixed**
- convention: the bootstrap's [artifacts] comment ran one line to 111 columns against the block's 80 — `framework/bootstrap/ductus.md` — **fixed**

## Skipped passes

*None.*

## Unexamined governance

*None.*
