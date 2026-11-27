---
next-task: 57
---

# 050 — Constitution Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 48. A citation names no line number: the rule and its pointers (AC24, owner decisions 2026-11-17)

- [x] §grounding's *Cite what you consulted* in `framework/constitution.md` names the forms a citation takes (a file with a symbol, test name, heading or quoted phrase) and never a line number, keeping its position and anchor, with the constitution within the 50,000-byte ceiling `scripts/lint-governing-size.sh` holds
- [x] `framework/commands/plan.md`, `framework/reference/plan.md` and `framework/reference/clarify.md` point at the bullet in place of their own "file and symbol, never a bare line number"; no live artifact restates the old wording
- [x] `AGENTS.md`'s *A `path:line` citation in a spec body is provenance* gotcha becomes a pointer to the bullet

- **Done when**: The constitution states the rule once, every restatement points at it, `scripts/lint-governing-size.sh` and the audit pass, and 050's next review reads the change.

## 49. The spec root holds no line-number citation (AC24, owner decisions 2026-11-17)

- [x] Runs after task 48, and with 066's task 25 and 067's task 37 in the built binary, which classify the strip and find each citation. Each citation whose line already names the symbol, heading or phrase it points at has its line suffix stripped, in a commit holding only strips
- [x] Each other citation, and each prose line reference, is rewritten to name what it pointed at in the commit that wrote it, read with `git blame` and `git show`
- [x] Each `done` spec the rewrite reopens, as `classify-spec-edit` judges it, moves to `in-progress` with `set-status`, its analysis owed

- **Done when**: `check-artifacts` reports no `line-number-citations` finding for any spec, `classify-spec-edit` reads the strip commit as mechanical for each spec it alone touches, and each reopened spec is `in-progress` with its analysis owed.

## 50. The Guiding Principles carry no Technology or Business list (AC25, owner decision 2026-11-17)

- [x] `framework/constitution.md`'s `## Guiding Principles` drops its opening sentence and its Technology and Business lists, keeping the `<!-- §principles -->` marker, the heading, §cost-levers and §design-principles, within the ceiling `scripts/lint-governing-size.sh` holds
- [x] 050's `succinctness-is-a-guiding-principle` scenario is deleted, `the-constitution-fits-one-read`'s Behavior stops naming the principles among what the constitution keeps, and each `plan.md` statement about the lists' current content is corrected; a figure measured at a named commit stays
- [x] Each live artifact that describes a removed principle as current is corrected, found by meaning rather than by token (§drift-prevention): `008-security-rules`' AC28 is annotated as superseded by this spec, and present-tense prose in specs including 008, 010, 016, 037, 039 and 077 is corrected, while a decision record or motivation stating what the constitution said then keeps its words
- [x] Each `done` spec the edits reopen, as `classify-spec-edit` judges it, moves to `in-progress` with `set-status`, its analysis owed

A read-only sweep on 2026-11-18, at `6f1286dc`, by meaning and by every principle name, found what the third box owes; re-derive it if the corpus has moved. To correct: 050's own scenario deletion, `the-constitution-fits-one-read` ("the principles" names the design principles) and the `plan.md` sentence calling the two lists byte-identical; 008's `spec.md` Motivation and Constitution Reference, its `plan.md` Overview, "currently reads:" quote and Affected Files row, and AC28's annotation; 010's `spec.md` "(§principles → Business)" and its `plan.md` "following the `### Business` principles list"; 037's `spec.md` "the principles list calls the system to be Supportable and Observable"; 039's `spec.md` Motivation and BULK Resolved Question and its `plan.md` "reliability principle"; and 077's `plan.md`, twice ("the Secure principle names", the Affected Files row). Each edit outside 008 and 050 is to a `spec.md` body or `plan.md`, so no `done` spec reopens; confirm with `classify-spec-edit` before committing. 010's AC11 needs no edit, since its "or a new dedicated subsection" branch, `### Cost levers`, stays. 016's naming Resolved Question, which this sweep missed, was corrected too. Kept as records: 072's past-tense motivation, 008's rejected alternative, 010's Resolved Question 6 and plan history, 050's §principles size row and cut account, and each `review.md` and `analysis.md` naming the deleted scenario.

