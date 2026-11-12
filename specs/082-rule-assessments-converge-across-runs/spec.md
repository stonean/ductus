---
status: planned
dependencies: [060-exec-analyze-assesses-each-loaded-rule, 081-reviews-and-checks-follow-what-changed]
cross-spec-impact: [022-deterministic-runtime, 050-constitution, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate]
next-criterion: 7
---

# 082 — Rule assessments converge across runs

`/{project}:analyze` records a verdict for each loaded rule it assesses, against the subjects it judged. A later run re-judges a rule only when its subjects changed. A record that judged fewer rules than were loaded says so on every path, not only under `ductus exec`.

## Motivation

An adopter reported that analyze's rule assessment did not converge. The spec was a backend feature spanning three repositories, assessed against eight rule files holding 141 rules. Three analyze runs over spec text that had stood unchanged for weeks found 10 new rule findings, then 8, then 4. Each would have fired on the first run. Each round cost a follow-up task, spec and plan edits, owner approvals, a commit, a review and another analysis. The third converged only because the host listed every rule's Verification by hand and judged them all in one pass. Nothing in the command or the record required that. The report arrived beside the one [081](../081-reviews-and-checks-follow-what-changed/spec.md) answers, and the owner moved this part here on 2026-11-11.

Read at `6158d7df`, the framework had these gaps:

- **Only findings were stored.** Steps 11 and 12 of `framework/commands/analyze.md` asked the host to judge every loaded MUST-tier and SHOULD-tier rule whose Verification trigger fired against the spec. `write-analysis` stored deferred and waived findings under `decisions:`, and `process-decisions` matched those alone. A rule judged not to fire, or to fire and be addressed already, left no trace, so every run re-judged every rule, and a different session could judge differently over the same text.
- **Exhaustiveness was unchecked on the interactive path.** `write-analysis` took `unexamined` from the caller. `rule-assessments-not-checked` counted only the rules steps 11 and 12 could not ask about: one with no Verification, or with no MUST or SHOULD keyword. A host that judged 30 of 141 rules wrote the same record as one that judged all 141. Under `ductus exec`, [060](../060-exec-analyze-assesses-each-loaded-rule/spec.md) had the walker send one request per loaded rule. Interactively nothing listed the rules for the host, although `discover-rule-files` with `rules` and `verification` could, since spec 080.

> **Signpost:** a miss deferred here. Spec 081's reopening closed at `6d583778` over spec 079's AC1 bound, with one review and two analyses (`measure-reopenings --since 2026-11-11` at `932d10e0`). Its first analysis judged a keyword-filtered subset of the 310 loaded rules' Verification triggers and missed `BE-PAYLOAD-001`, and its record could not say so. The owner deferred the miss's cause to this spec on 2026-11-11, since AC2 would have recorded each rule left unjudged. 079 records the miss as its task 34.

## Acceptance Criteria

- [ ] AC1: A fixture spec analyzed twice with no edit between the runs records identical rule verdicts. The second run asks the host about no rule whose stored verdict's subjects are unchanged.
- [ ] AC2: A run that judges fewer rules than were loaded records a non-zero `rule-assessments-not-checked` naming each rule left unjudged, on the interactive path as under `ductus exec`. A run that judges every loaded rule records zero, and a fixture proves both directions.
- [ ] AC3: A verdict that changes from not firing, or from addressed, to a finding while its subjects are unchanged is recorded as a finding of its rule's tier and also reported as a contradiction naming the earlier verdict; the contradiction alone holds nothing.
- [ ] AC4: Each new analysis-record field is in the constitution's audit-record table (§text-first-artifacts) and in 066's data model, which the record `write-analysis` writes follows; no analysis template exists to carry it.
- [ ] AC5: A verdict stored before its rule's Statement or Verification changed is judged again, and a fixture that edits a rule between two runs over an unchanged spec proves it.
- [ ] AC6: An `addressed` verdict whose citation does not resolve is recorded among the rules left unjudged, never as `addressed`, and a fixture proves it.

## Edge Cases

- **A rule no longer loaded** (its file removed or disabled): its stored verdict is dropped, never carried as a judgment of a rule nobody loaded.
- **A rule newly loaded**: it has no stored verdict, so the run judges it or names it unjudged.
- **A design subject added or removed** (a first `plan.md`, a deleted scenario): the digest set changes, so every rule is judged again.
- **A citation into a subject later deleted or renamed**: it no longer resolves, so that rule is unjudged until judged again.
- **A verdict map or unjudged list that does not parse**: the record is undeterminable, as any unparseable audit record is (§text-first-artifacts), never read as holding no verdicts.
- **Two runs writing the record**: `write-analysis` writes it whole and atomically, so the later run's verdicts stand and are what the next run reads.

