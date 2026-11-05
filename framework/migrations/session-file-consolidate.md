# session-file-consolidate

**Introduced in:** gvrn 0.10.0
**Summary:** Consolidate the session state file from `{config_dir}/{project}-session.json` (host- and project-name-specific) onto `.ductus/session.toml` at the repo root.

## Background

Pre-0.10.0 the session lived at `{cli-config-dir}/{project}-session.json`, e.g. `.claude/gov-session.json` for an adopter using Claude Code with project `gov`, `.claude/acme-session.json` for one using Claude with project `acme`, or `.augment/acme-session.json` for the same adopter on Auggie. The path baked in the AI CLI's config directory and the adopter's project name, which broke whenever those didn't match the runtime's hardcoded constant (`.claude/gov-session.json`).

The consolidation moves the file to `.ductus/session.toml` at the repo root: gitignored, host-agnostic, project-name-agnostic, and uniform across every adopter. TOML replaces JSON to align with `.govern.toml`'s on-disk format. Keys are kebab-case (`scenario-path`, `set-at`) rather than the legacy camelCase (`scenarioPath`, `setAt`).

## Procedure

> **For agent runtimes**: the backticked primitive name `migrate-session-file` in this section maps to the MCP tool `mcp__ductus__migrate-session-file` (Claude) or `mcp:ductus:migrate-session-file` (Auggie). When the `ductus` server is registered, **call the tool** for each legacy session file; when it is not, run `.ductus/bin/ductus migrate-session-file` per file.

1. **Locate candidate legacy files.** Iterate every selected agent's `config_dir`. The legacy filename per agent is `{config_dir}/{project}-session.json` after the bootstrap's placeholder substitution (e.g., `.claude/gov-session.json` on Claude with project `gov`, `.augment/acme-session.json` on Auggie with project `acme`).

2. **For each candidate path, invoke `migrate-session-file`** with the candidate path as the `legacy-path` argument. The primitive:

   - Returns `action: "no-legacy"` and exits silently when the candidate file is absent (idempotency — adopters who never used `/{project}:target` or who already migrated have nothing to do).
   - Returns `action: "migrated"` when a fresh translation lands at `.ductus/session.toml`. Applies the kebab-case key renames (`scenarioPath` → `scenario-path`, `setAt` → `set-at`) and preserves every other top-level key intact (handles adopters with non-standard usage like walker-context-seed fields). Deletes the legacy file via tempfile + rename atomic-write semantics on the target.
   - Returns `action: "kept-existing"` when `.ductus/session.toml` already exists at the repo root (the adopter has already re-targeted post-consolidation). The new file is left untouched; the legacy file is still deleted so it doesn't confuse future readers.

   The runtime resolves the target destination from its own `write-session::SESSION_FILE` constant (always `.ductus/session.toml`), so the migration cannot drift from the runtime's read path.

3. **Summary line.** Report `migrated session state: {legacy-paths-joined-with-commas} → .ductus/session.toml` in the post-scaffolding output. When every result was `no-legacy`, omit the line entirely (nothing to report).

## Notes

- The migration is one-way. There is no reverse path; adopters who pin to a pre-0.10.0 framework version and re-run a newer `/ductus` will not have a legacy file recreated.
- The migration does not regenerate the gitignore entry for `.ductus/session.toml` — that ships via the framework-managed gitignore block which the `apply-manifest` step rewrites on every `/ductus` run.
- Cross-module consistency between the migration destination and the runtime's read/write path is enforced at compile time by the runtime's unit tests (`runtime/src/primitives/migrate_session_file.rs::target_dest_matches_write_session_constant`) and at audit time by `scripts/audit/consolidation-pair.sh`.
