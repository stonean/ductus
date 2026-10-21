# 061 — Updates track the latest release tag Plan

Implements [061 — Updates track the latest release tag](spec.md).

## Overview

The change is almost entirely in the bootstrap's pre-flight and in the installer. A new **Source resolution** step opens the Pre-flight Phase. It settles the run's one ref: from `--ref`, else the recorded `[source] ref`, else the latest release. It validates that ref against the grammar, existence, and both floors, and fetches the version pin at it. Everything afterwards — runtime acquisition, the recorded choice, the self-update check, the archive fetch — reads the ref from there. The archive's extracted root is derived from the extraction instead of being spelled `ductus-main/`.

The installer gains the same flag and the same latest-release resolution. It becomes a release asset, uploaded by the job that already owns the release upload.

The runtime needs no behavior change. `fetch-archive` already takes any URL (`runtime/src/schema/primitives.rs:2468`), and `extract-archive` already returns every extracted path (`runtime/src/schema/primitives.rs:2446`). The one runtime edit is a doc comment. The release that ships this spec is nonetheless required, because the spec's first resolved question makes a release the only way a framework change reaches default-source adopters.

The rest is the prose-claim sweep and the discharge of nine declared cross-spec impacts, one reopen cycle each.

## Technical Decisions

### Source resolution runs host-side, first in pre-flight

The ref has to be known before the first fetch back into this repository. That first fetch is the version pin, and the runtime is acquired *from* the pin (`framework/bootstrap/ductus.md`, Runtime acquisition Branch 2 step 1). So resolution cannot be a runtime primitive: on a first run, State B, no runtime exists yet, and which runtime to acquire is exactly what resolution decides. It is host-side `curl`, like the two pre-flight fetches it replaces. Neither of those is primitive-backed, and both run as shown in every state (`framework/bootstrap/ductus.md`, State A, "Steps with no backticked primitive run as shown in every state").

The new subsection sits between the `{tempdir}` creation and **ductus runtime detection**. It runs these steps in order, and nothing that depends on the source is written until the last one:

1. **Read the inputs.** Read `--ref` from `$ARGUMENTS` (§Inputs gains the flag). Read the recorded `[source] ref`, `[migrations] last_applied` and `[runtime] path` from the active config file. A malformed config file already aborts at read (`framework/bootstrap/ductus.md`, §Project Configuration). Resolution reads it earlier than any other step did, so that rule now fires here.
2. **Check the grammar.** `--ref` given more than once halts. So does a value, from the flag or from the record, that is not `latest`, `main`, or `ductus-v<MAJOR>.<MINOR>.<PATCH>`. Precedence is flag, then record, then default.
3. **Resolve `latest`.** A `curl -sSI` of `https://github.com/stonean/ductus/releases/latest`, with the TLS flags and bounds stated below, without `-L`. The tag is the last path segment of the `Location` header after `/releases/tag/`. A response without a `Location`, or a tag outside `ductus-v<SemVer>`, halts, naming the URL and what came back.
4. **Check the release floor.** A resolved tag below `{ref-floor}` halts. `{ref-floor}` is a constant: the first release that carries this spec.
5. **Fetch the pin at the ref.** `raw.githubusercontent.com/stonean/ductus/{raw-ref}/version`. On a named tag a 404 means the tag does not exist, and the run halts naming it. This fetch is the existence check, so a named tag costs no extra request, and the pin is fetched once per run rather than once per check.
6. **Check the migration floor** (tags only; see below).
7. **Report the source**: one line naming the source, where it came from (`--ref`, recorded, or default), and the ref it resolved to. It is emitted here so that a run which later halts or aborts still says which release it was on (AC5).

**Every fetch resolution makes is bounded and verified.** The redirect read, the pin, both registry reads and the self-update fetch are HTTPS only at TLS 1.2 or later (`--proto '=https' --tlsv1.2`, `BE-DATA-001`), keep `curl`'s certificate and host-name verification, which curl's manual (8.7.1, `--insecure`) says it performs on every TLS connection unless `--insecure` is given (`BE-DATA-011`), and are bounded (`BE-TIMEOUT-001`) by two named values in the bootstrap's Derived paths table: `{connect-timeout}` (10 s) for the connection and `{request-timeout}` (60 s) for the whole request. They are the bootstrap's own, as `install.sh`'s two are its own; the two files fetch the same kinds of file, so they start at the same numbers, but neither reads the other, and either may change alone. A fetch that times out fails like any other, so that fetch's halt or abort fires. None is retried. The bounds were added by 061 task 24, the owner's answer (a) of 2026-10-20; the runtime-acquisition downloads and the markdown-only archive fetch are 048's and 015's and keep their own terms.

