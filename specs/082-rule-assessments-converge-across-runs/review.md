---
spec: 082-rule-assessments-converge-across-runs
last-run: 2026-11-21T21:06:03Z
reviewed-against: 91dd60ea89034d29c11ba9ae1dbfa0b2a3db2ef2
diff-base: 50754fad25305a8747eece8c8f5f4e1ef81fe203
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 24
scope: 27
unread:
  - .claude/commands/ductus/analyze.md
  - .ductus/reference/analyze.md
  - .pi/prompts/ductus-analyze.md
excluded: 17
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 4d2166e42b8f55b378577c4ef2603e04a3a61917d2ab868fdeee11e9aa7b9686
criteria-judged: 7
criteria-met: 7
criteria-verdicts:
  - label: AC1
    digest: 4a0857872309ed58872f075b8025ea392a67d1514aa9073d61358a31b9e879a4
  - label: AC2
    digest: 3cbfe7efe3091bf01d26a64f70eddbf26314a9372889cb0edd902548edc5ee57
  - label: AC3
    digest: 1befb19e8f5346e68ce01c19b43553569afe5562252156bb6544c24ae43f70f0
  - label: AC4
    digest: dd1ed9f021190ade822854187d0d3978039add99e0a0be057dcf54284cf84321
  - label: AC5
    digest: e5ee59d1cbe20e217298ae3af9280a4239ee38b7846777be598fd9dc11cf5523
  - label: AC6
    digest: 495b1c761328f69346efe24636618b4500b4bbb6e4e65cab9b5a65fef1522894
  - label: AC7
    digest: 06feb3f60fef3083ba599a719d6c82e30763134abb86ce53f52d4de175c0becb
blocking: false
dispositions:
  fixed: 8
  deferred: 0
  waived: 13
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
  - key: other — under ductus exec a rule owed a judgment against the changed sections is asked about whole, since the request carries spec.md and no section list, so AC7 holds there by the tally counting no finding at an unchanged section — `runtime/src/interpreter/mod.rs`
    outcome: waived
    reason: "082's plan records it under Trade-offs: a changed-sections field on the request would change 022's request shape at the same request count, and the finding the host returns at an unchanged section is demoted either way."
    decided-at: 2026-11-21T21:05:16Z
    decided-by: rule
  - key: perf — briefs.py's rejudged() and record.py's demoted() rebuild their sets for each rule — `scripts/analyze/briefs.py`
    outcome: waived
    reason: A share holds at most the 201 to 310 loaded rules, so the rebuilt sets cost nothing measurable, and no loaded performance rule governs a local script.
    decided-at: 2026-11-21T21:05:16Z
    decided-by: rule
  - key: other — AC7 names the not-fired and addressed verdicts judged against the changed sections and not the waived one, which the code, plan and reference also judge so — `specs/082-rule-assessments-converge-across-runs/spec.md`
    outcome: waived
    reason: AC7 states what its fixture proves and claims no closed set; a waived verdict is decided by its departure entry (spec 073), and the plan and the analyze reference state its treatment.
    decided-at: 2026-11-21T21:05:16Z
    decided-by: rule
  - key: other — tasks.md records a section-digest measurement (77 specs, median 34, max 586, 1.4 KB) with no method, which the plan's re-measure of the same day replaces — `specs/082-rule-assessments-converge-across-runs/tasks.md`
    outcome: waived
    reason: tasks.md is spent work-tracking and no durable index (constitution, Tasks Phase); plan.md holds the figure with its method and units.
    decided-at: 2026-11-21T21:05:16Z
    decided-by: rule
---

# Review — 082-rule-assessments-converge-across-runs

## Summary

Five-pass review of 082's reopening for AC7 (diff-base 50754fad, base-source reopen), passes over the tree at 91dd60ea, whose code change is 70c4f3da. Two reviewers in parallel: part A followed the runtime behaviors (section digests and verdict currency, write-analysis' carry and demotion, discover-rule-files' verdicts, the exec walker and tally) across the eight runtime files by their 50754fad..70c4f3da hunks and the code each behavior needed, drove edge cases on fixtures with the release binary (duplicate headings, a fenced heading, a heading holding : # and ", CRLF, an unparseable section-digest), applied all nine loaded rule files whole (285,584 bytes, each read against wc -c) and judged AC1 to AC7; part B ran the passes over the scripts and checked 082's plan and data model whole and the prose's claims against runtime/src, with part A's rule files applied to the scripts' security and efficiency passes. Examined 24 of the 27 scope entries; not counted: the three generated copies (.claude/commands/ductus/analyze.md, .ductus/reference/analyze.md, .pi/prompts/ductus-analyze.md), confirmed in sync by scripts/gen-claude-commands.sh --check. Fixed in this run: BE-INPUT-004 (MUST, low confidence: cite_resolves now refuses a link at any component of a citation's path, M40), two QUAL-TEST-001 (SHOULD: cases AM and AN in scripts/tests/test-analyze-scripts.py, each proved failing with its behavior removed), and two broken AC7 contracts (a section key is written unflattened, M38; a reordering of sections leaves every verdict current, M39). BE-CACHE-002 fires as before and the owner's stored waiver applies. The deleted path, specs/083-review-and-analysis-check-each-other/spec.md, is the retired 083's, removed by c5da762d, and no pass read it.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

