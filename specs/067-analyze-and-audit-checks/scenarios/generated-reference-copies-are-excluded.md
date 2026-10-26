---
section: "Link and reference integrity"
---

# Generated-reference-copies-are-excluded

## Context

[071 — Command reference split](../../071-command-reference-split/spec.md) moves each pipeline command's markdown-only reference out of its command file into `framework/reference/<command>.md`, which `/ductus` installs at `.ductus/reference/<command>.md`, one copy shared by every agent. This repository dogfoods that layout: `scripts/gen-claude-commands.sh` writes the same copies from the same sources.

The moved text carries links to the constitution, `](../constitution.md…)`. From `framework/reference/` they resolve to `framework/constitution.md`, and from an adopter's `.ductus/reference/` they resolve to `.ductus/constitution.md`, which `/ductus` installs beside them. This repository has no `.ductus/constitution.md`, so its generated copies' links resolve to nothing, by construction rather than by defect.

## Behavior

Under `--scope repository`, a file under `.ductus/reference/` is excluded by construction and counted, as every agent's generated command copies are ([`every-agents-generated-copies-are-excluded`](every-agents-generated-copies-are-excluded.md)). The sources under `framework/reference/` are checked like any other file.

The exclusion lives in the primitive, so an adopter's repository scope skips those files too. That is deliberate and matches the agent copies: the files are framework-managed, only a release can repair them, and their links are checked at the source, where Family 39 resolves their anchors and Family 35 resolves their links in a synthetic adopter tree.

## Edge Cases

- A file elsewhere under `.ductus/` is still checked; only `reference/` is generated.
- The spec-corpus scope is unaffected: it never reached `.ductus/`.
- A hand-written file an adopter puts in `.ductus/reference/` is excluded too. `/ductus` removes it on the next run unless it is pinned, so the exclusion hides it for at most one run.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
