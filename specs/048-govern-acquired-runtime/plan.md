# 048 — Ductus-Acquired Runtime Plan

Implements [048 — Ductus-Acquired Runtime](spec.md).

## Overview

Three deliverables that must land as one release, because each is incoherent without the others:

1. **Acquisition** — `/ductus` downloads, verifies, and installs the runtime into `~/.ductus/bin/`, exposes it per project through a gitignored `.ductus/bin/ductus` pointer, and registers that with each agent's MCP config.
2. **The requirement** — the constitutional amendment that makes the runtime mandatory, plus the reference sweep and the two cross-spec reopens it forces.
3. **The release surface** — a repo-root `version` file as the lockstep pin, a publish gate so a tag cannot ship a partial asset set, and a Windows asset published as `.tar.gz`.

The ordering constraint that shapes everything else: **acquisition cannot use the runtime.** `fetch-archive` and `extract-archive` are `ductus` primitives, and `ductus` is what is being acquired. Every acquisition step is host shell work expressed in `framework/bootstrap/ductus.md`'s procedure and pre-authorized in four permission grammars. No new primitive is written for this feature.

## Technical Decisions

### Acquisition is host shell work, and that is not a workaround

The runtime exposes `fetch-archive` (checksum-verified download) and `extract-archive` (traversal-safe expansion), and both are exactly what acquisition needs. Neither can be used: the pre-flight phase that acquires the binary runs when the binary is, by definition, absent.

So acquisition is a shell sequence in `ductus.md`'s markdown procedure — `curl`, a checksum tool, `tar`, `mkdir`, `chmod`, the pointer command — and the security properties `fetch-archive` provides in code are restated as procedure steps that the four agent permission seeds pre-authorize. This is the one place in the framework where a security-relevant sequence has no primitive backing it, and it is worth naming rather than discovering later.

The bootstrap already contains the precedent: `ductus.md:484` fetches the framework archive with a direct `curl` to `codeload.github.com`, chosen over the redirect form because a cross-host redirect defeats a pre-granted `curl` permission. Acquisition follows the same shape and the same reasoning.

### The pin lives in a repo-root `version` file

One SemVer line. `runtime/Cargo.toml`, the newest `runtime/CHANGELOG.md` heading, the `ductus-v{version}` tag, and this file all carry the same number, bumped in the release commit.

As planned, `/ductus` read it from the extracted archive — the same download that carried `framework/`, so the pin could not disagree with the framework revision it describes. Grounding at the time: `framework/bootstrap/ductus.md:484` fetches `codeload.github.com/stonean/ductus/tar.gz/refs/heads/main`, whose top-level directory is `ductus-main/`, so the file resolved at `ductus-main/version` after extraction. **That decision did not survive implementation.** Acquisition runs in the Pre-flight Phase, which precedes the archive fetch, so the file was never on disk when the step read it. Task 16 (`pin-is-readable-when-acquisition-needs-it`) moved the read to a direct fetch of `raw.githubusercontent.com/stonean/ductus/main/version` into `{tempdir}`, and retired `{staging-dir}` from the procedure.

A self-audit family asserts the three files agree, and the release workflow refuses a tag that names another version before anything is published (`scripts/check-release-tag.sh`, spec AC15). Those two checks are what make the single-number invariant real rather than a convention someone remembers.

### The pointer: symlink attempted, copy on failure

`.ductus/bin/ductus` must resolve to the store without elevated privileges on any supported platform. Rather than branching on OS, `/ductus` attempts a symlink and falls back to a copy when creation fails. Windows without developer mode lands on the copy; Windows *with* it, and every Unix platform, gets the symlink.

One code path, self-adapting, and no platform detection to keep in sync with the target-triple table. The cost is that a Windows copy consumes disk per project — accepted, since the store's primary win is one *download* per machine, and the fallback is invisible to every other surface.

### The version probe executes the binary

`{store}/ductus --version` prints `ductus {version}` (verified when this was planned, against a pre-rename binary that emitted `gvrn 0.27.2`). `/ductus` parses it and compares against the pin.

