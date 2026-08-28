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
        name: String,          // matches a primitive name from §The primitive library
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
  "extension-point": "writeCode | writeSpecBody | assessSpecQuality | performReview | askClarifyQuestion | routeInboxItem",
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

## Per-project file resolution

The two files the runtime reads out of the per-project directory resolve through one ordered chain each, declared once in `runtime/src/schema/paths.rs` (`CONFIG_CHAIN`, `SESSION_CHAIN`) and walked by every resolver. Newest first:

| Tier | Config | Session | Introduced by |
| --- | --- | --- | --- |
| 1 | `.ductus/config.toml` | `.ductus/session.toml` | [049](../049-rename-govern-to-ductus/spec.md) |
| 2 | `.govern/config.toml` | `.govern/session.toml` | [042](../042-consolidate-govern-per-project-files-under-govern-directory/spec.md) |
| 3 | `.govern.toml` | `.govern.session.toml` | pre-042 |

**Reads** return the newest tier that exists, falling back to the **oldest** when none does (a missing file the caller treats as "config absent" → defaults). **Writes** return the newest tier that exists, falling back to the **newest** when none does (a fresh project cuts over immediately; a pre-migration write stays on the file already holding the other sections). That empty-chain difference is the only place the two disagree.

Older tiers are never removed by a primitive — the bootstrap migration is the sole cutover. See the [`project-directory-resolution-chain`](scenarios/project-directory-resolution-chain.md) scenario.

## Primitive request/response schemas

Each primitive has a typed args struct (the CLI subcommand's `clap` derive shape) and a typed result struct. Below is the canonical JSON shape for each; the CLI surface translates command-line flags into the args; the MCP surface uses the same JSON via `rmcp` tool calls.

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
    "dependencies": ["021-runtime-boundary"],
    "review": { }
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

`open-questions` comes from the **spec body only** and is unchanged in meaning and value by spec 046 — every existing consumer keeps its behavior, and feature-targeted `/{project}:clarify` still branches on exactly this list.

`scenario-open-questions` is a **sibling** signal, never merged into it: the unresolved questions carried by `scenarios/*.md`, each tagged with its source scenario slug, in the shared scenario order. The two stay separate because they answer different questions — the body's count is what `clarified` asserts and what the `draft → clarified` edge turns on, while these are remaining work that gates `done`, so merging them would make a spec's status contradict its own body. Completion is where they join: a spec does not reach `done` while this list is non-empty ([046 — Scenario open-question visibility](../046-scenario-open-question-visibility/spec.md)). *(The earlier rationale — that merging "would route a feature-level target to feature-targeted clarify, which does not read scenarios" — no longer holds: feature-targeted clarify **reports** this field as of 022's `scenario-open-question-signal`. The conclusion is unchanged; only the reason is.)*

`scenario-files-unreadable` lists the slugs of scenario files that could not be read while collecting the above, and is **omitted from the payload when empty**. It exists so an empty `scenario-open-questions` is not two states wearing one value: *every scenario was examined and none carries a question* versus *a scenario could not be examined*. An unreadable scenario is never escalated into a `done`-blocking finding — nothing can be proven about a file that will not parse — but dropping it silently would have every consumer report clean over a subject it never read, which is `QUAL-CLAIM-001`. The collector is shared, so `check-review-gate`, `check-artifacts` and the dashboard all see the same unread set; `check-artifacts` records each as a `skipped` target with reason `artifact-unreadable`.

`label` carries the criterion's stable `AC{n}` identifier, or `null` for a criterion the labelling pass has not reached ([013 — Text-First Artifacts](../013-text-first-artifacts/scenarios/criterion-identifiers.md)). `text` keeps the `AC{n}:` prefix as authored — stripping it here would make the reported text differ from the file, and a host citing a criterion wants the line it can paste. The label is parsed by `label-criteria`'s `parse_label`, deliberately the same function `mark-criterion` resolves a `label` argument through, so every surface answering "what label does this criterion carry?" answers identically; a second matching rule is the drift this repo has already been bitten by once. Callers holding a reference across edits should prefer `label` to the positional index, which an insertion above the target silently redirects.

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

Assignment takes **`max(highest label in body, next-criterion)`**, incremented per assignment in body order — never `max(body) + 1`. That distinction is the entire retirement mechanism: deleting the highest-numbered criterion is the only case where the body maximum falls, and without the stored counter the next assignment would hand a retired label to a different requirement. The counter is monotonically non-decreasing, so it cannot.

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
    { "severity": "blocking", "field": "status", "message": "..." }
  ],
  "clean": false
}
```

### `resolve-anchor` — verify every `§<anchor>` reference resolves

Args:

```json
{ "path": "framework/constitution.md", "markers-path": null }
```

Result:

```json
{
  "references": [
    { "anchor": "runtime-boundary", "line": 459, "resolved": true }
  ],
  "unresolved": []
}
```

Scans `path` for `§<anchor>` references and resolves each against `<!-- §anchor -->` markers. The markers come from `path` itself when `markers-path` is omitted (the constitution self-consistency check); supply `markers-path` to resolve one file's references against a *different* file's markers — `/ductus:analyze` passes the constitution as `markers-path` so a spec's `§` references resolve against the constitution's sections (a spec carries no markers of its own, so resolving against itself would flag every reference as unresolved noise).

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
  "deprecated": []
}
```

