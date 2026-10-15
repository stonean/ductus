# 022 — Deterministic Runtime Tasks

Tasks derived from the [plan](plan.md). Complete in order. Each task is small enough to complete and verify in a single session; later tasks depend on earlier ones.

## 129. The pi command candidate

- [x] Implement the behavior described in `scenarios/the-pi-command-candidate.md`
- [x] Add {cli-config-dir}/prompts/{project}-{command_name}.md as the last candidate of Host::command_file_candidates (064), keeping the plural-then-singular order of the two pre-existing shapes
- [x] Extend the command_file_candidates_cover_both_layouts_plural_first test to the third element and add the .pi session-fixture test (the flat project-hyphenated form resolves last)
- [x] Sync the config-dir enumerations in the write-session doc comment and the check-corpus-links generated-copies comment to include .pi
- [x] Record the three-shape resolution order in 022's data-model.md under Per-project file resolution

- **Done when**: the third candidate resolves last with the pre-existing two in order, both pinned by tests, and the enumerations and data-model carry the shape. The release that ships it is 064's task 13, cut only after 022 returns to done.

## 130. Extend check-command-flags with the argument-hint token direction

- [x] Implement the behavior described in `scenarios/argument-hint-needs-a-token.md`

- **Done when**: check-command-flags reports a declared argument-hint whose body holds no substitution token; target/link/prune carry the token; and Family 30 is green with the new denominator on stderr.

## 131. Implement scenario: [every-agents-generated-copies-are-excluded](scenarios/every-agents-generated-copies-are-excluded.md)

- [x] Implement the behavior described in `scenarios/every-agents-generated-copies-are-excluded.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 132. Implement scenario: [a-multi-number-renumber-is-one-rewrite](scenarios/a-multi-number-renumber-is-one-rewrite.md)

- [x] Implement the behavior described in `scenarios/a-multi-number-renumber-is-one-rewrite.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 133. Implement scenario: [per-agent-scaffold-paths-ship-to-adopter](scenarios/per-agent-scaffold-paths-ship-to-adopter.md)

- [x] Implement the behavior described in `scenarios/per-agent-scaffold-paths-ship-to-adopter.md`

- **Done when**: `criterion-path-existence` records a `ships-to-adopter` skip, not a finding, for a criterion naming any registered agent's `ductus` install path, settings file, or project-scoped MCP target — pinned by tests that include an agent not dogfooded here and a cell carrying a trailing note — while a path no registry row derives still flags, and an unreadable registry leaves the Shared Files contribution intact.

## 134. Implement scenario: [a-renamed-file-contributes-its-rewrites](scenarios/a-renamed-file-contributes-its-rewrites.md)

- [x] Implement the behavior described in `scenarios/a-renamed-file-contributes-its-rewrites.md`

- **Done when**: `SweepIndex::build` and Family 19 both detect renames with the same explicitly stated threshold and rename limit, neither inheriting git or libgit2 defaults nor `diff.renames`; `mechanical_sweep_parity` carries a renamed-directory fixture asserting both halves derive the same pairs including the renamed files' pairs, and fails when either half falls back to delete-plus-add.

## 135. Implement scenario: [a-revision-qualified-path-is-not-a-working-tree-reference](scenarios/a-revision-qualified-path-is-not-a-working-tree-reference.md)

