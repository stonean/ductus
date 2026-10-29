---
title: "008-security-rules — research"
---

# 008 — Security Rules Research

## Sources

The v1 security rules were derived from the following authoritative sources; rows added by the rule-file reviews of 2026-10-19 are marked as such. Each rule names the sources it rests on in its own `**Source:**` field, which is canonical for that rule and may cite documents this page does not list.

### OWASP Cheat Sheet Series

Primary source for actionable security rules. Each cheat sheet is maintained by the OWASP community and represents industry consensus.

| Cheat Sheet | URL | Categories Informed |
| --- | --- | --- |
| Input Validation | cheatsheetseries.owasp.org/cheatsheets/Input_Validation_Cheat_Sheet | BE-INPUT |
| Authentication | cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet | BE-AUTHN |
| Session Management | cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet | BE-AUTHN, FE-STORAGE |
| Authorization | cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet | BE-AUTHZ |
| Password Storage | cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet | BE-AUTHN |
| Cryptographic Storage | cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet | BE-DATA |
| REST Security | cheatsheetseries.owasp.org/cheatsheets/REST_Security_Cheat_Sheet | BE-API |
| SQL Injection Prevention | cheatsheetseries.owasp.org/cheatsheets/SQL_Injection_Prevention_Cheat_Sheet | BE-INPUT |
| Logging | cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet | BE-LOG |
| Secrets Management | cheatsheetseries.owasp.org/cheatsheets/Secrets_Management_Cheat_Sheet | BE-DATA |
| Error Handling | cheatsheetseries.owasp.org/cheatsheets/Error_Handling_Cheat_Sheet | BE-ERR |
| File Upload | cheatsheetseries.owasp.org/cheatsheets/File_Upload_Cheat_Sheet | BE-INPUT |
| HTTP Headers | cheatsheetseries.owasp.org/cheatsheets/HTTP_Headers_Cheat_Sheet | BE-API, FE-CSP |
| XSS Prevention | cheatsheetseries.owasp.org/cheatsheets/Cross_Site_Scripting_Prevention_Cheat_Sheet | FE-XSS |
| CSRF Prevention | cheatsheetseries.owasp.org/cheatsheets/Cross-Site_Request_Forgery_Prevention_Cheat_Sheet | FE-CSRF |
| CSP | cheatsheetseries.owasp.org/cheatsheets/Content_Security_Policy_Cheat_Sheet | FE-CSP |
| Database Security (added 2026-10-19) | cheatsheetseries.owasp.org/cheatsheets/Database_Security_Cheat_Sheet | BE-DATA, BE-API |
| Multifactor Authentication (added 2026-10-19) | cheatsheetseries.owasp.org/cheatsheets/Multifactor_Authentication_Cheat_Sheet | BE-AUTHN |
| WebSocket Security (added 2026-10-19) | cheatsheetseries.owasp.org/cheatsheets/WebSocket_Security_Cheat_Sheet | BE-API |

### Standards and Specifications

| Standard | Relevance |
| --- | --- |
| OWASP Top 10 (2021; the rules now cite the 2025 edition) | Threat categorization and priority ordering |
| OWASP API Security Top 10 (2023) | API-specific threat categorization |
| RFC 2119 | Requirement level keywords (MUST, SHOULD, etc.) |
| RFC 9457 | Problem Details for HTTP APIs (error response format; obsoletes RFC 7807, which this table named until 2026-09-13 — `BE-ERR-004` cites 9457) |
| NIST SP 800-63B-4 | Digital Identity Guidelines — password and authenticator requirements |
| W3C Fetch Metadata Request Headers (added 2026-10-19) | The `Sec-Fetch-Site` request header behind the Fetch Metadata CSRF defense (`BE-API-011`, `FE-CSRF-001`) |
| XS-Leaks Wiki (added 2026-10-19) | Cross-site leak attacks and their defenses: CSS-selector exfiltration through an injected style (`FE-CSP-002`), and the leaks through window references that `Cross-Origin-Opener-Policy` prevents (`FE-CSP-009`) |
| Topics API explainer (added 2026-10-19) | `interest-cohort=()`, FLoC's `Permissions-Policy` opt-out, also forbids Topics calculation — the privacy opt-out `FE-CSP-007` no longer requires among its powerful-feature denials. The explainer's repository is archived and the API deprecated. |
| MDN browser compatibility data (`@mdn/browser-compat-data`, added 2026-10-19) | Per-browser support behind `FE-CSRF-001` and `FE-CSRF-002` (`SameSite` Lax-by-default; `SameSite=None` requiring `Secure`), `FE-XSS-009` (Trusted Types), and `FE-CSP-007` (`Permissions-Policy`). No security rule cites it since 2026-10-28, when `security-frontend.md` dropped its per-browser support claims. |
| RFC 6455 (added 2026-10-19) | The WebSocket Protocol: browsers send `Origin` on the opening handshake, and a server meant only for certain sites should verify it and refuse an unacceptable one with `403` (`BE-API-015`) |
| CWE-1385 (added 2026-10-19) | Missing Origin Validation in WebSockets: WebSocket connections are not restricted by the same-origin policy or CORS (`BE-API-015`) |
| RFC 9110 §5.3, §8.6 and RFC 9112 §6.3 (added 2026-10-19) | Repeated header lines combine into one comma-separated value; a `Content-Length` list of identical values may be processed as that single value, and differing values are a framing error (`BE-INPUT-013`) |
| PEP 740, PyPI attestation documentation, pip documentation (added 2026-10-19) | pip does not verify PEP 740 attestations on install, and its hash-checking mode (`--require-hashes`) is the integrity check it offers — the installer-gap exception in `BE-DEPS-004` |

## Notes

- Rules are generalized from the sources above — they do not prescribe specific libraries, languages, or frameworks
- The OWASP Cheat Sheet Series provided the most directly actionable content; most rules map to specific cheat sheet recommendations
- CIS Benchmarks informed the v1 infrastructure-level rules (management port exposure, database access) without being reproduced directly, and the Standards table listed them until 2026-10-19. No rule cites them any more: that review replaced the CIS citations on `BE-DATA-007` ("CIS Database Benchmarks") and on `BE-API-007` and `BE-API-008` ("CIS Benchmarks") with the OWASP Database Security Cheat Sheet, and `BE-API-007` also cites Redis's own security documentation
