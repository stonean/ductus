---
spec: 057-analyze-artifact-and-record-relocation
last-run: 2026-11-16T13:32:11Z
reviewed-against: 069cc658dec01b928ff21ddb25b21a89fd117df8
diff-base: 7ab30b2736a3d1a2f4f1f4ea7e7a2ba6f8badeee
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 6
scope: 9
unread:
  - runtime/CHANGELOG.md
  - runtime/src/primitives/mod.rs
  - runtime/src/schema/descriptions.rs
excluded: 75
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: a2e1a24df11eb481ff944403e68e5aa6f39260449d504e8333c604a0a1afccb8
blocking: false
dispositions:
  fixed: 2
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: "bug — a write or lock failure after an earlier artifact was written (the lock wait timing out, or the rename failing) returns a plain Io error that does not name the files already written, though the relocation is then half done; only a changed-since-read refusal becomes relocation-incomplete. Judged against the contract: the data model and task 27 state the half-done report for a refused (changed) file only, the Io error names its own path, and a re-run is idempotent (AC20), so no stated requirement is broken — `runtime/src/primitives/relocate_audit_records.rs:205-214`"
    outcome: waived
    reason: 057's contract (data model Errors table, task 27) states the half-done report for a changed-since-read refusal; an Io or lock-timeout failure after a write has no stated requirement, names its failing path, and the merge is idempotent so a re-run finishes the relocation
    decided-at: 2026-11-16T13:32:11Z
    decided-by: rule
---

# Review — 057-analyze-artifact-and-record-relocation

## Summary

057's review of task 27 (`fa844bdd`), which puts `relocate-audit-records` on the compare-and-set guard. One agent ran all five passes and read all nine loaded backend and cross rule files whole. It read 6 of the 9 scope entries whole: `relocate_audit_records.rs`, the guard's mutation list, and 057's spec, plan, data model and tasks (057 has no scenarios); `mod.rs`, `descriptions.rs` and the CHANGELOG were read at the window's hunks, with `PrimitiveError`, `ReadSnapshot` and `RELOCATE_AUDIT_RECORDS` whole. One finding, `BE-COMPAT-001` (must, low confidence): the plan did not classify task 27's two new errors; the fix round adds the classification, breaking within major version zero, so it no longer fires. Three observations: a broken contract, fixed in the run (a refused relocation left half done dropped the `disagreements` the plan says are named, never smoothed; `relocation-incomplete` now carries the disagreements found for the files it wrote, proven by two tests and mutants M7 and M8); a chore fixed (the write order, the artifacts in the order their blocks appear in `spec.md` then `spec.md`, in the data model, the description and the CHANGELOG); and one waived by the rule with its reason. The fix round was made in a worktree, its whole mutation list run there (8 of 8 killed) and the whole `cargo test` passed, then landed in the main tree. An independent agent re-read the round against the whole contract: spec and data model, 29,652 bytes, read whole; clean, with `plan.md` exempt as an analysis subject, and the loop's closing round was empty.

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

