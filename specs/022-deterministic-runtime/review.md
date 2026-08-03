---
spec: 022-deterministic-runtime
reviewed-at: 2026-08-03T15:28:35Z
reviewed-against: 9a9c38b3e8f2033034756ef170ec69a99bcdb858
diff-base: 2f226b5805d32ec2c2db23b94438519af7255dee
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Reviews the correction to `review-staleness-gate` — the staleness scope moving from the plan's Affected Files to the durable contracts `scenarios/*.md` and `data-model.md`. **0 MUST, 0 SHOULD — not blocking.** The change is a net simplification: the gate no longer parses plans at all, so `compute_review_scope::read_plan_affected` reverts to private, the Affected-Files matcher (`in_scope`) is deleted, `Vec` + `contains` + `sort` collapses to a `BTreeSet` that gives dedup and ordering for free, and the new `is_durable_contract` predicate mirrors `scripts/audit/review-freshness.sh` line for line. Reuse improves rather than regresses — one definition of stale now serves both enforcement points, where the prior version had two. Security is inert: a read-only git tree comparison over paths derived from committed history, no caller input, no spawn. `QUAL-CLAIM-001` is unchanged and still satisfied — the check fails open on a missing repo or an unresolvable sha, and cannot report a verdict it did not earn. The correction itself is the finding worth recording: the prior scope was reasoned about rather than measured, and reasoning produced a gate that blocked 34 of 48 specs — the precise failure its own doc comment warned against, using a rule I had already measured and rejected for Family 19 hours earlier. Both the scenario and the CHANGELOG record the numbers so the next reader inherits the evidence rather than the conclusion. Verified at this HEAD: 864 lib tests plus 11 suites green with four retargeted gate tests (stale contract, bookkeeping churn, review bookkeeping, unresolvable sha); `clippy -D warnings` clean — it caught a case-sensitive extension comparison that `cargo test` does not compile with, now using the codebase's `eq_ignore_ascii_case` convention; `fmt` clean; markdownlint clean across 390 files; the 19-family self-audit exit 0 with Family 19 wired; and `check-review-gate` passing on all 48 specs, measured, where the prior rule failed 34.

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

## Skipped passes

*None.*
