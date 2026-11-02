---
section: "Backend (`security-backend.md`)"
---

# An-operator-proxy-is-the-fetch-trust-boundary

## Context

`BE-INPUT-016` requires a server-side fetch to connect only to an address that passed `BE-INPUT-007`'s screen, the one the screen resolved or one a connect-time lookup screens again. Through a forward proxy the operator configured (`HTTPS_PROXY`, `ALL_PROXY` and their kin), the client sends the hostname in its `CONNECT` and the proxy resolves it, so no address the client screened is the one connected to, and the rule's address clause alone can be met only by refusing every proxied fetch. `fetch-archive` (022's plan, and 048's scenario `fetch-archive-reads-its-proxy-once`) screens the first URL and every redirect hop before any request, pins the first host's direct connection to the screened addresses and screens a redirect host's again at connect, and names the operator's proxy as the trust boundary for a proxied one; refusing proxied fetches would break every adopter whose network reaches the internet only through a proxy. Found by 048's analysis on 2026-10-29, which the owner routed here (the rule text is this spec's).

## Behavior

- `BE-INPUT-016` names an operator-configured forward proxy as the trust boundary for a fetch routed through it: the destination checks still apply to the submitted URL and to every redirect target before it is requested, and every connection the client itself resolves, but its lookup of the proxy's own host, is pinned or screened at connect.
- The clause covers a proxy the operator configures for the process, not one the fetched URL or a redirect names; the URL's scheme or `NO_PROXY` choosing among the operator's proxies is not such a choice.
- 048's analysis no longer fires `BE-INPUT-016` against `fetch-archive`'s proxied case once the rule carries the clause.

## Edge Cases

- A host the proxy does not serve (named in `NO_PROXY`, or a scheme no configured proxy serves) is reached directly, so its connection is pinned or screened at connect, the proxy's own host aside, whose lookup is exempt by name.
- A proxy's own host is resolved by the client to connect to the proxy, and that lookup is exempt from the destination checks, since the operator chose the host; a submitted URL or redirect target naming it is still screened before it is requested.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Is the client's lookup of an operator-configured proxy's own host subject to the destination checks and the pin?** → No: the clause exempts it, since the operator, not a URL or a redirect, chose the proxy, and screening it would refuse the usual proxy on a private address while binding nothing the proxy connects to, which RFC 9110 §9.3.6 has the proxy establish "to the destination origin server identified by the request target" (fetched 2026-11-02). `BE-INPUT-007` names the exemption as `BE-INPUT-016`'s. This matches 048's scenario `fetch-archive-reads-its-proxy-once`, whose second Resolved Question rejects screening the proxy's address, so `fetch-archive` needs no change (owner, 2026-11-02, option a).
