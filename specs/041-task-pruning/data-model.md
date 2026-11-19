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
accordingly. Every line goes through `SkipScanner`; a fenced block or HTML
comment still open at the end of the file leaves the lines from its opener on
in no block, so the result reports them as its **unexamined tail** (the
`unexamined-tail` bullet below), never as structure.

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

`<existing first H1 line>` is the file's first `# …` heading below its
frontmatter block, outside a fence or comment (preserves feature identity,
e.g. `# 041 — Spec Directory Pruning Tasks`); a `#` line inside the
frontmatter is a YAML comment, never the heading.
`<CANONICAL_EMPTY_TASKS_BODY>` is a constant embedded in the primitive equal
to `framework/templates/spec/tasks.md` with its own H1 line removed (the
intro line + the guidance comment). A unit test asserts the constant equals
that template body so the two never drift. A frontmatter block the file
opens with is kept whole above this output, one blank line before the H1 (see
*The `next-task:` counter*). This satisfies the acceptance
criterion "restores `tasks.md` to the template's initial state (heading plus
guidance comment)" without coupling the primitive to a template path at
runtime.

If the file has no `# …` heading, reset cannot preserve identity: the
primitive errors (`malformed-tasks`) and writes nothing. When the computed
reset output already equals the file content, `nothing-to-prune: true` and no
write occurs (idempotent).

### The `next-task:` counter

A `tasks.md` may open with a frontmatter block whose `next-task:` field is a
positive integer: the number the next appended task takes. It is a high-water
mark, as `next-criterion` is for criteria, and keeps a number a prune removed
from being assigned again.

- The counter's rules are `ductus prune-tasks --help`'s and `ductus append-task --help`'s: when each writes it, how the highest number is read, what an absent block, an existing block and a malformed one do, and what `--reset` keeps. `append-task` writes the counter only where one exists, so a file that was never pruned is written as it always was.

Every other `tasks.md` reader scans for headings and checkboxes, so the block
is invisible to `read-tasks`, `mark-task`, `check-stuck` and `check-artifacts`.
The helpers are `runtime/src/primitives/tasks_counter.rs`.

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
  write the reduced file atomically, **only when it is unchanged since the
  apply read it** (see *The write guard*).

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
  "sections-count": 2,
  "path": "specs/041-task-pruning/tasks.md",
  "form": "inline"
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

- `unexamined-tail` — present only when `tasks.md` ends inside an open fenced
  block (`cause: open-fence`) or HTML comment (`open-comment`). `from-line` is
  the 1-based line of the whole file the region opens on: it and every later
  line are in no section and were not examined, so the file never reads as
  clean. An apply, in either mode, writes nothing (`applied: false`) until the
  region is closed, and the preview still reports the sections it read. Absent
  means no region was open, never that none was checked.
- `sections-count`, `form`, `lists-file` — `sections` is inline while the
  result serialized fits `[artifacts] read-size-bytes`; past it `form` is
  `file`, `sections` is absent, `sections-count` stays inline in both forms,
  and `lists-file` names the JSON file under `.ductus/results/` holding
  `sections` whole. `removed-count` and `kept-count` count no list and stay
  inline. `form` and `lists-file` belong to the result, not to the summary
  (see *Walking every spec*). The bound is stated by
  `ductus prune-tasks --help`; a malformed read size halts the call before
  any write.

### The write guard

Every apply of the three primitives is a read-modify-write, so each guards the
window between its read and its rename (`BE-TXN-002`) by a compare before
persist: it keeps what it read, and the write reads the file again under one
project-wide advisory lock and renames the result over it only when the two
reads match. A file changed since the read is **refused, never forced**: it is
left as the other writer made it (an `append-task`, a `mark-task`, an editor),
and the host re-runs the command to read it as it now is (`BE-TXN-004`). The
lock makes the compare and the rename atomic among the writers that go through
the guard, so two cannot both pass; it is taken after `append-task`'s lock when a caller holds
both, waits no longer than the session lock's timeout, and is held for one
re-read and one rename. A writer that does not go through the guard takes no lock, so one
that lands between the re-read and the rename is still replaced: the guard narrows
that window and does not close it. The helper, its strategy and its bounds are
022's plan (*Technical Decisions*, the compare-and-set guard).

What a refusal reports:

