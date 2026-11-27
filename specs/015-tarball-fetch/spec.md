---
title: "015-tarball-fetch — spec"
status: done
dependencies: [007-govern-workflow, 012-multi-agent-govern, 029-bootstrap-runtime-autowire]
tags: [bootstrap, performance]
next-criterion: 15
---

# 015 — Tarball Fetch

Collapse `/ductus`'s ~35–50 individual `curl` fetches into a single archive download, extracted once into a temp directory and resolved as local paths. The manifest, strategies (`update`/`create`/`skip`/`merge`/`pinned`), and per-agent scaffolding flow are unchanged — only the **File Fetching** section's transport is replaced.

## Problem

Before this spec, `framework/bootstrap/ductus.md` issued one `curl` per file in the manifest. A single-agent run touched:

- ~14 `ductus`-owned shared files (constitution, rules, templates, registry)
- ~4 project-specific shared files (system, errors, events, inbox)
- 2–3 conditional shared files (AGENTS.md, CLAUDE.md, gitignore template)
- 1–N per-language gitignore patterns from `github.com/github/gitignore`
- ~14 slash command sources (13 in `framework/commands/` + 1 agent-specific configure)
- 1 `framework/bootstrap/ductus.md` (self-install)
- 0–N workflow templates (only those the user accepts)

That was ~35 fetches for one agent, ~50+ for two agents on first run, and the same volume on every routine re-run because `update`-strategy files had to be fetched before the content-equality check could decide whether to write. The bottleneck was per-call tool-invocation overhead, not bytes (see **Tradeoffs**, Bytes over the wire).

A single archive fetch removes that overhead while keeping every other guarantee `/ductus` makes today (idempotency, per-file strategies, pinning, failure granularity within the manifest pass).

## Behavior

### Source

`/ductus` issues exactly one `curl` for the framework against GitHub's archive host — the direct `codeload.github.com` endpoint, at the ref the run resolved:

```text
https://codeload.github.com/stonean/ductus/tar.gz/{archive-ref}
```

`{archive-ref}` is `refs/heads/main` on the `main` source and `refs/tags/{tag}` on a release; the default source is the latest release. This is the target that `https://github.com/stonean/ductus/archive/{archive-ref}.tar.gz` 302-redirects to; fetching it directly avoids a cross-host redirect that some agent hosts gate with a permission prompt even when `curl` is pre-granted (see [029 `archive-fetch-direct-codeload`](../029-bootstrap-runtime-autowire/scenarios/archive-fetch-direct-codeload.md)). GitHub names the archive's top-level directory after the ref — `ductus-main/` for `main`, `ductus-ductus-v0.55.0/` for that tag — so the framework root is derived from the extraction rather than predicted (see **Extract**).

> **Signpost:** the ref this section fetches at, and the framework root it derives, are [061 — Updates track the latest release tag](../061-updates-track-the-latest-release-tag/spec.md)'s. Every fetch a run makes back into the `ductus` repository — the version pin, the self-update bootstrap, and this archive — names one source resolved once per run: the latest release by default, or `main` or a named release tag through `--ref`, recorded in `.ductus/config.toml` `[source] ref`. This spec shipped against `main` and deferred exactly that option; the Tradeoffs entry and the Resolved Question below record the deferral and its adoption.

External fetches that are **not** part of the `ductus` repo are unchanged: per-language `.gitignore` patterns continue to come from `https://raw.githubusercontent.com/github/gitignore/main/{Language}.gitignore` as separate `curl` calls. They are not in the archive, and bundling them is out of scope.

### Extract

The archive lands in a **new** temp directory created on every run: `mktemp -d -t ductus-XXXXXX`, which the Pre-flight Phase runs before anything is fetched into it (`framework/bootstrap/ductus.md`, Pre-flight Phase). On macOS/Linux this lands under `$TMPDIR` or `/tmp`. Never reuse a directory from a prior run, even if one is still on disk — a fresh fetch is the only way `/ductus` picks up upstream changes, so the archive must be re-downloaded each invocation. After fetching the archive:

1. Extract the archive into the temp directory: `tar -xzf {archive} -C {tempdir}`.
2. Derive the framework root, `{framework-root}`: the single top-level directory the extraction produced. Treat it for the rest of the run as the local copy of the files a run reads from the `ductus` repo, which the archive carries and nothing else (AC14).

