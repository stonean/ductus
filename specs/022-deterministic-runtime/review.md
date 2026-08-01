---
spec: 022-deterministic-runtime
reviewed-at: 2026-08-01T23:05:00Z
reviewed-against: bd608a2993ee10f3d1e4b87d98f8b1215e1f6677
diff-base: 8928fcae982d9e5db2f53455a993b1693fb7e631
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 3
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Clean: 0 MUST, 0 SHOULD, 0 low-confidence across all five passes, against the two scenarios grooming added (tasks 70-71). Scope is the plan's Affected Files set (33 entries, larger than the 12-file modified-since set, so it wins per the not-a-union rule); the reviewed code is `derive_boundary.rs`, `diff_cross_spec.rs`, `mark_task.rs`, `read_tasks.rs`, `interpreter/mod.rs`, `schema/primitives.rs`, and the paired `framework/commands/` prose.

One finding surfaced during the run and was fixed before this record was written, which is the review doing its job rather than a clean first pass: `diff-cross-spec` degraded to bare empty lists on an uncommitted spec dir, but its contract defines empty lists as *the no-impact outcome*. That made `/gov:implement` steps 7 and 12 assert "no cross-spec impact" when the truth was "no window to diff, impact unknowable" — a positive claim the primitive could not vouch for, and the same silent-zero shape `QUAL-STUB-001` exists to catch. It now carries `guidance` like `derive-boundary`, the consuming prose reports unknown rather than absent, and two tests pin both presence and absence.

Security: no new input paths, no network, no secrets; `validate_no_traversal` and `write_atomic` are unchanged, and the new git reads are local-repository queries. Reuse: `first_commit_for_prefix` stays the single shared derivation across both primitives, and the two guidance strings are deliberately distinct messages rather than a forced shared helper. Quality: the `mark-task` write guard was widened from `previous != checked` to a content comparison because the clause tick can be the only change — without that, an already-coherent block would still rewrite. Efficiency: one extra bounded scan of a task range on completion, plus two cheap `is_empty()` ref lookups. Simplicity: the `zip` that collapses the two no-history shapes into one arm is the only clever construction and is commented at the point of use.

Deterministic corroboration at this HEAD: 892 tests pass, `cargo clippy --all-targets` clean, `cargo fmt` clean (enforced by the pre-commit hook, which rejected the first commit attempt), `scripts/audit/run-all.sh` exits 0, markdownlint clean over 365 files.

Recorded against a committed sha deliberately. The first attempt at this review resolved a scope from `cde0dc3..HEAD` that contained none of the reviewed code, because the work was still uncommitted — writing then would have stamped a sha onto a review of code that sha does not contain, and `check-review-gate` would have passed 022 to `done` on that record. The work was committed first (8928fca, bd608a2) so `reviewed-against` and the reviewed code agree, preserving the documented idempotency invariant that review output is a function of code plus rules.

Not blocking. 022 remains `in-progress` pending the `/gov:implement` completion gate.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] **Release sequence for the 045/046 work — do not tag `gvrn-v` until every box below is checked** (state refreshed 2026-07-30 by `/gov:groom`; tasks 70/71 now landed, so step (2)'s two scenarios are implemented and 022 awaits this review plus the done gate).
- [ ] `mark-task`'s checkbox-form done-when reconciliation is tick-only, never untick — decide whether that asymmetry should stand (logged during task 71, with both candidate resolutions and a recommendation).
- [ ] Running `gen-spec-deps.sh` manually right after creating a spec reported "No changes (all specs in sync)" while `dependencies` stayed `[]` — `list_specs()` enumerates via `git ls-files`, so a brand-new spec is invisible until staged.

## Skipped passes

*None.*
