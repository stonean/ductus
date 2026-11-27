---
spec: 088-rule-judgments-cost-only-what-changed
last-run: 2026-11-26T23:56:21Z
reviewed-against: 42202b22cd66eae509ccd56aada7aaa13df33893
diff-base: 658220e840c9171f460ecf000c895577913046ee
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 26
scope: 39
unread:
  - .claude/commands/ductus/configure.md
  - .ductus/reference/analyze.md
  - .ductus/reference/implement.md
  - framework/bootstrap/govern.md
  - framework/bootstrap/ductus.md
  - runtime/CHANGELOG.md
  - runtime/src/interpreter/mod.rs
  - runtime/src/interpreter/payload.rs
  - runtime/src/main.rs
  - runtime/src/primitives/mod.rs
  - runtime/src/primitives/write_analysis.rs
  - runtime/src/schema/descriptions.rs
  - runtime/src/schema/primitives.rs
excluded: 45
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 048859896e2a703ed2b4fa0e6bc003f746e10fa2a62c7632463a8abc8730764e
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
  fixed: 15
  deferred: 0
  waived: 1
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

Window `658220e8` to `42202b22` (base-source `reopen`, the commit before 088 first went `in-progress`), run in this checkout with a clean tree and every primitive through the release CLI, since the session's MCP server ran a build older than `a66c818a`. 39 scope entries; 26 examined, each read whole by a passes part, the pattern entry `framework/bootstrap/configure/*.md` as `claude.md` and `pi.md` and `specs/*/analysis.md` as the ten records `f550aac7` backfilled. Unread, for the next review: four entries not opened, `.claude/commands/ductus/configure.md`, `.ductus/reference/analyze.md` and `.ductus/reference/implement.md`, generated from sources read whole and reported in sync by `scripts/gen-claude-commands.sh --check`, and `framework/bootstrap/govern.md`, byte-identical to `framework/bootstrap/ductus.md` by `cmp`; and nine large files read at 088's own commits' hunks and the code around them: `framework/bootstrap/ductus.md` (its claude seed whole), `runtime/CHANGELOG.md`, `runtime/src/interpreter/mod.rs`, `runtime/src/interpreter/payload.rs` (`load_rules` whole), `runtime/src/main.rs`, `runtime/src/primitives/mod.rs`, `runtime/src/primitives/write_analysis.rs` (`RuleFields`, `derive_rule_fields`, `render_rule_fields` and `render_section_digest` whole), `runtime/src/schema/descriptions.rs` (its three changed constants whole) and `runtime/src/schema/primitives.rs`. The read modes and the split, four passes parts by behaviour each running all five passes and one criteria part, are the agent's recommendation, adopted by the operator, 2026-11-26. Loaded rule files: `configuration-cross.md`, `quality-cross.md`, `security-backend.md`, `security-cross.md` and `security-frontend.md` (231,962 bytes), each read whole by each passes part; no pass was skipped, and seven rule files are judged by analysis alone, by their headers. `specs/system.md` names no part, so the reuse pass read none. Findings: none. Observations: one contract break fixed, AC2's rule naming a removed rule left current while a file's copy could not stand in, fixed in the code by the operator's decision (the agent's recommendation, adopted by the operator, 2026-11-26) and recorded in the spec's Resolved Questions; 14 other fixes; one waived by the rule. The passes were re-run over the whole fix diff, each changed item read whole. `QUAL-TEST-001`: each test 088 added under `runtime/` is killed by a mutant in `scripts/mutation/rule-judgments-088.py`, the two the list did not name now by M34 and by four hand mutants of `a_rule_s_names_are_the_other_ids_its_statement_and_verification_hold`, and the review's new tests by M35 to M37. Criteria: each judged individually against the code its claim names and driven on fixtures, AC2 again with a dev build carrying the fix. Pi needs no counterpart: its bridge registers every tool `tools/list` serves. Not examined: the Cost section's figures measured at `1ffaf951`, `7f8095e1` and `254db2af` (the 1,009, 59 and 104 judgments owed, the 19.5 and 17.2 ms medians, the backfill's wall times, the 252,789-character total), read as recorded; `ductus exec`'s binding of `pinned`, which no command calls.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

3 of 17 commits in the window no spec claimed.

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

