---
title: "000-slash-commands — spec"
status: done
dependencies: []
tags: [commands, pipeline, templates]
next-criterion: 16
---

# 000 — Slash Command Templates

Generic, project-agnostic slash command templates that operationalize the `ductus` development pipeline. The sources live in `framework/commands/`, and `/ductus` installs them into each adopted agent's command location, substituting the project name and the agent's config directory.

## Problem

The constitution defines the pipeline (spec, plan, tasks, implement) and the spec lifecycle (draft, clarified, planned, in-progress, done), but provides no interactive tooling to enforce it. Adopter projects have built their own slash commands from scratch, duplicating the pipeline logic that should be shared.

## Behavior

`ductus` provides a set of `.md` command templates in `framework/commands/`. Each template carries the placeholders §Parameterization describes, resolved by the installer per project and per agent. The pipeline commands enforce the gates defined in the constitution.

### Command Set

The commands fall into two groups.

#### Pipeline commands (run in order)

- **specify** — create a spec from the template and set it as the session target
- **clarify** — resolve open questions, enumerate edge cases, verify acceptance criteria, advance draft to clarified
- **plan** — generate plan.md and tasks.md, run the readiness check, advance clarified to planned
- **implement** — walk through tasks, write code and tests, verify acceptance criteria, advance the spec through in-progress to done

#### Utility commands

This spec delivered five utilities beside the pipeline commands (a sixth it named, `next`, was never written — AC7):

- **help** — print a fixed overview of the pipeline and command usage (no file reads)
- **target** — set the working feature for the session, persisted in the session file
- **status** — read-only dashboard of all specs, their status, artifacts, dependencies, and next actions
- **analyze** — audit artifacts for consistency, completeness, and cross-spec alignment
- **configure** — install the permissions the commands need to run

Later specs added the rest of the set. The live set is the **Slash commands** manifest in `framework/bootstrap/ductus.md`, which `help` renders.

### Parameterization

Each command template must work for any project by replacing a project-specific placeholder:

- `{project}` — the project name, used in command references and in the installed command's path. **Both are layout-derived and neither is spelled here**: `framework/bootstrap/ductus.md` §Derived values carries the per-layout **Invocation** and **Command/skill path** rows, and they differ across layouts (`/{project}:clarify` is the `claude-style` form alone). As delivered, `.claude` and the colon form were the only ones, because Claude Code was the only agent.
- `{cli-config-dir}` — the selected agent's config directory (`.claude` on Claude Code). Added by `012-multi-agent-govern` when the command set became multi-agent; `scripts/gen-claude-commands.sh` substitutes both. As delivered, `{project}` was the only placeholder.

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

Pipeline commands reference spec templates from the project's `specs/templates/` directory, not from `ductus`. Each project copies the `ductus` templates into their own `specs/templates/` during bootstrap.

## Acceptance Criteria

- [x] AC1: Ten command template files exist in the command-source directory. **Never held exactly as written.** The first commit's `commands/` directory held thirteen files: nine of the ten this spec named, plus `create`, `question`, `scenario` and `triage`. The tenth, `next`, was never written — no commit in this repository contains a `next.md`. The directory is now `framework/commands/`, and the set is the **Slash commands** manifest in `framework/bootstrap/ductus.md`
- [x] AC2: Each template uses `{project}` as the only project-specific placeholder — **delivered, then widened by `012-multi-agent-govern`**: the generator substitutes a second one, `{cli-config-dir}`, so the command set can target more than one AI CLI. The single-placeholder property held as delivered; §Parameterization records both.
- [x] AC3: Pipeline commands (specify, clarify, plan, implement) enforce gates matching the constitution's spec lifecycle
- [x] AC4: The `help` command prints a self-contained guide without reading any files
- [x] AC5: The `target` command writes a session file and displays feature status
- [x] AC6: The `status` command scans all spec directories and displays a dashboard table
- [x] AC7: The `next` command maps current status to the correct pipeline command — **never delivered**: no `next` command exists in this repository's history. Its job is done by the next-action column `status` renders and the next step `target` recommends
- [x] AC8: The `analyze` command checks spec integrity, artifact completeness, plan consistency, task consistency, dependencies, and cross-spec references
- [x] AC9: The `configure` command configures permissions for common operations (git, lint, file reads)
- [x] AC10: The `specify` command determines the next feature number, creates the spec directory, and updates README — **delivered, then the README clause was removed by `003-bootstrap-automation`** (`9847647b`, 2026-06-10), which dropped the generated Feature Specs table together with every live consumer, `specify` among them. Numbering and directory creation still hold and are `create-feature`'s job today; no command source updates a README.
- [x] AC11: Commands reference `specs/templates/` for templates (not `ductus` templates)
- [x] AC12: Commands reference `.claude/{project}-session.json` for session state — **delivered, then superseded by `022-deterministic-runtime`**: session state is `.ductus/session.toml` for every adopter. See §Session State.
- [x] AC13: The `analyze` command runs `npx markdownlint-cli2` on the feature's files as part of its checks. It runs through the `lint-markdown` primitive, which invokes `markdownlint-cli2` and prefers the project's local install
- [x] AC14: The `specify` command prompts qualifying questions and creates `spec-and-plan.md` for lightweight track features — **delivered, then removed by `023-govern-refinement`**: `specify` asks no qualifying questions and the combined template is deleted. Checked because 000 delivered it; the removal is 023's.
- [x] AC15: Pipeline commands detect and handle both `spec.md` and `spec-and-plan.md` — **delivered, then removed by `023-govern-refinement`**: its AC10 stripped the two-filename fallback from every command source. The runtime retains the legacy filename in `is_spec_path` so an unmigrated adopter still resolves, which is a read tier rather than the detection branch this criterion describes.

## Resolved Questions

- **Configure and web fetch permissions** — leave to project-specific customization. The configure command handles universal operations (git, lint, file reads). Projects add their own web fetch domains.
- **Analyze and markdown lint** — the audit command includes a markdownlint check among its own checks. Lint compliance is a quality gate defined in the constitution. The check still runs (`framework/commands/analyze.md` step 7, through the `lint-markdown` primitive rather than a raw `npx markdownlint-cli2` call); what changed is the report — findings are rendered in the four severity tiers §text-first-artifacts defines, not as the single PASS/FAIL verdict this answer assumed.
- **Specify and dependencies** — specify accepts no dependency argument; dependencies are set during writing and clarifying, not at creation time. That holds, and `dependencies:` is derived from body links rather than hand-authored at all since `017-derive-dont-ask`. The stronger form of this answer — *"specify accepts only a description"* — was overtaken by `051-branch-scoped-spec-numbering`, which added `--branch`, `--branch-id <identifier>` and `--fold-into <feature>`; none of the three is a dependency.
- **Retire/archive command** — deferred at the time; **the deferral has since been discharged.** §spec-lifecycle now requires a retired feature's spec to be deleted rather than left at `done`, `/{project}:consolidate` is the supported route (it re-points every inbound pointer before removing the directory), and `retire-feature` is the primitive that performs the removal. The `specs/README.md` §Future Considerations entry that this answer pointed at is the stale half and is corrected in the same change.
- **Lightweight track handling** — the `specify` command detected lightweight track eligibility by prompting qualifying questions, created `spec-and-plan.md` when all answers indicated small and clear, and pipeline commands adapted based on which file existed. The decision was reversed by `023-govern-refinement`, which removed the track outright; AC14 and AC15 record what was delivered.
