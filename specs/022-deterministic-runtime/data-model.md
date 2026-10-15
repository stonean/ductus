# 022 — Deterministic Runtime Data Model

Defines the data structures the runtime owns: the parsed procedure AST, the JSON-over-stdio protocol envelope and message types, the primitive request/response schemas, and the extension-point schemas (the three initial-release points plus the follow-on request/response shapes). These types live in `runtime/src/schema/` as Rust types with `serde::{Serialize, Deserialize}` derives; their serialized JSON shape is the stable contract for host integrators.

## Procedure AST

Produced by `runtime/src/parser/`. Internal to the runtime; not serialized to disk. JSON serialization exists only for `runtime parse <file>`'s debug output.

```rust
struct Procedure {
    command: String,           // e.g., "status"
    steps: Vec<Step>,
}

enum Step {
    Primitive {
        number: StepNumber,    // "1", "1.1", "2", etc.
        name: String,          // matches a name from spec.md's Primitive library
        prose: String,         // surrounding prose for the markdown-only/MCP path
        location: SourceRange,
    },
    Extension {
        number: StepNumber,
        identifier: String,    // "writeCode", "assessSpecQuality", "writeSpecBody", ...
        prose: String,
        location: SourceRange,
    },
    Prose {
        number: StepNumber,
        text: String,
        location: SourceRange,
    },
}

struct StepNumber(Vec<u32>);   // [1, 2] for "1.2"; [3] for "3"

struct SourceRange {
    start_line: u32,
    start_col: u32,
    end_line: u32,
    end_col: u32,
}
```

The parser rejects a malformed procedure with a hard `ParseError::Invalid` — an unrecognized primitive name, or a single step naming two or more *distinct* primitives (only one can dispatch, so a second would be silently dropped on the exec path). `runtime parse --check` reports these, and `scripts/lint-procedure-parseability.sh` fails CI on them (for every `framework/commands/*.md` and `framework/bootstrap/*.md` file). A file that has no procedure-shaped Instructions section returns `ParseError::LegacyProse`, tolerated only for the files on the parseability allowlist.

## JSON-over-stdio protocol

Newline-delimited JSON. Each line is one complete JSON object terminated by `\n`. The envelope's `type` field is a discriminated union; the closed set of types below is the entire protocol surface.

### Envelope

```json
{ "type": "<discriminator>" }
```

### Outbound (runtime → host) messages

```json
{
  "type": "llm-request",
  "extension-point": "writeCode | writeSpecBody | assessSpecQuality | performReview | askClarifyQuestion | routeInboxItem | routeFold | verifyCriteria",
  "request-id": "<opaque string, unique per request>",
  "request": { }
}
```

```json
{
  "type": "gate-confirm",
  "gate": "<gate name, e.g. plan-finalize-status>",
  "request-id": "<opaque>",
  "prompt": "<user-facing prompt string>"
}
```

```json
{
  "type": "progress",
  "message": "<human-readable string>",
  "step": "<step number, e.g. '3.1'>",
  "primitive": "<primitive name if applicable>"
}
```

```json
{
  "type": "complete",
  "result": { },
  "runtime-version": "<semver>"
}
```

```json
{
  "type": "error",
  "code": "<machine-readable code, e.g. 'parse-error'>",
  "message": "<human-readable description>",
  "runtime-version": "<semver>",
  "location": { "file": "...", "line": 0, "col": 0 }
}
```

### Inbound (host → runtime) messages

```json
{
  "type": "llm-response",
  "request-id": "<matches an open llm-request>",
  "response": { }
}
```

```json
{
  "type": "gate-response",
  "request-id": "<matches an open gate-confirm>",
  "confirmed": true
}
```

The runtime ignores any other inbound JSON shape — it logs to stderr and continues waiting for a valid response.

A `gate-response` with `confirmed: true` binds `apply: true` into the arguments of every primitive the walker dispatches after the gate, because a gate is what authorizes the writes that follow. It is bound as an argument, not a context key, so no extension payload built from the context carries it. A primitive with an `apply` argument therefore previews before the gate and applies after it, and `/{project}:prune`'s step 7 performs the reduction step 6 confirmed. `confirmed: false` ends the walk with a clean `complete`. Scenario [a-confirmed-gate-authorizes-the-writes-after-it](scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md).

## Per-project file resolution

The two files the runtime reads out of the per-project directory resolve through one ordered chain each, declared once in `runtime/src/schema/paths.rs` (`CONFIG_CHAIN`, `SESSION_CHAIN`) and walked by every resolver. Newest first:

| Tier | Config | Session | Introduced by |
| --- | --- | --- | --- |
| 1 | `.ductus/config.toml` | `.ductus/session.toml` | [049](../049-rename-govern-to-ductus/spec.md) |
| 2 | `.govern/config.toml` | `.govern/session.toml` | [042](../042-consolidate-govern-per-project-files-under-govern-directory/spec.md) |
| 3 | `.govern.toml` | `.govern.session.toml` | pre-042 |

**Reads** return the newest tier that exists, falling back to the **oldest** when none does (a missing file the caller treats as "config absent" → defaults). **Writes** return the newest tier that exists, falling back to the **newest** when none does (a fresh project cuts over immediately; a pre-migration write stays on the file already holding the other sections). That empty-chain difference is the only place the two disagree.

Older tiers are never removed by a primitive — the bootstrap migration is the sole cutover. See the [`project-directory-resolution-chain`](scenarios/project-directory-resolution-chain.md) scenario.

### Command-file candidates — `Host::command_file_candidates`

`Host::command_file_candidates` (runtime/src/host.rs) is the single place the runtime derives the repo-relative paths where an installed slash-command file may live, in resolution order, at `ductus exec` time and in the `writeCode` payload. It covers the three flat-namespaced layouts:

1. `{cli-config-dir}/commands/{project}/{command_name}.md` — `claude-style` (Claude Code, Auggie), tried first;
2. `{cli-config-dir}/command/{project}/{command_name}.md` — `opencode` (singular `command/`);
3. `{cli-config-dir}/prompts/{project}-{command_name}.md` — `pi` (flat project-hyphenated prompt templates, [064](../064-pi-host-support/spec.md)), **appended last**.

Each adopter installs into exactly one layout, selected by the agent's registry `layout`; the `cli-config-dir` recorded in the gitignored session file is the real selector, and the candidate order is belt-and-braces — walking all three lets the runtime resolve any supported layout without knowing which agent wrote the file. The two pre-existing candidates keep their relative order exactly, so every pre-pi adopter resolves identically. Both resolution callsites (`main::run_exec`, `interpreter::payload::locate_command_file`) consume the candidate list unchanged. See the [`the-pi-command-candidate`](scenarios/the-pi-command-candidate.md) scenario.

## Primitive request/response schemas