- **A single file** — `prune-tasks` for one feature and `prune-plan` — fails
  the call with the `changed-since-read` operational error naming the file; no
  result summary is returned and nothing is written.
- **A `prune-tasks` walk** (`all` with `apply`) writes every other spec and
  lists each refused spec in `skipped` with reason `changed-since-read`, so its
  `features` (the lines whose `applied` is `true`) and its `skipped` say what it
  wrote and what it refused.
- **`prune-project`**, which rewrites up to four files, goes on past a refused
  file and reports `written` (the files rewritten) and `refused` (the files
  changed since the read, each with its listed removals and replacements
  unapplied); `applied` is `true` only when nothing was refused. A home to
  `create` is written create-only, which refuses to replace any file, so a home
  that appeared since the apply looked is `kept` and needs no read to compare
  against.

### Operational errors

Reported as `error` envelopes (not result fields). The primitive writes
nothing to the spec an error fires on; an applying `all` walk keeps the
reductions it already wrote to the specs it reached before the error, and
running it again resumes:

| Code | Condition |
| --- | --- |
| `feature-not-found` | feature directory absent under the spec-root |
| `tasks-file-missing` | feature directory exists but has no `tasks.md` (the command directs the user to `/{project}:plan` when there is no `plan.md` either, and otherwise reduces the plan and reports the missing task list); a walk skips such a spec instead |
| `changed-since-read` | an apply's `tasks.md` changed between its read and its write: nothing is written and the other writer's change stands; a walk lists the spec in `skipped` instead |
| `malformed-tasks` | file has no `# …` heading (reset cannot preserve identity), or its frontmatter never closes or carries a `next-task:` that is not a positive integer, or its highest task number is `4294967295`, which no counter can follow |
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

A fenced block or HTML comment still open at the end of the file leaves the
lines from its opener on in no block, so the result reports them as its
**unexamined tail** (the `unexamined-tail` bullet below), never as structure.

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
`decisions:` list holds a well-formed `waived` entry by a person under that
key, compared with the stored-decision key normalization every writer uses. A
deferred entry does not decide a section, nor does a waiver stamped
`decided-by: rule`, since the disposition rule waives every `plan-record`
finding it does not fix (spec 079), and a key analyze re-matched by judgment —
kept from an earlier wording — is left for the host to match.

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
  "sections-count": 1,
  "applied": true,
  "stale-sections": [],
  "size-before": { "lines": 30, "bytes": 800 },
  "size-after":  { "lines": 26, "bytes": 740 },
  "reopen-required": false,
  "form": "inline"
}
```

- `missing` — the feature has no `plan.md`. Nothing is examined and nothing
  is written. A domain outcome, not an error: `/{project}:analyze` dispatches
  the preview on every spec, and a spec below `planned` has no plan.
- `status` — the spec's frontmatter status, read on every call.
- `sections-examined` — `##` sections read, inside the record and out, so a
  zero `sections-count` over `sections-examined: 0` (no sections) reads
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
  holds no box, so each one written since is new, as `classify-spec-edit`
  reads it (spec 079). When it is computed, and for which tree, is as
  `ductus prune-plan --help` states, so a reopen from edits made before the
  run is known before the host writes anything, and an apply that cannot read
  HEAD writes nothing. HEAD is read through `ProjectRepository::read_at_head`, so
  a project in a subdirectory of its repository resolves. The command, not
  the primitive, performs the reopen with `set-status`, and decides it on a
  fresh preview taken once the run has finished writing to the spec, so the
  reopen follows the tree the run leaves.

- `unexamined-tail` — as `prune-tasks`': present only when `plan.md` ends
  inside an open fenced block or HTML comment, `from-line` is a line of the
  whole file, and an apply removes nothing (`applied: false`), whatever `remove`
  names, so a design-record section the open region swallowed into another
  section cannot be removed.
- `sections-count`, `form`, `lists-file` — `sections` is inline while the
  result serialized fits `[artifacts] read-size-bytes`; past it `form` is
  `file`, `sections` is absent, `sections-count` stays inline in both forms,
  and `lists-file` names the JSON file under `.ductus/results/` holding
  `sections` whole, as `prune-tasks`' does. The bound is stated by
  `ductus prune-plan --help`. A malformed read size halts the call before
  any write.

