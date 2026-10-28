---
section: "Follow-on scenarios"
---

# Artifacts-are-as-short-as-their-required-content

## Context

The owner asked on 2026-10-28 that everything the pipeline writes into an artifact be succinct without losing information the artifact requires. `the-constitution-states-rules-not-stories` set that shape for the constitution's own rules; nothing stated it for any other artifact, so adopters had no rule for their specs, plans, tasks, scenarios, rules, inbox items or records.

## Behavior

- `framework/constitution.md` §text-first-artifacts **Principles** carries this bullet, as the owner approved it on 2026-10-28:

  > **Write each artifact as short as its required content allows, and no shorter.** Required content is what a gate, a check, or a later reader acting on the artifact needs: every claim with its reason and boundary, its provenance (§grounding), and what is outstanding (§design-principles). Cut the rest: restatement (point at the canonical source, §drift-prevention), how a decision was reached (git keeps it), examples a rule does not need, and hedges. A cut that drops required content is a defect, not a saving. Nothing enforces this beyond `check-artifact-size`'s advisory read-size finding.

- "Required" is defined by pointing at the rules that already demand content, not by restating them, so it cannot drift from them.

## Edge Cases

- **The ceiling.** The bullet is 599 bytes (`wc -c`, newline included, 2026-10-28), and `framework/constitution.md` was 49,997 bytes against the 50,000-byte ceiling `scripts/lint-governing-size.sh` enforces; it landed within the ceiling (`plan.md` §The constitution fits one read). Landing it cuts at least 596 bytes elsewhere, measured before it is planned, keeping verbatim the passages other specs quote; otherwise the owner reverses `the-constitution-fits-one-read`'s resolution to hold 50,000, recorded in 072.
- **No copies.** The constitution loads at startup and every command reads it, so no command or template restates the rule (§drift-prevention).
- **Existing artifacts are not swept.** The rule governs what is written; it orders no pass over existing artifacts, which meet it when next edited.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
