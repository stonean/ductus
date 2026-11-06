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

- `base` — a commit object of the whole working tree: tracked files as they are on disk, untracked files included, ignored files excluded, deletions applied. Its parent is `HEAD`, or none on an unborn `HEAD`. It moves no ref, writes no index file and changes no working-tree file; its author and committer are a fixed `ductus` identity, so no `user.name` is needed.
- `files` — the number of files the snapshot holds, so a snapshot of nothing does not read as one of a tree.
- `budget` — the re-read budget AC6 sets: after this many re-reads that found a MUST or SHOULD, the host starts no round without the operator's decision. The runtime holds the value once (`runtime/src/primitives/fix_rounds.rs`, `RE_READ_BUDGET`), and `write-review`'s `budget-stop` derivation reads the same constant.
- Errors: a `feature` outside the allowlist, one that would leave the spec root, a feature directory that does not exist, a malformed `[review] re-reads-off` or config file, read here before the run writes any fix, a round directory that cannot be cleared, a project no work tree contains, or a libgit2 failure. An error is never a base: the host records the run's first round as not re-read (AC15).

## `brief-re-read`

Opens a round: snapshots the tree, derives what changed since its base, classifies it, and, when it needs a re-read, writes the brief.

Args: `{ "feature": "042-widget", "base": "<sha>" }`

Result:

```json
{ "kind": "re-read", "form": "inline", "paths-count": 2,
  "paths": ["runtime/src/x.rs", "specs/042-widget/data-model.md"],
  "snapshot": "<sha>", "brief": ".ductus/review-rounds/042-widget/<base>/brief.md",
  "result": ".ductus/review-rounds/042-widget/<base>/result.json", "re-reads-off": null }
```

