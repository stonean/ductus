---
spec: 058-findings-route-at-discovery
last-run: 2026-11-03T14:56:19Z
reviewed-against: c5ad7370515d8e0d55b5fa787214053f570c4e35
diff-base: 1405eacc329f3b87691feb513ef2f7d9203db830
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 7
scope: 149
skipped-passes: []
reviewed-digest:
  data-model.md: cdcda743c37e5d22ed113d0f8e5cbbcdc3be428fb608a1c910eaec5c1bf24518
  scenarios/analysis-drift-judges-the-record-it-writes.md: a1a169a7b9b620f1a3d488dc5f93ff9189750cdd9f7df60663e9db787f94bc36
  scenarios/analyze-findings-match-decisions-by-host-judgment.md: 33a403bd242b5f8bf765948b4f69b16868799b3be8d9d93065e16a300307a652
  scenarios/analyze-state-drift-judges-the-record-it-writes.md: 1d15accf4d667b1364919a64de40e89a6ea1d674e76244a597c6662a85ecf66e
  scenarios/auto-records-disposition-tasks-without-pausing.md: 79217820badbfd130ba5e69a78927c3d434bcda01e3c66faec2be2b16dcf2c7e
  scenarios/findings-cost-in-proportion-to-risk.md: 931531a5193a8223c7273d1a9f2c3362c87116950726dcb3d3115f8ebdd113f8
  scenarios/only-unreadable-targets-retain-decisions.md: 76a12d565f6e1056f780906fed41de7d27e4a23c314232eddca0563026bcc3e1
blocking: false
dispositions:
  fixed: 18
  deferred: 0
  waived: 15
  undispositioned: 0
decisions:
  - key: "perf: validate-frontmatter reads each record file twice — `runtime/src/primitives/validate_frontmatter.rs`"
    outcome: waived
    reason: bounded to two small files per spec; the decisions list is read apart from the record by design
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "reuse: write_analysis::plain re-implements the shared bullet-marker stripping — `runtime/src/primitives/write_analysis.rs`"
    outcome: waived
    reason: "predates 058: moved from render_captured_plain, not introduced by it"
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "simplicity: render_list_entry takes a base indent every caller passes empty — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: predates 058 on render_waivers_at; no behavior depends on it
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "simplicity: describe and disposition_suffix fall back on a companion already validated — `runtime/src/primitives/process_decisions.rs`"
    outcome: waived
    reason: DecisionRef is a wire type; carrying the companion inside the outcome variant would change its schema
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "reuse: append_task.rs parses a task heading two ways; the slug guard's grammar predates 058 — `runtime/src/primitives/append_task.rs`"
    outcome: waived
    reason: predates 058, not introduced by it
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "convention: check-review-gate, check-unfolded-specs, dashboard, and prune-tasks return MissingFrontmatter and UnclosedFrontmatter without naming either under # Errors (predates 058) — `runtime/src/primitives/check_review_gate.rs`"
    outcome: waived
    reason: predates 058; a doc-only omission, and each variant's own message names the file and the defect
    decided-at: 2026-09-26T18:25:59Z
    decided-by: andy@stone.dev
  - key: "other: review's description and help row say it blocks done on MUST violations, while the gate also blocks an unwaived SHOULD — `framework/commands/review.md`"
    outcome: waived
    reason: true of the review record, whose blocking flag is MUST-only, and 023's AC25 pins the exact text
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: loose \"gets a disposition\" phrasing in 050, 047, 041, 058 and walker.rs — `specs/047-analyze-findings-durability/spec.md`"
    outcome: waived
    reason: none asserts that an undecided finding holds done, and each run still proposes a disposition for every finding
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: 020's data model calls should-violations an advisory severity count rather than a post-waiver one — `specs/020-code-review/data-model.md`"
    outcome: waived
    reason: "pre-existing and not false: the row names the SHOULD count, and §Waivers and the §Expiry rule state the post-waiver exclusion"
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: the analyze-state-drift scenario's Context says only --fix ends the loop, and its close omits the reopen-by-route exit — `specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md`"
    outcome: waived
    reason: the Context records the state when the scenario was found, and the close is loose, not false
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "reuse: the blank-line removal rule is written out four times, in inbox-guidance-refresh step 4, remove-legacy-constitutions-region step 7 and strip-inbox-guidance steps 5 and 6 — `framework/migrations/strip-inbox-guidance.md`"
    outcome: waived
    reason: each procedure is applied once per repository from one archive, and editing an applied procedure reaches no adopter, so the copies cannot drift in effect
    decided-at: 2026-10-31T01:16:01Z
    decided-by: andy@stone.dev
  - key: "edge: a `~~~` fence would hide a placeholder from the removal while iter_bullets, which knows only backtick fences, still counts it — `framework/migrations/strip-inbox-guidance.md`"
    outcome: waived
    reason: "contrived: no template ever put the placeholder in a fence"
    decided-at: 2026-10-31T01:16:01Z
    decided-by: andy@stone.dev
  - key: "edge: step 8's item count may treat nested or tab-separated bullets differently from iter_bullets — `framework/migrations/strip-inbox-guidance.md`"
    outcome: waived
    reason: it feeds only the report line, and nothing gates on it
    decided-at: 2026-10-31T01:16:01Z
    decided-by: andy@stone.dev
  - key: "quality: a deterministic, byte-exact rewrite stays prose for the host to walk although it meets §runtime-boundary's eligibility test — `framework/migrations/strip-inbox-guidance.md`"
    outcome: waived
    reason: "pipeline machinery: 027's registry contract defines prose procedures, every sibling entry is prose, and a real adopter bootstrap verified this one on three subjects"
    decided-at: 2026-10-31T01:16:01Z
    decided-by: andy@stone.dev
  - key: "other: AC35 names the former placeholder item in the singular while the procedure removes two forms — `specs/058-findings-route-at-discovery/spec.md`"
    outcome: waived
    reason: "the criterion stays true: each form is still removed only as its exact text"
    decided-at: 2026-10-31T01:16:01Z
    decided-by: andy@stone.dev
