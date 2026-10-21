# 061 — Updates track the latest release tag Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Bootstrap: `--ref` and Source resolution

- [x] Add `--ref=<value>` to §Inputs' recognized flags in `framework/bootstrap/ductus.md`, and to the frontmatter `argument-hint`
- [x] Add the Source resolution subsection at the head of the Pre-flight Phase, after `{tempdir}` creation. Its steps: read the inputs, check the grammar, resolve `latest` from the `Location` header, check the release floor, fetch the pin at the ref as the existence check, check the migration floor, and report the source line
- [x] Add `{raw-ref}`, `{archive-ref}`, `{source-label}` and `{ref-floor}` to Derived paths. `{ref-floor}` is the version the release carrying this spec will be cut at (planned `0.55.0`)
- [x] Make Runtime acquisition Branch 2 step 1 read the resolved pin instead of fetching `main/version`, and remove the paragraph on two fetches agreeing because both name `main`
- [x] Write every halt with the message the spec's Failure behavior requires, including the pre-publication wording on the release floor
- [x] Mirror the file byte-for-byte to `framework/bootstrap/govern.md`

- **Done when**: every Failure behavior bullet in the spec maps to a halt in the Source resolution subsection with its named values. `framework/bootstrap/govern.md` and `framework/bootstrap/ductus.md` are byte-identical. Families 14 and 21 of `scripts/audit/run-all.sh` pass.

## 2. Bootstrap: record the choice, self-update from the source

- [x] Add the `[source]` managed-block write (`# ductus (source)`, via `merge-managed-block`) after ductus runtime detection and before the Self-update check, covering the `--ref=latest` rewrite and the nothing-recorded no-op
- [x] Point the Small fetch at `{raw-ref}` and make the stale notice name `{source-label}`
- [x] Document `[source]` in §Project Configuration: the TOML example and a `source.ref` field description pointing at this spec's data model
- [x] Mirror to `framework/bootstrap/govern.md`

- **Done when**: the procedure records a `--ref` before any path that can abort pre-flight, and a plain re-run after the stale abort resolves the recorded source. The stale notice names its source (AC14). §Project Configuration lists `[source]`. Family 21 passes.

## 3. Bootstrap: archive at the ref, derived framework root

