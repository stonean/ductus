---
section: "Follow-on scenarios"
---

# Block-element-scanner

## Context

The runtime's markdown walkers read documents a line at a time. `SkipScanner` supplies fence and HTML-comment awareness, and `inline_code_spans` implements the CommonMark equal-length-backtick-run rule, but nothing yields a document's **block-level elements** — the list item, table row, or paragraph that a single authorial claim occupies. A check that must decide "is this claim about that link?" needs that unit, and the alternative — splitting sentences — is a project of its own that the repo has deliberately avoided.

[045 — Decision-state drift detection](../../045-decision-state-drift-detection/spec.md) owns the requirement; this scenario carries the runtime work, per that spec's Implementation ownership split. It is a prerequisite for [link-adjacent-drift-family](link-adjacent-drift-family.md), which consumes the blocks this scanner yields.

## Behavior

**A block splitter.** A `pub(crate)` helper walks markdown content and yields each block-level element with the 1-based line number it starts on. Three kinds, decided in this order:

- a line whose trimmed form starts with `|` is a **table row**, and is a block by itself;
- a line matching the shared bullet grammar opens a **list item**, which extends through the indented continuation lines that follow it;
- any other maximal run of non-blank, non-heading lines is a **paragraph**.

Headings are structure rather than claims and are never yielded.

**The exempt contexts come from two places, not one.** Every line passes through `SkipScanner` before the splitter classifies it, so a line inside a fenced code block or an HTML comment never reaches a block. The blockquote exemption is applied by the splitter: a line whose trimmed form starts with `>` is dropped.

`SkipScanner` itself is **unchanged**. It is shared by `read-tasks`, `mark-task`, `prune-tasks`, and the task-number walkers, so teaching it a fourth skip region would silently change how every one of them reads a quoted task line. The narrower change is the correct one, and the two exemptions land where each belongs.

**One code-span computation, shared.** `inline_code_spans` is promoted from private to `pub(crate)` with no change to its behavior, so a consumer scanning block text for a term outside code font and a consumer reading path-like content inside code font compute spans the same way rather than each rolling its own.

## Edge Cases

- An unterminated fence or comment running to EOF: `SkipScanner` reports `in_region`, and everything after the opener stays skipped. No block is yielded from an unclosed region.
- A table row inside a list item is classified as a table row — the `|` test runs first, and a row is the finer-grained claim.
- A bullet opening immediately after a paragraph with no blank line between them ends the paragraph and starts a list item.
- A nested bullet at deeper indentation opens its own list item rather than joining its parent, so a claim in a sub-bullet is scoped to that sub-bullet.
- A blockquote interrupting a paragraph ends it; the quoted lines are dropped and the text after resumes as a new paragraph.
- A document with no block-level content at all — only headings, blank lines, and fenced code — yields nothing, which is a clean empty result rather than an error.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
