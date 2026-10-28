# 051 — Branch-scoped spec numbering Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Spent task sections are pruned per §tasks-phase; git history holds them, and the plan's Technical Decisions hold the decisions they settled.

## 28. Prune plan.md §Implementation notes into the design record

- [x] Move the durable decisions under §Implementation notes into §Technical Decisions or §Trade-offs, and drop the implementation journal, which git history holds
- [x] Re-run /ductus:analyze: the plan-record finding no longer fires

- **Done when**: plan.md has no section outside the design record, every decision the section carried survives in the design record, and analyze no longer reports plan-record for 051

## 29. A fold leaves no in-repo reference dangling (AC6, AC22)

- [x] `rewrite-spec-links` re-points a retiring directory's inbound links only in the `.md` files under the spec root, so a reference to it from `README.md`, `AGENTS.md`, `CLAUDE.md` or `.githooks/pre-commit` survives the fold and `check-orphaned-references` reports it. Never true: the walk and the referrer set were the same at `1657a4d6`, when AC22 was ticked. Found by the 2026-10-27 criteria verification.
- [x] Extend `rewrite-spec-links` to those four files, the referrers `check-orphaned-references` reads outside the spec root, rewriting whatever path token that check would flag, with the same whole-segment matching; the owner chose the shared primitive (2026-10-27), so `/{project}:consolidate` gets it too, which is cross-spec impact on `052-spec-supersession-and-consolidation` (its AC7).
- [x] Correct the prose that bounds the rewrite to the spec root: `framework/commands/fold.md`, `framework/commands/consolidate.md`, `framework/reference/fold.md`, and the `check_orphaned_references.rs` doc comment saying the framework repairs only what it authored.
- [x] Prove each new test fails without the change.

- **Done when**: A fold or consolidation leaves no reference to the retired directory in the spec root or the four referrer files, `check-orphaned-references` reports clean afterwards in a fixture that held one, AC6 and AC22 hold as written, and 052 carries its back-link

## 30. Disposition: a `./` or `../` path to the spec root is invisible to the fold and the orphan check (AC6)

- [ ] Found by task 29's verification (2026-10-27): the path scanner `check-orphaned-references` and `rewrite-spec-links` share counts a `/` before the spec root as a path character, so `[x](./specs/1234.1-staged/spec.md)` in `README.md` is neither re-pointed by a fold nor reported once the directory is gone (probed through the release binary). A root-absolute `[x](/specs/012-auth/spec.md)` is missed the same way (the 2026-10-27 review's probe). Pre-existing and shared by both primitives; AC6 says no reference dangles.
- [ ] The scanner is `067-analyze-and-audit-checks`' surface (scenario `orphaned-reference-check`), which another session holds; whether to route it there or fix it here is the owner's decision.

- **Done when**: the finding is fixed, routed, or discarded, with a discard's reason written on the task
