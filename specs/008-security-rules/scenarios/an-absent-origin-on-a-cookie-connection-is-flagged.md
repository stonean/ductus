---
section: "Two Rule Files"
---

# An-absent-origin-on-a-cookie-connection-is-flagged

## Context

`BE-API-015`'s Statement (a) refuses a WebSocket opening handshake whose `Origin` is absent unless the connection is authenticated by a credential the browser does not attach automatically (this spec's Resolved Questions 10 and 14), yet its Verification names no flag for that case, so `/{project}:analyze` passes a spec whose endpoint accepts a handshake with no `Origin` on a cookie-authenticated connection. This spec's review of 2026-11-20 raised the gap, which predates 0.60.0. A Verification that flags what its Statement requires is the principle Resolved Question 19 applied to `FE-DEPS-009` (the agent's recommendation, adopted by the operator, 2026-11-23).

## Behavior

`framework/rules/security-backend.md` states, and `/{project}:analyze` flags accordingly:

- **`BE-API-015`.** The Verification flags such an endpoint that accepts a handshake with no `Origin` while the connection is authenticated by a credential the browser attaches automatically, such as a cookie. The Statement is unchanged, so a spec that meets it draws no new finding.

## Edge Cases

- **A connection authenticated by a credential the browser does not attach automatically** (a token in the first message, say) may still accept a handshake with no `Origin`, as (a) allows, and is not flagged.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