- [x] Fetch the archive from `codeload.github.com/stonean/ductus/tar.gz/{archive-ref}`, keeping the direct-codeload rationale
- [x] Define `{framework-root}` as the single top-level directory the extraction produced (the first path component of `extract-archive`'s `files`, or the one directory `tar` created), halting on zero or several
- [x] Replace every `ductus-main/` in `framework/bootstrap/ductus.md` and `framework/bootstrap/ductus-procedure.md` with `{framework-root}`, including §The archive half's address
- [x] Add the source line to Post-Scaffolding Output in `framework/bootstrap/ductus-procedure.md`
- [x] Mirror to `framework/bootstrap/govern.md`

- **Done when**: neither bootstrap file contains `ductus-main` or `refs/heads/main` outside a statement of the `main` source's own `{archive-ref}`, and `scripts/audit/run-all.sh` passes.

## 4. Installer

- [x] Replace `agent="${1:-claude}"` with a POSIX argument loop: the first non-flag word is the agent, `--ref=<value>` may appear in any position, and a repeated `--ref` or an unknown `--` flag halts. Leave every `case "$agent"` arm and its `dest=` byte-identical
- [x] Resolve and validate the ref as the bootstrap does (grammar, `latest` from the `Location` header, `{ref-floor}` by a POSIX SemVer comparison), and fetch the bootstrap from `{raw-ref}`, treating a 404 on a named tag as "does not exist", all before any write
- [x] When `--ref` was given, name `/ductus --ref=<value>` as the next command in the completion message
- [x] Replace the header comment's live-on-main and no-release-pinning claims, and document `--ref` and the release one-liner in its usage block

- **Done when**: `sh -n install.sh` and shellcheck pass. Family 14 passes unchanged. The installer never writes a file for a ref that fails any check.

## 5. Installer test

- [x] Create `scripts/tests/test-install.sh`. It runs `install.sh` in a temp repo with a stubbed `curl` on `PATH`, and asserts:
  - the default resolves the tag the stub's `Location` names;
  - `--ref=main` and `--ref=<tag>` fetch `{raw-ref}`;
  - a missing `Location`, a non-`ductus-v*` tag, a tag below `{ref-floor}`, a 404 tag, an empty or unknown value, and a repeated `--ref` each halt with nothing written;
  - `--ref` in either position beside the agent key is accepted;
  - the completion message names `/ductus --ref=<value>`.
- [x] Break each check once and watch it fail, before trusting it
- [x] Wire the test into `.github/workflows/framework-checks.yml`, with `install.sh` and the test in the job's trigger paths

- **Done when**: the test passes, each assertion has been seen to fail against a broken installer, and `framework-checks.yml` runs it.

## 6. Audit families

- [x] Family 14 (`scripts/audit/installer-registry-parity.sh`): assert that the `{ref-floor}` in `install.sh` equals the one in `framework/bootstrap/ductus.md`, and prove the assertion fails on a mismatch
- [x] Family 36 (`scripts/audit/self-url-resolution.sh`): derive the slug from the `codeload.github.com/<owner>/<repo>/tar.gz/` URL the bootstrap fetches, and restate the live-on-main rationale in its comment and remediation text
- [x] Update Family 36's line in `framework/commands/audit.md` and in `scripts/audit/README.md`

- **Done when**: `scripts/audit/run-all.sh` passes, and Family 36 reports the same slug and URL counts as before the change.

## 7. Release workflow

- [x] In `release-assets` (`.github/workflows/runtime-release.yml`): check out the tag, stage `install.sh`, assert it in the complete-set step, and add it to the upload's `files`
- [x] Add a post-release job that fetches `releases/download/<tag>/install.sh` and compares it byte-for-byte with the tag's copy
- [x] Run `scripts/lint-release-ordering.sh` and `scripts/tests/test-lint-release-ordering.sh`, adjusting the lint only if the added job needs it

- **Done when**: the release-ordering lint and its test pass, and the workflow uploads `install.sh` only from `release-assets`.

## 8. Prose-claim sweep and release procedure

- [x] `README.md`: both one-liners become the `releases/latest/download/install.sh` form with `-L`, and *Installing (per agent)* describes the release default and `--ref`
- [x] `CLAUDE.md` Non-negotiables and the `AGENTS.md` Workflow entry: keep *commit directly to `main`*, restating the reason
- [x] `AGENTS.md` release entry: state that a framework-only change reaches default-source adopters at the next `ductus-v*` release and bumps like any other (AC15), and restate the *the commit is what breaks them* paragraph for a pin read at the resolved ref
- [x] `runtime/src/primitives/fetch_archive.rs`: correct the live-on-main doc comment
- [x] Re-run the plan's sweep: `git grep` for the claims by meaning, excluding review and analysis records and published history

- **Done when**: the sweep returns only the nine declared specs' hits (tasks 11–19), 028 and 032 (task 10), and past-tense records (049, `runtime/CHANGELOG.md`, other specs' `plan.md` and `tasks.md` design records). AC7 and AC15 hold against the tree.

## 9. Pre-release exercise

- [x] In a scratch git repository, install the working-tree bootstrap and run `/ductus` with `--ref=main`, end to end, confirming all three fetches name `main` and `[source] ref = "main"` is recorded
- [x] Run it plain, confirming `latest` resolves to the current release and halts on the release floor with the pre-publication message, writing nothing past the Permission Setup seed
- [x] Run it with `--ref=` empty, an unknown value, `--ref` twice, and a nonexistent `ductus-v*` tag, confirming each halts naming its value
- [x] Set a `[migrations] last_applied` to an id present at `main` and absent from a lower tag's registry, and confirm the migration floor's lookup order (the lower tag is also below the release floor pre-release, so exercise the lookup by reading it rather than by relying on its halt)
- [x] Run `--ref=latest` with a recorded `main`, confirming the block is rewritten without `ref`

- **Done when**: each outcome above is observed, and the observations are recorded for the review. The latest-release and named-tag success paths, which need a release carrying this spec, are named as unobservable before release (plan, Trade-offs).

## 10. 028 and 032: the descriptor substitution

- [x] Replace the parenthetical *(live-on-main)* in `specs/028-antigravity-agent/spec.md` and `specs/032-opencode-agent/spec.md` with one uniform replacement, in its own commit

- **Done when**: the commit's diff is that one substitution and nothing else, and neither spec's status moved.

## 11. Discharge 003

- [x] `done → in-progress`, in its own commit
- [x] Correct the `curl-sh-installer` scenario (lines 15 and 17) and add a `> **Signpost:**` back-link to 061
- [x] Review (scoped to the reopen), analyze, and `→ done`

- **Done when**: 003 is `done` with a current review and analysis, and its scenario links back to 061.

## 12. Discharge 007

