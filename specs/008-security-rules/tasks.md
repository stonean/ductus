---
title: "008-security-rules — tasks"
next-task: 51
---

# 008 — Security Rules Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 45. The owner's rule decisions of 2026-11-19 for `framework/rules/security-backend.md` and `framework/rules/security-frontend.md`

- [x] `BE-API-013`: PII is barred from URL query parameters only, as CWE-598 has it (fetched 2026-11-19: it lists PII such as email addresses in a query string and says nothing of the path); credentials stay barred from query and path. PII leaves the path-segment clause in the Statement, the Verification's flags and the server-sends-a-URL clauses, and each ownership clause stays coherent with `SEC-DATA-001` (spec 077 makes the same change) and `FE-STORAGE-003`
- [x] `BE-API-013`: the opaque identifier an RFC 8058 one-click `List-Unsubscribe` URI carries MAY appear in that URI, worded as `FE-STORAGE-003` has it, with the matching clause in the Verification, since a backend-only project was flagged for a link a frontend project is told is allowed
- [x] `BE-AUTHN-022`: for an account with no local authenticator bound, a fresh sign-in round trip through its external identity provider within the change flow satisfies the first sentence's re-authentication, and the Verification names it; the rule gave a federated-only account no compliant design. Any OpenID Connect parameter named is fetched from OpenID Connect Core 1.0 first
- [x] `BE-API-015`: (a) refuses a handshake whose `Origin` is absent unless the connection is authenticated by a credential the browser does not attach automatically, and the trailing MAY sentence goes; same behavior, one statement
- [x] `BE-DATA-012`: the Verification flags specs that omit the revocation path or name a restart as the revocation; a restart stays a valid way for a replacement to reach the running application
- [x] `BE-AUTHN-020` and `BE-DEPS-006` are removed now: neither was in a tagged release (added on `main` 2026-10-19, deprecated 2026-11-02 for `SEC-AUTHN-003` and `SEC-DEPS-001`). Both rule sections go, their IDs leave `runtime/src/primitives/shipped_rule_ids.txt` and every list a test holds to the files, both IDs are recorded under `retired-rules:` in this spec's frontmatter, and every statement counting four 0.60.0 deprecations is corrected to two (`BE-AUTHN-009` and `BE-AUTHN-013`, which shipped in 0.59.11 and keep their 0.62.0 removal). The changelog entry names each successor. `specs/017-derive-dont-ask/spec.md` and spec 077's artifacts mention the dropped stubs: 017's and 077's own tasks correct those
- [x] `FE-STORAGE-006`: tokens and credentials stay a flat MUST NOT in the Cache API; other user-specific authenticated content, PII included, MAY be cached when the spec states it is partitioned per user and purged at sign-out, and the Verification flags a cache with either missing. An offline-capable app had no compliant design
- [x] `FE-DEPS-008`: the verification runs before the dependencies are built into a production artifact, not as part of installation, by the package manager's own mechanism where it has one (`npm audit signatures` stays the example); where it has none the spec says so, and that statement is the documented exception
- [x] `FE-DEPS-009`: the Verification flags a missing (a) where private packages exist and a missing (b) always, with no dependency-churn condition, and (c) requires the plan to name where a new dependency's name is checked (an allowlist, a lockfile-diff review step, or a tool), so an analysis can see it
- [x] Left as they are, on reading the rule text against the audit's finding: `BE-DATA-013` and `FE-XSS-010`

- **Done when**: each item is stated in the rule file as decided, the spec records the requirement (scenario `security-rules-flag-what-their-sources-support`, and a Resolved Questions entry per decision with its reason) and corrects any criterion the change makes false, any external claim the new text makes is fetched and not recalled, bytes before and after are measured per rule and per file, `runtime/CHANGELOG.md` `[Unreleased]` gains an entry per changed rule, the whole `cargo test` and both artifact workflows' steps pass, and the spec's next review and analysis read the change

