# ductus

**Spec-driven development for AI coding agents.** Describe a feature in plain English; your agent turns it into a spec, a plan, tasks, and reviewed code — and every feature lands with a written record of *why* it was built the way it is.

`ductus` is tech-stack agnostic, ships as plain markdown, and works with Claude Code, Auggie, Antigravity, and OpenCode. There's nothing to compile and no dependency to add — you install a single command into your project and drive the rest through a handful of verb-named slash commands.

## Why ductus

AI agents are fast, but left to their own devices they're inconsistent: they guess at ambiguous requirements, lose the reasoning behind a change as soon as the chat scrolls away, and reinvent structure on every task. `ductus` puts a thin, opinionated pipeline in front of the agent so that:

- **Ambiguity is caught upstream of code.** Open questions get resolved in the spec, not discovered halfway through implementation.
- **Every feature carries its "why."** The spec is a living document that stays accurate after the code ships — not a ticket that gets buried when it closes.
- **The surface area is small.** A few commands map to things you already do: write a ticket, surface unknowns, sketch an approach, build it, audit it.
- **Artifacts stay portable.** Everything is markdown with YAML frontmatter — readable in GitHub, Obsidian, or `cat`, with no proprietary format to escape.

## Quick start

Install `ductus` into any project:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/stonean/ductus/main/install.sh | sh
```

This installs the `/ductus` bootstrap command for Claude Code — see [Installing per agent](docs/slash-commands.md#installing-per-agent) to target Auggie, Antigravity, or OpenCode instead. Then, in your agent, run:

```text
/ductus my-project
```

That one command scaffolds the `specs/` directory, installs the full set of slash commands, wires up the constitution and agent rules, and prints your next steps. It's idempotent — safe to re-run any time to pull the latest `ductus` files.

Now build your first feature by walking it through the pipeline:

```text
/specify   add user login with email and password
/clarify              # resolve open questions the spec surfaced
/plan                 # technical decisions, affected files, tasks
/implement            # work the tasks; code gets written here
/review               # audit the code against the rules
```

Each command advances the feature one step and leaves a durable artifact behind. That's the whole loop.

## How it works

Every feature moves through one pipeline. The status on each spec tracks where it is:

```text
draft ──/clarify──▶ clarified ──/plan──▶ planned ──/implement──▶ in-progress ──/implement──▶ done
```

- **Spec** (`/specify`, `/clarify`) — define *what* the feature does and *why*, with concrete acceptance criteria and a list of open questions. No open questions may remain before planning.
- **Plan** (`/plan`) — turn the spec into technical decisions, affected files, and an ordered task list. Persistence-heavy features also get a data model.
- **Implement** (`/implement`) — work the tasks; this is where code is written. Status moves to `in-progress`, then `done` once the review gate passes.
- **Review** (`/review`) — audit the implementation against the framework's rules (security, reuse, quality, efficiency, simplicity). Blocking violations keep the feature out of `done` until they're fixed or explicitly waived.

`/analyze` can run at any time to check a feature's artifacts against each other — it's a safety check, not a gate.

You don't have to start at `draft`. A brownfield feature can enter with a sparse sketch spec and gain precision as you touch the code; a `done` feature reopens automatically when a bug or change request surfaces. See [framework/constitution.md](framework/constitution.md) for the authoritative rules.

## Commands

Adoption installs a full set of verb-named, session-aware commands. Use `/target` to switch the working feature; `/specify` creates one and targets it automatically.

Each entry below says what the command **does**; the section it links to says when you would reach for it and why it exists.

- **Pipeline — advance state**
  - [`/specify`](docs/slash-commands.md#specify--a-new-feature-spec-targeted-for-the-session) — create a spec for new work no existing spec covers, and target it
  - [`/clarify`](docs/slash-commands.md#clarify--open-questions-resolved-and-the-spec-advanced-to-clarified) — resolve the spec's open questions and advance it to `clarified`
  - [`/plan`](docs/slash-commands.md#plan--technical-decisions-affected-files-and-an-ordered-task-list) — turn the spec into technical decisions, affected files, and an ordered task list
  - [`/implement`](docs/slash-commands.md#implement--the-code-and-the-specs-move-to-in-progress-then-done) — work the task list and write the code
  - [`/review`](docs/slash-commands.md#review--reviewmd-and-a-hold-on-done-while-violations-stand) — analyze the code against the applicable rules, and block `done` on violations
  - [`/analyze`](docs/slash-commands.md#analyze--a-report-of-where-a-features-own-artifacts-disagree) — audit a feature's artifacts against each other: drifted checkboxes, stale reviews, dead links
- **Refine — adjust a spec's artifacts**
  - [`/amend`](docs/slash-commands.md#amend--a-question-or-scenario-recorded-with-the-lifecycle-back-edge-taken) — add a question or scenario to a spec, reopening it if its status requires
  - [`/supersede`](docs/slash-commands.md#supersede--the-supersedes-key-on-one-spec-the-annotation-on-the-other) — record that a newer spec counters an older one, and annotate the older one
- **Destructive — these remove content**
  - [`/prune`](docs/slash-commands.md#prune--spent-task-sections-in-one-tasksmd) — drop completed task sections from `tasks.md`, which is not for durable content
  - [`/fold`](docs/slash-commands.md#fold--the-branch-scoped-staging-directory-after-migrating-its-content) — merge a branch-scoped spec into its durable home and remove the staging directory
  - [`/consolidate`](docs/slash-commands.md#consolidate--an-entire-spec-directory) — re-point every reference to a replaced spec, then remove it
- **Brownfield — absorb existing reality**
  - [`/log`](docs/slash-commands.md#log--one-raw-line-in-specsinboxmd) — add an item to `specs/inbox.md` to be picked up later with `/groom`
  - [`/groom`](docs/slash-commands.md#groom--every-inbox-item-routed-to-its-real-home) — walk the inbox and route each item to a rule, a spec, or a scenario
- **Orient**
  - [`/target`](docs/slash-commands.md#target--the-sessions-working-feature) — set the feature the other commands act on
  - [`/status`](docs/slash-commands.md#status--the-pipeline-view-of-every-feature) — show every feature's pipeline status and what is holding it back
  - [`/link`](docs/slash-commands.md#link--a-registered-sibling-service-so-cross-service-references-resolve) — register another service's repo so cross-service references resolve
  - [`/help`](docs/slash-commands.md#help--the-command-reference-generated-from-what-is-installed) — show the command reference for this project
- **Bootstrap — one-time per project**
  - [`/ductus`](docs/slash-commands.md#ductus--the-framework-installed-or-updated-in-this-project) — install or update the framework in this project
  - [`/configure`](docs/slash-commands.md#configure--agent-permissions-for-the-ductus-commands) — grant your agent the permissions the `ductus` commands need

Each command is documented in full — what it is for, why it exists, and what it will not do — in **[docs/slash-commands.md](docs/slash-commands.md)**.

## Rules

**Rules are how `ductus` knows what "good" means for your project.** A spec says what a feature should do; the rules say what any code is held to regardless of feature — and `/review` audits the implementation against them before a spec can reach `done`. Without them the review has taste and nothing else.

They are plain markdown in your project's rule-file directory — the `rules/` directory under your spec root once scaffolded, or [`framework/rules/`](framework/rules/) here in `ductus`'s own repo — and they are the **only** normative source a review may cite: `/review` is instructed not to invent criteria beyond these files and your `AGENTS.md`. That is what keeps two reviews of the same code from disagreeing, and what makes a finding arguable — every one quotes the rule it came from.

**Every rule uses RFC 2119 language, and the distinction is load-bearing.** **MUST** / **MUST NOT** violations are blocking: they hold the spec out of `done` until they are fixed or waived with a recorded reason. **SHOULD** / **SHOULD NOT** violations are advisory — reported, never blocking.

**Every rule carries a permanent ID** (`BE-AUTHN-003`, `FE-XSS-001`, `QUAL-CLAIM-001`). IDs are never renumbered or reused, even when a rule moves within its file, so a waiver, a code comment, or a spec can cite one and still mean the same thing years later.

### Which rules load

A rule file's **filename suffix** decides which projects load it, and there are exactly three:

| Suffix | Loads for | Example |
| --- | --- | --- |
| `-backend.md` | projects with a backend surface | `security-backend.md` |
| `-frontend.md` | projects with a frontend surface | `security-frontend.md` |
| `-cross.md` | every project, unconditionally | `quality-cross.md` |

Set `[rules] surfaces` in `.ductus/config.toml` to declare your surfaces (`["backend"]`, `["backend", "frontend"]`, or `[]` for cross-only); leave it unset and `ductus` falls back to the stack it detects. A file with an unrecognized suffix loads for *every* stack and warns — the default is never a silent skip.

To stand a rule file down without deleting it, list it under `[[review.disabled-rule-files]]` with a mandatory `reason`. The reason is the audit trail, and it is why the opt-out is a config entry rather than a deletion.

### Security — `security-backend.md`, `security-frontend.md`

The reason the rule system exists. Backend covers authentication, authorization, input validation, data protection, API security, logging and audit, dependency management, and error handling. Frontend covers XSS, CSRF, secure client-side storage, authentication UX, content security policy, dependencies, and handling of sensitive data.

These carry the highest proportion of **MUST** rules, because the failures they describe are exploitable rather than merely untidy.

### Code quality — `quality-cross.md`

Three failure modes that look like working code and are not, on any surface:

- **Silent stubs** — an unimplemented path whose contract implies it does work, returning success anyway. A no-op rate limiter ships indistinguishably from a real one.
- **Unverified external contracts** — code whose correctness depends on a schema, an API shape, or a file format it does not own, with nothing that fails loudly when the assumption is wrong.
- **Unsubstantiated clean results** — a result that reports "clean" when it means "could not check". A caller cannot tell a verified-clean answer from an unverifiable one, and will read the reassuring one.

### Reliability — `reliability-backend.md`

Behavior under partial failure: bounded timeouts on every outbound call, retry discipline, circuit breakers, graceful shutdown that does not drop in-flight work, and bulkheads that shed load rather than queue unboundedly. These are design-time commitments a plan states, not patterns a linter finds.

### Performance — `performance-backend.md`, `performance-frontend.md`

Backend: query efficiency (N+1, unindexed filters, unbounded reads), caching with a stated expiry, connection-pool discipline, payload budgets, and offloading slow work off the request path. Frontend: Core Web Vitals budgets, bundle size, image delivery, resource loading, and web-font discipline.

### API contracts — `api-backend.md`

The shape, stability, and documentation of an interface other people call: versioning, pagination, status-code semantics, and breaking-change discipline. Deliberately separate from security — `security-backend.md` §BE-API owns the HTTP *attack* surface, this owns the *contract*.

### Concurrency — `concurrency-backend.md`

Shared-state races, locking and deadlock avoidance, transaction isolation, and distributed coordination.

### Observability — `observability-backend.md`

Metrics, distributed tracing, and health signaling — so a failure in production is diagnosable rather than merely visible.

### Accessibility — `accessibility-frontend.md`

WCAG 2.2 AA: semantic HTML, keyboard navigation and focus, ARIA usage, color and contrast, accessible forms, and text alternatives. AA is the baseline written into law in several jurisdictions, which is why it is a shipped rule file rather than a suggestion.

### Configuration — `configuration-cross.md`

Named constants and environment variables: no magic numbers, no undocumented env vars, and fail-fast on invalid configuration rather than starting up degraded.

### Waivers

When a **MUST** violation is intentional, record a waiver rather than silencing the gate:

```bash
/review --waive <rule-id> --reason "<text>"
```

The waiver is anchored to the `(rule, file)` pair, so code moving within a file keeps it while a rename or a fix expires it and the finding re-blocks — a waiver cannot quietly outlive the thing it excused. The schema is open, so an organization can require its own fields (a ticket, a second approver) and `ductus` preserves them. See [specs/020-code-review/data-model.md](specs/020-code-review/data-model.md).

### Writing your own

Add a file to the rule-file directory — `rules/` under your spec root, or [`framework/rules/`](framework/rules/) here — with one of the three suffixes and it is discovered automatically — no registration step, no manifest entry. Give each rule a permanent ID, state it in RFC 2119 language, and record the rationale: a rule whose reasoning is not written down is one nobody can argue with when it is wrong.

**A rule file you wrote is yours.** `/ductus` only ever rewrites the rule files it ships; it does not touch, move, or delete a file you added, on any upgrade. The one case that needs a decision is a shipped file you have *edited* — that one is overwritten on the next update unless you pin it in `.ductus/config.toml` `[pinned] files`. Upstreaming the change is usually better than pinning: a pinned file stops receiving fixes.

## Installing (per agent)

`ductus` operates a **live-on-main** model — the installer fetches the latest from `main`. Omit the agent to install for Claude Code, or name it explicitly.

### Claude Code

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/stonean/ductus/main/install.sh | sh
```

