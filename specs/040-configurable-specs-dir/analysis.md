---
spec: 040-configurable-specs-dir
last-run: 2026-11-18T13:01:04Z
analyzed-against: 0d0940a73247e9a002b29058b1ff127072a5a60b
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 5
analyzed-digest:
  plan.md: 6a6dcecc10571e2fba06295c0f2f79fc9d14b482d1e4506ad27dee347f22f854
  review.md: 5280606253fec7d7067877b077f4f49edc5e8c350e59a1bfb1ed0baea958d0aa
  scenarios/command-prose-resolves-spec-root.md: c5c186bdacde6d69fc244feddcf02c05c25b490791cfee1992d252c8d167b7d7
  scenarios/spec-root-rule-stated-once.md: 5052a667c592db856f7f22678dce99ba21fafb172cfc8968e81ccf0eb7a46507
  spec.md: 80d01cbe3b774c1a9280a6e77673889c7cd27b67c89cfd54d8ed2b3d00cde960
  tasks.md: 8808a07fd01dbdeac14d13afe063ac93e8e1c2d919fc3d2d6403de6a0800a9fe
unexamined-by-reason:
  not-a-live-claim: 2
  root-absent: 1
  ships-to-adopter: 2
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
  security-backend.md: 9873cc885c4bdc76eff79a7269a5a6858a0b84c3010ff4b1bc27e7989f708183
  security-cross.md: bf70990f24e7e67926772be0d95deb7e1c1492bc12bdcb650a23b25385bcd285
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#Config schema: `[paths] specs-root`, default `specs`"}
  BE-INPUT-002: {verdict: addressed, cite: "spec.md#Setting"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Config schema: `[paths] specs-root`, default `specs`"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#Config schema: `[paths] specs-root`, default `specs`"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#Config schema: `[paths] specs-root`, default `specs`"}
  CFG-ENV-001: {verdict: addressed, cite: "plan.md#Config schema: `[paths] specs-root`, default `specs`"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  CFG-ENV-007: {verdict: addressed, cite: "plan.md#Config schema: `[paths] specs-root`, default `specs`"}
  CFG-ENV-008: {verdict: addressed, cite: "plan.md#Config schema: `[paths] specs-root`, default `specs`"}
  CFG-ENV-012: {verdict: addressed, cite: "plan.md#Config schema: `[paths] specs-root`, default `specs`"}
blocking: false
dispositions:
  fixed: 6
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: grounding — spec.md Motivation says the literal specs/ appears across roughly three dozen framework markdown files, with no command or file citation
    outcome: waived
    reason: a count taken when the spec was written, to size the sweep the feature required; the corpus has moved since, so re-deriving its method would count a different set and is no mechanical fix
    decided-at: 2026-11-18T13:01:04Z
    decided-by: rule
---

# Analysis — 040-configurable-specs-dir

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 5 unexamined target(s). Dispositions: 6 fixed, 0 deferred, 1 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — spec.md Motivation says the literal specs/ appears across roughly three dozen framework markdown files, with no command or file citation — `specs/040-configurable-specs-dir/spec.md` — **waived**: a count taken when the spec was written, to size the sweep the feature required; the corpus has moved since, so re-deriving its method would count a different set and is no mechanical fix (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 2
- root-absent: 1
- ships-to-adopter: 2

## Fixed in this run

- rule-assessment — BE-INPUT-004: Operator-set specs-root becomes a filesystem path but only a charset allowlist is committed; no canonicalization plus whole-component containment check (symlink escape) is stated in spec or plan. — `specs/040-configurable-specs-dir/spec.md` — **fixed**
- rule-assessment — CFG-ENV-001: Optional specs-root declares default literal `specs` but names no default constant (CFG-ENV-001's named-constant form). — `specs/040-configurable-specs-dir/plan.md` — **fixed**
- rule-assessment — CFG-ENV-008: Runtime resolver falls back to `specs` on empty and has no fail-fast parse of a malformed value; validation only when /ductus writes it, while spec calls empty malformed. — `specs/040-configurable-specs-dir/plan.md` — **fixed**
- rule-assessment — CFG-ENV-012: New [paths] section states no loader treatment (fail or warn by name) for an unrecognized key. — `specs/040-configurable-specs-dir/plan.md` — **fixed**
- grounding — spec.md says the adopter pre-commit hook now matches the spec root by shape, with no file or symbol cited — `specs/040-configurable-specs-dir/spec.md` — **fixed**
- grounding — plan.md says the runtime config/schema layer already parses [services] and [rules], with no file or symbol cited — `specs/040-configurable-specs-dir/plan.md` — **fixed**
