---
status: draft
dependencies: [020-code-review, 073-decided-rule-departures-are-recorded, 078-review-fix-rounds-are-re-read-within-a-bound, 082-rule-assessments-converge-across-runs]
cross-spec-impact: [020-code-review, 050-constitution, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 082-rule-assessments-converge-across-runs]
next-criterion: 9
---

# 083 — Review and analysis check each other

A fix made for one audit's finding is judged by both audits' criteria in the run that makes it. A review records a verdict per rule, as an analysis does. And a rule one audit judges to apply, which the other judges out of reach, is reported and settled in the run that finds the disagreement, so it never returns a cycle later as the other audit's blocker.

## Motivation

An adopter reported on 2026-11-21, at ductus 0.60.0, that one spec had taken five review runs and two analysis runs and still owed an analysis, each run finding something new. Its code was a markdown procedure an agent follows, reviewed against backend rule files. No pass stopped at its first finding: every record held several. The adopter's records were not readable here, so the sequence below is the report's.

- **A fix was judged by the other audit only a cycle later.** The first analysis found a check-then-act race (`BE-RACE-003`); its task added "a failed add re-reads the file and checks again" to the plan and the procedure. The next review read that sentence and recorded nothing against it. The second analysis judged it an unbounded retry (`BE-RETRY-001`, a MUST), and the fix it offered, which the owner chose, removed the re-check. The next review recorded the removal as a broken contract against the spec's Behavior: without the re-check, an approved add can write a second copy, the race the first analysis had found.
- **The two audits judged rule reach apart.** The second review's Summary said no rule's Statement reached the procedure. The first analysis, over the design that procedure implements, found four MUSTs. The running fifth review treats thirteen backend rules as candidates for the same procedure.

Read at `d39cbdd8`, the framework had these gaps:

- **The fix-round loop covered review's own fixes only.** [078](../078-review-fix-rounds-are-re-read-within-a-bound/spec.md)'s Resolved Questions excluded analyze's fixes as "mechanical chores its step-20 re-check re-detects". A gating analysis finding is never a chore: it was deferred to a task, which no re-check read. A round's brief carried the spec, its data model and scenarios, and no rule (`ductus brief-re-read --help`), so a review's fix met the rules only at the next analysis.
- **An analysis's fix met the spec's contract only at the next review.** `framework/reference/analyze.md` §Plan consistency (blocking if plan exists) held the plan against `specs/system.md` alone, never against the spec's Behavior or criteria, and step 20 of `framework/commands/analyze.md` left a gating finding first produced by its re-check "undispositioned, for the next run to decide".
- **A review kept no per-rule verdict.** §Load rules in `framework/reference/review.md` ([020](../020-code-review/spec.md)'s task 14) said "a Statement the code does not reach records nothing", so a blanket dismissal and a judgment of every rule wrote the same record. [082](../082-rule-assessments-converge-across-runs/spec.md) had made the analysis's rule judgments persist and converge across analysis runs, and nothing compared them with a review's. Review read nothing from `analysis.md` but its freshness (`write-review`'s `analyze-freshness`), and the only input the two audits shared was the departure entries of [073](../073-decided-rule-departures-are-recorded/spec.md).