### Auggie

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/stonean/ductus/main/install.sh | sh -s -- auggie
```

Auggie needs a one-time manual MCP registration (`auggie mcp add ductus …`) — see [Registering the runtime](docs/slash-commands.md#registering-the-runtime).

### Antigravity

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/stonean/ductus/main/install.sh | sh -s -- antigravity
```

Then run `/ductus {project-name}`. The installer creates the right directory for your agent and drops the bootstrap command in place — for Antigravity it's wrapped as a skill under `.agents/skills/ductus/`, since Antigravity discovers dir-form skills rather than verbatim command files. It's safe to re-run. (`agy`, the Antigravity CLI command name, works in place of `antigravity`.)

### OpenCode

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/stonean/ductus/main/install.sh | sh -s -- opencode
```

OpenCode installs the bootstrap as a verbatim command at `.opencode/command/ductus.md` (invoked `/ductus`) and reads `AGENTS.md` natively — no `CLAUDE.md`. `/ductus` wires the `ductus` runtime automatically by writing the project's root `opencode.json`; because OpenCode loads config once at startup, restart it after the first wiring (see [Registering the runtime](docs/slash-commands.md#registering-the-runtime)).

The same bootstrap supports every agent, so re-run `/ductus --add-agent` from any adopted agent later to add others. `/ductus` acquires the runtime and wires it in the same run — automatically for Claude and OpenCode (both keep MCP config in a committed repo file), or by surfacing a one-time registration step for Auggie and Antigravity (see [Registering the runtime](docs/slash-commands.md#registering-the-runtime)).

## Brownfield adoption

You don't need to clone `ductus` or rewrite history to adopt it. Install the command, run `/ductus`, then let specs accrete naturally:

- Use `/specify` with a sparse description to stub a skeleton spec for an existing feature — sparse acceptance criteria are valid here.
- Let those specs gain precision incrementally through bug fixes, enhancements, and `/clarify`.
- Drop raw items into `specs/inbox.md` with `/log` without breaking flow, and route them later with `/groom`.

Adoption spreads by feature area, not in a big bang. The goal is for `inbox.md` to eventually disappear.

### Bugs are unwritten scenarios

`ductus` treats every bug as evidence that a spec is missing, ambiguous, or violated. When one surfaces, follow the decision tree in order:

1. **No spec exists** — write the spec first, then fix the code.
2. **Spec is ambiguous** — fix the spec, then fix the implementation.
3. **Spec is clear, implementation is wrong** — add a scenario, then fix the code.

A scenario is a spec at a lower level of abstraction — same format, same discipline. Scenarios live in `specs/NNN-feature/scenarios/slug.md`, each gets a linked task in the parent spec, and any can be targeted directly with `/target feature/scenario-slug`.

## The runtime

The `ductus` runtime is the deterministic execution layer the pipeline runs on. It parses the prose of each command and runs the mechanical work (reading specs, walking tasks, checking dependencies, atomic checkbox updates, gate handshakes) in native Rust instead of slow LLM tokens — invoking the model only where semantic judgment actually matters (`assessSpecQuality`, `writeCode`, `writeSpecBody`).

**You do not install it.** `/ductus` acquires it during adoption: it reads the runtime version this framework revision pins, downloads the matching release asset for your platform, verifies its checksum, and installs it into a ductus-owned store. Your `PATH` is not consulted, and nothing binary enters your repository.

| | |
| --- | --- |
| The store | `~/.ductus/bin/ductus` — one binary per machine, written only by `/ductus` |
| The pointer | `.ductus/bin/ductus` — per project, gitignored, resolves to the store |

The pointer is what lets a committed MCP config work for the whole team: `.mcp.json` is shared, so a machine-specific absolute path in it would break every other contributor and every CI checkout. A repo-relative pointer resolves for all of them.

`/ductus` compares the pin against the installed binary on every run and re-acquires on mismatch, so upgrading is a routine `/ductus`. A machine running two ductus projects pinned to different versions holds one binary — whichever ran most recently — until `/ductus` runs in the other.

### Supplying your own binary

Set `[runtime] path` in `.ductus/config.toml` and `/ductus` downloads nothing, resolving the pointer to the binary you name:

```toml
[runtime]
path = "runtime/target/release/ductus"
```

This is the supported route for building from source, for an air-gapped or firewalled checkout, and for a platform with no published asset. A version mismatch against the pin warns rather than halts — you have stated deliberately which binary you want. A path that does not exist, or will not execute, halts naming it; `/ductus` never falls back to downloading, which would discard your choice without saying so.

### When acquisition fails

A network failure, an unpublished asset for your platform, or a checksum mismatch halts the run naming the store path and the release URL — so you can place the binary there by hand and re-run, or set `[runtime] path`. There is no silent degradation: the runtime is required, and a requirement that quietly is not one leaves both execution paths alive.

Binaries are published for `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, and `x86_64-pc-windows-msvc`. Every target ships a `.tar.gz` plus a `.sha256` sidecar, and a release publishes only when all five are present. If a runtime process crashes mid-procedure, just re-run the command — state lives in your markdown, and writes are filesystem-atomic, so the runtime resumes from the next incomplete step.

