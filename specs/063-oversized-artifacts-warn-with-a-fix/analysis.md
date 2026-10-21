---
spec: 063-oversized-artifacts-warn-with-a-fix
last-run: 2026-10-21T00:43:35Z
analyzed-against: 82e1c9537474a65f4800c67f628949ea802cc2fa
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 2
analyzed-digest:
  data-model.md: b0684b84bc923f52fbfd751cfeafaef3bb0289081e57ba61de3176269c983f1a
  plan.md: 0bb088d8c29be6eb23078743807e5801026e5a8de402b568f853e0c1b85b5409
  review.md: 444a9707b99b239e2402b590f367bd6188f963ae5044bc8c47cf53c05222d852
  scenarios/an-invalid-threshold-fails-fast.md: 59bceffaed0e278952f69acb4f847a3639b7390314cf212dfbef405624c37f23
  scenarios/an-unknown-artifacts-key-is-named.md: 912ce563e772f628884c9096a8fafb00526379e935c99f40d7ca96090d986e1a
  spec.md: 7fa7fa75e4b187840654e7fdbc2b5cf6a397239258fc50447dd99958c0d7b4a8
  tasks.md: b6fd4656772abccc9f31988b2af8d1b059c2a7dc730541b01054f70721bea02e
unexamined-by-reason:
  root-absent: 2
blocking: false
dispositions:
  fixed: 0
  routed: 0
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
---

# Analysis — 063-oversized-artifacts-warn-with-a-fix

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 2 unexamined target(s). Dispositions: 0 fixed, 0 routed, 3 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — spec.md says an adopter's 2,200-line plan.md was never read in full, citing no source — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **discarded**: motivation that cannot be cited without naming another project, which AGENTS.md forbids; the threshold rests on the 2026-09-28 and 2026-10-04 measurements the spec does cite
- rule-assessment — BE-METRIC-001: the check-artifact-size MCP handler commits to no rate, error or duration metrics — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — **discarded**: the runtime is a per-agent stdio process with no metrics pipeline, and no primitive emits metrics; adding the first belongs to a spec about the runtime's observability (as discarded in 022, 041 and 062)
- rule-assessment — BE-REPR-003: threshold.bytes is a JSON integer with no maximum, so a configured read size above 2^53 − 1 would not survive a binary64 parser — `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md` — **discarded**: a read size above 2^53 bytes changes no page count, since no subject approaches 8 PiB, so a binary64 client's rounding cannot change a result; a maximum would invent a bound no host sets

## Unexamined targets

- root-absent: 2

## Fixed in this run

*None.*