### `run-generator` — invoke a bash generator in `--dry-run`

Args:

```json
{ "script": "scripts/gen-spec-deps.sh" }
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

`scenario-open-question-count` is the total unresolved questions across the spec's scenarios, and `scenarios-with-questions` names the scenarios carrying them in shared scenario order. Both are distinct from `open-question-count`, which stays spec-body-only; the two signals are never summed. They drive three rendering changes: the existing Scenarios column gains a `{count} ({n} open)` suffix when non-zero (unchanged otherwise, so the glance table grows no ninth column), the Next Action cell overrides to `clarify (scenario)`, and a callout below the table names every affected spec with its carrying scenarios — no cap, since a truncated list reads as "these are the ones needing attention" while hiding others. When a spec is in recovery state *and* carries scenario questions, `clarify (recovery)` wins the cell — it is the more upstream defect — but **both** callouts render, because the scenario questions still need resolving after the recovery walk.

The full pipeline view pre-rendered as one markdown fragment, in `/ductus:status`'s documented order: preamble, dashboard table, counts and callouts, and the cross-service references readout (blocks separated by blank lines; the readout omitted when no spec declares references). The runtime resolves each spec's `references:` index internally for the readout — the same classification `resolve-references` exposes, with the matched service's `description` appended from the `[services]` registry — so one `dashboard` call covers the whole view on the runtime path. `/{project}:…` next actions and callout texts substitute the adopter's `[host] project` namespace. Returned data the host may restyle, never stdout printing (§runtime-boundary: no user-facing rendering owned by the runtime); the structured fields stay authoritative for hosts that render their own view. The canonical piece-by-piece formats live in `/ductus:status`'s Rendering reference, which is also the markdown-only path.

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

**`path` here is the spec *directory*, and on the exec path it is pinned once written.** A successful `create-feature` (and, on `/{project}:target`, a `resolved` `resolve-feature`) retargets the session, so the walker lets its `feature` and `path` override even session-seeded values — then **pins both keys for the rest of the walk**. The pin is what keeps the later `write-session` binding a directory: nearly every spec-reading primitive that can run in between (`read-spec`, `label-criteria`, `mark-criterion`) reports its own `path`, the spec **file**, and results merge at the top level by bare key. On a repo whose session file already carries a target the seeded-key guard covered this; on a fresh repo those keys are unseeded, so without the pin the file path won and the session target was recorded as `specs/{feature}/spec.md`. The retargeting primitives stay exempt from their own pin, so a walk that resolves twice still records the second.

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

The deterministic surface behind `/ductus:implement`'s completion-gate step 13, which the host previously walked by hand on every completion attempt. Evaluates the gate's three checks in documented order, first failure wins:

1. **`markdown-lint`** — the feature directory's markdown lint (recursive `{root}/{feature}/**/*.md` glob through the `lint-markdown` machinery — the raw `npx markdownlint-cli2` invocation the step used to name). Violations are echoed in `violations`, or a non-zero exit the parser could not attribute.
2. **`scenario-open-questions`** — one or more of the feature's scenarios carry unresolved open questions. The message names the count and the scenarios; `guidance` points at scenario-targeted `/{project}:clarify`, the only command that can resolve them. Ordered ahead of the `review:` checks because an unresolved design question is the more upstream defect — reviewing a design that is about to change wastes the review. The list comes from `read-spec`'s collector, so the gate blocks on exactly the questions the reader reports; a second independent reader could disagree with the count the user was shown ([046 — Scenario open-question visibility](../046-scenario-open-question-visibility/spec.md)).
3. **`not-reviewed`** (`review:` absent or `last-run` null) and **`must-violations`** (`review.blocking: true`) — the spec frontmatter `review:` block.

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

The deterministic surface behind `/ductus:amend`'s question-route write, previously the only record-path with no primitive (asymmetric with the scenario route's `create-scenario` + `append-task`). Appends `- {question}` to the target artifact's `## Open Questions` section — the feature's `spec.md` by default, `scenarios/{slug}.md` when `scenario` is passed (slug validated against the framework slug grammar; `question` is single-line, embedded newlines rejected).

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
  "cross-spec-paths": ["specs/007-sibling/spec.md"],
  "inbox-additions": ["- security: token logged in plaintext — src/auth.rs (captured during 042)"]
}
```

The deterministic filter `/ductus:implement` steps 7 and 12 previously re-derived by hand per task (step 12's prose self-declared "no primitive owns this filter yet"). Diffs the feature's first spec-dir commit — the same base `derive-boundary` computes, through the shared revwalk helper — against the **working tree** (index and untracked files included), scoped to the spec root: `cross-spec-paths` lists changed paths outside the feature's own directory (sorted; `{specs-root}/inbox.md` excluded), and `inbox-additions` lists the bullet lines added to the inbox in the window (shared bullet grammar, so structural additions — heading, blanks on a brand-new file — never report as captured items). The working-tree diff is why the per-task summary (step 7, which fires before the task's commit) sees the run's uncommitted captures and sibling edits; on a clean tree the result equals the documented `git diff <first-commit>..HEAD -- {specs-root}/` form (step 12). Read-only; both lists empty is the no-impact domain outcome.

Resolved fork (scenario open question): a separate primitive rather than a mode on `derive-boundary` — the two results share only the diff base (boundary globs versus sibling-paths + inbox lines), so they share the `first_commit_for_prefix` walk as a `pub(crate)` helper instead of a result-shape union. `/ductus:review`'s captured-issues section stays on `compute-review-scope`, whose window starts at the in-progress transition — review wants the current work window, not the feature's whole history. The base is the transition commit's **parent**: `base..HEAD` excludes the base's own changes, and the reopen flow commits `/ductus:amend`'s back-edge flip together with the work it authorises, so a base *at* the transition excluded the whole subject (scenario `review-base-includes-the-transition-commit`).

### `append-inbox` — append one bullet to the inbox

Args:

```json
{ "text": "security: token logged in plaintext — src/auth.rs (captured during 022)", "dedup-prefix": "security: token logged" }
```

Result:

```json
{ "path": "specs/inbox.md", "created": false, "deduped": false, "item-count": 4 }
```

Appends `- [ ] {text}` (the checkbox inbox form the inbox template and constitution §bug-handling document) atomically to `{specs-root}/inbox.md`, creating the file when missing (from `framework/templates/project/inbox.md` when that file exists on disk — the framework source repo — else a bare `# Inbox` heading). Bullet scanning (dedup and counting) is comment/fence-aware — a `-` line inside the template's `<!-- Rules: … -->` guidance is not an item. With `dedup-prefix` supplied, an existing bullet whose text starts with the prefix (checkbox bullets included) suppresses the write and the result reports `deduped: true`. `item-count` reports the total inbox bullets after the call (the pre-existing total on a `deduped` no-op). Embedded newlines in `text` are rejected as an operational error (structure injection), matching `append-task`'s single-line rule.

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

