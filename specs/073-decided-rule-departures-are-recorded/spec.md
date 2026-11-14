---
status: done
dependencies: [016-cross-cutting-rules, 020-code-review, 058-findings-route-at-discovery, 066-audit-records-and-the-completion-gate, 082-rule-assessments-converge-across-runs]
cross-spec-impact: [020-code-review, 022-deterministic-runtime, 050-constitution, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 082-rule-assessments-converge-across-runs]
next-criterion: 9
---

# 073 — Decided Rule Departures Are Recorded

A spec states once that it deliberately departs from a rule, and both `/{project}:analyze` and `/{project}:review` read that decision, so an accepted departure is recorded — what was accepted, by whom, when and why — and is never indistinguishable from a rule the spec satisfies.

> **Signpost from [076 — Waived and deferred findings are distinct dispositions](../076-waived-and-deferred-findings-are-distinct-dispositions/spec.md).**
> *Waived* now names every decision not to address a finding, at any tier,
> replacing *discarded*, and *deferred* replaces *routed*. Whether a gating
> analyze finding can be waived stays this spec's question; 076 lets one be
> deferred, releasing `done` only for work on another spec.

## Motivation

`/{project}:review` recorded a per-file waiver with `--waive <rule-id> --reason`; `/{project}:analyze` had no counterpart (verified 2026-10-26 at HEAD c0952038). At `/{project}:analyze` each MUST-tier rule whose trigger fired got a semantic assessment, and a failing one joined the Blocking tier; a blocking finding could not be discarded — it was fixed or routed, and `write-analysis` refused the discard; `decisions:` held only `routed` and `discarded`; and the record kept tier counts and findings, never which rules were assessed or passed. Whether a spec's stated departure passed therefore rested on the assessing agent's judgment, which left no record of what was accepted, by whom or why, and could flip on the next run.

An adopter's spec listed two MUST rules — a compatibility rule, for a credential field removed from a response rather than versioned, and an idempotency rule — as deliberately not satisfied, and its analysis recorded `blocking-findings: 0` with no trace of either. The adopter's shared constitution also decided three MUST departures in advance and told authors to record each in Applicable Rules and waive it at review: the review half had a mechanism, the analyze half did not. That broke the constitution's "a check that cannot run MUST never be indistinguishable from a check that passed".

## Departure entries

A departure is stated once, in the spec, as a structured entry under `## Applicable Rules`: the rule ID, of any tier, the reason, and optionally a citation to the governing document that decided it; who decided it and when are read from the commit that introduced it (Resolved Questions). Both commands read the entry, so the decision is never restated per command or per run, and nothing depends on an author remembering to waive it twice. A departure every spec carries is settled at the rule or as a project-wide exception; this entry is for a spec-specific one.

## The analyze waiver

`/{project}:analyze` records each departure whose rule's trigger fires as a `waived` verdict in the rule-verdict map of [082 — Rule assessments converge across runs](../082-rule-assessments-converge-across-runs/spec.md), with the entry's reason, its derived `decided-by` and `decided-at`, and its citation. The rule is assessed as waived, never as failing, so it produces no finding. A waived rule is listed apart from the rules that passed, in the record and in the report, waived MUSTs first. The map is rewritten each run, so a waiver carries forward while its rule fires and is gone once it does not. It builds on the dispositions of [058 — Findings route at discovery](../058-findings-route-at-discovery/spec.md), the record and gate of [066 — Audit Records and the Completion Gate](../066-audit-records-and-the-completion-gate/spec.md), the review waivers of [020 — Code review](../020-code-review/spec.md), and the Applicable Rules template of [016 — Cross-cutting rules](../016-cross-cutting-rules/spec.md). The constitution's Finding dispositions rule changes in the same change (`cross-spec-impact:` on 050). 082's map carries no `waived` value until this spec adds it with the entries that write it, so 082's data model and 066's record schema gain it here (`cross-spec-impact:` on both; the owner, 2026-11-12).

## Edge Cases

- **An entry whose rule does not fire**: no verdict is recorded for it; the entry stays the spec's record of the decision.
- **An entry naming a rule since removed or deprecated**: a finding (AC1), so the entry is re-pointed or removed rather than silently waiving nothing.
- **Two entries naming one rule**: a finding (AC1); a rule has one departure.
- **An entry edited after it was committed**: `decided-by` and `decided-at` stay those of the commit that introduced it; the edit is the spec's history.
- **An entry removed**: the next run assesses the rule as any other, so a MUST failure blocks again.
- **A waived rule the assessment would pass**: still recorded `waived`, since the entry, not the assessment, decides it.

## Acceptance Criteria

