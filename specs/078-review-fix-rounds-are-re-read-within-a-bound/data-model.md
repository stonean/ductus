# 078 — Review fix rounds are re-read within a bound: Data Model

The shapes [078](spec.md)'s fix-round loop reads and writes: the round directory, four primitives, the re-read's result file, the `write-review` argument and record fields it adds, and the `[review]` switch. The record fields and the gate check are implemented under 066's scenario [`the-fix-round-loop-is-recorded-and-gated`](../066-audit-records-and-the-completion-gate/scenarios/the-fix-round-loop-is-recorded-and-gated.md); their shapes are stated here once.

## Round directory

`.ductus/review-rounds/{feature}/`, holding one subdirectory per briefed round, named by that round's base sha. Whichever primitive creates it first writes `.ductus/review-rounds/.gitignore` containing `*`, as `.ductus/sessions/` does (`runtime/src/session.rs`, `ignore_everything`), so nothing in it enters a snapshot, and so no round's paths, whatever the repository's own `.gitignore` says. It is created with `create_new`, so of two primitives creating it at once the first writes it and the second finds it present and leaves it (`fix_rounds::ignore_everything`, `BE-RACE-003`). Every primitive screens `feature` by the runtime's feature-name allowlist and then `validate_feature_argument` (`fix_rounds::validate_feature`, `BE-INPUT-002`), so `.` or a nested path names no round directory, and resolves `base` to a full sha (`fix_rounds::resolve_base`) before naming a directory from either, and checks the rounds directory resolves inside the project, links included (`validate_contained`), as the *Path arguments* section of `specs/022-deterministic-runtime/plan.md` states (`BE-INPUT-004`).

