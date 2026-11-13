---
spec: 073-decided-rule-departures-are-recorded
last-run: 2026-11-13T14:24:10Z
reviewed-against: becf827ff72768e3abebe249aec2dbc99438ee1b
diff-base: 743079d95497cc40aa67238bbd8b5cdae5b5785e
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 18
scope: 21
excluded: 1
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 8434e82adf1b4b244412bd9e542cfaaee558fb0c08727e8d8d58dea0269470e1
blocking: false
dispositions:
  fixed: 2
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
---

# Review — 073-decided-rule-departures-are-recorded

## Summary

Five passes over 073's window since its last review (diff-base 743079d9, base-source last-review): task 12's bound of check-departures' result (8b89ec50), its ticks (becf827f), and 073's artifact commits since 743079d9. Three reviewers: one ran all five passes across the window and checked AC1 to AC8 against the tree, which all hold; two went rule by rule over all 310 loaded rules, 125 and 185, each file read whole (158,365 and 177,722 characters of Statement and Verification by their own counts, labels included, against 156,067 and 173,097 by the jq extraction, which counts the text alone). examined counts the 18 in-scope files a pass read, by their patch hunks with named context ranges: interpreter/mod.rs, schema/primitives.rs, schema/descriptions.rs, tests/mcp.rs, tests/specs_root_override.rs and runtime/CHANGELOG.md by their hunks only, and plan.md lines 41-52 and 62-90 and data-model.md's table rows to 500 characters a line. Not counted: the 3 generated copies (.claude/commands/ductus/analyze.md, .ductus/reference/analyze.md, .pi/prompts/ductus-analyze.md), generated from framework/commands/analyze.md and framework/reference/analyze.md, which a pass read, with scripts/gen-claude-commands.sh re-run at becf827f and reported in sync. No MUST or SHOULD: BE-PAYLOAD-001, the finding task 12 was deferred for, is addressed, and the file form measured 197 bytes for 001-x on the built binary, as the data model states. Fix round 1 fixed two chores, the bound test's fixture comment and plan.md's Affected Files; four observations are waived by the rule with their reasons, one by its stored decision, and the stored decision on result_sizes' empty corpus expired, since the window gave the row entries.

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

- other — result_sizes is an ignored measurement that asserts nothing, so task 12's test-and-mutant item rests on the unit test and departures-bounded.py's mutants — `runtime/tests/result_sizes.rs` — **waived**: result_sizes is a measurement by design, ignored by cargo test and run by scripts/measure-result-sizes.sh; task 12's item is met by a_result_past_the_read_size_names_a_file_holding_every_list, which departures-bounded.py's M1 to M6 each turn red. (decided by the rule)
- convention — plan.md Affected Files omitted four files task 12 added or changed — `specs/073-decided-rule-departures-are-recorded/plan.md` — **fixed** (decided by the rule)
- convention — AGENTS.md's bullet on rule-judging agents states a measurement whose source, the agents' own reports, no committed artifact holds — `AGENTS.md` — **waived**: The bullet states its method and unit, each agent's own report when asked and the characters of each Verification read; the session transcripts it came from are per-user state outside git, so nothing committed can be cited. (decided by the rule)
- convention — the bound's unit test comment counted three unexamined entries where its fixture holds two — `runtime/src/primitives/check_departures.rs` — **fixed** (decided by the rule)
- other — result_sizes' check-departures row writes the same 300 entries into both corpora, so its Grows column reads no — `runtime/tests/result_sizes.rs` — **waived**: The row measures the bound, which its Bounded and default-read-size columns show; a spec's departure entries grow with that spec, not with the corpus the Grows column compares. (decided by the rule)
- other — RuleFacts::load re-reads the rule files discover-rule-files already returns — `runtime/src/primitives/check_departures.rs` — **waived**: check-departures is a standalone primitive whose callers pass only a feature; taking another primitive's output as an argument would couple two tools' interfaces to save one read of twelve files. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `specs/082-rule-assessments-converge-across-runs/spec.md` — 082-rule-assessments-converge-across-runs

## Unexamined governance

*None.*
