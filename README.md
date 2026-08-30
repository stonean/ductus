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

This installs the `/ductus` bootstrap command for Claude Code — see [Installing per agent](#installing-per-agent) to target Auggie, Antigravity, or OpenCode instead. Then, in your agent, run:

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

You don't have to start at `draft`. A brownfield feature can enter with a sparse sketch spec and gain precision as you touch the code; a `done` feature reopens automatically when a bug or change request surfaces. See [docs/introduction.md](docs/introduction.md) for the full mental model, and [framework/constitution.md](framework/constitution.md) for the authoritative rules.

## Commands

Adoption installs a full set of verb-named, session-aware commands. Use `/target` to switch the working feature; `/specify` creates one and targets it automatically.

Each table below says **when you would reach for a command**, not just what it does — several of these exist for a specific situation that is hard to guess from the name.

### Pipeline — advance state

| Command | When you reach for it |
| --- | --- |
| `/specify` | **New work, and no spec covers it.** Creates the feature directory and targets it. Before scaffolding anything it checks whether an existing spec already owns the work, because a duplicate spec is the one mistake that is expensive to undo later. Takes a rich description or a one-line sketch — sparse brownfield input is expected, not merely tolerated. Flags: `--branch` / `--branch-id <id>` with `--fold-into <feature>` for the branch-scoped form (see `/fold`), and `--supersedes <feature>` when the new spec counters an existing one. |
| `/clarify` | **The spec has open questions and you are about to plan against it.** Walks them one at a time, records each answer with its reasoning, and advances the spec to `clarified`. Planning over an unresolved question is how a plan gets written twice. |
| `/plan` | **The spec is settled and you need to know what the change touches.** Produces technical decisions, an affected-files list, and an ordered task list — plus a data model when the feature is persistence-heavy. |
| `/implement` | **Time to write code.** Walks the task list one task at a time, and is the only command that writes application code. Moves the spec to `in-progress`, then to `done` once every task and acceptance criterion is checked and the review gate passes. |
| `/review` | **Before you call a feature done.** Audits the code against the project's rule files across five dimensions and writes `review.md`. MUST violations hold the spec out of `done` until they are fixed or waived with a recorded reason — so "we will fix it later" leaves a trace instead of evaporating. `--all`, `--fix`, and `--waive <rule-id> --reason "<text>"` supported. |
| `/analyze` | **Something is out of sync and you want to know what.** Audits a feature's artifacts against *each other* — a task list that no longer matches the plan, a ticked criterion whose file is gone, a spec at `done` with a blocking review. Read-only; `--fix` corrects checkbox drift, `--all` scans every feature. |

### Refine — adjust a spec's artifacts

**Commands split on how many specs they write, and that is what decides whether a capability is a flag or a command of its own.** `/amend`, `/prune`, `/clarify`, `/plan` and `/implement` each write **one** spec, and each declares that single-spec scope. `/fold`, `/supersede` and `/consolidate` write **two**, so none of them fits inside a single-spec command as a flag — widening one to accommodate a two-spec operation qualifies every statement it makes about its own scope. That is why `--supersedes` is a flag on `/specify` (which is writing the spec anyway) while a *retroactive* supersession is `/supersede`, and why `/fold` gains no `--into`.

| Command | When you reach for it |
| --- | --- |
| `/amend` | **A question or a new behavior surfaced against a spec that has already moved on.** Records it and takes the lifecycle back-edge for you — a new question reopens the spec to `draft`, a new scenario reopens a `done` spec to `in-progress` — so a spec's status never quietly disagrees with its content. One spec. |
| `/fold` | **Two branches each need a spec, and both would claim the same next number.** That collision is a merge conflict in the one file two people are most likely to add at once. `/specify --branch` numbers the second one from a branch identifier instead — `1234.1-slug` — which cannot collide, so the merge is clean by construction. `/fold` is how you discharge it afterwards: run it on the upstream branch *after* the merge, and the staging spec's content moves into the spec it was standing in for (as a body edit or as a scenario), every inbound pointer is re-pointed, and the directory is retired. A branch-scoped spec is **retired, not completed** — `/status` reports one as pending, and the `done` gate blocks while the fold is outstanding. Two specs. |
| `/supersede` | **A later spec countered an earlier one, and nothing records that.** Without a marker, a reader — human or agent — cannot tell a live decision from one that was overturned, and the reflex is to delete the stale spec, which strands every pointer into it. This writes the `supersedes:` key on the newer spec and a reciprocal annotation on the older one naming what no longer holds. The earlier spec **stays**, annotated, as the record of what shipped. Use `/specify --supersedes` instead when you are writing the countering spec right now. Two specs. |

`/fold` does remove a directory, but only after moving its content into the spec it was standing in for — nothing is lost, which is why it sits here rather than in the table below.

### Destructive — these remove content

Two commands delete rather than rewrite. Both confirm before they act, neither writes a backup, and **git history is the only recovery** — so they are separated here rather than sitting as ordinary rows among commands that only ever add.

| Command | What it destroys | When you reach for it |
| --- | --- | --- |
| `/prune` | Task sections in one `tasks.md` | **`tasks.md` has turned into a changelog of finished work and you cannot see what is left.** Drops spent (fully checked) task sections, or `--reset` back to template state. Scoped to a single artifact the framework classes as ephemeral work-tracking: the durable record of what was done lives in the spec, the code, and git. One spec. |
| `/consolidate` | An entire spec directory — `spec.md`, scenarios, plan, tasks, review | **An old spec was never really a separate concern** — it overlapped a sibling from the start, or duplicates one you have since written. Re-points every inbound pointer at the target first, so nothing is left dangling, then removes the source. Two specs. |

> **`/consolidate` is the only command that removes a durable artifact.** `tasks.md` is ephemeral work-tracking; `spec.md` is a source of truth. Consolidation **migrates nothing** — the guard proves the target exists, never that anything actually landed there — so the confirmation names the content you are losing, scenario by scenario, rather than merely naming the directory. Reach for `/supersede` instead whenever the earlier spec actually delivered something: that spec is the record of what shipped, and consolidating it would invert the relationship.

### Brownfield — absorb existing reality

| Command | When you reach for it |
| --- | --- |
| `/log` | **You noticed something mid-task and do not want to derail to deal with it.** Drops a raw one-liner into `specs/inbox.md` and gets out of the way. No triage, no routing, no decision — that is `/groom`'s job, later. |
| `/groom` | **The inbox has accumulated and you are ready to decide where each item belongs.** Walks it one item at a time and routes each to its real home: a rule for a cross-cutting concern, a new spec, a scenario under an existing spec, a chore left alone, or a discard. Confirms each route before it writes, and reopens a `done` spec when an item lands under it. |

### Orient

| Command | When you reach for it |
| --- | --- |
| `/target` | **You are about to work on a different feature.** Sets the working feature (or `feature/scenario`) for the session, so the other commands stop needing an argument. |
| `/status` | **"Where is everything?"** A dashboard of every feature's progress, or a focused view of the current target — including which specs are blocked, which have unresolved scenario questions, and which carry a pending fold. |
| `/link` | **A spec in this repo relates to a spec in another service's repo.** Registers that service in `.ductus/config.toml` so a cross-service reference resolves to the linked spec's real status instead of a dead link. `--list` shows registered services and their resolution health. |
| `/help` | **You forgot which command does what.** Project overview and command reference, generated from the installed commands rather than hand-maintained. |

### Bootstrap — one-time per project

| Command | When you reach for it |
| --- | --- |
| `/ductus` | **Adopting the framework, or pulling the latest version of it.** The installer that placed every other command. Idempotent — safe to re-run any time. |
| `/configure` | **Your agent keeps asking permission for the same `ductus` operations.** Configures agent permissions for the `ductus` commands so the pipeline stops prompting on every step. |

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

Auggie needs a one-time manual MCP registration (`auggie mcp add ductus …`) — see [Registering the runtime](#registering-the-runtime).

### Antigravity

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/stonean/ductus/main/install.sh | sh -s -- antigravity
```

Then run `/ductus {project-name}`. The installer creates the right directory for your agent and drops the bootstrap command in place — for Antigravity it's wrapped as a skill under `.agents/skills/ductus/`, since Antigravity discovers dir-form skills rather than verbatim command files. It's safe to re-run. (`agy`, the Antigravity CLI command name, works in place of `antigravity`.)

### OpenCode

```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/stonean/ductus/main/install.sh | sh -s -- opencode
```

OpenCode installs the bootstrap as a verbatim command at `.opencode/command/ductus.md` (invoked `/ductus`) and reads `AGENTS.md` natively — no `CLAUDE.md`. `/ductus` wires the `ductus` runtime automatically by writing the project's root `opencode.json`; because OpenCode loads config once at startup, restart it after the first wiring (see [Registering the runtime](#registering-the-runtime)).

The same bootstrap supports every agent, so re-run `/ductus --add-agent` from any adopted agent later to add others. `/ductus` acquires the runtime and wires it in the same run — automatically for Claude and OpenCode (both keep MCP config in a committed repo file), or by surfacing a one-time registration step for Auggie and Antigravity (see [Registering the runtime](#registering-the-runtime)).

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
- **`[services]`** — register sibling services so cross-service reference links resolve to the linked spec's lifecycle status (see [Cross-service references](#cross-service-references)). Add entries with `/link`, not by hand.

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

## Security rules

`ductus` ships enforceable security rules using RFC 2119 language — **MUST/MUST NOT** are blocking, **SHOULD/SHOULD NOT** are advisory. `/review` loads the rule files for your configured `[rules] surfaces` — or, when that setting is unset, the rule files that match your detected stack.

- [framework/rules/security-backend.md](framework/rules/security-backend.md) — auth, input validation, data protection, API security, logging, dependencies, error handling
- [framework/rules/security-frontend.md](framework/rules/security-frontend.md) — XSS, CSRF, secure storage, auth handling, content security, dependencies

When a MUST violation is intentional, record a waiver instead of silencing the gate:

```bash
/review --waive <rule-id> --reason "<text>"
```

Waivers are anchored to the rule ID and file path — if the file is renamed or the rule stops firing there, the waiver expires and the finding re-blocks. The waiver schema is open, so organizations can layer on their own required fields. See [specs/020-code-review/data-model.md](specs/020-code-review/data-model.md).

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
- **[docs/introduction.md](docs/introduction.md)** — the long-form pitch for spec-driven development
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
