---
section: "Consolidation"
---

# The-content-move-and-the-consolidation-commit-together

## Context

An adopter consolidating one sequential spec into another merged the source's content into the target first, as the command assumes, with `git mv` for each scenario and for `data-model.md`. The move left every link to a moved file dangling: in the observed run, 12 inside the source directory (its `spec.md`, `plan.md` and `tasks.md` linking `scenarios/{slug}.md`) and 8 in other specs linking `../{source}/scenarios/{slug}.md`. The shipped pre-commit hook runs `check-corpus-links` over the spec root and refuses a commit holding a dangling link. So the move could not be committed on its own, an operator who wanted it reviewed and committed before the removal could not do that, and `consolidate.md` said nothing about the order.

A probe on 2026-10-29 (the shipped hook, this repository's release build, an adopter-shaped tree under `/tmp`) found that the order alone does not rescue the run. When the target links the source, as a target absorbing one usually does, step 4 re-pointed the target's own link through `../{target}/`. `derive-dependencies` recorded that as a 1-cycle, and the hook refused the consolidation commit too, whether the content had been moved or copied. The rewrite is `rewrite-spec-links`, which 051 owns, and its fix is 051's scenario [a link inside the target stays inside it](../../051-branch-scoped-spec-numbering/scenarios/a-link-inside-the-target-stays-inside-it.md).

The same report found three smaller gaps:

- A re-pointed link's label still names the removed spec.
- A link into the source's `tasks.md` by task number lands on the target's `tasks.md`, where that number is another task.
- Plain-text mentions of the source, by slug or number, are left as written. That is correct, since the command re-points pointers, but it went unreported, so the operator could not tell that a sweep was owed.

## Behavior

- `consolidate.md` states the order:
  1. Merge the content into the target, with body edits and `git mv` for each scenario and data model the target is to hold.
  2. Run the command.
  3. Commit the move and the consolidation as one commit.

  It says why: a moved file leaves every link to it dangling until step 4 re-points the inbound ones and step 5 removes the source's own, and the pre-commit hook refuses a commit with a dangling link.
- It names the two ways to look at the work before the removal lands:
  - **Review the uncommitted tree.** Nothing the command writes is committed until the operator commits, and git restores the source directory from the last commit. Edits to the source that were never committed are the exception: nothing recovers them.
  - **Commit a checkpoint.** Copy rather than move, so every link still resolves and the merge commits on its own. The consolidation then re-points the inbound links onto the copies and removes the originals. The cost is git's rename pairing, which holds only within one commit, so `git log --follow` on a copied scenario stops at the checkpoint.
- Step 2 names, for each scenario, whether the target holds one of the same file name, so an operator who copied can see what landed by name. A name match proves nothing about content, so the confirmation still names each scenario as destroyed.
- Step 7 reports `rewrite-spec-links`' `mentions`, meaning each line that still names the source by its directory name or its number, as the sweep the operator owes: a present-tense pointer to the removed spec is corrected, and a past-tense record of it stays. It also reports two kinds of `kept-tails` reference:
  - one into `tasks.md`, whose task number now means one of the target's tasks, to re-point at the right task or turn into prose;
  - one naming a file the target does not hold, which the hook refuses until it is fixed.
- AC9 says what the command writes into the target: the links the target held to the source, re-pointed relative to the file that holds them, and nothing else.

## Edge Cases

- **A kept tail names a file the target does not hold**, for example `../{source}/tasks.md` re-pointed at a target with no `tasks.md`: the link dangles and the hook refuses the commit, loudly.
- **Nothing was moved**: no link dangles before step 4, and the order still holds, since committing follows the command.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Should consolidation move the source's scenarios and data model itself?** No. Moving a scenario writes the target's scenarios (AC9), reopens a `done` target through the scenario back-edge, and owes a task for each. It also leaves each scenario's `section:` naming a section of the source, and choosing which target section it now elaborates is a semantic judgment. That is `/{project}:fold`'s content migration, which this spec keeps out of consolidation deliberately (§Consolidation). Resolved 2026-10-29.
- **Should the rewrite run before the content move, or re-point links to files already moved, so that a move commit passes the hook?** No. A move commit also needs the source's own links repaired, which means writing into the directory about to be removed through a second mode of the shared rewrite. All it would buy over the copy checkpoint is git's rename pairing in the checkpoint commit, and the copy checkpoint and the uncommitted-tree review give a reviewed checkpoint without it. Resolved 2026-10-29.
- **Should the rewrite change a link label that is exactly the source's slug, or report it?** Report it, through `mentions`. A label is prose: changing `[010-src]` to `[020-tgt]` changes what the sentence claims, which is right for a present-tense pointer and wrong for a record of what the source decided. A rewrite that guesses wrong is worse than a precise report, which is the stance `check-corpus-links` takes. Resolved 2026-10-29.
- **Should a link into the source's `tasks.md` be left dangling instead of re-pointed?** No. `rewrite-spec-links` re-points every tail and lists the ones a fold did not write in `kept-tails`, by 051's decision, and that list was unreleased at `ductus-v0.59.11`, which is why the adopter saw the re-pointing as silent. A dangling link would block the commit on a sentence the operator has to rewrite by hand either way, so the report names a `tasks.md` kept tail as a task-number reference, and the operator fixes the sentence as well as the path. Resolved 2026-10-29.
