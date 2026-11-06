---
section: "The completion gate"
---

# The-fix-round-loop-is-recorded-and-gated

## Context

[078 — Review fix rounds are re-read within a bound](../../078-review-fix-rounds-are-re-read-within-a-bound/spec.md) adds a fix-round loop to `/{project}:review`: each round of fixes is re-read by an independent agent, verified by a diff check against the previous re-read's prescriptions, or exempt by a path rule. Its outcome must reach `review.md`, and a round that needed a re-read and has none, with no waiver, must hold `done`. The record and the gate are this spec's, so the change lands here. The shapes are in [078's data model](../../078-review-fix-rounds-are-re-read-within-a-bound/data-model.md), **`write-review` — the loop's argument and record fields** and **`check-review-gate` — the fix-round check**.

## Behavior

- `write-review` takes an optional `fix-rounds` list, one entry per round, each a `check` of `exempt`, `re-read`, `diff-checked` or `not-re-read`, with a `result` on a re-read and an optional waiver `reason` on a round not re-read.
- From the list it derives `fix-rounds`, `re-reads`, `diff-checked`, `not-re-read`, `budget-stop` and `round-waivers` into `review.md`'s frontmatter, and renders them in the generated Summary, naming each exempt and each waived round. No count is accepted from the caller.
- An absent list writes none of those fields; an empty list writes them at zero.
- When `.ductus/config.toml`'s `[review] re-reads-off` is set, `write-review` reads it itself and waives each un-re-read round without a reason with the switch's reason, naming the config file as `waived-by`.
- `check-review-gate` gains a check after the review's `undispositioned-findings`: `fix-round-loop-open` while `fix-rounds-open` is present (spec 079), then `fix-round-not-re-read`, which blocks while `not-re-read` exceeds the number of `round-waivers`, naming the count, with guidance to re-run `/{project}:review`. A record without the loop fields passes and carries guidance that it does not record whether its fixes were re-read.
- `framework/constitution.md`'s audit-record table and `framework/reference/review.md`'s field semantics name the new fields; the constitution's addition is matched by a cut, since it sits at its 50,000-byte ceiling.

## Edge Cases

- A `result` on any check but `re-read`, a missing `result` on a re-read, or a `reason` on any check but `not-re-read` is refused before anything is written.
- A waiver `reason` with no `decided-by` is refused, as a new waiver of a violation is.
- A malformed `re-reads-off` halts `write-review`, naming the setting, the value and the file.
- A hand-edited record with fewer `round-waivers` than `not-re-read` blocks, whatever its other counts say.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
