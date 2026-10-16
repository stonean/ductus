---
title: "003-bootstrap-automation — spec"
status: in-progress
dependencies: [000-slash-commands, 001-system-spec-templates, 002-project-scaffolding]
tags: [bootstrap, commands]
next-criterion: 18
---

# 003 — Bootstrap Automation

`ductus` slash commands that dogfood the same pipeline commands adopting projects use (`/ductus:about`, `/ductus:target`, `/ductus:status`, `/ductus:setup`, `/ductus:specify`, `/ductus:clarify`, `/ductus:plan`, `/ductus:implement`, `/ductus:analyze`, `/ductus:next`), plus a `ductus`-specific `/ductus:init` that scaffolds new projects from templates.

> **Note:** the command set evolved after this spec shipped. `/ductus:about` is now `/ductus:help`, `/ductus:setup` is now `/ductus:configure` (renamed by [012-multi-agent-govern](../012-multi-agent-govern/spec.md)), and `/ductus:next` was retired — pipeline next-step suggestions are surfaced by `/ductus:status` and `/ductus:target` instead. The brownfield commands (`/ductus:specify`, `/ductus:log`, `/ductus:groom`) and the elaborate command (`/ductus:amend`) were added by later specs and are scaffolded alongside the original set. `/ductus:init` was **retired on 2026-09-15** — the ten dogfooded pipeline commands this spec delivered are unaffected, but the init half below (§`/ductus:init`, AC5–AC17, and the Resolved Questions that decided its shape) records what shipped rather than live behaviour.
>
> **Note:** path references below (`commands/`) reflect the original layout. The repository was later reorganized so command sources live in `framework/commands/`; the generator script `scripts/gen-claude-commands.sh` produces `.claude/commands/ductus/` from those sources. Adopting projects' destination paths did not change.

## Problem

The `ductus` README describes a 7-step manual bootstrap process: init git, copy files, fill in AGENTS.md, create CLAUDE.md, create specs directory, add first spec, follow the pipeline. This is error-prone and tedious. A slash command can automate the mechanical steps while prompting for the decisions that require human input.

Additionally, the `ductus` project itself has no slash commands. Agents working on `ductus` specs operate without pipeline enforcement — no status gates, no dependency checks, no structured workflow. Governance should dogfood the same commands it provides to adopting projects.

## Behavior

### Standard pipeline commands

Copy all ten command templates from `commands/` into `.claude/commands/ductus/`, replacing `{project}` with `ductus`. This gives `ductus` the same slash commands as any adopting project:

- `/ductus:about` — pipeline overview
- `/ductus:target` — set session target feature
- `/ductus:status` — dashboard of all specs
- `/ductus:setup` — configure permissions
- `/ductus:specify` — create new feature spec
- `/ductus:clarify` — resolve open questions (draft → clarified)
- `/ductus:plan` — create plan and tasks (clarified → planned)
- `/ductus:implement` — execute tasks (planned → done)
- `/ductus:analyze` — audit artifacts for consistency
- `/ductus:next` — auto-advance to next phase

These commands enforce the same pipeline gates, dependency checks (via AGENTS.md boundaries), and conventions that adopting projects follow.

### /ductus:init

> **Retired 2026-09-15.** `/ductus:init` no longer exists: `.claude/commands/ductus/init.md` was deleted and `scripts/gen-claude-commands.sh` no longer spares it from the prune loop. It was retired rather than repaired because an orphan no sweep reaches drifts on every rename with nothing able to report it, and `/ductus` already covers the greenfield case that was its last remaining role (`framework/bootstrap/ductus.md` prompts for the spec root, ships the `inbox.md` and `rules/` manifest rows, and resolves every `specs/…` destination under the configured name). The subsections below record what the command did; they are not live behaviour. The audit family that existed to compare its file list against `/ductus` is `026-framework-self-audit` AC4.
>
> **`004-tech-stack-selection` was consolidated here and its directory removed in the same change**, because every criterion it carried described this command's input collection and nothing it delivered outlived the command. What it delivered, recorded here because consolidation migrates no content and git history is otherwise the only copy: it replaced the single **Primary language(s)** question in §Inputs with a tech-stack questionnaire — project type (backend / frontend / fullstack), then per-section framework, language, database, messaging, test runner and CSS/UI, each with 2–4 example choices plus *Other* and *Skip* — populated the `AGENTS.md` **Tech Stack** table from the selections with a layer→role mapping, and derived the `.gitignore` language patterns from them instead of from a separate prompt. Its one scenario, `framework-implies-language`, held that a framework which unambiguously determines its language (Rails → Ruby, Django → Python, Gin → Go) suppresses the language *question* but still writes the language *row*.
>
> **The capability is not replaced, and that is a decision rather than an oversight.** `/ductus` collects **Primary language(s)** — the exact question 004 removed — and carries none of the questionnaire; its §Post-Scaffolding Output tells the operator to fill in `AGENTS.md` by hand. So the local greenfield path reverts to pre-004 behaviour. It is out of scope for the retirement because init was **never shipped to adopters** — it had no **Shared Files** manifest row, so no adopter ever had the questionnaire and none loses it; the loss falls only on the maintainer scaffolding a brand-new project locally, which is the role the retirement decision weighed and accepted. Rule-file selection is unaffected: `discover-rule-files` reads `[rules] surfaces`, which `/ductus` collects explicitly, not the Tech Stack table. What does still read that table is `/{project}:review`'s tech-stack alignment check — and it read a hand-filled table for every `/ductus`-adopted project already, so nothing about that changes either.

### Inputs

