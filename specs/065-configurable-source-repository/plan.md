# 065 — Configurable Source Repository Plan

Implements [065 — Configurable Source Repository](spec.md).

## Overview

One mechanism, every fetch site, zero runtime change. A `DUCTUS_REPO`
environment variable (default `stonean/ductus`) parameterizes the repository
component of every exec-time fetch URL — `install.sh`'s, and `/ductus`'s
latest-release resolution, version pin, migrations registry, runtime release
download and framework archive — plus the self-update/staleness comparison
fetch, which must name the same origin as the bootstrap that installed the file
being compared. The
default is spelled in the committed text, so unset behavior is byte-identical
and every audit that greps the committed URLs (self-url resolution, placeholder
recognition) stays green. `govern.md` is re-copied from `ductus.md` (Family 21
byte-identity).

## Technical Decisions

### D1 — One env var, "REPO" as the unit of configuration

`DUCTUS_REPO` carries `owner/repo`. Each fetch site composes its URL from it:
`https://raw.githubusercontent.com/$DUCTUS_REPO/{raw-ref}/…`,
`https://codeload.github.com/$DUCTUS_REPO/tar.gz/{archive-ref}`,
`https://github.com/$DUCTUS_REPO/releases/download/…`. A full-base-URL variable
(`DUCTUS_RAW_BASE`, `DUCTUS_CODELOAD_BASE`, …) would fragment the default
across hosts and invite a half-forked adoption; one `owner/repo` value composed
into every host keeps all four sites provably consistent (spec AC5).

### D2 — Shell-default syntax in the fetch instructions

Each fetch line renders as `…${DUCTUS_REPO:-stonean/ductus}…` — the literal
default stays in the committed text (audits grep it) and the shell expands it
at execution. `ductus.md`'s fetch instructions are executed by the host
(markdown-only path) or substituted by the host (runtime path), so the shell
default syntax is the one canonical spelling.

### D3 — self-url derivation keeps the canonical slug

`scripts/audit/self-url-resolution.sh` derives the slug from the archive URL in
`ductus.md` with a regex over `codeload.github.com/<owner>/<repo>/tar.gz/`,
extended to unwrap the `${DUCTUS_REPO:-…}` default
(`scripts/audit/self-url-resolution.sh:85-87`). Because the default remains the
literal `stonean/ductus` in the committed text, the derived slug is unchanged. A fork that changes the default
changes the URL the family derives from — the family's own stated design.

### D4 — staleness fetch parameterized

The self-update check fetches the installed file's origin to compare. It must
use `$DUCTUS_REPO` too, so a fork adoption compares its installed bootstrap
against its own upstream rather than the canonical one (spec AC7) — otherwise
every `/ductus` run in a fork adoption would see its installed file as
"divergent" from canonical main on the first byte of difference.

### D5 — no runtime change

The parameterization is entirely in `install.sh` and markdown fetch prose. Like
any framework-only change it reaches a default-source adopter at the next
`ductus-v*` release (061); a project on `--ref=main` sees it on commit.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `install.sh` | Modify | Bootstrap fetch reads `$DUCTUS_REPO` (default stonean/ductus); header documents it |
| `framework/bootstrap/ductus.md` | Modify | Version-pin fetch, release download, framework archive fetch, self-update fetch render `$DUCTUS_REPO`; pre-flight/fetch sections document it |
| `framework/bootstrap/govern.md` | Modify | Byte-identical mirror of ductus.md (Family 21) |
| `specs/065-configurable-source-repository/{plan.md, tasks.md, spec.md}` | Modify | This spec |

## Trade-offs

- **Env var over config key** — the variable must guide the *first* fetch,
  before `.ductus/config.toml` exists; a config key is a documented non-goal
  (spec §Design Decisions).
- **One `owner/repo` over per-host base URLs** — less flexible, but a single
  value is the only shape that cannot half-fork an adoption across hosts (D1).
- **Prose URLs stay canonical** — documentation citations keep pointing at
  `stonean/ductus`; only fetches parameterize (spec §Design Decisions).
- **Known limitation** — `$DUCTUS_REPO` is fork-wide, not per-fetch: an adopter
  cannot fetch the archive from their fork but the runtime from the canonical
  release. That asymmetry is a feature (a half-fork is a silent-mix risk, spec
  AC5), and the release-download site accepts the same value as the others.
