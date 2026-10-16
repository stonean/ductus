# 067 — Analyze and Audit Checks Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Move the scenarios

- [x] `git mv` the twenty-five scenarios task 142 of `022-deterministic-runtime` places here, remap each `section:` to a heading of this spec, and rewrite each link into 022 or 066 as a `../../022-deterministic-runtime/` or `../../066-audit-records-and-the-completion-gate/` path
- [x] Point `scripts/audit/marker-list-parity.sh` at `criterion-path-existence-family.md`'s new path in the same commit

- **Done when**: the twenty-five files sit under `scenarios/` with their history intact, each `section:` names a heading of `spec.md`, `check-corpus-links` reports no broken link, and `scripts/audit/marker-list-parity.sh` passes.

## 2. Move the data-model sections

- [x] Move the `check-artifacts`, `validate-frontmatter`, `traverse-deps`, `resolve-anchor`, `check-rule-ids`, `derive-dependencies` / `derive-references`, `check-orphaned-references`, `check-command-flags` and `check-promotion-coverage` sections out of 022's data model, keeping every shape and field meaning and pointing at the scenario where the rationale lives
- [x] Add the `check-corpus-links` shape from `runtime/src/schema/primitives.rs`
- [x] Point the constitution's canonical-sources row for the scenario-to-task referencing rule at this data model

- **Done when**: `data-model.md` holds every check's shape, 022's data model holds none of them, the constitution row names this file, and the file is at or under 50,000 bytes.

## 3. Verify the criteria against the tree

- [x] Verify AC1–AC4 against `runtime/src/primitives/` and tick each that holds

- **Done when**: every acceptance criterion is ticked, each verified against the code that implements it.
