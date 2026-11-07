---
spec: 035-groom-session-target
last-run: 2026-11-07T15:12:57Z
reviewed-against: 101a6e8e6dbd2c579db01ebc7f6d9efbdc14adb0
diff-base: c2b25537780a22490380d2304f7619ca2a816c23
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 3
scope: 3
excluded: 55
contract-breaks: 0
skipped-passes: []
fix-rounds: 0
re-reads: 0
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  scenarios/confirmation-names-reopen.md: d6ad68d0a9e3b0abe940a01b40da7559b804329f58656874bfcfc4a88f1ac160
  scenarios/reopen-done-spec-on-scenario.md: 5f1b1f7cc9a4fb3d5042d48ef2aae191697ff3821a0170ead1d019460cd6d6b2
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Review — 035-groom-session-target

## Summary

Five passes over 3 of 3 in-scope files, each read in full: `scenarios/confirmation-names-reopen.md`, `spec.md` and `tasks.md`. The window is the default, base `c2b25537` with `base-source: reopen`, the parent of `2f6be508`, which reopened this spec when spec 079's task 18 corrected the scenario; it was not overridden. It holds that commit's and `29ec6cc2`'s edits to this directory: the scenario's Step 3 Edge Case, the status line, and task 7's prose naming what this reopening owes. The 55 paths only other specs' commits changed are excluded, each listed below with the specs that claimed it.

All 12 rule files `discover-rule-files` selects were loaded, every rule's Statement read, 304 in all. No rule fires: the three files are spec text, with no code path, test, configuration value, secret, network call, dependency or UI for any Statement to apply to, and the rules that verify against a spec or plan (`BE-LOCK-001`, `BE-QUERY-001`, `BE-QUERY-002`, `BE-QUERY-004`) concern a contended resource, a query or a bulk write, which none of the three introduces.

The quality pass checked the corrected Edge Case against the contract it describes, `framework/commands/groom.md` steps 3 and 7, read for that check and outside the scope, so not counted in `examined`: step 3's confirmation names a reopen only on a scenario route, and step 7 decides a spec edit's reopen with `classify-spec-edit` on the written diff and confirms it through `gate-confirm` before `set-status`, as the scenario now says. 0 MUST, 0 SHOULD, 0 low-confidence findings and 0 observations, so no fix was written: the fix-round base `13f8c4c2` was taken before the record, and the loop's one round was empty, so no round is recorded.

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

*None.*

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `.claude/commands/ductus/review.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `.ductus/reference/analyze.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `.ductus/reference/plan.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `.ductus/reference/review.md` — 079-review-and-analysis-work-converges
- `.pi/prompts/ductus-analyze.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `.pi/prompts/ductus-review.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `docs/developing-ductus.md` — 079-review-and-analysis-work-converges
- `framework/commands/analyze.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `framework/commands/review.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `framework/reference/analyze.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `framework/reference/plan.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `framework/reference/review.md` — 079-review-and-analysis-work-converges
- `framework/templates/spec/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/CHANGELOG.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/check_artifacts.rs` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `runtime/src/primitives/check_review_gate.rs` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `runtime/src/primitives/check_stuck.rs` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `runtime/src/primitives/compute_review_scope.rs` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `runtime/src/primitives/decisions.rs` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/measure_reopenings.rs` — 022-deterministic-runtime, 041-task-pruning, 079-review-and-analysis-work-converges
- `runtime/src/primitives/prune_tasks.rs` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/read_tasks.rs` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/remove_inbox_item.rs` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `runtime/src/primitives/write_analysis.rs` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/write_review.rs` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/schema/descriptions.rs` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `scripts/mutation/reopen-measure.py` — 041-task-pruning, 079-review-and-analysis-work-converges
- `scripts/mutation/scope-attribution.py` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/008-security-rules/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/010-agent-autonomy/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/014-reclarify-backedge/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/019-config-decisions/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/data-model.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/scenarios/coverage-expansion-primitives.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/scenarios/groom-command-acceleration.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `specs/023-govern-refinement/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/026-framework-self-audit/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/030-cross-service-references/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/032-opencode-agent/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/041-task-pruning/scenarios/a-suffixed-task-number-is-a-task.md` — 041-task-pruning, 079-review-and-analysis-work-converges
- `specs/041-task-pruning/tasks.md` — 041-task-pruning, 079-review-and-analysis-work-converges
- `specs/048-govern-acquired-runtime/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/054-remove-supersession/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/058-findings-route-at-discovery/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/059-project-in-a-repository-subdirectory/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/062-concurrent-session-targets/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/data-model.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/plan.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/research.md` — 041-task-pruning, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/review.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/spec.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges

## Unexamined governance

*None.*
