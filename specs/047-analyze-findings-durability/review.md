---
spec: 047-analyze-findings-durability
last-run: 2026-10-27T04:19:28Z
reviewed-against: 88768fe67738169b56f4f60554eb67b383858398
diff-base: 865e97ae554c65b42bdf842d2a291e69981a9ffe
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 44
scope: 58
skipped-passes: []
reviewed-digest:
  scenarios/analyze-record-freshness.md: 62dd9d6588815fc3eefc88cc768f6196e6ea22db2ef29d13d2c8499fc598e3f6
  scenarios/analyze-run-durability.md: 3bfd4bd619bac879b90e7e695d68a73b2788aa0e0e7a65d524ec4fd5e9daf481
blocking: false
dispositions:
  fixed: 13
  routed: 0
  discarded: 10
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
---

# Review — 047-analyze-findings-durability

## Summary

Five passes over the window 865e97ae..88768fe6 — 047's diff base, the parent of e3e54b07, which reopened it to annotate AC14 for 041's `a-plan-edit-does-not-reopen-a-done-spec`: on a `done` spec the analyze comparison now ignores an in-place `plan.md` or `tasks.md` edit. The window is shared with 041 and 063 and was reviewed once in depth; this record carries that review, read from the same passes, with identical observations and dispositions. An independent reviewer ran security, reuse, quality, efficiency and simplicity over e3e54b07 and 3ce51eb9 against api-, concurrency-, observability-, performance-, reliability- and security-backend plus configuration- and quality-cross: 0 MUST, 0 SHOULD against a loaded rule, and 23 observations across three rounds — 13 fixed, 10 discarded with reasons. For 047 the rounds corrected the AC14 annotation and `analyze-record-freshness`'s "Two exclusions" paragraph to state the rule with its condition, and to keep a deleted plan counting. Round 1 (fbf2737c) and round 2 (2f1c33c8) were each re-read independently; round 2's re-read found no MUST or SHOULD. Examined: 44 of the 52 window files, every changed file but seven generated copies diffed against their sources and 88768fe6's 058 scenario, outside this review. `scope` counts 58 entries, each one path: the 52 window files, 047's three plan Affected Files rows untouched since the base and not read, and the three review records this window writes for 041, 047 and 063, which are the passes' output rather than their subject.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- other: 041's plan, data model and two scenario edge cases still described reopen-required's design-record trigger — `specs/041-task-pruning/plan.md` — **fixed**
- other: 063's Resolved Question ended on /prune reopening a spec when a moved decision edits the design record — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **fixed**
- other: restatements of the plan rule dropped its condition, adding no unchecked task, or said the comparison leaves the files out — `framework/reference/review.md` — **fixed**
- bug: a deleted plan.md or tasks.md on a done spec read current, though analyze reports a missing one as blocking — `runtime/src/primitives/analyze_subjects.rs` — **fixed**
- other: the plan.md and tasks.md filename literals are repeated with no shared constant (CFG-CONST-001 note) — `runtime/src/primitives/analyze_subjects.rs` — **discarded**: a pre-existing pattern across 14 and 15 modules with no shared constant; introducing one is its own change
- other: the review reference said a plan correction leaves the analyze row current, true only of a record already current — `framework/reference/review.md` — **fixed**
- other: /amend's reconcile pass offers to reopen a done spec for any uncommitted tasks.md change — `framework/commands/amend.md` — **discarded**: it reopens only on the operator's confirmation, matching 023's recorded decision that the operator is the discriminator; 014 owns the delta, and 041's scenario records the boundary
- other: 072's out-of-scope note says changing the plan rule needs its own spec — `specs/072-governing-documents-stay-within-one-read/spec.md` — **discarded**: the note's subject, why 072 did not take the change, still holds
- other: 023's living-specs and 054 call every factual correction meaningful — `specs/023-govern-refinement/scenarios/living-specs.md` — **discarded**: both concern spec.md bodies or audit artifacts, never a plan or task list
- other: §spec-lifecycle resolves the plan rule against the shared test only by sentence order — `framework/constitution.md` — **discarded**: coherent as read, the rule sitting between the three cases and Anything else, and the section is held at its byte ceiling
- other: 013's and 016's plans carried further stale constitution claims outside the routed list — `specs/013-text-first-artifacts/plan.md` — **fixed**
- other: the replay proof is true by construction for the new binary — `specs/041-task-pruning/tasks.md` — **discarded**: its proof is the old binary's half, which reopened all 18; the new half confirms the change landed
- other: AGENTS.md's two-disposals entry contradicted the sentence appended to it — `AGENTS.md` — **fixed**
- other: the analyze reference's trim advice read as self-contradictory in one sentence — `framework/reference/analyze.md` — **fixed**
- test: a data-model.md trim at done was unpinned, so widening the plan-trim exemption to it survived — `runtime/src/primitives/check_artifact_size.rs` — **fixed**
- perf: analyze_freshness re-reads spec.md its callers already parsed — `runtime/src/primitives/analyze_subjects.rs` — **discarded**: one small file read per call, negligible
- other: 016's plan item 2 still described a link to 008's data model and an Edge Case the round deleted — `specs/016-cross-cutting-rules/plan.md` — **fixed**
- other: without_plan_edits' doc comment said the commands that read tasks.md reopen a spec for an added task — `runtime/src/primitives/analyze_subjects.rs` — **fixed**
- other: 063's Resolved Question says /prune reopens a spec only when it adds owed work — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **discarded**: loose, not false, in that Resolved Question: prune reopens when the diff adds owed work
- other: AGENTS.md's disposals entry left a task-list fix that adds a task uncovered — `AGENTS.md` — **fixed**
- other: 016's plan item 4 named cross-references §rules does not carry — `specs/016-cross-cutting-rules/plan.md` — **fixed**
- edge: an unreadable plan.md or tasks.md on a done spec is dropped like an edit in place and reads current — `runtime/src/primitives/analyze_subjects.rs` — **discarded**: contrived: no command writes an unreadable plan or task list
- edge: an in-place plan or tasks edit that could produce a blocking analyze finding is still ignored on a done spec — `runtime/src/primitives/analyze_subjects.rs` — **discarded**: the chosen design: only a deletion is kept, and the one consumer is the advisory review row

## Skipped passes

*None.*

## Unexamined governance

*None.*
