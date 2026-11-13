---
spec: 073-decided-rule-departures-are-recorded
last-run: 2026-11-13T18:31:20Z
reviewed-against: 3eeaf635ad0c3d440669b2bac2de72303256ac12
diff-base: 53c88c30633ae2ffc6c219c85e4f2944e1c1c43f
must-violations: 0
should-violations: 0
low-confidence: 1
examined: 13
scope: 21
unread:
  - .claude/commands/ductus/review.md
  - .ductus/reference/review.md
  - .pi/prompts/ductus-review.md
  - runtime/CHANGELOG.md
  - runtime/src/primitives/write_review.rs
  - runtime/src/schema/primitives.rs
  - specs/066-audit-records-and-the-completion-gate/plan.md
  - specs/066-audit-records-and-the-completion-gate/tasks.md
excluded: 48
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 65d2a75b18511e7879ced65bf4b30d5ec2991607da5ee997a6e34aa8bfa4bad5
blocking: false
dispositions:
  fixed: 11
  deferred: 0
  waived: 8
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
---

# Review — 073-decided-rule-departures-are-recorded

## Summary

Window 53c88c30..HEAD (base-source last-review), 21 scope entries. 073's change in it is task 13's code, e6dc9fdc, with 57a3953e's correction and 073's task and record commits. Three AGENTS.md commits name no spec: 455556a1, 80d71dcc and 35ec180f. The passes ran in three agents divided by behavior: task 13's bound on write-review's departure lists; the shipped text that states it; and 073's artifacts with AGENTS.md. Each agent read all nine loaded rule files whole.
Examined 13, each read whole: AGENTS.md, framework/commands/review.md, framework/reference/review.md, write_review/departure_waivers.rs, schema/descriptions.rs, tests/golden/review-basic.jsonl, tests/mcp.rs, the two mutation lists, and 073's analysis.md, data-model.md, plan.md and tasks.md.
Unread 8, named in unread:

- the three generated mirrors, not opened; scripts/gen-claude-commands.sh was re-run and the tree changed only by the fix to their source;
- runtime/CHANGELOG.md, its 073 entries and e6dc9fdc's diff;
- runtime/src/primitives/write_review.rs, e6dc9fdc's diff, run and bound_lists whole;
- runtime/src/schema/primitives.rs, e6dc9fdc's diff and WriteReviewResult whole;
- 066's plan.md and tasks.md, this run's own corrections, outside the passes.
Fixed in the run, before this record: a SHOULD, QUAL-TEST-001 at departure_waivers.rs. A mutant measuring the lists in place of the whole result survived every test. A test at the read size and one byte under now kills it as M10, and all 10 mutants of review-departures-bounded.py were killed. Eleven observations were fixed too.
The low-confidence BE-PAYLOAD-001 on process-waivers' result is the analysis's to judge against plan.md's stance that those lists are 020's.
The design call the owner has not approved is recorded in plan.md: a departure entry decides over a deferral of its rule. It is put to the owner at done.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: BE-PAYLOAD-001 — plan.md's review section leaves process-waivers' applied/expired/retained/notices unbounded while a departure waiver anchors every in-scope file where its rule fires

- **File**: `specs/073-decided-rule-departures-are-recorded/plan.md:60`
- **Rule**: Response payloads MUST have a bounded maximum size. Endpoints that return collections able to exceed a small fixed maximum MUST paginate per `BE-PAGE` (api-backend.md) rather than return an unbounded list in a single response.
- **Finding**: The section ends by naming process-waivers' result as 020's and unbounded 'and unchanged in kind by 073'. ProcessWaiversResult (runtime/src/schema/primitives.rs) carries no form or lists-file, and `applied` holds one anchor per (rule, file) pair. 073's departure waiver is stored for every in-scope file the rule fires at (departure_waiver_entries), so a departure grows `applied`, `expired` and `retained` with the scope, and its expiry adds `notices`. The plan commits to no maximum size for them. Low confidence because the unbounded result and its stated owner (020) predate 073 and the earlier analysis accepted the plan's stance; whether a design that enlarges a pre-existing unbounded list owes the bound is the open call.
- **Auto-fixable**: no

## Waived findings

*None.*

## Deferred findings

*None.*

## Observations

