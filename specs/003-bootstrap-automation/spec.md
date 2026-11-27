---
title: "003-bootstrap-automation — spec"
status: done
dependencies: [000-slash-commands, 001-system-spec-templates, 002-project-scaffolding]
tags: [bootstrap, commands]
next-criterion: 18
---

# 003 — Bootstrap Automation

`ductus` slash commands that dogfood the same pipeline commands adopting projects use, installed in `.claude/commands/ductus/` from the sources in `framework/commands/`.

## Problem

When this spec was written, the `ductus` README described a 7-step manual bootstrap process: init git, copy files, fill in AGENTS.md, create CLAUDE.md, create specs directory, add first spec, follow the pipeline. That was error-prone and tedious. A slash command can automate the mechanical steps while prompting for the decisions that require human input.

Additionally, the `ductus` project itself had no slash commands. Agents working on `ductus` specs operated without pipeline enforcement — no status gates, no dependency checks, no structured workflow. Governance should dogfood the same commands it provides to adopting projects.

## Behavior

### Standard pipeline commands

Install every command source in `framework/commands/` into `.claude/commands/ductus/`, replacing `{project}` with `ductus`. `scripts/gen-claude-commands.sh` does it, and the pre-commit hook runs it. This gives `ductus` the same slash commands as any adopting project, and they enforce the same pipeline gates, dependency checks (via `traverse-deps`), and conventions that adopting projects follow.

## Acceptance Criteria

### Standard pipeline commands

- [x] AC1: Every command source in `framework/commands/` is installed at `.claude/commands/ductus/` with `{project}` replaced by this repo's namespace, `ductus`. `scripts/gen-claude-commands.sh` globs `framework/commands/`, so it tracks the set rather than a fixed list, and the criterion carries no count for the same reason
- [x] AC2: Commands reference `.ductus/session.toml` for session state, and `.govern.session.toml` survives only as the legacy fallback `target.md` still resolves
- [x] AC3: Pipeline gates enforce status transitions (draft → clarified → planned → in-progress → done, constitution §spec-lifecycle)
- [x] AC4: Dependency readiness is checked by `traverse-deps` at `/ductus:clarify` step 5 and `/ductus:analyze` step 3

## Resolved Questions

- **Is a greenfield `/ductus:init` kept?** No. It was retired on 2026-09-15 and its command file deleted, because an orphan no sweep reaches drifts on every rename with nothing able to report it, and `/ductus` already covers the greenfield case that was its last remaining role (`framework/bootstrap/ductus.md` prompts for the spec root, ships the `inbox.md` and `rules/` manifest rows, and resolves every `specs/…` destination under the configured name). The tech-stack questionnaire that `004-tech-stack-selection` delivered into init's input collection went with it, and is not replaced, by decision rather than oversight: `/ductus` collects **Primary language(s)** and carries none of the questionnaire, and its `framework/bootstrap/ductus-procedure.md` §Post-Scaffolding Output tells the operator to fill in `AGENTS.md` by hand. That is out of scope for the retirement because init was never shipped to adopters (it had no **Shared Files** manifest row), so no adopter ever had the questionnaire and none loses it; the loss falls only on the maintainer scaffolding a brand-new project locally, which is the role the retirement decision weighed and accepted. Rule-file selection is unaffected: `discover-rule-files` reads `[rules] surfaces`, which `/ductus` collects explicitly, not the Tech Stack table. What does still read that table is `/{project}:review`'s tech-stack alignment check (`framework/reference/review.md` §Resolve target and scope, which reads `AGENTS.md`'s `Tech Stack` section), and it read a hand-filled table for every `/ductus`-adopted project already.

## References

Declared dependencies for this spec, surfaced here so the `derive-dependencies` runtime primitive sees them in the body.

- [000-slash-commands](../000-slash-commands/spec.md)
- [001-system-spec-templates](../001-system-spec-templates/spec.md)
- [002-project-scaffolding](../002-project-scaffolding/spec.md)
