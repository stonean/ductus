---
next-task: 14
---

# 049 — Rename govern to ductus Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Phase A lands the runtime first: it must read both the old and the new per-project directory
before this repo's own `.govern/` moves, or the binary loses its config mid-rename. Phase B
sweeps the framework, Phase C the specs, Phase D this repo's own dogfooded state, and Phase E
verifies the whole against the audit families. The crates.io half of AC12 lands with the
release, not here.
