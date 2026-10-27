---
section: "The completion gate"
---

# Only-gating-findings-hold-done

## Context

[058 — Findings route at discovery](../../058-findings-route-at-discovery/spec.md)'s `findings-cost-in-proportion-to-risk` narrowed which findings hold `done` to the gating ones: unwaived review MUST and SHOULD violations, and analyze hard-fail and blocking findings. The gate's last check blocked while either record's `dispositions.undispositioned` was above zero, and that map counts every tier, so an undecided advisory analyze finding or review observation held `done` as firmly as a MUST.

## Behavior

- **The last check reads the gating counts, never the map's `undispositioned`.** `check_review_gate::disposition_block` sums the review record's `must-violations` and `should-violations` and the analysis's `hard-fail` and `blocking-findings`, and blocks on any record whose sum is above zero with `blocked: {N} outstanding gating finding(s) — {file}: {n}, …`. Its guidance names fixing each one, or waiving a review SHOULD with its rationale.
- **In practice it is where an unwaived SHOULD binds.** A MUST sets the review record's `blocking`, and a hard-fail or blocking finding sets the analysis's, so each of those blocks at an earlier check; only `should-violations` has no earlier one.
- **The block keeps its wire name.** `blocked-by` stays `undispositioned-findings`, so a host matching on it keeps working.
- **Every other check is unchanged**, the map-less record's `record-predates-dispositions` included: the map is still written, and its absence still blocks an `in-progress` spec.

## Edge Cases

- **A record whose `dispositions:` map counts undispositioned findings and has no gating count** passes the check, including one written before this change.
- **A waived SHOULD** is excluded from `should-violations` by `write-review`, so it does not hold `done`.
- **A spec with no `analysis.md`** never reaches this check: `not-analyzed` blocks first.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
