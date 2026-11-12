# 063 — Oversized artifacts warn with a fix Data Model

Defines the configuration key `check-artifact-size` reads, the primitive's request and response, and the `artifact-size` finding `/{project}:analyze` records from it. The serialized types — `CheckArtifactSizeArgs`, `CheckArtifactSizeResult`, `ReadSizeThreshold`, `ThresholdSource`, `OversizedArtifact`, `ArtifactKind`, `ArtifactFix`, `FixKind`, `OnDone`, `DecisionsState`, `SizeSpecHalf`, `SizeProjectHalf`, `ProjectFileSize` and `ProjectSizeState` — live in `runtime/src/schema/primitives.rs` with kebab-case serde derives, and their JSON is the stable host contract, following the primitive-schema convention in [022 — Deterministic Runtime](../022-deterministic-runtime/data-model.md). A skipped subject reuses the existing `SkippedTarget`.

## Configuration

```toml
[artifacts]
# The size, in bytes, above which a spec artifact may not be read in one call
# by an agent's file reader. A whole number of bytes, at least 1. Unset means
# 50000. Raise it for a host with a larger read cap, lower it for one with a
# smaller cap or token-dense artifacts. There is no value that turns the check
# off. Read by /{project}:clarify, /{project}:plan and /{project}:analyze
# through check-artifact-size (spec 063).
read-size-bytes = 50000
```

Resolved through the same config ladder every reader uses (`.ductus/config.toml`, then `.govern/config.toml`, then the legacy root `.govern.toml`). Any value other than a positive integer — a string, a float, a boolean, zero, a negative number, an array — is **invalid**, and so is an `artifacts` that is not a table. An invalid value halts the primitive with the error below, never the default: only an unset key, or an absent file, means 50,000. A config file that does not parse as TOML is an operational error too. Any other key in `[artifacts]` is named among the result's `notices` and never halts (`scenarios/an-unknown-artifacts-key-is-named.md`).

### The invalid-threshold error

`PrimitiveError::InvalidReadSize` carries the config file's `path`, the `setting` (`[artifacts] read-size-bytes`, or `artifacts` when the section is not a table) and the `value` as TOML renders it (a string in double quotes), flattened to one line. Its message names all three and the accepted form:

```text
invalid `{setting}` in {path}: {value} — `[artifacts] read-size-bytes` takes a whole number of bytes, at least 1; remove it to use the 50000-byte default
```

Every command that invokes `check-artifact-size` halts on it. A string or a fraction is malformed (`CFG-ENV-008`); zero or a negative is out of range (`CFG-ENV-005`). Both halt alike.

## `CheckArtifactSizeArgs`

| Field | Type | Meaning |
| --- | --- | --- |
| `feature` | string, optional | Feature directory name under the configured spec root. |
| `project-files` | boolean, default `false` | Also measure the four project files: `AGENTS.md` at the project root, and `system.md`, `events.md` and `errors.md` under the spec root (`scenarios/project-files-are-measured.md`). On the CLI, `--project-files`. |
| `staged` | boolean, default `false` | Measure each project file's index copy against its `HEAD` copy, for the pre-commit hook. Requires `project-files` and excludes `feature`. On the CLI, `--staged`. |

A call naming neither `feature` nor `project-files` is an argument error (`missing-argument`), as is `staged` without `project-files`; `staged` with `feature` is an `invalid-argument`.

## `CheckArtifactSizeResult`

The shared keys sit at the top level. The spec half and the project half follow, flattened and sharing no key: a result carries the keys of the half the call asked for and none of the other's, and a feature-only call returns exactly the keys it always has.

| Field | Type | Meaning |
| --- | --- | --- |
| `threshold` | `ReadSizeThreshold` | The threshold the check ran at, and where it came from. |
| `examined` | integer | Subjects that exist and were measured, project files included. |
| `oversized` | list of `OversizedArtifact` | Subjects over the threshold, in subject order, the spec's then the project files'. Empty means none over it, among the `examined`. |
| `skipped` | list of `SkippedTarget` | Subjects that exist but could not be examined: family `artifact-size`, reason `artifact-unreadable`. A missing spec subject is never listed; a missing project file is reported in `project-files`. |
| `notices` | list of strings | Human-readable notices every caller shows: a line for each key in `[artifacts]` the shared config-key registry does not recognize, `unknown config key: [artifacts] {key} ({config file}) — this version of ductus does not read it` (`scenarios/an-unknown-artifacts-key-is-named.md`, for spec 019), then the unreadable-decisions notice. An invalid threshold is the error above, never a notice. |

### The spec half (`SizeSpecHalf`), present with `feature`

| Field | Type | Meaning |
| --- | --- | --- |
| `feature` | string | Echoed from the args. |
| `status` | string | The spec's frontmatter `status`, which the `on-done` effects are computed against. |
| `decisions` | `DecisionsState` | Whether `analysis.md`'s stored decisions were read. |
| `path` | string | Repo-relative path of `spec.md`. |

### The project half (`SizeProjectHalf`), present with `project-files`