---

# Review — 058-findings-route-at-discovery

## Summary

0 MUST violations, 0 SHOULD violations, 0 low-confidence findings. 076 reopened 058, so this review follows the shared-window rule: 076's review (reviewed against `104cdf69`) read the window's shared code in depth, and nothing in 058's scope changed after it. The passes, run by one read-only agent on 2026-11-03, read 058's own artifacts as 076's sweeps rewrote them since 058's last review at `afb8a865`, judging each changed sentence against the code and references it describes. `examined: 7`: `spec.md`, `data-model.md`, the four changed scenarios, and `plan.md` (lines 1–459 in full, 460–581 by grep only); `tasks.md` and the two records were not read. The code those sentences describe was consulted in part to judge them (`decisions.rs`, `process_decisions.rs`, `write_review.rs`, `write_analysis.rs`, `check_review_gate.rs`, `schema/primitives.rs`, `validate_frontmatter.rs`, `append_task.rs`, and the framework references) and is not counted; 076's review covers it. No rule finding, and every ticked criterion still holds. 18 observations, each fixed in `c5ad7370` as the reviewer prescribed it and checked by diff against the prescriptions, with no re-read; two of them restore 076's AC18 and AC7. The 15 stored decisions were each re-judged against the tree and all still hold, so they are carried with their text and reason unchanged.

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

