---
section: "Fold-back on merge"
---

# Rewrites-preserve-line-endings

## Context

Fold-back re-points every inbound pointer to a retiring directory by rewriting the `.md` files that name it (`rewrite-spec-links`). The rewrite is line-wise: `content.lines()` splits the file, each line is examined, and `push_line` re-emits it followed by a bare `\n`.

`str::lines()` strips a trailing `\r` from every line it yields. So on a checkout whose files carry CRLF endings, a rewrite that changes one link re-emits **every** line with LF — converting the whole file, silently, as a side effect of moving a pointer.

Sibling primitives already do this correctly, each having solved it independently:

- `create_feature::stamp_fold_target` detects `\r\n` in the template and joins with the ending it found (`runtime/src/primitives/create_feature.rs:206`).
- `derive_references` picks its `line_ending` the same way (`runtime/src/primitives/derive_references.rs:398`).

And `check_stuck` carries CRLF regression tests recording that a hand-rolled splitter missing the `\r\n` close fence was a real defect (`runtime/src/primitives/check_stuck.rs:215`, `:351`). The convention is established. `rewrite-spec-links` is the writer that was *reported* as departing from it; it is not the only one (see Behavior).

The runtime's CI runs on three platforms, so this is not hypothetical — but it is invisible to a repository whose files are already LF, which is why it survived review of the primitive itself and was found only by comparing it against its siblings.

## Behavior

A primitive that rewrites an existing text file preserves that file's line endings. A file read as CRLF is written back as CRLF; a file read as LF is written back as LF. The rewrite's diff is confined to the lines whose content actually changed.

The rule is a property of *rewriting*, not of `rewrite-spec-links`. Measured across the primitives that reassemble a file they did not create: **three carry the detection** — `derive_dependencies`, `derive_references`, and `create_feature::stamp_fold_target`, three independent copies of the same few lines — and **seven do not**:

| Writer | What a CRLF file becomes |
| --- | --- |
| `rewrite_spec_links` | whole file → LF |
| `remove_inbox_item` | whole file → LF |
| `append_question` | whole file → LF |
| `prune_tasks` | whole file → LF |
| `append_task` (phased insert; `stitch` on append) | whole file → LF; an appended block → **mixed** |
| `write_review` (frontmatter splice) | frontmatter → LF, body preserved → **mixed** |
| `invalidate_review` | **mixed**, inherited from the splice it reuses |

The two **mixed** outcomes are the sharper failure. A clean conversion is at least uniform and shows up as one large diff; a partial one leaves a file whose halves disagree, which no subsequent reader can distinguish from a hand-edit. Every spec reviewed on a CRLF checkout is in that state today.

So the detection belongs in **one** shared place that every writer calls, and the three existing copies are retired into it. Fixing only the writer that was reported would leave six traps set and a helper with one caller — which is the same duplication one level up.

A file with mixed endings has no correct answer to preserve, so the dominant ending governs and the file is normalized to it. That is a deliberate choice rather than an oversight: mixed endings are already a defect in the file, and preserving them line-by-line would encode it permanently.

## Edge Cases

- **A file with no trailing newline** keeps its final byte unchanged — the existing `ends_with_newline` handling is correct and must survive the refactor.
- **An empty file** rewrites to an empty file; there is no line to infer an ending from and nothing to write.
- **A single-line file with no newline at all** carries no ending to detect. It takes the platform-neutral default (LF), since no evidence points either way and inventing CRLF would be the more surprising answer.
- **A file whose only change is inside frontmatter** (a `folds-into:` re-point) is subject to the same rule — the frontmatter and body halves of the rewrite share one writer, so they cannot disagree about endings.
- **A rewrite that changes nothing** still must not write. The existing `count == 0` early-return already guarantees this, and it is what keeps a CRLF file untouched when no link names the retiring directory.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