- other — the new MCP test repeats the assertions of the `lists_in_file` helper (form is file, the listed keys absent, result within the read size, then the list file read) because the helper hardcodes `SMALL_READ_SIZE` (400) and the write-review file form is about 550 bytes. Parameterizing the helper by read size lets the test call it, as the sibling bounded tests do. — `runtime/tests/mcp.rs:2020` — **fixed** (decided by the rule)
- other — `superseded-deferrals` has no consuming step: `framework/commands/review.md` step 11 reports `malformed-departures` (and now names its file form), but neither it nor `framework/reference/review.md` tells the host to report `superseded-deferrals` in either form, so task 13's 'every consumer reads the file form' holds for one of the two lists. The list was added by an earlier 073 task and the data model only requires the result to name it, so task 13 neither introduced nor widened the gap. — `framework/commands/review.md:134` — **waived**: Introduced by an earlier 073 task, outside this window, and 073's data model requires only that the result name each superseded deferral, which it does in both forms; an observation about the pipeline's own machinery made while reviewing framework work. (decided by the rule)
- other — `runtime/tests/result_sizes.rs` calls `write-review` on a corpus whose subject holds no departure entries (only `check-departures` has the `Prep::Departures` mode), so the ignored growth measurement cannot see `malformed-departures` or `superseded-deferrals` grow; the bound is proven by the unit test, the MCP test and M1 to M9 instead. — `runtime/tests/result_sizes.rs:218` — **waived**: result_sizes.rs is the ignored measurement harness for 080's research.md, not a gate, and is outside 073's task 13 scope entries; the bound is proven by tests that fail when it is removed. An observation about the pipeline's own machinery made while reviewing framework work. (decided by the rule)
- other — a malformed `[artifacts] read-size-bytes` halts `write-review` after `review.md` is already written: `bound_lists` runs after `write_atomic`, so the call returns an error naming the setting while the record is on disk. Driven on the built binary with `read-size-bytes = "big"` and 3 malformed entries. Newly reachable for a result whose only list is a departure list, which before task 13 never read the setting. The doc comment states the order ('The record is written already') and the error names the setting. — `runtime/src/primitives/write_review.rs:649` — **waived**: The write-then-bound order is the precedent `rewrite-spec-links` and the stale-paths bound already set and the doc comment states it; the error names the setting, so it cannot read as a pass, and the same malformed setting halts every other bounded primitive of the run. An observation about the pipeline's own machinery made while reviewing framework work. (decided by the rule)
- convention — 066's plan.md still states the pre-task-13 shape of write-review's result: line 33 says write-review's result is otherwise scalars, and line 36 says analyze-freshness is its only list so its file holds analyze-stale-paths alone and that a result with no stale path reads no setting. After e6dc9fdc the result carries two more lists and one file holds all three keys; 066's tasks.md line 184 also names bound_stale_paths, renamed bound_lists in the same commit (no other reference to the old name remains). — `specs/066-audit-records-and-the-completion-gate/plan.md:33` — **fixed** (decided by the rule)
- other — WRITE_REVIEW's description and the CHANGELOG entry do not say that a malformed `[artifacts] read-size-bytes` now makes write-review fail after review.md is written whenever a departure list is non-empty. Task 13 widened the trigger from a stale analysis's paths to the two departure lists, so a spec with one malformed entry plus a bad setting gets exit 1 and no result, and the malformed-departures line the host should report is lost. Driven on a fixture: exit 1, `invalid [artifacts] read-size-bytes ...`, specs/001-x/review.md on disk, empty stdout. WRITE_ANALYSIS states the same fact for its own call (`A read size that does not parse fails the call after the record is written`); 066's plan.md records it for write-review. — `runtime/src/schema/descriptions.rs:325` — **fixed** (decided by the rule)
- convention — framework/commands/review.md Scope Boundaries says `Past the read size, steps 1, 2, 9 and 12's primitives,`dashboard` under `--all`, and`read-section` write lists files under `.ductus/results/`. No other files are modified`. write-review at step 11 writes one too (verified on a fixture: .ductus/results/write-review/001-x/<sha>.json), and under `ductus exec` step 12 is a no-op, so step 11 is then the only writer and is not named. Step 12's final write-review covers it only when the loop runs. — `framework/commands/review.md:39` — **fixed** (decided by the rule)
- convention — step 11's new clause (`in the file form (`form` is `file`) the list is under that name in`lists-file`, and`malformed-departures-count`says how many`) and reference §Waivers' new sentence (`Past the read size both lists are absent from the result and held whole in`lists-file`, under their own names`) restate what WRITE_REVIEW states, where descriptions.rs's header says a command or a reference cites the primitive rather than restating it, and the command's Config entry for `[artifacts] read-size-bytes` already says a step's result moves its lists to `lists-file` `as each primitive's description states`; the same §Waivers paragraph points at `ductus write-review --help` for the derivation two lines later. — `framework/commands/review.md:134` — **waived**: 073's data-model.md Size paragraph and task 13's second criterion name step 11 and §Waivers as the consumers that read the file form, so removing either copy edits 073's data model (a reopen) to save one clause each; the clause in step 11 also carries what the host does with the file (report one line per entry), which is policy and not in the description. (decided by the rule)
- other — step 11 tells the host to report `malformed-departures` but nothing tells it to report `superseded-deferrals`, in either form, though a superseded deferral is a stored person's decision pruned from `decisions:` with the result as its only trace. No acceptance criterion in 073's spec.md requires the report, and the list and the step wording predate this window (introduced in 073's review round 1, before diff-base 53c88c30). — `framework/commands/review.md:134` — **waived**: Outside 073's window and outside its acceptance criteria: the observation concerns the pipeline's own machinery and task 13 delivered what its criteria state (the file form read by step 11 for malformed-departures and by §Waivers for both lists); the owner may route it through /groom if the report is wanted. (decided by the rule)
- other — the design call (a departure entry decides over a deferral: a new `--defer` of an entry's violation is refused, a stored deferral is superseded and named in `superseded-deferrals`) matches the code and is stated as the design in plan.md's review section and in data-model.md's Review waiver marker, but spec.md carries no criterion, edge case or Resolved Question for it, and the owner has not approved it (tasks.md task 13 says so). Driven on a fixture: a new deferral at a departed rule exits 1 naming the rule and the entry's spec.md line; the same finding with a decision-key exits 0, is waived, and is named in superseded-deferrals. — `specs/073-decided-rule-departures-are-recorded/plan.md:60` — **waived**: Not a break of 073's contract (spec.md is silent, the code matches the plan). It is already routed to the owner at the done-approval, per tasks.md task 13; once the owner approves or reverses it, a Resolved Question in spec.md should record it. (decided by the rule)
- other — data-model.md's Size paragraph for write-review states the file form as 551 bytes compact (552 from the CLI) for 'one-digit counts', but its enumeration of the terms that vary omits `examined` and `scope` (and the violation counts). Re-derived on the built binary: feature 001-x, never-run analysis, one malformed entry, `--examined 0` gives 548 compact (549 from the CLI); `--examined 1000` gives 551 (552). The recorded figure is only reproducible with a 4-digit `examined`, and it is not a bound. — `specs/073-decided-rule-departures-are-recorded/data-model.md:71` — **fixed** (decided by the rule)
- other — data-model.md's Size paragraph for check-departures states 'measured on the built binary: 197 bytes for `001-x`', which is the compact JSON; the CLI prints 198 with its trailing newline (re-measured: 3 entries, 1 finding, 3 unexamined, read-size 10). Task 13's correction (57a3953e) made the same distinction for write-review and left this one. — `specs/073-decided-rule-departures-are-recorded/data-model.md:57` — **fixed** (decided by the rule)
- other — plan.md's constitution section states the file 'at 49,969 bytes (`wc -c`, 2026-11-13)'; `wc -c framework/constitution.md` is now 49,979. 081's commit 401e474b moved it by 10 bytes after 073's change, so the figure is a decayed measurement beside a 50,000 ceiling the lint enforces. — `specs/073-decided-rule-departures-are-recorded/plan.md:70` — **fixed** (decided by the rule)
- other — plan.md's Affected Files table omits the files task 13 created or changed: `scripts/mutation/review-departures-bounded.py` (created) and `scripts/mutation/freshness-results-have-a-bounded-size.py` (066's, two anchors updated). `compute-review-scope` still puts both in scope, through modified-since, not through the plan. — `specs/073-decided-rule-departures-are-recorded/plan.md:92` — **fixed** (decided by the rule)
- other — plan.md's check-departures section lists `entries` as '(rule, tier, reason, cited, decided-by, decided-at)', where data-model.md's result table, the canonical source, and the primitive's output also carry `line`. — `specs/073-decided-rule-departures-are-recorded/plan.md:40` — **fixed** (decided by the rule)
- other — tasks.md task 12 still carries 'Owed, in order: `/ductus:review` ... `/ductus:analyze` ... `check-review-gate`; and the owner's approval for `done`', which is spent: the review ran (53c88c30) and the analysis ran (f03e1f69). A checked task's handoff prose is spent value (constitution §tasks-phase). Nothing reads it as a gate, so no gate misreads it. — `specs/073-decided-rule-departures-are-recorded/tasks.md:118` — **fixed** (decided by the rule)
- other — task 13 is ticked while its Done-when ends 'and 073's next analysis records BE-PAYLOAD-001 addressed', a condition only the owed analysis can meet. analysis.md at HEAD is blocking: true with the gating deferral to task 13, and its digests of data-model.md, plan.md and tasks.md are stale against the files, so the record cannot read as a pass. Both states are named as owed in task 13's own paragraph. — `specs/073-decided-rule-departures-are-recorded/tasks.md:136` — **waived**: The unmet clause is recorded as owed in task 13's paragraph (the analysis run), and check-review-gate at HEAD blocks on review-owed, so the incomplete state is not indistinguishable from a complete one. (decided by the rule)
- convention — the new AGENTS.md Workflow bullet (455556a1) says to pass write-review's and write-analysis's structured arguments as files and 'never inline over MCP', but the Gotchas bullet 'After changing runtime source' still says a structured argument goes on the CLI as `--{name}-json FILE` 'or by driving `ductus mcp` over stdio ... `tools/call`', which is inline MCP; the two restate one instruction and disagree. The new bullet also omits that the CLI refuses a FILE outside the system temp directory (driven: `--findings-json ../f1.json` exits 1, 'must be an absolute path inside the system temp directory'), which the Gotchas bullet names as a 'staging directory'. A reader of only the Workflow bullet writes the file in the repository and is refused. — `AGENTS.md:58` — **fixed** (decided by the rule)
- convention — two window bullets pass §drift-prevention's population test as rules true for every project running the pipeline, so by AGENTS.md's own header they belong in the constitution: 80d71dcc's 'Recommend on the merits, never on the owner's remaining allowance' (no constitution sentence says it; §recommendations asks only for the computed quantity), and 35ec180f's added clause 'check every merit you claim for an option against a source read whole, cited beside the claim' (restates §grounding's 'A partial read is not a read' and §recommendations). 35ec180f also writes the incident ('On 2026-11-13 an option was put to the owner ... both were false') into the bullet, which §drift-prevention routes to its commit, and, unlike its siblings, carries no '(owner, date)' tag. framework/constitution.md is 49,979 of 50,000 bytes (wc -c), so moving either needs a same-size cut elsewhere; the bullets' 46K/19.7K/329 measurement in 455556a1 names no session or method, so it is not re-derivable. — `AGENTS.md:48` — **waived**: Routing a rule to the constitution and cutting an owner-directed bullet's text is the owner's call (the commits record the owner's 2026-11-13 decisions), and the constitution has 21 bytes of room, so a move needs a cut of its own. Surfaced for the owner; nothing in the window breaks a loaded rule. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md` — 020-code-review, 060-exec-analyze-assesses-each-loaded-rule
- `.ductus/reference/analyze.md` — 020-code-review, 060-exec-analyze-assesses-each-loaded-rule
- `.github/workflows/runtime-release.yml` — 040-configurable-specs-dir, 048-govern-acquired-runtime
- `.pi/prompts/ductus-analyze.md` — 020-code-review, 060-exec-analyze-assesses-each-loaded-rule
- `framework/bootstrap/ductus.md` — 040-configurable-specs-dir, 048-govern-acquired-runtime
- `framework/bootstrap/govern.md` — 040-configurable-specs-dir, 048-govern-acquired-runtime
- `framework/bootstrap/hooks/ductus-pre-commit` — 040-configurable-specs-dir, 048-govern-acquired-runtime
- `framework/commands/analyze.md` — 020-code-review, 060-exec-analyze-assesses-each-loaded-rule
- `framework/constitution.md` — 020-code-review, 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `framework/reference/analyze.md` — 020-code-review, 060-exec-analyze-assesses-each-loaded-rule
- `runtime/src/main.rs` — 020-code-review, 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `runtime/src/primitives/bounded_lists.rs` — 058-findings-route-at-discovery, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/compute_review_scope.rs` — 020-code-review, 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `runtime/src/primitives/fetch_archive.rs` — 040-configurable-specs-dir, 048-govern-acquired-runtime
- `runtime/src/primitives/host_config.rs` — 040-configurable-specs-dir, 048-govern-acquired-runtime
- `runtime/src/primitives/process_decisions.rs` — 058-findings-route-at-discovery, 066-audit-records-and-the-completion-gate
- `scripts/mutation/fetch-archive-startup-env.py` — 040-configurable-specs-dir, 048-govern-acquired-runtime
- `scripts/mutation/process-decisions-retained.py` — 058-findings-route-at-discovery, 066-audit-records-and-the-completion-gate
- `scripts/mutation/review-read-credit.py` — 020-code-review, 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `specs/002-project-scaffolding/analysis.md` — 002-project-scaffolding, 010-agent-autonomy
- `specs/002-project-scaffolding/plan.md` — 002-project-scaffolding
- `specs/002-project-scaffolding/review.md` — 002-project-scaffolding
- `specs/002-project-scaffolding/spec.md` — 002-project-scaffolding
- `specs/010-agent-autonomy/analysis.md` — 002-project-scaffolding, 010-agent-autonomy
- `specs/010-agent-autonomy/review.md` — 010-agent-autonomy
- `specs/010-agent-autonomy/spec.md` — 010-agent-autonomy
- `specs/020-code-review/data-model.md` — 020-code-review, 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `specs/020-code-review/spec.md` — 020-code-review, 060-exec-analyze-assesses-each-loaded-rule
- `specs/020-code-review/tasks.md` — 020-code-review, 060-exec-analyze-assesses-each-loaded-rule
- `specs/040-configurable-specs-dir/spec.md` — 040-configurable-specs-dir, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/data-model.md` — 040-configurable-specs-dir, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/plan.md` — 040-configurable-specs-dir, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/review.md` — 040-configurable-specs-dir, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/scenarios/fetch-archive-reads-its-proxy-once.md` — 040-configurable-specs-dir, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/scenarios/retired-namespace-tools-are-off-limits.md` — 040-configurable-specs-dir, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/scenarios/state-a-version-checks-the-pin.md` — 040-configurable-specs-dir, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/spec.md` — 040-configurable-specs-dir, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/tasks.md` — 040-configurable-specs-dir, 048-govern-acquired-runtime
- `specs/058-findings-route-at-discovery/data-model.md` — 058-findings-route-at-discovery, 066-audit-records-and-the-completion-gate
- `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md` — 020-code-review, 060-exec-analyze-assesses-each-loaded-rule
- `specs/060-exec-analyze-assesses-each-loaded-rule/tasks.md` — 020-code-review, 060-exec-analyze-assesses-each-loaded-rule
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 020-code-review, 058-findings-route-at-discovery, 066-audit-records-and-the-completion-gate, 081-reviews-and-checks-follow-what-changed
- `specs/066-audit-records-and-the-completion-gate/scenarios/process-decisions-results-have-a-bounded-size.md` — 058-findings-route-at-discovery, 066-audit-records-and-the-completion-gate
- `specs/081-reviews-and-checks-follow-what-changed/analysis.md` — 081-reviews-and-checks-follow-what-changed
- `specs/081-reviews-and-checks-follow-what-changed/plan.md` — 081-reviews-and-checks-follow-what-changed
- `specs/081-reviews-and-checks-follow-what-changed/review.md` — 081-reviews-and-checks-follow-what-changed
- `specs/081-reviews-and-checks-follow-what-changed/spec.md` — 081-reviews-and-checks-follow-what-changed
- `specs/081-reviews-and-checks-follow-what-changed/tasks.md` — 081-reviews-and-checks-follow-what-changed

## Unexamined governance

*None.*