### Registering the runtime

`/ductus` wires the MCP server in the same run that acquires the binary. Where it points depends on where your agent reads MCP config:

- **Claude** — `/ductus` writes `.mcp.json` naming the repo-relative pointer; just start a fresh session. Fully automatic.
- **OpenCode** — `/ductus` writes the `ductus` `mcp` block into your committed root `opencode.json`, also naming the pointer; because OpenCode loads config once at startup, quit and restart it. No manual `mcp add`.
- **Auggie** — Auggie reads MCP servers from your user-level `~/.augment/settings.json`, which `/ductus` does not write. It surfaces a one-line command to run once per machine — `auggie mcp add ductus --command ~/.ductus/bin/ductus --args "mcp"` — then start a fresh session.
- **Antigravity** — Antigravity reads MCP servers only from your home-level `~/.gemini/config/mcp_config.json` (project-local config is ignored), which `/ductus` does not write. It surfaces an instruction: add a `ductus` block naming that same store path, then reload with the in-prompt `/mcp` overlay.

The two home-level agents name the **absolute store path** rather than the pointer, for the mirror-image reason: their config is per-machine and serves every project, so no project-relative path could be correct in it.

From that session on, the pipeline takes the deterministic path. File writes are additive — an existing MCP config keeps its other servers, and a `ductus` entry that's already present is left untouched.