7 of 7 acceptance criteria judged met.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

### WAIVED: BE-CACHE-002 — the stored-verdict currency key leaves out specs/system.md

- **File**: `runtime/src/primitives/rule_verdicts.rs`
- **Rule**: Cache keys SHOULD incorporate every input that affects the cached value, including the authorization scope (user, tenant, role) when the value is principal-specific, entered as an identifier such as a user or tenant ID and never as a session token, API key, or other secret.
- **Finding**: A stored rule verdict is reused while its sections, its rule file and the loaded rule-file set are unchanged; about three rules' Verifications judge against specs/system.md, which is not an input, so an edit there leaves their verdicts current.
- **Auto-fixable**: no
- **Waived**: The owner's choice at this review (2026-11-12, option a): the loaded rule-file set joined the currency key, and specs/system.md stays out of it, since every edit there would re-judge every rule of every spec to protect about three rules. 082's Resolved Questions record the decision.

## Deferred findings

*None.*

## Acceptance criteria

- AC1 — met: rule_verdicts::standing reuses verdicts over unchanged subjects; tests a_second_run_over_an_unchanged_spec (write_analysis.rs), a_second_call_over_an_unchanged_spec_lists_no_rule_to_judge (discover_rule_files.rs) and the exec AC1 walk in interpreter/mod.rs
- AC2 — met: write_analysis::derive_rule_fields pushes unjudged rules into rule-assessments-not-checked on both paths; tests 'Spec 082, AC2' in write_analysis.rs and interpreter/mod.rs
- AC3 — met: write_analysis contradiction handling with Standing::demoted and rule_verdicts::contradiction; superseded in part by AC7 as the criterion says; tests a_finding_after_a_quiet_or_addressed_verdict_is_a_recorded_contradiction and a_finding_at_an_unchanged_section_..._holds_nothing
- AC4 — met: section-digest is in framework/constitution.md's analysis-record row, 066's data-model.md and 082's data-model.md; AnalyzeBlock.section_digest round-trips in the AC4 test in write_analysis.rs; an unparseable section-digest driven on the release binary gave record-unreadable
- AC5 — met: rule-digest per file in standing(); tests 'Spec 082, AC5' in write_analysis.rs and interpreter/mod.rs (not changed in this window, read only)
- AC6 — met: write_analysis::derive_rule_fields sends an addressed verdict whose cite fails rule_verdicts::cite_resolves to unjudged; tests 'Spec 082, AC6' in write_analysis.rs and interpreter/mod.rs
- AC7 — met: analyze_subjects::file_sections/section_digest, rule_verdicts::section_currency/Standing::demoted, write_analysis carry/demotion, tally record_assessment; fixture a_plan_section_edit_leaves_a_not_fired_verdict_current runs with and without the digests, plus exec and discover tests; discover-rule-files driven on a fixture with duplicate headings, a fenced heading, a YAML-significant heading and CRLF reported only the changed sections

## Observations

