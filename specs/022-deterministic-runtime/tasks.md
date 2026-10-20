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

Authorized by the owner 2026-10-20, answering (b) to: (a) leave 017 in progress until this task runs, or (b) authorize it now. The release needs 022 at done, and 022 cannot reach done with this task open, so (a) would have changed when the task is spent, not whether. Measured that day: dtolnay/rust-toolchain's README says a pinned SHA must be "within the history of the master branch. Any commit that is not within the history of master will eventually get garbage-collected", and the head of its `1.99.1` branch is one commit ahead of master, so its 10 steps pin a master SHA and pass `toolchain: 1.99.1`, the input master's action.yml requires. Each tool is pinned to the version the ductus-v0.59.11 release run installed, and the second bullet also covers the two installs it does not name: ziglang (`pip3 --require-hashes` with the manylinux x86_64 wheel's sha256) and setup-node's Node (`20.20.2`, an exact version; setup-node's README documents no checksum check of the Node it downloads).

Answered (c) by the owner 2026-10-20, for the third subtask's question of which mechanism keeps the pins current: (a) Dependabot, (b) Renovate, (c) a fifth check in scripts/check-runtime-currency.sh, or (d) Dependabot for the workflow actions plus (c) for the rest. Measured that day over 15 pins (9 action pins, 8 of them in .github/workflows and 1 in the adopter template; 6 tool pins: cargo-audit, cargo-zigbuild, cargo-cyclonedx, ziglang, Node, and the `toolchain:` input against runtime/rust-toolchain.toml; the question as put said 16 and 7, a miscount of the six it named): GitHub's docs say Dependabot updates `actions/checkout@<commit> #<tag>` and searches `/.github/workflows` and a root `action.yml`, so it reaches 8 of the 15; Renovate's defaults reach 10 (those 8, setup-node's `node-version` and dtolnay's `toolchain` input), while the template needs a file-pattern entry and the cargo-audit, cargo-zigbuild, cargo-cyclonedx and ziglang pins need custom regex managers; (c) reaches all 15 and opens no pull request. Four action pins are a major release behind whichever is chosen: setup-node v6 to v7, upload-artifact v4 to v7, download-artifact v4 to v8, and the template's checkout v4 to v7.

- [x] Pin every uses: in .github/workflows/*.yml to a full commit SHA with its tag in a comment (8 actions, 39 references on 2026-10-20: actions/checkout, dtolnay/rust-toolchain, actions/cache, taiki-e/install-action, actions/upload-artifact, actions/download-artifact, softprops/action-gh-release, actions/setup-node)
- [x] Pin the versions of cargo-zigbuild and cargo-cyclonedx that runtime-release.yml installs, and of each tool taiki-e/install-action installs
- [x] Add an update process for the pins: a fifth check in scripts/check-runtime-currency.sh (the owner's answer (c) above), run weekly by runtime-currency.yml, that reports every pinned action behind its latest release, every tool pin behind its registry, and every dtolnay/rust-toolchain `toolchain:` input that differs from runtime/rust-toolchain.toml, with a test
- [x] State the pinning and its update process in plan.md's runtime.yml and Release workflow sections
- [x] Close the Node gap in framework-checks.yml, which pinning by exact version did not: at the pinned commit setup-node's download code (src/distributions/base-distribution.ts, official_builds/official_builds.ts) has no checksum check and reads its version manifest from actions/node-versions@main, a branch. Answered (a) by the owner 2026-10-20, to: (a) drop setup-node and run the runner image's Node (24.21.0 on ubuntu-26.04), (b) fetch Node from nodejs.org and check it against a pinned sha256 from its SHASUMS256.txt, or (c) keep setup-node at 20.20.2 and record a documented exception in plan.md. Measured that day: Node 20 reached end of life on 2026-04-30 (nodejs/Release schedule.json), and the lockfile's markdownlint-cli2 0.23.2 and markdownlint declare `node >=22`, which is why `npm ci` warns EBADENGINE on every run
- [x] Pin the shipped adopter template framework/templates/ci/adopter-generators.yml the same way, and state the pinning and its update process in 017's plan (§CI safety net): 017's analysis of 2026-10-20 routed its BE-DEPS-006 finding here, and 017 stays blocking until this lands

- **Done when**: analyze's BE-DEPS-006 assessment of 022's plan passes, and every workflow references its actions and installed tools immutably

## 4. Canonicalize host- and LLM-supplied paths before the containment check (BE-INPUT-004)

BE-INPUT-004's Verification, rewritten 2026-10-19, asks for path canonicalization followed by a containment check comparing whole path components; validate_no_traversal (runtime/src/primitives/mod.rs) is textual, rejecting absolute, rooted and .. paths without resolving a symlink, and writeCode's write-boundary check matches globs.

- [ ] Decide where canonicalization applies (48 validate_no_traversal call sites on 2026-10-20, and the writeCode boundary), record it as a scenario on 022, and implement it with tests that fail when a symlink inside the project escapes it
- [ ] State the commitment in plan.md
- [ ] State it in 063's plan too (§Subjects, absence, and readability), whose `feature` argument passes only `validate_no_traversal`: 063's analysis of 2026-10-20 routed its BE-INPUT-004 finding here, and 063 stays blocking until this lands

- **Done when**: analyze's BE-INPUT-004 assessment of 022's plan passes, and a path that escapes the project through a symlink is refused

## 5. Config readers name unknown keys (CFG-ENV-012, for 019)

- [x] Implement the behavior described in `scenarios/config-readers-name-unknown-keys.md`
- [x] A registry in runtime/src/schema/config_keys.rs declares every ductus-owned section and its recognized keys, entry fields and array-of-table fields, with [workflows] retired
- [x] discover-rule-files returns a notice per unknown key in [rules] and [review], and dashboard carries config.unknown-keys and renders a callout
- [x] Host::load, Paths::load, resolve-constitutions, resolve-references, derive-references and check-orphaned-references warn on stderr for the section each reads, once per process
- [x] A test per reader fails when it reads a key the registry lacks, and a test fails if a mistyped key in an owned section is dropped silently
- [x] The contracts that hold discover-rule-files' notices (scenarios/review-runtime-acceleration.md) and dashboard's config (scenarios/dashboard-primitive.md), and framework/commands/status.md's rendering reference, state the behavior

- **Done when**: Every runtime reader of a ductus-owned config section reports an unknown key by name, the registry is the one declaration of recognized keys, and 022 is re-reviewed back to done.
