---
spec: 088-rule-judgments-cost-only-what-changed
last-run: 2026-11-27T01:10:09Z
reviewed-against: b8cf8efabb8438f02c24acb97731ebc60ed4fe5a
diff-base: 812b7a890e96e1128a31b154e6f71fcd1ce2bed0
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 8
scope: 21
unread:
  - .claude/commands/ductus/configure.md
  - .ductus/reference/analyze.md
  - .ductus/reference/implement.md
  - framework/bootstrap/ductus.md
  - framework/bootstrap/govern.md
  - runtime/CHANGELOG.md
  - runtime/src/interpreter/mod.rs
  - runtime/src/interpreter/payload.rs
  - runtime/src/main.rs
  - runtime/src/primitives/mod.rs
  - runtime/src/primitives/write_analysis.rs
  - runtime/src/schema/descriptions.rs
  - runtime/src/schema/primitives.rs
excluded: 19
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 16f35f2d6ef5050a71927161866f13d7218722f3b210512e9a1811c5d738748e
criteria-judged: 3
criteria-met: 3
criteria-verdicts:
  - label: AC1
    digest: cb2c56fd3d69e18781c8a8e48df4a769cccd2444ef38618d90946ad326eadee0
  - label: AC2
    digest: 92aff8f28c1ee066f5c9e34da87210788b1e28f5c260ec9b5ce0ae85002b9361
  - label: AC3
    digest: bbb72df257098dc94a6a7358a3172ced5ac77771e3a6217815595d990fbe1fa8
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 0
  undispositioned: 0
decisions:
  - key: "other: `runtime/tests/result_sizes.rs` measures `backfill-section-digest` on corpora holding no `analysis.md`, so its row reports a result that does not grow while the result's `written` and `unmatched` lists grow with a corpus — `runtime/tests/result_sizes.rs`"
    outcome: waived
    reason: "Not mechanical: a Prep writing records that match their specs' design subjects is new fixture code, and past_the_read_size_the_lists_move_to_the_file already proves the result's lists move to the file past [artifacts] read-size-bytes, while the spec's Cost section now states the result over this corpus."
    decided-at: 2026-11-26T23:08:11Z
    decided-by: rule
---

# Review — 088-rule-judgments-cost-only-what-changed

## Summary

Window `812b7a89` to `b8cf8efa` (base-source `last-review`), the review after task 8, which leaves a shipped rule's ID out of what a file judged whole stales; run in this checkout with a clean tree, every primitive through the release CLI. 21 scope entries; 8 examined, each read whole by the passes part: `runtime/src/primitives/rule_verdicts.rs`, `runtime/src/primitives/check_rule_ids.rs`, `scripts/mutation/rule-judgments-088.py` and 088's `spec.md`, `plan.md`, `data-model.md`, `tasks.md` and `analysis.md`. Unread, for the next review: the 13 entries the last record left unread, which join any review's scope. `runtime/src/schema/descriptions.rs` was read at its `DISCOVER_RULE_FILES` constant whole and `runtime/CHANGELOG.md` at task 8's hunk. The other eleven were not opened: seven changed in no 088 commit in this window, `framework/bootstrap/ductus.md`, `runtime/src/interpreter/mod.rs`, `runtime/src/interpreter/payload.rs`, `runtime/src/main.rs`, `runtime/src/primitives/mod.rs`, `runtime/src/primitives/write_analysis.rs` and `runtime/src/schema/primitives.rs`; and four are copies, `.claude/commands/ductus/configure.md`, `.ductus/reference/analyze.md` and `.ductus/reference/implement.md`, whose generated copies `scripts/gen-claude-commands.sh --check` reports in sync, and `framework/bootstrap/govern.md`, byte-identical to `ductus.md` by `cmp`. One passes part ran all five passes over the five loaded rule files (231,962 bytes, each read whole), and one criteria part judged every criterion; seven rule files are judged by analysis alone, by their headers. `specs/system.md` names no part, so the reuse pass read none. Findings: none. Observations: three chores fixed in `data-model.md`, `spec.md` and `plan.md`; the passes re-ran over the fix diff. Criteria: each judged individually against the code its claim names and driven on fixtures with the release binary, AC2's shipped exclusion on a `--depth 1` clone. Not examined: the Cost section's timing figures; a runtime older than its project's rule files, which lockstep versioning rules out.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

0 of 4 commits in the window no spec claimed.

3 of 3 acceptance criteria judged met.

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

- AC1 — met: `runtime/src/primitives/write_analysis.rs` `derive_rule_fields` writes both counts, proved by `a_run_carrying_one_verdict_and_judging_one_rule_records_both_counts`; the release binary recorded `rules-judged: 1` and `rules-carried: 1` on a fixture run carrying one verdict and judging one rule afresh.
- AC2 — met: `runtime/src/primitives/rule_verdicts.rs` `RuleEdits::between` and `judged_copy`, proved by `rule_verdicts`' tests with `a_file_judged_whole_judges_each_rule_naming_an_id_no_loaded_rule_defines` and a real `--depth 1` clone test; on a shallow-clone fixture the release binary judged the whole file, the namers of a removed rule and of an undefined ID, and no namer of a defined or shipped rule.
- AC3 — met: `runtime/src/primitives/backfill_section_digest.rs` `compute`, shipped as `section-digest-backfill` sorting last in `framework/migrations.toml`; its tests and a three-record fixture on the release binary wrote the current and other-status records, named the stale one and wrote nothing on a second run, and a run over a clone of HEAD wrote nothing.

## Observations

- convention: the Read bullet of `specs/088-rule-judgments-cost-only-what-changed/data-model.md` counts a record whose design subjects hold no section under `no-design-subject`, but `compute` counts one only after a match, so a record whose design-subject files are gone reads `unmatched`; the clause moves to the Write bullet — `specs/088-rule-judgments-cost-only-what-changed/data-model.md` — **fixed** (decided by the rule)
- convention: the Cost section of `specs/088-rule-judgments-cost-only-what-changed/spec.md` gave the backfill description's 2,533 at `812b7a89` with no unit, beside a 2,906 that counts the description and the schema; it now says the description is 2,533 characters — `specs/088-rule-judgments-cost-only-what-changed/spec.md` — **fixed** (decided by the rule)
- convention: the Affected Files table of `specs/088-rule-judgments-cost-only-what-changed/plan.md` held no row for `runtime/src/primitives/check_rule_ids.rs`, which task 8 changed; the row is added — `specs/088-rule-judgments-cost-only-what-changed/plan.md` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/review.md`
- `.ductus/reference/review.md`
- `.pi/prompts/ductus-review.md`
- `framework/commands/review.md`
- `framework/reference/review.md`
- `runtime/src/primitives/compute_review_scope.rs`
- `runtime/src/primitives/write_review.rs`
- `runtime/src/primitives/write_review/departure_waivers.rs`
- `runtime/src/primitives/write_review/project_record.rs`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/mcp.rs`
- `runtime/tests/walker.rs`
- `scripts/mutation/outcomes-outside-the-scope-086.py`
- `scripts/mutation/project-record-084.py`
- `scripts/mutation/scope-attribution.py`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/073-decided-rule-departures-are-recorded/review.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/plan.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/tasks.md`

## Unexamined governance

*None.*
