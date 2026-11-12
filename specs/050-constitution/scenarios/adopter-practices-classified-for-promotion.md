---
section: "Classification"
---

# Adopter-practices-classified-for-promotion

## Context

An adopter recorded seven practices in its own project that it reports hold for every project running ductus, offered for the constitution or `/{project}:review`'s source:

1. Record a decision as the operator's only when the operator made it; otherwise "the agent's recommendation, adopted by the operator, DATE".
2. Name a function, test or file in tasks and decisions, never a line number.
3. A document follows a move by content, not file name, since a file a move split maps to as many paths as its parts went to.
4. Learn what a primitive does in a throwaway clone (`git clone --local` to `/tmp`, run with the main tree's binary).
5. Measure a reviewer's claim against the tree before recording it, breaking things in a clone while reviewers read.
6. Recommend a waiver only where a recorded requirement backs it, never for a cost.
7. Before routing a review observation, ask whether it is design rationale, whose home is a short why at the code.

Some sit beside existing text: (2) is one of this repository's `AGENTS.md` gotchas, (3) is near §drift-prevention's retired-filename rule, and (6) is near `AGENTS.md`'s rule on weighing process cost in a recommendation.

## Behavior

- Each practice is classified once, with its reason, by this spec's Classification: promoted to the constitution, promoted to one command's `framework/reference/<command>.md` with the constitution pointing at it, kept project-only, merged into the existing rule it restates, or declined.
- A promoted practice has one normative statement and, where this repository's `AGENTS.md` already carries it, a mirror line pointing at it, per the Promotion mechanism.
- Any promotion keeps the constitution within the read size 072 bounds it to.

## Edge Cases

- A practice that is already true of the constitution's text in other words: merged or declined, never a second copy.
- A practice true only for projects using a capability ductus does not ship (a particular host's clone behavior): project-only.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **For each of the seven practices, which destination, and what is the constitution's measured size after the promotions?** Decided one at a time by the owner (2026-11-12):
  1. Promoted to `framework/reference/clarify.md`, where Resolved Questions are written: a decision is recorded as the operator's only when the operator made it. Rejected: the constitution's §recommendations, which covers every record at a constitution cost; and project-only, since this repository's `AGENTS.md` carries only a narrower delegation form.
  2. Merged into §grounding's *Cite what you consulted*: a durable artifact names a file plus a symbol, never a bare line number, and `framework/reference/plan.md`, `framework/commands/plan.md` and `framework/commands/analyze.md`, which advise `path:line`, are corrected in the same change; `AGENTS.md`'s gotcha becomes a pointer. Rejected: project-only, which leaves the constitution prescribing `path:line` to every adopter.
  3. Merged into §drift-prevention's *A retired filename is not a retired name*: a reference follows a moved file's content, so a split file's referrers re-point to each part they cite. Rejected: project-only, and declining it as implied, since a split maps one name to several paths, which the bullet does not say.
  4. Project-only: the `git clone --local` and main-binary recipe is host and repository machinery, the case this scenario's second edge case routes to project-only.
  5. Promoted to `framework/reference/review.md` §Run review passes: a pass's finding is checked against the tree before it is recorded. Rejected: the constitution, at a cost no other command needs.
  6. Merged into §design-principles' waive disposition: a waiver records the requirement that puts the work out of scope, never its cost alone, which still admits a recorded requirement such as `AGENTS.md`'s waiver of pipeline-machinery observations. Rejected: `framework/reference/review.md` §Waivers alone, which leaves analysis waivers uncovered.
  7. Promoted to `framework/reference/review.md`'s Observations: an observation that is design rationale is fixed in the run as a short comment at the code, a chore. Rejected: declining it as §drift-prevention's one-file-trap routing, which does not reach a review's routing step.

  The three constitution merges add about 246 bytes to a constitution at 49,990 of its 50,000 bytes (`wc -c` at `ab2ad085`), so they land with a compensating cut in the same change, and its measured size is recorded in the commit that lands them, checked by `scripts/lint-governing-size.sh`.
