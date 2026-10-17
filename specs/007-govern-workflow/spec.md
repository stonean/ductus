---
title: "007-govern-workflow — spec"
status: done
dependencies: [003-bootstrap-automation]
tags: [bootstrap, commands]
next-criterion: 15
---

# 007 — Ductus Workflow

> **Note:** [011-brownfield-process](../011-brownfield-process/spec.md) renamed this spec from `007-adopt-workflow` to `007-govern-workflow` and gave the ductus command two brownfield additions: a triage → inbox migration step, and `/capture` in its command manifest for initializing a spec from an existing feature. Both have since gone. The migration step was removed in the April 2026 command reorganization (`8558ac8b`), and [023](../023-govern-refinement/spec.md) folded `/capture` into `/specify`.

A slash command that adopts `ductus` in an existing (brownfield) project and keeps an adopted project current on every re-run. The user installs one file, the bootstrap, into their agent's command directory and runs it — no clone of the `ductus` repo is required. The command fetches the framework from GitHub, writes each file to its location under a per-file conflict strategy, substitutes placeholders, and displays brownfield-specific next steps.

The command supports every AI coding CLI in its Agent Registry. Each agent gets its native directory paths and configuration formats — no backward-compatibility shims.

`framework/bootstrap/ductus.md` is the canonical procedure, with the sections that run after the framework archive is fetched in `framework/bootstrap/ductus-procedure.md`. This spec states the command's contract; the procedure owns the steps, the manifest tables, and the messages.

## Distribution Model

> **Superseded in part by [012-multi-agent-govern](../012-multi-agent-govern/spec.md).** 007 shipped one bootstrap file per CLI — one for Claude Code, one for Auggie — with the target CLI implicit in which file the user installed. 012 replaced them with the single registry-driven installer described here, which selects agents at run time. The `{cli-config-dir}` placeholder approach is 007's and still holds. AC1, AC3 and AC14 record what the per-CLI model delivered and what replaced it.

The deliverable is one markdown file, `framework/bootstrap/ductus.md`, serving every supported agent (`012-multi-agent-govern`). It carries an **Agent Registry** with one row per agent, and each agent's paths are derived from its row's `layout` (`028-antigravity-agent`).

The user installs it with `install.sh`, which is attached to every release and places the bootstrap at the agent's install path (`003-bootstrap-automation`):

```bash
curl --proto '=https' --tlsv1.2 -sSfL https://github.com/stonean/ductus/releases/latest/download/install.sh | sh -s -- <agent>
```

Omitting `<agent>` installs for Claude Code. Every `/ductus` run then installs a copy of the bootstrap into each agent it scaffolds, so the command stays invokable from all of them.

The "program" is a prompt. Its deterministic steps — the archive fetch and extract, the manifest passes, the managed-block merges — are runtime primitives, and `/ductus` acquires the runtime itself during adoption (`048-govern-acquired-runtime`).

## Inputs

The command needs three project inputs:

1. **Project name** — lowercase, alphanumeric, hyphens allowed. Used for `{project}` placeholder substitution and command directory naming.
2. **Project description** — one-line description for `AGENTS.md`.
3. **Primary language(s)** — comma-separated list for `.gitignore` language patterns.

It also asks which rule surfaces to install and what to name the spec-root directory. The project name may come from `$ARGUMENTS`. Anything else missing is read back from `.ductus/config.toml`, where an earlier run persisted it, and only what is still missing is prompted for — after the pre-flight phase passes, so a pre-flight abort never discards answers the user just typed (**Collect Project Inputs** in `framework/bootstrap/ductus.md`).

The agents to scaffold are chosen at run time: an explicit `--agents=` list, else every agent whose config directory already exists, else a prompt (`012-multi-agent-govern`).

## Pre-flight Checks

Before scaffolding, verify:

- The current directory **is** an existing git repository. If not, stop and report: "This is not a git repository. Run `git init` first."
- Whether the spec-root directory (`specs/` by default) already exists. If it does, `ductus` is already adopted and the run is an update: report "Existing {spec-root}/ directory found — running in update mode." and proceed, letting each file's conflict strategy decide what is rewritten.

A pre-flight phase then resolves the fetch source, acquires and wires the runtime when it is absent, and compares the installed bootstrap with upstream (scenario `ductus-self-update-precheck`), all before the framework archive is fetched.

## File Fetching

> **Signpost:** [061 — Updates track the latest release tag](../061-updates-track-the-latest-release-tag/spec.md) made every fetch back into the `ductus` repository — the bootstrap self-update, the runtime version pin, and the framework archive — name one source resolved per run.

Every file the command installs from the `ductus` repo comes from one framework archive, fetched once per run and extracted into a fresh temp directory (`015-tarball-fetch`). The archive is taken at the source the run resolved: the latest release by default, or `main` or a named release tag through `--ref`. Per-language `.gitignore` patterns are the exception — each is its own fetch from GitHub's gitignore collection.