**Each floor halt names where its tag came from** (`CFG-ENV-005`): `from --ref`, `recorded as [source] ref in {active config file}`, or `the latest release`, which a tag resolved from `latest` is however `latest` was given. A recorded value below a floor therefore names the setting and the file to fix, as the grammar halt already does.

**Runtime acquisition Branch 2 step 1 stops fetching the pin.** It reads the one resolution fetched. This also retires the paragraph that justified two fetches agreeing "only because both name `main`".

### The ref is recorded after acquisition, before the self-update check

`--ref=main` or a tag is written as a `# ductus (source)` line-prefix managed block. A key in `[source]` other than `ref` is reported by name and never halts, as the bootstrap's §Project Configuration rule for unknown keys says (spec 019's `CFG-ENV-012` decision; `runtime/src/schema/config_keys.rs` registers `[source]` with `ref` alone). The block holds a `[source]` table in the active config file, written with `merge-managed-block`, as the `[host]` block is (`framework/bootstrap/ductus.md`, §Instructions step 6). The primitive appends a new block at the end of the file (`runtime/src/primitives/merge_managed_block.rs:28`). A trailing TOML table cannot capture another table's keys, so appending is safe.

The write happens right after **ductus runtime detection**, because that is the first point at which the primitive is reachable in both states: over MCP in State A, and through `{pointer-path}` in State B. It happens before the **Self-update check**. The self-update's stale path aborts pre-flight, and the re-run that follows must read the choice from the record. Otherwise it would resolve the default and replace the bootstrap again, which is the flip-flop the spec's persistence answer exists to prevent.

`merge-managed-block` has no remove action. So `--ref=latest` rewrites an existing block as a bare `[source]` table with no `ref` key, which reads as the default. With no block present, `--ref=latest` writes nothing, which is the spec's edge case. A runtime removal action was considered and not taken (Trade-offs).

### One ref, three URL forms

The Derived paths table (`framework/bootstrap/ductus.md`, §Derived paths) gains three values:

| Name | `main` | a tag `T` |
| --- | --- | --- |
| `{raw-ref}` | `main` | `T` |
| `{archive-ref}` | `refs/heads/main` | `refs/tags/T` |
| `{source-label}` | `main` | `T`, with "latest release" or "recorded" noted from its origin |

The pin, the self-update fetch and the archive fetch are written against them:

- `raw.githubusercontent.com/stonean/ductus/{raw-ref}/version`
- `raw.githubusercontent.com/stonean/ductus/{raw-ref}/framework/bootstrap/ductus.md`
- `codeload.github.com/stonean/ductus/tar.gz/{archive-ref}`

The latest release is resolved once, so a release published during the run cannot split it (spec, *One run, one source*).

### The framework root is derived from the extraction, not predicted

GitHub names the archive's top directory after the ref: `ductus-main/` for `main`, and `ductus-ductus-v0.54.2/` for that tag (probed 2026-09-28). The naming rule is GitHub's, not this project's. Rather than restate it, `{framework-root}` is **the single top-level directory the archive extracted to**. On the runtime path it is the first path component of `extract-archive`'s `files`. On the markdown path it is the one directory `tar` created. Zero or several top-level directories halt, with the existing fetch-or-extract message.

This replaces the literal `ductus-main/` at every occurrence: six in `framework/bootstrap/ductus.md` and two in `framework/bootstrap/ductus-procedure.md`, counted with `git grep -c`.

### The migration floor reads two small registries

`[migrations] last_applied` holds a migration **id**, not a version (`framework/bootstrap/ductus-procedure.md`, §Pre-run Migrations step 2), and the registry mapping ids to `introduced_in` ships only in the archive. Pre-flight runs before the archive, so it fetches `framework/migrations.toml` at the resolved tag:

- **The id is present** in the tag's registry: the tag knows that migration, so it is not older than it. Pass.
- **The id is absent**, so fetch `main`'s registry. Registry entries are appended, and removed only when they sunset.
  - **Present at `main`**: the migration was added after the tag. Halt, naming the tag and the migration's `introduced_in`.
  - **Absent at `main` too**: the migration has been retired, so the project's layout predates every live entry. Pass, as §Pre-run Migrations' stale-reference behavior already treats a retired id.

Either registry fetch failing halts, naming its URL, as the pin's does (`BE-BULK-002`): the floor is never passed on a registry the run could not read. A null `last_applied` passes, and on the `main` source the check is skipped entirely. The `main` registry read is a validation lookup, not a source fetch. Nothing in the run's content comes from it, so *One run, one source* is not touched. Only the target tag's registry cannot tell a migration newer than the tag from a retired one, and those two need opposite answers.

