---
spec: 048-govern-acquired-runtime
scenario: pin-is-readable-when-acquisition-needs-it
reviewed-at: 2026-08-19T16:22:13Z
reviewed-against: e3028e37629c10e0cd3630749914e2041d196390
diff-base: 2ad7cdc0000000000000000000000000000000
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 048-govern-acquired-runtime

## Summary

Clean at `e3028e3` — 0 MUST, 0 SHOULD, 0 low-confidence, 0 observations.

Scope: `framework/bootstrap/ductus.md`, its byte-identical `govern.md` mirror, the new scenario, and task 16.

**The defect was a blocking one and the fix is verified, not argued.** `/ductus` could not acquire the runtime on a greenfield adoption: every first-time adopter is State B by definition, State B's first act is Runtime acquisition, and its step 1 read the pin from a file that only exists after the archive extract hundreds of lines later. The halt was deliberate — guessing a version would install an untested runtime — so a faithful walk stopped. Underneath it sat a second defect: `{tempdir}` was created inside the self-update check, which runs *after* runtime detection, so even fetching the pin during acquisition had nowhere to fetch it to. The second check owned the resource the first one needed.

**Three changes, each minimal.** `{tempdir}` moves to the Pre-flight preamble; acquisition step 1 fetches the pin into it; `{staging-dir}` retires. That last one is worth naming: it appeared only in these steps, was never defined in §Derived values, and named what the rest of the document calls `{tempdir}` — an undefined placeholder sitting in the one procedure every first-run adopter executes.

**`govern.md` was updated in the same change**, byte-identically. Family 21 exists precisely for this: every pre-rename adopter's self-update fetch resolves to that path, so fixing only `ductus.md` would have shipped the broken procedure to exactly the adopters who cannot yet reach the fixed one. The family confirms parity.

**Verified by re-running the greenfield adoption against the fixed bootstrap with nothing supplied by hand** — the prior run needed the pin handed to it to get past step 1. Pin fetched and read at step 1, archive and sidecar fetched, digest verified before install, store installed, re-probe reporting `0.31.0` against pin `0.31.0`, the self-update fetch reusing the same `{tempdir}` rather than creating a second, and the pointer resolving and executing. The harness asserts each step rather than printing progress, so a regression fails it.

**The one property the fix trades away is stated rather than buried.** The pin and the framework tree now arrive in two fetches instead of travelling in one archive, so they agree because both name `main`. A push landing between them is the sole divergence, bounded by one run, and the next `/ductus` re-acquires because acquisition is idempotent and re-probes the store. The prior arrangement made that divergence impossible — by reading a file that was not there.

`QUAL-CLAIM-001` is satisfied in the direction that matters here: the halt survives with an accurate message naming the pin URL, so an offline adopter still fails loudly rather than proceeding on a guessed version. Only the reason it can fail has moved.

Verified against the whole CI surface: markdownlint, six `lint-*` scripts, both `scripts/tests` suites, shellcheck over every tracked shell file, actionlint, all three generators plus both derive primitives with a clean tree after, `scripts/audit/run-all.sh` (27 families, re-run after committing), and under `runtime/` `cargo fmt --check`, `clippy --release --all-targets --locked -- -D warnings`, and `cargo test --release --locked` — the last of which matters here, since a bootstrap edit is the class of change that moved a parity golden earlier this session.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Observations

*None.*

## Skipped passes

*None.*
