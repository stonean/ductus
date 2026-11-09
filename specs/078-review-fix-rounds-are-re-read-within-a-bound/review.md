---
spec: 078-review-fix-rounds-are-re-read-within-a-bound
last-run: 2026-11-09T20:01:53Z
reviewed-against: 43b33b3013f1aae38fbd1d8cd8b8f3964e0683cf
diff-base: 494a1f3f15b2244a8a226e0dad0ae20d159beef8
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 14
scope: 15
excluded: 17
contract-breaks: 0
skipped-passes: []
fix-rounds: 3
re-reads: 2
diff-checked: 1
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 95cd98fd2db64be4ac315c056ac68c8e70b480723cc124a4db436ca5b570cf78
  scenarios/a-reviews-own-record-is-no-fix-round.md: 8a0190ef5a926425199a4381d558f52db626cdd81803b4e23c9aa1055b130440
  scenarios/fix-round-results-have-a-bounded-size.md: bd6472ece8d3586d1de656fd1d1689848d455ed79183249ff003e9db24817de7
blocking: false
dispositions:
  fixed: 5
  deferred: 1
  waived: 5
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
  - key: "bug: a window holding only deletions under an empty plan reaches the nothing-to-review-yet branch — `framework/commands/review.md`"
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
  - key: "other: when the rename exemption cannot run, check-review-freshness answers stale with nothing saying the exemption did not run — `runtime/src/primitives/check_review_freshness.rs`"
    outcome: waived
    reason: "Owner, 2026-11-04: 066's specified, fail-closed behavior, outside 078's code; the marker is an enhancement, not a defect."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "bug: review-freshness.sh's call() sets no timeout and catches no OSError, so an exec failure ends Family 19 in a traceback with no finding or coverage line — `scripts/audit/review-freshness.sh`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "bug: review-freshness.sh's undeterminable advice to re-run review is wrong for a review.md that does not parse — `scripts/audit/review-freshness.sh`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
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
  - key: "convention: 026's family-19-review-freshness edge cases contradict its rewritten Behavior — `specs/026-framework-self-audit/scenarios/family-19-review-freshness.md`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "convention: 066's review-staleness-on-done-specs says review-freshness.sh has always carried the exemption — `specs/066-audit-records-and-the-completion-gate/scenarios/review-staleness-on-done-specs.md`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "convention: 026's family-19-mechanical-sweep-exemption says Family 19 compares shas — `specs/026-framework-self-audit/scenarios/family-19-mechanical-sweep-exemption.md`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "convention: review-freshness.sh's header and audit.md step 19 say the gate asserts only last-run and blocking — `scripts/audit/review-freshness.sh`"
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
  - key: "convention: review-freshness.sh lists done specs with its own dashboard call and keeps a copy of Paths::load — `scripts/audit/review-freshness.sh`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "convention: review-freshness.sh:97's test-and-command ends its block, the set -e trap should -e be added — `scripts/audit/review-freshness.sh`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: rule QUAL-CLAIM-001 at scripts/audit/review-freshness.sh
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "21"
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
---

# Review — 078-review-fix-rounds-are-re-read-within-a-bound

## Summary

Window `494a1f3f..43b33b30` (base-source `last-review`, `review-owed` true), 14 scope entries from three 078 commits:

- `45ec6154`, the analysis;
- `55209792`, task 17: an empty `[review] re-reads-off` counts as absent (CFG-ENV-001, which the analysis deferred);
- `c6626c4b`, which copied `framework/bootstrap/ductus.md` over the retired `framework/bootstrap/govern.md` once Family 21 reported the drift.

The later commits are the 080 session's and were excluded.

One independent read-only agent ran the five passes over the first 13 entries, by their window hunks and the code those call. It drove the binary on one fixture per value of the switch: absent, empty, whitespace, a no-break space, one, fifteen and sixteen characters, and sixteen control characters. `examined` is 14: this session read `govern.md`'s change itself, its diff against `ductus.md`; the two are byte-identical by `cmp`, and Family 21 is clean at `c6626c4b`. The 12 rule files `discover-rule-files` selected were applied, and CFG-ENV-001, -002, -005 and -008, CFG-CONST-004, QUAL-TEST-001 and BE-INPUT-001 and -002 are met. There is no violation.

