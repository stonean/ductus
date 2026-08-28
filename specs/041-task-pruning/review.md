---
spec: 041-task-pruning
reviewed-at: 2026-08-28T01:24:04Z
reviewed-against: a9be853143093fc9891a87048ba286fc187ddfcd
diff-base: 9ab3163db47064584fd29ef4a7eb041865be3767
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 041-task-pruning

## Summary

Re-run 2026-08-28 against the current rule set. 0 MUST, 0 SHOULD, 0 low-confidence; not blocking.

**Why this re-run happened.** The original review ran 2026-07-11 and recorded one SHOULD. Two rule IDs now in force did not exist then — `FE-DEPS-005` (2026-07-21) and `QUAL-CLAIM-001` (2026-08-02) — so the verdict was re-derived rather than trusted, and the prior finding re-checked against the code as it stands rather than against its own Status line.

**The prior SHOULD is genuinely resolved, verified in the code.** It reported `heading_is_numeric` / `split_numbered_heading` triplicated across `prune_tasks.rs`, `read_tasks.rs`, and `mod.rs`. Those local definitions are gone: `primitives/mod.rs` is now the single `pub(crate)` home for both, with `heading_is_numeric` defined as `.is_some()` on the splitter so the predicate cannot drift from it, and a unit test asserting the two agree. Landed under 022's `numbered-heading-grammar-single-source` (022 task 78, 2026-08-02). The count drops out because the finding no longer fires, not because it was reclassified.

**`QUAL-CLAIM-001` was assessed against `prune-tasks` and does not fire.** The rule flags a code path that returns a clean or empty result while some part of its nominal subject went unexamined. Every unexaminable path here is an operational error instead: a missing feature directory returns `FeatureNotFound`, a missing `tasks.md` returns `TasksFileMissing`, and an unreadable file propagates the I/O error. If `run` returns `Ok` at all, it read and segmented its subject. The result is also self-describing where it matters — `nothing_to_prune`, `gate`, `applied`, per-section classification records, and `size_before` / `size_after` — so a no-op is distinguishable from a blocked reset and from a preview. This is the rule's documented compliant case: a total function whose subject is always fully examinable.

`FE-DEPS-005` governs frontend dependency network egress; 041's subject is a Rust runtime primitive and its command prose, with no frontend surface in scope.

**The stale count is corrected.** `spec.md` recorded `should-violations: 1` while this report recorded `0` — the report was updated when the finding was resolved, the spec frontmatter was not, so the two disagreed for the four weeks since. Both are now written from the same run.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Observations

*None.*

## Skipped passes

*None.*
