# 048 — Govern-Acquired Runtime Plan

Implements [048 — Govern-Acquired Runtime](spec.md).

## Overview

Three deliverables that must land as one release, because each is incoherent without the others:

1. **Acquisition** — `/govern` downloads, verifies, and installs the runtime into `~/.govern/bin/`, exposes it per project through a gitignored `.govern/bin/gvrn` pointer, and registers that with each agent's MCP config.
2. **The requirement** — the constitutional amendment that makes the runtime mandatory, plus the reference sweep and the two cross-spec reopens it forces.
3. **The release surface** — a repo-root `version` file as the lockstep pin, a publish gate so a tag cannot ship a partial asset set, and a Windows asset published as `.tar.gz`.

The ordering constraint that shapes everything else: **acquisition cannot use the runtime.** `fetch-archive` and `extract-archive` are `gvrn` primitives, and `gvrn` is what is being acquired. Every acquisition step is host shell work expressed in `framework/bootstrap/govern.md`'s procedure and pre-authorized in four permission grammars. No new primitive is written for this feature.

## Technical Decisions

### Acquisition is host shell work, and that is not a workaround

The runtime exposes `fetch-archive` (checksum-verified download) and `extract-archive` (traversal-safe expansion), and both are exactly what acquisition needs. Neither can be used: the pre-flight phase that acquires the binary runs when the binary is, by definition, absent.

So acquisition is a shell sequence in `govern.md`'s markdown procedure — `curl`, a checksum tool, `tar`, `mkdir`, `chmod`, the pointer command — and the security properties `fetch-archive` provides in code are restated as procedure steps that the four agent permission seeds pre-authorize. This is the one place in the framework where a security-relevant sequence has no primitive backing it, and it is worth naming rather than discovering later.

The bootstrap already contains the precedent: `govern.md:484` fetches the framework archive with a direct `curl` to `codeload.github.com`, chosen over the redirect form because a cross-host redirect defeats a pre-granted `curl` permission. Acquisition follows the same shape and the same reasoning.

### The pin lives in a repo-root `version` file

One SemVer line. `runtime/Cargo.toml`, the newest `runtime/CHANGELOG.md` heading, the `gvrn-v{version}` tag, and this file all carry the same number, bumped in the release commit.

`/govern` reads it from the extracted archive at `{staging-dir}/version` — the same download that carried `framework/`, so the pin cannot disagree with the framework revision it describes. Grounding: `framework/bootstrap/govern.md:484` fetches `codeload.github.com/stonean/govern/tar.gz/refs/heads/main`, whose top-level directory is `govern-main/`, so the file resolves at `govern-main/version` after extraction.

A self-audit family asserts the four artifacts agree (spec AC15). That check is what makes the single-number invariant real rather than a convention someone remembers.

### The pointer: symlink attempted, copy on failure

`.govern/bin/gvrn` must resolve to the store without elevated privileges on any supported platform. Rather than branching on OS, `/govern` attempts a symlink and falls back to a copy when creation fails. Windows without developer mode lands on the copy; Windows *with* it, and every Unix platform, gets the symlink.

One code path, self-adapting, and no platform detection to keep in sync with the target-triple table. The cost is that a Windows copy consumes disk per project — accepted, since the store's primary win is one *download* per machine, and the fallback is invisible to every other surface.

### The version probe executes the binary

`{store}/gvrn --version` prints `gvrn {version}` (verified: the installed binary emits `gvrn 0.27.2`). `/govern` parses it and compares against the pin.

A recorded marker file was rejected in the spec: this feature *sanctions* hand-placing a binary into the store, so a marker would be stale or absent in exactly the supported cases. Executing the binary also fails usefully — a store entry that will not run reports no version, reads as "no usable runtime", and re-acquires.

### Detection states collapse from three to two

[029](../029-bootstrap-runtime-autowire/spec.md) defined **A** (runtime live), **B** (binary on `PATH`, unwired), **C** (absent → markdown path plus a tip). With acquisition, `PATH` is never consulted and absence is work to perform rather than a state to report:

| State | Condition | Behavior |
| --- | --- | --- |
| **A** | a `gvrn`-namespaced MCP tool is in the session inventory | unchanged — deterministic path, no pre-flight work |
| **B** | no such tool | acquire (or resolve the `[runtime]` supplied binary), wire the MCP config, add tool permissions, join the pending-restart set |

Former State C's tip in §Post-Scaffolding Output is deleted, not repurposed: with the runtime required there is no degraded-but-working outcome left to advertise.

### The amendment and the sweep land together