Four observations:

- the `govern.md` drift, fixed in `c6626c4b`;
- a stale doc comment on `InvalidReReadsOff`, fixed in the run;
- the switch's length measured before normalizing, covered by the stored waiver, whose reason still holds;
- AC14's wording read alone, waived: the data model's row states the rule.

Of the stored decisions, five on in-scope files matched, each checked against its file; 27 on files outside the scope are retained.

3 fix round(s): 2 re-read, 1 diff-checked, 0 exempt, 0 not re-read.

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

- bug: task 17's ductus.md edit was not copied to the retired bootstrap path, framework/bootstrap/govern.md, which Family 21 holds byte-identical — `framework/bootstrap/govern.md` — **fixed** (decided by the rule)
- convention: InvalidReReadsOff's doc still said a reason shorter than the minimum once trimmed halts, which an empty one no longer does — `runtime/src/primitives/mod.rs` — **fixed** (decided by the rule)
- convention: AC14's 'a switch without a valid reason halts the run', read alone, covers an empty re-reads-off, which now counts as absent — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **waived**: The data model's re-reads-off row, the durable contract the switch rests on, says an empty value counts as absent and so is no switch, as task 17 records (owner, 2026-11-09). (decided by the rule)
- bug: a window holding only deletions under an empty plan reaches the nothing-to-review-yet branch — `framework/commands/review.md` — **deferred** to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 19
- bug: an open re-read MUST or SHOULD left at a stop is an observation, and undispositioned it never holds done — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — **waived**: Option B (owner, 2026-11-04): low confidence; the loop asks the operator to defer or waive each open finding at a stop, and a design change would cost 078 another cycle.
- bug: the re-reads-off reason's 16-character minimum is measured before its control characters and runs of spaces are collapsed — `runtime/src/primitives/fix_rounds.rs` — **waived**: Contrived input; the check matches the data model's 'once trimmed'. (decided by the rule)
- other: the loop primitives accept a misspelled [review] key silently — `runtime/src/primitives/fix_rounds.rs` — **waived**: /review's step 2, discover-rule-files, reports an unknown [review] key (CFG-ENV-012), verified on a fixture: 'unknown config key: [review] re-read-off'. (decided by the rule)
- other: a fixer's uncommitted edit that another spec's commit sweeps in leaves the round as excluded and is never re-read here — `runtime/src/primitives/fix_rounds.rs` — **waived**: It follows this spec's Edge Case on other sessions: that spec's review reads it as its own change, and git worktree isolates sessions. (decided by the rule)
- convention: InvalidReReadsOff's edited doc line ran past its paragraph's wrap (fix round 1's NOTE) — `runtime/src/primitives/mod.rs` — **fixed** (decided by the rule)
- convention: InvalidReReadsOff's doc omitted that a review that is not a table raises it too (fix round 1's NOTE) — `runtime/src/primitives/mod.rs` — **fixed** (decided by the rule)
- convention: InvalidReReadsOff's doc put a review that is not a table under 'the key is present' (fix round 2's NOTE) — `runtime/src/primitives/mod.rs` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.pi/extensions/ductus.ts` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-configure.md` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `AGENTS.md` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `README.md` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `framework/bootstrap/configure/pi.md` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `framework/bootstrap/pi/ductus-bridge.ts` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `framework/constitution.md` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `scripts/tests/pi-bridge-harness.mjs` — 080-deterministic-runtime-efficiency
- `specs/007-govern-workflow/tasks.md` — 007-govern-workflow, 080-deterministic-runtime-efficiency
- `specs/050-constitution/plan.md` — 050-constitution, 080-deterministic-runtime-efficiency
- `specs/064-pi-host-support/plan.md` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `specs/064-pi-host-support/spec.md` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/plan.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/research.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/antigravity-keeps-the-model-path.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/opencode-keeps-the-model-path.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/tasks.md` — 007-govern-workflow, 064-pi-host-support, 080-deterministic-runtime-efficiency

## Unexamined governance

*None.*
