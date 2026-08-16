# ductus-rename

**Introduced in:** ductus 0.28.0
**Summary:** Converge a project named for the retired `govern` / `gvrn` identifiers onto `ductus` — the per-project directory, the MCP server registration, the agent permission entries, the installed bootstrap entry point, the pre-commit hook and its invoker, the constitution references the previous hop wrote into the adopter-owned seed files, and the two managed-block markers.

## Background

The project was renamed (spec 049). The in-repo sweep reaches everything the framework ships; it cannot reach the state that lives in an adopter's own checkout — their MCP registration, their per-agent permission entries, their `.govern/` directory, their installed `govern` command file, and the `# govern` markers delimiting the blocks `/ductus` rewrites on every run. This migration converges all of it in one run.

The runtime reads `.ductus/` first and falls back to `.govern/` and then the pre-042 root files, so an adopter who upgrades the binary before re-running the bootstrap is never broken; this migration completes the cutover rather than performing it.

**What does *not* change.** The slash-command namespace is `host.project`, written from the adopter's own `project.name` — `/anvil:specify` stays `/anvil:specify`. Only the bootstrap entry point carries the project's own name, so only it is renamed here. An adopter whose `project` happens to be `gov` or `govern` is the sole case where the namespace itself moves, and that is handled in step 7.

## Procedure

This migration has no runtime primitive; it is file moves and in-place rewrites the host performs directly. It runs under the batch migration consent (the §Pre-run Migrations "apply N migrations?" prompt) — there is **no** additional per-file prompt.

Registry ordering by `introduced_in` places it after `govern-dir-consolidate` (0.22.0) and `constitution-relocate` (0.24.0), both of which land their files under `.govern/`. Those migrations are the historical record of where things went at the time; this one completes the second hop. An adopter arriving from any prior layout therefore converges in a single run: pre-042 root files move to `.govern/` and then to `.ductus/`, a 042-era project moves once, and an already-converged project matches no step below.

**Convergence rule (applies to every move).** Because the runtime reads `.ductus/` first, a lingering `.govern/` file after a write has moved is *dead* — the new file wins on read. So each move **converges** rather than skip-and-leaves: when the destination already exists, compare it to the source — if identical, delete the source silently; if they differ, delete the source and emit one line `warning: {source} diverged from {destination}; removed the stale copy ({destination} wins; the old content was already ignored — recover from git if needed).` A stale file is never left in place. When the destination does not exist, move via `git mv` when the source is tracked (so the rename is recorded) or `mv` otherwise, preserving any adopter customization.

1. **Idempotency check.** Look for any of these:
   - a `.govern/` directory
   - an MCP registration naming the `gvrn` server key or the bare `gvrn` command
   - a permission entry naming `mcp__gvrn__`, `mcp:gvrn:`, `mcp(gvrn/`, or `"gvrn*"`
   - an installed bootstrap entry point named `govern` (per layout, see step 6)
   - a `.githooks/govern-pre-commit` file, or a `.githooks/pre-commit` that invokes it
   - a `# govern` line in `.gitignore`, a `# govern (host)` line in the active config, or a `.govern/constitution.md` reference in `CLAUDE.md` / `AGENTS.md` / `README.md`

   If none is present, exit silently — the project is already converged.

2. **Move the per-project directory.** If `.govern/` exists, move each file under it to the matching path under `.ductus/` (creating `.ductus/`) under the convergence rule, then remove `.govern/` if the move left it empty. This carries `config.toml`, `session.toml`, `constitution.md`, and `scripts/` together; anything else an adopter placed under `.govern/` moves with them rather than being stranded.

3. **Re-point the seed references the previous hop wrote.** `constitution-relocate` (0.24.0) pointed the three adopter-owned seed files at `.govern/constitution.md`; step 2 just moved that file again. Those files are strategy `create` — never overwritten by the manifest — so a migration is the only thing that can follow the move, and **following steps 2 and 4 of `constitution-relocate` literally, then step 2 here, leaves a dangling reference.** For each file below that exists, rewrite a `.govern/constitution.md` reference to `.ductus/constitution.md`; a file whose reference is absent or hand-altered is left alone and named in one line — `warning: {file} does not carry the expected .govern/constitution.md reference; update it to .ductus/constitution.md by hand — the constitution has moved.` A wholly absent file is silently skipped.
   - `CLAUDE.md` — the `@import .govern/constitution.md` line.
   - `AGENTS.md` — the inline link and the artifact-list mention.
   - `README.md` — the Documentation link and the pipeline link.

   An adopter arriving from **before** `constitution-relocate` gets the same end state by a different route: that migration runs first in the same batch and writes `.govern/constitution.md`, which this step then advances. An adopter already on `.ductus/` matches nothing here.

4. **Re-point pins.** In `.ductus/config.toml` `[pinned] files`, rewrite any entry beginning `.govern/` to the matching `.ductus/` path, so a customized (pinned) file stays both discoverable at the new location and protected from overwrite.

5. **Pinned-invoker warning.** A pinned file opts out of updates, so this migration does **not** rewrite one. For each file in `[pinned] files` that still contains a `.govern/` reference, emit one line: `warning: pinned {file} still references .govern/; update it to .ductus/ — the directory has moved.` This makes the breakage visible rather than silent.

6. **Rename the installed bootstrap entry point.** Per the adopter's layout, under the convergence rule:
   - `claude-style` — `{config_dir}/commands/govern.md` → `{config_dir}/commands/ductus.md`
   - `opencode` — `{config_dir}/command/govern.md` → `{config_dir}/command/ductus.md`
   - `antigravity` — `{config_dir}/skills/govern/SKILL.md` → `{config_dir}/skills/ductus/SKILL.md`, renaming the containing directory and rewriting the frontmatter `name:` to `ductus`

   Apply this independently for every agent the project has configured — an adopter may carry more than one.

