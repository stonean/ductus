---
status: in-progress
dependencies: [021-runtime-boundary]
next-criterion: 33
---

# 022 — Deterministic Runtime

The runtime is the deterministic execution layer for ductus. It interprets slash command procedures, runs the mechanical work each command requires, and invokes the LLM only at named extension points where semantic judgment is genuinely required. Slash command markdown files remain the source of truth and stay LLM-readable; the runtime is an accelerator, not a replacement.

Spec [021-runtime-boundary](../021-runtime-boundary/spec.md) made the runtime constitutionally admissible under [§runtime-boundary](../../framework/constitution.md#runtime-boundary). This spec defines what the runtime actually is.

> **Where the audit records live, and where findings go.** Since [057 — Analyze artifact and record relocation](../057-analyze-artifact-and-record-relocation/spec.md),
> `/{project}:review`'s record is `review.md`'s own frontmatter and
> `/{project}:analyze`'s is `analysis.md`'s; `spec.md` carries neither, and a
> residual block there is a Blocking validation finding. Since
> [058 — Findings route at discovery](../058-findings-route-at-discovery/spec.md), no run writes to the inbox: each finding is fixed,
> routed or discarded in the run that found it, and both records carry a
> `dispositions:` map and a `decisions:` list. The primitives that write and read
> both records are specified in `066-audit-records-and-the-completion-gate`, and
> the analyze and audit checks in `067-analyze-and-audit-checks`.

## Motivation

A `/ductus:analyze` invocation against a single feature takes 2–4 minutes of LLM wall-clock time today. `/ductus:implement`'s per-task gate cycle is the same order of magnitude. Most of that time is spent on work with no semantic content: parsing frontmatter, walking checkbox lists, verifying acceptance criteria one by one, running stuck-detection by reading git log output, deriving the runtime write boundary, checking generator drift via prose instructions that invoke bash scripts. The LLM is doing mechanical orchestration in slow tokens-per-second instead of fast bytes-per-second.

The deterministic share per command is large:

- `/ductus:target`, `/ductus:status` — 100% deterministic.
- `/ductus:analyze` — 11 of 13 check sections are mechanical; only per-rule Verification reads require semantic judgment.
- `/ductus:implement` — every gate, every checkbox update, every boundary derivation, every status transition is deterministic. The LLM's actual contribution is *writing the code for each task* — that work justifies its cost, but the deterministic scaffold around it is pure overhead.
- `/ductus:plan`, `/ductus:specify` — file creation, status advancement, template copying is deterministic. Filling in technical decisions and spec body is semantic.
- `/ductus:clarify`, `/ductus:review`, `/ductus:groom` — predominantly semantic, but each carries a deterministic scaffold (read spec, identify open questions, advance status when none remain) currently walked by the LLM.

Across a pipeline cycle, the wall-clock bulk is mechanical work the runtime can absorb. The savings compound: faster per invocation, more invocations per session, lower token cost in aggregate, and — most importantly — interactive instead of "go get coffee while ductus runs."

The reliability story from earlier drafts is preserved as a secondary benefit: deterministic execution of deterministic procedures eliminates the probabilistic-miss class of failures that LLM-walked mechanical work occasionally produces. But the headline is wall-clock and cost.

## Architecture

Three pieces, each independently testable.

### Parseable slash command procedures

Slash command markdown files in `framework/commands/*.md` are rewritten so the existing Instructions section IS the procedure — no separate Procedure block, no twin maintenance. The runtime parses the prose directly, using these conventions:

- **Numbered steps** (`1.`, `2.`, ...) are procedure steps. Sub-numbering (`1.1`, `1.2`) marks sub-steps.
- **Backtick-quoted primitive names** inside a step (e.g., `` `read-spec` ``, `` `mark-task` ``, `` `derive-boundary` ``) are primitive calls. The full primitive set is enumerated in [The primitive library](#the-primitive-library) below.
- **HTML-comment extension-point markers** (e.g., `<!-- llm:writeCode -->`, `<!-- llm:askClarifyQuestion -->`) mark the seams where the runtime invokes the LLM. The named identifier matches an entry in [LLM extension points](#llm-extension-points).

A markdown-only adopter (no runtime on `PATH`) reads the prose as today — the conventions are unobtrusive and the prose stays human-readable. The runtime reads the same prose and extracts structure. There is no second source of truth to drift from.

A **parseability check** runs in CI: every slash command's Instructions section must parse cleanly under these conventions, and a parse failure fails the build. It catches the only relevant kind of drift (a prose convention slipped) at PR time, as `scripts/lint-procedure-parseability.sh` in `.github/workflows/framework-checks.yml`.

### The interpreter

The interpreter is the runtime's procedure-execution engine, invoked by an agent host as a subprocess. Given a slash command name and arguments, it:

1. Parses the command's Instructions section under the structural conventions described above.
2. Walks the steps in order.
3. For each primitive step, calls the corresponding primitive operation in-process.
4. For each LLM extension point, emits a JSON request message to stdout, suspends, and resumes when the agent host writes a JSON response message to stdin.
5. Manages pipeline state — session file, status transitions, gate confirmations, checkbox updates — through primitives.
6. Returns when the procedure completes, halts at a gate, errors, or is cancelled.

The interpreter is stateful within a single subprocess run (in-memory only) and stateless across runs. Every invocation reads state from the markdown source of truth and the session file; nothing persists in the runtime between invocations.

### The primitive library

Primitives are the deterministic operations the interpreter offers procedures. Each primitive has a CLI subcommand (invokable standalone) and an MCP tool (callable by any agent host). The initial primitive set:

- `read-spec` — parse spec frontmatter and body sections.
- `read-tasks` — parse tasks.md into a structured task list with checkboxes.
- `mark-task` / `mark-criterion` — flip checkbox state in a markdown file.
- `set-status` — update spec frontmatter status field.
- `derive-boundary` — compute the runtime write boundary from git diff against the spec dir's first commit.
- `check-stuck` — count commits on tasks.md since status entered `in-progress` and surface cycles.
- `validate-frontmatter` — full frontmatter schema check.
- `resolve-anchor` — verify every `§<anchor>` reference resolves to a `<!-- §anchor -->` marker.
- `traverse-deps` — verify spec dependencies exist as directories and have compatible status.
- `check-rule-ids` — verify cited rule IDs exist in loaded rule files and aren't deprecated.
- `run-generator` — invoke a bash generator in `--dry-run` mode and report drift.
- `lint-markdown` — wrap `npx markdownlint-cli2`.
- `gate-confirm` — surface a pipeline gate to the user through the agent host and await confirmation.

The audit-record and completion-gate primitives are specified in `066-audit-records-and-the-completion-gate`, and the analyze and audit checks — `validate-frontmatter`, `resolve-anchor`, `traverse-deps` and `check-rule-ids` among them — in `067-analyze-and-audit-checks`.

Each primitive is deterministic, independently testable, and has a markdown-only fallback (either a bash script or a prose instruction the LLM can execute).

## Surfaces

The runtime binary exposes two interfaces. Agent hosts pick whichever they integrate with; both are first-class.

### MCP server

Invoked as `runtime mcp`. Exposes every primitive in [The primitive library](#the-primitive-library) as an MCP tool. Any MCP-capable agent host (Claude Code, Auggie, future hosts) connects without per-host integration code. The LLM walks the slash command prose as the orchestrator and calls MCP primitives for each operation that has one. Wall-clock improvement is moderate — each primitive call is microseconds instead of LLM-cognitive-seconds — but the LLM's decide-which-primitive-and-read-the-result orchestration loop remains.

This surface satisfies §runtime-boundary principle 5 ("MCP is the seam"). It is the universal-compatibility face of the runtime.

### Subprocess interpreter

Invoked as `runtime exec <command> [args...]`. The agent host spawns the runtime as a subprocess and communicates via newline-delimited JSON over stdio:

- `{"type":"llm-request","extension-point":"<name>","request":{...}}` — runtime suspends; awaits an LLM-driven response from the host.
- `{"type":"llm-response","response":{...}}` — host returns the LLM's structured output; runtime resumes.
- `{"type":"gate-confirm","gate":"<name>","prompt":"..."}` — runtime suspends; awaits user confirmation routed through the host.
- `{"type":"gate-response","confirmed":<bool>}` — host returns the user's decision.
- `{"type":"progress","message":"..."}` — informational, non-blocking.
- `{"type":"complete","result":{...}}` — procedure finished; runtime exits 0.
- `{"type":"error","code":"...","message":"..."}` — procedure halted; runtime exits non-zero.

Stderr is reserved for human-readable logs.

This surface drives the entire procedure end-to-end from the runtime, bypassing the LLM's orchestration loop. Wall-clock improvement is dramatic — minutes to seconds for `/ductus:analyze` — because the LLM is invoked only at named extension points for genuine semantic work. Adopting this surface is opt-in per-host: each integrating host implements the JSON message protocol (~few hundred lines of code). Claude Code is the first integrator; the protocol is documented and stable for others.

### Surface choice by adopter

- **Before the MCP server is registered**: neither surface. Since `048-govern-acquired-runtime` every bootstrapped project has the runtime, acquired into a ductus-owned store rather than found on `PATH`, so a host walks the prose only between acquisition and the restart that registers the server.
- **Auggie or other MCP-only host**: uses the MCP surface. Fast primitives, LLM orchestrates. No host-side integration code.
- **Claude Code or other JSON-protocol-integrated host**: uses the subprocess interpreter surface for slash commands. May also call individual primitives directly via MCP when convenient.

## LLM extension points

An extension point is a named seam in a slash command's Instructions section where semantic work happens. Each extension point has:

- A unique identifier (e.g., `writeCode`, `writeSpecBody`, `askClarifyQuestion`) declared in an HTML-comment marker on the relevant step.
- A schema for the request payload prepared at the seam.
- A schema for the response payload validated at the seam.
- Surrounding prose in the Instructions section describing what should happen — this is what the LLM reads in the markdown-only and MCP-server cases.

How extension points are invoked depends on the surface:

- **Subprocess interpreter**: the runtime emits a JSON `llm-request` message to stdout and suspends. The agent host calls the LLM with the structured request, validates the response against the schema, and writes a `llm-response` message to stdin. The runtime resumes.
- **MCP server**: the LLM is already the orchestrator and walks the surrounding prose itself. The HTML-comment marker is advisory — it tells the LLM "this step is the semantic part, do it yourself, don't try to find a primitive." Surrounding prose still names primitives that may be useful before or after the extension point.
- **Markdown-only**: same as MCP-server — LLM reads the marker as a hint and follows the prose instructions for the semantic work.

The extension point inventory across the initial release and follow-on scenarios:

**Shipped in the initial release** (single-shot — one `llm-request` / `llm-response` round-trip):

- `assessSpecQuality` — `/ductus:analyze`'s per-rule Verification reads. Request: spec content + a rule's Verification phrase. Response: pass/fail + finding text.
- `writeCode` — `/ductus:implement` walk-through-tasks step 4. Request: task description, plan-relevant files, write boundary. Response: list of file edits.
- `writeSpecBody` — `/ductus:specify` and `/ductus:plan` template-fill moments. Request: template + feature description + section name. Response: filled-in section content.

> **Signpost:** under `ductus exec`, `/ductus:analyze` asks `assessSpecQuality` about each loaded rule once, one request per rule, in the tier the rule's own Statement carries, and the host judges whether each rule's trigger fires. That walk, and the `rule-file-unreadable` reason it records beside `rule-assessments-not-checked`, is [060 — Exec analyze assesses each loaded rule](../060-exec-analyze-assesses-each-loaded-rule/spec.md)'s; this spec's data model states the request contract.

**Cache-breakpoint contract.** The `writeCode` request serializes its fields in this order: `constitution-excerpts`, `constitution-excerpts-unexaminable` (present only when the load could not examine something — see `data-model.md`), `plan-relevant-files`, `write-boundary`, `task`. The first three are the stable prefix that does not vary between tasks in the same `/ductus:implement` walk; the suffix (`task`) is the per-task variable. Hosts that implement prompt caching SHOULD place a cache anchor immediately after `write-boundary` and before `task` so the prefix's token cost is paid once per walk rather than once per task. The contract is SHOULD, not MUST — hosts without prompt caching still produce correct results, just at higher token cost. Independent host integrations (Claude Code, Auggie, third-party) converge on the same anchor position by following this contract.

**Added by follow-on scenarios on this spec** (the data model carries every extension point's schema, including later ones):

- `askClarifyQuestion` — `/ductus:clarify`'s open-question loop: multi-turn and user-mediated, the agent host showing the runtime-prepared question to the user and returning the answer.
- `performReview` — `/ductus:review`'s semantic passes.
- `routeInboxItem` — `/ductus:groom`'s routing decision.
- `verifyCriteria` — `/ductus:implement`'s criterion verification at the completion gate.
- `routeFold` — `/ductus:fold`'s body-edit-or-scenario decision.

> **Signpost:** `routeFold`, and the branch-scoped feature-directory form the shared `parse_feature_dir` recognizes, came from [051 — Branch-scoped spec numbering](../051-branch-scoped-spec-numbering/spec.md).

## Markdown-only path

The LLM walks a slash command's prose Instructions when no runtime surface is available, which since `048-govern-acquired-runtime` means only between acquiring the runtime and the restart that registers its MCP server. The structural conventions (numbered steps, backtick-quoted primitive names, HTML-comment extension-point markers) stay unobtrusive to a human or LLM reader: primitives read as named operations to perform, and extension points as labelled markers. The bash scripts under `scripts/` remain authoritative for the primitives that have bash counterparts (the `gen-*.sh` generators `run-generator` invokes, and `lint-frontmatter.sh`). The markdown-only reference sections stay because they specify the policy the primitives implement, and the parseability check holds the conventions in place.

## Slash command rewiring

This spec rewrites slash command markdown files in `framework/commands/*.md` so each Instructions section follows the parseable conventions. The initial release covers six commands; the remaining three ship as scenarios on this spec.

### Initial release — six commands

The pipeline backbone:

1. `/ductus:status` — 100% deterministic; smallest surface; validates the architecture.
2. `/ductus:target` — 100% deterministic; exercises session-file write path and constitution loading.
3. `/ductus:analyze` — headline wall-clock win. Extension point: `assessSpecQuality` (per-rule Verification reads).
4. `/ductus:implement` — largest behavioral surface. Extension point: `writeCode` (per-task work).
5. `/ductus:plan` — template-driven plan generation. Extension point: `writeSpecBody` (shared with `/ductus:specify`).
6. `/ductus:specify` — new feature scaffolding. Extension point: `writeSpecBody`.

Three extension points are designed and shipped in the initial release: `assessSpecQuality`, `writeCode`, `writeSpecBody`. All are single-shot (one `llm-request` / `llm-response` round-trip per invocation) — no multi-turn user-mediated interaction.

### Follow-on scenarios

Each subsequent command lands as a scenario on this spec, in this order:

1. **Scenario: `/ductus:clarify`** — first follow-on. Introduces the `askClarifyQuestion` extension point, which is multi-turn and user-mediated (the agent host shows the question, awaits the user's response, returns it to the runtime). The trickier ABI ships after the single-shot pattern proves out in the initial release.
2. **Scenario: `/ductus:review`** — introduces `performReview`. Runtime's value-add is small (predominantly LLM work) but ships for pipeline completeness.
3. **Scenario: `/ductus:groom`** — introduces `routeInboxItem`. Similar to review: runtime accelerates only the bookkeeping around the inbox walk; routing decisions are LLM-driven.

### Per-rewrite checklist

Each command's rewrite:

- Preserves the command's existing behavior — same primitives invoked, same gates, same order.
- Adopts strict numbered-step format with sub-numbering for sub-steps.
- Quotes every primitive call in backticks, using a name from [The primitive library](#the-primitive-library).
- Marks every LLM seam with an HTML-comment extension-point marker, using a name from [LLM extension points](#llm-extension-points).
- Passes the parseability check after the edit.

## Install policy

Superseded by `048-govern-acquired-runtime`: the runtime is required, and `/ductus` acquires the pinned release into a ductus-owned store during adoption. One rule of the opt-in policy this section first stated still holds: no slash command detects a missing runtime and prints an install nag (AC17).

## CI integration

`.github/workflows/runtime.yml` builds the binary, runs its test suite and exercises every primitive against fixture inputs whenever runtime source paths change. The parseability check runs in `.github/workflows/framework-checks.yml` on every PR. The `markdown-only-pipeline.yml` workflow that proved the runtime-absent path was removed by `048-govern-acquired-runtime`, which made the runtime required.

## Bash script relationships

Stable relationships post-rewrite:

- **The two frontmatter derivations are primitives, not scripts.** `dependencies:` and `references:` are derived by `derive-dependencies` and `derive-references`, which replaced `.ductus/scripts/gen-spec-deps.sh`, `gen-cross-service-refs.sh` and their shared `lib/specs-root.sh` (067's `adopter-generator-promotion`). "No LLM in the loop" never disqualified them: §runtime-boundary's eligibility criteria require **determinism**, and criterion 2(b) names "a bash script the framework invokes (pre-commit hooks, generators, CI)" as *currently mechanical*, which is the eligible case. What kept them in bash was principle 3's rule that the runtime must not be a prerequisite for a pipeline gate, which `048-govern-acquired-runtime` retired. (Named, not linked: an inline link would add a dependency edge 022 → 048, and 048 already depends transitively on 022.)
- **`run-generator`** stays, and its scope narrows to what it was always for: invoking a *project-owned* generator from a procedure — in this repository, `/{project}:analyze` runs `scripts/gen-help-tables.sh` through it — with `--dry-run` hardcoded so a walker dispatch can never write. ductus's own two derivations are no longer among its callers.
- **`lint-frontmatter.sh`** — repositioned as the markdown-only fallback for the runtime's `validate-frontmatter` primitive. Same intent, two implementations, both ship; the prose Instructions invoke whichever is available.
- **`lint-tool-coverage.sh`** — stays bash-only, with no runtime counterpart. It runs in `.github/workflows/framework-checks.yml` and in `scripts/audit/check-zero.sh`, where a runtime counterpart would buy nothing.

## Acceptance Criteria

- [x] AC1: A single binary builds from this repo and exposes two surfaces: `runtime mcp` (MCP server) and `runtime exec <command> [args...]` (subprocess interpreter).
- [x] AC2: Every primitive in [The primitive library](#the-primitive-library) is exposed as an MCP tool by `runtime mcp`, under a bare `<verb>-<noun>` name that each host prefixes with its server name, as §runtime-host-integration states.
- [x] AC3: The subprocess interpreter's JSON-over-stdio message protocol is documented in the runtime's docs and stable enough for third-party agent hosts to integrate against.
- [x] AC4: The six initial-release slash commands (`/ductus:status`, `/ductus:target`, `/ductus:analyze`, `/ductus:implement`, `/ductus:plan`, `/ductus:specify`) have their prose Instructions sections rewritten to follow the structural conventions and parse cleanly under the runtime parser. The remaining three (`/ductus:clarify`, `/ductus:review`, `/ductus:groom`) are not rewritten in this spec — they ship as scenarios.
- [x] AC5: The three initial-release LLM extension points (`assessSpecQuality`, `writeCode`, `writeSpecBody`) each have a request schema, response schema, and a corresponding HTML-comment marker in the relevant Instructions step.
- [x] AC6: A parseability check is added to `.github/workflows/markdown-only-pipeline.yml` and passes against every slash command file — the six rewritten ones under the new conventions, and the three not-yet-rewritten ones under the legacy prose-walk path (the parser tolerates files that do not yet declare a Procedure-shaped Instructions section). Delivered as written; that workflow is renamed to `.github/workflows/framework-checks.yml` by `048-govern-acquired-runtime`, which carried the parseability check across unchanged.
- [x] AC7: The binary executes each of the six initial-release commands end-to-end against a fixture repo and produces output consistent with the LLM-driven path against the same fixture, within the determinism bounds defined for each command.
- [x] AC8: Median wall-clock time per invocation drops from minutes to seconds for each of the six initial-release commands when the runtime is present.
- [x] AC9: The markdown-only path (no binary on `PATH`) continues to complete every pipeline cycle (greenfield, brownfield, reopen) as it did before this spec. Delivered as written; the guarantee is removed by `048-govern-acquired-runtime`, which made the runtime a requirement rather than an option, so the no-binary cycle this criterion asserts is since retired.
- [x] AC10: `framework/runtime-tools.txt` is populated with every MCP tool name the binary exposes; `scripts/lint-tool-coverage.sh` passes against the rewritten slash command files.
- [x] AC11: A new CI workflow at `.github/workflows/runtime.yml` builds the binary, runs its test suite, exercises every primitive against fixture inputs, and fails on any test failure.
- [x] AC12: The existing `.github/workflows/markdown-only-pipeline.yml` workflow continues to pass with the runtime binary absent from `PATH`. Delivered as written; that workflow no longer exists — `048-govern-acquired-runtime` replaced it with the end-to-end acquisition workflow, so the invariant this criterion asserts is removed.
- [x] AC13: State-modifying primitives (`mark-task`, `mark-criterion`, `set-status`) use filesystem-atomic writes (write to temp + atomic rename) so a runtime crash mid-write leaves coherent markdown.
- [x] AC14: The runtime self-reports its build-time version via `runtime --version`; parse failures emit a descriptive `error` JSON message that includes the runtime version and notes version-mismatch as a possible cause.
- [x] AC15: The binary is distributed via GitHub release artifacts cross-compiled for `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu` (and `x86_64-pc-windows-msvc` if cross-compilation is friction-free in CI), with sha256 checksums; adopter-facing install instructions exist in a README Runtime section.
- [x] AC16: The `/ductus` bootstrap command's completion output includes a one-line pointer to the README Runtime section, mentioning the runtime is optional. Delivered as written; the pointer remains, but the optionality it announces is removed by `048-govern-acquired-runtime`, which made the runtime required.
- [x] AC17: No slash command in `framework/commands/*.md` detects the missing runtime and prints an install nag on invocation; the markdown-only path remains a first-class path per §runtime-boundary principle 3. Delivered as written, and the no-nag half still holds; principle 3's first-class markdown-only guarantee is since retired by `048-govern-acquired-runtime`.
- [x] AC18: `/ductus:analyze` against this spec passes with no hard-fail or blocking findings.
- [x] AC19: `npx markdownlint-cli2` against all rewritten slash command files and new spec files passes.
- [x] AC20: `apply-manifest` rejects a substitution key that is placeholder-shaped (`{project}`) or empty, before any filesystem operation, so a malformed map halts the walk with zero writes rather than writing a tree in which every placeholder survived literally. The rejection is exact — it refuses only keys incapable of matching a placeholder — so a legal bare key carrying spaces and punctuation still substitutes.
- [x] AC21: `apply-manifest` reports the substitution count it previously computed and discarded: per entry as an optional value that is absent rather than zero when substitution never ran, and in aggregate alongside `entries-substituted`, so a total is never read without its denominator. The bare-key contract is stated in the primitive's module docs, the `substitutions` schema doc comment, and `framework/bootstrap/ductus.md` §Placeholder Substitution (and its `govern.md` twin), and both counts are surfaced in `framework/bootstrap/ductus-procedure.md`'s §Post-Scaffolding Output.
- [x] AC28: Every name in `PRIMITIVE_REGISTRY` resolves to a clap subcommand and every registry-backed subcommand is in the registry, asserted by **set-equality** in `main.rs` so both a missing variant and a phantom verb fail with the divergence named. `exec`, `mcp`, `parse` and clap's synthesised `help` are excluded by name with each reason stated, never by loosening the assertion to containment. The prose claiming `main.rs` is the one unpinned surface is corrected wherever it stands — `AGENTS.md` and `runtime/src/schema/registry.rs` — and the manual `cargo run -- <name> --help` step it prescribed is discharged.
- [x] AC29: The config-sourced path trust boundary is stated on `validate_no_traversal`, where the code that relies on it can be read against it: a primitive reading a path from committed config may skip the check only while every writer of that table records a path a human chose, and a writer that *originates* the value moves the table into the validated tier in the same change. `resolve_constitutions::classify` and `resolve_references::classify` cite that statement rather than restating it, each naming what it obliges of its own table. The statement says plainly that nothing enforces it.
- [x] AC31: `write-review` and `diff-cross-spec` report `inbox-standing` — the **standing** backlog, distinct from the window each already reports — so `/{project}:review` and `/{project}:implement`'s completion summary render an `inbox` row on every run: `clean`, `outstanding` with the oldest item's `git blame` date, `age undeterminable` when blame cannot run, and `no-file` as its own state. The row is never omitted and never gates. **Superseded by 058**: `write-review` and `diff-cross-spec` no longer report `inbox-standing`, and the row left the review and implement output. `dashboard` reports it, and `/{project}:status` renders it on every run in the same four states, still never gating.

## Non-Goals

- Replacing the LLM at semantic extension points — that work is by definition outside the runtime.
- Replacing the slash command markdown files with a different source format. Markdown stays the source of truth; structural conventions are layered into the existing prose, not extracted to a separate file or format.
- Rewriting ductus's own generator scripts under `scripts/` (`gen-*.sh`) as primitives; this repository's pre-commit hook calls them directly, and `run-generator` only invokes one with `--dry-run` from a procedure. The two frontmatter generators adopters ran are the exception, promoted to primitives for the reason [Bash script relationships](#bash-script-relationships) records.
- Persisting any state outside the markdown source of truth and the session file — runtime state is derived and gitignored per principle 1.
- Auto-installing the runtime on `/ductus` adoption — opt-in per principle 3. **Reversed by `048-govern-acquired-runtime`**, under which `/ductus` acquires the runtime as part of adoption.
- Daemon mode, long-running services, or background processes — per the non-scope list in §runtime-boundary.
- Reimplementing `npx markdownlint-cli2` natively — the runtime wraps it as a primitive.
- Building any non-MCP integration surface (LSP, web UI, REST). Speculative; would need its own spec under §runtime-boundary's eligibility criteria.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Partial-failure semantics** — every primitive distinguishes two outcome categories. (1) Domain outcomes: the primitive completed; result reports success/failure of the check it performed (e.g., `validate-frontmatter` returns "3 issues" or "clean"). These are findings, not errors — procedure continues, findings accumulate. (2) Operational errors: the primitive could not complete (file unreadable, git non-zero, parse failure, OS error). Default: halt the procedure, emit a structured `error` JSON message to the host, exit non-zero. Per-primitive overrides: `mark-task` / `mark-criterion` / `set-status` rollback partial writes before halting (atomicity); `gate-confirm` denial is a clean `complete` outcome, not an error; `run-generator` non-zero exit is a drift finding, not an operational error. The runtime does NOT ask the LLM what to do on error (blurs determinism boundary), does NOT auto-fall-back to prose for the remainder (state-recovery problem), and does NOT continue past operational errors (risks cascading damage). The companion question for when the runtime *process itself* crashes is *Runtime-failure fallback semantics* below.
- **Per-command opt-out** — none. The only mechanisms are the agent host's MCP client configuration and the `[runtime] path` setting: no env var, config section, CLI flag or per-command annotation. A configuration surface is permanent technical debt, and no concrete case asks to trust the runtime for one command but not another. If one appears, a future spec or scenario adds the knob with a motivating example.
- **Distribution channels** — GitHub release assets, which `/ductus` acquires from: pre-built binaries for `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu` and `x86_64-pc-windows-msvc`, with sha256 checksums, built and published on tag, and every release also publishes the `ductus` crate to crates.io (`.github/workflows/runtime-release.yml`). Homebrew was struck, since maintaining a tap adds ongoing cost for marginal benefit; Linux distribution and Windows package managers are left to the community, and the binary does not update itself.
- **MCP tool naming convention** — kebab-case `<verb>-<noun>`, registered under bare names (`read-spec`, in `runtime/src/mcp/server.rs`) that each host prefixes with its own server name, as §runtime-host-integration lists. The verb-noun pattern forces clarity at the naming layer. The `ductus:` namespace prefix this question first chose is not how the server registers its tools, so AC2 states the bare convention.
- **Versioning enforcement** — rely on schema-evolution discipline plus clear parse errors. No `framework-version:` field added to the constitution; no startup version comparison; no warn-on-mismatch. The binary self-reports its build-time version via `runtime --version` (read from `Cargo.toml` at compile time). On parse failure the binary emits a descriptive `error` JSON message that includes the runtime version and points the adopter at version-mismatch as a possibility. §runtime-boundary's lockstep-versioning policy is the actual protection — adopters who follow tagged releases get matching framework + binary; the parse-error path covers the remaining mismatch cases. Rejected: warn-and-continue (warnings get ignored); explicit refuse-on-mismatch (premature infrastructure for a problem that hasn't materialized); silent best-effort (worst outcome — silently wrong results).
- **Runtime-failure fallback semantics** — surface and fall back. When the runtime *process itself* crashes (segfault, OOM, signal — distinct from a primitive returning a structured error, covered by *Partial-failure semantics* above): (1) host surfaces a user-visible notice including exit code, runtime version, and an issue-reporting URL; (2) host falls back to LLM-walked prose execution for the same command so the user's request still completes. The MCP surface case is already covered by existing fallback discipline (primitive fails → LLM follows the prose fallback marker → bash script or LLM execution). The runtime contributes: filesystem-atomic writes for state-modifying primitives (`mark-task`, `mark-criterion`, `set-status` — write to temp + atomic rename so crashes leave coherent markdown), clear exit-code signaling (0 = clean complete; 1-127 = clean operational error with terminal `error` message; 128+ = signal-killed crash with no terminal message), and version reporting. The runtime does NOT catch its own crashes, persist mid-run state for resume, or auto-retry. Host-side fallback implementation is the expected partner behavior, referenced in the spec but not a runtime AC (Claude Code and Auggie each implement their own integration). Rejected: silent fallback (hides bugs); surface-only halt (bad UX for recoverable cases).
- **Implementation language** — Rust. Reasons: (1) Anthropic's `rmcp` crate is the most mature MCP SDK and is the reference implementation; (2) smallest static binary and fastest cold-start, both of which matter per-invocation; (3) strongest type system for the procedure interpreter's state-machine work; (4) memory safety relevant for parsing user-supplied markdown and JSON; (5) it ships in the lineage of Rust CLI tools such as `ripgrep` and `fd`: a single static binary, fast cold-start, sensible exit codes. Go was the credible alternative (faster builds, easier cross-compilation, more accessible to newcomer contributors, adequate MCP libraries); rejected because for a foundational tool that's modified rarely and invoked frequently by many adopters, production characteristics outweigh development-velocity characteristics. Other languages (Node/Bun/Deno, Python, C++, Zig, Swift) rejected for distribution complexity, startup overhead, ecosystem fit, or maturity.

- **State management within a procedure run** — in-memory only. The markdown source of truth and the session targets (`.ductus/session.toml`, and each process's own under `.ductus/sessions/`, written by `write-session`) are the durable journal; primitives write to them synchronously (e.g., `mark-task` flips the checkbox in the file before moving to the next task). In-memory state holds working copies: position in the procedure, parsed file contents, pending extension-point payloads. Crash recovery is "user re-invokes the slash command; runtime reads markdown to observe what's already done and resumes from the next incomplete step." Observability is stderr (human-readable progress logs the host captures) plus `progress` JSON messages over stdout. Rejected: a transient run file (duplicates markdown's journal, adds cleanup, and risks drift between two state sources); a persistent log file (no out-of-band sink — the runtime's log is its output streams). Consistent with §runtime-boundary principle 1 (markdown is source of truth).
- **Initial scope** — the first release rewired six commands (`/ductus:status`, `/ductus:target`, `/ductus:analyze`, `/ductus:implement`, `/ductus:plan`, `/ductus:specify`) plus the full MCP server, covering the pipeline backbone and the headline wall-clock wins. `/ductus:clarify`, `/ductus:review` and `/ductus:groom` followed as scenarios: they add the least acceleration and carry the trickier multi-turn extension-point ABI, so the single-shot pattern proved out first.
- **Extension point ABI** — the runtime exposes two surfaces. (1) MCP server (`runtime mcp`) — every primitive as an MCP tool; the LLM walks slash command prose and calls primitives via MCP. Universal compatibility, zero per-host integration; used by Auggie and any other MCP-capable host. (2) Subprocess interpreter (`runtime exec <command>`) — the runtime drives the procedure end-to-end and uses JSON-over-stdio to invoke the LLM at extension points (`llm-request` / `llm-response` / `gate-confirm` / `gate-response` / `progress` / `complete` / `error` messages). Opt-in per host; Claude Code integrates first. The two surfaces share the same internal primitive implementations; agent hosts pick whichever fits their integration model. Rejected: MCP-with-continuation-tokens (server-side state across calls, protocol complexity, doesn't address the orchestration-tax problem cleanly); in-process library/callback (fragments adoption by language).
- **Install policy** — reversed by `048-govern-acquired-runtime`: the runtime is required, and `/ductus` acquires it. See [Install policy](#install-policy).
- **Procedure format and source** — prose IS the procedure. Slash command Instructions sections are rewritten to follow strict structural conventions (numbered steps, backtick-quoted primitive names, HTML-comment extension-point markers) that the runtime parses directly. No separate Procedure section, no twin maintenance, no drift-between-two-implementations to detect. Rejected: (a) YAML frontmatter (bloats metadata), (b) separate fenced code block (twin maintenance with a CI drift check is real cost for no benefit over a single source), (c) sibling `procedure.md` file (two files per command, drift risk, requires markdown-only adopters to know about both). Rejected source models: hand-authored separate Procedure section (twin maintenance), generated-from-structured-source with prose derived (breaks `framework/constitution.md` §text-first-artifacts principle 1). The cost is a one-time rewrite of every slash command file to match the conventions; the spec already commits to that rewrite. The CI parseability check on Instructions sections is the only drift-detection mechanism needed.
- **Prose-vs-procedure drift check** — superseded by *Procedure format and source*: prose is the single source, so the CI parseability check is the only drift check needed.
- **Scope reframing** — earlier drafts wrapped four `/ductus:analyze` checks; rejected. The cost to remove is LLM-mechanical orchestration across every command, so the runtime is the procedure interpreter, with primitives as its library.
- **Capability eligibility rule** — reversed. It held that work is runtime-eligible only when its calling context has an LLM fallback, which kept the pre-commit generators in bash; [Bash script relationships](#bash-script-relationships) records why that misread §runtime-boundary, whose criteria require determinism.
- **Generator dry-runs as a top-level capability dropped** — wrapping generators adds no reliability (bash is already deterministic) and saves no tokens. `run-generator` remains a thin wrapper a procedure can call.
