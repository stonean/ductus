---
status: planned
dependencies: [051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation]
review:
  last-run: null
  reviewed-against: null
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  blocking: false
next-criterion: 13
---

# 053 — Supersession reconciliation

Walk the superseded spec's claims when a supersession is declared: classify each, annotate what the superseding spec removed, and surface what conflicts rather than deciding it.

## Motivation

[052 — Spec supersession and consolidation](../052-spec-supersession-and-consolidation/spec.md) records the *relation* — a `supersedes:` key, a reciprocal annotation, and the commands that declare it. That much prevents a reader mistaking a countered spec for a live one at the top of the file, and it is enough on its own.

It was not enough where a superseding spec counters only *part* of its predecessor. A banner reading "everything below is historical" is correct for `005-workflows`, whose whole feature went away, and over-broad everywhere else: the claims the superseding spec never touched are still live, the ones it countered still read as current, and nothing distinguishes them. The reader is left doing the comparison by hand, which is the work the declaration was supposed to have already done.

Split from 052 because the relation and the claim-level work are separable in one direction: a declaration plus an annotation ships without reconciliation, while reconciliation is meaningless without a declared edge to scope it.

## Behavior

Reconciliation runs when a supersession is declared — at spec creation and on a retroactive declaration alike — while the superseding spec's claims are being authored and the intent is held. Deferring it to completion re-creates the recovery problem the declaration exists to avoid.

It behaves like a merge over two declared endpoints, not a search:

- Claims the superseding spec **removes** are annotated as superseded.
- Claims it **contradicts without removing** are conflicts: surfaced to the operator, never resolved silently and never by picking a side.
- Claims it does not touch are left alone.
- Anything that cannot be classified from the pair is reported as unclassified rather than guessed.

**This does not resurrect the rejected check.** The criterion-supersession check was measured across unscoped pairs hunting for supersessions nobody declared — 455 pairs, 215 firing, every sample a false positive. A declared edge collapses the search to two specs and changes what a false positive costs: not an unsolicited finding on a `done` spec, but a candidate offered to an operator who has already said these two conflict.

## Annotate or edit

Annotation is the default and editing is never automatic, because the two are not symmetric in cost: a meaningful body edit to a `done` spec takes the `done` → `in-progress` back-edge, while an annotation is a mechanical edit that leaves the status alone. Reopening is the wrong signal — a superseded spec is not unfinished, it is finished and later countered.

- **An acceptance criterion is never edited.** A superseded criterion stays ticked and gains an annotation: it *was* delivered, and the removal belongs to the later spec. Editing it would falsify a delivery record.
- **Body prose may be edited** where it genuinely reads better as current state, but only on an operator confirmation that names the reopen it causes — the way every other back-edge in the pipeline is named before it is taken.

## Scope of the read

Reconciliation reads the two declared specs — bodies and criteria — plus the superseded spec's scenarios, since a scenario is a spec at a lower level of abstraction and can carry a claim the supersession touches. Nothing else: no plan, no data model, no tasks file, no source tree, no third spec.

This is not a widening of what the pipeline permits. [051 — Branch-scoped spec numbering](../051-branch-scoped-spec-numbering/spec.md)'s fold-back already declares the same bound, reading two full specs on the authority of a declared pointer while excluding plans, data models, and source. The declaration is what authorizes the read; without one there would be a corpus to scan, which is the rejected check.

One consequence follows and is stated rather than left to be discovered. From the pair alone, reconciliation can determine what the superseding spec **declares it removes**. It cannot determine that a claim was *never delivered in the first place* — that needs a read of the tree. Such a claim is reported as unclassified, and the never-completed determination stays with the criterion-verification pass that already reads the tree and already makes that call.

## Edge cases

Two states must not be allowed to read like a clean reconciliation, both instances of the rule that a check which could not run must never be indistinguishable from one that passed:

