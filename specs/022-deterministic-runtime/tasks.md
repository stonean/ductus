# 022 — Deterministic Runtime Tasks

Tasks derived from the [plan](plan.md). Complete in order. Each task is small enough to complete and verify in a single session; later tasks depend on earlier ones.

## 141. Place every scenario, section and criterion before anything moves

- [x] Read all 116 scenarios in full and place each by the test: a scenario moves with the primitive whose contract its Behavior changes; one about a command's rewiring, the exec walker or an extension point stays in 022 even when it names a gate or a record (`implement-completion-gate`, `review-exec-wiring`, `review-runtime-acceleration`, `exec-analyze-derives-its-list-seeds` stay)
- [x] 066 — review and analyze records and the completion gate: `write-review` (observations, examined/scope), `write-analysis`, `process-decisions`, `process-waivers`, `compute-review-scope`, `check-review-gate` (with its Cross-spec impact and Record freshness subsections), `diff-cross-spec`
- [x] 067 — analyze and audit checks: `check-artifacts` and all its families, `validate-frontmatter`, `traverse-deps`, `resolve-anchor`, `check-rule-ids`, `check-promotion-coverage`, `check-orphaned-references`, `check-command-flags`, `derive-dependencies`/`derive-references`, `check-corpus-links`
- [x] 022 keeps the rest, including Severity tiers (all three specs consume them), the extension points, the spec-artifact, inbox, routing, dashboard and bootstrap primitives, and AC1-21, AC28, AC29 and AC31 (AC31's live claim is `dashboard`)
- [x] Criteria: AC24, AC26, AC27, AC30 and AC22's write-analysis/gate half go to 066; AC23, AC25, AC32 and AC22's check-artifacts half go to 067; restate each in present tense without its history annotations
- [x] Write the manifest (section, scenario and criterion to destination, plus every reference outside the moving file) as task 142's checklist before task 142 starts

- **Done when**: every data-model section, scenario and acceptance criterion of 022 has one destination (022, 066, 067, or retired), and every reference to a moving or retiring file outside it is listed.

## 142. Retire superseded text and remove restatement

The manifest task 141 produced. Slugs: `066-audit-records-and-the-completion-gate`, `067-analyze-and-audit-checks`.

**Scenarios to 066 (16):** `a-done-spec-has-no-transition-to-gate`, `a-multi-number-renumber-is-one-rewrite`, `a-renamed-file-contributes-its-rewrites`, `a-review-states-what-it-read`, `an-empty-scope-flag-does-not-decide-the-denominator`, `process-waivers-file-lists`, `review-base-includes-the-transition-commit`, `review-scope-includes-uncommitted-work`, `review-scope-plan-affected-table-format`, `review-scope-union`, `review-staleness-gate`, `review-staleness-on-done-specs`, `the-committed-tree-horizon`, `the-cross-spec-impact-gate`, `write-analysis-and-the-second-gate`, `write-review-known-field-quoting`.

**Scenarios to 067 (25):** `a-revision-qualified-path-is-not-a-working-tree-reference`, `adopter-corpus-link-integrity`, `adopter-generator-promotion`, `analyze-artifact-checks`, `anchor-reference-kinds`, `argument-hint-needs-a-token`, `block-element-scanner`, `check-artifacts-skipped-targets`, `criterion-adopter-scope-destinations`, `criterion-negated-creation-phrasing`, `criterion-non-assertion-phrasings`, `criterion-path-existence-family`, `derive-references-unstaged-drift-is-reported`, `derive-unparseable-frontmatter-is-reported`, `every-agents-generated-copies-are-excluded`, `frontmatter-severity-tiers-on-both-sides`, `link-adjacent-drift-family`, `orphan-check-adopter-authored-references`, `orphaned-reference-check`, `orphaned-reference-historical-roots`, `per-agent-scaffold-paths-ship-to-adopter`, `sibling-link-grammar-is-the-shared-one`, `sibling-symlink-trust-boundary`, `the-promotion-coverage-line`, `traverse-deps-cycle-check`.

**Retired (5):** the three named below, plus `review-gate-unexaminable-contracts` (every Behavior bullet but one describes the guidance `the-committed-tree-horizon` removed, and that one asserts four gate checks against today's eleven) and `check-stuck-read-blob-reuse` (its subject, `read_blob_from_tree`, does not exist; `find_in_progress_commit` resolves status through `status_of_commit` and `tree_entry_id`). **022 keeps the other 70.** A scenario changing contracts in two destinations goes with the one that dominates and is not split: `adopter-generator-promotion` and `review-staleness-on-done-specs` move; `coverage-expansion-primitives`, `criterion-label-assignment`, `primitive-robustness-hardening`, `review-runtime-acceleration`, `review-scope-parse-fidelity`, `runtime-review-019-hardening`, `scenario-open-question-signal`, `severity-is-a-closed-set-not-a-string`, `spec-side-parser-hardening`, `unreadable-scenario-is-reported` and `waiver-processing-order` stay.

**Data model.** 066: `write-analysis`, `check-review-gate` with Cross-spec impact and Record freshness, `diff-cross-spec`, both `write-review` addenda, `process-decisions`, and new `compute-review-scope` and `process-waivers` shapes; the observation newline rejection `write_review.rs` enforces lands in the observations section, since retiring `review-observations-write-through` removes its only statement. 067: `validate-frontmatter`, `resolve-anchor`, `traverse-deps`, `check-promotion-coverage`, `check-rule-ids`, `check-orphaned-references`, `check-command-flags`, `check-artifacts` (stating the `tasks.md`-history rule `scenario-consistency` applies at `done`, which the current text calls a skip), `derive-dependencies` / `derive-references`, and a new `check-corpus-links` shape. 022 keeps the rest, Severity tiers included. Measured by 8-word shingles against every scenario, `framework/commands/*.md` and the constitution, 2.0 KB of the data model's 79.7 KB of prose is at least half verbatim elsewhere (2.8 KB at 6 words, 3.6 KB at 5), so the cut below is by meaning, not overlap; after the moving sections leave, 022 is about 51.8 KB.

**Criteria.** 066 gets AC22's `write-analysis` and gate half, AC24, AC26, AC27 and AC30; 067 gets AC22's `check-artifacts` half, AC23, AC25 and AC32. They leave 022 outright, with no stub; `next-criterion` stays 33.

**Links.** `derive-dependencies` harvests `spec.md` bodies only, so scenario and data-model links induce no edge; still, no 022 file links 066 or 067 and no 066 file links 067 (backticked slugs). Moved scenarios linking 022 use `../../022-deterministic-runtime/`. `section:` frontmatter is remapped to a heading of the new spec.

**References outside the moving files.** A pointer to where a contract or file lives now is repointed; a past-tense account of what 022 did (`022's tasks 107 and 108`, `022 gave check-review-gate a record`), a published changelog entry, a spent `tasks.md` entry and a `review.md` body stay as written, with their links repointed so they resolve.

- Spec corpus, in the move commit (`check-corpus-links` gates it): 017 `spec.md:111,188`, `data-model.md:117`, four scenario signposts at line 8 and `generator-sync-claim-honesty.md:67`; 020 `scenarios/waiver-file-lists.md:13`, `scenarios/review-flag-parsing-is-specified.md:65`; 023 `scenarios/extend-existing-scenario-task.md:75`; 026 `spec.md:139,162`, `family-18-marker-list-parity.md:22,26,56`, `family-19-review-freshness.md:9`, `family-39-shipped-anchor-resolution.md:9,26`; 030 `data-model.md:49`, `scenarios/referenced-service-spec-root.md:11`; 040 `spec.md:33`, `plan.md:11,58`; 041 `scenarios/plan-records-the-design-as-it-stands.md:65`; 042 `spec.md:26,70`; 045 `data-model.md:122`, `plan.md:33-36,133-136`; 047 `scenarios/analyze-record-freshness.md:9`; 050 `scenarios/a-review-may-precede-its-commit.md:11,13`; 051 `scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md:25`; 058 `scenarios/only-unreadable-targets-retain-decisions.md:13`; 059 `plan.md:172`; 060 `spec.md` AC9, `plan.md:11,119`; 063 `plan.md:131`; and inside 022, `spec.md`, `plan.md` and `data-model.md` links into moving scenarios and sections, plus the cross-destination links between scenarios.
- Kept as written: 058 `spec.md:383-384` (058's own record of the obligation it discharged on those scenarios, backticked, not linked); 057 `tasks.md:204`, 020 `tasks.md:105,108`, 045 `tasks.md`, 059 `tasks.md` (spent); `review.md` bodies of 002, 030, 040, 059; `AGENTS.md:60` and `:153` (past-tense accounts); `runtime/CHANGELOG.md` entries.
- Retirement commit: `AGENTS.md:45`'s `the-inbox-row` link, `runtime/src/primitives/inbox_standing.rs:10`.
- Sweep commit: `AGENTS.md:45,57,117`, the routing entry at `:71`, `:135` and `:161`; `framework/constitution.md:696` and its scenario-link bullet; `framework/bootstrap/ductus-procedure.md:257`, `framework/migrations.toml:199`, `framework/migrations/generator-primitives.md:10`; `.githooks/pre-commit:20,33`, `.github/workflows/framework-checks.yml:65`, `generators.yml:29`, `scripts/install-hooks.sh:32`, `scripts/audit/adopter-shell-behavior.sh:27`, `check-zero.sh:14`; `runtime/README.md:62` (also the retired `stonean/govern` host); `runtime/src/schema/primitives.rs:4,4757,4872`, `schema/paths.rs:213`, and in `runtime/src/primitives/`: `check_artifacts.rs:4-6,2241`, `check_corpus_links.rs:26`, `check_orphaned_references.rs:20`, `check_promotion_coverage.rs:31`, `compute_review_scope.rs:10,21`, `derive_dependencies.rs:10`, `derive_references.rs:51`, `mechanical_sweep.rs:54`, `process_waivers.rs:31`, `traverse_deps.rs:10`, `write_review.rs:1759`; `runtime/tests/mcp.rs:302`, `parity.rs:236`; the `runtime/CHANGELOG.md:1524` link. `scripts/audit/marker-list-parity.sh:20,75,268` moves with its scenario.
- Done specs whose durable contracts the sweep edits, expected on the Family 19 re-review bill: 017, 020, 023, 026, 030, 041, 045, 047, 050, 051, 058.

- [x] Retire, pointer-first, the three wholly superseded scenarios (`review-observations-write-through`, `the-inbox-row`, `the-analyze-record-states-what-it-captured`): repoint every inbound reference to where the content lives now (058's spec and data model, `dashboard`), then delete the files
- [x] Retire `review-gate-unexaminable-contracts` and `check-stuck-read-blob-reuse` the same way
- [x] For the four partly superseded scenarios (`the-cross-spec-impact-gate`, `merge-managed-block-multi-subsection-end`, `review-staleness-on-done-specs`, `commands-dir-parameterization`) delete the passage restating the replaced design; a one-line pointer to the replacing spec is the decision record
- [x] In every other scenario, edit each Behavior, Edge Case or Resolved Question that states a replaced requirement or names machinery that no longer exists to what is true now, or delete it with a one-line pointer; collapse an inline "Changed by NNN" pair to the current statement; leave Context, which records the situation that prompted the scenario
- [x] Data model: keep shapes and field meanings; delete what a 022 scenario or a command source already states, and reduce each pointer paragraph to one sentence

- **Done when**: no superseded scenario or passage remains, no file references a deleted one, and no data-model paragraph restates a scenario or command source.

## 143. Create 066 and 067 and move their content

- [x] Create each through `/ductus:specify`'s procedure (`create-feature` from `framework/templates/spec/`); its routing gate offers 022 — decline, recording in the new spec's Resolved Questions that 022's data model exceeded one read and this slice has one subject
- [x] Dependency direction is 067 -> 066 -> 022: 066 never links 067, and 022 names both by backticked slug only; any spec 066 or 067 would link that also links them is named, not linked (run `derive-dependencies` and expect no cycles)
- [x] Move each placed data-model section, scenario (`git mv`, so the rename is visible) and criterion; 066's data model also gains the shapes `compute-review-scope` and `process-waivers` never had (from `runtime/src/schema/primitives.rs`, kebab-case: compute-review-scope args feature, since; result diff-base, scope, modified-since, plan-affected; process-waivers args feature, fired [{rule, file}], skipped-passes; result applied, expired, retained, notices)
- [x] Each new spec's plan.md lists the `runtime/` files of its primitives under Affected Files, which is what its review scope is computed from; `derive-routing-candidates` offers only the lowest-numbered spec whose plan claims `runtime/`, which stays 022, so the routing entry in `AGENTS.md` names the three homes
- [ ] Walk each new spec to done: clarify, plan, implement (the moves are its tasks), review (both new data models read in full), analyze, completion gate
- [x] Measure: 022, 066 and 067 data models each at or under 50,000 bytes

- **Done when**: 066 and 067 are done, each data model is at or under 50,000 bytes, and 022 keeps nothing that moved.

## 144. Sweep every reference, routing rule and canonical row the split moves

- [x] Repoint every reference to a moved scenario or section: runtime doc comments (`runtime/src/schema/primitives.rs` alone had 39 references to 022), other specs, `AGENTS.md`, `runtime/CHANGELOG.md`, `framework/` and scripts, as its own commit
- [x] `scripts/audit/marker-list-parity.sh` reads `criterion-path-existence-family.md` by its 022 path (lines 75 and 268): change it in the same commit as that scenario's move
- [x] `framework/constitution.md`'s canonical-sources row for the scenario-to-task referencing rule names 022's data model: point it at 067's (this ships to adopters, so the round ends in release 0.59.7)
- [x] Reword `AGENTS.md`'s *Route shared runtime work to spec 022* entry to name three homes (022 core and shared surfaces, 066 records and gate, 067 checks) and re-key 050 plan.md's W23 classification row in the same commit, so promotion-coverage reports 0 unmatched keys
- [ ] Verify: `check-corpus-links` 0 broken, a grep for every moved and retired slug across `runtime/`, `scripts/`, `framework/`, `AGENTS.md` and the changelog finds only live paths, then `scripts/audit/review-freshness.sh` on the committed tree, with a scoped `--since` re-review of any done spec it flags

- **Done when**: no reference anywhere in the repository resolves to a moved or retired location, the routing rule names three homes, and the self-audit passes.

## 145. Prune 022's spent tasks, close it, and release 0.59.7

- [ ] Prune 022's spent task sections with `/ductus:prune` (tasks.md is ephemeral; git keeps them)
- [ ] Review and analyze 022, read its data model in full, and run the completion gate
- [ ] Ask the operator once to approve 066, 067 and 022 to done and release 0.59.7, saying it became two new specs because one would have left both data models within a few KB of the limit
- [ ] Cut 0.59.7 per the AGENTS.md release entries: bump version, runtime/Cargo.toml and the changelog heading; one build without --locked; full local gate and `scripts/audit/run-all.sh` after committing; tag; `git push --atomic origin main ductus-v0.59.7`; `git ls-remote`; read every workflow run including verify-published and verify-installer; expect 13 assets

- **Done when**: 022, 066 and 067 are done, 022's tasks.md holds no spent sections, and ductus-v0.59.7 is published with every workflow run green.

## 146. Disposition out-of-spec finding: the constitution says a scenario's sibling link induces a dependencies edge, and derive-dependencies harvests spec.md bodies only

- [x] Fix: `framework/constitution.md` §text-first-artifacts' scenario-link bullet and `AGENTS.md`'s matching Gotchas entry say a correct scenario link creates a `dependencies:` edge and can close a cycle; `list_tracked_specs` / `is_spec_path` admit only `spec.md` and `spec-and-plan.md`, and neither the primitive nor the bash generator it replaced ever read a scenario. State what the derivation reads, keep the depth-error half, and land it with the 0.59.7 constitution change

- **Done when**: the constitution bullet and the AGENTS.md entry describe the file set `derive-dependencies` actually harvests, verified against `runtime/src/primitives/derive_dependencies.rs`