- bug — a RelocationIncomplete refusal drops the disagreements the plan says are named, never smoothed: the artifacts already written carry the spec block's values, so the run that errored never reports which keys disagreed, and the safe re-run finds artifact and block equal and reports none; the only evidence the two copies drifted is gone from the result (driven: a re-run over an artifact already holding the block's values returns no disagreements) — `runtime/src/primitives/relocate_audit_records.rs:202-225` (breaks the spec's contract) — **fixed** (decided by the rule)
- convention — the write order is stated as review.md, analysis.md, then spec.md, but the code writes the artifacts in the order their blocks appear in spec.md, then spec.md (driven: a spec with analyze before review wrote analysis.md first); the claim sits in the data model Errors paragraph, the RELOCATE_AUDIT_RECORDS description and the CHANGELOG entry — `specs/057-analyze-artifact-and-record-relocation/data-model.md:107` — **fixed** (decided by the rule)
- bug — a write or lock failure after an earlier artifact was written (the lock wait timing out, or the rename failing) returns a plain Io error that does not name the files already written, though the relocation is then half done; only a changed-since-read refusal becomes relocation-incomplete. Judged against the contract: the data model and task 27 state the half-done report for a refused (changed) file only, the Io error names its own path, and a re-run is idempotent (AC20), so no stated requirement is broken — `runtime/src/primitives/relocate_audit_records.rs:205-214` — **waived**: 057's contract (data model Errors table, task 27) states the half-done report for a changed-since-read refusal; an Io or lock-timeout failure after a write has no stated requirement, names its failing path, and the merge is idempotent so a re-run finishes the relocation (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md` — 066-audit-records-and-the-completion-gate
- `.claude/commands/ductus/consolidate.md` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `.claude/commands/ductus/fold.md` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `.claude/commands/ductus/prune.md` — 022-deterministic-runtime, 041-task-pruning
- `.claude/commands/ductus/review.md` — 066-audit-records-and-the-completion-gate
- `.ductus/reference/fold.md` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `.pi/prompts/ductus-analyze.md` — 066-audit-records-and-the-completion-gate
- `.pi/prompts/ductus-consolidate.md` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `.pi/prompts/ductus-fold.md` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `.pi/prompts/ductus-prune.md` — 022-deterministic-runtime, 041-task-pruning
- `.pi/prompts/ductus-review.md` — 066-audit-records-and-the-completion-gate
- `AGENTS.md` — 050-constitution
- `framework/commands/analyze.md` — 066-audit-records-and-the-completion-gate
- `framework/commands/consolidate.md` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `framework/commands/fold.md` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `framework/commands/prune.md` — 022-deterministic-runtime, 041-task-pruning
- `framework/commands/review.md` — 066-audit-records-and-the-completion-gate
- `framework/reference/fold.md` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `runtime/src/main.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/bounded_lists.rs` — 022-deterministic-runtime
- `runtime/src/primitives/derive_dependencies.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/derive_references.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/invalidate_review.rs` — 051-branch-scoped-spec-numbering
- `runtime/src/primitives/prune_plan.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/src/primitives/prune_project.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/src/primitives/prune_tasks.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/src/primitives/rewrite_spec_links.rs` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `runtime/src/primitives/write_analysis.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/write_review.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/schema/paths.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/src/schema/primitives.rs` — 022-deterministic-runtime, 041-task-pruning, 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation, 067-analyze-and-audit-checks
- `runtime/src/session.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/tests/golden/review-basic.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/line_ending_discipline.rs` — 066-audit-records-and-the-completion-gate
- `runtime/tests/mcp.rs` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation, 067-analyze-and-audit-checks
- `scripts/mutation/compare-and-set-041.py` — 022-deterministic-runtime, 041-task-pruning
- `scripts/mutation/compare-and-set-051.py` — 051-branch-scoped-spec-numbering
- `scripts/mutation/compare-and-set-066.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/compare-and-set-067.py` — 067-analyze-and-audit-checks
- `scripts/mutation/containment-and-bounds-067.py` — 067-analyze-and-audit-checks
- `scripts/mutation/owner-reviews-067.py` — 067-analyze-and-audit-checks
- `scripts/mutation/prune-project-containment.py` — 022-deterministic-runtime, 041-task-pruning
- `scripts/mutation/prune-project.py` — 022-deterministic-runtime, 041-task-pruning
- `scripts/mutation/rewrite-spec-links.py` — 051-branch-scoped-spec-numbering
- `specs/022-deterministic-runtime/plan.md` — 022-deterministic-runtime, 041-task-pruning, 051-branch-scoped-spec-numbering, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/022-deterministic-runtime/tasks.md` — 022-deterministic-runtime, 041-task-pruning, 051-branch-scoped-spec-numbering, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/041-task-pruning/analysis.md` — 041-task-pruning
- `specs/041-task-pruning/data-model.md` — 022-deterministic-runtime, 041-task-pruning
- `specs/041-task-pruning/plan.md` — 022-deterministic-runtime, 041-task-pruning
- `specs/041-task-pruning/review.md` — 022-deterministic-runtime, 041-task-pruning
- `specs/041-task-pruning/spec.md` — 022-deterministic-runtime, 041-task-pruning
- `specs/041-task-pruning/tasks.md` — 022-deterministic-runtime, 041-task-pruning
- `specs/050-constitution/analysis.md` — 050-constitution
- `specs/050-constitution/plan.md` — 050-constitution
- `specs/050-constitution/review.md` — 050-constitution
- `specs/050-constitution/scenarios/the-constitution-fits-one-read.md` — 050-constitution
- `specs/050-constitution/spec.md` — 050-constitution
- `specs/051-branch-scoped-spec-numbering/data-model.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/051-branch-scoped-spec-numbering/plan.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/051-branch-scoped-spec-numbering/review.md` — 051-branch-scoped-spec-numbering
- `specs/051-branch-scoped-spec-numbering/scenarios/the-spec-root-is-read-before-the-first-write.md` — 051-branch-scoped-spec-numbering
- `specs/051-branch-scoped-spec-numbering/spec.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/051-branch-scoped-spec-numbering/tasks.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/052-spec-supersession-and-consolidation/spec.md` — 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/066-audit-records-and-the-completion-gate/plan.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/data-model.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/plan.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/review.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/derive-references-results-are-bounded.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/tasks.md` — 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/data-model.md` — 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/spec.md` — 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency

## Unexamined governance

*None.*
