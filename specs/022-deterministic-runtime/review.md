---
spec: 022-deterministic-runtime
reviewed-at: 2026-09-07T19:18:25Z
reviewed-against: c929cfce114c1abe511dd653478e1221022d8599
diff-base: 6c40644dc0a6240090403bd71867bf0b7a79e4f6
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Clean across all five passes: 0 MUST, 0 SHOULD, 0 low-confidence, nothing blocking. The window was narrowed with the documented `--since=6c40644` override — 022's derived diff base predates the `0.28.0` cycle and resolves hundreds of files (AGENTS.md §Gotchas), while the change actually under review is tasks 107 and 108 plus 047's freshness work, ten commits. The override yielded a 55-entry scope covering every file those commits touched.

The subject is the freshness rework: `analyze_subjects` (new), `check_review_gate`, `write_review`, `write_analysis`, `compute_review_scope`, and the `schema::primitives` records, against `security-*.md`, `quality-cross.md`, `api-backend.md`, `concurrency-backend.md`, `configuration-cross.md`, `observability-backend.md`, `performance-*.md`, `reliability-backend.md`, and AGENTS.md §Gotchas / §Boundaries / §Design Principles.

**Security** — nothing. `subject_digest` walks with `follow_links(false)`, so a symlinked scenario is never opened through its destination; every path is confined to a feature directory already screened by `validate_no_traversal`; no network, no user-controlled path outside the validated slug.

**Reuse** — nothing, and the positive direction is worth recording since it was the scenario's own open question. `review_freshness` and `analyze_freshness` are one call to `freshness_of` differing only in a `fn(&str) -> bool` predicate, and `strip_analyze_block` reuses `write_review::splice_top_level_block` rather than re-deriving "find the top-level key and its extent". Two records, one comparison, no second implementation for the two surfaces to disagree through.

**Quality** — six defects, all in prose rather than logic, all fixed in `c929cfc` before this record was written and none outstanding. Tasks 107 and 108 replaced two commit comparisons with digest comparisons and left the superseded reasoning standing inside the functions they rewrote: `check_review_gate`'s passing-verdict comment still explained the uncommitted-contract notice that the same change deleted, and still claimed the analyze check fails open on an unresolvable `analyzed-against`; `passing_notices`' doc carried a half-edited sentence with no predicate; `ReviewBlock::reviewed_digest` held two paragraphs giving opposite answers for an empty digest, which is the exact distinction the field's `Option` wrapper exists to draw; `ReviewGateBlock::AnalyzeStale` still defined staleness as changed "after `analyze.analyzed-against`"; `write_review::analyze_freshness_of` linked `analyze_subjects::freshness`, which does not exist; and `docs/analyze.md` §`analyzed-against` asserted that nothing gates on analyze freshness — false since 047 — one page above the section describing the gate that does, with the record's two new fields undocumented. None maps to a loaded rule: `QUAL-STUB-001`, `QUAL-GROUND-001`, and `QUAL-CLAIM-001` govern code paths, and every one of these is a correct code path with false prose attached. They are recorded here rather than as findings for that reason, and as fixed rather than open because they are. The durable half is in AGENTS.md: the prose-claim-sweep rule now covers the changed code's own comments and `docs/`, since none of the six was reachable by grepping an identifier — the words that went false were `committed tree`, `diff`, and `since it ran`.

**Efficiency** — nothing rising to a finding. The gate digests the feature directory twice per run, once per subject set, and 022 is the corpus's largest feature at ~90 scenarios; the code already avoids a third walk by threading each computed `RecordFreshness` to the passing verdict rather than recomputing it, and the cost it replaced was two git tree diffs.

**Simplicity** — nothing. `already_done_block` is a seam beside `markdown_lint_block` and `pending_fold_block` rather than logic folded into `run_with_lint`, which keeps that function under the line ceiling clippy enforces and keeps each gate check independently readable.

Verified before recording: clippy clean under `-D warnings` with `--all-targets`, 1263 unit tests plus every integration suite green, and `--test parity` green with the goldens unchanged — this window edited no `framework/commands/*.md`, so `implement-basic.jsonl` needed no re-bless.

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
