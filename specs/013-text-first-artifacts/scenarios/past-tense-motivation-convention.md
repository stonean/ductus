---
section: "Behavior"
---

# Past-tense-motivation-convention

## Context

A `## Motivation` legitimately describes the world *before* the feature. Written in the present tense, it becomes false the moment the spec ships — and unlike a broken link, nothing marks it stale.

Measured, not assumed. `/{project}:analyze` against 046 on 2026-07-31 found three of its four Motivation bullets falsified on ship: `/{project}:status` described as carrying no scenario question count, `check-artifacts` as having no family for them, `/{project}:target` as never seeing them — each true when written and false once the spec landed. `041-task-pruning` is `done` with a Motivation still reading "`ductus` has no command to reclaim that space", false since `/{project}:prune` shipped; it only *reads* correctly because the next sentence introduces the command, narrative framing a bulleted list lacks. This is systemic rather than a per-spec slip.

**Not deterministically detectable, and confirmed so.** [045 — Decision-state drift detection](../../045-decision-state-drift-detection/spec.md) evaluated exactly this case during its clarify and scoped it out on measurement: its link-adjacent check fires only on same-feature sibling links, and across all 47 specs `## Motivation` sections contain **zero** of those (the 21 links there are cross-feature, outside scope). Catching it needs tense analysis, which no deterministic check carries. It is an authoring convention, and this scenario places it where conventions are cheapest to follow.

## Behavior

**The spec template carries the convention.** `framework/templates/spec/spec.md`'s `## Motivation` section gains authoring guidance: write the section in the past tense, describing the state that motivated the feature rather than asserting a present condition that the feature is about to falsify.

Guidance in the template rather than a normative rule, deliberately. It reaches the author at the moment of writing, where following it costs nothing, and it ships to every adopter through `/ductus`'s template copy. The rejected alternative was constitution §spec-requirements: normative and citable, but it binds every adopter to a rule no check enforces, and a rule that only a human can verify is the kind the framework's own design principles warn against depending on. `AGENTS.md` was rejected earlier and more firmly — it is `ductus`'s contributor guidance, invisible to adopters, which would leave the convention unshipped for exactly the audience that needs it.

**Existing `done` specs are not swept.** The convention applies to specs written after it lands. A retroactive pass over 47 Motivation sections is a large, purely editorial diff across shipped contracts, and the cost of the stale prose it would fix is a reader's second glance — not a broken gate. 046 already had such a pass, opportunistically; 041 has not, and stays as it is.

## Edge Cases

- A Motivation that is already past-tense needs no change; the guidance is a prompt, not a validator.
- A spec whose Motivation describes a *continuing* condition the feature does not remove stays present-tense correctly — the convention targets claims the feature falsifies, not every present-tense sentence.
- A `## Motivation` naming a same-feature sibling artifact is still in 045's link-adjacent scope and can be flagged there; this convention does not exempt it, and the two are complementary rather than overlapping.
- Adopters who pin `framework/templates/spec/spec.md` keep their own version and do not receive the guidance — the standard consequence of pinning, not a gap here.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
