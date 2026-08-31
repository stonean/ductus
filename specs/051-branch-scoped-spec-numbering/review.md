---
spec: 051-branch-scoped-spec-numbering
reviewed-at: 2026-08-31T01:00:00Z
reviewed-against: 8edc39c497185d9e8f0f76bfd8ba169e3118ef6a
diff-base: eb52e24a8bf9ce362430abe583d2ef63dd9bafc9
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 051-branch-scoped-spec-numbering

## Summary

Re-review triggered by a one-phrase edit to this spec's `data-model.md` — a durable contract — during spec 054. Zero MUST violations, zero SHOULD violations, zero low-confidence findings, not blocking.

**Why this ran at all, stated plainly.** 051's implementation did not change. The edit was a post-completion note citing 052 by a title that 054 retired, corrected to the new one. Family 19's mechanical-sweep exemption did not apply and should not have: it exempts repo-wide *token* renames, which produce the same variant across many files, and a one-cell edit to a single file is not that. So the family behaved exactly as designed, and the honest remedy was to re-run rather than to revert a correct fix in order to dodge a gate.

**What was examined.** 051's implementation surface as it now stands: `retire_feature.rs` (the branch-scoped form parse, the sequential refusal and the opt-in that 052 gated it behind, and the ungated anti-stranding guard), `rewrite_spec_links.rs` (whole-path-segment matching, the `folds-into` frontmatter rewrite, and the `examined` bound), `check_unfolded_specs.rs`, `create_feature.rs`'s branch-scoped numbering and identifier sanitization, and `framework/commands/fold.md`.

Against the loaded rules it holds up. `retire-feature` orders its guards so the irreversible step is unreachable from a typo, and returns `retired: false` as a named domain outcome for an already-absent directory rather than a bare success — `QUAL-CLAIM-001` satisfied by construction. `rewrite-spec-links` reports `examined` beside `rewritten`, so an empty rewrite reads as *nothing pointed here* rather than *nothing was checked*. `check-unfolded-specs` reports a `folds-into` target absent from the tree as declared rather than broken, which is correct: before the merge that absence is the normal state, and existence is enforced at fold-back. No silent stubs, and no unowned external contracts — the paths and formats are the project's own.

**One thing worth recording rather than filing as a finding.** This is the third review in one session triggered by a durable contract changing, and in two of the three the *code* was untouched — 052's `data-model.md` was deleted because its subject was removed, and 051's carried a stale title. Family 19 keys on `scenarios/*.md` and `data-model.md` because those are what a review reads, which is right; the consequence is that prose-only edits to those two artifacts cost a review pass. Not a defect, and not something to weaken — a check that exempted prose edits would need to judge which prose matters — but it is a real cost of the rule, and worth knowing before a future sweep touches many data models at once.

**Bounds on this review.** `compute-review-scope`'s window runs from 051's own in-progress parent, so it spans every change since, including all of 054 and Family 34. This pass deliberately examined 051's implementation surface as it now stands rather than every file in that window, and says so instead of implying the wider set was read.

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
