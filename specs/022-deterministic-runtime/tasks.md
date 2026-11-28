---
next-task: 66
---

# 022 — Deterministic Runtime Tasks

Tasks derived from the [plan](plan.md). Complete in order. Each task is small enough to complete and verify in a single session; later tasks depend on earlier ones.

## 65. registerService is an extension point ductus exec serves

- [x] Implement the behavior described in `scenarios/link-registration-is-an-extension-point.md`

- **Done when**: `registerService` is in spec.md's §LLM extension points and the data model with its request and response schemas, `runtime/src/schema/extensions.rs` types and validates them, `build_extension_request` in `runtime/src/interpreter/payload.rs` builds its request, the scenario's open question is resolved, and a test walks `ductus exec link` to its end on a fixture with a canned response and holds the block the walk wrote, each as `scenarios/link-registration-is-an-extension-point.md` states.

This task waits on spec 030's task adding the primitive that performs `link.md`'s **Additive write**, which the walk calls (the scenario's resolved question).

The review after this task runs with `--since` at the commit `chore(022): taiki-e/install-action pinned at v2.90.11`, the base of the review committed as `review(022): the window since its last review; …`, which read the window from `f62c003f` to there. Its record holds open the finding this task closes, and lists entries unread, so the default window would start at `f62c003f` again. The entries it left unread still join that review's scope (the agent's recommendation, adopted by the operator, 2026-11-28).
