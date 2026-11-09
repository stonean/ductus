---
spec: 078-review-fix-rounds-are-re-read-within-a-bound
last-run: 2026-11-09T21:41:13Z
analyzed-against: aa5e3096156aa2a0fc41338e2ba67240dd959b2a
hard-fail: 0
blocking-findings: 0
advisory: 5
unexamined: 2
analyzed-digest:
  data-model.md: 95cd98fd2db64be4ac315c056ac68c8e70b480723cc124a4db436ca5b570cf78
  plan.md: 9836d6779b53a075eae043e1327d1db8de9d3aa88cf0f64fdcaec5516c7777af
  review.md: 3330a8dae59991d2ea8e19895300447af70f73ecff42d631c7c83944fe77946b
  scenarios/a-reviews-own-record-is-no-fix-round.md: 8a0190ef5a926425199a4381d558f52db626cdd81803b4e23c9aa1055b130440
  scenarios/fix-round-results-have-a-bounded-size.md: bd6472ece8d3586d1de656fd1d1689848d455ed79183249ff003e9db24817de7
  spec.md: c48110d371473584e66dd08a295650f35e66c8c071a008ba5df0c3600facf2e3
  tasks.md: b961250dcbf08c4186c5f3878b05c7986d8b55cf5b415514e415fd9b0339c48f
blocking: false
dispositions:
  fixed: 0
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

0 hard-fail, 0 blocking, 5 advisory; not blocking. 2 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 5 waived, 0 undispositioned.

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

## Unexamined targets

2 target(s) unexamined; no breakdown recorded.

## Fixed in this run

*None.*