A recorded marker file was rejected in the spec: this feature *sanctions* hand-placing a binary into the store, so a marker would be stale or absent in exactly the supported cases. Executing the binary also fails usefully — a store entry that will not run reports no version, reads as "no usable runtime", and re-acquires.

### Detection states collapse from three to two

[029](../029-bootstrap-runtime-autowire/spec.md) defined **A** (runtime live), **B** (binary on `PATH`, unwired), **C** (absent → markdown path plus a tip). With acquisition, `PATH` is never consulted and absence is work to perform rather than a state to report:

| State | Condition | Behavior |
| --- | --- | --- |
| **A** | a `ductus`-namespaced MCP tool is in the session inventory | unchanged — deterministic path, no pre-flight work |
| **B** | no such tool | acquire (or resolve the `[runtime]` supplied binary), wire the MCP config, add tool permissions, join the deferred-restart set |

As planned this set was inspected in the Pre-flight abort and State B stopped there. Task 13 (`state-b-continues-in-session`) moved State B's restart to the **Closing restart** at the end of the run, so State B acquires, wires, and then continues through the CLI rather than stopping.

Former State C's tip in `framework/bootstrap/ductus.md` §Post-Scaffolding Output loses its audience rather than its section: with the runtime required there is no degraded-but-working outcome left to advertise, so the tip is reserved for the one degraded outcome that remains — acquisition was attempted and failed, and the run halted. It is omitted in State A and in a successful State B. As planned this read *deleted, not repurposed*, which is the reverse of what shipped.

### The amendment and the sweep land together

`framework/constitution.md` §runtime-boundary principle 3 and the Opt-in invariant are replaced; §text-first-artifacts' "usable standalone" narrows to the artifacts. Landing that ahead of the reference sweep would leave the constitution declaring a requirement while 26 per-step fallback sentences still tell hosts what to do without a runtime — a window in which the canonical source and the commands disagree.

Per §cross-spec-impact, [021](../021-runtime-boundary/spec.md) (which owns the opt-in invariant, its AC1 and AC2) and [029](../029-bootstrap-runtime-autowire/spec.md) (which owns the detection states) record the change in their own bodies and reopen to `in-progress` in the same commit.

### The release surface changes are independently landable

The `version` file, the publish gate, and the Windows `.tar.gz` asset touch only `runtime-release.yml` and the repo root. They can ship before anything else and are safe on their own — a `version` file nothing reads yet, and a gate that only fires on a partial matrix. Sequencing them first means the acquisition work is written against a release surface that already behaves the way it assumes.

### `fetch-archive`'s HTTP configuration is read once