`framework/constitution.md` §runtime-boundary principle 3 and the Opt-in invariant are replaced; §text-first-artifacts' "usable standalone" narrows to the artifacts. Landing that ahead of the reference sweep would leave the constitution declaring a requirement while 26 per-step fallback sentences still tell hosts what to do without a runtime — a window in which the canonical source and the commands disagree.

Per §cross-spec-impact, [021](../021-runtime-boundary/spec.md) (which owns the opt-in invariant, its AC1 and AC2) and [029](../029-bootstrap-runtime-autowire/spec.md) (which owns the detection states) record the change in their own bodies and reopen to `in-progress` in the same commit.

### The release surface changes are independently landable

The `version` file, the publish gate, and the Windows `.tar.gz` asset touch only `runtime-release.yml` and the repo root. They can ship before anything else and are safe on their own — a `version` file nothing reads yet, and a gate that only fires on a partial matrix. Sequencing them first means the acquisition work is written against a release surface that already behaves the way it assumes.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `version` | Create | The repo-root SemVer pin, read from the fetched archive |
| `.github/workflows/runtime-release.yml` | Modify | Publish gate on the complete asset set; Windows asset as `.tar.gz` |
| `.github/workflows/markdown-only-pipeline.yml` | Delete | Asserts the retired opt-in invariant |
| `.github/workflows/acquisition.yml` | Create | Replacement: end-to-end acquisition on each runner platform |
| `framework/constitution.md` | Modify | §runtime-boundary principle 3 + Opt-in invariant; §text-first-artifacts narrowing |
| `framework/bootstrap/govern.md` | Modify | Acquisition procedure, MCP shapes, detection states, permission seeds, gitignore block, Shared Files note |
| `framework/commands/*.md` | Modify | Remove the 26 per-step markdown-only fallback instructions |
| `framework/migrations.toml` | Modify | Adopter migration entry rewriting the MCP command |
| `framework/migrations/runtime-path-rewrite.md` | Create | That entry's procedure body |
| `scripts/audit/version-agreement.sh` | Create | Self-audit family asserting the four version artifacts agree |
| `scripts/audit/run-all.sh` | Modify | Register the new family |
| `README.md` | Modify | Drop the `PATH` install; correct the Windows cross-compilation claim |
| `.mcp.json` | Modify | This repo's own registration → the pointer |
| `.govern/config.toml` | Modify | This repo's `[runtime]` key → its build output |
| `.gitignore` | Modify | `.govern/bin/` entry |
| `specs/021-runtime-boundary/spec.md` | Modify | Record the amendment; reopen to `in-progress` |
| `specs/029-bootstrap-runtime-autowire/spec.md` | Modify | Record the collapsed detection states; reopen to `in-progress` |
| `AGENTS.md` | Modify | Replace the stale-binary gotcha with the `[runtime]` key workflow |

## Trade-offs

**One version per machine.** The store holds a single binary, so two projects pinned to different versions cannot both be correct simultaneously; whichever ran `/govern` last wins until the other runs it. Rejected alternatives: a versioned store (`gvrn-{version}`), which fixes coexistence but not the home-config agents whose single MCP entry still names one version; and per-project binaries, which fix both but leave two of four agents unable to express a project-local path at all. Accepted because it is strictly better than the `PATH` behavior it replaces — govern owns the path and every run corrects it — and because it is the only shape uniform across all four agents.

**A gitignored artifact the repo cannot verify.** The pointer is per-contributor and outside version control, so nothing in CI proves an adopter's pointer is correct. Mitigated by making it cheap to repair: a missing or dangling pointer is the expected state of any un-bootstrapped checkout, and `/govern` recreates it without ceremony.

**Adoption is now gated on a download.** Firewalled, air-gapped, and unpublished-platform adopters move from *degraded* to *blocked* until they use the `[runtime]` supplied-binary key. This is the deliberate cost of the requirement; the mitigation is that the halt names the store path and the release URL, so the recovery is discoverable at the moment it is needed rather than in documentation the reader has not opened.

**A security-relevant sequence with no primitive behind it.** Download, checksum, extract, and install run as shell steps because the runtime is unavailable at that moment. The verification gate is therefore procedure rather than code, and its correctness depends on `govern.md` being followed. Accepted as unavoidable; noted here so a later reader does not mistake it for an oversight.

**Pre-granting execution of a downloaded binary.** The permission seeds authorize running the store path without a prompt. Safe only because verification precedes it and a mismatch halts before anything is written — but it is a real widening of what the bootstrap may do unattended, and it is stated in the spec rather than buried in a settings template.
