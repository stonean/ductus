---
spec: 073-decided-rule-departures-are-recorded
last-run: 2026-11-13T19:38:37Z
reviewed-against: 621a4cd6ebc16dcd73b5d4fcfcb56e7f5c0ae96f
diff-base: b7ee0b26dbd997cbce030aa8ecd78a19c9c3cde3
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 15
scope: 26
unread:
  - .claude/commands/ductus/review.md
  - .ductus/reference/review.md
  - .pi/prompts/ductus-review.md
  - runtime/CHANGELOG.md
  - runtime/src/primitives/write_review.rs
  - runtime/src/schema/primitives.rs
  - scripts/mutation/owner-reviews-067.py
  - scripts/mutation/process-decisions-results-bounded.py
  - scripts/mutation/reopen-measure.py
  - scripts/mutation/reviews-follow-what-changed.py
  - scripts/mutation/scope-attribution.py
excluded: 2
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: cc819aa341154309c3bf65ffb9ab3541b4bc938a8b70a51ff3baebe85e2b076f
blocking: false
dispositions:
  fixed: 4
  deferred: 0
  waived: 4
  undispositioned: 0
decisions:
  - key: "other — an unclosed trailing backtick in a departs: item yields a second rule ID — `runtime/src/primitives/departures.rs`"
    outcome: waived
    reason: The entry is malformed either way and waives nothing, so the finding it draws is accurate.
    decided-at: 2026-11-13T05:27:33Z
    decided-by: rule
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
  - key: "other — the design call (a departure entry decides over a deferral: a new `--defer` of an entry's violation is refused, a stored deferral is superseded and named in `superseded-deferrals`) matches the code and is stated as the design in plan.md's review section and in data-model.md's Review waiver marker, but spec.md carries no criterion, edge case or Resolved Question for it, and the owner has not approved it (tasks.md task 13 says so). Driven on a fixture: a new deferral at a departed rule exits 1 naming the rule and the entry's spec.md line; the same finding with a decision-key exits 0, is waived, and is named in superseded-deferrals. — `specs/073-decided-rule-departures-are-recorded/plan.md:60`"
    outcome: waived
    reason: Not a break of 073's contract (spec.md is silent, the code matches the plan). It is already routed to the owner at the done-approval, per tasks.md task 13; once the owner approves or reverses it, a Resolved Question in spec.md should record it.
    decided-at: 2026-11-13T18:30:11Z
    decided-by: rule
  - key: "other — task 13 is ticked while its Done-when ends 'and 073's next analysis records BE-PAYLOAD-001 addressed', a condition only the owed analysis can meet. analysis.md at HEAD is blocking: true with the gating deferral to task 13, and its digests of data-model.md, plan.md and tasks.md are stale against the files, so the record cannot read as a pass. Both states are named as owed in task 13's own paragraph. — `specs/073-decided-rule-departures-are-recorded/tasks.md:136`"
    outcome: waived
    reason: The unmet clause is recorded as owed in task 13's paragraph (the analysis run), and check-review-gate at HEAD blocks on review-owed, so the incomplete state is not indistinguishable from a complete one.
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

Window b7ee0b26..HEAD (base-source last-review), 26 scope entries, three commits naming 073:

- 242f1276, the analysis record with tasks 14 and 15 appended;
- 8f07204b, task 14, which bounds process-waivers' result through bounded_lists::place, with its consumers, tests and mutants;
- 621a4cd6, task 15, which re-anchors 21 stale mutation anchors and adds framework-checks.yml step (u).
The passes ran in three agents divided by behavior: the bound, the shipped text and CI step, and 073's artifacts with task 15's re-anchors. Each read all nine loaded rule files whole.
Examined 15, each read whole: process_waivers.rs, write_review/departure_waivers.rs, tests/mcp.rs, tests/walker.rs, waivers-bounded.py, tests/golden/review-basic.jsonl, schema/descriptions.rs, framework/commands/review.md, framework/reference/review.md, framework-checks.yml, waiver-notices-flatten.py, and 073's data-model.md, plan.md, tasks.md and analysis.md.
Unread 11, named in unread:
- the three generated mirrors, not opened; the generator was re-run with the tree unchanged but for the fix to their source;
- runtime/CHANGELOG.md, its 073 entries and the window's diff;
- write_review.rs, its one changed test;
- primitives.rs, the window's diff and ProcessWaiversResult;
- the three large mutation lists, only their re-anchored entries;
- process-decisions-results-bounded.py and reviews-follow-what-changed.py, read in part.
All 21 re-anchored mutants were run in a clone and killed.
Fixed in the run, before this record:
- the departure-expiry notice's flattening, which no test proved, now pinned by a test and waiver-notices-flatten.py M5 (all five killed);
- Scope Boundaries name step 8's lists file;
- plan.md's Affected Files name 020's plan.md;
- a redundant test line.

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

