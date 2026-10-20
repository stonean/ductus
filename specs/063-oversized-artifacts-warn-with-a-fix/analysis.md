---
spec: 063-oversized-artifacts-warn-with-a-fix
last-run: 2026-10-20T12:58:02Z
analyzed-against: 9d3b4ad3b50f50084d995727e599a7c2c5afbe46
hard-fail: 0
blocking-findings: 1
advisory: 3
unexamined: 2
analyzed-digest:
  data-model.md: ded66d44180f7288cd2914d8409b18b7761fa37b97a7cec299da63162ffdf2cb
  plan.md: dc48b595acadde1d9118008e6aa899702f40f62fe5ca1d858d64eeeac81b5d65
  review.md: 8713b157244db5b309e4fb033904afa4362c09d70e0b7534be79e5a688088b30
  scenarios/an-invalid-threshold-fails-fast.md: fb341859bc463f2c51d84856d76f158208a4b8180f9d94bc5537fc9f1bb62c71
  scenarios/an-unknown-artifacts-key-is-named.md: 912ce563e772f628884c9096a8fafb00526379e935c99f40d7ca96090d986e1a
  spec.md: f90a5d5e8d4af443a9378d4ec528387edda313fc3109468131ea8ec7f4843dc9
  tasks.md: b6fd4656772abccc9f31988b2af8d1b059c2a7dc730541b01054f70721bea02e
unexamined-by-reason:
  root-absent: 2
blocking: true
dispositions:
  fixed: 1
  routed: 1
  discarded: 3
  undispositioned: 0
decisions:
  - key: grounding — spec.md says an adopter's 2,200-line plan.md was never read in full, citing no source
    outcome: discarded
    reason: motivation that cannot be cited without naming another project, which AGENTS.md forbids; the threshold rests on the 2026-09-28 and 2026-10-04 measurements the spec does cite
    decided-at: 2026-10-04T18:18:21Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — BE-METRIC-001: the check-artifact-size MCP handler commits to no rate, error or duration metrics"
    outcome: discarded
    reason: the runtime is a per-agent stdio process with no metrics pipeline, and no primitive emits metrics; adding the first belongs to a spec about the runtime's observability (as discarded in 022, 041 and 062)
    decided-at: 2026-10-04T18:18:21Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — BE-REPR-003: threshold.bytes is a JSON integer with no maximum, so a configured read size above 2^53 − 1 would not survive a binary64 parser"
    outcome: discarded
    reason: a read size above 2^53 bytes changes no page count, since no subject approaches 8 PiB, so a binary64 client's rounding cannot change a result; a maximum would invent a bound no host sets
    decided-at: 2026-10-20T02:12:12Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — BE-INPUT-004: plan.md commits the host-supplied feature name only to validate_no_traversal, a textual check, not to canonicalization before a whole-component containment check"
    outcome: routed
    target: specs/022-deterministic-runtime/tasks.md
    decided-at: 2026-10-20T12:58:02Z
    decided-by: andy@stone.dev
---

# Analysis — 063-oversized-artifacts-warn-with-a-fix

## Summary

0 hard-fail, 1 blocking, 3 advisory; blocking. 2 unexamined target(s). Dispositions: 1 fixed, 1 routed, 3 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

- rule-assessment — BE-INPUT-004: plan.md commits the host-supplied feature name only to validate_no_traversal, a textual check, not to canonicalization before a whole-component containment check — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — **routed** to `specs/022-deterministic-runtime/tasks.md`

## Advisory findings

- grounding — spec.md says an adopter's 2,200-line plan.md was never read in full, citing no source — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **discarded**: motivation that cannot be cited without naming another project, which AGENTS.md forbids; the threshold rests on the 2026-09-28 and 2026-10-04 measurements the spec does cite
- rule-assessment — BE-METRIC-001: the check-artifact-size MCP handler commits to no rate, error or duration metrics — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — **discarded**: the runtime is a per-agent stdio process with no metrics pipeline, and no primitive emits metrics; adding the first belongs to a spec about the runtime's observability (as discarded in 022, 041 and 062)
- rule-assessment — BE-REPR-003: threshold.bytes is a JSON integer with no maximum, so a configured read size above 2^53 − 1 would not survive a binary64 parser — `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md` — **discarded**: a read size above 2^53 bytes changes no page count, since no subject approaches 8 PiB, so a binary64 client's rounding cannot change a result; a maximum would invent a bound no host sets

## Unexamined targets

- root-absent: 2

## Fixed in this run

- applicable-rules — Applicable Rules cites CFG-ENV-005, whose Verification trigger (an environment variable with a documented range) does not fire against a file-based key — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **fixed**
