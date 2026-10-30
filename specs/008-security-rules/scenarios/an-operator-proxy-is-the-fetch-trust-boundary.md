---
section: "Backend (`security-backend.md`)"
---

# An-operator-proxy-is-the-fetch-trust-boundary

## Context

`BE-INPUT-016` requires a server-side fetch to connect to the address that passed `BE-INPUT-007`'s screen rather than resolving the hostname again. Through a forward proxy the operator configured (`HTTPS_PROXY`, `ALL_PROXY` and their kin), the client sends the hostname in its `CONNECT` and the proxy resolves it, so no address the client screened is the one connected to, and the rule as written can be met only by refusing every proxied fetch. `fetch-archive` (022's plan, and 048's scenario `fetch-archive-reads-its-proxy-once`) screens the first URL and every redirect hop before any request, pins a direct connection to the screened addresses, and names the operator's proxy as the trust boundary for a proxied one; refusing proxied fetches would break every adopter whose network reaches the internet only through a proxy. Found by 048's analysis on 2026-10-29, which the owner routed here (the rule text is this spec's).

## Behavior

- `BE-INPUT-016` names an operator-configured forward proxy as the trust boundary for a fetch routed through it: the destination checks still apply to the submitted URL and to every redirect target before it is requested, and the connection pin applies to every connection the client itself resolves.
- The clause covers a proxy the operator configures for the process, not one the fetched URL or a redirect can select.
- 048's analysis no longer fires `BE-INPUT-016` against `fetch-archive`'s proxied case once the rule carries the clause.

## Edge Cases

- A host the proxy does not serve (named in `NO_PROXY`, or a scheme no configured proxy serves) is reached directly, so the pin applies to it.
- A proxy's own host is resolved by the client to connect to the proxy; whether that lookup is screened is the project's decision, and the rule's clause says which it requires.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
