---
status: in-progress
dependencies: [009-scenario-targeting, 022-deterministic-runtime]
review:
  last-run: null
  reviewed-against: null
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  blocking: false
---

# 046 — Scenario open-question visibility

Make a scenario's unresolved open questions count toward its parent spec's completion state and remaining work, without merging them into the spec body's own question set.

A scenario exists to organize information — to keep `spec.md` from becoming one huge document. Its questions are still the spec's questions for the purpose of "is this feature done, and what is left?", even though they are authored, displayed, and resolved separately.

## Motivation

Every signal that answers *"does this spec have unresolved questions?"* reads the spec body only, so scenario-scoped questions are invisible wherever it matters:

- `read-spec` derives its open-question count from the spec body; nothing unions in `scenarios/*.md`.
- `/{project}:target` reads a scenario file only when a scenario is explicitly targeted, so a feature-level target never sees them.
- `/{project}:status` loads scenario detail only for the session-targeted scenario; per-spec rows carry a scenario *count* and nothing about their questions.
- `check-artifacts` has no family for them, so `/{project}:analyze` stays silent.

The consequence is that a spec whose scenario carries blocking design questions reports zero open questions, and the Status → next action table routes it to `/{project}:implement`.

### Observed case

Adopter repo `svc-zmc-api`, spec `033-shared-request-primitives`, scenario
`query-filter-type-conformance`: three open questions, one deciding a wire contract. `read-spec` reported no open questions, `check-artifacts` returned a single unrelated finding, and the pipeline pointed at `/implement`. Implementing then would have meant building against three unmade decisions.

### What already works

Two capabilities exist and are **not** part of this gap:

- `/{project}:clarify` has a scenario-targeted branch that resolves scenario open questions in place.
- `/{project}:target` and `/{project}:status` do route to `clarify (scenario-targeted)` when a scenario is targeted and carries questions.

The gap is **discovery and completion-gating at feature level**, not resolution. A contributor who already knows which scenario to target is served today; one who targets the feature is not.

## Design decision: independent resolution, shared completion

Resolution stays independent. `/{project}:clarify`'s documented boundary — spec-level and scenario-level questions are independent concerns, and feature-targeted clarify does not surface
scenario questions — is **retained**. Feature-targeted `/{project}:clarify` is unchanged.

Completion is shared. Because a scenario is an organizational split of the spec rather than a separate artifact class, its unresolved questions block the parent spec's `done`.

This rules out unioning scenario questions into the spec body's open-question count. That would make a feature-level target route to feature-targeted `/{project}:clarify`, which by its own boundary does not read scenarios — the command would arrive with nothing to act on. The signal is therefore **separate and additive**, not merged.

## Behavior

### A distinct scenario-question signal

The spec-reading surface reports scenario open questions as their own field, distinct from the spec body's open-question count, with each entry tagged by its source scenario file. The spec body count keeps its current meaning and its current effect on routing.

### Completion gating

A spec does not reach `done` while any scenario under it has unresolved open questions. This joins the existing `/{project}:review` gate on the `in-progress → done` transition rather than replacing
it: both must pass. §spec-lifecycle's definition of `done` and §readiness-check's "All open questions are resolved" state explicitly that scenario questions are included.

### What counts as an open question

The gate rests on an existing definition rather than a new one. §spec-requirements states that open questions must be resolved before the plan phase, and §spec-lifecycle makes `draft` the status that tolerates them: an **open question is an undecided blocker**. A question that would survive into `done` was never one.

A question deferred pending a condition — "not now; revisit when X lands" — is therefore **resolved, with a condition**, and belongs in the scenario's `## Resolved Questions`, whose template describes it as holding answers preserved for context. Deciding not to decide yet, and recording what will settle it, is an answer. The question stays next to the behavior it concerns; only its section changes.

This is deliberately a **convention, not machinery**. A `## Deferred Questions` section that the gate skipped, or a skip marker, would be a sanctioned hiding place: anything blocking could be relabelled to ship past the gate, which is the failure this spec exists to prevent, reintroduced under an approved name. The trade-off is accepted with its cost stated — classification is the author's judgment and nothing mechanical verifies it. A convention that can be misapplied is safer than a mechanism that legitimises the misapplication.

Both exits are surfaced wherever the gate or the finding fires: resolve the question, or record it as deferred with its trigger.

### Remaining work surfaces

`/{project}:target` and `/{project}:status` report outstanding scenario questions for a feature even when no scenario is targeted, and name the scenarios carrying them so the contributor can target one.
The recommended next action for such a feature is scenario-targeted clarification, not implementation.

