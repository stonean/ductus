---
title: "000-slash-commands — spec"
status: done
dependencies: []
tags: [commands, pipeline, templates]
next-criterion: 16
cross-spec-impact: [014-reclarify-backedge]
---

# 000 — Slash Command Templates

Generic, project-agnostic slash command templates that operationalize the `ductus` development pipeline. The sources live in `framework/commands/`, and `/ductus` installs them into each adopted agent's command location, substituting the project name and the agent's config directory.

> **Signpost:** [066 — Audit Records and the Completion Gate](../066-audit-records-and-the-completion-gate/spec.md)'s scenario `criteria-are-verified-in-review` moved criterion verification from `/{project}:implement`'s completion gate into `/{project}:review`'s criteria step; the scenarios `validation-gates` and `criterion-route-after-draft` say so.

## Problem

The constitution defines the pipeline (spec, plan, tasks, implement) and the spec lifecycle (draft, clarified, planned, in-progress, done), but provides no interactive tooling to enforce it. Adopter projects have built their own slash commands from scratch, duplicating the pipeline logic that should be shared (the plan's Overview names the one these commands were derived from, as its first version recorded at `2c838b20`).

## Behavior

`ductus` provides a set of `.md` command templates in `framework/commands/`. Each template carries the placeholders §Parameterization describes, resolved by the installer per project and per agent. The pipeline commands enforce the gates defined in the constitution.

### Command Set

The commands fall into two groups.

#### Pipeline commands (run in order)

- **specify** — create a spec from the template and set it as the session target
- **clarify** — resolve open questions, enumerate edge cases, verify acceptance criteria, advance draft to clarified
- **plan** — generate plan.md and tasks.md, run the readiness check, advance clarified to planned
- **implement** — walk through tasks, write code and tests, advance the spec through in-progress to done (`/{project}:review` verifies the acceptance criteria first)

#### Utility commands

This spec delivered five utilities beside the pipeline commands :

- **help** — print a fixed overview of the pipeline and command usage (no file reads)
- **target** — set the working feature for the session, persisted in the session file
- **status** — read-only dashboard of all specs, their status, artifacts, dependencies, and next actions
- **analyze** — audit artifacts for consistency, completeness, and cross-spec alignment
- **configure** — install the permissions the commands need to run

Later specs added the rest of the set. The live set is the **Slash commands** manifest in `framework/bootstrap/ductus.md`, which `help` renders.

### Parameterization

Each command template must work for any project by replacing a project-specific placeholder:

- `{project}` — the project name, used in command references and in the installed command's path. **Both are layout-derived and neither is spelled here**: `framework/bootstrap/ductus.md` §Derived values carries the per-layout **Invocation** and **Command/skill path** rows, and they differ across layouts (`/{project}:clarify` is the `claude-style` form alone).
- `{cli-config-dir}` — the selected agent's config directory (`.claude` on Claude Code). Added by `012-multi-agent-govern` when the command set became multi-agent; `scripts/gen-claude-commands.sh` substitutes both.

### Session State

Commands share state through `.ductus/session.toml`, the session file that tracks the working feature. It sits at the repository root, gitignored, at the same path for every adopter whatever its agent or project name; the `write-session` primitive writes it and `resolve-session` reads it. Since `062-concurrent-session-targets`, an agent process with a session identity keeps its own target under `.ductus/sessions/`, and `.ductus/session.toml` is the shared default that every target write also sets (§concurrent-features).

**The key set is stated once**, in `022-deterministic-runtime`'s `write-session-primitive` scenario, and is deliberately not restated here: a second copy of a canonical shape is what [§drift-prevention](../../framework/constitution.md#drift-prevention) forbids. `framework/migrations/session-file-consolidate.md` migrates an adopter's legacy session file.

### Gate Enforcement

Pipeline commands enforce gates before executing:

- **clarify** works on a `draft` spec. A `clarified`, `planned` or `in-progress` spec with no open questions stops with an "already `{status}`" message; one whose body carries open questions takes clarify's recovery path, which reverts it to `draft` (`014-reclarify-backedge`).
- **plan** requires `clarified`: a `draft` spec is directed to clarify, a `planned` or `in-progress` one to implement, and a `done` one to amend. Before any template copy, an existing `plan.md`, `tasks.md` or `data-model.md` is surfaced with a keep-or-replace prompt (014).
- **implement** requires `planned` or `in-progress`: a `draft` spec is directed to clarify, a `clarified` one to plan, and a `done` one to amend (scenario `implement-stops-below-planned`). A `planned` spec moves to `in-progress` without a prompt, since invoking the command is the approval to start, and `in-progress → done` waits on the completion gate.
- **amend** owns the back-edge to `draft`: recording a question on a `clarified`, `planned` or `in-progress` spec reverts it in the same write (014).

### Template References

Pipeline commands reference spec templates from the project's `specs/templates/` directory, not from `ductus`. Each project copies the `ductus` templates into their own `specs/templates/` during bootstrap (the `ductus`-owned shared files section of `framework/bootstrap/ductus.md`).

## Acceptance Criteria

- [x] AC1: `framework/commands/` holds a command template file for each command in the **Slash commands** manifest in `framework/bootstrap/ductus.md`.
- [x] AC2: Each template uses `{project}` and `{cli-config-dir}` as its only project-specific placeholders; the generator substitutes both, so the command set can target more than one AI CLI (§Parameterization).
- [x] AC3: Pipeline commands (specify, clarify, plan, implement) enforce gates matching the constitution's spec lifecycle
- [x] AC4: The `help` command prints a self-contained guide without reading any files
- [x] AC5: The `target` command writes a session file and displays feature status
- [x] AC6: The `status` command scans all spec directories and displays a dashboard table
- [x] AC8: The `analyze` command checks spec integrity, artifact completeness, plan consistency, task consistency, dependencies, and cross-spec references
- [x] AC9: The `configure` command configures permissions for common operations (git, lint, file reads)
- [x] AC10: The `specify` command determines the next feature number and creates the spec directory, which are `create-feature`'s job; `specify` and `create-feature` write no README.
- [x] AC11: Commands reference `specs/templates/` for templates (not `ductus` templates)
- [x] AC13: The `analyze` command runs `npx markdownlint-cli2` on the feature's files as part of its checks. It runs through the `lint-markdown` primitive, which invokes `markdownlint-cli2` and prefers the project's local install

## Resolved Questions

- **Configure and web fetch permissions** — leave to project-specific customization. The configure command handles universal operations (git, lint, file reads). Projects add their own web fetch domains.
- **Analyze and markdown lint** — the audit command includes a markdownlint check among its own checks. Lint compliance is a quality gate defined in the constitution. The check still runs (`framework/commands/analyze.md` step 7, through the `lint-markdown` primitive rather than a raw `npx markdownlint-cli2` call); findings are rendered in the four severity tiers §text-first-artifacts defines.
- **Specify and dependencies** — specify accepts no dependency argument; dependencies are set during writing and clarifying, not at creation time. That holds, and `dependencies:` is derived from body links rather than hand-authored at all since `017-derive-dont-ask`. Its other arguments are `--branch`, `--branch-id <identifier>` and `--fold-into <feature>` (`051-branch-scoped-spec-numbering`); none of the three is a dependency.
- **Retire/archive command** — supported: §spec-lifecycle requires a retired feature's spec to be deleted rather than left at `done`, `/{project}:consolidate` is the supported route (it re-points every inbound pointer it reads before removing the directory), and `retire-feature` is the primitive that performs the removal.
- **Lightweight track handling** — none. `specify` asks no qualifying questions and every feature takes the one pipeline and `spec.md`; `023-govern-refinement` removed the track.
