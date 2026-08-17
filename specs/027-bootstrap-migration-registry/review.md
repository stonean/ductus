---
spec: 027-bootstrap-migration-registry
reviewed-at: 2026-08-17T02:24:23Z
reviewed-against: 11ff132d453c829246d6b9394ec862603a60b0d6
diff-base: c8d0121313f7994cb939dcf6781366f4645f8b10
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 027-bootstrap-migration-registry

## Summary

Re-run after `/ductus:audit` Family 19 reported this spec's review stale: the sweep that removed a named adopter project from the repository edited `scenarios/migration-chain-reference-integrity.md`, a durable contract, after the prior verdict was recorded.

The staleness is correct and the exemption correctly did not apply. The sweep was a token substitution in intent but not in form — the replacement text varied by sentence (`the adopter still carries`, `the real adopter bootstrap`, `the adopter bootstrap`), so no single pair recurs across two files and `mechanical_sweep`'s repo-wide test rightly declines to exempt it. A conservative gate on a prose-only edit is the cheap direction of error.

Passes re-run over the same 40-file scope: 0 MUST, 0 SHOULD, 0 low-confidence, nothing waived, no pass skipped. The delta since the prior verdict is prose only — no runtime behavior, no contract semantics — and the two defects that verdict recorded as fixed (the repo-escaping candidate path and the unreachable `covers` disjunct) remain fixed.

Verification: 953 tests pass (`--locked --release`), clippy clean on `--all-targets`, markdownlint clean over 420 files.

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
