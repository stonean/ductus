# 078 — Review fix rounds are re-read within a bound: Data Model

The shapes [078](spec.md)'s fix-round loop reads and writes: the round directory, four primitives, the re-read's result file, the `write-review` argument and record fields it adds, and the `[review]` switch. The record fields and the gate check are implemented under 066's scenario [`the-fix-round-loop-is-recorded-and-gated`](../066-audit-records-and-the-completion-gate/scenarios/the-fix-round-loop-is-recorded-and-gated.md); their shapes are stated here once.

## Round directory

`.ductus/review-rounds/{feature}/`, holding one subdirectory per briefed round, named by that round's base sha. Whichever primitive creates it first writes `.ductus/review-rounds/.gitignore` containing `*`, as `.ductus/sessions/` does (`runtime/src/session.rs`, `ignore_everything`), so nothing in it enters a snapshot, and so no round's paths, whatever the repository's own `.gitignore` says. Two primitives creating it at once both write the same `*`, so repeating the write is harmless (`BE-RACE-003`). Every primitive screens `feature` through `validate_feature_argument` and resolves `base` to a full sha (`fix_rounds::resolve_base`) before naming a directory from either, and checks the rounds directory resolves inside the project, links included (`validate_contained`), as the *Path arguments* section of `specs/022-deterministic-runtime/plan.md` states (`BE-INPUT-004`).

