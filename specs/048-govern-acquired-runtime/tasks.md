# 048 — Ductus-Acquired Runtime Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Phase 1 is independently safe and lands first so later work is written against a release surface that already behaves as it assumes. Phase 2 builds acquisition. Phase 3 flips the requirement — its tasks land as one change, because a constitution declaring the runtime mandatory while commands still describe fallbacks is a window where the canonical source and the commands disagree.

## Phase 1 — Release surface

### 1. Add the repo-root `version` file and its agreement audit

- [x] Create `version` at the repo root carrying the current runtime version, one SemVer line
- [x] Add `scripts/audit/version-agreement.sh`: `version`, `runtime/Cargo.toml`, and the newest `runtime/CHANGELOG.md` heading must agree; a newest-tag lag is expected (the release commit precedes the tag) and is not a finding
- [x] Register the family in `scripts/audit/run-all.sh` and in `framework/commands/audit.md`'s family list
- [x] Document the release-time bump in `AGENTS.md`'s runtime-release entry — the file joins `Cargo.toml` and the CHANGELOG in the same commit

- **Done when**: `version` exists and agrees with the other three artifacts, `scripts/audit/run-all.sh` exits 0 with the new family registered, and the family fails when `version` is edited out of agreement.

### 2. Gate the release publish on a complete asset set

- [x] Add a job to `.github/workflows/runtime-release.yml` that runs after the build matrix and asserts all five target assets plus sidecars are present before the release is published
- [x] Keep `fail-fast: false` on the build matrix — every target's failure must still be visible in one run
- [x] Verify the gate fails the workflow when one matrix leg fails, rather than publishing a partial release

- **Done when**: a simulated single-target failure blocks the publish and reports which asset is missing, and a complete matrix publishes exactly as before.

### 3. Publish the Windows asset as `.tar.gz`

- [x] Change the Windows leg of `runtime-release.yml` to produce `ductus-x86_64-pc-windows-msvc.tar.gz` plus its `.sha256` sidecar
- [x] Confirm the archive contains `ductus.exe` and that `tar` extracts it on a `windows-latest` runner

- **Done when**: the Windows release asset is a `.tar.gz` extracting cleanly with `tar` on `windows-latest`, so `unzip` is needed on no platform.

## Phase 2 — Acquisition

> **Paused after Phase 1 (2026-08-15), pending the project rename to `ductus`.**
> Phases 2 and 3 write the store path, the pointer path, and the adopter
> migration that rewrites every adopter's MCP config to them. Landing those on
> the current names and renaming afterwards would migrate adopters twice in
> close succession — first to `.ductus/bin/`, then to whatever the rename
> chooses — and would change the release tag scheme one release after this
> work is tagged. Resuming after the rename means acquisition writes the final
> paths from the start and adopters converge in one pass. Phase 1 is unaffected
> by the rename's outcome and is already landed.

### 4. Specify the acquisition procedure in the bootstrap

- [x] Write the acquisition sequence into `framework/bootstrap/ductus.md`: read the pin from `{staging-dir}/ductus-main/version`, derive the target triple, fetch archive + sidecar, verify the digest, extract, install into `~/.ductus/bin/`, set the executable bit
- [x] Specify the failure behavior: a checksum mismatch or download failure halts the run with an error naming the store path and the release URL, leaving the store and pointer unwritten
- [x] Specify the `[runtime]` supplied-binary branch: no download, pointer resolves to the configured path, version mismatch warns, a missing or non-executing path halts naming it
- [x] Specify the idempotency probe: execute the store path, compare the reported version against the pin, re-acquire on mismatch, and treat a binary that will not execute as absent

- **Done when**: `ductus.md` specifies acquisition end to end including every failure branch, and a reader can follow it without consulting this spec.

### 5. Materialize the pointer

- [x] Specify pointer creation in `ductus.md`: attempt a symlink, fall back to a copy when creation fails, so no supported platform requires elevated privileges
- [x] Specify repair — a missing or dangling pointer is recreated without ceremony, being the expected state of any checkout not yet bootstrapped on this machine
- [x] Add `.ductus/bin/` to the framework-managed `.gitignore` block

- **Done when**: the pointer resolves to the store on Unix and on Windows without developer mode, a fresh clone's missing pointer is recreated by `/ductus`, and `git status` reports nothing untracked after a bootstrap.

### 6. Seed the acquisition permissions in all four grammars

- [x] Add `mkdir`, the platform checksum tool, the pointer command, and execution of the store path to each agent's seed in `framework/bootstrap/ductus.md`'s registry table — Claude `Bash(…)`, Auggie `launch-process` regexes, Antigravity `command(…)`, OpenCode `permission.bash` globs
- [x] Walk the §Derived values **Layout-derived** table row by row per `AGENTS.md`'s agent-registry rule, and record the per-agent impact in the commit message
- [x] Confirm no acquisition step prompts on a bootstrap — including checksum verification

- **Done when**: a bootstrap on each agent completes acquisition with no permission prompt, and each grammar's entry is verified against that agent's matcher rather than assumed from another's.

### 7. Rewrite MCP registration and the detection states

- [x] Update the MCP shapes in `ductus.md`: `project-committed` targets name the repo-relative pointer, `user-global` / `home-level` targets name the absolute store path
- [x] Collapse the three detection states to two per the data model, deleting former State C and its §Post-Scaffolding tip
- [x] Update the pre-flight binary probe from a `PATH` lookup to a store check, and its §Permission Setup seed entry with it
- [x] Verify the additive-merge rules are unchanged — other servers, other top-level keys, and a malformed config are all handled as before

