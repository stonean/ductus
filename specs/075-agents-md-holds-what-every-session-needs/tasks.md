---
next-task: 8
---

# 075 — AGENTS.md holds what every session needs Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 7. Verify the criteria

Blocked on `063` task 15, on `050` task 44's last part, and on tasks 4–6. `041` task 23 and `055` tasks 14 and 15 are done.

Where the evidence already is: AC6, AC11 and AC12's `claude -p` run is recorded in `b4f4d724`'s message, so the `/tmp` fixture need not survive; rebuild it from that message to re-run it. Read the criteria against these delivering commits: AC1 and AC13 `73f21fc9`; AC2 `cb30b333`; AC3, AC4 and AC9 `b763977c`; AC10 `ab5eaa49`; AC15 `c6a0dca2` and `e8ed5d62`. AC8's layout records are in `cb30b333` and `b763977c`; the size check (`063` task 15) and the hook step (task 5) still owe theirs.

After this task, `041` (task 22 pending, another line of work), `050` (tasks 42 and 44), `055` (no task pending), `063` and this spec each need `/ductus:review`, `/ductus:analyze` and the owner's approval to reach `done`. The release tag follows.

- [ ] Read each of this spec's criteria against the tree, including AC6, AC11 and AC12's recorded `claude -p` run over a fixture project under `/tmp`.
- [ ] For AC7, confirm that `041`'s walk landed on `main` no later than the size check and the hook step. Confirm with `git tag --contains` that no `ductus-v*` tag holds either commit without the other.
- [ ] Confirm that each commit landing the block, the templates and the size check records every layout's impact (AC8).
- [ ] Run every step of `.github/workflows/framework-checks.yml`, `generators.yml` and `runtime.yml` locally, and run `scripts/audit/run-all.sh` after committing.

- **Done when**: every criterion holds against the tree and the local gate and the audit pass. The release follows the specs reaching `done`, per `AGENTS.md`'s Workflow.
