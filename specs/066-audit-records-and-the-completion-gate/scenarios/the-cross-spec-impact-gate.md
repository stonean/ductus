---
section: "The completion gate"
---

# The-cross-spec-impact-gate

## Context

[050](../../050-constitution/spec.md)'s `a-declared-cross-spec-impact-gates-done` requires that a declared cross-spec obligation gate `done`, and that discharge be provable rather than asserted. This scenario is the runtime half: the frontmatter field, the gate check, and where each sits.

The gate's earlier obligation of this category, the fold check, was removed by 051's `a-pending-fold-never-holds-done`; this check inherits only its reasoning — a spec carrying an obligation nobody has discharged is not a candidate for `done` — and shares none of its code.

## Behavior

A spec declares affected specs in a `cross-spec-impact:` frontmatter key — a list of feature slugs. `check-review-gate` gains a check, ordered ahead of the review record, that blocks `in-progress → done` while any declared entry is undischarged, naming the undischarged entries in the message and pointing at the back-edge in the guidance.

**An entry is discharged when the named spec's body links back to the declaring spec.** The reciprocal link is the signpost §cross-spec-impact already requires, so the check proves the obligation was met rather than trusting that a key was removed honestly. Link parsing is the corpus's existing sibling-link machinery — the same reading `derive-dependencies` performs — so the check introduces no second parser.

The result reports per-entry state, not a single boolean: `discharged`, `undischarged`, `target-missing` for an entry naming no feature directory, and `self-reference` for an entry naming the declaring spec itself. `target-missing` is a finding rather than a silent pass — a declaration pointing nowhere is a typo the operator wants named, and treating it as discharged would let one letter disable the gate.

A missing target blocks here while an owed `folds-into` never holds `done` — `check-unfolded-specs` and the pipeline view carry an owed fold past the transition, and nothing but this gate carries an undischarged impact — and since a fold moves the declaring spec's own content while an impact requires a change to another spec's, the two keys share no code.

## Edge Cases

- **The key is absent.** No check runs and nothing is reported. Absence is never a finding: most specs affect no other spec, and a per-spec informational line would put noise on the whole corpus.
- **The key is present but empty.** Indistinguishable from absent, the posture `[constitutions]` and `dependencies: []` already take for an empty collection.
- **A declared entry names the declaring spec itself.** Reported as a finding rather than trivially discharged — a spec cannot discharge an obligation to itself, and a self-entry is a typo.
- **The target links back from a scenario rather than the spec body.** Discharged. The signpost's job is that a reader of the affected spec finds the pointer, and a scenario under it is part of what they read.
- **The target's link is added and later removed.** The gate re-blocks if the declaring spec is reopened, which is correct: the obligation is undischarged again. It does not retroactively reopen a `done` spec — no gate does, and `/{project}:analyze`'s drift checks are where that surfaces.
- **A `--fix` path.** None. The check reports and never writes: which section of the affected spec should carry the signpost is the routing judgment `/{project}:amend` puts to the operator, and a wrong auto-write into another spec is worse than a precise refusal.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
