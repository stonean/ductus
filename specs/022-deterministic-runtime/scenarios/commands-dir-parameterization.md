---
section: "Follow-on scenarios"
---

# Commands-dir parameterization

## Context

The runtime resolves command files at two callsites that both hardcode `.claude/commands/ductus/`:

- `runtime/src/main.rs:209-220` — `run_exec`'s candidate list for `ductus exec <command>`. The middle entry is `repo.join(".claude/commands/gov").join(format!("{command}.md"))`.
- `runtime/src/interpreter/payload.rs:378-390` — `locate_command_file`, used during anchor extraction (resolving `Reference: §<anchor>` lines under the `## Scope Boundaries` section of a command file). Same middle-entry shape.

Both paths bake in two assumptions that hold in this repo but break for adopters:

1. **The host's config dir is `.claude/`.** True for Claude Code, false for Auggie (`.augment/`) and any future host. The constitution already establishes `{cli-config-dir}` as a template variable for exactly this reason (the configure-permission flow uses it), but the runtime's lookup table doesn't honor that variable.
2. **The project's slash-command namespace is `gov/`.** True for this repo (the framework's own dogfooded slash commands live under `/ductus:*`), false for any adopter (`/acme:*` lives under `.claude/commands/acme/`, `/globex:*` under `.claude/commands/globex/`, etc.). The constitution establishes `{project}` as the namespace variable.

In this repo the bug is invisible because `framework/commands/<name>.md` (the first candidate) always wins the search — the source files are sitting right there. In an adopter project that has only run `/ductus` (so they have `.claude/commands/<project>/*.md` but no `framework/commands/` tree), the runtime never finds the command file and `ductus exec` errors out.

Surfaced 2026-05-24 during the `.ductus/session.toml` consolidation sweep on spec 022. Left out of scope at the time because the design space is non-obvious — multiple shapes are viable (env var, `.ductus/config.toml` config block, CLI flag, runtime config file written by `/ductus` at bootstrap) — and the bug doesn't bite in the framework repo itself.

## Behavior

The runtime needs to know two values at command-resolution time: the host's config-dir name (`{cli-config-dir}`) and the project's slash-command namespace (`{project}`). The same two values are already substituted into generated command files at install time, so the question is where the *runtime* reads them from at exec time.

### Design picks to evaluate

This scenario is the place to choose the source-of-truth shape during implementation. Three options, ordered by recommended preference:

**Option 1 — `.ductus/config.toml` config block (recommended).** The project root already carries `.ductus/config.toml` (per spec 022.40 the session file consolidated onto `.ductus/session.toml`, but `.ductus/config.toml` proper still owns project-level config like `pins`). Add a top-level block:

```toml
[host]
project = "ductus"
```

`/ductus`'s bootstrap writes the value at install time (it already knows it — it substitutes it into every generated file). The runtime reads it at exec time via a small loader. Adopters who hand-roll their `.ductus/config.toml` get a clear, version-controlled config surface. A missing value falls back to the directory name of the repo, preserving today's behavior in the source-of-truth repo. `cli-config-dir` is per-contributor and is not committed here — see `cli-config-dir-per-contributor`.

**Option 2 — environment variables.** `DUCTUS_CLI_CONFIG_DIR` and `DUCTUS_PROJECT`, honored by both callsites. Adopters set them in their shell profile or CI env. Lightest-touch implementation but invisible at the repo level — onboarding a new contributor requires knowing to set them, which violates "everything the runtime needs is in the repo."

**Option 3 — CLI flag (`ductus --cli-config-dir=.augment --project=acme exec ...`).** Maximally explicit but forces every caller (including `/ductus:*` slash command bodies) to pass both flags on every invocation. Heaviest ergonomic cost; rejected.

### Implementation shape (assuming Option 1)

1. Add `Host { cli_config_dir: String, project: String }` to the runtime's config loader. Default values: `cli_config_dir = ".claude"`, `project` = the repo's directory basename.
2. Both callsites call `Host::load`, which reads `project` from the resolved config file's `[host]` block; `cli_config_dir` comes from the per-contributor session file instead — see `cli-config-dir-per-contributor`.
3. Replace the hardcoded path strings in both callsites with `Host::command_file_candidates`, which tries three layouts in order: `{cli_config_dir}/commands/{project}/{name}.md` (claude-style), `{cli_config_dir}/command/{project}/{name}.md` (opencode), and `{cli_config_dir}/prompts/{project}-{name}.md` (pi).
4. `/ductus`'s bootstrap (`framework/bootstrap/ductus.md`) writes a `[host]` block carrying only `project` into the adopter's active config file during the install, as a managed block — re-runs update it in place rather than appending duplicate sections. `cli-config-dir` goes to the session file — see `cli-config-dir-per-contributor`.
5. Fixtures under `runtime/tests/fixtures/` exercise `ductus exec` against non-Claude layouts: `exec-auggie` (`.augment`, `project = "acme"`, with `cli-config-dir` in its legacy `[host]` block) is the regression test for this scenario, and `exec-opencode` resolves the singular `command/` directory with `cli-config-dir` in its session file.

### Markdown-only path

The runtime's parity contract requires the markdown-only fallback to keep working. The hardcoded path is a runtime-only concern — the markdown-only walk reads command files via the LLM, which already follows the natural file-system layout of the adopter's project. No change required on the markdown-only side.

## Edge Cases

- **`.ductus/config.toml` missing or `[host]` block absent.** `project` defaults to the repo directory basename, and `cli_config_dir`, named neither in the session file nor in `[host]`, defaults to `.claude`.
- **`[host]` values disagree with the actual on-disk layout.** The lookup misses and the existing "command file not found" error fires, listing every candidate path tried (so the user can see the mismatch). No silent fallback to legacy hardcoded paths — fail loud.
- **An adopter migrates from one host to another (Claude → Auggie or vice versa).** `cli-config-dir` is per-contributor and lives in the gitignored session file — see `cli-config-dir-per-contributor`.
- **Project name contains characters that aren't filesystem-safe.** Out of scope here — `{project}` is already constrained by the bootstrap to lowercase ASCII per the existing template-substitution contract. The runtime can trust it.
- **The framework's source repo.** Its `.ductus/config.toml` sets `[host] project = "ductus"`, and the first candidate (`framework/commands/<name>.md`) wins every lookup there, so the installed candidates are exercised by the fixtures rather than by this repository.

## Open Questions

*None — captured during scenario authoring; design picks surfaced as options for the implementation choice.*

## Resolved Questions

**Source-of-truth shape — `.ductus/config.toml` `[host]` block (Option 1).** Resolved 2026-05-24 during task 41 implementation. The runtime loads `project` from a `[host]` block in `.ductus/config.toml`, defaulting to the repo directory basename when the block is missing. `/ductus`'s bootstrap writes the block idempotently. `cli_config_dir` comes from the per-contributor session file instead — see `cli-config-dir-per-contributor`. Option 2 (env vars) was rejected as invisible at the repo level; Option 3 (per-invocation CLI flags) was rejected for ergonomic cost.
