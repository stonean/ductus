---
spec: 063-oversized-artifacts-warn-with-a-fix
last-run: 2026-10-20T02:12:12Z
analyzed-against: 2bfdfa4ed142d2e42433b94d5e49d77258e4e784
hard-fail: 0
blocking-findings: 0
advisory: 4
unexamined: 1
analyzed-digest:
  data-model.md: 0bd537b2f16e9aa7114a70a1e21549711827954dcdb028809ddfe8d92c2285a5
  plan.md: f7b6d50c9e9c64acd243cc7706c592ea0ad50748e85965f0d9535cda5a63aa0c
  review.md: e8ff711044ea803da7915a503a303cd32e111e36cb5067a0c9c5f8efef9a7c60
  scenarios/an-invalid-threshold-fails-fast.md: fb341859bc463f2c51d84856d76f158208a4b8180f9d94bc5537fc9f1bb62c71
  spec.md: 4af7443771113805915bc3e44fa8a13e04d24cae57c9289fbbd341f5c11a9207
  tasks.md: f4dbc68549f2850cb90c974e13362509ac8f01837795898dc807d13955e58ebb
unexamined-by-reason:
  root-absent: 1
blocking: false
dispositions:
  fixed: 3
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
  - key: "rule-assessment — CFG-ENV-012: plan.md introduces the [artifacts] section without saying how an unrecognized key in it is treated"
    outcome: routed
    target: specs/019-config-decisions/tasks.md
    decided-at: 2026-10-20T02:12:12Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — BE-REPR-003: threshold.bytes is a JSON integer with no maximum, so a configured read size above 2^53 − 1 would not survive a binary64 parser"
    outcome: discarded
    reason: a read size above 2^53 bytes changes no page count, since no subject approaches 8 PiB, so a binary64 client's rounding cannot change a result; a maximum would invent a bound no host sets
    decided-at: 2026-10-20T02:12:12Z
    decided-by: andy@stone.dev
---

# Analysis — 063-oversized-artifacts-warn-with-a-fix

## Summary

0 hard-fail, 0 blocking, 4 advisory; not blocking. 1 unexamined target(s). Dispositions: 3 fixed, 1 routed, 3 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — spec.md says an adopter's 2,200-line plan.md was never read in full, citing no source — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **discarded**: motivation that cannot be cited without naming another project, which AGENTS.md forbids; the threshold rests on the 2026-09-28 and 2026-10-04 measurements the spec does cite
- rule-assessment — BE-METRIC-001: the check-artifact-size MCP handler commits to no rate, error or duration metrics — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — **discarded**: the runtime is a per-agent stdio process with no metrics pipeline, and no primitive emits metrics; adding the first belongs to a spec about the runtime's observability (as discarded in 022, 041 and 062)
- rule-assessment — CFG-ENV-012: plan.md introduces the [artifacts] section without saying how an unrecognized key in it is treated — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — **routed** to `specs/019-config-decisions/tasks.md`
- rule-assessment — BE-REPR-003: threshold.bytes is a JSON integer with no maximum, so a configured read size above 2^53 − 1 would not survive a binary64 parser — `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md` — **discarded**: a read size above 2^53 bytes changes no page count, since no subject approaches 8 PiB, so a binary64 client's rounding cannot change a result; a maximum would invent a bound no host sets

## Unexamined targets

- root-absent: 1

## Fixed in this run

- rule-assessment — BE-INPUT-004: plan.md does not commit to the traversal check on the host-supplied feature name it joins under the spec root — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — **fixed**
- rule-assessment — BE-INPUT-008: plan.md reads the decisions list and the spec status from YAML frontmatter without naming the safe-loading mode — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — **fixed**
- grounding — plan.md states Cargo's pre-1.0 compatibility boundary and 0.53.0's removal of result fields without a source — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — **fixed**
