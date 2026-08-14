---
section: "Follow-on scenarios"
---

# Scenario-open-question-signal

## Context

Every runtime surface answering *"does this spec have unresolved questions?"* read the spec body only. `read-spec` derived its count from the body, `dashboard` loaded scenario detail solely for a session-targeted scenario, and `check-artifacts` had no family for scenario questions at all. A spec whose scenario carried blocking design questions therefore reported zero, and the pipeline routed it to implementation.

[046 — Scenario open-question visibility](../../046-scenario-open-question-visibility/spec.md) owns the requirement and the rule; this scenario carries the runtime work, per that spec's Implementation ownership split. Depends on [scenario-question-parser-fix](scenario-question-parser-fix.md) — every surface below reads a count, and the count must be correct before anything consumes it.

## Behavior

**A sibling field on `read-spec`.** `scenario-open-questions` lists the unresolved questions carried by `scenarios/*.md`, each tagged with its source scenario slug, in shared scenario order. `open-questions` keeps its current meaning, value, and routing effect. The two are never merged, because they answer different questions: `open-questions` is the spec body's own count, whose emptiness is what `clarified` asserts and what the `draft → clarified` edge turns on, while `scenario-open-questions` is remaining work that gates `done`. Merging them would make a spec's status contradict its own body.

**One ordering rule.** Scenario enumeration and ordering go through the shared scenario-file listing, which sorts case-insensitively with a raw-byte tiebreak. The tiebreak is load-bearing — directory reads yield filesystem order, so names differing only in case would otherwise sort differently across machines. Every surface presenting scenarios uses that one comparator, so a reader never sees two orders for the same directory.

**The completion gate.** The pre-done review gate evaluates scenario open questions as a third check, ordered after markdown lint and before the `review:` block, first failure still winning. Its blocked message names the scenarios carrying questions. An unresolved design question is more upstream than a missing review, so surfacing it first avoids sending a contributor to run review against a design about to change.

**The analyze family.** `check-artifacts` gains a `scenario-open-questions` family — blocking on a `done` spec, advisory otherwise — and `--fix` reverts `done → in-progress` with a non-silent notice, mirroring the review-state-drift revert. No grandfather exemption: an absent `review:` block genuinely marks a spec as predating its feature, but an unresolved scenario question is a present-tense defect whenever it arrived.

**The readouts.** The dashboard's existing Scenarios column gains an outstanding-count suffix, the Next Action cell overrides to scenario-targeted clarify, and a callout below the table names the specs and their question-carrying scenarios with no cap. Recovery state wins the Next Action cell when both apply; both callouts render.

**Clarify reports, and does not resolve.** A feature-targeted `/{project}:clarify` run reports outstanding scenario questions from the same field, naming every carrying scenario and the scenario-targeted command that resolves them. It reports in every gate branch where the field is non-empty — including the two that terminate without modifying a file (`already {status}` and `done`) and the `draft` rows before advancing to `clarified`. It neither walks nor resolves them: resolution stays scenario-targeted, so a feature-targeted run still writes to no scenario. The report is not a gate — `done` remains the only mechanized block, and a spec still advances `draft → clarified` carrying scenario questions. Its purpose is narrower and specific: a command that holds the signal must not answer an unresolved spec with an affirmative next step.

## Edge Cases

