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

- [ ] Pin every uses: in .github/workflows/*.yml to a full commit SHA with its tag in a comment (8 actions, 39 references on 2026-10-20: actions/checkout, dtolnay/rust-toolchain, actions/cache, taiki-e/install-action, actions/upload-artifact, actions/download-artifact, softprops/action-gh-release, actions/setup-node)
- [ ] Pin the versions of cargo-zigbuild and cargo-cyclonedx that runtime-release.yml installs, and of each tool taiki-e/install-action installs
- [ ] Add an update process for the pins (Dependabot or Renovate for github-actions) and teach scripts/check-runtime-currency.sh to read a SHA-pinned dtolnay/rust-toolchain
- [ ] State the pinning and its update process in plan.md's runtime.yml and Release workflow sections
- [ ] Pin the shipped adopter template framework/templates/ci/adopter-generators.yml the same way, and state the pinning and its update process in 017's plan (§CI safety net): 017's analysis of 2026-10-20 routed its BE-DEPS-006 finding here, and 017 stays blocking until this lands

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
