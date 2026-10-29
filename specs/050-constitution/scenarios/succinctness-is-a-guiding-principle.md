---
section: "Ownership"
---

# Succinctness-is-a-guiding-principle

## Context

The owner chose on 2026-10-29 to raise succinctness from a writing rule to a guiding principle. `artifacts-are-as-short-as-their-required-content` put the rule in §text-first-artifacts **Principles**, where it governs what is written but is not one of the criteria a design is judged against.

## Behavior

- The Technology list under §principles carries this line, as the owner approved it on 2026-10-29:

  > - **Succinct:** artifacts hold all their readers need, and no more

- "All" forbids dropping required content and "no more" forbids the rest. What counts as required stays defined once, by the §text-first-artifacts bullet, so the line carries no pointer and restates none of it.
- It is an evaluation criterion like its siblings, not a design principle: the exit an oversized artifact needs already follows from §design-principles' rule against features that depend on human diligence, and `075-agents-md-holds-what-every-session-needs` supplies one for the project-level files.

## Edge Cases

- **The ceiling.** The line is 67 bytes (`wc -c`, newline included), and `framework/constitution.md` measured 49,938 bytes against the 50,000-byte ceiling `scripts/lint-governing-size.sh` enforces (2026-10-29). Landing it cuts at least 5 bytes elsewhere in the same change, dropping no required content and keeping verbatim any passage another spec quotes.
- **No copies.** No command, template or `AGENTS.md` entry restates the line.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