If the fetch or extraction fails — non-zero exit, an extraction that produced zero or several top-level directories, or any required manifest entry absent from the extract — abort the run with a clear error:

> Failed to fetch or extract the `ductus` archive ({reason}). Anything it fetched is left in `{tempdir}` for inspection. Re-run after checking network connectivity, or report this if it persists.

Aborting on archive failure is intentional and a behavior change from the current per-file warning model: a missing archive means **every** file is missing, so there is nothing to scaffold partially. Per-file granularity within the extract is preserved (see **Per-file resolution** below).

### Per-file resolution

The manifest's source paths (e.g., `framework/constitution.md`, `framework/commands/specify.md`) are now resolved against the extracted framework root rather than concatenated with the raw URL prefix. For each manifest entry:

1. Compute the local source path: `{framework-root}/{source-path}`.
2. If the local source path does not exist — the file was renamed, removed upstream, or the manifest is out of sync — warn `Source not found in archive: {source-path}; skipping.` and continue with the remaining entries. This preserves the current "do not abort on a single fetch error" guarantee.
3. Apply the existing strategy (`update`, `create`, `skip`, `merge`, `pinned`) using the local file as the new content. Content comparison for `update` strategy is a local file diff against the destination — same semantics as today, just no network round-trip.
4. Apply placeholder substitution after reading the local source, before writing to the destination. Same rules as today (including the `ductus.md` self-install exception that keeps `{project}` and `{cli-config-dir}` literal).

### Cleanup

`/ductus` does not delete the temp directory. The path is logged in the run summary (and, on abort, in the error message) so the user can inspect it if needed. Both macOS (`/var/folders/.../T/`) and Linux (`/tmp` on systemd-tmpfiles distros) are assumed to sweep their temp directories automatically, as their default temp policies do; about 1.3 MB of extracted files (1,312,690 bytes across 78 files, the member sizes of the `codeload.github.com` tarball of `main` at `d56ff370`, summed on 2026-11-23) waiting for the next sweep is acceptable in exchange for not granting an `rm -rf` permission to the bootstrap.

The leftover directory is for inspection only — the next `/ductus` run creates its own fresh temp directory via `mktemp` and never reuses a prior extract.

### Permission bootstrap