## Configuration

`.ductus/config.toml` is an optional project file — `/ductus` runs fine without it. Create it only if you need one of these behaviors:

- **`[pinned]`** — list destination paths `ductus` should never overwrite, even files it normally updates (e.g. a customized `.ductus/constitution.md`).
- **`[rules]`** — declare which rule surfaces your project needs: `surfaces = ["backend"]`, `["frontend"]`, or both. `/ductus` prompts for this on first run, then installs only the matching rule files (cross-cutting `-cross` rules always apply) and `/review` enforces only those. Leave it unset to let `ductus` derive the surface from your stack and install every rule file.
- **`[paths]`** — rename the top-level directory that holds every `ductus` artifact: `specs-root = "governance"`. Defaults to `specs`; set it to avoid colliding with a sibling framework's directory (e.g. RSpec's `spec/`). `/ductus` prompts for it on first run; once set, every command and the runtime resolve it. A single directory name — no path separators, no `..`, no leading slash.
- **`[services]`** — register sibling services so cross-service reference links resolve to the linked spec's lifecycle status (see [Cross-service references](docs/slash-commands.md#cross-service-references)). Add entries with `/link`, not by hand.

```toml
[pinned]
files = [".ductus/constitution.md"]

[rules]
surfaces = ["backend"]

[paths]
specs-root = "governance"
```

