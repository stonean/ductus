---
spec: 082-rule-assessments-converge-across-runs
last-run: 2026-11-12T20:51:35Z
reviewed-against: 8974812c9c5ec73cba45e570515bfdd8deda69c3
diff-base: cca8ffb09614fb66144fad76b0c3fa3b3fa4d45c
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 27
scope: 31
excluded: 76
contract-breaks: 0
skipped-passes: []
fix-rounds: 3
re-reads: 2
diff-checked: 1
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: f2342da1234ffcbc2c2f76156b21515970299eff57dc0ba3a2e29dccf32c29e8
blocking: false
dispositions:
  fixed: 12
  deferred: 0
  waived: 14
  undispositioned: 0
waivers:
  - rule: BE-CACHE-002
    file: runtime/src/primitives/rule_verdicts.rs
    reason: "The owner's choice at this review (2026-11-12, option a): the loaded rule-file set joined the currency key, and specs/system.md stays out of it, since every edit there would re-judge every rule of every spec to protect about three rules. 082's Resolved Questions record the decision."
    waived-at: 2026-11-12T20:51:35Z
    waived-by: andy@stone.dev
decisions:
  - key: "bug — cite_resolves matches a heading's text exactly, so a closing # sequence, a setext heading or an indented heading cannot be cited by its plain text — `runtime/src/primitives/rule_verdicts.rs`"
    outcome: waived
    reason: It uses the shared heading_lines reader resolve-anchor uses, as 082's Resolved Questions specify; an unresolved citation is recorded unjudged, never lost.
    decided-at: 2026-11-12T20:51:35Z
    decided-by: rule
  - key: perf — cite_resolves re-reads the cited file for each addressed verdict, and currency scans the rules linearly — `runtime/src/primitives/rule_verdicts.rs`
    outcome: waived
    reason: Negligible at 310 rules; each read is one small design subject of one spec.
    decided-at: 2026-11-12T20:51:35Z
    decided-by: rule
  - key: other — scenarios/./s.md and scenarios//s.md resolve as citations — `runtime/src/primitives/rule_verdicts.rs`
    outcome: waived
    reason: Both name the same file the plain path does.
    decided-at: 2026-11-12T20:51:35Z
    decided-by: rule
  - key: other — a judged-rules entry naming no loaded rule is ignored, while an unknown rule-verdicts key is refused — `runtime/src/primitives/write_analysis.rs`
    outcome: waived
    reason: The typo leaves the real rule named unjudged, which the gate holds at done, so nothing is lost or passed.
    decided-at: 2026-11-12T20:51:35Z
    decided-by: rule
  - key: simplicity — check_new_verdicts repeats the rule lookup standing.currency already does — `runtime/src/primitives/write_analysis.rs`
    outcome: waived
    reason: One map lookup per verdict; merging them would couple the refusal to the currency walk.
    decided-at: 2026-11-12T20:51:35Z
    decided-by: rule
  - key: other — the gate holds done on an analysis naming an unjudged rule whose file was later removed, until the next analysis — `runtime/src/primitives/check_review_gate.rs`
    outcome: waived
    reason: The next analysis drops a removed rule; the gate reads the record it is given, as every gate check does.
    decided-at: 2026-11-12T20:51:35Z
    decided-by: rule
  - key: other — the gate does not compare an analysis record's rule-digest with the rule files at done — `runtime/src/primitives/check_review_gate.rs`
    outcome: waived
    reason: Analysis freshness is over the spec's subjects by 066's design; a changed rule is judged again at the next analysis, as AC5 requires.
    decided-at: 2026-11-12T20:51:35Z
    decided-by: rule
  - key: other — an exec finding stores at as spec.md#<section> whichever subject the host found it in — `runtime/src/interpreter/analyze_tally.rs`
    outcome: waived
    reason: The assessSpecQuality response carries only a spec section; at is where the report points, and a reused finding is reported by its stored message.
    decided-at: 2026-11-12T20:51:35Z
    decided-by: rule
  - key: other — an exec step asking about nothing records one rule-assessments-not-checked beside the unjudged rules write-analysis names — `runtime/src/interpreter/analyze_tally.rs`
    outcome: waived
    reason: "082's plan Trade-offs record the overlap: it overstates the gap, never hides one."
    decided-at: 2026-11-12T20:51:35Z
    decided-by: rule
  - key: "other — a passed: true response with a blank addressed-by reads as not fired, not as an unresolved citation — `runtime/src/interpreter/analyze_tally.rs`"
    outcome: waived
    reason: "082's data model: without addressed-by a pass is not fired, and a blank citation is none."
    decided-at: 2026-11-12T20:51:35Z
    decided-by: rule
  - key: convention — feature-directory resolution (validate_feature_argument then is_dir) appears in write-analysis, discover-rule-files and the exec payload — `runtime/src/interpreter/payload.rs`
    outcome: waived
    reason: Each goes through the shared validate_feature_argument; what repeats is one is_dir check.
    decided-at: 2026-11-12T20:51:35Z
    decided-by: rule
  - key: other — split_reused runs before the loaded-none and tier checks, so a procedure repeating a tier step would count reused findings twice — `runtime/src/interpreter/mod.rs`
    outcome: waived
    reason: Unproven, and analyze.md has one assessSpecQuality step per tier.
    decided-at: 2026-11-12T20:51:35Z
    decided-by: rule
  - key: other — a record written before 082 carries no rule-digest and passes the gate with nothing saying its rules were judged — `runtime/src/primitives/check_review_gate.rs`
    outcome: waived
    reason: "082's plan: such a record reuses nothing, and the spec's next analysis writes the fields."
    decided-at: 2026-11-12T20:51:35Z
    decided-by: rule
  - key: other — runtime/tests/result_sizes.rs measures write-analysis with no contradictions, since its harness calls each primitive with default arguments — `runtime/tests/result_sizes.rs`
    outcome: waived
    reason: A contradiction needs a prior record and rule files; the two bounded-form unit tests in write_analysis.rs cover both forms.
    decided-at: 2026-11-12T20:51:35Z
    decided-by: rule
