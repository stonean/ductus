# 066 — Audit Records and the Completion Gate Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Move the scenarios

- [ ] `git mv` the sixteen scenarios task 142 of `022-deterministic-runtime` places here, remap each `section:` to a heading of this spec, and rewrite each link into 022 as a `../../022-deterministic-runtime/` path; a link into `067-analyze-and-audit-checks` becomes a backticked slug

- **Done when**: the sixteen files sit under `scenarios/` with their history intact, each `section:` names a heading of `spec.md`, and `check-corpus-links` reports no broken link.

## 2. Move the data-model sections

- [ ] Move the `write-analysis`, `check-review-gate`, `diff-cross-spec`, `process-decisions` and both `write-review` sections out of 022's data model, keeping every shape and field meaning and pointing at the scenario where the rationale lives
- [ ] Add the `compute-review-scope` and `process-waivers` shapes from `runtime/src/schema/primitives.rs`

- **Done when**: `data-model.md` holds the seven primitives' shapes, 022's data model holds none of them, and the file is at or under 50,000 bytes.

## 3. Verify the criteria against the tree

- [ ] Verify AC1–AC5 against `runtime/src/primitives/` and tick each that holds

- **Done when**: every acceptance criterion is ticked, each verified against the code that implements it.
