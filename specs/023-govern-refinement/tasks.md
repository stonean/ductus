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

Waiting on a task that session `3fd29c6a` adds on 022, which specifies `create-scenario` in its `ask-consolidation` scenario (the owner routes it): `create-scenario` refuses an existing slug with an exclusive create, not a `dest.exists()` check and then a rename over the file. The owner deferred 023's `BE-RACE-003` finding to it (2026-11-14). The same re-judge, run because `640e3d49` added plan rationales after its judges read the plan (owner, 2026-11-14), found `BE-SCHEMA-002`, `BE-COMPAT-001` and `BE-PAYLOAD-001`, which the owner chose to fix in the plan. Once the task exists, in one sitting:

- add after `plan.md` §Phase sequencing's `BE-INPUT-004` paragraph the paragraph the owner approved (2026-11-14): "The tool contracts this spec adds or changes — `create-scenario`, `append-task`, and `merge-permissions`' `revoke` argument, `allow-revoked` result and `ConflictingRevoke` refusal (AC37) — take their schemas from the typed structs in `runtime/src/schema/primitives.rs`, from which `schemars` derives each MCP tool schema (`BE-SCHEMA-002`; the *Project layout* section of `specs/022-deterministic-runtime/plan.md`). Each is additive under `BE-COMPAT-001` — two new tools, an optional request field, a new response field, and a refusal only that field can reach — so the release that carries them is a minor bump, as 022's `ask-consolidation` scenario reasons for the two tools. Their lists are bounded by what they describe, as the *MCP surface* section of `specs/022-deterministic-runtime/plan.md` states every tool result is (`BE-PAYLOAD-001`): `revoked` holds distinct entries of the `revoke` list, under `canonical` the closed retired set the runtime holds (`RETIRED_ALLOW`), and `ConflictingRevoke` names a subset of the entries the caller passed.";
- add a plan sentence naming the deferral's task;
- judge all 310 rules and run the step-15 pass once, after both edits; a finding that run raises on text no edit touched goes to the owner, not to another run (owner, 2026-11-14);
- record the three fixed and `BE-RACE-003` deferred, `decided-by` the owner, and waive by the rule any `BE-REPR-002` SHOULD on the new tool fields, since no `specs/system.md` declares a casing convention.

- **Done when**: `analysis.md` is current with no live hard-fail or blocking finding.
