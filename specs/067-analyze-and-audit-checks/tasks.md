# 067 — Analyze and Audit Checks Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Move the scenarios

- [x] `git mv` the twenty-five scenarios task 142 of `022-deterministic-runtime` places here, remap each `section:` to a heading of this spec, and rewrite each link into 022 or 066 as a `../../022-deterministic-runtime/` or `../../066-audit-records-and-the-completion-gate/` path
- [x] Point `scripts/audit/marker-list-parity.sh` at `criterion-path-existence-family.md`'s new path in the same commit

- **Done when**: the twenty-five files sit under `scenarios/` with their history intact (`a-revision-qualified-path-is-not-a-working-tree-reference`, `adopter-corpus-link-integrity`, `adopter-generator-promotion`, `analyze-artifact-checks`, `anchor-reference-kinds`, `argument-hint-needs-a-token`, `block-element-scanner`, `check-artifacts-skipped-targets`, `criterion-adopter-scope-destinations`, `criterion-negated-creation-phrasing`, `criterion-non-assertion-phrasings`, `criterion-path-existence-family`, `derive-references-unstaged-drift-is-reported`, `derive-unparseable-frontmatter-is-reported`, `every-agents-generated-copies-are-excluded`, `frontmatter-severity-tiers-on-both-sides`, `link-adjacent-drift-family`, `orphan-check-adopter-authored-references`, `orphaned-reference-check`, `orphaned-reference-historical-roots`, `per-agent-scaffold-paths-ship-to-adopter`, `sibling-link-grammar-is-the-shared-one`, `sibling-symlink-trust-boundary`, `the-promotion-coverage-line`, `traverse-deps-cycle-check`), each `section:` names a heading of `spec.md`, `check-corpus-links` reports no broken link, and `scripts/audit/marker-list-parity.sh` passes.

## 2. Move the data-model sections

- [x] Move the `check-artifacts`, `validate-frontmatter`, `traverse-deps`, `resolve-anchor`, `check-rule-ids`, `derive-dependencies` / `derive-references`, `check-orphaned-references`, `check-command-flags` and `check-promotion-coverage` sections out of 022's data model, keeping every shape and field meaning and pointing at the scenario where the rationale lives
- [x] Add the `check-corpus-links` shape from `runtime/src/schema/primitives.rs`
- [x] Point the constitution's canonical-sources row for the scenario-to-task referencing rule at this data model

- **Done when**: `data-model.md` holds every check's shape, 022's data model holds none of them, the constitution row names this file, and the file is at or under 50,000 bytes.

## 3. Verify the criteria against the tree

- [x] Verify AC1–AC4 against `runtime/src/primitives/` and tick each that holds

- **Done when**: every acceptance criterion is ticked, each verified against the code that implements it.

## 4. Correct analyze-artifact-checks' claim that check-artifacts takes --all

- [x] `scenarios/analyze-artifact-checks.md` says the primitive runs "given a feature (or `--all`)"; `check-artifacts` takes `--feature` alone and the data model leaves `--all` iteration with the caller, so the Behavior names the one argument it takes

- **Done when**: the scenario's Behavior agrees with `ductus check-artifacts --help` and with the data model

## 5. Implement scenario: scenario-consistency-reads-tasks-history

- [x] Implement the behavior described in `scenarios/scenario-consistency-reads-tasks-history.md`
- [x] Delete `pruning_evidence`; `check_scenario_consistency` asks `ever_tasked_slugs` at every status, and the message below `done` names the remedy instead of the fingerprint
- [x] Rebuild the scenario-consistency tests on git-backed fixtures, and add the restart-at-1 and reset-at-done shapes, each proven to fail against the old code
- [x] State the rule in the module and function docs, the `check-artifacts` tool description, `data-model.md`'s family line, `analyze-artifact-checks`' Behavior, and `framework/commands/analyze.md` step 8 and its Scenario consistency section

- **Done when**: an in-progress spec whose `tasks.md` restarted at 1 reports none of its implemented scenarios, a `done` spec with a reset `tasks.md` reports a scenario no revision ever named, the `done` message is byte-identical, and no live text describes the pruning fingerprint

## 6. Report an unread tasks.md history as skipped targets

