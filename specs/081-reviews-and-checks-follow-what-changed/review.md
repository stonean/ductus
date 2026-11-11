---
spec: 081-reviews-and-checks-follow-what-changed
last-run: 2026-11-11T22:36:19Z
reviewed-against: 07522e25de26b58a1933ff90b27eaa40094b54fc
diff-base: fa4b0dd28735bcede13a868542e7e3f903b28fe9
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 46
scope: 59
excluded: 21
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest: {}
blocking: false
dispositions:
  fixed: 12
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Review — 081-reviews-and-checks-follow-what-changed

## Summary

Five-pass review of 081's first reopening over `fa4b0dd2..07522e25` plus this run's fixes (base-source `reopen`). Scope 59 entries, examined 46. **Read:** the window's whole diff for every changed path outside the spec directories (the 33688944 landing and 2496b7c4's workflows), with surrounding source wherever a claim needed it; `scripts/mutation/reviews-follow-what-changed.py` whole; 081's criteria, plan and tasks; the doc blocks of `analyze_subjects.rs`, `classify_spec_edit.rs` and `invalidate_review.rs` a sweep by meaning reached; `framework/commands/review.md` and `prune.md` at the lines that sweep hit; the window's changes under `runtime/tests/*.rs` (`mechanical_sweep_parity.rs`, `result_sizes.rs`), the pattern's other files unchanged and unread; and the re-bless diff of `runtime/tests/golden/review-basic.jsonl`, which moves only its fixture repo's `diff-base` sha, the field the parity test names, after the review command's text changed. **Not read (13):** the nine generated mirrors under `.claude/commands/ductus/`, `.ductus/reference/` and `.pi/prompts/`, generated from sources read above, `scripts/gen-claude-commands.sh --check` in sync; the `runtime/tests/golden/*` pattern's other goldens, unchanged in the window and grepped for the retired names with no hit, the grep proven on `runtime/CHANGELOG.md`; and the directories of 026, 050 and 066, whose changed files are those specs' own under 081's attribution and go to their analyses, 066's data-model gate list checked against `ReviewGateBlock` only. **Driven on real state:** `compute-review-scope` reports `review-owed: false` for 079 and 047, which the artifacts-only debcc8cf reopened, and leaves 2496b7c4's workflows, a commit naming 081 alone, out of both windows; `check-review-gate` on 038 blocks on `analyze-stale` alone. The AC2, AC8, AC9 and AC10 fixture tests were read, not mutated again; `run.py --check` finds all 277 anchors of the two mutation lists valid. **One MUST, fixed in the run:** `BE-COMPAT-001`, since the plan did not classify the retired tool, the narrowed `blocked-by` set and `review-owed`'s changed answer as breaking; its Technical Decisions now do, shipping with the 0.60.0 minor bump, and `runtime/CHANGELOG.md` marks both entries breaking. **Observations: 12, all fixed by the rule**, one a broken contract: `framework/reference/review.md` still said a scenario or data-model correction stales a review, against AC4. The rest are present-tense claims of the retired review staleness check that task 4's sweep missed, in the review and prune commands, `docs/analyze.md`, six runtime files' doc comments and the workflows' depth comment, and the plan's 047 rows. Over the fixed tree, markdownlint, actionlint, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, the touched modules' unit tests and the parity suite pass.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Deferred findings

*None.*

## Observations

- bug: §Run review passes said a scenario or data model's correction stales this review, which AC4 says the file no longer says — `framework/reference/review.md:96` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: step 1 described review-owed: false as a reopening that changed no code, scenario or data model, where a scenario or data-model edit owes no review — `framework/commands/review.md:119` — **fixed** (decided by the rule)
- bug: step 10 said the review record stays valid because plan.md and tasks.md are not durable contracts, implying a scenario or data-model edit would stale it — `framework/commands/prune.md:68` — **fixed** (decided by the rule)
- bug: the freshness section said the review record's reviewed-digest works identically to the analysis record's at the gate, a check spec 081 retired — `docs/analyze.md:224` — **fixed** (decided by the rule)
- bug: is_review_contract's, is_analyze_subject's and exempt_renames's docs and a test comment said a digested review.md or spec.md would stale every review, named the gate's durable contracts and the deleted stale_review_block, and cited the deleted review-freshness.sh, the doc edit the plan lists for this file — `runtime/src/primitives/analyze_subjects.rs` — **fixed** (decided by the rule)
- bug: render_paths's doc named the retired review-staleness block as one of its two messages, and a test comment sent a contributor to a review that would go stale — `runtime/src/primitives/check_review_gate.rs:761` — **fixed** (decided by the rule)
- bug: the module doc said the gate's review staleness uses mechanical_sweep's index, which only the analysis record's freshness now does — `runtime/src/primitives/classify_spec_edit.rs:17` — **fixed** (decided by the rule)
- bug: the module doc described the gate as diffing a spec's durable contracts between reviewed-against and HEAD, a check spec 081 retired — `runtime/src/primitives/invalidate_review.rs:4` — **fixed** (decided by the rule)
- bug: a test comment said an unreadable sweep index leaves the review stale, where it leaves the record stale — `runtime/src/primitives/mechanical_sweep.rs:572` — **fixed** (decided by the rule)
- bug: reviewed_digest's doc said counting review.md would stale every review the instant it was recorded — `runtime/src/schema/primitives.rs:57` — **fixed** (decided by the rule)
- convention: the depth-1 checkout comment named Families 7 and 8 as dashboard's callers and omitted Family 37, in framework-checks.yml and runtime-release.yml alike — `.github/workflows/framework-checks.yml:43` — **fixed** (decided by the rule)
- bug: the plan's Affected Files and Cross-spec impact named 047's analyze-run-durability scenario as edited, where debcc8cf edited analyze-record-freshness and 047's spec — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `specs/026-framework-self-audit/spec.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/041-task-pruning/scenarios/a-plan-edit-does-not-reopen-a-done-spec.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/047-analyze-findings-durability/scenarios/analyze-record-freshness.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/047-analyze-findings-durability/spec.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/050-constitution/plan.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/050-constitution/scenarios/a-review-may-precede-its-commit.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/050-constitution/spec.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/plan.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-multi-number-renumber-is-one-rewrite.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-renamed-file-contributes-its-rewrites.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/scenarios/freshness-results-have-a-bounded-size.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-staleness-on-done-specs.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/scenarios/the-committed-tree-horizon.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/spec.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/067-analyze-and-audit-checks/scenarios/orphaned-reference-check.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/067-analyze-and-audit-checks/scenarios/the-promotion-coverage-line.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/spec.md` — 026-framework-self-audit, 041-task-pruning, 047-analyze-findings-durability, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges

## Unexamined governance

*None.*
