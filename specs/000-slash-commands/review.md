---
spec: 000-slash-commands
reviewed-at: 2026-08-17T02:24:23Z
reviewed-against: 11ff132d453c829246d6b9394ec862603a60b0d6
diff-base: 1eda6f6f
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 000-slash-commands

## Summary

All five passes ran over this spec's last two open scenarios and the code that closed them. Clean at close: 0 MUST, 0 SHOULD, 0 low-confidence, nothing waived, no pass skipped.

Two of the three questions resolved against existing rules rather than new design. `criterion-route-after-draft`'s first question turned on which command already performs a back-edge on a classified input — `/{project}:amend` says it does, `/{project}:clarify` says it does not — so the criterion route extends amend's classifier instead of widening a gate spec 014 narrowed deliberately. Its second question was posed on a false premise: it assumed §spec-lifecycle's `done` back-edge is scenario-triggered, when the constitution states three edges exist and the third covers any meaningful body edit, naming "new scope" explicitly and routing it "via the same `/amend` flow used for scenarios". No lifecycle change was needed.

The third question needed measurement, and the measurement changed the answer twice. The mechanism was mis-located — the scenario→task family did not vouch coarsely on `done` specs, it skipped them wholesale before pruning evidence was consulted. And the candidate the question proposed (the scenario file postdating the `done` transition) does not discriminate, since both states postdate it. Measured over 46 `done` specs, the file-shape alternative would have produced exactly one finding and it a false positive, which is the direction §tasks-phase forbids; the per-scenario history probe produces zero. Both directions were then proven against real history rather than inferred: a fixture reproducing the never-tasked shape fires, and the same fixture with a task added and later pruned does not.

One SHOULD was raised during the efficiency pass and **fixed** rather than recorded, per this repo's rule that a SHOULD gates `done` as a MUST does: the first implementation walked history once per unmapped scenario, where one walk answers every slug. It now walks once, stops early once every slug is accounted for, and does no git work at all in the common case.

Verification: 953 tests pass (`--locked --release`), clippy clean on `--all-targets`, `scripts/audit/run-all.sh` exit 0, markdownlint clean over 420 files, and the new rule swept across all 47 `done` specs producing zero findings — matching the prediction exactly.

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
