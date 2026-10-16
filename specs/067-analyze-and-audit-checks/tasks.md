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

- [ ] Implement the behavior described in `scenarios/scenario-consistency-reads-tasks-history.md`
- [ ] Delete `pruning_evidence`; `check_scenario_consistency` asks `ever_tasked_slugs` at every status, and the message below `done` names the remedy instead of the fingerprint
- [ ] Rebuild the scenario-consistency tests on git-backed fixtures, and add the restart-at-1 and reset-at-done shapes, each proven to fail against the old code
- [ ] State the rule in the module and function docs, the `check-artifacts` tool description, `data-model.md`'s family line, `analyze-artifact-checks`' Behavior, and `framework/commands/analyze.md` step 8 and its Scenario consistency section

- **Done when**: an in-progress spec whose `tasks.md` restarted at 1 reports none of its implemented scenarios, a `done` spec with a reset `tasks.md` reports a scenario no revision ever named, the `done` message is byte-identical, and no live text describes the pruning fingerprint
