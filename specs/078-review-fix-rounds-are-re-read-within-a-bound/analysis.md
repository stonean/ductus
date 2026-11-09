---
spec: 078-review-fix-rounds-are-re-read-within-a-bound
last-run: 2026-11-09T19:34:21Z
analyzed-against: 494a1f3f15b2244a8a226e0dad0ae20d159beef8
hard-fail: 0
blocking-findings: 1
advisory: 5
unexamined: 2
analyzed-digest:
  data-model.md: 2eab71db0a907e9bebb9fc7ffdbe6056a0d117d7f66c877991a7f82aafb99d54
  plan.md: ad6c1d6a69b774826a25a56c883f3313cfa75f8c679a7d50a6b96c49089f2d9a
  review.md: bf876f68329130c233294dbc95815daa2a7818e9430e9e51fe19295f593f8278
  scenarios/a-reviews-own-record-is-no-fix-round.md: 8a0190ef5a926425199a4381d558f52db626cdd81803b4e23c9aa1055b130440
  scenarios/fix-round-results-have-a-bounded-size.md: bd6472ece8d3586d1de656fd1d1689848d455ed79183249ff003e9db24817de7
  spec.md: c48110d371473584e66dd08a295650f35e66c8c071a008ba5df0c3600facf2e3
  tasks.md: 7e41aff1c658ba6d22415680cc922232b54698e193ca2421fc91d22328986eb7
blocking: true
dispositions:
  fixed: 2
  deferred: 1
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
  - key: "rule-assessment — CFG-ENV-001: re-reads-off is declared optional with no default, absence naming what it disables, but an empty value halts where that declaration counts an empty value as absent"
    outcome: deferred
    target: specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md
    task: "17"
    gating: true
    decided-at: 2026-11-09T19:34:21Z
    decided-by: andy@stone.dev
  - key: grounding — the rejected alternative's reason, that compute-review-scope writes nothing outside .ductus/results/, cites nothing
    outcome: waived
    reason: A rejected alternative's reason in a decision record (owner, 2026-11-03), the shape of the stored cost waiver beside it; its source is the primitive's compiled description, which no 078 artifact names and this command does not read.
    decided-at: 2026-11-09T19:34:21Z
    decided-by: rule
---

# Analysis — 078-review-fix-rounds-are-re-read-within-a-bound

## Summary

0 hard-fail, 1 blocking, 5 advisory; blocking. 2 unexamined target(s). Dispositions: 2 fixed, 1 deferred, 5 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

- rule-assessment — CFG-ENV-001: re-reads-off is declared optional with no default, absence naming what it disables, but an empty value halts where that declaration counts an empty value as absent — `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md` — **deferred** to `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md` task 17

## Advisory findings

- rule-assessment — BE-REPR-002: result.json's and read-re-read's severities are upper-case MUST, SHOULD and NOTE while 078's other enum values are lower-case kebab — `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md` — **waived**: The severities are the RFC 2119 keywords the rule files and every brief's severity bar spell in capitals, and a re-read writes them as it reads them; this project declares no casing convention (it has no specs/system.md).
- grounding — the Motivation's account of 076's avoidable rounds cites no commit or record — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **waived**: History that motivates the spec and governs no behavior; 076's review commits keep it.
- grounding — the rejected alternative's cost, that diff_since without the index hashes every file on every review, cites nothing — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **waived**: A decision record: it states the cost as the owner weighed it on 2026-11-03.
- grounding — the overlap Family 7 sees is listed without the run or commit that produced it — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **waived**: The split rationale's example; every audit run re-derives the overlap.
- grounding — the rejected alternative's reason, that compute-review-scope writes nothing outside .ductus/results/, cites nothing — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **waived**: A rejected alternative's reason in a decision record (owner, 2026-11-03), the shape of the stored cost waiver beside it; its source is the primitive's compiled description, which no 078 artifact names and this command does not read. (decided by the rule)

## Deferred to other specs

*None.*

## Unexamined targets

2 target(s) unexamined; no breakdown recorded.

## Fixed in this run

- grounding — the claim that ductus exec's walker no-ops the fix and loop steps cites no source — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **fixed** (decided by the rule)
- grounding — the claim that the round's git diff shows the text as git stores it cites nothing — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **fixed** (decided by the rule)
