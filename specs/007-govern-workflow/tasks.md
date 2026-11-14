---
title: "007-govern-workflow — tasks"
next-task: 9
---

# 007 — Ductus Command Tasks

Tasks derived from the [plan](plan.md). Complete in order.

> **Note:** these tasks were written against the original layout and command names. The ductus installer now lives at `framework/bootstrap/ductus.md`; command sources moved to `framework/commands/`. Several command names were later renamed (`about → help`, `setup → configure`, `scenario → elaborate`, `triage → groom`, `next` retired). See `spec.md` for the full history.

## 7. A first adoption with existing specs is not an update

- [ ] Implement the behavior described in `scenarios/a-first-adoption-with-existing-specs-is-not-an-update.md`

Implemented in `02bbda81` (landed on `main` 2026-11-12): the box stays open for the done-when's real adopter bootstrap, which must print "No earlier adoption found — running as a first adoption." and the first-run next steps on a first-adoption subject with existing specs, and the update message naming `{spec-root}/templates/spec.md` on a subject holding only that file. For a `pi` layout the signal includes a `{config_dir}/prompts/{govern|ductus|<project>}-*.md` file other than `ductus.md`, the layout analogue of the command directory.

- **Done when**: the scenario's described behavior is correctly implemented, the spec's §Pre-flight Checks no longer reads an existing spec root as adoption, and a real adopter bootstrap of a first-adoption subject with existing specs shows the first-run next steps

## 8. The self-update check stops the run only when the running agent's copy is stale

- [x] Edit the `ductus-self-update-precheck` scenario's Behavior bullet and its "Mixed results across selected agents" edge case to say what the bootstrap edit below implements; the scenario's own reason, that the abort prevents destructive work done with stale rules, is the case for it
- [x] `framework/bootstrap/ductus.md` §Self-update check and §Pre-flight abort, with `govern.md` copied: stop the run only when the running agent's installed `ductus.md` is stale. Overwrite another selected agent's stale copy, report it among the agents updated, and continue. The notice no longer says the running session uses older instructions when the running copy matched upstream
- [ ] Verify with a real adopter bootstrap: a two-agent project upgraded with `install.sh claude --ref=main`, the other agent's copy stale, runs through in one session. Deferred from 080's task 29, which found the abort in task 11's upgrade run (owner, 2026-11-09)

- **Done when**: a multi-agent upgrade whose running agent's copy is current, with another selected agent's stale, completes in one session, and the scenario says so.
