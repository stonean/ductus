---
status: draft
dependencies: [060-exec-analyze-assesses-each-loaded-rule, 081-reviews-and-checks-follow-what-changed]
next-criterion: 5
---

# 082 — Rule assessments converge across runs

`/{project}:analyze` records a verdict for each loaded rule it assesses, against the subjects it judged. A later run re-judges a rule only when its subjects changed. A record that judged fewer rules than were loaded says so on every path, not only under `ductus exec`.

## Motivation

An adopter reported that analyze's rule assessment did not converge. The spec was a backend feature spanning three repositories, assessed against eight rule files holding 141 rules. Three analyze runs over spec text that had stood unchanged for weeks found 10 new rule findings, then 8, then 4. Each would have fired on the first run. Each round cost a follow-up task, spec and plan edits, owner approvals, a commit, a review and another analysis. The third converged only because the host listed every rule's Verification by hand and judged them all in one pass. Nothing in the command or the record required that. The report arrived beside the one [081](../081-reviews-and-checks-follow-what-changed/spec.md) answers, and the owner moved this part here on 2026-11-11.

Read at `6158d7df`, the framework had these gaps:

- **Only findings were stored.** Steps 11 and 12 of `framework/commands/analyze.md` asked the host to judge every loaded MUST-tier and SHOULD-tier rule whose Verification trigger fired against the spec. `write-analysis` stored deferred and waived findings under `decisions:`, and `process-decisions` matched those alone. A rule judged not to fire, or to fire and be addressed already, left no trace, so every run re-judged every rule, and a different session could judge differently over the same text.
- **Exhaustiveness was unchecked on the interactive path.** `write-analysis` took `unexamined` from the caller. `rule-assessments-not-checked` counted only the rules steps 11 and 12 could not ask about: one with no Verification, or with no MUST or SHOULD keyword. A host that judged 30 of 141 rules wrote the same record as one that judged all 141. Under `ductus exec`, [060](../060-exec-analyze-assesses-each-loaded-rule/spec.md) had the walker send one request per loaded rule. Interactively nothing listed the rules for the host, although `discover-rule-files` with `rules` and `verification` could, since spec 080.

## Acceptance Criteria

- [ ] AC1: A fixture spec analyzed twice with no edit between the runs records identical rule verdicts. The second run asks the host about no rule whose stored verdict's subjects are unchanged.
- [ ] AC2: A run that judges fewer rules than were loaded records a non-zero `rule-assessments-not-checked` naming each rule left unjudged, on the interactive path as under `ductus exec`. A run that judges every loaded rule records zero, and a fixture proves both directions.
- [ ] AC3: A verdict that changes from not firing, or from addressed, to a finding while its subjects are unchanged is reported as a contradiction rather than recorded as a new finding.
- [ ] AC4: Each new analysis-record field is in the constitution's audit-record table (§text-first-artifacts) and in the analysis template, per §drift-prevention's template-rule alignment.

## Applicable Rules

- `QUAL-CLAIM-001`: a record that judged part of the loaded rules must not read like one that judged them all.

## Open Questions

- Which subjects does a verdict's digest cover: `spec.md` and `plan.md` whole, or only the sections the verdict cites?
- Does a verdict of `addressed` have to cite the section or anchor that addresses the rule, checked by `resolve-anchor`?
- Does a contradiction block `done`, or is it advisory and resolved by the host re-judging the rule?
- Where do the verdicts live? A verdict per rule in `analysis.md` frontmatter is 141 entries for the adopter's project, so its size against `[artifacts] read-size-bytes` matters.
- Does `write-analysis` refuse a record missing a verdict for a loaded rule, or record each missing one under `rule-assessments-not-checked`?
