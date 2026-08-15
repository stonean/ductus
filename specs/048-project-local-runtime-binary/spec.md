---
status: draft
dependencies: [027-bootstrap-migration-registry, 029-bootstrap-runtime-autowire, 042-consolidate-govern-per-project-files-under-govern-directory]
review:
  last-run: null
  reviewed-against: null
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  blocking: false
next-criterion: 13
---

# 048 — Project-Local Runtime Binary

`/govern` acquires the `gvrn` runtime itself — downloading the platform-appropriate release binary into `.govern/bin/`, registering that path with the agent's MCP server, and treating the project-local copy as the canonical runtime for the project. The binary joins `config.toml`, `session.toml`, and `scripts/` under `.govern/` as a per-project, per-contributor artifact rather than a machine-global one on `PATH`.

## Motivation

The runtime was an out-of-band install. The adopter downloaded a release archive by hand, verified its checksum, and copied `gvrn` onto their `PATH`; only afterward did `/govern` detect the binary and wire the MCP registration ([029-bootstrap-runtime-autowire](../029-bootstrap-runtime-autowire/spec.md)). `/govern` automated the *registration* and left the *acquisition* to the reader. Three costs followed from that split:

- **Adoption had a manual step outside the pipeline.** The one command that exists to make a project ready could not make the runtime available; it could only notice that someone else already had. The pipeline's own advice — install `gvrn`, it cuts token use — pointed out of the pipeline.
- **`PATH` decided which build answered.** The registered MCP entry named the bare command (`{"command": "gvrn"}`), so the binary that served a session was whichever one the agent's inherited shell environment resolved first. Two projects on one machine could not run different runtime versions, and a stale binary earlier on `PATH` silently shadowed a newer one.
- **Nothing bound the binary's version to the framework's.** [§runtime-boundary](../../framework/constitution.md#runtime-boundary) requires the runtime to ship in lockstep with the framework — "an adopter's `govern` version pins their compatible runtime version, eliminating schema/runtime drift as a failure mode" — but no mechanism enforced the pin. The framework arrived from a `/govern` archive fetch; the runtime arrived from whenever the adopter last ran `install`.

Moving the binary under `.govern/bin/` puts acquisition, version pinning, and registration in the hands of the single command that already owns every other piece of project setup.

## Install location

The runtime lives at `.govern/bin/gvrn` (`.govern/bin/gvrn.exe` on Windows) — under the `.govern/` directory that already holds the project's govern-owned per-project files ([042-consolidate-govern-per-project-files-under-govern-directory](../042-consolidate-govern-per-project-files-under-govern-directory/spec.md)).

- The binary is **gitignored**, never committed. It is a machine-specific build artifact, and [§text-first-artifacts](../../framework/constitution.md#text-first-artifacts) requires binary artifacts to be gitignored and regenerated on demand by their consumers. The framework-managed `.gitignore` block gains the entry; the existing anchored `/.govern/session.toml` line establishes the pattern of ignoring specific paths under `.govern/` so committed `config.toml` and `scripts/` stay tracked.
- Because it is gitignored, the binary is **per-contributor**: each contributor's `/govern` run acquires the build for their own platform, and a teammate on a different OS is unaffected. This matches how `cli-config-dir` is already handled — per-contributor state stays out of committed config.
- The file is written with the executable bit set.

## Acquisition

`/govern` downloads the release asset matching the host platform, verifies it, and installs it into `.govern/bin/`.

- **Asset naming** follows what `.github/workflows/runtime-release.yml` publishes: `gvrn-{target}.tar.gz` plus a sibling `gvrn-{target}.tar.gz.sha256` on Unix, and `gvrn-{target}.zip` plus `gvrn-{target}.zip.sha256` on Windows, under `https://github.com/stonean/govern/releases/download/gvrn-v{version}/`.
- **Target triples** are the published set: `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, and `x86_64-pc-windows-msvc`. `/govern` derives the triple from the host platform and architecture.
- **Integrity is verified before install.** The sidecar `.sha256` is fetched and checked against the computed digest of the downloaded archive. A mismatch aborts the acquisition without writing anything into `.govern/bin/`, and the run degrades to the markdown path rather than installing an unverified binary. This is stricter than the framework archive fetch, which tolerates a missing sidecar because GitHub's auto-generated source tarballs ship without one — the runtime's release assets always carry theirs.
- **Acquisition never blocks the run.** A network failure, an unpublished asset for the host platform, or a checksum mismatch produces a warning and the markdown-only path, never an abort. [§runtime-boundary](../../framework/constitution.md#runtime-boundary) principle 3 makes the runtime opt-in for adopters: no pipeline gate may require it, and a project that cannot download it must still complete every cycle.
- **Re-runs are idempotent.** When `.govern/bin/gvrn` is already present and reports the resolved version, `/govern` performs no download. When the resolved version differs, the binary is replaced — the upgrade path is a routine `/govern` run.

## MCP registration

The registered server command becomes the project-local path instead of the bare name. The shapes in `/govern`'s §MCP wiring change accordingly:

- **Claude** — `.mcp.json` at the repo root: `{"mcpServers": {"gvrn": {"command": ".govern/bin/gvrn", "args": ["mcp"]}}}`.
- **OpenCode** — the committed root `opencode.json` `mcp` block: `{"gvrn": {"type": "local", "command": [".govern/bin/gvrn", "mcp"], "enabled": true}}`.

Both are `project-committed` targets, so a repo-relative command is coherent: every contributor's checkout resolves the same path, while the binary each path points at is their own gitignored copy. The additive-merge rules are unchanged — an existing MCP config keeps its other servers, other top-level keys are preserved, and a malformed JSON config is never clobbered.

The `surface-instruction` agents (Auggie, Antigravity) read MCP config from the user's **home** directory, shared across every project on the machine. A repo-relative command in a project-agnostic config is resolved against whatever the agent's working directory happens to be, so the project-local path does not carry over unchanged; how those agents register a per-project binary is an open question below.

## Detection states

[029-bootstrap-runtime-autowire](../029-bootstrap-runtime-autowire/spec.md) defined three pre-flight states: **A** (runtime live this session), **B** (binary present on `PATH`, not wired), **C** (binary absent — markdown path plus a tip suggesting the adopter install it). Making acquisition `/govern`'s job collapses the distinction that produced State C: a missing binary is no longer a terminal condition to report, it is work to perform.

- The **binary probe** changes from a `PATH` lookup (`command -v gvrn` / `which gvrn`) to a filesystem check for `.govern/bin/gvrn`. Its permission-seed entry in §Permission Setup changes with it.
- **State A** (a `gvrn`-namespaced MCP tool is in the session's inventory) is unchanged, including its binding execution contract.
- The former **State C** path acquires the binary, then wires and permissions it exactly as State B does, joining the same **pending-restart set** and the same single combined **Pre-flight abort**. An adopter with no runtime at all reaches the deterministic path after one `/govern` run and one restart, with no manual install step in between.
- The **State C tip** in §Post-Scaffolding Output ("installing `gvrn` cuts token use") loses its audience on the happy path and is reserved for the degraded case — acquisition was attempted and failed.

## Reference migration

Every live artifact that names `gvrn` as an **executable** is updated to the project-local path. The set of surfaces:

- `framework/bootstrap/govern.md` — the §MCP wiring JSON shapes, the §Permission Setup probe seed, the §Detection mechanism probe, the §gvrn runtime detection state definitions, the §Post-Scaffolding tip, and the matching §Edge Cases rows.
- `README.md` — the "Install the runtime" and "Registering the runtime" sections, which currently document a manual `sudo install -m 0755 gvrn /usr/local/bin/gvrn`.
- This repository's own `.mcp.json`, subject to the framework-repo question below.

References to `gvrn` as a *name* — the crate, the MCP server key, the `gvrn-v*` release tag scheme, the `mcp__gvrn__*` tool prefix — are unaffected. Prose describing runtime behavior (`gvrn exec` resolution, primitive contracts) is likewise unaffected: those name the runtime, not a path to invoke.

Historical specs whose bodies record the pre-048 shape (028, 031, 032) are covered by the [§spec-lifecycle](../../framework/constitution.md#spec-lifecycle) mechanical-edit rule when the change is the uniform token substitution; anything requiring more than that substitution takes the normal back-edge per [§cross-spec-impact](../../framework/constitution.md#cross-spec-impact).

## Adopter migration

Existing adopters have an MCP entry naming the bare `gvrn` command and, usually, a binary on their `PATH`. A `framework/migrations.toml` entry rewrites the registered command to the project-local path on the next `/govern` run, following the registry contract in [027-bootstrap-migration-registry](../027-bootstrap-migration-registry/spec.md): a `[[migrations]]` row with an `introduced_in` version and a procedure body under `framework/migrations/`. The procedure is idempotent and exits silently when the target config is absent or already migrated. The `PATH`-installed binary is left alone — removing it is the adopter's call, not `/govern`'s.

## Acceptance Criteria

- [ ] AC1: A `/govern` run on a project with no runtime installed downloads the host-platform release asset, verifies its `.sha256` sidecar, and writes an executable `.govern/bin/gvrn` (`.govern/bin/gvrn.exe` on Windows)
- [ ] AC2: `.govern/bin/` is present in the framework-managed `.gitignore` block, and a freshly bootstrapped project reports no untracked binary under `git status`
- [ ] AC3: A checksum mismatch leaves `.govern/bin/` unwritten, emits a warning naming the expected and computed digests, and the run continues on the markdown path
- [ ] AC4: A download failure (network error, missing asset for the host platform) emits a warning and completes the run on the markdown path — no abort, no partial binary
- [ ] AC5: After acquisition on a `write-file` agent, the MCP config registers `.govern/bin/gvrn` — `.mcp.json` for Claude, the root `opencode.json` `mcp` block for OpenCode — additively, preserving every other server and top-level key
- [ ] AC6: A second `/govern` run with the resolved version already installed performs no download and leaves `.govern/bin/gvrn` byte-unchanged
- [ ] AC7: A `/govern` run whose resolved version differs from the installed binary's `--version` replaces the binary and reports the upgrade
- [ ] AC8: An adopter whose MCP config names the bare `gvrn` command has it rewritten to the project-local path by the registered migration, and re-running the migration is a no-op
- [ ] AC9: The pre-flight binary probe checks `.govern/bin/gvrn` rather than `PATH`, and the §Permission Setup seed grants exactly what that probe needs
- [ ] AC10: An adopter with no runtime reaches the deterministic path in one `/govern` run plus one restart — acquisition, MCP wiring, and tool permissions all land in the same pre-flight pass and surface in one combined abort
- [ ] AC11: The markdown-only CI job (`.github/workflows/markdown-only-pipeline.yml`) passes with `.govern/bin/` absent, exercising a full pipeline cycle without the runtime
- [ ] AC12: No live artifact under `framework/` or `README.md` documents installing `gvrn` onto `PATH` as the supported path, and none registers a bare `gvrn` MCP command

## Open Questions

- **Which version does `/govern` resolve?** The framework is fetched from the `main` branch tarball (unversioned), while the runtime is released under `gvrn-v{version}` tags. §runtime-boundary requires lockstep, but the archive carries no version stamp today. Candidates: a version file added to the framework archive, the GitHub "latest release" API, or an explicit `.govern/config.toml` pin. The choice determines whether lockstep is actually enforced or merely asserted.
- **How do `surface-instruction` agents register a project-local binary?** Auggie's `~/.augment/settings.json` and Antigravity's `~/.gemini/config/mcp_config.json` are home-level and shared across every project. A repo-relative `.govern/bin/gvrn` there is resolved against the agent's working directory, and a machine-absolute path pins the config to one project. Options: surface an absolute path per project, keep those agents on a `PATH` install, or introduce a resolver shim.
- **Does a `PATH`-installed `gvrn` remain a supported fallback?** When acquisition fails but a `PATH` binary exists, `/govern` could wire that instead of degrading to the markdown path. This preserves today's behavior for offline or firewalled adopters at the cost of reintroducing the ambiguity 048 exists to remove.
- **What does the framework's own repository do?** `govern` builds its runtime from source (`runtime/target/release/gvrn`); its committed `.mcp.json` currently names the bare command. Vendoring a *released* binary into `.govern/bin/` would mean the maintainer's MCP server runs a build that is not the one they just compiled — the stale-binary failure `AGENTS.md` already records. A dev override (symlink, config key, or an exemption for this repo) needs a decision.
- **Which platforms are guaranteed?** The README states a Windows binary "appears when cross-compilation succeeds", so the Windows asset is not a guarantee. What `/govern` does on a platform with no published asset — warn and degrade, or offer a `cargo install` fallback — is unresolved.
- **What new permissions does acquisition need?** Downloading, verifying, extracting, `chmod`-ing, and executing a project-local binary may exceed the current bootstrap seed for each agent's permission grammar. Whether the existing `curl`/`tar`/`chmod` grants cover it, and what each agent's settings template must add, needs to be worked through per agent.
- **Is `--version` the right idempotency probe?** Comparing the installed binary's `--version` output against the resolved version requires executing an untrusted-until-verified binary on every run. A recorded version marker alongside the binary would avoid the execution, at the cost of a second artifact that can drift from the file it describes.
