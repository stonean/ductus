---
spec: 023-govern-refinement
reviewed-at: 2026-08-30T23:02:22Z
reviewed-against: 5b9be37caf1da07caf621f95485ea45849ff5ff1
diff-base: d1c56d429153541bbdbb6111eaaca8db9968245f
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 023-govern-refinement

## Summary

Re-review to reconcile the record with the code. The previous run recorded one SHOULD — `QUAL-CLAIM-001` against `scripts/audit/permission-entry-shape.sh`, where check 32b rendered an absent retired-entries section as the same `0` a present-but-empty one produces — and its own Suggested fix says "Fixed during this pass". The fix did land: `retired_summary` is initialised to "no retired-entries section present, so no overlap was examined" before the guard and reassigned inside it, and the two states read differently on stderr today.

What did not land was the bookkeeping. The count stayed at `should-violations: 1` and the finding stayed filed under **SHOULD violations**, which is precisely the state §implement-phase forbids: "a spec sitting at `done` with a non-zero SHOULD count and the finding still filed under its original heading". The counts state what is *outstanding*, and this one was not.

So the fix here is the record, not the code. All five passes ran over the unchanged scope and produced no findings; the rule no longer fires at that file, verified by running the family and reading its stderr summary rather than by trusting the earlier note.

Surfaced by a corpus-wide sweep of every spec's `review:` block — 54 examined, this the only one carrying a non-zero MUST or SHOULD. Worth recording how it hid: nothing binds a finding's *disposition* to its count. `/{project}:review` regenerates both from the pass, so a fix applied during the pass that never triggers a re-run leaves the two disagreeing, and Family 31 compares the frontmatter block against `review.md` — which agreed with each other, both being stale. The check that would have caught it is the §implement-phase completion filter, read by a human at the gate.

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

- other: nothing mechanically catches a `done` spec carrying a non-zero SHOULD count with the finding still under its original heading — the state §implement-phase forbids. Family 31 compares the frontmatter `review:` block against `review.md`, but both are written by the same run, so a fix applied *during* a pass that never re-runs leaves them consistently stale and the family clean. 023 sat in that state and was found by a hand sweep, not by tooling. A check family reading `should-violations > 0 && status == done` would be cheap and would close it. — `scripts/audit/review-block-agreement.sh`

## Skipped passes

*None.*
