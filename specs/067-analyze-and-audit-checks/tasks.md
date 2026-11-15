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

## 33. The criterion-path, body-path and corpus-link probes refuse a link out of the project (BE-INPUT-004, from 045 and 026)

- [ ] `check-artifacts`' `criterion-path-existence` and `body-path-existence` probes refuse a symbolic link leaving the project before probing, as the sibling-link family already does. Today `lexical_in_project` in `runtime/src/primitives/check_artifacts.rs` refuses a `..` that leaves the project, and `exists()` then follows a symbolic link out of it. The owner deferred 045's BE-INPUT-004 (MUST) finding here on 2026-11-14; 045's task 17 cites this task
- [ ] `check-corpus-links` refuses a link target that leaves the project, lexically or through a symbolic link, before probing it, and a tracked markdown file that is such a link before reading it: today `runtime/src/primitives/check_corpus_links.rs` tests `lexical_join(base, rel).exists()`, which probes a `..` past the project root and follows a symbolic link out of it, and reads each tracked file it lists without either check. Its resolution stays lexical, never canonicalized, so one corpus answers alike in two checkouts. Decide in the task, and state in 067's data model, whether such a target is reported broken or skipped, and what the containment base is for a project in a subdirectory of its repository (spec 059), where a link climbing out of the project into the repository resolves today and the shipped pre-commit hook blocks on a broken one. `/{project}:audit` Families 26 and 35a delegate their link resolution to it. The owner deferred that part of 026's BE-INPUT-004 (MUST) analysis finding here on 2026-11-15; 026's `analysis.md` records the deferral
- [ ] 067's data model, `CHECK_ARTIFACTS`' and `CHECK_CORPUS_LINKS`' descriptions state what each probe reports for such a link
- [ ] A test with an outward link: a path a criterion and a spec body each name, and a relative link's target, each a symbolic link to a file outside the project, is not probed through, and the test turns red under its mutant

Owed when this lands, in the same change: 067's `plan.md` decision "A path that leaves the project is `outside-project`" and the `outside-project` row of `scenarios/check-artifacts-skipped-targets.md` still say the screen is lexical; both state the link check too.

- **Done when**: No probe of the three follows a link leaving the project, proven by tests their mutants turn red; the whole `cargo test` passes; and 067's next review reads the change.

## 34. A check holds the dogfooded feature-directory grammar to the shipped hook's (051's AC15)

- [ ] A check, an audit family or a test, fails when the feature-directory grammar in `.githooks/pre-commit` or in `scripts/lint-frontmatter.sh` (`FEATURE_DIR_RE`) diverges from the one `framework/bootstrap/hooks/ductus-pre-commit` ships. Each copy exists because its surface runs before any binary resolves or cannot ask a primitive for its corpus, and no check holds either to the shipped one, as 051's AC15 annotation says. The owner placed this on 067 on 2026-11-14; 051's task 36 names this task in AC15's annotation before 051's analysis runs
- [ ] The check runs where the framework's checks run (`scripts/audit/run-all.sh` or `framework-checks.yml`), and it reports how many copies it compared, so a parse that found no grammar never reads as a pass
- [ ] It is proven to fail on a deliberately diverged fixture of each copy, and to pass on the tree

- **Done when**: Both copies are held to the shipped hook's grammar by a check that fails on a diverged fixture of each and passes on the tree, and 067's next review reads the change.
