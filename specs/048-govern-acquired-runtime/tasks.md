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
- [x] Collapse the three detection states to two per the data model, deleting former State C and its tip in `framework/bootstrap/ductus.md` §Post-Scaffolding Output
- [x] Update the pre-flight binary probe from a `PATH` lookup to a store check, and its `framework/bootstrap/ductus.md` §Permission Setup seed entry with it
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

- **Done when**: `framework/bootstrap/ductus.md` binds the host to the `ductus` namespace in **both** states and for the whole run — no tool outside the namespace performs a procedure step or counts as evidence a runtime is available — stated once under §ductus runtime detection rather than inside either state, with the reasoning naming the resolver-vs-layout mismatch and the requirement scoped to runtime stand-ins so it does not read as a blanket ban on the adopter's other MCP servers. Mirrored byte-identically to `framework/bootstrap/govern.md`. The parity suite passes unchanged.

### 15. Publish the crate before creating the GitHub release

- [x] Implement the behavior described in `scenarios/release-halves-publish-together.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

### 16. Make the version pin readable when acquisition needs it

- [x] Implement the behavior described in `scenarios/pin-is-readable-when-acquisition-needs-it.md`

- **Done when**: `{tempdir}` is created in the Pre-flight Phase preamble before either check; Runtime acquisition step 1 fetches the pin from raw into `{tempdir}/version` and reads it there, with §Derived values naming that path; the self-update check states the directory already exists rather than creating a second one; the halt survives with a message naming the pin URL; `{staging-dir}` no longer appears in the procedure; `framework/bootstrap/govern.md` is byte-identical (Family 21); and a fresh greenfield adoption reaches an installed, version-matching runtime with **no hand-supplied pin**.

### 17. Version-check the runtime in State A

- [x] Implement the behavior described in `scenarios/state-a-version-checks-the-pin.md`

- **Done when**: State A probes the resolved binary against `{pin}` and branches three ways — proceed silently on a match, warn and continue for a project-supplied `[runtime] path`, and otherwise acquire `{pin}` then run the remainder through `{pointer-path} <primitive>` with the acquisition carried to the Closing restart, since the running MCP server holds the old binary regardless of the store; `framework/bootstrap/govern.md` is byte-identical (Family 21); and the shipped `ductus-pre-commit` hook probes each primitive's availability before calling it, halting with the installed version and the missing subcommand rather than a bare clap error. Proven against a stale runtime that lacks the subcommands (guard halts with a diagnosis) and a current one (guard silent); Family 22 green.

### 18. Implement scenario: [fetch-archive-reads-its-proxy-once](scenarios/fetch-archive-reads-its-proxy-once.md) — proxy configuration read once, SSRF screen stated for proxies

- [x] Implement the behavior described in `scenarios/fetch-archive-reads-its-proxy-once.md`

- **Done when**: the scenario's open questions are resolved; `fetch-archive` reads no environment variable per call; `docs/runtime.md`'s inventory lists the proxy variables with their effect; the SSRF screen's behavior under a proxy is implemented and stated; an adopter behind a proxy can still fetch; `cargo test` passes. Routed from 062's review (CFG-ENV-001 waived there on `runtime/src/primitives/fetch_archive.rs`).

### 19. fetch-archive identifies itself by the repository URL

- [x] runtime/src/primitives/fetch_archive.rs — the client sets no User-Agent; send `ductus <version> (https://github.com/stonean/ductus)`, the form AGENTS.md and scripts/check-runtime-currency.sh use (found by 050's task 35 adopter bootstrap, 2026-10-27; routed from 050 task 38)

- **Done when**: every fetch-archive request carries a User-Agent naming ductus, its version and `https://github.com/stonean/ductus`, and a test fails when the header is removed

### 20. Disposition out-of-spec finding: claude-md-imports.py's mutation anchors no longer occur in the code

- [x] scripts/mutation/claude-md-imports.py — `run.py --check` reports both M1 and M2 with an invalid anchor: the `previous == Some('@')` block they replace no longer occurs in `runtime/src/primitives/check_orphaned_references.rs`, which the 051/067 re-read rounds of 2026-10-28 rewrote after the list landed in 96934872 (002, 044, 055), so the list can no longer prove its tests fail (found by task 19's `--check` over every list, 2026-10-29) — fixed: `38fc39f6` (051 task 30) changed the anchored `line[..start - 1]` to `line[..token - 1]` with the behavior unchanged; both anchors now spell `token - 1`, `--check` reports none invalid, and both mutations are killed

- **Done when**: the finding is fixed, routed, or discarded, with a discard's reason written on the task

### 21. lint-tool-coverage checks the host-integration pointer, not the retired fallback

- [x] scripts/lint-tool-coverage.sh — still paired each tool reference with the per-step markdown-only fallback AC17 removed, exempted every command carrying the §runtime-host-integration pointer, and so judged 0 references and exited 0 silently on the real tree, as it did on an empty manifest (048's review, 2026-10-29: SHOULD QUAL-CLAIM-001 and an observation). It now requires every command naming a runtime tool, a `# cli-only:` name included, to carry the pointer, prints what it examined, and exits 2 when it could examine nothing; `scripts/tests/test-lint-tool-coverage.sh` runs beside it in `framework-checks.yml` step (d), and 021's and 022's plans and the manifest comment describe it

- **Done when**: the lint judges the pointer rule on the real tree and reports its counts, its test fails when the pointer check or the empty-manifest refusal is removed, and no artifact describes the fallback-marker check as current

### 22. The release refuses a tag that does not name version before it publishes

- [x] .github/workflows/runtime-release.yml — AC15 said a self-audit family asserts that `version` matches the newest `ductus-v*` tag, but Family 20 deliberately leaves the tag out and deferred it to the release pipeline, where no step compared them: a mistyped tag published the crate, the irreversible half, before anything failed (048's review, 2026-10-29). `scripts/check-release-tag.sh <ref-name>` now exits 1 unless the ref is `ductus-v` plus `version`, the release `audit` job runs it first, and `publish` waits for that job; `scripts/lint-release-ordering.sh` property 4 asserts that ordering through every `needs:`, and its test (K, L, M) and `scripts/tests/test-check-release-tag.sh` run in `framework-checks.yml` step (k). AC15, the data model, the resolved question on the version file, `version-agreement.sh` and the audit docs say where the tag is checked

- **Done when**: a tag naming another version than `version` fails the release before `publish`, the check's test and the ordering lint's test each fail when what they assert is removed, and no artifact says a self-audit family compares the tag

### 23. AC9 says it has never been true for Antigravity

- [x] specs/048-govern-acquired-runtime/spec.md — AC9 said the §Permission Setup seed grants exactly what the store probe needs, but Antigravity's seed has never carried `~/.ductus/bin/ductus --version` and keeps the pre-048 `command(which)`, which `ductus.md:291` already discloses (048's review, 2026-10-29; owner chose the annotation). AC9 now carries a never-true annotation like AC22's; the seed itself stays 022's

- **Done when**: AC9 states what holds for each of the four seeds, matching `ductus.md` §Permission Setup and the store-probe paragraph

### 24. The runtime tip that could never render is gone

- [x] framework/bootstrap/ductus-procedure.md — §ductus runtime tip, and its Post-Scaffolding Output bullet, rendered only on a failed acquisition, which halts in pre-flight before this file is read, so it could never appear; 048's spec said the tip is reserved for that case (048's review, 2026-10-29; owner chose deletion). Both are removed, and spec.md's Detection states bullet says the tip is gone and that the halt names the recovery; the Reference migration list, the record of 048's sweep, is unchanged

- **Done when**: no artifact specifies output for a run that halts before it is read, and 048's spec describes the failed-acquisition path as it is

### 25. The seed paragraph states 022's narrowing rule, not a broader one

- [x] framework/bootstrap/ductus.md — §Permission Setup's bold lead said each seed entry grants only what a step runs, while every seed but Pi's keeps `ls` and `command -v` (`which` on Antigravity), which no step runs; 022's task 18 answer (a) narrowed only the entries that can run another command or write outside the project (048's review, 2026-10-29; owner chose the narrower lead). The lead now states that rule and names the two it keeps, in `ductus.md` and the byte-identical `govern.md`

- **Done when**: the paragraph's lead describes every seed entry truthfully and `govern.md` is byte-identical to `ductus.md`

### 26. State A's version check names the probe each branch runs and what the seed covers

- [x] framework/bootstrap/ductus.md — State A's version check said its probe is Runtime acquisition step 2's and that the Permission Setup seed pre-authorizes it; for a `[runtime] path` the probe is Branch 1 step 3 and prompts, as §Permission Setup itself says (048's review, 2026-10-29; owner chose the correction). The sentence now names both branches' probes and that only the store form is seeded, in `ductus.md` and the byte-identical `govern.md`

- **Done when**: the sentence agrees with §Permission Setup and with both acquisition branches, and `govern.md` is byte-identical to `ductus.md`

### 27. The pointer's prompt on OpenCode is stated with the condition that makes it true

- [x] framework/bootstrap/ductus.md — §Pointer materialization and AC22's annotation said the pointer's link prompts on Claude, OpenCode and Antigravity, while OpenCode, by its documentation as 022 task 18 reads it, allows an unmatched command until `/configure` adds its catch-all `ask`, so a first bootstrap does not prompt there (048's review, 2026-10-29, low confidence, no live probe; owner chose the qualification). Both now state the condition, in `ductus.md`, the byte-identical `govern.md` and spec.md

- **Done when**: no artifact claims a prompt on OpenCode before `/configure` has added its catch-all `ask`

### 28. Record the 2026-10-29 review, analyze, and pass the completion gate

- [ ] Handoff state, 2026-10-30. The review record is written (`0af1f01a`: three re-read rounds; the third round's findings routed to 051 task 35, which `ea859f7c` has since landed). `/ductus:analyze` then found 17 blocking findings against the rules as rewritten since the 2026-09-30 record; the owner decided each, and the fixes are the commit after `c8314288` whose subject names 048's analysis (spec and plan pointers, `{probe-timeout}`, the pin's grammar, a proxy value that does not parse refusing every fetch, `SSL_CERT_*` captured at startup, `--*-json` paths held to a staging directory, the hook's echoed version kept printable). That commit is `d4b75e66`, and its re-read found 1 MUST, 7 SHOULD and 10 NOTEs, recorded with recommended dispositions as task 30 (handed off by the owner, 2026-10-30, undecided). `/tmp/review-048/analysis_findings.py` drafted the analysis record's 39 findings and is scratch: re-derive them from `d4b75e66`'s message. Owed, in order: (1) task 30, decided one item at a time, fixed, and re-read until a round reports no MUST or SHOULD; (2) because `d4b75e66` changed `data-model.md` and three scenarios, a refresh of `review.md` over the window since `0af1f01a`, leaning on that review as the in-depth one; (3) `/ductus:analyze`'s re-check (the semantic steps over the 36 rules the run fired, plus any the changed text fires) and `write-analysis` with every finding's disposition: the owner's decisions are in the commit message, and the grounding discards' reason is that a decision record keeps its reasons as they were; (4) `BE-INPUT-016` stays blocking until 008 task 35 adds the forward-proxy clause; (5) task 29's Windows measurement; (6) the completion gate, verifying each criterion (AC9, AC15 and AC22 were edited)

- **Done when**: review.md records the review with no outstanding MUST or SHOULD, analysis.md is current, and `check-review-gate` passes for 048

### 29. Measure on Windows whether the store and the pointer can be replaced while a server runs from them

- [ ] spec.md §Edge Cases says a session left open while the store is replaced keeps running the old binary because "the process holds its own image". That holds on Unix, where a rename keeps the running inode, but is unmeasured on Windows, where replacing a file in use as an executable image generally fails. Two paths reach it there: State A's live-but-stale re-acquisition writes `~/.ductus/bin/ductus.exe` by tempfile and rename, and Pointer materialization's `cp` fallback copies over `.ductus/bin/ductus.exe`, while the session's MCP server may be running either one. Found by 048's 2026-10-29 analysis; routed here by the owner, since an unmeasured gap is a task (§design-principles)
- [ ] On `windows-latest`, from a CI job a `main` push runs (AGENTS.md: never a CI-only branch), start `ductus mcp` from the store and from a copied pointer, then replace each the way the bootstrap does; record each outcome with its command and error
- [ ] Correct every artifact that states the behavior to what was measured: spec.md §Edge Cases, AC13 and AC7, `framework/bootstrap/ductus.md` (and `govern.md`) §Runtime acquisition and §Pointer materialization, and the halt or retry the bootstrap needs where a replacement fails

- **Done when**: The Windows outcome of replacing the store and the pointer under a running server is measured in CI and recorded with its method, and every artifact stating that behavior matches it

### 30. Decide and fix the re-read of d4b75e66 (1 MUST, 7 SHOULD, 10 NOTE)

- [ ] Found 2026-10-30 by an independent read-only re-read of `d4b75e66` (the fixes for 048's analysis). Each item gives the finding and the disposition recommended to the owner, who has not yet decided any of them. Decide one at a time, then fix, route or discard.
- [ ] MUST, `BE-TIMEOUT-001`: the shipped hook's capability probe (`"$ductus_bin" "$primitive" --help`) and its `--version` echo (`framework/bootstrap/hooks/ductus-pre-commit`, the guard loop) wait on the binary unbounded, and so do its `derive-*` calls; `scenarios/state-a-version-checks-the-pin.md`'s guard paragraph names no bound. Recommended: bound the guard's two probes with a portable watchdog (macOS has no `timeout`) at `{probe-timeout}`'s 10 s, decide whether the `derive-*` calls take a bound, and name it in the scenario
- [ ] SHOULD: `runtime/src/main.rs`'s startup comment above `init_proxy_env` still says the certificate variables are the one exception, read at the first fetch; they are captured there since `d4b75e66`. Recommended: fix the comment
- [ ] SHOULD: `scenarios/fetch-archive-reads-its-proxy-once.md` (Behavior) and `docs/runtime.md` (the certificate row and the read-once paragraph) say the certificate variables are read only at startup, while with neither set the first fetch reads both again through `load_native_certs` (`fetch_archive.rs`, `native_certs`'s `platform` branch), as the scenario's own Resolved Question says. Recommended: state that re-read in both
- [ ] SHOULD: `framework/bootstrap/ductus.md` (and `govern.md`) `{probe-timeout}` paragraph says an expired `[runtime] path` probe halts, but State A's `[runtime] path` branches warn or re-acquire. Recommended: give State A a halt for a `[runtime] path` that does not execute, as Branch 1 step 4 does
- [ ] SHOULD: the new test doc comments name the wrong mutations: `captured_cert_paths_are_read_instead_of_the_platform_store` says M3 (M4 kills it), `the_cert_capture_splits_the_directory_list` says M4 (M5), `a_cli_json_file_is_read_only_from_a_staging_directory` says M5 (M6); `a_proxy_value_that_does_not_parse_refuses_every_fetch` omits M3; none names `scripts/mutation/fetch-archive-startup-env.py`. Recommended: correct and name the list
- [ ] SHOULD: `a_cli_json_file_is_read_only_from_a_staging_directory` (`runtime/src/primitives/mod.rs`) fails whenever the crate lies inside a staging root, as a `/tmp` export or a Linux checkout with `TMPDIR` unset does, because it uses the process's staging roots. Recommended: test through `validate_in_staging`'s roots seam with an outside file made outside them
- [ ] SHOULD: the proxy-once scenario's rewritten edge case says a malformed value and a `socks5` value were both skipped and went direct; a `socks5` value was applied and failed at connect, naming the scheme. Recommended: split the history
- [ ] SHOULD: AC22's new parenthetical says opencode.ai/docs/permissions "does not state the no-match fallback itself", but the page, fetched 2026-10-30T01:30Z ("Last updated: Oct 29, 2026"), carries a Tip that commands with arguments "require explicit permission", against AC22's reading that an unmatched command is allowed. Recommended: quote the Tip and correct the OpenCode clause, or measure with a real OpenCode run and record the method
- [ ] NOTE: untested behaviors (the re-reader's own mutants survived): the `CERT_ENV` capture in `init_proxy_env`; a scheme-less proxy (`proxy.corp:3128`) being accepted (`parse`'s `has_host` filter); `flatten_line` in `redact_proxy`; the `NO_PROXY` exemptions applied in `parse`; the hook's 80-column cut. Recommended: add tests for the four that a test can reach
- [ ] NOTE (contrived): under `set -euo pipefail` the hook's sanitizing pipeline exits 141 before the halt message when `--version` output exceeds a pipe buffer. Recommended: cut the first line before piping (`${installed%%$'\n'*}`)
- [ ] NOTE (contrived): `redact_proxy` takes the first `://` as the scheme, so a scheme-less value with `://` in its userinfo shows part of it. Recommended: take the scheme only before the last `@`
- [ ] NOTE (contrived): `read_cli_json_file`'s staging refusal and `load_json_arg`'s JSON error print the raw path, so a newline in it reaches stderr; the local `resolved` holds the path as given. Recommended: flatten both and rename the local
- [ ] NOTE: spec.md (Install location) and plan.md (the pointer) say an unelevated process creates a symlink "only" with the flag; Microsoft's page says the flag allows it, not that it is the only way. Recommended: attribute only what the page says
- [ ] NOTE: plan.md's fetch-archive paragraph says 022's plan names the proxy URL `CFG-ENV-006`'s sensitive value; 022's plan names it under `BE-LOG-002`. Recommended: correct the citation
- [ ] NOTE (contrived): plan.md's `BE-INPUT-003` paragraph calls `{tempdir}` mktemp's output, but Branch 2 steps 4 and 6 interpolate it unquoted, so a `TMPDIR` with a space splits the operands. Recommended: quote `{tempdir}` there
- [ ] NOTE: the `{probe-timeout}` instruction and the `{tempdir}` writes for the `--*-json` files are agent-facing changes with no per-agent impact recorded (AGENTS.md); Claude's seed has no Write rule for `{tempdir}`. Recommended: walk the registry's Layout-derived rows and record each agent's behavior
- [ ] NOTE (contrived, predates `d4b75e66`): a proxy variable set to a non-UTF-8 value reads as unset (`std::env::var(name).ok()`), so fetches go direct. Recommended: read with `var_os` and refuse such a value
- [ ] NOTE: `apply_proxies`' doc cites reqwest 0.13.4 while Cargo.lock locks 0.13.5 (the claim holds); `native_certs` loads each path separately, losing rustls-native-certs' cross-path de-duplication (duplicates are harmless; the doc says "as rustls-native-certs reads them"); the `--*-json` flags' `--help` text omits the staging bound. Recommended: correct the docs and the help text

- **Done when**: The owner has dispositioned every item; each fixed one is fixed, each routed one written to its target, each discarded one recorded with its reason in review.md's next record; and an independent re-read of the fix round reports no MUST or SHOULD
