---
status: draft
dependencies: [082-rule-assessments-converge-across-runs]
next-criterion: 2
---

# 088 — Rule judgments cost only what changed

A `/{project}:analyze` run judges a loaded rule afresh only when what that rule reads has changed, and its record says how many rules it judged afresh and how many stored verdicts it carried, so what rule assessment costs is read from the record rather than inferred.

## Motivation

- [082](../082-rule-assessments-converge-across-runs/spec.md) made a stored verdict stand while the sections it concerned were unchanged (its AC1 and AC7), so a run judged a rule again only against changed sections. Whether that held in practice was not known: the `analyze(…)` commit subjects reported "306 rules judged" without saying how many were judged afresh.
- **Measured** at `4cfcf172` (2026-11-25) over the 37 `analysis.md` records changed by commits after `1dc4736e`, 082's second `done`, each compared with the same path at its commit's parent: 0 advisory contradictions (082's AC3) were recorded, and 0 in the 161 records since its first `done`, `a3629a95`. `rule-verdicts` holds only rules that fired, and `write-analysis`'s `judged-rules` argument is not stored, so no record said how many rules its run judged afresh.
- **Derived**, by applying 082's reuse rule (its data model's Relations and `discover-rule-files`' reasons) to each prior and new record, not by reading what any run judged: 26 of the 37 runs owed a full judgment of every loaded rule, 19 because the prior record predated AC7's section digest, 5 because it carried no `rule-digest` and 2 because none existed; 7 owed a full judgment of each rule in an edited rule file (90 or 175 of 306); 4 owed judgments against changed sections only; none could carry a rule outright. Of 11,364 rule judgments owed, 2,402 (21.1%) were narrowed to changed sections. 22 commits touched `framework/rules/` after `a3629a95`.
- 082's Motivation recorded an adopter on `ductus-v0.59.11` getting 10, then 8, then 4 new rule findings over three runs on unchanged text.

## Behavior

- An analysis record states how many loaded rules its run judged afresh and how many stored verdicts it carried, so a run's rule-assessment cost is measured from the record.
- Which further judgments a run may skip is undecided; the Open Questions hold the candidates. A MUST rule still gates wherever it is judged.

## Acceptance Criteria

- [ ] AC1: `write-analysis` records, for each run, the count of loaded rules judged afresh and the count of stored verdicts carried, and a test proves both counts on a fixture run that carries one verdict and judges one rule afresh.

## Applicable Rules

- `QUAL-CLAIM-001` — a run's record says what it judged, so a run that judged nothing never reads like one that carried everything.

## Open Questions

- When a rule file changes, is every rule in it judged again, as now, or only each rule whose own statement or Verification changed, by a digest per rule rather than per file? Seven of the 37 measured runs re-judged 90 or 175 rules for an edited file.
- May a rule be left unjudged for a change on a deterministic ground, such as its surface against the spec's stack or the paths the change touched, and who decides the ground? §rules has `/{project}:analyze` load every discovered rule file regardless of stack.
- Should a record whose `analyzed-digest` still matches its subjects gain a `section-digest` by a deterministic backfill, so the 19 runs' cause, a prior record predating AC7, stops forcing a full judgment, or does each record drain at its spec's next run?
