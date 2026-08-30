---
section: "Acceptance-criterion path existence"
---

# Removal-claims-are-checkable

## Context

`criterion-path-existence` checks a path named in a `done` spec's acceptance criterion still resolves. A criterion phrased as a *removal* — "`framework/workflows/` does not exist", "`install.sh` is deleted", "renamed to `configuration-cross.md`" — carries one of fourteen non-assertion phrases and is exempted whole, each path recorded as `not-a-live-claim`.

The exemption is correct as far as it goes: testing such a path for presence would manufacture a finding out of prose. But it stops one step short. A criterion asserting a path is **absent** is exactly as checkable as one asserting it is present — the assertion is simply inverted. Skipping it means those criteria have had **nothing** check the substance of their claim, ever, and they sit ticked on `done` specs indefinitely.

The corpus makes the scale concrete: 72 criteria across 18 specs carry a `not-a-live-claim` skip today, concentrated in 023 (14), 018 (9), and 027 (8).

`AGENTS.md` already names the consequence and prescribes a hand-walk at the completion gate — "walk the `skipped` array entry by entry and check each criterion's claim against the tree by hand". That remedy works and it is a **diligence dependency**, which [§design-principles](../../../framework/constitution.md#design-principles) rejects wherever the code can hold the rule instead. Here it can, for the subset that asserts absence.

The walk was performed once, on 2026-08-30, and found two stale claims among the 72 — `003`'s AC2 naming `.govern.session.toml` as the current session file, and `025`'s AC3 quoting a warning string a later refactor had reworded. A hit rate around 3% is low enough that nobody would find these by chance and high enough that the corpus accumulates them.

## Behavior

A criterion whose claim is that a path is **absent** is checked for absence rather than skipped.

- The non-assertion marker list splits by what the phrase asserts. A **removal** phrase (`does not exist`, `is deleted`, `no longer exists`, `is removed`) makes the path a checkable absence claim: the family reports when the path is *present*. A **hedge or scope** phrase (an adopter-scoped path, a migration subject, a shape) stays exempt, because it asserts nothing testable either way.
- The finding names the inversion plainly — the criterion says the path is gone and it is not — so a reader is never left deducing which direction the check ran.
- A **rename** phrase is two claims, and the family checks the half it can: the new path must exist. Whether the old one is gone is the removal case above when the criterion says so, and unstated otherwise.
- Whatever stays exempt is still reported in `skipped`, and the reason set says why it is unverifiable rather than merely unchecked.

## Edge Cases

- **An adopter-scoped path is not a repository claim.** `specs/rules/security-backend.md` exists in an adopter's tree after scaffolding and never here; inverting the check would report every such criterion as broken. These keep an exemption, and it must be distinguishable in the reason set from a claim that was genuinely checked.
- **The inversion must not fire on a path the criterion merely mentions.** A criterion reading "X is deleted; its actions are inlined into `ductus.md`" names two paths with opposite claims. Attributing the removal phrase to every path on the line would report `ductus.md` as wrongly present — a false finding on a true criterion, which is worse than the silence it replaces.
- **A criterion recording history is not making a live claim in either direction.** "A workflow registry existed at `framework/workflows/registry.json`… since retired" describes what was true then. It happens to also be checkable as an absence, and treating it so is fine — but the wording that marks it historical must not be mistaken for a removal claim about some *other* path in the same sentence.
- **This closes only the absence half.** A criterion whose claim is about file *content* — a quoted warning string, a heading's exact text — stays unverifiable by this family, and `025`'s AC3 is that case. The bound belongs in the family's own statement, so a clean result is not read as "every criterion checked".

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
