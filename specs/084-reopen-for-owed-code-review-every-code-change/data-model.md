# 084 — Reopen for owed code, review every code change Data Model

The structures this spec adds or changes. Once each is built, the primitive's compiled description (`ductus <name> --help`) is canonical for its result and the constitution's review-record table for the record's fields (§drift-prevention); this file states what 084 adds to them.

## Project review record — `{specs-root}/review.md`

The review record's schema, in one file beside `inbox.md` and `system.md`, holding the latest review of code no spec claims. It has no status and no gate; its readers are `check-unreviewed-code` and the project review's own primitives.

| Field | Value in the project record |
| --- | --- |
| `spec` | `project`, a name no feature directory can take, since every feature directory carries a number |
| `last-run` | the run's time; `null` in the seed, which no review wrote |
| `reviewed-against` | the head the run's passes read up to; in the seed, `HEAD` at the update, absent when the repository had no commit. The window for code no spec claims starts after the commit finishing the newest review that read its scope, or here while that review's record is held, and until one exists after the commit that added the record |
| `diff-base` | the commit the run's window started after; absent in the seed |
| `must-violations`, `should-violations`, `low-confidence`, `blocking` | as in a feature's record; `blocking` holds no gate here |
| criteria fields (`criteria-judged`, `criteria-met`, `criteria-not-met`, `criteria-verdicts`) | absent: the record has no criteria to judge |
| `reviewed-digest` | absent: there are no contracts to digest |
| every other field | as in a feature's record |

The seed's Summary says no review ran and that code no spec claimed before its `reviewed-against` was not reviewed.

## `check-owed-work` result

| Field | Type | Meaning |
| --- | --- | --- |
| `feature` | string | The spec read |
| `owes` | boolean | `true` when either list below is non-empty |
| `unchecked-boxes` | list of `{line, count}` | Each unchecked box in `tasks.md`, under any heading, outside fences and HTML comments |
| `unticked-criteria` | list of `{label, text}` | Each unticked acceptance criterion in `spec.md`, `label` absent when unlabelled |

A missing `tasks.md` holds no box; a missing `spec.md` is an error, never `owes: false`.

Called with `all` in place of `feature`, the result is the corpus form. A call names one, never both and never neither, so a call that lost its feature is refused rather than answered for the corpus (the agent's recommendation, adopted by the operator, 2026-11-24):

| Field | Type | Meaning |
| --- | --- | --- |
| `specs` | list of `{feature, unchecked-boxes, unticked-criteria}` | Each `done` spec that owes work |
| `examined-count` | integer | `done` specs read |
| `skipped` | list of `{feature, reason}` | Each spec, at any status, the walk could not judge: untracked, linked out of the project, holding no `spec.md`, with a status that will not read, or with a file that is not UTF-8 text; never counted as owing nothing |

Past the read size each form's lists move to the file `lists-file` names, each with its `-count` inline.

## `check-unreviewed-code` result

| Field | Type | Meaning |
| --- | --- | --- |
| `at` | sha | The commit the check read |
| `mode` | `block` or `warn` | `[review] unreviewed-push`, `block` when absent |
| `state` | `none`, `found` or `not-examined` | `not-examined` when any subject below could not be read, whatever else was found |
| `done-specs` | list of `{feature, window-start, paths, deleted, commits}` | Each `done` spec with code changed after its window start by commits that claim it, and code a staging spec folded into it below `done` brought; `deleted` holds the changed paths the commit's tree no longer holds, and `commits` counts the commits that gave it a path |
| `unclaimed` | `{window-start, paths, deleted, commits}` or absent | Code changed after the project record's window start by commits no spec claims, with each path that record leaves open or left unread, so `paths` is a project review's scope; absent when there is none |
| `not-examined` | list of `{subject, reason}` | Each spec, or `project`, that could not be examined, with why (a shallow history, a record that does not parse, a `spec.md` missing or stating no status, a tree holding no project record) |
| `done-specs-count`, `unclaimed-path-count`, `not-examined-count` | integer | Entries in each list, in both forms |
| `examined-count`, `reported-path-count` | integer | How many subjects were read, and how many paths reported, `deleted` ones among them |
| `commits-walked`, `commits-reporting`, `commits-not-examined` | integer, `since` only | First-parent commits from `at` back to `since`, which is left out; those at which the check's state is `found`; and those at which it is `not-examined`, which `commits-reporting` does not count |

Past the read size `done-specs`, `unclaimed` and `not-examined` move to the file `lists-file` names.

Under `block` the CLI exits 1 when `state` is `found` or `not-examined`, and 0 otherwise; under `warn` it exits 0 and prints the same report.

## `measure-reopenings` additions

| Field | Where | Meaning |
| --- | --- | --- |
| `held-owed-work` | each entry of `reopenings`, and of `not-counted` with a `start` | Whether any commit in the reopening that touched the spec's directory left its `tasks.md` holding an unchecked box or its `spec.md` an unticked criterion; absent on a `no-in-progress-commit` entry, which has no pass to read and is not judged |
| `never-owed-count` | top level and `ReopeningCounts` | Closed reopenings, counted or not, with `held-owed-work: false` |

## `dashboard` addition

`unreviewed-code`: `{state, done-spec-count, path-count, unclaimed-path-count, reason}` from `check-unreviewed-code` at `HEAD`, rendered by `/{project}:status` on every run. `reason` is present on `not-examined` alone, naming each subject not examined with why, or the check's error when it failed, whose counts are then zero; the counts stand beside it, so code found where the check could read is not hidden by a subject it could not.

## Configuration

`[review] unreviewed-push`: `"block"` or `"warn"`, optional, `block` when absent (`CFG-ENV-001`); listed in `framework/bootstrap/ductus.md` §Project Configuration (`CFG-ENV-002`); any other value, or a `[review]` that is not a table, fails naming the key and the file (`CFG-ENV-008`).
