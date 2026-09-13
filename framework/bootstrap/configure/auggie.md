---
description: Configure settings.local.json with permissions for slash commands.
---

# Configure

Configure `{cli-config-dir}/settings.local.json` with the tool permissions needed for slash commands to run without manual approval.

## Instructions

> The `merge-permissions` runtime primitive currently targets the Claude permission shape (`permissions.allow` / `permissions.deny` as string arrays). Auggie's `toolPermissions` array of `{toolName, shellInputRegex, permission}` objects is structurally different and is NOT yet served by a deterministic primitive — whether `merge-permissions` grows a format argument or whether a separate Auggie-format primitive is introduced is a plan-phase decision tracked as an open question on the `framework-list-dedup` scenario (`specs/022-deterministic-runtime/scenarios/framework-list-dedup.md`). Until that lands, Auggie callers walk the prose below: install the canonical set, remove exact-match duplicates by host-side splice.

1. Read `{cli-config-dir}/settings.local.json` (create it if missing, with `{"toolPermissions":[]}`).
2. Ensure the `toolPermissions` array contains **all** of the following entries AND that no exact-match duplicate of an entry (matched on `toolName` + `shellInputRegex` when present) survives the run. Add any canonical entries that are missing; remove duplicates so that each `(toolName, shellInputRegex)` pair appears at most once. First-occurrence wins; later duplicates are removed in place. Do not reorder or rewrite non-duplicate entries beyond the canonical set listed below.

   **File operations:**
   - `{ "toolName": "str-replace-editor", "permission": { "type": "allow" } }`
   - `{ "toolName": "save-file", "permission": { "type": "allow" } }`
   - `{ "toolName": "remove-files", "permission": { "type": "deny" } }`

   **Search and read:**
   - `{ "toolName": "view", "permission": { "type": "allow" } }`
   - `{ "toolName": "grep-search", "permission": { "type": "allow" } }`
   - `{ "toolName": "codebase-retrieval", "permission": { "type": "allow" } }`

   **Web access:**
   - `{ "toolName": "web-fetch", "permission": { "type": "allow" } }`
   - `{ "toolName": "web-search", "permission": { "type": "allow" } }`

   **Shell commands — read-only operations:**
   - `{ "toolName": "launch-process", "shellInputRegex": "^ls ", "permission": { "type": "allow" } }`

   File-content parsers (`awk`, `grep`, `cat`, `head`, `for` loops over files) are intentionally **not** in the canonical set, matching `configure/claude.md`. The runtime primitives and Auggie's own `view` / `grep-search` / `codebase-retrieval` tools cover those reads on the deterministic and markdown-only paths respectively; shell pipelines are not a sanctioned third path. See `framework/constitution.md` §runtime-boundary. The five that once sat here are retired in step 4.

   **Shell commands — git:**
   - `{ "toolName": "launch-process", "shellInputRegex": "^git add ", "permission": { "type": "allow" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^git commit ", "permission": { "type": "allow" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^git push ", "permission": { "type": "allow" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^git log", "permission": { "type": "allow" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^git diff", "permission": { "type": "allow" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^git status", "permission": { "type": "allow" } }`

   **Shell commands — utility:**
   - `{ "toolName": "launch-process", "shellInputRegex": "^curl ", "permission": { "type": "allow" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^gh api ", "permission": { "type": "allow" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^mkdir -p ", "permission": { "type": "allow" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^chmod \\+x ", "permission": { "type": "allow" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^command -v ", "permission": { "type": "allow" } }`

   **Shell commands — build / lint:**
   - `{ "toolName": "launch-process", "shellInputRegex": "^make", "permission": { "type": "allow" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^markdownlint", "permission": { "type": "allow" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^npx markdownlint-cli2", "permission": { "type": "allow" } }`

   **Shell commands — hooks and generators:**
   - `{ "toolName": "launch-process", "shellInputRegex": "^git config core\\.hooksPath", "permission": { "type": "allow" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^git config --(get|unset) core\\.hooksPath", "permission": { "type": "allow" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^\\./.githooks/pre-commit", "permission": { "type": "allow" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^\\./?scripts/install-hooks\\.sh", "permission": { "type": "allow" } }`

   **Runtime MCP tools (`mcp:ductus:*` — generated from `framework/runtime-tools.txt`):**

   <!-- generated:mcp-allow:start -->
   - `{ "toolName": "mcp:ductus:read-spec", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:read-tasks", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:mark-task", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:mark-criterion", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:set-status", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:derive-boundary", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:discover-rule-files", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:process-waivers", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:compute-review-scope", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:write-review", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:write-analysis", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:check-step-references", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:check-stuck", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:validate-frontmatter", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:resolve-anchor", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:resolve-references", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:resolve-constitutions", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:traverse-deps", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:check-rule-ids", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:run-generator", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:lint-markdown", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:gate-confirm", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:fetch-archive", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:extract-archive", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:apply-manifest", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:enforce-manifest", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:merge-managed-block", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:merge-permissions", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:migrate-session-file", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:create-scenario", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:append-task", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:label-criteria", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:prune-tasks", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:dashboard", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:write-session", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:resolve-feature", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:create-feature", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:create-plan-artifacts", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:check-review-gate", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:append-question", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:diff-cross-spec", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:append-inbox", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:remove-inbox-item", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:check-artifacts", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:derive-routing-candidates", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:check-corpus-links", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:check-orphaned-references", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:check-command-flags", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:check-review-agreement", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:derive-dependencies", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:derive-references", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:check-unfolded-specs", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:rewrite-spec-links", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:retire-feature", "permission": { "type": "allow" } }`
   - `{ "toolName": "mcp:ductus:invalidate-review", "permission": { "type": "allow" } }`
   <!-- generated:mcp-allow:end -->

   **Shell commands — denied (destructive):**
   - `{ "toolName": "launch-process", "shellInputRegex": "rm -rf ", "permission": { "type": "deny" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "rm -r ", "permission": { "type": "deny" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "rm -fr ", "permission": { "type": "deny" } }`

   **Shell commands — denied (dangerous git):**
   - `{ "toolName": "launch-process", "shellInputRegex": "^git mv ", "permission": { "type": "deny" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^git push --force", "permission": { "type": "deny" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^git push -f ", "permission": { "type": "deny" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^git reset --hard", "permission": { "type": "deny" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^git rm ", "permission": { "type": "deny" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^git clean -fd", "permission": { "type": "deny" } }`

   **Shell commands — denied (other dangerous):**
   - `{ "toolName": "launch-process", "shellInputRegex": "^chmod -R 777 ", "permission": { "type": "deny" } }`
   - `{ "toolName": "launch-process", "shellInputRegex": " > ", "permission": { "type": "deny" } }`

