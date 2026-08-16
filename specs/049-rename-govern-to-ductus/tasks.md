# 049 — Rename govern to ductus Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Phase A lands the runtime first: it must read both the old and the new per-project directory
before this repo's own `.govern/` moves, or the binary loses its config mid-rename. Phase B
sweeps the framework, Phase C the specs, Phase D this repo's own dogfooded state, and Phase E
verifies the whole against the audit families. The crates.io half of AC12 lands with the
release, not here.

## Phase A — Runtime

### 1. Add the third resolution tier for the per-project directory

- [x] In `runtime/src/schema/paths.rs`, add `.ductus/config.toml` and `.ductus/session.toml` as
      the primary constants; demote the `.govern/` pair to a middle tier and keep the
      `.govern.toml` / `.govern.session.toml` root pair as the oldest
- [x] Generalize `active_path` from a `(new, legacy)` pair to an ordered slice, first-existing
      wins, defaulting to the primary when none exists
- [x] Update `config_path`, `config_display_name`, `resolve_config`, `session_path`, and
      `session_path_for_write` to resolve through the chain
- [x] Extend the existing resolver tests to cover all three tiers and every precedence pair,
      including both-legacy-present

- **Done when**: `cargo test` passes and the resolver returns the newest existing tier for
  reads, the active tier for writes, and the primary for a fresh project.

### 2. Rename the crate, binary, library, and MCP server key

- [x] `runtime/Cargo.toml`: `name`, `[[bin]] name`, `[lib] name`, `description`, `repository`
- [x] `runtime/src/host.rs`: `FALLBACK_PROJECT` → `ductus`
- [x] Sweep `runtime/src/**` for the server key, doc comments, error strings, and the
      `PRIMITIVE_NAMES` / `TOOL_NAMES` surrounding prose — leaving bare `<verb>-<noun>`
      primitive names untouched, since they never carried the project name
- [x] Rebuild and confirm the binary is `runtime/target/release/ductus`

- **Done when**: `cargo build --release` produces `ductus`, `cargo test` passes, and no file
  under `runtime/src/` names the old binary or server key except a legacy path constant.

### 3. Re-bless the goldens and update the fixtures

- [x] Rename the new-layout fixture files under `runtime/tests/fixtures/**` to the new
      per-project directory; leave every legacy-layout fixture
      (`.govern.session.toml`, `.govern.toml`) exactly as it is
- [x] Re-bless all 9 goldens with `BLESS=1 cargo test --test parity` — do not hand-edit them
- [x] Confirm `scripts/audit/fixture-session-shape.sh` still passes, since it asserts on the
      legacy fixture filenames

- **Done when**: `cargo test` is green, the golden diff contains only name and path changes,
  and the legacy-layout fixtures are byte-identical to before.

## Phase B — Framework

### 4. Rename the bootstrap and repoint every adopter-facing URL

- [x] `git mv framework/bootstrap/govern.md framework/bootstrap/ductus.md`
- [x] Update the three adopter-facing URLs — the archive fetch, the self-update fetch (which
      must name the *new* bootstrap path), and the post-scaffolding documentation links
- [x] Sweep the bootstrap body: MCP registration shapes, the per-agent permission seeds in all
      four grammars, the pre-flight probe, the Agent Registry's derived-values table, and the
      Shared Files manifest rows
- [x] Walk the §Derived values **Layout-derived** table row by row per `AGENTS.md`'s
      agent-registry rule and record the per-agent impact in the commit message

- **Done when**: the bootstrap names only the new project, the self-update fetch points at the
  new path, and each of the four agent grammars has been checked against its own matcher
  rather than inferred from another's.

### 5. Register the adopter migration

- [x] Add a `framework/migrations.toml` entry with `introduced_in` set to the release carrying
      this work, and `target_paths` covering the per-project directory, the installed command
      files, and the MCP config
- [x] Write `framework/migrations/ductus-rename.md`: rewrite the MCP server key and command,
      rewrite the permission entries in all four grammars, move the per-project directory under
      042's convergence rule (`git mv` when tracked, converge when the destination exists),
      rewrite `[pinned] files` entries, warn on a pinned invoker still naming the old path, and
      reinstall the command files under the new namespace
- [x] Make it idempotent and silent when the project is already converged
- [x] Leave `framework/migrations/govern-dir-consolidate.md` unchanged

- **Done when**: the entry validates against the registry's duplicate-id and
  reference-integrity guard, a pre-rename project converges in one run, and a re-run is a
  no-op.

### 6. Sweep the framework sources

- [x] `framework/constitution.md`, `framework/commands/*.md`, `framework/templates/**`,
      `framework/rules/**`, `framework/runtime-tools.txt` surroundings
- [x] Keep placeholders as placeholders — `{project}` and `{cli-config-dir}`, never the
      substituted literals, per `AGENTS.md`'s generator boundary
- [x] Leave every version- and tag-adjacent occurrence untouched

- **Done when**: `scripts/audit/placeholder-roundtrip.sh` passes and no framework source names
  the old project outside a legacy path or a published-version reference.

