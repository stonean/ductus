# 041 — Spec Directory Pruning Data Model

Defines the structures the two prune primitives own. `prune-tasks` reduces
`tasks.md`: the in-memory **segmentation** it builds, the per-section
**classification**, the two reduction **modes**, and its request/response
**schema**. `prune-plan` reduces `plan.md`: its section segmentation, the
**design record** a section is classified against, the **finding** it shares
with `/{project}:analyze`, and its schema (see *`prune-plan`* below). The
**serialized** types — `PruneTasksArgs`, `PruneMode`, `Classification`,
`PruneGate`, `PruneAction`, `SizeSummary`, `PruneSection`,
`PruneTasksSummary`, `PruneTasksLine`, `PruneTasksResult`, `PrunePlanArgs`,
`PlanRemoval`, `PlanFinding`, `PlanSection`, `PrunePlanSummary`,
`PrunePlanResult`, `PruneWalk`, `PruneWalkEntry`, `SkippedFeature` and
`SkipReason` — live in
`runtime/src/schema/primitives.rs` with `serde::{Serialize, Deserialize}`
derives; their serialized JSON is the stable host contract, consistent with
the primitive-schema convention in
[022 — Deterministic Runtime](../022-deterministic-runtime/data-model.md).

The parsing reuses the existing `tasks.md` machinery in
`runtime/src/primitives/mod.rs` — `detect_tasks_structure`, `parse_atx_heading`,
`split_numbered_heading`, `SkipScanner`, and the `checkbox::find_checkbox_line`
helper — so `prune-tasks` recognizes exactly the same task set as `read-tasks`
and `mark-task`. Nothing about the grammar is re-invented. It does **not** use
`iter_phase_ranges` (which `append-task` does): that helper yields line ranges,
while the reduction needs each task's governing phase as an *index into its own
block list* so an emptied phase container can be dropped, so `segment` tracks
the current phase inline as it walks.

## Segmentation

A `tasks.md` is segmented into one ordered `Vec<Block>`; the preamble is
simply the first `Structure` block, not a separate field. Structure detection is shared: `detect_tasks_structure` yields
`Flat` (task headings at level 2, `## N.`) or `Phased` (task headings at
level 3, `### N.`, under `## …` phase containers). `task_level` is 2 or 3
accordingly.

```rust
enum PruneMode { KeepPending, Reset }

/// Private to `primitives/prune_tasks.rs` — never serialized. A block owns
/// its own lines, so keep-pending rebuilds the file by concatenating the
/// survivors rather than by slicing ranges out of the original.
enum Kind {
    /// The preamble, the `# …` heading, and any other non-task heading
    /// group. Always kept.
    Structure,
    /// Phased files only: a `## …` non-numeric heading. Dropped only when
    /// it governs at least one task section and every one is dropped; a
    /// phase governing none is kept.
    Phase,
    /// A numbered task section. Dropped when spent.
    Task,
}

struct Block {
    kind: Kind,
    lines: Vec<String>,            // the block's own lines, verbatim
    number: String,                // "1", "12", … (Task only)
    heading: String,               // title text, sans the `N.` prefix
    phase: Option<String>,         // containing phase heading (phased only)
    checkbox_total: u32,           // task-list checkboxes in the section
    checkbox_checked: u32,         // of which are `[x]`
    governing_phase: Option<usize>, // index of the phase container, if any
}

enum Classification {
    /// >= 1 checkbox and every checkbox is checked. Removable.
    Spent,
    /// >= 1 checkbox and at least one is unchecked. Always preserved.
    Pending,
    /// Zero checkboxes (prose/structural task). Always preserved — never
    /// classified spent.
    NoCheckbox,
}
```

A section's checkboxes are counted with `checkbox::find_checkbox_line`,
which already excludes `- **Done when**:` lines (they are not `[ ]`/`[x]`
markers). A `Task` block's lines run up to the next heading whose level is
`<= task_level` — identical to `mark-task`'s `locate_task_range`.

**Classification rule.**

| Checkboxes present | All checked | Classification | Removable |
| --- | --- | --- | --- |
| ≥ 1 | yes | `Spent` | yes |
| ≥ 1 | no | `Pending` | no |
| 0 | — | `NoCheckbox` | no |

## Reduction modes

### keep-pending (default)

Output, in document order:

For each `Block`, in order:

- `Structure` (the preamble, the `# …` heading, any other non-task heading
  group) → **kept verbatim**.
