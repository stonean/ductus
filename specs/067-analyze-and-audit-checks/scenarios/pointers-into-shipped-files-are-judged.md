---
section: "Frontmatter, dependency, anchor and rule checks"
---

# Pointers-into-shipped-files-are-judged

## Context

[071 — Command reference split](../../071-command-reference-split/spec.md) moves each pipeline command's markdown-only reference into its own file, and every step that pointed at "the markdown-only reference below" now names a section and a file: **§Grounding (advisory)** in `.ductus/reference/analyze.md`. Such a line names another document, so under [`anchor-reference-kinds`](anchor-reference-kinds.md) every `§` on it is `qualified`: counted, never judged. A pointer naming a heading the reference file does not have would pass every check this repository runs, which is exactly the dangling reference the move invites.

That scenario rejected resolving qualified references, because it would turn the primitive into a link resolver with a file-loading policy, an error surface for unreadable targets, and a question about what counts as a document root. The **Shared Files** manifest answers all three for the documents it ships, so for those documents the judgment is bounded rather than general. The operator chose this over a section-pointer finding in `check-step-references`, which reads only `## Instructions` and would miss pointers in a command's Context, Scope and Gate sections and in the installer.

## Behavior

A fourth kind, **`cross-document`**, sits between `qualified` and `intra-document`:

- A line that names **exactly one** document other than the markers file, where that document is a destination the `## Shared Files` section of `framework/bootstrap/ductus.md` maps to a source, has each `§` on it judged. The reference resolves when the text after `§` begins with a heading of the mapped source (longest first, the intra-document rule), a heading of the citing file, or a marker in the markers file. Otherwise it is unresolved and listed in `unresolved`.
- The result carries a `cross-document` count beside `qualified` and `intra-document`.
- A pointer written as an explicit pair, `**§X** in` followed by a shipped document in backticks, is judged on any line, against that document's headings alone: the author has said which document the section belongs to, so neither a local heading nor a marker rescues it. A command step routinely names `plan.md` or `spec.md` beside its pointer; measured 2026-10-25, 59 of 100 pointers sat on such lines.
- Otherwise a line naming several documents, or one the manifest does not map, stays `qualified`, and so does a line whose mapped source cannot be read. Widening the line rule to "exactly one *shipped* document" was measured and rejected: it would have reported 9 references meant for the other document on their line.

The map is read once per call from the `## Shared Files` section alone, a row giving its destination and source as one backticked span each. It is keyed by the destination exactly as written there, and a document on the citing line matches when the path inside its backticks equals a destination. Where `framework/bootstrap/ductus.md` is absent, which is every adopter, the map is empty and no reference is judged this way. That is the same discriminator `check-artifacts`' adopter destinations use.

**The marker and citing-file fallbacks are required, not lenient.** Measured 2026-10-25 before the move: the framework and spec corpus held 16 references on lines naming exactly one mapped document, and every one was a constitution marker beside a path such as `specs/inbox.md`. A rule reading only the mapped source's headings would have reported all 16. And a pointer line can cite a section of its own file too, as the installer's rule-surface line cites §Shared Files of its own file, `ductus.md`, beside review's reference.

## Edge Cases

- The markers file named on the same line does not count as the one document, the same as for `qualified`.
- A heading that starts with a digit cannot be named by a `§` reference, whose grammar starts with a letter; 071's reference files carry no numbered headings for that reason.
- A shortened pointer (`§Rules` for the heading `Rules (blocking and advisory)`) is unresolved, which is the safe direction: the match is a prefix of the text after `§`, never of the heading.
- Family 39 reports the `cross-document` count on stderr with the others.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
