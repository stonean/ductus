---
section: "The review record"
---

# An-empty-scope-flag-does-not-decide-the-denominator

## Context

`write-review` derives `scope` itself rather than accepting it, so a caller cannot shrink the denominator to match whatever it happened to read. One branch defeated that: `resolve_scope_size` opened with `if args.empty_scope { return 0; }`, so the flag decided the denominator instead of the derivation.

The realistic way to reach that branch wrongly is not carelessness. `compute-review-scope`'s payload scaled with history, and on a wide diff base it exceeded the MCP output cap until `scope-results-have-a-bounded-size` bounded it — measured 2026-09-14, **8 of 54** specs' payloads exceed 100KB, the largest 159,303 bytes, while every other primitive's result sits under 25KB. An over-cap MCP call returns a hard error with **no result at all**, which cannot be mistaken for content but reads very naturally as *nothing in scope* — and `framework/commands/review.md` step 1 says an empty scope jumps straight to `write-review`.

The record that produced was `0/0/0`, `scope: 0`, `examined` absent, `blocking: false`: self-consistent, and invisible to every gate this project has. Family 31 could not see it because both of its `examined` arms guard on `total > 0`, so `scope: 0` is unreachable at any `examined` value. Family 19 could not, because the digest is valid. `check-review-gate` could not, because the review is non-blocking. That is `QUAL-CLAIM-001` inside the artifact the backfill campaign existed to make trustworthy, arriving through the one door `examined` and `scope` do not watch.

## Behavior

**The denominator is always derived.** `resolve_scope` no longer consults `empty-scope`; it resolves `compute-review-scope` against the run's own `diff-base` in every case but an empty base, a spec that never entered `in-progress`, which is `0`. A genuinely empty window still resolves to `0`, so the honest empty review is unchanged and byte-identical to before. A *false* empty-scope run now carries the real `scope: N` beside a zero `examined` — which was precisely the shape Family 31 reported as `examined-nothing`, so the hole closed with **no new check written**. Spec 057 has since retired Family 31 along with the record's second home, so the shape is still recorded and nothing mechanical reads it: what this scenario fixed — the denominator being derived rather than supplied — holds exactly, and the *reporting* of the resulting divergence is now a reader's job at the completion gate.

**The flag keeps its rendering job.** `empty-scope` still selects the "Review scope is empty" Summary. That is a genuine presentation choice and is untouched; what it may no longer do is decide a number that is supposed to be evidence.

## Edge Cases

- A spec with no recorded `in-progress` transition derives an empty window, so `scope` resolves to `0` on its own merits. The flag is not what produces the zero, which is the whole distinction.
- The `empty-scope` Summary and a non-zero derived `scope` can now co-occur. That pairing is not a contradiction to be smoothed over — it is the report saying, legibly, that the caller claimed an empty scope while the repository disagreed.
- `compute-review-scope` returning an `Err` no longer collapses to `0`: `write-review` refuses a `diff-base` no scope can be derived from, naming it, and writes nothing, so the false-zero is closed at the error door too. See the Resolved Question below.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Should a `compute-review-scope` that *errors* also stop collapsing to `scope: 0`?**
  Yes, by refusing the write (owner, 2026-11-07, 079's task 20). It had been deferred
  until an error was observed in a review run, and one was at `dcc1e0d4`: a mistyped
  full sha recorded `examined: 8` beside `scope: 0`. `write-review` now refuses a
  `diff-base` no scope can be derived from, and only an empty one, a spec that never
  entered `in-progress`, records `scope: 0`. The stance this overturns, rendering an
  unexamined state rather than propagating it, was reasoned from the governance
  registry, where an unrelated typo in the project config would lose the run's findings.
  Here the error is in the run's own argument, so the host still holds its findings and
  writes them again over the base `compute-review-scope` returned, and on a shallow clone
  `compute-review-scope` stops the run before a finding exists. Rejected: a rendered
  unknown-scope state, which adds a field to the constitution's review-record table at
  49,988 of its 50,000-byte ceiling for no behavior refusing lacks.
