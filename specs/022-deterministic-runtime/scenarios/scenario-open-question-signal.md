---
section: "Follow-on scenarios"
---

# Scenario-open-question-signal

## Context

Every runtime surface answering *"does this spec have unresolved questions?"* read the spec body only. `read-spec` derived its count from the body, `dashboard` loaded scenario detail solely for a session-targeted scenario, and `check-artifacts` had no family for scenario questions at all. A spec whose scenario carried blocking design questions therefore reported zero, and the pipeline routed it to implementation.

[046 — Scenario open-question visibility](../../046-scenario-open-question-visibility/spec.md) owns the requirement and the rule; this scenario carries the runtime work, per that spec's Implementation ownership split. Depends on [scenario-question-parser-fix](scenario-question-parser-fix.md) — every surface below reads a count, and the count must be correct before anything consumes it.

## Behavior

**A sibling field on `read-spec`.** `scenario-open-questions` lists the unresolved questions carried by `scenarios/*.md`, each tagged with its source scenario slug, in shared scenario order. `open-questions` keeps its current meaning, value, and routing effect. The two are never merged: merging would route a feature-level target to feature-targeted clarify, which does not read scenarios and would arrive with nothing to act on.

**One ordering rule.** Scenario enumeration and ordering go through the shared scenario-file listing, which sorts case-insensitively with a raw-byte tiebreak. The tiebreak is load-bearing — directory reads yield filesystem order, so names differing only in case would otherwise sort differently across machines. Every surface presenting scenarios uses that one comparator, so a reader never sees two orders for the same directory.

**The completion gate.** The pre-done review gate evaluates scenario open questions as a third check, ordered after markdown lint and before the `review:` block, first failure still winning. Its blocked message names the scenarios carrying questions. An unresolved design question is more upstream than a missing review, so surfacing it first avoids sending a contributor to run review against a design about to change.

**The analyze family.** `check-artifacts` gains a `scenario-open-questions` family — blocking on a `done` spec, advisory otherwise — and `--fix` reverts `done → in-progress` with a non-silent notice, mirroring the review-state-drift revert. No grandfather exemption: an absent `review:` block genuinely marks a spec as predating its feature, but an unresolved scenario question is a present-tense defect whenever it arrived.

**The readouts.** The dashboard's existing Scenarios column gains an outstanding-count suffix, the Next Action cell overrides to scenario-targeted clarify, and a callout below the table names the specs and their question-carrying scenarios with no cap. Recovery state wins the Next Action cell when both apply; both callouts render.

## Edge Cases

- No `scenarios/` directory, or scenarios with no Open Questions section: empty list, no gate block, no finding, no readout change.
- An unreadable or malformed scenario file contributes nothing and never blocks the gate — nothing can be proven about a file that will not parse, and an unknown is not escalated into a defect.
- Non-`.md` files and case-varying filenames are decided by the shared listing, not re-derived here.
- A question resolved between the gate check and the status write is accepted: the gate re-reads at check time and the resolution can only turn a block into a pass.
- Adding a scenario that carries questions still takes the scenario back-edge to `in-progress`, never the question back-edge to `draft` — the spec body's own Open Questions section is not written to.

## Open Questions

*None — resolved in 046's clarify before this scenario was written.*

## Resolved Questions

*None yet.*
