---
spec: 045-decision-state-drift-detection
last-run: 2026-10-16T19:38:46Z
analyzed-against: 2ce89f9d91c5ef1d03f1294506ade742e1aa8ef3
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 17
analyzed-digest:
  data-model.md: d649e14cc23a5a2b7ed881962c878afd00a9050637618e0896bdea61306d2e61
  plan.md: c73224f28a1afa2006f4ae3f91683e929af5eb156a35a94d45593b3884bbca3e
  review.md: d269d71dd0769ced1e9f624ea82c4f8aa8c9fd8a7e7c61c32cef35071c41bc5e
  spec.md: 17a8fe5b38d47ab2c9a972f1c72d1ce66454eb27ee2fdb27e068bbc36c9944fa
  tasks.md: 8b46078bdad8d7f1d9d0823451246d232c0be79926c3411bc6b9223d3ce8d8bf
unexamined-by-reason:
  not-a-live-claim: 8
  root-absent: 4
  ships-to-adopter: 5
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 3
  undispositioned: 0
decisions:
  - key: "body-path-existence — names `../spec.md`, which existed in this repository and no longer resolves: \"For each inline `[text](target)` in a scanned artifact, resolve `target` relative to the *containing file's* directory and keep it only when the result lands inside the feature directory. This is what makes `../spec.md` from `scenarios/foo.md` a sibling while `../022-deterministic-runtime/spec.md` from `spec.md` is not — the distinction the spec's §Scope depends on, and the one the Motivation measurement relied on (21 cross-feature links in `## Motivation` sections, all out of scope).\""
    outcome: discarded
    reason: "Correct text: the example states its own base (`from scenarios/foo.md`), where `../spec.md` is a sibling; the check resolves it from plan.md instead, which it cannot know not to do. Operator-approved discard (2026-10-16)."
    decided-at: 2026-10-16T19:38:46Z
    decided-by: andy@stone.dev
  - key: "body-path-existence — names `framework/workflows/registry.json`, which existed in this repository and no longer resolves: \"| `framework/workflows/registry.json` | yes | no → finding |\""
    outcome: discarded
    reason: "Correct text: a worked-example table row whose lead-in says both subjects were deleted; the row shows the check's output on them. Operator-approved discard (2026-10-16)."
    decided-at: 2026-10-16T19:38:46Z
    decided-by: andy@stone.dev
  - key: "body-path-existence — names `scripts/audit/registry-equivalence.sh`, which existed in this repository and no longer resolves: \"| `scripts/audit/registry-equivalence.sh` | yes | no → finding |\""
    outcome: discarded
    reason: "Correct text: a worked-example table row whose lead-in says both subjects were deleted; the row shows the check's output on them. Operator-approved discard (2026-10-16)."
    decided-at: 2026-10-16T19:38:46Z
    decided-by: andy@stone.dev
---

# Analysis — 045-decision-state-drift-detection

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 17 unexamined target(s). Dispositions: 0 fixed, 0 routed, 3 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- body-path-existence — names `../spec.md`, which existed in this repository and no longer resolves: "For each inline `[text](target)` in a scanned artifact, resolve `target` relative to the *containing file's* directory and keep it only when the result lands inside the feature directory. This is what makes `../spec.md` from `scenarios/foo.md` a sibling while `../022-deterministic-runtime/spec.md` from `spec.md` is not — the distinction the spec's §Scope depends on, and the one the Motivation measurement relied on (21 cross-feature links in `## Motivation` sections, all out of scope)." — `specs/045-decision-state-drift-detection/plan.md` — **discarded**: Correct text: the example states its own base (`from scenarios/foo.md`), where `../spec.md` is a sibling; the check resolves it from plan.md instead, which it cannot know not to do. Operator-approved discard (2026-10-16).
- body-path-existence — names `framework/workflows/registry.json`, which existed in this repository and no longer resolves: "| `framework/workflows/registry.json` | yes | no → finding |" — `specs/045-decision-state-drift-detection/data-model.md` — **discarded**: Correct text: a worked-example table row whose lead-in says both subjects were deleted; the row shows the check's output on them. Operator-approved discard (2026-10-16).
- body-path-existence — names `scripts/audit/registry-equivalence.sh`, which existed in this repository and no longer resolves: "| `scripts/audit/registry-equivalence.sh` | yes | no → finding |" — `specs/045-decision-state-drift-detection/data-model.md` — **discarded**: Correct text: a worked-example table row whose lead-in says both subjects were deleted; the row shows the check's output on them. Operator-approved discard (2026-10-16).

## Unexamined targets

- not-a-live-claim: 8
- root-absent: 4
- ships-to-adopter: 5

## Fixed in this run

*None.*
