---
section: "Follow-on scenarios"
---

# Every-agents-generated-copies-are-excluded

## Context

`check-corpus-links`' repository scope excludes generated command copies by construction: the generator does not rewrite relative links when it changes a file's depth, so a copy's links are broken by design. A repository can commit more than one agent's copies — this one commits Claude's and Pi's ([064](../../064-pi-host-support/spec.md)) — and the exclusion used to follow the session's agent alone, so a Claude session reported Pi's copies as broken links.

## Behavior

Under `--scope repository`, a file under the config dir of any agent in the bootstrap's Agent Registry (`.claude`, `.augment`, `.agents`, `.opencode`, `.pi`) is excluded by construction and counted, whichever agent the session identifies. `host::AGENT_CONFIG_DIRS` carries the set, and a test holds it to the registry table's `config_dir` column.

## Edge Cases

- A dot-directory no agent owns (`.github/`) is still checked.
- The spec-corpus scope is unaffected: it never reached the agents' directories.
- An agent added to the registry fails the registry test until the list gains its `config_dir`.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
