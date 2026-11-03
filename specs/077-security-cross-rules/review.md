---
spec: 077-security-cross-rules
last-run: 2026-11-03T19:11:22Z
reviewed-against: 6e2d64bf4ba720cd5092b148214c0d5e7d3a4ea9
diff-base: 898a49ecbde8e13c9bff79aff165fc0e8116a39c
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 90
scope: 168
skipped-passes: []
reviewed-digest:
  data-model.md: cbf317f3d8635097191d82c1b4a9cc35aedd9275dd026eaeb68a9685f241be0d
blocking: false
dispositions:
  fixed: 32
  deferred: 2
  waived: 0
  undispositioned: 0
decisions:
  - key: "bug: the brownfield security audit leaves a case yielded to a rule in a file it does not read (configuration-cross.md, api-backend.md, observability-backend.md) unpreviewed, against the changelog's and 077 §Distribution's claim that it is reported under the owner's ID — `framework/bootstrap/ductus-procedure.md:209`"
    outcome: deferred
    target: specs/077-security-cross-rules/tasks.md
    task: "11"
    decided-at: 2026-11-03T19:11:22Z
    decided-by: andy@stone.dev
  - key: "other: once 0.62.0 removes the four deprecated rules, check-rule-ids will report a Blocking unknown-rule finding for each record that must keep the old ID (077's spec, 008's and 017's signposts) — `specs/077-security-cross-rules/spec.md`"
    outcome: deferred
    target: specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md
    decided-at: 2026-11-03T19:11:22Z
    decided-by: andy@stone.dev
  - key: rule QUAL-GROUND-001 at scripts/audit/deprecation-removal.sh
    outcome: deferred
    target: specs/067-analyze-and-audit-checks/tasks.md
    task: "21"
    decided-at: 2026-11-03T19:11:22Z
    decided-by: andy@stone.dev
---

# Review — 077-security-cross-rules

## Summary

0 MUST and 0 SHOULD violations counted, 0 low-confidence findings. One SHOULD fired, QUAL-GROUND-001 on Family 40's restated label parse, and is deferred to 067 task 21 under the owner's 2026-11-02 decision, so it leaves the counts. The five passes (security, reuse, quality, efficiency, simplicity) ran on 2026-11-03 over base `898a49ec`, the parent of 077's `in-progress` commit, measured and not overridden, against the working tree at `3e9f3554` and `eb1878a1`, in five read-only agent slices: 077's rule files and the code checking them; how 077 ships the file, with its prose and CI; Family 40; the spec artifacts 077's work wrote; and the window's residue no other record read (076's sweep edits to other specs and docs, `runtime/tests/`, and `c5ad7370`'s `decisions.rs` and analyze-reference hunks, the only code changed after 076's review). The host re-checked each slice's observations against the tree. `examined: 90` counts the in-scope files the passes read, whole or by the window's hunks, of the 168 the scope resolves at write time. Not examined (78): the 18 generated mirrors, which a generator re-run on a clean clone of `3e9f3554` reported in sync with their sources (`.ductus/reference/audit.md` is regenerated from this run's `audit.md` at its commit); 076's 10 command and reference sources and 10 runtime files, read in depth by 076's review through its last fix round `104cdf69`; 8 runtime files and `framework/runtime-tools.txt` carrying another session's uncommitted 078 work, with 078's `data-model.md`, `plan.md` and `tasks.md` and 066's new fix-round scenario; the own artifacts of 057, 058 and 076, reviewed by their own records; `framework/bootstrap/govern.md`, byte-identical to `ductus.md` by `cmp`; and this run's three writes, 047's `tasks.md`, 067's new scenario and this record. 34 observations: 32 fixed in the run and 2 deferred (the brownfield audit's yielded cases to 077 task 11, and records citing removed IDs to 067's scenario `a-record-cites-a-removed-rule`), each by the owner's decisions of 2026-11-03: the 29 reviewer-prescribed fixes in one approval, each applied as prescribed and checked by diff against its prescription with no re-read, as 076's and 058's prescribed rounds were; and six judgment calls one at a time, among them 047 reopened, `done` to `in-progress`, to correct AC2, and Family 40's README entry and `audit.md` item 40 rewritten to point at its scenario, a prose-only change not re-read. NOTEs below the bar (contrived inputs, a duplicated test helper, wording no artifact relies on) are recorded in no list.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Deferred findings

### DEFERRED: QUAL-GROUND-001 — Family 40 restates the runtime's label parse instead of binding to it

- **File**: `scripts/audit/deprecation-removal.sh:136-173`
- **Finding**: The family's correctness depends on check-rule-ids' section_is_deprecated, heading_id_regex and parse_atx_heading, and its embedded python restates them; no test runs the runtime's parse against the family's. The owner's 2026-11-02 decision keeps the parse here until a primitive replaces it.
- **Auto-fixable**: no
- **Suggested fix**: 067 task 21: a primitive returning each label, sharing check-rule-ids' reader, with this script as its entry point.
- **Deferred**: to `specs/067-analyze-and-audit-checks/tasks.md` task 21

## Observations

