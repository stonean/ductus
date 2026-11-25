# project-review-record

**Introduced in:** ductus 0.65.0
**Summary:** Seed `specs/review.md`, the record code no spec claims is reviewed under, and report the code the pre-push hook will report at the next push (spec 084).

## Background

Since ductus 0.65.0, code no spec claims is reviewed under one project record, `specs/review.md` under the configured spec root, and `.ductus/bin/ductus check-unreviewed-code` reports the code no committed review covers: a `done` spec's, and code no spec claims. The pre-push hook this run installs (`.githooks/ductus-pre-push`) runs it on each pushed commit, and under `[review] unreviewed-push`, `"block"` when absent, stops a push it reports code for. A commit whose tree holds no project record is reported as not examined, so the record is seeded here. A seed's window starts after the commit that adds it, which is the commit carrying this update, so the update's own hooks and command files owe no review.

## Procedure

1. **Seed the record.** Invoke `write-review` with `seed` alone (CLI: `.ductus/bin/ductus write-review --seed`). It writes the record with `last-run: null` and `reviewed-against` at `HEAD`, and leaves an existing record, seeded or reviewed, as it is. When its result's `seed` reads `written`, report `project-review-record: seeded specs/review.md — commit it with this run's other changes`; when it reads `kept`, report nothing.

2. **Report the backlog.** Invoke `check-unreviewed-code` once, with no argument, reading `done-specs` and `not-examined` from the file `lists-file` names when its `form` is `file`. It reads `HEAD`, which does not hold the seed yet, so its `not-examined` names `project`: leave that entry out, since committing this run examines it. Report each `done-specs` entry as `project-review-record: {feature} has {N} path(s) changed since its last review — /{project}:review {feature} reviews them`, `{N}` its `paths` and `deleted` together, and each other `not-examined` entry as `project-review-record: {subject} not examined: {reason}`. When it reported any, add: `project-review-record: [review] unreviewed-push is "block" unless .ductus/config.toml sets it, so the next push stops on that code until it is reviewed; "warn" reports it and lets the push through.` Write no configuration. A failed call is reported with its error and ends nothing: the seed is written, and the next push's hook reports the same.

## Verification

- After a run, `specs/review.md` exists with `last-run: null`, and a second run reports nothing and leaves it byte-identical.
- `.ductus/config.toml` differs from before the run only by `[migrations].last_applied`.
- A `done` spec whose code changed since its review is named with its path count, and the `[review] unreviewed-push` line follows it.
- `check-unreviewed-code --at` the commit carrying the update reports none of that commit's files.
