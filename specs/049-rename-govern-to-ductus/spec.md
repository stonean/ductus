---
status: in-progress
dependencies: [022-deterministic-runtime, 027-bootstrap-migration-registry, 042-consolidate-govern-per-project-files-under-govern-directory, 048-govern-acquired-runtime]
review:
  last-run: null
  reviewed-against: null
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  blocking: false
next-criterion: 14
---

# 049 — Rename govern to ductus

<!-- audit:ignore-introducing-drift:file -->
<!-- This spec is the *introducing* spec for the rename catalogued in
     scripts/audit/introducing-drift.sh: the retired names are the subject of
     its prose, not residual drift. It is also excluded from the rename sweep
     for the same reason — a spec that records a transition has to be able to
     name both sides of it. -->

The project takes the name **ductus**. This spec covers what that means across every surface the old name reached — the framework, the runtime binary and its crate, the per-project directory, the MCP server key and tool prefixes, the release tag scheme, and the command namespace — and how existing adopters converge without breaking.

## Motivation

The name was spread across every layer of the project and had no single source of truth. A measured count at rename time: **934 `gvrn` references across 137 files**, **794 `.govern/` path references**, and **1322 `/gov:` namespace references**. In-repo that is a mechanical sweep, and [§spec-lifecycle](../../framework/constitution.md#spec-lifecycle)'s case (a) already covers it — a uniform token substitution across live artifacts, which does not reopen `done` specs.

What made this more than a find-and-replace is that parts of the name were **already published and cannot be recalled**:

- The `gvrn` crate was published to crates.io. A published crate name cannot be reused or transferred to a differently-named crate.
- 47 `gvrn-v*` release tags exist, each with attached binaries adopters may still be downloading.
- Every adopter's checkout carried the name in files the project does not own — their MCP server registration, their agent permission entries (`mcp__gvrn__*` and the per-agent equivalents), their committed `.govern/config.toml`, and their installed command files.

So the rename divided into three populations with different rules: **live artifacts**, which are swept; **published history**, which stays as written because rewriting it would invalidate what adopters already hold; and **adopter state**, which converges through the migration registry ([027](../027-bootstrap-migration-registry/spec.md)) on the next bootstrap run.

Timing was the other constraint. [048](../048-govern-acquired-runtime/spec.md) was mid-implementation and its remaining phases define the runtime's install path, the per-project pointer, and an adopter migration rewriting every adopter's MCP config to those paths. Completing 048 first would have migrated adopters twice in close succession — once onto the old-name paths, once onto the new — and would have changed the release tag scheme one release after tagging that work. 048 was paused after its Phase 1, which is name-independent and already landed.

## What the name touches

Four classes, each with a different disposition.

**Swept** — live artifacts under `framework/`, `scripts/`, `runtime/src/`, `.github/`, `README.md`, `AGENTS.md`, and `specs/NNN-*/`, per the `AGENTS.md` rename rule's scope. Uniform substitution, one change, no back-edges.

**Migrated** — adopter-held state the sweep cannot reach: the MCP server key and command, the agent permission entries naming the old tool prefixes, the per-project directory, and the installed command files under the old namespace.

**Left as written** — published history. Release tags, their attached assets, CHANGELOG entries describing past releases, and commit messages are the record of what was named when. `AGENTS.md` already states this rule for renames: *"Commit messages and published PR/release notes stay as written (git history is the audit trail of what was named when)."*

**Superseded** — the crates.io crate, which cannot be renamed in place.

**Contributor-local** — state keyed to the old *path* rather than to the old name, which no in-repo migration can reach because it is not in the repo and differs per contributor: the local checkout directory, the git remote URL, and any per-project agent state an AI CLI keys by working-directory path (Claude Code stores per-project memory under a slug derived from the absolute path, and that memory may itself contain absolute references). Renaming the GitHub repository and the local checkout orphans all of it silently — nothing errors, the state is simply no longer found. This population is documented rather than automated: it is per-user, per-machine, and outside any artifact the project ships.

The maintainer checklist for a local checkout, since none of it is reachable by the adopter migration:

| Item | Action |
| --- | --- |
| the local checkout directory | rename it to match the repository |
| `git remote` | repoint to the new URL — the redirect works, but leaving it stale keeps the retired name live in daily use |
| per-project agent memory | an AI CLI that stores memory under a path-derived slug (Claude Code: `~/.claude/projects/<path-slug>/memory/`) will not find it after the move — copy the directory to the new slug, and correct any absolute paths recorded *inside* it |
| shell aliases, editor workspaces, worktrees | anything else keyed to the old path |

## Adopter migration

A registry entry ([027](../027-bootstrap-migration-registry/spec.md)) converges an adopter in a single bootstrap run, and is idempotent on re-run. The migration's scope is every piece of adopter state naming the old project: the MCP registration, the permission entries, the per-project directory and its contents, and the installed command files.

The ordering constraint is that an adopter running the migration is, by definition, running a command whose own name may be changing. The migration must therefore be reachable from the adopter's *current* installation and leave them with a working *new* one — a bootstrap that renames its own entry point mid-run.

## Sequencing and current state

**State at hand-off (2026-08-15).** All eight open questions are resolved and recorded below; the spec sits at `draft` with zero open questions, so the next pipeline action is `/{project}:clarify`, which short-circuits the question loop and goes straight to its validation gate and the `draft → clarified` confirmation. No decision recorded here should be reopened.

The work sequence, and why it is in this order:

1. **049 (this spec)** — clarify gate, then plan, then implement. The rename lands before 048 resumes.
2. **048 resumes** at its Phase 2. Its Phase 1 (the repo-root `version` pin, the release publish gate, one archive format on every platform) is name-independent and already shipped. Phases 2 and 3 write the store path, the pointer path, and an adopter migration to them, which is why they wait for the names.
3. **013's migration registry entry**, whose `introduced_in` names the release that first carries the labelling primitive.
4. **The release** — version bump, CHANGELOG, and the first `ductus-v{version}` tag, which closes out 013 and 022.

The completion bar for that tag is the operator's, recorded in [022's task 86](../022-deterministic-runtime/tasks.md): every identified piece of work ships first, **including all MUST and SHOULD findings from `/{project}:review` and every issue `/{project}:analyze` reports** across the affected specs. A finding deferred past the tag is not deferred, it is shipped.