For the full schema, see [specs/019-config-decisions/data-model.md](specs/019-config-decisions/data-model.md).

## Cross-service references

When a project spans multiple services — each its own repo with its own `ductus` install — a spec can link a spec in another service and see its live lifecycle status. The reference is a standard markdown link to the linked spec's **canonical repo URL**; that URL is identity and navigation only and is **never fetched**. `ductus` reads the linked spec's `status` from its **local checkout**, resolved through the `.ductus/config.toml [services]` registry.

References are informative, never dependencies: they do not enter `dependencies:`, do not gate completion, and never block a pipeline gate. They are harvested into a derived `references:` frontmatter index — distinct from `dependencies:` — by the `derive-references` runtime primitive; you never hand-author it.

### Documenting a reference in a spec

You author a reference by writing a **normal inline markdown link** in the spec **body** — nothing goes in the frontmatter, and there is no special syntax. The link's href must be an **absolute `http(s)` URL** whose path contains the target spec's `/specs/NNN-slug/` segment in the other service's repo:

```markdown
Tokens follow the contract in
[api 014-auth-tokens](https://github.com/acme/api/blob/main/specs/014-auth-tokens/spec.md).
```

On the next commit (or any `ductus derive-references --write` run) the derivation harvests that link into the frontmatter:

```yaml
references:
  - service: api      # the [services] alias whose repo matches the URL host
    spec: 014-auth-tokens
```

