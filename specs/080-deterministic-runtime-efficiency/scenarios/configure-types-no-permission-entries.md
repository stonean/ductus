---
section: "Behavior"
---

# Configure-types-no-permission-entries

## Context

On Claude Code, `/{project}:configure`'s step 1 has the model pass `merge-permissions` the canonical allow, deny and retired sets entry by entry. The cost round measured 132 entries, 3,897 characters of arguments, in a 13.28 s step of the run's 28.9 s and 3,047 output tokens (`research.md`, R2). The primitive already holds the canonical allow set, as the ceiling it refuses anything beyond. Step 5 then edits `.claude/settings.local.json` with the host's file tools to add `additionalDirectories`. That edit was denied under `-p` with `Edit` allowed, so an interactive session is asked to approve it on every run.

## Behavior

- On Claude Code, `/{project}:configure` merges the canonical allow, deny and retired sets with no entry passed by the model: the primitive applies the sets the framework ships.
- The same call ensures `permissions.additionalDirectories` holds the `specs/` directory and the project's command directory as absolute paths, so the command changes the settings file with no host file write and no approval prompt.
- The result still reports what was added, deduplicated and retired, so the command's report is unchanged.

## Edge Cases

- A caller that passes its own entries is still refused beyond the ceiling.
- An entry the adopter added stays where it is.
- Auggie, Antigravity and OpenCode merge their own permission shapes host-side, which `merge-permissions` does not serve. Their cost is not measured here, and their commands are unchanged.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
