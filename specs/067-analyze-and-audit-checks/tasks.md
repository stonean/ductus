---
next-task: 33
---

# 067 — Analyze and Audit Checks Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 31. plan-consistency names a plan.md it cannot read (QUAL-CLAIM-001, from 000's review)

- [x] `check-artifacts`' `plan-consistency` family records a `plan.md` that exists but cannot be read, rather than returning silently (`check_plan_consistency` in `runtime/src/primitives/check_artifacts.rs`): today a planned spec whose `plan.md` holds bytes that are not UTF-8 reads `clean: true` with no `plan-consistency` skip, its only trace another family's `artifact-unreadable` skip for the same file (reproduced on a fixture by 000's review, 2026-11-14). The owner deferred 000's QUAL-CLAIM-001 finding here on 2026-11-14 (000's `review.md` stored decision). Decide in the task whether it is a skip under the family or a blocking finding, since the family is blocking when a plan exists
- [x] `CHECK_ARTIFACTS`' description, which says the families other than those it names are fully examinable by construction, 067's data model and the analyze reference's §Plan consistency state the outcome
- [x] A test proves an unreadable `plan.md` is named, failing under its mutant, and `framework-checks.yml` step (u) passes

- **Done when**: A planned spec whose `plan.md` cannot be read no longer reads clean under `plan-consistency`, proven by a test its mutant turns red; the whole `cargo test` passes; and 067's next review reads the change.

## 32. derive-references' result is bounded (BE-PAYLOAD-001, from 017's analysis)

- [x] Implement the behavior described in `scenarios/derive-references-results-are-bounded.md`
- [x] `derive-references` goes through `bounded_lists::place` as `derive-dependencies` does (a whole derivation the CLI gates and reports on, and its bound): `updated`, `unwritten`, `untracked-skipped`, `absent`, `unparseable` and `unwritable` move to the file `lists-file` names past `[artifacts] read-size-bytes`, each with its count inline. The owner placed 017's analysis finding here on 2026-11-14, and 017's scenario `generator-sync-claim-honesty` points at this one
- [x] `DERIVE_REFERENCES`' description and 067's data model, §`derive-dependencies` / `derive-references`, state the bound; 080's §Bounded results table gains no row, since it lists 080's own population (owner, 2026-11-14)
- [x] A test proves the inline form under the read size and the file form past it, and the CLI's non-zero exit on an `unwritable` spec moved to the file, each failing under its mutant, and `framework-checks.yml` step (u) passes

- **Done when**: derive-references' lists return inline, or in a lists-file past the read size, with the CLI still failing on an unwritable spec, proven by tests their mutants turn red; the whole `cargo test` passes; and 067's next review reads the change.
