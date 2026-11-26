# 063 — Oversized artifacts warn with a fix Data Model

Defines the configuration key `check-artifact-size` reads, the primitive's request and response, and the `artifact-size` finding `/{project}:analyze` records from it. The serialized types — `CheckArtifactSizeArgs`, `CheckArtifactSizeResult`, `ReadSizeThreshold`, `ThresholdSource`, `OversizedArtifact`, `ArtifactKind`, `ArtifactFix`, `FixKind`, `DecisionsState`, `SizeSpecHalf`, `SizeProjectHalf`, `ProjectFileSize`, `ProjectSizeState`, `PartSize`, `PartFinding` and `PartFindingKind`, with `form` the shared `ListsForm` — live in `runtime/src/schema/primitives.rs` with kebab-case serde derives, and their JSON is the stable host contract, following the primitive-schema convention in [022 — Deterministic Runtime](../022-deterministic-runtime/data-model.md). A skipped subject reuses the existing `SkippedTarget`.

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

The shared keys sit at the top level. The spec half and the project half follow, flattened and sharing no key: a result carries the keys of the half the call asked for and none of the other's.

| Field | Type | Meaning |
| --- | --- | --- |
| `threshold` | `ReadSizeThreshold` | The threshold the check ran at, and where it came from. |
| `examined` | integer | Subjects that exist and were measured, project files and their parts included. |
| `form` | `inline` or `file` | Where the lists below and the project half's `parts` and `part-findings` are: in the result, or past the read size in the file `lists-file` names. |
| `oversized` | list of `OversizedArtifact` | Subjects over the threshold, in subject order: the spec's, then the project files', then their parts'. Empty means none over it, among the `examined`. Absent in the file form. |
| `oversized-count` | integer | Entries in `oversized`, in either form. |
| `skipped` | list of `SkippedTarget` | Subjects that exist but could not be examined: family `artifact-size`, reason `artifact-unreadable`. A missing spec subject is never listed; a missing project file is reported in `project-files`. With `project-files`, an unreadable part is listed, and so is a directory under `.ductus/docs/` that is, or lies under, a symbolic link leaving the project or cannot be listed, since the parts it holds are unknown. Absent in the file form. |
| `skipped-count` | integer | Entries in `skipped`, in either form. |
| `notices` | list of strings | Human-readable notices every caller shows: a line for each key in `[artifacts]` the shared config-key registry does not recognize, `unknown config key: [artifacts] {key} ({config file}) — this version of ductus does not read it` (`scenarios/an-unknown-artifacts-key-is-named.md`, for spec 019), then the unreadable-decisions notice. An invalid threshold is the error above, never a notice. Absent in the file form. |
| `notices-count` | integer | Entries in `notices`, in either form. |
| `lists-file` | string, optional | In the file form only: the file, from the project root, holding `oversized`, `skipped` and `notices` whole, and with `project-files` also `parts` and, unless `staged`, `part-findings`, each key present even when empty. |

**Size** (BE-PAYLOAD-001). `oversized` and `skipped` grow with a spec directory's scenario count, `notices` with the unknown keys in `[artifacts]`, and `parts` and `part-findings` with the project's parts (spec 087), so the five are bounded as `check-artifacts`' lists are: while the result fits `[artifacts] read-size-bytes` they are inline, and past it `form` is `file`, none of them is inline, and `lists-file` names a file under `.ductus/results/check-artifact-size/{key}/` holding them whole, `{key}` being the feature, or `project-files` for a call naming none. Every other collection is bounded by construction: `project-files` holds exactly four records, an entry's `fixes` at most three, and the rest are scalars. The file form's own size is 339 bytes of compact JSON (340 as the CLI prints it, with its trailing newline), measured on the dev build over one draft spec `001-x` and no project file, at `read-size-bytes = 10`, each count and `examined` one digit. It grows by three bytes per character of the feature directory's name, which it carries three times, by each further digit of a count, `examined` or the threshold, and by the length of the `status` and `decisions` words. With `project-files` the same call is 631 bytes, the four records all `absent` and no part, so `parts-count` and `part-findings-count` are each 0; a measured record adds its `bytes`, and a staged one its `head-bytes` and `refused`. Consumers read the lists from `lists-file` in the file form: step 17 of `framework/commands/analyze.md`, step 10 of `clarify.md` and step 8 of `plan.md`, and under `ductus exec` the walker, which restores them (`restore_lists`) before `AnalyzeTally` reads the result. The staged CLI reads `skipped-count`, never `skipped`.

