---
section: "Follow-on scenarios"
---

# A-primitive-is-probed-in-a-local-clone

## Context

An adopter on 0.61.0 reported no stated way to see what a primitive answers about an artifact that does not exist yet, such as how `classify-spec-edit` classed an edit before it was made (the primitive is since retired, spec 084). A probe in the project's tree is an edit the pipeline reads: under a `done` spec it can leave work owed and reopen it, a probe left behind rides the next commit, and restoring the directory from git silently reverts uncommitted pipeline state (§spec-lifecycle). Verified 2026-11-22 (UTC) in this repository: with a line added to a `done` spec in a `git clone --local` copy, `classify-spec-edit` run from the clone by the project's binary answered for the clone (`correction`) and the project's `git status` stayed empty.

## Behavior

- §grounding's Rules carry the rule as one bullet: a primitive is probed on an artifact not yet written in a `git clone --local` copy, never in the project's tree, the probe written there and committed when the primitive reads history, and the project's runtime binary run from the clone, not its MCP tools, which read the project.
- The constitution stays within the ceiling `scripts/lint-governing-size.sh` holds it to (AC23).

## Edge Cases

- A clone holds only what is committed: a probe that needs the project's uncommitted work copies it in first.
- The session files are gitignored, so a primitive that reads the session target finds none in the clone unless one is written there.
- The binary is untracked, so it is run by its path in the project or the ductus store, never expected inside the clone.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