- `paths` — the files inside the project that differ between `base` and a snapshot of the working tree taken by this call, less what `exempt` and `excluded` hold (`079-review-and-analysis-work-converges`'s data model), tree to tree, bounded to the project as `compute-review-scope` bounds it (`runtime/src/primitives/compute_review_scope.rs`, `project_diff_options` and `project_paths`). Not `modified-since`, whose diff runs through the index and would report every unstaged edit a snapshot base already holds. The reviewed feature's own `review.md` is never one: it is the command's output, rewritten whole on every run (`runtime/src/primitives/fix_rounds.rs`, `round_paths`; scenario `a-reviews-own-record-is-no-fix-round`).
- `kind` — `re-read` when `paths` holds a path; `exempt` when only `exempt` or `excluded` do, `exempt` holding each analyze subject of this feature that is not a durable contract (`runtime/src/primitives/analyze_subjects.rs`, `is_analyze_subject` and not `is_review_contract`); `empty` when all three are empty. `exempt` and `re-read` take a snapshot, the next round's base, so a later round never re-includes this one's changes; only `re-read` writes a brief, and, with the loop on, only `empty` returns no `snapshot`.
- `re-reads-off` — the `[review] re-reads-off` reason when the switch is set; then no brief is written, and `snapshot`, `brief` and `result` are null.
- `form`, `paths-count` and `lists-file` — the result's bound. Its maximum size is `[artifacts] read-size-bytes`, 50,000 bytes by default, of serialized result (scenario `fix-round-results-have-a-bounded-size`, `BE-PAYLOAD-001`): up to it `form` is `inline` and the `paths` is in the result; past it `form` is `file`, the `paths` is absent, and `lists-file` names a JSON file under `.ductus/results/brief-re-read/{feature}/` holding them whole, one entry per line, written through `bounded_lists::place` as 066's `compute-review-scope` result is. `kind` and `paths-count` stay inline in both forms, so a file-form result never reads as an empty round. `round.toml` and the brief carry every path in either form.
- The brief names the spec, its data model and scenarios when it has them, the command that shows the round's change (`git diff --relative {base} {snapshot}`, run from the project root, so no path reaches a shell, which can also show paths left out of the round), the `paths` as a JSON list, the severity bar, and the result file's path and format. Nothing in it comes from the host. Briefing a base again replaces that round's directory, so an earlier `result.json` is never read for a newer brief.

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

- `state` — `complete`; `incomplete` when the file is absent or `complete` is not `true`; `unparseable` when it is not JSON of the shape above, with `problem` naming the field. Only `complete` is a finished re-read (AC7, AC15).
- `unprescribed` — the MUST and SHOULD findings with no `edits`. A re-read is clean when it is `complete` with `must` and `should` at zero.
- `demoted` — the findings the re-read labelled MUST or SHOULD on a path outside the round's `round.toml` `paths`, counted as NOTEs and carried in `findings` as NOTEs, so a finding outside the round changes nothing in it (AC4; `runtime/src/primitives/fix_rounds.rs`, `demote_out_of_round`). A complete result whose `round.toml` is absent or does not parse has no round to be scoped by, and is `unparseable`.
- `form`, `findings-count` and `lists-file` — the result's bound. Its maximum size is `[artifacts] read-size-bytes`, 50,000 bytes by default, of serialized result (scenario `fix-round-results-have-a-bounded-size`, `BE-PAYLOAD-001`): up to it `form` is `inline` and the `findings` are in the result; past it `form` is `file`, the `findings` are absent, and `lists-file` names a JSON file under `.ductus/results/read-re-read/{feature}/` holding them whole, one entry per line, written through `bounded_lists::place` as 066's `compute-review-scope` result is. `state` and every count stay inline in both forms, so a file-form result never reads as a clean re-read. Past the read size `place` also writes `.ductus/results/.gitignore` and removes that feature's stale list files; the call is read-only otherwise.

## `check-prescriptions`

The diff check of a round against the previous re-read's prescriptions.

Args: `{ "feature": "042-widget", "base": "<sha>" }` — `base` names the **previous** round, whose `round.toml` gives the snapshot the edits apply to and whose `result.json` gives the edits.

Result: `{ "verdict": "matched", "snapshot": "<sha>", "examined": 3, "form": "inline", "differing-count": 0, "unprescribed-count": 0, "failed-count": 0, "differing": [], "unprescribed": [], "failed": [], "open": 0 }`

- Each prescribed file is taken from the snapshot, its edits applied in order, each `old` replaced only where it starts at exactly one position, overlapping ones counted, so an ambiguous prescription never applies. The result is compared with a fresh snapshot of the working tree as git stores both — the edited text hashed as a blob against the blob the snapshot holds — so a checkout that converts line endings on disk compares alike; an `old` is matched as git stores the text, which is what the round's `git diff` shows. A file matches when the working tree holds its text with every edit applied, or with only its MUST and SHOULD findings' edits applied, since a NOTE's edit the operator declines leaves text the re-read already judged acceptable (`compare`, `apply_edits`): a file's NOTE edits are declined all together or not at all. `failed` lists, for a file that matches neither, each edit whose `old` occurred zero times or more than once: `{ "finding", "edit", "path", "occurrences" }`, the indexes as `result.json` orders them; an edit to a file the snapshot does not hold as text occurs zero times. `differing` lists the prescribed files whose result is not the working tree's; `unprescribed` the files changed since the previous re-read's snapshot that no edit names, by the tree-to-tree diff `brief-re-read` uses, the feature's own `review.md` left out as there.
- `verdict` — `matched` when all three are empty; `differs` otherwise; `could-not-run` when no `round.toml` was written at the base or it does not parse, `result.json` is missing, incomplete or unparseable, it prescribes no edit — over which a match would pass vacuously — or an edit names a path outside the project, with `problem` naming which (AC15). `could-not-run` carries no `snapshot` and is never a match.
- `snapshot` — the fresh snapshot, returned on a `matched` check alone: the next round's base, so a later round never re-lists the verified edits. `differs` carries none, since the round is re-read whole from its own base.
- `open` — the previous re-read's MUST and SHOULD findings with no `edits`. A `matched` round with `open` at zero ends the loop; with `open` above zero those findings still need a fix, a deferral or a waiver.
- `form`, the three counts and `lists-file` — the result's bound. Its maximum size is `[artifacts] read-size-bytes`, 50,000 bytes by default, of serialized result (scenario `fix-round-results-have-a-bounded-size`, `BE-PAYLOAD-001`): up to it `form` is `inline` and the `differing`, `unprescribed` and `failed` are in the result; past it `form` is `file`, the `differing`, `unprescribed` and `failed` are absent, and `lists-file` names a JSON file under `.ductus/results/check-prescriptions/{feature}/` holding all three whole, one entry per line, written through `bounded_lists::place` as 066's `compute-review-scope` result is. `verdict` and every count stay inline in both forms, so a file-form result never reads as a matched check.
- It writes git objects for the snapshot and, past the read size, the list file its result names, with `bounded_lists::place`'s `.ductus/results/.gitignore` and its removal of that feature's stale list files, and nothing else.

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

The generated Summary renders the counts and names each exempt round and each waived one.

## `check-review-gate` — the fix-round check

**`fix-round-loop-open`** (spec 079) — `review.md` still carries `fix-rounds-open`, so the run stopped between its two writes.

**`fix-round-not-re-read`** — `review.md`'s `not-re-read` exceeds its `round-waivers` count: the message names the count, and the guidance is to re-run `/{project}:review`, whose loop re-reads the fix or records the operator's waiver. It follows check 11, `undispositioned-findings`. A record without the loop fields passes and carries `guidance` saying it does not record whether its fixes were re-read, as an undeterminable record does.

## `[review] re-reads-off`

```toml
[review]
re-reads-off = "Reviews here run on a host without subagents, and no one runs the briefs."
```

| Key | Type | Default | Notes |
| --- | --- | --- | --- |
| `re-reads-off` | string | absent: the loop runs | Switches the loop off; the value is the reason, at least 16 characters once trimmed, the length `[[review.disabled-rule-files]]` requires. Any other value halts `start-fix-rounds`, before the run writes any fix, `brief-re-read`, and `write-review` when the run passes `fix-rounds`, naming the setting, the value, the config file and the 16-character minimum (`CFG-ENV-005`, `CFG-ENV-008`); a run that passes none reads no switch, since it has no round to waive. Registered in `runtime/src/schema/config_keys.rs` and documented in `framework/bootstrap/ductus.md` §Project Configuration (`CFG-ENV-002`). |
