---
section: "Two Rule Files"
---

# Security-rules-flag-what-their-sources-support

## Context

A pre-tag audit of the shipped rule files on 2026-11-19 found rules that flagged more than their sources support, left a legitimate design with no compliant form, or kept two never-released stubs. The owner decided each finding that way one at a time (owner, 2026-11-19). The checks read OWASP REST Security Cheat Sheet ("Passwords, security tokens, and API keys should not appear in the URL", no PII named) and CWE-598 (PII such as email addresses or phone numbers in "the query string"; no path), both fetched 2026-11-19.

## Behavior

`framework/rules/security-backend.md` and `framework/rules/security-frontend.md` state, and `/{project}:analyze` flags accordingly:

- **`BE-API-013`.** Credentials (passwords, session identifiers, access, refresh and ID tokens, API keys) stay barred from URL query parameters and path segments, in what the server accepts and in what it sends. PII is barred from a URL's query only. A path segment holding PII is not flagged, and the `?email=` flag stays. The opaque identifier an RFC 8058 one-click `List-Unsubscribe` URI carries MAY appear in that URI, worded as `FE-STORAGE-003` words it, with the matching "other than" clause in the Verification. Each "Not flagged" hand-off names only a case its receiver still judges.
- **`BE-AUTHN-022`.** An account with no local authenticator bound, which signs in only through an external identity provider, satisfies the re-authentication with a fresh sign-in round trip through that provider within the change flow. The Verification names it beside password, second factor and passkey. The rule names no OpenID Connect parameter, so none needed fetching.
- **`BE-API-015`.** (a) refuses a handshake whose `Origin` is absent unless the connection is authenticated by a credential the browser does not attach automatically, and the trailing MAY sentence is gone. The behavior is unchanged.
- **`BE-DATA-012`.** The Verification flags specs that omit the revocation path or name a restart as the revocation. A restart stays a valid way for a replacement to reach the running application.
- **`FE-STORAGE-006`.** Tokens and credentials stay a flat MUST NOT in the Cache API. Other user-specific authenticated content, PII included, MAY be cached when the spec states the cache is partitioned per user and purged at sign-out. The Verification flags a service worker that caches such content with either missing, and keeps the `Cache-Control: no-store` flag.
- **`FE-DEPS-008`.** The signature and provenance verification runs before the dependencies are built into a production artifact, not as part of installation, by the package manager's own mechanism where it has one (`npm audit signatures` stays the example). Where the package manager has none, the spec says so, and that statement is the documented exception.
- **`FE-DEPS-009`.** The Verification flags a missing (a) where private packages exist and a missing (b) always, with no dependency-churn condition. (c) requires the plan to name where a new dependency's name is checked (an allowlist, a lockfile-diff review step, or a tool), so an analysis can see it.
- **`BE-AUTHN-020` and `BE-DEPS-006` are removed.** Neither was in a tagged release. Their IDs are listed under `retired-rules:` in this spec's frontmatter, leave `runtime/src/primitives/shipped_rule_ids.txt`, and stay unreused. Two 0.60.0 deprecations remain, `BE-AUTHN-009` and `BE-AUTHN-013`, which shipped in 0.59.11 and keep their labels and their 0.62.0 removal. The successors are `SEC-AUTHN-003` and `SEC-DEPS-001`.
- **Left as written.** `BE-DATA-013` and `FE-XSS-010`: the audit's reading did not hold against the rule text.

## Edge Cases

- **`FE-STORAGE-003` keeps its PII bar.** Its Statement, flags and Source are unchanged; its Not-flagged hand-off to `BE-API-013` names PII in a query parameter only, so PII in a path stays `FE-STORAGE-003`'s. Its fragment and non-served-endpoint hand-offs from `BE-API-013` remain true, since it still bars PII there.
- **`SEC-DATA-001`.** Spec 077 narrows it the same way for `security-cross.md`, so a project loading both files is told one thing.
- **A project citing a removed ID.** An adopting project that cites `BE-AUTHN-020` or `BE-DEPS-006` gets an unknown-rule finding from `check-rule-ids`, unless its spec lists the ID under `retired-rules:`. The changelog names each successor.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