What the generator keys on:

- **`NNN-slug` is the identity.** Everything in the URL before a `/blob/<ref>/` or `/tree/<ref>/` branch segment is the repo, matched against `.ductus/config.toml [services]` to resolve the alias; the branch is ignored, so two links to the same spec on different branches collapse to one reference. A URL matching no registered service is still recorded, with `service: null` (the `unregistered` outcome above).
- **Absolute URL, not a sibling link.** `[label](../014-auth-tokens/spec.md)` is a *sibling* link and becomes a **dependency** (a different generator, the blocking `dependencies:` graph) — never a cross-service reference. Use the full canonical URL precisely so the two stay distinct.
- **Opt-outs are honored.** A link is **not** harvested if it sits under a `## See also` heading, inside a fenced code block, wrapped in `` `backticks` `` (inline code reads as an illustrative example, not a live link), or on a blockquote (`>`) line. These are the same navigational opt-outs `dependencies:` honors — use them for example or "see also" links you don't want to register.

Register a service with `/link` (alias, repo URL, local checkout path, optional description):

```toml
[services.api]
repo = "https://github.com/acme/api"
path = "../api"
description = "owns shared data models"
```

The registry is **required for status resolution, optional for referencing** — an unregistered link is just navigation. `/status` shows each reference's resolution outcome (and, on `ok`, the linked status); `/analyze` reports a provably broken one as an Advisory finding. The outcome depends on what can be proven:

| Outcome | Meaning |
| --- | --- |
| `ok` | Registered, checkout reachable, target spec resolves — surfaces the linked lifecycle status |
| `unregistered` | The repo matches no `[services]` entry — a plain navigational link; run `/link` to register the service |
| `not-checked-out` | Registered, but the local `path` is missing or unusable — `unknown`, never reported as broken |
| `broken` | Registered and reachable, but the target spec does not resolve (renamed, moved, deleted, or mistyped) — an `/analyze` finding |
| `status-unreadable` | The target exists but its `status` cannot be read — `unknown`, the defect is upstream's |

