---
section: "Fold-back on merge"
---

# A-definition-and-a-title-follow-the-link-grammar

## Context

`rewrite-spec-links` re-points an inline link by finding `](` and taking everything up to the next `)` as the destination. A reference-style definition (`[label]: ../{from}/spec.md`) in a file under the spec root was therefore left naming the retiring directory and dangled once it was retired, though AC6 says no reference to it dangles in the files the primitive reads. Nothing else caught it: `check-corpus-links` and `derive-dependencies` read only inline links, so the commit was not refused, and `mentions` listed the line without correcting it. A referrer outside the spec root already had such a definition re-pointed by the token pass, so the gap was in the spec root only.

The same reading took a link's title as part of its last path segment, so `[t](../{from}/spec.md 'x')` was reported in `kept-tails` as a kept tail `spec.md 'x'` though `spec.md` is the file the fold writes.

## Behavior

- A line that is a link reference definition (at most three spaces of indent, a non-empty label that does not start with `^`, then `]:`) is re-pointed through the same rule as an inline link: its destination, bare or in `<...>`, names the retiring directory as a whole path segment, and is rewritten with its fragment, its tail and its kept-tail and local-path rules unchanged. Its title and spacing are kept as written.
- A link's destination is what precedes its first whitespace, or the text inside `<...>`, and the rest of the target is its title. The title is never re-pointed and never part of a kept-tail reference.
- A link with no title, no `<...>` and no surrounding space is rewritten as before.
- A footnote definition (`[^1]: ...`) and a bare `<path>` are not links: they are left as written, and `mentions` lists a line that still names the retiring directory.

## Edge Cases

- A definition whose destination names a different directory sharing the prefix is left alone, as an inline link is.
- A definition outside the spec root is re-pointed by the token pass, as before; a definition in `system.md` takes the link pass first.
- `check-corpus-links` still reads only inline links; teaching it definitions is not part of this scenario.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
