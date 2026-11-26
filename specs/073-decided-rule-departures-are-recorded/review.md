---
spec: 073-decided-rule-departures-are-recorded
last-run: 2026-11-24T01:13:27Z
reviewed-against: 296ab36cebaa4e6466c7942a7a1daa62d4f9df79
diff-base: 0db66fcc2811a609f2b2801a0b8d55ef6b271876
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 21
scope: 35
unread:
  - .claude/commands/ductus/review.md
  - .ductus/reference/review.md
  - .pi/prompts/ductus-review.md
  - runtime/CHANGELOG.md
  - runtime/src/primitives/mod.rs
  - runtime/src/primitives/write_analysis.rs
  - runtime/src/primitives/write_review.rs
  - runtime/src/schema/descriptions.rs
  - runtime/src/schema/primitives.rs
  - runtime/tests/mcp.rs
  - scripts/mutation/owner-reviews-067.py
  - scripts/mutation/reopen-measure.py
  - scripts/mutation/scope-attribution.py
  - specs/020-code-review/data-model.md
excluded: 60
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 19bfe627fbe2d70113e46d131f22339ca4d0d7ca31c8341afab464e63c28bdd4
criteria-judged: 15
criteria-met: 15
criteria-verdicts:
  - label: AC1
    digest: 664f4f346b5337f8506351de726525e4ff5f98144140924bed632628061241dd
  - label: AC2
    digest: 533a8ffc74e82ddc75e030482e350291924da0eb8f23ec094bd6b632904a0634
  - label: AC3
    digest: 6aed0d7516b8b20c7185b8c72379f685b84e2cc3c8b16ecca473b09482453d23
  - label: AC4
    digest: 939dcbbae3a09ce2023117e29b76594d9722eab38b50fa909087b59ff13ba22d
  - label: AC5
    digest: b1bb06d7b526c2f59236685a0e974fe629ff583f39afd74546f28c0a88e10da0
  - label: AC6
    digest: d4b493a0089b103deb9434f489f39ecf168d3d248841b99a6914eea3abb6da39
  - label: AC7
    digest: 880425813658ebc432b66727f0c0988f53798031ccad98d8ca2325166f24db29
  - label: AC8
    digest: 3b0aeef6903050f4af58be7f1a6caf94bd898ea39d1c2a35803405a81e0fb4dd
  - label: AC9
    digest: e7ba252a90c100134e4dfa49b3a729228ea2a80fe1f50ef44d558236e8758954
  - label: AC10
    digest: cb0803d91c541fa772a27ecdcb3e33ef17fd3c3a29b18d8ede3728d3ec1101bd
  - label: AC11
    digest: 9e6d31fb1b44b83d8cd920cdb651f891d7c7fb86f35a4164fc20994d164e9fbe
  - label: AC12
    digest: 491684df1f7dda5bfd5673bcbb4812224bfc3248842fdaa65740f46f2e62de2e
  - label: AC13
    digest: 66cac3ba044ac4da5351b0d32906f52697e09234b68700c6c4d81e55a9952e87
  - label: AC14
    digest: 940db6f462ea5c04d7c9faee0be9c410fdc96c98d827a0e70d19f223fedab490
  - label: AC15
    digest: da94b66e679f4bb4888ce5b796eb346542098212f129095183fa595cb4f93748
blocking: false
dispositions:
  fixed: 11
  deferred: 0
  waived: 10
  undispositioned: 0
