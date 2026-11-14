---
spec: 078-review-fix-rounds-are-re-read-within-a-bound
last-run: 2026-11-14T13:44:48Z
analyzed-against: 87b2240447b238b2c86a7ebfd964e9d0c44bb9ec
hard-fail: 0
blocking-findings: 0
advisory: 5
unexamined: 2
analyzed-digest:
  data-model.md: 1fc4eae1fc031a011a4aa2346152a20aed0ed867454993c3a31dd1ba8846149d
  plan.md: 84f5c48d9862a8c81749a229413e638cd59a1cda32d8cad312ff7d50165fe500
  review.md: 3330a8dae59991d2ea8e19895300447af70f73ecff42d631c7c83944fe77946b
  scenarios/a-reviews-own-record-is-no-fix-round.md: 8a0190ef5a926425199a4381d558f52db626cdd81803b4e23c9aa1055b130440
  scenarios/fix-round-results-have-a-bounded-size.md: 3cf9dc7ebb4213d371eadee7db64826448d2da4ed0bf3ffa96b4fecb897f78d9
  spec.md: 4a08b1d2f1911af781ff23182c4e3eae72a37d48a201e796502c809d12eb249f
  tasks.md: 046ea8e5f19a0ed15e64dc250cc4cd707dfff4e011214ec271ad99e16c36553f
unexamined-by-reason:
  never-existed: 1
  not-a-live-claim: 1
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
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#The record and the gate are 066's"}
  BE-INPUT-001: {verdict: addressed, cite: "data-model.md#Result file"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#Round directory"}
  BE-INPUT-003: {verdict: addressed, cite: "data-model.md#`brief-re-read`"}
  BE-INPUT-004: {verdict: addressed, cite: "data-model.md#Round directory"}
  BE-INPUT-008: {verdict: addressed, cite: "data-model.md#Result file"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "scenarios/fix-round-results-have-a-bounded-size.md#Behavior"}
  BE-RACE-003: {verdict: addressed, cite: "data-model.md#Round directory"}
  BE-REPR-002: {verdict: finding, tier: should, at: "data-model.md#Result file", message: "result.json's and read-re-read's severities are upper-case MUST, SHOULD and NOTE while 078's other enum values are lower-case kebab"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#The loop's end and its budget"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#The runtime writes the brief"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#The loop's end and its budget"}
  CFG-CONST-004: {verdict: addressed, cite: "plan.md#The switch is read by the runtime"}
  CFG-ENV-001: {verdict: addressed, cite: "data-model.md#`[review] re-reads-off`"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  CFG-ENV-005: {verdict: addressed, cite: "data-model.md#`[review] re-reads-off`"}
  CFG-ENV-008: {verdict: addressed, cite: "data-model.md#`[review] re-reads-off`"}
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 5
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-REPR-002: result.json's and read-re-read's severities are upper-case MUST, SHOULD and NOTE while 078's other enum values are lower-case kebab"
    outcome: waived
    reason: The severities are the RFC 2119 keywords the rule files and every brief's severity bar spell in capitals, and a re-read writes them as it reads them; this project declares no casing convention (it has no specs/system.md).
    decided-at: 2026-11-04T14:34:33Z
    decided-by: andy@stone.dev
  - key: grounding — the Motivation's account of 076's avoidable rounds cites no commit or record
    outcome: waived
    reason: History that motivates the spec and governs no behavior; 076's review commits keep it.
    decided-at: 2026-11-04T14:34:33Z
    decided-by: andy@stone.dev
  - key: grounding — the rejected alternative's cost, that diff_since without the index hashes every file on every review, cites nothing
    outcome: waived
    reason: "A decision record: it states the cost as the owner weighed it on 2026-11-03."
    decided-at: 2026-11-04T14:34:33Z
    decided-by: andy@stone.dev
  - key: grounding — the overlap Family 7 sees is listed without the run or commit that produced it
    outcome: waived
    reason: The split rationale's example; every audit run re-derives the overlap.
    decided-at: 2026-11-04T14:34:33Z
    decided-by: andy@stone.dev
  - key: grounding — the rejected alternative's reason, that compute-review-scope writes nothing outside .ductus/results/, cites nothing
    outcome: waived
    reason: A rejected alternative's reason in a decision record (owner, 2026-11-03), the shape of the stored cost waiver beside it; its source is the primitive's compiled description, which no 078 artifact names and this command does not read.
    decided-at: 2026-11-09T19:34:21Z
    decided-by: rule
---

# Analysis — 078-review-fix-rounds-are-re-read-within-a-bound

## Summary

0 hard-fail, 0 blocking, 5 advisory; not blocking. 2 unexamined target(s). Dispositions: 1 fixed, 0 deferred, 5 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-REPR-002: result.json's and read-re-read's severities are upper-case MUST, SHOULD and NOTE while 078's other enum values are lower-case kebab — `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md` — **waived**: The severities are the RFC 2119 keywords the rule files and every brief's severity bar spell in capitals, and a re-read writes them as it reads them; this project declares no casing convention (it has no specs/system.md).
- grounding — the Motivation's account of 076's avoidable rounds cites no commit or record — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **waived**: History that motivates the spec and governs no behavior; 076's review commits keep it.
- grounding — the rejected alternative's cost, that diff_since without the index hashes every file on every review, cites nothing — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **waived**: A decision record: it states the cost as the owner weighed it on 2026-11-03.
- grounding — the overlap Family 7 sees is listed without the run or commit that produced it — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **waived**: The split rationale's example; every audit run re-derives the overlap.
- grounding — the rejected alternative's reason, that compute-review-scope writes nothing outside .ductus/results/, cites nothing — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **waived**: A rejected alternative's reason in a decision record (owner, 2026-11-03), the shape of the stored cost waiver beside it; its source is the primitive's compiled description, which no 078 artifact names and this command does not read.

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- never-existed: 1
- not-a-live-claim: 1

## Fixed in this run

- task-consistency — tasks.md task 15 still carries handoff prose 079's completed and pruned task 28 made stale: its last sentence says 'before `done` 079's task 28 is corrected to the counts a simulated closing then reads' — `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md` — **fixed** (decided by the rule)
