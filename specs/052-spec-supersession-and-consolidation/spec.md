---
status: in-progress
dependencies: [041-task-pruning]
next-criterion: 46
---

# 052 — Spec consolidation

Give the corpus a way to remove a spec whose content belongs with another: re-point every inbound pointer at the target, then remove the source directory.

## Motivation

An adopted corpus accumulated small overlapping specs — work that was never a separate concern from a sibling, or that a later spec absorbed outright. The framework had no operation for that. The reflex was to delete the redundant directory, and deletion re-points nothing: every inbound pointer was stranded, and nothing reported it.

The mechanism to do it properly already existed and was exercised by fold-back. `rewrite-spec-links` re-points every inbound pointer at a target; `retire-feature` removes the directory and refuses when the target holds no `spec.md`, so content is never stranded. What was missing was a command that reached them for a **sequential** spec rather than a branch-scoped staging one — `retire-feature` refused the sequential form outright, because a sequential spec is completed rather than removed.

## Consolidation

Consolidation removes a spec directory whose content belongs with another.

- `/{project}:consolidate <spec> --into <spec>` is a **cleanup command**, in the family of [041 — Spec directory pruning](../041-task-pruning/spec.md): operator-initiated, confirmed, not a pipeline state transition, with git history as the only recovery.
- It performs **none** of fold's content migration — no body edit, no scenario creation, no task append, no status change, no review invalidation. Fold moves content because a staging spec was never meant to stand alone; consolidation assumes the merge already happened or that nothing needed merging.
- `retire-feature`'s refusal of the sequential directory form is **relaxed** here for an explicitly targeted consolidation — not removed. The anti-stranding guard is unchanged, and `/{project}:fold` still cannot reach a sequential directory.
- Because it does not migrate content, the confirmation must name **content loss**, not merely directory removal: the guard proves the target exists, never that anything landed there.

**The source spec's scenarios go with its directory.** Consolidation migrates nothing, so a scenario under the removed spec is destroyed along with everything else in it — deliberately unlike fold-back, which creates one scenario under the upstream spec for every scenario the retiring spec carried. The confirmation names them explicitly rather than leaving them inside a general claim about content: a scenario is a distinct artifact with its own open-question gate, and an operator who reads "the spec is removed" does not necessarily picture them.

Adding `--into` to `/{project}:fold` was considered and rejected. Fold's purpose, its enumeration step, its post-merge instruction, and its single-source rule for the fold target are all specific to the branch-scoped staging form; carrying a sequential path through them would qualify seven load-bearing statements and erode the one thing fold says clearly.

## One-spec and two-spec commands

Consolidation is its own command rather than a flag, and the reason is scope rather than taste. The commands split on how many specs one operation writes: `amend`, `prune`, `clarify`, `plan`, and `implement` each operate on one spec and each declares that single-spec scope. A batch flag that repeats the same one-spec operation for every spec in the corpus — `analyze --all`, `prune --all` — leaves that scope intact, since each write it makes is still one spec's. `fold` and `consolidate` differ in kind: one operation writes two specs, so neither fits inside a single-spec command as a flag — widening one to accommodate a two-spec operation qualifies every statement it makes about its own scope.

> **Signpost:** the batch-flag distinction was drawn when [041 — Spec directory pruning](../041-task-pruning/spec.md) gave `/prune` an `--all` flag, which repeats prune's one-spec reduction for every spec rather than writing two specs in one operation.

That distinction was undocumented, and it is the thing that explains why several operations are separate commands rather than flags. `docs/slash-commands.md` states it, and names which commands sit on each side.

## Interruption and re-runs

Consolidation writes two specs, and the runtime provides no transaction spanning them — the same condition fold-back documents, where the recovery for an interruption is not a rollback but a second run. The command inherits that contract, so each write is built so a re-run is a no-op where the first attempt already landed.

An already-applied step reports that outcome as a domain result rather than a failure, matching `retire-feature`'s already-absent and `invalidate-review`'s already-invalidated.

## Acceptance Criteria