decisions:
  - key: other — a fix between a run's two writes leaves the first write's departure waiver stored until the next run — `runtime/src/primitives/write_review.rs`
    outcome: waived
    reason: The next run expires the anchor once its rule no longer fires there, as it expires any waiver, and no count reads it in between.
    decided-at: 2026-11-13T05:27:33Z
    decided-by: rule
  - key: other — the constitution's departure bullet names no home for the entry's grammar — `framework/constitution.md`
    outcome: waived
    reason: The grammar is check-departures' compiled description and the analyze reference's Departure entries section; the constitution states the principle, with 31 bytes left under its 50,000-byte ceiling.
    decided-at: 2026-11-13T05:27:33Z
    decided-by: rule
  - key: other — RuleFacts::load re-reads the rule files discover-rule-files already returns — `runtime/src/primitives/check_departures.rs`
    outcome: waived
    reason: check-departures is a standalone primitive whose callers pass only a feature; taking another primitive's output as an argument would couple two tools' interfaces to save one read of twelve files.
    decided-at: 2026-11-13T05:27:33Z
    decided-by: rule
  - key: "other — a #[cfg(test)] thread-local sits in rule_verdicts.rs' production source — `runtime/src/primitives/rule_verdicts.rs`"
    outcome: waived
    reason: It is test instrumentation compiled out of every non-test build, the crate's existing seam for counting git reads.
    decided-at: 2026-11-13T05:27:33Z
    decided-by: rule
  - key: other — result_sizes is an ignored measurement that asserts nothing, so task 12's test-and-mutant item rests on the unit test and departures-bounded.py's mutants — `runtime/tests/result_sizes.rs`
    outcome: waived
    reason: result_sizes is a measurement by design, ignored by cargo test and run by scripts/measure-result-sizes.sh; task 12's item is met by a_result_past_the_read_size_names_a_file_holding_every_list, which departures-bounded.py's M1 to M6 each turn red.
    decided-at: 2026-11-13T14:24:10Z
    decided-by: rule
  - key: convention — AGENTS.md's bullet on rule-judging agents states a measurement whose source, the agents' own reports, no committed artifact holds — `AGENTS.md`
    outcome: waived
    reason: The bullet states its method and unit, each agent's own report when asked and the characters of each Verification read; the session transcripts it came from are per-user state outside git, so nothing committed can be cited.
    decided-at: 2026-11-13T14:24:10Z
    decided-by: rule
  - key: other — result_sizes' check-departures row writes the same 300 entries into both corpora, so its Grows column reads no — `runtime/tests/result_sizes.rs`
    outcome: waived
    reason: The row measures the bound, which its Bounded and default-read-size columns show; a spec's departure entries grow with that spec, not with the corpus the Grows column compares.
    decided-at: 2026-11-13T14:24:10Z
    decided-by: rule
  - key: "other — `superseded-deferrals` has no consuming step: `framework/commands/review.md` step 11 reports `malformed-departures` (and now names its file form), but neither it nor `framework/reference/review.md` tells the host to report `superseded-deferrals` in either form, so task 13's 'every consumer reads the file form' holds for one of the two lists. The list was added by an earlier 073 task and the data model only requires the result to name it, so task 13 neither introduced nor widened the gap. — `framework/commands/review.md:134`"
    outcome: waived
    reason: Introduced by an earlier 073 task, outside this window, and 073's data model requires only that the result name each superseded deferral, which it does in both forms; an observation about the pipeline's own machinery made while reviewing framework work.
    decided-at: 2026-11-13T18:30:11Z
    decided-by: rule
  - key: other — `runtime/tests/result_sizes.rs` calls `write-review` on a corpus whose subject holds no departure entries (only `check-departures` has the `Prep::Departures` mode), so the ignored growth measurement cannot see `malformed-departures` or `superseded-deferrals` grow; the bound is proven by the unit test, the MCP test and M1 to M9 instead. — `runtime/tests/result_sizes.rs:218`
    outcome: waived
    reason: result_sizes.rs is the ignored measurement harness for 080's research.md, not a gate, and is outside 073's task 13 scope entries; the bound is proven by tests that fail when it is removed. An observation about the pipeline's own machinery made while reviewing framework work.
    decided-at: 2026-11-13T18:30:11Z
    decided-by: rule
  - key: "other — a malformed `[artifacts] read-size-bytes` halts `write-review` after `review.md` is already written: `bound_lists` runs after `write_atomic`, so the call returns an error naming the setting while the record is on disk. Driven on the built binary with `read-size-bytes = \"big\"` and 3 malformed entries. Newly reachable for a result whose only list is a departure list, which before task 13 never read the setting. The doc comment states the order ('The record is written already') and the error names the setting. — `runtime/src/primitives/write_review.rs:649`"
    outcome: waived
    reason: The write-then-bound order is the precedent `rewrite-spec-links` and the stale-paths bound already set and the doc comment states it; the error names the setting, so it cannot read as a pass, and the same malformed setting halts every other bounded primitive of the run. An observation about the pipeline's own machinery made while reviewing framework work.
    decided-at: 2026-11-13T18:30:11Z
    decided-by: rule
  - key: convention — step 11's new clause (`in the file form (`form` is `file`) the list is under that name in `lists-file`, and `malformed-departures-count` says how many`) and reference §Waivers' new sentence (`Past the read size both lists are absent from the result and held whole in `lists-file`, under their own names`) restate what WRITE_REVIEW states, where descriptions.rs's header says a command or a reference cites the primitive rather than restating it, and the command's Config entry for `[artifacts] read-size-bytes` already says a step's result moves its lists to `lists-file` `as each primitive's description states`; the same §Waivers paragraph points at `ductus write-review --help` for the derivation two lines later. — `framework/commands/review.md:134`
    outcome: waived
    reason: 073's data-model.md Size paragraph and task 13's second criterion name step 11 and §Waivers as the consumers that read the file form, so removing either copy edits 073's data model (a reopen) to save one clause each; the clause in step 11 also carries what the host does with the file (report one line per entry), which is policy and not in the description.
    decided-at: 2026-11-13T18:30:11Z
    decided-by: rule
  - key: other — step 11 tells the host to report `malformed-departures` but nothing tells it to report `superseded-deferrals`, in either form, though a superseded deferral is a stored person's decision pruned from `decisions:` with the result as its only trace. No acceptance criterion in 073's spec.md requires the report, and the list and the step wording predate this window (introduced in 073's review round 1, before diff-base 53c88c30). — `framework/commands/review.md:134`
    outcome: waived
    reason: "Outside 073's window and outside its acceptance criteria: the observation concerns the pipeline's own machinery and task 13 delivered what its criteria state (the file form read by step 11 for malformed-departures and by §Waivers for both lists); the owner may route it through /groom if the report is wanted."
    decided-at: 2026-11-13T18:30:11Z
    decided-by: rule
  - key: "convention — two window bullets pass §drift-prevention's population test as rules true for every project running the pipeline, so by AGENTS.md's own header they belong in the constitution: 80d71dcc's 'Recommend on the merits, never on the owner's remaining allowance' (no constitution sentence says it; §recommendations asks only for the computed quantity), and 35ec180f's added clause 'check every merit you claim for an option against a source read whole, cited beside the claim' (restates §grounding's 'A partial read is not a read' and §recommendations). 35ec180f also writes the incident ('On 2026-11-13 an option was put to the owner ... both were false') into the bullet, which §drift-prevention routes to its commit, and, unlike its siblings, carries no '(owner, date)' tag. framework/constitution.md is 49,979 of 50,000 bytes (wc -c), so moving either needs a same-size cut elsewhere; the bullets' 46K/19.7K/329 measurement in 455556a1 names no session or method, so it is not re-derivable. — `AGENTS.md:48`"
    outcome: waived
    reason: Routing a rule to the constitution and cutting an owner-directed bullet's text is the owner's call (the commits record the owner's 2026-11-13 decisions), and the constitution has 21 bytes of room, so a move needs a cut of its own. Surfaced for the owner; nothing in the window breaks a loaded rule.
    decided-at: 2026-11-13T18:30:11Z
    decided-by: rule
  - key: "convention — `specs/066-audit-records-and-the-completion-gate/scenarios/process-waivers-file-lists.md` line 13 says `applied`, `expired` and `retained` \"still carry one WaiverRef per anchor and the result shape does not change\". Read now, the shape did change (form, four counts, lists-file, lists absent in the file form). The sentence compares against the pre-multi-path shape, which is how it was true when written; 066's data model carries the bound under **Size**. — `specs/066-audit-records-and-the-completion-gate/scenarios/process-waivers-file-lists.md:13`"
    outcome: waived
    reason: The sentence describes the multi-path change, not the present result; the present shape is stated in 066's data model Size paragraph and 073's data model. Editing a scenario is a meaningful edit under the spec lifecycle, which costs a reopen for no change in what the scenario requires.
    decided-at: 2026-11-13T19:38:08Z
    decided-by: rule
  - key: "other — a malformed `[artifacts] read-size-bytes` now halts `process-waivers` (review step 8) whenever the result holds any list, where before the bound it returned. The empty result still reads no setting, so the halt depends on the record's content. Driven on the built binary: size \"big\" with waivers exits 1 with the InvalidReadSize message; the same config with no waivers exits 0 with the inline empty result. — `runtime/src/primitives/process_waivers.rs:83`"
    outcome: waived
    reason: Documented in PROCESS_WAIVERS, 073's data model and the CHANGELOG, as QUAL-DELEG-001's enumeration of the shared predicate; every bounded sibling (process-decisions, compute-review-scope) halts on a malformed read size the same way, and the unit test pins both directions.
    decided-at: 2026-11-13T19:38:08Z
    decided-by: rule
  - key: perf — `run` clones all four lists into a `lists` JSON value and `place` serializes the whole result, on every call, before it knows whether the file form is needed. — `runtime/src/primitives/process_waivers.rs:85`
    outcome: waived
    reason: Identical to every bounded sibling's shape (process_decisions.rs run builds `lists` the same way); the lists grow with one spec's waivers, so the cost is bounded, and a different shape for this one primitive would be the divergence.
    decided-at: 2026-11-13T19:38:08Z
    decided-by: rule
  - key: "other — CI step (u) is narrower than its comment's headline: `run.py --check` validates anchors only (each `old` occurs exactly once), so a mutation whose test `filter` names no test still passes it, and the step stops at the first failing list, so several stale lists surface one per push (six lists were stale at once in task 15). The comment states the first-failure behavior; the filter gap is read from run.py (`--check` returns before cargo), not driven, since no cargo or mutation run was allowed in the shared tree. Also, (u) is the one step in this workflow whose fail paths (stale anchor, no list) have no committed test, unlike (d), (k), (m), (n), (o), (p), (q), (s) and (t); the proof is recorded in tasks.md task 15 and was reproduced here from a copy (stale anchor exit 1, run.py-only directory exit 1 with the stderr message, empty directory exit 2, clean tree 56 lists exit 0). — `.github/workflows/framework-checks.yml:268`"
    outcome: waived
    reason: "An observation about the pipeline's own machinery made while reviewing framework work: the step does what its comment says, fails loudly on every case it claims to catch (reproduced), and no 073 criterion requires a committed test or a filter check; the owner may route either through /groom."
    decided-at: 2026-11-13T19:38:08Z
    decided-by: rule
---

# Review — 073-decided-rule-departures-are-recorded

## Summary

Window `0db66fcc..296ab36c` (`base-source: reopen`). Step 1 resolved 34 scope entries, and `write-review` counts 35: this run's uncommitted edit to `specs/020-code-review/data-model.md` joins them. The 34 hold 073's ten commits from its reopen (the amend adding AC9 to AC14, the clarify, the plan, tasks 16 to 22, two plan fixes), `40dea24c`, a commit naming no spec, for `AGENTS.md`, and the five mutation lists the last record left unread. Rule files loaded: `configuration-cross.md`, `quality-cross.md` and the three `security-*.md` files.

The passes ran in three agents divided by behavior, each reading the five rule files whole (232,962 bytes): the entry reader, `check-departures` and the analyze path; scoped waivers, their expiry and `departure-notices`; and the shipped text, `AGENTS.md` and 073's artifacts. Two more judged the criteria, AC1 to AC8 and AC9 to AC15, each sent back once to read the code its criteria name rather than the tests alone.

Examined 21, each read whole: `AGENTS.md`, `docs/rules.md`, `framework/commands/review.md`, `framework/reference/review.md`, `framework/templates/spec/spec.md`, `departures.rs`, `check_departures.rs`, `process_waivers.rs`, `write_review/departure_waivers.rs`, `runtime/tests/golden/review-basic.jsonl`, the mutation lists `departures.py`, `departure-waivers.py`, `review-departures-bounded.py`, `waiver-notices-flatten.py`, `freshness-results-have-a-bounded-size.py`, `process-decisions-results-bounded.py` and `reviews-follow-what-changed.py`, and 073's `spec.md`, `plan.md`, `data-model.md` and `tasks.md`.

Unread 14, named in `unread`:

- `specs/020-code-review/data-model.md`, this run's fix to its waiver table, its other rows not read;
- the three generated mirrors, not opened: `scripts/gen-claude-commands.sh --check` exited 0 on a clean clone of `296ab36c`, and the generator was re-run after this run's edits to their sources;
- `runtime/CHANGELOG.md`, its window diff alone;
- `runtime/src/primitives/write_review.rs`, 073's hunks and the departure functions whole (55,554 of 346,103 bytes);
- `runtime/src/primitives/mod.rs`, `write_analysis.rs`, `runtime/src/schema/primitives.rs` and `runtime/tests/mcp.rs`, each its window diff and the item it changes (`inline_code_spans`, the AC15 test, `WriteReviewResult`, the bounded-form test);
- `runtime/src/schema/descriptions.rs`, the `CHECK_DEPARTURES`, `WRITE_REVIEW` and `PROCESS_WAIVERS` descriptions;
- `scripts/mutation/owner-reviews-067.py`, `reopen-measure.py` and `scope-attribution.py`, not opened: none changed for 073 in the window, and step (u) of `framework-checks` passed on their anchors at `296ab36c`.

Fixed in the run, before this record:

- `BE-INPUT-004` at `lost_coverage`: a covered file is contained with `leaves_project` before it is probed, and one leaving the project gets its own notice;
- AC13, judged not met because `write-review` drove one malformed `covers:` form, now drives seven, and was judged again;
- the `departs:` rule ID is read as a code span (`inline_code_spans`), a contract break;
- the notice for an uncovered site no longer claims the finding counts;
- tests for the scope clause and the outward link, with mutants M40, M41 and M63 run and killed;
- the data model's `tier` row and Size formulas, the plan's release and lost-coverage sentences, `spec.md` §Scoped entries, 020's waiver table, two schema doc comments and a test comment.

`cargo fmt`, `cargo clippy --all-targets -- -D warnings` and `cargo test` (3,847 passed, 0 failed) pass on the fixed tree; `review-basic` was re-blessed, its fixture's sha alone. AC6's "same change as the behavior" was judged as 073's delivery of the behavior, not one commit.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

1 of 40 commits in the window no spec claimed.

15 of 15 acceptance criteria judged met.

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

- AC1 — met: `runtime/src/primitives/departures.rs` parse, start_entry, read_child and finish and `runtime/src/primitives/check_departures.rs` judge_rule and judge_citations; driven on the built binary over a fixture: no reason, an unknown rule and a rule two entries name are Blocking, a deprecated rule Advisory, and an unresolved cited anchor is reported by resolve-anchor at analyze step 4 (advisory); unit tests an_unknown_rule_is_blocking_and_a_known_one_is_not and a_rule_two_entries_name_is_blocking_on_the_later_entry pass
- AC2 — met: `runtime/src/primitives/write_analysis.rs` derive_rule_fields and check_new_verdicts with `runtime/src/primitives/rule_verdicts.rs` fill_waived fill reason, decided-by, decided-at and cited from the entry and refuse a waiver with no entry or a finding against one; a_fired_must_rule_with_an_entry_is_waived_and_blocks_nothing (blocking-findings 0) and a_waiver_without_an_entry_and_a_finding_against_one_are_refused_with_no_record pass
- AC3 — met: `runtime/src/primitives/write_analysis.rs` render_waived writes `## Waived rules`, MUST first, apart from passing rules, the result carries waived and waived-count, and step 22 of `framework/commands/analyze.md` reports them; a_waived_rule_and_a_satisfied_one_differ_in_the_record_and_the_result and waived_rules_list_must_first_then_the_rest_each_in_id_order pass
- AC4 — met: `runtime/src/primitives/write_analysis.rs` derive_rule_fields carries a current fired Waived verdict while the rule fires and stores none once it does not; a_waiver_carries_forward_while_its_rule_fires_and_is_gone_when_it_does_not passes
- AC5 — met: `runtime/src/primitives/write_review.rs` departure_waiver_entries anchors an unscoped entry's rule at each in-scope file it fires at, stored as a departure waiver's file list in `review.md` waivers; a_must_departure_waives_the_rule_at_both_files_and_writes_each_anchor, a_departure_waiver_covers_every_site_in_its_file and a_violation_outside_the_review_scope_is_not_waived_and_still_counts in `runtime/src/primitives/write_review/departure_waivers.rs` pass
- AC6 — met: `framework/constitution.md` Finding dispositions carries 'A departure entry waives its rule before any finding exists' (073 task 7) and the analysis-record row lists rule-verdicts as addressed, finding or waived (073's first review fix round, d15a0188), both part of 073's delivery of the behavior before it first reached done, not one commit
- AC7 — met: `runtime/src/primitives/departures.rs` attribute, introducing_commits and describe give the author email and UTC author time of the oldest commit of the unbroken first-parent run holding the entry, an edit keeping it, and Unattributed for an uncommitted entry or a shallow repository; `scripts/tests/test-departure-attribution.sh` passed 10 of 10 cases, run by step (t) of `.github/workflows/framework-checks.yml` in its fetch-depth 0 job
- AC8 — met: `runtime/src/primitives/write_review.rs` departure_waiver_entries matches by rule and scope with no tier test; a_should_departure_leaves_no_should_violation in `runtime/src/primitives/write_review/departure_waivers.rs` passes (should-violations 0, waived 1, departure waiver stored)
- AC9 — met: the pre-window tests of `runtime/src/primitives/write_review/departure_waivers.rs` (header 'Tests (spec 073 AC5, AC8)'), among them a_must_departure_waives_the_rule_at_both_files_and_writes_each_anchor and a_should_departure_leaves_no_should_violation, are unedited across the window and this run (the window changed one doc comment and appended) and pass; the unscoped path of departure_waiver_entries waives every in-scope file
- AC10 — met: `runtime/src/primitives/write_review.rs` entry_decides decides a finding only at a covered file and site, so another site, a no-site finding and another file count by tier, and `runtime/src/primitives/process_waivers.rs` read_anchors, Kind::DepartureSite and place apply a site anchor at its site alone; a_scoped_entry_waives_its_rule_at_its_sites_only asserts waived 1 and must-violations 3, on a first run and on one finding the waiver stored
- AC11 — met: `runtime/src/primitives/write_review.rs` superseded_deferrals runs only over the findings entry_decides covers, so a new deferral at a covered site is refused naming the spec.md line and a stored one superseded, and one at another site stands; a_scoped_entry_refuses_a_new_deferral_only_at_a_site_it_covers and a_scoped_entry_supersedes_a_stored_deferral_only_at_a_site_it_covers pass
- AC12 — met: `runtime/src/primitives/write_review.rs` lost_coverage gives a notice naming the entry's spec.md line and covered sites for a covered file not in the tree, one per uncovered or no-site firing in a covered in-scope file, and none for a covered site where nothing fires; a_covered_file_not_in_the_tree_is_a_notice_naming_the_entry_and_its_sites, the_rule_firing_at_an_uncovered_site_of_a_covered_file_is_a_notice_naming_both_sites and a_covered_site_where_the_rule_does_not_fire_is_no_notice pass
- AC13 — met: `runtime/src/primitives/check_departures.rs` each_malformed_covers_is_blocking_and_keeps_its_entry_out drives the six forms as Blocking with no entry, and a_malformed_covers_waives_nothing_and_the_run_names_it in `runtime/src/primitives/write_review/departure_waivers.rs` now drives write-review over seven inputs (not a list, empty, blank file, blank site, repeated, absolute, a `..` segment), each named in malformed-departures with no waiver stored (extended in this review's run)
- AC14 — met: covers: is stated in `specs/073-decided-rule-departures-are-recorded/data-model.md` (Departure entry, Review waiver marker), the Applicable Rules comment of `framework/templates/spec/spec.md` and `ductus check-departures --help` (run), and a scoped entry's waiver in `specs/020-code-review/scenarios/waiver-expiry.md` rule 5, `framework/reference/review.md` §Waivers and the departure paragraph of `docs/rules.md`
- AC15 — met: `runtime/src/primitives/rule_verdicts.rs` fill_waived reads the entry by rule and never its covers, so a scoped entry's verdict is an unscoped one's; a_scoped_entry_waives_its_rule_as_an_unscoped_one_does in `runtime/src/primitives/write_analysis.rs` asserts the Waived verdict with reason, decided-by, decided-at and citation and blocking-findings 0

## Observations

- bug — a `departs:` item's rule ID was found by splitting the line on backticks, not with `inline_code_spans` as a `covers:` item's spans are, so a lone unclosed backtick named a rule the entry then waived and a rule ID in a double-backtick span named none, against the data model's one backticked rule ID and AC1's finding for an entry missing one — `runtime/src/primitives/departures.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- convention — the `check-departures` result row of `specs/073-decided-rule-departures-are-recorded/data-model.md` said `tier` is absent when the rule is not loaded, though `RuleFacts::load` also leaves it absent for a loaded rule whose Statement carries no MUST or SHOULD, as `CHECK_DEPARTURES` states — `specs/073-decided-rule-departures-are-recorded/data-model.md` — **fixed** (decided by the rule)
- convention — no test held the scope clause of `lost_coverage` that 073's Edge Cases require (a covered file the tree holds but the run's scope does not is no notice): removing `departures.in_scope(args, file)` left every test passing — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)
- bug — a `departure-notices` notice for a finding at a site a scoped entry does not cover said `so the finding counts` even when a hand `--waive` had waived that finding, and step 11 of `framework/commands/review.md` said the same — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)
- convention — the `WriteReviewResult` doc comments of `form` and `lists_file` in `runtime/src/schema/primitives.rs`, compiled into the result schema, named the bounded lists without `departure-notices` — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- other — the `process-waivers` and `write-review` **Size** paragraphs of `specs/073-decided-rule-departures-are-recorded/data-model.md` added the feature directory's name to a base figure that already held it, and `write-review`'s result names it three times; re-derived on the built binary as 206 bytes plus the name and 591 bytes plus three times the name — `specs/073-decided-rule-departures-are-recorded/data-model.md` — **fixed** (decided by the rule)
- convention — the comment above `REVIEW_READ_SIZE` in `runtime/tests/mcp.rs` gave the `write-review` file form's fixed fields as about 550 bytes, where the CLI prints 608 — `runtime/tests/mcp.rs` — **fixed** (decided by the rule)
- convention — the `BE-COMPAT-001` paragraph of `specs/073-decided-rule-departures-are-recorded/plan.md` said the departure surfaces ship with the pending minor bump, though `ductus-v0.60.0` (2026-11-20) shipped them and only `departure-notices` is unreleased — `specs/073-decided-rule-departures-are-recorded/plan.md` — **fixed** (decided by the rule)
- convention — the descriptions of `departure-notices` were looser than `lost_coverage`: the plan said one notice per in-scope finding where a repeated one is given once, and §Waivers of `framework/reference/review.md`, step 11 of `framework/commands/review.md`, `docs/rules.md` and the CHANGELOG left out the finding that names no site — `framework/reference/review.md` — **fixed** (decided by the rule)
- convention — §Scoped entries of `specs/073-decided-rule-departures-are-recorded/spec.md` listed the malformed `covers:` forms without an item that is not two code spans joined by `in` (`ProblemKind::CoversItem`), which the data model and plan name; AC13's list stays true as a subset of what `check-departures` reports — `specs/073-decided-rule-departures-are-recorded/spec.md` — **fixed** (decided by the rule)
- convention — the waiver table of `specs/020-code-review/data-model.md` described `anchors` and `file` without a scoped departure waiver, whose anchors carry no `covers` and which holds no `file` — `specs/020-code-review/data-model.md` — **fixed** (decided by the rule)
- other — a fix between a run's two writes leaves the first write's departure waiver stored until the next run — `runtime/src/primitives/write_review.rs` — **waived**: The next run expires the anchor once its rule no longer fires there, as it expires any waiver, and no count reads it in between. (decided by the rule)
- other — RuleFacts::load re-reads the rule files discover-rule-files already returns — `runtime/src/primitives/check_departures.rs` — **waived**: check-departures is a standalone primitive whose callers pass only a feature; taking another primitive's output as an argument would couple two tools' interfaces to save one read of twelve files. (decided by the rule)
- convention — AGENTS.md's bullet on rule-judging agents states a measurement whose source, the agents' own reports, no committed artifact holds — `AGENTS.md` — **waived**: The bullet states its method and unit, each agent's own report when asked and the characters of each Verification read; the session transcripts it came from are per-user state outside git, so nothing committed can be cited. (decided by the rule)
- other — `superseded-deferrals` has no consuming step: `framework/commands/review.md` step 11 reports `malformed-departures` (and now names its file form), but neither it nor `framework/reference/review.md` tells the host to report `superseded-deferrals` in either form, so task 13's 'every consumer reads the file form' holds for one of the two lists. The list was added by an earlier 073 task and the data model only requires the result to name it, so task 13 neither introduced nor widened the gap. — `framework/commands/review.md` — **waived**: Introduced by an earlier 073 task, outside this window, and 073's data model requires only that the result name each superseded deferral, which it does in both forms; an observation about the pipeline's own machinery made while reviewing framework work. (decided by the rule)
- other — a malformed `[artifacts] read-size-bytes` halts `write-review` after `review.md` is already written: `bound_lists` runs after `write_atomic`, so the call returns an error naming the setting while the record is on disk. Driven on the built binary with `read-size-bytes = "big"` and 3 malformed entries. Newly reachable for a result whose only list is a departure list, which before task 13 never read the setting. The doc comment states the order ('The record is written already') and the error names the setting. — `runtime/src/primitives/write_review.rs` — **waived**: The write-then-bound order is the precedent `rewrite-spec-links` and the stale-paths bound already set and the doc comment states it; the error names the setting, so it cannot read as a pass, and the same malformed setting halts every other bounded primitive of the run. An observation about the pipeline's own machinery made while reviewing framework work. (decided by the rule)
- convention — step 11's file-form clauses for the departure lists, and the sentence on them in §Waivers of `framework/reference/review.md`, restate what `WRITE_REVIEW` states, where the header of `runtime/src/schema/descriptions.rs` says a command cites the primitive rather than restating it — `framework/commands/review.md` — **waived**: 073's data-model.md Size paragraph and task 13's second criterion name step 11 and §Waivers as the consumers that read the file form, so removing either copy edits 073's data model (a reopen) to save one clause each; the clause in step 11 also carries what the host does with the file (report one line per entry), which is policy and not in the description. (decided by the rule)
- other — step 11 tells the host to report `malformed-departures` but nothing tells it to report `superseded-deferrals`, in either form, though a superseded deferral is a stored person's decision pruned from `decisions:` with the result as its only trace. No acceptance criterion in 073's spec.md requires the report, and the list and the step wording predate this window (introduced in 073's review round 1, before diff-base 53c88c30). — `framework/commands/review.md` — **waived**: Outside 073's window and outside its acceptance criteria: the observation concerns the pipeline's own machinery and task 13 delivered what its criteria state (the file form read by step 11 for malformed-departures and by §Waivers for both lists); the owner may route it through /groom if the report is wanted. (decided by the rule)
- convention — two window bullets pass §drift-prevention's population test as rules true for every project running the pipeline, so by AGENTS.md's own header they belong in the constitution: 80d71dcc's 'Recommend on the merits, never on the owner's remaining allowance' (no constitution sentence says it; §recommendations asks only for the computed quantity), and 35ec180f's added clause 'check every merit you claim for an option against a source read whole, cited beside the claim' (restates §grounding's 'A partial read is not a read' and §recommendations). 35ec180f also writes the incident ('On 2026-11-13 an option was put to the owner ... both were false') into the bullet, which §drift-prevention routes to its commit, and, unlike its siblings, carries no '(owner, date)' tag. framework/constitution.md is 49,979 of 50,000 bytes (wc -c), so moving either needs a same-size cut elsewhere; the bullets' 46K/19.7K/329 measurement in 455556a1 names no session or method, so it is not re-derivable. — `AGENTS.md` — **waived**: Routing a rule to the constitution and cutting an owner-directed bullet's text is the owner's call (the commits record the owner's 2026-11-13 decisions), and the constitution has 21 bytes of room, so a move needs a cut of its own. Surfaced for the owner; nothing in the window breaks a loaded rule. (decided by the rule)
- other — a malformed `[artifacts] read-size-bytes` now halts `process-waivers` (review step 8) whenever the result holds any list, where before the bound it returned. The empty result still reads no setting, so the halt depends on the record's content. Driven on the built binary: size "big" with waivers exits 1 with the InvalidReadSize message; the same config with no waivers exits 0 with the inline empty result. — `runtime/src/primitives/process_waivers.rs` — **waived**: Documented in PROCESS_WAIVERS, 073's data model and the CHANGELOG, as QUAL-DELEG-001's enumeration of the shared predicate; every bounded sibling (process-decisions, compute-review-scope) halts on a malformed read size the same way, and the unit test pins both directions. (decided by the rule)
- perf — `run` clones all four lists into a `lists` JSON value and `place` serializes the whole result, on every call, before it knows whether the file form is needed. — `runtime/src/primitives/process_waivers.rs` — **waived**: Identical to every bounded sibling's shape (process_decisions.rs run builds `lists` the same way); the lists grow with one spec's waivers, so the cost is bounded, and a different shape for this one primitive would be the divergence. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/groom.md`
- `.claude/commands/ductus/implement.md`
- `.claude/commands/ductus/specify.md`
- `.ductus/reference/groom.md`
- `.ductus/reference/implement.md`
- `.pi/prompts/ductus-groom.md`
- `.pi/prompts/ductus-implement.md`
- `.pi/prompts/ductus-specify.md`
- `.ductus/docs/agents/developing-ductus.md`
- `framework/bootstrap/ductus.md`
- `framework/bootstrap/govern.md`
- `framework/commands/groom.md`
- `framework/commands/implement.md`
- `framework/commands/specify.md`
- `framework/constitution.md`
- `framework/reference/groom.md`
- `framework/reference/implement.md`
- `runtime/src/interpreter/payload.rs`
- `runtime/src/primitives/check_artifacts.rs`
- `runtime/src/primitives/compute_review_scope.rs`
- `runtime/src/primitives/derive_routing_candidates.rs`
- `runtime/src/schema/extensions.rs`
- `runtime/tests/golden/implement-basic.jsonl`
- `runtime/tests/golden/specify-basic.jsonl`
- `scripts/audit/archive-contents.sh`
- `scripts/mutation/commit-counts.py`
- `scripts/mutation/routing-path-claims.py`
- `specs/010-agent-autonomy/plan.md`
- `specs/015-tarball-fetch/analysis.md`
- `specs/015-tarball-fetch/plan.md`
- `specs/015-tarball-fetch/review.md`
- `specs/015-tarball-fetch/spec.md`
- `specs/017-derive-dont-ask/scenarios/a-path-list-commit-leaves-the-shared-index-current.md`
- `specs/017-derive-dont-ask/spec.md`
- `specs/017-derive-dont-ask/tasks.md`
- `specs/020-code-review/plan.md`
- `specs/020-code-review/scenarios/waiver-expiry.md`
- `specs/020-code-review/spec.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/scenarios/routing-ranks-by-claimed-path.md`
- `specs/022-deterministic-runtime/scenarios/specify-routes-before-scaffolding.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/022-deterministic-runtime/tasks.md`
- `specs/050-constitution/analysis.md`
- `specs/050-constitution/plan.md`
- `specs/050-constitution/review.md`
- `specs/050-constitution/spec.md`
- `specs/050-constitution/tasks.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/plan.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/process-waivers-file-lists.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/079-review-and-analysis-work-converges/tasks.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/plan.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/spec.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/tasks.md`

## Unexamined governance

*None.*
