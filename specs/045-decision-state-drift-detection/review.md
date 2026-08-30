---
spec: 045-decision-state-drift-detection
reviewed-at: 2026-08-30T23:27:06Z
reviewed-against: 5f948e196d790d7dff035f6c02d93fb015176230
diff-base: 1eda6f6f626eb368473b1dcae957392ba0e210d0
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 045-decision-state-drift-detection

## Summary

Incremental review over a reopen that produced no code. The `removal-claims-are-checkable` scenario was written, implemented against `check_artifacts.rs`, measured over all 54 specs, and withdrawn; the implementation is reverted and the finding is recorded as a resolved question. `criterion-path-existence` is byte-identical to its state before the reopen.

All five passes ran. No findings — there is no new code to find them in.

**What the reopen bought is the measurement, and it is worth more than the feature would have been.** The premise survived scrutiny: a criterion claiming a path is absent is exactly as checkable as one claiming presence, and 72 criteria across 18 specs had nothing verifying their substance. What failed is attribution. Of the 31 removal-marked criteria only 5 name a single path; 26 name several and make opposite claims about them in one sentence, so a phrase-position heuristic cannot tell which path "deleted" is about. Run over the corpus, the clause-scoped implementation produced 20 findings of which roughly 8 were false — `023`'s AC3 reporting `framework/constitution.md` as wrongly present because *deleted* referred to the verbs `/capture` and `/elaborate`.

It also **broke the existing remedy**, which is the part worth remembering. An annotation is appended at the end of a criterion; clause scoping stopped it qualifying the earlier clause, and six findings that annotations had correctly suppressed came back. A change that defeats the mechanism the corpus already relies on has to clear a much higher bar than one that merely adds coverage, and this did not.

The quality pass has one observation on process rather than code: the hand-walk of all 72 criteria happened *before* the implementation attempt, and that ordering is why the rejection is credible. The walk established which claims actually hold, so every finding the new check produced could be judged true or false against a known answer. Had the check been built first, its 20 findings would have looked like a discovery.

The withdrawal is recorded as a resolved question rather than a scenario carrying a "rejected" note, matching how this corpus already records the criterion-supersession check that was measured at 455 pairs. A scenario describes behavior that ships.

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