Each primitive has a typed args struct (the CLI subcommand's `clap` derive shape) and a typed result struct. Below is the canonical JSON shape for each; the CLI surface translates command-line flags into the args; the MCP surface uses the same JSON via `rmcp` tool calls.

### Severity tiers — three closed sets, not one

The field is spelled `severity` on five shapes, and the sets behind it are **three distinct vocabularies** rather than one. They live in `runtime/src/schema/severity.rs`, the same single-source-of-truth shape `status.rs` has for the lifecycle set:

| Vocabulary | Values | Carried by |
| --- | --- | --- |
| Review | `must`, `should` | `ReviewFinding` |
| Analyze | `hard-fail`, `blocking`, `advisory`, `informational` | `FrontmatterFinding`, `ArtifactFinding` |
| Rule | `must`, `should`, `info`, `""` | `AssessSpecQualityRule`, `AssessSpecQualityFinding` |

Which condition earns which **analyze** tier is the constitution's to state, not this document's — see §text-first-artifacts (Validation Severity), and `framework/commands/analyze.md` step 2 for the host's per-finding rendering rule. This table registers the *sets*, not the assignment.

They are three types deliberately. A single merged enum would make the cross-vocabulary error — an analyze tier emitted into a review finding — representable in the type introduced to forbid it.

**An unrecognized value is rejected, never defaulted.** `severity` arrives from the `performReview` extension point, written freehand by an LLM, and a defaulted value would file an unrecognized MUST as a SHOULD and let the spec pass `check-review-gate`'s MUST check. The bucketing is a `match` over the closed set with no catch-all arm, so adding a tier is a compile error rather than a silent demotion.

Rejection names the extension point, the vocabulary, the offending value and the legal set:

```text
schema-mismatch in `performReview`: unrecognized review severity "mandatory" — expected one of: must, should
```

**Matching is case-insensitive and serialization is canonical lower-case**, so `MUST`, the spelling the rule files use, is accepted: strictness targets unrecognized values, not capitalization (scenario `severity-is-a-closed-set-not-a-string`).

### `read-spec` — parse spec frontmatter and body sections

Args:

```json
{ "feature": "022-deterministic-runtime", "include-body": true }
```

Result:

```json
{
  "frontmatter": {
    "status": "clarified",
    "dependencies": ["021-runtime-boundary"]
  },
  "sections": [
    { "heading": "Motivation", "level": 2, "body": "..." },
    { "heading": "Architecture", "level": 2, "body": "..." }
  ],
  "acceptance-criteria": [
    { "checked": false, "text": "AC3: A single binary builds...", "label": "AC3" }
  ],
  "open-questions": [],
  "scenario-open-questions": [
    { "scenario": "framework-list-dedup", "text": "Format argument or separate primitive?" }
  ],
  "scenario-files-unreadable": ["broken-scenario"],
  "path": "specs/022-deterministic-runtime/spec.md"
}
```

`open-questions` comes from the **spec body only**; feature-targeted `/{project}:clarify` branches on exactly this list.

`scenario-open-questions` is a **sibling** signal, never merged into it: the unresolved questions carried by `scenarios/*.md`, each tagged with its source scenario slug, in the shared scenario order. The two stay separate because they answer different questions — the body's count is what `clarified` asserts and what the `draft → clarified` edge turns on, while these are remaining work that gates `done`, so merging them would make a spec's status contradict its own body. Completion is where they join: a spec does not reach `done` while this list is non-empty ([046 — Scenario open-question visibility](../046-scenario-open-question-visibility/spec.md)).

`scenario-files-unreadable` lists the slugs of scenario files that could not be read while collecting the above, and is **omitted from the payload when empty**. It exists so an empty `scenario-open-questions` is not two states wearing one value: *every scenario was examined and none carries a question* versus *a scenario could not be examined*. An unreadable scenario is never escalated into a `done`-blocking finding — nothing can be proven about a file that will not parse — but dropping it silently would have every consumer report clean over a subject it never read, which is `QUAL-CLAIM-001`. The collector is shared, so `check-review-gate`, `check-artifacts` and the dashboard all see the same unread set; `check-artifacts` records each as a `skipped` target with reason `artifact-unreadable`.

`label` carries the criterion's stable `AC{n}` identifier, or `null` for a criterion the labelling pass has not reached ([013 — Text-First Artifacts](../013-text-first-artifacts/scenarios/criterion-identifiers.md)). `text` keeps the `AC{n}:` prefix as authored — stripping it here would make the reported text differ from the file, and a host citing a criterion wants the line it can paste. The label is parsed by `label-criteria`'s `parse_label`, deliberately the same function `mark-criterion` resolves a `label` argument through, so every surface answering "what label does this criterion carry?" answers identically. Callers holding a reference across edits should prefer `label` to the positional index, which an insertion above the target silently redirects.

Both lists come from the same parser, which is comment- and fence-aware: questions inside an HTML comment or a fenced block are not entries, so a spec scaffolded from the shipped template reports zero rather than its commented-out examples. Scenario enumeration and ordering use the shared scenario-file listing, so the set and order match `check-artifacts` and `dashboard`. An absent `scenarios/` directory, an unreadable scenario, or one with no questions section each contribute nothing — an unknown is never escalated into a `done`-blocking finding.

### `read-tasks` — parse tasks.md into structured task list

Args:

```json
{ "feature": "022-deterministic-runtime" }
```

Result:

```json
{
  "tasks": [
    {
      "number": "1",
      "heading": "Bootstrap Rust crate",
      "subtasks": [
        { "text": "Create Cargo.toml", "checked": false }
      ],
      "done-when": "cargo build succeeds"
    }
  ],
  "path": "specs/022-deterministic-runtime/tasks.md"
}
```

### `mark-task` — flip checkbox state on a task

Args:

```json
{
  "feature": "022-deterministic-runtime",
  "task-number": "1",
  "subtask-index": 0,
  "checked": true
}
```

Result:

```json
{ "previous": false, "current": true, "path": "specs/.../tasks.md" }
```

### `mark-criterion` — flip checkbox state on an acceptance criterion

Args — **exactly one** of `criterion-index` or `label`:

```json
{
  "feature": "022-deterministic-runtime",
  "label": "AC7",
  "checked": true
}
```

Result:

```json
{ "previous": false, "current": true, "path": "specs/.../spec.md" }
```

`criterion-index` is 0-based over the criteria in body order and remains supported: it is the positional fallback for a criterion the labelling pass has not reached. `label` is preferred wherever one exists, because an index is only valid against the list as it stood when the index was computed — inserting a criterion above the target silently redirects it, while a label survives insertion, reordering, and removal (spec 013).

Supplying both, or neither, is refused rather than resolved by precedence: a caller that passes both is holding two references it believes name one criterion, and picking a winner would silently discard the disagreement the refusal surfaces. A `label` no criterion carries is an error naming the label, never a no-op — an unresolvable label means the caller's reference is stale, which is exactly the condition labels exist to make visible.

### `label-criteria` — assign stable `AC{n}` labels to acceptance criteria

The labelling pass [013](../013-text-first-artifacts/scenarios/criterion-identifiers.md) specifies. Args:

```json
{ "feature": "022-deterministic-runtime" }
```

Result:

```json
{
  "assigned": [
    { "label": "AC34", "criterion-index": 12 },
    { "label": "AC35", "criterion-index": 13 }
  ],
  "next-criterion": 36,
  "changed": true,
  "path": "specs/022-deterministic-runtime/spec.md"
}
```

`assigned` lists only the criteria this run labelled, each with the 0-based index it occupies in body order at the time of the run; an already-labelled criterion is left byte-identical and never appears. `next-criterion` is the counter as written back to the frontmatter — always greater than every label in the body. `changed` reports whether the file was written, so a no-op run says so rather than reporting a clean write it did not perform.

Assignment takes **`max(highest label in body, next-criterion)`**, incremented per assignment in body order — never `max(body) + 1` — so deleting the highest-numbered criterion never lets its label be reissued to a different requirement; the counter never decreases (scenario `criterion-label-assignment`).

Three refusals, each preferring a legible failure to a partial or silent one:

- A spec with **no frontmatter fence** is refused — there is nowhere to record the counter, and a labelling that cannot be recorded is worse than none.
- A **`next-criterion` that is not a positive integer** is refused rather than repaired in place: a corrupted counter may mean a label was already reissued, and a silent repair would hide that. The `criterion-labels` check family reports the same state as a finding — assignment refuses, enforcement reports.
- A spec with **no Acceptance Criteria section, or an empty one**, is a no-op that creates no counter. An absent `next-criterion` means "no labels assigned yet", which is a truthful state rather than a defect to pre-empt.

The write is atomic (tempfile + rename) like every other primitive write. The pass is idempotent: a second run finds nothing unlabelled and a counter that already exceeds the body, so it writes nothing.

### `set-status` — update spec frontmatter status field

Args:

```json
{
  "feature": "022-deterministic-runtime",
  "from": "clarified",
  "to": "planned"
}
```

Result:

```json
{ "previous": "clarified", "current": "planned", "path": "specs/.../spec.md" }
```

On mismatch (`from` doesn't equal current), returns an `error` envelope with `code: "status-mismatch"` and does not write.

### `derive-boundary` — compute runtime write boundary

Args:

```json
{ "feature": "022-deterministic-runtime" }
```

Result:

```json
{
  "boundary": [
    "README.md",
    "runtime/src/**",
    "specs/022-deterministic-runtime/**"
  ],
  "first-commit": "<sha>",
  "current-head": "<sha>"
}
```

The boundary is derived from `git diff --name-only <first-commit-on-spec-dir>..HEAD` plus the spec dir itself, emitted as **directory-zone globs** (scenario [writecode-boundary-derivation](scenarios/writecode-boundary-derivation.md)): each changed path contributes its parent directory as `{dir}/**`, because the writeCode validator enforcing this boundary must admit *new* files, which can never exact-match a previously-changed path. A root-level changed file stays an exact path — its zone would be `**`, permitting everything. On the exec path the walker merges the result into the `write-boundary` enforcement key as a **union** with any session-seeded value: a seed is a deliberate host/user grant the derivation never revokes, and on a fresh feature (derivation = spec glob only) the seed is what admits the first out-of-spec edit. With neither seed nor non-spec history, enforcement stays fail-closed — the first out-of-spec writeCode edit halts with `out-of-boundary-edit`.

### `check-stuck` — count tasks.md commits since `in-progress`

Args:

```json
{ "feature": "022-deterministic-runtime", "threshold": 10 }
```

Result:

```json
{ "commit-count": 3, "stuck": false, "since-sha": "<sha>", "threshold": 10 }
```

### `validate-frontmatter` — full frontmatter schema check

Args:

```json
{ "path": "specs/022-deterministic-runtime/spec.md" }
```

Result:

```json
{
  "findings": [
    { "severity": "hard-fail", "field": "status", "message": "..." }
  ],
  "clean": false
}
```

`severity` is the tier the constitution's §text-first-artifacts (Validation Severity) assigns to that finding's **condition** — not a fixed value for the family. Consumers render the tier the finding names rather than mapping the family to one tier; `framework/commands/analyze.md` step 2 states the host side of the same contract (scenario `frontmatter-severity-tiers-on-both-sides`).

### `resolve-anchor` — verify every `§<anchor>` reference resolves

Args:

```json
{ "path": "framework/constitution.md", "markers-path": null }
```

Result:

```json
{
  "references": [
    { "anchor": "runtime-boundary", "line": 459, "resolved": true, "kind": "markers" },
    { "anchor": "Workflow", "line": 51, "resolved": true, "kind": "qualified" },
    { "anchor": "Design", "line": 106, "resolved": true, "kind": "intra-document" }
  ],
  "unresolved": [],
  "qualified": 136,
  "intra-document": 6
}
```

Scans `path` for `§<anchor>` references and resolves each against `<!-- §anchor -->` markers. The markers come from `path` itself when `markers-path` is omitted (the constitution self-consistency check); supply `markers-path` to resolve one file's references against a *different* file's markers — `/ductus:analyze` passes the constitution as `markers-path` so a spec's `§` references resolve against the constitution's sections (a spec carries no markers of its own, so resolving against itself would flag every reference as unresolved noise).

**Not every `§X` is a claim about the markers file**, because `§` is the corpus-wide notation for any section, and `kind` records which of three it is (scenario [`anchor-reference-kinds`](scenarios/anchor-reference-kinds.md)):

- **`qualified`** — the reference's line names a markdown document **other than the markers file** (a backticked path, a markdown link target, or a bare `*.md` token). It is a claim about that document's sections, which this primitive cannot evaluate, so it is excluded by construction and **counted**. The match is line-scoped, so a row naming a file once and citing several of its sections excludes them all.
- **`intra-document`** — the anchor names a heading in the citing file, resolved against that file's own headings, longest first (`§Hook Installation` is not satisfied by `§Hook`).
- **`markers`** — a claim about the markers file, and the only kind that can come back unresolved.

An empty `unresolved` therefore means every claim about the markers file resolved, never that every `§` in the file is fine; the two counts ship with the verdict so the exclusion is visible. Prose naming another document without naming its file (`spec 022 §Versioning`) can be neither verified nor excluded and stays reported.

### `write-analysis` — record that analyze ran

Args:

```json
{
  "feature": "042-widget", "analyzed-at": "2026-09-06T14:00:00Z",
  "analyzed-against": "abc123", "hard-fail": 0, "blocking-findings": 0,
  "advisory": 8,
  "unexamined-by-reason": [["not-a-live-claim", 79], ["root-absent", 26]],
  "findings": [{ "tier": "advisory", "family": "link-adjacent-drift",
    "message": "...", "path": "specs/042-widget/spec.md",
    "disposition": { "outcome": "discarded", "reason": "..." } }],
  "decided-by": "dev@example.com"
}
```

Result:

```json
{ "spec-path": "specs/042-widget/spec.md", "blocking": false, "unexamined": 105, "replaced": true,
  "dispositions": { "fixed": 0, "routed": 0, "discarded": 1, "undispositioned": 7 } }
```

Writes `specs/{feature}/analysis.md` — the run's record in that file's frontmatter plus the report body's fixed skeleton — the durable record that `/{project}:analyze` ran, and what `check-review-gate` reads to hold a spec out of `done` until it has. Splices without disturbing sibling keys, and refuses a spec whose frontmatter does not parse rather than recording a clean run into it.

> **Findings carry their dispositions ([058 — Findings route at discovery](../058-findings-route-at-discovery/spec.md)).**
> Each finding arrives with its tier and disposition. The writer derives a
> `dispositions:` map, counting a live finding the call does not itemize as
> undispositioned, and stores each routed or discarded finding under
> `decisions:`, keyed `{family} — {message}`. It refuses a discard on a
> hard-fail or blocking finding — so each finding's `tier` is required — and
> requires `decided-by` when a decision is new. A finding may carry the
> `decision-key` of the stored decision the host matched it to, since many
> analyze messages are host-worded; absent, its key is `{family} — {message}`. [058's data model](../058-findings-route-at-discovery/data-model.md) records the shapes.

**Two values are derived, never accepted**, for the same reason: a field a caller can contradict is a field that will eventually be contradicted.

- `blocking` — from `hard-fail` and `blocking-findings`. No call can record a clean gate over a dirty run.
- `unexamined` — summed from `unexamined-by-reason` when a breakdown is supplied, so the total and its parts cannot disagree. A caller-supplied total is used only when no breakdown is given.
- `analyzed-digest` / `analyzed-unreadable` — taken by the primitive from the feature directory, never passed in, so no caller can record a digest it did not take, and this digest and the one `check-review-gate` recomputes come from one function — which is what makes the row `/{project}:review` renders agree with the gate. A subject that exists but could not be read or reached is recorded in `analyzed-unreadable` rather than digested as empty.

`write-review` derives `reviewed-digest` / `reviewed-unreadable` the same way over the narrower contract set.

**`unexamined-by-reason` is what makes `unexamined` actionable.** A bare total answers *that* something was unexamined and nothing about what, and the reasons are not equivalent — they split into two classes calling for opposite responses:

| class | reasons | response |
| --- | --- | --- |
| excluded by construction | `not-a-live-claim`, `ships-to-adopter`, `root-absent` | none; the exclusion is correct |
| could not be read | `target-missing`, `target-unparseable`, `no-readable-state`, `referrer-unreadable`, `rule-citations-not-checked`, `rule-assessments-not-checked`, `rule-file-unreadable`, `references-not-checked`, `applicable-rules-not-checked`, `grounding-not-checked` | a real gap in what the run saw |
| the spec's own artifact | `artifact-unreadable` | **blocking at `done`** — see below; appears as a finding there, never as a skip |

The first three `could not be read` reasons are `check-artifacts` skips. The rest are gaps no check decided on, so none is an exclusion by construction: `referrer-unreadable` counts the referrers `check-orphaned-references` reports in `skipped`; `rule-citations-not-checked` counts the spec's citations when `check-rule-ids` read no rule file (`examined: 0`), recorded unexamined rather than missing; `rule-assessments-not-checked` counts the loaded rules steps 11 and 12 did not ask about — a rule with no Verification, or whose Statement carries no RFC 2119 keyword to take a tier from — and, on the exec path, a step that loaded no rule at all or whose prose names no MUST or SHOULD tier; `rule-file-unreadable` counts the rule files steps 11 and 12 could not read, whose rules could not even be counted (spec 060); and `references-not-checked`, `applicable-rules-not-checked` and `grounding-not-checked` are `/{project}:analyze` steps 13–15, host prose the exec walker no-ops, recorded once each on every exec run. The exec walker tallies every one of these. On the interactive path the host records `referrer-unreadable` and `rule-citations-not-checked`, and records `rule-file-unreadable` and `rule-assessments-not-checked` for a rule file it could not read or a rule it could not place in either tier; the exec path's per-step count and steps 13–15's reasons do not arise there, since the host runs those steps itself.

`artifact-unreadable` is the one reason that escalates. Below `done` it is a skipped target like any other; **at `done` it is a blocking finding**, so it lands in `blocking-findings` rather than `unexamined` and the gate holds the spec. The general rule that an unknown is never escalated into a defect is about *other* files — another spec's frontmatter, an upstream service — where the defect is not this spec's. This one names the spec's own artifact, in its own directory, that its own analysis could not read. Otherwise an unreadable scenario carrying unresolved questions would pass the `done` gate `scenario-open-questions` exists to hold.

The reason set is closed, so the breakdown is bounded; the map is omitted when empty, so a fully-examined run carries no map rather than a map of zeroes.

**The subject depends on the spec's status when the record is written.** `criterion-path-existence` — the source of most skipped targets — examines `done` specs only, so a record written while a spec is reopened describes a smaller subject than the same spec at `done`. Write the record last, after the status is settled.

### `traverse-deps` — verify spec dependencies and status compatibility

Args:

```json
{ "feature": "022-deterministic-runtime" }
```

Result:

```json
{
  "dependencies": [
    {
      "feature": "021-runtime-boundary",
      "exists": true,
      "status": "done",
      "compatible": true
    }
  ],
  "compatible": true
}
```

### `check-promotion-coverage` — how much of a rules file is routed to its canonical source

Project-scoped: no feature argument, like `check-orphaned-references` and
`check-unfolded-specs`.

Args:

```json
{
  "rules-file": "AGENTS.md",
  "sections": ["Workflow", "Gotchas", "Boundaries", "Design Principles"],
  "table-file": "specs/050-constitution/plan.md",
  "table-section": "Classification",
  "pointer-link": "framework/constitution.md#"
}
```

Result:

```json
{
  "rule-bearing": 121,
  "table-keyed": 98,
  "pointer-citing": 74,
  "classified": 119,
  "unclassified": 2,
  "unclassified-entries": ["append-task has two shapes that both return exit 0 while writing nothing, and neither is an error."],
  "table-rows": 98,
  "unmatched-keys": [],
  "missing-sections": [],
  "table-state": "read"
}
```

Computes `unclassified = rule-bearing − (table-keyed ∪ pointer-citing)` over
committed markdown, so it needs no authored input — no per-entry marker, which
[§design-principles](../../framework/constitution.md#design-principles)
rejects. **All four terms are reported, not just the difference**, because
disagreements over this figure turn on its terms — how the pointer side is
counted, whether a key's trailing parenthetical is stripped — rather than on
decay.

The scenario [`the-promotion-coverage-line`](scenarios/the-promotion-coverage-line.md)
states the rest of the contract: `sections` is required, has no default and is a
set; a named section that does not occur is reported in `missing-sections`;
`unmatched-keys` is how a reworded entry surfaces, keys matching an entry's
bolded lead phrase by prefix in either direction after markup is normalized and
one trailing parenthetical stripped; `table-state` separates `read` from
`no-table`, and a supplied `table-file` that cannot be read is an error; only
top-level bullets outside fences and comments count; and `/{project}:audit`
Family 38 renders the line to stderr as a notice that never affects the exit
code. Counting reuses the shared comment- and fence-aware bullet grammar, with
the top-level constraint kept local rather than delegated, since that helper
trims leading whitespace (`QUAL-DELEG-001`).

### `check-rule-ids` — verify cited rule IDs exist and aren't deprecated

Args:

```json
{ "path": "specs/022-deterministic-runtime/spec.md", "rule-files": ["framework/rules/security-backend.md"] }
```

Result:

```json
{
  "citations": [
    { "rule-id": "SEC-AUTH-001", "found": true, "deprecated": false }
  ],
  "missing": [],
  "deprecated": [],
  "examined": 1
}
```

`examined` is the count of rule files read — the denominator `missing` is
asserted against. Without it, *these IDs do not exist* and *I was given nothing
to check against* are the same result: `known` is built by iterating
`rule-files`, so an empty list makes every citation `missing`, and
`/{project}:analyze` treats `missing` as blocking.

### `run-generator` — invoke a bash generator in `--dry-run`

Args:

```json
{ "script": "scripts/gen-help-tables.sh" }
```

Result:

```json
{ "drift": false, "stdout": "...", "stderr": "...", "exit-code": 0 }
```

Non-zero exit code is a drift finding (`drift: true`), not an operational error.

### `lint-markdown` — wrap `npx markdownlint-cli2`

Args:

```json
{ "paths": ["framework/constitution.md", "specs/**"], "fix": false }
```

Result:

```json
{ "violations": [], "clean": true, "exit-code": 0 }
```

### `gate-confirm` — surface a gate to the user through the host

Args:

```json
{ "gate": "plan-finalize-status", "prompt": "Advance status from clarified to planned?" }
```

Result:

```json
{ "confirmed": true }
```

Under the MCP surface, this is the only primitive whose semantics depend on host capability — an MCP host that cannot route a prompt to the user returns `confirmed: false` and the procedure halts at the gate.

### `dashboard` — `rendered-markdown` field (addendum)

The dashboard payload's full shape is canonical in [scenarios/dashboard-primitive.md](scenarios/dashboard-primitive.md); the coverage-expansion-primitives scenario adds one field:

```json
{ "rendered-markdown": "Target: 042-widget / planned / next: /ductus:implement\n\n| Feature | Status | … |\n…" }
```

The scenario-open-question-signal scenario adds two per-spec fields (spec 046):

```json
{ "scenario-open-question-count": 3, "scenarios-with-questions": ["audit-ci-hard-gate", "family-10-migration-coverage"] }
```

`scenario-open-question-count` is the total unresolved questions across the spec's scenarios, and `scenarios-with-questions` names the scenarios carrying them in shared scenario order. Both are distinct from `open-question-count`, which stays spec-body-only, and the two are never summed. How they render — the Scenarios column's `({n} open)` suffix, the `clarify (scenario)` Next Action and its precedence under `clarify (recovery)`, and the uncapped callout — is `/ductus:status`'s Rendering reference.

`inbox-standing` reports the standing inbox backlog, and `rendered-markdown` renders it as an `Inbox:` line, the last line of the counts-and-callouts block:

```json
{ "inbox-standing": { "state": "outstanding", "outstanding": 6, "oldest": "2026-05-19", "path": "specs/inbox.md" } }
```

`state` is `clean` (examined, empty), `outstanding`, or `no-file` (no readable `{specs-root}/inbox.md`), so a project with no inbox never renders as examined-and-clean. `oldest` is `YYYY-MM-DD` from `git blame` over the surviving bullets, so the whole-file rewrites `append-inbox` and `remove-inbox-item` perform do not reset a surviving line's date. It is **absent, not defaulted**, when the oldest item's date cannot be read — an inbox not yet committed, a surviving item behind a shallow clone's cut, or any other blame failure — and a caller reports that as *age undeterminable* rather than as today; an uncommitted item alone never makes it absent, being newer than every committed one. Being a history read, a CI job running it needs `fetch-depth: 0`. Counting uses the shared comment- and fence-aware bullet grammar the append and remove primitives use. It is **a notice, never a gate**: nothing blocks on inbox depth, because §brownfield-inbox's design rests on capture being free.

`rendered-markdown` is the full pipeline view pre-rendered as one markdown fragment, in `/ductus:status`'s documented order: preamble, dashboard table, counts and callouts, and the cross-service references readout (blocks separated by blank lines; the readout omitted when no spec declares references). The runtime resolves each spec's `references:` index internally for the readout — the same classification `resolve-references` exposes, with the matched service's `description` appended from the `[services]` registry — so one `dashboard` call covers the whole view on the runtime path. `/{project}:…` next actions and callout texts substitute the adopter's `[host] project` namespace. Returned data the host may restyle, never stdout printing (§runtime-boundary: no user-facing rendering owned by the runtime); the structured fields stay authoritative for hosts that render their own view. The canonical piece-by-piece formats live in `/ductus:status`'s Rendering reference, which is also the markdown-only path.

### `resolve-feature` — resolve an identifier to a feature directory

Args:

```json
{ "identifier": "22", "scenario": "scaffolding-primitives" }
```

Result (resolved):

```json
{
  "outcome": "resolved",
  "feature": "022-deterministic-runtime",
  "path": "specs/022-deterministic-runtime",
  "status": "in-progress",
  "candidates": [],
  "scenario": {
    "slug": "scaffolding-primitives",
    "path": "specs/022-deterministic-runtime/scenarios/scaffolding-primitives.md",
    "exists": true,
    "section": "Follow-on scenarios"
  }
}
```

Result (ambiguous / not-found):

```json
{ "outcome": "ambiguous", "candidates": ["022-deterministic-runtime", "023-command-runtime"] }
```

Matching order: exact directory name, then feature number (`7` and `007` both match the zero-padded `007-` prefix), then case-insensitive partial slug substring. Ambiguity and no-match are domain outcomes in the result — never operational errors; disambiguation stays with the user through the host. `scenario` is present only when the args named a slug and the outcome is `resolved`; `status` is best-effort (absent when `spec.md` is unreadable). The scenario `section` field falls back to the legacy `spec-ref` frontmatter key.

### `create-feature` — scaffold the next feature directory

Args:

```json
{ "title": "Webhook Delivery" }
```

Result:

```json
{
  "created": true,
  "feature": "043-webhook-delivery",
  "path": "specs/043-webhook-delivery",
  "template": "specs/templates/spec.md"
}
```

The number is `max(existing three-digit prefix) + 1`, zero-padded; the slug is the lowercased title with non-alphanumeric runs collapsed to single hyphens and trimmed. The spec template is resolved in `writeSpecBody`'s candidate order — `{specs-root}/templates/spec.md`, then `framework/templates/spec/spec.md` — and copied atomically with the source file's mode mirrored. An already-existing target directory is the `created: false` domain outcome (`template` absent, nothing written); a missing template is an operational error raised before the directory is created.

**`path` here is the spec *directory*, and on the exec path it is pinned once written.** A successful `create-feature` (and, on `/{project}:target`, a `resolved` `resolve-feature`) retargets the session, so the walker lets its `feature` and `path` override even session-seeded values — then **pins both keys for the rest of the walk**. The pin is what keeps the later `write-session` binding a directory: nearly every spec-reading primitive that can run in between (`read-spec`, `label-criteria`, `mark-criterion`) reports its own `path`, the spec **file**, and results merge at the top level by bare key. The retargeting primitives stay exempt from their own pin, so a walk that resolves twice still records the second.

### `create-plan-artifacts` — copy the plan/tasks/data-model templates into a feature directory

Args:

```json
{ "feature": "042-widget", "include-data-model": true, "overwrite": false }
```

Result:

```json
{
  "path": "specs/042-widget",
  "artifacts": [
    { "file": "plan.md", "path": "specs/042-widget/plan.md", "action": "created", "template": "specs/templates/plan.md" },
    { "file": "tasks.md", "path": "specs/042-widget/tasks.md", "action": "kept" },
    { "file": "data-model.md", "path": "specs/042-widget/data-model.md", "action": "created", "template": "specs/templates/data-model.md" }
  ]
}
```

The plan-side mirror of `create-feature` and the deterministic surface behind `/ductus:plan`'s template-copy and existing-artifact detection (step 3). Copies each missing artifact's template into the existing feature directory using the same candidate order and atomic, mode-mirroring write as `create-feature`; every needed template is resolved before the first write, so a missing template is one operational error with nothing half-copied. `data-model.md` joins the copy set only when `include-data-model` is passed (whether the feature has domain entities is the host's judgment), but a pre-existing `data-model.md` is always reported so the existing-artifact prompt sees the full set; when it is neither requested nor on disk it is omitted from `artifacts`.

Per-artifact `action` is the domain outcome: `created` (was missing, template copied), `kept` (pre-existing, untouched — never an error), or `replaced` (pre-existing, template copied over; only with `overwrite: true`, the confirmed "replace" branch of the prompt). `template` names the copied template and is absent on `kept`. No last-modified stamp accompanies `kept` entries — primitive results carry no wall-clock data (the same rule that keeps `write-session`'s `set-at` in the file, out of the result), so the exec envelope stream stays deterministic; the prompt's timestamp listing stays a markdown-only-path detail. A missing feature directory is an operational error (`create-feature` owns directory creation).

### `check-review-gate` — evaluate implement's pre-done review gate

Args:

```json
{ "feature": "042-widget" }
```

Result:

```json
{
  "passed": false,
  "blocked-by": "must-violations",
  "message": "blocked: spec has 3 MUST violation(s) — see specs/042-widget/review.md",
  "guidance": "Resolve the violations and re-run /ductus:review, or run /ductus:review --waive <rule-id> --reason \"...\" for each waivable finding.",
  "violations": []
}
```

The deterministic surface behind `/ductus:implement`'s completion gate. Evaluates every check in documented order, first failure wins. `blocked-by` names the failing one:

0. **`already-done`** — the spec is at `status: done`, so the transition this gate authorizes has already happened. Deliberately **not** `passed: true`: a pass for a spec the gate did not examine is the `QUAL-CLAIM-001` conflation, and could read as authorization to transition it again. Ordered first, so a completed spec is never held against a lint violation introduced after it closed, and because the freshness checks would answer wrongly: the completing `set-status` rewrites `spec.md`, an analyze subject. Only the exact value `done` short-circuits; an unrecognized status falls through to the checks, since `validate-frontmatter` owns reporting it (scenario `a-done-spec-has-no-transition-to-gate`).
1. **`markdown-lint`** — the feature directory's markdown lint (recursive `{root}/{feature}/**/*.md` glob through the `lint-markdown` machinery). Violations are echoed in `violations`, or a non-zero exit the parser could not attribute.
2. **`scenario-open-questions`** — one or more of the feature's scenarios carry unresolved open questions. The message names the count and the scenarios; `guidance` points at scenario-targeted `/{project}:clarify`, the only command that can resolve them. Ordered ahead of the review-record checks because an unresolved design question is the more upstream defect — reviewing a design that is about to change wastes the review. The list comes from `read-spec`'s collector, so the gate blocks on exactly the questions the reader reports ([046 — Scenario open-question visibility](../046-scenario-open-question-visibility/spec.md)).
3. **`undischarged-cross-spec-impact`** — the spec declares `cross-spec-impact` entries that are not discharged. See **Cross-spec impact** below.
4. **`not-reviewed`** (`review.md` absent, or its `last-run` missing or null) and 5. **`must-violations`** (`blocking: true`) — the review record, read from `review.md`. A `review.md` that exists but carries no parseable record is a third state, **undeterminable**, reported as such rather than as never-reviewed.
6. **`review-stale`** — the recorded review no longer describes the spec's **durable contracts**. See **Record freshness** below.
7. **`not-analyzed`** (`analysis.md` absent, or its `last-run` missing or null) and 8. **`analyze-findings`** (`blocking: true`) — the analyze record, read from `analysis.md`, with the same undeterminable third state. Ordered after every `review:` check because the pipeline is `review → analyze → done`: a spec whose review is missing or failing has not reached the point where analysis is the next thing owed, and naming the later gate for an earlier defect sends a contributor to the wrong command. An **advisory** analyze finding never blocks — that tier is made of checks introduced advisory with their own published promotion criteria, and gating here would promote all of them at once (spec 047; scenario `write-analysis-and-the-second-gate`).
9. **`analyze-stale`** — the recorded analysis no longer describes the spec's **analyze subjects**. See **Record freshness** below.
10. **`record-predates-dispositions`** — `review.md` or `analysis.md` has no `dispositions:` map, so it predates finding dispositions; absence is not zero. The message names the record, review before analysis, and the command to re-run.
11. **`undispositioned-findings`** — either record's `dispositions.undispositioned` is above zero. The message names the count per record and the command that dispositions them. The block asks for a decision, not a fix: discarding a false positive with its reason clears it.

Checks 10 and 11 sit after every other review and analyze check, because each of those names a more upstream defect.

**No check reads `folds-into`.** A pending fold never holds `done` ([051](../051-branch-scoped-spec-numbering/spec.md), scenario `a-pending-fold-never-holds-done`): a branch-scoped spec completes its own pipeline, fold-back consolidates its content afterwards, and `check-unfolded-specs` and the pipeline view carry the owed fold past `done`.

Each staleness check is ordered last within its half, because it is the weakest claim of its group: the others say a record is missing or failing, these say a passing record is out of date.

#### Cross-spec impact

`cross-spec-impact:` is a hand-authored list of feature slugs a spec owes a change to under §cross-spec-impact ([050](../050-constitution/spec.md), scenario `a-declared-cross-spec-impact-gates-done`). Check 3 sits ahead of the review record because a spec carrying an obligation nobody has discharged is not a candidate for `done`, so whether its review is fresh does not yet matter.

`cross-spec-impact` in the result is a per-entry list, not a boolean — partially discharged is the normal state of a multi-entry declaration:

| state | meaning |
| --- | --- |
| `discharged` | the named spec links back to the declaring spec, from its own body or from a scenario under it |
| `undischarged` | the named spec exists and carries no such link |
| `target-missing` | the entry names no feature directory — a typo, reported rather than treated as discharged, since otherwise one letter disables the gate |
| `self-reference` | the entry names the declaring spec, which cannot owe itself a change |

**Discharge is the reciprocal link, not the key's removal.** A key an author deletes to unblock themselves enforces nothing; a link they have to add lands the change where the affected spec's reader meets it — which is the signpost §cross-spec-impact already requires. A link from a scenario under the target counts, because a reader of that spec reads its scenarios.

Link reading reuses `derive-dependencies`' matcher rather than introducing a second parser, over the **pointer** scope rather than the edge scope that matcher is normally paired with. The edge scope drops blockquote-prefixed lines by policy, and a signpost on a `done` spec is written as a blockquote in this corpus — reusing that scope whole would make the canonical discharge artifact invisible to the gate built to require it.

**The list is empty in two different situations**, and `blocked-by` distinguishes them: the key is absent or empty (absence is never a finding — most specs affect no other spec), or an earlier check blocked first and this one never ran. An empty array is "not examined here", never "no declarations". On a **pass** the list is populated, so a declared-and-proven obligation does not render identically to one that was never declared.

There is no `--fix`. Which section of the affected spec should carry the signpost is the routing judgment `/{project}:amend` puts to the operator, and a wrong auto-write into another spec is worse than a precise refusal.

**A missing target blocks here, where an owed fold never blocks at all**: nothing but this check carries an undischarged impact past `done`, so it blocks on a missing target too. The two keys point in opposite directions — a fold moves the declaring spec's own content into its home, an impact asks for a change to another spec — and share no code (scenario `the-cross-spec-impact-gate`).

#### Record freshness

Both staleness checks are one computation — `analyze_subjects::freshness_of`, returning `RecordFreshness` (`never-run` / `current` / `stale` / `undeterminable`) — differing only in which files are the subject:

| record | subject set | digest field |
| --- | --- | --- |
| `review.md` | `scenarios/*.md`, `data-model.md` | `reviewed-digest` |
| `analysis.md` | every `.md` under the feature — `review.md` and `spec.md` included, `analysis.md` itself **excluded** | `analyzed-digest` |

That inversion is deliberate. `review.md` is `/{project}:review`'s **output**, so counting it would stale every review the instant it was recorded, and `spec.md` stays outside the review set because counting it would stale every review on any spec-body edit. Both are `/{project}:analyze`'s **inputs**, and `review-state-drift` asserts on `review.md`'s record directly, so excluding them there would exempt the edit that most often invalidates an analyze record. `tasks.md` and `plan.md` are outside the review's set for a separate reason — the first is ephemeral by construction (§tasks-phase), the second churns as Affected Files are revised.

**The comparison is of content, not commits.** Both commands read the **working tree**, while `reviewed-against` and `analyzed-against` record a **commit**; the two coincide only on a clean tree, which at the moment either command runs it usually is not, so comparing the recorded commit would block records whose run read the current content. Each record carries a per-path sha256 of what its run read. `analysis.md` is excluded from its own subject set — the record is written after the subjects are read, and its report body changes with its findings, so a digest covering it could never match — and `spec.md` is digested whole. The two shas survive as **provenance**, read for exactly one thing: the mechanical-sweep rename exemption, which genuinely needs two trees and whose candidates are reported rather than dropped when they are unavailable.

**A record with no digest is `undeterminable`** — not current, not stale, and not a sha-diff fallback. It does not block, so freshness stops being *enforced* for a spec until its next run writes a digest; it is self-healing rather than a hole, and deliberately not a grandfather clause — the record is not exempt, it is unreadable, and the passing verdict's `guidance` says which. `reviewed_digest` is an `Option` for a case the analyze side cannot have: a spec with no scenarios and no data model records an **empty** digest, which is current, and `Some({})` is the only way to distinguish that from `None`. `spec.md` is always an analyze subject, so an empty analyze digest cannot occur.

A **passing** gate therefore still carries `guidance` when either record is undeterminable (scenario `the-committed-tree-horizon`).

`message` is the canonical blocked text with the adopter's `[host] project` command namespace substituted into the `/{project}:review` references; `guidance` carries the resolve-or-waive follow-up on the `must-violations` branch and the scenario-targeted-clarify pointer on the `scenario-open-questions` branch. Every verdict is a domain outcome — the host halts on a blocked gate; the primitive never errors for one. An unreadable or unparseable scenario contributes no questions and therefore never blocks: an unknown is not escalated into a defect. It is still *reported* — `read-spec` returns it in `scenario-files-unreadable` and `check-artifacts` records it as a `skipped` target — so a passing gate over an unread scenario stays distinguishable from a passing gate over a fully-examined one. Not blocking and not reporting are separate obligations; this gate owes the first, the readers owe the second.

### `append-question` — append one bullet to Open Questions

Args:

```json
{ "feature": "042-widget", "question": "Should rate limits be configurable per tenant?", "scenario": "retry-on-timeout" }
```

Result:

```json
{
  "path": "specs/042-widget/scenarios/retry-on-timeout.md",
  "appended": true,
  "section-created": false,
  "status-reverted": false
}
```

The deterministic surface behind `/ductus:amend`'s question-route write. Appends `- {question}` to the target artifact's `## Open Questions` section — the feature's `spec.md` by default, `scenarios/{slug}.md` when `scenario` is passed (slug validated against the framework slug grammar; `question` is single-line, embedded newlines rejected).

Dedup uses amend's normalized-whitespace comparison (collapse whitespace runs, trim, case-insensitive) against exactly the entries `read-spec`'s question parser reports (continuation lines folded, placeholders skipped), so the runtime and markdown-only paths agree on question identity; a match is the `appended: false` domain outcome with the existing entry echoed verbatim in `duplicate-of`, nothing written. A missing section is created per template order — immediately before `## Resolved Questions` when present (the scenario scaffold), else at end of file (the spec template) — and a `*None …*` scaffold placeholder is replaced by the first real entry. On a spec target whose status is `clarified`, `planned`, or `in-progress`, the frontmatter status reverts to `draft` in the same atomic write (`status-reverted` + `previous-status` report it) — never a window where the body holds an unresolved question while the status claims otherwise. `done` is **excluded** from the back-edge (§spec-lifecycle, spec 014): a `done` spec reopens only via the scenario route (`done → in-progress`), so a question appended to a `done` spec is recorded but leaves the status at `done` — the command layer never routes a question there. Scenario targets have no status field and never back-edge; a status value outside the lifecycle set is left alone (`validate-frontmatter` owns flagging it).

### `diff-cross-spec` — implement's cross-spec impact surface

Args:

```json
{ "feature": "042-widget" }
```

Result:

```json
{
  "first-commit": "b39f2727dc6939ad145ede1830205e0d122075d3",
  "current-head": "550c8ddc33e6895da0ce6c81fa6f6e2c42049e9f",
  "cross-spec-paths": ["specs/007-sibling/spec.md"]
}
```

The deterministic filter behind `/ductus:implement`'s per-task summary and completion gate. Diffs the feature's first spec-dir commit — the same base `derive-boundary` computes, through the shared revwalk helper — against the **working tree** (index and untracked files included), scoped to the spec root: `cross-spec-paths` lists changed paths outside the feature's own directory (sorted; `{specs-root}/inbox.md` excluded, since it holds what a person logs, which is no spec's impact). The working-tree diff is why the per-task summary, which fires before the task's commit, sees the run's uncommitted sibling edits; on a clean tree the result equals the documented `git diff <first-commit>..HEAD -- {specs-root}/` form. Read-only; an empty list is the no-impact domain outcome.

Resolved fork (scenario open question): a separate primitive rather than a mode on `derive-boundary` — the two results share only the diff base, so they share the `first_commit_for_prefix` walk as a `pub(crate)` helper instead of a result-shape union.

### `append-inbox` — append one bullet to the inbox

Args:

```json
{ "text": "security: token logged in plaintext — src/auth.rs" }
```

Result:

```json
{ "path": "specs/inbox.md", "created": false, "item-count": 4 }
```

Appends `- [ ] {text}` (the checkbox inbox form the inbox template and constitution §bug-handling document) atomically to `{specs-root}/inbox.md`, creating the file when missing (from `framework/templates/project/inbox.md` when that file exists on disk — the framework source repo — else a bare `# Inbox` heading). Bullet scanning, for the count, is comment/fence-aware — a `-` line inside the template's `<!-- Rules: … -->` guidance is not an item. `item-count` reports the total inbox bullets after the call. Embedded newlines in `text` are rejected as an operational error (structure injection), matching `append-task`'s single-line rule.

### `check-orphaned-references` — adopter-owned files pointing at paths that are gone

Args: none.

Result:

```json
{
  "findings": [
    {
      "referrer": "CLAUDE.md",
      "target": ".ductus/constitution.md",
      "line": 3,
      "migration": ""
    }
  ],
  "examined": ["CLAUDE.md", ".githooks/pre-commit"],
  "skipped": [],
  "matched-prefixes": [
    ".ductus/",
    ".githooks/",
    ".govern/",
    "scripts/gen-",
    "scripts/lib/",
    "specs/"
  ],
  "attribution": "watermark",
  "last-applied": "ductus-rename"
}
```

The runtime half of [027](../027-bootstrap-migration-registry/spec.md)'s `migration-chain-reference-integrity`, invoked from **two** call sites so one rule has one implementation: `/{project}:analyze` §Project-level consistency (the durable adopter-facing surface) and `/{project}`'s Pre-run Migrations at batch end. Read-only, and it reports rather than repairs — the adopter may have hand-edited the reference, and a rewrite that guessed wrong is worse than a report that is precise.

- **Referrers** are the `create`-strategy files the manifest never overwrites and no migration re-points: `CLAUDE.md`, `AGENTS.md`, `README.md`, `.githooks/pre-commit`, and the spec root's `system.md`, resolved against `[paths] specs-root`. They are enumerated in code because the **Shared Files** manifest is never installed, so a list derived from it would be empty in the adopter checkout where the check runs.
- **`system.md` is examined but never repaired.** `ductus-rename` re-points only the references the framework wrote; an adopter's `system.md` reference is theirs, so it is reported and left alone (scenario `orphan-check-adopter-authored-references`).
- **Managed roots** are `.ductus/`, `.githooks/`, the configured spec root, and the **historical** roots `.govern/`, `scripts/gen-` and `scripts/lib/`. The orphan a migration leaves carries the root *before* the move, so a current-roots-only list is blind to it; the pre-042 roots are prefixes because the framework never owned all of `scripts/` (scenario `orphaned-reference-historical-roots`). A broken link into an adopter's own directories is out of scope.
- **`matched-prefixes` bounds the claim by scope, as `examined` bounds it by subject**: a clean result means no orphans among paths carrying one of these prefixes (`QUAL-CLAIM-001`).
- **`attribution` says what the empty `migration` means.** Under `registry` (`framework/migrations.toml` readable, the bootstrap call site) a finding's `migration` names the entry whose `target_paths` covers the missing path, latest hop winning, and an empty one means *no entry claims it*. Under `watermark` (an adopter checkout) there was nothing to ask, so `migration` is empty and `last-applied` is the only migration context (scenario `orphaned-reference-check`).
- **A pattern is not a reference.** A candidate containing `*` or `NNN` names a shape (`specs/NNN-*/spec.md`), and a path the project declares it *ships* to adopters is not local breakage.
- **A git object name is not a reference.** A path glued to a preceding `:` whose token is empty or revision-shaped — `git show abc1234:specs/x/spec.md` — names a blob no migration can move; a variable, a path or a link label before the colon marks a working-tree reference, which is still checked ([`a-revision-qualified-path-is-not-a-working-tree-reference`](scenarios/a-revision-qualified-path-is-not-a-working-tree-reference.md)).
- `examined` names the files the result describes and `skipped` the ones it could not read, so empty `findings` reads as clean only when `skipped` is empty.

### `derive-routing-candidates` — the homes proposed work could already have

Args:

```json
{ "description": "report a stale review on a done spec from the check-artifacts family", "routed-by": null }
```

Result:

```json
{
  "description": "report a stale review on a done spec from the check-artifacts family",
  "candidates": [
    {
      "route": "scenario",
      "source": "runtime-work",
      "target": "022-deterministic-runtime",
      "path": "specs/022-deterministic-runtime",
      "status": "in-progress",
      "reopens": false,
      "reason": "names the primitive `check-artifacts`; this spec's plan claims `runtime/`"
    }
  ],
  "sources-examined": ["runtime-work", "rule-surface", "spec-corpus"],
  "skipped": [],
  "gate-required": true,
  "derivation-incomplete": false
}
```

The deterministic half of `/{project}:specify`'s routing gate, which runs **before** `create-feature` writes anything, because a routing check after the scaffold is not a check. Three sources, in result precedence order:

- **`runtime-work`** — the description names a primitive (matched against the parser's `PRIMITIVE_NAMES`, so the signal tracks the shipped runtime rather than a hand-kept keyword list) or a path under `runtime/`. The home is the feature whose `plan.md` lists a `runtime/` path under `## Affected Files`, read through the same `compute_review_scope::read_plan_affected` the review scope uses. **Derived from the corpus, never a slug named in code**: an adopter's runtime-owning spec is not this repo's `022`, and a project with none yields no candidate rather than a wrong one.
- **`rule-surface`** — a rule file whose category stem the description shares, with the `-backend` / `-frontend` / `-cross` surface suffix set aside so `security-backend.md` matches on `security` and not on every backend file. Directory resolution and listing are `discover-rule-files`' (`{specs-root}/rules/`, else `framework/rules/`).
- **`spec-corpus`** — a spec whose slug shares vocabulary with the description. A spec already offered by `runtime-work` is not offered twice; the more specific claim wins.

Matching is lexical (lowercased alphanumeric tokens of four or more characters, minus a short stopword list) and the result is **advisory**. The semantic decision stays at the `routeInboxItem` extension point — groom's tree, reused rather than copied — and the choice stays with the operator: a new spec remains creatable over any candidate, and the gate's denial branch is how the operator takes one instead. `reopens` is true when accepting a candidate implies the `done → in-progress` back-edge, so the confirmation names the reopen before it happens exactly as `/{project}:groom`'s does. `route` is drawn from the `routeInboxItem` vocabulary; `spec` (create a new one) is the *absence* of candidates and never appears as one.

**Every source lands in `sources-examined` or in `skipped`, never both and never neither.** That invariant is what makes the two zero-candidate answers distinguishable: empty `candidates` with empty `skipped` is *examined and matched nothing* — a new spec is right, and a fresh adopter with no rule directory and a single-spec corpus is exactly this case; empty `candidates` with a non-empty `skipped` is *could not derive candidates*, a different answer that must not be reported as the first (`QUAL-CLAIM-001`). The runtime-work signal firing with no derivable home is a skip, not silence: that is precisely when the operator most needs to know the check could not answer. Only an empty `description` is an operational error — every other failure is reported as a skipped source, because raising would collapse "could not check" into "no gate".

`routed-by` names a command whose routing tree already ran (`/{project}:groom` recommending `/{project}:specify` is the case); it returns `gate-required: false` with no candidates and no skips, and the caller skips the decision and its confirmation rather than asking twice. `description` carries a `title` alias so the exec walker binds it and `create-feature` from one context value, and is echoed in the result so the routing extension reads one key whichever name the caller supplied.

Defined by `scenarios/specify-routes-before-scaffolding.md`; requiring spec [017 — Derive, Don't Ask](../017-derive-dont-ask/spec.md) AC26.

### `routeInboxItem` — `candidates` request field (addendum)

The request gains an optional `candidates` array carrying `derive-routing-candidates`' output, `skip_serializing_if` empty so the `/{project}:groom` payload is byte-unchanged. This keeps **one** routing point: `/{project}:specify` sends the proposed feature description as `item-text` with its derived candidates, so work arriving through conversation is routed by the same tree as work arriving through the inbox. Two routing rules that could disagree is the drift the scenario was written against. `item-text` falls back to the `description` key on the specify path, so neither entry point needs a second context key for the same value. The router treats candidates as evidence to weigh, never as a decision already made.

### `write-review` — `observations` argument and `## Observations` section (addendum)

Args (the observations half only; the findings/waivers/scope arguments are unchanged):

```json
{ "observations": [{ "text": "perf: the config file is re-read on every primitive call", "path": "runtime/src/schema/paths.rs",
  "disposition": { "outcome": "routed", "target": "specs/022-deterministic-runtime/tasks.md" } }],
  "decided-by": "dev@example.com" }
```

Result (the observations half only):

```json
{ "observations": 1, "dispositions": { "fixed": 0, "routed": 1, "discarded": 0, "undispositioned": 0 } }
```

An **observation** is something the reviewer judged real that maps to no loaded rule, so it cannot be a finding. `path` is optional. Observations never enter `must-violations` / `should-violations` / `low-confidence`, never affect `blocking`, and never change the exit code; they render in their own `## Observations` report section, which sits after `## Waived findings` and before `## Skipped passes` and emits `*None.*` when empty like every other section. The report frontmatter carries no `observations:` key.

Each observation carries a disposition — fixed, routed with its target, discarded with its reason, or undispositioned by default — rendered beside it in the section. The record counts the four outcomes in a `dispositions:` map, and a newly routed or discarded observation is stored under `decisions:`; nothing is written to the inbox. [058's data model](../058-findings-route-at-discovery/data-model.md) records the shapes.

`performReview`'s response carries an optional `observations` array, accumulated across passes exactly as `findings` is and filtered out of later passes' request payloads by the same rule. Each entry carries its `text` and `path` alone, since a pass does not disposition what it observes, so an exec run records every observation undispositioned. Required by spec [017 — Derive, Don't Ask](../017-derive-dont-ask/spec.md) AC25.

### `examined` / `scope` on `write-review` — the review's own denominator (addendum)

Arg (caller-supplied numerator) and result (both halves):

```json
{ "examined": 38, "scope": 46 }
```

Without a denominator, a review whose passes read the whole scope and found nothing and a review whose passes never ran would write **byte-identical** records — the reason `write-analysis` requires `unexamined` (scenario `a-review-states-what-it-read`).

The asymmetry between the two fields is the design. **`scope` is derived by the primitive**, resolving `compute-review-scope` against the run's own `diff-base` — the discipline that derives `blocking` and `reviewed-digest`, for the same reason: a caller that supplied it could shrink the subject to match whatever it read. **`examined` is the caller's claim**, because how many files the passes actually read is the one thing only the host knows.

Both are written to `review.md`'s frontmatter. An **unstated** `examined` is recorded as absent, never as a computed zero: a claim never made and a claim that came back empty are different facts, the same distinction §grounding draws between *could not examine* and *examined and found nothing*.

**Nothing reads these two fields**; they are read at the completion gate by a person.

**What it does not do.** It cannot prove the passes ran — an overstated `examined` is as available as an omitted one, exactly as an understated `unexamined` is. It makes the claim explicit and checkable rather than invisible, which is the bar `QUAL-CLAIM-001` sets and the bar the analyze half already cleared.

A record predating the fields carries neither key, so there is no denominator to judge until the next review of that spec writes a `scope`.

### `process-decisions` — classify stored decisions before dispositions

Added by [058 — Findings route at discovery](../058-findings-route-at-discovery/spec.md). Args:

```json
{ "feature": "042-widget", "record": "analysis",
  "fired": ["grounding — plan.md cites a missing path"], "restricted": false }
```

Result:

```json
{ "matched": [{ "key": "grounding — plan.md cites a missing path", "outcome": "routed", "target": "specs/042-widget/tasks.md" }],
  "expired": [], "retained": [], "notices": [] }
```

Classifies the routed and discarded decisions in `review.md`'s or `analysis.md`'s `decisions:` list against this run's finding keys, before the host proposes any disposition, so a settled finding is counted under its stored outcome rather than asked about again — the way `process-waivers` classifies waivers ahead of `write-review`. **matched**: the key fired. **expired**: it did not, on an unrestricted run, and the writer drops it — for a routed finding, the moment the routed work landed. **retained**: it did not, but `restricted` says the run did not evaluate every source. A malformed entry or a repeated key produces a notice. The primitive prunes nothing; the writer keeps a malformed entry, drops an entry with no fields at all, and drops a repeated key's entries when that key expires or is decided again (058's data model). `fired` is MCP-only, as `process-waivers`' is, and each fired key is compared in the flattened single-line form the writers store. Read-only; a `decisions:` list that does not parse is an error, never an empty result. The shapes are recorded in [058's data model](../058-findings-route-at-discovery/data-model.md).

### `remove-inbox-item` — remove one bullet from the inbox

Args:

```json
{ "item": "security: token logged in plaintext — src/auth.rs" }
```

Result:

```json
{ "path": "specs/inbox.md", "removed": true, "remaining-count": 3 }
```

The complement of `append-inbox` and the deterministic surface behind `/ductus:groom`'s per-item removal (step 8). Removes the first bullet from `{specs-root}/inbox.md` whose text — after the `-` bullet marker and an optional `[ ]`/`[x]` checkbox are stripped via the shared bullet grammar — equals the trimmed `item`, writing atomically. Bullet scanning is comment/fence-aware (shared with `append-inbox`), so a `-` line inside an HTML comment is neither counted nor removable. A double blank left at the removal seam is collapsed and the file ends in a single newline. A no-match, or a missing inbox file, is a clean domain outcome (`removed: false`), never an operational error; `remaining-count` reports the bullets left after the operation. Embedded newlines in `item` are rejected (single-line rule).

### `check-command-flags` — a declared interface vs. the body a host substitutes

Args: none.

Result:

```json
{
  "findings": [
    {
      "command": "framework/commands/review.md",
      "direction": "flag-unsurfaced",
      "flag": "--since",
      "reason": "Flags table documents --since but argument-hint omits it, so it is never surfaced"
    }
  ],
  "examined": ["framework/commands/amend.md", "framework/commands/review.md"],
  "with-flags-table": ["framework/commands/review.md"],
  "with-argument-hint": ["framework/commands/review.md"],
  "skipped": [],
  "commands-dir": "framework/commands",
  "guidance": ""
}
```

The runtime half of [020](../020-code-review/spec.md)'s `review-flag-parsing-is-specified`, invoked from `/audit` Family 30 (`scripts/audit/command-flag-hint-parity.sh`). It checks two directions of one contract — a declared `argument-hint` and the body a host substitutes into must agree. **Direction 1:** `argument-hint` is the surface a host renders when it offers a command, so a flag absent from it is a flag the operator is never shown. **Direction 3**, below, is a declared hint the body never substitutes ([this spec's `argument-hint-needs-a-token`](scenarios/argument-hint-needs-a-token.md)).

- **The subject is `framework/commands/*.md`, the sources**, never a host's generated copies: a copy carries whatever its source carries, and an adopter cannot repair one, so the check lives in `/audit` at the source rather than in `/{project}:analyze`.
- **Only a `Flags` section's table rows count, and only each row's first cell**, since the behavior column cross-references other flags. A cell may name two flags (`--waive <rule-id> --reason "<text>"`), so it is scanned rather than matched once.
- **A command documenting flags in prose is examined and contributes nothing**, so `with-flags-table` sits beside `examined`: an empty `findings` means *every tabled flag is surfaced*, never *every documented flag* (`QUAL-CLAIM-001`).
- **Direction 3** follows the scenario: the accepted token set, and every examined command declaring a hint as its subject. `with-argument-hint` is its denominator, kept separate from `with-flags-table`, and its findings carry `direction: "hint-unreachable"` against direction 1's `"flag-unsurfaced"`, so a consumer renders the right fix without reading prose.
- **Not covered: direction 2**, a body documenting an argument the hint omits. Deciding whether the hint should name it needs a semantic reading of the prose, and a positional argument has no table to scan; it is stated so it is not mistaken for coverage.
- **`guidance` is set when the run examined command files and found no Flags table at all**, so an extraction failure does not return a clean run's payload; empty otherwise.
- **Section membership comes from the shared fence- and comment-aware scanner** (`section_line_indices`), so a table inside a fenced example is not the command's contract, and Family 30's script stays a shell entry point over this primitive rather than a markdown parser of its own ([§runtime-boundary](../../framework/constitution.md#runtime-boundary) principle 3).

### `check-artifacts` — deterministic artifact-check families for one feature

Args:

```json
{ "feature": "022-deterministic-runtime" }
```

Result:

```json
{
  "feature": "022-deterministic-runtime",
  "status": "planned",
  "findings": [
    {
      "family": "artifact-completeness",
      "severity": "blocking",
      "message": "plan.md is required at status 'planned' but does not exist",
      "path": "specs/022-deterministic-runtime/plan.md"
    }
  ],
  "clean": false,
  "skipped": [
    {
      "family": "link-adjacent-drift",
      "reason": "target-missing",
      "path": "specs/022-deterministic-runtime/scenarios/renamed.md"
    }
  ],
  "path": "specs/022-deterministic-runtime/spec.md"
}
```

Nine families, mirroring `/ductus:analyze`'s markdown-only reference exactly (severity tiers included — the primitive mechanizes the documented policy): `artifact-completeness` (blocking — `plan.md`/`tasks.md` required at `planned`/`in-progress`/`done`; `data-model.md` never required), `task-consistency` (blocking, when `tasks.md` exists — strictly-increasing numbering, `Done when` presence), `scenario-consistency` (advisory — every `scenarios/*.md` has a referencing task, skipped for `done` specs and satisfied by §tasks-phase pruning evidence: zero task sections or non-contiguous numbering), `review-state-drift` (blocking, three conditions — a `done` spec with `review.last-run` unset, with `review.blocking: true`, or with a non-zero `review.should-violations`, the third because §implement-phase forbids reaching `done` over an outstanding SHOULD; a `done` spec with no `review.md` at all is grandfathered), `scenario-open-questions` (blocking at `done`, advisory otherwise), `link-adjacent-drift` (advisory — prose asserting an open state that its own sibling link's target contradicts), `criterion-path-existence` (advisory — a filesystem path named in a `done` spec's acceptance criterion that no longer resolves), `criterion-labels` (advisory — a duplicate `AC{n}` within one spec, a `next-criterion` that no longer exceeds the body, and an unlabelled criterion in a spec that carries a counter), and `disposition-drift` (blocking — a `done` spec whose `review.md` records undispositioned findings, silent for a record with no `dispositions:` map). **No family judges `analysis.md`**, neither its dispositions nor its own state: every family runs inside `/ductus:analyze`, whose own run is about to replace that record, so a finding read from it could never clear, and `/ductus:analyze` judges both from the record it writes instead. `--all` iteration stays with the caller. The command-frontmatter-completeness family stays in the markdown-only reference (it reads the host's command directory, which the runtime does not own).

`scenario-consistency`'s **referencing-task rule is canonical for every surface that asks "does a task reference this scenario?"** A task references a scenario when the scenario's **slug** appears in the task's heading, in any subtask's text, or in its `Done when` clause. The match is on the slug, not on the `scenarios/{slug}.md` path: the path form is what `append-task`'s default body emits, and the wider slug match is what tolerates a hand-written task naming the scenario without it. A second surface applying a narrower rule disagrees asymmetrically — `/{project}:amend`'s reconcile pass would offer a task for a scenario this family already considers mapped, producing the duplicate its dedup exists to prevent — so a surface restating the rule (as the markdown-only path must, per §runtime-host-integration) states *this* rule. `mapped_scenario_produces_no_finding` and `bare_slug_reference_satisfies_the_mapping` cover both authoring forms, so a narrowing on the runtime side fails a test rather than shipping.

`scenario-open-questions` reports when any `scenarios/*.md` carries an unresolved question, naming the count and the scenarios. At `done` that contradicts the completion rule, so the finding is blocking and `/{project}:analyze --fix` reverts the spec `done → in-progress` with a non-silent notice, as it does for review-state drift; before `done` it is advisory and `--fix` leaves the status alone. There is **no grandfather rule**: an unresolved scenario question is a present-tense defect whenever it arrived. The question list comes from `read-spec`'s collector, so this finding, the `check-review-gate` block and the count the user sees cannot disagree ([046 — Scenario open-question visibility](../046-scenario-open-question-visibility/spec.md)).

`criterion-labels` is the **enforcement half** of the labelling pass above ([013](../013-text-first-artifacts/scenarios/criterion-identifiers.md)): assignment is `label-criteria`'s, enforcement has to be separate because a criterion typed by hand in an editor never touches a primitive. Its three invariants are checkable from the artifact alone with no git-history read — a duplicate label is ambiguous rather than resolvable by picking the first match; a counter at or below the body maximum means the next assignment would reissue a label a live criterion already carries; and an unlabelled criterion is a defect **only once the spec carries `next-criterion` at all**. That last gate is the field's presence, deliberately not a per-spec grandfather date: 013 defines an absent counter as "no labels assigned yet", and the corpus backfill is what makes the check universal rather than an exemption list. The family runs at every status — a label is an identifier, not a contract about the delivered system, so a `draft` is as wrong to duplicate one in as a `done` spec — and it contributes nothing to `skipped`, since its whole subject is the spec's own frontmatter and criteria list.

`skipped` records each target a family could not examine, as `{family, reason, path}` over the closed reason set `target-missing` / `target-unparseable` / `no-readable-state` / `root-absent` / `ships-to-adopter` / `artifact-unreadable` / `not-a-live-claim`, because a family that read *targets* — a link's destination, a criterion's path — must not escalate an unreadable one into a finding ([045 — Decision-state drift detection](../045-decision-state-drift-detection/spec.md)). `clean` still means `findings.is_empty()`, so the assurance lives in the pair: `clean: true` with an empty `skipped` is verified-clean, and with a non-empty one partially examined. Hosts render the entries in the Informational tier (scenario [`check-artifacts-skipped-targets`](scenarios/check-artifacts-skipped-targets.md)). A sibling whose path traverses a symlink at or below the feature directory is reported `target-unparseable` rather than followed ([sibling-symlink-trust-boundary](scenarios/sibling-symlink-trust-boundary.md)).

### `derive-dependencies` / `derive-references` — the frontmatter index derivations

The two derived frontmatter indexes, promoted out of the shipped bash
generators by the `adopter-generator-promotion` scenario. Both harvest links
from the same body region — the shared scanner drops frontmatter, fenced code,
blockquote-prefixed lines, and `## See also` sections; `## References` is
deliberately not an opt-out — and differ in what they match and what they write.

Both accept `write` (perform the rewrite) and `staged` (limit the rewrite to
specs in the pending commit). **`write` is off by default**: the subprocess
interpreter has no per-step argument binding, so a step naming a primitive in a
code span is dispatched with no arguments at all. A writing default would make
every safety-net step in `/target`, `/clarify`, `/plan`, and `/implement`
rewrite the whole corpus. `run-generator` reaches the same guarantee by
hardcoding `--dry-run`; this is that guarantee expressed in the argument.

```json
{
  "drift": true,
  "updated": ["specs/017-derive-dont-ask/spec.md"],
  "unwritten": [],
  "absent": [],
  "examined": 51,
  "untracked-skipped": [],
  "cycles": [],
  "specs-root": "specs",
  "unparseable": [],
  "wrote": false
}
```

- `unwritten` — specs examined and found drifted but deliberately not written,
  the `staged` case. Neither "in sync" nor "not examined", so they are reported
  separately rather than folded into either. Present on **both** derive
  primitives: a `references:` entry derives from the body *and* the
  `[services]` registry, so a service rename drifts specs nobody edited or
  staged (scenario `derive-references-unstaged-drift-is-reported`).
- `absent` — specs tracked in the git index but missing from the worktree
  (deleted without staging the deletion). Nothing can be derived from one, so it
  is named rather than dropped, and it is excluded from `examined`, which
  counts only what the run read. Present on both derive primitives.
- `untracked-skipped` — untracked specs are never enumerated or rewritten
  (spec 017, `tracked-specs-not-worktree`). Reported so an empty `updated`
  cannot be read as "everything is in sync".
- `cycles` (`derive-dependencies` only) — each cycle's members sorted, the
  cycles sorted by least member; a single-member entry is a self-link. A
  **domain outcome**, not an error: the MCP surface returns it and the host
  decides. The CLI surface maps it to a non-zero exit, because its callers are
  the pre-commit hook and CI, where blocking is expressed as an exit code.
- `registered-services` (`derive-references` only) — registry size, so "no
  references found" stays distinguishable from "no readable config".
- `unparseable` — specs whose frontmatter opens a block it never closes, so
  neither splice could find its anchor and nothing was derived. Reported, never
  repaired (`validate-frontmatter` owns diagnosing a malformed block) and never
  an error: an unparseable spec is an unknown, not drift, so it does not affect
  the CLI's blocking contract. **An empty `updated` means examined-and-clean
  only when `unparseable` is also empty** — the `QUAL-CLAIM-001` pairing
  `check-artifacts` has with `skipped` and `derive-boundary` with `guidance`.
  A file with no frontmatter at all is not unparseable: there is no block to
  close, and reporting every such file would bury the signal.

`derive-dependencies` writes `dependencies: []` when a spec has no sibling
links — the key present and empty. `derive-references` instead removes the
`references:` key entirely. The asymmetry is deliberate: unifying the two would
rewrite frontmatter across every spec in a corpus.

## Extension-point schemas (initial release)

The three initial-release single-shot extension points, plus the follow-on points: `askClarifyQuestion` and `routeInboxItem`, whose typed shapes ship ahead of their scenarios per the extension-request-hygiene scenario; `verifyCriteria`, which ships with the implement-completion-gate scenario as `/ductus:implement`'s criterion-verification seam; and `routeFold`, `/ductus:fold`'s body-edit-or-scenario decision (spec 051). Each has request and response payload schemas; the runtime validates incoming responses against these and emits `error: schema-mismatch` on failure. An extension identifier outside this closed set is an `error: unknown-extension` at request-build time — never a raw walker-context dump. In every request that carries legacy-compat context fields after its typed prefix (`writeCode`, `writeSpecBody`, `performReview`), walker-internal accumulator keys (prior `llm:*` response echoes and the accumulated `findings` array) are filtered out; primitive results threaded through the context (`scope`, `diff-base`, `selected`, `rules-dir`, `notices`, …) pass through.

### `assessSpecQuality`

Used by `/ductus:analyze`'s per-rule Verification reads.

One request carries one rule. The exec walker sends one per loaded rule of the asking step's tier — step 11 asks about MUST-tier rules, step 12 about SHOULD-tier ones — so each rule is asked about once (spec 060). `rule.severity` is the tier the rule's Statement states through its RFC 2119 keyword, MUST winning when a Statement carries both; the step's prose only selects which rules it asks about. The walker evaluates no Verification trigger: the host judges whether the rule's trigger fires against the spec, and answers `passed: true` with a `null` finding when it does not. A rule with no keyword or no Verification is never sent, and is recorded under `rule-assessments-not-checked` instead.

Request payload:

```json
{
  "spec-path": "specs/022-deterministic-runtime/spec.md",
  "spec-content": "...full spec text...",
  "rule": {
    "id": "QUAL-CLARITY-001",
    "verification": "Acceptance criteria are concrete and testable",
    "severity": "must"
  }
}
```

Response payload:

```json
{
  "passed": false,
  "finding": {
    "severity": "must",
    "rule-id": "QUAL-CLARITY-001",
    "location": { "section": "Acceptance Criteria", "line": 213 },
    "message": "Acceptance criterion 8 ('parses cleanly') is not testable as written..."
  }
}
```

When `passed: true`, `finding` is `null`.

### `writeCode`

Used by `/ductus:implement`'s per-task work step.

Request payload:

```json
{
  "constitution-excerpts": ["..."],
  "constitution-excerpts-unexaminable": ["anchor-unresolved: §spec-phase"],
  "plan-relevant-files": [
    { "path": "runtime/src/primitives/read_spec.rs", "content": "..." },
    { "path": "runtime/src/schema/spec.rs", "content": "..." }
  ],
  "write-boundary": [
    "runtime/**",
    "specs/022-deterministic-runtime/**"
  ],
  "task": {
    "number": "3",
    "heading": "Implement read-spec primitive",
    "subtasks": ["..."]
  }
}
```

`constitution-excerpts-unexaminable` carries one entry per cause the excerpt load could not examine — `command-file-missing: {name}`, `command-file-unreadable: {path}`, `constitution-unreadable: {path}`, or `anchor-unresolved: §{anchor}` for an anchor the command's `Reference:` line declares that the constitution does not define. The constitution resolves as `.ductus/constitution.md`, then `framework/constitution.md`, and `constitution-unreadable` names both when neither exists. It is **omitted from the wire when empty** (`skip_serializing_if`), so a payload whose anchors all resolved is byte-identical to one built before the field existed, and it sits inside the stable cache prefix (it does not vary between tasks in one walk). Without it, a host could not tell "no constitutional context applies" from "the context could not be loaded" (`QUAL-CLAIM-001`).

Response payload:

```json
{
  "edits": [
    {
      "path": "runtime/src/primitives/read_spec.rs",
      "action": "create",
      "content": "..."
    },
    {
      "path": "runtime/src/primitives/mod.rs",
      "action": "edit",
      "patch": "..."
    }
  ],
  "summary": "Implemented read-spec primitive..."
}
```

Every edit path must fall within the `write-boundary`; the runtime rejects out-of-boundary edits and surfaces an `error: out-of-boundary-edit` before applying any edit. On the exec path `write-boundary` is the union of the session-seeded value and `derive-boundary`'s directory-zone globs (see that primitive's entry above); the seeded value alone carries it only until the derivation runs.

### `writeSpecBody`

Used by `/ductus:specify` and `/ductus:plan` at template-fill moments.

Request payload:

```json
{
  "template-path": "framework/templates/spec/spec.md",
  "template-content": "...",
  "section": "Motivation",
  "feature-description": "...",
  "existing-content": null
}
```

Response payload:

```json
{
  "content": "...filled-in section content...",
  "section": "Motivation"
}
```

When invoked from `/ductus:plan` to fill in plan sections, `template-path` points at the plan template and `section` enumerates the plan section to fill.

Field sourcing (extension-request-hygiene):

- `template-path` / `template-content` — resolved from the running command (`/ductus:plan` → the plan template, `/ductus:specify` → the spec template), trying `{specs-root}/templates/<file>` (the installed adopter layout) then `framework/templates/spec/<file>` (the framework source layout). Both are empty strings when no template exists on disk.
- `section` — the section heading named by the step prose ("Fill the `<name>` section …"); empty when the step fills a whole body rather than one section (`/ductus:specify`).
- `feature-description` — the `feature-description` walker-context key, seeded by the host from the slash command's `$ARGUMENTS` (session file or `key=value` exec argument); empty when the host seeds none.
- `existing-content` — the named section's current body from the file the running command owns (`/ductus:plan` reads `plan.md`, `/ductus:specify` reads `spec.md` — selected by command, never by fallback order); omitted when the file or section is absent or empty.

### `askClarifyQuestion` (follow-on)

Reserved by the [clarify-command-acceleration](scenarios/clarify-command-acceleration.md) scenario; the typed request builder ships ahead of it ([extension-request-hygiene](scenarios/extension-request-hygiene.md)) so the point never falls back to a raw context dump. One host-mediated request/response round trip per open question.

Request payload:

```json
{
  "spec-path": "specs/022-deterministic-runtime/spec.md",
  "spec-content": "...full spec text...",
  "question": {
    "text": "Should retries back off exponentially or linearly?",
    "section": "Open Questions"
  }
}
```

`question.section` is optional and omitted when the walker cannot attribute the question to a section. The question comes from an explicit `question` walker-context value when present, else the first entry of `read-spec`'s merged `open-questions` result.

**Exec-path scope note** (scenario [coverage-residue-cleanup](scenarios/coverage-residue-cleanup.md)): clarify steps 7–8 — edge-case enumeration and acceptance-criterion verification — carry no extension marker, so `ductus exec clarify` no-ops them by design. They do not fold into this point's one-question-per-round-trip ABI: they are spec-wide passes that must run even on the zero-questions short-circuit, when this loop performs no round trips at all. The markdown-only path and a host walking the command file directly perform them in full; a host driving exec performs them itself before accepting the status-advance gate. The reduction is documented in the command (`clarify.md`, Instructions preamble), keeping the two-paths guarantee honest rather than silently narrower.

Response payload:

```json
{ "answer": "Exponential, capped at 60s." }
```

The answer is the user's resolution verbatim; applying it to the spec body remains LLM work per the clarify scenario.

### `routeInboxItem` (follow-on)

Reserved by the [groom-command-acceleration](scenarios/groom-command-acceleration.md) scenario; typed builder ships ahead of it. Kept deliberately minimal: the item under decision, the closed route vocabulary (the groom decision tree's leaves, in walk order), and the specs the router may match — enough to make the routing decision without a walker-context dump.

Request payload:

```json
{
  "item-text": "Bug: retry loop never backs off",
  "routes": ["rule", "spec", "scenario", "chore", "discard"],
  "available-specs": [
    { "feature": "021-webhook-delivery", "status": "done" },
    { "feature": "022-deterministic-runtime", "status": "in-progress" }
  ]
}
```

`item-text` comes from the `item-text` walker-context key (seeded per inbox item by the groom walk); `available-specs` is scanned from the spec root (`NNN-slug` directories, sorted, with each spec's frontmatter `status` — status drives the done → in-progress reopen consent on a scenario route; empty status means the spec file was unreadable).

Response payload:

```json
{
  "route": "scenario",
  "feature": "021-webhook-delivery",
  "reason": "Durable edge case the spec covers at a high level."
}
```

`route` is one of the request's `routes` vocabulary (closed set — anything else is a schema mismatch); `feature` is present when the route targets an existing spec; `reason` is optional prose the host may surface in the per-item confirmation prompt.

### `routeFold` (follow-on)

Introduced by [051-branch-scoped-spec-numbering](../051-branch-scoped-spec-numbering/spec.md): `/ductus:fold` asks, per branch-scoped spec, what shape its content takes in the upstream spec it folds into. **Deliberately not `routeInboxItem`.** That point's vocabulary is a closed five-route set answering *where in the corpus does this work belong*; a fold has already been told where — its `folds-into` names the destination — and is asking only what shape the content takes on arrival. Widening the inbox vocabulary to cover it would break the closedness its other callers depend on.

Request payload:

```json
{
  "feature": "1234.1-retry-budget",
  "spec-content": "...full spec text of the branch-scoped spec...",
  "scenarios": [
    { "slug": "backoff-ceiling", "section": "Retry policy" }
  ],
  "fold-target": "021-webhook-delivery",
  "target-content": "...full spec text of the upstream spec...",
  "target-status": "done",
  "target-sections": ["Motivation", "Retry policy", "Acceptance Criteria"],
  "routes": ["body-edit", "scenario"]
}
```

`target-content` and `target-sections` are both present because "which section" is unanswerable without the document: `target-sections` is the closed vocabulary a `body-edit` route names, so the router picks a heading the file actually has rather than inventing one. `target-status` drives the guarded `done` → `in-progress` reopen the fold performs. `scenarios` lists what the source spec carries; those cross over on *either* route — the route decides the body's shape, never its scenarios'.

Response payload:

```json
{
  "route": "scenario",
  "scenario": "backoff-ceiling",
  "reason": "A durable behavioral split, not a clarification of the existing policy."
}
```

`route` is one of the request's `routes` vocabulary (closed set — anything else is a schema mismatch). Exactly one of `section` (on `body-edit`) and `scenario` (on `scenario`) is meaningful; both are optional in the schema rather than modelled as a sum type, because a missing field on the chosen arm is something the *command* surfaces to the operator at its confirmation prompt rather than a deserialization failure. `reason` is optional prose the host may surface at that prompt, which is where the routing is approved before any write.

### `verifyCriteria` (follow-on)

Introduced by the [implement-completion-gate](scenarios/implement-completion-gate.md) scenario: `/ductus:implement`'s completion gate sends one request carrying every acceptance criterion, and the LLM judges each criterion against the implementation — the verification stays semantic while the surrounding tallies and checkbox flips stay mechanical. Each `met: true` verdict drives one `mark-criterion` call; a `met: false` verdict leaves its checkbox unchecked and is reported, never batch-marked.

Request payload:

```json
{
  "spec-path": "specs/022-deterministic-runtime/spec.md",
  "spec-content": "...full spec text...",
  "criteria": [
    { "index": 0, "text": "`runtime exec implement` walks the procedure to completion.", "checked": false },
    { "index": 1, "text": "Out-of-boundary edits are rejected.", "checked": false }
  ]
}
```

`criteria` mirrors `read-spec`'s merged `acceptance-criteria` result in body order; `index` is the 0-based position `mark-criterion` addresses (the two share the same comment/fence-aware section walker, so index N here is the checkbox `mark-criterion` flips at N).

Response payload:

```json
{
  "results": [
    { "index": 0, "met": true },
    { "index": 1, "met": false, "note": "boundary rejection has no covering test yet" }
  ]
}
```

`results` carries one verdict per criterion. `note` is optional prose surfaced in the completion report — a failing criterion's note explains the failure; a missing verdict for a criterion is treated as not met (the gate only flips criteria the response affirmatively confirms).

## Versioning of these schemas

Schemas evolve in lockstep with the runtime binary per §runtime-boundary's lockstep-versioning rule. A breaking schema change increments the runtime's major version. Hosts integrating against the JSON protocol pin a runtime version; mismatches are surfaced by `error` envelopes that carry `runtime-version`, per the resolved Versioning Enforcement question in the spec.

## Notes

- All paths in request/response payloads are repo-relative (use `/` separators on all platforms).
- All timestamps are ISO-8601 UTC strings.
- All commit shas are 40-character lowercase hex strings.
- Unknown JSON fields in incoming envelopes are ignored. Unknown fields in outgoing envelopes are not emitted (forward-compatibility is reserved for future spec evolution, not stowaway fields).
