---
section: "Rule set scope"
---

# Url-pii-query-only-and-oauth-stricter-than-rfc-9700

## Context

`SEC-DATA-001` held that PII is barred from a URL's query, path and fragment, wider than its sources; `SEC-AUTHN-002`'s Source read as the origin of two MUSTs that RFC 9700 makes weaker. The owner decided on 2026-11-19 how each is stated.

## Behavior

- `SEC-DATA-001` bars a password, session identifier, access, refresh or ID token and API key from the query, path and fragment of a URL the project's code requests, sends or renders, and PII from its query only. A path segment or fragment holding PII is not flagged. The flag for the query stays (`?email=`).
- The opaque identifier an RFC 8058 one-click `List-Unsubscribe` URI carries MAY appear in that URI, which encodes the recipient and the list with no other carrier, worded as `FE-STORAGE-003` words it. The Verification excepts it.
- Its yields to `BE-API-013`, `BE-API-015` and `FE-STORAGE-003` name "any case" each reaches, so they hold as those rules narrow the same way.
- `SEC-AUTHN-002` keeps (a), `state` on every authorization request, and (b), PKCE for every authorization-code flow, confidential clients included. Its Source says they go beyond RFC 9700 on purpose: §2.1.1 makes PKCE RECOMMENDED for a confidential client and §2.1 lets PKCE or the OIDC `nonce` stand in for `state`.
- `BE-DEPS-006` and `BE-AUTHN-020`, never tagged, are removed from `security-backend.md` in 0.60.0, so `SEC-DEPS-001` and `SEC-AUTHN-003` have no deprecated predecessor to yield to. Only `BE-AUTHN-009` and `BE-AUTHN-013` stay deprecated, for removal in 0.62.0.

## Edge Cases

- A spec citing `BE-DEPS-006` or `BE-AUTHN-020` is Blocking as an unknown rule at once, with no Advisory window, unless its `retired-rules:` names the ID.
- PII in a fragment is no one's flag: CWE-598 speaks of the query string, and the OWASP REST cheat sheet names no PII.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
