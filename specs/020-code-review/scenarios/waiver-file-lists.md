---
section: "Waivers"
---

# Waiver-file-lists

## Context

A waiver's `file` named one path. When a rule fired in several files for the same reason, the record held one entry per file, each repeating the rule, the reason, `waived-at` and `waived-by`, differing only in `file`. An adopter's `review.md` surfaced exactly that. The duplicates could not be merged by hand either: a list in `file` was a non-string value in a known field, so the whole `waivers:` list failed to parse and `/ductus:review` halted.

The one-path anchor was never a decision against lists; no spec weighed one. What it protects is that each file expires on its own, and that a waiver never covers a file nobody judged — [waiver-expiry](waiver-expiry.md), rule 5. A list of explicit paths keeps both, provided each listed path stays its own anchor.

The runtime half — the primitives that read, prune and re-render the list — is 066's [process-waivers-file-lists](../../066-audit-records-and-the-completion-gate/scenarios/process-waivers-file-lists.md).

## Behavior

- A waiver lists its anchors, one or many, each a `(rule, file, site)` triple with its own count ([waiver-expiry](waiver-expiry.md)) under `anchors:` (the data model's Waiver record); a departure waiver lists one repo-relative path or many under `file`. A list means one judgment — one `rule`, `reason`, `waived-at` and `waived-by` — covers each anchor, so the anchors share their other fields.
- Listed paths are matched literally. A pattern such as `src/**/*.ts` is not expanded; it names a path that does not exist and expires as one. A waiver still never covers a file it does not list.
- Apply, expire and retain are decided per anchor, exactly as for a one-path entry. When one anchor expires, only that anchor leaves the entry, with the usual `waiver expired:` line for it ([waiver-expiry](waiver-expiry.md)); the entry's other anchors, its other fields and any adopter-authored fields are kept. The entry is dropped when its last anchor expires.
- The record renders a departure waiver's `file` as a single path when it holds one and as a list when it holds more, so a list pruned to one path — or written by hand with one — reads like any one-path entry. `anchors` is always a list. An entry recorded before sites, with `file` and no `anchors`, is not migrated: each of its paths expires on the next review ([waiver-expiry](waiver-expiry.md), step 7).
- `--waive <rule-id> --reason "<text>"` records one entry per invocation, anchored at every in-scope site the rule fires at in that run, each with the number of findings there. A later `--waive` for the same rule is a new entry, never an append to an existing one, because an entry's `waived-at` and `waived-by` attest to one judgment made once.
- Duplicate detection runs over anchors in record order — entries in list order, each entry's anchors in order. The first claim on a `(rule, file, site)` anchor applies. Each later claim emits the existing `duplicate waiver:` warning naming the anchor and `entry [N] ignored`, and only that anchor of entry N is ignored; its other anchors are classified as usual. Duplicates are kept until their anchor expires, when every claim on it is pruned together.

## Edge Cases

- **Empty list or blank anchor.** `anchors: []`, or an anchor with a blank `file` or `site` or a `covers` that is not a positive integer, is malformed and reported as `missing 'anchors'`; the entry is skipped and kept, like any malformed entry. A departure waiver's `file: []`, or a list holding a blank path, is reported the same way.
- **A list item of the wrong shape** — a `file` item that is not a string, an `anchors` item that is not a mapping — fails the `waivers:` parse, as a known field of the wrong type does. It is an error, never an empty list.
- **A malformed entry that lists an expired anchor.** Pruning compares anchors only, so the entry loses that anchor and keeps the rest; it is dropped when none remains. An entry missing `rule` names no anchor and is never pruned.
- **The same anchor twice in one entry.** The second occurrence is a duplicate claim and warns; both are pruned when the anchor expires.
- **Merging existing duplicates by hand.** An operator may collapse entries that share a rule, reason and author into one list. Which `waived-at` survives is the operator's call; the framework never merges entries itself.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