Replayed with this spec's behavior, measured on the report's sequence in audit runs (one `/{project}:review` or `/{project}:analyze` run that wrote a record). The rule: a defect is found at the first run this spec obliges to judge its rule, or the contract, against the text holding it, and a judge given the same rule and text reaches the verdict the later run reached (082's premise). The sequence took at least 8 runs, 5 reviews and 3 analyses, the third owed after the running review. Gap 1's behavior alone closes it in 6, since the first analysis judges its own fix of `BE-RACE-003` and the fix that reopened the race is never made. Gap 2's alone closes it in 7 when a review records a verdict for every loaded rule, the third review judging `BE-RETRY-001` against the procedure, and in no fewer than 8 when it records only the rules the analysis fired, since no analysis had fired that rule by then. Both together close it in 6 where the second review still judges the rules out of reach, and in 3 where it reaches them, as the running review does. Two defects outside both gaps, dated line citations and a lint that never ran reading as clean, sat in text earlier reviews had read whole. They stay where those reviews left them: this spec changes what a run is obliged to judge, not whether a judge sees what it reads.

## Behavior

- **A review records a verdict per rule.** Each rule a review judges against code is recorded with its verdict, as an analysis records its own (082), so a rule left unjudged is named and never reads as one that did not reach the code.
- **Each audit judges the rules the other found to apply.** A rule the analysis record fired against the design is judged by the next review against the code that implements it, wherever that code lies, as the criteria step judges a criterion. A rule a review found violated is judged again by the next analysis rather than taken from a stored verdict.
- **A disagreement is reported and settled.** A rule the analysis fired against the design, which a review judged not to reach the code, is a disagreement. Whichever audit writes second derives it from the other's record and names both verdicts and their citations, and the next run of either audit judges that rule again.
- **A fix is judged by both audits in the run that makes it.** A review's fix is judged against the loaded rules before its record is final, as it is against the spec. An analysis's fix for a gating finding on the spec's design is written in the run once confirmed, so the run's re-check judges the fixed text against every loaded rule and against the spec's Behavior and criteria. A gating finding that a run's own fix caused is dispositioned in that run.
- **Adopters get it.** Each change lands in what a `ductus-v<version>` release ships: the runtime, `framework/commands/`, `framework/reference/` and the constitution.

## Acceptance Criteria

- [ ] AC1: A review record holds a verdict for each rule the spec's analysis record fired, `finding` or `addressed`, judged against the code that implements the cited design wherever it lies: `violation`, `holds` with a code citation, or `not-reached` with a reason. A fired rule with no verdict is named in the record and holds `done`, and a fixture proves both directions.
- [ ] AC2: A rule the analysis record fired that the review record judged `not-reached` is a disagreement. `write-review` and `write-analysis` each derive it from the other record and name both verdicts and their citations, and the next run of either audit judges that rule again rather than reusing a stored verdict. A fixture reproduces it, an analysis firing a MUST rule at a plan section and a review judging that rule not reached, and the check reports it before the review's verdict changes and stops once it does.
- [ ] AC3: `discover-rule-files`, called with a feature, lists under `verdicts.judge` each rule the latest review record found violated, even while the analysis's stored verdict for it is current, and a fixture proves it.
- [ ] AC4: A fix a review makes is judged against the loaded rules in the same run, as its fix rounds are judged against the spec, and a fixture shows a round whose change fails a loaded rule counted as needing a fix.
- [ ] AC5: `write-analysis` refuses to record a hard-fail or blocking finding at a design subject of the spec in hand (`spec.md`, `plan.md`, `data-model.md` or a scenario) as deferred whole to that spec's own `tasks.md`, and refuses to record undispositioned a gating finding at a design subject whose other finding the same run recorded `fixed`. A fixture proves each refusal and each accepted record.
- [ ] AC6: `/{project}:analyze` judges whether `plan.md` contradicts the spec's Behavior or acceptance criteria, each contradiction a Blocking finding, in the step that judges the plan against `specs/system.md`.
- [ ] AC7: Every artifact that described the prior behavior is corrected in the same change: 078's Resolved Question on which fixes the loop covers, §Fix-round loop and §Load rules in `framework/reference/review.md`, the constitution's review-record row, and 066's data model, which carries each new review-record field.
- [ ] AC8: Each change reaches adopters through what a `ductus-v<version>` release ships, never only through this repository's `AGENTS.md` or `scripts/`.

## Applicable Rules

- `QUAL-CLAIM-001` — a rule left unjudged, or a disagreement not yet settled, never reads as agreement
- `QUAL-TEST-001` — each new check's fixture fails when its behavior is removed
- `BE-PAYLOAD-001` — the new record lists are bounded

## Open Questions

- Does a review record a verdict for every loaded rule against its scope, as an analysis does, or only for the rules the analysis record fired? The first review runs before any analysis exists, so only the first makes its dismissal of a rule checkable.
- Does a disagreement that stands after both audits judged the rule again hold `done`, and what settles it: a later verdict of either audit agreeing, a departure entry, or a decision the operator records?
- How is a review's fix judged against the rules: by re-running the rule passes over the fix round's changed paths, as a chore fix already re-runs the affected passes, or by the round's brief carrying the rules either record fired?
- Is a gating finding at a design subject of the spec in hand always fixed by a confirmed edit in the run, with only the code that edit owes becoming a task, or may the operator still defer the design edit to a task?
- Is a rule a review found violated, which the analysis judged did not fire, a disagreement too, or only a rule the next analysis judges again?
