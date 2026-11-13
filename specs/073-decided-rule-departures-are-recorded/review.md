---
spec: 073-decided-rule-departures-are-recorded
last-run: 2026-11-13T05:30:20Z
reviewed-against: 459109d647547119eb0affe7209708967a76ee5c
diff-base: be773e52baa5c7b5230228999844775e78f98962
must-violations: 0
should-violations: 0
low-confidence: 3
examined: 44
scope: 53
excluded: 26
contract-breaks: 0
skipped-passes: []
fix-rounds: 2
re-reads: 1
diff-checked: 1
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 4eba4d3bc844ba46873e189bda62668e52d39bef3c6bb3ee75569d8a226ac2f9
blocking: false
dispositions:
  fixed: 21
  deferred: 0
  waived: 6
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
  - key: other — the result_sizes case runs check-departures on a corpus with no departure entries, so its row cannot show growth — `runtime/tests/result_sizes.rs`
    outcome: waived
    reason: check-departures' result holds one spec's entries and its compiled description keeps it unbounded, so the case pins registration and serialization, not a bound.
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
---

# Review — 073-decided-rule-departures-are-recorded

## Summary

Five passes over 073's window since its reopen (diff-base be773e52, base-source reopen), divided among five reviewers by behavior — departure entries and check-departures, analyze's waived verdict, review's departure waivers, and the governing documents with 073's own artifacts — plus a rule-by-rule pass that read all 310 loaded rules' Statements and Verifications in full. examined counts the 44 in-scope files a pass read, whole or every diff hunk with its context; the large files were read by diff (write_analysis.rs, write_review.rs, interpreter/mod.rs, payload.rs, schema/primitives.rs, primitives/mod.rs, main.rs, mcp/server.rs) and runtime/CHANGELOG.md by its whole [Unreleased] section. Not counted: the 7 generated copies (.claude/commands/ductus/analyze.md, configure.md and review.md, .ductus/reference/analyze.md and review.md, .pi/prompts/ductus-analyze.md and ductus-review.md), generated from sources a pass read in full, with the generator re-run at HEAD and reported in sync; and runtime/src/primitives/compute_review_scope.rs and resolve_anchor.rs, which only fix round 1 changed and which that round's re-read read. The passes found one high-confidence SHOULD (QUAL-TEST-001 in check_departures.rs: no test failed if step 4 stopped reporting an unresolved cited: reference) and six broken contracts; fix round 1 (d15a0188) fixed all seven and fifteen chores. Its re-read found one SHOULD in docs/analyze.md, fixed by its prescribed edit in round 2 (459109d6) and diff-checked; the next round was empty. Three low-confidence findings stand and block nothing, and six observations are waived by the rule with their reasons.

2 fix round(s): 1 re-read, 1 diff-checked, 0 exempt, 0 not re-read.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: BE-PAYLOAD-001 — check-departures returns its entries, findings and unexamined lists unbounded

- **File**: `runtime/src/primitives/check_departures.rs`
- **Rule**: Response payloads MUST have a bounded maximum size. Endpoints that return collections able to exceed a small fixed maximum MUST paginate per `BE-PAGE` (api-backend.md) rather than return an unbounded list in a single response.
- **Finding**: check-departures has no lists-file form, where write-analysis bounds even its per-spec waived list through bounded_lists::place; its compiled description keeps it unbounded because it holds one spec's entries, so whether one spec's entries can exceed a small fixed maximum is the open question.
- **Auto-fixable**: no

### LOW-CONFIDENCE: QUAL-TEST-001 — the unreadable-document test returns early when a 0o000 mode does not bind

- **File**: `runtime/src/primitives/check_departures.rs:693-730`
- **Rule**: A test SHOULD fail when the behavior it names is removed or broken. A test that passes whether or not the code under test behaves as named SHOULD NOT be added or counted as coverage.
- **Finding**: Run as root, the permission does not bind, the test returns early and passes with the anchor-target-unreadable behavior removed; no suite configuration here runs as root, and the same pattern stands in older tests.
- **Auto-fixable**: no