`fetch-archive` still builds a client per call, because the SSRF pin is per URL (`resolve_to_addrs`, 022's `fetch-archive-dns-rebinding`). What that client reads is fixed at startup instead. `main` captures `ALL_PROXY`, `HTTP_PROXY`, `HTTPS_PROXY`, `NO_PROXY` and whether `REQUEST_METHOD` is set, beside the insecure-host allowlist, and each build applies them as explicit reqwest proxies or turns proxies off, which stops reqwest's per-build environment read. On Linux and other non-Apple Unix the system certificate roots are loaded once per process, at the first fetch, and passed through `tls_certs_only`, so `SSL_CERT_FILE` and `SSL_CERT_DIR` are read once; there the platform verifier was WebPKI over those same roots, so what is trusted does not change. Through a proxy, the operator's proxy is the documented trust boundary. The decisions and their grounding in the locked sources are the scenario's (`scenarios/fetch-archive-reads-its-proxy-once.md`).

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `version` | Create | The repo-root SemVer pin, fetched directly in pre-flight (task 16 moved the read off the archive) |
| `.github/workflows/runtime-release.yml` | Modify | Publish gate on the complete asset set; Windows asset as `.tar.gz` |
| `.github/workflows/framework-checks.yml` | Rename | Was `markdown-only-pipeline.yml`, whose job asserted the retired opt-in invariant |
| `.github/workflows/runtime-acquisition.yml` | Create | Replacement: end-to-end acquisition on each runner platform |
| `framework/constitution.md` | Modify | §runtime-boundary principle 3 + Opt-in invariant; §text-first-artifacts narrowing |
| `framework/bootstrap/ductus.md` | Modify | Acquisition procedure, MCP shapes, detection states, permission seeds, gitignore block, Shared Files note |
| `framework/commands/*.md` | Modify | Remove the 26 per-step markdown-only fallback instructions |
| `framework/migrations.toml` | Modify | Adopter migration entry rewriting the MCP command |
| `framework/migrations/runtime-store-path.md` | Create | That entry's procedure body (planned as `runtime-path-rewrite.md`; shipped under the registry id `runtime-store-path`) |
| `scripts/audit/version-agreement.sh` | Create | Self-audit family asserting `version`, `Cargo.toml` and the CHANGELOG agree; the tag is the release workflow's check |
| `scripts/audit/run-all.sh` | Modify | Register the new family |
| `README.md` | Modify | Drop the `PATH` install; correct the Windows cross-compilation claim |
| `.mcp.json` | Modify | This repo's own registration → the pointer |
| `.ductus/config.toml` | Modify | This repo's `[runtime]` key → its build output |
| `.gitignore` | Modify | `.ductus/bin/` entry |
| `specs/021-runtime-boundary/spec.md` | Modify | Record the amendment; reopen to `in-progress` |
| `specs/029-bootstrap-runtime-autowire/spec.md` | Modify | Record the collapsed detection states; reopen to `in-progress` |
| `AGENTS.md` | Modify | Replace the stale-binary gotcha with the `[runtime]` key workflow |
| `runtime/src/primitives/fetch_archive.rs` | Modify | Proxy variables applied from the startup capture; Linux roots loaded once; the proxy trust boundary stated (task 18) |
| `runtime/src/main.rs` | Modify | Capture the proxy variables at startup (task 18) |
| `runtime/Cargo.toml`, `runtime/Cargo.lock` | Modify | `rustls` and `rustls-native-certs` as direct dependencies, `webpki-root-certs` for tests (task 18) |
| `docs/runtime.md` | Modify | The environment-variable inventory: when each variable is read, and the proxy trust boundary (task 18) |

## Trade-offs

**One version per machine.** The store holds a single binary, so two projects pinned to different versions cannot both be correct simultaneously; whichever ran `/ductus` last wins until the other runs it. Rejected alternatives: a versioned store (`ductus-{version}`), which fixes coexistence but not the home-config agents whose single MCP entry still names one version; and per-project binaries, which fix both but leave two of four agents unable to express a project-local path at all. Accepted because it is strictly better than the `PATH` behavior it replaces — ductus owns the path and every run corrects it — and because it is the only shape uniform across all four agents.

**A gitignored artifact the repo cannot verify.** The pointer is per-contributor and outside version control, so nothing in CI proves an adopter's pointer is correct. Mitigated by making it cheap to repair: a missing or dangling pointer is the expected state of any un-bootstrapped checkout, and `/ductus` recreates it without ceremony.

**Adoption is now gated on a download.** Firewalled, air-gapped, and unpublished-platform adopters move from *degraded* to *blocked* until they use the `[runtime]` supplied-binary key. This is the deliberate cost of the requirement; the mitigation is that the halt names the store path and the release URL, so the recovery is discoverable at the moment it is needed rather than in documentation the reader has not opened.

**A security-relevant sequence with no primitive behind it.** Download, checksum, extract, and install run as shell steps because the runtime is unavailable at that moment. The verification gate is therefore procedure rather than code, and its correctness depends on `ductus.md` being followed. Accepted as unavoidable; noted here so a later reader does not mistake it for an oversight.

**Pre-granting execution of a downloaded binary.** The permission seeds authorize running the store path without a prompt. Safe only because verification precedes it and a mismatch halts before anything is written — but it is a real widening of what the bootstrap may do unattended, and it is stated in the spec rather than buried in a settings template.
