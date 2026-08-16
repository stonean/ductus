---
section: "Follow-on scenarios"
---

# Unreadable-scenario-is-reported

## Context

[046](../../046-scenario-open-question-visibility/spec.md) requires that a scenario's unresolved questions gate its parent spec's `done`, and its runtime half landed as [scenario-open-question-signal](scenario-open-question-signal.md). Both specs recorded one deliberate decision about failure: *an unreadable or malformed scenario file contributes nothing and never blocks the gate — nothing can be proven about a file that will not parse, and an unknown is not escalated into a defect.*

That decision is right, and it is only half the obligation. `collect_scenario_open_questions` skipped an unreadable file and returned a bare `Vec`, so **not blocking** was implemented as **not reporting**. A feature whose only scenario could not be read produced an empty question list — byte-identical to a feature whose scenarios were all read and carried nothing. Every downstream surface then asserted the reassuring reading: `check-review-gate` passed, `check-artifacts`' family reported clean, the dashboard rendered no callout, and feature-targeted `/{project}:clarify`'s report was suppressed entirely.

This is `QUAL-CLAIM-001` in the machinery that ships it — *a fully-implemented path whose output overstates what it verified* — and the asymmetry is the dangerous one: a scenario file that will not parse is disproportionately a scenario something is wrong with. Surfaced 2026-08-16 by `/{project}:review` on 046.

## Behavior

**The collector reports what it could not examine.** `collect_scenario_open_questions` returns a scan carrying both the questions it found and the slugs of the scenario files it could not read, rather than a bare list. It stays the single shared reader — a second, private one could disagree with the count the user was shown, which is the property 046 chose it for.

**`read-spec` surfaces it as `scenario-files-unreadable`**, a sibling to `scenario-open-questions`, omitted from the payload when empty so the ordinary case is byte-unchanged and no golden re-blesses. Empty therefore *means* something: every scenario was read.

**`check-artifacts` records each as a skipped target**, family `scenario-open-questions`, reason `artifact-unreadable` — an existing member of the closed reason set, not a new one. It is not a finding: the file is an unknown, not a defect. One unreadable file may be recorded by more than one family, which is what `family` on the skipped record distinguishes.

**Nothing gains a block.** `check-review-gate` still returns no block when the question list is empty, whatever the unread set holds. A gate that failed closed on its own inability to read is a gate people route around, and the fail-open posture is the same one the staleness check already takes.

**The dashboard deliberately discards it.** It renders a glance, and the distinction belongs where a caller goes for it — `read-spec` and the analyze skipped list. The discard is named in the code so it reads as a decision rather than an oversight.

## Edge Cases

- Every scenario readable: `scenario-files-unreadable` is absent from the payload and the skipped list gains nothing — the result is byte-identical to before this scenario.
- No `scenarios/` directory: nothing to read, nothing unread, no records.
- A readable scenario with no `## Open Questions` section: examined and clean — it is *not* reported as unreadable, which is the distinction this scenario exists to preserve.
- A scenario that is unreadable *and* carries questions is a contradiction in terms — it yields no questions, so it appears only in the unread set.
- The same unreadable file recorded by both `link-adjacent-drift` and `scenario-open-questions`: two records, distinguished by `family`, not a duplicate to dedupe.
- Invalid UTF-8 is the reachable form of unreadable in tests; a permissions failure or a dangling symlink takes the same branch.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
