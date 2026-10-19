---
section: "Follow-on scenarios"
---

# Fetch-archive-dns-rebinding

## Context

`fetch-archive`'s SSRF screen is a resolve-then-block check: `validate_fetch_url` resolves the request host and rejects any resolution that lands on an internal / non-public address, then hands the *original URL* to `reqwest`, which re-resolves the host independently at connect time. Because the two resolutions are separate DNS lookups — a TOCTOU window — a host that resolves to a public IP during validation and to an internal IP at connect time slips past the screen: classic DNS rebinding.

When this scenario was surfaced (2026-07-11, during the 0.18.0 fix-verification pass), the window was inherent to the resolve-then-block design BE-INPUT-007 prescribes, and BE-INPUT-007's Verification accepted resolution-time blocks, so the behavior was not a review violation and the hardening was low priority. Since 2026-10-19, `BE-INPUT-016` makes the hardened form a MUST for every server-side fetch: the client connects to the address that passed validation rather than resolving the host a second time. This scenario captures that form.

## Behavior

- `fetch-archive` connects to the exact address it validated: the address resolved and screened by `validate_fetch_url` is the address `reqwest` connects to, so no second, unscreened DNS lookup occurs between validation and connect. The mechanism is either a pinned `SocketAddr` handed to the connector (e.g. a `resolve`/`resolve_to_addrs` override or a custom `dns::Resolve`) or an equivalent connect-time re-check of the resolved address against the same internal-address predicate.
- When the address the connection would use fails the internal-address screen, the fetch is an operational error naming the rejected address — never a silent connect to an internal host.

## Edge Cases

- A host that resolves to multiple addresses is screened per address; the connection uses only a screened address.
- A legitimate public host whose DNS changes between separate fetches still succeeds — the address is pinned within a single fetch (resolved once and reused), not cached across fetches.
- CLI invocation of `fetch-archive` is screened identically; the hardening lives in the shared fetch path, not the MCP seam.
- **A redirect hop is pinned too.** Each redirect target is re-screened by `validate_fetch_url` (`redirect_verdict`), but the connection to a redirect host is not pinned: `resolve_to_addrs` covers only the initial host, so `reqwest` resolves a redirect host again at connect time and the rebinding window this scenario closes stays open on every hop. `BE-INPUT-016` requires the connection to use the validated address on every hop — a resolver that screens each lookup (a custom `dns::Resolve`) closes it for the initial host and every redirect alike. Found 2026-10-19 by the rule-file review.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