### Both floors apply to any resolved tag, not only a named one

The spec states the floors for a named tag. They are needed for `latest` too:

- A project on `main` can apply a migration no release carries yet. Returning it with `--ref=latest` would move it below that migration.
- Between this spec landing on `main` and its release being published, `main`'s bootstrap resolves `latest` to a release below `{ref-floor}`. Proceeding would place a bootstrap that fetches `main`, and the next run would replace it again.

The plan pass edits the spec's Failure behavior to say *a resolved tag*, and adds a criterion for the latest-release case, AC17. It does not rewrite AC10, whose meaning a rewrite would change.

### `{ref-floor}` is a constant written in two places

The floor is the version of the release that carries this spec. The plan assumes `0.55.0`, the next minor after `ductus-v0.54.2`, because the spec adds a user-facing flag. That is an assumption, and the implementing task writes the version actually cut. The floor appears in `framework/bootstrap/ductus.md` and in `install.sh`. Once that release exists it never changes, so the two copies cannot drift afterwards. Until then they can, so Family 14 (`scripts/audit/installer-registry-parity.sh`), which already parses both files, gains an assertion that they agree.

### The self-update check names its source

The upstream fetch uses `{raw-ref}`. The stale notice becomes: *The ductus command itself has updated from `{source-label}`*. Its closing *re-run* line is unchanged: once the ref is recorded, a plain re-run resolves the same source (AC14).

### Installer

`install.sh` gains a POSIX argument loop:

- The first non-flag word is the agent, defaulting to `claude` as now.
- `--ref=<value>` is accepted in any position. A repeated `--ref`, or an unknown `--` flag, halts.

The `case "$agent"` arms and their `dest=` assignments stay byte-identical, because Family 14 parses them (`scripts/audit/installer-registry-parity.sh`, header).

Resolution follows the bootstrap's rules:

- The grammar check.
- `latest` from the `Location` header.
- `{ref-floor}` through a small POSIX SemVer comparison.
- Existence through the bootstrap fetch itself, at `raw.githubusercontent.com/stonean/ductus/{raw-ref}/framework/bootstrap/ductus.md`. A 404 on a named tag halts naming it.

All of it happens before the payload verification the installer already does, so nothing is written for a bad ref. The migration floor and a recorded `[source] ref` are not checked: the installer reads no project configuration (spec, Edge Cases). The next `/ductus` run applies both. The plan pass corrects the spec's Installer sentence, which said the failure behavior applies *unchanged*.

When `--ref` was given, the completion message names `/ductus --ref=<value>` as the next command (AC13). The header comment's *live-on-main, no release-pinning knob* claim is replaced. The installer's own seeds already allow the `curl` it issues: `Bash(curl *)` for Claude, and the `^curl` matchers for the others (`install.sh` settings arms).

Both of the installer's requests, the `latest` redirect's headers and the bootstrap fetch, are HTTPS only at TLS 1.2 or later (`--proto '=https' --tlsv1.2`, `BE-DATA-001`), keep curl's certificate and host-name verification (`BE-DATA-011`), and are bounded (`BE-TIMEOUT-001`) by two named constants in `install.sh`: `CONNECT_TIMEOUT_SECONDS` (10) for the connection and `REQUEST_TIMEOUT_SECONDS` (60) for the whole request. The bootstrap was 134,396 bytes in ductus-v0.59.11, the latest release on 2026-10-20, and 137,600 on main, so a minute is ample on a slow link. A request that times out fails like any other curl failure, so the installer halts naming the URL and writes nothing. Neither request is retried. `scripts/tests/test-install.sh` case V fails when either request drops a bound. The bounds were added by 022 task 6, the owner's answer (b) of 2026-10-20.

### The installer ships as a release asset

`scripts/lint-release-ordering.sh` allows only `release-assets` to carry the release-upload action (its rule 2), so the installer is attached there:

1. A checkout step at the tag.
2. A copy of `install.sh` into `staged/`.
3. An assertion in *Assert the asset set is complete* that the installer is present.
4. `staged/install.sh` added to the upload's `files`.

061 adds no dependency manifest or lockfile; the per-change dependency scan stays `cargo audit` in `runtime.yml`, which 022's plan states and which `runtime-release.yml`'s `publish` job needs (`BE-DEPS-007`). A job after `release-assets` then fetches `releases/download/<tag>/install.sh` and compares it byte-for-byte with the tag's copy. That makes AC12 machine-checked on every release, rather than resting on the first adopter who runs the one-liner. The lint's rule 3 requires *some* job after `release-assets` to exercise acquisition. The new job is an addition beside `verify-published`, and the lint's test is re-run.

