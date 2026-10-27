---
section: "Fold-back on merge"
---

# A-pending-fold-never-holds-done

## Context

AC35 made a declared fold target block the pre-`done` gate, on the reasoning that a staging spec carrying `folds-into` still owes work. Scenario [a-fold-owed-to-another-tree-does-not-hold-done](a-fold-owed-to-another-tree-does-not-hold-done.md) narrowed the block to a target this tree holds, but kept it there: `check-review-gate` answers `pending-fold` whenever the target resolves here, and `/{project}:status` names `/{project}:fold` as the next action at every status, `in-progress` included.

That ordering is backwards, and an adopting project followed it faithfully: it read the framework as requiring a branch-scoped spec to be folded before it could be `done`. The intended order runs the other way. The staging spec completes its own pipeline — implemented, reviewed, analyzed — and reaches `done` like any other spec; fold-back then consolidates its durable content into the upstream spec. `done` says the spec's own work is finished. The fold is what remains before the work is complete, and it is upstream consolidation, not unfinished implementation.

A pending fold is therefore not in the category of an unresolved scenario question or an undischarged cross-spec impact. Those are claims the spec cannot yet make truthfully; a pending fold is a claim the spec makes truthfully, in a place it will not stay. And unlike an impact, the owed fold is carried past `done` by surfaces that already exist: `check-unfolded-specs`, and the pipeline view's `(fold pending)` qualification and callout.

## Behavior

- **The pre-`done` gate has no fold check.** `check-review-gate` never blocks on `folds-into`, wherever its target lives: a spec carrying the key passes or blocks on exactly the checks a spec without it does. `pending-fold` leaves the gate's block vocabulary rather than becoming unreachable.
- **Done first, then fold.** Below `done`, a spec carrying `folds-into` shows the pipeline view's ordinary next action for its status; at `done` it shows `/{project}:fold → {target}`. The scenario-question and recovery overrides keep their precedence over both.
- **The fold stays owed and visible.** The Status cell is still qualified `(fold pending)` at every status, the outstanding-fold callout still lists every spec carrying `folds-into`, and `check-unfolded-specs` still reports it until fold-back retires the directory, so AC34 holds as written.
- **`/{project}:fold` still accepts a staging spec at any status, and its writes do not change.** Folding into a `done` upstream spec still reopens it and invalidates its review (AC24, AC25), because the upstream spec gains content its recorded review never saw.
- **The cross-spec-impact gate is unchanged**, `target-missing` included. Its justification stands on its own reason — nothing but the gate carries an undischarged impact past `done` — rather than on the fold check as precedent.
- **The rule changes everywhere it is stated**: the constitution's §spec-lifecycle (the `done` row and the branch-scoped paragraph); this spec's AC35, §Fold-back on merge, Resolved Questions and the earlier scenario; `/{project}:implement`'s gate text, `/{project}:fold`, `/{project}:status`, and `/{project}:review`'s restatement of the gate order; the user docs; and 066's and 050's records of the gate.

## Edge Cases

- **A staging spec already `done` in a tree holding its target.** The gate reports that there is no transition to gate, as before, and the fold is the next action.
- **A staging spec folded before it reaches `done`.** Still possible: the content, scenarios and tasks move upstream as they always have. It is no longer what the framework steers toward.
- **A `folds-into` naming no spec anywhere.** Does not block `done`, as before; fold-back refuses it (AC28), and `/{project}:status` calls it out as not in this tree on every run.
- **Host tooling matching `blocked-by: pending-fold`.** The value no longer occurs; nothing else in the result's shape changes.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Does a fold into a `done` upstream spec still reopen it?** Yes, by operator decision on 2026-09-29. The reopen is the upstream spec's back-edge, not the staging spec's gate: the upstream spec gains content, so its recorded review has to see it. Dropping the reopen as well would treat fold-back as filing only, and was declined.
