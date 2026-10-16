---
section: "The review record"
---

# An-empty-scope-flag-does-not-decide-the-denominator

## Context

`write-review` derives `scope` itself rather than accepting it, so a caller cannot shrink the denominator to match whatever it happened to read. One branch defeated that: `resolve_scope_size` opened with `if args.empty_scope { return 0; }`, so the flag decided the denominator instead of the derivation.

The realistic way to reach that branch wrongly is not carelessness. `compute-review-scope`'s payload scales with history, and on a wide diff base it exceeds the MCP output cap — measured 2026-09-14, **8 of 54** specs' payloads exceed 100KB, the largest 159,303 bytes, while every other primitive's result sits under 25KB. An over-cap MCP call returns a hard error with **no result at all**, which cannot be mistaken for content but reads very naturally as *nothing in scope* — and `framework/commands/review.md` step 1 says an empty scope jumps straight to `write-review`.

The record that produced was `0/0/0`, `scope: 0`, `examined` absent, `blocking: false`: self-consistent, and invisible to every gate this project has. Family 31 could not see it because both of its `examined` arms guard on `total > 0`, so `scope: 0` is unreachable at any `examined` value. Family 19 could not, because the digest is valid. `check-review-gate` could not, because the review is non-blocking. That is `QUAL-CLAIM-001` inside the artifact the backfill campaign existed to make trustworthy, arriving through the one door `examined` and `scope` do not watch.

## Behavior

**The denominator is always derived.** `resolve_scope_size` no longer consults `empty-scope`; it resolves `compute-review-scope` against the run's own `diff-base` in every case. A genuinely empty window still resolves to `0`, so the honest empty review is unchanged and byte-identical to before. A *false* empty-scope run now carries the real `scope: N` beside a zero `examined` — which was precisely the shape Family 31 reported as `examined-nothing`, so the hole closed with **no new check written**. Spec 057 has since retired Family 31 along with the record's second home, so the shape is still recorded and nothing mechanical reads it: what this scenario fixed — the denominator being derived rather than supplied — holds exactly, and the *reporting* of the resulting divergence is now a reader's job at the completion gate.

**The flag keeps its rendering job.** `empty-scope` still selects the "Review scope is empty — no implementation files in scope" Summary. That is a genuine presentation choice and is untouched; what it may no longer do is decide a number that is supposed to be evidence.

## Edge Cases

- A spec with no recorded `in-progress` transition derives an empty window, so `scope` resolves to `0` on its own merits. The flag is not what produces the zero, which is the whole distinction.
- The `empty-scope` Summary and a non-zero derived `scope` can now co-occur. That pairing is not a contradiction to be smoothed over — it is the report saying, legibly, that the caller claimed an empty scope while the repository disagreed.
- `compute-review-scope` returning an `Err` still collapses to `0` through `map_or`, so the same false-zero remains reachable through the error door rather than the flag door. Deliberately not closed here — see the Resolved Question below.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Should a `compute-review-scope` that *errors* also stop collapsing to `scope: 0`?**
  Deferred, with a condition — not left open, and named in the Edge Cases above so it is
  not mistaken for an oversight. The same false-zero is reachable through
  `map_or(0, …)` when the derivation errors rather than when the flag is passed, and by
  the argument this scenario makes it is the same defect.

  It is not fixed here because closing it honestly means overturning a stance this
  primitive states deliberately one screen up: `render_unexamined_governance` renders an
  unexamined state rather than propagating it, *"Propagating it would mean an unrelated
  typo in the project config produced no `review.md` at all, losing the findings this run
  just computed."* That reasoning applies unchanged to an errored scope derivation, and
  `scope` is a `u32` with no room to say *unknown* — so the honest fix is a new rendered
  state, not a one-line change, and reversing a recorded design stance inside a batch
  commit is exactly the move [§drift-prevention](../../../framework/constitution.md#drift-prevention)
  warns about.

  **The trigger to revisit:** the first time a `compute-review-scope` error is actually
  observed in a review run, or any change that gives the record a way to express an
  underivable denominator. Until then the exposure is bounded and stated: the measured,
  live path was the flag, and that is the one this scenario closes.