A removal whose sections are removed has its seams normalized as keep-pending's
are, and is written only when `plan.md` is unchanged since the apply read it
(*The write guard* above), preserving the file's line endings. A digest that no
longer matches is `stale-sections`; a change anywhere else in the file, which
leaves every listed digest matching, is the `changed-since-read` error.

### Operational errors

Reported as `error` envelopes; the primitive writes nothing when any fires, except that a list file that cannot be written after an applied removal fails the call and the removal stands:

| Code | Condition |
| --- | --- |
| `feature-not-found` | feature directory absent under the spec-root |
| `missing-spec-file` / `status-field-missing` / `missing-frontmatter` | `spec.md` is absent, opens no frontmatter, or its frontmatter has no `status` |
| `missing-argument` | `apply` with an empty `remove`; neither `feature` nor `all` |
| `invalid-argument` | `remove` without `apply`, or naming a design-record section, which prune never removes; both `feature` and `all`; `all` with `apply` |
| `changed-since-read` | an apply's `plan.md` changed between its read and its write, in a section it does not list as much as in one it does: nothing is written and the other writer's change stands; the host previews again |
| `yaml` | `analysis.md`'s `decisions:` list does not parse — read as empty, it would propose every section already decided |
| `git` | a call against a `done` spec cannot read HEAD for the reopen trigger |
| `invalid-read-size` / `toml` | `[artifacts] read-size-bytes` is not a whole number of bytes, at least 1, or the config does not parse; the message reads ``invalid `[artifacts] read-size-bytes` in <config path>: <value> — …``. Read before any write, so an applying call halts before it removes anything |

## Walking every spec

Both primitives take `all: bool`, and exactly one of `feature` and `all` is
given: neither is `missing-argument`, both is `invalid-argument`. A walk
visits every feature directory `list_feature_dirs` recognizes under the
spec-root, sequential and branch-scoped alike, in `feature_dir_cmp` order —
sequential by number, then branch-scoped grouped by identifier with the
counter compared numerically, so `1234.2-x` precedes `1234.10-x`. It needs no
session target and writes none.

`prune-tasks` refuses `force` with `all`, and `prune-plan` refuses `apply` with
it, as `ductus prune-tasks --help` and `ductus prune-plan --help` state; the
first refusal comes before `prune-tasks` reads anything and the second before
`prune-plan` checks the `remove` list. A forced reset across the corpus would
discard every in-flight todo under one confirmation, and each plan section is a
judgment about one spec, so a walk is preview-only. With `apply`, `prune-tasks`
writes every spec's permitted reduction, the `--reset` gate applied per spec.

Each result is one object with two flattened, optional halves and the
result's own `form` and `lists-file`: a single-feature call carries the
summary above at the top level, and a walk carries only the walk:

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
  "features-count": 1,
  "skipped": [ { "feature": "063-draft", "reason": "no-tasks-file" } ],
  "skipped-count": 1,
  "form": "inline"
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
- `unexamined-count` — the listed features carrying an `unexamined-tail`; a
  spec with one is listed whatever else it has to reduce, and the other specs
  are still reduced. Absent at zero, inline in both forms, like `features-count`.
- `skipped` — for `prune-tasks`, each spec without a `tasks.md`
  (`no-tasks-file`), and under `apply` each spec whose `tasks.md` changed
  after the walk read it (`changed-since-read`; *The write guard*); for `prune-plan`, each spec without a `plan.md` and
  nothing else to report (`no-plan-file`). Any other error stops the walk, as
  it would stop a single-feature call. A `prune-tasks` walk reads a listed
  spec's status before an apply writes its reduction, so a status that will
  not read stops the walk with that spec's `tasks.md` untouched.

Each result is held to `[artifacts] read-size-bytes` as it is sent, a walk
as a whole: past it `form` is `file`, `features` and `skipped` are absent
with `features-count` and `skipped-count` inline, and `lists-file` names the
file under `.ductus/results/` holding both whole, for both primitives, as
`ductus prune-tasks --help` and `ductus prune-plan --help` state. A
`prune-plan` entry is a `PrunePlanSummary`, so it carries `sections-count`
beside its `sections` and no `form`; the bound applies to the walk, never to
an entry.

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
section count is `sections-examined`, and why `form` and `lists-file` belong
to the result, declared after the halves so they stay last on the wire, and
to neither half. A result deserializes back through its type with both
intact, in either form.

## `prune-project`

