---
spec: 052-spec-supersession-and-consolidation
reviewed-at: 2026-08-31T00:00:00Z
reviewed-against: 88b2fad2f7d1b73c9294c1e1e3943b2e1dc3300e
diff-base: 2818a378784f5b364dec93d6cd6d5031711f390e
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 052-spec-supersession-and-consolidation

## Summary

Re-review after spec 054 narrowed 052 to consolidation alone. The spec now describes one command — `/{project}:consolidate` — and this pass examined that implementation surface: `runtime/src/primitives/retire_feature.rs` (the gated sequential refusal and the anti-stranding guard), `runtime/src/primitives/rewrite_spec_links.rs` (pointer re-pointing and its `examined` bound), and `framework/commands/consolidate.md`.

Zero MUST violations, zero SHOULD violations, zero low-confidence findings, not blocking.

Against the loaded rules the consolidation code holds up. `retire-feature` orders its guards correctly — traversal validation, then the gated sequential refusal before anything touches the filesystem, then the ungated anti-stranding check — and returns `retired: false` as a distinct domain outcome for an already-absent directory rather than a bare success, which is what `QUAL-CLAIM-001` asks for. `rewrite-spec-links` reports `examined` alongside `rewritten`, so an empty rewrite reads as *nothing pointed here* rather than *nothing was checked* — the same rule, satisfied by construction. No silent stubs (`QUAL-STUB-001`): every incomplete path either refuses with a typed error or returns a named domain outcome. No unowned external contracts (`QUAL-GROUND-001`): the paths and formats are the project's own.

**Four defects were found and fixed during this pass**, all introduced by 054's step renumbering rather than by 052's original implementation, and all in prose rather than code: `specify.md` referenced `create-feature` at step 6 in two places after it moved to step 5, and `consolidate.md` referenced the confirmation at step 4 after it moved to step 3 and `rewrite-spec-links` at step 5 after it moved to step 4 — the latter having become a self-reference. They are recorded here rather than left as findings because nothing is outstanding: every `step N` reference in both files now resolves to a step that exists.

Worth stating as a bound on this review rather than a finding: **nothing verifies that a `step N` cross-reference resolves.** These four were caught by reading, not by a check, and the same class of error is reachable by any future edit that inserts or removes a step. The scope resolved by `compute-review-scope` was also far wider than 052's subject — its diff base is 052's own in-progress parent, so the window spans all of 054's removal work; this pass deliberately reviewed 052's implementation surface as it now stands rather than every file in that window, and says so instead of implying the wider set was examined.

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
