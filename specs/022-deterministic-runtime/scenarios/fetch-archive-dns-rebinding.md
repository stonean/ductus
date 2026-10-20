---
section: "Follow-on scenarios"
---

# Fetch-archive-dns-rebinding

## Context

`fetch-archive`'s SSRF screen is a resolve-then-block check: `validate_fetch_url` resolves the request host and rejects any resolution that lands on an internal / non-public address, then hands the *original URL* to `reqwest`, which re-resolves the host independently at connect time. Because the two resolutions are separate DNS lookups — a TOCTOU window — a host that resolves to a public IP during validation and to an internal IP at connect time slips past the screen: classic DNS rebinding.

When this scenario was surfaced (2026-07-11, during the 0.18.0 fix-verification pass), the window was inherent to the resolve-then-block design BE-INPUT-007 prescribes, and BE-INPUT-007's Verification accepted resolution-time blocks, so the behavior was not a review violation and the hardening was low priority. Since 2026-10-19, `BE-INPUT-016` makes the hardened form a MUST for every fetch subject to `BE-INPUT-007` — and `fetch-archive` screens every URL it fetches under `BE-INPUT-007`, because the URL arrives as a tool argument the runtime does not choose: the client connects to the address that passed validation rather than resolving the host a second time. This scenario captures that form.

## Behavior

- `fetch-archive` connects to the exact address it validated: the address resolved and screened by `validate_fetch_url` is the address `reqwest` connects to, so no second, unscreened DNS lookup occurs between validation and connect. The mechanism is either a pinned `SocketAddr` handed to the connector or an equivalent connect-time re-check of the resolved address against the same internal-address predicate. A custom `dns::Resolve` that screens every lookup does this for every host the fetch reaches; a `resolve`/`resolve_to_addrs` override pins only the hosts named when the client is built, so on its own it leaves a redirect host unpinned (see Edge Cases).
- When the address the connection would use fails the internal-address screen, the fetch is an operational error naming the rejected address — never a silent connect to an internal host.

## Edge Cases

- A host that resolves to multiple addresses is screened per address; the connection uses only a screened address.
- A legitimate public host whose DNS changes between separate fetches still succeeds — the address is pinned within a single fetch (resolved once and reused), not cached across fetches.
- CLI invocation of `fetch-archive` is screened identically; the hardening lives in the shared fetch path, not the MCP seam.
- **A redirect hop is pinned too.** Each redirect target is re-screened by `validate_fetch_url` (`redirect_verdict`), and the connection to it is screened as well: every connect-time lookup goes through a custom `dns::Resolve` that applies the same internal-address screen, so a redirect host that answers the screen with a public address and the connect with an internal one is refused, as `BE-INPUT-016` requires on every hop. The initial host stays pinned with `resolve_to_addrs`, which overrides the resolver for that host alone. Until 2026-10-20 only the initial host was pinned, and `reqwest` resolved a redirect host again, unscreened, at connect time; the 2026-10-19 rule-file review found the gap.
- **A proxy's own host is resolved but never refused.** Through a proxy the operator's proxy is the trust boundary, and it commonly sits on a private network; the screening resolver resolves the proxy's host without the internal-address screen, as it does a host named in `DUCTUS_FETCH_ALLOW_INSECURE_HOSTS`.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
