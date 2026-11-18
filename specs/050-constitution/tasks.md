---
next-task: 51
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

- [ ] `framework/constitution.md`'s `## Guiding Principles` drops its opening sentence and its Technology and Business lists, keeping the `<!-- §principles -->` marker, the heading, §cost-levers and §design-principles, within the ceiling `scripts/lint-governing-size.sh` holds
- [ ] 050's `succinctness-is-a-guiding-principle` scenario is deleted, `the-constitution-fits-one-read`'s Behavior stops naming the principles among what the constitution keeps, and each `plan.md` statement about the lists' current content is corrected; a figure measured at a named commit stays
- [ ] Each live artifact that describes a removed principle as current is corrected, found by meaning rather than by token (§drift-prevention): `008-security-rules`' AC28 is annotated as superseded by this spec, and present-tense prose in specs including 008, 010, 016, 037, 039 and 077 is corrected, while a decision record or motivation stating what the constitution said then keeps its words
- [ ] Each `done` spec the edits reopen, as `classify-spec-edit` judges it, moves to `in-progress` with `set-status`, its analysis owed

- **Done when**: `framework/constitution.md` has no `### Technology` or `### Business` heading, `scripts/lint-governing-size.sh` and `scripts/audit/run-all.sh` pass, no live artifact describes a removed principle as current, and each reopened spec is `in-progress` with its analysis owed.
