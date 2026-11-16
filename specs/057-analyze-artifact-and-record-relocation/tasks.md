---
next-task: 28
---

# 057 — Analyze artifact and record relocation Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Task 1 is first by constitutional rule, not by preference: §runtime-boundary
principle 4 requires a schema change to ship through the constitution with the
runtime following. Tasks 2-8 are the runtime; 9-12 the migration and the gates
around it; 13-14 the sweeps; 15-16 verification and the declared obligations.

## 27. relocate-audit-records guards its read-modify-write (BE-TXN-002, owner decision)

- [ ] `relocate-audit-records`' rewrite of `review.md` or `analysis.md`, merged with the existing artifact, and its rewrite of `spec.md` without the relocated block guard the window between reading each file and renaming the result over it with the compare-before-persist helper 041's task 27 adds to `runtime/src/primitives/mod.rs`: the file is read again just before the rename, and the write is refused when it changed since the primitive read it; a refusal after the artifact's write says which file was written, so the migration can report a relocation left half done. The owner decided on 2026-11-16 to close the BE-TXN-002 class by fixing each owner's primitives rather than waiving them, reopening this spec. Run after 041's task 27 lands
- [ ] 057's data model states what a refused write reports and its plan names the guard; `RELOCATE_AUDIT_RECORDS`' description in `runtime/src/schema/descriptions.rs` states it
- [ ] A test in which each file changes between its read and its write shows the write refused and the other writer's change kept, each failing under its mutant

- **Done when**: No relocation overwrites a change made between its read and its write, proven by tests their mutants turn red; the whole `cargo test` passes; and 057's next review reads the change.
