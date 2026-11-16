---
next-task: 25
---

# 066 — Audit Records and the Completion Gate Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 23. The gate's cross-spec read is contained (BE-INPUT-004, from 067's review)

- [x] `classify_cross_spec_entry` (`runtime/src/primitives/check_review_gate.rs`) reads the affected spec's `spec.md` and its scenarios only after containing each against the project root, through `validate_contained` or `refuse_outward_link` (`runtime/src/primitives/mod.rs`); an affected spec whose `spec.md` or `scenarios/` resolves outside the project is reported, with a message naming the link, and never read. Today the gate reads through such a link, so a file outside the project decides whether a declared impact is discharged: probed 2026-11-15, the same outside file with a back-link passed the check and without one blocked it. 067's `check-artifacts` cross-spec-impact-drift family calls the same function and inherits the fix; 067's review finding is deferred here. The owner decided on 2026-11-15 that 066 fixes its own gate, with the class stated once in 022's plan
- [x] 022's `plan.md` §Path arguments gains a bullet, *Paths drawn from committed text*, stating that a path the runtime builds from text the repository holds (a `cross-spec-impact:` slug, a `dependencies:` name, a link target, a plan's Affected Files entry) is contained before it is probed, listed or opened, as a path argument is, unless 067's fix round has already added it
- [x] A test with an outward link (the affected spec's `spec.md`, and its `scenarios/` directory) shows the gate reports it and reads nothing through it, and turns red under its mutant

- **Done when**: The gate never reads an affected spec through a link leaving the project, proven by a test its mutant turns red; the whole `cargo test` passes; and 066's next review reads the change.

## 24. write-review and write-analysis guard their read-modify-write (BE-TXN-002, owner decision)

- [ ] `write-review` and `write-analysis` guard the window between reading `review.md` or `analysis.md` and renaming the regenerated record over it with the compare-before-persist helper 041's task 27 adds to `runtime/src/primitives/mod.rs`: the file is read again just before the rename, and the write is refused when it changed since the primitive read it. Each carries lists from the prior record (`write-review` its `waivers:` and `decisions:`, `write-analysis` its `decisions:` and its still-current rule verdicts), so today a record written between its read and its rename loses them. The owner decided on 2026-11-16 to close the BE-TXN-002 class by fixing each owner's primitives rather than waiving them. Run after 041's task 27 lands and before 066's next analysis, so its design-subject edits are judged once
- [ ] 066's data model states what a refused write reports and its plan names the guard; `WRITE_REVIEW`'s and `WRITE_ANALYSIS`'s descriptions in `runtime/src/schema/descriptions.rs` state it
- [ ] A test per primitive in which the record changes between its read and its write shows the write refused and the other writer's record kept, each failing under its mutant

- **Done when**: Neither record write overwrites a record written between its read and its write, proven by tests their mutants turn red; the whole `cargo test` passes; and 066's next review reads the change.
