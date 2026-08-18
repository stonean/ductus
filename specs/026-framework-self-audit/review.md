---
spec: 026-framework-self-audit
scenario: family-23-sweep-target-manifest-parity
reviewed-at: 2026-08-18T01:43:30Z
reviewed-against: e6f70cafcff5cfea14e2a459db6ccc475cfd783a
diff-base: ca3b59b8156c2218ba95b2c23621574b906bfdd4
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

0 MUST violation(s), 0 SHOULD violation(s), 0 low-confidence finding(s). blocking: no.

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

- convention: /{project}:review's file scope is the *larger* of the plan's Affected Files and the files modified since diff-base, never their union — so on this run the 15-entry plan list won and the resolved scope excluded every file the change actually touched, including the new family script, AGENTS.md, and the constitution. The failure grows with spec maturity: any done spec whose plan lists more files than a follow-on scenario touches gets reviewed against the wrong set, and the report gives no sign that the changed files were never examined. Same shape as QUAL-CLAIM-001 one level up — the review reports on a subject it did not look at. — `framework/commands/review.md`
- convention: scripts/audit/run-all.sh captures each family's output with `output="$($script 2>&1)"` and prints it only when the family exits non-zero, so anything a family writes to stderr to qualify a clean result is discarded on exactly the runs where it qualifies something. Family 23 emits its entry/path counts and the direction it verified (manifest -> list, list completeness unchecked) this way; an operator reading an aggregate /{project}:audit pass never sees it. Affects all 22 families, not just this one. — `scripts/audit/run-all.sh`

## Skipped passes

*None.*
