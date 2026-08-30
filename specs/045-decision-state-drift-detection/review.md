---
spec: 045-decision-state-drift-detection
reviewed-at: 2026-08-30T23:39:53Z
reviewed-against: 98a228425cd34c78426cbbb7e57730eb5948abc3
diff-base: c27a08f44206460a9306bebce800561f109d8407
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 045-decision-state-drift-detection

## Summary

Incremental review over the third `review-state-drift` check: an outstanding SHOULD on a `done` spec, plus its documentation in `/{project}:analyze`. All five passes ran. No findings.

The reuse pass is what decided the shape, and it is the reason this is three lines rather than a new audit family. `review-state-drift` already owns the claim "the `review:` block disagrees with the state `done` asserts", already runs only on `done` specs, and already carries the grandfather rule for pre-review specs. It also ships to adopters through `/{project}:analyze`, where a maintainer-only audit family would not have — and the defect it catches is not maintainer-specific.

The quality pass looked at the `--fix` interaction, which is where this could have gone wrong. `--fix` reverts the status and does **not** touch the count. Editing the count would make the finding disappear while the SHOULD stayed unaddressed — erasing the evidence rather than resolving it, and the check would then be reporting on state it had itself rewritten. The count belongs to `/{project}:review`, which regenerates it from a pass; reverting the status is what pushes the operator back through that pass.

The message names **both** dispositions. `§implement-phase` gives two ways to address a SHOULD — fix it, or waive it with rationale — and a SHOULD whose answer is "keep as-is" takes the second. A message naming only "fix each" would push an operator toward the wrong one for exactly the findings most likely to be outstanding.

Verified by seeding: 023's own shape reinstated on its spec makes the check fire, restoring it makes it silent, and the corpus reports zero. The negative case is pinned too — a spec still in flight carrying three SHOULDs is not a finding, because the rule is about the state `done` asserts, not about the finding existing.

Worth recording what this closes and what it does not. It catches the count disagreeing with `done`; it cannot catch a count that is itself wrong, because the count and `review.md` are written together by one run. That remains the structural blind spot, and it is why this check reads the *status* rather than trying to re-derive the count.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Observations

*None.*

## Skipped passes

*None.*