---

# Review — 082-rule-assessments-converge-across-runs

## Summary

Five-pass review of 082 over its reopening window (`cca8ffb0`, base-source reopen); the passes read the tree at `8a599af9`, and this run's fixes are committed in `f54c1016` (prose, data models, generated copies) and `8974812c` (runtime). Method: 082's own change, its 20 commits from `31aeed91` to `57820d27` (+3,445 / −226 lines over 36 files), read with `git log -p` per file, other specs' commits in the same files left out. Four reviewers each followed one behavior across its files and ran all five passes over it (verdict currency; write-analysis' fields and refusals; the exec walker and the completion gate; the pipeline prose and 082's data model checked whole against the code), probing edge cases on fixtures in clones; a fifth applied each of the 203 loaded rules (nine files, backend plus cross) one at a time, Statement, Verification and Not-flagged clause. Every finding was re-checked against the tree before it was recorded. Coverage, from each reviewer's pass-by-pass account: the security pass applied the security rule files through the fifth reviewer, the other four reasoned from the code; extensions.rs and the prose files took no security or reuse pass, holding no logic; main.rs and descriptions.rs were read in 082's diffs only. Examined 27 of the 31 scope entries, most in 082's parts rather than whole. Not counted: the three generated copies (.claude/commands/ductus/analyze.md, .ductus/reference/analyze.md, .pi/prompts/ductus-analyze.md), confirmed by `scripts/gen-claude-commands.sh --check` and regenerated in sync after the fixes; and runtime/src/primitives/resolve_constitutions.rs, changed only by this run's fix, which the first fix round's re-read read. This run's edits to 022's and 066's data models and to 060's spec and plan, committed under a subject naming 082, are those specs' own (spec 081) and are listed under Excluded from scope; round 1's re-read read them. Fixed in this run: BE-PAYLOAD-001 (MUST; write-analysis' contradictions now bounded through bounded_lists::place), QUAL-CLAIM-001 (SHOULD; a bare unexamined total is recorded under the new reason unclassified instead of being replaced by the derived breakdown), CFG-CONST-001 (MUST; the shared unexamined reason keys are constants in write_analysis.rs), BE-COMPAT-001 (MUST; 082's plan now classifies each interface change), and BE-CACHE-002's loaded-rule-file part (the set joined the currency key, reason rule-files-changed); its specs/system.md part is waived by the owner. One broken contract was fixed: the exec walk's abort on a mislabelled single-tier finding. 060's AC5 was corrected by the owner's choice. Each fix has a test proved red; five new mutants (M23 to M27) are killed and `run.py --check` passes on 27; the whole cargo test passed, 36 suites, 3131 tests.

3 fix round(s): 2 re-read, 1 diff-checked, 0 exempt, 0 not re-read.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

### WAIVED: BE-CACHE-002 — a stored verdict's currency key leaves out specs/system.md

