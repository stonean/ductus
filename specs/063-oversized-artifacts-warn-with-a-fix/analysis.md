---
spec: 063-oversized-artifacts-warn-with-a-fix
last-run: 2026-11-14T00:36:06Z
analyzed-against: 9bd9b082dc17d157430f5fcf850d5aab5fe69b17
hard-fail: 0
blocking-findings: 1
advisory: 3
unexamined: 4
analyzed-digest:
  data-model.md: 61cea3c5ca5118ce321714e8c893b86ebc6a62ce5dd3fb95efe960fc52451a1b
  plan.md: 262fbfcbaa7486a1d24e2739f057c6095fd3f8cc7bdc33b28cfd7145f07e03e6
  review.md: 9b4d77a78680880207615a929551c987f797f8e35096f7d4c5c1fee561bdef67
  scenarios/a-plan-trim-does-not-reopen-a-done-spec.md: c77ea68f76643287537d7bd22ce126885e5fc1d56680d59e7970b862f1c2a6cf
  scenarios/an-invalid-threshold-fails-fast.md: 5945eec14f64769ce85570033863a2f209ed8784a7fd1f00d0e9d7b647cd9112
  scenarios/an-unknown-artifacts-key-is-named.md: 912ce563e772f628884c9096a8fafb00526379e935c99f40d7ca96090d986e1a
  scenarios/project-files-are-measured.md: cf613536df6ef606a837a844ac99964422bfe89f09e5ab79eff5702f4cecbc77
  spec.md: 0e92f3a4fa8113be8c40afa2a60e6c4a19a903fae3d867e66ca4cb3a943824a1
  tasks.md: 0c0e75eff875d5f2ffed27ff1bcda75fafb9a863ec77d01c43fefa1967807e79
unexamined-by-reason:
  root-absent: 4
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
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#The threshold: `[artifacts] read-size-bytes`"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#The threshold: `[artifacts] read-size-bytes`"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#Subjects, absence, and readability"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Subjects, absence, and readability"}
  BE-INPUT-008: {verdict: addressed, cite: "plan.md#`/analyze`: the `artifact-size` family and the decided rule"}
  BE-PAYLOAD-001: {verdict: finding, tier: must, at: "data-model.md#`CheckArtifactSizeResult`", message: "check-artifact-size's oversized, skipped and notices lists grow with a spec directory's scenario count, and neither the plan nor the data model commits to a maximum response size or a bound"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#The threshold: `[artifacts] read-size-bytes`"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#The threshold: `[artifacts] read-size-bytes`"}
  CFG-CONST-004: {verdict: addressed, cite: "plan.md#The threshold: `[artifacts] read-size-bytes`"}
  CFG-ENV-001: {verdict: addressed, cite: "plan.md#The threshold: `[artifacts] read-size-bytes`"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Documentation"}
  CFG-ENV-005: {verdict: addressed, cite: "plan.md#The threshold: `[artifacts] read-size-bytes`"}
  CFG-ENV-007: {verdict: addressed, cite: "plan.md#The threshold: `[artifacts] read-size-bytes`"}
  CFG-ENV-008: {verdict: addressed, cite: "plan.md#The threshold: `[artifacts] read-size-bytes`"}
  CFG-ENV-012: {verdict: addressed, cite: "scenarios/an-unknown-artifacts-key-is-named.md#Behavior"}
blocking: true
dispositions:
  fixed: 0
  deferred: 1
  waived: 3
  undispositioned: 0
decisions:
  - key: grounding — spec.md says an adopter's 2,200-line plan.md was never read in full, citing no source
    outcome: waived
    reason: motivation that cannot be cited without naming another project, which AGENTS.md forbids; the threshold rests on the 2026-09-28 and 2026-10-04 measurements the spec does cite
    decided-at: 2026-10-04T18:18:21Z
    decided-by: andy@stone.dev
  - key: grounding — Subjects bullet asserts that no command reads research.md and that review.md/analysis.md are regenerated whole by their write primitives, with no file-and-symbol citation at the claim
    outcome: waived
    reason: Low-risk scoping rationale; the Resolved Question on review.md and analysis.md cites framework/commands/analyze.md and check-review-gate for the readers, and the claim's truth is review's check, not this form check
    decided-at: 2026-11-14T00:36:06Z
    decided-by: rule
  - key: grounding — Cross-spec impact states spec 081 retired Family 19 and review staleness with no link or citation
    outcome: waived
    reason: Historical aside explaining why 058 owes no review; it asserts no contract this spec depends on, and adding a link would create a dependencies edge for a citation
    decided-at: 2026-11-14T00:36:06Z
    decided-by: rule
  - key: "rule-assessment — BE-PAYLOAD-001: check-artifact-size's oversized, skipped and notices lists grow with a spec directory's scenario count, and neither the plan nor the data model commits to a maximum response size or a bound"
    outcome: deferred
    target: specs/063-oversized-artifacts-warn-with-a-fix/tasks.md
    task: "17"
    reason: "the owner deferred it to 063's own task 17 on 2026-11-13: 063 owns the primitive, as 073 tasks 12-14 bounded 073's"
    gating: true
    decided-at: 2026-11-14T00:36:06Z
    decided-by: andy@stone.dev
---

# Analysis — 063-oversized-artifacts-warn-with-a-fix

## Summary

0 hard-fail, 1 blocking, 3 advisory; blocking. 4 unexamined target(s). Dispositions: 0 fixed, 1 deferred, 3 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

- rule-assessment — BE-PAYLOAD-001: check-artifact-size's oversized, skipped and notices lists grow with a spec directory's scenario count, and neither the plan nor the data model commits to a maximum response size or a bound — `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md` — **deferred** to `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md` task 17

## Advisory findings

- grounding — Subjects bullet asserts that no command reads research.md and that review.md/analysis.md are regenerated whole by their write primitives, with no file-and-symbol citation at the claim — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **waived**: Low-risk scoping rationale; the Resolved Question on review.md and analysis.md cites framework/commands/analyze.md and check-review-gate for the readers, and the claim's truth is review's check, not this form check (decided by the rule)
- grounding — Motivation cites an adopter's 2,200-line plan.md as fact with no substantiating artifact — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **waived**: motivation that cannot be cited without naming another project, which AGENTS.md forbids; the threshold rests on the 2026-09-28 and 2026-10-04 measurements the spec does cite
- grounding — Cross-spec impact states spec 081 retired Family 19 and review staleness with no link or citation — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — **waived**: Historical aside explaining why 058 owes no review; it asserts no contract this spec depends on, and adding a link would create a dependencies edge for a citation (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- root-absent: 4

## Fixed in this run

*None.*