### Analyze coverage

`/{project}:analyze` reports a finding for a spec with unresolved scenario questions, in the shape of the existing scenario-consistency family. The finding is blocking on a `done` spec — that state contradicts the completion rule above — and advisory otherwise.

This extends surfaces introduced by [022-deterministic-runtime](../022-deterministic-runtime/spec.md) and builds on scenario targeting from [009-scenario-targeting](../009-scenario-targeting/spec.md).

## Implementation ownership

This spec owns the requirement — what the signal means, that it gates `done`, and how it surfaces. It does not own the primitives that carry it.

The primitive-level changes land as **scenarios under 022-deterministic-runtime**, which owns the deterministic runtime and the command steps that invoke it:

- the distinct scenario-open-question field on the spec reader;
- the third pre-done gate check, ordered after markdown lint and before the `review:` block;
- the `scenario-open-questions` finding family and its `--fix` revert;
- the dashboard's Scenarios-column suffix, Next Action override, and callout;
- both parser fixes — comment- and fence-awareness, and the widened placeholder guard.

Each of those scenarios back-links here per §cross-spec-impact. Recording them on a `done` 022 takes the scenario back-edge to `in-progress` — the same edge this spec's first resolved question affirms.

What stays with this spec: the constitution amendments to §spec-lifecycle's `done` definition and §readiness-check, and the acceptance criteria below. Those criteria are verified against shipped behavior regardless of which spec's tasks produced it, so this spec cannot reach `done` before the 022 scenarios do.

## Edge Cases

- **No `scenarios/` directory, or scenarios with no `## Open Questions` section** — zero outstanding questions. No signal, no gate block, no finding, no change to any readout.
- **A scenario whose Open Questions section holds only a placeholder** — zero, per the placeholder rule above.
- **An unreadable or malformed scenario file** — never blocks the `done` gate and never produces a blocking finding. Nothing can be proven about a file that cannot be parsed, and an unknown is not escalated to a defect; it surfaces as informational, matching how an unreadable cross-reference target is classified. The defect is the file's, and lint owns it.
- **Non-`.md` files, and case-varying filenames, under `scenarios/`** — the shared scenario-file listing defines the set, so this feature inherits its behavior rather than defining a second rule.
- **Many scenarios carrying questions** — all are listed, with no cap and no truncation. A bounded readout that silently dropped scenarios would read as "these are the ones that need attention" while hiding others.
- **A question resolved between the gate check and the status write** — the gate re-reads the scenarios at check time, and `set-status` guards only the `from` status. A resolution landing inside that window is accepted: it can only turn a block into a pass, and the next gate run agrees.
- **A `done` spec that already carries unresolved scenario questions when this feature ships** — reported as a blocking finding and reverted to `in-progress` by `--fix`, with no grandfather exemption. Unlike an absent `review:` block, which genuinely marks a spec as predating its feature, unresolved scenario questions are a real present-tense defect regardless of when they arrived. Exempting them would preserve exactly the state this spec exists to prevent.

## Acceptance Criteria