> **AC7 is completed by [051 — Branch-scoped spec numbering](../051-branch-scoped-spec-numbering/spec.md)** (2026-10-27). `rewrite-spec-links`, which `/{project}:consolidate` shares with `/{project}:fold`, re-pointed links only in the `.md` files under the spec root, so a pointer from `README.md`, `AGENTS.md`, `CLAUDE.md` or `.githooks/pre-commit` survived a consolidation. 051's task 29 extends the rewrite to those four files.

- [x] AC7: `/{project}:consolidate <spec> --into <spec>` re-points every inbound pointer to the source directory at the target, then removes the source directory. A referrer `rewrite-spec-links` cannot rewrite safely (a symbolic link, one whose parent resolves outside the project, or one that is not UTF-8 text) is left as it is and named in the result's `skipped`, which the command reports; `check-orphaned-references` reports what it still names, or lists it as unreadable when it is not UTF-8. A symbolic link under the spec root to a markdown file or a directory is left as it is and named there too, and the pre-commit hook's `check-corpus-links` names a link in it that no longer resolves
- [x] AC8: `/{project}:consolidate` refuses when the target directory holds no `spec.md`, and no pointer is rewritten by the refused run
- [x] AC9: `/{project}:consolidate` writes nothing to the target spec's body, scenarios, `tasks.md`, `status`, or review record (`review.md`; the `review:` block before spec 057)
- [x] AC10: The confirmation prompt shown before a consolidation names the loss of the source spec's content, not only the removal of its directory
- [x] AC11: `retire-feature` accepts a sequential feature directory only through an explicitly targeted consolidation, and `/{project}:fold` remains unable to reach one
- [x] AC12: `/{project}:fold` gains no flag or argument for naming a fold target, and its `folds-into:` single-source rule is unchanged
- [x] AC25: `/{project}:consolidate` is installed into adopter projects by the bootstrap, and its documentation identifies it as the only command that removes a durable artifact
- [x] AC34: `docs/slash-commands.md` states which commands write to one spec and which write to two, and places `fold` and `consolidate` in the two-spec group
- [x] AC39: Spec 051's account of `retire-feature`'s sequential-form refusal is updated to record that this spec relaxes it for an explicitly targeted consolidation
- [x] AC42: Consolidation destroys the source spec's scenarios with its directory and migrates none of them to the target, and the confirmation names the scenarios individually
- [x] AC44: An interrupted consolidation converges when re-run, each step reporting an already-applied outcome as a domain result rather than a failure

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Why split from 051-branch-scoped-spec-numbering:** the two are separate commands with separate contracts. 051's `/{project}:fold` moves a branch-scoped staging spec's content into its upstream home; this spec's `/{project}:consolidate` merges one sequential spec into another. They share `rewrite-spec-links` and `retire-feature`, so their Affected Files overlap, and this spec reopened on 2026-10-27 only for the cross-spec impact of 051's task 29, which extended the shared rewrite. Folding them into one would merge two lifecycles to record one shared primitive. Resolved 2026-10-27.
- **Should consolidation verify that the target covers the source's claims before removing it?** No, and not behind a flag either. The comparison is real work an operator sometimes wants — the removal is irreversible and the guard proves only that the target *exists* — but it is semantic judgment over two documents, which is precisely what the agent running the command already does well when asked. Building it as a step would put a slow pairwise read in front of every consolidation to serve the minority that wants it, and building it as an opt-out flag would do the same while adding a surface. The command states the non-goal where an operator meets it, and points at asking for the comparison in the same conversation, before the confirmation.
- **Is `/{project}:consolidate` adopter-facing or maintainer-only?** Adopter-facing. The `/{project}:prune` precedent is weaker than it looks — prune removes only what the framework classes as not durable, `tasks.md` sections (ephemeral work-tracking) and the `plan.md` sections outside its design record once their durable pieces have moved home, whereas `spec.md` is a durable source of truth — so the question is right to flag the asymmetry. What decides it is the alternative: this feature exists because an adopting corpus needed cleaning up, and a maintainer-only command does not prevent an adopter removing specs, it only guarantees they do it with `rm -rf` — no pointer rewriting, no anti-stranding refusal, no confirmation naming what is lost. Safety comes from the guards, not from restricting distribution. The documentation carries the one thing the maintainer-only reading was right about: this is the only command that removes a durable artifact, and it should not read like an ordinary table row.
