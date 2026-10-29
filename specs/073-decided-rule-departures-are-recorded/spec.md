---
status: draft
dependencies: [016-cross-cutting-rules, 020-code-review, 058-findings-route-at-discovery, 066-audit-records-and-the-completion-gate]
cross-spec-impact: [050-constitution]
next-criterion: 7
---

# 073 — Decided Rule Departures Are Recorded

A spec states once that it deliberately departs from a rule, and both `/{project}:analyze` and `/{project}:review` read that decision, so an accepted departure is recorded — what was accepted, by whom, when and why — and is never indistinguishable from a rule the spec satisfies.

## Motivation

`/{project}:review` recorded a per-file waiver with `--waive <rule-id> --reason`; `/{project}:analyze` had no counterpart (verified 2026-10-26 at HEAD c0952038). At `/{project}:analyze` each MUST-tier rule whose trigger fired got a semantic assessment, and a failing one joined the Blocking tier; a blocking finding could not be discarded — it was fixed or routed, and `write-analysis` refused the discard; `decisions:` held only `routed` and `discarded`; and the record kept tier counts and findings, never which rules were assessed or passed. Whether a spec's stated departure passed therefore rested on the assessing agent's judgment, which left no record of what was accepted, by whom or why, and could flip on the next run.

An adopter's spec listed two MUST rules — a compatibility rule, for a credential field removed from a response rather than versioned, and an idempotency rule — as deliberately not satisfied, and its analysis recorded `blocking-findings: 0` with no trace of either. The adopter's shared constitution also decided three MUST departures in advance and told authors to record each in Applicable Rules and waive it at review: the review half had a mechanism, the analyze half did not. That broke the constitution's "a check that cannot run MUST never be indistinguishable from a check that passed".

## Departure entries

A departure is stated once, in the spec, as a structured entry under `## Applicable Rules`: the rule ID, the reason, who decided it and when, and optionally a citation to the governing document that decided it. Both commands read the entry, so the decision is never restated per command or per run, and nothing depends on an author remembering to waive it twice. A departure every spec carries is settled at the rule or as a project-wide exception; this entry is for a spec-specific one.

## The analyze waiver

`/{project}:analyze` records each departure whose rule's trigger fires as a waiver in `analysis.md`, keyed on rule and spec and carrying the entry's fields. A waived rule is listed apart from the rules that passed, in the record and in the report. A waiver carries forward when the record is regenerated while its rule still fires, and is pruned once it no longer does, as review's waivers are. It builds on the dispositions of [058 — Findings route at discovery](../058-findings-route-at-discovery/spec.md), the record and gate of [066 — Audit Records and the Completion Gate](../066-audit-records-and-the-completion-gate/spec.md), the review waivers of [020 — Code review](../020-code-review/spec.md), and the Applicable Rules template of [016 — Cross-cutting rules](../016-cross-cutting-rules/spec.md). The constitution's Audit records schema and Finding dispositions rule change in the same change (`cross-spec-impact:` on 050).

## Acceptance Criteria

- [ ] AC1: A spec can state a departure as a structured entry under `## Applicable Rules` naming the rule ID, the reason, who decided it and when, and optionally the governing document that decided it, and `/{project}:analyze` reports an entry missing a required field, or naming an unknown or deprecated rule ID, as a finding.
- [ ] AC2: `/{project}:analyze` records each departure whose rule's trigger fires as a waiver in `analysis.md`, keyed on rule and spec and carrying the entry's fields, and a failing assessment of a waived rule does not count toward `blocking-findings`.
- [ ] AC3: `analysis.md` and the analyze report list waived rules apart from passing ones, so a run in which a MUST rule was waived is distinguishable from one in which it was satisfied.
- [ ] AC4: A waiver carries forward when `analysis.md` is regenerated while its rule still fires, and is pruned from the record when the rule no longer fires.
- [ ] AC5: `/{project}:review` honors the same departure entries, so a departure decided once in the spec needs no second `--waive`.
- [ ] AC6: The constitution's Audit records schema and Finding dispositions rule describe the analyze waiver, in the same change as the behavior.

## Open Questions

- At review, does a departure entry waive its rule for every file in scope, or per file — given review anchors a waiver per rule and file and records a rule against extending that anchor?
- Is a decided waiver a disposition of a blocking finding — overturning 058's edge case that discarding one "would add a bypass the gate does not have" — or a state settled before any finding is produced, so the rule is assessed as waived and never as failing?
- Does a decided departure from a MUST revisit 008 Resolved Q1, which declined an acknowledge tier in favour of a SHOULD whose rationale asks for a recorded deviation, or is it distinct from a severity tier?
- How is `decided-by` derived rather than typed, and may a governing document's advance decision (a shared constitution's pre-decided departures) stand in for a per-spec operator decision?
- Does `analysis.md` keep a count of waived rules, beyond review's model of a `waivers:` list and a Waived findings body?
- Does a departure entry also apply to a SHOULD rule, since an unwaived SHOULD also holds a spec short of `done`?
