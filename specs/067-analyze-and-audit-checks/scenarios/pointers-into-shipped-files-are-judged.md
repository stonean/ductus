---
section: "Frontmatter, dependency, anchor and rule checks"
---

# Pointers-into-shipped-files-are-judged

## Context

[071 — Command reference split](../../071-command-reference-split/spec.md) moves each pipeline command's markdown-only reference into its own file, and every step that pointed at "the markdown-only reference below" now names a section and a file: **§Grounding (advisory)** in `.ductus/reference/analyze.md`. Such a line names another document, so under [`anchor-reference-kinds`](anchor-reference-kinds.md) every `§` on it is `qualified`: counted, never judged. A pointer naming a heading the reference file does not have would pass every check this repository runs, which is exactly the dangling reference the move invites.

That scenario rejected resolving qualified references, because it would turn the primitive into a link resolver with a file-loading policy, an error surface for unreadable targets, and a question about what counts as a document root. The **Shared Files** manifest answers all three for the documents it ships, so for those documents the judgment is bounded rather than general. The operator chose this over a section-pointer finding in `check-step-references`, which reads only `## Instructions` and would miss pointers in a command's Context, Scope and Gate sections and in the installer.

## Behavior

A fourth kind, **`cross-document`**, is decided before every other: an explicit pair first, then a line naming exactly one document, a shipped one, both `cross-document`; only then `qualified`, `intra-document` and `markers`, in that order.

- A line that names **exactly one** document other than the markers file, where that document is a destination the `## Shared Files` section of `framework/bootstrap/ductus.md` maps to a source, has each `§` on it judged. The reference resolves when the text after `§` begins with a heading of the mapped source (the intra-document rule's prefix match), a heading of the citing file, or a marker in the markers file. Otherwise it is unresolved and listed in `unresolved`.
- The result carries a `cross-document` count beside `qualified` and `intra-document`.
- A pointer written as an explicit pair, `**§X** in` followed by a shipped document in backticks, is judged on any line, against that document's headings alone: the author has said which document the section belongs to, so neither a local heading nor a marker rescues it. It resolves only when `X` equals one of those headings: the bold marks where the name ends, so a pointer that outlives a shortened heading is unresolved rather than matched by prefix. A command step routinely names `plan.md` or `spec.md` beside its pointer; measured 2026-10-25, 59 of 100 pointers sat on such lines.
- Otherwise a line naming several documents, or one the manifest does not map, stays `qualified`, and so does a line whose mapped source cannot be read. Widening the line rule to "exactly one *shipped* document" was measured and rejected: it would have reported 9 references meant for the other document on their line.
- A heading is an ATX heading line outside a ` ``` ` fenced block and outside an HTML comment, for the citing file and a mapped source alike: the record skeleton in review's reference holds `## Summary` inside a fence, and that is content, not a section a pointer can name.
- Outside a pair, a reference names a heading its text begins with, the heading ending where a word ends: `§Groundingx` names no heading `Grounding`. Nothing marks where a prose reference's name ends, so `§Hook Installation` may still resolve against a heading `Hook`.
- Measured 2026-10-26 over all 823 tracked `.md` files in this repository: exact pairs and skipped fences and comments changed no reference's kind or resolution, the same 159 unresolved before and after at `e6b43796`; the word-boundary rule then changed none either, 157 before and after at `36b3ddb6`, where the review round had fixed two.

The map is read once per call from the `## Shared Files` section alone, a row giving its destination and source as one backticked span each. It is keyed by the destination exactly as written there, and a document on the citing line matches when its path, a backticked path or a markdown link target with any `#` fragment stripped, equals a destination. Where `framework/bootstrap/ductus.md` is absent, which is every adopter, the map is empty and no reference is judged this way. That is the same discriminator `check-artifacts`' adopter destinations use.

**The marker fallback is required, not lenient.** Measured 2026-10-25 before the move: the framework and spec corpus held 16 references on lines naming exactly one mapped document, and every one was a constitution marker beside a path such as `specs/inbox.md`. A rule reading only the mapped source's headings would have reported all 16. The citing-file fallback lets a pointer line also cite a section of its own file, and no reference needs it today: measured 2026-10-26, each of the 36 line-rule `cross-document` references under `framework/`, `specs/` and `.ductus/` resolved by a constitution marker, as do the 44 in the generated `.claude/` and `.pi/` copies.

## Edge Cases

- The markers file named on the same line does not count as the one document, the same as for `qualified`.
- A heading that starts with a digit cannot be named by a `§` reference, whose grammar starts with a letter; 071's reference files carry no numbered headings for that reason.
- A shortened pointer (`**§Rules**` for the heading `Rules (blocking and advisory)`) is unresolved, which is the safe direction: a pair must equal a heading, and the line rule asks whether the text after `§` begins with a heading, never whether a heading begins with the text. A pair longer than its heading is unresolved too; the line rule cannot tell, since nothing on the line marks where a bare `§` name ends.
- Family 39 reports the `cross-document` count on stderr with the others.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
