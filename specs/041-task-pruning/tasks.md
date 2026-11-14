---
next-task: 25
---

# 041 — Spec Directory Pruning Tasks

Tasks derived from the [plan](plan.md). Complete in order. Phase A ships the
`prune-tasks` runtime primitive; Phase B adds the `/{project}:prune` command on
top and registers it. The primitive (Phase A) must build and pass its parity
test before the command (Phase B) wires to it.
