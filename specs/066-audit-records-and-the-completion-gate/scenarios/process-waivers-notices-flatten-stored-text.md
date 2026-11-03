---
section: "The review record"
---

# Process-waivers-notices-flatten-stored-text

## Context

The review of [076 — Waived and deferred findings are distinct dispositions](../../076-waived-and-deferred-findings-are-distinct-dispositions/spec.md) found that `process-waivers` prints a stored waiver's text as written. Its duplicate, retained and expired notices interpolate the `rule`, `file` and `reason` read from `review.md`'s `waivers:` list, and `/{project}:review` emits each notice verbatim. A hand-edited waiver can carry a control character in any of the three, and `write-review` keeps it across rewrites, escaped, so it reaches the operator raw. Since 076, `process-decisions`' notices and `check-review-gate`'s held-by-deferral suffix flatten every stored text they print; `process-waivers` is the remaining reader of a stored record that does not.

## Behavior

- Each `process-waivers` notice prints a stored waiver's `rule`, `file` and `reason` through `flatten_line`, so no control character in a stored waiver reaches a notice.
- Classification is unchanged: a waiver applies, expires, is retained or is a duplicate on its stored values as written. Only what a notice prints is flattened.

## Edge Cases

- A waiver whose fields hold no control character and no surrounding whitespace prints exactly as before.
- A tab is kept, as `flatten_line` keeps it in every other report.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
