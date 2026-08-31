---
status: done
dependencies: [041-task-pruning]
review:
  last-run: 2026-08-30T21:23:15Z
  reviewed-against: 62381d05f0bd4d78396f4efcbdcae889c1dd8e06
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  blocking: false
next-criterion: 46
---

# 052 — Spec consolidation

Give the corpus a way to remove a spec whose content belongs with another: re-point every inbound pointer at the target, then remove the source directory.

## Motivation

An adopted corpus accumulated small overlapping specs — work that was never a separate concern from a sibling, or that a later spec absorbed outright. The framework had no operation for that. The reflex was to delete the redundant directory, and deletion re-points nothing: every inbound pointer was stranded, and nothing reported it.

The mechanism to do it properly already existed and was exercised by fold-back. `rewrite-spec-links` re-points every inbound pointer at a target; `retire-feature` removes the directory and refuses when the target holds no `spec.md`, so content is never stranded. What was missing was a command that reached them for a **sequential** spec rather than a branch-scoped staging one — `retire-feature` refused the sequential form outright, because a sequential spec is completed rather than removed.

## Consolidation

Consolidation removes a spec directory whose content belongs with another.

- `/{project}:consolidate <spec> --into <spec>` is a **cleanup command**, in the family of [041 — Task pruning](../041-task-pruning/spec.md): operator-initiated, confirmed, not a pipeline state transition, with git history as the only recovery.
- It performs **none** of fold's content migration — no body edit, no scenario creation, no task append, no status change, no review invalidation. Fold moves content because a staging spec was never meant to stand alone; consolidation assumes the merge already happened or that nothing needed merging.
- `retire-feature`'s refusal of the sequential directory form is **relaxed** here for an explicitly targeted consolidation — not removed. The anti-stranding guard is unchanged, and `/{project}:fold` still cannot reach a sequential directory.
- Because it does not migrate content, the confirmation must name **content loss**, not merely directory removal: the guard proves the target exists, never that anything landed there.

**The source spec's scenarios go with its directory.** Consolidation migrates nothing, so a scenario under the removed spec is destroyed along with everything else in it — deliberately unlike fold-back, which creates one scenario under the upstream spec for every scenario the retiring spec carried. The confirmation names them explicitly rather than leaving them inside a general claim about content: a scenario is a distinct artifact with its own open-question gate, and an operator who reads "the spec is removed" does not necessarily picture them.

Adding `--into` to `/{project}:fold` was considered and rejected. Fold's purpose, its enumeration step, its post-merge instruction, and its single-source rule for the fold target are all specific to the branch-scoped staging form; carrying a sequential path through them would qualify seven load-bearing statements and erode the one thing fold says clearly.

## One-spec and two-spec commands

Consolidation is its own command rather than a flag, and the reason is scope rather than taste. The commands split on how many specs they write: `amend`, `prune`, `clarify`, `plan`, and `implement` each write one and each declares that single-spec scope. `fold` and `consolidate` write two, so neither fits inside a single-spec command as a flag — widening one to accommodate a two-spec operation qualifies every statement it makes about its own scope.

That distinction was undocumented, and it is the thing that explains why several operations are separate commands rather than flags. The README states it, and names which commands sit on each side.

## Interruption and re-runs

Consolidation writes two specs, and the runtime provides no transaction spanning them — the same condition fold-back documents, where the recovery for an interruption is not a rollback but a second run. The command inherits that contract, so each write is built so a re-run is a no-op where the first attempt already landed.

An already-applied step reports that outcome as a domain result rather than a failure, matching `retire-feature`'s already-absent and `invalidate-review`'s already-invalidated.

## Acceptance Criteria

- [x] AC7: `/{project}:consolidate <spec> --into <spec>` re-points every inbound pointer to the source directory at the target, then removes the source directory
- [x] AC8: `/{project}:consolidate` refuses when the target directory holds no `spec.md`, and no pointer is rewritten by the refused run
- [x] AC9: `/{project}:consolidate` writes nothing to the target spec's body, scenarios, `tasks.md`, `status`, or `review:` block
- [x] AC10: The confirmation prompt shown before a consolidation names the loss of the source spec's content, not only the removal of its directory
- [x] AC11: `retire-feature` accepts a sequential feature directory only through an explicitly targeted consolidation, and `/{project}:fold` remains unable to reach one
- [x] AC12: `/{project}:fold` gains no flag or argument for naming a fold target, and its `folds-into:` single-source rule is unchanged
- [x] AC25: `/{project}:consolidate` is installed into adopter projects by the bootstrap, and its documentation identifies it as the only command that removes a durable artifact
- [x] AC34: The README states which commands write to one spec and which write to two, and places `fold` and `consolidate` in the two-spec group
- [x] AC39: Spec 051's account of `retire-feature`'s sequential-form refusal is updated to record that this spec relaxes it for an explicitly targeted consolidation
- [x] AC42: Consolidation destroys the source spec's scenarios with its directory and migrates none of them to the target, and the confirmation names the scenarios individually
- [x] AC44: An interrupted consolidation converges when re-run, each step reporting an already-applied outcome as a domain result rather than a failure

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Should consolidation verify that the target covers the source's claims before removing it?** No, and not behind a flag either. The comparison is real work an operator sometimes wants — the removal is irreversible and the guard proves only that the target *exists* — but it is semantic judgment over two documents, which is precisely what the agent running the command already does well when asked. Building it as a step would put a slow pairwise read in front of every consolidation to serve the minority that wants it, and building it as an opt-out flag would do the same while adding a surface. The command states the non-goal where an operator meets it, and points at asking for the comparison in the same conversation, before the confirmation.
- **Is `/{project}:consolidate` adopter-facing or maintainer-only?** Adopter-facing. The `/{project}:prune` precedent is weaker than it looks — prune destroys `tasks.md`, which the project classes as ephemeral work-tracking, whereas `spec.md` is a durable source of truth — so the question is right to flag the asymmetry. What decides it is the alternative: this feature exists because an adopting corpus needed cleaning up, and a maintainer-only command does not prevent an adopter removing specs, it only guarantees they do it with `rm -rf` — no pointer rewriting, no anti-stranding refusal, no confirmation naming what is lost. Safety comes from the guards, not from restricting distribution. The documentation carries the one thing the maintainer-only reading was right about: this is the only command that removes a durable artifact, and it should not read like an ordinary table row.
