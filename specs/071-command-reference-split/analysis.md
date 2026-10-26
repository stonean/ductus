---
spec: 071-command-reference-split
last-run: 2026-10-26T15:53:14Z
analyzed-against: cde884d0c3dbf7c99ce992f8ee38da8ea801bf5f
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 5
analyzed-digest:
  plan.md: 136622e09b79354ae0c06856da094964fc80aeacdb9ad19c95646347d1653e19
  review.md: 4b9674011536ad9dc3d80ee4b316b19e3a768265ccfd805bf8810421cc2961dd
  spec.md: 2ca8759919d40223326a806b637b54a933c02b08f6d231ffe911865ffae22eda
  tasks.md: 2c4b207ac572275742b0cfb9e64e1b7e39d1409de6e146728efc4f6144d5decf
unexamined-by-reason:
  not-a-live-claim: 2
  root-absent: 2
  ships-to-adopter: 1
blocking: false
dispositions:
  fixed: 7
  routed: 0
  discarded: 3
  undispositioned: 0
decisions:
  - key: grounding — spec.md:24–28 states that Claude Code compaction keeps only the first 5,000 tokens of an invoked skill, citing no documentation
    outcome: discarded
    reason: motivating context the spec marks as an assumption, and no criterion rests on it (owner decision, 2026-10-26)
    decided-at: 2026-10-26T15:53:14Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md gives a ~20KB compaction window for analyze.md with no source
    outcome: discarded
    reason: derived from the 5,000-token figure the spec marks as an assumption; no criterion rests on it (owner decision, 2026-10-26)
    decided-at: 2026-10-26T15:53:14Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md:18–19 states that invoking a pipeline command loaded its whole body into the agent's context, citing no source
    outcome: discarded
    reason: the spec's premise, that a slash command's body is the prompt the host expands; AC20 measures bytes, not loading (owner decision, 2026-10-26)
    decided-at: 2026-10-26T15:53:14Z
    decided-by: andy@stone.dev
---

# Analysis — 071-command-reference-split

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 5 unexamined target(s). Dispositions: 7 fixed, 0 routed, 3 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — spec.md:24–28 states that Claude Code compaction keeps only the first 5,000 tokens of an invoked skill, citing no documentation — `specs/071-command-reference-split/spec.md` — **discarded**: motivating context the spec marks as an assumption, and no criterion rests on it (owner decision, 2026-10-26)
- grounding — plan.md gives a ~20KB compaction window for analyze.md with no source — `specs/071-command-reference-split/plan.md` — **discarded**: derived from the 5,000-token figure the spec marks as an assumption; no criterion rests on it (owner decision, 2026-10-26)
- grounding — spec.md:18–19 states that invoking a pipeline command loaded its whole body into the agent's context, citing no source — `specs/071-command-reference-split/spec.md` — **discarded**: the spec's premise, that a slash command's body is the prompt the host expands; AC20 measures bytes, not loading (owner decision, 2026-10-26)

## Unexamined targets

- not-a-live-claim: 2
- root-absent: 2
- ships-to-adopter: 1

## Fixed in this run

- rule-assessment — BE-COMPAT-001: plan.md changes the results of resolve-anchor (a fourth kind in a set 067's data model enumerates as three), check-step-references and check-corpus-links without classifying the change as additive or breaking — `specs/071-command-reference-split/plan.md` — **fixed**
- grounding — spec.md:89–91 and plan.md Trade-offs state that every .md file under an agent's commands directory registers as a command, citing no source — `specs/071-command-reference-split/spec.md` — **fixed**
- grounding — spec.md:129–135 and plan.md state that Pi substitutes $ARGUMENTS only in the command body it loads, citing no source — `specs/071-command-reference-split/spec.md` — **fixed**
- grounding — spec.md:126 states that the installer substitutes {project} in every file it copies, citing no source — `specs/071-command-reference-split/spec.md` — **fixed**
- grounding — spec.md:168–170 states that nothing in the runtime reads a reference file, which its Cross-file pointers section contradicts, and cites no source for the constitution fallback — `specs/071-command-reference-split/spec.md` — **fixed**
- grounding — spec.md:202 states that review.md's Behavior heading had gone before 071 without naming the commit that removed it — `specs/071-command-reference-split/spec.md` — **fixed**
- grounding — spec.md:413 states that the colon-form invocation is wrong for non-claude layouts, citing no source — `specs/071-command-reference-split/spec.md` — **fixed**
