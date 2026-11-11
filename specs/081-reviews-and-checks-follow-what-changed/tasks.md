# 081 — Reviews and checks follow what changed Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. A review is owed for code, by its own commits

- [ ] Add `is_project_artifact` to `runtime/src/primitives/mod.rs`, with unit tests for each member of the set, a configured spec root, and a path outside it.
- [ ] Add `CommitClaims::named` in `runtime/src/primitives/commit_attribution.rs`; derive `first_named` from it.
- [ ] Rewrite `paths_owing_a_review` and `record_is_held` in `runtime/src/primitives/compute_review_scope.rs` per the plan's owed rule, keeping `contracts_against_digest` for scope.
- [ ] Rewrite `attribute_window` per the plan's attribution table, recording a path no spec owes under `no spec` in `excluded`.
- [ ] Fixture tests proving both directions of AC1, AC8, AC9 and AC10: a contract-only window, a configured `specs-root`, a project in a repository subdirectory, a named commit with a second claimant, an unnamed commit with a listed and an unlisted path, and a merge commit. Each test is shown to fail with its behavior removed (`QUAL-TEST-001`).
- [ ] Build the release binary and re-run `compute-review-scope` over the 12 specs the spec's Motivation measured. Where the counts differ from the Resolved Questions' figures, correct the spec.

- **Done when**: the new tests pass and are proven to fail without their behavior, and the built binary's `review-owed` over those 12 specs matches what the spec records.

## 2. The gate stops blocking on a stale review digest

- [ ] Remove check 6 (`stale_review_block`), `ReviewGateBlock::ReviewStale` and the review half of `passing_notices` from `runtime/src/primitives/check_review_gate.rs`.
- [ ] Update the gate's tests: a contract-only change since the review passes every review check, and a code change since it still blocks on the review owed (AC2).
- [ ] Re-bless any golden whose recorded gate output names `review-stale`, reading each diff.

- **Done when**: `cargo test` passes with the two-direction test proven to fail without the change.

## 3. Retire `check-review-freshness` and Family 19

Land tasks 1 to 3 together, by `docs/developing-ductus.md` §Landing a runtime change, so the runtime moves once.

- [ ] Remove the primitive's seven sites (`docs/developing-ductus.md` §Adding or removing a runtime primitive) and delete `runtime/src/primitives/check_review_freshness.rs`.
- [ ] Add its allow entry to the retired list in `framework/bootstrap/configure/claude.md`, and re-run `scripts/gen-configure-mcp.sh`.
- [ ] Delete `scripts/audit/review-freshness.sh` and `scripts/mutation/family-19-calls-the-runtime.py`. Unregister Family 19 from `scripts/audit/run-all.sh`, `framework/reference/audit.md` and `scripts/audit/README.md`, leaving its number spent.
- [ ] Add the unreleased entry to `runtime/CHANGELOG.md`.

- **Done when**: tasks 1 to 3 are committed in one landing, every step of that procedure passing; `scripts/audit/run-all.sh` passes after the commit (Family 28 included); and the commit is pushed with every workflow run on it read.

## 4. The shipped text says the new rule

- [ ] Rewrite `compute-review-scope`'s and `check-review-gate`'s descriptions in `runtime/src/schema/descriptions.rs`. This lands with task 3 if it is reached first.
- [ ] Correct `framework/reference/review.md` (§Record the run in `review.md`, §Window and coverage), `framework/reference/implement.md` step 5, `framework/reference/fold.md` and `docs/analyze.md`.
- [ ] Prose-claim sweep by meaning over the live-artifact set: `reviewed-digest`, "durable contract", "stale", `review-stale`, Family 19, `check-review-freshness`. Keep the records of the decision as written.

- **Done when**: no live artifact outside a decision record says a scenario or data-model edit owes a review, or names Family 19 or `check-review-freshness` as current, by a grep whose patterns are in the commit message.

## 5. The constitution and `AGENTS.md`

- [ ] Amend §grounding's blast-radius rule with the division and the artifact set, and correct the review-record digest sentence, cutting `framework/constitution.md` to stay at or under 50,000 bytes.
- [ ] Rewrite `AGENTS.md`'s local-checks rule for an artifacts-only change, and remove the "durable contracts stale its review" gotcha.

- **Done when**: `scripts/lint-governing-size.sh` passes, AC4's statements are in the constitution, and `scripts/audit/run-all.sh` passes after the commit.

## 6. A spec-only push compiles nothing

- [ ] Cache `runtime/target/release/ductus` in `.github/workflows/framework-checks.yml` and `.github/workflows/generators.yml` under the plan's source-hash key, building only on a miss and printing why.
- [ ] Run `actionlint` over both.

- **Done when**: on a pushed spec-only commit following a cached build, both workflows' runs show no `Build the runtime` compile, read from `gh run view --json jobs`; and a run on a key miss shows the compile and its message.

## 7. Cross-spec corrections and back-links

- [ ] 079: AC4's annotation, AC6 and the Resolved Questions on scenario edits and attribution point here (AC5). The edit reopens 079 with `set-status` (`from: done`).
- [ ] 066: its gate checks and `blocked-by` values in the data model and scenarios.
- [ ] 026: its Family 19 criteria and scenarios say it is retired.
- [ ] 047: the `analyze-run-durability` row naming Family 19. This reopens 047.
- [ ] 050: a back-link for the constitution's division and check-selection rule.

- **Done when**: each of 079, 066, 026, 047 and 050 links back to 081, and `check-review-gate` reports 081's `cross-spec-impact:` discharged.
