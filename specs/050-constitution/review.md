---
spec: 050-constitution
reviewed-at: 2026-08-19T01:32:58Z
reviewed-against: b45f324942cb397bcad6ceaef87f825af78a2059
diff-base: e9262df
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 050-constitution

## Summary

Clean — 0 MUST, 0 SHOULD, 0 low-confidence, 0 observations.

Scope: the completion-claim filter in §design-principles, the §implement-phase rewrite that references it, the preamble count fix, the scenario, AC14, and the contributor-side mirror in `AGENTS.md`.

**Stated once, not twice.** The governing question for this amendment was whether it duplicates §implement-phase's outstanding-SHOULD rule, which already contained the principle scoped to review findings. It would have, written as a second rule — so §implement-phase now opens by referencing the filter as the instance where it fires most often and keeps only what is specific to the review gate. Family 6 (SSOT invariants) is green, which is the check that would have caught a restatement.

**The `AGENTS.md` entry points rather than restates**, matching the convention every other promoted entry follows: it names the constitution rule and records what the failure cost *here* — three occurrences in one session, on three different surfaces — rather than re-stating the rule text.

**A pre-existing staleness in the same section was fixed rather than worked around.** The preamble read "Two constraints … Both are hard filters" while four bullets followed. It now carries no count and says why, which is the section warning about staleness no longer containing an instance of it.

**Scope boundaries in the scenario are drawn deliberately.** The filter governs residue once known; it does not license frontfilling, and it does not touch a check honestly reporting what it excluded from scope — that is the first design principle being obeyed, not a caveat.

One defect was caught and fixed rather than shipped: the scenario's four constitution links carried the same depth error Family 26 had been built one commit earlier to detect, and the family caught them. Worth recording because it is direct evidence for that family — the error is invisible to inspection, since a wrong relative link renders identically to a right one.

Verification: 26 audit families green, markdownlint clean over 434 files, §design-principles anchor still resolves from all three referring sites.

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