The command carries a manifest of the files it installs. Each entry names:

- a source path, relative to the `ductus` repo root
- a destination path, relative to the project root
- a conflict strategy: `update` (overwrite with the framework's copy when it differs), `create` (write only when absent), `skip` (never overwrite), or `merge` (rewrite only a delimited managed region)

A destination listed in `.ductus/config.toml` `[pinned] files` is left alone instead of updated.

If the archive cannot be fetched or extracted, the run aborts before scaffolding: every entry would be missing, so no partial scaffold is possible. If one entry's source is absent from the archive, the command warns and continues with the rest — a single missing file never aborts the run.

## Scaffolding Behavior

The manifest tables in `framework/bootstrap/ductus.md` — **Shared Files** and **Per-Agent Scaffolding** — are the canonical list of what is installed where. This section states the contract they implement.

### Shared files

Written once per run, whatever agents are selected:

- **Framework-owned files** (strategy: `update`) — the constitution, installed at `.ductus/constitution.md`; the rule files under `specs/rules/`, filtered by the configured rule surfaces; the spec templates under `specs/templates/`; the markdown-lint config; and the shipped pre-commit hook. Each is refreshed from the framework on every run unless the project pins it.
- **Project documents** (strategy: `create`) — `specs/system.md`, `specs/errors.md`, `specs/events.md`, `specs/inbox.md`, and the project's own `.githooks/pre-commit`. Each is written once from its template, then left to the project.

### Per-agent files

For each selected agent, at paths derived from its `layout`:

- the bootstrap's own permission entries, merged into the agent's settings file without disturbing existing entries
- the slash commands, in the agent's native command form, with `{project}` and `{cli-config-dir}` resolved
- the agent's `configure` command, from its own permission source in `framework/bootstrap/configure/`
- a copy of the bootstrap itself, so `/ductus` can be invoked from that agent on later runs

Slash command templates use a `{cli-config-dir}` placeholder for agent-specific paths, resolved to the agent's config directory during copy. Session state is not per agent: it lives at `.ductus/session.toml` for every adopter.

### Files with conflict handling

- **AGENTS.md** (strategy: `skip`) — if it exists, leave it alone. If not, write it from the project template, substituting the project name and description.
- **CLAUDE.md** (strategy: `skip`) — written only when a selected agent reads `CLAUDE.md` natively (the `claude-style` layout), and only when the file does not exist. An adoption whose agents all read `AGENTS.md` gets no `CLAUDE.md`.
- **.gitignore** (strategy: `merge`) — the framework's patterns live in a managed block opened by a `# ductus` line. A run creates the file if it is missing, rewrites only that block, and removes any copy of a managed pattern found outside it. Language patterns are appended once per language below a `# {Language}` header, and are the project's to edit afterwards.

### Placeholder substitution

In every copied file, replace:

- `{project}` and `{project-name}` with the project name
- `{One-line project description.}` with the project description
- `{cli-config-dir}` with the agent's config directory

The exception is each agent's installed copy of the bootstrap, which keeps every placeholder literal: it reads them on each run, and its prose documents them for the other files.

### What the command does NOT do

The canonical list is **What This Command Does NOT Do** in `framework/bootstrap/ductus-procedure.md`. The command does not:

- Modify `README.md` — the project's README is its own; `ductus` doesn't touch it
- Create feature specs — the user does that via `/{project}:specify`
- Fill in AGENTS.md content — that requires project-specific knowledge
- Fill in system.md content — that requires architectural decisions
- Make git commits — the user decides when to commit
- Write findings to `specs/inbox.md` — its brownfield security audit reports gaps and writes nothing
- Run `/{project}:configure` — that happens after adoption, interactively
- Delete an adopted agent's tree — that is manual cleanup

## Post-Scaffolding Output

After scaffolding, the command displays (**Post-Scaffolding Output** in `framework/bootstrap/ductus-procedure.md`):

- the source the files came from
- a summary of files created, updated, unchanged, skipped, pinned, merged, and removed, grouped by agent
- any fetch failures encountered
- next steps, which differ between a first run and an update

On a first run the next steps are brownfield-specific, each command written in the adopted agent's own invocation form:

1. Run `configure` in each adopted agent to apply the full permission set
2. Fill in `AGENTS.md` — tech stack, project structure, code style, testing conventions, gotchas
3. Fill in `specs/system.md` — architecture, request lifecycle, shared infrastructure
4. Use `log` to record known issues and bugs in `specs/inbox.md`
5. Run `groom` to route each inbox item to its spec or scenario
6. Create the first feature spec with `specify`
7. Nothing to install for the runtime — this run acquired and wired it

## Self-Maintenance

The installed bootstrap stays in each agent's command directory after the run, and re-running it is how an adopted project is updated. Each run first compares the installed copy with the upstream bootstrap at the resolved source. A stale copy is replaced and the run stops for a session restart, so it never proceeds on the instructions it has just replaced (scenario `ductus-self-update-precheck`). A project that pins its installed copy keeps it, and is told when upstream has diverged.

## Edge Cases

- **No network access** — the pre-flight phase cannot resolve the source or fetch the bootstrap, and halts the run with an error before the archive is fetched, so no framework file is written. Only the additive permission entries seeded earlier in the run remain.
- **Partial previous run** — idempotency handles it: `create` skips files that exist, `update` rewrites only what differs, and the `.gitignore` managed block is rewritten in place rather than appended again.
- **`.gitignore` merge dedup** — the managed block is found by its `# ductus` line and replaced, never appended twice, and a managed pattern copied outside the block is removed.
- **A source file missing from the archive** — the command warns and continues with the remaining entries.
- **Empty `$ARGUMENTS`** — no project name provided. The command prompts for each missing input once the pre-flight phase has passed, unless an earlier run persisted it.
- **Invalid project name** — uppercase, spaces, or special characters. The command rejects with a clear error: "Project name must be lowercase, alphanumeric, and hyphens only."
- **Command directory doesn't exist** — the agent's command directory (e.g., `.claude/commands/`) may not exist yet. The command creates intermediate directories as needed.

## Acceptance Criteria

- [x] AC1: One bootstrap variant exists per supported CLI, in a top-level bootstrap directory. Delivered against the per-CLI bootstrap files of the time. **Superseded twice over:** `012-multi-agent-govern` replaced the per-CLI variants with a single registry-driven installer, so there is no longer one file per CLI; and the repository was later reorganized, so the top-level directory is gone — the installer is `framework/bootstrap/ductus.md`. Annotated rather than rewritten because the claim was superseded, not renamed: restating it as the single-file model would credit 007 with 012's work. `009`'s AC18 carries the same annotation for the same supersession
- [x] AC2: Running `curl` followed by `/ductus {name}` in an existing git repo produces a complete `ductus` scaffold
- [x] AC3: Each CLI variant scaffolds into its native directory paths (`.claude/` for Claude Code, `.augment/` for Auggie). **The requirement holds; the enumeration is superseded.** Native-path scaffolding is still exactly what the installer does, and `{cli-config-dir}` is still how it does it. But there are no longer "CLI variants" (012, per AC1), and the registry in `framework/bootstrap/ductus.md` now carries more agents than the two named here. Left as written because the two it names are still correct and the criterion asserts no exclusivity; the registry is the live enumeration
- [x] AC4: Existing files (.gitignore, AGENTS.md, CLAUDE.md) are not overwritten
- [x] AC5: `.gitignore` merge is idempotent — running twice does not duplicate the managed patterns
- [x] AC6: Fetch failures for individual files do not abort the entire process
- [x] AC7: All generated files pass `npx markdownlint-cli2`
- [x] AC8: Slash commands are installed in the CLI's native command directory with `{project}` and `{cli-config-dir}` placeholders resolved
- [x] AC9: `specs/inbox.md` is created as the brownfield entry point
- [x] AC10: The command is idempotent — safe to run again without duplicating content
- [x] AC11: Post-scaffolding output displays brownfield-specific next steps
- [x] AC12: Invalid project names are rejected with a clear error message
- [x] AC13: Intermediate directories are created as needed
- [x] AC14: Adding a new CLI requires only a new ductus variant file — no changes to the framework core. Delivered as written, and the *intent* survives: adding an agent still leaves the constitution, templates and command sources untouched. **The mechanism is superseded.** 012 replaced the variant file with a row in the Agent Registry, so adding an agent means a registry row plus a walk of the §Derived values **Layout-derived** table — command path, invocation, install path, settings file, permission shape, native rule-loading dir, native rules file, cleanup glob. That is more than "only a new file", and `AGENTS.md` §Workflow records that nothing audits per-layout behaviour parity, so the walk is contributor discipline rather than a gate

## Open Questions

None — all resolved.

## Resolved Questions

- ~~Should the command also fetch `sdd-context.md`?~~ No — framework-internal only.
- ~~Should there be a `--dry-run` mode?~~ No — the command is idempotent with create/skip/merge strategies, making dry-run unnecessary.
- ~~How should the file manifest be maintained?~~ Hardcoded in each ductus variant. Updated when the framework templates change. Since 012 there is one variant, so the manifest is the single installer's **Shared Files** and **Per-Agent Scaffolding** tables.
- ~~CLI-specific command variants or path variable?~~ Path variable (`{cli-config-dir}`) resolved at ductus time. One set of command templates, with the installer doing the substitution — once one installer per CLI, now one for every agent (012).
- ~~How should `/ductus:setup` work for Auggie?~~ Skipped here — Auggie permissions are global. Auggie later got a setup source of its own, which spec `012-multi-agent-govern` lists (named, not linked: 012 depends on this spec, so a link would close a cycle).

## References

Declared dependencies for this spec, surfaced here so the `derive-dependencies` runtime primitive sees them in the body.

- [003-bootstrap-automation](../003-bootstrap-automation/spec.md)