## Sequencing with 048

[048](../048-govern-acquired-runtime/spec.md) resumes after this spec lands, and writes the final paths from the start. Its Phase 1 — the version pin, the release publish gate, and the single archive format — is independent of the name and is already shipped.

The relationship runs the other way too: 048's remaining migration entry and this spec's migration entry both rewrite an adopter's MCP configuration. If both ship, they must compose — or 048's is authored already knowing the new names, and there is only one.

## Acceptance Criteria

- [ ] AC1: No live artifact under `framework/`, `scripts/`, `runtime/src/`, `.github/`, `README.md`, or `AGENTS.md` references the old project name, except where it is recording history
- [ ] AC2: The in-repo sweep is a uniform token substitution, so `done` specs it touches stay `done` per §spec-lifecycle case (a), and `/gov:analyze` reports no spec drifted by it
- [ ] AC3: A registry migration converges an adopter's MCP registration, permission entries, per-project directory, and installed command files in one bootstrap run
- [ ] AC4: Re-running that migration against an already-converged project is a no-op
- [ ] AC11: An adopter converges on the new per-project directory from any prior layout — pre-[042](../042-consolidate-govern-per-project-files-under-govern-directory/spec.md) legacy, consolidated under the old directory, or already converged — with no tracked file lost, `[pinned] files` entries rewritten to the new paths, and the two directory migrations composing in registry order
- [ ] AC5: An adopter who has not yet re-run the bootstrap is not silently broken: either their existing installation keeps working, or they are told what to run, with the message naming the command
- [ ] AC6: Published release tags, their attached assets, and CHANGELOG entries describing past releases are left exactly as written
- [ ] AC12: The retired crate is left installable rather than yanked, and its final published release describes the new name
- [ ] AC13: The first release under the new name continues the existing version series rather than restarting it, and the version-agreement audit family passes across the `version` pin, `runtime/Cargo.toml`, and the newest `runtime/CHANGELOG.md` heading
- [ ] AC7: The self-audit families that assert installer, registry, namespace, and host-namespace parity pass under the new name with no family disabled or exempted
- [ ] AC8: The runtime's own test suite, the parity goldens, and the generated command copies are consistent with the new name, with goldens re-blessed rather than hand-edited
- [ ] AC9: `README.md` and the bootstrap describe acquiring, registering, and invoking the project under the new name only
- [ ] AC10: The contributor-local checklist is documented for a maintainer renaming their own checkout — the local directory, the git remote, and per-project agent state keyed by path — since none of it is reachable by the adopter migration

