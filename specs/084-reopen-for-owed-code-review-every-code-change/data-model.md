# 084 — Reopen for owed code, review every code change Data Model

The structures this spec adds or changes. Once each is built, the primitive's compiled description (`ductus <name> --help`) is canonical for its result and the constitution's review-record table for the record's fields (§drift-prevention); this file states what 084 adds to them.

## Project review record — `{specs-root}/review.md`

The review record's schema, in one file beside `inbox.md` and `system.md`, holding the latest review of code no spec claims. It has no status and no gate; `check-unreviewed-code` is its only reader.

| Field | Value in the project record |
| --- | --- |
| `spec` | `project`, a name no feature directory can take, since every feature directory carries a number |
| `last-run` | the run's time; `null` in the seed, which no review wrote |
| `reviewed-against` | how far unclaimed code has been read: the window for code no spec claims starts here; in the seed, `HEAD` at the update |
| `diff-base` | the previous record's `reviewed-against`; absent in the seed |
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

## `check-unreviewed-code` result

| Field | Type | Meaning |
| --- | --- | --- |
| `at` | sha | The commit the check read |
| `mode` | `block` or `warn` | `[review] unreviewed-push`, `block` when absent |
| `state` | `none`, `found` or `not-examined` | `not-examined` when any subject below could not be read, whatever else was found |
| `done-specs` | list of `{feature, window-start, paths, commits}` | Each `done` spec with code changed after its window start by commits that claim it |
| `unclaimed` | `{window-start, paths, commits}` or absent | Code changed after the project record's `reviewed-against` by commits no spec claims; absent when there is none |
| `not-examined` | list of `{subject, reason}` | Each `done` spec, or `project`, whose window could not be placed, with why (a shallow history, a record that does not parse) |
| `examined-count`, `reported-path-count` | integer | How many subjects were read, and how many paths reported |
| `commits-reporting` | integer, `since` only | First-parent commits from `at` back to `since` at which the check reports code |

Under `block` the CLI exits 1 when `state` is `found` or `not-examined`, and 0 otherwise; under `warn` it exits 0 and prints the same report.

## `measure-reopenings` additions

| Field | Where | Meaning |
| --- | --- | --- |
| `held-owed-work` | each entry of `reopenings` | Whether any commit in the reopening that touched the spec's directory left its `tasks.md` holding an unchecked box or its `spec.md` an unticked criterion |
| `never-owed-count` | top level and `ReopeningCounts` | Closed reopenings with `held-owed-work: false`; `not-counted` reopenings are outside it |

## `dashboard` addition

`unreviewed-code`: `{state, done-spec-count, path-count, unclaimed-path-count}` or `{state: "not-examined", reason}`, from `check-unreviewed-code` at `HEAD`, rendered by `/{project}:status` on every run.

## Configuration

`[review] unreviewed-push`: `"block"` or `"warn"`, optional, `block` when absent (`CFG-ENV-001`); listed in `framework/bootstrap/ductus.md` §Project Configuration (`CFG-ENV-002`); any other value, or a `[review]` that is not a table, fails naming the key and the file (`CFG-ENV-008`).