## Applicable Rules

- `QUAL-CLAIM-001`: a record that judged part of the loaded rules must not read like one that judged them all.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Which subjects does a verdict's digest cover?** The spec's design subjects whole, `spec.md`, `plan.md`, `data-model.md` and each scenario, never `tasks.md`, `review.md` or `analysis.md`, by the record's existing per-path `analyzed-digest` entries for them, together with a hash of the rule's own text (its Statement and Verification), kept per rule file, so a rule an upgrade edits is judged again with the rest of its file: a not-fired rule has no entry to hold a hash of its own, and a hash for every loaded rule would cost about 8.8 KB a record at 310 rules, where the twelve files cost about 1 KB (the plan's Trade-offs; the owner, 2026-11-12). A stored verdict is current while all of them match; any design-subject edit re-judges every rule, which is what every run does before this spec. Rejected: the sections a verdict cites, since a `not fired` verdict, the bulk of them, cites none and would fall back to whole files anyway, at a second digest mechanism; and every `.md` under the feature, since a `tasks.md` tick or a `review.md` rewrite would then re-judge every rule (the agent's recommendation, adopted by the owner, 2026-11-12).
- **Does an `addressed` verdict cite what addresses the rule?** Yes, and `write-analysis` checks it: each `addressed` verdict names the section or anchor of a design subject that addresses the rule, and one whose citation does not resolve, as `resolve-anchor` resolves it, is not stored as `addressed` but recorded among the rules left unjudged, so the closed unexamined-reason set gains nothing. An uncited `addressed` is a judgment a later run cannot test and a different session can flip. Rejected: an optional citation, which leaves that flip; and a required but unchecked one, a check that cannot run looking like one that passed (the agent's recommendation, adopted by the owner, 2026-11-12). It depends on the record naming a missing verdict rather than being refused (the last question).
- **Does a contradiction block `done`?** No, it is advisory, and it never replaces the finding: a verdict that turns from not firing, or from `addressed`, to a finding while its subjects are unchanged joins its rule's tier as any finding does (a MUST Blocking, a SHOULD Advisory), and the report and the record's body add a contradiction note naming the earlier verdict. With current verdicts reused, a contradiction arises only when a rule is judged anyway, under `ductus exec` (060) or on a forced re-judge. AC3 says so. Rejected: a contradiction as a Blocking finding of its own, a gate over the host's non-determinism costing an analysis run per occurrence; and a note replacing the finding, which would drop a MUST failure from `blocking-findings` (the agent's recommendation, adopted by the owner, 2026-11-12).
- **Where do the verdicts live?** In `analysis.md`'s frontmatter, as a map of the rules that fired, each with its verdict (`addressed` with its citation, or `finding` with its message and location, so a current finding is reported again without re-judging, about 100 bytes more an entry; the owner, 2026-11-12), beside a list naming each loaded rule left unjudged; a loaded rule in neither did not fire. The record so grows with what fired, not with the rule library: about 26 to 61 bytes per fired rule (an ID averages 11.75 characters over this repository's 310), and 14 per unjudged one. Measured at `94e5b151`: the largest record is 19,901 bytes and the 73 records total 226,920; `check-artifact-size` does not measure `analysis.md`. Not measured: how many rules fire per spec, since no record yet stores an `addressed` verdict; one `ductus exec analyze` pass over a few specs would say, and if nearly every rule fires the map approaches a full one. Rejected: a map of every loaded rule, about 590 KB more across this repository's 73 records at 26 bytes for 310 rules, for what absence already says; the record's body, which would put state the next run reads where the body stays prose (§text-first-artifacts), unlike `decisions:`; and a sibling file, a third record kind (the agent's recommendation, adopted by the owner, 2026-11-12).
- **Does `write-analysis` refuse a record missing a verdict for a loaded rule?** No: it records each loaded rule left unjudged, as AC2 says, since a refused write leaves no record at all and a partial run must read as partial (`QUAL-CLAIM-001`). The count stays `rule-assessments-not-checked` in `unexamined-by-reason`, a reason-to-count map (066), and the names go on the unjudged-rules list beside the verdict map. Under `ductus exec`, where the walker asks about every loaded rule (060), the count keeps its meaning, the rules it could not ask about; on the interactive path it also counts the rules the host did not judge (the agent's recommendation, adopted by the owner, 2026-11-12).