The deterministic half of `/{project}:prune --project` (scenario `prune-walks-the-project-files`). It takes no feature: its subjects are the four project files 075 defines, in this order — `AGENTS.md` at the project root, then `system.md`, `events.md` and `errors.md` under the spec-root.

### Segmentation

Every line goes through `SkipScanner`, so nothing inside a fenced block or an HTML comment is structure or an entry. The lines between a `<!-- BEGIN {marker} -->` and its `<!-- END {marker} -->`, as `merge-managed-block` writes them, are never an entry either: `/ductus` owns them. A comment or managed region still open at the end of the file leaves the lines from its opening line on in no entry, so the file's record names them as its **unexamined tail** (below), never as walked. An unterminated fence is not one: it is a `block` entry through the end of the file. A `#` or `##` heading is structure, never an entry. An entry is one of:

- **`heading`** — a `###`-or-deeper heading and every line up to the next heading at its level or above.
- **`item`** — inside a `##` section's own body (before its first `###`), a top-level list item and every line up to the next top-level item or blank-line-separated block.
- **`row`** — a table body row. A table's header and delimiter rows are structure.
- **`block`** — any other blank-line-separated run of lines in a `##` section's own body or the preamble.

An entry's **text** is its lines from its first to its last non-blank line, joined by `\n`. Its `digest` is the lowercase-hex sha256 of that text.

### Catalog identifiers and the literal search

An `events.md` or `errors.md` entry carries an `identifier`. It is the first code span on the entry's first line (for a `row`, in its first cell), or, for a `heading` entry with none, the heading text trimmed. An entry with neither carries no identifier.

The search reads every file the project's repository tracks outside the spec-root, except the four subjects, once per preview, through `ProjectRepository::discover`. It reports each file holding an identifier as a literal byte substring. An identifier built at runtime does not appear literally, which is why the walk words zero hits as *no literal occurrence found*.

### Request/response schema

Args:

```json
{ "apply": true,
  "remove":  [{ "path": "AGENTS.md", "digest": "9f2c…" }],
  "replace": [{ "path": "AGENTS.md", "digest": "4ab1…",
                "with": "- **Shared knowledge stays in git.** [§drift-prevention](.ductus/constitution.md#drift-prevention)." }],
  "create":  ["system"] }
```

- `apply` — `false` (the default) is a preview and writes nothing.
- `remove` — each entry to delete, by the path and digest a preview reported. On the CLI, one `--remove <digest>:<path>` per entry.
- `replace` — each entry to replace with the text `with`, which may span lines. On the CLI, `--replace-json FILE` carries the list, as the primitive's description states, since the text does not fit a scalar flag (022 task 24).
- `create` — the homes to create from their template, from `system`, `events` and `errors`. On the CLI, one `--create <kind>` each.

