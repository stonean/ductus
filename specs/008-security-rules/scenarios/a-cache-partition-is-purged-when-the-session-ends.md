---
section: "Two Rule Files"
---

# A-cache-partition-is-purged-when-the-session-ends

## Context

`FE-STORAGE-006` lets service-worker code store user-specific authenticated content, PII included, in the Cache API when the spec states the cache is partitioned per user and purged at sign-out (this spec's Resolved Question 17, owner, 2026-11-19). A session that expires or is revoked without a sign-out, on a shared device a user walked away from, leaves that partition on the device until the next sign-out there, which may never come. `FE-AUTHN-002` already requires the application to act when it learns the session expired or was revoked, by a re-authentication prompt or a redirect to the login page, so the purge has a moment the application already handles. This spec's review of 2026-11-20 raised the gap (the agent's recommendation, adopted by the operator, 2026-11-23).

## Behavior

`framework/rules/security-frontend.md` states, and `/{project}:analyze` flags accordingly:

- **`FE-STORAGE-006`.** Responses that carry user-specific authenticated content other than tokens and credentials MAY be stored only when the spec states the cache is partitioned per user and purged at sign-out and when the application learns the session expired or was revoked. The Verification flags a service-worker spec that caches such content without the partition, the sign-out purge or the expiry-and-revocation purge. Tokens and credentials stay a flat MUST NOT.

## Edge Cases

- **An application that is offline cannot learn of an expiry or a revocation,** so its partition is purged when it next learns of one, the moment `FE-AUTHN-002`'s prompt or redirect handles, and not before.
- **`FE-PII-004`'s `Clear-Site-Data` on the logout response stays a SHOULD** and covers sign-out alone; the expiry-and-revocation purge is `FE-STORAGE-006`'s.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
