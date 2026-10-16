---
title: "000-slash-commands — spec"
status: done
dependencies: []
tags: [commands, pipeline, templates]
next-criterion: 16
---

# 000 — Slash Command Templates

Generic, project-agnostic slash command templates that operationalize the `ductus` development pipeline. Projects copy these commands into their `.claude/commands/{project}/` directory and customize the project name and any project-specific paths.

> **Note:** subsequent specs renamed several commands and removed one. References below to `about`, `setup`, `validate`, and `next` reflect the original design; the current names are `help` (about), `configure` (setup, renamed by [012-multi-agent-govern](../012-multi-agent-govern/spec.md)), `analyze` (validate, renamed by `023-govern-refinement`), and the auto-advance `next` command was retired in favor of explicit pipeline gates surfaced by `/{project}:status` and `/{project}:target`.
>
> **Note:** [014-reclarify-backedge](../014-reclarify-backedge/spec.md) made `/{project}:amend` the owner of the `clarified` / `planned` / `in-progress` → `draft` back-edge (the command mutates spec status when recording a question on a non-`draft` spec), added a recovery path to `/{project}:clarify` for hand-edited inconsistent state (a non-`draft` spec with open questions in the body), and added overwrite-protection to `/{project}:plan` (existing `plan.md` / `tasks.md` / `data-model.md` are surfaced with a keep/replace prompt before any template copy). Gate-enforcement language below predates these changes; the current behavior of the three commands is documented in their command sources under `framework/commands/`.
>
> **Note:** path references below (`commands/`, `templates/`) reflect the original layout. The repository was later reorganized so command sources live in `framework/commands/` and templates in `framework/templates/{spec,project}/`. Adopting projects' destination paths did not change.

## Problem

The constitution defines the pipeline (spec, plan, tasks, implement) and the spec lifecycle (draft, clarified, planned, in-progress, done), but provides no interactive tooling to enforce it. Adopter projects have built their own slash commands from scratch, duplicating the pipeline logic that should be shared.

## Behavior

`ductus` provides a set of `.md` command templates in a `commands/` directory. Each template uses a placeholder `{project}` that adopters replace with their project name. The commands enforce the pipeline gates defined in the constitution.

### Command Set

Ten commands organized into two groups:

#### Pipeline commands (run in order)

- **specify** — prompt qualifying questions to detect lightweight track, create spec (or spec-and-plan) from template, set as session target, add to README (the qualifying questions and the combined document were removed by `023-govern-refinement`; `specify` always creates `spec.md` today)
- **clarify** — resolve open questions, enumerate edge cases, verify acceptance criteria, advance draft to clarified
- **plan** — generate plan.md and tasks.md, run readiness check, advance clarified to planned
- **implement** — walk through tasks, write code/tests, verify acceptance criteria, advance planned to done

#### Utility commands

- **about** — print a fixed overview of the pipeline and command usage (no file reads)
- **target** — set the working feature for the session, persisted in a session file
- **status** — read-only dashboard of all specs, their status, artifacts, dependencies, and next actions
- **next** — auto-advance the targeted feature by running the appropriate pipeline command
- **validate** — read-only audit of artifacts for consistency, completeness, and cross-spec alignment
- **setup** — configure `.claude/settings.local.json` with permissions needed for commands to run

### Parameterization

Each command template must work for any project by replacing a project-specific placeholder:

- `{project}` — the project name, used in command references and in the installed command's path. **Both are layout-derived and neither is spelled here**: `framework/bootstrap/ductus.md` §Derived values carries the per-layout **Invocation** and **Command/skill path** rows, and they differ across all three layouts (`/{project}:clarify` is the `claude-style` form alone). As delivered, `.claude` and the colon form were the only ones, because Claude Code was the only agent.
- `{cli-config-dir}` — the selected agent's config directory (`.claude` on Claude Code). Added by `012-multi-agent-govern` when the command set became multi-agent; `scripts/gen-claude-commands.sh` substitutes both. As delivered, `{project}` was the only placeholder.

Commands reach session state through `.ductus/session.toml` (see §Session State).

### Session State

Commands share state through a session file that tracks the current working feature.

