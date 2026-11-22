---
spec: 076-waived-and-deferred-findings-are-distinct-dispositions
last-run: 2026-11-22T23:06:10Z
reviewed-against: c453e9ef7351f70b45c6b41d5ff73e5a280463ad
diff-base: 8ef7d0e588796fce044711797399b19e01f35efe
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 3
scope: 4
unread:
  - framework/constitution.md
excluded: 37
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 313bff884620d9e3ad7d04077712c212bfc69f8b50e2fa999f4eb5f259775e32
criteria-judged: 18
criteria-met: 18
criteria-verdicts:
  - label: AC1
    digest: 5b0d25cb110335086cc4784c987ce86674b0e97ad756124892b1861dfdb1ce82
  - label: AC9
    digest: 7a55b1ae2df0a4d8260b1693054abc39e8cd286d2b5bdb9970b76bc60bdaf6ec
  - label: AC2
    digest: 3c580e3c4cd80c3de68eb57ef71080f0de642a2361f43226eb19e77bb16bbbc4
  - label: AC3
    digest: 6d47854237c4643f18abc902dd2ece9f6773829383757901024a4c9f25e86ba8
  - label: AC4
    digest: 22ed745baec679cd29935784c4ca02e049263101a07a1a15ee09fcd9a07e4cae
  - label: AC10
    digest: 5f144542d1ab9eb1b4d71bb7c3b5a2549e62b98cef8e9c3cdb163eed2d8ce0ec
  - label: AC11
    digest: 8a19bebc111b4efd50141681d758c2e8b57620360bf7188a682346d21ea286ba
  - label: AC12
    digest: 7cdb00dcdfd7724bada879aa5d8e306b924b5a5b03f39d5cd7b537b180e72165
  - label: AC13
    digest: ff8a2d4050b126fb395b476a6d60be129854eb0718b1aade2cc68a4f05d9219d
  - label: AC5
    digest: d2ac455c39a7a377302ec6f00390fe13a572a9fcc17a89196b6a3e80453949e3
  - label: AC6
    digest: 6b454e97d10b1abe9d2797bd400addf84c29418ca2c725c95d1bcc56d15b6783
  - label: AC7
    digest: 6315cc3418f380544b7092ff34707e7ef42dee00e803110653937b7eef96f436
  - label: AC14
    digest: d59f73e1b1dd2d121b3090e90117ea7baa866d84a42654dee3ac1e2df4db0b22
  - label: AC15
    digest: c36dc50bf65a8e8326d2557f9ceff5070bb9996fb2b8df715c9102691ad5f30e
  - label: AC16
    digest: 13325a47147563efcc31e9074d311d7c73ad652d0a2179284291d8e11270d37e
  - label: AC17
    digest: 8e4b6c5197ae60b94ff14ce3c52e0e7e01a95034861279004e592e6d8940a9f2
  - label: AC18
    digest: 87e9198d214224ab9bfad4506110b271404e58778ac87ac434f2f18b9ee6c014
  - label: AC8
    digest: b6391ef0c0915c830302cfdb0b4302424320134890923e01e115e38447b15fae
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: "bug: the dedup keeps the higher-ranked duplicate's disposition, so a deferral only a dropped duplicate carried would be lost — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: "--defer names a rule ID and the dedup collapses only findings sharing that rule and file, so duplicates carry the same deferral; no caller supplies differing ones"
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "perf: each finding's existence check re-parses the stored list and re-reads a named tasks.md — `runtime/src/primitives/decisions.rs`"
    outcome: waived
    reason: bounded by one run's findings; the lists and task files are a few entries
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "simplicity: Buckets is a borrowed copy of Sorted, and the two writers' stored_decisions loops are alike — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: a refactor with no behavior change; each loop names its own argument paths
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "bug: releases compares a target's feature to the feature argument as raw strings, so 001-x/ would release its own task — `runtime/src/primitives/decisions.rs`"
    outcome: waived
    reason: "contrived: every caller passes the canonical name resolve-feature returns"
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: a deferral's target kind is not checked, so README.md counts as pipeline work — `runtime/src/primitives/decisions.rs`"
    outcome: waived
    reason: 076's data model names the target kinds and no refusal for another; a README.md target lies under no spec, releases nothing, and lapses the next run its finding fires
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "bug: a task with no checkboxes counts as complete, so a deferral to it lapses at once — `runtime/src/primitives/decisions.rs`"
    outcome: waived
    reason: the data model's completion row read literally; 0 of 957 tasks read-tasks parses across this corpus's 70 tasks.md files on 2026-11-03 have neither a subtask nor a done-when checkbox, and the failure re-asks or blocks rather than releasing
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "simplicity: Work::External is handled in the same arm as Pending — `runtime/src/primitives/process_decisions.rs`"
    outcome: waived
    reason: the variant documents why a work-item-only deferral never lapses; no behavior hangs on merging them
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: no gate-level test seeds a pre-076 routed/discarded map — `runtime/src/primitives/check_review_gate.rs`"
    outcome: waived
    reason: the gate reads the map through Dispositions, whose alias test covers parsing
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: the gate names a stored gating deferral whose task has since been checked without saying it may have lapsed — `runtime/src/primitives/check_review_gate.rs`"
    outcome: waived
    reason: the gate reports what the record marks gating; lapse is judged by process-decisions when the next review or analyze run reads the record, and the gate's guidance sends the operator to re-run
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: the gate reads a record's release as last written, so another spec's work completed since the last run leaves its finding released until a run re-judges it — `runtime/src/primitives/check_review_gate.rs`"
    outcome: waived
    reason: "the gate reports what the record states; release and lapse are judged when a review or analyze run writes the record, by design (076's data model: derived on every run, never stored), and the next run re-judges it"
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: a work-item ID accepts vertical tab and form feed — `runtime/src/primitives/decisions.rs`"
    outcome: waived
    reason: "contrived: no tracker issues an ID holding either; the record escapes both and every report flattens them, so neither can break the record or a report line"
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: a hand-edited deferral naming a task in a nested tasks.md is judged by its spec's status — `runtime/src/primitives/decisions.rs`"
    outcome: waived
    reason: only a hand-edited record can reach it; the writers refuse that target
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: a stored target with .. lapses as complete rather than reading as unreadable — `runtime/src/primitives/decisions.rs`"
    outcome: waived
    reason: it names no feature directory under the spec root, so it is judged as lying outside every spec, as a rule file is, and nothing is opened; the writers refuse a traversal
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: the gate's outstanding-findings guidance lists this spec's own task and an external work item alone as deferrals that release nothing, omitting a rule file and complete or unreadable work — `runtime/src/primitives/check_review_gate.rs`"
    outcome: waived
    reason: the guidance is true as written and names the cases an operator reaches for; the full release condition is stated once, in the review reference's §Deferrals, which the gate's references point at
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: 076's plan Affected Files omits docs/rules.md and the 058, 066, 020 and 050 scenarios the sweep edited — `specs/076-waived-and-deferred-findings-are-distinct-dispositions/plan.md`"
    outcome: waived
    reason: the review scope unites Affected Files with every file modified since the diff base, which holds each of them, and the plan's Overview names the by-meaning sweep
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: 076's plan does not state the same-work match rule or the expired-decisions refusal — `specs/076-waived-and-deferred-findings-are-distinct-dispositions/plan.md`"
    outcome: waived
    reason: the plan's Data Model section points at the data model for the writer refusals, and the data model states both
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "security: process-waivers' duplicate, retained and expired notices print a stored waiver's rule, file and reason as written — `runtime/src/primitives/process_waivers.rs`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "13"
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
---