- [x] `done → in-progress`
- [x] Correct `ductus-self-update-precheck` (line 23) to `{framework-root}` and `{raw-ref}`, with a signpost to 061
- [x] Review, analyze, `→ done`

- **Done when**: 007 is `done` with a current review and analysis, and links back to 061.

## 13. Discharge 015

- [x] `done → in-progress`
- [x] Correct §Source and the framework-root lines (spec lines 36, 39, 49, 51, 61), and mark the Resolved Question deferring ref pinning as adopted by 061, with a signpost
- [x] Review, analyze, `→ done`

- **Done when**: 015 is `done` with a current review and analysis, and no longer describes ref pinning as deferred.

## 14. Discharge 023

- [x] `done → in-progress`
- [x] Annotate the read-fallback Resolved Question: its premise that no version can be pinned was superseded by 061, and its decision stands. Add a signpost
- [x] Review, analyze, `→ done`

- **Done when**: 023 is `done` with a current review and analysis, and links back to 061.

## 15. Discharge 026

- [x] `done → in-progress`
- [x] Restate `family-36-self-url-resolution` line 24's rationale and its slug-derivation description to match task 6, with a signpost
- [x] Review, analyze, `→ done`

- **Done when**: 026 is `done` with a current review and analysis, and its Family 36 scenario matches the script.

## 16. Discharge 029

- [x] `done → in-progress`
- [x] Correct `archive-fetch-direct-codeload` (lines 9, 16, 20) to `{archive-ref}` and `{framework-root}`, with a signpost
- [x] Review, analyze, `→ done`

- **Done when**: 029 is `done` with a current review and analysis, and links back to 061.

## 17. Discharge 048

- [x] `done → in-progress`
- [x] Correct spec line 59, data-model line 18, and `pin-is-readable-when-acquisition-needs-it` lines 12 and 48 to the pin read at the resolved ref
- [x] Restate *Version currency, and what the store gives up* with the cost from 061's store answer
- [x] Add the installer to the release asset set, and add a signpost
- [x] Review, analyze, `→ done`

- **Done when**: 048 is `done` with a current review and analysis, and its pin, store-cost and asset-set statements match 061.

## 18. Discharge 050

- [x] `done → in-progress`
- [x] Correct the Resolved Question *"Does a constitution-only change need a version bump?"* per 061's first resolved question, with a signpost
- [x] Review, analyze, `→ done`

- **Done when**: 050 is `done` with a current review and analysis, and the Resolved Question no longer rests on the archive tracking `main`.

## 19. Discharge 056

- [x] `done → in-progress`
- [x] Correct the Resolved Question at spec line 147 to address the archive half through `{framework-root}`, with a signpost
- [x] Review, analyze, `→ done`

- **Done when**: 056 is `done` with a current review and analysis, and links back to 061.

## 20. Close-out checks

- [x] Confirm `{ref-floor}` in both copies equals the version the release will be cut at
- [x] Run the full local gate: `cargo test` from `runtime/` (counting result lines against targets), `scripts/audit/run-all.sh`, the framework-checks steps, and `lint-markdown` over the changed markdown
- [x] Confirm that none of tasks 1–8's commits has reached `origin` ahead of the release (plan, Rollout)

- **Done when**: the gate is green with its counts recorded, the floor matches the planned release, and AC16's nine back-links resolve.

## 21. Disposition out-of-spec finding: cargo audit reports chacha20 0.10.1 as yanked

- [x] `runtime/Cargo.lock` — `chacha20` 0.10.1 (a dependency of `rand` 0.10.2) was yanked on crates.io on 2026-08-27, and 0.10.2 is current. `cargo audit` reports it as 1 allowed warning and exits 0. `cargo tree -e all --target all` finds no path to `rand` or `chacha20`, so neither crate is compiled into any target. The pin dates from d33c5333; 061 never touched `Cargo.lock`. The fix is `cargo update -p chacha20 --precise 0.10.2`, which the dry run shows changes that one entry — fixed

- **Done when**: the finding is fixed, routed, or discarded, with a discard's reason written on the task

## 22. Re-close 056 after the Family 36 correction 061's review routed there

- [x] 056 was reopened (e55a855d) and its consumer table and cross-spec-impact Resolved Question corrected to name the codeload `tar.gz/` URL as Family 36's subject (3e47e14f), routed by 061's review (1689d9af)
- [x] Run `/ductus:analyze` on 056: detection, then measure `unexamined` at `done` with the `set-status` round trip (not committed), then `write-analysis`, and commit
- [x] `check-review-gate` on 056 passes (its review stays current: 056 has no durable contracts, so its `reviewed-digest` is empty), then `set-status` in-progress → done, committed alone

