---
section: "Follow-on scenarios"
---

# Independent-review-findings-await-the-owner

## Context

On 2026-10-19 an independent reviewer re-checked, against fetched sources, the security rules changed after the second rule-file review pass. Every wording, fact, and citation fix was applied (commit 7577068c). Six findings would change what a rule requires, so they were left for the owner and are recorded here as open questions, each with the reviewer's evidence and a recommendation. Ask them one at a time (constitution §recommendations).

## Behavior

- Each open question below is answered by the owner, the rule changes (or stays) accordingly, and the answer moves to Resolved Questions with its date.

## Edge Cases

- A finding the owner declines leaves its rule as it is; the declined option is recorded so it is not re-raised.

## Open Questions

- BE-AUTHN-001 parameter minimums: its Verification names only OWASP's headline settings (Argon2id 19 MiB/t=2/p=1; scrypt N=2^17), but the OWASP Password Storage Cheat Sheet lists several equivalent configurations (e.g. Argon2id 46 MiB/t=1) that analyze would wrongly flag. (a, recommended) accept any configuration OWASP lists as equivalent; or (b) keep only the headline settings?
- BE-AUTHN-001 PBKDF2 condition: the Statement allows PBKDF2 "where FIPS compliance is required" but the Verification never checks that condition. (a, recommended) flag PBKDF2 chosen without a stated FIPS requirement; or (b) make PBKDF2 an unconditional option and drop the condition?
- BE-AUTHN-022 MFA changes: the current password alone can authorize removing or replacing a second factor, while the OWASP Multifactor Authentication Cheat Sheet asks for an existing enrolled factor. (a, recommended) require an existing enrolled factor or passkey for MFA changes; or (b) keep the current password as sufficient?
- BE-API-015 vs BE-API-013 on a token in the WebSocket handshake URL: BE-API-013 allows single-use short-lived values in URLs; BE-API-015's Verification flags any token in the handshake URL, following the OWASP WebSocket Security Cheat Sheet ("Avoid tokens in URL query strings"). (a, recommended) keep BE-API-015 stricter and say explicitly it narrows BE-API-013 for WebSockets; or (b) allow BE-API-013's single-use ticket in the handshake URL?
- BE-API-011 / FE-CSRF-001 fallback when Sec-Fetch-Site is absent: the rules allow rejection or an Origin check; OWASP's fail-open option also lists CSRF tokens or additional validation. (a, recommended) keep the Origin-only fallback (a design using tokens already complies through the token defense); or (b) add tokens and additional validation as fallbacks?
- SameSite as the sole CSRF defense (BE-API-011, FE-CSRF-001): the rules ban it; OWASP allows SameSite alone in narrow deployments meeting listed conditions, and the BE rationale already calls the ban deliberate. (a, recommended) keep the ban; or (b) allow OWASP's narrow conditions?

## Resolved Questions

*None yet.*