| File | Written by | Holds |
| --- | --- | --- |
| `{base}/round.toml` | `brief-re-read` | `base`, `snapshot` (the tree the re-read reads, and the next round's base), `paths` |
| `{base}/brief.md` | `brief-re-read` | The re-read's brief, rendered whole by the runtime |
| `{base}/result.json` | the re-read | Its findings and prescriptions; see **Result file** |

`start-fix-rounds` removes the feature's earlier subdirectories, so a run starts from no rounds.

## `start-fix-rounds`

Called once per `/{project}:review` run, when the passes end and before any fix.

Args: `{ "feature": "042-widget" }`

Result: `{ "base": "<sha>", "files": 812 }`

- `base` — a commit object of the whole working tree: tracked files as they are on disk, untracked files included, ignored files excluded, deletions applied. Its parent is `HEAD`, or none on an unborn `HEAD`. It moves no ref, writes no index file and changes no working-tree file; its author and committer are a fixed `ductus` identity, so no `user.name` is needed.
- `files` — the number of files the snapshot holds, so a snapshot of nothing does not read as one of a tree.
- Errors: a feature directory that does not exist, a project no work tree contains, or a libgit2 failure. An error is never a base: the host records the run's first round as not re-read (AC15).

## `brief-re-read`

Opens a round: derives what changed since its base, classifies it, and, when it needs a re-read, snapshots the tree and writes the brief.

Args: `{ "feature": "042-widget", "base": "<sha>" }`

Result:

```json
{ "kind": "re-read", "paths": ["runtime/src/x.rs", "specs/042-widget/data-model.md"],
  "snapshot": "<sha>", "brief": ".ductus/review-rounds/042-widget/<base>/brief.md",
  "result": ".ductus/review-rounds/042-widget/<base>/result.json", "re-reads-off": null }
```

- `paths` — the files inside the project that differ between `base` and a snapshot of the working tree taken by this call, tree to tree, bounded to the project as `compute-review-scope` bounds it (`runtime/src/primitives/compute_review_scope.rs`, `project_diff_options` and `project_paths`). Not `modified-since`, whose diff runs through the index and would report every unstaged edit a snapshot base already holds.
- `kind` — `empty` when `paths` is empty; `exempt` when every path is an analyze subject of this feature that is not a durable contract (`runtime/src/primitives/analyze_subjects.rs`, `is_analyze_subject` and not `is_review_contract`); `re-read` otherwise. `exempt` and `re-read` take a snapshot, the next round's base, so a later round never re-includes this one's changes; only `re-read` writes a brief, and, with the loop on, only `empty` returns no `snapshot`.
- `re-reads-off` — the `[review] re-reads-off` reason when the switch is set; then no brief is written, and `snapshot`, `brief` and `result` are null.
- The brief names the spec, its data model and scenarios when it has them, the command that shows the round's change (`git diff --relative {base} {snapshot}`, run from the project root, so no path reaches a shell), the `paths` as a JSON list, the severity bar, and the result file's path and format. Nothing in it comes from the host. Briefing a base again replaces that round's directory, so an earlier `result.json` is never read for a newer brief.

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
| `findings[].severity` | `MUST`, `SHOULD` or `NOTE` | The brief's severity bar. A finding outside the round's `paths` is a `NOTE`. |
| `findings[].path` | string | From the project root; one non-empty line. |
| `findings[].text` | string | One non-empty line. |
| `findings[].edits` | list | Optional prescriptions: each a `path` from the project root, the exact `old` text, and its `new` replacement. An empty `path` or `old` is refused, so a new file is never a prescription. |

JSON because a re-read agent already emits its tool arguments as JSON, and an `old`/`new` pair is the shape of the host's own edit tool; TOML's multi-line strings drop a newline after their opening quotes, and a markdown fence collides with fenced content.

## `read-re-read`

Args: `{ "feature": "042-widget", "base": "<sha>" }`

Result: `{ "state": "complete", "must": 0, "should": 1, "notes": 2, "unprescribed": 0, "findings": [ … ] }`

- `state` — `complete`; `incomplete` when the file is absent or `complete` is not `true`; `unparseable` when it is not JSON of the shape above, with `problem` naming the field. Only `complete` is a finished re-read (AC7, AC15).
- `unprescribed` — the MUST and SHOULD findings with no `edits`. A re-read is clean when it is `complete` with `must` and `should` at zero.

## `check-prescriptions`

The diff check of a round against the previous re-read's prescriptions.

Args: `{ "feature": "042-widget", "base": "<sha>" }` — `base` names the **previous** round, whose `round.toml` gives the snapshot the edits apply to and whose `result.json` gives the edits.

Result: `{ "verdict": "matched", "snapshot": "<sha>", "examined": 3, "differing": [], "unprescribed": [], "failed": [], "open": 0 }`

- Each prescribed file is taken from the snapshot, its edits applied in order, each `old` replaced only where it starts at exactly one position, overlapping ones counted, so an ambiguous prescription never applies. The result is compared with a fresh snapshot of the working tree as git stores both — the edited text hashed as a blob against the blob the snapshot holds — so a checkout that converts line endings on disk compares alike. `failed` lists each edit whose `old` occurred zero times or more than once: `{ "finding", "edit", "path", "occurrences" }`, the indexes as `result.json` orders them, and a file the snapshot does not hold as text has none. `differing` lists the prescribed files whose result is not the working tree's; `unprescribed` the files changed since the previous re-read's snapshot that no edit names, by the tree-to-tree diff `brief-re-read` uses.
- `verdict` — `matched` when all three are empty; `differs` otherwise; `could-not-run` when no `round.toml` was written at the base or it does not parse, `result.json` is missing, incomplete or unparseable, it prescribes no edit — over which a match would pass vacuously — or an edit names a path outside the project, with `problem` naming which (AC15). `could-not-run` carries no `snapshot` and is never a match.
- `snapshot` — the fresh snapshot, returned on a `matched` check alone: the next round's base, so a later round never re-lists the verified edits. `differs` carries none, since the round is re-read whole from its own base.
- `open` — the previous re-read's MUST and SHOULD findings with no `edits`. A `matched` round with `open` at zero ends the loop; with `open` above zero those findings still need a fix, a deferral or a waiver.
- It writes git objects for the snapshot and nothing else.

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
| `budget-stop` | `true` when at least two re-reads found a MUST or SHOULD and the last re-read did |
| `round-waivers` | One per `not-re-read` entry with a `reason`, or, when the switch is set, with the switch's reason and the config file as `waived-by`. Omitted when empty |

The generated Summary renders the counts and names each exempt round and each waived one.

## `check-review-gate` — check 12

**`fix-round-not-re-read`** — `review.md`'s `not-re-read` exceeds its `round-waivers` count: the message names the count, and the guidance is to re-run `/{project}:review`, whose loop re-reads the fix or records the operator's waiver. It follows check 11, `undispositioned-findings`. A record without the loop fields passes and carries `guidance` saying it does not record whether its fixes were re-read, as an undeterminable record does.

## `[review] re-reads-off`

```toml
[review]
re-reads-off = "Reviews here run on a host without subagents, and no one runs the briefs."
```

| Key | Type | Default | Notes |
| --- | --- | --- | --- |
| `re-reads-off` | string | absent: the loop runs | Switches the loop off; the value is the reason, at least 16 characters once trimmed, the length `[[review.disabled-rule-files]]` requires. Any other value halts `brief-re-read`, and `write-review` when the run passes `fix-rounds`, naming the setting, the value, the config file and the 16-character minimum (`CFG-ENV-005`, `CFG-ENV-008`); a run that passes none reads no switch, since it has no round to waive. Registered in `runtime/src/schema/config_keys.rs` and documented in `framework/bootstrap/ductus.md` §Project Configuration (`CFG-ENV-002`). |
