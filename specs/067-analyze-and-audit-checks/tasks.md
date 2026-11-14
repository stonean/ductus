---
next-task: 32
---

# 067 — Analyze and Audit Checks Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 31. plan-consistency names a plan.md it cannot read (QUAL-CLAIM-001, from 000's review)

- [ ] `check-artifacts`' `plan-consistency` family records a `plan.md` that exists but cannot be read, rather than returning silently (`check_plan_consistency` in `runtime/src/primitives/check_artifacts.rs`): today a planned spec whose `plan.md` holds bytes that are not UTF-8 reads `clean: true` with no `plan-consistency` skip, its only trace another family's `artifact-unreadable` skip for the same file (reproduced on a fixture by 000's review, 2026-11-14). The owner deferred 000's QUAL-CLAIM-001 finding here on 2026-11-14 (000's `review.md` stored decision). Decide in the task whether it is a skip under the family or a blocking finding, since the family is blocking when a plan exists
- [ ] `CHECK_ARTIFACTS`' description, which says the families other than those it names are fully examinable by construction, 067's data model and the analyze reference's §Plan consistency state the outcome
- [ ] A test proves an unreadable `plan.md` is named, failing under its mutant, and `framework-checks.yml` step (u) passes

- **Done when**: A planned spec whose `plan.md` cannot be read no longer reads clean under `plan-consistency`, proven by a test its mutant turns red; the whole `cargo test` passes; and 067's next review reads the change.
