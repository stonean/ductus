---
spec: 052-spec-supersession-and-consolidation
reviewed-at: 2026-08-30T21:23:15Z
reviewed-against: 62381d05f0bd4d78396f4efcbdcae889c1dd8e06
diff-base: 2818a378784f5b364dec93d6cd6d5031711f390e
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 052-spec-supersession-and-consolidation

## Summary

Incremental review over the `stranded-session-after-removal` scenario and its implementation: the `§concurrent-features` rule, `/{project}:consolidate`'s new clear step, `/{project}:fold`'s citation of the shared rule, and the step-order test that pinned the change. The spec's earlier surface is unchanged since the previous review and is not re-examined.

All five passes ran. No findings.

The reuse pass is what this change answers rather than raises: two commands held two answers to one question, and the rule now lives once in the section that already owns the session file's semantics, with both commands citing it instead of restating half of it each. `/{project}:fold`'s behavior is byte-identical — only its prose changed — so there is no cross-spec impact on 051 to record.

The quality pass looked hardest at the conditional. Clearing runs only when the session actually named the removed feature, so a session pointing elsewhere is untouched; and it runs *after* the removal, since the session cannot be stranded until the directory is gone. Both are pinned by the step-order test, which caught the new step on its first run — the reason that test exists.

One thing the change deliberately does **not** claim, and says so: the session file is per-contributor and gitignored, so a teammate's stale target is unreachable from this repository. The rule states that bound rather than implying the problem is closed — the distinction between what was fixed and what merely cannot be, which is the same discipline `QUAL-CLAIM-001` asks of a clean result.

The simplicity pass raised nothing. The clear reuses `write-session`'s existing clear mode, which already preserves `cli-config-dir`; no new primitive, no new argument, and no third outcome invented for a case nobody has.

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