- [x] AC1: A spec can state a departure as a structured entry under `## Applicable Rules` naming a rule ID of any tier and the reason, and optionally the governing document that decided it, and `/{project}:analyze` reports as a finding an entry missing a required field, naming an unknown or deprecated rule ID, naming a rule another entry already names, or citing a governing document `resolve-anchor` cannot resolve.
- [x] AC2: `/{project}:analyze` records each departure whose rule's trigger fires as a `waived` verdict in 082's rule-verdict map, with the entry's reason, its derived `decided-by` and `decided-at`, and its citation; the rule is assessed as waived and produces no finding, so it never counts toward `blocking-findings`.
- [x] AC3: `analysis.md` and the analyze report list waived rules apart from passing ones, so a run in which a MUST rule was waived is distinguishable from one in which it was satisfied.
- [x] AC4: A waiver carries forward when `analysis.md` is regenerated while its rule still fires, and is pruned from the record when the rule no longer fires.
- [x] AC5: `/{project}:review` honors the same departure entries at every file in the spec's review scope, writing each file's anchor into `review.md`'s `waivers:`, so a departure decided once in the spec needs no second `--waive`.
- [x] AC6: The constitution's Finding dispositions rule, and the analysis-record row 082 adds, describe the analyze waiver, in the same change as the behavior.
- [x] AC7: An entry's `decided-by` and `decided-at` are the author and date of the commit that introduced it; an entry history cannot attribute (not yet committed, or a shallow checkout) is recorded `decided-by: unattributed`, and a fixture proves both, its test run in a job with a full-depth checkout.
- [x] AC8: A departure entry naming a SHOULD rule waives it at review as one naming a MUST does, and a fixture proves it.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Is a decided departure a disposition of a blocking finding, or settled before any finding exists?** Settled before: a rule a departure entry names is assessed as waived, never as failing, so it produces no finding, and 058's AC30 and `write-analysis`'s refusal of a waiver on a blocking finding stand unchanged. The bypass 058's edge case names is gated by the review of the spec edit that adds the entry, and made visible by the record and the report listing each waived MUST before anything else (AC3). Rejected: a disposition of a blocking finding confirmed through `gate-confirm`, which changes 058's AC30, the refusal and the reference, four edits for a confirmation the listing and a derived `decided-by` stand in for (the agent's recommendation, adopted by the owner, 2026-11-12).
- **At review, does a departure entry waive its rule for every file in scope, or per file?** Every file in the spec's review scope, at each run: the run writes the per-file anchors into `review.md`'s `waivers:`, so each still expires on its own and the record names the files, and a file added later where the rule fires is covered with no second decision (AC5). 020's scenario `waiver-expiry` keeps its rule that a waiver never extends to another location, with one exception sentence: a departure is bounded by the spec's own scope. Rejected: an entry listing its files, a list the author must keep current as code moves, losing coverage silently on a rename; and an entry covering only where its rule fired when it was written, which needs history for nothing more (the agent's recommendation, adopted by the owner, 2026-11-12).
- **Does a decided departure from a MUST revisit 008's Resolved Question 1?** No: it is not a severity tier. The rule stays MUST and no tier is added; a departure entry is the per-spec record of a deviation that 008's question asked a SHOULD's rationale to request, here extended to a MUST, as 020 already records single-author MUST waivers at review. 008 is unchanged, since its reason for refusing an acknowledge tier, no extra severity level, still holds (the agent's recommendation, adopted by the owner, 2026-11-12).
- **How is `decided-by` derived, and may a governing document stand in for a per-spec decision?** From history: `/{project}:analyze` reads the author and date of the commit that introduced the entry, and when history cannot say (an entry not yet committed, a shallow checkout) the record carries `decided-by: unattributed`, a state of its own, never a silent pass; the test that reads history runs in a job with a full-depth checkout (§design-principles). The entry's optional governing-document citation is checked with `resolve-anchor`, so a cited advance decision is verifiable. A governing document never replaces `decided-by`: its advance decision is shown by the citation, while the entry's author is who accepted it for this spec. Rejected: typed `decided-by` and `decided-at` alone, an attribution nothing checks (the agent's recommendation, adopted by the owner, 2026-11-12).
- **Does `analysis.md` keep a count of waived rules?** No count of its own: a waived rule is a `waived` value in the rule-verdict map spec 082 adds to the record (`082-rule-assessments-converge-across-runs`), beside `addressed` and `finding`, and its details stay in the spec's departure entry, so the count is derived from the map. The map is rewritten each run from the spec and the rules that fired, so a waiver carries forward while its rule fires and is gone once it does not, which AC4 asks. 073 so builds on 082 and lands after it. Rejected: a `waivers:` list of the record's own, a second structure holding what the map does; and a list with an integer beside it, a count that can disagree with its list (the agent's recommendation, adopted by the owner, 2026-11-12).
- **Does a departure entry also apply to a SHOULD rule?** Yes: an entry may name a rule of any tier, the tier coming from the rule. At `/{project}:analyze` a SHOULD finding is advisory and already waived by the disposition rule with its reason, so the entry matters at `/{project}:review`, where an unwaived SHOULD holds `done` (§implement-phase) and the entry removes the second `--waive` there too (AC5). Rejected: MUST only, which needs a tier lookup and leaves a SHOULD departure needing `--waive` at review (the agent's recommendation, adopted by the owner, 2026-11-12).
- **Does a departure entry decide over a deferral of its rule?** Yes, at `/{project}:review` as at `/{project}:analyze`, where `write-analysis` refuses a verdict on a rule with an entry: a new `--defer` of a violation of the entry's rule in the scope is refused, naming the rule and the entry's `spec.md` line, so the entry is removed first, and a stored deferral of one is superseded, pruned from `decisions:` and named in `write-review`'s `superseded-deferrals` (`plan.md`, how `write-review` derives the waivers). Rejected: the precedence reversed, a deferral deciding over the entry, which review would then apply where analysis already refuses it (the call 073's first review fix round made, `d15a0188`, kept by the owner, 2026-11-13).
- **Why split from 066-audit-records-and-the-completion-gate:** 066 owns the audit records and the completion gate. 073 owns the decided-departure feature, which writes into 066's surfaces (the analysis record's `rule-verdicts`, review's `waivers:`) without changing what they are. By the owner's rule that a change to a shared runtime surface lands in the spec that requires it while the surface's owner links back (AGENTS.md, owner 2026-11-04), the feature stays here and 066's data model and signpost point at it.
