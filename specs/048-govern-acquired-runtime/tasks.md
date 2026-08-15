# 048 — Govern-Acquired Runtime Tasks

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

- [x] Change the Windows leg of `runtime-release.yml` to produce `gvrn-x86_64-pc-windows-msvc.tar.gz` plus its `.sha256` sidecar
- [x] Confirm the archive contains `gvrn.exe` and that `tar` extracts it on a `windows-latest` runner

- **Done when**: the Windows release asset is a `.tar.gz` extracting cleanly with `tar` on `windows-latest`, so `unzip` is needed on no platform.

## Phase 2 — Acquisition

### 4. Specify the acquisition procedure in the bootstrap

- [ ] Write the acquisition sequence into `framework/bootstrap/govern.md`: read the pin from `{staging-dir}/govern-main/version`, derive the target triple, fetch archive + sidecar, verify the digest, extract, install into `~/.govern/bin/`, set the executable bit
- [ ] Specify the failure behavior: a checksum mismatch or download failure halts the run with an error naming the store path and the release URL, leaving the store and pointer unwritten
- [ ] Specify the `[runtime]` supplied-binary branch: no download, pointer resolves to the configured path, version mismatch warns, a missing or non-executing path halts naming it
- [ ] Specify the idempotency probe: execute the store path, compare the reported version against the pin, re-acquire on mismatch, and treat a binary that will not execute as absent

- **Done when**: `govern.md` specifies acquisition end to end including every failure branch, and a reader can follow it without consulting this spec.

### 5. Materialize the pointer

- [ ] Specify pointer creation in `govern.md`: attempt a symlink, fall back to a copy when creation fails, so no supported platform requires elevated privileges
- [ ] Specify repair — a missing or dangling pointer is recreated without ceremony, being the expected state of any checkout not yet bootstrapped on this machine
- [ ] Add `.govern/bin/` to the framework-managed `.gitignore` block

- **Done when**: the pointer resolves to the store on Unix and on Windows without developer mode, a fresh clone's missing pointer is recreated by `/govern`, and `git status` reports nothing untracked after a bootstrap.

### 6. Seed the acquisition permissions in all four grammars

- [ ] Add `mkdir`, the platform checksum tool, the pointer command, and execution of the store path to each agent's seed in `framework/bootstrap/govern.md`'s registry table — Claude `Bash(…)`, Auggie `launch-process` regexes, Antigravity `command(…)`, OpenCode `permission.bash` globs
- [ ] Walk the §Derived values **Layout-derived** table row by row per `AGENTS.md`'s agent-registry rule, and record the per-agent impact in the commit message
- [ ] Confirm no acquisition step prompts on a bootstrap — including checksum verification

- **Done when**: a bootstrap on each agent completes acquisition with no permission prompt, and each grammar's entry is verified against that agent's matcher rather than assumed from another's.

### 7. Rewrite MCP registration and the detection states

- [ ] Update the MCP shapes in `govern.md`: `project-committed` targets name the repo-relative pointer, `user-global` / `home-level` targets name the absolute store path
- [ ] Collapse the three detection states to two per the data model, deleting former State C and its §Post-Scaffolding tip
- [ ] Update the pre-flight binary probe from a `PATH` lookup to a store check, and its §Permission Setup seed entry with it
- [ ] Verify the additive-merge rules are unchanged — other servers, other top-level keys, and a malformed config are all handled as before

- **Done when**: each agent registers the correct path for its config scope, no committed config contains a machine-specific absolute path, and an adopter with no runtime reaches the deterministic path in one `/govern` run plus one restart.

### 8. Register the adopter migration

- [ ] Add a `framework/migrations.toml` entry rewriting an MCP config that names the bare `gvrn` command to the govern-owned path, with `introduced_in` set to the release carrying this work
- [ ] Write its procedure body under `framework/migrations/`, idempotent and silent when the target config is absent or already migrated
- [ ] Leave any `PATH`-installed binary alone — removing it is the adopter's call

- **Done when**: an adopter config naming bare `gvrn` is rewritten on the next `/govern` run, re-running the migration is a no-op, and a config already migrated is untouched.

## Phase 3 — The requirement

### 9. Amend the constitution and sweep the references

- [ ] Replace §runtime-boundary principle 3 and the Opt-in invariant with the requirement and the acquisition-asserting CI job
- [ ] Narrow §text-first-artifacts' "usable standalone with no tooling beyond the AI agent" to the artifacts rather than the pipeline
- [ ] Remove the 26 per-step markdown-only fallback instructions from `framework/commands/*.md` and `framework/bootstrap/govern.md`, leaving the Markdown-only reference sections that specify check policy and procedure
- [ ] Update `README.md`: drop the `PATH` install as the supported route, correct the Windows cross-compilation claim
- [ ] Re-run `scripts/gen-claude-commands.sh`

- **Done when**: no live artifact under `framework/` or `README.md` describes the runtime as optional or documents a `PATH` install, the policy-bearing reference sections survive intact, and `npx markdownlint-cli2` passes.

### 10. Record the change in 021 and 029

- [ ] Add a post-completion note to `specs/021-runtime-boundary/spec.md` recording that this spec retires the opt-in invariant its AC1 and AC2 delivered, with a back-link
- [ ] Add the same to `specs/029-bootstrap-runtime-autowire/spec.md` for the collapsed detection states
- [ ] Reopen both `done → in-progress` per §cross-spec-impact, in the same commit as task 9

- **Done when**: both specs describe the current state rather than the retired one, both are `in-progress`, and neither still asserts a guarantee the constitution no longer makes.

### 11. Replace the opt-in CI job with an acquisition job

- [ ] Delete `.github/workflows/markdown-only-pipeline.yml` — it asserts the retired invariant
- [ ] Add a job exercising acquisition end to end on each runner platform: fetch the published asset, verify the sidecar, install to a temporary home, execute the binary, and fail when any step fails
- [ ] Give every job that runs `cargo` under `runtime/` the `components: clippy, rustfmt` input per `AGENTS.md`'s toolchain-pin gotcha

- **Done when**: the acquisition job passes on macOS, Linux, and Windows runners, and fails when the asset or its sidecar is unavailable.

### 12. Point this repository at its own build

- [ ] Set `.govern/config.toml`'s `[runtime]` key to `runtime/target/release/gvrn`
- [ ] Update `.mcp.json` to the pointer, replacing the bare `gvrn` command
- [ ] Replace `AGENTS.md`'s stale-binary gotcha with the `[runtime]` workflow — a `runtime/` change is live after `cargo build --release` and a session restart, with no `cargo install` step
- [ ] Confirm a maintainer's MCP calls exercise the freshly built binary

- **Done when**: an MCP tool call in this repo runs the binary the maintainer just compiled, and `AGENTS.md` describes that workflow rather than the `cargo install` one it replaces.