The tarball flow needs `mktemp` and, on Claude, pre-allowed `Read` globs that cover any `ductus-*` temp directory. There is no `tar` allow, since `tar` runs any command (022's task 18, question 2), and no `rm` addition — the temp directory is left for the OS to sweep (see **Cleanup** above).

Update each registry row's `settings_template`:

- **Claude:** add `Bash(mktemp -d -t ductus-XXXXXX)`, the exact command a step runs (`framework/bootstrap/ductus.md` §Permission Setup retires the broad `Bash(mktemp *)`), and three `Read(...)` globs covering both macOS temp roots and Linux `/tmp`, in the double-leading-slash form (see "Why the double-leading-slash form" below):
  - `Read(//private/var/folders/**/T/ductus-*/**)` (macOS canonical path)
  - `Read(//var/folders/**/T/ductus-*/**)` (macOS non-canonical, defensive)
  - `Read(//tmp/ductus-*/**)` (Linux)

The merge logic in **Permission Setup** is unchanged — entries are added if missing, never reordered or deduplicated. Existing adopters get the new entries on their next routine `/ductus` re-run, before any `tar`, `mktemp`, or extracted-archive read is performed.

#### Why pre-allow the temp-path Read globs

Claude Code's permission system records a `Read(...)` allow with the **exact absolute path** when the agent first reads a file outside the project root and the user accepts the prompt. Combined with the spec's mandate that every `/ductus` run gets a fresh `mktemp` directory, that means: without pre-allowed globs, every run prompts once and writes a new `Read(...)` entry into `settings.local.json` for the run's unique path. Those entries accumulate forever and never match a future run.

Pre-allowing the globs at bootstrap solves this in a small fixed set of entries — future Read calls against any `ductus-*` temp path match the glob and skip the prompt. The entries are scoped to `ductus-*` directories under known temp roots, so they cannot grant Read on unrelated files.

#### Why the double-leading-slash form

Claude Code anchors a permission pattern as its documentation says: "A pattern like `/Users/alice/file` isn't an absolute path. The single leading slash anchors at the settings source, not the filesystem root. Use `//Users/alice/file` for absolute paths" (code.claude.com/docs/en/permissions). A single-slash rule in `.claude/settings.local.json` therefore names a path under the project, not the temp directory, so the seed carries the three `//` forms alone and `framework/bootstrap/ductus.md` §Permission Setup retires the single-slash ones.

> **Signpost ([022-deterministic-runtime](../022-deterministic-runtime/spec.md)):** 022's task 18 settled the anchoring above, so the seed carries only the three `//` forms.

#### One-time cleanup of stale per-run entries

Adopters who ran the tarball flow before this fix shipped will already have several `Read(/private/var/folders/.../T/ductus-XXXXXX.{suffix}/...)` entries in their `settings.local.json`. The bootstrap merge does not delete them (per the spec's "never reorder or deduplicate" rule). Adopters can remove the stale entries manually — they are inert (the new glob covers any future case) but cosmetically noisy.

### Self-update notice

Before this spec, the integrity check that re-fetches `ductus.md` on a corrupted write re-ran `curl`. With a tarball, re-fetch means reading the same file again from the extracted archive — no second network call. The check itself is unchanged; it just operates on the local source.

The self-update notice (shown when the installed `ductus.md` differs from the fetched version) continues to fire. The comparison runs ahead of the archive: `framework/bootstrap/ductus.md` §Self-update check fetches the bootstrap on its own in pre-flight, at the same resolved ref as the archive, so the copy it compares against is the one the archive carries for that ref.

## Tradeoffs

- **One archive failure vs. many per-file warnings.** Under the per-file flow a single 404 on `framework/templates/project/inbox.md` produced a warning and the rest of the manifest proceeded. With a tarball, that file would simply not exist in the extract, producing the same per-entry warning. The genuine new failure mode is the archive itself failing — and that's a clean abort, since partial scaffolding from a missing archive is impossible.
- **New permissions.** For Claude, one `Bash` addition, the exact `Bash(mktemp -d -t ductus-XXXXXX)` (no `tar`, as AC7 records), plus three `Read(...)` globs covering `ductus-*` temp paths under macOS (`/private/var/folders/**/T/`, `/var/folders/**/T/`) and Linux (`/tmp/`), in the double-leading-slash form because Claude Code anchors a single-slash pattern at the settings source (see **Permission bootstrap → Why the double-leading-slash form**). Cost is one-time per adopter, applied on the same `/ductus` run that introduces the change. No `rm` allow is needed because the temp directory is left for the OS to sweep.
- **Bytes over the wire.** The archive carries only the files a run reads (AC14): 78 files and 403,618 bytes compressed for `main` at `d56ff370`, fetched from `codeload.github.com` on 2026-11-23. Untrimmed, it was the whole repository, 1,424 files and 6,096,062 bytes compressed at `ductus-v0.64.0` (`git archive --format=tar.gz` of the tag, which matched codeload's own 3,523,152 bytes at `ductus-v0.59.11` to within 0.06%). Against per-file fetches the design saves round-trips and tool-call invocations, and perceived run time.
- **Loss of partial progress.** Under the per-file flow a network failure mid-fetch produced ~10 successful files plus warnings on the rest; with a tarball, a network failure produces zero files and a clean abort. Both outcomes leave the project in a recoverable state — re-run resumes idempotently.
- **Pinning at a ref.** When this spec shipped, every fetch was hardcoded to `main` and the tarball URL pointed at `main` too. A `.ductus/config.toml` `[source] ref` option overriding the ref was named as a natural follow-up and left out of scope — see **Resolved Questions**. **Adopted by 061**, which resolves the ref per run and defaults it to the latest release rather than `main`.

## Acceptance Criteria

- [x] AC1: `framework/bootstrap/ductus.md`'s **File Fetching** section is replaced with the archive-fetch + extract + local-path-resolution flow above
- [x] AC2: A successful `/ductus` run on a single-agent project downloads the framework as one archive, which is one `curl` against the `ductus` repo (plus per-language gitignore fetches, which remain unchanged). Pre-flight makes small fetches of its own ahead of it: the runtime version pin (048), the self-update bootstrap (007's `ductus-self-update-precheck`), the latest-release redirect (061) and, when a migration has been applied, one or two registry reads for the migration floor (061).
- [x] AC3: All existing manifest strategies (`update`, `create`, `skip`, `merge`, `pinned`) behave identically to today, sourcing files from the extracted archive
- [x] AC4: A failed archive fetch produces a clean abort with a clear error message and no partial scaffolding
- [x] AC5: A missing source file within the archive produces a per-entry warning and the remaining manifest continues
- [x] AC6: The temp directory path is logged in the run summary (and on abort, in the error message); `/ductus` does not delete it
- [x] AC7: Claude's `settings_template` in the registry adds the exact `Bash(mktemp -d -t ductus-XXXXXX)` allow entry and no `tar` entry, since `tar` runs any command, applied via the existing **Permission Setup** merge; no `rm` allow is added. Pi has no permission-gating settings, so its `settings_template` is `{}` by design and carries neither.
- [x] AC8: The Claude `settings_template` also adds `Read(...)` globs for `ductus-*` temp paths under both macOS temp roots and Linux `/tmp`, so per-run `Read(...)` entries do not accumulate across `/ductus` invocations
- [x] AC9: The **Post-Write Integrity Check** for `ductus.md` works against the local source — no additional `curl`
- [x] AC10: The self-update notice continues to fire when the installed `ductus.md` differs from the archive's copy
- [x] AC11: Per-language gitignore fetches against `github.com/github/gitignore` remain unchanged (separate `curl` calls)
- [x] AC12: A re-run on an already-adopted project still reports `unchanged` for files whose archive copy matches the destination
- [x] AC13: Each `/ductus` invocation creates a fresh temp directory via `mktemp` and re-fetches the archive; a prior run's extracted directory is never reused as the source for the current run
- [x] AC14: The framework archive `/ductus` downloads carries exactly the files a `/ductus` run reads from it and no other tracked file, and a check over `git archive` of a commit fails both when the archive omits a file the run reads and when it carries a file no step of the run reads

## Open Questions

*None — all resolved.*

## Resolved Questions

- **`.ductus/config.toml` ref pinning** — defer. `.ductus/config.toml` already supports additive sections (currently `[pinned]`), so a `[source] ref = "..."` option can be added in a later spec without migration cost. No adopter has asked for ref pinning, and `main` parity with the current per-file flow keeps this spec's scope to the transport change. v0.1.0 is tagged, but until there is concrete demand, every adopter would still pin to `main` — adding the surface now means documenting and maintaining a feature nobody is using. **Adopted 2026-09-28 by 061**, which adds `[source] ref` and a `--ref` flag and makes the latest release the default. The trigger was not an adopter asking to pin but an update landing on an unfinished series on `main`; 061's Motivation records it.
- **Fallback to per-file fetch on archive failure** — no fallback. `codeload.github.com` and `raw.githubusercontent.com` are both GitHub-fronted CDNs with correlated availability, so partial outages affecting only one are rare. Maintaining two transport code paths is a permanent tax for a transient failure mode; `/ductus` is idempotent and a re-run after the outage clears is the same recovery path used for any transient `curl` failure today. The clean abort already tells the user what happened.
- **`rm` permission scope** — drop the `rm` permission entirely; let the OS sweep the temp directory. macOS (`/var/folders/.../T/`) and Linux (`/tmp` via systemd-tmpfiles) both auto-purge their temp roots, so a few hundred KB of extracted files waiting for the next sweep is acceptable. Granting `rm -rf` — even scoped — is a sharper allow than the rest of ductus's bootstrap permissions (`curl`, `ls`, `tar`, `mktemp` are all read-or-create) and adopters with custom `$TMPDIR` would have to edit settings to keep cleanup working. Skipping `rm` removes that friction.

## References

Declared dependencies for this spec, surfaced here so the `derive-dependencies` runtime primitive sees them in the body.

- [007-govern-workflow](../007-govern-workflow/spec.md)
- [012-multi-agent-govern](../012-multi-agent-govern/spec.md)