7. **Rename the command namespace only when it named the project.** If `[host] project` is `gov` or `govern`, rewrite it to `ductus` and move the installed command directory to match (`{config_dir}/commands/{project}/` → `{config_dir}/commands/ductus/`, and the layout equivalents). Any other value is the adopter's own project name and is left untouched. Report the namespace change in the summary when it fires, since every documented invocation the adopter has written down changes with it.

8. **Rename the pre-commit hook and re-point its invoker.** The manifest ships `.githooks/ductus-pre-commit` (strategy `update`), so after the rename an adopter carrying the old `.githooks/govern-pre-commit` has **both**, and the one git actually runs is whichever `.githooks/pre-commit` names. That file is strategy `create` — adopter-owned, never overwritten — so nothing else in the run touches it.

   Under the convergence rule, move `.githooks/govern-pre-commit` to `.githooks/ductus-pre-commit`; the manifest's `update` pass rewrites the destination's contents later in the same run, so the move exists to remove the orphan rather than to preserve it. Then, in `.githooks/pre-commit`, rewrite an invocation of the old name to the new one (`./.githooks/govern-pre-commit` → `./.githooks/ductus-pre-commit`, matching a bare or `./`-prefixed form). When `pre-commit` exists but names neither hook — an adopter who rewrote it — leave it and emit one line: `warning: .githooks/pre-commit does not invoke .githooks/govern-pre-commit; point it at .githooks/ductus-pre-commit by hand — the hook has been renamed.`

   Skipping this is **not** cosmetic and does not fail loudly. The old hook body invokes the generators by their pre-042 root paths (`scripts/gen-spec-deps.sh`), which `govern-dir-consolidate` moved to `.govern/scripts/` and step 2 above moved again to `.ductus/scripts/` — so an adopter who runs the full chain is left with a `pre-commit` calling a script whose own targets no longer exist. Observed on a real adopter bootstrap (spec 048's `papur`): the hook was broken by the migration chain that was supposed to converge it.

9. **Rewrite the MCP registration.** In each agent's MCP target, rename the server key `gvrn` to `ductus` and rewrite its `command` from `gvrn` to `ductus`, preserving `args` and every other server entry byte-for-byte:
   - `claude` — `.mcp.json` (repo root), `mcpServers` object
   - `opencode` — `opencode.json` (repo root), `mcp` block

   For `auggie` and `antigravity` the MCP config lives outside the repo (`~/.augment/settings.json`, `~/.gemini/config/mcp_config.json`), which this migration must not silently mutate. Emit one line instead, naming the file and the command to run: `warning: {file} still registers the gvrn MCP server; re-register it as ductus (auggie: auggie mcp add ductus --command ductus --args "mcp") and restart the agent.`

10. **Rewrite the permission entries.** In each agent's settings file, rewrite the runtime tool-permission entries in that agent's own grammar, leaving every other entry untouched:
    - `claude` — `mcp__gvrn__<tool>` → `mcp__ductus__<tool>` in `permissions.allow`
    - `auggie` — `"toolName": "mcp:gvrn:<tool>"` → `"mcp:ductus:<tool>"` in `toolPermissions`
    - `antigravity` — `mcp(gvrn/*)` → `mcp(ductus/*)` in `permissions.allow`
    - `opencode` — the `"gvrn*"` key → `"ductus*"` in the `permission` map, preserving its position so no later broad rule shadows it

11. **Rewrite the managed-block markers.** Both markers delimit regions `/ductus` rewrites on every run, and `merge-managed-block` keyed to a new marker would **append a second block** rather than update the old one, leaving the adopter with two:
    - `.gitignore` — replace the `# govern` line with `# ductus`
    - the active config file — replace the `# govern (host)` line with `# ductus (host)`

    Replace the marker line only; leave the block contents to the scaffolding pass that follows.

12. **Summary line.** When anything changed, report `renamed govern → ductus: {comma-separated list of what changed}` in the post-scaffolding output. Omit the line entirely when nothing changed.

## Notes

- The migration is one-way. There is no reverse path.
- Step 11 is the step whose omission is silent: nothing errors when a marker is missed, the next `/ductus` run simply appends a second managed block and the adopter's `.gitignore` grows a duplicate region on every run afterward.
- An adopter who has not yet re-run the bootstrap keeps working: the retired repository name redirects, the old bootstrap fetches the new one through it, and the runtime's `.govern/` fallback covers the window between the binary upgrade and this migration.
- **The retired bootstrap path is load-bearing, not courtesy.** Every installed pre-rename `/govern` has `framework/bootstrap/govern.md` hardcoded in its self-update fetch, and that bootstrap aborts the run when the fetch fails — before §Pre-run Migrations, so this migration would never execute. GitHub resolves the retired *repository* name on `raw.githubusercontent.com` and `codeload.github.com` (verified: 200, no redirect hop), but nothing resolves a retired *file* path. The old path therefore stays, byte-identical to the current bootstrap, guarded by `/{project}:audit` Family 21.
- **One narrow exception, and it fails loudly.** An adopter who has **pinned** their bootstrap file suppresses the self-update, so their run continues past the pre-flight abort and reaches the archive fetch — which their pre-rename copy issues against the retired repository name. GitHub canonicalizes that request, returning a tarball rooted at `ductus-main/` while the pinned procedure looks for `govern-main/`, and it aborts with its documented missing-directory error. Pinning the bootstrap is opting out of updates, so this is the pin working as designed; the fix is to unpin it (or re-run the installer) and let the self-update land.
- The retired `gvrn` crate is left published rather than yanked, so an existing `cargo install gvrn` keeps resolving. Removing it is the adopter's call; this migration does not touch an installed binary.