- **Referrers** are the `create`-strategy files the manifest never overwrites and no migration re-points: `CLAUDE.md`, `AGENTS.md`, `README.md`, `.githooks/pre-commit`, and the spec root's `system.md`. They are enumerated in code rather than derived from the **Shared Files** manifest because that manifest lives in `framework/bootstrap/ductus.md`, a this-repo artifact that is fetched into staging and never installed — so a manifest-derived list would be empty in the adopter checkout where the check does its work. The first four are spelled literally; `system.md` is resolved against `[paths] specs-root`, because a hardcoded `specs/system.md` would examine nothing on a project that renamed its root and report clean for the wrong reason.
- **`system.md` is examined but never repaired, and the asymmetry is deliberate.** `ductus-rename` step 3 re-points the constitution reference in `CLAUDE.md`, `AGENTS.md`, and `README.md` — the references the framework itself wrote. An adopter's `system.md` reference is theirs, so the framework reports it and leaves it alone. It was absent from this list until a live adopter bootstrap found a `system.md` pointing at a directory that no longer existed while the run completed clean: no migration step named the file and no check looked at it, so an adopter-authored reference to a framework path was invisible to both halves. Reporting is the whole of the fix — it is what stops a run looking clean over a reference already known to be broken.
- **Managed roots** are `.ductus/`, `.githooks/`, the configured spec root (read from config, not assumed), and the **historical** roots `.govern/`, `scripts/gen-`, and `scripts/lib/`. A broken link into the adopter's own `docs/` is their business; scoping the check is what keeps it from being noise.
- **The historical roots are the point, not completeness for its own sake.** The orphan a migration chain leaves is a reference to the path *before* the move, so it carries the *old* root — a current-roots-only list is blind to exactly the class this check exists to catch. Measured on the 048 AC10 adopter run: after the generators moved root `scripts/` → `.govern/scripts/` → `.ductus/scripts/`, the adopter's `AGENTS.md` still named `scripts/gen-spec-deps.sh`, the check reported clean, and the operator found it by reading the file. The pre-042 entries are prefixes (`scripts/gen-`, `scripts/lib/`) rather than the `scripts/` directory because the framework owned those and never the whole directory; a `scripts/` root would flag an adopter's own `scripts/build.sh`.
- **`matched-prefixes` bounds the claim by scope, as `examined` bounds it by subject.** A reference carrying none of the listed prefixes is not reported under `skipped`, because nothing recognized it as a reference at all — so without this field a clean result reads as *no orphans* when it means *no orphans among paths carrying one of these prefixes*. That gap is what let a real orphan sit under a clean verdict, and it is `QUAL-CLAIM-001` about the boundary of a claim rather than its subject.
- **`attribution` is the field that keeps the result honest.** `registry` means `framework/migrations.toml` was readable — the bootstrap call site — and a finding's `migration` names the entry whose `target_paths` covers the missing path, latest hop winning since that is the one that left the reference dangling. `watermark` means no registry was available — an adopter checkout — so `migration` is empty for an entirely different reason and `last-applied` is the only migration context there is. An empty `migration` under `registry` means *asked and no entry claims it*; under `watermark` it means *nothing to ask*. Rendering the two alike would assert an attribution nobody computed, which is `QUAL-CLAIM-001` inside a check written to enforce it.
- **A pattern is not a reference.** A candidate containing `*` or `NNN` is documentation naming a shape (`specs/NNN-*/spec.md`), and a path the project declares it *ships* to adopters is not local breakage — both exemptions were earned by running the primitive against ductus's own repo, where they produced thirteen findings and zero real defects.
- `examined` names the files the result describes and `skipped` names the ones it could not read, so empty `findings` reads as clean only when `skipped` is empty.

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

