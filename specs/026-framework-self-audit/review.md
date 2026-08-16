---
spec: 026-framework-self-audit
reviewed-at: 2026-08-16T13:09:52Z
reviewed-against: 49a14d3c43cc0aa4a231c2c0fd40e14fb5ef6894
diff-base: ee6a07d8e9ba17ce26c26a1a1ec9d2d55adca2fb
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

0 MUST violation(s), 0 SHOULD violation(s), 0 low-confidence finding(s). blocking: no.

One finding was raised during this review and **fixed before it was finalised**, so the counts above state what is outstanding, not what was found. It is recorded below with a **Status** line naming the commit that closed it, per the report's reconciliation rule.

Scope resolved to 72 files, of which 11 are code: install.sh, runtime/src/schema/paths.rs, four audit families (review-freshness, run-all, sibling-coupling, transitional-bootstrap-parity) and five lint scripts. The remainder are command sources, migrations and spec artifacts — `/ductus:analyze`'s subject rather than the five code passes'. 4 of the 11 loaded rule files declare themselves design-time/analyze-enforced rather than code-pass rules.

The audit machinery in scope is otherwise in good shape and was checked against the failure this spec exists to prevent. sibling-coupling.sh — the script AGENTS.md's first Design Principle records as having silently disabled a release gate via a GNU-awk extension — now uses POSIX match() with RSTART/RLENGTH and carries an explicit precondition probe (`awk 'BEGIN { if (match("x", /x/)) exit 0; exit 1 }'`) that emits a finding and returns 1 when awk cannot evaluate match(), citing QUAL-CLAIM-001 by name. That is the compliant shape, and it was exercised rather than assumed: the family runs clean here under BSD awk (20200816), the environment where the original defect bit. run-all.sh likewise treats a missing or non-executable family script as a finding rather than a silent zero, so its zero-output/exit-0 result is meaningful. install.sh is clean — TLS-pinned curl (`--proto '=https' --tlsv1.2 -fsSL`), an allowlisted agent arm with an explicit reject-and-exit default, quoted expansions, and permission seeds written only when absent.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None outstanding.* The finding below was fixed in-window.

### SHOULD: QUAL-CLAIM-001 — the audit and lint scripts that form the release gate were never linted

- **File**: `.github/workflows/framework-checks.yml:35-70`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish *"examined the subject and found nothing"* from *"could not examine the subject"*, rather than emitting the same value for both.
- **Finding**: Nine of the eleven code files in this spec's scope are shell — four audit families and five lint scripts — and nothing in the repo invoked shellcheck against them, verified by grep across `.github/`, `scripts/`, `.githooks/` and `framework/`. A tuned `.shellcheckrc` was committed but never executed. This matters more for 026 than anywhere else: these scripts *are* the self-audit, `run-all.sh` is the hard release gate in `runtime-release.yml`, and a script that aborts before examining its subject is exactly the failure this spec's own history records (`sibling-coupling.sh`'s GNU-awk extension, green locally and dead on every macOS machine). A green `framework-checks` run did not distinguish "the audit scripts are sound" from "the audit scripts were never examined". `sibling-coupling.sh` had since been given a per-family precondition probe, which is the right fix for that one family but a hand-applied guard rather than coverage across the suite.
- **Auto-fixable**: no
- **Status**: fixed in `49a14d3`. Adds step (j) to `framework-checks.yml`, covering 42 tracked scripts — every audit family, every lint, the generators, the installer and the hooks. The step asserts shellcheck is present and the enumeration non-empty *before* linting, so it cannot pass by examining nothing, and avoids `mapfile` so it runs verbatim on bash 3.2. Verified both ways: clean across all 42 files, and exit 1 on an introduced SC2034. The generalisation of `sibling-coupling.sh`'s precondition-probe pattern into `scripts/audit/lib.sh` is **not** done — it remains a per-family guard, and is the residual half of this finding worth considering separately.

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Skipped passes

*None.*
