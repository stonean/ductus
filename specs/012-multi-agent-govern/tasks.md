---
title: "012-multi-agent-govern — tasks"
---

# 012 — Multi-Agent Ductus Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Move setup sources into per-agent directory

- [x] Create `commands/setup/` directory.
- [x] Move `commands/setup.md` → `commands/setup/claude.md` (no content change).
- [x] Verify `commands/setup.md` no longer exists.

Done when: `commands/setup/claude.md` exists with the previous file contents intact, and the old flat-named file is deleted.

## 2. Refresh `ductus`'s own setup command

- [x] Re-derive `.claude/commands/ductus/setup.md` from `commands/setup/claude.md` with `{cli-config-dir}` resolved to `.claude` and `{project}` resolved to `gov`.

Done when: `.claude/commands/ductus/setup.md` matches `commands/setup/claude.md` with placeholders resolved (Command File Parity satisfied).

## 3. Write the unified `ductus/ductus.md`

- [x] Replace `ductus/ductus.md` with the unified content:
  - Agent registry table near the top with the the per-agent fields (`key`, `name`, `config_dir`, `settings_template`, `rules_file_note`) and a row per supported agent.
  - Pre-flight checks (git repo, existing `specs/` warning) preserved from the existing file.
  - Agent selection logic with the precedence: `--agents=` → auto-detect → `--add-agent` / first-run prompt. Reject unknown agent keys before any scaffolding. Reject zero-agent selection in prompt paths.
  - Per-agent scaffolding loop that runs the manifest, writes the setup source from `commands/setup/{key}.md`, creates the empty session JSON, merges the agent's `settings_template` into `settings.local.json`, and installs a copy of `ductus.md` into `{config_dir}/commands/ductus.md`.
  - Shared-file scaffolding (constitution, templates, AGENTS.md, CLAUDE.md, .gitignore, specs/system.md, etc.) running once per `/ductus` invocation outside the loop.
  - Post-write integrity check that the installed `ductus.md` starts with `# Ductus`, with re-fetch on failure.
  - Post-scaffolding output covering created/updated/unchanged/skipped/pinned/merged files, the `rules_file_note` for each scaffolded agent, the self-update notice when the installed `ductus.md` was updated, first-run vs update-mode next steps, and migration guidance for users coming from 007's per-CLI files.

Done when: `ductus/ductus.md` exists in unified form, passes `npx markdownlint-cli2`, and never references the second per-CLI bootstrap file.

## 4. Delete the second per-CLI bootstrap file

- [x] Remove the per-CLI bootstrap file the unified `ductus/ductus.md` supersedes.
- [x] Confirm no other file in the repo references it (README, specs, commands).

Done when: the file is gone and the repo has no remaining references to it.

## 5. Update `README.md`

- [x] Under each per-agent curl snippet in the "Adopting in an Existing Project" section, add a single framing line stating that subsequent agents do not require a second curl — re-running `/ductus` with `--add-agent` adopts additional agents from the unified file.
- [x] Confirm the README still documents both supported agents' install paths.

Done when: the README's adoption section explains the multi-agent flow without changing the existing per-agent install instructions.

## 6. Remove "Ductus File Parity" from `CLAUDE.md`

- [x] Delete the "Ductus File Parity" section from `CLAUDE.md`.
- [x] Leave the "Command File Parity" section in place.

Done when: `CLAUDE.md` contains only the "Command File Parity" rule under "Governance Repo Rules" and passes `npx markdownlint-cli2`.

## 7. Add signpost to `specs/007-govern-workflow/spec.md`

- [x] Add a signpost paragraph below the existing 011 signpost noting that the multi-file design is superseded by 012, with a link to `../012-multi-agent-govern/spec.md`.
- [x] Leave 007's status at `done`.

Done when: 007's spec links to 012 as the successor, status remains `done`, and the file passes `npx markdownlint-cli2`.

## 8. Lint and verify

- [x] Run `npx markdownlint-cli2` over every modified or created file: `ductus/ductus.md`, `commands/setup/claude.md`, `.claude/commands/ductus/setup.md`, `README.md`, `CLAUDE.md`, `specs/007-govern-workflow/spec.md`, `specs/012-multi-agent-govern/spec.md`, `specs/012-multi-agent-govern/plan.md`, `specs/012-multi-agent-govern/data-model.md`, `specs/012-multi-agent-govern/tasks.md`.
- [x] Walk the spec's Acceptance Criteria list and confirm each is satisfied by the changes.

Done when: markdownlint reports zero errors and every acceptance criterion is verified against the implementation.

## 9. Promote spec to `done`

- [x] After the user confirms the acceptance criteria are met, set `specs/012-multi-agent-govern/spec.md` status to `done`.

Done when: the spec status is `done` and the change is committed.

## 10. Implement scenario: settings-template-bash-allowlist

- [x] Implement the behavior described in [`scenarios/settings-template-bash-allowlist.md`](scenarios/settings-template-bash-allowlist.md).

- **Done when**: The Claude row's `settings_template` in `framework/bootstrap/ductus.md` includes every Bash pattern a routine `/ductus` run invokes (audit pass complete); an end-to-end `/ductus` smoke test produces no Bash-permission prompts during routine bootstrap.

## 11. Decide whether the per-agent configure commands carry the host-integration pointer

- [ ] framework/bootstrap/configure/*.md — each configure command's Instructions invoke primitives (`configure/claude.md` step 1: "Invoke `merge-permissions`") without the constitution's §runtime-host-integration pointer, which the constitution has open every command's Instructions; `scripts/lint-tool-coverage.sh`, the only check of that rule, covers `framework/commands/*.md` alone and now says so (048's review, 2026-10-29). Decide whether the configure commands carry the pointer, and if so add it to each and bring them into the lint's scope with a test case

- **Done when**: each configure command either carries the pointer and is in the lint's scope, or 012's plan records why it is exempt

## 12. Real adopter bootstrap after the agent removal

- [ ] framework/bootstrap/ductus.md, install.sh and the migrations — the agent removal (chore: drop Auggie, Antigravity and OpenCode support) rewrote the registry, Permission Setup, MCP wiring, the Pi layout and three migrations, and no real run has exercised them; AGENTS.md asks a real adopter bootstrap of any framework/bootstrap/ or framework/migrations/ change, on a subject far enough behind to cross the whole chain. Bootstrap a scratch adopter from an older release on claude, then upgrade it with /ductus --ref=main, per docs/developing-ductus.md (Exercising an installer change without a push); and run a real pi against a pi-selected scratch adopter, since pi's loader has no other test. Run from a session on the pulled, rebuilt tree, never this repository's stale MCP server; budget a fix-and-rerun. Holds 012's done, and so the ductus-v0.60.0 tag (owner, 2026-11-11)

- **Done when**: both runs completed on main's bootstrap, each run's outcome is stated in the commit that closes this task, and every defect found is fixed or dispositioned
