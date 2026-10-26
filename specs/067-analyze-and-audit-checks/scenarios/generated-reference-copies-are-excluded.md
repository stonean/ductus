---
section: "Link and reference integrity"
---

# Generated-reference-copies-are-excluded

## Context

[071 — Command reference split](../../071-command-reference-split/spec.md) moves each pipeline command's markdown-only reference out of its command file into `framework/reference/<command>.md`, which `/ductus` installs at `.ductus/reference/<command>.md`, one copy shared by every agent. This repository dogfoods that layout: `scripts/gen-claude-commands.sh` writes the same copies from the same sources.

The moved text carries links to the constitution, `](../constitution.md…)`. From `framework/reference/` they resolve to `framework/constitution.md`, and from an adopter's `.ductus/reference/` they resolve to `.ductus/constitution.md`, which `/ductus` installs beside them. This repository has no `.ductus/constitution.md`, so its generated copies' links resolve to nothing, by construction rather than by defect.

## Behavior

Under `--scope repository`, in a tree that carries `framework/reference/`, a file under `.ductus/reference/` is a generated copy: it is excluded by construction and counted, as every agent's generated command copies are ([`every-agents-generated-copies-are-excluded`](every-agents-generated-copies-are-excluded.md)), and the sources under `framework/reference/` are checked like any other file.

In a tree without those sources, an adopter or the audit's synthetic adopter tree (Family 35a), the files under `.ductus/reference/` are the installed ones. Their links resolve there, beside the installed `.ductus/constitution.md`, so they are checked. That is what keeps a link which resolves from `framework/reference/` but dangles once installed, such as `../commands/<command>.md`, inside Family 35a's subject. Excluding the directory in every tree was the first version and was measured and rejected: it took the twelve reference files out of 35a, which went from 28 files examined to 16.

## Edge Cases

- A file elsewhere under `.ductus/` is still checked; only `reference/` is generated.
- The spec-corpus scope is unaffected: it never reached `.ductus/`.
- A hand-written file an adopter puts in `.ductus/reference/` is checked like the installed ones, and `/ductus` removes it on the next run unless it is pinned.
- An adopter's hook is unaffected either way: it checks the spec-corpus scope.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
