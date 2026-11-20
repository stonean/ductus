---
next-task: 9
---

# 038 — Backend concurrency rules Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 8. The owner's rule decision of 2026-11-19 for `framework/rules/concurrency-backend.md`

- [ ] `BE-COORD-003`: the Verification flags a commit and a publish made as separate steps only for a message or event the spec does not declare loss-tolerant, matching the Statement's operation that must both commit and publish. The Statement is unchanged

- **Done when**: each item is stated in the rule file as decided, the spec records the requirement (a scenario, and a Resolved Questions entry per decision with its reason) and corrects any criterion the change makes false, any external claim the new text makes is fetched and not recalled, bytes before and after are measured per rule and per file, `runtime/CHANGELOG.md` `[Unreleased]` gains an entry per changed rule, the whole `cargo test` and both artifact workflows' steps pass, and the spec's next review and analysis read the change
