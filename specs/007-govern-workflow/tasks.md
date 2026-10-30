---
title: "007-govern-workflow — tasks"
---

# 007 — Ductus Command Tasks

Tasks derived from the [plan](plan.md). Complete in order.

> **Note:** these tasks were written against the original layout and command names. The ductus installer now lives at `framework/bootstrap/ductus.md`; command sources moved to `framework/commands/`. Several command names were later renamed (`about → help`, `setup → configure`, `scenario → elaborate`, `triage → groom`, `next` retired). See `spec.md` for the full history.

## 1. Add `{cli-config-dir}` placeholder to command templates

- [x] Update all `.md` files in `commands/` to replace hardcoded `.claude/` references with `{cli-config-dir}/`.

Done when: every `.claude/` reference in `commands/*.md` that is CLI-specific uses `{cli-config-dir}/` instead.

## 2. Re-derive the installed commands from updated templates

- [x] Regenerate `.claude/commands/ductus/*.md` from the updated `commands/` templates with `{cli-config-dir}` resolved to `.claude` and `{project}` resolved to `gov`.

Done when: all installed commands match the updated templates with placeholders resolved.

## 3. Create `ductus/ductus.md` for Claude Code

- [x] Write the Claude Code ductus command in the `ductus/` directory with full file manifest, pre-flight checks, input collection, fetch logic, placeholder substitution, conflict handling, and post-scaffolding output.

Done when: `ductus/ductus.md` exists, passes markdownlint, and contains the complete manifest with `.claude` as the config directory.

## 4. Create `ductus/ductus-auggie.md` for Auggie

- [x] Same structure as `ductus.md` but targeting `.augment/` paths, with setup step omitted from next steps.

Done when: `ductus/ductus-auggie.md` exists, passes markdownlint, and targets `.augment/` paths.

## 5. Update spec status to `done`

- [x] Set the spec status to `done` and run markdownlint on all modified files.

Done when: spec status is `done`, all modified files pass markdownlint.

## 6. Implement scenario: ductus-self-update-precheck

- [x] Implement the behavior described in `scenarios/ductus-self-update-precheck.md`

Done when: the scenario's described behavior is correctly implemented and tested.

## 7. A first adoption with existing specs is not an update

- [ ] Implement the behavior described in `scenarios/a-first-adoption-with-existing-specs-is-not-an-update.md`

- **Done when**: the scenario's described behavior is correctly implemented, the spec's §Pre-flight Checks no longer reads an existing spec root as adoption, and a real adopter bootstrap of a first-adoption subject with existing specs shows the first-run next steps