Status resolution runs only where the linked service is already checked out locally; `ductus` never fetches or clones a repo. For the full schema, see [specs/030-cross-service-references/data-model.md](specs/030-cross-service-references/data-model.md).

## Updating an adopted project

Re-run `/ductus` to pull the latest framework files. Each file is handled by one of three strategies:

| Strategy | Behavior | Examples |
| --- | --- | --- |
| `update` | Always overwritten with the latest version | `.ductus/constitution.md`, spec templates, slash commands |
| `create` | Created on first run, skipped on re-run | `specs/system.md`, `specs/errors.md`, `specs/events.md` |
| `skip` | Never overwritten | `AGENTS.md`, `CLAUDE.md` |

`.gitignore` uses a `merge` strategy — `ductus` patterns are appended below a `# ductus` marker. Pin individual files you've customized with `[pinned]` in `.ductus/config.toml` (above). `ductus` is a reference, not a runtime dependency: if you'd rather not use `/ductus`, diff the repo and apply changes at your own pace.

## Viewing artifacts

`ductus` artifacts are plain markdown with YAML frontmatter, so any markdown viewer or PKM tool can browse them:

- **GitHub** — push `specs/` and browse inline; relative links resolve natively
- **[Obsidian](https://obsidian.md)**, **[Logseq](https://logseq.com)**, **[Foam](https://foambubble.github.io/foam/)** — graph view and backlinks out of the box
- **[Quartz](https://quartz.jzhao.xyz)** or **[MkDocs](https://www.mkdocs.org)** — publish a static site
- Plain `cat`, a GitHub PR review, or any markdown editor — no viewer required

Artifacts stay the portable source of truth; structured viewers are derived views (see [constitution §text-first-artifacts](framework/constitution.md#text-first-artifacts)).

## Repository layout

This repo is the source for everything `ductus` ships, plus its own dogfooded specs.

- **[framework/](framework/)** — everything that ships to adopting projects
  - [constitution.md](framework/constitution.md) — guiding principles, pipeline, spec lifecycle, quality standards (authoritative)
  - [rules/](framework/rules/) — domain rule sets adopted by reference
  - [templates/](framework/templates/) — starter files for specs and project scaffolding
  - [commands/](framework/commands/) — slash command sources
  - [bootstrap/](framework/bootstrap/) — the `ductus.md` installer and per-agent permission files
- **[install.sh](install.sh)** — the `curl … | sh` installer that places the `/ductus` bootstrap command for your agent
- **[docs/slash-commands.md](docs/slash-commands.md)** — the full reference for every slash command
- **[runtime/](runtime/)** — the `ductus` deterministic runtime (Rust)
- **[specs/](specs/)** — `ductus`'s own feature specs; it develops itself with its own pipeline. See [specs/README.md](specs/README.md) for cross-cutting decisions and deferred work.
- **[scripts/](scripts/)** — maintenance and generator scripts

`ductus` currently distributes to four AI coding agents: **Claude Code** (`.claude/` paths), **Auggie** (`.augment/` paths), **Antigravity** (`.agents/` paths, installed as a skill), and **OpenCode** (`.opencode/` command tree plus a committed root `opencode.json`). Adding another is a single registry row plus a permission file (or, for a new layout, a derived-values branch) — see [framework/bootstrap/ductus.md](framework/bootstrap/ductus.md#agent-registry).

## Contributing

All `.md` files must pass `npx markdownlint-cli2` using the project config; see [constitution §markdown-standards](framework/constitution.md#markdown-standards) for the rule set. `ductus` dogfoods its own pipeline — changes to the framework go through the same `/specify → /plan → /implement → /review` loop, recorded under [specs/](specs/).

## License

[MIT](LICENSE)
</content>
</invoke>
