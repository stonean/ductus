# 022 — Deterministic Runtime Data Model

Defines the data structures the runtime owns: the parsed procedure AST, the JSON-over-stdio protocol envelope and message types, the primitive request/response schemas, and the extension-point schemas (the three initial-release points plus the follow-on request/response shapes). The shapes of the primitives that write a spec's audit records and gate its completion are `066-audit-records-and-the-completion-gate`'s, and those of the analyze and audit checks are `067-analyze-and-audit-checks`'s. These types live in `runtime/src/schema/` as Rust types with `serde::{Serialize, Deserialize}` derives; their serialized JSON shape is the stable contract for host integrators.

## Procedure AST

Produced by `runtime/src/parser/`. Internal to the runtime; not serialized to disk. JSON serialization exists only for `ductus parse <file>`'s debug output.

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

The parser rejects a malformed procedure with a hard `ParseError::Invalid` — an unrecognized primitive name, or a single step naming two or more *distinct* primitives (only one can dispatch, so a second would be silently dropped on the exec path). `ductus parse --check` reports these, and `scripts/lint-procedure-parseability.sh` fails CI on them (for every `framework/commands/*.md` and `framework/bootstrap/*.md` file). A file that has no procedure-shaped Instructions section returns `ParseError::LegacyProse`, tolerated only for the files on the parseability allowlist.

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

Each adopter installs into one layout; walking all three resolves any of them without knowing which agent wrote the file, and both resolution callsites (`main::run_exec`, `interpreter::payload::locate_command_file`) consume the list unchanged (scenario [`the-pi-command-candidate`](scenarios/the-pi-command-candidate.md)).

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

They are three types so that an analyze tier emitted into a review finding is unrepresentable. **An unrecognized value is rejected, never defaulted**, with a `schema-mismatch` naming the extension point, the vocabulary, the value and the legal set; the bucketing is a `match` with no catch-all arm. Matching is case-insensitive and serialization canonical lower-case (scenario `severity-is-a-closed-set-not-a-string`).

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

`scenario-open-questions` is a **sibling** signal, never merged into it: the unresolved questions carried by `scenarios/*.md`, each tagged with its slug, in the shared scenario order. They gate `done`, not `clarified` ([046 — Scenario open-question visibility](../046-scenario-open-question-visibility/spec.md); scenario `scenario-open-question-signal`).

`scenario-files-unreadable` lists the slugs of scenario files the shared collector could not read, **omitted when empty**, so an empty `scenario-open-questions` means every scenario was read (scenario `unreadable-scenario-is-reported`).

`label` is the criterion's stable `AC{n}` identifier, or `null` before the labelling pass reaches it ([013 — Text-First Artifacts](../013-text-first-artifacts/scenarios/criterion-identifiers.md)); `text` keeps the `AC{n}:` prefix as authored. The label is parsed by `label-criteria`'s `parse_label`, the function `mark-criterion` resolves a `label` through, so the two cannot disagree.

Both lists come from one comment- and fence-aware parser (scenario `scenario-question-parser-fix`), and scenarios are enumerated through the shared listing `check-artifacts` and `dashboard` use. An absent `scenarios/` directory, an unreadable scenario, or one with no questions section contributes nothing.

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

`criterion-index` (0-based, body order) is the fallback for a criterion the labelling pass has not reached; `label` is preferred because it survives insertion and reordering. Supplying both, or neither, is refused, and a `label` no criterion carries is an error naming it (scenario `criterion-label-assignment`).

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

`assigned` lists only the criteria this run labelled, with each one's 0-based index in body order; `next-criterion` is the counter as written back, always above every label in the body; `changed` says whether the file was written. Assignment takes **`max(highest label in body, next-criterion)`**, never `max(body) + 1`, so a deleted criterion's label is never reissued. A spec with no frontmatter fence, or a `next-criterion` that is not a positive integer, is refused rather than repaired; an absent or empty Acceptance Criteria section is a no-op that creates no counter. The write is atomic and the pass idempotent (scenario `criterion-label-assignment`).

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

