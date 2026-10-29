---
section: "Fold-back on merge"
---

# The-spec-root-is-read-before-the-first-write

## Context

`rewrite-spec-links` read every referrer before its first write, but walked the spec root reading and writing one markdown file at a time. A file under the spec root that was not a referrer and could not be read as UTF-8 text stopped the walk with an error after the files before it had been re-pointed. `kept-tails` is returned only with a result, so the error took with it every kept tail the walk had found, and a re-run, finding nothing that names the retiring directory, reports none: a reference re-pointed onto the target's own `plan.md`, which a fold never writes, is then reported by nothing, since the orphan checks test only that a path exists. The pre-commit hook's `check-corpus-links` refuses a commit while such a file is present, but a fold can still meet one in a working tree: untracked, committed with `--no-verify`, under `specs/templates/`, which that check excludes, or in a project without the hook. `/{project}:fold` and `/{project}:consolidate` both call the step idempotent by construction. Found by [048](../../048-govern-acquired-runtime/spec.md)'s review on 2026-10-29.

## Behavior

- Every markdown file under the spec root, outside the retiring directory, is read before anything is written, as every referrer already is.
- One that is not a referrer and cannot be read as UTF-8 text, or a directory under the spec root that cannot be listed, stops the run before anything is written, naming it. A file the rewrite cannot read may hold a link it would leave dangling once the directory is retired, and the orphan check never reads a file that is not a referrer, so nothing would report it.
- A write that fails part-way still leaves the files before it re-pointed and loses that run's `kept-tails`. The resumption rows of `/{project}:fold` and `/{project}:consolidate` say so, and a run stopped there reports its kept tails as unknown, not as none.

## Edge Cases

- **A non-UTF-8 file that holds no link to the retiring directory** still stops the run: the rewrite cannot tell, and the fix is to convert or remove the file and re-run.
- **A non-UTF-8 referrer under the spec root** (`system.md`) is classified with the referrers, as before: listed once in `skipped`, for `check-orphaned-references` to report what it leaves.
- **A symbolic link under the spec root** is neither followed nor listed, as before.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
