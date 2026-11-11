---
title: "003-bootstrap-automation — tasks"
---

# 003 — Bootstrap Automation Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Create standard pipeline commands

- [x] Create `.claude/commands/ductus/` directory
- [x] Copy all ten command templates from `commands/` into `.claude/commands/ductus/`
- [x] Replace every `{project}` with `gov` in all ten files
- [x] Verify commands reference `.ductus/session.toml` for session state (was `.claude/gov-session.json` pre-0.10.0)

Done when: all ten `/ductus:*` commands exist, all `{project}` placeholders are replaced with `gov`, and no template placeholders remain.

## 2. Create /ductus:init command

- [x] Write `.claude/commands/ductus/init.md` with instructions for: collecting inputs (project name, path, description, primary languages), pre-flight directory check, scaffolding steps 1–11 from the spec
- [x] Include gitignore language fetch from `https://raw.githubusercontent.com/github/gitignore/main/{Language}.gitignore`
- [x] Include next-steps output directing user to new session, `/{project}:setup`, and AGENTS.md/system.md

Done when: `/ductus:init` exists with all scaffolding steps, input collection, pre-flight check, and next-steps display.

## 3. Final review and lint

- [x] Run `npx markdownlint-cli2` on all files in `.claude/commands/ductus/`
- [x] Verify no `{project}` placeholders remain in standard commands (should all be `gov`)
- [x] Verify init command uses `{project}` only where it refers to the new project being scaffolded
- [x] Spot-check a few commands against their `commands/` templates to confirm accurate derivation
- [x] Update spec status to `planned`

Done when: all eleven commands pass lint, placeholders are correct, and spec status is `planned`.

## 4. Installer script (`install.sh`)

- [x] `install.sh` resolves the agent (positional arg, default `claude`) and places the bootstrap at the 012-registry path for claude
- [x] The fetch is tempfile-guarded (`mktemp` + `EXIT` trap) and idempotent
- [x] README Quick start and per-agent installs reduced to a single `curl … | sh` line each
- [x] Verified end to end against the live `ductus.md` for each agent
- [x] Resolve the installer↔registry parity open question — enforced by `scripts/audit/installer-registry-parity.sh` (audit Family 14)

Done when: the one-line `curl … | sh` installer places the bootstrap for every supported agent per `scenarios/curl-sh-installer.md`, and the README installs are reduced to one line per agent.

## 5. Implement scenario: a-language-with-no-template-of-its-name

- [ ] Resolve the scenario's open question with `/ductus:clarify`
- [ ] Implement the behavior described in `scenarios/a-language-with-no-template-of-its-name.md`
- [ ] `framework/bootstrap/ductus.md`'s `.gitignore` merge step 5, and its byte copy `framework/bootstrap/govern.md`, carry the decided rule and name each unappended language in the closing summary
- [ ] A real adopter bootstrap naming TypeScript shows the decided outcome

- **Done when**: the scenario's open question is resolved, the step follows it, and a real adopter bootstrap naming TypeScript ends as the scenario states.

## 6. Disposition: nothing installs the project README template since init's retirement

- [ ] `framework/templates/project/project-readme.md` reaches no adopter: `/ductus:init`, which copied it, was retired on 2026-09-15 (this spec's Retired note), and `/ductus` has no manifest row for it, while `framework/bootstrap/ductus-procedure.md`'s What This Command Does NOT Do says it never modifies `README.md`. Only audit Family 1 (`scripts/audit/cross-doc-consistency.sh`, sub-check 1b) reads it, to keep its pipeline diagram consistent. Found by 002's analysis, 2026-11-11; the owner deferred the decision here, since this spec retired `init`
- [ ] Put the choice to the owner: `/ductus` creates `README.md` from the template when none exists, a `create` row in Shared files with conflict handling of `framework/bootstrap/ductus.md` and its byte copy `framework/bootstrap/govern.md`; or the template is retired, with 002's README criteria annotated and Family 1's sub-check 1b, `scripts/audit/README.md`, 026's spec and 023's AC28 swept
- [ ] Correct this repository's `AGENTS.md`, whose Project Structure lists `project-readme` among the templates consumed once at adoption, with the change the owner chooses

- **Done when**: the template is installed by `/ductus` or retired, every artifact that names it says which, and `AGENTS.md`'s template list matches
