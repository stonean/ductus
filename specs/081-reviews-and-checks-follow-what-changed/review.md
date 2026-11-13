---
spec: 081-reviews-and-checks-follow-what-changed
last-run: 2026-11-13T17:39:15Z
reviewed-against: 992476e7f6f31e3f989956330a6f601be0a7265d
diff-base: c433ad7893579b8b6e2a01cebac4c9b9cf9720e0
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 9
scope: 18
unread:
  - .claude/commands/ductus/review.md
  - .ductus/reference/review.md
  - .pi/prompts/ductus-review.md
  - framework/reference/review.md
  - runtime/CHANGELOG.md
  - runtime/src/primitives/write_review.rs
  - runtime/src/schema/primitives.rs
  - runtime/tests/golden/review-basic.jsonl
  - specs/inbox.md
excluded: 3
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest: {}
blocking: false
dispositions:
  fixed: 2
  deferred: 0
  waived: 10
  undispositioned: 0
decisions:
  - key: "other — on a run with an empty `diff-base` (a spec never `in-progress`), `write-review` refuses every non-empty `unread` as outside the scope, though `compute-review-scope` returns the plan entries as that scope (`scope: [src/a.rs]`, fixture repo3): the message tells the host to list only entries compute returned, and the entry is one. `resolve_scope` returns `BTreeSet::new()` for the empty base before any scope is derived, while its own doc says that scope still holds a first review's plan entries. AC12 refuses \"an entry the run's scope does not hold\"; this one is held. — `runtime/src/primitives/write_review.rs:828-831`"
    outcome: waived
    reason: "No contract break: an empty diff-base records scope 0 (spec 079; resolve_scope's doc says so), so the run's scope holds no entry and AC12's refusal applies as written; such a record credits nothing, since the credit needs its diff-base to be a commit; and framework/reference/review.md §Window and coverage tells the host to pass the current commit as the base. Only the refusal's wording, which points at compute-review-scope's list, misleads here."
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
  - key: "other — a credit survives the revert of the record that granted it: `read_records` reads each record from the commit that wrote it, and a later commit deleting that review.md does not withdraw the credit, although `last_review_commit` treats a record reverted away as no earlier review ('what the reverted run read has no standing'). Driven: fixtures-A/f3 (record committed, then `git rm` of it) still answers `scope: []`, `review-owed: false` for the chore path. AC13 asks only for a record 'committed in the window', which the reverted record was, so the contract holds; the inconsistency is with 079's revert rule. — `runtime/src/primitives/compute_review_scope.rs:827-874`"
    outcome: waived
    reason: "Observation about the pipeline's own machinery in framework work, and AC13's own text ('a review record committed in the window') is met: the run did read the path. Closing it needs a decision (should a revert withdraw a read?) that a spec owner makes, not a mechanical fix; deferring it would not be fed back by any gate."
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
  - key: "other — a credited path that the window deleted is not listed in `excluded`: `split_changes` skips deleted paths, so the credit shows only as `review-owed: false` and the path stays in `deleted`. AC13 says a credited path 'is listed in `excluded` under the spec whose review read it'. Driven: fixtures-A/f8 (chore deletes src/old.rs, record `unread: []`) answers `excluded: [specs/002-y/review.md]`, `deleted: [src/old.rs]`, `review-owed: false`; f9 (record names it unread) answers `review-owed: true`. The same holds for another spec's deleted path under 079, so the behavior is the existing one. — `runtime/src/primitives/compute_review_scope.rs:351-377`"
    outcome: waived
    reason: A deletion has nothing to read and was never listed in `excluded` under 079 either; the gate's answer (no review owed) is the one AC13 asks for, so only the sentence's reach is loose. Observation about the pipeline's machinery, not a break of what the criterion protects.
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
  - key: "other — `invalidate-review` keeps `unread` on an invalidated record (its 24-entry SCALARS list does not name it), so a record reading `last-run: null` still states what the run it disowns read; confirmed on fixture (`unread:\\n  - src/mine.rs` survives beside `last-run: null`). `read_records` requires `last_run.is_some()`, so no credit follows. — `runtime/src/primitives/invalidate_review.rs:117-142`"
    outcome: waived
    reason: a pipeline-machinery observation outside 081's plan files; the credit's `last-run` guard makes the kept field inert, and 066 owns the invalidation surface
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
  - key: other — `unread` accepts duplicate entries and writes each (`[src/mine.rs, src/mine.rs]` rendered twice); `compute-review-scope` reads the list as a set, so nothing differs. — `runtime/src/primitives/write_review.rs:1360-1375`
    outcome: waived
    reason: harmless to every reader; deduplicating adds a requirement the spec does not state
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
  - key: other — no test pins `--unread-json` hydration (`hydrate_write_review`) or that a run's two writes carry one `unread`; both hold on the built binary (loop-open write then final write each render the same list), as the sibling `--*-json` flags are equally unpinned. — `runtime/src/main.rs:380`
    outcome: waived
    reason: verified by driving the built binary; the sibling write-review flags have no hydration test either, so a pin here is a new test surface, not a fix to 081's code
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
  - key: "other — write-review records `unread: []` beside `examined: 1` and `scope: 4` (driven on a fixture): the two self-reported claims contradict each other, and the record then credits every path of another spec's commit in its window as read. Nothing compares `examined` plus the length of `unread` with `scope`. AC12 asks for the in-scope refusal only, and the `examined` doc already says nothing proves the passes ran, so this is the same self-report the pipeline has always taken. — `runtime/src/primitives/write_review.rs:496-497`"
    outcome: waived
    reason: "an observation about the pipeline's own machinery made while reviewing framework work: AC12 requires the scope-membership refusal and a run that states nothing writing nothing, both of which hold, and `examined` is documented as a caller's unverified claim (write_review.rs and WriteReviewArgs::examined); a consistency refusal would be a new requirement on 081, not a breach of its contract, and does not make this review's own result wrong."
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
  - key: other — render_unread repeats render_open_findings' block-list loop (`key:` then `  - {yaml_string}` per entry). A shared helper would serve both, but the two differ in the empty case (`[]` against omitted), and it would edit 079's renderer for a three-line loop. — `runtime/src/primitives/write_review.rs:1360-1384`
    outcome: waived
    reason: "an observation about the pipeline's own machinery that adds no requirement: two three-line loops whose empty cases differ, and extracting one touches spec 079's render_open_findings for no behavior change."
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
  - key: other — The credit's conditions (a record committed in the window, its window holding the commit, no skipped pass, budget stop or open fix-round loop, no unread means no credit) are restated in COMPUTE_REVIEW_SCOPE, framework/reference/review.md, the 020 and 066 data models, the CHANGELOG entry and the plan. Constitution drift-prevention prefers a pointer to the canonical source (the primitive's compiled description) over copies, and each copy can drift. The copies agree today (checked against read_records in compute_review_scope.rs). — `specs/066-audit-records-and-the-completion-gate/data-model.md:123`
    outcome: waived
    reason: Task 9 of 081 and AC11/AC12 require the reference and both data models to state unread and the credit; the copies match the code now. Pipeline-machinery observation on framework work, waived rather than deferred per AGENTS.md Workflow.
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
  - key: other — AGENTS.md's runtime-currency Workflow bullet says a code commit naming no spec 'owes every open spec a review (spec 081; ...)'; after task 8 that holds only until a committed complete review credits its paths. — `AGENTS.md:Workflow (runtime-currency bullet)`
    outcome: waived
    reason: The sentence explains why a currency fix names 022; it is true at commit time and for every window no record credits, so it contradicts nothing the credit changes.
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
---