### The spec half (`SizeSpecHalf`), present with `feature`

| Field | Type | Meaning |
| --- | --- | --- |
| `feature` | string | Echoed from the args. |
| `status` | string | The spec's frontmatter `status`. At `done`, each `warning` ends with the rule on what reopens the spec. |
| `decisions` | `DecisionsState` | Whether `analysis.md`'s stored decisions were read. |
| `path` | string | Repo-relative path of `spec.md`. |

### The project half (`SizeProjectHalf`), present with `project-files`

| Field | Type | Meaning |
| --- | --- | --- |
| `project-files` | list of `ProjectFileSize` | One record per file, in subject order: `AGENTS.md`, then `system.md`, `events.md` and `errors.md` under the spec root. |
| `parts` | list of `PartSize` | One record per part of a project file (spec 087), by project file in subject order, then in the shared scenario ordering: on disk, every `*.md` directly under `.ductus/docs/agents/`, `system/`, `events/` or `errors/`, linked or not; staged, every index entry there. Empty when there are none. Absent in the file form. |
| `parts-count` | integer | Entries in `parts`, in either form. |
| `part-findings` | list of `PartFinding` | The integrity findings over the parts, sorted by path then kind. Empty when there are none. Absent in the file form, and staged, where they are not computed. |
| `part-findings-count` | integer, optional | Entries in `part-findings`, in either form. Absent staged. |

`ProjectFileSize`:

| Field | Type | Meaning |
| --- | --- | --- |
| `path` | string | Project-relative path. |
| `kind` | string | `agents`, `system`, `events` or `errors`. |
| `state` | `ProjectSizeState` | `measured`; `absent` (no such file, or, staged, none in the index, a file deleted in the commit among them); or `unreadable` (it also appears in `skipped`, and staged, an unresolved conflict or an unreadable blob is one). A missing spec subject is never reported, but a missing project file is, so a project with no `events.md` reads differently from one whose `events.md` was measured. |
| `bytes` | integer, optional | The length on disk, or, staged, the index copy's. Present exactly when `state` is `measured`. |
| `head-bytes` | integer, optional | Staged only: the `HEAD` copy's length. Absent when `HEAD` has none, or the repository has no commits. |
| `refused` | boolean, optional | Staged only, for a measured file: the staged copy is over the threshold and either has no `HEAD` copy or is larger than it. An unchanged or shrunk over-limit file is not refused. |

`PartSize`:

| Field | Type | Meaning |
| --- | --- | --- |
| `path` | string | Project-relative path. |
| `project-file` | string | The project-relative path of the project file whose parts directory holds it. |
| `state` | `ProjectSizeState` | `measured`, or `unreadable` (it also appears in `skipped`): a part that is, or lies under, a symbolic link leaving the project is never opened, and staged, an unresolved conflict or an unreadable blob is one. Never `absent`, since only what exists is listed. |
| `bytes` | integer, optional | The length on disk, or, staged, the index copy's. Present exactly when `state` is `measured`. |
| `linked` | boolean, optional | Whether its project file links it. Absent staged, and when its project file exists but could not be read, so its links are unknown; an absent project file links nothing. |
| `head-bytes` | integer, optional | Staged only: the `HEAD` copy's length, as a project file's. |
| `refused` | boolean, optional | Staged only, for a measured part: as a project file's. A part deleted in the commit is no entry, so it is never refused. |