- `Task` classified `Spent` → **dropped**.
- `Task` classified `Pending` or `NoCheckbox` → **kept verbatim** (its own
  already-checked boxes included; prune never edits a section's interior).
- `Phase` → **dropped only when the reduction empties it**: it governs at
  least one `Task`, and every one it governs is dropped. A phase that
  governs no `Task` is kept, since the prune did not empty it.

Seams between kept blocks are normalized to a single blank line and the file
ends with exactly one trailing newline, so the result is `markdownlint`-clean.

When no section is `Spent`, no block is dropped, so the computed output
equals the input, flat or phased: the primitive sets `nothing-to-prune: true`
and writes nothing even under `apply: true`.

### reset (`--reset`)

Output is the feature's identity plus a canonical empty task body:

```text
<existing first H1 line>

<CANONICAL_EMPTY_TASKS_BODY>
```

`<existing first H1 line>` is the file's first `# …` heading (preserves
feature identity, e.g. `# 041 — Spec Directory Pruning Tasks`).
`<CANONICAL_EMPTY_TASKS_BODY>` is a constant embedded in the primitive equal
to `framework/templates/spec/tasks.md` with its own H1 line removed (the
intro line + the guidance comment). A unit test asserts the constant equals
that template body so the two never drift. This satisfies the acceptance
criterion "restores `tasks.md` to the template's initial state (heading plus
guidance comment)" without coupling the primitive to a template path at
runtime.

If the file has no `# …` heading, reset cannot preserve identity: the
primitive errors (`malformed-tasks`) and writes nothing. When the computed
reset output already equals the file content, `nothing-to-prune: true` and no
write occurs (idempotent).

## Primitive request/response schema

### `prune-tasks` — reduce a feature's `tasks.md`

Args:

```json
{ "feature": "041-task-pruning", "reset": false, "force": false, "apply": false }
```

- `feature` — feature directory under the configured spec-root. Exactly one
  of `feature` and `all` is given (see *Walking every spec* below).
- `reset` — `false` = keep-pending; `true` = full reset.
- `force` — override the `reset` status gate on a non-`done` spec.
- `apply` — `false` = preview (compute + classify, **no write**); `true` =
  write the reduced file atomically.

Result for one feature (a **compact summary — never the file body**):

```json
{
  "mode": "keep-pending",
  "applied": false,
  "gate": "not-applicable",
  "nothing-to-prune": false,
  "removed-count": 3,
  "kept-count": 2,
  "size-before": { "lines": 412, "bytes": 18234 },
  "size-after":  { "lines": 180, "bytes": 7920 },
  "sections": [
    { "number": "1", "heading": "Wire crate", "phase": "Phase A — Bootstrap",
      "classification": "spent",   "checkbox-total": 4, "checkbox-checked": 4, "action": "removed" },
    { "number": "2", "heading": "Add CLI",    "phase": "Phase A — Bootstrap",
      "classification": "pending", "checkbox-total": 3, "checkbox-checked": 1, "action": "kept" }
  ],
  "path": "specs/041-task-pruning/tasks.md"
}
```

- `mode` — `"keep-pending"` or `"reset"`, echoing the resolved mode.
- `applied` — whether a write happened (`false` on preview, on
  `nothing-to-prune`, and on a blocked reset).
- `gate` — `"not-applicable"` for keep-pending; for reset, `"allowed"`
  (status is `done`, or `force` supplied) or `"blocked-needs-force"` (status
  is not `done` and `force` absent). A blocked reset is a **domain outcome**,
  not an operational error: the primitive returns `applied: false`,
  `gate: "blocked-needs-force"`, and writes nothing. The command surfaces the
  refusal (name the status, point at keep-pending, mention `--reset --force`).