- AC1 — met: `runtime/src/primitives/write_analysis.rs` `derive_rule_fields` and `render_rule_fields` write `rules-judged` and `rules-carried` on every record, proved by `a_run_carrying_one_verdict_and_judging_one_rule_records_both_counts` (1 and 1) with the carried-quiet, unresolved-addressed and partition tests; the release binary on a fixture carrying one verdict and judging one rule afresh wrote `rules-judged: 1`.
- AC2 — met: `runtime/src/primitives/rule_verdicts.rs` `standing`, `RuleEdits::between` and `judged_copy` judge only an edited rule and each rule naming it or a removed one, and the whole file with its reason when the copy cannot stand in, each rule naming an ID no loaded rule defines with it; proved by `an_edit_to_one_rule_judges_it_and_each_rule_naming_it_alone`, `a_shallow_clone_past_analyzed_against_judges_the_whole_file` (a real `--depth 1` clone) and `a_file_judged_whole_judges_each_rule_naming_an_id_no_loaded_rule_defines`, and driven with a dev build on a shallow clone.
- AC3 — met: `runtime/src/primitives/backfill_section_digest.rs` `compute` and `matching_text`, shipped as `section-digest-backfill` in `framework/migrations.toml` sorting after every entry; proved by its ten tests and `backfill_section_digest_preserves_crlf`, and on a three-record fixture the release binary wrote the current and other-status records, named the stale one and wrote nothing on a second run.

## Observations

- convention: `AnalyzeBlock::rule_digest`'s doc comment in `runtime/src/schema/primitives.rs` still stated the per-file rule 088 retired, a verdict current only while its file's digest matches; it now states the rule-by-rule comparison at `analyzed-against` — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- convention: the docstring of `scripts/mutation/rule-verdicts-converge.py` still said a verdict is reused while the rule's file is unchanged, and credited AC5 to M2 without saying M2 now holds the per-file gate that opens 088's rule-by-rule comparison — `scripts/mutation/rule-verdicts-converge.py` — **fixed** (decided by the rule)
- other: the Cost section of `specs/088-rule-judgments-cost-only-what-changed/spec.md` stated no figure for the growth of `DISCOVER_RULE_FILES`, which loads with the tool in every session; it now states 4,393 to 5,244 characters with its method and commits — `specs/088-rule-judgments-cost-only-what-changed/spec.md` — **fixed** (decided by the rule)
- convention: the Edge Cases of `specs/088-rule-judgments-cost-only-what-changed/spec.md` said the two counts sum to the rules a run assessed, while `specs/088-rule-judgments-cost-only-what-changed/data-model.md` and `derive_rule_fields` make that sum the two counts plus `unjudged-rules`; the bullet now says so — `specs/088-rule-judgments-cost-only-what-changed/spec.md` — **fixed** (decided by the rule)
- convention: `scripts/mutation/rule-judgments-088.py` held no mutant for `a_project_in_a_subdirectory_is_backfilled`, though the plan says each behaviour has one; M34 now rebinds the project to its repository root, which only that test kills — `scripts/mutation/rule-judgments-088.py` — **fixed** (decided by the rule)
- convention: the link guard in `compute` in `runtime/src/primitives/backfill_section_digest.rs` had no test, and the data model and `BACKFILL_SECTION_DIGEST` named a linked feature directory or record but not a linked file in it; a unix test and M35 are added, and both texts now say a file in it — `runtime/src/primitives/backfill_section_digest.rs` — **fixed** (decided by the rule)
- convention: the `backfill-section-digest` result example in `specs/088-rule-judgments-cost-only-what-changed/data-model.md` omitted `pinned-count`, which every result carries — `specs/088-rule-judgments-cost-only-what-changed/data-model.md` — **fixed** (decided by the rule)
- other: the Cost section of `specs/088-rule-judgments-cost-only-what-changed/spec.md` gave the backfill's result only on the fixture corpora of `scripts/measure-result-sizes.sh`, which hold no `analysis.md`; it now also gives 1,458 characters over a clone of this repository at `42202b22` — `specs/088-rule-judgments-cost-only-what-changed/spec.md` — **fixed** (decided by the rule)
- other: `runtime/tests/result_sizes.rs` measures `backfill-section-digest` on corpora holding no `analysis.md`, so its row reports a result that does not grow while the result's `written` and `unmatched` lists grow with a corpus — `runtime/tests/result_sizes.rs` — **waived**: Not mechanical: a Prep writing records that match their specs' design subjects is new fixture code, and past_the_read_size_the_lists_move_to_the_file already proves the result's lists move to the file past [artifacts] read-size-bytes, while the spec's Cost section now states the result over this corpus. (decided by the rule)
- convention: task 6 in `specs/088-rule-judgments-cost-only-what-changed/tasks.md` was checked but still ended with a list of figures still to measure, all of which the spec's Cost section states; the list is removed — `specs/088-rule-judgments-cost-only-what-changed/tasks.md` — **fixed** (decided by the rule)
- convention: the Cost section of `specs/088-rule-judgments-cost-only-what-changed/plan.md` said AC2's added read is timed by `/usr/bin/time -p`, whose 0.01 s cannot resolve the 2.3 ms the spec reports by `time.perf_counter`; the plan now names the spec's method — `specs/088-rule-judgments-cost-only-what-changed/plan.md` — **fixed** (decided by the rule)
- bug: AC2 asks that a rule naming one its file no longer holds be judged again, but while a changed file's copy at `analyzed-against` could not stand in, `RuleEdits::between` in `runtime/src/primitives/rule_verdicts.rs` added only the file's current rules to `changed`, so a rule in another file naming a removed rule kept its verdict; it now adds each ID a loaded rule names that no loaded rule defines (the agent's recommendation, adopted by the operator, 2026-11-26) — `runtime/src/primitives/rule_verdicts.rs` (breaks the spec's contract) — **fixed**
- convention: the doc comment of the new test `a_feature_holding_a_link_leaving_the_project_is_named_and_left` said the linked text would match, but the digest walk does not follow the link, so without the guard the feature reads `unmatched`; the clause is removed — `runtime/src/primitives/backfill_section_digest.rs` — **fixed** (decided by the rule)
- convention: `specs/088-rule-judgments-cost-only-what-changed/data-model.md` defined a name as a changed rule's ID, which an ID no loaded rule defines is not; it now says such an ID too — `specs/088-rule-judgments-cost-only-what-changed/data-model.md` — **fixed** (decided by the rule)
- convention: the Cost section of `specs/088-rule-judgments-cost-only-what-changed/spec.md` gave the descriptions' new lengths, 5,244 and 2,533 characters, at no commit; each now cites this review's commit by its subject — `specs/088-rule-judgments-cost-only-what-changed/spec.md` — **fixed** (decided by the rule)
- convention: the new test `a_file_judged_whole_judges_each_rule_naming_an_id_no_loaded_rule_defines` passed with the filter on defined IDs dropped, since every defined name in its fixture belonged to a stale rule; a rule naming an unchanged defined rule and M37 are added — `runtime/src/primitives/rule_verdicts.rs` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/plan.md`
- `.ductus/docs/agents/developing-ductus.md`
- `.ductus/reference/review.md`
- `.pi/prompts/ductus-plan.md`
- `AGENTS.md`
- `README.md`
- `docs/slash-commands.md`
- `framework/commands/plan.md`
- `framework/migrations/report-project-file-links-into-docs.md`
- `framework/reference/review.md`
- `runtime/src/interpreter/analyze_tally.rs`
- `runtime/src/primitives/check_artifact_size.rs`
- `runtime/src/primitives/project_parts.rs`
- `runtime/src/primitives/prune_project.rs`
- `runtime/src/primitives/prune_project/moves.rs`
- `runtime/src/primitives/resolve_constitutions.rs`
- `runtime/src/schema/paths.rs`
- `runtime/tests/check_artifact_size_cli.rs`
- `scripts/mutation/project-parts-087.py`
- `scripts/mutation/resolve-constitutions-size.py`
- `scripts/mutation/reviews-follow-what-changed.py`
- `specs/002-project-scaffolding/analysis.md`
- `specs/003-bootstrap-automation/analysis.md`
- `specs/009-scenario-targeting/analysis.md`
- `specs/019-config-decisions/analysis.md`
- `specs/027-bootstrap-migration-registry/analysis.md`
- `specs/036-quality-cross-rules/analysis.md`
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/analysis.md`
- `specs/047-analyze-findings-durability/scenarios/analyze-record-freshness.md`
- `specs/050-constitution/spec.md`
- `specs/052-spec-supersession-and-consolidation/analysis.md`
- `specs/056-bootstrap-archive-boundary-split/analysis.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md`
- `specs/064-pi-host-support/analysis.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/066-audit-records-and-the-completion-gate/tasks.md`
- `specs/082-rule-assessments-converge-across-runs/data-model.md`
- `specs/082-rule-assessments-converge-across-runs/plan.md`
- `specs/082-rule-assessments-converge-across-runs/spec.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/analysis.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/review.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/tasks.md`

## Unexamined governance

*None.*
