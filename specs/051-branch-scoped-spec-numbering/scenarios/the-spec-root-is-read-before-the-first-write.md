---
section: "Fold-back on merge"
---

# The-spec-root-is-read-before-the-first-write

## Context

`rewrite-spec-links` read every referrer before its first write, but walked the spec root reading and writing one markdown file at a time. A file under the spec root that was not a referrer and could not be read as UTF-8 text stopped the walk with an error after the files before it had been re-pointed. `kept-tails` is returned only with a result, so the error took with it every kept tail the walk had found, and a re-run, finding nothing that names the retiring directory, reports none: a reference re-pointed onto the target's own `plan.md`, which a fold never writes, is then reported by nothing, since the orphan checks test only that a path exists. `check-corpus-links` skips such a file, so one can be committed past the hook. `/{project}:fold` and `/{project}:consolidate` both call the step idempotent by construction. Found by [048](../../048-govern-acquired-runtime/spec.md)'s review on 2026-10-29.

## Behavior

- Every markdown file under the spec root, outside the retiring directory, is read before anything is written, as every referrer already is.
- One that is not UTF-8 text is listed in `skipped`, with the reason a referrer gets, and left as it is; the walk goes on past it.
- One that cannot be read for any other reason stops the run before anything is written.
- A write that fails part-way still leaves the files before it re-pointed and loses that run's `kept-tails`. The resumption rows of `/{project}:fold` and `/{project}:consolidate` say so, and a run stopped there reports its kept tails as unknown, not as none.

## Edge Cases

- **A non-UTF-8 file that links to the retiring directory** keeps the stale link. It is named in `skipped`, which the command reports, and `check-corpus-links` names it among the files it could not read.
- **A non-UTF-8 referrer under the spec root** (`system.md`) is classified with the referrers, as before, and listed once.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