- [x] Scenario open questions are reported as a field distinct from the spec body's open-question count, with each entry tagged by its source scenario file
- [x] The spec body's open-question count is unchanged in meaning and value by this feature
- [x] Feature-targeted `/{project}:clarify` behavior is unchanged — it neither surfaces nor resolves scenario questions
- [x] A spec with one or more unresolved scenario open questions cannot be advanced to `done`
- [x] The `done` block is reported with the blocking scenario named, not as a generic gate failure
- [x] A spec whose scenarios have no unresolved questions advances to `done` exactly as it does today
- [x] §spec-lifecycle's `done` definition and §readiness-check state that scenario open questions are included
- [x] `/{project}:target` on a feature with unresolved scenario questions reports their count and the scenarios carrying them, without a scenario being targeted
- [x] When several scenarios carry questions, `/{project}:target` lists all of them in case-insensitive filename order and recommends no specific one
- [x] `/{project}:target` on such a feature recommends scenario-targeted clarification rather than `/{project}:implement`
- [x] `/{project}:status` shows the outstanding scenario-question count as a suffix on the existing Scenarios column, and leaves the cell unchanged when the count is zero
- [x] `/{project}:status` overrides the Next Action cell to `clarify (scenario)` for a spec with outstanding scenario questions, and renders a callout below the table naming the specs and the scenarios carrying them
- [x] A spec in recovery state with outstanding scenario questions renders `clarify (recovery)` in the Next Action cell and both callouts
- [x] `/{project}:analyze` reports a finding for a spec with unresolved scenario questions — blocking at `done`, advisory otherwise
- [x] A feature with no `scenarios/` directory, or with scenarios carrying no Open Questions section, produces no finding and no behavior change
- [x] Scenario open questions are parsed by the same parser the spec body uses, with `## Resolved Questions` entries excluded
- [x] Questions inside HTML comments or fenced code blocks are not counted, in a scenario or in a spec body — a spec scaffolded from the shipped template reports zero open questions, not the template's commented-out examples
- [x] Both the spec placeholder (`*None — all resolved.*`) and the scenario placeholder (`*None — captured during scenario authoring.*`) are skipped, including when authored as a list bullet
- [x] Recording a scenario that carries open questions on a `done` spec takes the scenario back-edge to `in-progress`, not the question back-edge to `draft`; the spec body's `## Open Questions` section is not written to
- [x] The pre-done review gate evaluates scenario open questions as a third check, ordered after markdown lint and before the `review:` block, and the first failing check still wins
- [x] The gate's blocked message names the scenarios carrying unresolved questions
- [x] An unreadable or malformed scenario file never blocks the `done` gate and never produces a blocking finding
- [x] A `done` spec carrying unresolved scenario questions when this feature ships is reported and reverted with no grandfather exemption
- [x] All scenarios carrying questions are listed with no cap or truncation
- [x] The gate's guidance and the analyze finding's suggested fix both offer two exits — resolve the question, or record it in `## Resolved Questions` as deferred with its trigger condition
- [x] A question recorded under `## Resolved Questions` never blocks `done`, whether or not it names a deferral condition
- [x] No section or marker exists that exempts a question from the gate while it remains under `## Open Questions`
- [x] The primitive-level changes land as scenarios under `022-deterministic-runtime`, each back-linking to this spec, with 022's data-model updated for the new field and finding family
- [x] The constitution amendments to §spec-lifecycle and §readiness-check land with this spec, not with 022
- [x] `/{project}:analyze --fix` reverts a `done` spec with unresolved scenario questions to `in-progress`, matching the review-state-drift revert, and emits a non-silent notice naming the spec

## Open Questions

*None — all resolved.*

## Resolved Questions

**Does adding a scenario that carries open questions take the question back-edge (`clarified | planned | in-progress → draft`) rather than the scenario back-edge (`done → in-progress`)?**

No — the scenario back-edge is unchanged. Adding a question-carrying scenario moves a `done` spec to `in-progress`, exactly as adding any other scenario does.

The question back-edge exists because, per §spec-lifecycle, `draft` is the only status that tolerates open questions — but that reasoning is about **spec-body** questions. Scenario questions are a separate concern under this spec's independent-resolution decision, and the spec body's count is unchanged in meaning and value. Taking the question edge would therefore park the spec at `draft` (the "has unresolved questions" state) while its `## Open Questions` section is empty — a status contradicting its own artifact — and would route the contributor to feature-targeted `/{project}:clarify`, which does not read scenarios and would arrive with nothing to act on. That is the same dead end that ruled out unioning the counts.

The routing pressure comes from the scenario-question signal and the `done` gate instead. `in-progress` with an outstanding scenario-question signal is both true and actionable: it routes to scenario-targeted `/{project}:clarify`, which can resolve them. This also keeps the back-edge rule unconditional — one trigger, one edge, no branch on scenario content.

**Where does the `done` gate live — `/{project}:implement`'s completion step, the review gate alongside `review.blocking`, or a `check-artifacts` finding that `/{project}:analyze --fix` reverts?**

Both the review gate and the analyze finding, mirroring how review-state drift already works. These are not competing options: §spec-lifecycle states the review gate *"composes with `/{project}:analyze` (which flags drifted `done` specs) and the shipped CI template … per the Design Principles rule: never depend on human diligence."*

- **Prevention** — the pre-done review gate gains scenario open questions as a third check. It is structurally identical to the two already there: a deterministic pre-done condition returning a canonical `blocked: …` message as a domain outcome, evaluated before the completion confirmation and the status write.
- **Detection** — the `check-artifacts` scenario-open-questions family catches a spec that reached `done` anyway (a bypassed gate, a hand-edited frontmatter, a spec predating this feature), and `--fix` reverts it `done → in-progress` exactly as it does for review drift.

Check order within the gate is markdown lint → scenario open questions → review block, first failing check wins. An unresolved design question is more upstream than a missing review, so surfacing it first avoids sending a contributor to run `/{project}:review` against a design that is about to change.

