---
spec: 023-govern-refinement
reviewed-at: 2026-08-30T14:57:14Z
reviewed-against: d1c56d429153541bbdbb6111eaaca8db9968245f
diff-base: 0ce71ab99fe2268a8f52ba9e05787758016ea365
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 023-govern-refinement

## Summary

0 MUST, 0 SHOULD, 0 low-confidence. Reviewed the AC34–AC40 delta: the `Write(path)` removal from the canonical allow set, `/ductus:configure` step 4's retired-entries list, `merge-permissions`' new `revoke` argument, and audit Family 32 (32a shape, 32b disjointness). This repo has no `specs/rules/` directory, so no rule files were loaded and no finding can be rule-cited; the passes ran as an uncited correctness and consistency review rather than a rule-mapped one, and the zero counts should be read that way. What the pass covered and confirmed: the revoke pass runs before dedup and canonical-presence so no retired copy survives and none is re-added; retirement cannot reach the deny array because the primitive has no deny-side argument, not because a caller is asked to refrain; `allow`/`revoke` overlap is rejected before any filesystem read, so a contradictory call leaves no partial write; the conflict list is deduped so a repeated canonical entry cannot inflate the reported count (fixed during the pass); non-string and malformed-JSON inputs are preserved or refused rather than rewritten. Family 32's extraction requires a parenthesised argument, so the bare `Edit`/`Write` tool grants are excluded by construction, and its bullet anchor keeps §2's counter-example prose and §4's retired list — both of which contain the offending literals — outside every scanned window; all three negative paths were exercised. 30 unit tests on the primitive, 1294 across the runtime, 32 audit families, and an end-to-end CLI run against a pre-fix adopter file (nine retired, second run `unchanged`, adopter-authored and deny-side entries intact).

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
