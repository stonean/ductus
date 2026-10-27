---
spec: 063-oversized-artifacts-warn-with-a-fix
last-run: 2026-10-27T04:23:55Z
analyzed-against: 88768fe67738169b56f4f60554eb67b383858398
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 3
analyzed-digest:
  data-model.md: 42dc6d741a76d613b5a1424a86725d08c2d7cb52b2f3901ced2cd736559fa426
  plan.md: a9a23c6762a4b832f05386a545bf12d8d03f26464011b400fd8b91477dd06d40
  review.md: b0a87b69e2a2cfe1a2b2a92c0437dff119442cfca1f75003d3e6b98a29d1b292
  scenarios/a-plan-trim-does-not-reopen-a-done-spec.md: 981fb71fcb69757942c915c7b000474cfbe278bd7b9e89aefbfda7307e1e28fa
  scenarios/an-invalid-threshold-fails-fast.md: 59bceffaed0e278952f69acb4f847a3639b7390314cf212dfbef405624c37f23
  scenarios/an-unknown-artifacts-key-is-named.md: 912ce563e772f628884c9096a8fafb00526379e935c99f40d7ca96090d986e1a
  spec.md: d813778cf902688d43db3288b48df70adc79747be1d00c23f8d4da5b00f55f4d
  tasks.md: bce08a811fb786c60ff4e72b1de7d637bd0ea324a1214ace33e06e3aa2907b76
unexamined-by-reason:
  root-absent: 3
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

0 hard-fail, 0 blocking, 3 advisory; not blocking. 3 unexamined target(s). Dispositions: 0 fixed, 0 routed, 3 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — spec.md says an adopter's 2,200-line plan.md was never read in full, citing no source — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **discarded**: motivation that cannot be cited without naming another project, which AGENTS.md forbids; the threshold rests on the 2026-09-28 and 2026-10-04 measurements the spec does cite
- rule-assessment — BE-METRIC-001: the check-artifact-size MCP handler commits to no rate, error or duration metrics — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **discarded**: the runtime is a per-agent stdio process with no metrics pipeline, and no primitive emits metrics; adding the first belongs to a spec about the runtime's observability (as discarded in 022, 041 and 062)
- rule-assessment — BE-REPR-003: threshold.bytes is a JSON integer with no maximum, so a configured read size above 2^53 − 1 would not survive a binary64 parser — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **discarded**: a read size above 2^53 bytes changes no page count, since no subject approaches 8 PiB, so a binary64 client's rounding cannot change a result; a maximum would invent a bound no host sets

## Unexamined targets

- root-absent: 3

## Fixed in this run

*None.*