The deterministic half of `/{project}:specify`'s routing gate, which runs **before** `create-feature` writes anything — creating a spec is the action the two routing rules in `AGENTS.md` §Workflow exist to prevent, so a check after the scaffold is not a check. Three sources, in result precedence order:

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
{ "observations": [{ "text": "perf: the config file is re-read on every primitive call", "path": "runtime/src/schema/paths.rs" }] }
```

Result (the observations half only):

```json
{ "observations": 1, "observations-captured": 1 }
```

An **observation** is something the reviewer judged real that maps to no loaded rule, so it cannot be a finding. `path` is optional. Observations never enter `must-violations` / `should-violations` / `low-confidence`, never affect `blocking`, and never change the exit code; they render in their own `## Observations` report section, which sits after `## Captured issues` and before `## Skipped passes` and emits `*None.*` when empty like every other section. The report frontmatter is unchanged — no `observations:` key — so a run with no observations produces a byte-identical report to one written before this addendum.

**Recording is capture.** The same call appends one bullet per observation to `{specs-root}/inbox.md` in the form ``- [ ] {text} — `{path}` (captured during review of {feature})`` (the path clause dropped when absent), via the `append-inbox` primitive rather than a second append implementation, dedup-guarded on that whole rendered line so a re-run over an unchanged repo appends nothing. `observations` counts what the report rendered; `observations-captured` counts what was newly appended, so a caller can tell *nothing to capture* from *capture ran and everything was already there*. The inbox write happens **before** `review.md` is written and an I/O failure fails the whole call: a report whose section claims a capture that did not happen is the defect this write-through removes (QUAL-CLAIM-001), and the reverse order would reintroduce it one level down. Observation `text` and `path` carry `append-inbox`'s single-line rule, screened up front so a rejection touches no artifact.

