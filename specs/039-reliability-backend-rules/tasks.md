---
next-task: 9
---

# 039 — Backend reliability rules Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 8. The owner's rule decision of 2026-11-19 for `framework/rules/reliability-backend.md`

- [ ] `BE-RETRY-005`: a message the spec declares expendable MAY be dropped once its redelivery bound is exhausted when the drop is recorded where the responsible team can see it; the Verification stops flagging that disposition and keeps flagging a discard the spec does not declare or does not record. The MUST for every other message is unchanged

- **Done when**: each item is stated in the rule file as decided, the spec records the requirement (scenario `expendable-messages-may-be-dropped-when-recorded`, and a Resolved Questions entry per decision with its reason) and corrects any criterion the change makes false, any external claim the new text makes is fetched and not recalled, bytes before and after are measured per rule and per file, `runtime/CHANGELOG.md` `[Unreleased]` gains an entry per changed rule, the whole `cargo test` and both artifact workflows' steps pass, and the spec's next review and analysis read the change
