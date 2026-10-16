---
section: "Fold-back on merge"
---

# A-fold-owed-to-another-tree-does-not-hold-done

## Context

The pre-`done` gate blocked a branch-scoped spec for as long as it carried `folds-into` (AC35 as it then stood), because the fold is outstanding work. That held when the fold could be done here. It failed when the target lived on a line of development this tree does not hold.

An adopter's release branch carried a branch-scoped spec whose fold target exists only on a later release line, which merges afterwards. The work was finished: every acceptance criterion verified, the review at 0 MUST and 0 SHOULD, the analysis not blocking. `check-review-gate` still answered `pending-fold`. Fold-back needs both specs in one tree, which this branch will never be, so the gate's only exit was a manual `set-status` past it. Without that, shipped work stays `in-progress`, the one status that claims work is under way, for the life of the branch.

The framework already treats that absence as normal. This spec says the target "normally names a spec the branch cannot see" and that its absence is never a finding (AC31); `check-unfolded-specs` reports it as the pre-merge state; and `/{project}:status` qualifies the row as carrying a pending fold whatever its status. The owed fold is tracked and visible without the gate holding `done` hostage to it.

## Behavior

- **The fold check blocks only when the fold can be done here.** When the `folds-into` target resolves in the current tree, by the test fold-back applies, a feature directory holding a `spec.md`, the gate still blocks with `pending-fold`: the fold is possible, so it is the work owed before `done`. **Superseded by the sibling scenario [a-pending-fold-never-holds-done](a-pending-fold-never-holds-done.md)**: the gate has no fold check, so a target in this tree does not block either, and `pending-fold` is gone.
- **An absent target does not block.** When the target does not resolve in this tree, the fold check passes and the checks after it decide, so the spec reaches `done` through the normal gate when everything else passes.
- **The fold stays owed, and stays visible.** `check-unfolded-specs` keeps reporting the spec, now with `status: done`, and `/{project}:status` renders it `done (fold pending)`, until fold-back discharges it in the first tree that holds both specs. Fold-back still refuses a target that does not exist (AC28), so a mistyped `folds-into` surfaces there, as it does today.
- **`/{project}:fold` accepts a `done` staging spec.** The staging spec's status does not gate the fold. The upstream spec's reopen rules are unchanged: the scenario edge or the meaningful-body-edit edge, and only from `done`.
- **The rule changes everywhere it is stated**: the constitution's §spec-lifecycle (the `done` row and the branch-scoped paragraph) and §numbering, this spec's AC35 and §Fold-back on merge, `/{project}:implement`'s gate text, `/{project}:fold`, `/{project}:status`, the user docs, and 066's record of the gate.

## Edge Cases

- **The target arrives later.** When the branch merges into a tree that holds the target, the spec is already `done`, so `check-review-gate` answers that there is no transition to gate (066's AC4). `check-unfolded-specs` and `/{project}:status` report the fold until it is run.
- **A target that does not exist anywhere.** Indistinguishable here from one on another line, so it does not block `done`. It is caught where it always was: fold-back refuses it, and `/{project}:status` calls it out as not in this tree on every run.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Does the cross-spec-impact gate's `target-missing` block need the same change?** No. The fold rule can relax because two things carry the owed fold past `done`: `check-unfolded-specs` and the pipeline view keep reporting it, and fold-back refuses a missing target in the first tree that holds both specs. An undischarged impact has neither. Only the pre-`done` gate reads `cross-spec-impact:`, so letting an absent target pass there would let the obligation disappear the moment the spec reached `done`, which is the failure §cross-spec-impact exists to prevent. `target-missing` also catches a mistyped or non-directory entry. It stays a block.