`performReview`'s response carries a matching optional `observations` array, accumulated across passes exactly as `findings` is and filtered out of later passes' request payloads by the same rule; without that leg the section would render `*None.*` on every run whether or not the reviewer had any. Defined by `scenarios/review-observations-write-through.md`; requiring spec [017 — Derive, Don't Ask](../017-derive-dont-ask/spec.md) AC25.

### `remove-inbox-item` — remove one bullet from the inbox

Args:

```json
{ "item": "security: token logged in plaintext — src/auth.rs (captured during 022)" }
```

Result:

```json
{ "path": "specs/inbox.md", "removed": true, "remaining-count": 3 }
```

The complement of `append-inbox` and the deterministic surface behind `/ductus:groom`'s per-item removal (step 8). Removes the first bullet from `{specs-root}/inbox.md` whose text — after the `-` bullet marker and an optional `[ ]`/`[x]` checkbox are stripped via the shared bullet grammar — equals the trimmed `item`, writing atomically. Bullet scanning is comment/fence-aware (shared with `append-inbox`), so a `-` line inside an HTML comment is neither counted nor removable. A double blank left at the removal seam is collapsed and the file ends in a single newline. A no-match, or a missing inbox file, is a clean domain outcome (`removed: false`), never an operational error; `remaining-count` reports the bullets left after the operation. Embedded newlines in `item` are rejected (single-line rule).

### `check-command-flags` — flags a command documents but never surfaces

Args: none.

Result:

```json
{
  "findings": [
    {
      "command": "framework/commands/review.md",
      "flag": "--since",
      "reason": "Flags table documents --since but argument-hint omits it, so it is never surfaced"
    }
  ],
  "examined": ["framework/commands/amend.md", "framework/commands/review.md"],
  "with-flags-table": ["framework/commands/review.md"],
  "skipped": [],
  "commands-dir": "framework/commands",
  "guidance": ""
}
```

The runtime half of [020](../020-code-review/spec.md)'s `review-flag-parsing-is-specified`, invoked from `/audit` Family 30 (`scripts/audit/command-flag-hint-parity.sh`). `argument-hint` is the surface a host renders when it offers a command, so a flag absent from it is a flag the operator is never shown — the defect an adopter reported as `--since` "doesn't show as an option" while `review.md`'s Flags table listed eight entries and its hint named three. Measured against that state: 6 findings (`--security`, `--simplicity`, `--quality`, `--since`, `--waive`, `--reason`); 0 once the hint was corrected.

- **The subject is `framework/commands/*.md`, the sources.** Not the generated copies under a host's commands directory: a copy carries whatever its source carries, so checking both reports every finding twice and neither copy is the one to fix. It also settles where the check belongs — an adopter told their installed `review.md` disagrees with itself cannot act on it, because the file is regenerated from ductus and the repair is a ductus release. That is why this is `/audit` (maintainer, at the source) rather than `/{project}:analyze`, whose command-frontmatter family is deliberately frontmatter-only and reads the installed copies.
- **Only a `Flags` section's table rows count, and only each row's first cell.** The behavior column routinely cross-references other flags (`Composes with all other flags`), and harvesting it would manufacture a finding against whichever row mentioned one. A single row may still name two flags — `--waive <rule-id> --reason "<text>"` is one row and both halves of a pair — so the cell is scanned rather than matched once.
- **A command documenting flags in prose is examined and contributes nothing.** `implement.md` describes `--auto` under a `### Flags` heading with no table, and its hint names it — correct, and invisible to a table-shaped check. This is why `with-flags-table` exists alongside `examined`: an empty `findings` means *every tabled flag is surfaced*, never *every documented flag is surfaced*, and a caller quantifying a clean verdict states which one it means (`QUAL-CLAIM-001`).
- **`guidance` is set when the run examined command files and found no Flags table at all.** Two empty sets compare equal, so without it an extraction failure returns the payload of a clean run — the same reasoning `derive-boundary` records for an underivable git window. Empty otherwise, so its silence means "examined and current".
- **Section membership comes from the shared fence- and comment-aware scanner** (`section_line_indices`), not a second heading walk. These command bodies embed example output and artifact fragments; a table row inside a fence is an illustration, not the command's contract. Reusing the scanner is also what keeps the check out of the `awk`/`sed` markdown-parsing shape [§runtime-boundary](../../framework/constitution.md#runtime-boundary) principle 3 names — Family 30's script is a shell entry point over this primitive, not a shell reimplementation of it.

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

Eight families, mirroring `/ductus:analyze`'s markdown-only reference exactly (severity tiers included — the primitive mechanizes the documented policy): `artifact-completeness` (blocking — `plan.md`/`tasks.md` required at `planned`/`in-progress`/`done`; `data-model.md` never required), `task-consistency` (blocking, when `tasks.md` exists — strictly-increasing numbering, `Done when` presence), `scenario-consistency` (advisory — every `scenarios/*.md` has a referencing task, skipped for `done` specs and satisfied by §tasks-phase pruning evidence: zero task sections or non-contiguous numbering), `review-state-drift` (blocking — a `done` spec with `review.last-run` unset or `review.blocking: true`; a `done` spec with no `review:` block is grandfathered), `scenario-open-questions` (blocking at `done`, advisory otherwise), `link-adjacent-drift` (advisory — prose asserting an open state that its own sibling link's target contradicts), `criterion-path-existence` (advisory — a filesystem path named in a `done` spec's acceptance criterion that no longer resolves), and `criterion-labels` (advisory — a duplicate `AC{n}` within one spec, a `next-criterion` that no longer exceeds the body, and an unlabelled criterion in a spec that carries a counter). `--all` iteration stays with the caller. The command-frontmatter-completeness family stays in the markdown-only reference (it reads the host's command directory, which the runtime does not own).