- `status` — the spec's frontmatter status, read from `spec.md` only when
  `reset` is true. On keep-pending the field is `None` and is **omitted from
  the JSON entirely** (`skip_serializing_if`) rather than serialized as
  `null`, which is why the example above carries no `status` key — a host
  must treat it as absent, not as a present null.
- `sections` — one compact record per task section: its identity, its
  classification, checkbox counts, and the `action` taken
  (`"removed"` | `"kept"`). Bounded by the task count; the section **bodies
  are never included**, which is the token-reduction contract that motivates
  the primitive doing its own write (per the runtime-eligibility resolution
  in [spec.md](spec.md)).

### Operational errors

Reported as `error` envelopes (not result fields). The primitive writes
nothing to the spec an error fires on; an applying `all` walk keeps the
reductions it already wrote to the specs it reached before the error, and
running it again resumes:

| Code | Condition |
| --- | --- |
| `feature-not-found` | feature directory absent under the spec-root |
| `tasks-file-missing` | feature directory exists but has no `tasks.md` (the command directs the user to `/{project}:plan` when there is no `plan.md` either, and otherwise reduces the plan and reports the missing task list); a walk skips such a spec instead |
| `malformed-tasks` | file has no `# …` heading (reset cannot preserve identity) |
| `missing-spec-file` / `status-field-missing` / `missing-frontmatter` | `spec.md` is absent, opens no frontmatter, or its frontmatter has no `status`, when a `reset` reads it for its gate or a walk reads it for a spec it lists |
| `yaml` | that `spec.md` frontmatter does not parse |
| `missing-argument` | neither `feature` nor `all` |
| `invalid-argument` | both `feature` and `all`; `force` with `all` |

## `prune-plan`

### Segmentation

A `plan.md` is segmented into an ordered list of blocks. Every line is fed
through `SkipScanner` first, so a heading inside a fenced block or an HTML
comment is not structure. A block opens at each heading of level 2 or
above: a `##` heading opens a **section**, and a `#` heading opens a
structural block. The lines before the first heading are the **preamble**.
Preamble and structural blocks are always kept; a `###` or deeper heading
stays inside the section that holds it.

A section's **text** is its lines from the heading to its last non-blank
line, joined by `\n`. The digest and the size both read the text, so a
blank line at a seam changes neither.

### Classification

A section is **design record** when its heading, trimmed, equals one of the
plan template's own `##` headings under ASCII case folding:

| Heading | Presence in a plan |
| --- | --- |
| Overview | Always |
| Technical Decisions | Always |
| Affected Files | Always |
| Data Model | Optional |
| Trade-offs | Always |
| Open Questions Resolved | Optional |
| Cross-spec impact | Optional |

Every other section is **outside the record**. Nothing but case is forgiven:
*Tradeoffs* is outside it. The set is a constant in the primitive, held to
`framework/templates/spec/plan.md` by a unit test that parses the template's
`##` headings and asserts equality, in template order — the same pinning, and
the same bounded divergence for a customized template, as the `--reset` body.

### Finding

Each section outside the record carries the advisory `/{project}:analyze`
records for it, built by one function, `plan_section_finding(heading)`:

- `family` — `plan-record`.
- `message` — `plan.md §{heading} is outside the design record`.

`write-analysis` keys a stored decision `{family} — {message}`, so the key is
fully determined by the heading. A section is **decided** when `analysis.md`'s
`decisions:` list holds a well-formed `discarded` entry under that key,
compared with the stored-decision key normalization every writer uses. A
routed entry does not decide a section, and a key analyze re-matched by
judgment — kept from an earlier wording — is left for the host to match.

### Request/response schema

Args:

```json
{ "feature": "041-task-pruning", "apply": true,
  "remove": [{ "heading": "Implementation notes", "digest": "9f2c…" }] }
```

- `feature` — feature directory under the configured spec-root. Exactly one
  of `feature` and `all` is given (see *Walking every spec* below).
- `apply` — `false` (the default) is a preview and writes nothing; `true`
  removes the sections `remove` lists.
- `remove` — each section to remove, named by the heading and digest a
  preview reported. On the CLI, one `--remove <digest>:<heading>` per
  section; the digest is hex, so the first `:` ends it.

Result (a **compact summary — never a section's text**):

```json
{
  "path": "specs/041-task-pruning/plan.md",
  "missing": false,
  "status": "done",
  "sections-examined": 5,
  "sections": [
    { "heading": "Implementation notes", "ordinal": 3, "lines": 3, "bytes": 42,
      "digest": "9f2c…",
      "finding": { "family": "plan-record",
                   "message": "plan.md §Implementation notes is outside the design record" },
      "decided": false }
  ],
  "applied": true,
  "stale-sections": [],
  "size-before": { "lines": 30, "bytes": 800 },
  "size-after":  { "lines": 26, "bytes": 740 },
  "reopen-required": false
}
```

- `missing` — the feature has no `plan.md`. Nothing is examined and nothing
  is written. A domain outcome, not an error: `/{project}:analyze` dispatches
  the preview on every spec, and a spec below `planned` has no plan.
- `status` — the spec's frontmatter status, read on every call.
- `sections-examined` — `##` sections read, inside the record and out, so an
  empty `sections` over `sections-examined: 0` (no sections) reads
  differently from one over `sections-examined: 5` (all in the record). It is
  named apart from a walk's `examined`, which counts features, because a
  result's two halves share one key space (below).
- `sections` — every section outside the record, as read before any removal,
  decided or not: `ordinal` is its 1-based position among all `##` sections,
  `lines` and `bytes` measure its text, and `digest` is the lowercase-hex
  sha256 of its text.
- `applied` — whether the listed sections were removed.
- `stale-sections` — each heading in `remove` with no section of that heading
  and digest left to claim, because the section changed or went since the
  preview. Non-empty is the **`stale-sections` domain outcome**: the whole
  apply is refused and nothing is written, since the host's moves were
  judged against the text it read. Duplicate headings each claim a distinct
  section.
- `size-after` — on a preview, the size were every undecided section removed;
  on an apply, the size written, or `size-before` when it was refused.
- `reopen-required` — present on every call against a `done` spec, the one
  status prune reopens; absent means not computed, never `false`. It is `true`
  when `tasks.md` adds an unchecked checkbox: holds an unchecked checkbox
  line, trimmed and outside fenced blocks and HTML comments, more times than
  `tasks.md` at HEAD holds that line, so a box moved or re-indented adds
  nothing and one added or reworded does. An edit to `plan.md` never sets it,
  whatever it changes, since §spec-lifecycle gives a plan edit no back-edge
  on a `done` spec (scenario
  `a-plan-edit-does-not-reopen-a-done-spec`). A `tasks.md` absent at HEAD
  triggers nothing. A preview answers for the tree as it stands, so a reopen
  from edits made before the run is known before the host writes anything. An apply answers for the tree it leaves, whether or
  not the removal was refused, because the host's moves are already on disk,
  and computes it before its own write, so an apply that cannot read HEAD
  writes nothing. HEAD is read through `ProjectRepository::read_at_head`, so
  a project in a subdirectory of its repository resolves. The command, not
  the primitive, performs the reopen with `set-status`, and decides it on a
  fresh preview taken once the run has finished writing to the spec, so the
  reopen follows the tree the run leaves.

A removal whose sections are removed has its seams normalized as keep-pending's
are, and is written with `write_atomic`, preserving the file's line endings.

### Operational errors

Reported as `error` envelopes; the primitive writes nothing when any fires:

| Code | Condition |
| --- | --- |
| `feature-not-found` | feature directory absent under the spec-root |
| `missing-spec-file` / `status-field-missing` / `missing-frontmatter` | `spec.md` is absent, opens no frontmatter, or its frontmatter has no `status` |
| `missing-argument` | `apply` with an empty `remove`; neither `feature` nor `all` |
| `invalid-argument` | `remove` without `apply`, or naming a design-record section, which prune never removes; both `feature` and `all`; `all` with `apply` |
| `yaml` | `analysis.md`'s `decisions:` list does not parse — read as empty, it would propose every section already decided |
| `git` | a call against a `done` spec cannot read HEAD for the reopen trigger |

## Walking every spec

Both primitives take `all: bool`, and exactly one of `feature` and `all` is
given: neither is `missing-argument`, both is `invalid-argument`. A walk
visits every feature directory `list_feature_dirs` recognizes under the
spec-root, sequential and branch-scoped alike, in `feature_dir_cmp` order —
sequential by number, then branch-scoped grouped by identifier with the
counter compared numerically, so `1234.2-x` precedes `1234.10-x`. It needs no
session target and writes none.

`prune-tasks` refuses `force` with `all` before reading anything: a forced
reset across the corpus would discard every in-flight todo under one
confirmation. With `apply` it writes every spec's permitted reduction, the
`--reset` gate applied per spec. `prune-plan` refuses `apply` with `all`,
before it checks the `remove` list: each plan section is a judgment about one
spec, so a walk is preview-only.

Each result is one object with two flattened, optional halves: a
single-feature call carries the summary above at the top level, unchanged,
and a walk carries only the walk:

```json
{
  "examined": 60,
  "features": [
    { "feature": "000-slash-commands", "gate": "not-applicable", "status": "done",
      "applied": false, "removed-count": 18, "kept-count": 0,
      "size-before": { "lines": 134, "bytes": 7514 },
      "size-after":  { "lines": 9, "bytes": 418 },
      "path": "specs/000-slash-commands/tasks.md" }
  ],
  "skipped": [ { "feature": "063-draft", "reason": "no-tasks-file" } ]
}
```

- `examined` — every feature directory walked, the skipped and the
  nothing-to-reduce ones included, so a clean corpus reads as examined rather
  than as silence.
- `features` — each spec with something to report, in walk order: for
  `prune-tasks` a spec whose reduction is not a no-op, as a `PruneTasksLine`;
  for `prune-plan`, as the full `PrunePlanSummary`, a spec whose plan holds a
  section outside the record, and a `done` spec whose `reopen-required` is
  `true` even when it has no such section or no `plan.md`, so a reopen from
  edits made before the run reaches the corpus preview.
- `skipped` — for `prune-tasks`, each spec without a `tasks.md`
  (`no-tasks-file`); for `prune-plan`, each spec without a `plan.md` and
  nothing else to report (`no-plan-file`). Any other error stops the walk, as
  it would stop a single-feature call. A `prune-tasks` walk reads a listed
  spec's status before an apply writes its reduction, so a status that will
  not read stops the walk with that spec's `tasks.md` untouched.

A `PruneTasksLine` is the summary without its per-section records — `gate`,
`status`, `applied`, the two counts, the two sizes and `path`. A corpus
preview prices a spec by its counts; the records would carry every task
section in the corpus, 842 of them and 161,841 bytes over this repository's
60 specs when the walk was built, against 15,373 bytes as lines, status included. `status` is
read for every line, keep-pending included, because only a `done` spec can be
reopened and the corpus preview prices each row by it. A plan summary keeps
its sections, since the host judges each one by heading.

The two halves of a result share one key space, so no key may appear in
both: a summary field named like `examined` would be consumed by the half
deserialized first and lost to the other, which is why `prune-plan`'s
section count is `sections-examined`.

## Notes

- All paths are repo-relative with `/` separators on every platform.
- The write uses the shared `write_atomic` (tempfile + rename); a crash
  mid-write leaves `tasks.md` unchanged, and no backup sidecar is produced
  (recovery is git history).
- `size-before`/`size-after` report both line and byte counts so the command
  can render a "materially smaller" summary without seeing the content.
