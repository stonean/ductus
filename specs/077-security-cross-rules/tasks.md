---
next-task: 14
---

# 077 — Security cross rules Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 13. The owner's rule decisions of 2026-11-19 for `framework/rules/security-cross.md`

- [x] `SEC-DATA-001`: PII is barred from a URL's query only, as CWE-598 has it; credentials stay barred from query, path and fragment. Spec 008 makes the same change to `BE-API-013`, and each ownership clause stays coherent with it and with `FE-STORAGE-003`
- [x] `SEC-DATA-001`: the opaque identifier an RFC 8058 one-click `List-Unsubscribe` URI carries MAY appear in that URI, worded as `FE-STORAGE-003` has it, with the matching clause in the Verification
- [x] `SEC-AUTHN-002`: (a) and (b) stay as they are, and one clause says they go beyond RFC 9700 on purpose: RFC 9700 §2.1.1 and §2.1 (fetched 2026-11-19) make PKCE RECOMMENDED for confidential clients, let a confidential OpenID Connect client use `nonce` instead, and let PKCE or `nonce` stand in for `state`
- [x] This spec's own statements about `BE-AUTHN-020` and `BE-DEPS-006`, which spec 008 removes now (never in a tagged release), are corrected so they stay true; the successors `SEC-AUTHN-003` and `SEC-DEPS-001` are unchanged

- Bytes measured from git, per `###` section of `framework/rules/security-cross.md` (heading to the next `##` or `###` heading), `fa4ae5c7` to `51fe0d34`: file 23,779 to 24,283 (+504); `SEC-DATA-001` 3,993 to 4,326 (+333); `SEC-AUTHN-002` 5,109 to 5,280 (+171)

- **Done when**: each item is stated in the rule file as decided, the spec records the requirement (scenario `url-pii-query-only-and-oauth-stricter-than-rfc-9700`, and a Resolved Questions entry per decision with its reason) and corrects any criterion the change makes false, any external claim the new text makes is fetched and not recalled, bytes before and after are measured per rule and per file, `runtime/CHANGELOG.md` `[Unreleased]` gains an entry per changed rule, the whole `cargo test` and both artifact workflows' steps pass, and the spec's next review and analysis read the change
