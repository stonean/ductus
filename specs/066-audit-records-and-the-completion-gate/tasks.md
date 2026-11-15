---
next-task: 24
---

# 066 — Audit Records and the Completion Gate Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 23. The gate's cross-spec read is contained (BE-INPUT-004, from 067's review)

- [ ] `classify_cross_spec_entry` (`runtime/src/primitives/check_review_gate.rs`) reads the affected spec's `spec.md` and its scenarios only after containing each against the project root, through `validate_contained` or `refuse_outward_link` (`runtime/src/primitives/mod.rs`); an affected spec whose `spec.md` or `scenarios/` resolves outside the project is reported, with a message naming the link, and never read. Today the gate reads through such a link, so a file outside the project decides whether a declared impact is discharged: probed 2026-11-15, the same outside file with a back-link passed the check and without one blocked it. 067's `check-artifacts` cross-spec-impact-drift family calls the same function and inherits the fix; 067's review finding is deferred here. The owner decided on 2026-11-15 that 066 fixes its own gate, with the class stated once in 022's plan
- [ ] 022's `plan.md` §Path arguments gains a bullet, *Paths drawn from committed text*, stating that a path the runtime builds from text the repository holds (a `cross-spec-impact:` slug, a `dependencies:` name, a link target, a plan's Affected Files entry) is contained before it is probed, listed or opened, as a path argument is, unless 067's fix round has already added it
- [ ] A test with an outward link (the affected spec's `spec.md`, and its `scenarios/` directory) shows the gate reports it and reads nothing through it, and turns red under its mutant

- **Done when**: The gate never reads an affected spec through a link leaving the project, proven by a test its mutant turns red; the whole `cargo test` passes; and 066's next review reads the change.
