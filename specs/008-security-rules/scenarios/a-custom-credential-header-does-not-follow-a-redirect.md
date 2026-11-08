---
section: "Backend (`security-backend.md`)"
---

# A-custom-credential-header-does-not-follow-a-redirect

## Context

No rule in `security-backend.md` (read 2026-10-25 at HEAD f24c55c6) covers a credential an outbound HTTP client sends in a header of its own — an API-token header rather than `Authorization`. HTTP clients strip a fixed set of sensitive headers when a redirect changes host, and a custom credential header is not in that set: Go's `net/http` client forwards a header of the application's own on a redirect to any host, and keeps `Authorization` and cookies on a redirect to the same hostname or a subdomain whatever the scheme or port (its documentation and source, as `BE-DATA-013` cites them). A server the client trusts, or anyone able to make it answer with a redirect, can then receive the credential.

## Behavior

- `security-backend.md` carries a rule, at the MUST tier, that an outbound HTTP client sending a credential in a request header — `Authorization`, a cookie, or a header of the application's own — or in the request body refuses to follow redirects, or follows only a redirect to the same origin, the credential reaching it by the client's own decision.
- The rule's Verification names what a reviewer checks in the client's construction, phrased per 008's Verification phrasing.
- Every claim the rule makes about library behavior is fetched from the library's documentation or source at authoring time, never recalled, and the rule's Source cites what was fetched.

## Edge Cases

- A credential in `Authorization` only: covered too, since the rule is scoped by origin and a library's stripping on a host change leaves a same-host redirect to another scheme or port (Resolved Questions).
- A client library that never sends the header on a redirect to a different origin: the rule's trigger still fires, and the spec relies on the library for a header credential only by showing it, citing the library's documentation or source; never for a body credential.
- A redirect the caller follows by hand: the same requirement applies to the caller's loop.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Does the rule cover `Authorization` on a same-host redirect to another scheme or port?** Yes (owner, 2026-10-29): the rule is scoped by origin — scheme, host and port — for every credential header, `Authorization` included, since Go's `net/http` compares hostnames only and so keeps `Authorization` on a same-host `https`-to-`http` redirect. The rule is `BE-DATA-013`.
