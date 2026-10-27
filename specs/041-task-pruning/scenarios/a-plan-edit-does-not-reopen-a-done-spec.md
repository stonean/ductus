---
section: "Framework consistency — each artifact by its durability class"
---

# A Plan Edit Does Not Reopen a Done Spec

## Context

`plan.md` records the design as it currently stands (§plan-phase, from this spec's `plan-records-the-design-as-it-stands`). A later spec that changes a file a `done` spec's plan names therefore leaves that plan stale. Correcting it was a meaningful body edit that reopened the `done` spec for a full review and analyze. On 2026-10-16 one dead-reference sweep reopened 18 `done` specs this way. Every reopen was a plan edit correcting a stale claim: the `amend(NNN): the plan …` commits ending at `8191e695`. Re-closing them took a review and an analyze each. `072-governing-documents-stay-within-one-read` measured this as one of the two untracked causes of the slowdown it addressed, and left it out of its own scope. Review freshness already ignores plans, because only scenarios and data models are durable contracts (`framework/reference/review.md` §Window and coverage). The reopen therefore buys a re-gate whose review cannot see anything a plan edit changed.

## Behavior

- An edit confined to a `done` spec's `plan.md` or `tasks.md` that adds no unchecked task takes no back-edge, and the spec stays `done`, whatever claim the edit corrects.
- §spec-lifecycle states this beside its three mechanical cases rather than as a fourth, since a plan correction changes a claim and so fails their shared test, and every surface that computes whether an edit reopens a spec agrees with it, `prune-plan`'s `reopen-required` among them.
- The constitution amendment fits under its size ceiling (`scripts/lint-governing-size.sh`), either by removing as many bytes as it adds or by landing after `050-constitution`'s cut.
- Replaying the 2026-10-16 plan edits under the new rule reopens none of the 18 specs.

## Edge Cases

- **A plan edit recording a changed design decision.** The later spec that changed the behavior owns and gates that decision, so the `done` plan's edit records it and still takes no back-edge.
- **A `done` spec's analyze record that digests the plan.** A plan or tasks edit does not stale it, because freshness on a `done` spec compares neither file; see Resolved Questions.
- **One change that also edits `spec.md`, a scenario or `data-model.md`.** Those parts keep today's rules; only the plan and tasks part is exempt.
- **A `tasks.md` edit that adds an unchecked task.** It is new work, not a correction, so it still reopens the spec; see Resolved Questions.
- **A `plan.md` trim `check-artifact-size` proposes for an oversized `done` plan.** It is a plan edit like any other, so its advice says it does not reopen the spec.
- **`/{project}:amend`'s reconcile offer.** On a `done` spec with an uncommitted `tasks.md` change, amend offers to reopen it and reopens only if the operator confirms. Declining for a change that adds no unchecked task is this rule applied, so the offer is left as it is; `014-reclarify-backedge`'s `reopen-after-informal-edits` owns its delta (decided 2026-10-26, under the owner's pre-authorization).

## Open Questions

*None.*

## Resolved Questions

- **Does the exemption cover a plan edit that records a changed design decision, or only a factual correction?** Both (owner pre-authorized 2026-10-26). The later spec that changed the behavior owns and gates that change, so re-gating the `done` spec would re-verify nothing.
- **Does a `done` spec's analyze record, stale only because its plan changed, need handling in the release audit or the pre-`done` gate?** Neither (owner pre-authorized 2026-10-26). Analyze freshness leaves `plan.md` and `tasks.md` out of the comparison on a `done` spec, as review freshness leaves out everything but the durable contracts, so such an edit does not stale the record at all. A spec still in progress keeps both in its comparison, because the pre-`done` gate reads them there.
- **Does a `tasks.md` edit that adds an unchecked task take no back-edge too?** No (resolved 2026-10-26, under the owner's pre-authorization). An unchecked task on a `done` spec is outstanding work, and a status of `done` beside it would make unfinished work indistinguishable from finished work (§design-principles). So the exemption covers a `tasks.md` edit that adds no unchecked checkbox, and `prune-plan`'s `reopen-required` keeps that one trigger.
