---
spec: 045-decision-state-drift-detection
last-run: 2026-11-12T00:32:34Z
analyzed-against: ab2ad0853b359dfe50598581bc9302daf093ef2a
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 17
analyzed-digest:
  data-model.md: a0eaede24370550bdeeb354b9d1251bcaf0a83109219f58944b2dd066ee69855
  plan.md: 5ea3131d1c7c0aad7f1ef620a41cd5d111a538ab58ce71a3f4a4a83d7399f85e
  review.md: 81930205c4a39cb4d673aeeb283d6b60b62e42ca1adf83f88010240f8ecba2a6
  spec.md: 82cad93cbea7f84885222be8beb9a2f8ac03a59ced2de7ff4757f99428b8f854
  tasks.md: 8b46078bdad8d7f1d9d0823451246d232c0be79926c3411bc6b9223d3ce8d8bf
unexamined-by-reason:
  not-a-live-claim: 8
  root-absent: 4
  ships-to-adopter: 5
blocking: false
dispositions:
  fixed: 6
  deferred: 1
  waived: 3
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-INPUT-004: the criterion-path family probes a path taken from spec text with repo.join(candidate).exists() and no `..` screen, so a criterion can make the runtime stat a file outside the project"
    outcome: deferred
    target: specs/067-analyze-and-audit-checks/tasks.md
    task: "27"
    reason: the owner ruled that spec text is input to this rule (2026-11-11) and deferred the screen to 067, which owns the analyze checks
    decided-at: 2026-11-11T17:22:42Z
    decided-by: andy@stone.dev
  - key: "body-path-existence — names `../spec.md`, which existed in this repository and no longer resolves: \"For each inline `[text](target)` in a scanned artifact, resolve `target` relative to the *containing file's* directory and keep it only when the result lands inside the feature directory. This is what makes `../spec.md` from `scenarios/foo.md` a sibling while `../022-deterministic-runtime/spec.md` from `spec.md` is not — the distinction the spec's §Scope depends on, and the one the Motivation measurement relied on (21 cross-feature links in `## Motivation` sections, all out of scope).\""
    outcome: waived
    reason: "correct as written: `../spec.md` is the worked example of a link resolved from the hypothetical `scenarios/foo.md`, not a path the plan claims exists; the family resolves it from plan.md's directory instead"
    decided-at: 2026-11-11T17:22:42Z
    decided-by: rule
  - key: "body-path-existence — names `framework/workflows/registry.json`, which existed in this repository and no longer resolves: \"| `framework/workflows/registry.json` | yes | no → finding |\""
    outcome: waived
    reason: "correct as written: the row is the worked example's expected outcome, the path deleted by 531e3ea and no longer resolving; the deletion is stated on the example's lead-in line, which the per-line exemption does not reach"
    decided-at: 2026-11-11T17:22:42Z
    decided-by: rule
  - key: "body-path-existence — names `scripts/audit/registry-equivalence.sh`, which existed in this repository and no longer resolves: \"| `scripts/audit/registry-equivalence.sh` | yes | no → finding |\""
    outcome: waived
    reason: "correct as written: the row is the worked example's expected outcome, the script deleted by 3ff65445 and no longer resolving; the deletion is stated on the example's lead-in line, which the per-line exemption does not reach"
    decided-at: 2026-11-11T17:22:42Z
    decided-by: rule
---

# Analysis — 045-decision-state-drift-detection

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 17 unexamined target(s). Dispositions: 6 fixed, 1 deferred, 3 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- body-path-existence — names `../spec.md`, which existed in this repository and no longer resolves: "For each inline `[text](target)` in a scanned artifact, resolve `target` relative to the *containing file's* directory and keep it only when the result lands inside the feature directory. This is what makes `../spec.md` from `scenarios/foo.md` a sibling while `../022-deterministic-runtime/spec.md` from `spec.md` is not — the distinction the spec's §Scope depends on, and the one the Motivation measurement relied on (21 cross-feature links in `## Motivation` sections, all out of scope)." — `specs/045-decision-state-drift-detection/plan.md` — **waived**: correct as written: `../spec.md` is the worked example of a link resolved from the hypothetical `scenarios/foo.md`, not a path the plan claims exists; the family resolves it from plan.md's directory instead (decided by the rule)
- body-path-existence — names `framework/workflows/registry.json`, which existed in this repository and no longer resolves: "| `framework/workflows/registry.json` | yes | no → finding |" — `specs/045-decision-state-drift-detection/data-model.md` — **waived**: correct as written: the row is the worked example's expected outcome, the path deleted by 531e3ea and no longer resolving; the deletion is stated on the example's lead-in line, which the per-line exemption does not reach (decided by the rule)
- body-path-existence — names `scripts/audit/registry-equivalence.sh`, which existed in this repository and no longer resolves: "| `scripts/audit/registry-equivalence.sh` | yes | no → finding |" — `specs/045-decision-state-drift-detection/data-model.md` — **waived**: correct as written: the row is the worked example's expected outcome, the script deleted by 3ff65445 and no longer resolving; the deletion is stated on the example's lead-in line, which the per-line exemption does not reach (decided by the rule)

## Deferred to other specs

- rule-assessment — BE-INPUT-004: the criterion-path family probes a path taken from spec text with repo.join(candidate).exists() and no `..` screen, so a criterion can make the runtime stat a file outside the project — `specs/045-decision-state-drift-detection/plan.md` — **deferred** to `specs/067-analyze-and-audit-checks/tasks.md` task 27

## Unexamined targets

- not-a-live-claim: 8
- root-absent: 4
- ships-to-adopter: 5

## Fixed in this run

- grounding — spec.md Motivation: "it explicitly does not read source to confirm the claim is true" describes analyze's grounding step with no citation — `specs/045-decision-state-drift-detection/spec.md` — **fixed** (decided by the rule)
- grounding — spec.md Motivation: "artifact-vs-artifact consistency is /{project}:analyze's job by design" describes the review/analyze split with no citation — `specs/045-decision-state-drift-detection/spec.md` — **fixed** (decided by the rule)
- grounding — plan.md §Two families inside check-artifacts: "check-artifacts is already in TOOL_NAMES, PRIMITIVE_NAMES, ..." names registration sites with no line or command, and TOOL_NAMES no longer exists — `specs/045-decision-state-drift-detection/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md §Release path: "analyze-basic.jsonl records 13 lines" and "003-analyze is at status: clarified" carry no command, and the golden holds 17 lines — `specs/045-decision-state-drift-detection/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md §Implementation ownership: "check-artifacts' compiled description names its families and 067's data model records them" gives no path — `specs/045-decision-state-drift-detection/plan.md` — **fixed** (decided by the rule)
- decision-resolution — plan.md still scopes the path family to done specs and counts eighteen criteria, while AC16 covers in-progress and done specs and the spec holds nineteen (§decision-resolution: 067's widening left the plan describing the prior state) — `specs/045-decision-state-drift-detection/plan.md` — **fixed** (decided by the rule)
