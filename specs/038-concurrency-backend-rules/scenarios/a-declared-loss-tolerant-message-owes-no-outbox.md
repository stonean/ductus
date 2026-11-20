---
section: "Rule set scope"
---

# A-declared-loss-tolerant-message-owes-no-outbox

## Context

`BE-COORD-003` (`framework/rules/concurrency-backend.md`, read at `afe503c8`) bars a dual write for an operation that "must both commit a database change and publish a message or event". Its Verification flags every plan that commits and then publishes, or publishes and then commits, as separate steps with no outbox, change-data capture or single source of truth, including a plan whose message or event the spec accepts losing (a best-effort metric, a cache-invalidation hint, a notification a later poll supersedes). For such a message nothing "must" be published, so the Statement does not cover it and the flag reaches past the Statement. The Statement has no term for this; the owner's decision (2026-11-19) names it "loss-tolerant".

## Behavior

- The Verification flags a commit and a publish made as separate steps only for a message or event the spec does not declare loss-tolerant. A spec that declares a message or event loss-tolerant owes no mechanism for it, and the first sentence's requirement to name one is scoped the same way.
- The Statement is unchanged: an operation that must both commit and publish, or write a second datastore that must stay consistent, makes the two atomic.
- The declaration is the spec's: a plan that merely omits the outbox does not make its message loss-tolerant, so it stays flagged.

## Edge Cases

- A write to a second datastore or search index that must stay consistent: not a message or event, so the declaration does not excuse it.
- A message the spec declares loss-tolerant on one path and requires on another: each is judged on its own declaration.
- A declared loss-tolerant message whose publish is also redelivered: `BE-COORD-002` and `BE-RETRY-005` own that, as they own it today.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
