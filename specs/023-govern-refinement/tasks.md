---
next-task: 26
---

# 023 — `ductus` Refinement Tasks

Tasks derived from the [plan](plan.md).

## Phase C — Follow-on scenarios

### 24. Review and analyze the corrections spec 079 made

Spec 079 task 10 corrected scenarios `living-specs`, `configure-dedup-permissions` and `configure-retires-formerly-canonical-entries`, and annotated AC19 and AC36; a scenario edit stales the review. It owes `/ductus:review`, then `/ductus:analyze`, then the completion gate, which run once no box in this file is open (spec 079).

- **Done when**: a review and an analysis are recorded on this spec, and `/ductus:implement` has returned it to `done`.

### 25. Re-check the analysis after the 2026-11-14 re-judge

The 2026-11-14 re-judge, run because `640e3d49` added plan rationales after its judges read the plan (owner, 2026-11-14), found four MUST findings: `BE-SCHEMA-002`, `BE-COMPAT-001`, `BE-PAYLOAD-001` and `BE-RACE-003`. The owner approved one `plan.md` paragraph, after §Phase sequencing's `BE-INPUT-004` paragraph, that fixes all four by pointing at commitments 022 makes; `BE-RACE-003`'s sentence names `create-scenario`'s no-clobber create, which 022's task 33 delivered (`509948e8`), in place of the deferral first chosen. The paragraph, as approved, goes after the paragraph that ends "so `scenarios/{slug}.md` cannot name another directory.": "The tool contracts this spec adds or changes — `create-scenario`, `append-task`, and `merge-permissions`' `revoke` argument, `allow-revoked` result and `ConflictingRevoke` refusal (AC37) — take their schemas from the typed structs in `runtime/src/schema/primitives.rs`, from which `schemars` derives each MCP tool schema (`BE-SCHEMA-002`; the *Project layout* section of `specs/022-deterministic-runtime/plan.md`). Each is additive under `BE-COMPAT-001` — two new tools, an optional request field, a new response field, and a refusal only that field can reach — so the release that carries them is a minor bump, as 022's `ask-consolidation` scenario reasons for the two tools. Their lists are bounded by what they describe, as the *MCP surface* section of `specs/022-deterministic-runtime/plan.md` states every tool result is (`BE-PAYLOAD-001`): `revoked` holds distinct entries of the `revoke` list, under `canonical` the closed retired set the runtime holds (`RETIRED_ALLOW`), and `ConflictingRevoke` names a subset of the entries the caller passed. `create-scenario` refuses a slug already taken exclusively, not by a check and then a write: it persists its tempfile with no-clobber (`write_new_atomic`), as 022's `ask-consolidation` scenario states (`BE-RACE-003`)." `BE-RACE-003`'s message also named `append-task`, which reads `tasks.md`, computes the next number and renames the whole file over it, so two concurrent calls lose one task: a lost update `BE-TXN-002` owns, while 022's `ask-consolidation` scenario claims the second writer's `persist` fails, though `write_atomic` is a rename that replaces the file. The owner deferred that half (2026-11-14) to a task the owner routed (2026-11-15) to the session that adds 022's tasks; 023 waits on its number, and a plan sentence names it before the re-judge. It owes one re-judge of all 310 rules and the step-15 items after the paragraph and that sentence, then the record: the four fixed and the `append-task` lost update deferred to that task if the run raises it, `decided-by` the owner, and any `BE-REPR-002` SHOULD on the new tool fields waived by the rule, since no `specs/system.md` declares a casing convention. A finding that re-judge raises on text the paragraph did not touch goes to the owner, not to another run (owner, 2026-11-14).

- **Done when**: `analysis.md` is current with no live hard-fail or blocking finding.
