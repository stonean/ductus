# ductus

**Focused, grounded AI-assisted development: durable specs for your business rules, built-in rules for security, accessibility, and performance.**

Your AI coding agent works from the feature in front of it — its spec, plan, and tasks — instead of piecing together context from the whole codebase. The constitution and the commands keep it grounded: it reads the source rather than recalling it, and asks when something isn't settled. Your business rules live in specs that stay accurate after the code ships, and every feature is reviewed against built-in rules, plus any you write, before it's done.

`ductus` works with Claude Code, Auggie, Antigravity, OpenCode, and Pi. It is tech-stack agnostic and its artifacts are plain markdown, so nothing is added to your project's dependency manifest and there is nothing to compile.

## What you get

- **Your agent works on the feature, not the whole codebase.** `/target` sets the feature every command acts on, so the agent starts from that feature's spec, plan, and tasks. The constitution requires it to read the source rather than answer from memory, and to say what it did not examine.
- **You find out what you don't know before the code does.** Describe a feature in plain English and `/specify` turns it into a spec; `/clarify` then works through the questions it raises, and nothing is planned until they are answered. Explaining a feature is where its gaps show — a rubber duck, except this one asks back.
- **A reviewer that already knows security, accessibility, and performance.** `/review` audits every feature against the [rules](#rules) your project loads, and every finding quotes the rule it came from by a permanent ID, so a finding can be argued with rather than taken on faith.
- **Your standards are enforced, not suggested.** A rule written into a prompt is one the agent may follow. A `ductus` MUST violation holds the feature out of `done` until it is fixed or waived with a recorded reason, and the waiver expires when the code it excused changes.
- **"Done" means done.** A feature reaches `done` only after its review and analysis have run and every finding has been fixed, routed to where the work belongs, or discarded with its reason. Both records state what they did not examine, so a check that could not run never passes for a clean one.
- **Every bug makes the project sharper.** A bug that exposes a missing rule becomes one, so every later review checks for it; a bug in specified behavior becomes a scenario recording what correct looks like.
- **Your business rules stay written down.** The spec records what each feature does and why it was built that way, and it stays accurate after the code ships — reasoning that otherwise ends up scattered across chat threads, wiki pages, and closed tickets.

## Quick start

You install a single command, and `/ductus` acquires everything else — including the deterministic runtime the pipeline executes on. Install `ductus` into any project:

```bash
curl --proto '=https' --tlsv1.2 -sSfL https://github.com/stonean/ductus/releases/latest/download/install.sh | sh
```

This installs the `/ductus` bootstrap command for Claude Code — see [Installing](#installing-per-agent) to target Auggie, Antigravity, or OpenCode instead. Then, in your agent, run:

```text
/ductus my-project
```

That one command scaffolds the `specs/` directory, installs the full set of slash commands, acquires and wires the runtime, sets up the constitution and agent rules, and prints your next steps. It's idempotent — safe to re-run any time to update to the latest `ductus` release.

The installed commands are **namespaced to your project**, so the name you passed becomes the prefix:

```text
/my-project:specify   add user login with email and password
/my-project:clarify              # resolve open questions the spec surfaced
/my-project:plan                 # technical decisions, affected files, tasks
/my-project:implement            # work the tasks; code gets written here
/my-project:review               # audit the code against the rules
```

Each command advances the feature one step and leaves a durable artifact behind. That's the whole loop.

## Rules

**Rules are how `ductus` knows what "good" means for your project.** A spec says what a feature should do; the rules say what any code is held to, whatever the feature. They are plain markdown, they are the only thing `/review` may cite besides your `AGENTS.md`, and every rule carries a permanent ID (`BE-AUTHN-003`, `FE-XSS-001`, `QUAL-CLAIM-001`) that a waiver, a code comment, or a spec can still cite years later.

`ductus` ships rule files for:

- **Security**, backend and frontend — authentication, authorization, input validation, data protection, XSS, CSRF
- **Accessibility** — WCAG 2.2 AA
- **Performance**, backend and frontend — query efficiency, caching, Core Web Vitals, bundle size
- **Reliability**, **concurrency**, and **observability** — timeouts and retries, races and locking, metrics and tracing
- **API contracts** and **configuration** — versioning and breaking changes; no magic numbers, undocumented environment variables, or feature flags without a declared default

One more, `quality-cross.md`, applies to every project and catches five failure modes that look like working code and are not:

- **Silent stubs** — an unimplemented path whose contract implies it does work, returning success anyway. A no-op rate limiter ships indistinguishably from a real one.
- **Unverified contracts** — code whose correctness depends on a schema, an API shape, or a file format defined somewhere else — by another system or elsewhere in the repository — with nothing that fails loudly when the assumption is wrong.
- **Unsubstantiated clean results** — a result that reports "clean" when it means "could not check". A caller cannot tell a verified-clean answer from an unverifiable one, and will read the reassuring one.
- **Unenumerated delegation to shared code** — a hand-rolled predicate, parser or validator replaced by a shared one without first listing what the local version enforced by construction, so the change silently widens or narrows what it accepts.
- **Tests that cannot fail** — a test that passes whether or not the behavior it names works: it asserts nothing about the result, pins a value incidental to it, lets a fallback satisfy the assertion meant for the primary path, can fail only under a condition it never sets up, or never reaches the code it names.

Your own rules are a markdown file added to the rule-file directory: discovered automatically, and never touched by an update. Which files load for your stack, how MUST and SHOULD differ, how waivers expire, and how to write a rule are in **[docs/rules.md](docs/rules.md)**.

## How it works

Under the hood, `ductus` is spec-driven development: every feature moves through one pipeline, and the status on each spec tracks where it is:

```text
draft ──/clarify──▶ clarified ──/plan──▶ planned ──/implement──▶ in-progress ──[/review + /analyze gates]──▶ done
```

- **Spec** (`/specify`, `/clarify`) — define *what* the feature does and *why*, with concrete acceptance criteria and a list of open questions. No open questions may remain before planning.
- **Plan** (`/plan`) — turn the spec into technical decisions, affected files, and an ordered task list. Persistence-heavy features also get a data model.
- **Implement** (`/implement`) — work the tasks; this is where code is written. Status moves to `in-progress`, then `done` once both gates pass.
- **Review** (`/review`) — audit the implementation against the framework's rules (security, reuse, quality, efficiency, simplicity). MUST violations and outstanding SHOULDs keep the feature out of `done` until they're fixed or explicitly waived. Each observation outside the rules is fixed, routed to where the work belongs, or discarded with its reason, once you confirm it; a route can reopen a `done` spec, and an observation left undispositioned holds `done` too. `/review` is a gate, not a state transition: it records findings, and `/implement` is what writes `done`.
- **Analyze** (`/analyze`) — audit the feature's own artifacts against each other. It can run at any time as a safety check, and it is also the pipeline's second gate: `/implement` will not write `done` until an analysis has run, left no blocking findings, and given every finding a disposition: fixed, routed to where the work belongs, or discarded with its reason. Each run records what it found — and what it could not examine — in `specs/{feature}/analysis.md`, whose frontmatter is the record and whose body is the report. See **[docs/analyze.md](docs/analyze.md)**.

You don't have to start at `draft`. A brownfield feature can enter with a sparse sketch spec and gain precision as you touch the code; a `done` feature takes the back-edge to `in-progress` when `/amend` records a new scenario or a meaningful body edit lands. See [framework/constitution.md](framework/constitution.md) for the authoritative rules.

## Commands

Adoption installs a full set of verb-named, session-aware commands. Use `/target` to switch the working feature; `/specify` creates one and targets it automatically.

Command names below are written bare for readability. As installed they carry your project's namespace — `/my-project:specify` on Claude Code and Auggie; `/my-project-specify` on Antigravity, which discovers flat-named skills rather than a namespaced command directory; `/my-project-specify` on Pi, which discovers flat-named prompt templates the same way; `/my-project/specify` on OpenCode, which namespaces by subdirectory.

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
- **Destructive — these remove content**
  - [`/prune`](docs/slash-commands.md#prune--spent-tasks-and-plan-sections-outside-the-design-record) — drop completed task sections from `tasks.md`, or with `--reset` reset it to template state, pending tasks included; move the durable pieces of each plan section outside the design record home, then remove the section; `--all` repeats it for every spec
  - [`/fold`](docs/slash-commands.md#fold--the-branch-scoped-staging-directory-after-migrating-its-content) — merge a branch-scoped spec into its durable home and remove the staging directory
  - [`/consolidate`](docs/slash-commands.md#consolidate--an-entire-spec-directory) — re-point every reference to a replaced spec, then remove it
- **Brownfield — absorb existing reality**
  - [`/log`](docs/slash-commands.md#log--one-raw-line-in-specsinboxmd) — add a todo you capture by hand to `specs/inbox.md`, to be picked up later with `/groom`
  - [`/groom`](docs/slash-commands.md#groom--every-inbox-item-routed-to-its-real-home) — walk the inbox and route each item to a rule, a spec, a scenario, a chore done in the pass, or discard
- **Orient**
  - [`/target`](docs/slash-commands.md#target--the-sessions-working-feature) — set the feature the other commands act on
  - [`/status`](docs/slash-commands.md#status--the-pipeline-view-of-every-feature) — show every feature's pipeline status and what is holding it back
  - [`/link`](docs/slash-commands.md#link--a-registered-sibling-service-so-cross-service-references-resolve) — register another service's repo so cross-service references resolve
  - [`/help`](docs/slash-commands.md#help--the-command-reference-generated-from-what-is-installed) — show the command reference for this project
- **Bootstrap — one-time per project**
  - [`/ductus`](docs/slash-commands.md#ductus--the-framework-installed-or-updated-in-this-project) — install or update the framework in this project
  - [`/configure`](docs/slash-commands.md#configure--agent-permissions-for-the-ductus-commands) — grant your agent the permissions the `ductus` commands need

Each command is documented in full — what it is for, why it exists, and what it will not do — in **[docs/slash-commands.md](docs/slash-commands.md)**.

## Installing (per agent)

The installer is attached to every `ductus` release, and the one-liner fetches the latest release's copy (`-L` follows GitHub's redirect to its asset host). By default it places the latest release's bootstrap, and `/ductus` then updates from that release — never from unreleased work on `main`. Omit the agent argument to install for Claude Code, or name one explicitly:

```bash
curl --proto '=https' --tlsv1.2 -sSfL https://github.com/stonean/ductus/releases/latest/download/install.sh | sh -s -- <agent>
```

| Agent | Argument | Where the bootstrap lands | Note |
| --- | --- | --- | --- |
| Claude Code | *(omit)* or `claude` | `.claude/commands/ductus.md` | Reads `CLAUDE.md` natively. MCP wiring is fully automatic. |
| Auggie | `auggie` | `.augment/commands/ductus.md` | Reads `CLAUDE.md` natively. Needs a one-time manual MCP registration — see [Registering the runtime](docs/runtime.md#registering-the-runtime). |
| Antigravity | `antigravity` (or `agy`) | `.agents/skills/ductus/SKILL.md` | Wrapped as a skill, since Antigravity discovers dir-form skills rather than verbatim command files. Reads `AGENTS.md`. Needs a one-time manual MCP registration. |
| OpenCode | `opencode` | `.opencode/command/ductus.md` | Reads `AGENTS.md` natively — no `CLAUDE.md`. `/ductus` writes the project's root `opencode.json`; because OpenCode loads config once at startup, restart it after the first wiring. |
| Pi | `pi` | `.pi/prompts/ductus.md` | Installed as a prompt template, which Pi expands natively (arguments, `description`/`argument-hint` frontmatter). Reads `AGENTS.md`. `/ductus` writes the `.pi/extensions/ductus.ts` bridge that exposes the runtime tools (Pi has no built-in MCP); accept the project-trust prompt once, then restart pi after wiring. |

Then run `/ductus {project-name}` in your agent. The installer creates the right directory for your agent and drops the bootstrap command in place; it's safe to re-run.

To start from another source, add `--ref` beside the agent argument — `sh -s -- claude --ref=main` for the tip of `main`, or `--ref=ductus-v0.55.0` for a named release — then run `/ductus --ref=<the same value> {project-name}`. The installer writes no project configuration, so that first `/ductus` run is what records the choice; a plain `/ductus` would use the latest release (see [Updating an adopted project](#updating-an-adopted-project)).

The same bootstrap supports every agent, so re-run `/ductus --add-agent` from any adopted agent later to add others. `/ductus` acquires the runtime and wires it in the same run — automatically for Claude, OpenCode, and Pi (the first two keep MCP config in a committed repo file, and Pi's bridge is a project-local extension file), or by surfacing a one-time registration step for Auggie and Antigravity (see [Registering the runtime](docs/runtime.md#registering-the-runtime)).

## Brownfield adoption

You don't need to clone `ductus` or rewrite history to adopt it. Install the command, run `/ductus`, then let specs accrete naturally:

- Use `/specify` with a sparse description to stub a skeleton spec for an existing feature — sparse acceptance criteria are valid here.
- Let those specs gain precision incrementally through bug fixes, enhancements, and `/clarify`.
- Drop raw items into `specs/inbox.md` with `/log` without breaking flow, and route them later with `/groom`.

Adoption spreads by feature area, not in a big bang. The *brownfield backlog* in `inbox.md` drains toward empty as that happens — but the file itself stays: it is where people log todos, an ongoing role rather than a migration phase. Nothing a pipeline run finds is written there. `/review`, `/analyze`, and `/implement` fix, route, or discard each of their own findings in the run that found it, so a spec cannot reach `done` while one waits somewhere no gate reads ([§brownfield-inbox](framework/constitution.md#brownfield-inbox)).

### Bugs are unwritten scenarios

`ductus` treats every bug as evidence that a rule, a spec, or a scenario is missing, ambiguous, or violated. When one surfaces, follow the decision tree in order — the first matching condition determines the route:

1. **No rule covers this cross-cutting concern** — the bug surfaces a class of behavior that should be governed at the rules tier (perf budget, observability commitment, security control, accessibility minimum). Promote it to a rule, new or amended, then fix the code.
2. **No spec exists for the behavior** — write the spec first, then fix the code.
3. **Spec exists but is ambiguous or incomplete** — fix the spec, then fix the implementation.
4. **Spec is clear but implementation is wrong** — add a scenario capturing the correct behavior, then fix the code.

In all four cases the artifact update is the primary outcome, not a bug report. A scenario is a spec at a lower level of abstraction — same format, same discipline. Scenarios live in `specs/NNN-feature/scenarios/slug.md`, each gets a linked task in the parent spec, and any can be targeted directly with `/target feature/scenario-slug`.

## The runtime

The `ductus` runtime is the deterministic execution layer the pipeline runs on. It parses the prose of each command and runs the mechanical work (reading specs, walking tasks, checking dependencies, atomic checkbox updates, gate handshakes) in native Rust instead of slow LLM tokens — invoking the model only where semantic judgment actually matters (`askClarifyQuestion`, `assessSpecQuality`, `performReview`, `registerService`, `routeFold`, `routeInboxItem`, `verifyCriteria`, `writeCode`, `writeSpecBody`).

**You do not install it.** `/ductus` acquires it during adoption: it reads the runtime version this framework revision pins, downloads the matching release asset for your platform, verifies its checksum, and installs it into a ductus-owned store. Your `PATH` is not consulted, and nothing binary enters your repository.

| | |
| --- | --- |
| The store | `~/.ductus/bin/ductus` — one binary per machine, written only by `/ductus` |
| The pointer | `.ductus/bin/ductus` — per project, gitignored, resolves to the store |

The pointer is what lets a committed MCP config work for the whole team: `.mcp.json` is shared, so a machine-specific absolute path in it would break every other contributor and every CI checkout. A repo-relative pointer resolves for all of them.

`/ductus` reads the pin from the release it updates from, compares it against the installed binary on every run, and re-acquires on mismatch, so upgrading is a routine `/ductus`. A machine running two ductus projects on different releases holds one binary — whichever ran most recently — and the two re-acquire it over each other, one `/ductus` run at a time. A project that needs its own binary sets `[runtime] path`.

The runtime is **required**: acquisition failure halts the run rather than degrading, because a requirement that quietly is not one leaves both execution paths alive. For supplying your own binary, what to do when acquisition fails, how the MCP server is registered per agent, and the environment variables the runtime reads, see **[docs/runtime.md](docs/runtime.md)**.

## Configuration

`.ductus/config.toml` is the project's configuration and persisted-decisions store. `/ductus` creates and maintains it — it writes the `[host]` namespace block on every run and persists the inputs it collects on first run — so an adopted project always has one. What you may want to edit by hand:

- **`[pinned]`** — list destination paths `ductus` should never overwrite, even files it normally updates (e.g. a customized `.ductus/constitution.md`).
- **`[rules]`** — declare which rule surfaces your project needs: `surfaces = ["backend"]`, `["frontend"]`, or both. `/ductus` prompts for this on first run, then installs only the matching rule files (cross-cutting `-cross` rules always apply) and `/review` enforces only those. Leave it unset to let `ductus` derive the surface from your stack and install every rule file.
- **`[paths]`** — rename the top-level directory that holds every `ductus` artifact: `specs-root = "governance"`. Defaults to `specs`; set it to avoid colliding with a sibling framework's directory (e.g. RSpec's `spec/`). `/ductus` prompts for it on first run; once set, every command and the runtime resolve it. A single directory name — no path separators, no `..`, no leading slash.
- **`[runtime]`** — `path` to a binary you supply, instead of the one `/ductus` downloads (see [docs/runtime.md](docs/runtime.md#supplying-your-own-binary)).
- **`[[review.disabled-rule-files]]`** — stand a rule file down from `/review` with a mandatory `reason` (see [Which rules load](docs/rules.md#which-rules-load)).
- **`[artifacts]`** — `read-size-bytes`, the size in bytes above which `/clarify`, `/plan` and `/analyze` warn that a spec artifact may not be read in one call by an agent's file reader, naming the fixes for it. A whole number of bytes, `50000` when unset: raise it for a host with a larger read cap, lower it for a smaller one or for token-dense artifacts. Any other value halts the command that reads it, naming the setting, the value and the file; only an unset key means the default. It cannot switch the warning off — discard one file's finding in `/analyze` instead.
- **`[services]`** — register sibling services so cross-service reference links resolve to the linked spec's lifecycle status. Add entries with `/link`, not by hand.
- **`[constitutions]`** — register a governance document your organization owns, so one house rule reaches every project instead of being retyped into each (see [Shared constitutions](#shared-constitutions)). Hand-edited; `repo` is never fetched, the local `path` is what is read.

```toml
[pinned]
files = [".ductus/constitution.md"]

[rules]
surfaces = ["backend"]

[paths]
specs-root = "governance"
```

`/ductus` also maintains `[host]`, `[project]`, `[source]`, and `[migrations]`; those are written for you and are not meant to be hand-edited — `[source]` records the release a project updates from, and changes with `/ductus --ref` (see [Updating an adopted project](#updating-an-adopted-project)). The fully commented schema — every section, with the values each key accepts — is in [`framework/bootstrap/ductus.md` §Project Configuration](framework/bootstrap/ductus.md#project-configuration); the `[services]` schema is declared in [specs/030-cross-service-references/data-model.md](specs/030-cross-service-references/data-model.md).

## Cross-service references

When a project spans multiple services — each its own repo with its own `ductus` install — a spec can link a spec in another service and see its live lifecycle status. You author one as a normal inline markdown link to the linked spec's **canonical repo URL**; that URL is identity and navigation only and is **never fetched**. `ductus` reads the linked spec's `status` from its **local checkout**, resolved through the `.ductus/config.toml [services]` registry you populate with `/link`.

References are informative, never dependencies: they do not enter `dependencies:`, do not gate completion, and never block a pipeline gate. They are harvested into a derived `references:` frontmatter index — distinct from `dependencies:` — by the `derive-references` runtime primitive; you never hand-author it. `/status` shows each reference's resolution outcome and, where it resolves, the linked lifecycle status; `/analyze` reports a provably broken one as an Advisory finding.

Single-service adopters see none of this. For the authoring rules, what the generator keys on, the registry schema, and the five resolution outcomes, see **[docs/cross-service-references.md](docs/cross-service-references.md)**.

## Shared constitutions

`ductus` ships one constitution to every adopter, covering rules true for any project running the pipeline. A **shared constitution** covers the other kind: rules true for *your organization's* projects and nobody else's — a house code style, a deployment gate, a review convention. Register one and it reaches every project, instead of being retyped into each repo's `AGENTS.md` and drifting from there.

Register it in `.ductus/config.toml` by hand. As with `[services]`, the `repo` URL is identity only and is **never fetched** — the local `path` is a checkout you clone and update, so there is no transport, no auth story, and offline is the normal case:

```toml
[constitutions.acme]
repo = "https://github.com/acme/governance"
path = "../governance"
```

`ductus` reads one file from it, `constitution.md`, and `/target` loads it alongside the shipped constitution once per session — so every command inherits it with no per-command step. More than one may be registered. The shipped `.ductus/constitution.md` keeps receiving framework updates; you never pin it to add a house rule. The framework constitution stays a floor: a shared source may add rules and tighten them, never loosen them — [§governance-precedence](framework/constitution.md#governance-sources-and-which-governs) is where that order is stated, and where the fact that nothing enforces it is stated too.

If a registered checkout is missing, `ductus` warns and continues rather than blocking a teammate who has not cloned it — and `/review` and `/analyze` then report that source as **unexamined** rather than letting the run read as clean. A malformed *entry*, though, is a different severity and fails fast: an alias that is not a bare TOML key, a `repo` that is not URL-shaped, or an empty `path` halts the run naming the alias and the field, because that is a mistake in your own committed config rather than machine-local state. Projects that register nothing see none of this.

For the registry schema, the loading order, the resolution outcomes, and what the feature deliberately does not do, see **[docs/shared-constitution.md](docs/shared-constitution.md)**.

## Updating an adopted project

Re-run `/ductus` to update to the latest `ductus` release. Each file is handled by one of three strategies:

| Strategy | Behavior | Examples |
| --- | --- | --- |
| `update` | Always overwritten with the latest version | `.ductus/constitution.md`, spec templates, slash commands |
| `create` | Created on first run, skipped on re-run | `specs/system.md`, `specs/errors.md`, `specs/events.md` |
| `skip` | Never overwritten | `AGENTS.md`, `CLAUDE.md` |

`.gitignore` uses a `merge` strategy — `ductus` patterns are appended below a `# ductus` marker. Pin individual files you've customized with `[pinned]` in `.ductus/config.toml` (above). If you'd rather not run `/ductus` at all, the framework files are readable in this repo: diff them and apply changes at your own pace.

### Choosing the source

By default every `/ductus` run resolves the latest release — GitHub's latest, never a draft or a prerelease — and fetches the bootstrap, the runtime pin, and the framework files all from that one tag, so an update lands on a released state rather than on whatever `main` holds that moment. A change committed to `main` reaches a default project at the next `ductus-v*` release. Pass `--ref` to choose otherwise:

| Flag | Source |
| --- | --- |
| *(none)* | the recorded source, else the latest release |
| `--ref=main` | the tip of `main`, ahead of any release |
| `--ref=ductus-v0.55.0` | that release, and it stays there when newer ones ship |
| `--ref=latest` | the latest release, clearing a recorded choice |

`--ref=main` or a tag is recorded in `.ductus/config.toml` `[source] ref`, which is committed, so the release a project is on is shared by the whole team, and every later plain run uses it. Every run prints `Source: …` naming what it resolved. A tag older than the first release that honors `--ref` (`ductus-v0.55.0`), or older than a migration your project has already applied, is refused before anything is written; so is a malformed value or a repeated `--ref`. An unresolvable latest release halts rather than falling back to `main`.

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
- **[docs/](docs/)** — the reference manuals: [slash-commands.md](docs/slash-commands.md), [analyze.md](docs/analyze.md), [rules.md](docs/rules.md), [runtime.md](docs/runtime.md), [cross-service-references.md](docs/cross-service-references.md), [shared-constitution.md](docs/shared-constitution.md)
- **[runtime/](runtime/)** — the `ductus` deterministic runtime (Rust)
- **[specs/](specs/)** — `ductus`'s own feature specs; it develops itself with its own pipeline. See [specs/README.md](specs/README.md) for cross-cutting decisions and deferred work.
- **[scripts/](scripts/)** — maintenance and generator scripts

`ductus` currently distributes to these AI coding agents: **Claude Code** (`.claude/` paths), **Auggie** (`.augment/` paths), **Antigravity** (`.agents/` paths, installed as a skill), **OpenCode** (`.opencode/` command tree plus a committed root `opencode.json`), and **Pi** (`.pi/` prompt templates plus a project-local tool-bridge extension). Adding another is a registry row, an MCP registration row, a permission file, an `install.sh` `case` arm, and a row in the install table above (or, for a new layout, a derived-values branch too) — `/ductus:audit` Family 14 fails when the registry and the installer disagree, so a missed arm is loud rather than silent. See [framework/bootstrap/ductus.md](framework/bootstrap/ductus.md#agent-registry).

## Contributing

All `.md` files must pass `npx markdownlint-cli2` using the project config; see [constitution §markdown-standards](framework/constitution.md#markdown-standards) for the rule set. `ductus` dogfoods its own pipeline — changes to the framework go through the same `/specify → /plan → /implement → /review` loop, recorded under [specs/](specs/).

## License

[MIT](LICENSE)