- **Nothing to classify.** A superseded spec with no criteria — a brownfield sketch, legitimately — yields an empty classification. That is *examined and empty*, not *examined and clean*, and the two are reported differently.
- **Nothing readable.** A spec or scenario that will not parse contributes no classification and is not escalated into a conflict, since nothing can be proven about a file that cannot be read. It is named in the result and excluded from the counts, the way fold-back's enumeration already names what it could not examine.

A superseding spec declaring it removes something the superseded spec never claimed is reported as a mismatch rather than dropped: it usually means the declaration named the wrong predecessor.

## Acceptance Criteria

- [ ] AC1: Declaring a supersession walks the superseded spec's claims and classifies each as superseded, still standing, or conflicting
- [ ] AC2: A conflicting claim is surfaced to the operator and is never resolved silently or by picking a side
- [ ] AC3: A pair whose reconciliation is incomplete is reported as incomplete, never rendered indistinguishable from a fully reconciled pair
- [ ] AC4: Reconciliation annotates by default and never edits a superseded claim without the operator confirming it
- [ ] AC5: A confirmation offering to edit a `done` spec's body names the `done` to `in-progress` back-edge the edit causes, before the edit happens
- [ ] AC6: Reconciliation never edits an acceptance criterion of the superseded spec; a superseded criterion stays ticked and is annotated
- [ ] AC7: Reconciliation reads only the two declared specs and the superseded spec's scenarios — never a plan, data model, tasks file, source tree, or third spec
- [ ] AC8: Reconciliation classifies a claim the superseding spec declares it removes, and reports a claim it cannot classify from the declared pair as unclassified rather than inferring it
- [ ] AC9: The never-completed determination is left to the existing criterion-verification pass rather than reimplemented inside reconciliation
- [ ] AC10: Reconciliation runs at declaration time, on both a creation-time and a retroactive declaration, and not at the superseding spec's completion gate
- [ ] AC11: A superseded spec carrying no classifiable claims is reported as examined-with-nothing-to-reconcile, distinct from a reconciliation that examined claims and found no conflicts
- [ ] AC12: A superseded spec or scenario that cannot be read or parsed is named in the result and excluded from the classified counts, never silently counted as reconciled

## Open Questions

<!-- None. The decisions were resolved on 052 before this spec was split out of it;
     each is recorded under Resolved Questions below. -->

## Resolved Questions

- **Why split from spec-supersession-and-consolidation:** the two are separable in one direction, and only one direction. A declared supersession plus its reciprocal annotation is useful on its own — it stops a reader mistaking a countered spec for a live one, which is the whole of 052 — while reconciliation is meaningless without a declared edge to scope it. Bundling them would have held the annotation behind the claim-level walk, and the annotation is the half that pays for itself immediately. The shared Affected Files are the consequence of that ordering rather than evidence against it: 053 extends surfaces 052 built, which is what a dependent spec does. 052 states the same split from its side, under `## See also`.
- **May reconciliation edit the superseded spec's body claims, or only annotate them?** Annotate by default; editing is available, never automatic, and carries a stated cost. The tension is live in the corpus — §spec-lifecycle says spec bodies represent current state with git as the historical record, while `005-workflows` says it stays `done` *as* the historical record — but a mechanical consequence settles it: a meaningful body edit to a `done` spec takes the back-edge while an annotation does not. An acceptance criterion is never edited, since it records a delivery event; body prose may be, on a confirmation naming the reopen.
- **How does reconciliation bound its reads?** To the declared pair — both specs in full plus the superseded spec's scenarios — and nothing else. The premise that this exceeds what the pipeline permits is false: fold-back already declares exactly this bound. One consequence revises what reconciliation may claim: it classifies what the superseding spec declares it removes, reports what it cannot classify rather than inferring it, and leaves the never-delivered call to the criterion-verification pass that reads the tree.
- **When does reconciliation run?** At declaration, while the superseding spec's claims are being authored. Deferring it to the completion gate would re-create the recovery problem the declaration exists to avoid — the information is cheap at declaration and effectively unrecoverable afterward, which is the finding the whole supersession feature rests on.
