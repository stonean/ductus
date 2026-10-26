---
section: "Follow-on scenarios"
---

# The Constitution States Rules, Not Stories

## Context

The constitution reached 865 lines and 148,528 bytes, about 37k tokens loaded into every adopter session by `/{project}:target`, and the operator asked whether it could be made more concise (2026-10-17). A measurement split the bytes: 130 bold-led rules whose lead sentences totalled 6 KB sat on 62 KB of body text, tables made up 10 KB, and the remaining 69 KB was section prose. The length was not rules. It was rationale, worked examples and restatement, accumulated as rules were promoted from contributor guides, each carrying the story that produced it. Bytes are the cost, not lines: the lines are paragraphs, so a line count barely moves when the document shrinks.

## Behavior

- Every rule in the constitution is stated as the rule, one load-bearing reason, and its boundary where it has one. Worked examples, incident accounts, "this is X applied to Y" restatements and repeated enforcement disclaimers are not carried. The story behind a rule lives in the scenario or spec that adopted it, and in git.
- A concision pass holds byte-identical: every `<!-- §anchor -->` marker, every heading, every table, the pipeline diagram, and every bold rule lead. The RFC 2119 keywords in each section are kept. The 2026-10-26 cut to one read (`the-constitution-fits-one-read`) narrowed this for that cut: the anchors, the diagram, every heading another file links to and every bold lead an `AGENTS.md` entry or another spec cites stayed byte-identical, while a subsection, table or lead that only restated what the document states elsewhere could go.
- The 2026-10-17 pass took the document from 148,528 to 116,548 bytes (21.5%), with every rule kept. A pilot on §grounding's Rules cut 46%. The whole document cut less, because some sections were already tight and some passages are quoted elsewhere.

## Edge Cases

- **A promotion round adding a rule** writes it in this shape, as the reword test in 050's plan prescribes, so the document does not grow back by accretion.
- **A passage another spec quotes stays verbatim.** 021 restates the runtime principles, the eligibility criteria and the acquisition invariant, and its AC1 and AC9 bind §text-first-artifacts' opening properties and plain principle bullets. Tightening those would falsify the quoting spec.
- **A rule found stale during a pass is corrected, not only shortened.** This pass found §implement-phase still citing a review-agreement check that 057 retired.
- **Removed story is not lost.** It remains in the adopting scenario, the spec, and git history; the constitution is the one artifact every adopter loads in full.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
