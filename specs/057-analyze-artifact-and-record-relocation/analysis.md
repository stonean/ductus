---
spec: 057-analyze-artifact-and-record-relocation
last-run: 2026-11-16T14:01:05Z
analyzed-against: 3ed84fe1748b1968e4738c76f70af9872bf72f14
hard-fail: 0
blocking-findings: 1
advisory: 4
unexamined: 5
analyzed-digest:
  data-model.md: 22d5694ca1d392b9a0096e8ac6db5a88f785467a3234557c1859c9e99c5df050
  plan.md: e76c56e10ac55fa3f2b75f1386eb4de52dbe377cd7e32fc54716a8835e6fd9a3
  review.md: 160cda4637a41288c50df4efba8d17297cad6a450d53c698da2f9f5b2fc4f3d7
  spec.md: 54d95f190dd90199987ca4fe114a8275409cac4fd8675affb72de89295af9e81
  tasks.md: 5ba7ae3f4aac7b32c5aacfc6dd642c2c6c5a44f2d261ce2d17c1f871939be100
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
  BE-ERR-004: {verdict: addressed, cite: "data-model.md#Errors"}
  BE-INPUT-001: {verdict: addressed, cite: "data-model.md#`relocate-audit-records` result"}
  BE-INPUT-004: {verdict: addressed, cite: "data-model.md#`relocate-audit-records` result"}
  BE-LOCK-001: {verdict: addressed, cite: "plan.md#The corpus sweep is a primitive, invoked per spec"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "data-model.md#`relocate-audit-records` result"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#The corpus sweep is a primitive, invoked per spec"}
  BE-TXN-002: {verdict: finding, tier: must, at: "plan.md#Why the runtime work is sequenced additively", message: "process-waivers and invalidate-review change file to read-modify-write review.md, and write-review/write-analysis rewrite files they read, but the plan names a lost-update guard only for relocate-audit-records"}
  BE-TXN-004: {verdict: addressed, cite: "data-model.md#Errors"}
blocking: true
dispositions:
  fixed: 2
  deferred: 0
  waived: 4
  undispositioned: 1
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
  - key: grounding — The plan describes a probe against spec 047 and says three raw parsers own `waivers` with serde(flatten), without citing the probe's command or naming the three parsers
    outcome: waived
    reason: The plan cites process_waivers.rs:65 and invalidate_review.rs:94 for two of the parsers and schema/primitives.rs for the Value constraint, so the claim is mostly grounded; the probe was a one-off measurement behind a design decision, and no criterion rests on it.
    decided-at: 2026-11-16T14:01:05Z
    decided-by: rule
  - key: task-consistency — tasks.md's introduction still describes tasks 1-16 ('Tasks 2-8 are the runtime; 9-12 the migration...') though only task 27 remains; the prose references pruned tasks
    outcome: waived
    reason: tasks.md is ephemeral (the constitution's tasks phase), and its spent-task prose is what /{project}:prune's reset removes; no gate reads it and it adds no requirement.
    decided-at: 2026-11-16T14:01:05Z
    decided-by: rule
---

# Analysis — 057-analyze-artifact-and-record-relocation

## Summary

0 hard-fail, 1 blocking, 4 advisory; blocking. 5 unexamined target(s). Dispositions: 2 fixed, 0 deferred, 4 waived, 1 undispositioned. An undispositioned advisory finding does not hold `done`; a hard-fail or blocking one holds it until the re-check no longer produces it.

## Hard failures

*None.*

## Blocking findings

- rule-assessment — BE-TXN-002: process-waivers and invalidate-review change file to read-modify-write review.md, and write-review/write-analysis rewrite files they read, but the plan names a lost-update guard only for relocate-audit-records — `specs/057-analyze-artifact-and-record-relocation/plan.md` — **undispositioned**

## Advisory findings

- rule-assessment — BE-DEPS-007: The plan changes CI configuration (framework/templates/ci/adopter-generators.yml) and Cargo.toml/Cargo.lock but commits to no automated vulnerability scan on dependency-set changes. — `specs/057-analyze-artifact-and-record-relocation/plan.md` — **waived**: 057's dependency change is a version bump in runtime/Cargo.toml and Cargo.lock, and the CI template change adds no dependency; the scan the rule asks for already runs on every change to the runtime's manifest and lockfile (runtime.yml's audit job runs cargo audit on any push touching runtime/**), and where it runs is the project's choice under the rule's Not-flagged clause. (decided by the rule)
- grounding — Motivation states corpus measurements (54 spec directories, average 37.5 frontmatter lines, 231 for 022, every spec carrying a review.md) as fact with no command, query or method — `specs/057-analyze-artifact-and-record-relocation/spec.md` — **waived**: A dated historical measurement ('at the time of writing') whose subject, the pre-relocation frontmatter, no longer exists, so no command could reproduce it now; its denominator and unit are stated, and it motivates the change without gating any criterion. (decided by the rule)
- grounding — The plan describes a probe against spec 047 and says three raw parsers own `waivers` with serde(flatten), without citing the probe's command or naming the three parsers — `specs/057-analyze-artifact-and-record-relocation/plan.md` — **waived**: The plan cites process_waivers.rs:65 and invalidate_review.rs:94 for two of the parsers and schema/primitives.rs for the Value constraint, so the claim is mostly grounded; the probe was a one-off measurement behind a design decision, and no criterion rests on it. (decided by the rule)
- task-consistency — tasks.md's introduction still describes tasks 1-16 ('Tasks 2-8 are the runtime; 9-12 the migration...') though only task 27 remains; the prose references pruned tasks — `specs/057-analyze-artifact-and-record-relocation/tasks.md` — **waived**: tasks.md is ephemeral (the constitution's tasks phase), and its spent-task prose is what /{project}:prune's reset removes; no gate reads it and it adds no requirement. (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 2
- root-absent: 3

## Fixed in this run

- rule-assessment — BE-PAYLOAD-001: The result lists `relocated` and `disagreements` (and RelocationIncomplete's) state no maximum; bound is only implicit in the two record schemas. — `specs/057-analyze-artifact-and-record-relocation/data-model.md` — **fixed**
- rule-assessment — BE-INPUT-004: relocate-audit-records takes a spec path and writes review.md, analysis.md and spec.md under it, and no artifact commits that path to canonicalization and containment or points at the project's containment mechanism — `specs/057-analyze-artifact-and-record-relocation/plan.md` — **fixed**
