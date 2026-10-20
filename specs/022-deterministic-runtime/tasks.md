# 022 — Deterministic Runtime Tasks

Tasks derived from the [plan](plan.md). Complete in order. Each task is small enough to complete and verify in a single session; later tasks depend on earlier ones.

## 1. Rule-file review 2026-10-19: re-point rule citations the review changed

- [x] scenarios/the-constitutions-registry-validates-its-values.md cites CFG-ENV-008 (malformed value) for a malformed registry entry, not CFG-ENV-003 (missing required value)
- [x] scenarios/review-runtime-acceleration.md's degenerate-surfaces analogy names CFG-ENV-008
- [x] scenarios/fetch-archive-dns-rebinding.md's Context records that BE-INPUT-016 made address pinning a MUST on 2026-10-19 (it was low priority under BE-INPUT-007 alone)
- [x] plan.md's cargo audit step cites BE-DEPS-001 and the per-change scan of BE-DEPS-007

- **Done when**: Every rule citation in 022's artifacts matches the rule's current meaning in framework/rules/, and 022 is re-reviewed back to done.

## 2. Pin every redirect hop to its screened address (BE-INPUT-016)

- [x] Implement the behavior described in `scenarios/fetch-archive-dns-rebinding.md`
- [x] fetch-archive's connections, including every redirect hop, use only addresses that passed the internal-range screen (e.g. a screening dns::Resolve instead of a per-host resolve_to_addrs pin)
- [x] A test fails if a redirect host is resolved a second time without the screen

- **Done when**: No fetch-archive connection, initial or redirected, resolves a host without screening the addresses it connects to, and 022 is re-reviewed back to done.

## 3. Pin CI actions and build tools immutably, with an update process (BE-DEPS-006)

BE-DEPS-006 (rewritten 2026-10-19) asks a plan covering CI or build tooling to commit to immutable pinning and an update process for the pins; 022's analysis of 2026-10-20 found neither in plan.md, and the workflows reference actions by tag.

