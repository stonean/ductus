---
spec: 060-exec-analyze-assesses-each-loaded-rule
last-run: 2026-11-18T17:31:07Z
reviewed-against: 12dc867c749c5d6982377a6754c7f31684ae5cea
diff-base: 800164c3eaad8cadf8807986a539d13e26a7d3ac
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 17
scope: 18
unread:
  - docs/runtime.md
excluded: 0
contract-breaks: 0
skipped-passes: []
fix-rounds: 5
re-reads: 3
diff-checked: 1
not-re-read: 0
budget-stop: true
reviewed-digest:
  scenarios/an-assessment-sees-the-loaded-rule-files.md: 7a1341466362f69527aecd8d09472a6d9ba5ec6d9db5e30a0e316814fae06007
  scenarios/an-assessment-sees-the-rule-it-asks-about.md: 03489f5510e4b9cf5f38194f6a0e8ab6ffcff7272359914c991129ba72a9b70f
  scenarios/rule-judging-runs-from-repo-scripts.md: 19aaa6a8d61cbcfb40306a9c46c2e264d1d61d35b1b322bf38686e7abc93cd96
blocking: false
dispositions:
  fixed: 23
  deferred: 0
  waived: 1
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
---

# Review — 060-exec-analyze-assesses-each-loaded-rule

## Summary

Two reviewers read the 17-entry scope whole, each against half the rule files. R1 applied the three security files and quality-cross.md (203,809 bytes) in the security, quality, reuse and simplicity passes. R2 applied the other eight (204,717 bytes) in the efficiency and quality passes. Each reported every scope and rule file read whole against its wc -c size. Each read AGENTS.md from the full copy its session loaded at start, the file as committed at HEAD. R1 read review.md's Observations section only to its placeholder, so this run dispositioned its observations. They found 11 rule violations, all fixed in this run: BE-TIMEOUT-001 twice, CFG-ENV-001 twice, CFG-ENV-002, CFG-CONST-001, BE-INPUT-004, QUAL-CLAIM-001 three times and QUAL-GROUND-001. The passes' 16 observations are fixed, three of them broken contracts. Of the fix-round re-reads' eight NOTEs, seven are fixed and one waived. Their nine SHOULDs are fixed, the last two by edits a diff check matched, and the third round ran past the budget of two by the owner's decision. Each fix has a test case, and each case was seen failing against a mutant of the code it names: 62 mutants. docs/runtime.md entered the scope with this run's fix, so the passes did not read it; each fix-round re-read read it whole.

5 fix round(s): 3 re-read, 1 diff-checked, 1 exempt, 0 not re-read. Exempt: round 5. The budget stopped the loop: 3 re-reads found a MUST or SHOULD, against a budget of 2.

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