| Field | Type | Meaning |
| --- | --- | --- |
| `project-files` | list of `ProjectFileSize` | One record per file, in subject order: `AGENTS.md`, then `system.md`, `events.md` and `errors.md` under the spec root. |

`ProjectFileSize`:

| Field | Type | Meaning |
| --- | --- | --- |
| `path` | string | Project-relative path. |
| `kind` | string | `agents`, `system`, `events` or `errors`. |
| `state` | `ProjectSizeState` | `measured`; `absent` (no such file, or, staged, none in the index, a file deleted in the commit among them); or `unreadable` (it also appears in `skipped`, and staged, an unresolved conflict or an unreadable blob is one). A missing spec subject is never reported, but a missing project file is, so a project with no `events.md` reads differently from one whose `events.md` was measured. |
| `bytes` | integer, optional | The length on disk, or, staged, the index copy's. Present exactly when `state` is `measured`. |
| `head-bytes` | integer, optional | Staged only: the `HEAD` copy's length. Absent when `HEAD` has none, or the repository has no commits. |
| `refused` | boolean, optional | Staged only, for a measured file: the staged copy is over the threshold and either has no `HEAD` copy or is larger than it. An unchanged or shrunk over-limit file is not refused. |

A project file over the threshold joins `oversized` with kind `project-file`, whose one fix is `prune-project`, with no `on-done` whatever the feature's status. Its `warning` and `message` are worded as a spec subject's, and a stored waiver in the named feature's `analysis.md` covers it by the decided rule below. Staged, `oversized` is judged on the staged copy. The CLI, with `--staged`, exits 1 when any file is refused or any is in `skipped`, naming each on stderr (`ductus check-artifact-size --help`); a repository or `HEAD` that cannot be read is an error, never a clean result, and an index that cannot be read leaves each project file `unreadable` and in `skipped`, so the exit is 1.

Subject order is `spec.md`, `plan.md`, `tasks.md`, `data-model.md`, then the scenarios in the shared scenario ordering (case-insensitive, raw-byte tiebreak).

## `ReadSizeThreshold`

| Field | Type | Meaning |
| --- | --- | --- |
| `bytes` | integer | The threshold applied: the configured value, or 50,000. |
| `source` | `ThresholdSource` | `default` (unset, or no config file) or `config` (a positive integer read from the file). |

## `OversizedArtifact`

| Field | Type | Meaning |
| --- | --- | --- |
| `path` | string | Repo-relative path of the subject. |
| `kind` | `ArtifactKind` | `spec`, `plan`, `tasks`, `data-model`, `scenario`, or `project-file`. |
| `bytes` | integer | The file's length on disk. |
| `pages` | integer | `ceil(bytes / threshold)`; at least 2 for every entry here. |
| `fixes` | list of `ArtifactFix` | The fixes for this kind, in the spec's table order. Never includes a waiver. |
| `warning` | string | The rendered one-line warning the commands print. |
| `message` | string | The `artifact-size` finding message, in the format below. |
| `decided` | boolean | A stored waiver still covers this subject (the decided rule below). Always `false` unless `decisions` is `read`. |
| `decision-key` | string, optional | The stored decision's key. Present exactly when `decided` is `true`. |

## `ArtifactFix`

| Field | Type | Meaning |
| --- | --- | --- |
| `fix` | `FixKind` | `prune`, `split`, `trim`, `promote`, or `prune-project`. |
| `text` | string | The fix as the warning words it. A `split` names the manual route and no split command. |
| `on-done` | `OnDone`, optional | Present only when `status` is `done`: `never` (prune, and a `plan.md` trim), `always` (split, promote, and a `data-model.md` trim), or `if-requirement-changes` (any other trim, which edits `spec.md`). Never present on a `project-file`: it belongs to no spec. |

Fixes by kind: `tasks` → `prune`, then `split` and `trim` as the fixes that apply if it is still over after pruning; `spec` and `plan` → `split`, `trim`; `data-model` → `split`, `trim` (whose text forbids sub-files); `scenario` → `promote`; `project-file` → `prune-project`.

## `DecisionsState`

`absent` (no `analysis.md`), `read` (the list was read, and is empty when the record stores no decisions), or `unreadable` (the list could not be read or does not parse; nothing is decided, and a notice says so). The primitive never fails on an unreadable list.

## The `artifact-size` finding

`/{project}:analyze` records each oversized subject, a project file included, as an advisory finding with family `artifact-size`, path the subject's, and message:

```text
`{path}` is {bytes} bytes, {pages} read pages at the {threshold}-byte read size
```

`{bytes}` and `{threshold}` are written as plain integers with no separators, so the stored key parses without locale assumptions. The stored key is the usual `{family} — {message}`.

## The decided rule

A subject is decided when the `decisions:` list holds a decision with outcome `waived` whose key:

1. carries the family `artifact-size` before its ` — ` separator, and
2. parses under the message format above to the **same path** as the subject, and
3. records a page count **greater than or equal to** the subject's current `pages`.

The first such decision in list order supplies `decision-key`. A key that does not parse under the format is ignored, never matched. A deferred decision never decides a subject: only a waiver records that the operator accepted the size.
