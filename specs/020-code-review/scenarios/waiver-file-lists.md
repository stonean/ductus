---
section: "Waivers"
---

# Waiver-file-lists

## Context

A waiver's `file` named one path. When a rule fired in several files for the same reason, the record held one entry per file, each repeating the rule, the reason, `waived-at` and `waived-by`, differing only in `file`. An adopter's `review.md` surfaced exactly that. The duplicates could not be merged by hand either: a list in `file` was a non-string value in a known field, so the whole `waivers:` list failed to parse and `/ductus:review` halted.

The one-path anchor was never a decision against lists; no spec weighed one. What it protects is that each file expires on its own, and that a waiver never covers a file nobody judged — [waiver-expiry](waiver-expiry.md), rule 5. A list of explicit paths keeps both, provided each listed path stays its own anchor.

The runtime half — the primitives that read, prune and re-render the list — is 066's [process-waivers-file-lists](../../066-audit-records-and-the-completion-gate/scenarios/process-waivers-file-lists.md).

## Behavior

- A waiver's `file` is one repo-relative path or a list of them. A list means one judgment — one `rule`, `reason`, `waived-at` and `waived-by` — covers each path. The anchor is still the `(rule, file)` pair: an entry listing N paths is N anchors that share their other fields.
- Listed paths are matched literally, as a single `file` is. A pattern such as `src/**/*.ts` is not expanded; it names a path that does not exist and expires as one. A waiver still never covers a file it does not list.
- Apply, expire and retain are decided per anchor, exactly as for a one-path entry. When one listed path expires, only that path leaves the list, with the usual `waiver expired: rule {rule-id} at {file} ({reason})` line for it; the entry's other paths, its other fields and any adopter-authored fields are kept. The entry is dropped when its last path expires.
- The record renders `file` as a single path when an entry holds one and as a list when it holds more, so a list pruned to one path — or written by hand with one — reads like any one-path entry. Existing one-path records are unchanged and need no migration.
- `--waive <rule-id> --reason "<text>"` records one entry per invocation, and its `file` lists every in-scope file the rule fires at in that run — a single path when there is one. A later `--waive` for the same rule is a new entry, never an append to an existing one, because an entry's `waived-at` and `waived-by` attest to one judgment made once.
- Duplicate detection runs over anchors in record order — entries in list order, each entry's paths in order. The first claim on a `(rule, file)` pair applies. Each later claim emits the existing `duplicate waiver: rule {rule-id} at {file} — entry [N] ignored` warning, and only that pair of entry N is ignored; its other paths are classified as usual. Duplicates are kept until their pair expires, when every claim on it is pruned together.

## Edge Cases

- **Empty list or blank path.** `file: []`, or a list holding a blank path, is malformed and reported as `missing 'file'`; the entry is skipped and kept, like any malformed entry.
- **A list item that is not a string** — a mapping, a nested list — fails the `waivers:` parse, as a non-string known field does today. It is an error, never an empty list.
- **A malformed entry that lists an expired anchor.** Pruning compares anchors only, so the entry loses that path and keeps the rest; it is dropped when no path remains. An entry missing `rule` names no anchor and is never pruned.
- **The same path twice in one entry.** The second occurrence is a duplicate claim and warns; both are pruned when the pair expires.
- **Merging existing duplicates by hand.** An operator may collapse entries that share a rule, reason and author into one list. Which `waived-at` survives is the operator's call; the framework never merges entries itself.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