`PartFinding`:

| Field | Type | Meaning |
| --- | --- | --- |
| `kind` | `PartFindingKind` | `missing-part` (a project file's link resolves into a parts directory, to no file); `unlinked-part` (a part its own project file does not link; a link from another project file is an ordinary link); `nested-file` (a markdown file at any depth in a subdirectory of a parts directory); `not-a-parts-directory` (a directory under `.ductus/docs/` other than the four and `constitution/`, a shared constitution's); or `not-a-part` (a markdown file directly under `.ductus/docs/`). |
| `path` | string | The missing part, the unlinked part, the nested file, the stray directory or the stray file. |
| `message` | string | The finding, worded by the runtime, with its fix. |

A link is read outside fenced blocks and HTML comments, with a fragment stripped and a root-absolute target resolved against the project root. None of the five holds a spec's `done` or stops a commit.

A project file over the threshold joins `oversized` with kind `project-file`, whose one fix is `prune-project`, and whose `warning` names no reopen whatever the feature's status. Its `warning` and `message` are worded as a spec subject's, and a stored waiver in the named feature's `analysis.md` covers it by the decided rule below. Staged, `oversized` is judged on the staged copy. The CLI, with `--staged`, exits 1 when any file is refused or `skipped-count` is above zero, so in either form, naming each on stderr, an unmeasurable one from `project-files` (`ductus check-artifact-size --help`); a repository or `HEAD` that cannot be read is an error, never a clean result, and an index that cannot be read leaves each project file `unreadable` and in `skipped`, with `.ductus/docs` skipped too, so the exit is 1.

A part over the threshold joins `oversized` with kind `project-part`, whose one fix is `split-part`, and whose `warning`, like a project file's, names no reopen. Its `message` is the `artifact-size` finding message, so a stored waiver covers it by the decided rule below. Staged, it is judged on the staged copy.

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
| `kind` | `ArtifactKind` | `spec`, `plan`, `tasks`, `data-model`, `scenario`, `project-file`, or `project-part`. |
| `bytes` | integer | The file's length on disk. |
| `pages` | integer | `ceil(bytes / threshold)`; at least 2 for every entry here. |
| `fixes` | list of `ArtifactFix` | The fixes for this kind, in the spec's table order. Never includes a waiver. |
| `warning` | string | The rendered one-line warning the commands print. On a `done` spec it ends with one sentence: a fix reopens the spec only if it leaves an unchecked box in its `tasks.md` or an unticked criterion in its `spec.md` (spec 084). |
| `message` | string | The `artifact-size` finding message, in the format below. |
| `decided` | boolean | A stored waiver still covers this subject (the decided rule below). Always `false` unless `decisions` is `read`. |
| `decision-key` | string, optional | The stored decision's key. Present exactly when `decided` is `true`. |

## `ArtifactFix`

| Field | Type | Meaning |
| --- | --- | --- |
| `fix` | `FixKind` | `prune`, `split`, `trim`, `promote`, `prune-project`, or `split-part`. |
| `text` | string | The fix as the warning words it. A `split` names the manual route and no split command. |

Fixes by kind: `tasks` → `prune`, then `split` and `trim` as the fixes that apply if it is still over after pruning; `spec` and `plan` → `split`, `trim`; `data-model` → `split`, `trim` (whose text forbids sub-files); `scenario` → `promote`; `project-file` → `prune-project`; `project-part` → `split-part`.

## `DecisionsState`

`absent` (no `analysis.md`), `read` (the list was read, and is empty when the record stores no decisions), or `unreadable` (the list could not be read or does not parse; nothing is decided, and a notice says so). The primitive never fails on an unreadable list.

## The `artifact-size` finding

`/{project}:analyze` records each oversized subject, a project file or a part included, as an advisory finding with family `artifact-size`, path the subject's, and message:

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
