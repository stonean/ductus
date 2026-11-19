---
next-task: 29
---

# 057 — Analyze artifact and record relocation Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 27. relocate-audit-records guards its read-modify-write (BE-TXN-002, owner decision)

- [x] `relocate-audit-records`' rewrite of `review.md` or `analysis.md`, merged with the existing artifact, and its rewrite of `spec.md` without the relocated block guard the window between reading each file and renaming the result over it with the compare-before-persist helper 041's task 27 adds to `runtime/src/primitives/mod.rs`: the file is read again just before the rename, and the write is refused when it changed since the primitive read it; a refusal after the artifact's write says which file was written, so the migration can report a relocation left half done. The owner decided on 2026-11-16 to close the BE-TXN-002 class by fixing each owner's primitives rather than waiving them, reopening this spec. Run after 041's task 27 lands
- [x] 057's data model states what a refused write reports and its plan names the guard; `RELOCATE_AUDIT_RECORDS`' description in `runtime/src/schema/descriptions.rs` states it
- [x] A test in which each file changes between its read and its write shows the write refused and the other writer's change kept, each failing under its mutant

The migration `framework/migrations/audit-record-relocate.md` is not edited to name the new errors (owner, 2026-11-16): the error text names the files written and refused and says a re-run is safe, an agent running the step reports a primitive's error, and only an adopter upgrading from before 057 can reach it, since one who ran the migration never runs it again and a fresh adoption has no blocks to relocate.

- **Done when**: No relocation overwrites a change made between its read and its write, proven by tests their mutants turn red; the whole `cargo test` passes; and 057's next review reads the change.

## 28. A merge carries each key's whole value

- [ ] Implement the behavior described in `scenarios/a-merge-carries-each-keys-whole-value.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested
