---
spec: 078-review-fix-rounds-are-re-read-within-a-bound
last-run: 2026-11-22T00:28:58Z
reviewed-against: 584a48ea8ee92dd2d94f3ed7a8550f0b87bdce75
diff-base: 70c4f3da78274e98f56356e7121e8d5bf45d8250
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 16
scope: 19
unread:
  - .claude/commands/ductus/review.md
  - .ductus/reference/review.md
  - .pi/prompts/ductus-review.md
excluded: 74
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 168e5910602ec2917f2b05ea82a7359ebf3961dabb571dfccdb2179c5f5be526
  scenarios/a-reviews-own-record-is-no-fix-round.md: 8a0190ef5a926425199a4381d558f52db626cdd81803b4e23c9aa1055b130440
  scenarios/fix-round-results-have-a-bounded-size.md: 3cf9dc7ebb4213d371eadee7db64826448d2da4ed0bf3ffa96b4fecb897f78d9
criteria-judged: 17
criteria-met: 17
criteria-verdicts:
  - label: AC1
    digest: 211554d4188d2ed027105533972232dddbf6a6091c045b4ce9f843691c96a470
  - label: AC2
    digest: c9c07bb30fce7c201ef2d02eb1e7d71d0d82c39972945e01b4fa0b0d03a33f9b
  - label: AC3
    digest: 6cb4a3a752b7387e793a7a30de7169cfe88722d460feeccb43a9c7754b7f4a20
  - label: AC10
    digest: ad7c4934fa751943e5125f9d6c5104fc54f8d13e07a733ddd568043676cce4fc
  - label: AC4
    digest: 458e1d53275357395a09e2bbbbe54c97e3377cc39afbddd6fc493a9f7f8f3b99
  - label: AC5
    digest: be42962c23828b1f0e59575c78598877b2b217e9036500cf71787ab040362e4b
  - label: AC11
    digest: e1f75d6ac7015a5a5807346a5d53e2d16f2937f0b326dbe3faa181fd04ca5a5f
  - label: AC6
    digest: ff3c696e0b329b22a0ecc02ca6a552a6e0e56015d22f7b1491d8cfbad25d40ee
  - label: AC7
    digest: 0f322d4292d4342f86ca90eba6d731e7f5174ec8d98ec717d4e1b7e9f4494723
  - label: AC8
    digest: bd527431a3305f9eb84b8584808bc35110370584db90c5e31ca4f43c7e7f2925
  - label: AC12
    digest: 1b9f425dc0c7022c35da1a35b4bf0e43ca78e3bde6ea63c9bc790493da4c6cb6
  - label: AC13
    digest: 1e70bcb13983e547ed4a3cab8340ed9048a627034845a0999e98ba5464739960
  - label: AC14
    digest: d1b9de2469f4a7a61e4136d4be3594e52d6c7d510d089e0104d970444594ccad
  - label: AC15
    digest: eabe0d9e7c16f9c5f93b1b6aea5e338f21d9e279a651b420286d7e341e4e367e
  - label: AC9
    digest: c7e910b0622eb9c723109c313176dca243115fe8dbc78dbdd56d95a32086065a
  - label: AC16
    digest: c24753ca0fd84bbb16156a7e3f5d95a634b913d12a2d4a05dbf71bb3355f4185
  - label: AC17
    digest: c8384de25450e099aec7653b3b62ff45a0a18db12b8cca1da50cd8ae2f7560bc
blocking: false
dispositions:
  fixed: 2
  deferred: 0
  waived: 7
  undispositioned: 0
decisions:
  - key: "bug: 066's data-model example lists two `modified-since` paths with a count of 1 — `specs/066-audit-records-and-the-completion-gate/data-model.md`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "19"
    decided-at: 2026-11-04T13:33:22Z
    decided-by: andy@stone.dev
  - key: "convention: `compute-review-scope`'s file-form test asserts `deleted` on a fixture that deletes nothing, so three mutations survive — `runtime/src/primitives/compute_review_scope.rs`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "19"
    decided-at: 2026-11-04T13:33:22Z
    decided-by: andy@stone.dev
  - key: "bug: an open re-read MUST or SHOULD left at a stop is an observation, and undispositioned it never holds done — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): low confidence; the loop asks the operator to defer or waive each open finding at a stop, and a design change would cost 078 another cycle."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "perf: a file-form loop result's free text (an echoed path or key in problem, the re-reads-off reason) has no bound — `runtime/src/primitives/check_prescriptions.rs`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): reaching it takes an agent-written path or key, or a configured reason, tens of kilobytes long; the lists, which carry the growth, are bounded."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "other: the Family 19 parity test never seeds the never-run or failed-call branches — `runtime/tests/mechanical_sweep_parity.rs`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "bug: check-review-freshness emits last_run and analyzed_against where 066's data-model example shows kebab-case — `specs/066-audit-records-and-the-completion-gate/data-model.md`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "convention: 066's review-staleness-on-done-specs says review-freshness.sh has always carried the exemption — `specs/066-audit-records-and-the-completion-gate/scenarios/review-staleness-on-done-specs.md`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "convention: mechanical_sweep.rs and its parity test carry doc comments the window made stale — `runtime/src/primitives/mechanical_sweep.rs`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "convention: RecordFreshness::Undeterminable's doc names only a missing digest, where an unparseable review.md also answers it — `runtime/src/schema/primitives.rs`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "convention: the Family 19 parity test spawns bash without ductus_command, so session identities reach the runtime — `runtime/tests/mechanical_sweep_parity.rs`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "other: 066's plan states counts of audit scripts and done specs with no method, and they re-derive differently — `specs/066-audit-records-and-the-completion-gate/plan.md`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: rule BE-PAYLOAD-001 at specs/066-audit-records-and-the-completion-gate/data-model.md
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "20"
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "bug: check-prescriptions does not normalize an edit's path, so ./a, a//b or a/./b fails the call or differs — `runtime/src/primitives/check_prescriptions.rs`"
    outcome: waived
    reason: "Fail-safe: a path spelled other than repo-relative never matches, so the call fails or the round differs and is re-read (AC15); the brief asks for repo-relative paths."
    decided-at: 2026-11-09T18:37:37Z
    decided-by: rule
  - key: "perf: an exempt edit beside exactly the prescribed edits makes the diff check differ, costing a re-read of a verified round — `runtime/src/primitives/check_prescriptions.rs`"
    outcome: waived
    reason: It matches AC5 as written; exempting those paths from the diff check is a requirement change, and its cost is one re-read.
    decided-at: 2026-11-09T18:37:37Z
    decided-by: rule
  - key: "bug: the re-reads-off reason's 16-character minimum is measured before its control characters and runs of spaces are collapsed — `runtime/src/primitives/fix_rounds.rs`"
    outcome: waived
    reason: Contrived input; the check matches the data model's 'once trimmed'.
    decided-at: 2026-11-09T18:37:37Z
    decided-by: rule
  - key: "other: the loop primitives accept a misspelled [review] key silently — `runtime/src/primitives/fix_rounds.rs`"
    outcome: waived
    reason: "/review's step 2, discover-rule-files, reports an unknown [review] key (CFG-ENV-012), verified on a fixture: 'unknown config key: [review] re-read-off'."
    decided-at: 2026-11-09T18:37:37Z
    decided-by: rule
  - key: "security: with --allow-sequential, retire-feature screens the feature name by validate_feature_argument's denylist alone, and --feature . deleted every file under specs/ on a fixture — `runtime/src/primitives/retire_feature.rs`"
    outcome: deferred
    target: specs/052-spec-supersession-and-consolidation/tasks.md
    task: "4"
    decided-at: 2026-11-09T18:37:37Z
    decided-by: andy@stone.dev
  - key: "bug: a record's current digest clears review-owed for its contracts even when that run stopped early — `runtime/src/primitives/compute_review_scope.rs`"
    outcome: waived
    reason: It matches compute-review-scope's description and 079's window design, in the narrow case of a contract-only reopening after a restricted run; not 078's loop.
    decided-at: 2026-11-09T18:37:37Z
    decided-by: rule
  - key: "other: a fixer's uncommitted edit that another spec's commit sweeps in leaves the round as excluded and is never re-read here — `runtime/src/primitives/fix_rounds.rs`"
    outcome: waived
    reason: "It follows this spec's Edge Case on other sessions: that spec's review reads it as its own change, and git worktree isolates sessions."
    decided-at: 2026-11-09T18:37:37Z
    decided-by: rule
  - key: "other: a supplied Summary gets the loop's account but not the generated one's excluded-from-scope sentence — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: 079's contract; the frontmatter's excluded count and the Excluded from scope section carry it.
    decided-at: 2026-11-09T18:37:37Z
    decided-by: rule
  - key: "other: fix rounds driven by mechanical NOTEs have no length bound — `framework/reference/review.md`"
    outcome: waived
    reason: The Behavior permits NOTE rounds and AC6's budget counts re-reads with a MUST or SHOULD; a bound on NOTE rounds is a requirement decision.
    decided-at: 2026-11-09T18:37:37Z
    decided-by: rule
  - key: "other: 066's task 19 items 2 and 4 prescribe old text that no longer occurs — `specs/066-audit-records-and-the-completion-gate/tasks.md`"
    outcome: waived
    reason: 066's own task list, outside 078's code; the stored deferrals that pointed there expired this run, their issues gone.
    decided-at: 2026-11-09T18:37:37Z
    decided-by: rule
  - key: "convention: AC14's 'a switch without a valid reason halts the run', read alone, covers an empty re-reads-off, which now counts as absent — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md`"
    outcome: waived
    reason: The data model's re-reads-off row, the durable contract the switch rests on, says an empty value counts as absent and so is no switch, as task 17 records (owner, 2026-11-09).
    decided-at: 2026-11-09T20:01:53Z
    decided-by: rule
  - key: "bug: a waived round's reason ending before a closing quote or bracket, or in an ellipsis or a colon, still takes a second stop — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: "Contrived input: which closing characters end a sentence is a style choice, and the record's contract (078's data model, 066's two loop scenarios) asks only that the Summary name each waived round, which it does; the comment that claimed more is fixed in this run."
    decided-at: 2026-11-09T21:28:14Z
    decided-by: rule
  - key: "other: analysis.md counts as fixed in its run a finding an earlier commit had removed — `specs/078-review-fix-rounds-are-re-read-within-a-bound/analysis.md`"
    outcome: waived
    reason: analysis.md is write-analysis's output, which the analysis this change owes rewrites; whether a finding an earlier commit removed counts as fixed is analyze's machinery, outside 078's code.
    decided-at: 2026-11-09T21:28:14Z
    decided-by: rule
---

# Review — 078-review-fix-rounds-are-re-read-within-a-bound

## Summary

Five-pass review of 078's reopening (diff-base 70c4f3da, base-source reopen) at HEAD 584a48ea. Its code commit is ef4ff965 (task 20, AC16 and AC17: a re-read raises only reachable cases, and the budget stop quotes the runtime's figures), with the spec-only 91dd60ea. Passes (part P2, one reviewer for 078 and 055) read those commits' hunks in each scope file, and the files whole where a behavior needed context, against the five rule files `discover-rule-files` selected (232,805 bytes, each read whole against `wc -c`): no finding and two observations. The criteria step (part C2) judged every criterion individually: 17 of 17 met, the loop driven with the release binary on a fixture. It also showed that AC16's test, `the_brief_carries_exactly_the_derived_paths` in `runtime/src/primitives/brief_re_read.rs`, compared the brief with the constant that builds it, so a bar that lost its reachable-trigger clause passed: a QUAL-TEST-001 SHOULD, fixed in this run by an assertion on the clause's own text, its mutant in `scripts/mutation/reachable-trigger-bar-078.py` killed. Also fixed: the plan's Affected Files (the `start.toml` record and loop figures in `fix_rounds.rs`, `descriptions.rs`, `fix_rounds.rs` in the line-ending exempt list) and the data model's `start-fix-rounds` errors, which lacked a `start.toml` that cannot be written. P2 re-ran its passes over the fix. Seven stored waivers still apply; ten stored decisions are dropped: the deletions-only window one, since step 1 now names each deleted path, and nine naming `scripts/audit/review-freshness.sh`, `runtime/src/primitives/check_review_freshness.rs` and two 026 scenarios, which 081 removed. Examined 16 of 19, the new mutation list included; unread: the three generated copies (`.claude/commands/ductus/review.md`, `.ductus/reference/review.md`, `.pi/prompts/ductus-review.md`), confirmed in sync by `scripts/gen-claude-commands.sh --check`.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

17 of 17 acceptance criteria judged met.

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

- AC1 — met: `framework/commands/review.md` step 12 loops brief-re-read, hands the brief path to an independent agent, read-re-read, and stops on kind empty; driven in a fixture: re-read, exempt and empty rounds each came back as stated.
- AC2 — met: `brief_re_read.rs` `split_exempt/classify`: driven, a `spec.md`-only edit gave kind exempt and the round named in write-review's Summary; `analysis.md` and `scenarios/s.md` each forced re-read, and the feature's own `review.md` was left out; tests `a_spec_body_edit_is_exempt`, `anything_else_needs_a_re_read`.
- AC3 — met: `render_brief` in `brief_re_read.rs` renders paths only from the derived round (driven: brief JSON list held only `a.rs`, the exempt `spec.md` left out) with the severity bar; test `the_brief_carries_exactly_the_derived_paths`, `an_exempt_path_leaves_a_mixed_rounds_brief_and_record`.
- AC10 — met: `start_fix_rounds.rs/fix_rounds`::snapshot: driven, an untracked file appeared in the next round's paths, and after three snapshots git for-each-ref, HEAD and the index (status ' M') were unchanged; tests `the_snapshot_moves_nothing`, `the_snapshot_is_the_working_tree`.
- AC4 — met: `fix_rounds::demote_out_of_round`: driven, a MUST on `zzz.rs` outside the round read back as NOTE with demoted:1, and it did not count in re-reads-with-findings; step 12 says to disposition each NOTE as an observation; test `a_round_path_spelled_another_way_is_not_demoted`.
- AC5 — met: `check_prescriptions.rs`: driven, applying exactly the prescribed edit gave verdict matched, an extra untracked file gave differs with unprescribed [extra.txt], an old text absent gave failed occurrences 0 and differs.
- AC11 — met: check-prescriptions takes only feature and base and reads the round's `result.json` (a file the brief names under `.ductus/review-rounds`/, outside the round's paths); driven, edits came from that file with no host argument.
- AC6 — met: start-fix-rounds returns budget 2 (`fix_rounds::RE_READ_BUDGET`); step 12 and reference Fix-round loop stop at that many re-reads-with-findings, report open findings and quote the runtime figures; budget-stop true derived by write-review from two finding re-reads (driven). The stop itself is host procedure, not a runtime refusal.
- AC7 — met: `read_re_read.rs`: driven, no result file gave state incomplete, complete:false gave incomplete, malformed JSON gave unparseable; none count in re-reads-with-findings, and step 12 leaves the round not re-read.
- AC8 — met: step 12 and reference Fix-round loop: a host that cannot start an agent prints the brief path for a fresh session whose `result.json` read-re-read reads; driven, a not-re-read round with a reason became a round-waivers entry with waived-by decided-by (write-review).
- AC12 — met: write-review --fix-rounds-json driven: record gained fix-rounds, re-reads, diff-checked, not-re-read, budget-stop, round-waivers, and the Summary rendered the account both generated and appended to a supplied --summary; a write without fix-rounds wrote none of the fields (unexamined, not zero); fix-rounds-open written with --loop-open; enumeration matches.
- AC13 — met: `check_review_gate.rs`: driven on a committed fixture, a record with not-re-read 1 and no waiver gave blocked-by fix-round-not-re-read, and the same round with a reasoned waiver moved past it to the next gate (not-analyzed); fix-round-loop-open also blocks.
- AC14 — met: `fix_rounds::re_reads_off`: driven, a 5-character reason, and a non-string value, each halted start-fix-rounds and brief-re-read naming the setting, value and file; a valid reason gave re-reads-off in the brief-re-read result with no snapshot or brief, and write-review waived the not-re-read round with that reason and the config file as waived-by; tests in `fix_rounds.rs` `a_short_reason_halts` etc.
- AC15 — met: driven: a missing feature, a bad feature name and an unknown base each exited 1 with an error, never an empty round; a missing result file gave could-not-run for check-prescriptions, and an unparseable one too; step 12 and reference record a failed call as not-re-read.
- AC9 — met: `AGENTS.md` Workflow bullet 'A review's fix rounds are re-read by /ductus:review's fix-round loop' points at `framework/reference/review.md` §Fix-round loop and restates none of it; no other re-read rule remains.
- AC16 — met: `brief_re_read.rs` `SEVERITY_BAR` states a trigger shown possible in the project for any severity and that a case shown only by building it is no finding; driven, the written `brief.md` carries it. Weak spot: the test asserts text.contains(`SEVERITY_BAR`), the constant itself, so removing the clause fails no test.
- AC17 — met: start-fix-rounds writes `start.toml` (started-at); driven, three briefed rounds gave rounds 1,2,3 and re-reads-with-findings 0,1,2 with elapsed-seconds from the runtime clock (`fix_rounds::loop_figures`); step 12 and the reference budget bullet quote rounds, re-reads-with-findings, elapsed-seconds and elapsed-absent-reason; test `three_rounds_are_counted_from_the_round_directory` (not run: no cargo).

