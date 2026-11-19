---
next-task: 39
---

# 051 — Branch-scoped spec numbering Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Spent task sections are pruned per §tasks-phase; git history holds them, and the plan's Technical Decisions hold the decisions they settled.

## 37. invalidate-review and rewrite-spec-links guard their read-modify-write (BE-TXN-002, owner decision)

- [x] `invalidate-review`'s rewrite of `review.md` and `rewrite-spec-links`' rewrite of each referrer guard the window between reading the file and renaming the result over it with the compare-before-persist helper 041's task 27 adds to `runtime/src/primitives/mod.rs`: the file is read again just before the rename, and the write is refused when it changed since the primitive read it. `rewrite-spec-links` reads every pending file before its first write, so today its window spans the whole sweep; a referrer it refuses is named in the result beside the ones it wrote, so `/{project}:fold` and `/{project}:consolidate` can report a sweep that did not complete. The owner decided on 2026-11-16 to close the BE-TXN-002 class by fixing each owner's primitives rather than waiving them, reopening this spec. Run after 041's task 27 lands
- [x] 051's data model states what a refused write reports for each primitive and its plan names the guard; `INVALIDATE_REVIEW`'s and `REWRITE_SPEC_LINKS`' descriptions in `runtime/src/schema/descriptions.rs` state it
- [x] A test per primitive in which a file changes between its read and its write shows the write refused and the other writer's change kept, each failing under its mutant

- **Done when**: Neither primitive overwrites a change made between its read and its write, and a refused referrer is named in the result, proven by tests their mutants turn red; the whole `cargo test` passes; and 051's next review reads the change.

## 38. A definition and a title follow the link grammar (scenario a-definition-and-a-title-follow-the-link-grammar)

- [ ] Implement the behavior described in `scenarios/a-definition-and-a-title-follow-the-link-grammar.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested
