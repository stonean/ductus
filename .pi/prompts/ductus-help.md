---
description: Display an overview of the pipeline and its slash commands.
---

# Help

Display an overview of the pipeline and how to use its slash commands.

## Purpose

A static, at-a-glance guide to the pipeline: the states a spec moves through, the command for each transition, and the key concepts (session target, inbox, rules). Printed verbatim — it scans no files and runs no commands.

## Scope Boundaries

- Prints a fixed guide. Do NOT read spec files, list directories, run generators, or invoke any primitive — this command has no runtime path and no side effects.
- The command tables in the guide are pre-generated from each command's frontmatter `description:` — print them as written, and do NOT assemble them at print time.

## Instructions

Print the following guide exactly (do not scan files or run commands):

---

## ductus — Spec-Driven Development Pipeline

ductus is a set of slash commands that guide features from idea to implementation through a structured pipeline.

### Pipeline States

```text
draft → clarified → planned → in-progress → done
```

Three back-edges keep the lifecycle honest:

- `/ductus:amend` reverts a `clarified`, `planned`, or `in-progress` spec to `draft` when a new open question surfaces — `draft` is the only status that tolerates open questions. The next `/ductus:clarify` resolves the question and the spec advances forward again.
- `/ductus:amend` reverts a `done` spec to `in-progress` when a new scenario is added (the scenario route) — the scenario captures the change, the spec evolves with it.
- A **meaningful body edit** to any artifact under a `done` spec's directory takes the same edge. New scope, changed semantics, or a corrected fact reopens the spec; a uniform rename sweep, a cross-service reference change, and criterion-label assignment are mechanical and do not.

Each feature lives in `specs/NNN-feature-name/` and progresses through these states by running the corresponding command.

### Commands

#### Pipeline (advance state)

<!-- generated:commands-pipeline:start -->

| Command | Pipeline Gate | Description |
| --- | --- | --- |
| `/ductus:specify` | → draft | Create a new feature spec. |
| `/ductus:clarify` | draft → clarified | Resolve open questions and advance a spec from draft to clarified. |
| `/ductus:plan` | clarified → planned | Create a technical plan and task breakdown for a clarified spec. |
| `/ductus:implement` | planned → in-progress → done | Execute implementation tasks for the targeted feature. |
| `/ductus:review` | blocks `done` (MUST violations) | Audit code against rules — security, reuse, quality, efficiency, simplicity. Writes review.md; blocks done on MUST violations. |
| `/ductus:analyze` | — | Audit artifacts against each other — spec, plan, tasks, scenarios, frontmatter, dependencies, rule IDs. Detection never modifies an artifact it audits; each live finding is then fixed, routed, or discarded with confirmation, the run is recorded, and --fix reverts a done spec drifted by review state, unresolved scenario questions, or undispositioned findings. |

<!-- generated:commands-pipeline:end -->

#### Refine

<!-- generated:commands-refine:start -->

| Command | Description |
| --- | --- |
| `/ductus:amend` | Add a question or a scenario to the targeted spec (classifier-driven). |
| `/ductus:prune` | Prune a feature's spec directory — drop spent task sections or reset tasks.md, and move plan sections outside the design record home before removing them. |
| `/ductus:fold` | Fold a branch-scoped spec into its upstream home and retire the staging directory. |
| `/ductus:consolidate` | Merge a spec into another and remove its directory, re-pointing every inbound pointer first. |

<!-- generated:commands-refine:end -->

#### Brownfield (absorb existing reality)

<!-- generated:commands-brownfield:start -->

| Command | Description |
| --- | --- |
| `/ductus:log` | Record a raw item to the inbox. |
| `/ductus:groom` | Walk the inbox and route each item to its proper home. |

<!-- generated:commands-brownfield:end -->

#### Orient

<!-- generated:commands-orient:start -->

| Command | Description |
| --- | --- |
| `/ductus:target` | Set the working feature (and optionally scenario) for this session. |
| `/ductus:link` | Register a service so cross-service references resolve to its lifecycle status. |
| `/ductus:status` | Display the pipeline view for all feature specs. |
| `/ductus:help` | Display an overview of the pipeline and its slash commands. |

<!-- generated:commands-orient:end -->

#### Bootstrap (one-time per project)

<!-- generated:commands-bootstrap:start -->

| Command | Description |
| --- | --- |
| `/ductus` | Adopt or update ductus in an existing project. |
| `/ductus:configure` | Configure settings.local.json with permissions for slash commands. |

<!-- generated:commands-bootstrap:end -->

### Typical Session

```text
/ductus:configure                 # first time only
/ductus:status                    # see where everything stands
/ductus:target 000                # pick a feature to work on
/ductus:clarify                   # resolve open questions
/ductus:plan                      # generate implementation plan
/ductus:implement                 # write the code
```

### Key Concepts

- **Session target** — The feature you're currently working on. Most commands operate on the target by default. Each agent process launched with a session identity (`DUCTUS_SESSION`, or on Claude Code the session id it passes to its tools) keeps its own target, so several agents can share one working tree; `.ductus/session.toml` holds the shared default.
- **Dependencies** — Features declare dependencies in their spec. A feature is blocked until its dependencies reach `clarified` or later.
- **Artifacts** — Each feature directory can contain `spec.md`, `plan.md`, `tasks.md`, `data-model.md`, `research.md`, and a `scenarios/` subdirectory, plus the `review.md` and `analysis.md` records that `/ductus:review` and `/ductus:analyze` write.
- **Scenarios** — A scenario is a spec at a lower level of abstraction. Scenarios live in `specs/NNN-feature/scenarios/slug.md` and capture bugs, edge cases, and detailed behavior. Each scenario gets a linked task in `tasks.md`.
- **Bug decision tree** — When a bug is reported, the first matching condition decides the route: (1) no rule covers the cross-cutting concern → promote it to a rule, (2) no spec exists → write the spec first, (3) spec is ambiguous → fix the spec, (4) spec is clear → add a scenario.
- **Inbox** — `specs/inbox.md` is the place for todos you capture by hand. Items are recorded with `/ductus:log` and groomed with `/ductus:groom` into a rule, a spec, a scenario, a chore done in the pass, or a discard. Nothing a pipeline run finds lands there: `/ductus:review`, `/ductus:analyze`, and `/ductus:implement` fix, route, or discard their own findings. `/ductus:status` shows how many items are outstanding.
- **Finish before moving on** — Prefer completing a feature through the full pipeline before starting the next. Depth-first keeps context focused.

---