## Observations

- convention: `plan.md`'s Affected Files is stale for AC17: `fix_rounds.rs` is listed as 'Round directory, self-ignore, the re-reads-off reader, `result.json` parsing' though it now records `start.toml` and counts the loop's figures (`record_start`, `loop_figures`); `line_ending_discipline.rs` is listed as '`brief_re_read.rs` exempt' though `fix_rounds.rs` joined its EXEMPT list in ef4ff965; `schema/descriptions.rs`, edited by the commit, is not listed. — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — **fixed** (decided by the rule)
- convention: `data-model.md`'s start-fix-rounds Errors list enumerates the failures (feature screening, missing directory, malformed config, a round directory that cannot be cleared, no work tree, libgit2) but omits the new one, `start.toml`'s create-only write (PrimitiveError::Io), which start-fix-rounds can now raise before it snapshots. — `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md` — **fixed** (decided by the rule)
- bug: an open re-read MUST or SHOULD left at a stop is an observation, and undispositioned it never holds done — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — **waived**: Option B (owner, 2026-11-04): low confidence; the loop asks the operator to defer or waive each open finding at a stop, and a design change would cost 078 another cycle.
- convention: RecordFreshness::Undeterminable's doc names only a missing digest, where an unparseable review.md also answers it — `runtime/src/schema/primitives.rs` — **waived**: Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window.
- bug: the re-reads-off reason's 16-character minimum is measured before its control characters and runs of spaces are collapsed — `runtime/src/primitives/fix_rounds.rs` — **waived**: Contrived input; the check matches the data model's 'once trimmed'. (decided by the rule)
- other: the loop primitives accept a misspelled [review] key silently — `runtime/src/primitives/fix_rounds.rs` — **waived**: /review's step 2, discover-rule-files, reports an unknown [review] key (CFG-ENV-012), verified on a fixture: 'unknown config key: [review] re-read-off'. (decided by the rule)
- other: a fixer's uncommitted edit that another spec's commit sweeps in leaves the round as excluded and is never re-read here — `runtime/src/primitives/fix_rounds.rs` — **waived**: It follows this spec's Edge Case on other sessions: that spec's review reads it as its own change, and git worktree isolates sessions. (decided by the rule)
- other: fix rounds driven by mechanical NOTEs have no length bound — `framework/reference/review.md` — **waived**: The Behavior permits NOTE rounds and AC6's budget counts re-reads with a MUST or SHOULD; a bound on NOTE rounds is a requirement decision. (decided by the rule)
- convention: AC14's 'a switch without a valid reason halts the run', read alone, covers an empty re-reads-off, which now counts as absent — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **waived**: The data model's re-reads-off row, the durable contract the switch rests on, says an empty value counts as absent and so is no switch, as task 17 records (owner, 2026-11-09). (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md`
- `.ductus/reference/analyze.md`
- `.pi/prompts/ductus-analyze.md`
- `docs/analyze.md`
- `docs/developing-ductus.md`
- `docs/rules.md`
- `docs/shared-constitution.md`
- `framework/bootstrap/ductus.md`
- `framework/bootstrap/govern.md`
- `framework/commands/analyze.md`
- `framework/migrations/remove-legacy-constitutions-region.md`
- `framework/reference/analyze.md`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/primitives/discover_rule_files.rs`
- `runtime/src/primitives/resolve_anchor.rs`
- `runtime/src/primitives/rule_verdicts.rs`
- `runtime/src/primitives/write_analysis.rs`
- `scripts/analyze/check.py`
- `scripts/analyze/detect.py`
- `scripts/analyze/record.py`
- `scripts/analyze/step15-brief.md`
- `scripts/mutation/analysis-design-fix-067.py`
- `scripts/mutation/command-reference-split.py`
- `scripts/mutation/exec-scenario-anchors-067.py`
- `scripts/mutation/owner-reviews-067.py`
- `scripts/mutation/rule-files-left-to-analysis-020.py`
- `scripts/mutation/rule-verdicts-converge.py`
- `scripts/tests/test-analyze-scripts.py`
- `specs/020-code-review/plan.md`
- `specs/020-code-review/review.md`
- `specs/020-code-review/spec.md`
- `specs/020-code-review/tasks.md`
- `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/024-rule-loader/review.md`
- `specs/024-rule-loader/spec.md`
- `specs/034-performance-backend-rules/review.md`
- `specs/034-performance-backend-rules/spec.md`
- `specs/037-observability-backend-rules/plan.md`
- `specs/037-observability-backend-rules/review.md`
- `specs/037-observability-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/plan.md`
- `specs/038-concurrency-backend-rules/review.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/039-reliability-backend-rules/plan.md`
- `specs/039-reliability-backend-rules/review.md`
- `specs/039-reliability-backend-rules/spec.md`
- `specs/055-shared-constitution/plan.md`
- `specs/055-shared-constitution/tasks.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/068-api-backend-rules/review.md`
- `specs/068-api-backend-rules/spec.md`
- `specs/069-accessibility-frontend-rules/review.md`
- `specs/069-accessibility-frontend-rules/spec.md`
- `specs/070-performance-frontend-rules/review.md`
- `specs/070-performance-frontend-rules/spec.md`
- `specs/079-review-and-analysis-work-converges/analysis.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/082-rule-assessments-converge-across-runs/analysis.md`
- `specs/082-rule-assessments-converge-across-runs/data-model.md`
- `specs/082-rule-assessments-converge-across-runs/plan.md`
- `specs/082-rule-assessments-converge-across-runs/review.md`
- `specs/082-rule-assessments-converge-across-runs/spec.md`
- `specs/inbox.md`

## Unexamined governance

*None.*