Result (never an entry's text):

```json
{
  "threshold": { "bytes": 50000, "source": "default" },
  "files": [
    { "path": "AGENTS.md", "kind": "agents", "state": "present",
      "bytes": 131607, "over-limit": true,
      "unexamined-tail": { "from-line": 210, "cause": "open-comment" } },
    { "path": "specs/system.md", "kind": "system", "state": "absent" }
  ],
  "form": "inline",
  "entries-count": 2,
  "entries": [
    { "path": "AGENTS.md", "section": "Gotchas", "kind": "item",
      "start-line": 120, "end-line": 123, "bytes": 412, "digest": "9f2c…" },
    { "path": "specs/events.md", "section": "Event Catalog", "kind": "heading",
      "start-line": 40, "end-line": 52, "bytes": 380, "digest": "77d0…",
      "identifier": "order.created", "hits": [] }
  ],
  "search": { "state": "ran", "files-searched": 312,
              "outside-project": [], "outside-project-count": 0 },
  "applied": false,
  "stale-entries": [],
  "created": [],
  "kept": [],
  "written": [],
  "refused": []
}
```

- `threshold` — `[artifacts] read-size-bytes`, resolved by the function `check-artifact-size` uses, so an invalid value fails with that primitive's error (`PrimitiveError::InvalidReadSize`).
- `files` — all four subjects, in subject order. `state` is `present`, `absent`, `unreadable` or `outside-project`. `bytes` and `over-limit` are present when the file is `present`, and an `unreadable` or `outside-project` file contributes no entries. `outside-project` is a file that is, or lies under, a symbolic link resolving outside the project, judged by `validate_contained` against the project root (BE-INPUT-004): it is neither read nor written. A file that is simply not there stays `absent`. A file exactly at the threshold is not over it. `unexamined-tail` is present when a `present` file ends inside an open HTML comment (`cause: open-comment`) or a `<!-- BEGIN {marker} -->` with no END (`cause: unterminated-managed-region`), and `from-line` is the 1-based line the open region begins on: that line and every line after it are in no entry and were not examined, so the file never reads as walked. When both are open, the managed region, which opened first, names the line.
- `entries` — every entry of every present file, in file and line order. `section` is the enclosing `##` heading's text, absent in the preamble. `hits` is present exactly when `identifier` is and the search ran, and lists the files holding the identifier.
- `form`, `entries-count`, `lists-file` — `entries`, each with its `hits`, and the search's `outside-project` are inline while the result serialized fits `[artifacts] read-size-bytes` (a result exactly at it fits), and `form` is `inline`. Past it `form` is `file`, `entries` and the search's `outside-project` are absent, and `lists-file` names the file holding them whole, from the project root: a JSON object with the key `entries`, and the key `outside-project` when the search ran, written by the bounding `check-artifact-size` and `prune-tasks` use, under `.ductus/results/prune-project/project-files/`. `entries-count` and the search's `outside-project-count` are inline in both forms. `hits` move with their entry, never apart from it, since each entry is a record whether or not it has hits. Both a preview and an apply bound the result, and an apply has written by the time a list file that cannot be written fails the call.
- `search` — on a preview, `ran` with `files-searched`, or `unavailable` with a `reason`, as `ductus prune-project --help` states; in the `unavailable` case no entry carries `hits`. It is absent on an apply, which reads no source. A ran search also carries `outside-project`, the tracked files it left unread because each is, or lies under, a symbolic link resolving outside the project (project-relative paths, in search order; not counted in `files-searched`), with `outside-project-count`. Each candidate is checked only when an identifier is searched for, since otherwise nothing is read.
- `stale-entries` — each listed `remove` or `replace` with no entry of that path and digest left to claim. Non-empty is the **`stale-entries` domain outcome**, which `ductus prune-project --help` states; its refusal includes `create`. Duplicate digests each claim a distinct entry.
- `created` and `kept` — the `create` kinds written from their template, and those whose file already existed, which are left as they are, one created by another writer since the apply looked included.
- `written` and `refused` — the project files an apply rewrote, and those it refused because each changed after the apply read it (*The write guard*), each in subject order and project-relative. A refused file is left as the other writer made it, its listed removals and replacements unapplied, and the rest of the apply is written. `applied` is `false` when `refused` is non-empty.

An apply removes or replaces the listed entries, then collapses the blank lines at each changed seam to at most one, rewriting nothing else in the file and preserving its line endings. It writes each changed file only when it is unchanged since the apply read it (*The write guard*), the read and the write both going through `validate_contained`. Before an apply writes anything, every listed `create` home is checked the same way, and one that is, or lies under, a symbolic link resolving outside the project fails the whole call with `invalid-path`, so a refused create never leaves half an apply on disk. A created home gets the framework template's text, compiled into the primitive as the `--reset` body is and held to `framework/templates/project/{kind}.md` by a unit test. It is written create-only (`write_new_atomic`), at mode `0644` on Unix, since the tempfile it persists is `0600`.

### Operational errors

| Code | Condition |
| --- | --- |
| `missing-argument` | `apply` with `remove`, `replace` and `create` all empty |
| `invalid-argument` | `remove`, `replace` or `create` without `apply`; a listed `path` that is not one of the four subjects |
| `invalid-path` | an apply lists a `create` home whose path is, or lies under, a symbolic link resolving outside the project; nothing is written |
| `invalid-path` | a project file became, or now lies under, a symbolic link resolving outside the project between the apply's read and its write; that file is not written, and the call fails |
| `check-artifact-size`'s invalid-threshold error | `[artifacts] read-size-bytes` is not a positive whole number, or `artifacts` is not a table |

## Notes

- All paths are repo-relative with `/` separators on every platform.
- The write is a tempfile + rename guarded by a compare before persist
  (*The write guard*); a crash mid-write leaves `tasks.md` unchanged, and no
  backup sidecar is produced (recovery is git history).
- `size-before`/`size-after` report both line and byte counts so the command
  can render a "materially smaller" summary without seeing the content.
