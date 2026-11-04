---
spec: 078-review-fix-rounds-are-re-read-within-a-bound
last-run: 2026-11-04T14:34:33Z
analyzed-against: f47d2b0326adac7a3c6e0e238bb54c5938dd037f
hard-fail: 0
blocking-findings: 2
advisory: 4
unexamined: 0
analyzed-digest:
  data-model.md: f855990873b205ddbd5d52713e7ff2c6af6ac6df3e5cc48e1c1f8f220ea40457
  plan.md: 6b7bae54107f3a167caed070f44f4bd65f29cc05bbdecfcd3c781f3a1cddaa20
  review.md: 78e56eaa0d889a62bbe934b8d97d619c88f839b8c3ee7a1a3b7e8d06d79f5fda
  scenarios/a-reviews-own-record-is-no-fix-round.md: f1aeb3c3fd26d0e17a98ef6bb433babcabe2abb53a3914e2de4f2ddf33be784d
  scenarios/fix-round-results-have-a-bounded-size.md: bd6472ece8d3586d1de656fd1d1689848d455ed79183249ff003e9db24817de7
  spec.md: 1dcee6113dadae81491d194880174d5c44025149d99ea9f7ea9a9e41651b260e
  tasks.md: d8d9523270c751cd6f043e80bcc54699a46a7721a1abb943985e029e7a033cd1
blocking: true
dispositions:
  fixed: 12
  deferred: 2
  waived: 4
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-PAYLOAD-001: brief-re-read's paths, read-re-read's findings and check-prescriptions' differing, unprescribed and failed lists are returned inline with no stated maximum size"
    outcome: deferred
    target: specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md
    task: "10"
    gating: true
    decided-at: 2026-11-04T14:34:33Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — CFG-CONST-001: the re-read budget of two is applied by the host's loop and by write-review's budget-stop derivation, and no artifact names one location for it"
    outcome: deferred
    target: specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md
    task: "14"
    gating: true
    decided-at: 2026-11-04T14:34:33Z
    decided-by: andy@stone.dev
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
---

# Analysis — 078-review-fix-rounds-are-re-read-within-a-bound

## Summary

0 hard-fail, 2 blocking, 4 advisory; blocking. 0 unexamined target(s). Dispositions: 12 fixed, 2 deferred, 4 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

- rule-assessment — BE-PAYLOAD-001: brief-re-read's paths, read-re-read's findings and check-prescriptions' differing, unprescribed and failed lists are returned inline with no stated maximum size — `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md` — **deferred** to `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md` task 10
- rule-assessment — CFG-CONST-001: the re-read budget of two is applied by the host's loop and by write-review's budget-stop derivation, and no artifact names one location for it — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — **deferred** to `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md` task 14

## Advisory findings

- rule-assessment — BE-REPR-002: result.json's and read-re-read's severities are upper-case MUST, SHOULD and NOTE while 078's other enum values are lower-case kebab — `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md` — **waived**: The severities are the RFC 2119 keywords the rule files and every brief's severity bar spell in capitals, and a re-read writes them as it reads them; this project declares no casing convention (it has no specs/system.md).
- grounding — the Motivation's account of 076's avoidable rounds cites no commit or record — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **waived**: History that motivates the spec and governs no behavior; 076's review commits keep it.
- grounding — the rejected alternative's cost, that diff_since without the index hashes every file on every review, cites nothing — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **waived**: A decision record: it states the cost as the owner weighed it on 2026-11-03.
- grounding — the overlap Family 7 sees is listed without the run or commit that produced it — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **waived**: The split rationale's example; every audit run re-derives the overlap.

## Deferred to other specs

*None.*

## Unexamined targets

*None — every target was examined.*

## Fixed in this run

- rule-assessment — BE-INPUT-004: no artifact commits the feature and base arguments to a lexical screen, canonical resolution and whole-component containment before the round directory is opened — `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md` — **fixed**
- rule-assessment — BE-COMPAT-001: 078 changes write-review's arguments and check-review-gate's blocked-by, and neither the spec nor the plan classifies the change as additive or breaking — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — **fixed**
- rule-assessment — BE-RACE-003: the rounds directory's .gitignore is created if absent, and no artifact names an atomic mechanism or states that repeating the write is harmless — `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md` — **fixed**
- rule-assessment — CFG-ENV-005: the re-reads-off halt is specified as naming the setting, the value and the file, not the violated 16-character minimum — `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md` — **fixed**
- grounding — the Motivation's 933k subagent tokens and 34 minutes of agent time, repeated in a Resolved Question, cite no source and no method — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **fixed**
- grounding — the claim that comparing as git stores both makes line-ending conversion compare alike cites nothing — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **fixed**
- grounding — the claim that libgit2's add_all applies the working tree's deletions itself cites nothing — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — **fixed**
- grounding — the claim that git's default gc.pruneExpire is two weeks cites no documentation — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — **fixed**
- grounding — 066's definition of an observation is named without its path or section — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — **fixed**
- grounding — the claim that write-review resolves the constitution registry itself cites no symbol — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — **fixed**
- grounding — the exec walker's per-step dispatch and its no-op of step 10 are stated without a source — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — **fixed**
- grounding — the known limitation that framework/reference/review.md is over the read size states no measured figure or method — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — **fixed**
