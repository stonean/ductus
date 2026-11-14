---
spec: 030-cross-service-references
last-run: 2026-11-14T02:54:53Z
analyzed-against: 25e4234ecf44aa3c64aa819466f964807140a3db
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 7
analyzed-digest:
  data-model.md: 170d5eed7d17cfb3b2b188ec149f04b5c3c75b1a46dd550fd9e74f1168d39d8f
  plan.md: 40db1695c0b529cfd049319fc4fbc21877d35bc3141360ec61760cc358d7773c
  review.md: 5a365b19b48e569e558cb4dd841ac00a91915edf11044b2a68f87bdd5d929012
  scenarios/referenced-service-spec-root.md: c76e9244f75881073a432932610036a6f425830df413a593c1e50c65c1b7dd13
  spec.md: 98e4038813b2dc408f3a9f2385ad10859b02be07eecd7e6b071d05fd367ac6f2
  tasks.md: a6646cdd3f86e0ab7e5599dfd8f7cc0a05dd14305fc7b1ea4e652bd4da7627e2
unexamined-by-reason:
  not-a-live-claim: 4
  root-absent: 3
rule-digest:
  accessibility-frontend.md: 5c1e7a9b5f625f14e6a1879c145ee30e25c5f5aff04ae4c853ee589e21b1efbb
  api-backend.md: 0caddd70fd1a546d80a5e77d254a38c36d158fcad464ee2493491a23cde06ea5
  concurrency-backend.md: 15209bb99ee1c7a63cfcde08a73f6de5df857834776cea43fadb7f0f1040370c
  configuration-cross.md: 2a807ba8facdfd1c3738383e6d44a1f6f2c6f91b387eac0ca5ed90d75dc9e178
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 0522a169c04113521bad128dfef283cd7e910264bf49c4a5320f27ba625fcdee
  security-backend.md: d89d6dad3ab51586ae4e68944b0c8e9ad4d5da721560f10590de3cdecc904d3f
  security-cross.md: dc98261801405fccaf5de9eb8f8ed9b020ebc78fcb5bfb124778a5fcc287950b
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-DEPS-007: {verdict: finding, tier: should, at: "plan.md#Affected Files", message: "Plan modifies .github/workflows/generators.yml (CI configuration) but commits to no automated dependency scan on manifest or lockfile changes."}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#D1 — Registry: `.ductus/config.toml [services]`"}
  BE-INPUT-002: {verdict: addressed, cite: "scenarios/referenced-service-spec-root.md#Behavior"}
  BE-INPUT-004: {verdict: addressed, cite: "scenarios/referenced-service-spec-root.md#Behavior"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#D3 — Runtime primitive: `resolve-references`"}
  CFG-ENV-001: {verdict: addressed, cite: "data-model.md#Service registry entry"}
  CFG-ENV-002: {verdict: addressed, cite: "data-model.md#Service registry entry"}
  CFG-ENV-003: {verdict: addressed, cite: "data-model.md#Service registry entry"}
  CFG-ENV-008: {verdict: addressed, cite: "data-model.md#Service registry entry"}
  CFG-ENV-009: {verdict: addressed, cite: "data-model.md#Service registry entry"}
  CFG-ENV-012: {verdict: addressed, cite: "plan.md#D1 — Registry: `.ductus/config.toml [services]`"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-DEPS-007: Plan modifies .github/workflows/generators.yml (CI configuration) but commits to no automated dependency scan on manifest or lockfile changes."
    outcome: waived
    reason: "advisory and no chore: 030's CI change adds a derivation run step to generators.yml and the pre-commit hook, running a repo-local primitive, and changes no dependency set; cargo audit runs in runtime.yml on every runtime/** change (as 024's and 026's analyses waived the same finding)"
    decided-at: 2026-11-14T02:54:53Z
    decided-by: rule
  - key: grounding — D6 states 'the additive discipline already used for `.mcp.json` and permission merges' as fact about existing behavior, with no primitive or file citation
    outcome: waived
    reason: an aside naming precedent for D6's prescriptive write rule, which holds on its own; citing the merges it means is no mechanical chore, and no criterion depends on it
    decided-at: 2026-11-14T02:54:53Z
    decided-by: rule
---

# Analysis — 030-cross-service-references

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 7 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 2 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-DEPS-007: Plan modifies .github/workflows/generators.yml (CI configuration) but commits to no automated dependency scan on manifest or lockfile changes. — `specs/030-cross-service-references/plan.md` — **waived**: advisory and no chore: 030's CI change adds a derivation run step to generators.yml and the pre-commit hook, running a repo-local primitive, and changes no dependency set; cargo audit runs in runtime.yml on every runtime/** change (as 024's and 026's analyses waived the same finding) (decided by the rule)
- grounding — D6 states 'the additive discipline already used for `.mcp.json` and permission merges' as fact about existing behavior, with no primitive or file citation — `specs/030-cross-service-references/plan.md` — **waived**: an aside naming precedent for D6's prescriptive write rule, which holds on its own; citing the merges it means is no mechanical chore, and no criterion depends on it (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 4
- root-absent: 3

## Fixed in this run

*None.*