- [x] Implement the behavior described in `scenarios/a-revision-qualified-path-is-not-a-working-tree-reference.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 136. Implement scenario: [review-scope-includes-uncommitted-work](scenarios/review-scope-includes-uncommitted-work.md)

- [x] Implement the behavior described in `scenarios/review-scope-includes-uncommitted-work.md`

- **Done when**: `compute-review-scope` diffs from the diff base to the working tree (index and untracked included, ignored excluded), and resolves the base to `HEAD` when `spec.md` on disk reads `in-progress` and the spec at `HEAD` does not. Tests cover an uncommitted edit, a staged edit, an untracked file, an ignored file, `--since=HEAD`, an uncommitted `done → in-progress` reopen, an uncommitted `planned → in-progress` start, and an uncommitted `in-progress → done` flip; each test of changed behavior fails against the old code.

## 137. Implement scenario: [fetch-archive-names-its-timeout](scenarios/fetch-archive-names-its-timeout.md)

- [x] Implement the behavior described in `scenarios/fetch-archive-names-its-timeout.md`

- **Done when**: `fetch_archive.rs` declares `FETCH_TIMEOUT` (30 s) beside the module's other bounds and sets it on the client it builds; a test drives the client with a short timeout against a proxy that never answers and fails when the client ignores the timeout it is given; plan.md's transport section names the constant and states the bound as the wait for the response and each body read.

## 138. Correct plan.md's YAML safe-loading sentence

- [x] plan.md's `serde_norway` bullet says YAML is deserialized only into typed structures; four modules also parse into `serde_norway::Value` in production code (`write_analysis.rs` does so only in its tests) — state both, and why a generic value is still safe loading

- **Done when**: the bullet names both deserialization targets, and its safe-loading claim rests on what `serde_norway` does with a tagged node rather than on the typed-only premise.

## 139. Bring spec.md under the read size

- [x] Collapse each resolved question that a later decision reversed, superseded or dropped to one line naming the decision and what replaced it; git keeps the full text
- [x] Correct the resolved questions current code contradicts: MCP tools register bare names such as `read-spec` (`runtime/src/mcp/server.rs`), and every release publishes to crates.io
- [x] Remove history narration elsewhere in the body where git already records it, keeping every requirement and criterion
- [x] Measure with `check-artifact-size`: spec.md at or under 50,000 bytes

- **Done when**: spec.md is at or under 50,000 bytes, no resolved question states a decision current code contradicts, and every acceptance criterion's requirement is unchanged.

## 140. Trim data-model.md in place

- [x] Remove history narration git already records (which spec or task added a field, what a shape used to be, the incident that forced a rule), keeping each contract it accompanies
- [x] Where a section restates a 022 scenario, keep the shape and the contract's one-line statement and point at the scenario for the reasoning; prefer 022's own scenarios, which review staleness also covers
- [x] Remove sections that only say a shape moved elsewhere, leaving a one-line pointer
- [x] Keep every Args/Result block, and the `scenario-consistency` referencing-task rule the constitution's canonical-sources map points at
- [x] Measure before and after with `check-artifact-size`, and check every link with `check-corpus-links`

- **Done when**: data-model.md is measurably smaller, every primitive's Args/Result shape and every contract statement survives either in place or behind a pointer to a 022 scenario that states it, and every link resolves.

## 141. Place every scenario, section and criterion before anything moves

- [ ] Read all 116 scenarios in full and place each by the test: a scenario moves with the primitive whose contract its Behavior changes; one about a command's rewiring, the exec walker or an extension point stays in 022 even when it names a gate or a record (`implement-completion-gate`, `review-exec-wiring`, `review-runtime-acceleration`, `exec-analyze-derives-its-list-seeds` stay)
- [ ] 066 — review and analyze records and the completion gate: `write-review` (observations, examined/scope), `write-analysis`, `process-decisions`, `process-waivers`, `compute-review-scope`, `check-review-gate` (with its Cross-spec impact and Record freshness subsections), `diff-cross-spec`
- [ ] 067 — analyze and audit checks: `check-artifacts` and all its families, `validate-frontmatter`, `traverse-deps`, `resolve-anchor`, `check-rule-ids`, `check-promotion-coverage`, `check-orphaned-references`, `check-command-flags`, `derive-dependencies`/`derive-references`, `check-corpus-links`
- [ ] 022 keeps the rest, including Severity tiers (all three specs consume them), the extension points, the spec-artifact, inbox, routing, dashboard and bootstrap primitives, and AC1-21, AC28, AC29 and AC31 (AC31's live claim is `dashboard`)
- [ ] Criteria: AC24, AC26, AC27, AC30 and AC22's write-analysis/gate half go to 066; AC23, AC25, AC32 and AC22's check-artifacts half go to 067; restate each in present tense without its history annotations
- [ ] Write the manifest (section, scenario and criterion to destination, plus every reference outside the moving file) as task 142's checklist before task 142 starts

- **Done when**: every data-model section, scenario and acceptance criterion of 022 has one destination (022, 066, 067, or retired), and every reference to a moving or retiring file outside it is listed.

## 142. Retire superseded text and remove restatement

- [ ] Retire, pointer-first, the three wholly superseded scenarios (`review-observations-write-through`, `the-inbox-row`, `the-analyze-record-states-what-it-captured`): repoint every inbound reference to where the content lives now (058's spec and data model, `dashboard`), then delete the files
- [ ] For the four partly superseded scenarios (`the-cross-spec-impact-gate`, `merge-managed-block-multi-subsection-end`, `review-staleness-on-done-specs`, `commands-dir-parameterization`) delete the passage restating the replaced design; a one-line pointer to the replacing spec is the decision record
- [ ] Data model: keep shapes and field meanings; delete what a 022 scenario or a command source already states (measured 2026-10-15: 6.3 KB of the 78 KB of prose is at least half verbatim elsewhere), and reduce each pointer paragraph to one sentence

- **Done when**: no superseded scenario or passage remains, no file references a deleted one, and no data-model paragraph restates a scenario or command source.

## 143. Create 066 and 067 and move their content

- [ ] Create each through `/ductus:specify`'s procedure (`create-feature` from `framework/templates/spec/`); its routing gate offers 022 — decline, recording in the new spec's Resolved Questions that 022's data model exceeded one read and this slice has one subject
- [ ] Dependency direction is 067 -> 066 -> 022: 066 never links 067, and 022 names both by backticked slug only; any spec 066 or 067 would link that also links them is named, not linked (run `derive-dependencies` and expect no cycles)
- [ ] Move each placed data-model section, scenario (`git mv`, so the rename is visible) and criterion; 066's data model also gains the shapes `compute-review-scope` and `process-waivers` never had (from `runtime/src/schema/primitives.rs`, kebab-case: compute-review-scope args feature, since; result diff-base, scope, modified-since, plan-affected; process-waivers args feature, fired [{rule, file}], skipped-passes; result applied, expired, retained, notices)
- [ ] Each new spec's plan.md lists the `runtime/` files of its primitives under Affected Files, so `derive-routing-candidates` offers it
- [ ] Walk each new spec to done: clarify, plan, implement (the moves are its tasks), review (both new data models read in full), analyze, completion gate
- [ ] Measure: 022, 066 and 067 data models each at or under 50,000 bytes

- **Done when**: 066 and 067 are done, each data model is at or under 50,000 bytes, and 022 keeps nothing that moved.

## 144. Sweep every reference, routing rule and canonical row the split moves

- [ ] Repoint every reference to a moved scenario or section: runtime doc comments (`runtime/src/schema/primitives.rs` alone had 39 references to 022), other specs, `AGENTS.md`, `runtime/CHANGELOG.md`, `framework/` and scripts, as its own commit
- [ ] `scripts/audit/marker-list-parity.sh` reads `criterion-path-existence-family.md` by its 022 path (lines 75 and 268): change it in the same commit as that scenario's move
- [ ] `framework/constitution.md`'s canonical-sources row for the scenario-to-task referencing rule names 022's data model: point it at 067's (this ships to adopters, so the round ends in release 0.59.7)
- [ ] Reword `AGENTS.md`'s *Route shared runtime work to spec 022* entry to name three homes (022 core and shared surfaces, 066 records and gate, 067 checks) and re-key 050 plan.md's W23 classification row in the same commit, so promotion-coverage reports 0 unmatched keys
- [ ] Verify: `check-corpus-links` 0 broken, a grep for every moved and retired slug across `runtime/`, `scripts/`, `framework/`, `AGENTS.md` and the changelog finds only live paths, then `scripts/audit/review-freshness.sh` on the committed tree, with a scoped `--since` re-review of any done spec it flags

- **Done when**: no reference anywhere in the repository resolves to a moved or retired location, the routing rule names three homes, and the self-audit passes.

## 145. Prune 022's spent tasks, close it, and release 0.59.7

- [ ] Prune 022's spent task sections with `/ductus:prune` (tasks.md is ephemeral; git keeps them)
- [ ] Review and analyze 022, read its data model in full, and run the completion gate
- [ ] Ask the operator once to approve 066, 067 and 022 to done and release 0.59.7, saying it became two new specs because one would have left both data models within a few KB of the limit
- [ ] Cut 0.59.7 per the AGENTS.md release entries: bump version, runtime/Cargo.toml and the changelog heading; one build without --locked; full local gate and `scripts/audit/run-all.sh` after committing; tag; `git push --atomic origin main ductus-v0.59.7`; `git ls-remote`; read every workflow run including verify-published and verify-installer; expect 13 assets

- **Done when**: 022, 066 and 067 are done, 022's tasks.md holds no spent sections, and ductus-v0.59.7 is published with every workflow run green.