| File | Written by | Holds |
| --- | --- | --- |
| `{base}/round.toml` | `brief-re-read` | `snapshot` (the tree the re-read reads, and the next round's base), `paths` |
| `{base}/brief.md` | `brief-re-read` | The re-read's brief, rendered whole by the runtime |
| `{base}/result.json` | the re-read | Its findings and prescriptions; see **Result file** |

`start-fix-rounds` removes the feature's earlier subdirectories, so a run starts from no rounds.

## `start-fix-rounds`

Called once per `/{project}:review` run, when the passes end and before any fix.

Args: `{ "feature": "042-widget" }`

Result: `{ "base": "<sha>", "files": 812, "budget": 2 }`

- `base`, `files` and `budget` are as `ductus start-fix-rounds --help` states. The snapshot's author and committer are a fixed `ductus` identity, so no `user.name` is needed, and its parent is none on an unborn `HEAD`; the runtime holds the budget once (`runtime/src/primitives/fix_rounds.rs`, `RE_READ_BUDGET`), and `write-review`'s `budget-stop` derivation reads the same constant.
- Errors: a `feature` outside the allowlist, one that would leave the spec root, a feature directory that does not exist, a malformed `[review] re-reads-off` or config file, read here before the run writes any fix, a round directory that cannot be cleared, a project no work tree contains, or a libgit2 failure. An error is never a base: the host records the run's first round as not re-read (AC15).

## `brief-re-read`

Opens a round: snapshots the tree, derives what changed since its base, classifies it, and, when it needs a re-read, writes the brief.

Args: `{ "feature": "042-widget", "base": "<sha>" }`

Result:

```json
{ "kind": "re-read", "form": "inline", "paths-count": 2, "exempt-count": 0, "excluded-count": 0,
  "paths": ["runtime/src/x.rs", "specs/042-widget/data-model.md"],
  "snapshot": "<sha>", "brief": ".ductus/review-rounds/042-widget/<base>/brief.md",
  "result": ".ductus/review-rounds/042-widget/<base>/result.json", "re-reads-off": null, "exempt": [], "excluded": [] }
```

- `paths` — as `ductus brief-re-read --help` states. Its diff is tree to tree, never `modified-since`, whose diff runs through the index and would report every unstaged edit a snapshot base already holds; the project bound is `compute-review-scope`'s (`runtime/src/primitives/compute_review_scope.rs`, `project_diff_options` and `project_paths`), and the feature's own `review.md` is left out by `runtime/src/primitives/fix_rounds.rs`, `round_paths` (scenario `a-reviews-own-record-is-no-fix-round`).
- `kind` — as `ductus brief-re-read --help` states: `exempt` holds each analyze subject that is not a durable contract (`runtime/src/primitives/analyze_subjects.rs`, `is_analyze_subject` and not `is_review_contract`), and a round that takes a snapshot makes it the next base, so a later round never re-includes this one's changes.
- `re-reads-off` — as `ductus brief-re-read --help` states: the `[review] re-reads-off` reason, with no snapshot and no brief.
- `form`, `paths-count` and `lists-file` — the result's bound (scenario `fix-round-results-have-a-bounded-size`, `BE-PAYLOAD-001`), whose form and maximum size `ductus brief-re-read --help` states: a file-form result's `lists-file` is under `.ductus/results/brief-re-read/{feature}/`, written through `bounded_lists::place` as 066's `compute-review-scope` result is. `round.toml` and the brief carry every path in either form.
- The brief's contents are as `ductus brief-re-read --help` states; the command that shows the round's change is `git diff --relative {base} {snapshot}`, run from the project root, so no path reaches a shell, and it can also show paths left out of the round. Nothing in the brief comes from the host.

## Result file

`result.json`, written by the re-read last, after its reading is done.

```json
{ "complete": true,
  "findings": [
    { "severity": "SHOULD", "path": "framework/reference/review.md",
      "text": "the deferral sentence still names the retired verb",
      "edits": [{ "path": "framework/reference/review.md", "old": "routed", "new": "deferred" }] } ] }
```

| Field | Type | Notes |
| --- | --- | --- |
| `complete` | boolean | Must be `true`. A file without it, or no file, is an incomplete re-read. |
| (any other key) | — | Refused at every level, so a misspelled key is a problem `read-re-read` reports, never a prescription silently dropped. |
| `findings[].severity` | `MUST`, `SHOULD` or `NOTE` | The brief's severity bar. A finding outside the round's `paths` is a `NOTE`: `read-re-read` and `check-prescriptions` count a `MUST` or `SHOULD` there as one, by the round's own `round.toml`. |
| `findings[].path` | string | From the project root; one non-empty line. |
| `findings[].text` | string | One non-empty line. |
| `findings[].edits` | list | Optional prescriptions: each a `path` from the project root, the exact `old` text, and its `new` replacement. An empty `path` or `old` is refused, so a new file is never a prescription. |

JSON because a re-read agent already emits its tool arguments as JSON, and an `old`/`new` pair is the shape of the host's own edit tool; TOML's multi-line strings drop a newline after their opening quotes, and a markdown fence collides with fenced content.

## `read-re-read`

Args: `{ "feature": "042-widget", "base": "<sha>" }`

Result: `{ "state": "complete", "must": 0, "should": 1, "notes": 2, "unprescribed": 0, "demoted": 0, "form": "inline", "findings-count": 3, "findings": [ … ] }`

- `state` — as `ductus read-re-read --help` states; only `complete` is a finished re-read (AC7, AC15).
- `unprescribed` — as `ductus read-re-read --help` states.
- `demoted` — as `ductus read-re-read --help` states, so a finding outside the round changes nothing in it (AC4; `runtime/src/primitives/fix_rounds.rs`, `demote_out_of_round`).
- `form`, `findings-count` and `lists-file` — the result's bound (scenario `fix-round-results-have-a-bounded-size`, `BE-PAYLOAD-001`), whose form and maximum size `ductus read-re-read --help` states: a file-form result's `lists-file` is under `.ductus/results/read-re-read/{feature}/`, written through `bounded_lists::place` as 066's `compute-review-scope` result is.

## `check-prescriptions`

The diff check of a round against the previous re-read's prescriptions.

Args: `{ "feature": "042-widget", "base": "<sha>" }` — `base` names the **previous** round, whose `round.toml` gives the snapshot the edits apply to and whose `result.json` gives the edits.

Result: `{ "verdict": "matched", "snapshot": "<sha>", "examined": 3, "form": "inline", "differing-count": 0, "unprescribed-count": 0, "failed-count": 0, "differing": [], "unprescribed": [], "failed": [], "open": 0 }`

- The edits apply, and the result is compared with the working tree, as `ductus check-prescriptions --help` states, including each `failed` entry's fields and the files `differing` and `unprescribed` name (AC5; `compare`, `apply_edits`).
- `verdict`, `snapshot` and `open` are `ductus check-prescriptions --help`'s, including each `could-not-run` cause (AC15): a prescription-less result is one because a match over it would pass vacuously.
- `form`, the three counts and `lists-file` — the result's bound, as `ductus check-prescriptions --help` states (scenario `fix-round-results-have-a-bounded-size`, `BE-PAYLOAD-001`): the list file is written through `bounded_lists::place` as 066's `compute-review-scope` result is.
- What it writes is as `ductus check-prescriptions --help` states.

## `write-review` — the loop's argument and record fields

Args (the added half only):

```json
{ "fix-rounds": [
    { "check": "re-read", "result": "findings" },
    { "check": "diff-checked" },
    { "check": "exempt" },
    { "check": "not-re-read", "reason": "the operator declined: the change is a revert of round 1" } ],
  "decided-by": "dev@example.com" }
```

- `check` — `exempt`, `re-read`, `diff-checked` or `not-re-read`, one entry per round in order; a round with an empty scope is no round. `result` (`clean` or `findings`) is required on a `re-read` and refused elsewhere; `reason` is allowed only on a `not-re-read` and, when present, is its waiver, which requires `decided-by`.
- `fix-rounds` absent writes none of the fields below: the run stated nothing about its fixes, as an unstated `examined` is absent. An empty list writes them all at zero.

Record (`review.md` frontmatter, after `skipped-passes`):

```yaml
fix-rounds: 4
re-reads: 1
diff-checked: 1
not-re-read: 1
budget-stop: false
round-waivers:
  - round: 4
    reason: "the operator declined: the change is a revert of round 1"
    waived-by: dev@example.com
    waived-at: 2026-11-03T18:00:00Z
```

| Field | Derived as |
| --- | --- |
| `fix-rounds`, `re-reads`, `diff-checked`, `not-re-read` | The number of entries, and of entries with each `check` |
| `budget-stop` | `true` when at least `budget` re-reads (`RE_READ_BUDGET`) found a MUST or SHOULD and the last re-read did |
| `round-waivers` | One per `not-re-read` entry with a `reason`, or, when the switch is set, with the switch's reason and the config file as `waived-by`. Omitted when empty |

The Summary, supplied or generated, renders the counts and names each exempt round and each waived one: `write-review` appends that account to a Summary the caller supplies, as a paragraph of its own.

## `check-review-gate` — the fix-round check

**`fix-round-loop-open`** (spec 079) — `review.md` still carries `fix-rounds-open`, so the run stopped between its two writes.

**`fix-round-not-re-read`** — `review.md`'s `not-re-read` exceeds the `round-waivers` that carry a non-blank reason: the message names the count, and the guidance is to re-run `/{project}:review`, whose loop re-reads the fix or records the operator's waiver. It follows `undispositioned-findings`. A record without the loop fields passes and carries `guidance` saying it does not record whether its fixes were re-read, as an undeterminable record does.

## `[review] re-reads-off`

```toml
[review]
re-reads-off = "Reviews here run on a host without subagents, and no one runs the briefs."
```

| Key | Type | Default | Notes |
| --- | --- | --- | --- |
| `re-reads-off` | string | absent: the loop runs | Switches the loop off; the value is the reason, at least 16 characters once trimmed, the length `[[review.disabled-rule-files]]` requires. An empty value counts as absent, so the loop runs (`CFG-ENV-001`). Any other value halts `start-fix-rounds`, before the run writes any fix, `brief-re-read`, and `write-review` when the run passes `fix-rounds`, naming the setting, the value, the config file and the 16-character minimum (`CFG-ENV-005`, `CFG-ENV-008`); a run that passes none reads no switch, since it has no round to waive. Registered in `runtime/src/schema/config_keys.rs` and documented in `framework/bootstrap/ductus.md` §Project Configuration (`CFG-ENV-002`). |
