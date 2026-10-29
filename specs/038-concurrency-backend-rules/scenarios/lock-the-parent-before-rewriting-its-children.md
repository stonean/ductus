---
section: "Rule set scope"
---

# Lock-the-parent-before-rewriting-its-children

## Context

No rule in `concurrency-backend.md` (read 2026-10-25 at HEAD f24c55c6) covers a transaction that locks some of a parent's child rows and then deletes or rewrites the whole set. Two such transactions that each lock one child first and then reach for all of them wait on each other, and the database aborts one — as reported, PostgreSQL detects the deadlock after `deadlock_timeout` and cancels one transaction. The failure appears only under concurrent load.

## Behavior

- `concurrency-backend.md` carries a rule, at the MUST tier, that a transaction which deletes or rewrites a set of child rows as a whole first takes a lock on the parent row that owns the set, so concurrent writers to the same set serialize on one row before touching any child.
- The rule's Verification names what a reviewer checks in the transaction's statement order.
- Every claim the rule makes about database behavior is fetched from the database's documentation at authoring time, never recalled, and the rule's Source cites what was fetched.

## Edge Cases

- A transaction touching one child row only: the trigger does not fire.
- A set rewritten under a lock on a different serializing row (an advisory lock keyed on the parent): satisfies the rule if the rule's wording allows an equivalent serializer; authoring decides.
- Overlap with existing lock-ordering rules in the file: the new rule cross-references rather than restates, per 038's boundaries.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
