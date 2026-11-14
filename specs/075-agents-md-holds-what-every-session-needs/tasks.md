---
next-task: 8
---

# 075 — AGENTS.md holds what every session needs Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 7. Verify the criteria

- [x] Read each of this spec's criteria against the tree, including AC6, AC11 and AC12's recorded `claude -p` run over a fixture project under `/tmp`.
- [x] For AC7, confirm that `041`'s walk landed on `main` no later than the size check and the hook step. Confirm with `git tag --contains` that no `ductus-v*` tag holds either commit without the other.
- [x] Confirm that each commit landing the block, the templates and the size check records every layout's impact (AC8).
- [x] Run every step of `.github/workflows/framework-checks.yml`, `generators.yml` and `runtime.yml` locally, and run `scripts/audit/run-all.sh` after committing.

- **Done when**: every criterion holds against the tree and the local gate and the audit pass. The release follows the specs reaching `done`, per `AGENTS.md`'s Workflow.
