---
spec: 046-scenario-open-question-visibility
last-run: 2026-11-11T14:06:34Z
analyzed-against: c7991df7dca0a8d714d29ecb87e1e56f502a0b32
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 2
analyzed-digest:
  plan.md: 8ff27121b82564c7026813620f75aaedee639db7eecb789b535312df5ae62d03
  review.md: 6b4638eb124e6758822de7565bbd0dcfbda48bccc16855bf22dc51035b0a4632
  spec.md: c0d944c405532f28b8e7b28e2419c114ff2112ed3ea807ee6134ad1fc56c67dc
  tasks.md: 81e14e4461f19a447dd63216ba76ec4f448316c11ebc471e14752d6e39c72148
unexamined-by-reason:
  no-readable-state: 2
blocking: false
dispositions:
  fixed: 5
  deferred: 0
  waived: 3
  undispositioned: 0
decisions:
  - key: grounding — spec.md's What already works says /clarify's scenario-targeted branch resolves scenario questions in place and /target and /status route a targeted scenario to scenario-targeted clarify, citing no step
    outcome: waived
    reason: citing it needs a lookup in the command sources that state it, which is research rather than a chore
    decided-at: 2026-11-11T14:06:34Z
    decided-by: rule
  - key: grounding — spec.md names the shared scenario-file listing, its set and its case-insensitive order, by description only
    outcome: waived
    reason: naming the listing's symbol needs a lookup in the runtime source, which is research rather than a chore
    decided-at: 2026-11-11T14:06:34Z
    decided-by: rule
  - key: grounding — spec.md and plan.md say the dashboard already renders a recovery-state Next Action override and callout, citing no code
    outcome: waived
    reason: naming the code that renders it needs a lookup in the runtime source, which is research rather than a chore
    decided-at: 2026-11-11T14:06:34Z
    decided-by: rule
---

# Analysis — 046-scenario-open-question-visibility

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 2 unexamined target(s). Dispositions: 5 fixed, 0 deferred, 3 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — spec.md's What already works says /clarify's scenario-targeted branch resolves scenario questions in place and /target and /status route a targeted scenario to scenario-targeted clarify, citing no step — `specs/046-scenario-open-question-visibility/spec.md` — **waived**: citing it needs a lookup in the command sources that state it, which is research rather than a chore (decided by the rule)
- grounding — spec.md names the shared scenario-file listing, its set and its case-insensitive order, by description only — `specs/046-scenario-open-question-visibility/spec.md` — **waived**: naming the listing's symbol needs a lookup in the runtime source, which is research rather than a chore (decided by the rule)
- grounding — spec.md and plan.md say the dashboard already renders a recovery-state Next Action override and callout, citing no code — `specs/046-scenario-open-question-visibility/spec.md` — **waived**: naming the code that renders it needs a lookup in the runtime source, which is research rather than a chore (decided by the rule)

## Deferred to other specs

*None.*

## Unexamined targets

- no-readable-state: 2

## Fixed in this run

- rule-assessment — BE-PAYLOAD-001: spec.md lists scenario questions with no cap and committed to no size bound or paging for the read-spec field, the gate's message or /target's readout, beyond the pipeline view's cut — `specs/046-scenario-open-question-visibility/spec.md` — **fixed** (decided by the rule)
- grounding — spec.md says the scenario template describes Resolved Questions as holding answers preserved for context, naming no template file — `specs/046-scenario-open-question-visibility/spec.md` — **fixed** (decided by the rule)
- grounding — spec.md says /target already lists the available scenarios on an unmatched scenario slug, citing no step — `specs/046-scenario-open-question-visibility/spec.md` — **fixed** (decided by the rule)
- grounding — spec.md describes the review-state-drift --fix revert without citing the reference that owns it — `specs/046-scenario-open-question-visibility/spec.md` — **fixed** (decided by the rule)
- grounding — spec.md's parser Resolved Question states the parser's heading parameter, continuation folding, append-question sharing and the create-scenario placeholder with no source — `specs/046-scenario-open-question-visibility/spec.md` — **fixed** (decided by the rule)
