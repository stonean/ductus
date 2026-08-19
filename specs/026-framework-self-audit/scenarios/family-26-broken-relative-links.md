---
section: "Follow-on scenarios"
---

# Family-26-broken-relative-links

## Context

A relative markdown link whose target does not exist renders fine, reviews fine, and resolves to nothing. Nothing caught the class: markdownlint's `MD051` validates heading *fragments* and says nothing about whether the file exists, and `check-orphaned-references` scopes to adopter-owned referrers and ductus-managed path prefixes, so a spec linking a sibling spec at the wrong depth falls between the two.

28 broken links existed across the corpus when this family was written, and the dominant class is a depth error in a scenario file. A scenario lives one tier deeper than its spec — `specs/NNN-foo/scenarios/bar.md` — so a sibling spec is `../../NNN-other/` and the constitution is `../../../framework/`. Writing one `../` too few is the single easiest mistake to make in this repository's layout, and 22 of the 28 were exactly that.

The remaining six were a link to a script a later spec deleted, an illustrative spec name that never existed, two links to a `specs/spec.md` that was removed when the repo reorganized around `framework/`, and a link from the shipped bootstrap into this repo's own spec tree — meaningless in an adopter's checkout, where that path does not exist.

## Behavior

The family reports every relative markdown link whose target does not resolve, anchored to `file:line`, and distinguishes the two repair paths: when the target resolves one directory up it says so and gives the corrected path, and otherwise it directs the author to confirm the target still exists and to name a deleted one in prose rather than linking it.

**Inline code spans are stripped before matching, and that is load-bearing rather than tidy.** Documentation that discusses linking quotes link syntax constantly — `[text](target)`, `[plan](plan.md)` — always inside a code span. Without stripping them the family reports 7 false positives on the corpus, every one a document correctly *describing* a link rather than making one.

**Fences are toggled line by line, not stripped with a whole-text regex.** A regex that deletes a fenced block deletes its newlines with it, which shifts every line number after it — findings point at the wrong line, and the further into the file the further off. This was a real defect in the family's first draft, caught only because a reported line came back blank when read.

Two categories are excluded by construction, counted and reported rather than dropped: generated command copies under the agent config directory, whose links are broken by construction because the generator copies sources to a different directory depth without rewriting them, and adopter-facing templates, whose links resolve in the adopter's repo root after scaffolding rather than here.

A failed file listing is a finding, never a silent pass.

## Edge Cases

- **A link inside a fenced code block** is not a link; fences are skipped.
- **Documentation shapes** — a target naming `NNN`, a `{placeholder}`, or a literal `...` — are skipped and counted. Prose names link syntax as often as it names files.
- **A fragment-only link** (`#anchor`) and an external URL are out of scope; `MD051` already covers the former.
- **A link to a file a later spec deleted** is reported with the "confirm the target still exists" repair path rather than a depth hint, because no amount of `../` will find it. Naming it in prose is the fix, which is also what keeps a citation from becoming a dependency edge.
- **The shipped bootstrap** cannot link into this repo's spec tree at all: the file is installed into an adopter's checkout where no such path exists, so the citation is by name in both the live and retired copies, which a separate family holds byte-identical.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