Authorized by the owner 2026-10-20, answering (b) to: (a) leave 017 in progress until this task runs, or (b) authorize it now. The release needs 022 at done, and 022 cannot reach done with this task open, so (a) would have changed when the task is spent, not whether. Measured that day: dtolnay/rust-toolchain's README says a pinned SHA must be "within the history of the master branch. Any commit that is not within the history of master will eventually get garbage-collected", and the head of its `1.99.1` branch is one commit ahead of master, so its 10 steps pin a master SHA and pass `toolchain: 1.99.1`, the input master's action.yml requires. Each tool is pinned to the version the ductus-v0.59.11 release run installed, and the second bullet also covers the two installs it does not name: ziglang (`pip3 --require-hashes` with the manylinux x86_64 wheel's sha256) and setup-node's Node (`20.20.2`, an exact version; setup-node's README documents no checksum check of the Node it downloads), the last since replaced by the runner image's Node (the fifth subtask below).

Answered (c) by the owner 2026-10-20, for the third subtask's question of which mechanism keeps the pins current: (a) Dependabot, (b) Renovate, (c) a fifth check in scripts/check-runtime-currency.sh, or (d) Dependabot for the workflow actions plus (c) for the rest. Measured that day over 15 pins (9 action pins, 8 of them in .github/workflows and 1 in the adopter template; 6 tool pins: cargo-audit, cargo-zigbuild, cargo-cyclonedx, ziglang, Node, and the `toolchain:` input against runtime/rust-toolchain.toml; the question as put said 16 and 7, a miscount of the six it named): GitHub's docs say Dependabot updates `actions/checkout@<commit> #<tag>` and searches `/.github/workflows` and a root `action.yml`, so it reaches 8 of the 15; Renovate's defaults reach 10 (those 8, setup-node's `node-version` and dtolnay's `toolchain` input), while the template needs a file-pattern entry and the cargo-audit, cargo-zigbuild, cargo-cyclonedx and ziglang pins need custom regex managers; (c) reaches all 15 and opens no pull request. Four action pins are a major release behind whichever is chosen: setup-node v6 to v7, upload-artifact v4 to v7, download-artifact v4 to v8, and the template's checkout v4 to v7.

- [x] Pin every uses: in .github/workflows/*.yml to a full commit SHA with its tag in a comment (8 actions, 39 references on 2026-10-20: actions/checkout, dtolnay/rust-toolchain, actions/cache, taiki-e/install-action, actions/upload-artifact, actions/download-artifact, softprops/action-gh-release, actions/setup-node)
- [x] Pin the versions of cargo-zigbuild and cargo-cyclonedx that runtime-release.yml installs, and of each tool taiki-e/install-action installs
- [x] Add an update process for the pins: a fifth check in scripts/check-runtime-currency.sh (the owner's answer (c) above), run weekly by runtime-currency.yml, that reports every pinned action behind its latest release, every tool pin behind its registry, and every dtolnay/rust-toolchain `toolchain:` input that differs from runtime/rust-toolchain.toml, with a test
- [x] State the pinning and its update process in plan.md's runtime.yml and Release workflow sections
- [x] Close the Node gap in framework-checks.yml, which pinning by exact version did not: at the pinned commit setup-node's download code (src/distributions/base-distribution.ts, official_builds/official_builds.ts) has no checksum check and reads its version manifest from actions/node-versions@main, a branch. Answered (a) by the owner 2026-10-20, to: (a) drop setup-node and run the runner image's Node (24.21.0 on ubuntu-26.04), (b) fetch Node from nodejs.org and check it against a pinned sha256 from its SHASUMS256.txt, or (c) keep setup-node at 20.20.2 and record a documented exception in plan.md. Measured that day: Node 20 reached end of life on 2026-04-30 (nodejs/Release schedule.json), and the lockfile's markdownlint-cli2 0.23.2 and markdownlint declare `node >=22`, which is why `npm ci` warns EBADENGINE on every run
- [x] Pin the shipped adopter template framework/templates/ci/adopter-generators.yml the same way, and state the pinning and its update process in 017's plan (§CI safety net): 017's analysis of 2026-10-20 routed its BE-DEPS-006 finding here, and 017 stays blocking until this lands

- **Done when**: analyze's BE-DEPS-006 assessment of 022's plan passes, and every workflow references its actions and installed tools immutably

State on 2026-10-20: every subtask is done and every workflow pins immutably (`458fa3e3`, `d8d078db`, `f4da0512`, `02a5ac13`, `fa489f6a`). The analyze half waits on 022's next analysis, which stays blocking by design while task 4 is open. An independent BE-DEPS-006 assessment of 022's plan at `8595fcbd` found one gap, no checksum basis stated for the Rust toolchain download, which `d235e722` closed; 017's analysis at `742db59b` found BE-DEPS-006 passing for the workflows 017's plan covers.

## 4. Canonicalize host- and LLM-supplied paths before the containment check (BE-INPUT-004)

BE-INPUT-004's Verification, rewritten 2026-10-19, asks for path canonicalization followed by a containment check comparing whole path components; validate_no_traversal (runtime/src/primitives/mod.rs) is textual, rejecting absolute, rooted and .. paths without resolving a symlink, and writeCode's write-boundary check matches globs.

Authorized by the owner 2026-10-20, answering (b) to: (a) leave 063 in progress until this task runs, (b) authorize it now, or (c) canonicalize in `check-artifact-size` alone so 063 can close, leaving this task for the rest. Measured that day: 42 production `validate_no_traversal` call sites in 38 files (48 with tests), plus the writeCode boundary; 022 cannot reach done while this task or task 6 is open, and the release needs 022 at done, so (a) and (c) change when this task is spent, not whether. 063's review is stale under every option (a5e18ad7 and fc876871 edited its data model, a scenario and its spec).

- [x] Implement the behavior described in `scenarios/a-path-argument-resolves-inside-its-base.md`, once its two open questions (how the check decides, and which call sites go through it) are answered, with tests that fail when a symlink inside the project escapes it
- [x] State the commitment in plan.md
- [x] State it in 063's plan too (§Subjects, absence, and readability), whose `feature` argument passes only `validate_no_traversal`: 063's analysis of 2026-10-20 routed its BE-INPUT-004 finding here, and 063 stays blocking until this lands

- **Done when**: analyze's BE-INPUT-004 assessment of 022's plan passes, and a path that escapes the project through a symlink is refused

State on 2026-10-20: every subtask is done, and the owner answered both of the scenario's questions (a): canonicalize and compare, and every site that opens a path plus a walk of each feature directory, `check-artifacts` exempt. The release binary refuses the three escapes the scenario probed, and ten mutations of the check each failed the test that covers it. The analyze half waits on 022's next analysis, and 063 needs a re-review and a re-analysis.

## 5. Config readers name unknown keys (CFG-ENV-012, for 019)

- [x] Implement the behavior described in `scenarios/config-readers-name-unknown-keys.md`
- [x] A registry in runtime/src/schema/config_keys.rs declares every ductus-owned section and its recognized keys, entry fields and array-of-table fields, with [workflows] retired
- [x] discover-rule-files returns a notice per unknown key in [rules] and [review], and dashboard carries config.unknown-keys and renders a callout
- [x] Host::load, Paths::load, resolve-constitutions, resolve-references, derive-references and check-orphaned-references warn on stderr for the section each reads, once per process
- [x] A test per reader fails when it reads a key the registry lacks, and a test fails if a mistyped key in an owned section is dropped silently
- [x] The contracts that hold discover-rule-files' notices (scenarios/review-runtime-acceleration.md) and dashboard's config (scenarios/dashboard-primitive.md), and framework/commands/status.md's rendering reference, state the behavior

- **Done when**: Every runtime reader of a ductus-owned config section reports an unknown key by name, the registry is the one declaration of recognized keys, and 022 is re-reviewed back to done.

## 6. Name the timeout on every download in the release workflows and the installer (BE-TIMEOUT-001, BE-RETRY-001)

Owner-gated: not started until the owner authorizes it, since it changes the release path and the shipped installer. Check the route first: install.sh is 003's deliverable (`9847647b`, the curl-sh installer) whose source resolution 061 reshaped, so its half may belong to one of those specs rather than to 022.

- [ ] runtime-acquisition.yml:131-132 and install.sh:124 and :163 call curl with no connect or total timeout, and runtime-release.yml:623 has none and retries with a fixed delay and no jitter; found by 017's review of 2026-10-20, which bounded the new calls (scripts/check-runtime-currency.sh's fetch, the adopter CI template's download) and routed these pre-existing ones here
- [ ] Give each a named connect timeout and total timeout, and make the release retry jittered or remove it
- [ ] State the timeouts in plan.md's Release workflow section; install.sh changes are tested by scripts/tests/test-install.sh

- **Done when**: Every curl in the workflows and install.sh carries a named timeout, and analyze's BE-TIMEOUT-001 assessment of 022's plan passes

## 7. Move the three action pins a major release behind

Found by the pins check 022 task 3 added; the first weekly run (Monday 2026-10-26) exits 1 on these three until they move. Measured 2026-10-20: upload-artifact and download-artifact run only in runtime-release.yml, on a tag, which the owner is holding, so a bump there cannot be exercised before the release. An earlier note said the template's checkout could be exercised in a linux/amd64 container the way 017's `the-adopter-ci-template-can-run` was; that harness runs the template's `run:` steps and skips every `uses:` step, so it cannot. Also measured: taiki-e/install-action published 13 releases between 2026-09-20 and 2026-10-20, so its pin will turn the weekly run red most Mondays and need a bump each time, a consequence of the owner's option (c) for task 3. `scripts/check-runtime-currency.sh` has no way to hold a pin, so a held pin also fails every weekly run.

Breaking changes, from each release's notes (`gh release view`, 2026-10-20), against how the pins are used. upload-artifact v5.0.0 to v7.0.2: Node 24 (v6 requires runner 2.327.1), ESM (v7), and a new `archive` input to upload a single file unzipped; the release workflow uploads by `name` and `path` (build and sbom jobs), which none of them changes. download-artifact v5.0.0 to v8.0.2: v5 changes the path for a single artifact downloaded by ID, v7 Node 24, v8 ESM, a hash mismatch now fails the step by default (`digest-mismatch`), and a file that is not a zip is no longer unzipped (`skip-decompress`); the workflow downloads by `name` (acquire) and by `pattern` with `merge-multiple` (release-assets), neither by ID. checkout v5.0.0 to v7.0.1: Node 24 (runner 2.327.1), credentials persisted to a separate file (v6), and a refusal to check out a fork's PR under `pull_request_target` or `workflow_run` (v7); the template runs on `pull_request` and `push` with default inputs. Where each would first run: download-artifact's by-name use is in `acquire`, before the crates.io publish, and its by-pattern use is in `release-assets`, which `needs: publish`, so a failure there comes after a publish that cannot be repeated for that version.

Decision put to the owner 2026-10-20: (a) bump all three now and let the held release be the first run of the artifact pair; (b) bump all three now and add a push-triggered job to framework-checks.yml that uploads two artifacts with the new upload-artifact and downloads them with the new download-artifact by name and by pattern with merge-multiple, as the release does, kept so each later bump of either is exercised before a tag; (c) hold all three, which fails every weekly pins run until they move, since the check has no hold; or (d) bump the template's checkout now and the artifact pair in the release session.

- [ ] Owner decides whether and when (put 2026-10-20, above); then bump each pin's SHA and tag comment together, after reading each action's release notes for breaking changes between the pinned and the latest major
- [ ] actions/upload-artifact v4.6.2 to v7.0.2 and actions/download-artifact v4.3.0 to v8.0.2 in runtime-release.yml, and the adopter CI template's actions/checkout v4.4.0 to v7.0.1

- **Done when**: scripts/check-runtime-currency.sh pins reports no action pin behind, or the owner has decided to hold them
