---
status: planned
dependencies: [021-runtime-boundary, 027-bootstrap-migration-registry, 029-bootstrap-runtime-autowire, 042-consolidate-govern-per-project-files-under-govern-directory]
review:
  last-run: null
  reviewed-against: null
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  blocking: false
next-criterion: 25
---

# 048 — Govern-Acquired Runtime

`/govern` acquires the `gvrn` runtime itself — downloading the platform-appropriate release binary into a govern-owned store at `~/.govern/bin/`, exposing it to each project through a gitignored `.govern/bin/gvrn` pointer, and registering that with the agent's MCP server. The runtime stops being an out-of-band install the adopter places on `PATH` and becomes an artifact `/govern` owns, acquires, and keeps current.

Automating acquisition is what makes it possible to **require** the runtime, and this spec takes that step. The two changes are one change: the runtime could not be mandatory while obtaining it was the reader's problem, and it need not stay optional once the pipeline installs it.

## Motivation

The runtime was an out-of-band install. The adopter downloaded a release archive by hand, verified its checksum, and copied `gvrn` onto their `PATH`; only afterward did `/govern` detect the binary and wire the MCP registration ([029-bootstrap-runtime-autowire](../029-bootstrap-runtime-autowire/spec.md)). `/govern` automated the *registration* and left the *acquisition* to the reader. Three costs followed from that split:

