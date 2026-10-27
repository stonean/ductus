---
spec: 047-analyze-findings-durability
last-run: 2026-10-27T06:47:55Z
reviewed-against: 98a2eae0e909d19a50b246d127006f8440535e75
diff-base: 57f82b22e4ca9d7d78c9d481f82f13e72f28b092
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 45
scope: 62
skipped-passes: []
reviewed-digest:
  scenarios/analyze-record-freshness.md: 62dd9d6588815fc3eefc88cc768f6196e6ea22db2ef29d13d2c8499fc598e3f6
  scenarios/analyze-run-durability.md: 3bfd4bd619bac879b90e7e695d68a73b2788aa0e0e7a65d524ec4fd5e9daf481
blocking: false
dispositions:
  fixed: 24
  routed: 0
  discarded: 6
  undispositioned: 0
decisions:
  - key: "other: the plan.md and tasks.md filename literals are repeated with no shared constant (CFG-CONST-001 note) — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: discarded
    reason: a pre-existing pattern across 14 and 15 modules with no shared constant; introducing one is its own change
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: /amend's reconcile pass offers to reopen a done spec for any uncommitted tasks.md change — `framework/commands/amend.md`"
    outcome: discarded
    reason: it reopens only on the operator's confirmation, matching 023's recorded decision that the operator is the discriminator; 014 owns the delta, and 041's scenario records the boundary
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: 072's out-of-scope note says changing the plan rule needs its own spec — `specs/072-governing-documents-stay-within-one-read/spec.md`"
    outcome: discarded
    reason: the note's subject, why 072 did not take the change, still holds
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: 023's living-specs and 054 call every factual correction meaningful — `specs/023-govern-refinement/scenarios/living-specs.md`"
    outcome: discarded
    reason: both concern spec.md bodies or audit artifacts, never a plan or task list
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: §spec-lifecycle resolves the plan rule against the shared test only by sentence order — `framework/constitution.md`"
    outcome: discarded
    reason: coherent as read, the rule sitting between the three cases and Anything else, and the section is held at its byte ceiling
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: the replay proof is true by construction for the new binary — `specs/041-task-pruning/tasks.md`"
    outcome: discarded
    reason: its proof is the old binary's half, which reopened all 18; the new half confirms the change landed
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "perf: analyze_freshness re-reads spec.md its callers already parsed — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: discarded
    reason: one small file read per call, negligible
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "other: 063's Resolved Question says /prune reopens a spec only when it adds owed work — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`"
    outcome: discarded
    reason: "loose, not false, in that Resolved Question: prune reopens when the diff adds owed work"
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "edge: an unreadable plan.md or tasks.md on a done spec is dropped like an edit in place and reads current — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: discarded
    reason: "contrived: no command writes an unreadable plan or task list"
    decided-at: 2026-10-27T04:19:28Z
    decided-by: andy@stone.dev
  - key: "edge: an in-place plan or tasks edit that could produce a blocking analyze finding is still ignored on a done spec — `runtime/src/primitives/analyze_subjects.rs`"
    outcome: discarded
    reason: "the chosen design: only a deletion is kept, and the one consumer is the advisory review row"
    decided-at: 2026-10-27T04:19:28Z
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

# Review — 047-analyze-findings-durability

## Summary

Shared-window review of 058's `findings-cost-in-proportion-to-risk` change, `--since 57f82b22` (item 3's close): e38f5c1f, the change, with an independent review of its uncommitted form folded in; 84bb4aa1 and 6cf2bb6e, fix rounds 2 and 3, each re-read by an independent agent, round 3's re-read finding no MUST or SHOULD; and 98a2eae0, round 4, a doc comment, a comma and a discharged inbox item, not re-read as a prose and doc-comment round. Rules: the eleven files `discover-rule-files` selects. No rule fires: the gate reads counts from local records, the wire enum and the `blocked-by` value are unchanged, and each gating term has a test that fails without it (mutations M1-M5 killed). 0 MUST, 0 SHOULD; 30 observations, 24 fixed and 6 discarded. Examined 45: the hand-written files the window changed, read as diffs and, for `disposition_block` and its tests, the 058 scenarios and both owner scenarios, whole. The window's 11 generated copies under `.claude/`, `.pi/` and `.ductus/reference/` were compared byte for byte with their sources, not read. The rest of scope went unread: the plan Affected Files the window left unchanged, which earlier records cover, and the five review records this run writes. Stored decisions from earlier windows are retained, their sources lying outside this window. Its own changes in the window are the 058 signposts and AC7's annotation, read whole; the spec was reopened for them.

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