- convention: `barred_values`' doc comment said a missing verb gives `None`, which the code cannot detect — `runtime/src/primitives/rule_sections.rs:469` — **fixed**
- convention: the runtime workflow's trigger comment listed the rule-file corpus tests without SEC-LOG-001's value-list test — `.github/workflows/runtime.yml:60` — **fixed**
- bug: the brownfield security audit leaves a case yielded to a rule in a file it does not read (configuration-cross.md, api-backend.md, observability-backend.md) unpreviewed, against the changelog's and 077 §Distribution's claim that it is reported under the owner's ID — `framework/bootstrap/ductus-procedure.md:209` — **deferred** to `specs/077-security-cross-rules/tasks.md` task 11
- other: the 0.60.0 upgrade replaces an adopter-authored specs/rules/security-cross.md, against docs/rules.md's promise that a file you added is never touched — `framework/bootstrap/ductus.md:865` — **fixed**
- convention: the changelog said a copy pinned after 0.59.11 kept all four moved rules before ad8a36f9, though two arrived only in 1598b87a — `runtime/CHANGELOG.md:87` — **fixed**
- convention: 067 task 21 still said Family 40's test passes unchanged against the entry point, after its scenario was reworded — `specs/067-analyze-and-audit-checks/tasks.md:179` — **fixed**
- convention: 026 task 3 called round NOTEs discarded, a disposition name 076 retired — `specs/026-framework-self-audit/tasks.md:74` — **fixed**
- convention: the audit README's contract sentence named migration-coverage.sh as printing python findings, which it does not — `scripts/audit/README.md:33` — **fixed**
- convention: the audit README said Family 40's control case passes on a tree with no label, which it does not run on — `scripts/audit/README.md:79` — **fixed**
- convention: the audit README entry and audit.md item 40 copied Family 40's scenario lists rather than pointing at it — `framework/reference/audit.md:64` — **fixed**
- convention: 067's scenario attributed Family 40's first parse miss to an owner decision — `specs/067-analyze-and-audit-checks/scenarios/deprecation-labels-are-read-by-the-runtime.md:9` — **fixed**
- convention: 067's scenario edge case on an unreadable rule file did not say the other files are still read — `specs/067-analyze-and-audit-checks/scenarios/deprecation-labels-are-read-by-the-runtime.md:20` — **fixed**
- convention: 026 task 3 called spec 078 a draft after it was clarified — `specs/026-framework-self-audit/tasks.md:84` — **fixed**
- convention: 026 task 2 and 067 task 14 opened a bullet with the retired disposition name Routed — `specs/026-framework-self-audit/tasks.md:63` — **fixed**
- convention: 077's plan described lint-rule-ids.sh's pre-077 allowlist — `specs/077-security-cross-rules/plan.md:23` — **fixed**
- convention: 077's plan described CFG-ENV-006's pre-077 text in the present tense — `specs/077-security-cross-rules/plan.md:65` — **fixed**
- convention: 077's plan relisted BE-LOG-002's values word for word, dropping its one-time-code examples — `specs/077-security-cross-rules/plan.md:30` — **fixed**
- convention: 077's plan and task 3 said a deprecated rule's Verification is one sentence, where it is two — `specs/077-security-cross-rules/plan.md:54` — **fixed**
- convention: 077's plan counted its data-model twice among the registration sites — `specs/077-security-cross-rules/plan.md:156` — **fixed**
- convention: 008 task 44 and 077 task 8 said each gap was present at 224d63f5, though some concern rules 077 wrote — `specs/008-security-rules/tasks.md:584` — **fixed**
- convention: 008's signpost said 008's data-model registers SEC and called SEC-LOG-001 a successor — `specs/008-security-rules/spec.md:16` — **fixed**
- convention: 077 task 8's done-when said a round reported no MUST or SHOULD, where the bound applied was none in rule text — `specs/077-security-cross-rules/tasks.md:81` — **fixed**
- convention: 003 task 5 put implementing the scenario before resolving its open question — `specs/003-bootstrap-automation/tasks.md:48` — **fixed**
- convention: 077 task 10 did not record that run-all.sh failed at 62814405 on 076's Family 7 pairs, nor the commit whose gate passes — `specs/077-security-cross-rules/tasks.md:93` — **fixed**
- convention: 041's scenario called the lasting keep an analyze discard after 076's rename — `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md:30` — **fixed**
- convention: 063's plan said analyze's step names the discard as the further fix after 076's rename — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md:63` — **fixed**
- convention: docs/analyze.md's lapse rule listed less complete work than the Deferrals table, so a rule-file deferral read as never lapsing — `docs/analyze.md:172` — **fixed**
- convention: 020's data-model must-violations row omitted the violations a deferral releases — `specs/020-code-review/data-model.md:112` — **fixed**
- convention: 020's data-model decisions row omitted a deferral's optional reason — `specs/020-code-review/data-model.md:123` — **fixed**
- convention: 022 task 10 was half-swept, keeping routes and discards beside waivers — `specs/022-deterministic-runtime/tasks.md:139` — **fixed**
- convention: 066's scenario said process-waivers' duplicate notice interpolates the reason, which it does not — `specs/066-audit-records-and-the-completion-gate/scenarios/process-waivers-notices-flatten-stored-text.md:9` — **fixed**
- convention: 066 task 14 bounded four lists, where its scenario bounds every list, notices included — `specs/066-audit-records-and-the-completion-gate/tasks.md:96` — **fixed**
- other: 047's AC2 listed a finding's disposition without the deferral to an external work item alone that 076 added — `specs/047-analyze-findings-durability/spec.md:149` — **fixed**
- other: once 0.62.0 removes the four deprecated rules, check-rule-ids will report a Blocking unknown-rule finding for each record that must keep the old ID (077's spec, 008's and 017's signposts) — `specs/077-security-cross-rules/spec.md` — **deferred** to `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`

## Skipped passes

*None.*

## Unexamined governance

*None.*