## 46. `BE-DATA-012` sets aside a throwaway local or test credential (the owner's decision of 2026-11-23)

- [ ] Implement the behavior described in `scenarios/a-throwaway-credential-owes-no-revocation-path.md`
- [ ] `BE-DATA-012`'s Statement and Verification set aside a credential for a throwaway local or test resource that grants no access to a production or shared system, worded as `CFG-ENV-013` words it; `BE-DATA-003` and `BE-DATA-008` are unchanged
- [ ] This spec's Resolved Questions record the decision with its reason, and why `BE-DATA-003` and `BE-DATA-008` keep the word
- [ ] `runtime/CHANGELOG.md` gains an entry for the changed rule under `## [Unreleased]`

- **Done when**: the rule file states what the scenario does, its bytes before and after are measured for the rule and the file, the spec records the decision in Resolved Questions, the changelog entry sits under `## [Unreleased]`, the whole `cargo test` and both artifact workflows' steps pass, and the spec's next review and analysis read the change

## 47. `BE-DEPS-005`'s Verification flags what its Statement requires (the owner's decision of 2026-11-23)

- [ ] Implement the behavior described in `scenarios/be-deps-005-flags-what-its-statement-requires.md`
- [ ] `BE-DEPS-005`'s Verification flags a missing (a) where private packages exist, a missing (b) and a plan naming no place for (c)'s check, with no dependency-churn condition, and (c) requires the plan to name that place; `BE-DEPS-004` is unchanged
- [ ] This spec's Resolved Questions record the decision with its reason
- [ ] `runtime/CHANGELOG.md` gains an entry for the changed rule under `## [Unreleased]`

- **Done when**: the rule file states what the scenario does, its bytes before and after are measured for the rule and the file, the spec records the decision in Resolved Questions, the changelog entry sits under `## [Unreleased]`, the whole `cargo test` and both artifact workflows' steps pass, and the spec's next review and analysis read the change

## 48. `FE-STORAGE-006` purges the per-user cache when the session ends (the owner's decision of 2026-11-23)

- [ ] Implement the behavior described in `scenarios/a-cache-partition-is-purged-when-the-session-ends.md`
- [ ] `FE-STORAGE-006`'s MAY requires the per-user partition purged at sign-out and when the application learns the session expired or was revoked, and its Verification flags a cache missing the partition or either purge; tokens and credentials stay a flat MUST NOT
- [ ] This spec's Resolved Questions record the decision with its reason
- [ ] `runtime/CHANGELOG.md` gains an entry for the changed rule under `## [Unreleased]`, naming the narrowing

- **Done when**: the rule file states what the scenario does, its bytes before and after are measured for the rule and the file, the spec records the decision in Resolved Questions, the changelog entry sits under `## [Unreleased]`, the whole `cargo test` and both artifact workflows' steps pass, and the spec's next review and analysis read the change

## 49. `BE-API-015`'s Verification flags an absent `Origin` on a cookie-authenticated connection (the owner's decision of 2026-11-23)

- [ ] Implement the behavior described in `scenarios/an-absent-origin-on-a-cookie-connection-is-flagged.md`
- [ ] `BE-API-015`'s Verification flags an endpoint that accepts a handshake with no `Origin` while the connection is authenticated by a credential the browser attaches automatically; the Statement is unchanged
- [ ] This spec's Resolved Questions record the decision with its reason
- [ ] `runtime/CHANGELOG.md` gains an entry for the changed rule under `## [Unreleased]`

- **Done when**: the rule file states what the scenario does, its bytes before and after are measured for the rule and the file, the spec records the decision in Resolved Questions, the changelog entry sits under `## [Unreleased]`, the whole `cargo test` and both artifact workflows' steps pass, and the spec's next review and analysis read the change

## 50. `scripts/check-release-tag.sh` reads every entry above the newest version heading

- [ ] Implement the behavior described in `scenarios/the-tag-check-reads-every-entry-above-the-newest-version.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.
