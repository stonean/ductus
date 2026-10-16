# 066 — Audit Records and the Completion Gate Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Move the scenarios

- [x] `git mv` the sixteen scenarios task 142 of `022-deterministic-runtime` places here, remap each `section:` to a heading of this spec, and rewrite each link into 022 as a `../../022-deterministic-runtime/` path; a link into `067-analyze-and-audit-checks` becomes a backticked slug

- **Done when**: the sixteen files sit under `scenarios/` with their history intact (`a-done-spec-has-no-transition-to-gate`, `a-multi-number-renumber-is-one-rewrite`, `a-renamed-file-contributes-its-rewrites`, `a-review-states-what-it-read`, `an-empty-scope-flag-does-not-decide-the-denominator`, `process-waivers-file-lists`, `review-base-includes-the-transition-commit`, `review-scope-includes-uncommitted-work`, `review-scope-plan-affected-table-format`, `review-scope-union`, `review-staleness-gate`, `review-staleness-on-done-specs`, `the-committed-tree-horizon`, `the-cross-spec-impact-gate`, `write-analysis-and-the-second-gate`, `write-review-known-field-quoting`), each `section:` names a heading of `spec.md`, and `check-corpus-links` reports no broken link.

## 2. Move the data-model sections

- [x] Move the `write-analysis`, `check-review-gate`, `diff-cross-spec`, `process-decisions` and both `write-review` sections out of 022's data model, keeping every shape and field meaning and pointing at the scenario where the rationale lives
- [x] Add the `compute-review-scope` and `process-waivers` shapes from `runtime/src/schema/primitives.rs`

- **Done when**: `data-model.md` holds the seven primitives' shapes, 022's data model holds none of them, and the file is at or under 50,000 bytes.

## 3. Verify the criteria against the tree

- [x] Verify AC1–AC5 against `runtime/src/primitives/` and tick each that holds

- **Done when**: every acceptance criterion is ticked, each verified against the code that implements it.

## 4. State the reasons write-analysis adds to the breakdown itself

- [x] Add to `data-model.md`'s **Derived, never accepted** list the two reasons `write_analysis.rs` adds to `unexamined-by-reason` itself: `constitution-unresolved` (one per registered shared constitution that did not resolve) and `constitution-registry-unreadable` (a `[constitutions]` registry that will not parse), both unexamined inputs rather than skipped targets

- **Done when**: the data model names every reason the writer can record, and the closed reason table stays the set a caller supplies