- bug — cite_resolves matches a heading's text exactly, so a closing # sequence, a setext heading or an indented heading cannot be cited by its plain text — `runtime/src/primitives/rule_verdicts.rs` — **waived**: It uses the shared heading_lines reader resolve-anchor uses, as 082's Resolved Questions specify; an unresolved citation is recorded unjudged, never lost. (decided by the rule)
- perf — cite_resolves re-reads the cited file for each addressed verdict, and currency scans the rules linearly — `runtime/src/primitives/rule_verdicts.rs` — **waived**: Negligible at 310 rules; each read is one small design subject of one spec. (decided by the rule)
- other — scenarios/./s.md and scenarios//s.md resolve as citations — `runtime/src/primitives/rule_verdicts.rs` — **waived**: Both name the same file the plain path does. (decided by the rule)
- other — a judged-rules entry naming no loaded rule is ignored, while an unknown rule-verdicts key is refused — `runtime/src/primitives/write_analysis.rs` — **waived**: The typo leaves the real rule named unjudged, which the gate holds at done, so nothing is lost or passed. (decided by the rule)
- simplicity — check_new_verdicts repeats the rule lookup standing.currency already does — `runtime/src/primitives/write_analysis.rs` — **waived**: One map lookup per verdict; merging them would couple the refusal to the currency walk. (decided by the rule)
- other — an exec finding stores at as spec.md#<section> whichever subject the host found it in — `runtime/src/interpreter/analyze_tally.rs` — **waived**: The assessSpecQuality response carries only a spec section; at is where the report points, and a reused finding is reported by its stored message. (decided by the rule)
- other — an exec step asking about nothing records one rule-assessments-not-checked beside the unjudged rules write-analysis names — `runtime/src/interpreter/analyze_tally.rs` — **waived**: 082's plan Trade-offs record the overlap: it overstates the gap, never hides one. (decided by the rule)
- other — a passed: true response with a blank addressed-by reads as not fired, not as an unresolved citation — `runtime/src/interpreter/analyze_tally.rs` — **waived**: 082's data model: without addressed-by a pass is not fired, and a blank citation is none. (decided by the rule)
- other — split_reused runs before the loaded-none and tier checks, so a procedure repeating a tier step would count reused findings twice — `runtime/src/interpreter/mod.rs` — **waived**: Unproven, and analyze.md has one assessSpecQuality step per tier. (decided by the rule)
- other — under ductus exec a rule owed a judgment against the changed sections is asked about whole, since the request carries spec.md and no section list, so AC7 holds there by the tally counting no finding at an unchanged section — `runtime/src/interpreter/mod.rs` — **waived**: 082's plan records it under Trade-offs: a changed-sections field on the request would change 022's request shape at the same request count, and the finding the host returns at an unchanged section is demoted either way. (decided by the rule)
- bug — write-analysis flattened each section-digest key through single_line, so a heading holding a line-hazard character such as U+2028 read as changed on every run — `runtime/src/primitives/write_analysis.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug — a pure reordering of sections changed the whole-file digest and no section digest, so every quiet rule was owed a judgment of no changed section and was recorded unjudged — `runtime/src/primitives/rule_verdicts.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- perf — briefs.py's rejudged() and record.py's demoted() rebuild their sets for each rule — `scripts/analyze/briefs.py` — **waived**: A share holds at most the 201 to 310 loaded rules, so the rebuilt sets cost nothing measurable, and no loaded performance rule governs a local script. (decided by the rule)
- other — record.py's demoted() treats a heading changed-sections does not name as unchanged, relying unstated on unresolved_cites having refused an at that does not resolve — `scripts/analyze/record.py` — **fixed** (decided by the rule)
- other — 082's data model said a rule absent from reusable and judge is current, false for a quiet rule in rejudge — `specs/082-rule-assessments-converge-across-runs/data-model.md` — **fixed** (decided by the rule)
- other — the analyze reference said a verdict is never passed for a rule verdicts.reusable lists, though a rule in rejudge may be in both and its new finding at a changed section is passed — `framework/reference/analyze.md` — **fixed** (decided by the rule)
- other — docs/analyze.md said a changed rule file judges every rule again, where only its own rules are, and every rule only when a file is added or removed — `docs/analyze.md` — **fixed** (decided by the rule)
- other — 082's plan still counted three record fields in its Data Model section and the docs/analyze.md row, and five Resolved Questions where the spec holds ten — `specs/082-rule-assessments-converge-across-runs/plan.md` — **fixed** (decided by the rule)
- other — 082's plan said only a finding at a changed section of spec.md counts under ductus exec, though a finding naming no section, recorded at bare spec.md, always counts — `specs/082-rule-assessments-converge-across-runs/plan.md` — **fixed** (decided by the rule)
- other — AC7 names the not-fired and addressed verdicts judged against the changed sections and not the waived one, which the code, plan and reference also judge so — `specs/082-rule-assessments-converge-across-runs/spec.md` — **waived**: AC7 states what its fixture proves and claims no closed set; a waived verdict is decided by its departure entry (spec 073), and the plan and the analyze reference state its treatment. (decided by the rule)
- other — tasks.md records a section-digest measurement (77 specs, median 34, max 586, 1.4 KB) with no method, which the plan's re-measure of the same day replaces — `specs/082-rule-assessments-converge-across-runs/tasks.md` — **waived**: tasks.md is spent work-tracking and no durable index (constitution, Tasks Phase); plan.md holds the figure with its method and units. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md`
- `specs/020-code-review/spec.md`
- `specs/020-code-review/tasks.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/050-constitution/spec.md`
- `specs/055-shared-constitution/spec.md`
- `specs/055-shared-constitution/tasks.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md`
- `specs/079-review-and-analysis-work-converges/research.md`
- `specs/079-review-and-analysis-work-converges/spec.md`

## Unexamined governance

*None.*
