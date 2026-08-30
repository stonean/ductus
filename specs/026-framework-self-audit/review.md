---
spec: 026-framework-self-audit
reviewed-at: 2026-08-30T21:46:46Z
reviewed-against: fd2501ab6c105ab12070e2c56543e8812f33e5f3
diff-base: 2818a378784f5b364dec93d6cd6d5031711f390e
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

Incremental review over the `link-check-consolidation` scenario and its implementation: the `LinkScope` argument on `check-corpus-links`, the git-index enumeration, the scope-dependent exclusions, Family 26 rewritten as an entry point, and Family 33's documented token constraint. The spec's earlier surface is unchanged since the previous review and is not re-examined.

All five passes ran. One defect was found and fixed rather than recorded, and it was found by the audit rather than by reading.

The **security-adjacent** finding is the one worth naming: the consolidation hardcoded `.claude/` as an exclusion prefix in runtime source. That is correct for exactly one of the four hosts ductus ships to — every Auggie, Antigravity, and OpenCode adopter would have had their generated command copies examined, and those copies' links are broken *by construction* because the generator changes their depth without rewriting them. The result would have been a corpus-wide check reporting defects an adopter has no way to fix, in the file the adopter is told to trust. Resolved from `Host::load` instead (`fd2501a`). Family 13 exists for precisely this regression and caught it on the first full audit run; the test fixture is now assembled rather than written as a literal so it cannot re-trip the family it just proved.

The reuse pass is what the change answers rather than raises. Two implementations of one rule had already diverged once — the primitive resolving a root-absolute target against the repository root while the family resolved it against the filesystem root — and delegation makes that divergence impossible rather than repaired.

The quality pass looked hardest at the subject. The scenario named the narrowing hazard in advance, and it is the failure that would not have failed anything: calling the primitive with its default scope shrinks Family 26's subject from the repository to the spec corpus, and the family goes on exiting 0 over a fraction of its files. Verified by measurement rather than reasoning — 457 examined before, 457 after — pinned by a test asserting the repository scope examines strictly more than the spec-corpus scope, and reported on stderr on every run so a future narrowing is visible rather than inferred.

`runtime/tests/` moved from a silent pre-filter to a counted exclusion, which is the same discipline applied to the family's own bookkeeping: `excluded-by-construction` now reads 97 rather than 27, and the files it names are stated rather than dropped.

The efficiency and simplicity passes produced nothing. The git-index enumeration avoids descending into `runtime/target`, and no new primitive was added where an argument served.

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
