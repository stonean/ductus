---
section: "Frontmatter, dependency, anchor and rule checks"
---

# A-record-cites-a-removed-rule

## Context

`check-rule-ids` reports a citation of an ID that no loaded rule file defines as a Blocking "references unknown rule" finding, and its pattern matches any mention of an ID. A rule `ductus` ships is retired in two steps (constitution §rules): its label deprecates it, and the release the label names removes it. Some artifacts record a retirement and must keep the old ID, since AGENTS.md says a sweep must not rewrite the record of the thing being swept: a decision record, a spec's account of what it deprecated, a signpost.

077 deprecates `BE-DEPS-006`, `BE-AUTHN-009`, `BE-AUTHN-013` and `BE-AUTHN-020` for removal in 0.62.0. Its own `spec.md` (Motivation, §Rule set scope, §Deprecation and re-pointing, §Edge cases and AC5) and the signposts in 008's and 017's `spec.md` cite them. Once the rules are removed, each of those records draws the Blocking finding, and no artifact says how a record passes. Found by 077's review on 2026-11-03 (owner's call, option a, recorded on 077's review). Measured 2026-11-11 at `6158d7df`: `check-rule-ids` reads only a spec's `spec.md` (analyze step 5), where 15 lines cite the four IDs, 13 in 077 and one each in 008's and 017's blockquote signposts; 5 of the 15 carry a retirement word (`deprecat`, `remov`, `retir`) on the line, 4 more say "moved from", 1 sits in 077's Resolved Questions, and 4 carry neither (077 lines 17, 18, 35, 51). The removal deletes each rule's section, so afterwards nothing in a rule file names a removed ID, its successor or the spec that retired it; 008's and 077's specs are `done`.

## Behavior

A spec records the rules whose retirement it records in a `retired-rules:` frontmatter list of rule IDs. `check-rule-ids`, reading a spec, passes a citation of an ID on that spec's list that no loaded rule file defines, and reports it in a distinct `retired` state, never as found and never as missing. A citation of a removed rule in any other artifact, or of an ID the citing spec does not list, remains the Blocking unknown-rule finding.

## Edge Cases

- **A removed rule cited as a current requirement** stays a Blocking finding, so the removal's sweep stays enforced.
- **An ID on a spec's `retired-rules:` list that a loaded rule file still defines** is reported as found, as today: the list exempts nothing until the rule is gone.
- **A mistyped ID on the list** passes its own spec's citation of the same mistyped ID, and no other spec's; the list is checked only by the citations it exempts.
- **Family 40** (026's scenario `family-40-deprecated-rules-are-removed-on-time`) names every tracked file citing a due rule and leaves each to be re-pointed or classified as a record; the classification it leaves to the remover is the one this check makes, and the remover records it on the citing spec's `retired-rules:` list.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **How does `check-rule-ids` tell a record of a removed rule's retirement from a citation of it as a current requirement, and which artifacts count as records?** A spec frontmatter list, `retired-rules: [IDs]`, names the removed rules that spec records the retirement of; a citation of a listed ID in that spec passes and is reported as `retired`, and every other citation of a removed rule stays Blocking (owner, 2026-11-12). The schema gains the optional field in `framework/constitution.md` §text-first-artifacts' spec-files table. Measured at `ab2ad085`: the four rules 077 deprecates are cited on 15 lines in 3 specs, 13 in 077 (one its AC5), and one blockquoted signpost each in 008 and 017, so three lists cover all 15. Rejected: a tombstone heading left in the rule file, which makes the ID known in every context, current requirements included, and contradicts the removal and Family 40's heading parse; exempting blockquotes and Resolved Questions, which covers 3 of the 15; a retirement word on the line, which 4 of the 15 carry no form of; and rewriting the citations, which edits `done` specs 077 and 008, AC5 a criterion, and rewrites a record.
