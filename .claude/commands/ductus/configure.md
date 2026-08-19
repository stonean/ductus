---
description: Configure settings.local.json with permissions for slash commands.
---

# Configure

Configure `.claude/settings.local.json` with the permissions needed for slash commands to run without manual approval.

## Scope Boundaries

- Read and write only `.claude/settings.local.json`. Do NOT modify any other file.
- Add missing entries and remove exact-match duplicates from `permissions.allow` and `permissions.deny`; do NOT reorder or rewrite non-duplicate entries the user (or another command) added beyond the canonical set listed below. The `merge-permissions` primitive performs the canonical-presence + dedup passes automatically; only `additionalDirectories` is handled outside the primitive (it has no duplication problem — entries are presence-checked, not deduped).
- Do NOT scan source code, specs, or git history. This command only manages permissions.
- Reference: no constitution sections apply — this command operates on agent-specific permission state, not `ductus` artifacts.

## Instructions

1. Invoke `merge-permissions` (MCP: `merge-permissions`) to install the canonical `permissions.allow` and `permissions.deny` sets into `.claude/settings.local.json` and dedup exact-match entries from both arrays. The primitive creates the file if missing (with `{"permissions":{"allow":[],"deny":[]}}`), reads it otherwise, and writes atomically (tempfile + rename). It preserves untouched top-level keys and unspecified keys under `permissions` byte-for-byte; the action emitted is `created`, `updated`, or `unchanged` with per-array counts of entries added vs. duplicates removed. Otherwise (markdown-only path), the host walks the canonical sets below: read the file, ensure every canonical entry is present, remove exact-match duplicates from `permissions.allow` and `permissions.deny`, write atomically.

2. Canonical `permissions.allow` entries:

   **File operations:**
   - `Edit`
   - `Write`

   **Ductus state files (no per-write confirmation):**
   - `Edit(.ductus/session.toml)`
   - `Write(.ductus/session.toml)`
   - `Edit(.ductus/config.toml)`
   - `Write(.ductus/config.toml)`

   **Web access:**
   - `WebFetch`
   - `WebSearch`

   **Bash commands (read-only shell operations):**
   - `Bash(ls *)`

   File-content parsers (`awk`, `grep`, `cat`, `head`, `for` loops over files) are intentionally **not** in the canonical set. The runtime primitives and the host's `Read` / `Grep` / `Glob` tools cover those reads on the deterministic and markdown-only paths respectively; shell pipelines are not a sanctioned third path. See `framework/constitution.md` §runtime-boundary.

   **Git commands:**
   - `Bash(git add *)`
   - `Bash(git commit *)`
   - `Bash(git push *)`
   - `Bash(git log *)`
   - `Bash(git diff *)`
   - `Bash(git status *)`
   - `Bash(git show *)`

   **Git commands targeting another working tree (`-C <path>`):**
   - `Bash(git -C * add *)`
   - `Bash(git -C * commit *)`
   - `Bash(git -C * push *)`
   - `Bash(git -C * log *)`
   - `Bash(git -C * diff *)`
   - `Bash(git -C * status *)`
   - `Bash(git -C * show *)`

   **Utility:**
   - `Bash(curl *)`
   - `Bash(gh api *)`
   - `Bash(mkdir -p *)`
   - `Bash(chmod +x *)`
   - `Bash(command -v *)`

   **Build / lint:**
   - `Bash(make *)`
   - `Bash(markdownlint *)`
   - `Bash(markdownlint-cli2 *)`
   - `Bash(npx markdownlint-cli2 *)`

   **Hooks and generators (ductus's pre-commit pipeline):**
   - `Bash(git config core.hooksPath *)`
   - `Bash(git config --get core.hooksPath)`
   - `Bash(git config --unset core.hooksPath)`
   - `Bash(./.githooks/pre-commit)`
   - `Bash(scripts/install-hooks.sh)`
   - `Bash(./scripts/install-hooks.sh)`

   **Runtime MCP tools (`mcp__ductus__*` — generated from `framework/runtime-tools.txt`):**

   <!-- generated:mcp-allow:start -->
   - `mcp__ductus__read-spec`
   - `mcp__ductus__read-tasks`
   - `mcp__ductus__mark-task`
   - `mcp__ductus__mark-criterion`
   - `mcp__ductus__set-status`
   - `mcp__ductus__derive-boundary`
   - `mcp__ductus__discover-rule-files`
   - `mcp__ductus__process-waivers`
   - `mcp__ductus__compute-review-scope`
   - `mcp__ductus__write-review`
   - `mcp__ductus__check-stuck`
   - `mcp__ductus__validate-frontmatter`
   - `mcp__ductus__resolve-anchor`
   - `mcp__ductus__resolve-references`
   - `mcp__ductus__traverse-deps`
   - `mcp__ductus__check-rule-ids`
   - `mcp__ductus__run-generator`
   - `mcp__ductus__lint-markdown`
   - `mcp__ductus__gate-confirm`
   - `mcp__ductus__fetch-archive`
   - `mcp__ductus__extract-archive`
   - `mcp__ductus__apply-manifest`
   - `mcp__ductus__enforce-manifest`
   - `mcp__ductus__merge-managed-block`
   - `mcp__ductus__merge-permissions`
   - `mcp__ductus__migrate-session-file`
   - `mcp__ductus__create-scenario`
   - `mcp__ductus__append-task`
   - `mcp__ductus__label-criteria`
   - `mcp__ductus__prune-tasks`
   - `mcp__ductus__dashboard`
   - `mcp__ductus__write-session`
   - `mcp__ductus__resolve-feature`
   - `mcp__ductus__create-feature`
   - `mcp__ductus__create-plan-artifacts`
   - `mcp__ductus__check-review-gate`
   - `mcp__ductus__append-question`
   - `mcp__ductus__diff-cross-spec`
   - `mcp__ductus__append-inbox`
   - `mcp__ductus__remove-inbox-item`
   - `mcp__ductus__check-artifacts`
   - `mcp__ductus__derive-routing-candidates`
   - `mcp__ductus__check-orphaned-references`
   - `mcp__ductus__derive-dependencies`
   - `mcp__ductus__derive-references`
   <!-- generated:mcp-allow:end -->

3. Canonical `permissions.deny` entries:

   **Destructive file operations:**
   - `Bash(rm -rf *)`
   - `Bash(rm -r *)`
   - `Bash(rm -fr *)`
   - `Bash(*rm -rf *)`
   - `Bash(*rm -r *)`
   - `Bash(*rm -fr *)`

   **Dangerous git operations:**
   - `Bash(git mv *)`
   - `Bash(git push --force *)`
   - `Bash(git push -f *)`
   - `Bash(git reset --hard *)`
   - `Bash(git rm *)`
   - `Bash(git clean -fd *)`
   - `Bash(git -C * mv *)`
   - `Bash(git -C * push --force *)`
   - `Bash(git -C * push -f *)`
   - `Bash(git -C * reset --hard *)`
   - `Bash(git -C * rm *)`
   - `Bash(git -C * clean -fd *)`

   **Other dangerous commands:**
   - `Bash(chmod -R 777 *)`
   - `Bash(> *)`

4. Ensure `permissions.additionalDirectories` contains (host-side; not handled by `merge-permissions` — this field has no duplication problem, entries are presence-checked):
   - The `specs/` directory (absolute path)
   - The `.claude/commands/ductus/` directory (absolute path)

   Read the file (post-`merge-permissions` write), add any missing absolute paths to `additionalDirectories`, and write atomically.

5. Confirm what was added.
