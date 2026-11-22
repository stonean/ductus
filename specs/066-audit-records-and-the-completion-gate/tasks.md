---
next-task: 32
---

# 066 — Audit Records and the Completion Gate Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 23. The gate's cross-spec read is contained (BE-INPUT-004, from 067's review)

- [x] `classify_cross_spec_entry` (`runtime/src/primitives/check_review_gate.rs`) reads the affected spec's `spec.md` and its scenarios only after containing each against the project root, through `validate_contained` or `refuse_outward_link` (`runtime/src/primitives/mod.rs`); an affected spec whose `spec.md` or `scenarios/` resolves outside the project is reported, with a message naming the link, and never read. Today the gate reads through such a link, so a file outside the project decides whether a declared impact is discharged: probed 2026-11-15, the same outside file with a back-link passed the check and without one blocked it. 067's `check-artifacts` cross-spec-impact-drift family calls the same function and inherits the fix; 067's review finding is deferred here. The owner decided on 2026-11-15 that 066 fixes its own gate, with the class stated once in 022's plan
- [x] 022's `plan.md` §Path arguments gains a bullet, *Paths drawn from committed text*, stating that a path the runtime builds from text the repository holds (a `cross-spec-impact:` slug, a `dependencies:` name, a link target, a plan's Affected Files entry) is contained before it is probed, listed or opened, as a path argument is, unless 067's fix round has already added it
- [x] A test with an outward link (the affected spec's `spec.md`, and its `scenarios/` directory) shows the gate reports it and reads nothing through it, and turns red under its mutant

- **Done when**: The gate never reads an affected spec through a link leaving the project, proven by a test its mutant turns red; the whole `cargo test` passes; and 066's next review reads the change.

## 24. write-review and write-analysis guard their read-modify-write (BE-TXN-002, owner decision)

- [x] `write-review` and `write-analysis` guard the window between reading `review.md` or `analysis.md` and renaming the regenerated record over it with the compare-before-persist helper 041's task 27 adds to `runtime/src/primitives/mod.rs`: the file is read again just before the rename, and the write is refused when it changed since the primitive read it. Each carries lists from the prior record (`write-review` its `waivers:` and `decisions:`, `write-analysis` its `decisions:` and its still-current rule verdicts), so today a record written between its read and its rename loses them. The owner decided on 2026-11-16 to close the BE-TXN-002 class by fixing each owner's primitives rather than waiving them. Run after 041's task 27 lands and before 066's next analysis, so its design-subject edits are judged once
- [x] 066's data model states what a refused write reports and its plan names the guard; `WRITE_REVIEW`'s and `WRITE_ANALYSIS`'s descriptions in `runtime/src/schema/descriptions.rs` state it
- [x] A test per primitive in which the record changes between its read and its write shows the write refused and the other writer's record kept, each failing under its mutant

- **Done when**: Neither record write overwrites a record written between its read and its write, proven by tests their mutants turn red; the whole `cargo test` passes; and 066's next review reads the change.

## 25. Stripping a line-number citation is a mechanical edit (AC6, owner decisions 2026-11-17)

- [x] `runtime/src/primitives/line_citations.rs` defines a line-number citation as the plan's *A stripped line-number citation is a mechanical edit* states, with a finder and a strip, and unit tests holding a host and port, a URL with a port and a `user@host` address out, and a bare filename's, a range's, a column's, an en-dash range's and a `#L` anchor's citation in (`each_citation_form_is_found`, `hosts_ports_and_other_colons_are_not_citations`)
- [x] `mechanical_sweep`'s `pair_run` compares each removed line stripped of its citations with the added line; tests show a strip-only file exempt, a renumbered or an added citation not, and a strip beside a real change judged on that change, and a built fixture shows `classify-spec-edit` and the analysis freshness agree on a strip-only commit
- [x] `classify-spec-edit` reads a criterion or open question whose only change strips a citation as unchanged, only the removed side stripped, with tests that a criterion's strip is a correction and a citation added to one reopens
- [x] §spec-lifecycle in `framework/constitution.md` names the case beside (a) and (b); `CLASSIFY_SPEC_EDIT` and both modules' docs say so; 079's `spec.md` carries a signpost back-linking 066 for the classify change, and 066's frontmatter declares 079 under `cross-spec-impact:`
- [x] Each new behavior has a mutant its test kills, listed in `scripts/mutation/line-citation-strip-066.py`

- **Done when**: A commit that only strips line-number citations reads mechanical to `classify-spec-edit`, current to the analysis freshness and unchanged to `compute-review-scope`, proven by tests their mutants turn red; nothing else becomes mechanical; the whole `cargo test` passes; and 066's next review reads the change.

## 26. A partial strip is mechanical, and the unexamined reasons are a closed set (AC6, AC2; owner decisions 2026-11-18, from 066's pre-done check)

- [x] `line_citations` gains a test of whether one text is another with a non-empty subset of its citation suffixes removed and nothing else changed, and `mechanical_sweep`'s `pair_run` and `classify_spec_edit`'s `same_items` read a line or item through it, so removing one citation beside a kept one is mechanical in every form, an anchor and an en-dash range included, in a scenario, plan or data model and in a criterion or open question; the pre-done check drove an anchor stripped beside a kept citation as a reopen
- [x] `write-analysis` refuses an `unexamined-by-reason` reason outside the closed set the data model's table names, on the CLI and the MCP paths, from one runtime list a test holds equal to that table; the pre-done check recorded `bogus-reason=2`
- [x] 066's plan and data model state both, and `CLASSIFY_SPEC_EDIT` and `WRITE_ANALYSIS` in `runtime/src/schema/descriptions.rs` where they state the rule
- [x] Each new behavior has a test its mutant turns red, listed in `scripts/mutation/line-citation-strip-066.py`

- **Done when**: A partial strip reads mechanical to classify-spec-edit and current to the analysis freshness, an unknown reason is refused, each proven by a test its mutant turns red; the whole `cargo test` passes; and 066's next review reads the change.

## 27. Criteria are verified in review, and the completion gate makes no semantic judgment (scenario criteria-are-verified-in-review)

The design is the plan's *Criteria are judged in review, and the gate judges nothing*; the subtasks run in order (owner approval, 2026-11-18).

Owed after this task (2026-11-19): nothing. 066 itself is `done` (`0c4d20fc`), and so are the four specs this task reopened: 000 (`59d02adc`), 020 (`9f372f24`), 079 (`f803b28c`) and 081 (`ec6560b6`).

- [x] (a) Record: `write-review` takes `criteria`, one verdict per criterion by label or index, reads the criteria from `spec.md`, and derives `criteria-judged`, `criteria-met`, `criteria-not-met` and `criteria-verdicts`; a not-met verdict sets `blocking`, and a verdict addressing a criterion `spec.md` lacks is refused. Tests with mutants; 066's data model and `WRITE_REVIEW`; the constitution's review record table; 020's data model gains the fields, which reopens 020, with a signpost back to 066
- [x] (b) Owed: `compute-review-scope` reads `review-owed: true` while `spec.md` holds an unticked criterion, or a ticked one whose digest is not among the record's `criteria-verdicts`, whatever the window holds, and names each in `criteria-owing`. Tests with mutants, an artifact-only window and a record without the field among them; 066's data model and `COMPUTE_REVIEW_SCOPE`; 079's AC4 and 081's AC1 and AC8 annotated, which reopens both
- [x] (c) Gate: `check-review-gate` blocks `unticked-criterion` directly after `unchecked-task`. Tests with mutants; 066's data model's check list and `CHECK_REVIEW_GATE`
- [x] (d) Commands: `framework/commands/review.md` gains the criteria step (`verifyCriteria`, then `mark-criterion` with `checked` set to each verdict) and writes `spec.md`'s criterion ticks; `framework/commands/implement.md` loses its criteria steps; both references follow; the `verifyCriteria` request is built for the review walk and the goldens are re-blessed; a fixture shows a ticked criterion that the code outside the window contradicts unticked with the record blocking, and the same fixture passing once corrected
- [x] (e) Sweep: §implement-phase in `framework/constitution.md`; `AGENTS.md`'s pre-done verification ordering entry retired; 000's `validation-gates` and `criterion-route-after-draft` scenarios, which reopens 000; 013's `criterion-identifiers` context; 022's `implement-completion-gate` scenario and data model; 060's plan and 050's prose; each affected spec in `cross-spec-impact:` with a signpost back to 066
- [x] (f) Each new behavior has a test its mutant turns red, listed in `scripts/mutation/criteria-in-review-066.py`; the whole `cargo test` and every push workflow's steps pass

- **Done when**: the scenario's described behavior is correctly implemented and tested, each new behavior proven by a test its mutant turns red; the whole `cargo test` passes; and 066's next review reads the change.

## 28. Record paths are quoted only when they need it (scenario record-paths-are-quoted-only-when-they-need-it)

- [x] Implement the behavior described in `scenarios/record-paths-are-quoted-only-when-they-need-it.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested

## 29. A commit after the scope was resolved refuses the write (scenario a-commit-after-the-scope-was-resolved-refuses-the-write)

- [x] Implement the behavior described in `scenarios/a-commit-after-the-scope-was-resolved-refuses-the-write.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 30. A write result states what it wrote (scenario a-write-result-states-what-it-wrote)

- [x] Implement the behavior described in `scenarios/a-write-result-states-what-it-wrote.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 31. A moved-scope refusal's size is bounded (BE-PAYLOAD-001, from 066's analysis)

- [x] `refuse_a_moved_scope` (`runtime/src/primitives/write_review/moved_scope.rs`) names how many scope paths moved, the first three, each with up to three of the commits that changed it as a short sha and a subject cut at 72 characters, then how many paths more, and `git diff --name-only {reviewed-against} HEAD`, the command that lists every one, as plan.md's task 29 section states; its remedy reads each moved path and each path a re-run of `compute-review-scope` adds. The owner chose the fixed bound on 2026-11-22, the agent's recommendation, adopted by the operator
- [x] Tests show four moved paths name three and one more with the command, a path changed by four commits names three of them, and a long subject is cut, each failing under a mutant listed in `scripts/mutation/moved-scope-066.py`
- [x] The remedy wording follows in `framework/commands/review.md` step 11, `framework/reference/review.md` §Window and coverage, `WRITE_REVIEW` in `runtime/src/schema/descriptions.rs` and `runtime/CHANGELOG.md`

- **Done when**: The refusal's size is bounded whatever moved, proven by tests their mutants turn red; the whole `cargo test` passes; and 066's next review reads the change.