- [x] When `ever_tasked_slugs` cannot consult the history, record each unmapped scenario in `skipped` with reason `no-readable-state` instead of returning a clean family that examined nothing (QUAL-CLAIM-001, raised by the review's quality pass)
- [x] Assert the skip in the outside-a-repository, shallow-history and undecodable-revision tests, and that a fully mapped spec records none
- [x] Say so in the scenario, `check-artifacts-skipped-targets`' claim that the pre-045 families never skip, the `skipped` field's doc, the data model, and `analyze.md`'s Scenario consistency section

- **Done when**: an unconsultable history leaves every unmapped scenario in `skipped` as `no-readable-state`, never as a finding and never silently, and no text says scenario-consistency cannot skip

## 7. Implement scenario: body-path-existence-family

- [x] Implement the behavior described in `scenarios/body-path-existence-family.md`
- [x] Add `check_body_path_existence` to `check-artifacts`: spec body outside Acceptance Criteria, plan, data model and scenarios of a done spec; criterion grammar with the fragment stripped; relative candidates resolved from the citing file; root-absent, ships-to-adopter and line-scoped markers (the criterion list plus the body-only list); one batched history walk deciding finding vs `never-existed`; `no-readable-state` when history cannot be read
- [x] Git-backed tests for each rule, each proven to fail with the rule disabled
- [x] Add `never-existed` to the closed reason set everywhere it is listed, update every count of the families (`nine` to `ten`), 067 AC1, the data model, analyze.md step 8 and its reference, the tool description, the result's order doc and the criterion family's scope note
- [x] Measure check-artifacts on 067 and across every spec with the release build

- **Done when**: a done spec naming a path that existed and was deleted, on a line with no marker, yields one advisory finding quoting the line; a never-existed path, a marker line, an adopter path and an unreadable history each yield the matching skip and no finding; and every list of families and reasons names the new ones

## 8. Join a blockquote's hard-wrapped lines into one logical line

- [x] `logical_lines` joins consecutive `>` lines of one blockquote paragraph, so a note's marker on its first line covers a path on its next (021's and 030's post-completion notes were read line by line)
- [x] A test proving the join, failing with it disabled

- **Done when**: a hard-wrapped blockquote note whose marker and path sit on different physical lines records the path `not-a-live-claim`, and the test fails when blockquote lines are read singly

## 9. An identifier is one word to the negated-creation predicate

- [x] `asserts_negated_creation` splits words on everything but ASCII alphanumerics and `_`, so `introduced_in` is an identifier, not the creation verb `introduced` (043's `gvrn-v{introduced_in}` line was exempted by `not … introduced`)
- [x] A live-claim case in the clause-scoping test, failing with `_` treated as a separator
- [x] The word rule's statements name the identifier case: the predicate's doc and 067's negated-creation scenario

- **Done when**: a line whose only negator-then-verb pair runs through an `_`-joined identifier is a live claim, the test fails when `_` splits words, and the corpus delta is the one 043 line

## 10. Two marker widenings the body corpus earned

- [x] `no longer exists` widens to `no longer exist`, which subsumes it and catches the plural (021's pointer: AC7's workflow and two generators "no longer exist")
- [x] `target_paths` joins `target paths`: the TOML key's own spelling, naming a migration manifest's data (042's and 043's lines describing `target_paths`)
- [x] A predicate-level test for both, failing with either reverted
- [x] The list's statements agree: 045's canonical table and its count, analyze.md's restatement and count, 067's criterion scenario's count

- **Done when**: the corpus delta is the eight true lines and nothing else, the test fails with either widening reverted, and Family 18 passes

## 11. One block splitter: logical lines read through split_blocks

- [x] `split_blocks_reading` takes a `BlockReading` policy — keep blockquote paragraphs (`>` lines joined, a list item inside starting a new block) and keep headings as blocks; `split_blocks` is the default reading and is unchanged
- [x] `logical_lines` is that split with both kept, each block's lines unquoted and joined by a space — so it inherits one-line HTML-comment stripping, ATX heading parsing and the shared list-item test it duplicated imperfectly (found reviewing tasks 7–10)
- [x] Tests: the reading split keeps quotes and headings; a path inside a one-line comment is not read; each fails with its behavior disabled
- [x] The body family's doc comment rewrapped (task 8 left a line overlong)

- **Done when**: `logical_lines` holds no splitting logic of its own, `split_blocks`' results are unchanged for its caller, the new tests fail with their behavior disabled, and the corpus delta is explained line by line

## 12. Implement scenario: generated-reference-copies-are-excluded

- [x] Implement the behavior described in `scenarios/generated-reference-copies-are-excluded.md`
- [x] Add `.ductus/reference/` to `is_excluded`'s repository arm in `check_corpus_links.rs`, counted like the agent directories, in a tree that carries `framework/reference/`
- [x] A test that fails when the exclusion is removed, one showing another `.ductus/` file is still checked, and one showing installed reference files are checked where there are no sources

- **Done when**: `check-corpus-links --scope repository` is clean over this repository with the generated reference copies present, and the test fails with the exclusion removed (spec 071 task 10).

## 13. Implement scenario: pointers-into-shipped-files-are-judged

- [x] Implement the behavior described in `scenarios/pointers-into-shipped-files-are-judged.md`
- [x] Read the destination -> source map from the `## Shared Files` section of `framework/bootstrap/ductus.md`
- [x] Add the `cross-document` kind and count to `resolve-anchor`: a mapped source's heading, the citing file's heading, or a marker resolves; anything else is unresolved
- [x] Tests for each case, each failing with its behavior removed: heading, own heading, marker, neither, two documents, unmapped document, unreadable source, no map, and an explicit pair judged on a line naming other documents against its document alone
- [x] Family 39 reports the `cross-document` count; the data model's `resolve-anchor` section shows the kind and the count

- **Done when**: the tests pass and fail under mutation, Family 39 passes with every pointer into a reference file judged, and a before/after run over `framework/` and the spec corpus adds no unresolved reference (spec 071 task 11).

## 14. Disposition the fix-round re-read's findings on 2be18da6

- [x] `derive-dependencies` / `derive-references` (re-read rated MUST): a rewrite the re-parse guard refuses lands in `unparseable`, which is not drift, so both exit 0 and the hooks (stdout discarded), this repo's CI and the adopter CI template pass over a stale index; at 2be18da6^ a report-only run on the same input exited 1. Trigger, valid YAML that validate-frontmatter reports clean: `dependencies: [` / `009-old` / `]` (a multi-line flow list closing at column 0). Proposed fix, the owner's call (2026-11-10, option (a), exit non-zero): the splice skips a multi-line flow value until its brackets balance; a rewrite the guard still refuses goes into a new `unwritable` list (absent when empty), separate from `unparseable` (an unterminated fence); both primitives exit non-zero when it is non-empty, report-only and `--write` alike, with one stderr line per spec naming it and saying to edit that key by hand. Split the 'not drift / an unknown / validate-frontmatter owns it' sentences between the two classes in `schema/primitives.rs` (~1397, ~1473), `data-model.md` (~200) and `scenarios/derive-unparseable-frontmatter-is-reported.md` (~50-70). Applied by one agent in `runtime/src`, the mutation list (M40–M50) and `runtime/tests/derive_unwritable_cli.rs`, and the data model and scenario split by the operator; independently re-read twice, the second round finding no MUST or SHOULD; `unwritable` stays inline in both forms until task 25 moves it (owner, 2026-11-10)
- [x] `data-model.md` ~200: 'a frontmatter already invalid before the rewrite lands here too' holds only when a rewrite is due (an in-sync spec returns before the guard). Fixed with the first item's split: the sentence now sits under `unwritable` and says a rewrite must be due
- [x] `scenarios/orphaned-reference-check.md` ~74-77: 'an empty `migration` under `watermark` means the same' — say it is not a migration named "" either and means no registry was read. Fixed as worded
- [x] test: nothing pins the guard ahead of the `--staged` filter (moving it after the filter survives the lib and golden suites); add a test and a mutation. With it, put to the owner whether an unstaged spec in `unwritable` should stop a commit under the shipped hook's `--staged` run, as it does since the first item names it before the filter (left open by that fix's first re-read, 2026-11-10). The owner's call (2026-11-11, option (a), keep): it stops the commit, as the hook's cycle and corpus-link checks stop one on unstaged state, since the condition never clears on a later commit and only a hand edit fixes it. Pin it with one test per primitive, a `--staged` run with the spec unstaged naming it in `unwritable` and not `unwritten`, each killed by a mutation moving the guard after the filter, and say in both tool descriptions and this spec's data model that `unwritable` spans every tracked spec, staged or not; the result's shape does not change. Applied: `an_unstaged_unwritable_spec_is_named_under_staged` in both primitives, killed by M55 and M56, and the sentence in both descriptions and the data model
- [x] Decide: the command steps that consume `derive-dependencies` key on drift or a non-empty `updated` and read neither `unwritable` nor `unparseable`: `target.md` step 5, `analyze.md` step 6, `amend.md` step 3, `clarify.md` step 4 and `framework/reference/clarify.md` step 0, and the plan and implement safety nets in `framework/reference/plan.md` and `framework/reference/implement.md`. `drift` is false when `unwritable` is the only finding, so over MCP a spec the CLI now fails on goes unreported in those commands. Whether and how each surfaces it is the owner's call (found checking the first item, 2026-11-10). The owner's call (2026-11-11, option (a)): all seven report both lists when non-empty, without halting, an `unwritable` spec named with the hand edit its key needs and the hook's stop, an `unparseable` one named as not derived, never as in sync; `analyze.md` step 6 records each as an advisory finding beside drift, the spec's own hard fail staying step 2's. Applied by one sentence at each of the six safety nets. `analyze.md` step 6's sentence lands in task 25's commit with the `derive-dependencies` arm of `runtime/src/interpreter/analyze_tally.rs`, which restates that step's tiering for the exec walk: one advisory finding each for a non-empty `unwritable` and `unparseable`, read from their `-count` fields (owner, 2026-11-11, accepting that runtime cost), and killed by M53 and M54. This item first also claimed the arm records no drift in the file form; it does not hold, since the walker restores a file-form result's lists before tallying it (`restore_lists` before `record_primitive` in `runtime/src/interpreter/mod.rs`), so the `updated` arm is unchanged
- [x] wording: the derive-unparseable scenario ~14-15 says `splice_references` anchors on 'the line after `dependencies:`' (it now goes after the whole entry); `derive_dependencies.rs` ~650's fixture comment `# kept for history` is one that gets dropped; `pointers-into-shipped-files-are-judged.md` ~25 omits the bare `name.md` token `document_path` also accepts. Fixed: the scenario names the whole entry as the anchor, the fixture comment reads `# goes with the list`, and the pointers scenario names the bare token
- [x] The fix's independent re-read (2026-11-11, `de56c366` and `6b0f96fd`) found no MUST or SHOULD and three NOTEs, dispositioned by the owner's delegation of finding dispositions (2026-11-08): fixed, the seven consumer steps keyed on `unwritable` and `unparseable` being non-empty, which past the read size sit only in the lists file, so each now reads `unwritable-count` and `unparseable-count`, inline in both forms, naming the specs from the list or the lists file; waived, `runtime/tests/mcp.rs`' AC7 conformance table does not list `unwritable`, since the MCP handler calls the same `derive_dependencies::run` the unit test `past_the_read_size_unwritable_moves_to_the_list_file` drives, so the wire adds only the serialization of an optional list, the shape that table exercises for the other six; waived, `runtime/tests/result_sizes.rs` measures no case where `unwritable` grows, since it is an ignored measurement, the list grows only with specs whose rewrite would break their frontmatter, and the bound is enforced in code and pinned by that unit test

Handoff (2026-10-29). The owner decided the first item on 2026-11-10: (a), as recommended: the splice skips a multi-line flow value until its brackets balance; a rewrite the guard still refuses goes in a new `unwritable` list, absent when empty and separate from `unparseable` (which stays the unterminated fence); both primitives exit non-zero on it, report-only and `--write` alike, with one stderr line per spec saying to edit that key by hand, so the shipped hook (`set -euo pipefail`) stops an adopter's commit with that message. (b): the same with exit 0. The trigger is `dependencies: [`, an indented `009-old`, then `]` at column 0. Fix through one agent editing only `runtime/src` and `scripts/mutation/owner-reviews-067.py` (a mutation per new test; run that list and `--check` every list; gate fmt, release clippy `-D warnings`, `cargo test --release --locked`, release build last); commit in the background; re-read until none finds a MUST or SHOULD.

067's review is owed once tasks 21-24 and 27 are done (its record of 2026-10-16 is review-stale); tasks 14, 17-20, 25 and 26 are done, 17-19 after scenario-targeted clarifies (2026-11-11). Task 22's scenario still carries its open question, so it takes a scenario-targeted `/ductus:clarify` first, and `check-review-gate` blocks on it until then. Since 2026-11-11 the window also holds tasks 17-19's runtime commits (`63527403`, `286c2c09`, `8651d466`) and task 20's plan (`6158d7df`); `8651d466` reopened 002, 010, 014, 024, 036, 038 and 051 and `459b678f` reopened 045, each owing its own review and analysis of that commit, so review the shared window once in depth and let each later review read only what changed since, as `AGENTS.md` says. Since 2026-11-11 the reviews of 064 (`2411dca9`), 071 (`c9b2155c`), 046 (`c7991df7`) and 080 (`c810427e`) have read the agent removal's window (`b1d319f1`), 071's in depth, and `c9b2155c` corrected seven claims in 067's scenarios and data model, which 067's review reads; carry the newest such record's in-scope, still-holding decisions, as `AGENTS.md` says, ahead of the older carry list below. What earlier passes read, for the reuse rule: four slices read 067 and 026 at `72288af1`; fix rounds `44c0babe`, `2be18da6` and `d9ac40a3` were re-read and `c16eee54` (prose) was not; the records written at `c1cf51ff` in 002, 017, 050, 051, 052, 062, 064 and 072 cover `af99793c..c1cf51ff`; no record read `70bb4af5`. Carry byte-identically, fired through process-decisions: the 13 decisions in 071's `review.md` (all in 067's scope) and the `c1cf51ff` records' in-scope ones. The Summary states the fixed findings: QUAL-TEST-001 (validate_frontmatter's status test), QUAL-CLAIM-001 (Family 34's render crash exited 0), this task's MUST; Families 28's and 34's scripts are recorded here, being in this scope. Observations (owner decisions 2026-10-26):

- Fixed: indentless `- item` spliced into invalid YAML (`spec_links.rs`); derive-references split a block-form key from its items (`derive_references.rs`); Family 28 matched a script anywhere in the README, one way (`audit-family-parity.sh`); untested check-command-flags `run`, derive-dependencies `absent`, `framework/reference/` sources' link check and a nested markers file; a rule-ids test named for a removed window; traverse_deps named the retired `gen-spec-deps.sh`; findings_for's doc comment sat on a test; `sections` documented required yet an empty set reported zeros; an unparseable registry promised a skip (`check_orphaned_references.rs`); `orphaned-reference-check.md`'s `last_applied` null and command-file pointer; the data model's two exclusions and traverse-deps `cycles`; AC2 and `anchor-reference-kinds.md` missing the word-boundary and fence rules; the pointers scenario's link-target form; `criterion-negated-creation-phrasing.md`'s mis-resolved `045 §Resolved Questions`; each item of this task.
- Waived, contrived unless stated: an apostrophe in a plain flow scalar opening a quote that never closes in `flow_depth_change`, whose miscounted splice the re-parse guard refuses (`spec_links.rs`; the first item's first re-read, 2026-11-10); two Tarjan implementations (depth bounded by spec count) and an unparseable transitive node becoming a sink (validate-frontmatter hard-fails it) and `../` dependency names (read-only, the operator's own tree), all `traverse_deps.rs`; a digit as hint word boundary (`check_command_flags.rs`); fence-blind headings and minor untested branches, no claim resting on them (`check_promotion_coverage.rs`); `--section ""` not refused; pair_document restating the pair grammar, only message wording affected (`shipped-anchor-resolution.sh`); a maintainer-only reference allowed from any file, and exact lowercase names only, as its header says (`installer-command-parity.sh`); a renamed `findings` key passing, not a claimed case (`step-reference-integrity.sh`); a cosmetic double blank line (`derive_references.rs`); step 7 with `--agents=` omitting the host, as the owner worded it (`ductus.md`); 1c reducing bullets to distinct edges, as its header states (`cross-doc-consistency.sh`).
- Deferred: a cleanup in `ductus.md` to 071 task 22; fixed once that lands.

Then analyze 067 and run its gate, re-verifying its four ticked criteria.

- **Done when**: each item is fixed, deferred, or waived with the owner's decision written on it, and an independent re-read of the fix finds no MUST or SHOULD

## 15. Implement scenario: an-at-path-import-is-a-reference

- [x] Implement the behavior described in `scenarios/an-at-path-import-is-a-reference.md`
- [x] `managed_paths_in` reads an `@` at line start or after whitespace as the start of a reference, and an `@` glued to a word as not one; `an_at_path_import_is_a_reference` and `an_at_inside_a_word_is_not_an_import` each fail under their mutation in `scripts/mutation/claude-md-imports.py` (delivered in 96934872 for `002-project-scaffolding`'s `claude-md-template-imports-load`)
- [x] `data-model.md`'s check-orphaned-references bullets and the `a-revision-qualified-path-is-not-a-working-tree-reference` scenario state the `@` rule

- **Done when**: The scenario's behavior holds in `runtime/src/primitives/check_orphaned_references.rs`, its two tests fail with the behavior removed, and 067's data model and sibling scenario describe the rule as it runs.

## 16. Implement scenario: disposition-drift-is-advisory

- [x] Implement the behavior described in `scenarios/disposition-drift-is-advisory.md`
- [x] `check_artifacts`' `disposition-drift` family emits `AnalyzeSeverity::Advisory`; `a_done_spec_whose_review_owes_decisions_is_disposition_drift` asserts the tier, and mutation M3 in `scripts/mutation/findings-cost-in-proportion-to-risk.py` kills it (for `058-findings-route-at-discovery`'s `findings-cost-in-proportion-to-risk`)
- [x] `data-model.md`'s family list gives the tier, and `/ductus:analyze` no longer reverts on it under `--fix`

- **Done when**: The scenario's behavior holds in `runtime/src/primitives/check_artifacts.rs`, its test fails with the tier reverted, and 067's data model gives the family's tier.

## 17. Implement scenario: status-gated-families-are-not-silent

- [x] Implement the behavior described in `scenarios/status-gated-families-are-not-silent.md` (owner, 2026-11-11, option (d)). Landed: `path_family_examines` and `IgnoreRules` in `runtime/src/primitives/check_artifacts.rs`, seven new tests and `the_originating_case_is_reproduced`, killed by M57–M64 in `scripts/mutation/owner-reviews-067.py`
- [x] Decide with the owner how `criterion-path-existence` classifies gitignored adopter state under a tracked root, the scenario's last edge case, before the family's `in-progress` run lands. Decided (owner, 2026-11-11, option (b)): a `gitignored` skip, excluded by construction, by the repository's ignore rules through `git2`; and `/.ductus/results/` joins `framework/templates/project/gitignore` and this repository's `.gitignore`
- [x] Sweep the new scope and the new reasons, `status-gated` and `gitignored`, into what states the old ones, re-grepped by meaning when this task starts. A by-meaning grep on 2026-11-11 found: `runtime/src/primitives/check_artifacts.rs`, `runtime/src/schema/primitives.rs` (both reason-set doc comments) and `runtime/src/schema/descriptions.rs` (`CHECK_ARTIFACTS`); `framework/constitution.md` §implement-phase; `framework/reference/analyze.md` (the path-existence scope and flip-to-`done` bullets, and the class list in §Unexamined targets (informational)); `AGENTS.md` (the `skipped` gotcha and the "`unexamined` cannot be measured while in-progress" gotcha); this spec's data model and its `check-artifacts-skipped-targets`, `criterion-path-existence-family` and `body-path-existence-family` scenarios; 066's data model and its `write-analysis-and-the-second-gate` and `a-done-spec-has-no-transition-to-gate` scenarios; 058's `only-unreadable-targets-retain-decisions` scenario; 045's spec (AC16 and its scope paragraph) and data model; 050's spec and plan. Applied with the runtime change, plus `docs/analyze.md`'s family table, except two. 058's scenario is left as it stands: its two lists sit in its Context, dated 2026-09-25, and its Behavior points at the canonical class list. 045's edits landed after `status(045): done` (`fab8d548`), reopening 045, which takes its own review and analysis (owner, 2026-11-11, option (b), through 045's closing session; the transition approved by the owner here): AC16 names an `in-progress` or `done` spec's criterion and adds `gitignored` to its exclusions, with a note that 067 widened it; its Resolved Questions' scope line says `done` specs only when decided and `in-progress` too since 067; its data model's path-grammar scope line names `in-progress` specs, and its reason table gains `gitignored` and `status-gated` rows

- **Done when**: The scenario's open question is resolved; at `in-progress` both path families examine the spec, and at any other status each records one `status-gated` skip, excluded by construction; a criterion path git ignores is a `gitignored` skip, and the template carries `/.ductus/results/`; fixture tests assert each direction and fail with it removed; and every swept artifact states the new scope.

## 18. Implement scenario: a-scenario-file-is-checked-against-the-scenario-schema

- [x] Implement the behavior described in `scenarios/a-scenario-file-is-checked-against-the-scenario-schema.md` (owner, 2026-11-11, option (a2)). Landed: `artifact_kind`, `validate_spec_row`, `scenario_row_problems`, `validate_scenarios_beside` and `validate_record` in `runtime/src/primitives/validate_frontmatter.rs`, fifteen tests killed by M65–M75 in `scripts/mutation/owner-reviews-067.py`; all 75 `spec.md` runs cover the 314 scenarios with no finding
- [x] Sweep what states the old selection, re-grepped by meaning when this task starts. A grep on 2026-11-11 found: the `path` doc on `ValidateFrontmatterArgs` in `runtime/src/schema/primitives.rs` and `VALIDATE_FRONTMATTER` in `runtime/src/schema/descriptions.rs`; `framework/commands/analyze.md` step 2's sentence that the host judges each scenario's frontmatter; the paragraph of `framework/reference/analyze.md` §Frontmatter schema (hard fail) saying the primitive reads no scenario; this spec's data model; and the scenario-frontmatter reader `runtime/src/primitives/mod.rs` already holds, to share rather than copy. Applied: the `path` doc, `VALIDATE_FRONTMATTER`, analyze step 2, the reference's §Frontmatter schema (hard fail) and this spec's data model now state the row by path and the scenario coverage

- **Done when**: The scenario's open question is resolved; the path chooses the row and any other path is refused; a `spec.md` run checks each scenario beside it; fixture tests cover a section-only scenario (clean), a scenario missing `section` (hard fail), a legacy `spec-ref` scenario (clean), a `spec.md` (unchanged), a `spec.md` run reporting a malformed scenario, a direct record path and a refused path, each failing with its behavior removed; and analyze step 2 and its reference leave no scenario to the host.

## 19. Implement scenario: shared-constitution-references-are-judged

- [x] Implement the behavior described in `scenarios/shared-constitution-references-are-judged.md` (owner, 2026-11-11: options (a), (j) with (y), and (c)). Landed: `resolve-anchor` judges a reference on a line naming another document against the readable documents it names, a registered constitution's markers, the citing file's headings and the markers, records an unjudged one as `anchor-target-unjudgeable` or `anchor-target-unreadable`, recognizes the markers file by its resolved path and judges a whole-span citation (`runtime/src/primitives/resolve_anchor.rs`); the exec tally records both reasons (`runtime/src/interpreter/analyze_tally.rs`); eleven new tests and two rewritten, killed with M31 re-anchored by M76–M90
- [x] Build the minimal fixture (a shared constitution with `§a-b` and `§a-b-c`, one spec citing both) and prove the same-classification test fails against the current resolver before fixing. Done: `same_form_references_into_a_registered_constitution_classify_alike` failed on the unchanged resolver, all three references `kind: markers`, `resolved: false`, and passes since
- [x] Correct the constitution's Cross-document references bullet: shared constitutions, unchecked qualified references, and the stale longest-first claim. Done, with `AGENTS.md`'s matching gotcha
- [x] Update `framework/commands/analyze.md` step 4 and `framework/reference/analyze.md` to match, the two unexamined classes in §Unexamined targets (informational) among it, batched with task 17's edit of the same class lists. Done: step 4 records both reasons, the reference classifies them, and 066's two class lists follow
- [x] Decide with the owner what a reference gets when the only file its line names is not markdown, the false alarm the scenario's Context records, before the wider judgment lands. Decided (owner, 2026-11-11, option (a)): a non-markdown file named on the line is a document with no readable section, so a reference matching no marker or heading there is unexamined, excluded by construction; and a reference naming a skipped registered entry is could not be read (owner, 2026-11-11)
- [x] Decide with the owner how the references the wider judgment flags are disposed of before it lands: measured at 78897161, 42 in `spec.md` files (26 in prose, 16 in criteria, 7 of those in 5 `done` specs: 010, 014, 024, 055, 064, which a rewrite reopens and a waiver leaves as a stored discard) and 12 under `framework/`, which Family 39 fails on; 90 more sit in plans and scenarios that no gate reads. Decided (owner, 2026-11-11, option X, after the exact measurement): fix every one, 33 in `spec.md` files, 12 under `framework/` and 128 in plans and scenarios, and extend Family 39 (`scripts/audit/shipped-anchor-resolution.sh`) to every spec's `spec.md`, `plan.md` and scenarios so a new break fails CI. Applied by four agents on disjoint files, each fix verified with the built `resolve-anchor`; Family 39 then reported 0 findings over 542 files. The edits reopen 002, 010, 014, 024, 036, 038 and 051 (`classify-spec-edit`: criterion or scenario), applied under the owner's delegation of those reopens (2026-11-11); the other touched `done` specs read as corrections
- [x] Sweep what states that qualified references are counted and never judged, re-grepped by meaning when this task starts. A grep on 2026-11-11 found: `anchor-reference-kinds`' two resolved questions and `pointers-into-shipped-files-are-judged` in this spec, and this spec's AC2 and data model; 026's AC26 and `scripts/audit/shipped-anchor-resolution.sh`'s header; `RESOLVE_ANCHOR` in `runtime/src/schema/descriptions.rs`; and the config-sourced path boundary's list on `validate_no_traversal` in `runtime/src/primitives/mod.rs`, which gains `resolve-anchor`. Applied: both resolved questions of `anchor-reference-kinds` and its kind bullets and basename edge case, `pointers-into-shipped-files-are-judged`, this spec's AC2 and data model, 026's AC26, Family 39's header and its descriptions in `framework/reference/audit.md` and `scripts/audit/README.md`, `RESOLVE_ANCHOR`, and the config-sourced boundary's list

- **Done when**: The scenario's open questions are resolved; the markers file is recognized by its resolved path; references into registered documents, references on lines naming other documents, and whole-span citations are judged as the scenario's Behavior states, and what cannot be judged is recorded unexamined in its class; each behavior's test fails with it removed, M31 re-anchored; the flagged references are disposed of as the owner decided, leaving Family 39 with none the wider judgment introduced; and the constitution, the analyze reference and every swept artifact say so.

## 20. The plan's Overview describes the runtime work 067 did

- [x] specs/067-analyze-and-audit-checks/plan.md — the Overview says "No runtime behavior changes", while 18 commits to `runtime/src` carry 067's number, tasks 5–11 and its review rounds among them (048's review, 2026-10-29). Restate the Overview, and classify each primitive interface those commits changed as additive or breaking under BE-COMPAT-001, as the Technical Decisions section now does for `check-promotion-coverage`'s empty-`sections` refusal. Done: the Overview names the runtime work tasks 5–11, 14, 17–19 and 25 and the review rounds did, and **Primitive interface changes, classified under `BE-COMPAT-001`** classifies each commit's change, `abec64ab`'s breaking one shipped in 0.59.10 among them

- **Done when**: the Overview states the runtime work 067 did, and every primitive interface change in it is classified additive or breaking

## 21. Implement scenario: deprecation-labels-are-read-by-the-runtime

Starts once spec 076's runtime work has landed: registering a primitive edits `runtime/src/main.rs`, `runtime/src/mcp/server.rs` and `runtime/src/schema/primitives.rs`, which held 076's uncommitted work when this task was written (owner, 2026-11-02).

- [x] Implement the behavior described in `scenarios/deprecation-labels-are-read-by-the-runtime.md`
- [x] The primitive, registered at every site `docs/developing-ductus.md` lists, sharing `check-rule-ids`' label reader
- [x] `scripts/audit/deprecation-removal.sh` calls it and drops its own parse, and the cases of `scripts/tests/test-deprecation-removal.sh` carry over unchanged, its fixture builder supplying the runtime binary

- **Done when**: Family 40 reads every label through the primitive, its test passes, and the primitive's tests fail when its label reader is removed.

## 22. Implement scenario: a-record-cites-a-removed-rule

- [x] Resolve the scenario's open question with `/ductus:clarify`, before the release that removes 077's four deprecated rules (0.62.0)
- [ ] Implement the behavior described in `scenarios/a-record-cites-a-removed-rule.md`
- [ ] `check-rule-ids` passes a record's citation of a removed rule and still reports one as a current requirement, each shown by a test that fails without it

- **Done when**: a record citing a removed rule passes `check-rule-ids`, a current-requirement citation of one still fails it, and both are tested.

## 23. Implement scenario: spec-integrity-and-plan-consistency-are-run

- [ ] Implement the behavior described in `scenarios/spec-integrity-and-plan-consistency-are-run.md`

- **Done when**: every `/{project}:analyze` path runs §Spec integrity (blocking) and §Plan consistency (blocking if plan exists) of `framework/reference/analyze.md`, each failure a Blocking finding and each check a path cannot run recorded unexamined, proved to fail on a fixture holding each condition

## 24. check_artifacts.rs's module doc counts the families it owns

- [x] `runtime/src/primitives/check_artifacts.rs`'s module doc says it "Owns ten families", while it emits eleven family literals since 079 added `cross-spec-impact-drift`: state the count the code holds, or point at `schema::descriptions::CHECK_ARTIFACTS` instead of counting (054's review, 2026-11-09, outside its scope; deferred here by the delegated rule)

- **Done when**: the module doc states no family count the code does not hold

## 25. Move `unwritable` into derive-dependencies' bounded file form once 080 is done

- [x] Task 14's `unwritable` is inline in both forms, so a result past the read size still carries it, against 080's rule that a result growing with the corpus is bounded (`BE-PAYLOAD-001`). Once 080 is `done`, move it with the other lists: in `derive_dependencies.rs` `bound()`, add `"unwritable"` to the moved lists, make the field `Option<Vec<String>>` absent in the file form, with a test driving it past the bound and a mutation. Then add it to the `derive-dependencies` row of 080's `data-model.md` table, synced as a mechanical edit to a `done` spec (§spec-lifecycle), and state it here in the data model (owner, 2026-11-10). Landed with `past_the_read_size_unwritable_moves_to_the_list_file`, killed by M51 and M52; the row's edit classifies as reopening 080 (`classify-spec-edit`: data-model) and stales its review, so the owner reopened 080 to close it again (2026-11-11)

- **Done when**: derive-dependencies' file form moves `unwritable`, 080's table lists it, and a test fails with the list left inline

## 26. The shipped hook's header names every exit its derivations gate on

- [x] `framework/bootstrap/hooks/ductus-pre-commit`'s header says step 1, derive-dependencies, exits non-zero on a cycle, and says nothing of step 2's exit; since d90d2133 both also exit non-zero on an `unwritable` spec, which stops the commit whether or not that spec is staged (task 14's fourth item, owner, 2026-11-11). State both exits, and that `unwritable` spans the corpus as the cycle check does

- **Done when**: the header names the cycle and `unwritable` exits of step 1 and the `unwritable` exit of step 2, each as spanning every tracked spec

## 27. The path families record a candidate that leaves the project, never probe it

- [x] `check_criterion_path_existence` probes `repo.join(candidate).exists()` with no `..` screen, and `resolve_body_path` passes a candidate whose `..` is interior (`specs/../../x`) through as project-relative, so a criterion or body line naming such a path makes the runtime stat a file outside the project (`BE-INPUT-004`, `runtime/src/primitives/check_artifacts.rs`). Found by 045's analysis; the owner deferred it here, since this spec owns the analyze checks (2026-11-11). Resolve every candidate lexically, as `resolve_sibling` does, and record one whose resolution leaves the project as a skip instead of probing it, with a test per family that fails with the screen removed. Recording it as `root-absent`, since `..` is no top-level segment this repository contains, keeps the closed reason set unchanged; a new reason would also change 045's `data-model.md`, which lists that set, and reopen 045

- **Done when**: neither path family probes a path outside the project, each records such a candidate as a skip, and a test per family fails with the screen removed
