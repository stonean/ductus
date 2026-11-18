---
spec: 057-analyze-artifact-and-record-relocation
last-run: 2026-11-18T13:03:12Z
analyzed-against: c8817abe9439c9bc03aea111c0057d132f4a1240
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 5
analyzed-digest:
  data-model.md: 077af403c5bab7c157dd8d52049bc142fff03aad7be68ab0becbee2b5a946ed1
  plan.md: ad5c8600ebf2c8f10f0493970331c03356b77b11d7d98f61fbbca8bc776ef3ec
  review.md: 160cda4637a41288c50df4efba8d17297cad6a450d53c698da2f9f5b2fc4f3d7
  spec.md: 490f682a3fe0140bc89bba0587687921784426e5b568e01eeb66a03b24ba0c9f
  tasks.md: 1b5f609fa6804de99453ebf8e993b6ceec72e59fd76c25227f8f3d12d7133b7d
unexamined-by-reason:
  not-a-live-claim: 2
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
  security-backend.md: 9873cc885c4bdc76eff79a7269a5a6858a0b84c3010ff4b1bc27e7989f708183
  security-cross.md: bf70990f24e7e67926772be0d95deb7e1c1492bc12bdcb650a23b25385bcd285
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#Trade-offs"}
  BE-DEPS-007: {verdict: finding, tier: should, at: "plan.md#Affected Files", message: "The plan changes CI configuration (framework/templates/ci/adopter-generators.yml) and Cargo.toml/Cargo.lock but commits to no automated vulnerability scan on dependency-set changes."}
  BE-ERR-002: {verdict: addressed, cite: "data-model.md#Errors"}
  BE-INPUT-004: {verdict: addressed, cite: "data-model.md#`relocate-audit-records` result"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "data-model.md#`relocate-audit-records` result"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#The corpus sweep is a primitive, invoked per spec"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#The corpus sweep is a primitive, invoked per spec"}
  BE-TXN-004: {verdict: addressed, cite: "plan.md#The corpus sweep is a primitive, invoked per spec"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "spec.md#Applicable Rules"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 3
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-DEPS-007: The plan changes CI configuration (framework/templates/ci/adopter-generators.yml) and Cargo.toml/Cargo.lock but commits to no automated vulnerability scan on dependency-set changes."
    outcome: waived
    reason: 057's dependency change is a version bump in runtime/Cargo.toml and Cargo.lock, and the CI template change adds no dependency; the scan the rule asks for already runs on every change to the runtime's manifest and lockfile (runtime.yml's audit job runs cargo audit on any push touching runtime/**), and where it runs is the project's choice under the rule's Not-flagged clause.
    decided-at: 2026-11-16T14:01:05Z
    decided-by: rule
  - key: grounding — Motivation states corpus measurements (54 spec directories, average 37.5 frontmatter lines, 231 for 022, every spec carrying a review.md) as fact with no command, query or method
    outcome: waived
    reason: A dated historical measurement ('at the time of writing') whose subject, the pre-relocation frontmatter, no longer exists, so no command could reproduce it now; its denominator and unit are stated, and it motivates the change without gating any criterion.
    decided-at: 2026-11-16T14:01:05Z
    decided-by: rule
  - key: grounding — spec.md Behavior states 'Twelve documents describe the blocks today' and the shipped CI template's no-review-block skip as existing-system fact without a path citation
    outcome: waived
    reason: the Behavior states the state this spec changed when it was written, twelve documents and the CI template's skip that 057 itself replaced, as the plan's CI decision records; it is the spec's account of its starting point, not a claim about the tree today
    decided-at: 2026-11-18T13:03:12Z
    decided-by: rule
---

# Analysis — 057-analyze-artifact-and-record-relocation

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 5 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 3 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-DEPS-007: The plan changes CI configuration (framework/templates/ci/adopter-generators.yml) and Cargo.toml/Cargo.lock but commits to no automated vulnerability scan on dependency-set changes. — `specs/057-analyze-artifact-and-record-relocation/plan.md` — **waived**: 057's dependency change is a version bump in runtime/Cargo.toml and Cargo.lock, and the CI template change adds no dependency; the scan the rule asks for already runs on every change to the runtime's manifest and lockfile (runtime.yml's audit job runs cargo audit on any push touching runtime/**), and where it runs is the project's choice under the rule's Not-flagged clause. (decided by the rule)
- grounding — spec.md Motivation states measured figures (54 spec directories, 37.5 average and 231 lines of frontmatter) with no command or query naming the method — `specs/057-analyze-artifact-and-record-relocation/spec.md` — **waived**: A dated historical measurement ('at the time of writing') whose subject, the pre-relocation frontmatter, no longer exists, so no command could reproduce it now; its denominator and unit are stated, and it motivates the change without gating any criterion. (decided by the rule)
- grounding — spec.md Behavior states 'Twelve documents describe the blocks today' and the shipped CI template's no-review-block skip as existing-system fact without a path citation — `specs/057-analyze-artifact-and-record-relocation/spec.md` — **waived**: the Behavior states the state this spec changed when it was written, twelve documents and the CI template's skip that 057 itself replaced, as the plan's CI decision records; it is the spec's account of its starting point, not a claim about the tree today (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 2
- root-absent: 3

## Fixed in this run

*None.*