- **Done when**: 056 is `done` with a current analysis, and `check-review-gate` on 056 reports it already done

## 23. Disposition out-of-spec finding: 057's Applicable Rules cites QUAL-CLAIM-001, whose code-path trigger cannot fire against spec artifacts

- [x] `specs/057-analyze-artifact-and-record-relocation/spec.md` `## Applicable Rules` cites `QUAL-CLAIM-001`. 061's analysis (ba106a11) judged, with the operator, that the rule's Verification is a code-path check for `/review` (`framework/rules/quality-cross.md`: "verify code patterns rather than design-time commitments"), so analyze step 14 fires an advisory on any `## Applicable Rules` citation of it. 057 is `done` and its analysis (426eae14) recorded 0 advisory; its next analyze would raise this finding. Surfaced at 061's completion gate, outside 061's scope — discarded (operator, 2026-09-28): the citation accurately names the rule 057's design applies, and the advisory concerns the check's scope rather than a defect in 057; reopening a done spec to move it would cost a full reopen, analyze and close cycle for no change a reader needs. If 057's next analyze fires it, that run decides it in 057's own record

- **Done when**: the finding is fixed, routed, or discarded, with a discard's reason written on the task

## 24. Disposition the 2026-10-20 analysis's blocking findings

061's first analysis against the rule files as rewritten on 2026-10-19 (two independent passes over all 297 rules at `b49e40e7`) found four MUST-tier findings, which block `done` and can be fixed or routed but not discarded:

- `BE-TIMEOUT-001`: the plan names no timeout for the bootstrap's host-side fetches Source resolution and the self-update check make (the latest-release redirect read, the pin at `{raw-ref}`, `migrations.toml` at the tag and at `main`, the bootstrap at `{raw-ref}`); only `install.sh`'s two requests are bounded.
- `BE-DATA-001`: the plan commits to TLS 1.2 or later only for the README one-liner, not for the installer's requests or the bootstrap's fetches.
- `BE-DATA-011`: no 061 artifact commits to certificate and host-name validation for those connections.
- `CFG-ENV-005`: the release-floor and migration-floor halts name the tag and the floor but not the setting the tag came from, so a recorded `[source] ref` below a floor halts without naming `[source] ref` or the active config file; the grammar halt already names its origin.

Measured 2026-10-20: `framework/bootstrap/ductus.md` (and its byte-identical `govern.md`) carries seven `curl` calls and none has a timeout or `--proto '=https' --tlsv1.2`: three in Source resolution (lines 214, 234, 250), the `main` registry fetch beside line 250, and the self-update fetch (line 484), all 061's; the two runtime-acquisition downloads (lines 386 and 388), 048's; and the markdown-only archive fetch (line 744), 015's, which the runtime path replaces with `fetch-archive`. `install.sh` already passes the TLS flags and the bounds, so its half is a plan statement alone. The same analysis found three SHOULD-tier gaps the plan closes with a sentence each (BE-DEPS-007, CFG-ENV-012, BE-BULK-002).

Answered (a) by the owner 2026-10-20, to: (a) fix all four in this run: 061's five bootstrap fetches gain the TLS flags and named bounds (the installer's 10 s and 60 s), both floor halts name the tag's origin, `govern.md` follows, and 061's plan and spec state it, so 061 needs one more analyze and nothing else reopens; (b) the same, and also bound 048's two acquisition downloads and the markdown-only archive fetch, so every bootstrap `curl` matches, which reopens 048 for its plan statement and an analyze against the rewritten rules; or (c) route the four to a later task on 061, which keeps 061 out of `done`, and the release needs it there.

- [x] The five bootstrap fetches 061 owns (the latest-release redirect read, the pin, the two registry reads, the self-update fetch) pass `--proto '=https' --tlsv1.2` and the named bounds `{connect-timeout}` (10 s) and `{request-timeout}` (60 s) from the Derived paths table, and `framework/bootstrap/govern.md` follows byte for byte
- [x] Both floor halts name where the tag came from (`{tag origin}`), and the installer's named-tag floor halt says `from --ref`, which `scripts/tests/test-install.sh` case M checks
- [x] plan.md states the TLS floor, certificate validation and bounds for the bootstrap's and the installer's fetches, the halts' origin, and the three SHOULD sentences (BE-DEPS-007, CFG-ENV-012, BE-BULK-002); spec.md's Failure behavior, AC10 and AC17 name the origin

- **Done when**: 061's re-check produces none of the seven findings, and `check-review-gate` on 061 passes
