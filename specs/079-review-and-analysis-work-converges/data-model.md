# 079 — Review and analysis work converges Data Model

What this spec adds to records and results. A primitive's behavior is canonical in its compiled description (AC9), so this file holds only the fields a record stores or a caller reads, and the constants.

## `review.md` frontmatter

| Field | Type | Written when | Read by |
| --- | --- | --- | --- |
| `fix-rounds-open` | `true`, or absent | Every `write-review` of a run with a fix-round loop but the last, which omits it | `check-review-gate` blocks while it is present |
| `excluded` | integer | Every write, counting the paths left out of scope | The record's Summary; the body lists each path by the spec that claimed it |
| `contract-breaks` | integer | Every write, counting observations marked `contract-break` that the run did not fix | The measurement, as a gating finding left by that run |
| `open-findings` | list of paths, or absent | Each write leaving one open: each counted violation's file, and each broken contract's that the run did not fix, a person did not waive and no deferral releases | `compute-review-scope` from the record at `HEAD`, and `check-review-gate`, which blocks on a listed break |

An absent field means an older record or a `ductus exec` run, which has no loop; neither blocks. A run's two writes carry the same `reviewed-at`, so its `last-run` is one value. In `decisions:`, `decided-by: rule` marks a disposition AC2's rule decided, any other value names a person, and `contract-break: true` marks a decision on a broken contract, so a later run matching it still counts and lists the break, the file its key names when the run passes no path, unless it is in `expired-decisions`.

## `compute-review-scope` result

| Field | Type | Meaning |
| --- | --- | --- |
| `base-source` | `reopen`, `last-review`, `since`, `uncommitted-transition` or `none` | Where the window starts |
| `excluded`, `excluded-count` | list of paths, integer | As `ductus compute-review-scope --help` states; the list joins the bounded lists |
| `review-owed` | boolean | As `ductus compute-review-scope --help` states, narrowed by spec 081 so a scenario or data-model edit owes an analysis alone; true with no review record naming a run |

`modified-since` keeps its meaning, every path changed since the base, so `scope` plus `excluded` plus `deleted` accounts for it, apart from a plan entry on a first review, a path the last record lists as open, a contract its digest says changed and what a fold brings, which need not have changed in the window.

## `check-review-gate` result

`blocked-by` gains `fix-round-loop-open`, while `fix-rounds-open` is present; `review-owed`, ahead of every analyze value, whose `message` names what changed or was left open; and `unchecked-task`, after `scenario-open-questions`, whose `message` names the count and the first few boxes.

## `write-review` arguments

| Argument | Type | Meaning |
| --- | --- | --- |
| `loop-open` | boolean, MCP and CLI | as `ductus write-review --help` states |
| `fix-rounds[].excluded` | list of paths | as `ductus write-review --help` states |
| `observations[].contract-break` | boolean | as `ductus write-review --help` states |
| `observations[].disposition.by-rule` | boolean | as `ductus write-review --help` states |
| `new-waivers` | list of `rule` and `reason` | One `waivers:` entry each, as `ductus write-review --help` states. A run's two writes pass it alike |

`write-analysis` takes `by-rule` on a finding's disposition the same way. Both refuse on an unchecked box in `tasks.md` outside a task this run's own deferral names (by `refuse_while_tasks_are_open`, which counts boxes by `describe_unchecked`'s test), and `write-review` also on an empty scope (by `empty-scope`, an empty `diff-base` or a window holding only the run's own record) over an earlier record counting a finding, which a run's own earlier write is not, and on a `diff-base` no scope can be derived from, as `ductus write-review --help` and `ductus write-analysis --help` state.

## `brief-re-read` result and `round.toml`

`paths`, `exempt` and `excluded` and the round's `kind` are as `ductus brief-re-read --help` states; `exempt` and `excluded` are new lists, bounded as `paths` is.

## `diff-cross-spec` result

`window-base` replaces `first-commit`; it and `cross-spec-paths` are as `ductus diff-cross-spec --help` states. `derive-boundary` keeps its own `first-commit`.

## `measure-reopenings` (new primitive)

Arguments: `since` (an RFC 3339 UTC timestamp, `2026-10-01T00:00:00Z`, or a date, `2026-10-01`, read as that day's UTC midnight and compared with the closing commit's author date; otherwise a ref; default the last `REOPENING_WINDOW_DAYS` days from UTC midnight), and `at` (a ref; default `HEAD`).

| Result field | Type | Meaning |
| --- | --- | --- |
| `state` | `measured`, `shallow-history` or `nothing-to-compare` | A distinct value for each case the count cannot speak to |
| `since` | string | The window's start as applied: the timestamp or date as given, a `YYYY-MM-DD` UTC date by default, or a ref's full sha |
| `reopenings` | list | Each closed reopening: `feature`, `start`, `end`, `reviews`, `analyses`, `over-bound`, `decisions-by-person`, the decisions and waivers a person recorded at a run, a decision another spec's record already held with its key and outcome being that record's, and `run-decisions`, that count per run as `record`, `last-run` (the record's RFC 3339 UTC timestamp) and `decisions-by-person`. A second run is judged by the previous record of the same kind in that reopening |
| `not-counted` | list | Each closed reopening that cannot be counted: `feature`, `start` when a pass shows, `end`, `reason` (`no-recorded-run`, `record-unparseable`, or `no-in-progress-commit` for a status reaching `done` from another, as a squashed reopening does) |
| `waiting` | list of features | `in-progress` specs with no unchecked task at the commit measured |
| `over-bound-count`, `not-counted-count`, `waiting-count`, `closed-count`, `runs` | integers | The counts `/{project}:status` renders; `closed-count` holds both lists and `runs` only `reopenings` |

The lists are bounded as `compute-review-scope`'s are. `dashboard` carries the same counts and `since` in a `reopenings` block, whose `state` may also be `not-measured`, with a `reason`, when the measurement failed, so `/{project}:status` still renders.

## `classify-spec-edit` (new primitive)

Arguments, `kind`, `reasons` and how a criterion compares are `ductus classify-spec-edit --help`'s.

## Constants

| Constant | Value | Home |
| --- | --- | --- |
| `REOPENING_RUN_BOUND` | 1 review and 1 analysis, a second after a gating finding, a run cut off with its open mark, or one recording a skipped pass or a budget stop | `runtime/src/primitives/measure_reopenings.rs` |
| `REOPENING_WINDOW_DAYS` | 30 | `runtime/src/primitives/measure_reopenings.rs` |

AC20's sample of 10 is verified once, through `measure-reopenings --since`, so it is a criterion and not a constant.
