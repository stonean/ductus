---
section: "Follow-on scenarios"
---

# A-declared-cross-spec-impact-gates-done

## Context

[§cross-spec-impact](../../../framework/constitution.md#cross-spec-impact) states the rule and then states its own enforcement in one sentence: *"The originating spec's acceptance criteria include delivering the cross-spec update. This ensures the change is tracked as part of the work that discovered it."*

An acceptance criterion is author-written prose. Nothing requires one to exist, nothing checks that it names the affected spec, and a spec with no such criterion passes every gate the framework has. So the sentence describes a convention and reads as a mechanism, which is the shape [§grounding](../../../framework/constitution.md#grounding) rejects — a rule implying enforcement it does not have.

What exists points the other way. `diff-cross-spec` runs at `/{project}:implement` steps 7 and 13 and reports sibling spec paths **changed** in the feature's window, explicitly *"Informational; does not block."* That detects the cross-spec update an author *made*. Nothing detects the one they *owe*, and only the second is a defect — touching another spec is ordinary work, while owing a touch and closing anyway is the failure this section exists to prevent.

Observed on 2026-09-11 closing [055](../../055-shared-constitution/spec.md), which made an adopter's governance multi-source and therefore owed this spec an update. The obligation was identified early and written into 055's body as prose, then moved to `specs/inbox.md` when that section was retired at the completion gate. Both are channels the pipeline reads for other purposes and neither gates, so 055 passed its review, its analysis, the pre-`done` gate and the release audit with the obligation outstanding, and `ductus-v0.47.0` was published before the operator caught it. `diff-cross-spec` reported `cross-spec-paths: []` at that gate — true, and worthless: 055 had not touched 050 precisely because the work had not been done.

The failure was not a failure to notice. It was noticed, written down twice, and had no durable channel that could hold a transition.

## Behavior

§cross-spec-impact separates what the framework can enforce from what it cannot, and says which is which.

**A declared impact is an obligation that gates `done`.** When work on spec A identifies a change that belongs in spec B, A records the obligation in frontmatter rather than only in prose or an acceptance criterion, and A cannot reach `done` while any declaration is undischarged. This is the same category the pre-`done` gate already holds for an undischarged fold: a spec carrying an obligation nobody has discharged is not a candidate for `done`, so whether its review is fresh does not yet matter. The mechanism lives in [022](../../022-deterministic-runtime/spec.md) as a gate check; this section states the requirement.

**Discharge is provable, not asserted.** An impact is discharged when spec B carries the change with a back-link to A — the signpost §cross-spec-impact already requires. Keying discharge on the reciprocal link rather than on the declaration being removed is what makes it a gate: a key an author deletes to unblock themselves enforces nothing.

**Nothing detects an impact that was never declared, and that is stated rather than implied.** Whether work on A implies a change to B is semantic judgment, and no check can make it. [§design-principles](../../../framework/constitution.md#design-principles) is explicit that the right answer to a feature with no derivable design is to defer it, not to ship the disciplined version — so the undeclared case stays the author's and the reviewer's, named as such. What the framework closes is the narrower and more common failure: an obligation that *was* recognized and then evaporated because the only places to record it were prose and a backlog.

The acceptance-criterion sentence is replaced rather than kept alongside. Leaving it would give the rule two enforcement stories, one real and one aspirational, which is the drift [§drift-prevention](../../../framework/constitution.md#drift-prevention) exists to prevent — a criterion naming the affected spec remains good practice and is no longer described as what ensures the update happens.

## Edge Cases

- **An impact on a spec that is `done`.** Discharging it reopens that spec through the existing scenario or meaningful-body-edit back-edge. No new edge is added, and the reopen is the ordinary price §spec-lifecycle already sets.
- **Several affected specs.** The declaration is a list and discharge is per entry, so partially discharged is a real state the gate reports rather than rounding to blocked-or-clear. This is the axis on which the obligation differs most from a fold, which has exactly one target.
- **The affected spec does not exist yet.** The impact is on a spec that has to be written first; the declaration names it once it exists, and until then the work is an ordinary missing-spec route through `/{project}:specify`.
- **The project config gains a key.** Already exempt and stays exempt: §cross-spec-impact's existing paragraph treats `.ductus/config.toml` as a shared project-side database, so a new table is not cross-spec impact and never needs declaring.
- **A mechanical sweep touches another spec.** A uniform rename across live artifacts is not a cross-spec impact — it changes no claim, which is the test §spec-lifecycle already states — so it is not declarable and the gate never sees it.
- **The obligation is discovered after the spec closes.** The scope test still applies and reaching the closed spec costs a back-edge, exactly as a late finding does today. The gate governs the transition, not the question of whether an impact exists.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
