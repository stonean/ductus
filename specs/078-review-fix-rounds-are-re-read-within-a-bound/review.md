---
spec: 078-review-fix-rounds-are-re-read-within-a-bound
last-run: 2026-11-04T21:04:09Z
reviewed-against: 80b43a11210a79eed9cffe3ec89abcca52f88d9b
diff-base: f47d2b0326adac7a3c6e0e238bb54c5938dd037f
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 57
scope: 78
skipped-passes: []
fix-rounds: 2
re-reads: 1
diff-checked: 1
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 6ec369d934095bba58a36f9aadde26e484c9c0178443c7eff132524903c5818d
  scenarios/a-reviews-own-record-is-no-fix-round.md: f1aeb3c3fd26d0e17a98ef6bb433babcabe2abb53a3914e2de4f2ddf33be784d
  scenarios/fix-round-results-have-a-bounded-size.md: bd6472ece8d3586d1de656fd1d1689848d455ed79183249ff003e9db24817de7
blocking: false
dispositions:
  fixed: 23
  deferred: 7
  waived: 19
  undispositioned: 0
decisions:
  - key: "bug: a Summary the caller supplies drops the loop's account, so AC2's naming of exempt rounds rests on the host — `runtime/src/primitives/write_review.rs`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "17"
    decided-at: 2026-11-04T12:42:29Z
    decided-by: andy@stone.dev
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
  - key: "convention: `compute-review-scope`'s module doc still defines `scope` as the plain union — `runtime/src/primitives/compute_review_scope.rs`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "19"
    decided-at: 2026-11-04T13:33:22Z
    decided-by: andy@stone.dev
  - key: "convention: the command's Inputs Scope bullet contradicts step 1's deleted-file sentence — `framework/commands/review.md`"
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
  - key: "bug: a review run halted or cut off between step 11 and step 12's last write-review leaves a record without the loop fields, which gate check 12 passes as predating the loop — `runtime/src/primitives/check_review_gate.rs`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): fixing it needs a new requirement and another implement and review cycle for 078; the gate's passing notice names a record without the loop fields, and the malformed-switch path to it is fixed in this run."
    decided-at: 2026-11-04T20:51:31Z
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
  - key: "other: framework/reference/review.md is over the 50,000-byte read size, and the plan recorded it as a known limitation with no disposition — `framework/reference/review.md`"
    outcome: deferred
    target: specs/020-code-review/tasks.md
    task: "16"
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "security: about 24 primitives besides the loop's screen feature by validate_feature_argument, a denylist, alone — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: "Owner, 2026-11-04: 022's shared screen, outside 078's code (AGENTS.md's waiver of machinery observations); measured, no denylist-only primitive deletes outside its tests, and the loop's and retire-feature's destructive paths use the allowlist."
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
---

# Review — 078-review-fix-rounds-are-re-read-within-a-bound

## Summary

0 MUST and 0 SHOULD violations counted, 0 low-confidence findings; not blocking. Two violations fired and were fixed in the run, so they leave the counts. The first is BE-INPUT-002 (MUST): `start-fix-rounds`, `brief-re-read`, `read-re-read` and `check-prescriptions` screened `feature` by a denylist alone, and on a scratch fixture `start-fix-rounds --feature .` emptied every feature's rounds; the four now share `fix_rounds::validate_feature`'s allowlist. The second is QUAL-TEST-001 (SHOULD): `check-prescriptions`' bounded test held `differing-count` and `failed-count` at their defaults. Two more, on 066 task 11's Family 19 work, are deferred to 066, which releases them: QUAL-CLAIM-001 on `scripts/audit/review-freshness.sh` to its task 21, and BE-PAYLOAD-001 on its data model's `check-review-freshness` result to its task 20, with scenario `freshness-results-have-a-bounded-size`. Each new test was shown to fail by a mutation, all 13 killed: `loop-inputs-screened.py` M1–M6, `fix-round-results-bounded.py` M8–M9, `out-of-round-findings.py` M1 (re-anchored) and M4, `re-read-budget.py` M3 and `own-record-is-no-round.py` M3.

The five passes ran on 2026-11-04 over base `f47d2b03`, the first review's commit, by the owner's decision. The default base `059b830e` resolved 127 entries, and each entry past the 62 this base resolved was unchanged since `f47d2b03`: read by the first review at `65ec2d7f`, re-read by its fix rounds, or from other specs' commits their own reviews read. The passes read the tree at `69a42912` in four read-only agent slices: the loop primitives, their tests and mutation lists; the record, gate, scope and registration; the 066 and 026 work that `02bda11a` and `69a42912` brought into the window, with the CI workflows; and the framework prose, changelog and 078's artifacts. The host re-checked every finding and observation against the tree, reproducing BE-INPUT-002's deletion on a fixture and running `check-review-freshness` for the snake-case observation. Every runtime call ran on `runtime/target/release/ductus` built from `69a42912`, since this session's MCP server holds an earlier build. `examined: 57` counts the in-scope files the passes read, of the 62 the scope resolved when they ran. Not examined: the 5 generated mirrors (`.claude/commands/ductus/configure.md` and `review.md`, `.ductus/reference/audit.md` and `review.md`, `.pi/prompts/ductus-review.md`), which a generator re-run on a clean clone of `69a42912` reported in sync. Partial reads: `write_review.rs`, `check_review_gate.rs`, `schema/primitives.rs`, `mcp/server.rs`, `main.rs`, `interpreter/mod.rs` and `primitives/mod.rs` at their window hunks and every item of the fix-round loop; `framework/reference/review.md` at its window hunks, with §Fix-round loop, §Record the run in `review.md`, §Window and coverage and §Blocking semantics whole; `runtime/CHANGELOG.md` at its Unreleased section; `framework/bootstrap/ductus.md` and `framework/constitution.md` at every passage on the loop or the record; and the window hunks alone of `framework/reference/audit.md`, both configure files, `AGENTS.md`, the three workflows, `scripts/audit/README.md`, and 026's spec and 066's data model, plan and tasks. The scope resolves 78 entries at this write, 16 past the passes' 62: 7 from 066 task 14's commit `80b43a11`, which landed before this run wrote anything; 6 of this run's own writes, this record and 5 its fix rounds re-read; and `README.md`, `runtime/src/primitives/discover_rule_files.rs` and `runtime/tests/review_command.rs`, which the corrected Affected Files now names, unchanged since `f47d2b03` and read by the first review.

Fix rounds: 2, from the base `start-fix-rounds` took after `80b43a11` landed (`178ca808`), on the binary rebuilt from this run's tree. Round 1, this run's fixes and deferral writes (25 paths), was re-read by an independent agent with findings: 6 SHOULDs, each a sentence the round left untrue (the tick carve-out in three more places, step 9's lists file in the command, the read-size consumers in `ductus.md`, an over-broad plan row, the spec's `compute-review-scope` wording, the `start-fix-rounds` errors), and 1 NOTE, every one with prescribed edits; the re-read also ran the 12 mutations the round added or re-anchored in a scratch copy, all killed. Round 2 applied exactly those edits, and `check-prescriptions` matched them with none open, so it is diff-checked; the next round was empty.

49 observations, dispositioned by the owner on 2026-11-04, one decision covering each of the fixed group and the 066 group and the rest singly: 23 fixed in the run, the loop's NOTE among them, 1 deferred to 020's task 16, 19 waived, and 6 stored decisions that still hold, carried with their deferrals to 066's tasks 17 and 19. Of the waivers, 14 are on 066's and 026's Family 19 work, waived by AGENTS.md's rule for observations on machinery outside the code under review, which 066's own review reads; 5 are design or out-of-scope items. The lapsed deferral of the declined-NOTE edit to this spec's task 13 is dropped: its residue, that a file's NOTE edits match all or none, is now stated where the spec, data model and reference claimed any single edit could be declined. Nine stored decisions expired: four no longer fire, and five name files outside this window, 058's spec body, since fixed, and four whose work 066's tasks 18 and 19 still hold.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Deferred findings

### DEFERRED: QUAL-CLAIM-001 — an unrecognised check-review-freshness state counts as examined and current

- **File**: `scripts/audit/review-freshness.sh:172-196`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish *examined the subject and found nothing* from *could not examine the subject*.
- **Finding**: A state other than never-run, undeterminable and stale falls through to `examined += 1` with no finding, so an answer from a runtime of another version, or a future RecordFreshness state, reads as a current review.
- **Auto-fixable**: no
- **Suggested fix**: Count an unrecognised state as not answered, with a finding naming it.
- **Deferred**: to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 21

### DEFERRED: BE-PAYLOAD-001 — check-review-freshness returns freshness.paths with no stated maximum size

- **File**: `specs/066-audit-records-and-the-completion-gate/data-model.md:165-188`
- **Rule**: Response payloads MUST have a bounded maximum size. Endpoints that return collections able to exceed a small fixed maximum MUST paginate per `BE-PAGE` (api-backend.md) rather than return an unbounded list in a single response.
- **Finding**: The data model's only bound, "`paths` holds at most the spec's durable contracts", is no size limit, and `freshness_of` also lists each recorded key no longer on disk; write-review's analyze-freshness and the gate carry the same shape.
- **Auto-fixable**: no
- **Suggested fix**: State a maximum by the mechanism scope-results-have-a-bounded-size settles.
- **Deferred**: to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 20

## Observations

- convention: the MCP descriptions and result-type doc comments of brief-re-read, read-re-read and check-prescriptions omitted the review.md exclusion, the out-of-round demotion and the declined-NOTE match — `runtime/src/mcp/server.rs` — **fixed**
- bug: the Summary's budget sentence printed the budget constant as the count of re-reads that found a MUST or SHOULD — `runtime/src/primitives/write_review.rs` — **fixed**
- bug: check-prescriptions matches a file only with all its NOTE edits applied or none, while the spec, data model and reference said any NOTE edit may be declined — `runtime/src/primitives/check_prescriptions.rs` — **fixed**
- bug: a re-read finding whose path is spelled differently from round.toml's was demoted from MUST or SHOULD to NOTE — `runtime/src/primitives/fix_rounds.rs` — **fixed**
- other: no test pinned that brief-re-read reports the re-reads-off reason on an empty or exempt round — `runtime/src/primitives/brief_re_read.rs` — **fixed**
- convention: check-prescriptions' and read-re-read's doc comments and the data model omitted place's .gitignore write and stale-file removal — `runtime/src/primitives/check_prescriptions.rs` — **fixed**
- perf: read-re-read's and brief-re-read's bounded cloned the whole list only to measure it — `runtime/src/primitives/read_re_read.rs` — **fixed**
- convention: RoundRecord.base was written to round.toml and read by no primitive — `runtime/src/primitives/fix_rounds.rs` — **fixed**
- other: the line-ending claim held only for a single-line old text; an old is matched as the round's git diff shows it — `runtime/src/primitives/check_prescriptions.rs` — **fixed**
- convention: the command's Writes line named only compute-review-scope's lists file — `framework/commands/review.md` — **fixed**
- convention: the changelog's Added entry said the four primitives write only under .ductus/review-rounds/ — `runtime/CHANGELOG.md` — **fixed**
- other: the spec, AC1 and the plan put the loop before write-review and ended it at a clean re-read, where the command runs write-review on both sides and ends at the first empty round — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **fixed**
- convention: the changelog never named check-review-gate's new blocked-by value fix-round-not-re-read — `runtime/CHANGELOG.md` — **fixed**
- convention: the changelog's Changed entries for tasks 10-14 described changes to tools first added in the same Unreleased section — `runtime/CHANGELOG.md` — **fixed**
- other: the data model said two concurrent creators of the rounds .gitignore both write it, where ignore_everything uses create_new — `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md` — **fixed**
- convention: the command's Config list named four keys and omitted [artifacts] read-size-bytes, and ductus.md's consumer line omitted /review — `framework/commands/review.md` — **fixed**
- convention: the plan's Affected Files missed files 078 changed — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — **fixed**
- other: the spec said every exempt edit stales the analysis, where a criterion's tick does not — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **fixed**
- other: a Resolved Question called compute-review-scope read-only, where it writes its lists file — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **fixed**
- other: the data model's start-fix-rounds errors omitted InvalidPath and the clearing Io error — `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md` — **fixed**
- other: the data model's failed description said a file the snapshot does not hold as text has none, where its edits occur zero times — `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md` — **fixed**
- bug: a malformed [review] re-reads-off halted the run only at its first round, after step 10's fixes had landed — `runtime/src/primitives/start_fix_rounds.rs` — **fixed**
- convention: own-record-is-no-round.py's docstring did not cover the M3 this run added — `scripts/mutation/own-record-is-no-round.py` — **fixed**
- bug: a review run halted or cut off between step 11 and step 12's last write-review leaves a record without the loop fields, which gate check 12 passes as predating the loop — `runtime/src/primitives/check_review_gate.rs` — **waived**: Option B (owner, 2026-11-04): fixing it needs a new requirement and another implement and review cycle for 078; the gate's passing notice names a record without the loop fields, and the malformed-switch path to it is fixed in this run.
- bug: an open re-read MUST or SHOULD left at a stop is an observation, and undispositioned it never holds done — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — **waived**: Option B (owner, 2026-11-04): low confidence; the loop asks the operator to defer or waive each open finding at a stop, and a design change would cost 078 another cycle.
- perf: a file-form loop result's free text (an echoed path or key in problem, the re-reads-off reason) has no bound — `runtime/src/primitives/check_prescriptions.rs` — **waived**: Option B (owner, 2026-11-04): reaching it takes an agent-written path or key, or a configured reason, tens of kilobytes long; the lists, which carry the growth, are bounded.
- other: framework/reference/review.md is over the 50,000-byte read size, and the plan recorded it as a known limitation with no disposition — `framework/reference/review.md` — **deferred** to `specs/020-code-review/tasks.md` task 16
- security: about 24 primitives besides the loop's screen feature by validate_feature_argument, a denylist, alone — `runtime/src/primitives/mod.rs` — **waived**: Owner, 2026-11-04: 022's shared screen, outside 078's code (AGENTS.md's waiver of machinery observations); measured, no denylist-only primitive deletes outside its tests, and the loop's and retire-feature's destructive paths use the allowlist.
- other: when the rename exemption cannot run, check-review-freshness answers stale with nothing saying the exemption did not run — `runtime/src/primitives/check_review_freshness.rs` — **waived**: Owner, 2026-11-04: 066's specified, fail-closed behavior, outside 078's code; the marker is an enhancement, not a defect.
- bug: review-freshness.sh's call() sets no timeout and catches no OSError, so an exec failure ends Family 19 in a traceback with no finding or coverage line — `scripts/audit/review-freshness.sh` — **waived**: Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window.
- bug: review-freshness.sh's undeterminable advice to re-run review is wrong for a review.md that does not parse — `scripts/audit/review-freshness.sh` — **waived**: Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window.
- other: the Family 19 parity test never seeds the never-run or failed-call branches — `runtime/tests/mechanical_sweep_parity.rs` — **waived**: Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window.
- bug: check-review-freshness emits last_run and analyzed_against where 066's data-model example shows kebab-case — `specs/066-audit-records-and-the-completion-gate/data-model.md` — **waived**: Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window.
- convention: 026's family-19-review-freshness edge cases contradict its rewritten Behavior — `specs/026-framework-self-audit/scenarios/family-19-review-freshness.md` — **waived**: Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window.
- convention: 066's review-staleness-on-done-specs says review-freshness.sh has always carried the exemption — `specs/066-audit-records-and-the-completion-gate/scenarios/review-staleness-on-done-specs.md` — **waived**: Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window.
- convention: 026's family-19-mechanical-sweep-exemption says Family 19 compares shas — `specs/026-framework-self-audit/scenarios/family-19-mechanical-sweep-exemption.md` — **waived**: Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window.
- convention: review-freshness.sh's header and audit.md step 19 say the gate asserts only last-run and blocking — `scripts/audit/review-freshness.sh` — **waived**: Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window.
- convention: mechanical_sweep.rs and its parity test carry doc comments the window made stale — `runtime/src/primitives/mechanical_sweep.rs` — **waived**: Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window.
- convention: RecordFreshness::Undeterminable's doc names only a missing digest, where an unparseable review.md also answers it — `runtime/src/schema/primitives.rs` — **waived**: Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window.
- convention: the Family 19 parity test spawns bash without ductus_command, so session identities reach the runtime — `runtime/tests/mechanical_sweep_parity.rs` — **waived**: Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window.
- other: 066's plan states counts of audit scripts and done specs with no method, and they re-derive differently — `specs/066-audit-records-and-the-completion-gate/plan.md` — **waived**: Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window.
- convention: review-freshness.sh lists done specs with its own dashboard call and keeps a copy of Paths::load — `scripts/audit/review-freshness.sh` — **waived**: Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window.
- convention: review-freshness.sh:97's test-and-command ends its block, the set -e trap should -e be added — `scripts/audit/review-freshness.sh` — **waived**: Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window.
- bug: a Summary the caller supplies drops the loop's account, so AC2's naming of exempt rounds rests on the host — `runtime/src/primitives/write_review.rs` — **deferred** to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 17
- bug: 066's data-model example lists two `modified-since` paths with a count of 1 — `specs/066-audit-records-and-the-completion-gate/data-model.md` — **deferred** to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 19
- convention: `compute-review-scope`'s file-form test asserts `deleted` on a fixture that deletes nothing, so three mutations survive — `runtime/src/primitives/compute_review_scope.rs` — **deferred** to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 19
- convention: `compute-review-scope`'s module doc still defines `scope` as the plain union — `runtime/src/primitives/compute_review_scope.rs` — **deferred** to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 19
- convention: the command's Inputs Scope bullet contradicts step 1's deleted-file sentence — `framework/commands/review.md` — **deferred** to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 19
- bug: a window holding only deletions under an empty plan reaches the nothing-to-review-yet branch — `framework/commands/review.md` — **deferred** to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 19

## Skipped passes

*None.*

## Unexamined governance

*None.*