### Documentation, release procedure, and the retired path

- **README.** Both one-liners become `curl --proto '=https' --tlsv1.2 -sSfL https://github.com/stonean/ductus/releases/latest/download/install.sh | sh`. `-L` follows GitHub's redirect to its asset host, in the user's shell: two `302`s, to `releases/download/<tag>/<asset>` and then to `release-assets.githubusercontent.com` (probed 2026-09-28, spec, installer Resolved Question). *Installing (per agent)* describes the release default and `--ref`.
- **`CLAUDE.md` and the `AGENTS.md` Workflow entry.** They keep *commit directly to `main`*, with the reason restated: releases are cut from `main` by tag.
- **The `AGENTS.md` release entry (AC15).** It states that a framework-only change reaches default-source adopters at the next `ductus-v*` release and bumps like any other release. Its *the commit is what breaks them* paragraph is restated too. The pin is now read at the resolved ref, so a version bump on `main` ahead of its tag halts only projects on `--ref=main`. A default-source project reads a pin from a release that was created only after its asset set was complete.
- **Family 36** (`scripts/audit/self-url-resolution.sh`) derives the repository slug from a `github.com/<owner>/<repo>/archive/` URL in the bootstrap (lines 80–90), and its comment restates *live-on-main*. The derivation moves to the `codeload.github.com/<owner>/<repo>/tar.gz/` form the bootstrap actually fetches, the comment is restated, and the family's line in `framework/commands/audit.md` follows.
- **Family 21.** `framework/bootstrap/govern.md` stays byte-identical to `ductus.md`, as Family 21 requires (`scripts/audit/transitional-bootstrap-parity.sh`). Old `/govern` installs still fetch it from `main`: that URL is compiled into bootstraps already installed, so it is out of this spec's reach. They then run the new bootstrap, which defaults to the latest release.

### Rollout: the bootstrap reaches `origin` only together with its release

Two things break if `framework/bootstrap/ductus.md` or the README one-liner reaches `origin/main` ahead of the release that carries them:

- Every adopter's existing bootstrap fetches `main`'s bootstrap and self-updates to it. The new bootstrap then resolves `latest` to `ductus-v0.54.2`, below `{ref-floor}`, and halts.
- The README one-liner 404s, because `ductus-v0.54.2` has no installer asset.

So the implementation is committed locally, the spec closes, and `main` is pushed together with the release tag in one sitting. That extends `AGENTS.md`'s *bump and tag in the same sitting* to the whole change. The window a published tag still leaves, roughly eleven minutes until the release exists (`AGENTS.md`, *A pushed tag is not a published release*), halts on the floor. Its message therefore says the release carrying this bootstrap may still be publishing, and that re-running shortly or passing `--ref=main` proceeds.

### Discharging nine declared impacts

Each of 003, 007, 015, 023, 026, 029, 048, 050 and 056 follows the 060-to-022 precedent (`6b7a0772`, `2dc4eb59`, `dba5175f`, `b4183757`). The steps are: `done → in-progress` in its own commit, then the corrected claims plus a `> **Signpost:**` blockquote linking back to 061, then a scoped review, an analyze, and `→ done`. The link sits in a blockquote so it induces no `dependencies:` edge (the `derive-dependencies` contract excludes blockquote-prefixed lines), and so no cycle with 061's own links to 003, 015 and 048.

The corrections, from the 2026-09-28 sweep:

- **003**: `curl-sh-installer` scenario lines 15 and 17.
- **007**: `ductus-self-update-precheck` line 23.
- **015**: spec lines 36, 39, 49, 51 and 61. Its Resolved Question deferring ref pinning is marked adopted (§drift-prevention, Decision resolution).
- **023**: the read-fallback Resolved Question's premise, noted as superseded. The decision itself stands.
- **026**: `family-36-self-url-resolution` line 24.
- **029**: `archive-fetch-direct-codeload` lines 9, 16 and 20.
- **048**:
  - spec line 59, and the *Version currency* cost the spec's store answer restates;
  - data-model line 18;
  - `pin-is-readable-when-acquisition-needs-it` lines 12 and 48;
  - the release asset set, which gains the installer.
- **050**: the Resolved Question at spec line 228.
- **056**: spec line 147.

The hits in these specs' `plan.md` and `tasks.md` are design records of how the work was done at the time. They stay, unless a line states current behavior as a present-tense fact, in which case it is corrected in the same reopen.

