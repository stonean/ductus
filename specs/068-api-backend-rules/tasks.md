---
next-task: 9
---

# 068 — Backend API contract rules Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 8. The owner's rule decisions of 2026-11-19 for `framework/rules/api-backend.md`

- [ ] `BE-REPR-003`: the MUST stays, one clause says it goes beyond RFC 7493 §2.2 on purpose (fetched 2026-11-19: SHOULD NOT and RECOMMENDED, no MUST NOT), and the Verification flags only a field the spec does not bound within −(2^53 − 1) to 2^53 − 1
- [ ] `BE-PATCH-001`: the project format may be documented in `specs/system.md` or in the spec that introduces the endpoint, as long as it defines how an absent field and an explicit `null` each apply, which is what the Verification already passes
- [ ] Left as it is: `BE-COND-001` stays a MUST as written

- **Done when**: each item is stated in the rule file as decided, the spec records the requirement (a scenario, and a Resolved Questions entry per decision with its reason) and corrects any criterion the change makes false, any external claim the new text makes is fetched and not recalled, bytes before and after are measured per rule and per file, `runtime/CHANGELOG.md` `[Unreleased]` gains an entry per changed rule, the whole `cargo test` and both artifact workflows' steps pass, and the spec's next review and analysis read the change
