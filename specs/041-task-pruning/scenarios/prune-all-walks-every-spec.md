---
section: "Behavior"
---

# Prune-all-walks-every-spec

## Context

This spec scopes prune to the session target, and that stays the right default: the target is the spec being worked. But a corpus-wide cleanup then costs one `/{project}:target` plus one `/{project}:prune` per spec, which is tedious enough that it does not happen. And the corpus already holds work for it — `plan-records-the-design-as-it-stands` counts four journal-shaped plans among 89, before counting a single spent `tasks.md` section.

## Behavior

**`/{project}:prune --all` runs the same reduction over every spec in the corpus.** It needs no session target and leaves any target that is set unchanged. It walks every feature directory — sequential and branch-scoped alike — in corpus order, applying `prune-reduces-the-spec-directory` to each.

**Both halves are walked by default: every `tasks.md` and every plan.** There is no flag that adds plans and none that removes them; declining a plan section at its confirmation is how the operator leaves a plan alone.

**One preview for the corpus, then decisions at the grain their content needs.** The preview is a table with one row per spec that has anything to reduce — its status, what each artifact would lose, and the sections kept because `analysis.md` already decided them — plus a total. Status is what prices a reopen: only a `done` spec can be reopened. A reopen the tree already carries, from edits made before the run, is named on that spec's row, which a `done` spec gets for that reopen alone, even with nothing to reduce; one its moves would cause is named at that spec's plan confirmation, once its moves are known — both by `prune-reduces-the-spec-directory`'s diff-visible trigger. The `tasks.md` reductions are deterministic and hold nothing durable, so one confirmation covers all of them. Plan sections are judged one spec at a time, with the per-section confirmation `prune-reduces-the-spec-directory` specifies, because each is a judgment about that spec's own content.

**`--reset` keeps its status gate, applied per spec.** `--all --reset` resets `tasks.md` on every `done` spec and names each spec it skipped, with its status. `--force` is refused alongside `--all`: a forced reset across the corpus would discard every in-flight todo under one confirmation, which is the silent loss [§pipeline-boundaries](../../../framework/constitution.md#pipeline-boundaries) forbids. A deliberate forced reset is still available one spec at a time.

**The completion report names what changed:** per spec, the reductions written, each section moved and where its pieces went, and every spec reopened.

## Edge Cases

- **A spec with nothing to reduce.** Unless it is a `done` spec whose tree already requires a reopen, which gets a row for that alone, it is omitted from the table but counted in the total examined, so a clean corpus reads as *examined N, nothing to prune* rather than as silence.
- **An interrupted run.** Each spec's writes are atomic and each reduction is idempotent, so running `--all` again resumes: sections already removed are simply not found.
- **A spec with no `tasks.md`** — a `draft` or `clarified` spec. There is no task list to reduce; its plan, if it has one, is still examined.
- **A session target is set.** The walk ignores it and leaves it as it was.
- **The first `--all` after the plan advisory ships.** No spec's analysis yet holds a waiver for a plan section, so every design section under another heading is proposed, and declining one keeps it for that run only. The lasting keep is made by running `/{project}:analyze` on those specs and waiving the advisory with its reason, after which later passes list the sections as kept rather than proposing them.

## What this does not do

It changes nothing about how one spec is reduced — every per-spec rule is `prune-reduces-the-spec-directory`'s. The walk itself is runtime work owned here: both `prune-tasks` and `prune-plan` take `all`, and the schema lives in this spec's `data-model.md`.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Does `--all` walk plans by default, or only `tasks.md` unless asked?** Both halves by default — option (a) of three, the others being plans behind a flag (b) and plans only for specs whose last analyze recorded the plan advisory (c). Before it was recorded, the proposal was worked through by reading every section this repository's plans hold outside the design record: a fence-aware `##` scan found them in 8 of 60 plans, 7 of those specs `done`. Three are journals. 051's *Implementation notes* carries twelve design decisions no Technical Decisions entry holds, which move into the record and reopen 051, plus a claim the session-seeded write boundary has since made false. 057's *Implementation notes* and *Resuming from here* are spent or already homed, save one `cargo fmt` trap recorded nowhere else, which moves to `AGENTS.md` without reopening 057. 056's *Post-split measurement* is evidence, removed without a reopen. Two are the results of `/{project}:plan`'s cross-validation step (018's *Cross-spec context*, 022's *Cross-Spec Validation*) — evidence, removed without a reopen. Three are design under other headings (027's and 041's *Known limitations*, 050's *Classification*, which 050's AC1 requires), kept by a discard, or for the two limitations folded into Trade-offs at the price of a reopen. So the plan half costs this repository one reopen by default. The same walk exposed a gap the per-spec rule then closed: prune had no memory of a decline, so the three design sections would have been proposed on every pass, breaking the single-stored-discard cost `plan-records-the-design-as-it-stands` promises — prune now honors analyze's stored discards (`prune-reduces-the-spec-directory`). (b) puts the half that carries durable content behind the flag least likely to be passed; (c) makes the walk depend on every spec having a fresh analyze, which nothing guarantees for a `done` spec. Resolved 2026-09-29.
