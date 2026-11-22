---
next-task: 46
---

# 067 — Analyze and Audit Checks Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 31. plan-consistency names a plan.md it cannot read (QUAL-CLAIM-001, from 000's review)

- [x] `check-artifacts`' `plan-consistency` family records a `plan.md` that exists but cannot be read, rather than returning silently (`check_plan_consistency` in `runtime/src/primitives/check_artifacts.rs`): before this task a planned spec whose `plan.md` held bytes that are not UTF-8 read `clean: true` with no `plan-consistency` skip, its only trace another family's `artifact-unreadable` skip for the same file (reproduced on a fixture by 000's review, 2026-11-14). The owner deferred 000's QUAL-CLAIM-001 finding here on 2026-11-14 (000's `review.md` stored decision). Decide in the task whether it is a skip under the family or a blocking finding, since the family is blocking when a plan exists
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

- [x] `check-artifacts`' `criterion-path-existence` and `body-path-existence` probes refuse a symbolic link leaving the project before probing, as the sibling-link family already does. Before this task `lexical_in_project` in `runtime/src/primitives/check_artifacts.rs` refused a `..` that leaves the project, and `exists()` then followed a symbolic link out of it. The owner deferred 045's BE-INPUT-004 (MUST) finding here on 2026-11-14; 045's task 17 cites this task
- [x] `check-corpus-links` refuses a link target that leaves the project, lexically or through a symbolic link, before probing it, and a tracked markdown file that is such a link before reading it: before this task `runtime/src/primitives/check_corpus_links.rs` tested `lexical_join(base, rel).exists()`, which probed a `..` past the project root and followed a symbolic link out of it, and read each tracked file it listed without either check. Its resolution stays lexical, never canonicalized, so one corpus answers alike in two checkouts. Decide in the task, and state in 067's data model, whether such a target is reported broken or skipped, and what the containment base is for a project in a subdirectory of its repository (spec 059), where a link climbing out of the project into the repository resolves today and the shipped pre-commit hook blocks on a broken one. `/{project}:audit` Families 26 and 35a delegate their link resolution to it. The owner deferred that part of 026's BE-INPUT-004 (MUST) analysis finding here on 2026-11-15; 026's `analysis.md` records the deferral
- [x] 067's data model, `CHECK_ARTIFACTS`' and `CHECK_CORPUS_LINKS`' descriptions state what each probe reports for such a link
- [x] A test with an outward link: a path a criterion and a spec body each name, and a relative link's target, each a symbolic link to a file outside the project, is not probed through, and the test turns red under its mutant

Owed when this lands, in the same change: 067's `plan.md` decision "A path that leaves the project is `outside-project`" and the `outside-project` row of `scenarios/check-artifacts-skipped-targets.md` said the screen was lexical; both now state the link check too.

- **Done when**: No probe of the three follows a link leaving the project, proven by tests their mutants turn red; the whole `cargo test` passes; and 067's next review reads the change.

## 34. A check holds the dogfooded feature-directory grammar to the shipped hook's (051's AC15)

- [x] A check, an audit family or a test, fails when the feature-directory grammar in `.githooks/pre-commit` or in `scripts/lint-frontmatter.sh` (`FEATURE_DIR_RE`) diverges from the one `framework/bootstrap/hooks/ductus-pre-commit` ships. Each copy exists because its surface runs before any binary resolves or cannot ask a primitive for its corpus, and no check holds either to the shipped one, as 051's AC15 annotation says. The owner placed this on 067 on 2026-11-14; 051's task 36 names this task in AC15's annotation before 051's analysis runs
- [x] The check runs where the framework's checks run (`scripts/audit/run-all.sh` or `framework-checks.yml`), and it reports how many copies it compared, so a parse that found no grammar never reads as a pass
- [x] It is proven to fail on a deliberately diverged fixture of each copy, and to pass on the tree

- **Done when**: Both copies are held to the shipped hook's grammar by a check that fails on a diverged fixture of each and passes on the tree, and 067's next review reads the change.

## 35. resolve-anchor's and read-deprecation-labels' results are bounded (BE-PAYLOAD-001, from 026's analysis)

- [x] `resolve-anchor`'s and `read-deprecation-labels`' result lists go through `bounded_lists::place`: inline while the result fits `[artifacts] read-size-bytes`, and past it in the file `lists-file` names, each list's count inline. Before this task neither had a file form, and each grew with the references or labels in the files it reads. `/{project}:audit` Family 39 (`scripts/audit/shipped-anchor-resolution.sh`) calls `resolve-anchor` and Family 40 (`scripts/audit/deprecation-removal.sh`) calls `read-deprecation-labels`. The owner deferred 026's BE-PAYLOAD-001 (MUST) analysis finding here on 2026-11-15; 026's `analysis.md` records the deferral, with its reason that the families' stdout report is the CI log `/audit` exits on, not a response the rule governs
- [x] Both descriptions and 067's data model state the bound, and every caller reads the lists from `lists-file` in the file form: Families 39 and 40, `/{project}:analyze` step 4, and the exec walker
- [x] A test proves the inline form under the read size and the file form past it for each primitive, each failing under its mutant, and `framework-checks.yml` step (u) passes

- **Done when**: Both results return inline, or in a lists-file past the read size, with every caller reading the file form, proven by tests their mutants turn red; the whole `cargo test` passes; and 067's next review reads the change.

## 36. derive-dependencies and derive-references guard their write (BE-TXN-002, owner decision)

- [x] Under `write`, `derive-dependencies` and `derive-references` guard the window between reading each `spec.md` and renaming its rewritten frontmatter over it with the compare-before-persist helper 041's task 27 adds to `runtime/src/primitives/mod.rs`: the file is read again just before the rename, and a spec that changed since it was read is not written and is named in the result, never counted as written or in sync, so the pre-commit hook can stop on it. Before this task each read, spliced and called `write_atomic` with no guard. The owner decided on 2026-11-16 to close the BE-TXN-002 class by fixing each owner's primitives rather than waiving them. Run after 041's task 27 lands and before 067's next analysis, so its design-subject edits are judged once
- [x] 067's data model states what a refused spec reports and its plan names the guard; `DERIVE_DEPENDENCIES`' and `DERIVE_REFERENCES`' descriptions in `runtime/src/schema/descriptions.rs` state it
- [x] A test per primitive in which a `spec.md` changes between its read and its write shows it unwritten, named in the result and the other writer's change kept, each failing under its mutant

The shipped `framework/bootstrap/hooks/ductus-pre-commit` needs no change: it runs each derivation under `set -euo pipefail`, so a refused spec's non-zero exit stops the commit and its stderr line shows. Its header comment names only the `unwritable` exit, which stays true, and is left as it is, since an edit under `framework/bootstrap/` owes a real adopter bootstrap.

- **Done when**: Neither derivation's write overwrites a change made between its read and its write, and a spec it refuses is named in the result, proven by tests their mutants turn red; the whole `cargo test` passes; and 067's next review reads the change.

## 37. check-artifacts reports a line-number citation (AC5, owner decisions 2026-11-17)

- [x] Runs after 066's task 25 lands, since it reads that task's `line_citations` definition. `check_line_number_citations` in `runtime/src/primitives/check_artifacts.rs` reads `spec.md`, `plan.md`, `tasks.md`, `data-model.md`, `research.md` and each scenario through the outward-link guard, skips a fenced line and a blockquote line, and reports each citation as one blocking `line-number-citations` finding naming the citation and its line, at every status; an unreadable artifact is recorded as the other families record one
- [x] Tests show a citation in each examined file reported, one in a fence, a blockquote, `review.md` or `analysis.md` not, a host and port not, and the family reporting at `draft` as at `done`
- [x] `CHECK_ARTIFACTS` in `runtime/src/schema/descriptions.rs`, `ArtifactFinding.family` in `runtime/src/schema/primitives.rs`, the data model's family list, `framework/reference/analyze.md` and AC1's family list name the family; every other list or count of the families is swept (`docs/developing-ductus.md` §Adding a member to a closed set)
- [x] Each new behavior has a mutant its test kills, listed in `scripts/mutation/line-number-citations-067.py`

- **Done when**: `check-artifacts` reports every line-number citation in a spec's examined artifacts as blocking and nothing AC5 excludes, proven by tests their mutants turn red; every list of the families names it; the whole `cargo test` passes; and 067's next review reads the change.

## 38. A none list item is no open question

- [x] Implement the behavior described in `scenarios/a-none-list-item-is-no-open-question.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested

## 39. A plan heading is matched without regard to case

- [x] Implement the behavior described in `scenarios/a-plan-heading-is-matched-without-regard-to-case.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested

## 40. A bare port span names its remedy

- [x] Implement the behavior described in `scenarios/a-bare-port-span-names-its-remedy.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested

## 41. The orphan check skips git-ignored local state

- [x] Implement the behavior described in `scenarios/orphan-check-skips-ignored-local-state.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested

## 42. A header without a key keeps the previous table's keys

- [x] Implement the behavior described in `scenarios/promotion-coverage-header-without-a-key.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested

## 43. The owner's decisions of 2026-11-19 on the artifact checks are recorded where this spec keeps decisions

- [x] The set-apart colon span keeps its detection and its finding gains a remedy hint; narrowing detection to a span beside a file path was weighed and set aside because it drops most of what the 2026-11-17 decision targeted
- [x] `check-rule-ids` keeps its citation grammar and its not-installed answer, since each change offered trades a waivable false positive for a silent miss on a mistyped citation
- [x] `check-artifacts` keeps `clean` unwidened for a spec it could not read, since the skip is recorded and `validate-frontmatter` fails the same file
- [x] `check-corpus-links` is not taught reference-style definition lines, which would newly block adopter commits

- **Done when**: the spec's Resolved Questions state each decision with its reason

## 44. AC6 and AC7: an analysis judges its own design fix in the run, and the plan against the spec

- [x] Update `plan.md`'s design record for AC6 and AC7 first. The decisions are this spec's Resolved Questions of 2026-11-21.
- [x] AC6: `write-analysis` (066's surface) refuses a hard-fail or blocking finding at a design subject of the spec in hand recorded as deferred whole to its own `tasks.md`, and refuses one recorded undispositioned at a design subject a fix of the run edited; steps 19 and 20 of `framework/commands/analyze.md` and `framework/reference/analyze.md` §Finding dispositions say the design edit is made in the run once confirmed. Fixture: each refusal and each accepted record, failing with the refusal removed.
- [x] AC7: step 15 and `framework/reference/analyze.md` §Plan consistency (blocking if plan exists) gain the plan-against-the-spec's-Behavior-and-criteria judgment; update the `plan-consistency` line of this spec's `data-model.md`, which enumerates the host's judgments.
- [x] Correct in the same change the statements this makes false: 058's `spec.md` routing list ("The spec in hand: a task on its `tasks.md`, a scenario, or a body edit"), 078's Resolved Question premise ("Analyze's fixes are mechanical chores its step-20 re-check re-detects") and the first paragraph of `framework/reference/review.md` §Fix-round loop; add 058, 066 and 078 to `cross-spec-impact:`.

- **Done when**: Both refusals are proven by a fixture both ways, the command and reference state the in-run design fix and the plan-against-spec judgment, and 058, 078 and the review reference no longer say otherwise.

## 45. AC8 and AC9: /analyze judges every scenario's § references

- [x] Update `plan.md`'s design record for AC8 and AC9 first. The decisions (a scenario's bare `spec.md` names its parent; a registered service's checkout is not read, and `--help` says so) are this spec's Resolved Questions of 2026-11-21.
- [x] AC8: step 4 of `framework/commands/analyze.md` runs `resolve-anchor` on each scenario under the spec, reports each reference with the scenario's path, and counts the scenario files as examined. The interactive path landed with this task's first commit (the step's text, `scenario_anchors` in `scripts/analyze/detect.py`, which reports `scenarios-examined`, and case AO in `scripts/tests/test-analyze-scripts.py`). The `ductus exec` walker, which had dispatched step 4 once, on `spec.md`, through `dispatch_primitive`'s `resolve-anchor` arm in `runtime/src/interpreter/mod.rs`, then ran `resolve-anchor`'s `compute` on `spec.md` and on each `.md` under `scenarios/`, merged before `bound` so the merged result meets the read size once, tagged each scenario reference with a `path` field on `AnchorReference` (absent on `spec.md`'s own, so existing goldens held), listed a scenario's unresolved anchor as `<path>: <anchor>` as `detect.py` does, reported `scenarios-examined`, and recorded a scenario it could not read under `anchor-target-unreadable`. A test that fails with the per-scenario fan-out removed proved it, no golden moved, `022-deterministic-runtime` joined this spec's `cross-spec-impact:`, and a blockquote signpost in 022's `spec.md` links back here, since the walker is 022's core. The § references the release binary reported unresolved in `scenarios/a-record-cites-a-removed-rule.md` (Deprecation, Edge, Rule) and `scenarios/adopter-generator-promotion.md` (Bash) were then read in context: the two scenarios now name the spec they cite (`5fbf75dd`), and prose that only looks like a reference was left.
- [x] AC9 in `resolve-anchor`: in a scenario, a bare `spec.md` resolves to the parent's; a quoted heading after `§` (`§"Cache invalidation"`) is reported as a form the check does not read; an unresolved reference whose next line completes it is reported as wrapped; `--help` states that a path leaving the project is unjudged even inside a registered `[services]` checkout.
- [x] Fixture, from the adopter report: one scenario beside a `spec.md` with `## Surface` and `## Cache invalidation`. Before the fix `(spec §Surface)` is unresolved, ``(`spec.md` §Surface)`` is resolved but unjudged, `§"Cache invalidation"` is absent from `references`, and ``(`../spec.md` §Surface)`` resolves; assert each form in both directions.

- **Done when**: The fixture proves each form both ways, analyze's step 4 covers every scenario with scenarios counted as examined, and `resolve-anchor --help` states the service-checkout answer.
