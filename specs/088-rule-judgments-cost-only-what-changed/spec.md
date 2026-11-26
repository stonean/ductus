---
status: clarified
dependencies: [082-rule-assessments-converge-across-runs]
next-criterion: 3
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
- When a loaded rule file's digest differs from the one the record names, only the rules whose own text changed are judged again: the runtime reads that file at the record's `analyzed-against` commit and compares it with the current file rule by rule, so a rule whose heading, Statement and Verification are unchanged keeps its verdict. When history cannot supply the old file, as in a shallow clone, every rule in it is judged again and the result says why.
- No other judgment is skipped: every loaded rule is judged unless 082's reuse or the rule above carries its verdict, and a MUST rule still gates.

## Edge Cases

- A rule file added to or removed from the loaded set is the `rule-files-changed` case it is today: every loaded rule is judged.
- A rule whose ID now sits in a different file than at `analyzed-against` is judged again, as is a rule whose ID is new; a rule removed leaves its verdict behind.
- A record with no `analyzed-against`, or one naming a commit the repository does not hold, re-judges each changed file whole, and the result names the reason.
- The two counts AC1 records sum to the loaded rules a run assessed, a not-fired verdict carried counting as carried.

## Acceptance Criteria

- [ ] AC1: `write-analysis` records, for each run, the count of loaded rules judged afresh and the count of stored verdicts carried, and a test proves both counts on a fixture run that carries one verdict and judges one rule afresh.
- [ ] AC2: `discover-rule-files` with a `feature` puts in `judge` only the rules of a changed rule file whose own text differs from that file at the record's `analyzed-against`, and every rule of it when that commit's copy cannot be read, naming why; a fixture proves an edit to one rule re-judges that rule alone, and one in a shallow clone re-judges the whole file.

## Applicable Rules

- `QUAL-CLAIM-001` — a run's record says what it judged, so a run that judged nothing never reads like one that carried everything.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **A rule-file edit re-judges only the rules whose own text changed, derived from git** (the agent's recommendation, adopted by the operator, 2026-11-25). `rule-digest` held one sha256 per file, and `discover-rule-files`' description stated that "a file whose content alone changed makes only its own rules non-current", so seven of the 37 measured runs re-judged 90 or 175 of 306 rules for one edited file. Comparing the file at the record's `analyzed-against` with the current one adds nothing to a record and one `git show` per changed file per run. Rejected: a digest per rule in the record, about 11 to 25 KB more on each of the 80 records and a frontmatter schema change through the constitution; and keeping the per-file rule.
- **No rule is left unjudged on a deterministic ground** (the agent's recommendation, adopted by the operator, 2026-11-25). `/{project}:analyze` keeps judging every loaded rule, narrowed only by 082's reuse and the answer above. `/ductus` installs only the rule files `[rules] surfaces` selects (`framework/bootstrap/ductus.md` §Project Configuration: "Which rule surfaces /{project}:review enforces and /ductus installs"), so an adopter's analysis already loads only its surfaces' files, and a skip by surface would save this repository's 107 frontend rules of 306 per full run (40 accessibility, 45 security, 22 performance, by rule heading in `framework/rules/*-frontend.md`) and little elsewhere. Rejected: honouring `[rules] surfaces` in analysis, which amends §rules ("never from `/{project}:analyze`") and 033's AC7, reopening 033, and leaves a spec that describes frontend behavior unjudged against frontend rules; and a skip by the paths a change touched, which needs each of the 306 rules to declare the paths it governs.
- **A record without a `section-digest` gains one at its spec's next analysis** (the operator's decision, 2026-11-25). 50 of the 78 `analysis.md` records held none (a frontmatter scan, 2026-11-25), so each of those specs' next run judges every loaded rule once and writes the digest from that judgment. Rejected: a deterministic backfill on each record whose `analyzed-digest` still matched its subjects, which would have narrowed those next runs to the changed sections, through a runtime mode and a migration shipped to adopters.
