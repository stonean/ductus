---
next-task: 16
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

## 14. Remove BE-AUTHN-009 and BE-AUTHN-013 at their 0.62.0 target (Resolved Question "Which version removes the deprecated IDs?")

- [x] Remove the `BE-AUTHN-009` and `BE-AUTHN-013` sections from `framework/rules/security-backend.md`, whose labels name removal in 0.62.0, the release that now follows. The owner chose 0.62.0 with this removal on 2026-11-22, the agent's recommendation, adopted by the operator. Regenerate `runtime/src/primitives/shipped_rule_ids.txt` (`BLESS_RULE_INDEX=1 cargo test --lib shipped_rule_index`)
- [x] List both IDs under `retired-rules:` in the `spec.md` of each spec whose directory records them, 008, 026, 067 and this one, per 067's scenario `a-record-cites-a-removed-rule`; annotate AC5 as superseded in part by the removal
- [x] `runtime/CHANGELOG.md`'s `[Unreleased]` names the removal as BREAKING: an adopter spec citing either ID now gets the Blocking unknown-rule finding, as this spec's Resolved Question states

- **Done when**: Neither rule is in a framework rule file, the release audit's Family 40 passes with `version` at 0.62.0, the whole `cargo test` passes, and 077's next review reads the change.

## 15. Disposition out-of-spec finding: 008's scenario says BE-AUTHN-009 and BE-AUTHN-013 remain deprecated

- [x] `specs/008-security-rules/scenarios/security-rules-flag-what-their-sources-support.md` — its Behavior bullet on `BE-AUTHN-020` and `BE-DEPS-006`'s removal says "Two 0.60.0 deprecations remain, `BE-AUTHN-009` and `BE-AUTHN-013`, which shipped in 0.59.11 and keep their labels and their 0.62.0 removal", and task 14 removes both — fixed: reworded in the past tense, which reopens 008 for a review and an analysis (the agent's recommendation, adopted by the operator, 2026-11-22, after 077's review found the earlier waiver's reason rested on the reopen's cost)

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task
