# runtime-store-path

**Introduced in:** ductus 0.28.0
**Summary:** Rewrite an MCP registration that names the bare `ductus` command to the ductus-owned path — the repo-relative pointer.

## Background

Before spec 048, the runtime was installed out of band onto `PATH` and registered by its bare name, so the resolved binary was whatever the shell found first — nothing owned it and nothing kept it at the version the framework pinned. `/ductus` now acquires the pinned release into a ductus-owned store (`~/.ductus/bin/ductus`) and reaches it from each project through a gitignored pointer (`.ductus/bin/ductus`).

An existing adopter's MCP config still names the bare command. This migration repoints it. The binary on their `PATH` is left exactly where it is — removing it is the adopter's call, not `/ductus`'s, and nothing here consults `PATH` again.

Composes with `ductus-rename` (same release, and ordered after it by the lexicographic tie-break on equal `introduced_in`): that migration renames the server **key** and the command **name**, this one rewrites the command **path**. An adopter arriving from before either gets both in one run — key and name first, then the path.

## Procedure

This migration has no runtime primitive; it is an in-place config rewrite the host performs directly. It runs under the batch migration consent (the "apply N migrations?" prompt in `ductus-procedure.md` §Pre-run Migrations) — there is **no** additional per-file prompt.

1. **Idempotency check.** If `.mcp.json` (Claude) does not exist, or does not register a `ductus` server, or the registration already names a path rather than the bare command, exit silently.

2. **Rewrite the registration.** Set `.mcp.json`'s `mcpServers.ductus.command` to `.ductus/bin/ductus`, preserving `args` and every other server and top-level key byte-for-byte.

   A `.mcp.json` that is **not valid JSON** is left untouched, with one warning line naming it — a hand-maintained config is never clobbered:

   `warning: {file} is not valid JSON; leaving it unmodified — repoint its ductus server command to .ductus/bin/ductus by hand.`

3. **Leave `PATH` alone.** Do not remove, rename, or warn about a `ductus` binary elsewhere on the adopter's `PATH`. It is theirs.

4. **Summary line.** When at least one registration was rewritten, report `repointed ductus MCP registration → .ductus/bin/ductus: {file}` in the post-scaffolding output. Omit the line entirely when nothing changed.

## Notes

- The migration is one-way. There is no reverse path.
- It rewrites the **registration only**. The store and the pointer are materialized by the same `/ductus` run's pre-flight acquisition, not here — so a project whose config this migration repoints has a pointer to resolve by the time the next session starts.
- A project that sets `[runtime] path` keeps its own binary: acquisition resolves the pointer to that path instead of the store, and this migration's rewrite still points the config at the pointer, which is the indirection that makes both cases identical to the MCP client.
