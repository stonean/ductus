---
section: "Behavior"
---

# A-retired-feature-leaves-no-spec

## Context

The constitution said what happens to an obsolete **scenario** — [§scenarios](../../../framework/constitution.md#scenarios): *"If a scenario becomes obsolete, it is deleted — not marked with a status."* It said nothing about an obsolete **spec**.

That gap was not theoretical, and it had already been papered over once. Spec 054 retired supersession. Spec 053 had shipped the reconciliation pass and nothing in it survived, so 054's **AC18** deleted 053's directory outright and re-pointed every inbound reference to 052. The right outcome — reached by one spec's explicit acceptance criterion rather than by any standing rule. Nothing would have made the next retirement do the same.

It surfaced on 2026-09-05 through an error rather than a defect. `/{project}:status` was summarised to the user as showing 052's feature retired while its spec sat at `done`; the user's response was that the spec should then be deleted, because *specs carry durable information and a removed feature is not durable*. The summary was wrong — 054 removed only 052's supersession half, and its consolidation half ships today as `/{project}:consolidate` — but the principle drawn from it was right, and checking it exposed that the framework had been practising the rule without ever stating it.

## Behavior

**When a feature is removed from the project, its spec directory goes with it.**

`done` means *delivered, and still true*. A spec describing something the project no longer has is neither. Leaving it in place quietly converts the corpus from a description of the system into an archive of everything the system was ever intended to be — two different artifacts, and only the first is worth trusting.

The cost is not confined to the retired spec. A reader cannot tell a live spec from a retired one by looking, so **one retired spec left at `done` puts every other spec's status in question**. That is the same asymmetry `QUAL-CLAIM-001` names for results: the reassuring reading is the one a reader takes, and here it is taken about the whole corpus.

- **Content that belongs with another spec** goes through `/{project}:consolidate`, which re-points every inbound pointer **before** removing the directory. Pointer-first is not politeness: a deleted spec with live inbound references trades one durability problem for a worse one — dangling links that `check-corpus-links` will then block commits on.
- **Content that survives nowhere** takes the spec with it. 053 is the worked example.
- **Partial retirement is an ordinary body edit.** The spec stays and describes what remains. 052 is that example, and the distinction is the one the mis-summary blurred: 054 removed a *half*, so 052 was edited down and kept, and its H1 was narrowed to name consolidation alone.

Git history is the record of what *was*, which is why a living artifact never has to be. Deleting a retired spec loses nothing.

## Edge Cases

- **A spec whose feature is retired but whose scenarios document current behavior.** Then the feature is not fully retired; that is the partial case, and the spec stays with the surviving content.
- **Inbound references from `done` specs.** They are re-pointed, not left dangling. Consolidation does this; a hand deletion must do it too, and `check-corpus-links` fails the commit if it does not — the one part of this rule that is already mechanically enforced.
- **A retired feature whose spec is the *target* of another's `folds-into`.** The fold is discharged first. Removing a fold target strands the staging spec with nowhere to go, which `/{project}:fold` refuses at fold-back and nothing would catch here.

## What this does not do

**Nothing enforces it.** The rule is convention plus `/{project}:consolidate` doing the right thing when invoked; nothing notices a retired feature whose spec was left behind, because nothing can tell "retired" from "done" without knowing what the project still ships. That is the same footing the analyze record had before [047](../../047-analyze-findings-durability/spec.md) gave it a gate, and it is stated here rather than left to be discovered — a rule whose enforcement is "someone remembers" is a diligence dependency, and naming it as one is the minimum owed while it stays that way.

A check is conceivable — a `done` spec whose acceptance criteria name only paths that no longer resolve is a candidate signal — and `criterion-path-existence` already computes most of it. It is deliberately not built here: that family's `not-a-live-claim` exemptions exist precisely because criteria legitimately describe removals, so the signal would need care to avoid firing on every spec that retired something. Writing the rule down first is what makes that check specifiable later.

## Resolved Questions

- **Why the constitution rather than 052, which owns consolidation?** Because the rule holds whether or not `/{project}:consolidate` is used — a feature removed with nothing to consolidate into is still a spec that must go. 052 owns the *mechanism*; this is a statement about what a spec is for, which is the constitution's register and this spec's subject.
- **Does this conflict with "spec bodies are living documents that represent current state"?** It is the same principle taken to its limit. A living document that describes nothing living is not current state, and the anti-proliferation stance — evolve the existing spec rather than spawning a new one — is about where *new* content goes, not about keeping dead content indefinitely.
