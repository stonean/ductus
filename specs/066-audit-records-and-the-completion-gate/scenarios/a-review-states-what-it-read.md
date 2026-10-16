---
section: "The review record"
---

# A-review-states-what-it-read

## Context

`write-review` took `findings` as an argument and recorded no denominator. A review whose five passes read every file in scope and found nothing, and a review whose passes never ran at all, produced **byte-identical** records: the same `0/0/0` counts, the same `reviewed-digest`, the same `blocking: false`. Nothing downstream could separate them — not `check-review-gate`, not Family 19's freshness resolution, not Family 31's agreement check, not a reader.

`write-analysis` has not had this hole since spec 047. It requires `unexamined`, and `analyze.md` argues the case in as many words: *"A record carrying only the finding counts would collapse exactly that distinction into the reassuring reading, inside the artifact a later gate trusts, which is the worst possible place for `QUAL-CLAIM-001`."* That reasoning was written for the analyze half and never carried across to the review half, so the rule existed and the machinery did not enforce it.

On 2026-09-12 the gap was exercised rather than theorised. Two reviews were recorded for `017-derive-dont-ask` and `048-govern-acquired-runtime` by calling `compute-review-scope`, `process-waivers` and `write-review` directly — skipping `discover-rule-files` and all five passes — with `0/0/0` counts under a Summary reading *"Reviewed … across all five dimensions"*. Both records were false and both passed every gate. Re-running the reviews properly over the same scope found fifteen defects, three of them `QUAL-CLAIM-001` violations in shipped adopter CI, including a review-blocking gate that enumerated nothing on a configured spec root and exited 0. The operator caught it from how fast the command returned, not from any artifact.

## Behavior

`write-review` records a numerator and a denominator, and the two come from different places on purpose.

- **`scope` is derived by the primitive**, never accepted as an argument — the same discipline that already derives `blocking`, `reviewed-digest`, and the Unexamined-governance section. It resolves `compute-review-scope` against the run's own `diff-base`, so the number is the scope the review was told to cover rather than whatever that base resolves to later. A caller cannot shrink the subject to match what it happened to read.
- **`examined` is the caller's claim** about how many in-scope files the passes read. It is the one number only the host knows, exactly as `unexamined` is for `write-analysis`.
- Both are written to `review.md`'s frontmatter.
- An **unstated** `examined` is recorded as absent, never as a computed zero. A claim never made and a claim that came back empty are different facts, and the record says which — the same distinction §grounding draws between *could not examine* and *examined and found nothing*.

No check reads the two fields since spec 057 retired Family 31 with the record's second home; a person reads them at the completion gate.

**What this does not do, stated rather than implied.** It cannot prove the passes ran. A caller can overstate `examined` as easily as omit it, exactly as it can understate `unexamined`. What it buys is that the claim becomes explicit and checkable instead of invisible — which is the bar `QUAL-CLAIM-001` sets and the same bar the analyze half already cleared, no higher.

## Edge Cases

- **An empty scope.** The scope is derived even when `empty-scope` is passed, so a genuinely empty scope records `scope: 0`, and `examined: 0` beside it is coherent rather than a false claim.
- **A record written before this field existed.** It carries neither `examined` nor `scope`, and nothing reports the absence; the next `/{project}:review` of that spec writes a `scope`, and a person reads the fields at the completion gate.
- **An unresolvable window.** A spec whose directory has no commit yet has no scope to compute. The resolution yields zero rather than failing the write — the findings the run computed still have to land, for the reason the governance section is rendered rather than propagated.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
