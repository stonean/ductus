---
status: draft
dependencies: [027-bootstrap-migration-registry, 042-consolidate-govern-per-project-files-under-govern-directory, 048-govern-acquired-runtime]
review:
  last-run: null
  reviewed-against: null
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  blocking: false
next-criterion: 10
---

# 049 — Rename govern to ductus

The project takes the name **ductus**. This spec covers what that means across every surface the old name reached — the framework, the runtime binary and its crate, the per-project directory, the MCP server key and tool prefixes, the release tag scheme, and the command namespace — and how existing adopters converge without breaking.

## Motivation

The name was spread across every layer of the project and had no single source of truth. A measured count at rename time: **934 `gvrn` references across 137 files**, **794 `.govern/` path references**, and **1322 `/gov:` namespace references**. In-repo that is a mechanical sweep, and [§spec-lifecycle](../../framework/constitution.md#spec-lifecycle)'s case (a) already covers it — a uniform token substitution across live artifacts, which does not reopen `done` specs.

What made this more than a find-and-replace is that parts of the name were **already published and cannot be recalled**:

- The `gvrn` crate was published to crates.io. A published crate name cannot be reused or transferred to a differently-named crate.
- 47 `gvrn-v*` release tags exist, each with attached binaries adopters may still be downloading.
- Every adopter's checkout carried the name in files `govern` does not own — their MCP server registration, their agent permission entries (`mcp__gvrn__*` and the per-agent equivalents), their committed `.govern/config.toml`, and their installed command files.

So the rename divided into three populations with different rules: **live artifacts**, which are swept; **published history**, which stays as written because rewriting it would invalidate what adopters already hold; and **adopter state**, which converges through the migration registry ([027](../027-bootstrap-migration-registry/spec.md)) on the next bootstrap run.

Timing was the other constraint. [048](../048-govern-acquired-runtime/spec.md) was mid-implementation and its remaining phases define the runtime's install path, the per-project pointer, and an adopter migration rewriting every adopter's MCP config to those paths. Completing 048 first would have migrated adopters twice in close succession — once onto the old-name paths, once onto the new — and would have changed the release tag scheme one release after tagging that work. 048 was paused after its Phase 1, which is name-independent and already landed.

## What the name touches

Four classes, each with a different disposition.

**Swept** — live artifacts under `framework/`, `scripts/`, `runtime/src/`, `.github/`, `README.md`, `AGENTS.md`, and `specs/NNN-*/`, per the `AGENTS.md` rename rule's scope. Uniform substitution, one change, no back-edges.

**Migrated** — adopter-held state the sweep cannot reach: the MCP server key and command, the agent permission entries naming the old tool prefixes, the per-project directory, and the installed command files under the old namespace.

**Left as written** — published history. Release tags, their attached assets, CHANGELOG entries describing past releases, and commit messages are the record of what was named when. `AGENTS.md` already states this rule for renames: *"Commit messages and published PR/release notes stay as written (git history is the audit trail of what was named when)."*

**Superseded** — the crates.io crate, which cannot be renamed in place.

## Adopter migration

A registry entry ([027](../027-bootstrap-migration-registry/spec.md)) converges an adopter in a single bootstrap run, and is idempotent on re-run. The migration's scope is every piece of adopter state naming the old project: the MCP registration, the permission entries, the per-project directory and its contents, and the installed command files.

The ordering constraint is that an adopter running the migration is, by definition, running a command whose own name may be changing. The migration must therefore be reachable from the adopter's *current* installation and leave them with a working *new* one — a bootstrap that renames its own entry point mid-run.

## Sequencing with 048

[048](../048-govern-acquired-runtime/spec.md) resumes after this spec lands, and writes the final paths from the start. Its Phase 1 — the version pin, the release publish gate, and the single archive format — is independent of the name and is already shipped.

The relationship runs the other way too: 048's remaining migration entry and this spec's migration entry both rewrite an adopter's MCP configuration. If both ship, they must compose — or 048's is authored already knowing the new names, and there is only one.

## Acceptance Criteria

- [ ] AC1: No live artifact under `framework/`, `scripts/`, `runtime/src/`, `.github/`, `README.md`, or `AGENTS.md` references the old project name, except where it is recording history
- [ ] AC2: The in-repo sweep is a uniform token substitution, so `done` specs it touches stay `done` per §spec-lifecycle case (a), and `/gov:analyze` reports no spec drifted by it
- [ ] AC3: A registry migration converges an adopter's MCP registration, permission entries, per-project directory, and installed command files in one bootstrap run
- [ ] AC4: Re-running that migration against an already-converged project is a no-op
- [ ] AC5: An adopter who has not yet re-run the bootstrap is not silently broken: either their existing installation keeps working, or they are told what to run, with the message naming the command
- [ ] AC6: Published release tags, their attached assets, and CHANGELOG entries describing past releases are left exactly as written
- [ ] AC7: The self-audit families that assert installer, registry, namespace, and host-namespace parity pass under the new name with no family disabled or exempted
- [ ] AC8: The runtime's own test suite, the parity goldens, and the generated command copies are consistent with the new name, with goldens re-blessed rather than hand-edited
- [ ] AC9: `README.md` and the bootstrap describe acquiring, registering, and invoking the project under the new name only

## Open Questions

- **What is the binary called?** `gvrn` was a contraction chosen for typing — it is typed in every MCP registration, every `exec` invocation, and every hook. `ductus` is longer; a contraction (`duct`, `dts`) trades readability for keystrokes. This also determines the store filename that [048](../048-govern-acquired-runtime/spec.md) installs.
- **Does the per-project directory become `.ductus/`?** 794 references, and it is *committed adopter state* — `config.toml`, `scripts/`, and the gitignored session file live there ([042](../042-consolidate-govern-per-project-files-under-govern-directory/spec.md)). Renaming it means every adopter's repository takes a directory move in the migration; keeping `.govern/` leaves the old name permanently visible in every adopting project.
- **What is the default command namespace?** `/gov:` today, 1322 references. The namespace is already per-project configurable through `.govern/config.toml` `[host] project`, so this decides the *default* and what the framework's own repo uses — but changing it renames every installed command file for every adopter.
- **What happens to the crates.io crate?** A published name cannot be reused. Publishing `ductus` as a new crate and deprecating `gvrn` is the mechanical answer; whether the old crate is yanked, left with a deprecation notice, or left untouched needs a decision, as does whether the version series continues or restarts.
- **What happens to the release tag scheme?** `gvrn-v{version}` is what the release workflow triggers on and what [048](../048-govern-acquired-runtime/spec.md)'s acquisition constructs asset URLs from. Whether new releases tag `ductus-v{version}`, and whether the 47 historical tags are left untouched (they hold assets adopters may still fetch), needs resolving together with the crate question.
- **Do the MCP server key and tool prefixes change?** `gvrn` is the server key; `mcp__gvrn__*` and `mcp:gvrn:*` are the tool prefixes every agent's permission entries name, and `scripts/audit/host-namespace-parity.sh` audits them. Renaming breaks every adopter's permission grant until the migration runs; not renaming leaves the old name in the one surface an adopter sees on every tool call.
- **Is the GitHub repository renamed?** The bootstrap fetches its archive from a hardcoded `codeload.github.com` URL under the current repository path, and every adopter's install pulls from it. GitHub redirects renamed repositories, so this may be safe — but the redirect behavior for the `codeload` archive endpoint specifically, and whether the bootstrap should be updated in the same change, needs confirming rather than assuming.
- **Are `done` spec bodies swept, or only non-spec live artifacts?** The `AGENTS.md` rename rule includes `specs/NNN-*/` bodies in the sweep scope, which keeps them accurate as living documents. That is a large diff touching nearly every spec, and its uniformity is what keeps those specs `done` — so the question is whether any spec's reference to the old name is *historical* (recording what a thing was called at the time) rather than current, and therefore must survive the sweep.
