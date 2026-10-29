# 048 — Ductus-Acquired Runtime Data Model

Three artifacts this feature introduces, plus the layout they describe. The `[runtime]` config section is owned here, following the convention that the introducing spec owns its `.ductus/config.toml` section ([030](../030-cross-service-references/spec.md) owns `[services]` the same way).

## The `version` file

A repo-root file named `version`, containing exactly one line: a SemVer string with no `v` prefix, no leading or trailing whitespace beyond the single trailing newline.

```text
0.28.0
```

| Property | Value |
| --- | --- |
| Path | `version` (repo root) |
| Format | one SemVer line, `MAJOR.MINOR.PATCH` |
| Written by | the release commit, by hand, alongside `runtime/Cargo.toml` and `runtime/CHANGELOG.md` |
| Read by | `/ductus`, fetched from `raw.githubusercontent.com/stonean/ductus/{raw-ref}/version` — the ref the run resolved, the latest release by default — into the pre-flight temp directory and read there |
| Meaning | the runtime version this framework revision requires |

**The agreement invariant.** These four must carry the same value, and a self-audit family asserts it:

| Artifact | Location |
| --- | --- |
| `version` | repo root |
| `runtime/Cargo.toml` | `version = "…"` under `[package]` |
| `runtime/CHANGELOG.md` | the newest `## [X.Y.Z]` heading |
| the release tag | newest `ductus-v*` by SemVer |

A pre-tag window exists by construction: the release commit advances the first three, and the tag follows moments later. The audit family runs against the working tree, so it compares the first three strictly and treats a newest-tag lag as expected rather than as a finding. The tag is checked where it exists: the release workflow's `audit` job runs `scripts/check-release-tag.sh`, and `publish` waits for that job, so a tag naming another version stops the release before anything is published.

## The `[runtime]` config section

```toml
[runtime]
path = "runtime/target/release/ductus"
```

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `path` | string | no | A binary the project supplies itself, relative to the repo root or absolute. When set, `/ductus` performs no download and resolves the pointer to this path. |

Absent (the normal case) means `/ductus` acquires the pinned version into the store. Present means the project has taken responsibility for supplying the runtime — the supported route for building from source, for an air-gapped or firewalled checkout, and for a platform with no published asset.

Behavior when set:

- No download is attempted, and no store write occurs.
- The pointer resolves to the configured path.
- The binary's reported version is compared against the pin, and a mismatch **warns** rather than halts — a project naming a path has stated deliberately which binary it wants, and a development build is expected to run ahead of the last release.
- A path that does not exist, or will not execute, **halts** naming the configured path. Never a silent fall-through to downloading, which would discard the project's stated choice without saying so.

## Filesystem layout

| Path | Scope | Committed | Description |
| --- | --- | --- | --- |
| `~/.ductus/bin/ductus` | machine | no (outside any repo) | The store. One binary per machine, executable bit set, written only by `/ductus`. `ductus.exe` on Windows. |
| `.ductus/bin/ductus` | project | no (gitignored) | The pointer. Symlink where creation succeeds, copy otherwise. Named by every `project-committed` MCP config. |

The `.gitignore` block gains `.ductus/bin/`, joining the anchored `/.ductus/session.toml` line that already establishes per-path ignores under a committed `.ductus/` directory ([042](../042-consolidate-govern-per-project-files-under-govern-directory/spec.md)).

## Release assets

Published under `https://github.com/stonean/ductus/releases/download/ductus-v{version}/`, one archive plus one sidecar per target. All five are required for a release to publish.

| Target triple | Asset | Sidecar |
| --- | --- | --- |
| `aarch64-apple-darwin` | `ductus-aarch64-apple-darwin.tar.gz` | `.sha256` |
| `x86_64-apple-darwin` | `ductus-x86_64-apple-darwin.tar.gz` | `.sha256` |
| `x86_64-unknown-linux-gnu` | `ductus-x86_64-unknown-linux-gnu.tar.gz` | `.sha256` |
| `aarch64-unknown-linux-gnu` | `ductus-aarch64-unknown-linux-gnu.tar.gz` | `.sha256` |
| `x86_64-pc-windows-msvc` | `ductus-x86_64-pc-windows-msvc.tar.gz` | `.sha256` |

The complete set the release workflow asserts before publishing carries more than these five: the SBOM (`ductus.cdx.json` plus its `.sha256` sidecar) and, since [061](../061-updates-track-the-latest-release-tag/spec.md), the installer `install.sh`, staged from a checkout at the tag. The installer has no sidecar: it is fetched over HTTPS and piped to `sh`, and a post-release job compares the published copy byte for byte with the tag's.

Windows publishes `.tar.gz` rather than `.zip` so extraction is `tar` on every platform — Windows 10+ ships `bsdtar`, and `tar` is already granted in all four permission grammars, so `unzip` never enters the permission surface.

## Detection states

Replaces [029](../029-bootstrap-runtime-autowire/spec.md)'s three-state model. `PATH` is not consulted in either state.

| State | Condition | Behavior |
| --- | --- | --- |
| **A** | a `ductus`-namespaced MCP tool is in the session's inventory | Runtime live. Deterministic path; no pre-flight acquisition work; contributes nothing to either restart set. |
| **B** | no `ductus`-namespaced tool | Resolve the binary (acquire, or use `[runtime]`), materialize the pointer, wire the MCP config, add tool permissions, join the deferred-restart set, then **continue the run through the CLI** at the pointer path and surface in the single **Closing restart** at the end. |

State B originally aborted in pre-flight. The scenario `state-b-continues-in-session` moved that single restart to the end of the run, so the phase now sorts restart-requiring writes into two sets: a **pending-restart set** that aborts immediately (a stale installed `ductus.md`) and a **deferred-restart set** that does not (State B's acquisition and wiring), carried to the **Closing restart**. Only the first stops the run.

## Acquisition command set

Each step, and the permission grammar entry it needs. `curl`, `tar`, `chmod`, `ls`, and `mktemp` are already seeded; the rest are added by this feature.

| Step | Command | Seeded today |
| --- | --- | --- |
| fetch archive + sidecar | `curl` | yes |
| verify digest | `shasum -a 256` (macOS) / `sha256sum` (Linux) / `certutil -hashfile` (Windows) | **no** |
| create the store directory | `mkdir` | **no** |
| extract | `tar` | yes |
| set the executable bit | `chmod` | yes |
| materialize the pointer | `ln` (falling back to `cp`) | **no** |
| probe the version | executing the store path | **no** |

The verification entry is the one that matters most. Leaving it unseeded does not add a safety check — it places a permission prompt at the one gate that must never be waved through, and a prompt that appears on every bootstrap trains the reflex to approve it. The gate that protects the adopter is the digest comparison itself, which halts before anything is written, and it is stronger unattended than dependent on a dialog read during setup.