### LOW-CONFIDENCE: BE-DATA-006 — a commit author's email is copied into committed analysis.md and review.md records

- **File**: `runtime/src/primitives/write_analysis.rs:349-376`
- **Rule**: Personally Identifiable Information (PII) MUST be minimized at collection, scoped to a documented purpose, and deletable on user request. Data retention periods MUST be defined for each PII field.
- **Finding**: decided-by and waived-by carry the email of the commit that introduced an entry, with a stated purpose and a minimal field set but no retention or deletion path; the value is already in the repository's own history, which is why the finding is low-confidence.
- **Auto-fixable**: no

## Waived findings

*None.*

## Deferred findings

*None.*

## Observations

- bug — a departure waiver anchored every file the run's findings named, not only files in the review scope its contract bounds it to — `runtime/src/primitives/write_review.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug — cites_a_rule_id matched a rule ID anywhere in an entry's token, so text around one escaped the unknown-rule finding with no rule file loaded — `runtime/src/primitives/check_rule_ids.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug — analyze steps 11 and 12 told the host to answer {verdict: waived}, which fails the ductus exec extension schema — `framework/commands/analyze.md` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug — an unreadable revision in the attribution walk was attributed instead of unattributed — `runtime/src/primitives/departures.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug — a stored departure waiver was never refreshed from its entry, so an edited reason or a later commit's attribution never reached the record — `runtime/src/primitives/write_review.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug — a deferral of a violation a departure entry decides made write-review fail, on every later run for a stored one — `runtime/src/primitives/write_review.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- convention — the attribution mapping and its unattributed literal were written in three modules — `runtime/src/primitives/departures.rs` — **fixed** (decided by the rule)
- convention — the departure problem messages were mapped in two modules — `runtime/src/primitives/check_departures.rs` — **fixed** (decided by the rule)
- other — fill_waived carried a branch no caller reaches — `runtime/src/primitives/rule_verdicts.rs` — **fixed** (decided by the rule)
- convention — render_rule_sections' doc comment omitted the ## Waived rules section it emits — `runtime/src/primitives/write_analysis.rs` — **fixed** (decided by the rule)
- convention — list_item's difference from the shared bullet_text was unstated — `runtime/src/primitives/departures.rs` — **fixed** (decided by the rule)
- convention — analyze step 12 cited step 5 for step 5.1's entries — `framework/commands/analyze.md` — **fixed** (decided by the rule)
- convention — analyze step 14 did not say a malformed departs: item's rule is collected as any other — `framework/commands/analyze.md` — **fixed** (decided by the rule)
- convention — analyze steps 5.1 and 22 restated compiled descriptions — `framework/commands/analyze.md` — **fixed** (decided by the rule)
- convention — analyze step 22 named a passed-rules list it does not render — `framework/commands/analyze.md` — **fixed** (decided by the rule)
- convention — docs/analyze.md called non-firing Applicable Rules entries advisory with no departure exception — `docs/analyze.md` — **fixed** (decided by the rule)
- convention — docs/rules.md said review waives a departed rule at every file in scope rather than where it fires — `docs/rules.md` — **fixed** (decided by the rule)
- convention — the review reference's departure paragraph restated write-review's compiled description — `framework/reference/review.md` — **fixed** (decided by the rule)
- convention — 073's data model listed only some of check-departures' Blocking findings — `specs/073-decided-rule-departures-are-recorded/data-model.md` — **fixed** (decided by the rule)
- convention — the changelog split an unreleased primitive across Added and Changed — `runtime/CHANGELOG.md` — **fixed** (decided by the rule)
- convention — the constitution's analysis-record row did not name the waived verdict, as AC6 asks — `framework/constitution.md` — **fixed** (decided by the rule)
- other — an unclosed trailing backtick in a departs: item yields a second rule ID — `runtime/src/primitives/departures.rs` — **waived**: The entry is malformed either way and waives nothing, so the finding it draws is accurate. (decided by the rule)
- other — a fix between a run's two writes leaves the first write's departure waiver stored until the next run — `runtime/src/primitives/write_review.rs` — **waived**: The next run expires the anchor once its rule no longer fires there, as it expires any waiver, and no count reads it in between. (decided by the rule)
- other — the result_sizes case runs check-departures on a corpus with no departure entries, so its row cannot show growth — `runtime/tests/result_sizes.rs` — **waived**: check-departures' result holds one spec's entries and its compiled description keeps it unbounded, so the case pins registration and serialization, not a bound. (decided by the rule)
- other — the constitution's departure bullet names no home for the entry's grammar — `framework/constitution.md` — **waived**: The grammar is check-departures' compiled description and the analyze reference's Departure entries section; the constitution states the principle, with 31 bytes left under its 50,000-byte ceiling. (decided by the rule)
- other — RuleFacts::load re-reads the rule files discover-rule-files already returns — `runtime/src/primitives/check_departures.rs` — **waived**: check-departures is a standalone primitive whose callers pass only a feature; taking another primitive's output as an argument would couple two tools' interfaces to save one read of twelve files. (decided by the rule)
- other — a #[cfg(test)] thread-local sits in rule_verdicts.rs' production source — `runtime/src/primitives/rule_verdicts.rs` — **waived**: It is test instrumentation compiled out of every non-test build, the crate's existing seam for counting git reads. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `README.md` — 048-govern-acquired-runtime
- `docs/runtime.md` — 048-govern-acquired-runtime
- `framework/bootstrap/ductus-procedure.md` — 048-govern-acquired-runtime
- `framework/bootstrap/ductus.md` — 048-govern-acquired-runtime
- `framework/bootstrap/govern.md` — 048-govern-acquired-runtime
- `framework/bootstrap/hooks/ductus-pre-commit` — 048-govern-acquired-runtime
- `framework/templates/project/gitignore` — 048-govern-acquired-runtime
- `scripts/measure-replace-under-server.sh` — 048-govern-acquired-runtime
- `specs/020-code-review/scenarios/waiver-expiry.md` — 020-code-review, 022-deterministic-runtime, 050-constitution, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 082-rule-assessments-converge-across-runs
- `specs/022-deterministic-runtime/data-model.md` — 020-code-review, 022-deterministic-runtime, 050-constitution, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 082-rule-assessments-converge-across-runs
- `specs/022-deterministic-runtime/spec.md` — 020-code-review, 022-deterministic-runtime, 050-constitution, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 082-rule-assessments-converge-across-runs
- `specs/048-govern-acquired-runtime/data-model.md` — 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/plan.md` — 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/scenarios/pointer-follows-the-store.md` — 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/scenarios/state-a-version-checks-the-pin.md` — 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/spec.md` — 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/tasks.md` — 048-govern-acquired-runtime
- `specs/050-constitution/spec.md` — 020-code-review, 022-deterministic-runtime, 050-constitution, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 082-rule-assessments-converge-across-runs
- `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md` — 020-code-review, 022-deterministic-runtime, 050-constitution, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 082-rule-assessments-converge-across-runs
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 020-code-review, 022-deterministic-runtime, 050-constitution, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 082-rule-assessments-converge-across-runs
- `specs/066-audit-records-and-the-completion-gate/spec.md` — 020-code-review, 022-deterministic-runtime, 050-constitution, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 082-rule-assessments-converge-across-runs
- `specs/067-analyze-and-audit-checks/spec.md` — 020-code-review, 022-deterministic-runtime, 050-constitution, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 082-rule-assessments-converge-across-runs
- `specs/082-rule-assessments-converge-across-runs/analysis.md` — 082-rule-assessments-converge-across-runs
- `specs/082-rule-assessments-converge-across-runs/data-model.md` — 020-code-review, 022-deterministic-runtime, 050-constitution, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 082-rule-assessments-converge-across-runs
- `specs/082-rule-assessments-converge-across-runs/plan.md` — 020-code-review, 022-deterministic-runtime, 050-constitution, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 082-rule-assessments-converge-across-runs
- `specs/082-rule-assessments-converge-across-runs/spec.md` — 020-code-review, 022-deterministic-runtime, 050-constitution, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 082-rule-assessments-converge-across-runs

## Unexamined governance

*None.*