### 7. Sweep the scripts and add the retired-name guard entries

- [x] `scripts/gen-claude-commands.sh` (`PROJECT`), `scripts/gen-configure-mcp.sh` (the four
      permission-string emitters), `scripts/gen-help-tables.sh`
- [x] `scripts/audit/*.sh` hardcoded names, including `host-namespace-parity.sh`'s contract
      assertion against `paths.rs`, `manifest-parity.sh`'s permission-prefix greps,
      `runtime-hardcoded-paths.sh`'s command-dir grep, and `check-zero.sh`'s generator path
- [x] Add the rename's tokens to `introducing-drift.sh`'s `RENAMED_TOKENS` catalog
- [x] `scripts/lint-procedure-parseability.sh`'s runtime binary path

- **Done when**: every audit and lint script runs against the new names and
  `scripts/audit/run-all.sh` reaches the specs sweep with no name-related finding.

### 8. Update the workflows and the release tag scheme

- [x] `.github/workflows/runtime-release.yml`: trigger pattern `gvrn-v*` → `ductus-v*`, asset
      base names, and the crates.io publish step
- [x] `.github/workflows/runtime.yml`, `generators.yml`, `markdown-only-pipeline.yml`
- [x] Give every job that runs `cargo` under `runtime/` the `components: clippy, rustfmt` input
      per `AGENTS.md`'s toolchain-pin gotcha, and keep `fetch-depth: 0` on history-reading jobs
- [x] Leave the existing `gvrn-v*` tags and their assets alone

- **Done when**: the release workflow triggers on the new tag scheme, publishes the new asset
  names, and no workflow references the old binary.

### 9. Rewrite the project-level documents

- [x] `README.md`: acquiring, registering, and invoking under the new name only
- [x] `AGENTS.md`: the runtime-release entry's tag scheme and three-artifact bump, the
      stale-binary gotcha's binary path, the generator gotcha's command directory
- [x] `CLAUDE.md` import paths
- [x] Add the contributor-local checklist for a maintainer renaming their own checkout — local
      directory, git remote, and per-project agent state keyed by path (AC10)

- **Done when**: `README.md` and the bootstrap describe only the new name, and the
  contributor-local checklist is documented somewhere a maintainer will find it.

## Phase C — Specs

### 10. Sweep the spec corpus

- [x] Apply the substitution table across `specs/NNN-*/**`, excluding every occurrence that
      names a published version, tag, or asset
- [x] Rename spec directories whose slug carries the old project name only if their slug is
      part of the live artifact set — otherwise leave the slug and sweep the body
- [x] Add the file-scope `<!-- audit:ignore-introducing-drift:file -->` marker to this spec
- [x] Confirm every change in the diff is the same substitution, so the `done` specs it touches
      stay `done` per §spec-lifecycle case (a)

- **Done when**: `/ductus:analyze` reports no spec drifted by the sweep, no `done` spec changed
  status, and the published-artifact occurrences the spec counted are still present.

### 11. Record the runtime behavior change on 022

- [x] Open the `done → in-progress` back-edge on `specs/022-deterministic-runtime/`
- [x] Add a scenario for the three-tier resolution chain, back-linking to this spec per
      §cross-spec-impact
- [x] Update `specs/022-deterministic-runtime/data-model.md` with the resolution order
- [x] Add the matching task to 022's `tasks.md` so the scenario→task mapping family stays clean

- **Done when**: 022 carries the scenario, its data-model states the resolution order, and the
  scenario→task mapping check passes.

## Phase D — This repository's own state

### 12. Move this repo onto the new layout

- [x] `git mv .govern .ductus`, and correct the stale basename comment in its `config.toml`
      while leaving `[host] project` explicitly pinned
- [x] Update `.mcp.json`'s server key and command
- [x] Regenerate the command copies with `scripts/gen-claude-commands.sh` and remove the old
      `.claude/commands/gov/` directory — regenerate, never hand-move
- [x] Update `.gitignore`'s framework-managed block for the new session path

- **Done when**: `git status` is clean after a regeneration, the commands live under the new
  namespace, and an MCP tool call in this repo resolves through the new server key.

## Phase E — Verification

### 13. Prove the sweep against the audit families

- [x] `scripts/audit/run-all.sh` exits 0 with no family disabled or exempted — specifically the
      installer-command, installer-registry, manifest, and host-namespace parity families (AC7)
- [x] `cargo test` green, including the re-blessed parity goldens (AC8)
- [x] `npx markdownlint-cli2` clean
- [x] Grep the live-artifact set for the old tokens and confirm every surviving occurrence is a
      legacy path constant, a published-version reference, or a historical migration body (AC1,
      AC6)
- [x] Exercise the migration against a scratch copy of a pre-rename project and a
      pre-042 project, confirming both converge and a re-run is a no-op (AC3, AC4, AC11)

- **Done when**: the full self-audit and test suite pass under the new name, and the migration
  converges both adopter populations idempotently.
