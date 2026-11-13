---
section: "The review record"
---

# Process-decisions-results-have-a-bounded-size

## Context

`process-decisions` returns one list per outcome and its `notices` (`ductus process-decisions --help`), one entry per stored decision the record holds. Each grows with the record's `decisions:` list, and no artifact commits to a maximum size or to pagination, which `BE-PAYLOAD-001` requires of a response that returns a collection. [058 — Findings route at discovery](../../058-findings-route-at-discovery/spec.md) introduced the first three lists, and the analysis of [076 — Waived and deferred findings are distinct dispositions](../../076-waived-and-deferred-findings-are-distinct-dispositions/spec.md) found the gap when 076 added `lapsed`. The owner deferred the finding here (2026-11-03), beside the bound `scope-results-have-a-bounded-size` gives `compute-review-scope` and `diff-cross-spec`, so one mechanism covers the audit-record primitives' result lists.

## Behavior

- **Every list in `process-decisions`' result has a stated maximum size**, stated in `data-model.md` beside the result's shape, by the mechanism `scope-results-have-a-bounded-size` settles.
- **A bound never drops a decision the caller acts on.** The host passes `lapsed` and `expired` to the writer as `expired-decisions` and asks again about each lapsed finding, so a result past the bound gives the caller a way to reach every entry, and never reads as a short complete list.

## Edge Cases

- **A record whose decisions fit.** The result is returned inline, but for any retained decision: the caller acts on none, so whenever one is retained the `retained` list and each one's own notice are in the file `lists-file` names, and `notices` carries one line per reason instead (task 22).

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
