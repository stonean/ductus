---
spec: 010-agent-autonomy
reviewed-at: 2026-08-17T00:14:48Z
reviewed-against: d8c5c616648e9ae2ee06af0e8c9abd4e09613bc1
diff-base: 07db968b79347c49df55d31000975335fc40ca04
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 010-agent-autonomy

## Summary

All five passes ran over the 56-file scope resolved from `07db968..HEAD` (the modified-since set, larger than the plan's Affected Files). Clean: 0 MUST, 0 SHOULD, 0 low-confidence, nothing waived, no pass skipped.

The prior run of this review reported one SHOULD — `QUAL-CLAIM-001` against `SweepIndex::build`, which discarded `git2`'s `diff.foreach` result and could fold a half-delivered diff into an index that granted exemptions it had not earned. It has been **fixed** rather than carried, per this repo's rule that a SHOULD blocks `done` exactly as a MUST does: both failure branches now return one named `SweepIndex::unreadable()`, so an unreadable diff has a single representation and every path reports as changed. Shipped in `0.29.3` with a regression test (`an_unreadable_diff_grants_no_exemptions`).

The same run recorded one observation — `ductus derive-routing-candidates` was the only CLI subcommand with no clap doc comment, so it listed blank in `ductus --help`. Also fixed in `0.29.3` and its inbox bullet removed, so no observation is outstanding.

Verification: 938 tests pass (`--locked --release`), clippy clean on `--all-targets`, and `scripts/audit/run-all.sh` exits 0 — confirmed to be a real pass rather than a silent one by deliberately desyncing the version file and watching Family 20 go red.

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
