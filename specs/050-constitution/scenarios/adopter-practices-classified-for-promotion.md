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

Some sit beside existing text: (2) is one of this repository's `AGENTS.md` gotchas, (3) is near §drift-prevention's retired-filename rule, and (6) is near `AGENTS.md`'s rule that process cost never weighs in a recommendation.

## Behavior

- Each practice is classified once, with its reason, by this spec's Classification: promoted to the constitution, promoted to one command's `framework/reference/<command>.md` with the constitution pointing at it, kept project-only, merged into the existing rule it restates, or declined.
- A promoted practice has one normative statement and, where this repository's `AGENTS.md` already carries it, a mirror line pointing at it, per the Promotion mechanism.
- Any promotion keeps the constitution within the read size 072 bounds it to.

## Edge Cases

- A practice that is already true of the constitution's text in other words: merged or declined, never a second copy.
- A practice true only for projects using a capability ductus does not ship (a particular host's clone behavior): project-only.

## Open Questions

- For each of the seven practices, which destination: the constitution, one command's reference (`review.md` for 5–7?), merged into the existing rule it sits beside (2, 3, 6), project-only, or declined — and what is the constitution's measured size after the promotions against 072's bound?

## Resolved Questions

*None yet.*
