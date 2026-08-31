---
spec: 022-deterministic-runtime
reviewed-at: 2026-08-31T00:57:50Z
reviewed-against: bc231336baa9f84affa27b581db0201586a83ae1
diff-base: 45ac2c848c6cda764c74bfab9caf8bdf1a957cfb
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 1
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Clean. Five passes over the window `45ac2c8..bc23133` — the `writeCode` payload bundler (`payload.rs`, `extensions.rs`), one re-blessed parity golden, and the governance prose the rule files do not cover. The prior run's single SHOULD (`QUAL-CLAIM-001`, an empty `constitution-excerpts` array indistinguishable from a constitution that could not be read) is fixed at `c7797ae`, not waived: `load_constitution_excerpts` now returns a scan carrying what it read and what it could not, surfaced as `constitution-excerpts-unexaminable` with `skip_serializing_if` so a clean payload stays byte-identical. Its own follow-on — absolute paths in those labels, which would carry a contributor's home directory into an outbound payload — is fixed at `bc23133`. 0 MUST, 0 SHOULD, 0 low-confidence; not blocking.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] chore: delete the stale "do not git push until 0.37.0 is tagged" entry from AGENTS.md §Workflow — the entry says to remove it once the release is tagged, and `ductus-v0.37.0` through `ductus-v0.40.0` all exist with the `version` file at 0.40.0, so the push block it describes no longer applies and reads as a live prohibition to any contributor who takes it at face value

## Observations

*None.*

## Skipped passes

*None.*
