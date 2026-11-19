---
section: "Behavior"
---

# A-merge-carries-each-keys-whole-value

## Context

`relocate-audit-records` merges a `review:` or `analyze:` block into a `review.md` or `analysis.md` that already exists, and the block's own lines come from `spec.md`. A key's value is not always on its own line: `waivers:`, `decisions:`, `reviewed-digest:` and `criteria-verdicts:` hold a list or map on the lines indented beneath the key, and a block scalar holds its text there too. A merge that moves only the key line empties such a key while reporting success.

## Behavior

A key is moved, kept, renamed or dropped together with the lines that belong to its value: the indented lines beneath it, a sequence written at the key's own indent, and the blank lines between them. A key the block does not override keeps its lines, in order, in the merged artifact. A key the block overrides is replaced by the block's value whole, and the key is reported in `disagreements` when its whole value differs, indented lines included; a difference of indentation alone is not one. A key renamed by the legacy-spelling fold keeps its lines under the new spelling. A blank line between two lines of a `review:` or `analyze:` block does not end the block: every line of the block is relocated and none remains in `spec.md`. Input with no multi-line key and no blank line inside a block merges as it did before.

## Edge Cases

- **A one-line flow collection** (`key: [a, b]`) has no indented lines and is carried on its key line.
- **A multi-line flow collection** is grouped like any other value: its indented continuation lines belong to its key, and are carried, replaced or dropped with it; a continuation at column zero is not grouped, and stays in order when its key is kept. The record is written in block style, so none is expected.
- **A blank line between two top-level keys** is dropped, as before.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