# Review — 076-waived-and-deferred-findings-are-distinct-dispositions

## Summary

Review of 076's reopening for 020's AC19 cross-spec impact (diff-base 8ef7d0e5, base-source reopen) at HEAD c453e9ef. The window's code is 020's, reviewed in its record (a641a31e) and excluded here, so the scope is 076's spec.md, plan.md and data-model.md, which the passes part (P76) read whole against the 99,611-byte rules list, read whole on a second pass after its first ran without it: no finding. The criteria step (part C76) judged every criterion individually against the code wherever it lies, the lib tests of decisions, write_review, check_review_gate and process_decisions run in a clone: AC9 was not met, since 073's task 7 (6a0a5277) removed the constitution's "A question is deferred with its trigger" and §spec-requirements' pointer for a deferred question landed on a findings-only definition; the sentence is restored inside §brownfield-inbox's Deferred definition (the operator's decision, 2026-11-22; 49,850 bytes against the 50,000 ceiling), and AC9 is judged met on the fixed text: 18 of 18 met. The fix-round re-read of the constitution and the data model was clean. Two observations fixed by the rule (the data model's missing `by-rule` rows; a rename history in the plan), and two stored decisions matched. Examined 3 of 4: the constitution, which the fix added as uncommitted work, was read by the fix round's re-read, not the passes. One test, write_analysis's markdown-lint test, fails in a clone with no node_modules, an environment gap unrelated to 076.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

18 of 18 acceptance criteria judged met.

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

## Acceptance criteria

- AC1 — met: framework/constitution.md §brownfield-inbox Finding dispositions: Deferred (pipeline work, external work item, or both) and Waived (with its reason) are defined once.
- AC9 — met: Met after this run's fix: `framework/constitution.md` §brownfield-inbox Finding dispositions defines Deferred as covering a question too ("a question is deferred with its trigger, in Resolved Questions", the sentence 6a0a5277 removed, restored by the operator's decision of 2026-11-22), and §spec-requirements' "A deferred decision (§brownfield-inbox) is resolved with its trigger, in Resolved Questions" points at it.
- AC2 — met: framework/constitution.md §design-principles completion-filter bullet lists a task, scenario or criterion, no waived finding.
- AC3 — met: runtime/src/primitives/decisions.rs Disposition validation refuses a deferral with no target or non-blank work-item at `{at}.target`; tests a_deferral_names_its_work and a_stored_deferral_may_name_only_a_work_item pass in a clone.
- AC4 — met: decisions.rs stores target/task/work-item as given and refuses a waiver carrying one ('a waiver names no future work'); tests a_work_item_id_is_stored_as_given, a_deferral_renders_the_work_it_names, a_work_item_is_flattened_in_the_report_and_kept_in_the_record pass.
- AC10 — met: decisions.rs refuses a work-item spanning a line (LF, CR, U+0085, U+2028, U+2029) naming `{at}.work-item`, blank counts as absent; tests a_work_item_spanning_a_line_separator_is_refused and a_work_item_id_is_stored_as_given pass.
- AC11 — met: decisions.rs validate_new_target refuses a missing target/task naming it; decisions::work_state treats an absent spec/task as Complete; tests a_new_deferral_names_work_that_exists, a_deferral_to_missing_work_is_refused_before_any_write pass.
- AC12 — met: framework/commands/review.md --defer ... [--work-item], commands/analyze.md and reference/implement.md disposition task each accept an external work-item ID alone or beside pipeline work.
- AC13 — met: write_review.rs: a waiver replaces a standing deferral only through expired-decisions, a violation is deferred or waived never both (tests a_waiver_replaces_a_standing_deferral_only_through_expired_decisions, a_violation_is_deferred_or_waived_never_both pass).
- AC5 — met: write_review.rs renders Waived findings and Deferred findings sections apart; write_analysis.rs counts deferred/waived apart and renders released deferrals under 'Deferred to other specs'; dispositions tests pass.
- AC6 — met: grep of framework/reference/review.md and framework/reference/*.md, framework/commands/*.md finds no 'ticket'; §Deferrals carries work-item on a deferral only.
- AC7 — met: Sweep of framework/, docs/, runtime/src prose for waive/defer applied to findings found only defined-sense uses; remaining 'routed/discarded' hits are inbox/groom or the legacy alias reader.
- AC14 — met: schema/primitives.rs Dispositions has deferred (alias routed) and waived (alias discarded); decisions.rs reads outcome routed/discarded as deferred/waived; tests incl. the legacy-outcome ones pass.
- AC15 — met: framework/commands/review.md argument-hint and step 10/11 offer --defer for a MUST or SHOULD violation beside fix and --waive; reference/review.md §Deferrals.
- AC16 — met: runtime check_review_gate tests (a_holding_deferral_is_named_where_the_gate_blocks, a_must_held_by_a_deferral_is_named_where_it_blocks, held_deferrals_the_gate_cannot_read_are_said_so) and decisions::releases (other spec and Pending only) pass in a clone; 97 check_review_gate tests passed.
- AC17 — met: process_decisions.rs lapse: tests a_deferral_to_a_task_lapses_when_the_task_is_complete, a_deferral_to_a_spec_or_rule_lapses_when_its_work_is_complete, a_re_deferral_after_a_lapse_is_stored pass; external-only reported Work::External so it retires only when finding stops firing.
- AC18 — met: Sweep of framework/, docs/, runtime/src, scripts for routed/discarded applied to dispositions: only decisions.rs/primitives.rs/check_artifact_size.rs legacy readers and docs/analyze.md's pre-076 note remain.
- AC8 — met: 020, 058, 066 and 073 spec.md each link 076-waived...; release check-review-gate on 076 reports cross-spec-impact-count 0 (blocked only by review-owed).

## Observations

- bug: framework/constitution.md §brownfield-inbox defined Deferred for findings alone since 073's task 7 (6a0a5277) removed "A question is deferred with its trigger", so §spec-requirements' pointer for a deferred question landed on a findings-only definition and 076's AC9 was not met; the sentence is restored inside the Deferred definition (the operator's decision, 2026-11-22) — `framework/constitution.md` (breaks the spec's contract) — **fixed**
- convention: 076's data model omitted the `by-rule` field of a writer's `Disposition` and the refusal `require_companion` raises under `{at}.by-rule` for an outcome other than fixed or waived; the table and the refusals table now carry both — `specs/076-waived-and-deferred-findings-are-distinct-dispositions/data-model.md` — **fixed** (decided by the rule)
- convention: 076's plan named the bucketing loop with its rename history, `run` (now `sort_findings`; at `a76c99e8`), where the plan records the design as it stands; it now names `sort_findings` — `specs/076-waived-and-deferred-findings-are-distinct-dispositions/plan.md` — **fixed** (decided by the rule)
- other: 076's plan Affected Files omits docs/rules.md and the 058, 066, 020 and 050 scenarios the sweep edited — `specs/076-waived-and-deferred-findings-are-distinct-dispositions/plan.md` — **waived**: the review scope unites Affected Files with every file modified since the diff base, which holds each of them, and the plan's Overview names the by-meaning sweep
- other: 076's plan does not state the same-work match rule or the expired-decisions refusal — `specs/076-waived-and-deferred-findings-are-distinct-dispositions/plan.md` — **waived**: the plan's Data Model section points at the data model for the writer refusals, and the data model states both

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/review.md`
- `.ductus/reference/review.md`
- `.pi/prompts/ductus-review.md`
- `framework/commands/review.md`
- `framework/reference/review.md`
- `runtime/CHANGELOG.md`
- `runtime/src/main.rs`
- `runtime/src/primitives/decisions.rs`
- `runtime/src/primitives/mod.rs`
- `runtime/src/primitives/process_decisions.rs`
- `runtime/src/primitives/process_waivers.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/primitives/write_review.rs`
- `runtime/src/primitives/write_review/departure_waivers.rs`
- `runtime/src/schema/descriptions.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/tests/golden/review-basic.jsonl`
- `scripts/mutation/site-anchors-020.py`
- `specs/020-code-review/analysis.md`
- `specs/020-code-review/data-model.md`
- `specs/020-code-review/plan.md`
- `specs/020-code-review/review.md`
- `specs/020-code-review/scenarios/waiver-expiry.md`
- `specs/020-code-review/spec.md`
- `specs/020-code-review/tasks.md`
- `specs/022-deterministic-runtime/analysis.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/025-rule-opt-out/spec.md`
- `specs/058-findings-route-at-discovery/data-model.md`
- `specs/058-findings-route-at-discovery/plan.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/066-audit-records-and-the-completion-gate/analysis.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/plan.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/process-waivers-file-lists.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`

## Unexamined governance

*None.*