- bug: detect.py printed check-unfolded-specs' result under findings, a key it does not have, so an unfolded spec never reached the summary; it reads unfolded now — `scripts/analyze/detect.py` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: record.py never passed restricted to process-decisions, so a run with a target it could not read expired stored decisions step 18 retains; it passes it for every could-not-be-read reason now — `scripts/analyze/record.py` (breaks the spec's contract) — **fixed** (decided by the rule)
- gap: record.py refused every spec with a references: index, against the scenario's runnable on any feature; it records a broken reference and a duplicate repository as Advisory findings now — `scripts/analyze/record.py` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: split.py's render_file dropped each other-heading's body, which a judge needs to tell a malformed rule heading — `scripts/analyze/split.py` — **fixed** (decided by the rule)
- bug: judges' malformed-headings and step 15's unchecked integrity items never reached the record; check.py prints both, and record.py refuses an unacknowledged heading and records an unchecked item under integrity-not-checked — `scripts/analyze/record.py` — **fixed** (decided by the rule)
- bug: split.py's greedy balance could put a share or cell over its budget with no note; it adds bins until each fits, and names one that cannot — `scripts/analyze/split.py` — **fixed** (decided by the rule)
- bug: split.py and detect.py discarded discover-rule-files' notice lines, which analyze emits verbatim — `scripts/analyze/split.py` — **fixed** (decided by the rule)
- bug: an uncaught exception exited 1, the refused status, with a traceback (a missing DUCTUS_BIN, --share-bytes 0, a malformed hand-back); each is now exit 2 or a named check failure — `scripts/analyze/analyze_lib.py` — **fixed** (decided by the rule)
- bug: record.py recorded merged.json with no freshness check, so a stale merge could be written; it merges the checked hand-backs again itself — `scripts/analyze/record.py` — **fixed** (decided by the rule)
- bug: detection results carried no digest, so split.py and record.py read results older than the feature; detect.py records the feature's digest and both refuse a stale one — `scripts/analyze/detect.py` — **fixed** (decided by the rule)
- claim: combine.py's docstring said it refused on any check failure while the code exempted step 15; it refuses on step 15's too — `scripts/analyze/combine.py` — **fixed** (decided by the rule)
- claim: plan.md said both defaults came from the docs' Sizing section, which states neither figure; it now gives each figure's source and method — `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — **fixed** (decided by the rule)
- doc: plan.md's new section recorded verification evidence, a replay and mutant runs, which belongs in the commit; removed — `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — **fixed** (decided by the rule)
- test: detect.py's success path and record.py's unexamined reasons, duplicate-key refusal and references handling had no case; cases O, W, AA, AB and Z cover them, Z against the real release binary — `scripts/tests/test-analyze-scripts.py` — **fixed** (decided by the rule)
- simplicity: split.py's balance returned loads no caller used; fit uses them to keep each bin within its budget — `scripts/analyze/split.py` — **fixed** (decided by the rule)
- reuse: git rev-parse HEAD was spelled in split.py and record.py, and detect.py called read-spec twice; analyze_lib.head serves both, and detect.py calls read-spec once — `scripts/analyze/analyze_lib.py` — **fixed** (decided by the rule)
- other: fix-round 1 re-read: detect.py never removed a stale det/resolve-references.json, so a spec that dropped its references kept the last run's findings; detect.py now clears det/ each run — `scripts/analyze/detect.py` — **fixed** (decided by the rule)
- other: fix-round 1 re-read: detection freshness digested only the feature directory and never compared HEAD; it now digests the rule files and constitution, compares HEAD, and digests the working tree — `scripts/analyze/analyze_lib.py` — **fixed** (decided by the rule)
- other: fix-round 1 re-read: analyze_lib's docstring miscounted the variables it reads, and a huge ANALYZE_TIMEOUT_SECONDS overflowed subprocess; it names TMPDIR, and the value is bounded by MAX_TIMEOUT_SECONDS — `scripts/analyze/analyze_lib.py` — **fixed** (decided by the rule)
- other: fix-round 2 re-read: case Z's docstring called not-installed the one list the runtime omits when empty, which retired also is; corrected as prescribed — `scripts/tests/test-analyze-scripts.py` — **fixed** (decided by the rule)
- other: fix-round 2 re-read: record.py read resolve-references' lists leniently; it reads both strictly, with a case each — `scripts/analyze/record.py` — **fixed** (decided by the rule)
- other: fix-round 2 re-read: detection freshness missed an uncommitted edit outside the feature, rule files and constitution; detect.py records a digest of the working tree's uncommitted state, and case AI covers an untracked and a tracked edit — `scripts/analyze/analyze_lib.py` — **fixed** (decided by the rule)
- other: fix-round 3 re-read: plan.md said the working-tree digest lets a refusal name the file, which it cannot; it names the feature's, rule and constitution files and reports any other edit as the working tree — `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — **fixed** (decided by the rule)
- other: fix-round 3 re-read: tree_state ignores git's exit status, so a failing git diff digests as empty, and text mode folds CRLF, so a line-ending-only edit is invisible — `scripts/analyze/analyze_lib.py` — **waived**: git diff HEAD fails only with no HEAD commit, which head() refuses before the digest is compared, and the feature, rule files and constitution, the files detection judges by content, are digested byte for byte; the re-read judged both cases contrived (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

*None.*

## Unexamined governance

*None.*