- **Done when**: each agent registers the correct path for its config scope, no committed config contains a machine-specific absolute path, and an adopter with no runtime reaches the deterministic path in one `/ductus` run plus one restart.

### 8. Register the adopter migration

- [x] Add a `framework/migrations.toml` entry rewriting an MCP config that names the bare `ductus` command to the ductus-owned path, with `introduced_in` set to the release carrying this work
- [x] Write its procedure body under `framework/migrations/`, idempotent and silent when the target config is absent or already migrated
- [x] Leave any `PATH`-installed binary alone — removing it is the adopter's call

- **Done when**: an adopter config naming bare `ductus` is rewritten on the next `/ductus` run, re-running the migration is a no-op, and a config already migrated is untouched.

## Phase 3 — The requirement

### 9. Amend the constitution and sweep the references

- [x] Replace §runtime-boundary principle 3 and the Opt-in invariant with the requirement and the acquisition-asserting CI job
- [x] Narrow §text-first-artifacts' "usable standalone with no tooling beyond the AI agent" to the artifacts rather than the pipeline
- [x] Remove the 26 per-step markdown-only fallback instructions from `framework/commands/*.md` and `framework/bootstrap/ductus.md`, leaving the Markdown-only reference sections that specify check policy and procedure
- [x] Update `README.md`: drop the `PATH` install as the supported route, correct the Windows cross-compilation claim
- [x] Re-run `scripts/gen-claude-commands.sh`

- **Done when**: no live artifact under `framework/` or `README.md` describes the runtime as optional or documents a `PATH` install, the policy-bearing reference sections survive intact, and `npx markdownlint-cli2` passes.

### 10. Record the change in 021 and 029

- [x] Add a post-completion note to `specs/021-runtime-boundary/spec.md` recording that this spec retires the opt-in invariant its AC1 and AC2 delivered, with a back-link
- [x] Add the same to `specs/029-bootstrap-runtime-autowire/spec.md` for the collapsed detection states
- [x] Reopen both `done → in-progress` per §cross-spec-impact, in the same commit as task 9

- **Done when**: both specs describe the current state rather than the retired one, both are `in-progress`, and neither still asserts a guarantee the constitution no longer makes.

### 11. Replace the opt-in CI job with an acquisition job

- [x] Delete `.github/workflows/markdown-only-pipeline.yml` — it asserts the retired invariant
- [x] Add a job exercising acquisition end to end on each runner platform: fetch the published asset, verify the sidecar, install to a temporary home, execute the binary, and fail when any step fails
- [x] Give every job that runs `cargo` under `runtime/` the `components: clippy, rustfmt` input per `AGENTS.md`'s toolchain-pin gotcha

- **Done when**: the acquisition job passes on macOS, Linux, and Windows runners, and fails when the asset or its sidecar is unavailable.

### 12. Point this repository at its own build

- [x] Set `.ductus/config.toml`'s `[runtime]` key to `runtime/target/release/ductus`
- [x] Update `.mcp.json` to the pointer, replacing the bare `ductus` command
- [x] Replace `AGENTS.md`'s stale-binary gotcha with the `[runtime]` workflow — a `runtime/` change is live after `cargo build --release` and a session restart, with no `cargo install` step
- [x] Confirm a maintainer's MCP calls exercise the freshly built binary

- **Done when**: an MCP tool call in this repo runs the binary the maintainer just compiled, and `AGENTS.md` describes that workflow rather than the `cargo install` one it replaces.

## Phase A — Follow-on scenarios

### 13. Implement scenario: [state-b-continues-in-session](scenarios/state-b-continues-in-session.md) — acquire, then keep going through the CLI

- [x] Implement the behavior described in `scenarios/state-b-continues-in-session.md`

- **Done when**: State B acquires, wires, seeds permissions and then **continues in the same session**, invoking each remaining primitive as `{pointer-path} <primitive>` rather than aborting; the single restart moves to after scaffolding and its message says the work is done and the restart is for the MCP tool surface; the self-update abort is unchanged and still fires before anything else; State A is unaffected; a step with no CLI equivalent falls back to that step's markdown-only specification only, not the whole run; the scenario's open question — whether any step between pre-flight and end-of-scaffolding genuinely needs MCP rather than the CLI — is answered by walking those sections' primitives before the abort is moved; a legacy adopter's bootstrap drops from three restarts to two and a current adopter's from two to one, which is what AC10 states; `framework/bootstrap/ductus.md` documents the continuation for both paths and the generated copies are re-rendered; `scripts/lint-procedure-parseability.sh` and `npx markdownlint-cli2` clean.

### 14. Implement scenario: [retired-namespace-tools-are-off-limits](scenarios/retired-namespace-tools-are-off-limits.md) — a retired server's tools are treated as absent

- [x] Implement the behavior described in `scenarios/retired-namespace-tools-are-off-limits.md`

- **Done when**: `framework/bootstrap/ductus.md` §State B binds the host to the `ductus` namespace for the remainder of the run — retired-namespace MCP tools are not called, not preferred over the pointer CLI, and not read as evidence a runtime is available — and the reasoning names the resolver-vs-layout mismatch rather than restating the detection rule. The parity suite passes unchanged.