- other — the unit test `the_lists_past_the_read_size_name_a_file_holding_them_whole` re-sets `read_size(tmp.path(), "10")` at the head of its "each list alone" block; `tmp` already holds 10 from the preceding `held` block and is not read again before the later `"big"` write, so the call does nothing. — `runtime/src/primitives/process_waivers.rs:983` — **fixed** (decided by the rule)
- convention — `specs/066-audit-records-and-the-completion-gate/scenarios/process-waivers-file-lists.md` line 13 says `applied`, `expired` and `retained` "still carry one WaiverRef per anchor and the result shape does not change". Read now, the shape did change (form, four counts, lists-file, lists absent in the file form). The sentence compares against the pre-multi-path shape, which is how it was true when written; 066's data model carries the bound under **Size**. — `specs/066-audit-records-and-the-completion-gate/scenarios/process-waivers-file-lists.md:13` — **waived**: The sentence describes the multi-path change, not the present result; the present shape is stated in 066's data model Size paragraph and 073's data model. Editing a scenario is a meaningful edit under the spec lifecycle, which costs a reopen for no change in what the scenario requires. (decided by the rule)
- other — a malformed `[artifacts] read-size-bytes` now halts `process-waivers` (review step 8) whenever the result holds any list, where before the bound it returned. The empty result still reads no setting, so the halt depends on the record's content. Driven on the built binary: size "big" with waivers exits 1 with the InvalidReadSize message; the same config with no waivers exits 0 with the inline empty result. — `runtime/src/primitives/process_waivers.rs:83` — **waived**: Documented in PROCESS_WAIVERS, 073's data model and the CHANGELOG, as QUAL-DELEG-001's enumeration of the shared predicate; every bounded sibling (process-decisions, compute-review-scope) halts on a malformed read size the same way, and the unit test pins both directions. (decided by the rule)
- perf — `run` clones all four lists into a `lists` JSON value and `place` serializes the whole result, on every call, before it knows whether the file form is needed. — `runtime/src/primitives/process_waivers.rs:85` — **waived**: Identical to every bounded sibling's shape (process_decisions.rs run builds `lists` the same way); the lists grow with one spec's waivers, so the cost is bounded, and a different shape for this one primitive would be the divergence. (decided by the rule)
- convention — framework/commands/review.md Scope Boundaries says `Past the read size, steps 1, 2, 9, 11 and 12's primitives, dashboard under --all, and read-section write lists files under .ductus/results/`. Since task 14, step 8's `process-waivers` writes one too (driven on a fixture with read-size-bytes = 10: `.ductus/results/process-waivers/001-x/<sha>.json`), and step 8's own text now names that file form, so the Scope Boundaries list omits a writer. Task 14's sweep changed steps 8 and 11 but not this line; the same line was fixed once before for write-review (073 review record). — `framework/commands/review.md:39` — **fixed** (decided by the rule)
- other — CI step (u) is narrower than its comment's headline: `run.py --check` validates anchors only (each `old` occurs exactly once), so a mutation whose test `filter` names no test still passes it, and the step stops at the first failing list, so several stale lists surface one per push (six lists were stale at once in task 15). The comment states the first-failure behavior; the filter gap is read from run.py (`--check` returns before cargo), not driven, since no cargo or mutation run was allowed in the shared tree. Also, (u) is the one step in this workflow whose fail paths (stale anchor, no list) have no committed test, unlike (d), (k), (m), (n), (o), (p), (q), (s) and (t); the proof is recorded in tasks.md task 15 and was reproduced here from a copy (stale anchor exit 1, run.py-only directory exit 1 with the stderr message, empty directory exit 2, clean tree 56 lists exit 0). — `.github/workflows/framework-checks.yml:268` — **waived**: An observation about the pipeline's own machinery made while reviewing framework work: the step does what its comment says, fails loudly on every case it claims to catch (reproduced), and no 073 criterion requires a committed test or a filter check; the owner may route either through /groom. (decided by the rule)
- other — the departure-expiry notice in process_waivers::compute prints the stored rule, file and reason flattened (shown_rule, shown_file, shown_reason), but no test or mutant proves it: replacing them with the raw rule, file and reason there survives `cargo test --lib -- process_waivers write_review departure` (236 passed, run in a clone). waiver-notices-flatten.py M3 and M4 matched twice before task 15 because of this second notice (added by task 5); task 15 lengthened each to the final else-branch only, so M3's name ('the expired notice prints the stored values raw') now covers one of the two expired notices and the departure one has no mutant. — `runtime/src/primitives/process_waivers.rs:212` — **fixed** (decided by the rule)
- convention — plan.md's Affected Files table does not name specs/020-code-review/plan.md, which task 14 edited (8f07204b) to state the process-waivers bound and back-link 073, though it names 020's scenarios/waiver-expiry.md and 066's and 022's data models. The task 15 files (six scripts/mutation lists, framework-checks.yml step (u)) are also unnamed, but task 15 is a disposition of an out-of-spec finding, so they are outside the plan by design. — `specs/073-decided-rule-departures-are-recorded/plan.md:106` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `specs/020-code-review/plan.md` — 020-code-review, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 020-code-review, 066-audit-records-and-the-completion-gate

## Unexamined governance

*None.*
