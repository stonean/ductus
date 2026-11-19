---
section: "Edge Cases"
---

# An-open-fenced-block-or-comment-is-not-examined

## Context

`prune-tasks` and `prune-plan` read a file's structure outside fenced blocks and HTML comments. A fence or `<!--` with no closer makes every later line part of that region, so every later heading is absorbed into the block that holds the opener, and removing that block removes them too: a pending task after a spent one, or a design-record section after a section outside the record.

## Behavior

A `tasks.md` or `plan.md` that ends inside an open fenced block or HTML comment is reported, never read as clean. The result carries `unexamined-tail` with the 1-based `from-line` of the opener in the whole file and a `cause` of `open-fence` or `open-comment`, and says the lines from there on were not examined. An apply of that file writes nothing, in either mode of `prune-tasks` and for any `remove` list of `prune-plan`, and reports `applied` false, until the region is closed. Under `--all` such a spec is listed whatever else it has to reduce, with its tail, and counted in `unexamined-count` beside `features-count`; every other spec is still reduced. A file that ends outside every region reports nothing, and its result carries no new key. How a fence closes is the shared scanner's rule; this behavior depends only on whether a region is open at the end of the file.

## Edge Cases

- **The opener is in a section prune would keep.** The tail is still reported and the apply still writes nothing: the lines after it are unread, so no section after it can be judged spent or outside the record.
- **A fence that closes.** No tail, and the reduction is the one it always was.
- **`--reset`.** Held back like any apply, since a reset discards sections the tail hid.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