- No `scenarios/` directory, or scenarios with no Open Questions section: empty list, no gate block, no finding, no readout change.
- An unreadable or malformed scenario file contributes nothing and never blocks the gate — nothing can be proven about a file that will not parse, and an unknown is not escalated into a defect.
- Non-`.md` files and case-varying filenames are decided by the shared listing, not re-derived here.
- A question resolved between the gate check and the status write is accepted: the gate re-reads at check time and the resolution can only turn a block into a pass.
- Adding a scenario that carries questions still takes the scenario back-edge to `in-progress`, never the question back-edge to `draft` — the spec body's own Open Questions section is not written to.
- A feature with an empty `scenario-open-questions` list sees no change to any clarify branch: the report is suppressed entirely rather than rendered as "0 outstanding", so a clean feature reads exactly as it does today.
- Clarify's report never mutates. The `already {status}` and `done` branches keep their guarantee of modifying no file, and a feature-targeted run never writes to a scenario even when it names one.
- Recovery state and outstanding scenario questions together: recovery is the more upstream defect and governs the walk, exactly as it wins the dashboard's Next Action cell. The scenario questions are still reported, and remain to be resolved after the recovery walk returns the spec to `clarified`.
- On the markdown-only path the host reads each scenario's `## Open Questions` section directly, which the feature-targeted Scope Boundaries must permit as a narrow carve-out — those sections only, never scenario bodies.
- A scenario that arrived outside `/{project}:amend` is reported like any other. The report is derived from state at read time rather than from the event that created the scenario, so no entry path can bypass it.

## Open Questions

*None — all resolved.*

## Resolved Questions

**Should `/{project}:clarify` join `/{project}:target` and `/{project}:status` as a feature-targeted surface for the `scenario-open-questions` field?**

Yes — as a **reporting** surface only. Clarify names the scenarios carrying questions and the scenario-targeted command that resolves them; it does not walk or resolve them, and it does not gate.

[046](../../046-scenario-open-question-visibility/spec.md) diagnosed the gap correctly as *"discovery and completion-gating at feature level, not resolution"*, then enumerated the discovery surfaces without clarify among them. Clarify is a discovery surface — it is where a contributor goes to ask what is unresolved. It already holds the answer and discards it: its gate step invokes `read-spec`, whose result carries `scenario-open-questions` alongside the `open-questions` count it branches on (`runtime/src/primitives/read_spec.rs:45`, `runtime/src/schema/primitives.rs:388`). The consequence was a spec with zero body questions and a non-empty list stopping at *"Run `/{project}:implement` to continue implementation"* — an affirmative next step over exactly the questions the `done` gate blocks on, from a framework that ships `QUAL-CLAIM-001`.

This scenario's earlier never-merged rationale — *"feature-targeted clarify … would arrive with nothing to act on"* — was true of a **merged** count and false of the **separate** field. The Behavior section above now carries the rationale that survives: the two counts answer different questions. Merging remains rejected; only the reason changed.

**Reporting, not resolution.** Walking scenario questions from a feature-targeted run would mean writes to N scenario files across two abstraction levels, which the Scope Boundaries exclude by design, and resolution is already served by the scenario-targeted branch. 046's independent-*resolution* decision is retained in full; only the "does not *surface*" half is revised.

**Reporting, not gating.** Blocking `draft → clarified` on scenario questions would change what `clarified` means and displace the `done` gate 046 chose deliberately. The binding already exists upstream — §readiness-check counts *"the spec body's **and** those carried by any scenario under it"* at `planned → implement`. Clarify's job is to make it visible before planning, not to add a second gate.

**Why reporting is the durable form.** Scenarios can arrive outside `/{project}:amend` — hand-authored, migrated, merged in — bypassing every entry-time status consequence. A read-time report is derived from state, so it fires regardless of how the scenario got there. Entry-time detection is bypassable; this is not.

**Cross-spec impact, owed and not yet paid** (§cross-spec-impact, §drift-prevention *Decision resolution*). This decision contradicts prose in 046, which is `done` and outside a scenario-targeted run's boundary. Correcting it takes 046's meaningful-body-edit back-edge to `in-progress`. The sites: `046/spec.md:48-53` (the independent-resolution decision's second half), its acceptance criterion `:117` (*"Feature-targeted `/{project}:clarify` behavior is unchanged"*), `046/plan.md:27,29`, and `046/tasks.md:92`. The implementation sites in `framework/commands/clarify.md` are `:43` (the "not surfaced" boundary) and `:42` (the read prohibition, which needs the markdown-only carve-out recorded in Edge Cases above).
