---
section: "Generators and Hooks"
---

# A-path-list-commit-leaves-the-shared-index-current

## Context

Under `git commit -- <paths>`, git builds a temporary index from `HEAD` and the named paths and runs the hook with `GIT_INDEX_FILE` set to it. `.githooks/pre-commit` writes each file it carries (`git update-index --add --cacheinfo`) to that index, its `real_index`, so the commit holds the rewritten file: a generated output, or a named file `markdownlint-cli2 --fix`, a derivation or the labelling pass rewrote. Git then carries only the named paths into the shared index, as they were before the hook ran. The shared index keeps each carried file's pre-commit blob, which `git status` reads as a staged reversal, so a later whole-index commit, by any session, reverts the file. On 2026-11-23 `7eb29d1f`, a path-list commit of `framework/commands/implement.md` and `framework/reference/implement.md`, left the shared index holding the old `.claude/commands/ductus/implement.md`, `.ductus/reference/implement.md` and `.pi/prompts/ductus-implement.md`, reset by hand. Found by [083](../../083-commits-are-attributed-to-specs-without-a-subject-convention/spec.md)'s implementation, deferred here from its task 7. The shipped `framework/bootstrap/hooks/ductus-pre-commit` re-stages only spec files already in the commit, which a path-list commit names, so only this repository's hook has it.

## Behavior

- After a path-list commit, the shared index holds the committed blob of every file the hook rewrote and carried, a generated output or a named file, so `git diff --cached` names none of them and a later whole-index commit cannot revert them.
- An entry of the shared index the commit does not carry is left as it was, so another session's staged work is neither staged nor unstaged by it.
- A whole-index commit (`git commit` with no paths, or `-a`) behaves as before.

## Edge Cases

- A generated output another session staged in the shared index before the path-list commit: the hook carries the regenerated output into the commit and leaves that session's staged entry, which now disagrees with `HEAD`, as it was.
- A named markdown file `--fix` rewrites: the commit holds the fixed blob, and so does the shared index, where git had staged the unfixed working copy (task 50).
- A path-list commit whose hook rewrites nothing: the shared index is untouched beyond git's own update of the named paths.
- `scripts/tests/test-pre-commit-staging.sh` holds a path-list case asserting the shared index after the commit, for a generated output and for a named file `--fix` rewrote, and each case fails with its behavior removed.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
