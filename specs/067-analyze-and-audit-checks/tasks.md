---
next-task: 35
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

## 33. The criterion-path and body-path probes refuse a link out of the project (BE-INPUT-004, from 045)

- [ ] `check-artifacts`' `criterion-path-existence` and `body-path-existence` probes refuse a symbolic link leaving the project before probing, as the sibling-link family already does. Today `lexical_in_project` in `runtime/src/primitives/check_artifacts.rs` refuses a `..` that leaves the project, and `exists()` then follows a symbolic link out of it. The owner deferred 045's BE-INPUT-004 (MUST) finding here on 2026-11-14; 045's task 17 cites this task
- [ ] 067's data model and `CHECK_ARTIFACTS`' description state what each probe reports for such a link
- [ ] A test with an outward link: a path a criterion and a spec body each name, which is a symbolic link to a file outside the project, is not probed through, and the test turns red under its mutant

- **Done when**: Neither probe follows a link leaving the project, proven by a test its mutant turns red; the whole `cargo test` passes; and 067's next review reads the change.

## 34. A check holds the dogfooded feature-directory grammar to the shipped hook's (051's AC15)

- [ ] A check, an audit family or a test, fails when the feature-directory grammar in `.githooks/pre-commit` or in `scripts/lint-frontmatter.sh` (`FEATURE_DIR_RE`) diverges from the one `framework/bootstrap/hooks/ductus-pre-commit` ships. Each copy exists because its surface runs before any binary resolves or cannot ask a primitive for its corpus, and no check holds either to the shipped one, as 051's AC15 annotation says. The owner placed this on 067 on 2026-11-14; 051's task 36 names this task in AC15's annotation before 051's analysis runs
- [ ] The check runs where the framework's checks run (`scripts/audit/run-all.sh` or `framework-checks.yml`), and it reports how many copies it compared, so a parse that found no grammar never reads as a pass
- [ ] It is proven to fail on a deliberately diverged fixture of each copy, and to pass on the tree

- **Done when**: Both copies are held to the shipped hook's grammar by a check that fails on a diverged fixture of each and passes on the tree, and 067's next review reads the change.