# Review — 081-reviews-and-checks-follow-what-changed

## Summary

Window c433ad78..HEAD (base-source reopen), 18 scope entries. Every commit in it names a spec, so this record's unread credits nothing in another window. The passes ran in four agents divided by behavior: the credit, write-review's unread, the shipped text, and a second pass applying to write-review's change the eight rule files its first agent had not read. All nine loaded rule files were read whole and applied to each examined entry's change.
Examined 9, each read whole: framework/commands/review.md, framework/constitution.md, runtime/src/main.rs, runtime/src/primitives/compute_review_scope.rs, runtime/src/schema/descriptions.rs, scripts/mutation/review-read-credit.py, and 081's spec.md, plan.md and tasks.md.
Unread 9, named in unread: the three generated mirrors (.claude/commands/ductus/review.md, .pi/prompts/ductus-review.md, .ductus/reference/review.md), not opened, their sources read and scripts/gen-claude-commands.sh re-run with the tree left unchanged; framework/reference/review.md, its window change in §Window and coverage read in the diff and checked against the code; runtime/CHANGELOG.md, lines 1-40, which hold 081's entry; runtime/src/primitives/write_review.rs, lines 1-4100 (every non-test function, the window's changes and its new tests), the later test code unread; runtime/src/schema/primitives.rs, the window's diff and ReviewBlock's fields; runtime/tests/golden/review-basic.jsonl, read whole but against quality-cross.md alone; specs/inbox.md, an uncommitted /log entry another session made during the run, after the passes ended.
Fixed in the run, before this record:

