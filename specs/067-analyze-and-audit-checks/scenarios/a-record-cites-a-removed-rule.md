---
section: "Frontmatter, dependency, anchor and rule checks"
---

# A-record-cites-a-removed-rule

## Context

`check-rule-ids` reports a citation of an ID that no loaded rule file defines as a Blocking "references unknown rule" finding, and its pattern matches any mention of an ID. A rule `ductus` ships is retired in two steps (constitution §rules): its label deprecates it, and the release the label names removes it. Some artifacts record a retirement and must keep the old ID, since AGENTS.md says a sweep must not rewrite the record of the thing being swept: a decision record, a spec's account of what it deprecated, a signpost.

077 deprecates `BE-DEPS-006`, `BE-AUTHN-009`, `BE-AUTHN-013` and `BE-AUTHN-020` for removal in 0.62.0. Its own `spec.md` (Motivation, §Rule set scope, §Deprecation and re-pointing, §Edge cases and AC5) and the signposts in 008's and 017's `spec.md` cite them. Once the rules are removed, each of those records draws the Blocking finding, and no artifact says how a record passes. Found by 077's review on 2026-11-03 (owner's call, option a, recorded on 077's review).

## Behavior

A citation of a removed rule in an artifact that records its retirement passes `check-rule-ids`, while a citation of a removed rule as a current requirement remains the Blocking unknown-rule finding. How the check tells the two apart is this scenario's open question.

## Edge Cases

- **A removed rule cited as a current requirement** stays a Blocking finding, so the removal's sweep stays enforced.
- **Family 40** (026's scenario `family-40-deprecated-rules-are-removed-on-time`) names every tracked file citing a due rule and leaves each to be re-pointed or classified as a record; the classification it leaves to the remover is the one this check makes.

## Open Questions

- How does `check-rule-ids` tell a record of a removed rule's retirement from a citation of it as a current requirement (a tombstone heading the removal leaves in the rule file, an exemption for record sections such as Resolved Questions, a deprecation account or a signpost, or another mark), and which artifacts count as records?

## Resolved Questions

*None yet.*