## Open Questions

*None — all resolved.*

## Resolved Questions

**Are `done` spec bodies swept, or only non-spec live artifacts?**

Spec bodies are swept, with one exception: an occurrence that names a **published artifact** — a released version, a release tag, or a release asset — survives.

The exception is what makes this mechanical rather than a judgment call, and it is measurable: of 210 `gvrn` occurrences in spec bodies, 69 sit adjacent to a version or tag (`gvrn-v0.23.0`, `gvrn 0.24.0`, `introduced_in = "0.22.0"`). Those name things that exist under that name. Rewriting them would make the artifacts lie — `framework/migrations/govern-dir-consolidate.md` records *"Introduced in: gvrn 0.22.0"*, and no `ductus 0.22.0` was ever published, so an adopter reading it would be told they installed a release that does not exist.

Everything else in a spec body describes the project as it *is*, and specs are living documents representing current state. Leaving them would have 50 specs describing a project by a name it no longer has.

The rule is therefore: sweep every occurrence except those naming a published version, tag, or asset. Because the exception is a determinable class rather than a per-occurrence decision, every change in the resulting diff is still the same substitution — which is what [§spec-lifecycle](../../framework/constitution.md#spec-lifecycle) case (a) requires, so the `done` specs the sweep touches stay `done`.

**What happens to the crates.io crate, and does the version series continue?**

`ductus` is published as a new crate — a published name cannot be reused or transferred — and it **continues the existing version series** rather than restarting.

Continuity is the load-bearing half. The CHANGELOG, the release tags, and the `version` pin [048](../048-govern-acquired-runtime/spec.md) introduces all describe one piece of software with one history; restarting at `0.1.0` would put a discontinuity in the middle of that record, and 048's version-agreement family compares the pin against `Cargo.toml` and the newest CHANGELOG heading with no way to express "the series restarted here". The rename changes what the software is called, not what it is.

The `gvrn` crate is **left installable**, with a final release whose description points at the new name. It is a binary crate, so its users are people who ran `cargo install` — yanking would break them, including any pinned CI, and a rename is not a reason to break an existing install. A deprecation notice reaches the same people at the moment they next look, which is what a rename actually owes them.

**What happens to the release tag scheme?**

New releases tag `ductus-v{version}`; the 47 existing `gvrn-v*` tags are left exactly as they are.

This follows from the binary name and the continuity decision above, and needs no separate judgment. The historical tags hold published assets that adopters on older versions may still be fetching — they belong to the *published history* population, which is left as written. The release workflow's trigger pattern is updated to the new scheme in the sweep, so the first release under the new name is the first `ductus-v*` tag, continuing the same version series.

**Do the MCP server key and tool prefixes change?**

Yes — `ductus` is the server key, producing `mcp__ductus__<verb>-<noun>` on Claude Code and `mcp:ductus:<verb>-<noun>` on Auggie and Antigravity.

No separate decision was needed: the server key is the binary name, resolved above. What the rename requires is that the migration rewrite each agent's **permission entries**, which name the prefix directly and are the reason an adopter's tool calls stop prompting. Those entries live in per-agent settings files in four different grammars, and `scripts/audit/host-namespace-parity.sh` audits the result, so the sweep is checkable rather than trusted.

The bare primitive names in `framework/runtime-tools.txt` do not change. They never carried the project name — namespacing is supplied by each host's MCP registration, which is exactly why the tool-coverage lint is unaffected by any of this.

**Is the GitHub repository renamed?**

Yes, and the redirect is a transition mechanism rather than a permanent one.

GitHub's rename behavior, confirmed against its documentation rather than assumed: web traffic and every `git clone` / `fetch` / `push` against the old location continue to work — **but only for as long as the old name is not reused**, and calls to Actions hosted in a renamed repository are explicitly *not* redirected.

The Actions caveat does not reach adopters here. The only workflow this project ships is `framework/templates/ci/adopter-generators.yml`, which uses `actions/checkout` and nothing hosted in this repository.

Three adopter-facing URLs carry the repository path, and all three are updated in the sweep: the archive fetch (`codeload.github.com/…/tar.gz/refs/heads/main`), the bootstrap's self-update fetch (`raw.githubusercontent.com/…/framework/bootstrap/govern.md`), and the documentation links in the post-scaffolding output.

The self-update fetch is what makes the cutover self-completing. An adopter still holding the old bootstrap fetches it through the redirect, receives the *new* bootstrap carrying the new URLs, and the self-update check writes it — so each adopter converges on their next run, and the redirect only has to survive until then. This is the same shape as the runtime's new-location-first read with a legacy fallback: the old path stays live exactly long enough to hand over.

One operational constraint follows and is permanent: **a repository named `govern` must never be created under this account again.** Doing so silently severs the redirect for every adopter who has not yet re-bootstrapped — no error on this side, no signal, and the failure appears only in their next bootstrap. It is the one failure mode in this rename with no detection.

**What is the default command namespace?**

`/ductus:` — one name across the project, the binary, the crate, the MCP server key, the release tag prefix, and the commands.

A concern was raised against this and does not survive contact with the code. [022](../022-deterministic-runtime/spec.md)'s naming decision justifies the `gvrn:` / `gov:` split partly on giving `scripts/lint-tool-coverage.sh` "an unambiguous string to match against in prose", which reads as though collapsing the two names would break that lint. It would not: the lint reads bare `<verb>-<noun>` names from `framework/runtime-tools.txt` and greps for those literal strings — it never matches a namespace prefix. `runtime-tools.txt` states the same thing from the other side: *"Names are bare `<verb>-<noun>` strings — server-level namespacing is supplied by each host's MCP server registration."*

The discriminator between primitives and slash commands has therefore always been an **explicit registry**, not a naming convention. That is a stronger arrangement than the one 022 described, and it is unaffected by the rename: a name in `runtime-tools.txt` is a primitive, and everything else in the namespace is a command, regardless of what either is prefixed with.

The namespace stays per-project configurable through `[host] project`; this decides the default and what this repository uses. Every adopter's installed command files are renamed by the migration.

**Does the per-project directory become `.ductus/`?**

Yes. Everything changes name, the GitHub repository included, so a directory named after what the project used to be called would be the one piece of the old name left sitting in every adopting repository's root — in `config.toml`, the one file in the layout an adopter actually opens.

The move follows the pattern [042](../042-consolidate-govern-per-project-files-under-govern-directory/spec.md) already proved, including the property that makes it safe: **the runtime reads the new location first and falls back to the legacy one**, so an adopter who upgrades the binary before re-running the bootstrap is never broken, and the migration completes the cutover rather than performing it. 042's `govern-dir-consolidate` body supplies the rest of the shape — a convergence rule when the destination already exists, `git mv` for tracked files so the rename is recorded, rewriting `[pinned] files` entries to the new paths, and a warning for a pinned invoker still referencing the old one.

The honest cost is that this is adopters' **second** directory move; 042 performed the first. It is folded into the rename they are already taking rather than deferred, because deferring it makes three moves out of two.

The question also surfaced a population the spec had missed. The four classes above cover state named by the old *name*; renaming the repository and the local checkout also orphans state keyed by the old *path* — a contributor's checkout directory, their git remote, and per-project agent memory an AI CLI stores under a path-derived slug. None of it errors when it goes missing; it is simply never found again. It is per-user and per-machine, so it is documented as a maintainer checklist rather than automated.

**What is the binary called?**

`ductus` — the same token as the crate, the release tag prefix (`ductus-v{version}`), and the MCP server key. No contraction.

The question assumed `gvrn` was a contraction chosen for keystrokes. It was not. [022](../022-deterministic-runtime/spec.md)'s naming decision records the actual reason: the `gvrn:` prefix **disambiguates MCP primitives from slash commands** (`gov:<command>`), and `lint-tool-coverage.sh` depends on those being distinct strings to match in prose — `gov:` was considered for the tools and rejected for colliding. The project has two names because two *namespaces* needed separating, not because one needed shortening.

That reframes the choice. Keystroke economy buys very little here: a human types the binary name when installing, once. Every other occurrence — the MCP registration, the hooks, the `exec` invocations, the pre-flight probe — is written by the bootstrap or driven by the agent, and a name that reads clearly in a config file is worth more than one that is quick to type.

So the disambiguation 022 requires is preserved, but relocated: it now lives between the MCP prefix (`ductus`) and the command namespace, which is resolved separately below. Both remain distinct, greppable strings; they are simply derived from one project name rather than from two unrelated ones.

This also fixes the store filename [048](../048-govern-acquired-runtime/spec.md) installs, which was blocked on this answer.
