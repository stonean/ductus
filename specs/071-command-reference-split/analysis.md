---
spec: 071-command-reference-split
last-run: 2026-11-11T13:48:08Z
analyzed-against: 9c895475bd0db518a348aa8af410db87a0df1626
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 0
analyzed-digest:
  plan.md: 53a34859404e41a950ab9024027f7d0b6d0415207845d60fa70de31465664066
  review.md: 79cccceb9c88226a67841c2837c8394295224db19ce5b1afc23a1428c227ec66
  spec.md: a3eb479b5e006b77dbcef8ead5f4dad7480e39fd6c1f31adc04d665d2755e14f
  tasks.md: d90b344d6c1d26be945ce82e87c067dedb3843b88a32a26841709aa275d81172
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 3
  undispositioned: 0
decisions:
  - key: grounding — spec.md:24–28 states that Claude Code compaction keeps only the first 5,000 tokens of an invoked skill, citing no documentation
    outcome: waived
    reason: motivating context the spec marks as an assumption, and no criterion rests on it (owner decision, 2026-10-26)
    decided-at: 2026-10-26T15:53:14Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md gives a ~20KB compaction window for analyze.md with no source
    outcome: waived
    reason: derived from the 5,000-token figure the spec marks as an assumption; no criterion rests on it (owner decision, 2026-10-26)
    decided-at: 2026-10-26T15:53:14Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md:18–19 states that invoking a pipeline command loaded its whole body into the agent's context, citing no source
    outcome: waived
    reason: the spec's premise, that a slash command's body is the prompt the host expands; AC20 measures bytes, not loading (owner decision, 2026-10-26)
    decided-at: 2026-10-26T15:53:14Z
    decided-by: andy@stone.dev
---

# Analysis — 071-command-reference-split

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 0 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 3 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — spec.md:24–28 states that Claude Code compaction keeps only the first 5,000 tokens of an invoked skill, citing no documentation — `specs/071-command-reference-split/spec.md` — **waived**: motivating context the spec marks as an assumption, and no criterion rests on it (owner decision, 2026-10-26)
- grounding — plan.md gives a ~20KB compaction window for analyze.md with no source — `specs/071-command-reference-split/plan.md` — **waived**: derived from the 5,000-token figure the spec marks as an assumption; no criterion rests on it (owner decision, 2026-10-26)
- grounding — spec.md:18–19 states that invoking a pipeline command loaded its whole body into the agent's context, citing no source — `specs/071-command-reference-split/spec.md` — **waived**: the spec's premise, that a slash command's body is the prompt the host expands; AC20 measures bytes, not loading (owner decision, 2026-10-26)

## Deferred to other specs

*None.*

## Unexamined targets

*None — every target was examined.*

## Fixed in this run

*None.*
