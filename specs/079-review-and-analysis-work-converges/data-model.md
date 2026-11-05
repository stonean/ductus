# 079 — Review and analysis work converges Data Model

What this spec adds to records and results. A primitive's behavior is canonical in its compiled description (AC9), so this file holds only the fields a record stores or a caller reads, and the constants.

## `review.md` frontmatter

| Field | Type | Written when | Read by |
| --- | --- | --- | --- |
| `fix-rounds-open` | `true`, or absent | The first `write-review` of a run with a fix-round loop; the last write omits it | `check-review-gate` blocks while it is present |
| `excluded` | integer | Every write, counting the paths left out of scope | The record's Summary; the body lists each path by the spec that claimed it |
| `contract-breaks` | integer | Every write, counting observations marked `contract-break` that the run did not fix | The measurement, as a gating finding left by that run |

An absent field means an older record or a `ductus exec` run, which has no loop; neither blocks. A run's two writes carry the same `reviewed-at`, so its `last-run` is one value. In `decisions:`, `decided-by: rule` marks a disposition AC2's rule decided, and any other value names a person.

## `compute-review-scope` result

| Field | Type | Meaning |
| --- | --- | --- |
| `base-source` | `reopen`, `last-review`, `since`, `uncommitted-transition` or `none` | Where the window starts |
| `excluded`, `excluded-count` | list of paths, integer | Paths in `modified-since` the scope left out; the list joins the bounded lists |
| `review-owed` | boolean | False when a review record names a run and the scope holds no path outside the spec's own directory and no scenario or data-model edit; true with no such record |

`modified-since` keeps its meaning, every path changed since the base, so `scope` plus `excluded` plus `deleted` accounts for it.

## `write-review` arguments

| Argument | Type | Meaning |
| --- | --- | --- |
| `loop-open` | boolean, MCP and CLI | Writes `fix-rounds-open`; passed only by the review command's first write, so `ductus exec` never sets it |
| `fix-rounds[].excluded` | list of paths | The paths `brief-re-read` left out of that round |
| `observations[].contract-break` | boolean | Takes a gating finding's dispositions and counts in `contract-breaks` |
| `observations[].disposition.by-rule` | boolean | Records `decided-by: rule` |

`write-analysis` takes `by-rule` on a finding's disposition the same way.

## `brief-re-read` result and `round.toml`

`paths` holds the round's non-exempt paths, which are all the brief names. `exempt` and `excluded` are new lists, bounded as `paths` is: `exempt` holds the paths the exemption covers, and `excluded` those changed only by a commit another spec claims. A round whose paths are all exempt or excluded is `exempt`.

## `diff-cross-spec` result

`window-base` replaces `first-commit`: the parent of the spec's last `in-progress` commit, or its first spec-directory commit when it has never been `in-progress`. `cross-spec-paths` holds only what commits belonging to the spec changed, plus uncommitted sibling paths. `derive-boundary` keeps its own `first-commit`.

## `measure-reopenings` (new primitive)

Arguments: `since` (a ref or date; default the last `REOPENING_WINDOW_DAYS` days), and `at` (a ref; default `HEAD`).

| Result field | Type | Meaning |
| --- | --- | --- |
| `state` | `measured`, `shallow-history` or `nothing-to-compare` | A distinct value for each case the count cannot speak to |
| `reopenings` | list | Each closed reopening: `feature`, `start`, `end`, `reviews`, `analyses`, `over-bound`, `decisions-by-person`. A second run is judged by the previous record of the same kind in that reopening |
| `not-counted` | list | Each closed reopening holding no recorded run or a record that does not parse, with its reason |
| `waiting` | list of features | `in-progress` specs with no unchecked task |
| `over-bound-count`, `not-counted-count`, `waiting-count`, `closed-count`, `runs` | integers | The counts `/{project}:status` renders |

The lists are bounded as `compute-review-scope`'s are. `dashboard` carries the same counts in a `reopenings` block.

## `classify-spec-edit` (new primitive)

Arguments: `feature`, and `base` (a ref; default `HEAD`, compared to the working tree).

| Result field | Type | Meaning |
| --- | --- | --- |
| `kind` | `none`, `correction` or `reopens` | `correction` leaves a `done` spec `done` |
| `reasons` | list of `criterion`, `open-question`, `unchecked-task`, `scenario`, `data-model`, `other-file` | Why it reopens; empty otherwise |

A criterion compares by its text with the `AC{n}:` label removed, so labelling alone stays a correction.

## Constants

| Constant | Value | Home |
| --- | --- | --- |
| `REOPENING_RUN_BOUND` | 1 review and 1 analysis, a second after a gating finding | `runtime/src/primitives/measure_reopenings.rs` |
| `REOPENING_WINDOW_DAYS` | 30 | `runtime/src/primitives/measure_reopenings.rs` |

AC20's sample of 10 is verified once, through `measure-reopenings --since`, so it is a criterion and not a constant.
