---
section: "The primitive library"
---

# A-review-states-what-it-read

## Context

`write-review` took `findings` as an argument and recorded no denominator. A review whose five passes read every file in scope and found nothing, and a review whose passes never ran at all, produced **byte-identical** records: the same `0/0/0` counts, the same `reviewed-digest`, the same `blocking: false`. Nothing downstream could separate them — not `check-review-gate`, not Family 19's freshness resolution, not Family 31's agreement check, not a reader.

`write-analysis` has not had this hole since spec 047. It requires `unexamined`, and `analyze.md` argues the case in as many words: *"A record carrying only the finding counts would collapse exactly that distinction into the reassuring reading, inside the artifact a later gate trusts, which is the worst possible place for `QUAL-CLAIM-001`."* That reasoning was written for the analyze half and never carried across to the review half, so the rule existed and the machinery did not enforce it.

On 2026-09-12 the gap was exercised rather than theorised. Two reviews were recorded for `017-derive-dont-ask` and `048-govern-acquired-runtime` by calling `compute-review-scope`, `process-waivers` and `write-review` directly — skipping `discover-rule-files` and all five passes — with `0/0/0` counts under a Summary reading *"Reviewed … across all five dimensions"*. Both records were false and both passed every gate. Re-running the reviews properly over the same scope found fifteen defects, three of them `QUAL-CLAIM-001` violations in shipped adopter CI, including a review-blocking gate that enumerated nothing on a configured spec root and exited 0. The operator caught it from how fast the command returned, not from any artifact.

## Behavior

`write-review` records a numerator and a denominator, and the two come from different places on purpose.

- **`scope` is derived by the primitive**, never accepted as an argument — the same discipline that already derives `blocking`, `reviewed-digest`, `inbox-standing`, and the Unexamined-governance section. It resolves `compute-review-scope` against the run's own `diff-base`, so the number is the scope the review was told to cover rather than whatever that base resolves to later. A caller cannot shrink the subject to match what it happened to read.
- **`examined` is the caller's claim** about how many in-scope files the passes read. It is the one number only the host knows, exactly as `unexamined` is for `write-analysis`.
- Both are written to `review.md` frontmatter **and** the spec's `review:` block, so the two records have two sides to compare.
- An **unstated** `examined` is recorded as absent, never as a computed zero. A claim never made and a claim that came back empty are different facts, and the record says which — the same distinction §grounding draws between *could not examine* and *examined and found nothing*.

`check-review-agreement` (`/{project}:audit` Family 31) reads them:

- `examined-nothing` — `examined: 0` over a `scope` greater than zero. The record states outright that the passes read nothing, so its counts describe nothing.
- `examined-unstated` — no `examined` on a record that carries a `scope`. Reported distinctly, because the repair differs: one run made an empty claim, the other made none.
- `examined` and `scope` join the paired-field set, so a hand-edit to one of the two records is caught like any other divergence.

**What this does not do, stated rather than implied.** It cannot prove the passes ran. A caller can overstate `examined` as easily as omit it, exactly as it can understate `unexamined`. What it buys is that the claim becomes explicit and checkable instead of invisible — which is the bar `QUAL-CLAIM-001` sets and the same bar the analyze half already cleared, no higher.

## Edge Cases

- **An empty scope.** `empty_scope` short-circuits the resolution to zero, so the "nothing to review yet" report records `scope: 0`, and `examined: 0` beside it is coherent rather than a finding. Family 31 only fires when `scope` exceeds zero.
- **A record written before this field existed.** It carries neither `examined` nor `scope`, so Family 31 has no denominator to judge against and stays silent. The exemption is bounded and self-correcting rather than a permanent hole: the next `/{project}:review` of that spec writes a `scope`, and from then on an absent `examined` is reported. This is the same shape as the `analyze-state-drift` grandfather rule, and deliberately not a per-spec date.
- **An unresolvable window.** A spec whose directory has no commit yet has no scope to compute. The resolution yields zero rather than failing the write — the findings the run computed still have to land, for the reason the governance section is rendered rather than propagated.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
