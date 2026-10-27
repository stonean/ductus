---
spec: 058-findings-route-at-discovery
last-run: 2026-10-27T06:47:55Z
reviewed-against: 98a2eae0e909d19a50b246d127006f8440535e75
diff-base: 57f82b22e4ca9d7d78c9d481f82f13e72f28b092
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 45
scope: 81
skipped-passes: []
reviewed-digest:
  data-model.md: c7e234cb3bb24c79adb55d9b627fe41f2f346388a8644637c34cacdfa20cdb2c
  scenarios/analysis-drift-judges-the-record-it-writes.md: 9181d8ef6a4ad54cf2847a1fdac96d42ab509b598079c4901a17471f5b8d6b56
  scenarios/analyze-findings-match-decisions-by-host-judgment.md: d14bea60a4dfdba81edc5180b77367629154699b6808fda0739ab8a902776660
  scenarios/analyze-state-drift-judges-the-record-it-writes.md: 97833e416068dabe61138bc0e644535f9c2bbfdb065a7cce89d0b7a22180a607
  scenarios/auto-records-disposition-tasks-without-pausing.md: 7d09684e1a17de3c9aee551cada7245b9e1f7b640f643a8c35b7d96166190da0
  scenarios/findings-cost-in-proportion-to-risk.md: 7ff3dc556498c6147e5e27d651bb181b04d2263e7ddad7ed983dec039329cbba
  scenarios/only-unreadable-targets-retain-decisions.md: 76a12d565f6e1056f780906fed41de7d27e4a23c314232eddca0563026bcc3e1
blocking: false
dispositions:
  fixed: 24
  routed: 0
  discarded: 6
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
  - key: "other: review's description and help row say it blocks done on MUST violations, while the gate also blocks an unwaived SHOULD — `framework/commands/review.md`"
    outcome: discarded
    reason: true of the review record, whose blocking flag is MUST-only, and 023's AC25 pins the exact text
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: loose \"gets a disposition\" phrasing in 050, 047, 041, 058 and walker.rs — `specs/047-analyze-findings-durability/spec.md`"
    outcome: discarded
    reason: none asserts that an undecided finding holds done, and each run still proposes a disposition for every finding
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: the implement reference's \"non-blocking\" counts a SHOULD as blocking where review vocabulary does not — `framework/reference/implement.md`"
    outcome: discarded
    reason: the inline definition names exactly which findings it covers, and non-blocking is the scenario's own term
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: 020's data model calls should-violations an advisory severity count rather than a post-waiver one — `specs/020-code-review/data-model.md`"
    outcome: discarded
    reason: "pre-existing and not false: the row names the SHOULD count, and §Waivers and the §Expiry rule state the post-waiver exclusion"
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: the analyze-state-drift scenario's Context says only --fix ends the loop, and its close omits the reopen-by-route exit — `specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md`"
    outcome: discarded
    reason: the Context records the state when the scenario was found, and the close is loose, not false
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "edge: an expired waiver on a low-confidence finding re-counts toward low-confidence, not its severity's count — `framework/reference/review.md`"
    outcome: discarded
    reason: "contrived: waiving a finding that counts toward no gate has no effect worth documenting, and step 1's exclusion stays true"
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
---

# Review — 058-findings-route-at-discovery

## Summary