- bug: the analyze-state-drift scenario's Context still named the disposition discarded and a route, as did its analysis-drift sibling — `specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md` — **fixed**
- convention: the spec called a write that resolves a finding a deferral, which by 076's terms is outstanding work — `specs/058-findings-route-at-discovery/spec.md` — **fixed**
- bug: the data model's malformed rule named target or reason as the companion, though a deferral may name only a work item, and omitted forbidden fields — `specs/058-findings-route-at-discovery/data-model.md` — **fixed**
- bug: the data model's key row omitted the violation key rule {rule-id} at {file} that a review record stores since 076 — `specs/058-findings-route-at-discovery/data-model.md` — **fixed**
- bug: the plan said a new decision is one whose key and outcome are not stored, though a deferral naming other work is new too — `specs/058-findings-route-at-discovery/plan.md` — **fixed**
- bug: the plan's gate checks 3 and 4 said any unwaived violation and any hard-fail or blocking finding blocks, though a released one does not — `specs/058-findings-route-at-discovery/plan.md` — **fixed**
- other: the data model's undispositioned formula named the record's gating fields, which since 076 are net of released findings — `specs/058-findings-route-at-discovery/data-model.md` — **fixed**
- convention: the undispositioned-findings block's message column omitted the held-by-deferral suffix — `specs/058-findings-route-at-discovery/data-model.md` — **fixed**
- convention: decided-by's row said it is required for observations only, not for a newly deferred violation — `specs/058-findings-route-at-discovery/data-model.md` — **fixed**
- convention: the 076 signpost restated the release condition without still to do, and without pointing at §Deferrals — `specs/058-findings-route-at-discovery/spec.md` — **fixed**
- convention: the signpost's stored outcomes, which still read the old names, read as if the body kept them — `specs/058-findings-route-at-discovery/spec.md` — **fixed**
- other: the waiver list read as complete before the next sentence added review violations — `specs/058-findings-route-at-discovery/spec.md` — **fixed**
- other: a garden-path clause on a scenario deferred to on the spec in hand — `specs/058-findings-route-at-discovery/spec.md` — **fixed**
- convention: the process-decisions result table described the shape before 076's lapsed list, task and work-item — `specs/058-findings-route-at-discovery/data-model.md` — **fixed**
- convention: the deferred and waived rows duplicated 076's definitions — `specs/058-findings-route-at-discovery/data-model.md` — **fixed**
- convention: the data model pointed at framework/commands/review.md for sections now in framework/reference/review.md, on a line apart from its anchors — `specs/058-findings-route-at-discovery/data-model.md` — **fixed**
- convention: decisions.rs's module doc pointed at the same moved section — `runtime/src/primitives/decisions.rs` — **fixed**
- other: the analyze reference called a write that resolves a finding a deferral — `framework/reference/analyze.md` — **fixed**
- perf: validate-frontmatter reads each record file twice — `runtime/src/primitives/validate_frontmatter.rs` — **waived**: bounded to two small files per spec; the decisions list is read apart from the record by design
- reuse: write_analysis::plain re-implements the shared bullet-marker stripping — `runtime/src/primitives/write_analysis.rs` — **waived**: predates 058: moved from render_captured_plain, not introduced by it
- simplicity: render_list_entry takes a base indent every caller passes empty — `runtime/src/primitives/write_review.rs` — **waived**: predates 058 on render_waivers_at; no behavior depends on it
- simplicity: describe and disposition_suffix fall back on a companion already validated — `runtime/src/primitives/process_decisions.rs` — **waived**: DecisionRef is a wire type; carrying the companion inside the outcome variant would change its schema
- reuse: append_task.rs parses a task heading two ways; the slug guard's grammar predates 058 — `runtime/src/primitives/append_task.rs` — **waived**: predates 058, not introduced by it
- convention: check-review-gate, check-unfolded-specs, dashboard, and prune-tasks return MissingFrontmatter and UnclosedFrontmatter without naming either under # Errors (predates 058) — `runtime/src/primitives/check_review_gate.rs` — **waived**: predates 058; a doc-only omission, and each variant's own message names the file and the defect
- other: review's description and help row say it blocks done on MUST violations, while the gate also blocks an unwaived SHOULD — `framework/commands/review.md` — **waived**: true of the review record, whose blocking flag is MUST-only, and 023's AC25 pins the exact text
- other: loose "gets a disposition" phrasing in 050, 047, 041, 058 and walker.rs — `specs/047-analyze-findings-durability/spec.md` — **waived**: none asserts that an undecided finding holds done, and each run still proposes a disposition for every finding
- other: 020's data model calls should-violations an advisory severity count rather than a post-waiver one — `specs/020-code-review/data-model.md` — **waived**: pre-existing and not false: the row names the SHOULD count, and §Waivers and the §Expiry rule state the post-waiver exclusion
- other: the analyze-state-drift scenario's Context says only --fix ends the loop, and its close omits the reopen-by-route exit — `specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md` — **waived**: the Context records the state when the scenario was found, and the close is loose, not false
- reuse: the blank-line removal rule is written out four times, in inbox-guidance-refresh step 4, remove-legacy-constitutions-region step 7 and strip-inbox-guidance steps 5 and 6 — `framework/migrations/strip-inbox-guidance.md` — **waived**: each procedure is applied once per repository from one archive, and editing an applied procedure reaches no adopter, so the copies cannot drift in effect
- edge: a `~~~` fence would hide a placeholder from the removal while iter_bullets, which knows only backtick fences, still counts it — `framework/migrations/strip-inbox-guidance.md` — **waived**: contrived: no template ever put the placeholder in a fence
- edge: step 8's item count may treat nested or tab-separated bullets differently from iter_bullets — `framework/migrations/strip-inbox-guidance.md` — **waived**: it feeds only the report line, and nothing gates on it
- quality: a deterministic, byte-exact rewrite stays prose for the host to walk although it meets §runtime-boundary's eligibility test — `framework/migrations/strip-inbox-guidance.md` — **waived**: pipeline machinery: 027's registry contract defines prose procedures, every sibling entry is prose, and a real adopter bootstrap verified this one on three subjects
- other: AC35 names the former placeholder item in the singular while the procedure removes two forms — `specs/058-findings-route-at-discovery/spec.md` — **waived**: the criterion stays true: each form is still removed only as its exact text

## Skipped passes

*None.*

## Unexamined governance

*None.*
