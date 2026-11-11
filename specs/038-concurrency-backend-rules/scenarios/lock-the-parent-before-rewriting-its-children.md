---
section: "Rule set scope"
---

# Lock-the-parent-before-rewriting-its-children

## Context

No rule in `concurrency-backend.md` (read 2026-10-25 at HEAD f24c55c6) names the order for a transaction that locks some of a parent's child rows and then deletes or rewrites the whole set: `BE-LOCK-002` asks for one consistent acquisition order whenever more than one lock is held, which covers the case in principle. Two such transactions that each lock one child first and then reach for all of them wait on each other, and the database aborts one: PostgreSQL waits `deadlock_timeout` before it checks for a deadlock, then resolves it "by aborting one of the transactions involved" (PostgreSQL documentation, *Explicit Locking* and *Lock Management*, fetched 2026-11-08). The failure appears only under concurrent load.

## Behavior

- `concurrency-backend.md`'s `BE-LOCK-002` flags, at its SHOULD tier, a spec or plan whose transaction deletes or rewrites a set of child rows as a whole without first taking a lock on the parent row that owns the set, or an equivalent serializer, so concurrent writers to the same set serialize on one row before touching any child.
- The Verification names what a plan states: the parent's lock, or its equivalent, taken before any child row's.
- Every claim the rule makes about database behavior is fetched from the database's documentation at authoring time, never recalled, and the rule's Source cites what was fetched.

## Edge Cases

- A transaction touching one child row only: the trigger does not fire.
- A set rewritten under a lock on a different serializing row (an advisory lock keyed on the parent): satisfies the rule, which names such a lock as an equivalent serializer.
- Overlap with existing lock-ordering rules in the file: the requirement is a clause of `BE-LOCK-002`, so it restates nothing and mints no ID.
- A transaction that rewrites the sets of several parents: it takes the parents' locks in one consistent order, as `BE-LOCK-002` already asks of any transaction holding more than one lock.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **MUST or SHOULD, and a rule of its own or a clause of `BE-LOCK-002`?** SHOULD, as a clause of `BE-LOCK-002` (decided by the owner's delegation, 2026-11-08). PostgreSQL resolves such a deadlock "by aborting one of the transactions involved, allowing the other(s) to complete" (*Explicit Locking*, Deadlocks) after waiting `deadlock_timeout`, "the amount of time to wait on a lock before checking to see if there is a deadlock condition" (*Lock Management*), both fetched 2026-11-08 from postgresql.org/docs/current: the cost is an aborted transaction and the wait, not corrupted data, so this spec's Resolved Question *MUST vs. SHOULD default* and AC5 keep it at SHOULD with the rest of lock ordering. Rewriting the whole set after one child's lock holds more than one lock in no consistent order, which `BE-LOCK-002`'s Statement already covers, so a rule of its own would restate that Statement and report one defect under two IDs, against 008's data model (`specs/008-security-rules/data-model.md`), §Verification phrasing item 5; the clause names the order for this case. Rejected: MUST, which a deadlock the database resolves does not meet; and a rule of its own, a second Statement for a case the first already holds.
