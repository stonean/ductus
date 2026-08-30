---
description: Configure settings.local.json with permissions for slash commands.
---

# Configure

Configure `.claude/settings.local.json` with the permissions needed for slash commands to run without manual approval.

## Scope Boundaries

- Read and write only `.claude/settings.local.json`. Do NOT modify any other file.
- Add missing entries, remove exact-match duplicates from `permissions.allow` and `permissions.deny`, and remove the **retired** entries listed in step 4; do NOT reorder or rewrite any other non-duplicate entry the user (or another command) added beyond the canonical set listed below. Retirement is confined to that explicit list — entries this framework itself once shipped — so an entry an adopter authored is never touched, however closely it resembles one. The `merge-permissions` primitive performs the canonical-presence + dedup passes automatically; only `additionalDirectories` is handled outside the primitive (it has no duplication problem — entries are presence-checked, not deduped).
- Do NOT scan source code, specs, or git history. This command only manages permissions.
- Reference: no constitution sections apply — this command operates on agent-specific permission state, not `ductus` artifacts.

## Instructions

1. Invoke `merge-permissions` (MCP: `merge-permissions`) to install the canonical `permissions.allow` and `permissions.deny` sets into `.claude/settings.local.json`, dedup exact-match entries from both arrays, and retire the step-4 entries by passing them as `revoke`. The primitive creates the file if missing (with `{"permissions":{"allow":[],"deny":[]}}`), reads it otherwise, and writes atomically (tempfile + rename). It preserves untouched top-level keys and unspecified keys under `permissions` byte-for-byte; the action emitted is `created`, `updated`, or `unchanged` with per-array counts of entries added vs. duplicates removed. The `revoke` pass is allow-side only and runs before the dedup and canonical-presence passes, so every copy of a retired entry is removed and none is re-added; an entry named in both `allow` and `revoke` is rejected as a caller error rather than resolved by pass order. Otherwise (markdown-only path), the host walks the canonical sets below: read the file, remove every exact match for a step-4 retired entry from `permissions.allow`, ensure every canonical entry is present, remove exact-match duplicates from `permissions.allow` and `permissions.deny`, write atomically.

2. Canonical `permissions.allow` entries. **Only the bulleted entries below are canonical** — a pattern appearing in the surrounding prose is explanation, never an entry to install, and some of it names patterns that must *not* be added:

   **File operations:**
   - `Edit`
   - `Write`

   **Ductus state files (no per-write confirmation):**
   - `Edit(.ductus/session.toml)`
   - `Edit(.ductus/config.toml)`

   A path-scoped entry uses `Edit(path)`, never `Write(path)`. Claude Code matches file permissions against `Edit` rules only, and an `Edit` rule covers every file-editing tool including `Write` — so a path-scoped `Write` entry grants nothing its `Edit` sibling has not already granted, and the host warns about it at session start. The two that once sat here (`Write(.ductus/session.toml)`, `Write(.ductus/config.toml)`) were removed for that reason; do not re-add them, and reach for `Edit(path)` when scoping a new one. This is a different defect from the wildcard-position rule below — that shape grants too much, this one grants nothing — but both are the entry's *shape* being wrong. The bare `Edit` / `Write` entries under **File operations** are unaffected: they name tools rather than paths, and removing `Write` there would revoke a real tool grant.

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

   **Git commands targeting another working tree (`-C <path>`):** intentionally **none**. An allow pattern must never place its wildcard before the subcommand: a `Bash(git -C <wildcard> status <wildcard>)` shape lets that leading wildcard span inserted options, not just the path, so `git -C . -c core.pager='!sh -c "…"' status` matches and is approved with no prompt — and both `-c` and `--exec-path` run arbitrary commands. The seven `-C` variants that once lived here were removed for exactly the over-broadening reason `framework/bootstrap/configure/antigravity.md` gives for omitting them there. `git -C` invocations fall through to the host's normal Ask prompt; do not re-add them. The same hazard does not apply to the deny set below, where a wildcard that matches more only refuses more.

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
   - `mcp__ductus__check-corpus-links`
   - `mcp__ductus__check-orphaned-references`
   - `mcp__ductus__check-command-flags`
   - `mcp__ductus__check-review-agreement`
   - `mcp__ductus__derive-dependencies`
   - `mcp__ductus__derive-references`
   - `mcp__ductus__check-unfolded-specs`
   - `mcp__ductus__rewrite-spec-links`
   - `mcp__ductus__retire-feature`
   - `mcp__ductus__write-supersession-annotation`
   - `mcp__ductus__invalidate-review`
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

   The six `git -C *` patterns above keep their leading wildcard deliberately, and must not be narrowed to match the allow set: on the deny side a pattern that matches more refuses more, so the same shape that is a hole in an allow entry is a stronger guard here.

   **Other dangerous commands:**
   - `Bash(chmod -R 777 *)`
   - `Bash(> *)`

4. Retired `permissions.allow` entries — remove every one of these that is present. Passed to `merge-permissions` as `revoke`; on the markdown-only path, spliced out of the array by exact string match.

   **Wildcard before the subcommand** (dropped from the canonical set in ductus 0.33.0, spec 023 task 21; the leading `*` spans inserted options, and `-c` / `--exec-path` run arbitrary commands, so each of these approved arbitrary execution with no prompt):
   - `Bash(git -C * add *)`
   - `Bash(git -C * commit *)`
   - `Bash(git -C * push *)`
   - `Bash(git -C * log *)`
   - `Bash(git -C * diff *)`
   - `Bash(git -C * status *)`
   - `Bash(git -C * show *)`

   **Path-scoped `Write(...)`** (dropped from the canonical set alongside this list, spec 023 `configure-inert-write-path-entries`; file permission checks match only `Edit(path)` rules, so these granted nothing their `Edit(...)` siblings in step 2 had not already granted, and the host warned about each at session start):
   - `Write(.ductus/session.toml)`
   - `Write(.ductus/config.toml)`

   **Why this list lives here and not in a `/ductus` migration.** The registry in `framework/migrations.toml` is the framework's usual home for adopter-side cleanup, and it is the wrong mechanism for this one. Its `[migrations].last_applied` marker lives in the **committed** `.ductus/config.toml`, while this file is **per-contributor and gitignored** — so the first teammate to run `/ductus` would mark the entry applied for the whole repo and every other teammate would keep the retired entries forever. `/ductus` also never writes the full permission set (it seeds only the bootstrap entries and directs the operator here). This command has neither problem: it is the sole owner of this file, it runs per-contributor, and it is idempotent, so retirement simply happens on the next run for each person who runs it.

   **The list is append-only and must stay disjoint from steps 2–3.** An entry that appears in both the canonical set and this list would be removed and re-added on every run — `merge-permissions` rejects that outright, and `/ductus:audit` Family 32 catches it at maintainer time. Never move an entry here without deleting it from the canonical set in the same edit. Deny-side entries are never retired: an over-broad *denial* refuses more rather than approving more, so the `Bash(git -C * …)` patterns in step 3 are correct and must stay.

5. Ensure `permissions.additionalDirectories` contains (host-side; not handled by `merge-permissions` — this field has no duplication problem, entries are presence-checked):
   - The `specs/` directory (absolute path)
   - The `.claude/commands/ductus/` directory (absolute path)

   Read the file (post-`merge-permissions` write), add any missing absolute paths to `additionalDirectories`, and write atomically.

6. Confirm what was added, and report what was retired — name each removed entry, so the adopter sees the change to their permission file rather than finding it by diff.
