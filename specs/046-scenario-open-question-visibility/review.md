---
spec: 046-scenario-open-question-visibility
reviewed-at: 2026-08-16T17:09:11Z
reviewed-against: ec40f796433bf8e6fa25ce33c542166e6703a368
diff-base: 2e24054345cd91bf0932dad3418e61d4cbf615b4
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 046-scenario-open-question-visibility

## Summary

0 MUST violation(s), 0 SHOULD violation(s), 0 low-confidence finding(s). blocking: no.

One finding was raised and **fixed before this report was finalised**, so the counts state what is outstanding rather than what was found; it is recorded below with a Status line naming the commit.

Re-run after this spec took the `done → in-progress` back-edge to record a decision 022's `scenario-open-question-signal` revised: feature-targeted `/{project}:clarify` now *reports* scenario open questions, where this spec had asserted it "does not surface" them and is "unchanged".

**A note on why the gate did not force this re-review.** `check-review-gate` reported 046 current, and that is correct rather than a miss: staleness is deliberately scoped to a spec's **durable contracts** — `scenarios/*.md` and `data-model.md` — and 046 has neither. The scoping was chosen under measurement, not assumption: the first cut used the plan's Affected Files and blocked 34 of 48 specs, because old specs list shared surfaces every later spec also touches. The consequence worth stating plainly is that a spec with no durable contracts is structurally exempt from the staleness check, so a passing gate is not evidence its verdict is current. This spec's verdict dated from 2026-07-31 while five of the runtime primitives in its Affected Files moved underneath it. That is the same gap already logged as 022 task 88 (staleness on `done` specs), viewed from a second angle.

Scope resolved to the plan's 14 Affected Files — five runtime primitives, four command sources, the constitution and 022's artifacts — with `modified-since` empty because the diff base is the commit this spec reopened at. That is a genuinely reviewable surface rather than the whole-repo scope the long-lived specs produce, and it is the spec's real subject.

The scenario-question implementation holds up otherwise: one shared collector feeds `read-spec`, `check-review-gate`, `check-artifacts` and `dashboard`, so the count the user sees, the count the gate blocks on, and the count analyze reports cannot disagree — which is the property this spec chose it for. Ordering goes through the one shared scenario listing (case-insensitive, raw-byte tiebreak), so two surfaces never present two orders. The `done`-only blocking tier with no grandfather rule is deliberate and argued in the spec. QUAL-STUB-001 and QUAL-GROUND-001 are clean across the scope.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None outstanding.* The finding below was fixed in-window.

### SHOULD: QUAL-CLAIM-001 — an unreadable scenario read as a clean one

- **File**: `runtime/src/primitives/read_spec.rs:80-82`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish *"examined the subject and found nothing"* from *"could not examine the subject"*, rather than emitting the same value for both.
- **Finding**: This spec and 022's signal scenario both record one decision about failure — an unreadable scenario contributes nothing and never blocks, because nothing can be proven about a file that will not parse. That decision is right and is retained. It is also only half the obligation: `collect_scenario_open_questions` implemented *not blocking* as *not reporting*, taking a bare `continue` on the unreadable branch and returning a plain `Vec`. A feature whose only scenario could not be read therefore produced an empty question list byte-identical to one whose scenarios were all read and carried nothing, and every consumer asserted the reassuring reading: `check-review-gate` returned no block, `check-artifacts`' family reported clean with no skipped record, the dashboard rendered no callout, and the feature-targeted clarify report added in the same session was suppressed entirely. The asymmetry is the dangerous one — a scenario file that will not parse is disproportionately a scenario something is wrong with — and the rule names exactly the missing forms: a distinct variant, a skipped list, or a count of what *was* examined.
- **Auto-fixable**: no
- **Status**: fixed in `ec40f79`. The collector returns a scan carrying the questions **and** the slugs it could not read; `read-spec` surfaces `scenario-files-unreadable`, and `check-artifacts` records each as a `skipped` target with the existing `artifact-unreadable` reason. Nothing gained a block — the gate still fails open, since a gate that failed closed on its own inability to read is one people route around. The field is omitted when empty, so payloads and the parity goldens are byte-unchanged and *empty now means* every scenario was read. Two tests cover both directions (an unread scenario reported and not counted clean; a fully-examined feature reporting nothing unread), and the pre-existing single-skip assertion was scoped by family, since one unread file is legitimately recorded by two families. Routed per the runtime-work rule as 022's `unreadable-scenario-is-reported` (task 89), back-linked here, with 022's data-model updated; this spec gains AC32.

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Skipped passes

*None.*
