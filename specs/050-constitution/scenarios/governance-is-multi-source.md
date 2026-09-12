---
section: "Follow-on scenarios"
---

# Governance-is-multi-source

## Context

§Classification sorts every `AGENTS.md` entry into two destinations. **universal** — true for any project running the ductus pipeline, promoted to `framework/constitution.md`. **project-only** — true because of something particular to this repository, left in `AGENTS.md` unchanged. **borderline** is not a third destination; it is a holding state the reword test resolves into one of those two.

That partition was exhaustive while a project could receive exactly one governing document. [055](../../055-shared-constitution/spec.md) ended that condition: a project registers shared constitutions under `.ductus/config.toml` `[constitutions.*]`, and `/{project}:target` loads each resolved document alongside the shipped `.ductus/constitution.md`, so their rules bind every command in the session with no per-command step.

A rule true across one organization's repositories now fits neither tier. It is not universal — it does not hold for every ductus project, and promoting it would push one organization's convention onto every adopter. It is not project-only either — it holds in more than one repository, and a rule retyped once per repository is exactly the drift [§drift-prevention](../../../framework/constitution.md#drift-prevention) exists to prevent, one tier above the one this spec addressed.

055 reached the same conclusion from its own side while resolving whether shared content should extend to `AGENTS.md`, and it reached it *using this spec's classification*: a convention true across five repositories is not project-only, so `AGENTS.md` cannot be its home. That reasoning is recorded in 055's Resolved Questions and is the reason this scenario exists — 050 owns which rules adopters receive, so the tier it is missing has to be recorded here rather than inferred from the spec that noticed it ([§cross-spec-impact](../../../framework/constitution.md#cross-spec-impact)).

## Behavior

§Classification names a third destination: **shared** — true across one organization's projects and no one else's. Its canonical text lives in a constitution document that organization owns and registers under `[constitutions.*]`. It is not promoted to `framework/constitution.md`, and it is not left as a per-repository `AGENTS.md` entry.

The three tiers are then a question about the rule's population, which is what the original two were testing without having to say so:

- true for **every** ductus project → universal → `framework/constitution.md`
- true for **more than one** project but not all → shared → that organization's registered constitution
- true for **exactly this** repository → project-only → `AGENTS.md`

§Promotion mechanism carries over unchanged and is what makes the tier worth having: one normative statement, in the shared constitution, with `AGENTS.md` keeping a line that points at it and states nothing of its own. Without that, a shared rule is a copy per repository, which is the state the tier exists to end.

The reword test carries over too, with its scope narrowed to match the tier. A universal rule must be statable without naming machinery only this repository has; a shared rule must be statable without naming machinery only *one project in the organization* has, or it is project-only wearing a shared label.

One bound belongs with the rule rather than being discovered later: a shared rule may add to the framework's rules and tighten them, never loosen them. The framework constitution is a floor, and [§governance-precedence](../../../framework/constitution.md#governance-sources-and-which-governs) is where that order is stated — together with the fact that nothing enforces it.

## Edge Cases

- **This repository classifies no entry as shared today.** `ductus` registers no `[constitutions.*]` entry, so the tier adds a destination without reclassifying any existing `AGENTS.md` entry. AC1's requirement that every entry carry exactly one classification is unaffected, and AC4 holds because no project-only entry is touched.
- **A shared rule later becomes universal.** Reclassified like any other entry: the canonical text moves to `framework/constitution.md` and the shared source drops it. No new mechanism — the same one-normative-statement rule decides where it lives.
- **A shared rule contradicts a framework rule.** §governance-precedence settles which governs and states plainly that nothing checks it. This scenario adds no enforcement and must not read as though it did.
- **An adopter registers no shared constitution.** Classification behaves exactly as it does today: two destinations, the reword test unchanged. The tier is inert rather than absent, which is the same posture 055 takes for the feature as a whole.
- **A rule is shared in substance but cites the organization's own tooling.** The narrowed reword test rejects it for the same reason the original rejects a universal rule citing `scripts/audit/` — the citation is unresolvable for some project the rule claims to govern.
- **A registered checkout is missing on a contributor's machine.** The rule is still classified shared; what changes is that the run reports the source as unexamined rather than loading it. Classification describes where a rule's canonical text belongs, not whether a given session managed to read it.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