`scenario-consistency`'s **referencing-task rule is canonical for every surface that asks "does a task reference this scenario?"** A task references a scenario when the scenario's **slug** appears in the task's heading, in any subtask's text, or in its `Done when` clause. The match is on the slug, not on the `scenarios/{slug}.md` path: the path form is what `append-task`'s default body emits, and the wider slug match is what tolerates a hand-written task naming the scenario without it. A second surface applying a narrower rule disagrees asymmetrically — `/{project}:amend`'s reconcile pass would offer a task for a scenario this family already considers mapped, producing the duplicate its dedup exists to prevent — so a surface restating the rule (as the markdown-only path must, per §runtime-host-integration) states *this* rule. `mapped_scenario_produces_no_finding` and `bare_slug_reference_satisfies_the_mapping` cover both authoring forms, so a narrowing on the runtime side fails a test rather than shipping.

`scenario-open-questions` reports when any `scenarios/*.md` carries an unresolved question, naming the count and the scenarios. At `done` that state contradicts the completion rule outright, so the finding is blocking and `/{project}:analyze --fix` reverts the spec `done → in-progress` with a non-silent notice, exactly as it does for review-state drift; before `done` the questions are real remaining work but not yet a defect, so the finding is advisory and `--fix` leaves the status alone. Unlike `review-state-drift` there is **no grandfather rule** — an absent `review:` block genuinely marks a spec as predating that feature, but an unresolved scenario question is a present-tense defect whenever it arrived, and exempting it would preserve the state the check exists to surface. The question list comes from `read-spec`'s collector, so this finding, the `check-review-gate` block, and the count surfaced to the user can never disagree ([046 — Scenario open-question visibility](../046-scenario-open-question-visibility/spec.md)).

`criterion-labels` is the **enforcement half** of the labelling pass above ([013](../013-text-first-artifacts/scenarios/criterion-identifiers.md)): assignment is `label-criteria`'s, enforcement has to be separate because a criterion typed by hand in an editor never touches a primitive. Its three invariants are checkable from the artifact alone with no git-history read — a duplicate label is ambiguous rather than resolvable by picking the first match; a counter at or below the body maximum means the next assignment would reissue a label a live criterion already carries; and an unlabelled criterion is a defect **only once the spec carries `next-criterion` at all**. That last gate is the field's presence, deliberately not a per-spec grandfather date: 013 defines an absent counter as "no labels assigned yet", and the corpus backfill is what makes the check universal rather than an exemption list. The family runs at every status — a label is an identifier, not a contract about the delivered system, so a `draft` is as wrong to duplicate one in as a `done` spec — and it contributes nothing to `skipped`, since its whole subject is the spec's own frontmatter and criteria list.