Shared-window review of 058's `findings-cost-in-proportion-to-risk` change, `--since 57f82b22` (item 3's close): e38f5c1f, the change, with an independent review of its uncommitted form folded in; 84bb4aa1 and 6cf2bb6e, fix rounds 2 and 3, each re-read by an independent agent, round 3's re-read finding no MUST or SHOULD; and 98a2eae0, round 4, a doc comment, a comma and a discharged inbox item, not re-read as a prose and doc-comment round. Rules: the eleven files `discover-rule-files` selects. No rule fires: the gate reads counts from local records, the wire enum and the `blocked-by` value are unchanged, and each gating term has a test that fails without it (mutations M1-M5 killed). 0 MUST, 0 SHOULD; 30 observations, 24 fixed and 6 discarded. Examined 45: the hand-written files the window changed, read as diffs and, for `disposition_block` and its tests, the 058 scenarios and both owner scenarios, whole. The window's 11 generated copies under `.claude/`, `.pi/` and `.ductus/reference/` were compared byte for byte with their sources, not read. The rest of scope went unread: the plan Affected Files the window left unchanged, which earlier records cover, and the five review records this run writes. Stored decisions from earlier windows are retained, their sources lying outside this window. Its own artifacts were read in the window: spec, plan, data model, tasks and the four scenarios the change edited; the deliverable is the gate, drift tier, disposition-task rule and AGENTS.md entry above.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- bug: write-analysis's summary said a hard-fail or blocking finding holds done until it is fixed or routed — `runtime/src/primitives/write_analysis.rs` — **fixed**
- other: the constitution's Finding dispositions lead read a review observation as a gating finding — `framework/constitution.md` — **fixed**
- other: "only a gating finding holds done" was false for an out-of-spec implement finding's disposition task — `framework/constitution.md` — **fixed**
- other: 047's §Constitution amendment signpost and AC7 said every analyze finding must get a disposition before done — `specs/047-analyze-findings-durability/spec.md` — **fixed**
- other: the gate's check-11 doc comments still described it as reading the dispositions maps — `runtime/src/primitives/check_review_gate.rs` — **fixed**
- edge: no --fix trigger reverts a done spec whose own analysis is blocking once disposition drift stopped reverting — `framework/reference/analyze.md` — **fixed**
- test: the gate's MUST and analysis terms were reachable only from a hand-edited record and no test exercised them — `runtime/src/primitives/check_review_gate.rs` — **fixed**
- other: 058's first Resolved Question cross-referenced a section that now says the opposite before its supersession note — `specs/058-findings-route-at-discovery/spec.md` — **fixed**
- convention: --waive was documented for a MUST violation only, while the gate's guidance directs it at a SHOULD — `framework/commands/review.md` — **fixed**
- other: review's description and help row say it blocks done on MUST violations, while the gate also blocks an unwaived SHOULD — `framework/commands/review.md` — **discarded**: true of the review record, whose blocking flag is MUST-only, and 023's AC25 pins the exact text
- other: the disposition-drift reference said the report names which findings nobody decided — `framework/reference/analyze.md` — **fixed**
- other: loose "gets a disposition" phrasing in 050, 047, 041, 058 and walker.rs — `specs/047-analyze-findings-durability/spec.md` — **discarded**: none asserts that an undecided finding holds done, and each run still proposes a disposition for every finding
- other: 058's spec said an undecided finding's decision persists — `specs/058-findings-route-at-discovery/spec.md` — **fixed**
- other: the implement reference's "non-blocking" counts a SHOULD as blocking where review vocabulary does not — `framework/reference/implement.md` — **discarded**: the inline definition names exactly which findings it covers, and non-blocking is the scenario's own term
- other: 058's analyze section said the findings disposition drift counts do not hold done, though analysis.md's count includes gating ones — `specs/058-findings-route-at-discovery/spec.md` — **fixed**
- other: docs/analyze.md gave the same false reason for --fix not reverting on disposition drift — `docs/analyze.md` — **fixed**
- other: the scenario's first Resolved Question said the gate reads only should-violations — `specs/058-findings-route-at-discovery/scenarios/findings-cost-in-proportion-to-risk.md` — **fixed**
- other: the review reference's §Waivers said a waiver drops a finding from must-violations only — `framework/reference/review.md` — **fixed**
- other: "only an unwaived SHOULD reaches this check" was stated flat, though a hand-edited record reaches it with a MUST — `framework/reference/implement.md` — **fixed**
- other: the drift report sentence understated analysis.md's undispositioned count — `framework/reference/analyze.md` — **fixed**
- other: the analyze-state drift reference bullet said routing anywhere clears a blocking record — `framework/reference/analyze.md` — **fixed**
- other: 020's data model calls should-violations an advisory severity count rather than a post-waiver one — `specs/020-code-review/data-model.md` — **discarded**: pre-existing and not false: the row names the SHOULD count, and §Waivers and the §Expiry rule state the post-waiver exclusion
- other: the analyze-state-drift scenario's Context says only --fix ends the loop, and its close omits the reopen-by-route exit — `specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md` — **discarded**: the Context records the state when the scenario was found, and the close is loose, not false
- other: §Per-run waiver processing said an expired waiver re-counts toward must-violations and flips blocking, false for a SHOULD — `framework/reference/review.md` — **fixed**
- other: docs/analyze.md's "every other gating count blocks earlier" was the last unscoped sibling — `docs/analyze.md` — **fixed**
- other: 058's data model read as saying analysis.md's undecided findings do not hold done — `specs/058-findings-route-at-discovery/data-model.md` — **fixed**
- other: docs/analyze.md's scoped clause parsed as "the runtime writes every other gating count" without its commas — `docs/analyze.md` — **fixed**
- other: ReviewSeverity::is_blocking's doc comment said a SHOULD never blocks done, and called it the bucketing predicate — `runtime/src/schema/severity.rs` — **fixed**
- other: the inbox still held "check-review-gate never reads should-violations", which the gate change discharged — `specs/inbox.md` — **fixed**
- edge: an expired waiver on a low-confidence finding re-counts toward low-confidence, not its severity's count — `framework/reference/review.md` — **discarded**: contrived: waiving a finding that counts toward no gate has no effect worth documenting, and step 1's exclusion stays true

## Skipped passes

*None.*

## Unexamined governance

*None.*
