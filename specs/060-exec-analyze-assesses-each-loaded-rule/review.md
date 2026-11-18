---
spec: 060-exec-analyze-assesses-each-loaded-rule
last-run: 2026-11-18T18:45:59Z
reviewed-against: 537e1c9b66f539ae472945af7342803774a4f98c
diff-base: 554e09851d2776a8a525af156d3fa89cf4534afd
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
scope: 5
unread:
  - docs/runtime.md
excluded: 2
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  scenarios/an-assessment-sees-the-loaded-rule-files.md: 7a1341466362f69527aecd8d09472a6d9ba5ec6d9db5e30a0e316814fae06007
  scenarios/an-assessment-sees-the-rule-it-asks-about.md: 03489f5510e4b9cf5f38194f6a0e8ab6ffcff7272359914c991129ba72a9b70f
  scenarios/rule-judging-runs-from-repo-scripts.md: 19aaa6a8d61cbcfb40306a9c46c2e264d1d61d35b1b322bf38686e7abc93cd96
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: "perf: build_assess_spec_quality_request re-reads the spec file for every rule request, up to 167 times per step — `runtime/src/interpreter/payload.rs`"
    outcome: waived
    reason: each read is a small local file the OS caches, beside an LLM round trip per request; reading it once would thread a second value through the walker context for a gain nobody has measured
    decided-at: 2026-09-27T19:20:13Z
    decided-by: andy@stone.dev
  - key: "other: 039's ticked task 5 describes BE-DRAIN-002's propagation delay as counted within the grace period, which 034's review reworded — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: 036's ticked task 2 names the manifest slot it used at the time — `specs/036-quality-cross-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: QUAL-GROUND-001 cedes configuration to CFG rules that do not cover a required key missing from a file or flag — `framework/rules/quality-cross.md`"
    outcome: deferred
    target: specs/017-derive-dont-ask/scenarios/required-config-values-from-any-source.md
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md`"
    outcome: deferred
    target: specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: 025's task 10 named only AC4 among the places that describe skip-and-warn — `specs/025-rule-opt-out/tasks.md`"
    outcome: deferred
    target: specs/025-rule-opt-out/tasks.md
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: 063's task 12 missed AC5, plan.md, README.md, the command sources and the tool description that still describe the fallback — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`"
    outcome: deferred
    target: specs/063-oversized-artifacts-warn-with-a-fix/tasks.md
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md`"
    outcome: deferred
    target: specs/033-rule-surface-setting/tasks.md
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: 019's task 1 did not cover a host reading config by hand on the markdown-only path — `specs/019-config-decisions/tasks.md`"
    outcome: deferred
    target: specs/019-config-decisions/tasks.md
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: FE-SEMHTML-001 and FE-ARIA-001 both forbid a div with role=button — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: consistent overlap of two MUSTs with no contradiction; consolidating them changes rule ownership, which 069's clarify owns
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: FE-KBD-002 and FE-CONTRAST-001 both require 3:1 for focus indicators — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: the owner decided FE-KBD-002's MUST carries the 3:1 ratio (069 Resolved Questions, 2026-10-19); the overlap is consistent
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: accessibility-frontend.md's first header paragraph is long — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: style only; the content is accurate, and 069's first open question owns what the header claims about conformance
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md`"
    outcome: waived
    reason: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: BE-REPR-001's Verification names specs/system.md for the API-wide representation — `framework/rules/api-backend.md`"
    outcome: waived
    reason: the Statement's one representation across the API needs a declared one to be checkable; the Verification names where it is declared
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: BE-SCHEMA-001's SDL-versus-introspection split and BE-ERRENV-001's overlap with BE-ERR-002 — `framework/rules/api-backend.md`"
    outcome: waived
    reason: both splits are recorded as intended in 068; low
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: ticked task-1 items in 037, 038 and 039 say one RFC 2119 keyword, and 039's task 4 names an alert — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md`"
    outcome: waived
    reason: a resolved question records the reasoning at the time; rewriting it records a fiction
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: fix-round 3 re-read: tree_state ignores git's exit status, so a failing git diff digests as empty, and text mode folds CRLF, so a line-ending-only edit is invisible — `scripts/analyze/analyze_lib.py`"
    outcome: waived
    reason: git diff HEAD fails only with no HEAD commit, which head() refuses before the digest is compared, and the feature, rule files and constitution, the files detection judges by content, are digested byte for byte; the re-read judged both cases contrived
    decided-at: 2026-11-18T17:31:07Z
    decided-by: rule
  - key: "other: fix-round re-read: _settle's DUCTUS_BIN check runs for every script, so briefs.py, which calls no primitive, and a usage error also fail at startup with no runtime built — `scripts/analyze/analyze_lib.py`"
    outcome: waived
    reason: briefs.py runs only after split.py, which needs the runtime, so the check fails no run that could otherwise succeed, and its message names the build command
    decided-at: 2026-11-18T18:45:59Z
    decided-by: rule
---

# Review — 060-exec-analyze-assesses-each-loaded-rule

## Summary

By the owner's decision the window is --since=554e0985, the last review's commit: everything before it was read whole by two reviewers and three fix-round re-reads, and that record kept budget-stop only because its last round was diff-checked. The window holds 537e1c9b, the fix for the analysis judges' five findings. One reviewer read the four scope entries, the spec, its three scenarios and all 12 rule files whole, each reported against its wc -c size, in all five passes. It took the pass mapping from its brief rather than re-reading review.md's sections. It found two low-confidence SHOULDs, both fixed: QUAL-TEST-001 (case AJ now also takes a budget of 1) and CFG-ENV-001 (WORK_DIR_NAME names ANALYZE_WORK's default). Its one observation is fixed too: DUCTUS_BIN is now checked at startup, naming the setting, before any work. The stored waiver on tree_state's git exit status was re-fired, since its issue still stands. The fix round's re-read was clean, with one NOTE, waived. Each fix's case was seen failing against a mutant. docs/runtime.md entered the scope with this run's fix, so the pass did not read it; the fix-round re-read read it whole.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

## Observations

- bug: a bad DUCTUS_BIN is not checked at startup, so detect.py deletes the prior det/*.json and creates the work directory before failing, and the error names the path but not DUCTUS_BIN; the plan says each setting is checked at startup, a bad value fails before any work, and the error names the setting — `scripts/analyze/analyze_lib.py:112` — **fixed** (decided by the rule)
- other: fix-round 3 re-read: tree_state ignores git's exit status, so a failing git diff digests as empty, and text mode folds CRLF, so a line-ending-only edit is invisible — `scripts/analyze/analyze_lib.py` — **waived**: git diff HEAD fails only with no HEAD commit, which head() refuses before the digest is compared, and the feature, rule files and constitution, the files detection judges by content, are digested byte for byte; the re-read judged both cases contrived (decided by the rule)
- other: fix-round re-read: _settle's DUCTUS_BIN check runs for every script, so briefs.py, which calls no primitive, and a usage error also fail at startup with no runtime built — `scripts/analyze/analyze_lib.py` — **waived**: briefs.py runs only after split.py, which needs the runtime, so the check fails no run that could otherwise succeed, and its message names the build command (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `scripts/npm-audit-allowed.json` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/plan.md` — 022-deterministic-runtime

## Unexamined governance

*None.*
