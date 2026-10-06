# 065 — Configurable Source Repository Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Parameterize `install.sh`'s bootstrap fetch

- [x] Read `DUCTUS_REPO` (default `stonean/ductus`); compose the RAW URL as `https://raw.githubusercontent.com/$DUCTUS_REPO/main/framework/bootstrap/ductus.md`
- [x] Document the variable (and its default) in `install.sh`'s header comment
- [x] Probe: with `DUCTUS_REPO` unset, the rendered URL is byte-identical to the prior text

- **Done when**: `install.sh`'s fetch honors the variable with the documented default, and the unset render is unchanged.

## 2. Parameterize the bootstrap's four fetch sites

- [x] `framework/bootstrap/ductus.md` §Runtime acquisition (Branch 2): version-pin fetch renders `…${DUCTUS_REPO:-stonean/ductus}/main/version`
- [x] Release download fetches render `…${DUCTUS_REPO:-stonean/ductus}/releases/download/ductus-v{pin}/ductus-{triple}.tar.gz` (and `.sha256`), placeholders intact
- [x] §Self-update check: the staleness-comparison fetch renders `…${DUCTUS_REPO:-stonean/ductus}/main/framework/bootstrap/ductus.md`
- [x] §File Fetching: the framework archive fetch renders `https://codeload.github.com/${DUCTUS_REPO:-stonean/ductus}/tar.gz/refs/heads/main`
- [x] Document `$DUCTUS_REPO` in the bootstrap's pre-flight / fetch narrative (one sentence noting the default is the canonical repository)
- [x] `cp framework/bootstrap/ductus.md framework/bootstrap/govern.md` (Family 21)

- **Done when**: all four sites use the single `$DUCTUS_REPO` default, placeholders survive, the narrative documents it, and the two bootstrap files are byte-identical.

## 3. Verify audits and self-url derivation

- [x] `scripts/audit/self-url-resolution.sh` derives the canonical slug from the archive URL's default text (unchanged)
- [x] `scripts/audit/run-all.sh` passes on the default tree (self-url, placeholder-roundtrip, Family 21 govern parity, and the rest)
- [x] Confirm no other comitted file needs the parameterization (`configure/pi.md`'s canonical-source URL is doc prose, not a fetch — verify and leave)

- **Done when**: the full audit is green with defaults in place, and the one verified doc-prose URL is deliberately left canonical.

## 4. Fork adoption smoke test

- [x] In a scratch project, set `DUCTUS_REPO=<fork>` (e.g. the fork under test), run the parameterized `install.sh`, and confirm the bootstrap is fetched from the fork's raw URL
- [x] Confirm the four fetch instructions in the installed bootstrap render the fork's owner/repo when the variable is set (host-side inspection of the rendered commands)
- [x] Record the probe result (which origin each site hit) in this spec's plan or the commit message

- **Done when**: a `DUCTUS_REPO` set to a fork makes `install.sh` fetch from that fork, and the inspection shows all four sites resolving the same fork origin.

## 5. Review, analyze, done

- [x] `/{project}:review` (verify the parameterization against the ACs; confirm no fetch site regressed to a hardcoded fork or a second default)
- [x] `/{project}:analyze` (check-artifacts clean; dependencies current)
- [x] Transition to `done`

- **Done when**: the spec is `done` with current review and analyze records, and 065's ACs are verified against the shipped text.

## 6. Implement scenario: [a-non-canonical-origin-is-announced](scenarios/a-non-canonical-origin-is-announced.md)

- [x] Implement the behavior described in `scenarios/a-non-canonical-origin-is-announced.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 7. Inventory DUCTUS_REPO (CFG-ENV-002)

- [x] List `DUCTUS_REPO` in `docs/runtime.md` §Environment variables — purpose, optional, default `stonean/ductus` — and widen the inventory's scope sentence to the installer and `/ductus` (613842d4)

- **Done when**: the inventory lists DUCTUS_REPO with its purpose, its optionality and its default.
