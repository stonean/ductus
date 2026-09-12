---
section: "Follow-on scenarios"
---

# The-cross-spec-impact-gate

## Context

[050](../../050-constitution/spec.md)'s `a-declared-cross-spec-impact-gates-done` requires that a declared cross-spec obligation gate `done`, and that discharge be provable rather than asserted. This scenario is the runtime half: the frontmatter field, the gate check, and where each sits.

The gate already holds one obligation of this category. `check_review_gate`'s `pending_fold_block` blocks `in-progress → done` whenever `folds-into` is present, and its doc comment states the reasoning the new check inherits verbatim — *"a spec carrying an obligation nobody has discharged is not a candidate for `done`, so whether its review is fresh does not yet matter."* Both checks therefore sit ahead of the `review:` and `analyze:` blocks, for that one reason.

The resemblance stops at the reasoning, and the question of whether the two should share code has an answer rather than a preference.

## Behavior

A spec declares affected specs in a `cross-spec-impact:` frontmatter key — a list of feature slugs. `check-review-gate` gains a check, ordered beside `pending_fold_block`, that blocks `in-progress → done` while any declared entry is undischarged, naming the undischarged entries in the message and pointing at the back-edge in the guidance.

**An entry is discharged when the named spec's body links back to the declaring spec.** The reciprocal link is the signpost §cross-spec-impact already requires, so the check proves the obligation was met rather than trusting that a key was removed honestly. Link parsing is the corpus's existing sibling-link machinery — the same reading `derive-dependencies` performs — so the check introduces no second parser.

The result reports per-entry state, not a single boolean: `discharged`, `undischarged`, and `target-missing` for an entry naming no feature directory. `target-missing` is a finding rather than a silent pass — a declaration pointing nowhere is a typo the operator wants named, and treating it as discharged would let one letter disable the gate.

### It does not share logic with `folds-into`, and does not enhance it

Extracting a common helper was evaluated and rejected. The two diverge on every axis the code turns on:

| | `folds-into` | `cross-spec-impact` |
| --- | --- | --- |
| Cardinality | one target | a list |
| Discharge | the key's absence | the target's reciprocal link |
| Reads the target spec | **never**, deliberately | necessarily |
| Partial state | does not exist | the normal case |

`pending_fold_block` is four lines over an `Option<&str>` — `let target = folds_into?` and build the result. A helper generalising that over a list-valued key whose discharge requires reading another spec from disk would be longer than both call sites and would have to carry the deliberate not-checked rule as a parameter. That rule is load-bearing: a fold target normally lives on the upstream branch before the merge, so the fold check *must not* look for it, while the cross-spec check *must*. One function honouring both would be two functions wearing a shared signature.

Enhancing `folds-into` to carry this instead is rejected for a stronger reason than code shape: the two obligations point in opposite directions. A fold moves the **declaring** spec's own content into its home, and a branch-scoped spec is retired rather than completed — it has no `done` state at all. A cross-spec impact requires a change to **another** spec's content, and discharging it leaves the declaring spec perfectly completable. Folding them together would give the branch-scoped form a `done` state it must not have.

What they share is the gate's ordering rationale and their position in the check sequence. That is a category in `ReviewGateBlock`, expressed as a new variant beside `PendingFold` and a doc comment citing the same reasoning — not a shared function.

## Edge Cases

- **The key is absent.** No check runs and nothing is reported. Absence is never a finding, matching `folds-into`: most specs affect no other spec, and a per-spec informational line would put noise on the whole corpus.
- **The key is present but empty.** Indistinguishable from absent, the posture `[constitutions]` and `dependencies: []` already take for an empty collection.
- **A declared entry names the declaring spec itself.** Reported as a finding rather than trivially discharged — a spec cannot discharge an obligation to itself, and a self-entry is a typo.
- **The target links back from a scenario rather than the spec body.** Discharged. The signpost's job is that a reader of the affected spec finds the pointer, and a scenario under it is part of what they read.
- **The target's link is added and later removed.** The gate re-blocks if the declaring spec is reopened, which is correct: the obligation is undischarged again. It does not retroactively reopen a `done` spec — no gate does, and `/{project}:analyze`'s drift checks are where that surfaces.
- **A `--fix` path.** None. The check reports and never writes: which section of the affected spec should carry the signpost is the routing judgment `/{project}:amend` puts to the operator, and a wrong auto-write into another spec is worse than a precise refusal.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