- **File**: `runtime/src/primitives/rule_verdicts.rs`
- **Rule**: Cache keys SHOULD incorporate every input that affects the cached value
- **Finding**: A stored rule verdict is reused while the design subjects, its rule file and (since this review's fix) the loaded rule-file set are unchanged. About three rules' Verifications (BE-APIVER-001, BE-REPR-001, BE-REPR-002) judge against specs/system.md, which is not an input, so an edit there leaves their verdicts current.
- **Auto-fixable**: no
- **Waived**: The owner's choice at this review (2026-11-12, option a): the loaded rule-file set joined the currency key, and specs/system.md stays out of it, since every edit there would re-judge every rule of every spec to protect about three rules. 082's Resolved Questions record the decision.

## Deferred findings

*None.*

## Observations

- bug — a host labelling a single-tier rule's finding in the other tier aborted the whole ductus exec analyze at write-analysis' tier refusal, writing no record; the tally now counts it in the rule's tier, honoring the label only for a rule carrying both MUST and SHOULD — `runtime/src/interpreter/analyze_tally.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- convention — stale doc comments: RuleVerdict::Finding said the tier is not stored, WriteAnalysisArgs::rule_verdicts omitted the required tier, and derive_rule_fields and the floor comment said the floor reads the rule text's tier — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- convention — write-analysis' refusal comment and WRITE_ANALYSIS overstated what is refused: a prior record whose rule fields alone do not parse is overwritten with every rule judged, and an unlistable rule directory is an error — `runtime/src/primitives/write_analysis.rs` — **fixed** (decided by the rule)
- convention — DISCOVER_RULE_FILES said every reusable entry carries a tier and that a rule in neither list is current; a rule of an unreadable file is in neither and is named under unreadable — `runtime/src/schema/descriptions.rs` — **fixed** (decided by the rule)
- convention — 082's Resolved Question on contradictions placed them under ductus exec or a forced re-judge, which neither path produces — `specs/082-rule-assessments-converge-across-runs/spec.md` — **fixed** (decided by the rule)
- convention — 082's data model gave the verdicts block with rules and feature, where all is required too, and showed reason as null where it is absent — `specs/082-rule-assessments-converge-across-runs/data-model.md` — **fixed** (decided by the rule)
- convention — 022's data model said verdicts.reason is absent when some verdict is reusable; it is absent whenever the prior verdicts can be reused — `specs/022-deterministic-runtime/data-model.md` — **fixed** (decided by the rule)
- convention — 066's data model omitted write-analysis' refusal of a finding in a tier its rule cannot fail in — `specs/066-audit-records-and-the-completion-gate/data-model.md` — **fixed** (decided by the rule)
- convention — docs/analyze.md's record field table and example lacked rule-digest, rule-verdicts and unjudged-rules — `docs/analyze.md` — **fixed** (decided by the rule)
- convention — the mutation list's header mapped criteria to mutants but omitted M8, M11 to M15, M19 and M20 — `scripts/mutation/rule-verdicts-converge.py` — **fixed** (decided by the rule)
- convention — the gate's unjudged-rules block shows three names and its guidance did not say analysis.md's unjudged-rules holds them all; analyze_gate_block's doc comment gave stale check numbers — `runtime/src/primitives/check_review_gate.rs` — **fixed** (decided by the rule)
- other — 060's ticked AC5 said an exec analyze asks about every loaded rule, false since 082 reuses current verdicts; its summary line and plan said the same — `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md` — **fixed**
- bug — cite_resolves matches a heading's text exactly, so a closing # sequence, a setext heading or an indented heading cannot be cited by its plain text — `runtime/src/primitives/rule_verdicts.rs` — **waived**: It uses the shared heading_lines reader resolve-anchor uses, as 082's Resolved Questions specify; an unresolved citation is recorded unjudged, never lost. (decided by the rule)
- perf — cite_resolves re-reads the cited file for each addressed verdict, and currency scans the rules linearly — `runtime/src/primitives/rule_verdicts.rs` — **waived**: Negligible at 310 rules; each read is one small design subject of one spec. (decided by the rule)
- other — scenarios/./s.md and scenarios//s.md resolve as citations — `runtime/src/primitives/rule_verdicts.rs` — **waived**: Both name the same file the plain path does. (decided by the rule)
- other — a judged-rules entry naming no loaded rule is ignored, while an unknown rule-verdicts key is refused — `runtime/src/primitives/write_analysis.rs` — **waived**: The typo leaves the real rule named unjudged, which the gate holds at done, so nothing is lost or passed. (decided by the rule)
- simplicity — check_new_verdicts repeats the rule lookup standing.currency already does — `runtime/src/primitives/write_analysis.rs` — **waived**: One map lookup per verdict; merging them would couple the refusal to the currency walk. (decided by the rule)
- other — the gate holds done on an analysis naming an unjudged rule whose file was later removed, until the next analysis — `runtime/src/primitives/check_review_gate.rs` — **waived**: The next analysis drops a removed rule; the gate reads the record it is given, as every gate check does. (decided by the rule)
- other — the gate does not compare an analysis record's rule-digest with the rule files at done — `runtime/src/primitives/check_review_gate.rs` — **waived**: Analysis freshness is over the spec's subjects by 066's design; a changed rule is judged again at the next analysis, as AC5 requires. (decided by the rule)
- other — an exec finding stores at as spec.md#<section> whichever subject the host found it in — `runtime/src/interpreter/analyze_tally.rs` — **waived**: The assessSpecQuality response carries only a spec section; at is where the report points, and a reused finding is reported by its stored message. (decided by the rule)
- other — an exec step asking about nothing records one rule-assessments-not-checked beside the unjudged rules write-analysis names — `runtime/src/interpreter/analyze_tally.rs` — **waived**: 082's plan Trade-offs record the overlap: it overstates the gap, never hides one. (decided by the rule)
- other — a passed: true response with a blank addressed-by reads as not fired, not as an unresolved citation — `runtime/src/interpreter/analyze_tally.rs` — **waived**: 082's data model: without addressed-by a pass is not fired, and a blank citation is none. (decided by the rule)
- convention — feature-directory resolution (validate_feature_argument then is_dir) appears in write-analysis, discover-rule-files and the exec payload — `runtime/src/interpreter/payload.rs` — **waived**: Each goes through the shared validate_feature_argument; what repeats is one is_dir check. (decided by the rule)
- other — split_reused runs before the loaded-none and tier checks, so a procedure repeating a tier step would count reused findings twice — `runtime/src/interpreter/mod.rs` — **waived**: Unproven, and analyze.md has one assessSpecQuality step per tier. (decided by the rule)
- other — a record written before 082 carries no rule-digest and passes the gate with nothing saying its rules were judged — `runtime/src/primitives/check_review_gate.rs` — **waived**: 082's plan: such a record reuses nothing, and the spec's next analysis writes the fields. (decided by the rule)
- other — runtime/tests/result_sizes.rs measures write-analysis with no contradictions, since its harness calls each primitive with default arguments — `runtime/tests/result_sizes.rs` — **waived**: A contradiction needs a prior record and rule files; the two bounded-form unit tests in write_analysis.rs cover both forms. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.ductus/reference/clarify.md` — 013-text-first-artifacts, 022-deterministic-runtime, 041-task-pruning
- `.ductus/reference/specify.md` — 013-text-first-artifacts, 022-deterministic-runtime, 041-task-pruning
- `AGENTS.md` — 050-constitution
- `framework/migrations/criterion-label-backfill.md` — 013-text-first-artifacts, 022-deterministic-runtime, 041-task-pruning
- `framework/reference/clarify.md` — 013-text-first-artifacts, 022-deterministic-runtime, 041-task-pruning
- `framework/reference/specify.md` — 013-text-first-artifacts, 022-deterministic-runtime, 041-task-pruning
- `runtime/src/primitives/check_artifacts.rs` — 022-deterministic-runtime, 045-decision-state-drift-detection, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `runtime/src/primitives/label_criteria.rs` — 013-text-first-artifacts, 022-deterministic-runtime, 041-task-pruning
- `runtime/src/primitives/measure_reopenings.rs` — 079-review-and-analysis-work-converges
- `runtime/src/primitives/read_spec.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/src/primitives/read_tasks.rs` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `runtime/tests/mcp.rs` — 013-text-first-artifacts, 022-deterministic-runtime, 041-task-pruning, 079-review-and-analysis-work-converges
- `scripts/mutation/outward-link-reads-067.py` — 067-analyze-and-audit-checks
- `specs/013-text-first-artifacts/scenarios/criterion-identifiers.md` — 013-text-first-artifacts, 022-deterministic-runtime, 041-task-pruning
- `specs/019-config-decisions/data-model.md` — 019-config-decisions, 079-review-and-analysis-work-converges
- `specs/019-config-decisions/spec.md` — 019-config-decisions, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/data-model.md` — 013-text-first-artifacts, 022-deterministic-runtime, 041-task-pruning, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/plan.md` — 022-deterministic-runtime, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/022-deterministic-runtime/scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/scenarios/a-path-argument-resolves-inside-its-base.md` — 022-deterministic-runtime, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/022-deterministic-runtime/scenarios/append-task-honours-slug-with-explicit-body.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/scenarios/ask-consolidation.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/scenarios/check-stuck-tasks-md-advancement.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/scenarios/coverage-expansion-primitives.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/scenarios/criterion-label-assignment.md` — 013-text-first-artifacts, 022-deterministic-runtime, 041-task-pruning
- `specs/022-deterministic-runtime/scenarios/dashboard-primitive.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/scenarios/derive-boundary-uncommitted-spec-dir.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/scenarios/framework-list-dedup.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/scenarios/scaffolding-primitives.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/spec.md` — 022-deterministic-runtime, 050-constitution, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/023-govern-refinement/scenarios/configure-retires-formerly-canonical-entries.md` — 023-govern-refinement, 026-framework-self-audit, 041-task-pruning, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `specs/026-framework-self-audit/scenarios/family-39-shipped-anchor-resolution.md` — 023-govern-refinement, 026-framework-self-audit, 041-task-pruning, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `specs/026-framework-self-audit/scenarios/family-40-deprecated-rules-are-removed-on-time.md` — 023-govern-refinement, 026-framework-self-audit, 041-task-pruning, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `specs/030-cross-service-references/data-model.md` — 030-cross-service-references, 079-review-and-analysis-work-converges
- `specs/030-cross-service-references/spec.md` — 030-cross-service-references, 079-review-and-analysis-work-converges
- `specs/041-task-pruning/data-model.md` — 013-text-first-artifacts, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 041-task-pruning, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `specs/045-decision-state-drift-detection/data-model.md` — 045-decision-state-drift-detection, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/045-decision-state-drift-detection/spec.md` — 045-decision-state-drift-detection, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/048-govern-acquired-runtime/tasks.md` — 048-govern-acquired-runtime, 079-review-and-analysis-work-converges
- `specs/050-constitution/plan.md` — 050-constitution
- `specs/050-constitution/spec.md` — 022-deterministic-runtime, 050-constitution, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/050-constitution/tasks.md` — 050-constitution
- `specs/051-branch-scoped-spec-numbering/data-model.md` — 051-branch-scoped-spec-numbering, 079-review-and-analysis-work-converges
- `specs/051-branch-scoped-spec-numbering/scenarios/a-fold-target-is-held-to-the-name-grammar.md` — 051-branch-scoped-spec-numbering, 079-review-and-analysis-work-converges
- `specs/051-branch-scoped-spec-numbering/spec.md` — 051-branch-scoped-spec-numbering, 079-review-and-analysis-work-converges
- `specs/055-shared-constitution/data-model.md` — 055-shared-constitution, 079-review-and-analysis-work-converges
- `specs/055-shared-constitution/spec.md` — 055-shared-constitution, 079-review-and-analysis-work-converges
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md` — 022-deterministic-runtime, 050-constitution, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/062-concurrent-session-targets/data-model.md` — 062-concurrent-session-targets, 079-review-and-analysis-work-converges
- `specs/062-concurrent-session-targets/spec.md` — 062-concurrent-session-targets, 079-review-and-analysis-work-converges
- `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md` — 023-govern-refinement, 026-framework-self-audit, 041-task-pruning, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 022-deterministic-runtime, 045-decision-state-drift-detection, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/scenarios/only-gating-findings-hold-done.md` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/scenarios/process-decisions-results-have-a-bounded-size.md` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-scope-includes-uncommitted-work.md` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/scenarios/write-analysis-and-the-second-gate.md` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/spec.md` — 022-deterministic-runtime, 050-constitution, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/067-analyze-and-audit-checks/data-model.md` — 022-deterministic-runtime, 045-decision-state-drift-detection, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/067-analyze-and-audit-checks/plan.md` — 022-deterministic-runtime, 045-decision-state-drift-detection, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/adopter-corpus-link-integrity.md` — 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/067-analyze-and-audit-checks/scenarios/analyze-artifact-checks.md` — 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/067-analyze-and-audit-checks/scenarios/check-artifacts-skipped-targets.md` — 045-decision-state-drift-detection, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md` — 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/067-analyze-and-audit-checks/scenarios/sibling-symlink-trust-boundary.md` — 022-deterministic-runtime, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/spec-integrity-and-plan-consistency-are-run.md` — 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/067-analyze-and-audit-checks/tasks.md` — 067-analyze-and-audit-checks
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md` — 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/scenarios/fix-round-results-have-a-bounded-size.md` — 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/data-model.md` — 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/plan.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/spec.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/tasks.md` — 048-govern-acquired-runtime, 079-review-and-analysis-work-converges
- `specs/080-deterministic-runtime-efficiency/data-model.md` — 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/spec.md` — 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency

## Unexamined governance

*None.*