Describing the check in `/{project}:implement`'s completion step remains correct for the markdown-only path, but prose alone is not the home — that would leave the check unmechanized on the runtime path.

**How does `/{project}:status` render outstanding scenario questions in a glance-sized table row?**

By mirroring the existing recovery-state treatment, which already does both of the things this question framed as alternatives — a Next Action override in the row plus a callout line below the table. No new column is added.

- **Scenarios column** — the existing count gains a suffix when the spec has outstanding scenario questions (`3` → `3 (2 open)`), and is unchanged when it has none. Reusing the column keeps the glance table from growing.
- **Next Action** — overrides to `clarify (scenario)`, the same shape as the existing `clarify (recovery)` override.
- **Callout below the table** — names the specs and the scenarios carrying questions. A table cell cannot hold scenario slugs, so this is where the naming requirement is met for `/{project}:status`; `/{project}:target` renders a single feature and names them inline instead.

**Precedence.** When a spec is in recovery state (spec-body questions at `clarified` or later) *and* has outstanding scenario questions, recovery wins the Next Action cell — it is the more upstream defect and reverts the spec to `draft`, after which the scenario questions remain to be resolved. Both callouts render; only the cell is exclusive.

**When several scenarios carry questions, does `/{project}:target` recommend a specific one, or list them all?**

List them all; do not single one out. The recommended *action* stays singular and unambiguous — scenario-targeted `/{project}:clarify` — but which scenario to target is the contributor's choice.

Nothing mechanical can rank them. Question count is not importance: one wire-contract decision outweighs three cosmetic ones, and no deterministic signal distinguishes them. An ordering presented as a recommendation would be arbitrary authority manufactured from a signal that does not exist, which §grounding cuts against. There is also a direct precedent — `/{project}:target` given an unmatched scenario slug already lists the available scenarios and asks the user to choose.

Ordering is the existing case-insensitive filename order used by the shared scenario-file listing — the same set and order `check-artifacts` derives scenario slugs from and the dashboard counts. Reusing it keeps the readout deterministic and avoids introducing a second ordering rule.

**Is the scenario question parser the same one that reads the spec body's `## Open Questions`?**

Yes — the same parser, applied to the scenario body with the same section heading. It is already parameterized by heading, already folds wrapped continuation lines, and is already shared with `append-question` so that dedup and reads agree on the entry set. `## Resolved Questions` needs no special handling: the section walker scopes to a single heading, so resolved entries are excluded structurally.

Reuse is conditional on two defects being fixed as part of this feature, both found by inspecting current behavior:

1. **The questions parser is not comment-aware.** It uses the plain section walker, while the acceptance-criteria parser uses the comment- and fence-aware variant. Scaffolding a spec from the shipped template and reading it back returns the template's three *commented-out* example questions as real open questions, while acceptance criteria correctly comes back empty. Today this is largely latent, because `/{project}:specify` overwrites the comment block moments later. Under this spec it stops being latent: a commented-out bullet in a scenario's Open Questions becomes an unresolved question that blocks the parent spec from `done`, turning a cosmetic miscount into a hard gate failure. The parser moves to the comment- and fence-aware walker, which also fixes the phantom-question count for spec bodies.

2. **The placeholder guard is spec-specific.** It skips an exact match on `*None — all resolved.*`, but `create-scenario` emits `*None — captured during scenario authoring.*`. Neither is a list bullet, so both are ignored today — the safety is incidental rather than intended. The guard becomes a small set covering both placeholder strings so the behavior does not depend on nobody authoring a placeholder as a bullet.

Both are pre-existing defects that this feature raises the severity of rather than introduces; they are folded in here rather than tracked separately, because the `done` gate is not safe to ship without them.

**Does this interact with the link-target state read specified in spec `045-decision-state-drift-detection`?**

Yes, in one direction only: this spec owns the scenario open-question capability and 045 consumes it. The dependency is declared in 045's frontmatter and absent from this spec's, keeping the generated graph acyclic — which is also why 045 is referenced here by slug rather than by inline link.

What 045 needs is exactly what the parser decision above specifies: a scenario's open-question count, read through the shared comment-aware parser. It reads that count; it does not reinterpret it. 045's own question about how a link targeting a scenario behaves — a scenario has no `status` field, so the question count is the only target state available — belongs to 045 and constrains nothing here.

One invariant is worth stating so it is not rediscovered later: the count 045 reads MUST be the same count that gates `done`. A second, independent scenario-question reader in 045 could flag a link as contradicting a state the gate disagreed with. The shared parser named above is the single source, which the parser criterion already requires.

This question adds no work to this spec; it records the relationship.