The boundary is `git diff --name-only <first-commit-on-spec-dir>..HEAD` plus the spec dir, emitted as **directory-zone globs**: each changed path contributes `{dir}/**`, so a new file in a touched directory is admitted, and a root-level file stays an exact path. The exec walker merges it into `write-boundary` as a **union** with any session-seeded value; with neither, the first out-of-spec writeCode edit halts with `out-of-boundary-edit` (scenario [writecode-boundary-derivation](scenarios/writecode-boundary-derivation.md)).

### `check-stuck` — count tasks.md commits since `in-progress`

Args:

```json
{ "feature": "022-deterministic-runtime", "threshold": 10 }
```

Result:

```json
{ "commit-count": 3, "stuck": false, "since-sha": "<sha>", "threshold": 10 }
```

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

`rendered-markdown` is the full pipeline view as one markdown fragment, in `/ductus:status`'s documented order — preamble, table, counts and callouts, and the cross-service references readout, resolved the way `resolve-references` resolves them — with `/{project}:…` texts in the adopter's `[host] project` namespace. It is data the host may restyle, never printed; the structured fields stay authoritative, and `/ductus:status`'s Rendering reference holds the piece-by-piece formats.

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

The deterministic half of `/{project}:specify`'s routing gate, run **before** `create-feature` writes anything (scenario `specify-routes-before-scaffolding`; requiring spec [017 — Derive, Don't Ask](../017-derive-dont-ask/spec.md) AC26). Three sources, in result precedence order:

- **`runtime-work`** — the description names a primitive (from the parser's `PRIMITIVE_NAMES`) or a path under `runtime/`. The candidate is the lowest-numbered feature whose `plan.md` lists a `runtime/` path under `## Affected Files`, read through `compute_review_scope::read_plan_affected` — derived from the corpus, never a slug named in code.
- **`rule-surface`** — a rule file whose category stem the description shares, the `-backend` / `-frontend` / `-cross` suffix set aside; directories resolve as `discover-rule-files` resolves them.
- **`spec-corpus`** — a spec whose slug shares vocabulary with the description, unless `runtime-work` already offered it.

Matching is lexical (lowercased alphanumeric tokens of four or more characters, minus a short stopword list) and **advisory**: the decision stays at the `routeInboxItem` extension point and with the operator. `reopens` is true when accepting a candidate implies the `done → in-progress` back-edge; `route` is drawn from the `routeInboxItem` vocabulary, and `spec` never appears as a candidate. **Every source lands in `sources-examined` or in `skipped`, never both and never neither**, so empty `candidates` with empty `skipped` (*matched nothing*) and with a non-empty one (*could not derive*) stay distinct; the runtime-work signal with no derivable home is a skip. Only an empty `description` is an operational error. `routed-by` returns `gate-required: false` with nothing else, so a tree that already ran is not asked twice; `description` carries a `title` alias and is echoed in the result.

### `routeInboxItem` — `candidates` request field (addendum)

The request gains an optional `candidates` array carrying `derive-routing-candidates`' output, `skip_serializing_if` empty so the `/{project}:groom` payload is byte-unchanged. This keeps **one** routing point: `/{project}:specify` sends the proposed feature description as `item-text` with its derived candidates, so work arriving through conversation is routed by the same tree as work arriving through the inbox. Two routing rules that could disagree is the drift the scenario was written against. `item-text` falls back to the `description` key on the specify path, so neither entry point needs a second context key for the same value. The router treats candidates as evidence to weigh, never as a decision already made.

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

**Exec-path scope** (scenario [coverage-residue-cleanup](scenarios/coverage-residue-cleanup.md)): `ductus exec clarify` no-ops clarify steps 7–8, spec-wide passes with no extension marker that must run even when this loop makes no round trip; a host driving exec performs them itself, as `clarify.md`'s Instructions preamble states.

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
    { "index": 0, "text": "`ductus exec implement` walks the procedure to completion.", "checked": false },
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