`skipped` records each target a family could not examine, as `{family, reason, path}` over the closed reason set `target-missing` / `target-unparseable` / `no-readable-state` / `root-absent` / `ships-to-adopter` / `artifact-unreadable` / `not-a-live-claim`. It exists because the two families added by [045 — Decision-state drift detection](../045-decision-state-drift-detection/spec.md) read *targets* — a link's destination, a criterion's path — and 045 forbids escalating an unreadable one into a finding. Without the list, a family that examined every target and found nothing would return exactly what a family that could examine nothing returns, which is the shape `QUAL-CLAIM-001` forbids. `clean` is unchanged and still means `findings.is_empty()`, so the assurance lives in the pair: `clean: true` with an empty `skipped` is verified-clean, `clean: true` with a non-empty `skipped` is partially examined. The five families predating 045, and `criterion-labels` after them, always return it empty — their subjects are fully examinable by construction — and hosts render its entries in the Informational tier, where the cross-service reference unknowns already sit. `target-unparseable` has a second producer beyond an unreadable file: a sibling whose path traverses a symlink at or below the feature directory is reported unexaminable rather than followed, which is how `link-adjacent-drift` closes the half of its trust boundary lexical resolution cannot see without giving up repeat-run determinism ([sibling-symlink-trust-boundary](scenarios/sibling-symlink-trust-boundary.md)).

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
  primitives. It was scoped to `derive-dependencies` until the
  `derive-references-unstaged-drift-is-reported` scenario: a `references:` entry
  derives from the body *and* the `[services]` registry, so a service rename
  drifts specs nobody edited — and an unedited spec is never staged. Narrowing
  the reference walk to the staged set made that class structurally invisible.
- `absent` — specs tracked in the git index but missing from the worktree
  (deleted without staging the deletion). Nothing can be derived from one, so it
  is named rather than dropped, and it is excluded from `examined`. Added
  2026-08-27 after 022's own review flagged the count as a `QUAL-CLAIM-001`
  instance: `examined` was the enumeration size, which asserted a subject the
  run had not read. Present on both derive primitives.
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

The three initial-release single-shot extension points, plus the follow-on points: `askClarifyQuestion` and `routeInboxItem`, whose typed shapes ship ahead of their scenarios per the extension-request-hygiene scenario, and `verifyCriteria`, which ships with the implement-completion-gate scenario as `/ductus:implement`'s criterion-verification seam. Each has request and response payload schemas; the runtime validates incoming responses against these and emits `error: schema-mismatch` on failure. An extension identifier outside this closed set is an `error: unknown-extension` at request-build time — never a raw walker-context dump. In every request that carries legacy-compat context fields after its typed prefix (`writeCode`, `writeSpecBody`, `performReview`), walker-internal accumulator keys (prior `llm:*` response echoes and the accumulated `findings` array) are filtered out; primitive results threaded through the context (`scope`, `diff-base`, `selected`, `rules-dir`, `notices`, …) pass through.

### `assessSpecQuality`

Used by `/ductus:analyze`'s per-rule Verification reads.

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
  "task": {
    "number": "3",
    "heading": "Implement read-spec primitive",
    "subtasks": ["..."]
  },
  "plan-relevant-files": [
    { "path": "runtime/src/primitives/read_spec.rs", "content": "..." },
    { "path": "runtime/src/schema/spec.rs", "content": "..." }
  ],
  "write-boundary": [
    "runtime/**",
    "specs/022-deterministic-runtime/**"
  ],
  "constitution-excerpts": ["..."]
}
```

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

**Exec-path scope note** (scenario [coverage-residue-cleanup](scenarios/coverage-residue-cleanup.md)): clarify steps 7–8 — edge-case enumeration and acceptance-criterion verification — carry no extension marker, so `ductus exec clarify` no-ops them by design. They do not fold into this point's one-question-per-round-trip ABI: they are spec-wide passes that must run even on the zero-questions short-circuit, when this loop performs no round trips at all. The markdown-only path and a host walking the command file directly perform them in full; a host driving exec performs them itself before accepting the status-advance gate. The reduction is documented in the command (`clarify.md`, Instructions preamble), keeping the two-paths guarantee honest rather than silently narrower; a dedicated spec-review extension point remains future work if exec-driven clarify becomes hot.

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