- **Done when**: `framework/constitution.md` has no `### Technology` or `### Business` heading, `scripts/lint-governing-size.sh` and `scripts/audit/run-all.sh` pass, no live artifact describes a removed principle as current, and each reopened spec is `in-progress` with its analysis owed.

## 51. A decision is attributed to whoever made it (scenario a-decision-is-attributed-to-whoever-made-it)

- [x] Implement the behavior described in `scenarios/a-decision-is-attributed-to-whoever-made-it.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 52. A primitive is probed in a local clone (scenario a-primitive-is-probed-in-a-local-clone)

- [x] Implement the behavior described in `scenarios/a-primitive-is-probed-in-a-local-clone.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 53. A task uses what it needs, and no more (AC26, owner decision 2026-11-23)

- [x] `framework/constitution.md` §design-principles ends with the bullet the plan's **A task uses what it needs, and no more** names, and §Cost levers drops its `/cost` example, leaving the file within 50,000 bytes (`scripts/lint-governing-size.sh`)
- [x] `AGENTS.md` drops 40dea24c's restatement of the filter and restores the Design Principles bullet it displaced, so audit Family 38 finds survey DP1's entry again
- [x] `scripts/audit/run-all.sh` exits 0 after committing, and `cargo test` passes, since `framework/constitution.md` is a `runtime.yml` trigger

- **Done when**: the constitution carries the filter within its ceiling, `AGENTS.md` carries no copy of it, and the audit and `cargo test` pass

## 54. The constitution says a superseded requirement is never kept (AC27, owner decision 2026-11-27)

- [x] Add AC27's rule to `framework/constitution.md` §drift-prevention's Decision resolution, with an equal cut elsewhere so the file stays within the 49,000-byte ceiling `scripts/lint-governing-size.sh` holds it to
- [x] Correct every live artifact that keeps a checked task as history, so none states the opposite of AC27

- **Done when**: §drift-prevention's Decision resolution states AC27's rule, `framework/constitution.md` is at most 49,000 bytes by `wc -c`, its anchors are unchanged, and `scripts/audit/shipped-anchor-resolution.sh` passes

## 55. Every spec's spent task sections are pruned (AC27, owner decision 2026-11-27)

- [ ] Run `/ductus:prune --all`, keeping pending tasks, so no checked task's text survives in any spec: 612 checked boxes across the 83 `tasks.md` files at `cfb0bfd9`, by `grep -c '^\s*- \[x\]'` over each
- [ ] Message the other session before pruning a spec it is working, 066 among them

Only `tasks.md` is pruned: `/ductus:prune --all`'s `plan.md` and `review.md` reductions are not applied (the agent's recommendation, adopted by the operator, 2026-11-27). 066's prune waits until the other lane reports 066 `done`, its request of 2026-11-27, since a prune while it is `in-progress` stales its analysis.

- **Done when**: no `specs/*/tasks.md` holds a checked task section, every pending task is kept, each pruned file carries a `next-task:` counter past its highest number, and `scripts/audit/run-all.sh` passes after the commit

## 56. Each ticked criterion with a Superseded note states its current requirement (AC27, owner decision 2026-11-27)

- [ ] Rewrite each ticked criterion carrying a Superseded note to the requirement that holds now, removing the note, or delete it where its behaviour is gone: 179 criteria in 48 specs at `cfb0bfd9`, the ticked `AC{n}:` lines of `specs/*/spec.md` matching `uperseded`
- [ ] Ask the owner, one criterion at a time, where the current requirement cannot be read from the note and its source; message the other session before editing a spec it is working

The set is every ticked criterion carrying such a note in any wording, not only `uperseded`: 59 more candidates in 31 specs at `b92e7d20` read "Narrowed by", "since retired", "no longer holds", "Changed by" or "replaced by" (the agent's recommendation, adopted by the operator, 2026-11-27). 066's are the other lane's, which rewrites them before 066's review. The links back to 020 in 024's AC2, AC3 and AC6, 034's, 037's, 038's and 039's AC7, 068's AC10, 069's AC11 and 070's AC9 discharge 020's `cross-spec-impact:`, so a rewrite keeps them.

- **Done when**: no ticked criterion in `specs/*/spec.md` carries a Superseded note, each rewritten criterion holds what its spec requires now, and `check-owed-work` reads each touched spec as owing what it owed before
