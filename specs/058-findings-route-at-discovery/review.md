---
spec: 058-findings-route-at-discovery
last-run: 2026-10-16T19:36:00Z
reviewed-against: ec72c234476d97f7057f68d83c6e92f9e88eb94b
diff-base: e8ba124ecaba41a153b486a8a68137a461c4805d
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 1
scope: 37
skipped-passes: []
reviewed-digest:
  data-model.md: eb5f2099c81634ab8f9dc5c7ff3073d2f6323dccf95c39a1cf1ec410d3b9b237
  scenarios/analysis-drift-judges-the-record-it-writes.md: 33f0948bc3e3b1de7f92abbd822ebb1de03ad08f2aa24c1509236ff7bd0baf7a
  scenarios/analyze-findings-match-decisions-by-host-judgment.md: d14bea60a4dfdba81edc5180b77367629154699b6808fda0739ab8a902776660
  scenarios/analyze-state-drift-judges-the-record-it-writes.md: e975a37075b739bcd087c8683f166b6955ce9bcc42c28f64e79bf9f49003f803
  scenarios/auto-records-disposition-tasks-without-pausing.md: 411e98d379ee2cb76ec7bef380e566ead077d5fa73bb1ecda34cb945ccda652e
  scenarios/only-unreadable-targets-retain-decisions.md: 76a12d565f6e1056f780906fed41de7d27e4a23c314232eddca0563026bcc3e1
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
decisions:
  - key: "perf: validate-frontmatter reads each record file twice — `runtime/src/primitives/validate_frontmatter.rs`"
    outcome: discarded
    reason: bounded to two small files per spec; the decisions list is read apart from the record by design
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "convention: a duplicate-key decision is pruned when its key expires or is decided again — `runtime/src/primitives/decisions.rs`"
    outcome: discarded
    reason: mirrors the waiver list's expiry, and process-decisions still reports each duplicate before it goes
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "convention: an all-empty decision entry is dropped on re-render — `runtime/src/primitives/decisions.rs`"
    outcome: discarded
    reason: an entry with no fields carries no decision to lose; render_waivers_at behaves the same
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "reuse: write_analysis::plain re-implements the shared bullet-marker stripping — `runtime/src/primitives/write_analysis.rs`"
    outcome: discarded
    reason: "predates 058: moved from render_captured_plain, not introduced by it"
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "simplicity: render_list_entry takes a base indent every caller passes empty — `runtime/src/primitives/write_review.rs`"
    outcome: discarded
    reason: predates 058 on render_waivers_at; no behavior depends on it
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "simplicity: describe and disposition_suffix fall back on a companion already validated — `runtime/src/primitives/process_decisions.rs`"
    outcome: discarded
    reason: DecisionRef is a wire type; carrying the companion inside the outcome variant would change its schema
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "perf: check-artifacts' history revwalk runs on the async worker, contrary to server.rs's module doc (predates 058) — `runtime/src/mcp/server.rs`"
    outcome: discarded
    reason: predates 058 (ccdd3ac6), not introduced by it
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "reuse: append_task.rs parses a task heading two ways; the slug guard's grammar predates 058 — `runtime/src/primitives/append_task.rs`"
    outcome: discarded
    reason: predates 058, not introduced by it
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "convention: check-review-gate, check-unfolded-specs, dashboard, and prune-tasks return MissingFrontmatter and UnclosedFrontmatter without naming either under # Errors (predates 058) — `runtime/src/primitives/check_review_gate.rs`"
    outcome: discarded
    reason: predates 058; a doc-only omission, and each variant's own message names the file and the defect
    decided-at: 2026-09-26T18:25:59Z
    decided-by: andy@stone.dev
---

# Review — 058-findings-route-at-discovery

## Summary

Scoped re-review, reopened for a cross-spec impact 067 task 7 owed and missed (50c6f693): the retention scenario's two lists of the excluded-by-construction class gain never-existed. examined: 1: only-unreadable-targets-retain-decisions.md in full. It also clears the release gate's Family 19 finding on this spec: the 022 split's pointer rewrite in the same scenario (d852b627) had not been reviewed, and it reads correctly, since 066's data model is where the class table now lives. The nine stored discards are retained. The five passes ran over the files counted as examined, against all eleven rule files and AGENTS.md, and found no finding and no observation. Not read and not counted: every other scope entry - the plan's Affected Files, unchanged in this window, and the files other specs' amendments and 067's tasks 10-11 changed after the base (including runtime/src/primitives/derive_dependencies.rs unless named above), which are the subject of their own reviews.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
