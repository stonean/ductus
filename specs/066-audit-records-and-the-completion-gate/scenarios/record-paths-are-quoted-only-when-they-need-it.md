---
section: "Record freshness"
---

# Record-paths-are-quoted-only-when-they-need-it

## Context

`reviewed-digest` and `analyzed-digest` are maps keyed by each subject's path, and `reviewed-unreadable` and `analyzed-unreadable` are lists of paths. `write-review` and `write-analysis` wrote each path bare, though every other string in both records goes through `yaml_string`. A scenario or data-model file whose name holds a colon followed by a space, or a space followed by `#`, therefore broke the record: `write-review` wrote a `review.md` it could not read again and the spec stayed blocked until the file was edited by hand, and `write-analysis` wrote a key that read back as a shorter path, so the recorded path set never equalled the current one and the analysis read stale on every run.

Quoting every key would rewrite all 76 recorded digest maps, changing `review.md`, which the analysis digest covers, so every spec's analysis would go stale at its next review.

## Behavior

- Both writers render each digest key and each unreadable item through `yaml_string`, so a path is quoted exactly when its plain form would not read back as the same string, and is otherwise written as before.
- A path written bare before is written byte for byte as before, so no stored record changes for an ordinary path and no analysis goes stale on that account.
- The record reads back to the same path set that was written: the next `write-review` accepts the record, and `freshness_of` reads an unchanged subject set as current.
- The path is never flattened or otherwise altered on the way out: a path holding a control character or line break is quoted and escaped, not rewritten to a different name.

## Edge Cases

- A name that reads as a number or boolean in YAML is quoted by the same rule; the writers' known paths end in `.md` or name a directory, so none does today.
- An unreadable item that names a directory (`scenarios`) is written bare.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
