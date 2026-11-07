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
| `excluded`, `excluded-count` | list of paths, integer | Paths in `modified-since` the scope left out; the list joins the bounded lists |
| `review-owed` | boolean | False when a review record names a run and holds nothing only a review clears, and the scope and the deletions it attributes to the spec hold no path outside a spec's directory and no edit to this spec's scenarios or data model, one its record's digest still describes being no edit; true with no such record |

`modified-since` keeps its meaning, every path changed since the base, so `scope` plus `excluded` plus `deleted` accounts for it, apart from a plan entry on a first review, a path the last record lists as open, a contract its digest says changed and what a fold brings, which need not have changed in the window.

## `check-review-gate` result

`blocked-by` gains `review-owed`, ahead of every analyze value, and its `message` names what changed or was left open, and `unchecked-task`, after `scenario-open-questions`, whose `message` names the count and the first few boxes.

## `write-review` arguments

| Argument | Type | Meaning |
| --- | --- | --- |
| `loop-open` | boolean, MCP and CLI | Writes `fix-rounds-open`; passed by every review-command write but the last, so `ductus exec` never sets it |
| `fix-rounds[].excluded` | list of paths | The paths `brief-re-read` left out of that round |
| `observations[].contract-break` | boolean | Takes a gating finding's dispositions and counts in `contract-breaks`; left open, its `path` is required and listed in `open-findings`; waived, the rule's stored waiver must be in `expired-decisions`. A `performReview` pass carries it too |
| `observations[].disposition.by-rule` | boolean | Records `decided-by: rule` |
| `new-waivers` | list of `rule` and `reason` | One `waivers:` entry each, listing every file the rule fires at this run that no waiver holds, stamped from `reviewed-at` and `decided-by`. A run's two writes pass it alike: an anchor the earlier write stored applies and is not stored again, and a rule it stored asks nothing of the last write |

`write-analysis` takes `by-rule` on a finding's disposition the same way. Both refuse while the spec's `tasks.md` holds an unchecked box under any heading, on `feature`, by `describe_unchecked`'s test, but for a box under a task the run's own deferral names, which is the work the record holds. A scope that is empty, by `empty-scope`, by an empty `diff-base` or by a window holding nothing but the run's own record, refuses over an earlier record counting a finding, which a run's own earlier write is not, or one it cannot read. A `diff-base` no scope can be derived from refuses the write.

## `brief-re-read` result and `round.toml`

`paths` holds the round's non-exempt paths, which are all the brief names. `exempt` and `excluded` are new lists, bounded as `paths` is: `exempt` holds the paths the exemption covers, and `excluded` those changed only by a commit another spec claims. A round whose paths are all exempt or excluded is `exempt`.

## `diff-cross-spec` result

`window-base` replaces `first-commit`: the parent of the commit the spec's reopening started at, or its first spec-directory commit when it has never been `in-progress`. `cross-spec-paths` holds only what commits belonging to the spec changed, plus uncommitted sibling paths. `derive-boundary` keeps its own `first-commit`.

## `measure-reopenings` (new primitive)

Arguments: `since` (a ref or date; default the last `REOPENING_WINDOW_DAYS` days), and `at` (a ref; default `HEAD`).

| Result field | Type | Meaning |
| --- | --- | --- |
| `state` | `measured`, `shallow-history` or `nothing-to-compare` | A distinct value for each case the count cannot speak to |
| `since` | string | The window's start as applied: a date, or a ref's full sha |
| `reopenings` | list | Each closed reopening: `feature`, `start`, `end`, `reviews`, `analyses`, `over-bound`, `decisions-by-person`, the decisions and waivers a person recorded at a run, and `run-decisions`, that count per run as `record`, `last-run` and `decisions-by-person`. A second run is judged by the previous record of the same kind in that reopening |
| `not-counted` | list | Each closed reopening that cannot be counted: `feature`, `start` when a pass shows, `end`, `reason` (`no-recorded-run`, `record-unparseable`, or `no-in-progress-commit` for a status reaching `done` from another, as a squashed reopening does) |
| `waiting` | list of features | `in-progress` specs with no unchecked task at the commit measured |
| `over-bound-count`, `not-counted-count`, `waiting-count`, `closed-count`, `runs` | integers | The counts `/{project}:status` renders; `closed-count` holds both lists and `runs` only `reopenings` |

The lists are bounded as `compute-review-scope`'s are. `dashboard` carries the same counts and `since` in a `reopenings` block, whose `state` may also be `not-measured`, with a `reason`, when the measurement failed, so `/{project}:status` still renders.

## `classify-spec-edit` (new primitive)

Arguments: `feature`, and `base` (a ref; default `HEAD`, compared to the working tree).

| Result field | Type | Meaning |
| --- | --- | --- |
| `kind` | `none`, `correction`, `mechanical` or `reopens` | `correction` and `mechanical` leave a `done` spec `done`; `none` means nothing but a run's records differs |
| `reasons` | list of `criterion`, `open-question`, `unchecked-task`, `scenario`, `data-model`, `other-file` | Why it reopens; empty otherwise |

A criterion compares by its text with the `AC{n}:` label and any cross-service reference removed, so labelling or a tick alone stays a correction, and taking a tick back reopens. `other-file` also holds a `spec.md` absent on either side and a deleted `plan.md` or `tasks.md`.

## Constants

| Constant | Value | Home |
| --- | --- | --- |
| `REOPENING_RUN_BOUND` | 1 review and 1 analysis, a second after a gating finding, a run cut off with its open mark, or one recording a skipped pass or a budget stop | `runtime/src/primitives/measure_reopenings.rs` |
| `REOPENING_WINDOW_DAYS` | 30 | `runtime/src/primitives/measure_reopenings.rs` |

AC20's sample of 10 is verified once, through `measure-reopenings --since`, so it is a criterion and not a constant.
