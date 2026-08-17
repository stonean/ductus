---
spec: 027-bootstrap-migration-registry
reviewed-at: 2026-08-17T01:55:47Z
reviewed-against: f1aed19
diff-base: c8d0121313f7994cb939dcf6781366f4645f8b10
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 1
skipped-passes: []
---

# Review — 027-bootstrap-migration-registry

## Summary

All five passes ran over the 40-file scope resolved from `c8d0121..HEAD` — the work that landed 027's `migration-chain-reference-integrity` and its runtime half. Clean at close: 0 MUST, 0 SHOULD, 0 low-confidence, nothing waived, no pass skipped.

Two findings were raised and **fixed** rather than recorded, per this repo's rule that a SHOULD gates `done` exactly as a MUST does. Both were in `check-orphaned-references` itself, shipped hours earlier in `0.29.4`, and both are the same shape — a claim wider than what the code had established. A candidate path built from a referrer file's *contents* could carry `..` past the repo root and still be reported as a broken *managed* reference, which asserts something about a path the check does not govern; and `covers` carried a second prefix disjunct that could never fire, because its input is normalized a line earlier. Fixed in `0.29.6` with two regression tests, including the sibling-prefix case (`.ductus` must not cover `.ductus-cache/`).

The security pass gave the traversal the most attention and concluded it was never exploitable: the primitive is existence-only and opens nothing, and the referrer files are the adopter's own committed artifacts, so no trust boundary is crossed. It was fixed on correctness grounds, not security ones — and on consistency, since `apply-manifest` already refuses a destination that escapes the repo root.

The reuse pass found the opposite of a finding: `check-orphaned-references` reaches for `check_artifacts::adopter_destinations` and `ships_to_adopter` rather than restating the ships-elsewhere rule, and `load_json_arg` is one generic helper serving both manifest hydrators.

Verification: 953 tests pass (`--locked --release`), clippy clean on `--all-targets`, `scripts/audit/run-all.sh` exit 0, markdownlint clean over 420 files. The primitive was also run against this repository — 0 findings, 4 referrers examined, 0 skipped — and against a fixture reproducing both adopter defects, which it reports.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] rule: `AGENTS.md` carries adopter-beneficial rules in a file adopters never receive — surveyed 2026-08-17, ~12 of 56 entries are strongly universal; §recommendations was promoted to the constitution as the first instance and is the model. The rest needs a spec.

## Observations

*None.*

## Skipped passes

*None.*