- a SHOULD, QUAL-TEST-001 at compute_review_scope.rs: no test failed with the credit window's start (walk.hide(base)) or the last-run guard removed. Two tests now pin them, with mutants M12 and M13.
- the broken contract under Observations, pinned by a test and M11.
- the raw echo, pinned by M14.
All 14 mutants of scripts/mutation/review-read-credit.py were killed.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

- bug — a path a record lists in `unread` is credited as read when the path holds `{`, so a changed file such as `src/{a}.rs` named unread by a complete record leaves every other spec's scope and owes no review there. `read_records` parses `unread` with `PlanEntries::new`, which reads every entry as a possible pattern (`entry_pattern` turns `{a}` into `(?:a)`), so the literal `src/{a}.rs` no longer matches itself. `unread` holds scope entries, which are literal paths (plan.md: 'lists scope entries as compute-review-scope returns them'), and `RunScope` and its test `a_changed_path_holding_a_brace_names_only_itself` already keep a changed path literal for this reason (QUAL-DELEG-001). AC14 requires a path the record lists in `unread` to stay in scope. Driven on the built binary: fixtures-A/f1 (record `unread: [src/{a}.rs]`, chore adds `src/{a}.rs`) answers `scope: []`, `review-owed: false`, `excluded` holding `src/{a}.rs`; the same fixture with `src/x.rs` (f7 and the unit test) keeps the path. A path holding `*` or `?` is unaffected (the pattern still matches the literal), and an unmatched `{` compiles to nothing and is kept literal; only a balanced `{...}` loses itself. Plan entries that are patterns stay patterns, which is the intent of the PlanEntries choice. — `runtime/src/primitives/compute_review_scope.rs:799-870` (breaks the spec's contract) — **fixed** (decided by the rule)
- convention — refuse_unread_outside_the_scope echoes each refused `unread` entry raw into the error text. A caller entry holding a newline or an ESC sequence reaches the terminal or the MCP tool-error text unflattened, where InvalidArgument.reason is documented as one line and the sibling code passes caller strings through flatten_line (write_review.rs lines 236, 1759, 1809; load_cli_json's own messages). Driven on a fixture with the built binary: an entry `src/a<LF>forged: line<ESC>[31mred` printed two lines, the second `forged: line^[[31mred is not in the run's scope`. No loaded rule maps to it: BE-INPUT-011 governs a log emitter, and this is error text of a local stdio process, so BE-ERR-002 and BE-INPUT-006 do not fire here either. — `runtime/src/primitives/write_review.rs:865-889` — **fixed** (decided by the rule)
- other — on a run with an empty `diff-base` (a spec never `in-progress`), `write-review` refuses every non-empty `unread` as outside the scope, though `compute-review-scope` returns the plan entries as that scope (`scope: [src/a.rs]`, fixture repo3): the message tells the host to list only entries compute returned, and the entry is one. `resolve_scope` returns `BTreeSet::new()` for the empty base before any scope is derived, while its own doc says that scope still holds a first review's plan entries. AC12 refuses "an entry the run's scope does not hold"; this one is held. — `runtime/src/primitives/write_review.rs:828-831` — **waived**: No contract break: an empty diff-base records scope 0 (spec 079; resolve_scope's doc says so), so the run's scope holds no entry and AC12's refusal applies as written; such a record credits nothing, since the credit needs its diff-base to be a commit; and framework/reference/review.md §Window and coverage tells the host to pass the current commit as the base. Only the refusal's wording, which points at compute-review-scope's list, misleads here. (decided by the rule)
- other — a credit survives the revert of the record that granted it: `read_records` reads each record from the commit that wrote it, and a later commit deleting that review.md does not withdraw the credit, although `last_review_commit` treats a record reverted away as no earlier review ('what the reverted run read has no standing'). Driven: fixtures-A/f3 (record committed, then `git rm` of it) still answers `scope: []`, `review-owed: false` for the chore path. AC13 asks only for a record 'committed in the window', which the reverted record was, so the contract holds; the inconsistency is with 079's revert rule. — `runtime/src/primitives/compute_review_scope.rs:827-874` — **waived**: Observation about the pipeline's own machinery in framework work, and AC13's own text ('a review record committed in the window') is met: the run did read the path. Closing it needs a decision (should a revert withdraw a read?) that a spec owner makes, not a mechanical fix; deferring it would not be fed back by any gate. (decided by the rule)
- other — a credited path that the window deleted is not listed in `excluded`: `split_changes` skips deleted paths, so the credit shows only as `review-owed: false` and the path stays in `deleted`. AC13 says a credited path 'is listed in `excluded` under the spec whose review read it'. Driven: fixtures-A/f8 (chore deletes src/old.rs, record `unread: []`) answers `excluded: [specs/002-y/review.md]`, `deleted: [src/old.rs]`, `review-owed: false`; f9 (record names it unread) answers `review-owed: true`. The same holds for another spec's deleted path under 079, so the behavior is the existing one. — `runtime/src/primitives/compute_review_scope.rs:351-377` — **waived**: A deletion has nothing to read and was never listed in `excluded` under 079 either; the gate's answer (no review owed) is the one AC13 asks for, so only the sentence's reach is loose. Observation about the pipeline's machinery, not a break of what the criterion protects. (decided by the rule)
- other — `invalidate-review` keeps `unread` on an invalidated record (its 24-entry SCALARS list does not name it), so a record reading `last-run: null` still states what the run it disowns read; confirmed on fixture (`unread:\n  - src/mine.rs` survives beside `last-run: null`). `read_records` requires `last_run.is_some()`, so no credit follows. — `runtime/src/primitives/invalidate_review.rs:117-142` — **waived**: a pipeline-machinery observation outside 081's plan files; the credit's `last-run` guard makes the kept field inert, and 066 owns the invalidation surface (decided by the rule)
- other — `unread` accepts duplicate entries and writes each (`[src/mine.rs, src/mine.rs]` rendered twice); `compute-review-scope` reads the list as a set, so nothing differs. — `runtime/src/primitives/write_review.rs:1360-1375` — **waived**: harmless to every reader; deduplicating adds a requirement the spec does not state (decided by the rule)
- other — no test pins `--unread-json` hydration (`hydrate_write_review`) or that a run's two writes carry one `unread`; both hold on the built binary (loop-open write then final write each render the same list), as the sibling `--*-json` flags are equally unpinned. — `runtime/src/main.rs:380` — **waived**: verified by driving the built binary; the sibling write-review flags have no hydration test either, so a pin here is a new test surface, not a fix to 081's code (decided by the rule)
- other — write-review records `unread: []` beside `examined: 1` and `scope: 4` (driven on a fixture): the two self-reported claims contradict each other, and the record then credits every path of another spec's commit in its window as read. Nothing compares `examined` plus the length of `unread` with `scope`. AC12 asks for the in-scope refusal only, and the `examined` doc already says nothing proves the passes ran, so this is the same self-report the pipeline has always taken. — `runtime/src/primitives/write_review.rs:496-497` — **waived**: an observation about the pipeline's own machinery made while reviewing framework work: AC12 requires the scope-membership refusal and a run that states nothing writing nothing, both of which hold, and `examined` is documented as a caller's unverified claim (write_review.rs and WriteReviewArgs::examined); a consistency refusal would be a new requirement on 081, not a breach of its contract, and does not make this review's own result wrong. (decided by the rule)
- other — render_unread repeats render_open_findings' block-list loop (`key:` then `- {yaml_string}` per entry). A shared helper would serve both, but the two differ in the empty case (`[]` against omitted), and it would edit 079's renderer for a three-line loop. — `runtime/src/primitives/write_review.rs:1360-1384` — **waived**: an observation about the pipeline's own machinery that adds no requirement: two three-line loops whose empty cases differ, and extracting one touches spec 079's render_open_findings for no behavior change. (decided by the rule)
- other — The credit's conditions (a record committed in the window, its window holding the commit, no skipped pass, budget stop or open fix-round loop, no unread means no credit) are restated in COMPUTE_REVIEW_SCOPE, framework/reference/review.md, the 020 and 066 data models, the CHANGELOG entry and the plan. Constitution drift-prevention prefers a pointer to the canonical source (the primitive's compiled description) over copies, and each copy can drift. The copies agree today (checked against read_records in compute_review_scope.rs). — `specs/066-audit-records-and-the-completion-gate/data-model.md:123` — **waived**: Task 9 of 081 and AC11/AC12 require the reference and both data models to state unread and the credit; the copies match the code now. Pipeline-machinery observation on framework work, waived rather than deferred per AGENTS.md Workflow. (decided by the rule)
- other — AGENTS.md's runtime-currency Workflow bullet says a code commit naming no spec 'owes every open spec a review (spec 081; ...)'; after task 8 that holds only until a committed complete review credits its paths. — `AGENTS.md:Workflow (runtime-currency bullet)` — **waived**: The sentence explains why a currency fix names 022; it is true at commit time and for every window no record credits, so it contradicts nothing the credit changes. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `specs/020-code-review/data-model.md` — 020-code-review, 066-audit-records-and-the-completion-gate
- `specs/020-code-review/spec.md` — 020-code-review
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 020-code-review, 066-audit-records-and-the-completion-gate

## Unexamined governance

*None.*