028 and 032 carry *live-on-main* only as a parenthetical descriptor. It is replaced by one uniform substitution, in its own commit, which is §spec-lifecycle case (a) and reopens neither.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/bootstrap/ductus.md` | Modify | §Inputs `--ref`. New Source resolution subsection. Derived paths. Pin read. `[source]` record. Self-update fetch and notice. Archive URL and `{framework-root}`. §Project Configuration `[source]`. `{ref-floor}`. |
| `framework/bootstrap/govern.md` | Modify | Byte-identical mirror of the above (Family 21). |
| `framework/bootstrap/ductus-procedure.md` | Modify | `{framework-root}` in place of `ductus-main/`. The source line in Post-Scaffolding Output. |
| `install.sh` | Modify | Argument loop, `--ref`, latest resolution, `{ref-floor}`, next-command message, header comment. |
| `scripts/tests/test-install.sh` | Create | Installer behavior against a stubbed `curl`. |
| `.github/workflows/framework-checks.yml` | Modify | Runs the installer test, with `install.sh` and the test in its trigger paths. |
| `.github/workflows/runtime-release.yml` | Modify | Installer staged, asserted and uploaded. Post-release installer check. |
| `scripts/lint-release-ordering.sh`, `scripts/tests/test-lint-release-ordering.sh` | Modify if needed | Keep the lint green with the added job. |
| `scripts/audit/installer-registry-parity.sh` | Modify | Assert both `{ref-floor}` copies agree. |
| `scripts/audit/self-url-resolution.sh`, `scripts/audit/README.md` | Modify | Slug from the codeload URL. The *live-on-main* rationale restated. |
| `framework/commands/audit.md` | Modify | Family 36 line (the generated `.claude/commands/ductus/audit.md` follows through the pre-commit hook). |
| `runtime/src/primitives/fetch_archive.rs` | Modify | Doc comment only. |
| `README.md`, `CLAUDE.md`, `AGENTS.md` | Modify | The prose-claim sweep. The AC15 release-procedure statement. |
| `specs/061-updates-track-the-latest-release-tag/spec.md` | Modify | The plan-pass corrections: floors on any resolved tag, AC17, the installer's reduced checks. |
| `specs/{003,007,015,023,026,029,048,050,056}-*/…` | Modify | Signposts and corrected claims, each in its own reopen. |
| `specs/028-antigravity-agent/spec.md`, `specs/032-opencode-agent/spec.md` | Modify | The uniform descriptor substitution. |

## Trade-offs

- **A runtime primitive for source resolution.** Rejected. Resolution decides which runtime to acquire, so it runs where no runtime exists yet. A primitive would need a host-side fallback for exactly the first-run case, which is two implementations of one contract.
- **Predicting the archive's directory name from the ref.** Rejected in favour of deriving it from the extraction. The name follows GitHub's rule, which already differs between `main` and a `ductus-v*` tag. A prediction would restate that rule and break silently when it changed.
- **Recording the ref before validating it.** Rejected. A bad `--ref` would then persist, and every later plain run would halt on it until someone edited the config by hand.
- **Recording it at the end of the run.** Rejected. The self-update's stale path aborts in pre-flight, so the choice would be lost on exactly the run that needs it.
- **A removal action on `merge-managed-block` for `--ref=latest`.** Not taken. A bare `[source]` table reads as the default with no runtime change. The cost is an empty table left in the config of a project that once chose a ref.
- **The migration floor from the target tag's registry alone.** Rejected. It cannot tell a migration newer than the tag from a retired one, and those need opposite answers.
- **Verification before release.** Rejected as insufficient on its own.
  - What can be exercised before the release exists: every halt; the `main` path end to end in a scratch project, using the working-tree bootstrap; and the installer's parsing and resolution against a stubbed `curl`. Pre-release, `latest` resolves to `ductus-v0.54.2`, which exercises the redirect read and the floor halt for real.
  - What cannot: the latest-release and named-tag *success* paths (AC1; AC3's first half) and the installer download (AC12). No release carries this spec yet, and every existing tag is below the floor.
  - Those paths share their URL construction with the `main` path and with the installer test, and differ only in the ref substituted. AC12 is machine-checked by the release run's new installer job. The success paths are observed on the release run's first adopter-side `/ductus`, in the same sitting.
  - This is a known limitation, not a deferral. Its evidence is recorded in the review.
- **The `main` source moving within one run.** Accepted, as the spec's Edge Cases state. Resolving `main` to a commit would need a GitHub API call, whose unauthenticated limit the spec's resolution answer rejected.
- **The rollout window.** For roughly eleven minutes after the tag is pushed, adopters whose bootstrap self-updates from `main` halt on the floor with a message saying so. That is bounded, visible, and the same class of window `AGENTS.md` already documents for a version bump.
