---
section: "Framework consistency — each artifact by its durability class"
---

# A Plan Edit Does Not Reopen a Done Spec

## Context

`plan.md` records the design as it currently stands (§plan-phase, from this spec's `plan-records-the-design-as-it-stands`). A later spec that changes a file a `done` spec's plan names therefore leaves that plan stale. Correcting it was a meaningful body edit that reopened the `done` spec for a full review and analyze. On 2026-10-16 one dead-reference sweep reopened 18 `done` specs this way. Every reopen was a plan edit correcting a stale claim: the `amend(NNN): the plan …` commits ending at `8191e695`. Re-closing them took a review and an analyze each. `072-governing-documents-stay-within-one-read` measured this as one of the two untracked causes of the slowdown it addressed, and left it out of its own scope. Review freshness already ignores plans, because only scenarios and data models are durable contracts (`framework/reference/review.md` §Window and coverage). The reopen therefore buys a re-gate whose review cannot see anything a plan edit changed.

## Behavior

- An edit confined to a `done` spec's `plan.md` or `tasks.md` is mechanical. It takes no back-edge and the spec stays `done`, whatever claim the edit corrects.
- §spec-lifecycle states this as a further mechanical case, and every surface that computes whether an edit reopens a spec agrees with it, `prune-plan`'s `reopen-required` among them.
- The constitution amendment fits under its size ceiling (`scripts/lint-governing-size.sh`), either by removing as many bytes as it adds or by landing after `050-constitution`'s cut.
- Replaying the 2026-10-16 plan edits under the new rule reopens none of the 18 specs.

## Edge Cases

- **A plan edit recording a changed design decision.** The later spec that changed the behavior owns and gates that decision, so the `done` plan's edit records it and still takes no back-edge.
- **A `done` spec's analyze record that digests the plan.** It goes stale when the plan changes, and that staleness neither reopens the spec nor blocks a release.
- **One change that also edits `spec.md`, a scenario or `data-model.md`.** Those parts keep today's rules; only the plan and tasks part is exempt.

## Open Questions

- Does the exemption cover a plan edit that records a changed design decision, or only a factual correction? Recommended: both, because the later spec owns and gates the behavior change, so re-gating the `done` spec re-verifies nothing.
- Does a `done` spec's analyze record, stale only because its plan changed, need handling in the release audit or the pre-`done` gate? Recommended: exclude `plan.md` and `tasks.md` from analyze freshness on `done` specs, matching review freshness, which already digests only durable contracts.

## Resolved Questions

*None yet.*