> **Superseded by `022-deterministic-runtime`.** As delivered the file was JSON at
> `{cli-config-dir}/{project}-session.json`, with camelCase keys (`setAt`), so its path
> baked in both the AI CLI's config directory and the adopter's project name. 022 moved it
> to `.ductus/session.toml` — repo root, gitignored, host- and project-name-agnostic, TOML
> with kebab-case keys, the same path for every adopter — and `write-session` is the
> primitive that writes it. **The key set is stated once**, in 022's
> `write-session-primitive` scenario, and is deliberately not restated here: a second copy
> of a canonical shape is what [§drift-prevention](../../framework/constitution.md#drift-prevention)
> forbids, and the three keys 000 delivered are already two short of it —
> `009-scenario-targeting` added `scenario` and `scenario-path`.
> `framework/migrations/session-file-consolidate.md` migrates an adopter's legacy file.

### Gate Enforcement

Pipeline commands enforce gates before executing:

- **clarify** gate: spec must be at `draft`; if already `clarified` or later, report and stop
- **plan** gate: spec must be at `clarified`; if `draft`, direct to clarify first
- **implement** gate: spec must be at `planned` or `in-progress`; if earlier, direct to the right command

### Lightweight Track Detection

> **Superseded by [023](../023-govern-refinement/spec.md).** The lightweight track was removed: `framework/templates/spec/spec-and-plan.md` is deleted, the constitution's §lightweight-track section is gone, `specify` no longer asks qualifying questions, and no command source carries the two-filename fallback. The section below records what 000 delivered. The runtime still *reads* a legacy `spec-and-plan.md` so an unmigrated adopter resolves (`is_spec_path`), and `framework/migrations/spec-and-plan-sunset.md` records the rename.

As delivered, the `specify` command determined whether a feature qualified for the lightweight track by prompting the user with qualifying questions:

- Does this touch more than one module or package?
- Are there open questions or unknowns about the approach?
- Does it involve data model changes beyond trivial?
- Will it be more than ~50 lines of spec?

If all answers indicated "small and clear," specify created `spec-and-plan.md` from a combined template instead of `spec.md`. The `clarify` and `plan` commands detected which file existed and adapted: `clarify` worked on whichever file was present, and `plan` skipped plan creation if `spec-and-plan.md` already contained the plan section.

### Template References

Pipeline commands reference spec templates from the project's `specs/templates/` directory, not from `ductus`. Each project copies the `ductus` templates into their own `specs/templates/` during bootstrap.

## Acceptance Criteria

- [x] AC1: Ten command template files exist in `commands/` directory
- [x] AC2: Each template uses `{project}` as the only project-specific placeholder — **delivered, then widened by `012-multi-agent-govern`**: the generator substitutes a second one, `{cli-config-dir}`, so the command set can target more than one AI CLI. The single-placeholder property held as delivered; §Parameterization records both.
- [x] AC3: Pipeline commands (specify, clarify, plan, implement) enforce gates matching the constitution's spec lifecycle
- [x] AC4: The `about` command prints a self-contained guide without reading any files
- [x] AC5: The `target` command writes a session file and displays feature status
- [x] AC6: The `status` command scans all spec directories and displays a dashboard table
- [x] AC7: The `next` command maps current status to the correct pipeline command
- [x] AC8: The `validate` command checks spec integrity, artifact completeness, plan consistency, task consistency, dependencies, and cross-spec references
- [x] AC9: The `setup` command configures permissions for common operations (git, lint, file reads)
- [x] AC10: The `specify` command determines the next feature number, creates the spec directory, and updates README — **delivered, then the README clause was removed by `003-bootstrap-automation`** (`9847647b`, 2026-06-10), which dropped the generated Feature Specs table together with every live consumer, `specify` among them. Numbering and directory creation still hold and are `create-feature`'s job today; no command source updates a README.
- [x] AC11: Commands reference `specs/templates/` for templates (not `ductus` templates)
- [x] AC12: Commands reference `.claude/{project}-session.json` for session state — **delivered, then superseded by `022-deterministic-runtime`**: session state is `.ductus/session.toml` for every adopter. See §Session State.
- [x] AC13: The `validate` command runs `npx markdownlint-cli2` on the feature's files as part of its checks
- [x] AC14: The `specify` command prompts qualifying questions and creates `spec-and-plan.md` for lightweight track features — **delivered, then removed by `023-govern-refinement`**: `specify` asks no qualifying questions and the combined template is deleted. Checked because 000 delivered it; the removal is 023's.
- [x] AC15: Pipeline commands detect and handle both `spec.md` and `spec-and-plan.md` — **delivered, then removed by `023-govern-refinement`**: its AC10 stripped the two-filename fallback from every command source. The runtime retains the legacy filename in `is_spec_path` so an unmigrated adopter still resolves, which is a read tier rather than the detection branch this criterion describes.

## Resolved Questions

- **Setup and web fetch permissions** — leave to project-specific customization. The setup command handles universal operations (git, lint, file reads). Projects add their own web fetch domains.
- **Validate and markdown lint** — the audit command includes a markdownlint check among its own checks. Lint compliance is a quality gate defined in the constitution. The check still runs (`framework/commands/analyze.md` step 7, through the `lint-markdown` primitive rather than a raw `npx markdownlint-cli2` call); what changed is the report — findings are rendered in the four severity tiers §text-first-artifacts defines, not as the single PASS/FAIL verdict this answer assumed.
- **Specify and dependencies** — specify accepts no dependency argument; dependencies are set during writing and clarifying, not at creation time. That holds, and `dependencies:` is derived from body links rather than hand-authored at all since `017-derive-dont-ask`. The stronger form of this answer — *"specify accepts only a description"* — was overtaken by `051-branch-scoped-spec-numbering`, which added `--branch`, `--branch-id <identifier>` and `--fold-into <feature>`; none of the three is a dependency.
- **Retire/archive command** — deferred at the time; **the deferral has since been discharged.** §spec-lifecycle now requires a retired feature's spec to be deleted rather than left at `done`, `/{project}:consolidate` is the supported route (it re-points every inbound pointer before removing the directory), and `retire-feature` is the primitive that performs the removal. The `specs/README.md` §Future Considerations entry that this answer pointed at is the stale half and is corrected in the same change.
- **Lightweight track handling** — the `specify` command detected lightweight track eligibility by prompting qualifying questions, created `spec-and-plan.md` when all answers indicated small and clear, and pipeline commands adapted based on which file existed. The decision was reversed by `023-govern-refinement`, which removed the track outright; see §Lightweight Track Detection above.
