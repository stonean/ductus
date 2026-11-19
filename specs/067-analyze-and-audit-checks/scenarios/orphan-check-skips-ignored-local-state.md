---
section: "Link and reference integrity"
---

# Orphan-check-skips-ignored-local-state

## Context

An adopter's `AGENTS.md` names local state such as `.ductus/session.toml` or `.ductus/sessions/`, which the shipped gitignore excludes and a fresh clone therefore lacks. `check-orphaned-references` reports each as an orphaned reference, while `criterion-path-existence` skips the same paths as gitignored.

## Behavior

- A managed path that is absent and that the repository's ignore rules ignore is not an orphaned reference. One the rules do not ignore is reported. Outside a git repository nothing is ignored.

## Edge Cases

- The exemption is silent, as the one for a path the project ships to adopters is: it is not recorded in `skipped`, since it excludes by construction and a skip would inflate the unexamined count under a reason that means something else.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
