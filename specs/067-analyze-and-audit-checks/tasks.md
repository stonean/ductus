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
