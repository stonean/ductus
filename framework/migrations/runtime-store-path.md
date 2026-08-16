# runtime-store-path

**Introduced in:** ductus 0.28.0
**Summary:** Rewrite an MCP registration that names the bare `ductus` command to the ductus-owned path — the repo-relative pointer for a project-committed config, the absolute store path for a home-level one.

## Background

Before spec 048, the runtime was installed out of band onto `PATH` and registered by its bare name, so the resolved binary was whatever the shell found first — nothing owned it and nothing kept it at the version the framework pinned. `/ductus` now acquires the pinned release into a ductus-owned store (`~/.ductus/bin/ductus`) and reaches it from each project through a gitignored pointer (`.ductus/bin/ductus`).

An existing adopter's MCP config still names the bare command. This migration repoints it. The binary on their `PATH` is left exactly where it is — removing it is the adopter's call, not `/ductus`'s, and nothing here consults `PATH` again.

Composes with `ductus-rename` (same release, and ordered after it by the lexicographic tie-break on equal `introduced_in`): that migration renames the server **key** and the command **name**, this one rewrites the command **path**. An adopter arriving from before either gets both in one run — key and name first, then the path.

## Procedure

This migration has no runtime primitive; it is an in-place config rewrite the host performs directly. It runs under the batch migration consent (the §Pre-run Migrations "apply N migrations?" prompt) — there is **no** additional per-file prompt.

1. **Idempotency check.** For each of the project-committed MCP targets that exists — `.mcp.json` (Claude) and the root `opencode.json` / `opencode.jsonc` (OpenCode) — read the registered `ductus` server entry. If no target file exists, or no file registers a `ductus` server, or every registration already names a path rather than the bare command, exit silently.

2. **Rewrite the project-committed registrations.** For each target whose `ductus` entry names the bare command:
   - `.mcp.json` — set `mcpServers.ductus.command` to `.ductus/bin/ductus`, preserving `args` and every other server and top-level key byte-for-byte.
   - `opencode.json` — set the first element of `mcp.ductus.command` to `.ductus/bin/ductus`, preserving the remaining elements, `type`, `enabled`, `$schema`, `permission`, and every other key.

   A target that is **not valid JSON** is left untouched, with one warning line naming it — a hand-maintained config is never clobbered:

   `warning: {file} is not valid JSON; leaving it unmodified — repoint its ductus server command to .ductus/bin/ductus by hand.`

3. **Surface the home-level registrations.** Auggie and Antigravity read MCP servers from a file in the user's home directory, which `/ductus` must not write. When the adopter has one of these agents configured, emit one line naming the command to run or the edit to make. These name the **absolute store path**, not the pointer: the file is per-machine and shared across every project, so no project-relative path can be correct in it.

   - Auggie: `warning: re-register the ductus MCP server against the store: auggie mcp add ductus --command ~/.ductus/bin/ductus --args "mcp" — then restart the agent.`
   - Antigravity: `warning: update ~/.gemini/config/mcp_config.json to run ~/.ductus/bin/ductus, then reload with /mcp.`

4. **Leave `PATH` alone.** Do not remove, rename, or warn about a `ductus` binary elsewhere on the adopter's `PATH`. It is theirs.

5. **Summary line.** When at least one registration was rewritten, report `repointed ductus MCP registration → .ductus/bin/ductus: {comma-separated files}` in the post-scaffolding output. Omit the line entirely when nothing changed.

## Notes

- The migration is one-way. There is no reverse path.
- It rewrites the **registration only**. The store and the pointer are materialized by the same `/ductus` run's pre-flight acquisition, not here — so a project whose config this migration repoints has a pointer to resolve by the time the next session starts.
- A project that sets `[runtime] path` keeps its own binary: acquisition resolves the pointer to that path instead of the store, and this migration's rewrite still points the config at the pointer, which is the indirection that makes both cases identical to the MCP client.