The command collects from the user (via `$ARGUMENTS` or interactive prompts):

- **Project name** — used for directory name and placeholder substitution
- **Project path** — where to create the project (defaults to sibling of `ductus`)
- **Project description** — one-line description for README and AGENTS.md
- **Primary language(s)** — used to fetch language-specific .gitignore patterns from github.com/github/gitignore

### Pre-flight Check

Before scaffolding, verify the target directory does not already exist. If it does, stop and report the conflict. Init is for new projects only — adding `ductus` to an existing project should follow the manual bootstrap steps in the README.

### Scaffolding Steps

1. Create the project directory and initialize git
2. Copy `ductus` files: `constitution.md`, `.markdownlint-cli2.jsonc`
3. Copy and customize `AGENTS.md` from template (replace `{project}` placeholder)
4. Create `CLAUDE.md` from template
5. Create `specs/` directory with system spec templates (`system.md`, `errors.md`, `events.md`)
6. Copy spec templates into `specs/templates/`
7. Copy slash command templates into `.claude/commands/{project}/`, replacing `{project}` placeholder
8. Create `.gitignore` from the `ductus` template, then fetch and append language-specific patterns from github.com/github/gitignore for each primary language
9. Create `README.md` from template, replacing `{project}` placeholder
10. Run `npx markdownlint-cli2` on all generated files
11. Display next steps: start a new Claude session in the project directory, run `/{project}:setup`, then fill in AGENTS.md and system.md

### What it does NOT do

- Fill in AGENTS.md sections (tech stack, code style, etc.) — that requires project-specific knowledge
- Write system.md content — that requires architectural decisions
- Create the first feature spec — the user does that via `/{project}:specify`
- Make any git commits — the user decides when to commit
- Run `/{project}:setup` — that runs in the new project's Claude session, not `ductus`'s

## Acceptance Criteria

### Standard pipeline commands

- [x] AC1: Every command template is copied to `.claude/commands/ductus/` with `{project}` replaced by this repo's namespace, `ductus`. The set was ten when this spec shipped and has grown since (see the Note above); `scripts/gen-claude-commands.sh` globs `framework/commands/`, so it tracks the set rather than a fixed list, and the criterion carries no count for the same reason
- [x] AC2: Commands reference `.ductus/session.toml` for session state, and `.govern.session.toml` survives only as the legacy fallback `target.md` still resolves
- [x] AC3: Pipeline gates enforce status transitions (draft → clarified → planned → done)
- [x] AC4: Dependency checks enforced via AGENTS.md boundary rule

### /ductus:init

> **Retired 2026-09-15.** AC5–AC17 are left ticked as the account of what shipped — each was true for the life of the command — but none states live behaviour, because the command no longer exists. AC1–AC4 above are unaffected: the ten dogfooded pipeline commands this spec also delivered still ship. This is partial retirement, so the spec stays and describes what remains, per [§spec-lifecycle](../../framework/constitution.md#spec-lifecycle).

- [x] AC5: Command exists at `.claude/commands/ductus/init.md` — **superseded 2026-09-15: the command was retired and the file deleted.** Annotated rather than swept because the claim stopped holding, which is not a rename
- [x] AC6: Command accepts project name, path, description, and primary language(s) as arguments or prompts for them
- [x] AC7: Verifies the target directory does not exist before proceeding
- [x] AC8: Creates a complete project directory with all `ductus` files
- [x] AC9: Replaces `{project}` placeholder in all copied templates
- [x] AC10: Copies slash command templates into `.claude/commands/{project}/`
- [x] AC11: Copies spec templates into `specs/templates/`
- [x] AC12: Creates system spec stubs from templates
- [x] AC13: Creates `.gitignore` from template and appends language-specific patterns fetched from github.com/github/gitignore
- [x] AC14: Runs markdownlint on generated files
- [x] AC15: Displays next steps directing the user to a new session and `/{project}:setup`
- [x] AC16: Does not make git commits automatically
- [x] AC17: Does not fill in project-specific content (AGENTS.md sections, system.md)

## Resolved Questions

- **Running setup automatically** — no. Init runs from the `ductus` repo; setup runs in the new project's Claude session. Tell the user to start a new session and run `/{project}:setup`. Promotes smaller, more accurate context per session.
- **Initial commit** — leave to the user. They may want to review generated files, fill in AGENTS.md, or make adjustments before committing.
- **Minimal flag** — not now. Templates are empty prompts that cost nothing to include. If a project doesn't use events, they delete `events.md`. See [specs/README.md](../README.md#future-considerations) for deferred rationale.
- **Existing directory** — verify it doesn't exist and stop if it does. Running in an existing directory risks overwriting files. The manual bootstrap steps in the README cover adding `ductus` to existing projects.
- **Command location** — `.claude/commands/ductus/init.md`, invoked as `/ductus:init`. Consistent with the slash command pattern from spec 000. **Reversed 2026-09-15:** the command was retired and the file deleted. The decision is kept rather than swept because it is the reference that records where init lived, which is the one survival [§drift-prevention](../../framework/constitution.md#drift-prevention) preserves for a retired name.
- **Language-specific gitignore** — init asks for primary language(s) and fetches patterns from github.com/github/gitignore to append to the minimal .gitignore template.

## References

Declared dependencies for this spec, surfaced here so the `derive-dependencies` runtime primitive sees them in the body.

- [000-slash-commands](../000-slash-commands/spec.md)
- [001-system-spec-templates](../001-system-spec-templates/spec.md)
- [002-project-scaffolding](../002-project-scaffolding/spec.md)