3. **Ordering:** deny entries must appear before allow entries in the `toolPermissions` array so that destructive commands are blocked even if a broader allow rule would match. When adding entries, insert deny entries at the top and allow entries after them.

4. **Retired `toolPermissions` entries** — remove every one of these that is present, matched on `toolName` + `shellInputRegex` and ignoring the `permission` value, so an entry a contributor flipped to `deny` or `ask` is still recognized as the retired one rather than left behind.

   **File-content parsers** (dropped from the canonical set in the same edit that added this list; the runtime primitives and Auggie's own read tools cover these reads, and §runtime-boundary principle 3 names shell pipelines as **not** a sanctioned substitute for either). `configure/claude.md` had excluded them for that reason while this file still granted them — the constitution's rule was enforced for one agent and not the other, which is the per-layout gap `AGENTS.md` §Workflow records as unaudited:
   - `{ "toolName": "launch-process", "shellInputRegex": "^head " }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^cat " }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^awk " }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^grep " }`
   - `{ "toolName": "launch-process", "shellInputRegex": "^for " }`

   Retirement is confined to this explicit list — entries this framework itself once shipped — so an entry an adopter authored is never touched, however closely it resembles one.

   **Why this list lives here and not in a `/ductus` migration**, and **why it is append-only and must stay disjoint from step 2**: both reasons are stated once in `configure/claude.md` step 4 and hold verbatim here — the migration registry's `last_applied` marker lives in the committed config file while this one is per-contributor and gitignored, so the first teammate to run `/ductus` would mark it applied for everyone; and an entry appearing in both the canonical set and this list would be removed and re-added on every run. Deny-side entries are never retired: an over-broad denial refuses more rather than approving more.

   `merge-permissions` does not serve Auggie's `toolPermissions` shape yet (see the note above step 1), so this removal is a host-side splice, the same way step 2's canonical-presence and dedup passes are.

5. Write the updated file, confirm what was added, and report what was retired — name each removed entry, so the contributor sees the change to their permission file rather than finding it by diff.
