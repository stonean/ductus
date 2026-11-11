---
spec: 045-decision-state-drift-detection
last-run: 2026-11-11T17:22:42Z
analyzed-against: aedb476958508e84d243dfa55d91e37e58bd090b
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 17
analyzed-digest:
  data-model.md: e57856a8be48ada9fa99c2638b68e72a39f82c48352955c274764dd58a6a19ec
  plan.md: 3a3bf04801c6b583f3760b4cc4e215bfef112c1d522b603cc5f11308c634f5e2
  review.md: 81930205c4a39cb4d673aeeb283d6b60b62e42ca1adf83f88010240f8ecba2a6
  spec.md: b2816eebfb6451fdb1960a8b73e55e72dd082475afa8a0cac2b81402b2a2529a
  tasks.md: 8b46078bdad8d7f1d9d0823451246d232c0be79926c3411bc6b9223d3ce8d8bf
unexamined-by-reason:
  not-a-live-claim: 8
  root-absent: 4
  ships-to-adopter: 5
blocking: false
dispositions:
  fixed: 2
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

0 hard-fail, 0 blocking, 3 advisory; not blocking. 17 unexamined target(s). Dispositions: 2 fixed, 1 deferred, 3 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- body-path-existence — names `../spec.md`, which existed in this repository and no longer resolves: "For each inline `[text](target)` in a scanned artifact, resolve `target` relative to the *containing file's* directory and keep it only when the result lands inside the feature directory. This is what makes `../spec.md` from `scenarios/foo.md` a sibling while `../022-deterministic-runtime/spec.md` from `spec.md` is not — the distinction the spec's §Scope depends on, and the one the Motivation measurement relied on (21 cross-feature links in `## Motivation` sections, all out of scope)." — `specs/045-decision-state-drift-detection/plan.md` — **waived**: correct as written: `../spec.md` is the worked example of a link resolved from the hypothetical `scenarios/foo.md`, not a path the plan claims exists; the family resolves it from plan.md's directory instead (decided by the rule)
- body-path-existence — names `framework/workflows/registry.json`, which existed in this repository and no longer resolves: "| `framework/workflows/registry.json` | yes | no → finding |" — `specs/045-decision-state-drift-detection/data-model.md` — **waived**: correct as written: the row is the worked example's expected outcome, the path deleted by 531e3ea and no longer resolving; the deletion is stated on the example's lead-in line, which the per-line exemption does not reach (decided by the rule)
- body-path-existence — names `scripts/audit/registry-equivalence.sh`, which existed in this repository and no longer resolves: "| `scripts/audit/registry-equivalence.sh` | yes | no → finding |" — `specs/045-decision-state-drift-detection/data-model.md` — **waived**: correct as written: the row is the worked example's expected outcome, the script deleted by 3ff65445 and no longer resolving; the deletion is stated on the example's lead-in line, which the per-line exemption does not reach (decided by the rule)

## Deferred to other specs

- rule-assessment — BE-INPUT-004: the criterion-path family probes a path taken from spec text with repo.join(candidate).exists() and no `..` screen, so a criterion can make the runtime stat a file outside the project — `specs/045-decision-state-drift-detection/data-model.md` — **deferred** to `specs/067-analyze-and-audit-checks/tasks.md` task 27

## Unexamined targets

- not-a-live-claim: 8
- root-absent: 4
- ships-to-adopter: 5

## Fixed in this run

- rule-assessment — BE-INPUT-004: the families read files under the directory the feature argument names, and no 045 artifact stated the screen check-artifacts applies to that argument (validate_feature_contained) — `specs/045-decision-state-drift-detection/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-PAYLOAD-001: check-artifacts' findings and skipped lists grow with the feature, and no 045 artifact committed to their bound or pointed at 080's — `specs/045-decision-state-drift-detection/plan.md` — **fixed** (decided by the rule)
