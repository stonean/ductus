---
title: "007-govern-workflow — tasks"
next-task: 10
---

# 007 — Ductus Command Tasks

Tasks derived from the [plan](plan.md). Complete in order.

> **Note:** these tasks were written against the original layout and command names. The ductus installer now lives at `framework/bootstrap/ductus.md`; command sources moved to `framework/commands/`. Several command names were later renamed (`about → help`, `setup → configure`, `scenario → elaborate`, `triage → groom`, `next` retired). See `spec.md` for the full history.

## 7. A first adoption with existing specs is not an update

- [x] Implement the behavior described in `scenarios/a-first-adoption-with-existing-specs-is-not-an-update.md`

Implemented in `02bbda81` (landed on `main` 2026-11-12), and verified by the scratch-adopter bootstraps `2a4a2fab` records. For a `pi` layout the signal includes a `{config_dir}/prompts/{govern|ductus|<project>}-*.md` file, the layout analogue of the command directory.

- **Done when**: the scenario's described behavior is correctly implemented, the spec's §Pre-flight Checks no longer reads an existing spec root as adoption, and a scratch-adopter bootstrap of a first-adoption subject with existing specs shows the first-run next steps

## 8. The self-update check stops the run only when the running agent's copy is stale

- [x] Edit the `ductus-self-update-precheck` scenario's Behavior bullet and its "Mixed results across selected agents" edge case to say what the bootstrap edit below implements; the scenario's own reason, that the abort prevents destructive work done with stale rules, is the case for it
- [x] `framework/bootstrap/ductus.md` §Self-update check and §Pre-flight abort, with `govern.md` copied: stop the run only when the running agent's installed `ductus.md` is stale. Overwrite another selected agent's stale copy, report it among the agents updated, and continue. The notice no longer says the running session uses older instructions when the running copy matched upstream
- [x] Verify with a scratch-adopter bootstrap: a two-agent project upgraded with `install.sh claude --ref=main`, the other agent's copy stale, runs through in one session. Deferred from 080's task 29, which found the abort in task 11's upgrade run (owner, 2026-11-09)

- **Done when**: a multi-agent upgrade whose running agent's copy is current, with another selected agent's stale, completes in one session, and the scenario says so.

## 9. Exercise the shared bootstrap window's review fixes in a scratch-adopter bootstrap

- [ ] A `--ref` first adoption with specs of its own that stops at the self-update abort re-runs as a first adoption: no migration offered, and the first-run steps printed (8be4f935, cfb348b8)
- [ ] A run whose `--agents=` leaves out the running agent prints that agent's `ductus.md not compared` line (cfb348b8)

- **Done when**: the batched run 003's matching task names shows each outcome, which the commit ticking this task states; a defect it finds is fixed first