- **Adoption had a manual step outside the pipeline.** The one command that exists to make a project ready could not make the runtime available; it could only notice that someone else already had. The pipeline's own advice — install `gvrn`, it cuts token use — pointed out of the pipeline.
- **`PATH` decided which build answered.** The registered MCP entry named the bare command (`{"command": "gvrn"}`), so the binary that served a session was whichever one the agent's inherited shell environment resolved first. Two projects on one machine could not run different runtime versions, and a stale binary earlier on `PATH` silently shadowed a newer one.
- **Nothing bound the binary's version to the framework's.** [§runtime-boundary](../../framework/constitution.md#runtime-boundary) requires the runtime to ship in lockstep with the framework — "an adopter's `govern` version pins their compatible runtime version, eliminating schema/runtime drift as a failure mode" — but no mechanism enforced the pin. The framework arrived from a `/govern` archive fetch; the runtime arrived from whenever the adopter last ran `install`.

Making the binary govern's to acquire puts acquisition, version pinning, and registration in the hands of the single command that already owns every other piece of project setup.

A fourth cost is paid not by adopters but by the framework itself, and it is the reason acquisition is worth more than convenience: **every deterministic rule has to exist twice.** While a markdown-only adopter must be able to execute the pipeline, each check family, grammar, severity tier, and skip rule must be stated as prose an LLM can follow *and* implemented in the runtime, with a standing obligation to keep the two in agreement. That obligation is not theoretical — the `ships-to-adopter` skip reason lived in the runtime for a full release with no counterpart in `analyze.md`'s reference, so a host walking the documented path would have reported findings the runtime suppressed, and nothing caught it.

## Runtime requirement

The runtime is **required**. A project bootstrapped by `/govern` has the binary, so pipeline commands may assume determinism instead of specifying two ways to reach the same result.

This amends [§runtime-boundary](../../framework/constitution.md#runtime-boundary): principle 3 ("Opt-in for adopters — the runtime MUST NOT be a prerequisite for any pipeline gate") and the **Opt-in invariant** (the CI job asserting a full cycle with the binary absent from `PATH`) are replaced by the requirement and by a job that asserts acquisition instead. It narrows [§text-first-artifacts](../../framework/constitution.md#text-first-artifacts)'s "usable standalone with no tooling beyond the AI agent" to the artifacts rather than the pipeline.

Two things this deliberately does **not** change:

- **Artifacts stay markdown, and stay editable by hand.** Text-first governs the *artifacts*, not the tooling that reads them. The runtime parses and writes the same markdown a contributor edits in an editor, which is why the pre-commit hook exists as a backstop at all.
- **The markdown-only reference sections stay.** They are where policy is specified — `analyze.md`'s check families and their severity tiers, `review.md`'s procedure — and a primitive that "mirrors the reference, introducing no policy of its own" needs that reference to exist. What ends is the obligation to keep two *executable* paths in agreement; what remains is one specification with one implementation.

The requirement is what the amendment buys, and the amendment is the cost. Both belong to this spec: a change that makes the runtime mandatory without amending the principle forbidding it would be, in the constitution's own words, "a constitution violation, not a feature".

## Install location

The runtime is acquired **once per machine** into a govern-owned store, and reached from each project through a pointer.

- **The store** is `~/.govern/bin/gvrn` (`~/.govern/bin/gvrn.exe` on Windows): in the user's home directory, written only by `/govern`, and never placed on `PATH`. One download serves every govern project on the machine. It is govern-owned, which is the distinction this spec draws — not project-local versus global, but *acquired and version-managed by the pipeline* versus *whatever the shell resolves first*.
- **The pointer** is `.govern/bin/gvrn` in each project, resolving to the store. It exists for one reason: a committed MCP config must not name a machine-specific absolute path. `.mcp.json` and `opencode.json` are shared with the whole team, so `/Users/alice/.govern/bin/gvrn` in either breaks every other contributor and every CI checkout. A repo-relative pointer resolves correctly in all of them. How the pointer is materialized — symlink or copy — is a plan decision, constrained to require **no elevated privileges on any supported platform** (creating a symlink on Windows needs developer mode or elevation).
- **Nothing binary enters the repository.** The store lives outside it entirely, and the pointer is gitignored — [§text-first-artifacts](../../framework/constitution.md#text-first-artifacts) requires binary artifacts to be gitignored and regenerated on demand by their consumers. The framework-managed `.gitignore` block gains the `.govern/bin/` entry; the existing anchored `/.govern/session.toml` line establishes the pattern of ignoring specific paths under `.govern/` so committed `config.toml` and `scripts/` stay tracked. The pointer sits under the per-project `.govern/` directory established by [042-consolidate-govern-per-project-files-under-govern-directory](../042-consolidate-govern-per-project-files-under-govern-directory/spec.md), alongside the config and session files it already holds.
- **Per-contributor by construction**: each contributor's `/govern` run acquires the build for their own platform into their own home directory, and a teammate on a different OS is unaffected. This matches how `cli-config-dir` is already handled — per-contributor state stays out of committed config.
- The store file is written with the executable bit set.

### Version currency, and what the store gives up

`/govern` compares the project's pinned `version` against the installed binary on every run and re-acquires on mismatch, overwriting the store. The binary therefore matches the project the adopter most recently bootstrapped.

**The pin is project-scoped; the store is not.** A machine running two govern projects pinned to different versions holds exactly one binary — whichever the most recent `/govern` run installed — so a session opened in the *other* project runs a runtime its framework revision did not pin, until `/govern` runs there. This is the one guarantee a machine-global store gives up, and it is a deliberate trade for a design that is uniform across all four agents, downloads once per machine, and needs no per-project binary. It is also strictly better than the status quo it replaces: the same exposure exists on `PATH` today, except nothing owns the path and nothing corrects it, whereas here every `/govern` run does.

## Acquisition

`/govern` downloads the release asset matching the host platform, verifies it, and installs it into the store.

- **A project may supply its own binary instead.** When `.govern/config.toml` sets the `[runtime]` path key, `/govern` performs no download and resolves the pointer to that binary; a version mismatch against the pin warns rather than halts. This is the supported route for a project that builds from source, for an air-gapped or firewalled adopter, and for a platform with no published asset. See the resolved question below.
- **The version comes from the repo-root `version` file** in the archive `/govern` has just fetched — one SemVer line carried by the same download as the `framework/` tree it describes, so the pin cannot disagree with the framework revision it arrived with. See the resolved question below.
- **Asset naming** follows what `.github/workflows/runtime-release.yml` publishes: `gvrn-{target}.tar.gz` plus a sibling `gvrn-{target}.tar.gz.sha256` on Unix, and `gvrn-{target}.zip` plus `gvrn-{target}.zip.sha256` on Windows, under `https://github.com/stonean/govern/releases/download/gvrn-v{version}/`.
- **Target triples** are the published set: `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, and `x86_64-pc-windows-msvc`. `/govern` derives the triple from the host platform and architecture.
- **Integrity is verified before install.** The sidecar `.sha256` is fetched and checked against the computed digest of the downloaded archive. A mismatch aborts the acquisition without writing anything into the store or the pointer, and the run degrades to the markdown path rather than installing an unverified binary. This is stricter than the framework archive fetch, which tolerates a missing sidecar because GitHub's auto-generated source tarballs ship without one — the runtime's release assets always carry theirs.
- **Acquisition failure halts the run, and says how to recover.** A network failure, an unpublished asset for the host platform, or a checksum mismatch aborts `/govern` with an error naming the exact store path and the release URL, so an adopter behind a firewall can place the binary by hand and re-run. There is no silent degradation: a requirement that quietly is not one would leave both execution paths alive, which is the cost §Runtime requirement exists to end. There is also no `PATH` fallback — see the resolved question below.
- **Re-runs are idempotent.** When the store already holds the pinned version, `/govern` performs no download and leaves the binary byte-unchanged. When the versions differ, the store is overwritten — the upgrade path is a routine `/govern` run.

## MCP registration

The registered server command becomes a govern-owned path instead of the bare name, and — for the first time — **the same shape works for every agent in the registry**. The shapes in `/govern`'s §MCP wiring change accordingly:

- **Claude** — `.mcp.json` at the repo root: `{"mcpServers": {"gvrn": {"command": ".govern/bin/gvrn", "args": ["mcp"]}}}`.
- **OpenCode** — the committed root `opencode.json` `mcp` block: `{"gvrn": {"type": "local", "command": [".govern/bin/gvrn", "mcp"], "enabled": true}}`.
- **Auggie** — `auggie mcp add gvrn --command {absolute-store-path} --args "mcp"`, surfaced for the user to run once per machine.
- **Antigravity** — the same absolute store path in the `~/.gemini/config/mcp_config.json` block, surfaced for the user to add, then reloaded via `/mcp`.

The split follows the config's scope, not the agent. The two `project-committed` targets name the **repo-relative pointer**: their files are shared with the team, so a machine-specific absolute path in either would break every other contributor. The two `user-global` / `home-level` targets name the **absolute store path**: their files are per-machine and never committed, so an absolute path is exactly right there — and because the store belongs to no particular project, no project is privileged by it.

This is what removes the asymmetry the home-level agents used to carry. Their config holds a single `gvrn` entry serving every project on the machine, so no project-specific path could ever be correct in it; a store owned by none of them can. The additive-merge rules are unchanged — an existing MCP config keeps its other servers, other top-level keys are preserved, and a malformed JSON config is never clobbered.

## Detection states

[029-bootstrap-runtime-autowire](../029-bootstrap-runtime-autowire/spec.md) defined three pre-flight states: **A** (runtime live this session), **B** (binary present on `PATH`, not wired), **C** (binary absent — markdown path plus a tip suggesting the adopter install it). Making acquisition `/govern`'s job collapses the distinction that produced State C: a missing binary is no longer a terminal condition to report, it is work to perform.

- The **binary probe** changes from a `PATH` lookup (`command -v gvrn` / `which gvrn`) to a filesystem check for the store, plus the pointer for this project. Its permission-seed entry in §Permission Setup changes with it.
- **State A** (a `gvrn`-namespaced MCP tool is in the session's inventory) is unchanged, including its binding execution contract.
- The former **State C** path acquires the binary, then wires and permissions it exactly as State B does, joining the same **pending-restart set** and the same single combined **Pre-flight abort**. An adopter with no runtime at all reaches the deterministic path after one `/govern` run and one restart, with no manual install step in between.
- The **State C tip** in §Post-Scaffolding Output ("installing `gvrn` cuts token use") loses its audience on the happy path and is reserved for the degraded case — acquisition was attempted and failed.

## Reference migration

Every live artifact that names `gvrn` as an **executable** is updated to the govern-owned path — the pointer for project-committed surfaces, the absolute store path for home-level ones. The set of surfaces:

- `framework/bootstrap/govern.md` — the §MCP wiring JSON shapes, the §Permission Setup probe seed, the §Detection mechanism probe, the §gvrn runtime detection state definitions, the §Post-Scaffolding tip, and the matching §Edge Cases rows.
- `README.md` — the "Install the runtime" and "Registering the runtime" sections, which currently document a manual `sudo install -m 0755 gvrn /usr/local/bin/gvrn`.
- This repository's own `.mcp.json`, subject to the framework-repo question below.

References to `gvrn` as a *name* — the crate, the MCP server key, the `gvrn-v*` release tag scheme, the `mcp__gvrn__*` tool prefix — are unaffected. Prose describing runtime behavior (`gvrn exec` resolution, primitive contracts) is likewise unaffected: those name the runtime, not a path to invoke.

Historical specs whose bodies record the pre-048 shape (028, 031, 032) are covered by the [§spec-lifecycle](../../framework/constitution.md#spec-lifecycle) mechanical-edit rule when the change is the uniform token substitution; anything requiring more than that substitution takes the normal back-edge per [§cross-spec-impact](../../framework/constitution.md#cross-spec-impact).

## Adopter migration

Existing adopters have an MCP entry naming the bare `gvrn` command and, usually, a binary on their `PATH`. A `framework/migrations.toml` entry rewrites the registered command to the govern-owned path on the next `/govern` run, following the registry contract in [027-bootstrap-migration-registry](../027-bootstrap-migration-registry/spec.md): a `[[migrations]]` row with an `introduced_in` version and a procedure body under `framework/migrations/`. The procedure is idempotent and exits silently when the target config is absent or already migrated. The `PATH`-installed binary is left alone — removing it is the adopter's call, not `/govern`'s.

## Edge Cases

- **The store holds a binary that will not execute** — a truncated download, a wrong-architecture asset, a missing system library. The version probe reports nothing, which reads as *no usable runtime* rather than *version unknown*, and `/govern` re-acquires. This is why the probe executes the binary rather than reading a recorded marker.
- **The pointer is missing or dangling** — a fresh clone (it is gitignored, so it never arrives with the checkout), or a store cleared by hand. `/govern` recreates it. A dangling pointer is not an error state to report; it is the expected state of any project nobody has bootstrapped on this machine yet.
- **Two `/govern` runs write the store concurrently**, on one machine in two projects. The store write is atomic (tempfile + rename), matching every other govern write, so a reader sees the old binary or the new one and never a partial file. The last writer wins, which is the same resolution §Version currency already describes for the sequential case.
- **A session is open while the store is replaced.** An MCP server spawned from the old binary keeps running it — the process holds its own image. The next session picks up the new one. No attempt is made to signal or restart a live server.
- **The archive carries no `version` file, or an unparseable one.** A framework revision predating this spec has no pin to read. `/govern` halts naming the file, rather than guessing a version or falling through to "latest" — a wrong pin silently installs a runtime the framework was never tested against.
- **`[runtime]` names a path that does not exist or will not execute.** Halt naming the configured path. A project that has deliberately claimed responsibility for supplying its binary gets an error about *that* choice, never a silent fallback to downloading, which would discard the choice without saying so.
- **The home directory is unwritable, absent, or on a read-only mount** — some CI containers and locked-down images. Halt with the store path and the `[runtime]` key, since supplying a binary from a writable location is exactly the escape hatch for this case.
- **A pinned version whose tag exists but whose asset for this platform does not.** Prevented upstream by the release gate, but an adopter pinned to an older partial release can still meet it. Treated as any unavailable asset: halt, name the store path and the release URL.
- **The adopter has a `gvrn` on `PATH`.** Ignored entirely — not consulted, not warned about, not removed. Removing it is the adopter's call; `/govern` simply no longer looks there.

## Acceptance Criteria

- [ ] AC1: A `/govern` run on a machine with no runtime installed downloads the host-platform release asset, verifies its `.sha256` sidecar, and writes an executable `~/.govern/bin/gvrn` (`~/.govern/bin/gvrn.exe` on Windows)
- [ ] AC2: `.govern/bin/` is present in the framework-managed `.gitignore` block, and a freshly bootstrapped project reports no untracked binary under `git status`
- [ ] AC3: A checksum mismatch leaves the store and the pointer unwritten, and halts the run with an error naming the expected and computed digests — no partial binary, no unverified binary
- [ ] AC4: A download failure (network error, missing asset for the host platform) halts the run with an error naming the store path and the release URL, so the binary can be placed by hand and the run retried
- [ ] AC5: After acquisition on a `write-file` agent, the MCP config registers the repo-relative pointer `.govern/bin/gvrn` — `.mcp.json` for Claude, the root `opencode.json` `mcp` block for OpenCode — additively, preserving every other server and top-level key; neither committed file contains a machine-specific absolute path
- [ ] AC6: A second `/govern` run with the pinned version already installed performs no download and leaves the store byte-unchanged
- [ ] AC7: A `/govern` run whose pinned version differs from the installed binary's version replaces the store and reports the upgrade
- [ ] AC13: `.govern/bin/gvrn` resolves to the store on every supported platform without requiring elevated privileges, and a project whose pointer is missing or dangling has it repaired by the next `/govern` run
- [ ] AC14: On a `surface-instruction` agent, the surfaced registration instruction names the absolute store path — not the repo-relative pointer, which their home-level config cannot resolve
- [ ] AC15: A repo-root `version` file carries one SemVer line, and it matches `runtime/Cargo.toml`, the newest `runtime/CHANGELOG.md` heading, and the newest `gvrn-v*` tag; a self-audit family asserts that agreement
- [ ] AC8: An adopter whose MCP config names the bare `gvrn` command has it rewritten to the govern-owned path by the registered migration, and re-running the migration is a no-op
- [ ] AC9: The pre-flight binary probe checks the govern-owned store rather than `PATH`, and the §Permission Setup seed grants exactly what that probe needs
- [ ] AC10: An adopter with no runtime reaches the deterministic path in one `/govern` run plus one restart — acquisition, MCP wiring, and tool permissions all land in the same pre-flight pass and surface in one combined abort
- [ ] AC11: `.github/workflows/markdown-only-pipeline.yml` — the job asserting the retired opt-in invariant — is replaced by one that exercises acquisition end-to-end on each supported platform and fails when the runtime cannot be obtained
- [ ] AC16: The constitution is amended in the same change: §runtime-boundary principle 3 and the Opt-in invariant are replaced by the requirement, §text-first-artifacts' "usable standalone" is narrowed to the artifacts, and no live artifact still describes the runtime as optional
- [ ] AC18: A project setting `.govern/config.toml`'s `[runtime]` path key gets no download, a pointer resolving to the named binary, and a warning — not a halt — when that binary's version differs from the pin
- [ ] AC19: This repository sets the `[runtime]` key to its own build output, so a maintainer's MCP calls exercise the binary they just compiled rather than the last release
- [ ] AC20: `runtime-release.yml` gates the publish on all five target assets being present, so a tag ships the complete set or fails; the build matrix keeps `fail-fast: false` so every target's failure is reported in one run
- [ ] AC21: `README.md` no longer claims the Windows binary depends on cross-compilation succeeding — it is a native `windows-latest` build, matrix-equal with the other four
- [ ] AC22: Each of the four agent permission seeds grants the full acquisition sequence in its own grammar — `mkdir`, the platform checksum tool, the pointer command, and execution of the store path — so a bootstrap completes acquisition with no permission prompt, including at checksum verification
- [ ] AC23: The Windows release asset is published as `.tar.gz`, so extraction is `tar` on every platform and `unzip` never enters any agent's permission surface
- [ ] AC17: Every per-step markdown-only fallback instruction ("With no gvrn runtime registered, …") is removed from `framework/commands/*.md` and `framework/bootstrap/govern.md`, while the Markdown-only reference sections that specify check policy and procedure remain
- [ ] AC12: No live artifact under `framework/` or `README.md` documents installing `gvrn` onto `PATH` as the supported path, and none registers a bare `gvrn` MCP command
- [ ] AC24: Per §cross-spec-impact, [021-runtime-boundary](../021-runtime-boundary/spec.md) and [029-bootstrap-runtime-autowire](../029-bootstrap-runtime-autowire/spec.md) record this change in their own bodies and reopen to `in-progress`: 021 owns the opt-in invariant and the "adopters who install nothing still complete every pipeline cycle" property that the amendment retires, and 029 owns the three detection states this spec collapses. Every other `done` spec whose body describes the runtime as optional is swept in the same change

## Open Questions

*None — all resolved.*

## Resolved Questions

**Is `--version` the right idempotency probe?**

Yes. `/govern` executes the store binary and compares its reported version against the pin.

The objection the question raised — that this executes an untrusted-until-verified binary on every run — does not survive the acquisition contract. Nothing reaches the store without matching its published digest first; a mismatch halts before anything is written. The binary being probed is therefore one this pipeline verified and installed, and executing it is pre-authorized in every agent's seed.

What settles the remaining trade is a workflow this spec deliberately creates. Hand-placing a binary into the store is the supported route in three separate cases — the firewalled or air-gapped adopter, an adopter on a platform with no published asset, and the framework's own maintainer running a local build through the `[runtime]` key. A recorded marker would be absent or stale in every one of them, so `/govern` would either re-download over a deliberate choice or trust a record describing a file that has since been replaced. Asking the binary what it is cannot disagree with the binary.

It also fails usefully. A store entry that exists but will not execute — a truncated download, a wrong-architecture asset, a missing system library — reports no version, which reads as "no usable runtime" and re-acquires. A marker file would assert the version of something that cannot run, and the failure would surface later and further from its cause.

**What new permissions does acquisition need?**

The whole acquisition sequence is pre-authorized in every agent's grammar, execution of the acquired binary included.

The surface is larger than the question assumed, for a structural reason: **acquisition cannot use `fetch-archive` and `extract-archive`, because the runtime is what is being acquired.** Every step is host shell work, expressed four times — Claude's `Bash(…)` patterns, Auggie's `launch-process` regexes, Antigravity's `command(…)` tokens, and OpenCode's `permission.bash` globs.

Measured against the current seed, `curl`, `tar`, `chmod`, `ls`, `mktemp`, `awk`, and `command -v` are already granted. Acquisition adds four:

- `mkdir` — creating the store directory.
- a **checksum tool**, which varies by platform: `shasum -a 256` on macOS, `sha256sum` on Linux, `certutil -hashfile` on Windows.
- the **pointer command** (`ln`, or `cp` where a copy is used).
- **executing the store path itself**, for the version probe and thereafter as the MCP server command.

Pre-granting the *verification* step is the security-relevant half, and it runs opposite to intuition. Leaving it unauthorized does not add a safety check — it plants a permission prompt at the exact gate that must never be waved through, and a prompt that appears every time trains the reflex to approve it. The gate that actually protects the adopter is the checksum comparison itself, which halts before anything is written, and that gate is stronger when it runs unattended than when it depends on someone reading a dialog during setup.

Executing the acquired binary is pre-granted on the same reasoning, and only because verification precedes it: nothing is executed that has not already matched its published digest. An unverified binary is never written to the store at all, so there is no state in which the execution grant applies to something unchecked.

One simplification falls out of the enumeration: publishing the Windows asset as `.tar.gz` removes `unzip` from the surface entirely, since Windows 10+ ships `bsdtar` and `tar` is granted everywhere already. Extraction becomes one command on every platform.

**Which platforms are guaranteed?**

All five published targets, enforced at release time: a tag ships the complete set of assets or ships nothing.

The question assumed the Windows asset was the weak one, on the strength of `README.md`'s claim that "a Windows binary appears when cross-compilation succeeds". That claim is wrong twice. `runtime-release.yml`'s matrix builds `x86_64-pc-windows-msvc` **natively** on `windows-latest` (`cross: false`); the only cross-compiled target is `aarch64-unknown-linux-gnu`. Windows is matrix-equal with the rest, and the README is corrected as part of this spec's reference sweep.

The real exposure is `fail-fast: false` on the build matrix combined with an ungated publish. The flag itself is right — a maintainer wants every target's failure visible in one run rather than the first one aborting the rest — but nothing downstream requires all five to have succeeded, so a release can be tagged with an asset missing and nothing says so. While the runtime was optional that produced a degraded experience for one platform. Once it is required, it locks every adopter on that platform out of the pipeline until they take the supplied-binary route, and the first they hear of it is a halt.

So the publish is gated on the complete set while the matrix keeps reporting every failure. A blocked release the maintainer can see is strictly better than a partial one adopters discover for them.

Platforms outside the published set — a new architecture, an unusual libc — are not a separate mechanism: `/govern` halts as it does for any unavailable asset, naming the store path and the `[runtime]` supplied-binary key, and the adopter builds from source. That is the same escape hatch the firewalled and framework-developer cases use.

**What does the framework's own repository do?**

Nothing special — it uses a general **supplied-binary** mode that `.govern/config.toml` exposes to any project. A `[runtime]` key names a binary the project provides; when it is set, `/govern` performs no acquisition and resolves the pointer to that path instead of the store.

The framework repository sets it to its own build output, so the runtime answering a maintainer's MCP calls is the one they just compiled. Without it the store would hold the last *release* while the source tree moves ahead of it, and every MCP tool call would silently exercise the previous version — the stale-binary trap `AGENTS.md` already records, made structural rather than incidental. That trap is not hypothetical: during this spec's own clarification every MCP call in the session ran a binary predating the `label-criteria` primitive, which is why the corpus backfill was performed through the CLI against `runtime/target/release/gvrn`.

A version mismatch between a supplied binary and the pinned `version` **warns rather than halts**. The halt exists to stop an adopter running an unknown runtime by accident; a project that names a path has stated deliberately which binary it wants, and a developer's build is expected to be ahead of the last release. Reporting it keeps the divergence visible without blocking the work it enables.

Deliberately a general mechanism rather than an exemption for this repository, because the same seam answers three otherwise-separate problems: the framework maintainer building from source, the firewalled or air-gapped adopter placing a binary by hand (see the `PATH` question above), and an adopter on a platform with no published asset (see the platform question). One documented mode, three cases; a repo-name special case in framework logic would have served only the first.

**Does a `PATH`-installed `gvrn` remain a supported fallback?**

No. `PATH` is not a supported install location, a fallback, or a degraded path — the store is the only place `/govern` looks, and acquisition failure halts the run.

The question was posed as a trade against degrading to the markdown path, but §Runtime requirement removes that alternative: there is no markdown path to degrade to. What remains is a narrower comparison — a `PATH` binary of unknown version against a hard stop — and the hard stop wins, for two reasons.

First, **a mismatched runtime can be wrong, while a stop is merely blocking.** The runtime has published 52 releases in three months, including a breaking `create-scenario` argument-shape change; pairing an arbitrary `PATH` build with a pinned framework revision risks incorrect results that look like correct ones. A halt is legible and recoverable. Silent wrongness is neither, and it is precisely the schema/runtime drift §runtime-boundary names as the failure mode to eliminate.

Second, **the escape hatch is better than the fallback it replaces.** Because the store is a fixed, known path, an adopter who is firewalled, air-gapped, or on an unpublished platform can place a binary there by hand — or build one from source — and be on the fully supported path, at the correct version. The abort message names that path and the release URL, so the recovery is discoverable at the moment it is needed. A `PATH` fallback would instead leave them running an unknown version indefinitely, with nothing reporting the mismatch.

Keeping `PATH` would also reintroduce exactly what this spec exists to remove — "`PATH` decided which build answered" — after the store has just eliminated it everywhere else.

**How do `surface-instruction` agents register a project-local binary?**

The question dissolved rather than being answered: the install location moved, and with a machine-global store there is no longer a project-local path for them to fail to express.

The constraint was never the path's shape. Auggie's `~/.augment/settings.json` and Antigravity's `~/.gemini/config/mcp_config.json` hold **one `gvrn` entry serving every project on the machine**, so no project-specific string could ever be correct in them — a versioned path, a relative path, and an absolute path all name one project's binary and hand it to every other project. Every option that kept the binary under `.govern/` therefore had the same defect in a different disguise, and the worst of them was the absolute path, which *looks* project-local while silently privileging whichever project was registered first. That is the ambiguity this spec exists to remove, relocated from `PATH` to the home MCP config rather than fixed.

A store owned by no project is expressible in a per-machine config, because it is a per-machine fact. So these agents now register the same runtime as everyone else, by absolute path, and the asymmetry is gone.

The split is by **config scope, not by agent**:

- `project-committed` configs (`.mcp.json`, `opencode.json`) name the repo-relative pointer, because a machine-specific absolute path in a file the team shares breaks every other contributor and every CI checkout.
- `user-global` / `home-level` configs name the absolute store path, because their files are per-machine and never committed.

The pointer is what makes the first half work, and it exists for that reason alone. Claude Code would not strictly need it — `.mcp.json` documents `${VAR}` / `${VAR:-default}` expansion in `command` precisely so "teams [can] share configurations while maintaining flexibility for machine-specific paths" — but OpenCode's `{env:VAR}` substitution is documented only for provider, model, and instruction fields, is not stated to apply to an MCP `command` array, and has an open report of failing inconsistently for MCP entries. Depending on unverified substitution behavior in a committed file, where the failure surfaces as a broken runtime for every OpenCode adopter, is a worse trade than one small gitignored artifact. The pointer also keeps the two committed configs identical in shape to what this spec originally proposed.

Rejected alternatives:

- **A resolver shim** on a stable path, resolving the project's binary from the working directory. The only option that would make home-level agents genuinely per-project — but it asks `govern` to ship, version, and cross-compile a *second* binary across five targets to solve a binary-distribution problem, and the shim itself needs a machine-global install.
- **Per-project MCP server keys** (`gvrn-{project}`) to make absolute paths safe. Costs the `gvrn` tool namespace, which State A detection scans, the `mcp:gvrn:*` / `mcp__gvrn__*` permission entries name, and `scripts/audit/host-namespace-parity.sh` audits — a large blast radius to serve two of four agents.
- **Leaving those agents on a `PATH` install.** Honest, but it keeps two of four agents outside the acquisition path entirely and forces the framework to document two acquisition models.

**Which version does `/govern` resolve?**

A repo-root `version` file holding a single SemVer line, read out of the archive `/govern` has just fetched.

The file is **the version of the product**, not of one component. `runtime/Cargo.toml`, the newest `runtime/CHANGELOG.md` heading, the `gvrn-v{version}` release tag, and `version` all carry the same number. That is what makes [§runtime-boundary](../../framework/constitution.md#runtime-boundary)'s lockstep real rather than asserted: "the framework's version" and "the runtime's version" stop being two facts that can drift apart, because they are one fact recorded in one place.

Resolving it from the fetched archive is what closes the skew. The pin travels in the same download as the `framework/` tree it describes, so a `/govern` run cannot pair one revision's commands with another revision's runtime. It also needs no network call beyond the fetch already performed, and no rate-limited API.

The framework tree already records runtime versions — every `framework/migrations.toml` entry carries an `introduced_in` SemVer and the bootstrap loop sorts on it — so a framework-side statement about runtime versions is established practice, not a new concept. What was missing was a statement about *this* revision.

`version` is bumped **in the release commit**, alongside `Cargo.toml` and the CHANGELOG, rather than lagged to name the last already-published release. A file that deliberately lags is drift by design: nothing can distinguish "correctly lagging" from "someone forgot to advance it", whereas bumping together yields an invariant a self-audit family can assert outright — all four artifacts agree. The cost is a window between the release commit landing on `main` and the tag being pushed, during which `version` names assets that do not exist yet. That lands on the degrade path this spec already requires (warn, markdown-only, nothing written into `.govern/bin/`), and the next `/govern` run self-heals.

Rejected alternatives:

- **`runtime/Cargo.toml`'s version**, which the full-repo tarball already carries. Its meaning is "what this source tree builds as", which is deliberately ahead of the last published tag between the bump commit and the tag push — so acquisition would 404 in exactly the window a dedicated file is bumped through. Reusing it would also overload a build manifest with a distribution contract.
- **The GitHub "latest release" API**, which resolves the newest published runtime regardless of which framework revision was fetched. That is the *opposite* of lockstep — the adopter's framework no longer pins anything — and it adds an unauthenticated, rate-limited call to every run.
- **An adopter pin in `.govern/config.toml`** as the primary mechanism. Adopter-managed version state drifts by construction, and it puts the framework/runtime compatibility matrix on the person least placed to know it.
