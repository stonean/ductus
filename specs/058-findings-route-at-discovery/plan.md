# 058 — Findings route at discovery Plan

Implements [058 — Findings route at discovery](spec.md).

## Overview

The change has three layers, landed in dependency order:

1. **The runtime.** It stops writing to the inbox, records a disposition for
   every finding, persists routed and discarded decisions across runs, and
   gates `done` on outstanding gating findings — on undispositioned ones
   until scenario `findings-cost-in-proportion-to-risk`.
2. **The framework prose.** Commands, the constitution, templates, docs, and
   `AGENTS.md` all change to state the new contract.
3. **The cross-spec discharge.** Seven `done` specs are reopened, each has the
   claims 058 falsifies corrected, and each is returned to `done` through gates
   that only the new runtime has.

The ordering is forced by one fact. The MCP server runs the binary it was
started with (`AGENTS.md`, "After changing runtime source, verify through the
built binary"), and `.ductus/bin/ductus` is a symlink to
`runtime/target/release/ductus`. So every runtime task lands and is built
first. Then the session restarts. Only after that are any pipeline commands run
that consume the new primitives.

Disposition is a host judgment, and the runtime never makes it. The host picks
fix, route, or discard by walking the groom decision tree, and confirms each
write. The runtime does three things with that judgment: it counts the
dispositions, persists the routed and discarded decisions, and renders them.
Since scenario `findings-cost-in-proportion-to-risk` the gate no longer reads
the count; it reads the gating counts the records already carry.

## Technical Decisions

### The record shape is owned by the two writers, so the new fields are theirs

`write-review` and `write-analysis` both rewrite their artifact whole, with a
fixed field set:

- `render_report` writes the frontmatter at
  `runtime/src/primitives/write_review.rs:477-523` and writes the file with
  `write_atomic` at `:222`.
- `render_analysis` does the same at
  `runtime/src/primitives/write_analysis.rs:195-225`.

`framework/commands/review.md:482-483` already warns against inventing
disposition fields for exactly this reason: `write-review` "would drop them on
the next run". So `dispositions:` and `decisions:` join each writer's fixed set
rather than riding along as unknown keys.

**`dispositions:` is a typed field, and its absence has a meaning.** It is
`Option<Dispositions>` on `ReviewBlock` (`runtime/src/schema/primitives.rs:35`)
and `AnalyzeBlock` (`:136`). `None` means the record predates 058.
`Some(zeros)` means a run that found nothing. The two states must never
collapse, the same rule `examined: Option<u32>` already follows
(`primitives.rs:90-95`). Neither struct denies unknown fields; nothing in
`runtime/src` does, and `runtime/src/mcp/server.rs:131` explains why. So a
record that still carries `captured-issues` parses unchanged. The full shapes
are in [data-model.md](data-model.md).

**`decisions:` is read the way waivers are read.** Waivers live outside
`ReviewBlock` and are read by `read_recorded_waivers`
(`runtime/src/primitives/mod.rs:614-635`). That reader is hard-wired to
`review.md`'s `waivers:` key. It generalizes to a file-and-key reader,
`read_recorded_list`, so both records can read `decisions:`, and
`read_recorded_waivers` becomes a thin wrapper over it. Open-schema extras on
an entry are preserved with the `RawWaiverFull` pattern: a
`#[serde(flatten)] extra` map, re-rendered verbatim
(`write_review.rs:872-906`, `:967-981`).

### A new read-only primitive, `process-decisions`, classifies stored decisions before the host proposes

The host must know which findings already carry a stored decision *before* the
fix-and-route step. Otherwise it asks questions that were already answered. So
classification cannot wait for the writer, just as waiver classification does
not wait for it: `/{project}:review` calls `process-waivers` at step 8, before
`write-review` at step 11 (`framework/commands/review.md`, §Instructions).

`process-decisions` mirrors `process-waivers`
(`runtime/src/primitives/process_waivers.rs:70-120`):

- **Args:**
  - `feature`;
  - `record`: `review` or `analysis`;
  - `fired`: this run's finding keys;
  - `restricted`: true when a review had a pass that did not run (a
    dimension-restricting flag, or an empty scope that skipped them all), or
    an analysis could not read a target it meant to examine or could not
    resolve a registered shared constitution
    (scenario `only-unreadable-targets-retain-decisions`).
- **Result:**
  - `matched`: stored decisions whose key fired, each with its outcome and
    target or reason;
  - `expired`: stored decisions whose key did not fire on an unrestricted run;
  - `retained`: stored decisions whose key did not fire on a restricted run;
  - `notices`.
- **Malformed entries** are reported and never expired. Duplicates warn, and
  the first one wins. The data model names the two cases a re-render drops.

Both rules match `review.md` §Malformed and duplicate waivers
(`framework/commands/review.md:703-722`).

The writers then take:

- `expired-decisions`, applied the way `expired_waivers` is (the
  `WriteReviewArgs` field at `primitives.rs:522-524`, pruned by
  `surviving_waivers` at `write_review.rs:797-803`);
- `decided-by`, the author `--waive` already supplies for waivers
  (`git config user.email`).

There is no `new-decisions` list. Each writer derives its new decisions from
the findings it is handed: every routed or discarded live finding whose key
and outcome are not already stored becomes an entry, stamped
with the run's own timestamp (`reviewed-at` or `analyzed-at`) as `decided-at`
and with `decided-by` as its author. `decided-by` is required only when at
least one entry is new. Deriving the list, rather than accepting it, means the
stored decisions and the rendered dispositions come from one input and cannot
disagree.

A finding re-matched to a stored decision with the same outcome keeps that
decision's original stamp, is re-rendered unchanged, and counts toward
`routed` or `discarded` in `dispositions:`. A finding re-decided differently
replaces the stored entry.

The key is `{family} — {message}` for analyze findings. The old capture key's
leading `{category}` is host-assigned and exists only in prose
(`framework/commands/analyze.md:76`); `ArtifactFinding` carries no category
field (`primitives.rs:3664-3683`). For review observations the key is the
observation's rendered line, as `observation_line` builds it
(`write_review.rs:361-369`). Because observation text is the reviewer's own
wording, the host matches a new observation against `process-decisions`'
stored entries and passes the matched key back, rather than trusting the text
to reproduce.

### `write-review` stops capturing and starts recording dispositions

- **Remove the write-through.** Delete `capture_observations`
  (`write_review.rs:332-356`), its call before the report write (`:209-216`),
  `inbox_bullet_text` (`:376-381`), the `observations_captured` result field
  (`primitives.rs:590`), and `inbox_standing` from the result (`:240`;
  `primitives.rs:623`).
- **Remove the captured-issues window.** Delete `WriteReviewArgs.captured_issues`
  (`primitives.rs:529`), the `captured-issues:` frontmatter line
  (`write_review.rs:492`), and the `## Captured issues` section with
  `render_captured` (`:562-565`, `:661-679`). `invalidate-review` nulls
  `dispositions:` with the run's other counts, and keeps nulling
  `captured-issues` for the pre-058 records that still carry it. Its
  keep-every-other-key rule already preserves `decisions:`.
- **Observations carry their disposition.** `ReviewObservation`
  (`primitives.rs:459-468`) gains:
  - `disposition`, a `Disposition`: an outcome (`DispositionOutcome`:
    `fixed`, `routed`, `discarded`, or `undispositioned`, defaulting to
    `undispositioned`), plus a `target` for a route and a `reason` for a
    discard, each validated as required by its outcome;
  - `decision-key`: optional, naming a matched stored decision.

  `WriteReviewArgs` gains `expired-decisions` and `decided-by`, per
  §process-decisions above.

  `render_observations` (`:681-695`) renders the outcome and its target or
  reason beside each observation. The `dispositions:` map is computed from the
  observations; the caller never supplies it.
- **An empty scope still records.** Observations on an empty-scope run are
  dispositioned and counted, as they were captured before.
- **The report renders the counts.** `/{project}:review`'s stdout summary
  gains an `observations` row, rendered on every run from the result's
  `dispositions`, in place of the retired `captured` line and inbox row.

### `write-analysis` receives the findings, because capture was its only source of finding text

Today the writer receives per-tier counts plus the captured inbox bullets. The
bullets are the only finding text it sees, which is why `## Captured issues`
exists: see the 057 data model,
[057 — analyze artifact and record relocation](../057-analyze-artifact-and-record-relocation/spec.md).
With capture gone, the writer has to be given the findings.

- `WriteAnalysisArgs.captured_issues` (`primitives.rs:720`) is replaced by
  `findings`, a list of `AnalysisFinding`. Each entry carries `tier` (an
  `AnalysisTier`), `family`, `message`, `path`, `live`, and a `Disposition`
  shaped as above. The list covers both passes: findings fixed in the run
  (`live: false`, gone from the re-check) and live findings from the
  re-check. An optional `decision-key` names a matched stored decision, as on
  `ReviewObservation`; absent, the key is `{family} — {message}`
  (scenario `analyze-findings-match-decisions-by-host-judgment`). `expired-decisions` and `decided-by` join
  the args, per §process-decisions above.
- **Tier counts stay host-supplied scalars from the re-check.** On the exec
  path no host supplies them, so the walker tallies them from its own
  detection steps as it dispatches each, by the tier the step assigns
  (`runtime/src/interpreter/analyze_tally.rs`). The existing dispatch of
  `write-analysis` (`runtime/tests/golden/analyze-basic.jsonl:15`) bound none,
  so every exec record read 0/0/0 whatever detection found; 058's second review
  found it (QUAL-CLAIM-001). `undispositioned` is computed as the live tier
  total minus the live findings that carry a `routed` or `discarded` outcome.
  A finding the caller omits therefore counts as undispositioned by
  construction; it can never be silently counted as handled. That, against the
  walker's tally, is how a `ductus exec` run, which itemizes nothing, records
  every live finding as undispositioned (AC26).
- **AC30 is enforced in the writer.** A `discarded` outcome on a `hard-fail` or
  `blocking` finding is a validation error, raised before any write.
- **The body skeleton:**
  - `## Captured issues` (`write_analysis.rs:253-257`) becomes
    `## Fixed in this run`;
  - each tier section lists its live findings with their dispositions;
  - a tier count above its itemized findings renders a
    `{n} finding(s) not itemized` line, so the body cannot understate what the
    frontmatter counts;
  - the "Findings route to the inbox" Summary sentence (`:234`) and the inbox
    pointer in `tier_line` (`:266`) go;
  - checkbox stripping (`render_captured_plain`, `:289-313`) carries over to
    the new renderer, so 057's AC13 still holds.

### The inbox windows and the standing row leave the review and implement primitives

- **`compute-review-scope`:** drop `captured_issues` from
  `ComputeReviewScopeResult` (`primitives.rs:402`), along with `diff_since`,
  `inbox_in_worktree`, and `inbox_at`
  (`runtime/src/primitives/compute_review_scope.rs:80-85`, `:108`,
  `:134-206`).
- **`diff-cross-spec`:** drop `inbox_additions` and `inbox_standing`
  (`runtime/src/primitives/diff_cross_spec.rs:66`, `:71`, `:96-106`,
  `:128-148`, `:155-156`; `primitives.rs:3500`, `:3509`). **Keep** the
  exclusion of `inbox.md` from `cross_spec_paths` (`diff_cross_spec.rs:120`):
  editing a manual todo is not a cross-spec impact.
- **`inbox_standing::standing`** (`runtime/src/primitives/inbox_standing.rs:37`)
  survives, with `dashboard` as its only caller.

### `/{project}:status` renders the standing row, and the runtime now renders its strings

`DashboardResult` (`primitives.rs:1604-1624`) gains `inbox-standing`.
`render_callouts` (`runtime/src/primitives/dashboard.rs:191-309`) gains an
`Inbox:` line. Today the four state strings exist only in command prose
(`framework/commands/implement.md:93-96`, `framework/commands/review.md:780-783`),
and `dashboard` never reads the inbox (`review.md:767-768`). Rendering them in
`dashboard` makes the row deterministic and covers it with `status.md`'s
strict-stdout parity golden. The row is rendered on every run: outstanding with
the oldest item's date, outstanding with age undeterminable, clean, or no file.
The `InboxStanding.state` doc still says "three states"
(`primitives.rs:3554`); it is corrected in passing.

### The gate gains two blocks, checked last, predates before count

After the analyze checks (`runtime/src/primitives/check_review_gate.rs:182-201`)
and before the pass (`:216`), a new `disposition_block` checks four things, in
this order:

1. `review.md` has no `dispositions:` map;
2. `analysis.md` has no `dispositions:` map;
3. `review.md` has an unwaived MUST or SHOULD violation;
4. `analysis.md` has a hard-fail or blocking finding.

Checks 3 and 4 read the gating counts since scenario
`findings-cost-in-proportion-to-risk`; until then each read the record's
`undispositioned`, which counts every tier.

A missing map comes first, because a count from a record that predates the
field means nothing.

Two new `ReviewGateBlock` variants (the enum is at `primitives.rs:3129`):

- **`record-predates-dispositions`** names the record and the command to re-run.
- **`undispositioned-findings`** names the count per record and the command to
  re-run. The name predates the narrowing, and is kept as the wire value.

Blocking on a missing field departs from the existing precedent, where a record
that predates a field is `Undeterminable` and rides a notice on a passing gate
(`check_review_gate.rs:203-215`; `primitives.rs:63-74`). The departure is the
decision Resolved Question 5 made. It matches the gate's own refusal of a
grandfather clause for the analyze record (`check_review_gate.rs:269-274`).

Two stale doc comments are corrected while the gate is being edited: `run`'s
"all six block reasons" (`:48-49`) and the module doc (`:5-14`). The gate order
restated in `framework/commands/implement.md:83` and `:150-176`, and in the MCP
description (`runtime/src/mcp/server.rs:793-794`), gains the two checks.

### A new `disposition-drift` family, rather than a new condition inside analyze-state drift

When this was decided, `check-artifacts` carried an `analyze-state-drift`
family (`check_analyze_drift`) that emitted every analyze-record condition
under that one name. `--fix` never reverted on that family. Its only triggers
were review-state drift and scenario open questions. Task 42 later removed that
family. `/{project}:analyze` now judges analyze-state drift after step 21, from
the record it has just written, and `--fix` still does not revert on it
(`runtime/src/primitives/check_artifacts.rs`, module doc;
`framework/commands/analyze.md`, §Analyze state drift).

Adding the undispositioned condition to that family would have left the host
distinguishing conditions by message text. A separate `disposition-drift`
family instead gives `--fix` a family key to trigger on:

- It applies at `done` only.
- It reads `review.md` only. `analysis.md` is judged by `/{project}:analyze`
  after it writes that record, from the record it wrote, because a finding
  read from the record a run is about to replace could never clear (scenario
  `analysis-drift-judges-the-record-it-writes`).
- It is `Advisory`. It was `Blocking` until scenario
  `findings-cost-in-proportion-to-risk`, when the findings it counts stopped
  holding `done`.
- A record without a map produces nothing: it predates the field, and a
  backfilled map would assert dispositions nobody made
  (`check_disposition_drift` in `check_artifacts.rs`).

`--fix` reverted on it through the host's guarded `set-status` until that
scenario, and no longer does, so the family key now serves the report and
the stored decisions only. `analyze.md` gains a markdown-only "Disposition drift" section, and the stale family-order
doc on `CheckArtifactsResult.findings` is corrected in passing.

### `append-task` gains an opt-in title dedup, so disposition tasks stay idempotent

`append-task` dedups only on a scenario slug (`append_task.rs:85-103`). A
slug-less body appends every time, by design (`a_slugless_body_still_appends_every_time`).
A disposition task has no scenario to point at, so a re-run of an interrupted
`/{project}:implement` would append the same finding twice. That breaks the
constitution's "Re-capture is idempotent" rule, which this spec carries forward
rather than drops.

`AppendTaskArgs` (`primitives.rs:2364-2415`) gains `dedup-title: bool`, default
`false`. When set, an existing **pending** section with an identical title
returns its number with `appended: false`. A spent section with the same title
does not match, because the same finding surfacing again after being
dispositioned is new work.

The disposition task's shape:

- **Title:** `Disposition out-of-spec finding: {summary}`
- **Body:** `{path} — {detail}`
- **Done when:** the finding is fixed, routed, or discarded, with a discard's
  reason written on the task.

Working the task records its outcome on that body item before checking it —
`— fixed`, `— routed to {target}`, or `— discarded: {reason}` — so the
completion summary reads each disposition from `tasks.md` itself.

### Command procedures: the fix-and-route step is host work that `ductus exec` skips

Analyze's and review's fix-and-route steps need per-finding arguments the
walker does not hold. So they are **host responsibility** steps: the primitive
is named without backticks, under `<!-- audit:ignore-promotion -->`, per the
`AGENTS.md` rule on backticking a primitive "only when the walker can actually
supply its arguments". The subprocess walker no-ops such steps by design. That
is what makes an exec run write no fix or route, and, with the tier counts it
tallies, record everything undispositioned (AC26).

`process-decisions` is host responsibility for the same reason: its `fired`
keys are per-finding.

Routing reuses the existing `routeInboxItem` extension point
(`runtime/src/schema/extensions.rs:314-379`), with the finding's text as
`item_text`. The five-route vocabulary is unchanged. Only the docs on
`InboxRoute::Chore` ("stays in the inbox", `:370`) and `InboxRoute::Discard`
(`:372`) are rewritten, to describe the chore being fixed and the item being
removed.

- **`analyze.md`:**
  - Step 16 (capture) becomes three steps:
    - process decisions;
    - fix and route, gated per write;
    - re-run the detection steps when anything was written.
  - The `write-analysis` step, now step 19, passes `findings`,
    `expired-decisions`, and `decided-by` instead of `captured-issues`.
  - The Purpose, Scope Boundaries, frontmatter description, and
    §Finding capture (durability) (`:365-403`) are rewritten to match.
  - The renumbering moves the `write-analysis` dispatch, so
    `analyze-basic.jsonl` is re-blessed.
- **`review.md`:**
  - A process-decisions step (9) and a fix-and-route step (10) precede
    `write-review`, which moves to step 11 and passes dispositioned
    observations, `expired-decisions`, and `decided-by`. The step move is
    pinned by `runtime/tests/review_command.rs`, which is updated with it.
  - §Captured issues, §Observations' write-through (`:412-473`), and
    §The inbox row (`:763-806`) are replaced.
- **`implement.md`:**
  - Walk step 5 (`:132`) appends a disposition task through `append-task`
    with `dedup-title`.
  - Steps 7, 13, and 15 (`:68`, `:81`, `:86-99`) drop the window and the
    standing row, and list the pending disposition tasks instead.
  - The markdown-only gate step 6 (`:186`) follows.
  - The inbox write-boundary carve-out (`:50`) goes.
- **`amend.md`:** the chore guard and its routing row (`:149`, `:218`, `:234`)
  fix the chore instead of redirecting to `/{project}:log`.
- **`groom.md`:** the one surviving "leave it in the inbox" line for a chore
  (`:72`) contradicts step 8 (`:49`) and is corrected. The five routes are
  unchanged.
- **`log.md`**, **`help.md`**, and **`status.md`:** `status.md` gains the inbox
  row. `help.md:125` describes the inbox as manual capture.

### The constitution states the rule once, and every command points at it

- **§design-principles** (`framework/constitution.md:52`) drops "an inbox item"
  from the second disposition.
- **§grounding** (`:108`) routes an implementation-time assumption to a task or
  an open question.
- **§bug-handling** (`:302`) says a chore found by a run is fixed in that run,
  and a chore logged by hand lives in the inbox until it is done.
- **§brownfield-inbox:**
  - `:444-458` is rewritten to describe manual capture, which also resolves the
    `:449` contradiction ("leaves it in place") with `:456`;
  - `#### Automatic issue capture` (`:461-472`) is replaced by
    `#### Finding dispositions`, the three-disposition rule with the scope
    tiers, the disposition task, persisted decisions, and the gate.
- **The Frontmatter Schema rows** (`:552`, `:574`) replace `captured-issues`
  with `dispositions` and `decisions`.
- **§pipeline-boundaries** (`:752`) kept "an inbox item" as a destination,
  but named `/{project}:log` as its only producer; since `050-constitution`'s
  cut to one read, §brownfield-inbox names it.

### The inbox template changes, and a migration carries its guidance to existing adopters

`specs/inbox.md` is installed with strategy `create`
(`framework/bootstrap/ductus.md:670-677`; `runtime/src/primitives/apply_manifest.rs:16-17`),
and no migration targets it. So an adopter keeps whatever header it first
received.

- **The template.** `framework/templates/project/inbox.md` drops the intro's
  incidental-capture role (`:3-6`), the "chores… left in place" line
  (`:16-18`), and forms 2 and 3 (`:27-41`). This repository's own
  `specs/inbox.md` header follows.
- **The migration.** A `framework/migrations.toml` entry, `inbox-guidance-refresh`
  (`introduced_in = "0.53.0"`, `target_paths = ["specs/inbox.md"]`, an
  adopter-relative path, so Family 10b does not apply), gets a procedure file
  following the `audit-record-relocate.md` shape. The procedure:
  1. resolves `[paths] specs-root`;
  2. does nothing when the inbox is absent;
  3. checks idempotency: done when the leading guidance comment already equals
     the template's;
  4. skips a pinned inbox with a warning;
  5. otherwise replaces the first `<!-- Rules:` comment block with the
     template's, preserving line endings and leaving every item byte-identical;
  6. replaces the introduction paragraph too, but only when it is the pre-058
     template's text verbatim. That paragraph also described incidental
     capture, and a customized one is the adopter's.

### The adoption security audit reports instead of capturing

In `framework/bootstrap/ductus-procedure.md`:

- §Writing findings to the inbox and §Deduplication (`:209-223`) are removed.
- §Audit summary (`:225-227`) and §Security audit summary (`:369-381`) print
  each finding with its spec and a pointer to `/{project}:analyze`.
- The trigger (`:179-182`) and rule loading are unchanged.
- `ductus.md` carries no capture prose, so its Family 21 byte-identity with
  `govern.md` is unaffected.

### Cross-spec discharge is priced by destination and batched per spec

This follows the 057 precedent (`d6c1ee8a`, `ddfd95ed`). Each affected spec is:

1. reopened with `set-status` from `done`;
2. corrected for every claim 058 falsifies;
3. signposted back to 058 in a blockquote;
4. returned to `done` through its gate.

A blockquote signpost discharges the obligation, because
`cross_spec_impact_states` reads back-links with the `Pointers` scope, which
keeps blockquote lines (`check_review_gate.rs`, the doc above
`cross_spec_impact_states`). It also induces no `dependencies:` edge, which
would close a cycle with 058's own links.

| Spec | What changes | Destination file | Re-run cost |
| --- | --- | --- | --- |
| 008 | Adoption audit no longer writes the inbox | `spec.md` | review (record shape only) + analyze |
| 020 | `captured-issues`, Captured issues and Observations rows | `data-model.md`, `spec.md` | review + analyze |
| 022 | Three superseded scenarios; inbox fields in the data model | `scenarios/*.md`, `data-model.md`, `spec.md` | review + analyze |
| 047 | Its premise, that analyze findings persist to the inbox | `spec.md` | review (record shape only) + analyze |
| 050 | The `findings-route-by-scope` edge cases | `scenarios/findings-route-by-scope.md`, `spec.md` | review + analyze |
| 054 | AC10's `check-artifacts` family count (added by task 42, which moved `analyze-state-drift` out) | `spec.md` | review + analyze |
| 057 | `captured-issues` and the `## Captured issues` section | `data-model.md`, `spec.md` | review + analyze |

Every reopened spec needs a review run, including 008 and 047 whose durable
contracts do not change. That is AC29: a reopened spec whose `review.md`
predates dispositions is blocked until review re-runs. It is the upgrade cost
Resolved Question 5 accepted, paid here on seven specs. A review whose only work
is writing the new record shape is recorded truthfully with a small `examined`
against its `scope`. That is the disposition `AGENTS.md` already prescribes for
022's digest repair. 022's full re-review remains its own decision to spend and
is not implied by this plan. Its three scenario edits are reviewed; its 95
other scenarios are not re-read, and the record says so.

The discharge runs only after the restart onto the new runtime. Records written
by the old binary would carry no map and would fail AC29 at the next gate.

### Release

The bump is `0.52.1` → `0.53.0`, a minor release: primitive result schemas
lose fields, a primitive is added, and the gate blocks on new conditions. The
version goes in the repo-root `version` file, `runtime/Cargo.toml:3`, and a
`runtime/CHANGELOG.md` section. `Cargo.lock` is refreshed by one build without
`--locked`. Family 20 checks that they agree. The changelog section covers
every runtime change this release carries, not only 058's (tasks 1–48): 022's
task 123, exec analyze deriving its list seeds, and 051's task 26, a fold
owed to another tree no longer holding `done`. The tag `ductus-v0.53.0` is
cut after 058, 022, and 051 are all `done`, following the `AGENTS.md` release
entry. The new
primitive joins `framework/runtime-tools.txt`, whose parity with the registry
is asserted by `runtime/tests/mcp.rs:109-119`. The configure permission lists
gain it too (`framework/bootstrap/configure/claude.md:119-120`, `auggie.md:107-108`).

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/schema/primitives.rs` | Modify | `Dispositions`, `dispositions` on both records, `Decision` shapes, `process-decisions` args and result, observation disposition, `findings` on `write-analysis`, removed inbox fields, two gate variants, `dedup-title`, `inbox-standing` on the dashboard result |
| `runtime/src/primitives/process_decisions.rs` | Create | Classify stored decisions against this run's keys |
| `runtime/src/primitives/mod.rs` | Modify | `read_recorded_list`; register the new module |
| `runtime/src/primitives/write_review.rs` | Modify | Drop capture and the window; record dispositions and decisions |
| `runtime/src/primitives/write_analysis.rs` | Modify | Take findings; new skeleton; dispositions and decisions |
| `runtime/src/primitives/compute_review_scope.rs` | Modify | Drop the captured-issues window |
| `runtime/src/primitives/diff_cross_spec.rs` | Modify | Drop inbox additions and standing |
| `runtime/src/primitives/dashboard.rs` | Modify | Inbox standing and its rendered row |
| `runtime/src/primitives/inbox_standing.rs` | Modify | Its caller is now `dashboard`; the age dates each bullet by its text against `HEAD`'s blame |
| `runtime/src/primitives/check_review_gate.rs` | Modify | Two new blocks; doc corrections |
| `runtime/src/primitives/check_artifacts.rs` | Modify | `disposition-drift` family |
| `runtime/src/primitives/append_task.rs` | Modify | `dedup-title` |
| `runtime/src/primitives/invalidate_review.rs` | Modify | Remove `dispositions` with the run's other counts; keep removing `captured-issues` for pre-058 records |
| `runtime/src/primitives/validate_frontmatter.rs` | Modify | Report an unparseable `decisions:` list |
| `runtime/src/schema/registry.rs`, `runtime/src/mcp/server.rs`, `runtime/src/main.rs`, `runtime/src/interpreter/mod.rs` | Modify | Register and dispatch `process-decisions`; tool descriptions; comments naming the inbox |
| `runtime/src/interpreter/analyze_tally.rs` | Create | The exec walker's tally of the tier counts an `/{project}:analyze` walk records |
| `runtime/src/schema/extensions.rs` | Modify | `InboxRoute` docs |
| `runtime/tests/mcp.rs`, `runtime/tests/walker.rs`, `runtime/tests/golden/{review,analyze,implement,status}-basic.jsonl` | Modify | Wire assertions and goldens for the new shapes |
| `framework/runtime-tools.txt`, `framework/bootstrap/configure/{claude,auggie}.md` | Modify | The new tool name and its permission |
| `framework/commands/{review,analyze,implement,amend,groom,log,help,status}.md` | Modify | The new procedures |
| `.claude/commands/ductus/*.md` | Regenerate | Mirrors, via the pre-commit hook |
| `framework/constitution.md` | Modify | The rule, stated once |
| `framework/templates/project/inbox.md`, `specs/inbox.md` | Modify | Manual-capture guidance |
| `framework/migrations.toml`, `framework/migrations/inbox-guidance-refresh.md` | Create/Modify | Carry the guidance to adopter inboxes |
| `framework/bootstrap/ductus-procedure.md` | Modify | The audit reports instead of capturing |
| `docs/analyze.md`, `docs/slash-commands.md`, `README.md` | Modify | User-facing descriptions |
| `AGENTS.md` | Modify | Retire or rewrite the capture-era entries |
| `specs/{008,020,022,047,050,054,057}-*/` | Modify | Cross-spec discharge |
| `version`, `runtime/Cargo.toml`, `runtime/Cargo.lock`, `runtime/CHANGELOG.md` | Modify | Release `0.53.0` |

## Data Model

See [data-model.md](data-model.md).

## Trade-offs

- **Blocking on a missing `dispositions:` map, rather than a notice.** It costs
  every spec that is `in-progress` at upgrade, and every spec reopened
  afterwards, a review and an analyze run. The seven discharges here pay it. It
  was accepted, because reading absence as zero passes exactly the old run
  whose findings went to the inbox unseen.
- **A separate `disposition-drift` family instead of extending
  `analyze-state-drift`.** One more family name, in exchange for `--fix`
  triggering on a key rather than a message. `--fix` has since stopped
  reverting on it.
- **`process-decisions` as its own primitive rather than a mode of
  `process-waivers`.** Waivers are keyed on `(rule, file)` and applied to
  MUST violations; decisions are keyed on a finding key and carry an outcome.
  Folding them together would give one primitive two anchor rules.
- **Host-matched review keys.** Observation decisions depend on the host
  recognizing a re-observed issue. A miss costs one repeated question, never a
  silent waiver, because an unmatched stored decision is pruned rather than
  applied to something else.
- **Rejected: keeping `captured-issues` as a deprecated alias.** Nothing reads
  it. `AnalyzeBlock` never even had the field (`primitives.rs:136-239`), and
  an alias would keep a measurement of the wrong thing alive.
- **Rejected: a rewrite migration for existing records.** Records parse
  unchanged. Rewriting them would fabricate a `dispositions:` map for runs that
  never dispositioned anything. The analyze record refuses the same
  fabrication: a `done` spec with no `analysis.md` is exempted, not backfilled
  (`scripts/audit/analyze-record-backlog.sh`, header comment).
- **Known limitation: a legacy spec nobody touches is never re-checked for the
  adoption audit's gaps.** This is accepted in the spec's adoption-audit
  section.
- **Known limitation: 022's full re-review is not bought here.** Its record
  will state the small numerator honestly.
