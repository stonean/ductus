---
section: "Backend (`security-backend.md`)"
---

# A-custom-credential-header-does-not-follow-a-redirect

## Context

No rule in `security-backend.md` (read 2026-10-25 at HEAD f24c55c6) covers a credential an outbound HTTP client sends in a header of its own — an API-token header rather than `Authorization`. HTTP clients strip a fixed set of sensitive headers when a redirect changes host, and a custom credential header is not in that set: as reported, Go's `net/http` resends custom headers on a 307 or 308 to any host and strips only `Authorization` and cookies when the host changes. A server the client trusts, or anyone able to make it answer with a redirect, can then receive the credential.

## Behavior

- `security-backend.md` carries a rule, at the MUST tier, that an outbound HTTP client sending a credential in a header its HTTP library does not treat as sensitive refuses to follow redirects, or follows one only to the same origin with the header re-attached by the caller's own decision.
- The rule's Verification names what a reviewer checks in the client's construction, phrased per 008's Verification phrasing.
- Every claim the rule makes about library behavior is fetched from the library's documentation or source at authoring time, never recalled, and the rule's Source cites what was fetched.

## Edge Cases

- A credential in `Authorization` only: covered by the library's own stripping on a host change; whether a same-host redirect to another scheme or port needs the rule too is part of the authoring.
- A client library that strips every non-safelisted header on a cross-host redirect: the rule's trigger does not fire.
- A redirect the caller follows by hand: the same requirement applies to the caller's loop.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
