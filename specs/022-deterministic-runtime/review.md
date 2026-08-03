---
spec: 022-deterministic-runtime
reviewed-at: 2026-08-03T14:47:58Z
reviewed-against: 2f226b5805d32ec2c2db23b94438519af7255dee
diff-base: 52b89a7a55d9c068e0667f5585b2dea8c5d8d900
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 1
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Reviews the `review-staleness-gate` scenario — `ReviewGateBlock::ReviewStale` plus the `pub(crate)` promotion of `read_plan_affected` — and the `/gov:audit` Family 19 script that is its release-time counterpart. **0 MUST, 0 SHOULD — not blocking.** This review exists because the gate it reviews demanded it: committing the change tripped check 5 on 022 (`blocked: review is stale — 7 file(s) … changed since reviewed-against 52b89a7a`), which is the first time in this session that a process step was forced by tooling rather than remembered. Rule files loaded: the backend + cross set. Security is inert — the diff adds a read-only git comparison over paths derived from a committed plan, spawns nothing, and takes no caller input; `validate_no_traversal` on `feature` still runs upstream. `QUAL-CLAIM-001` is the pass that matters here and both halves satisfy it in the same direction: the runtime gate **fails open** on a missing repo, an unresolvable sha, or an absent Affected Files table, and Family 19 reports an unresolvable `reviewed-against` as its own finding rather than passing silently — neither can report a clean verdict it did not earn. Their opposite posture to Family 17 (which must fail *closed*) is stated in both, because the distinction is the load-bearing part: an empty derivation there means checking nothing, here it means blocking nothing. `QUAL-REUSE` is satisfied by promotion rather than duplication — `read_plan_affected` moved to `pub(crate)` instead of a second Affected-Files parser, which is the finding this session already recorded twice against re-implemented parsers. The scoping is the strongest evidence in the change: both wider rules were measured against the corpus and rejected on numbers (Affected Files 42/48, whole spec directory 31/48) before the durable-contract rule shipped at 10/48, verified to catch both `gvrn-v0.26.1` and `gvrn-v0.26.2`. Verified: 864 lib tests plus 11 suites green including four new gate tests (stale, out-of-scope, bookkeeping, fail-open), `clippy -D warnings` and `fmt` clean, shellcheck clean on the new family, markdownlint clean across 390 files, the 18-family audit exit 0, and `check-artifacts` clean on 022 and 026.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- convention: review-freshness — 10 done specs carry a review predating their own durable contracts (scenarios/ or data-model.md). Pre-existing debt, surfaced by the new Family 19 and captured to the inbox rather than left in a session. Clearing them is the precondition for wiring Family 19 into run-all.sh as a hard release gate.

## Skipped passes

*None.*
