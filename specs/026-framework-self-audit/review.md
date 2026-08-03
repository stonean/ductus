---
spec: 026-framework-self-audit
reviewed-at: 2026-08-03T03:05:16Z
reviewed-against: d99df57ecd05936029a1d29d08706ff48904ae01
diff-base: 113a1bc0000000000000000000000000000000
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

Re-run after the `family-17-contract-binding` scenario (task 22), which closes the single SHOULD violation the 2026-08-02 pass recorded. That finding was `QUAL-GROUND-001`: Family 17 mirrored four runtime-owned contracts as bare shell literals with no guard, so a renamed key or a fifth agent would leave it exiting 0 while checking the wrong thing. The agent config dirs are now derived from the Agent Registry table's `config_dir` column in `framework/bootstrap/govern.md` — the canonical source per the constitution's canonical-sources map — and the three single-site contracts are asserted against `runtime/src/host.rs` and `runtime/src/schema/paths.rs`. Crucially the failure direction is right: a derivation that yields nothing emits a finding and exits non-zero rather than falling back to a built-in list, which is the family's own governing distinction applied to itself. Verified that the derived set matches the four registered agents and that the derivation returns empty against a table-less file. The residual — a reordered registry column would yield wrong values rather than none, and the assertions are substring checks rather than a parse — is recorded in the scenario's Edge Cases alongside the runtime-exposed-namespace fix that would close it. Shell safety re-checked: the derived values are used only as directory names in a `for` loop and a `[ -d ]` test, never interpolated into an eval or a command string. The three captured issues from the prior pass (release-sequence checklist, mark-task tick-only asymmetry, generator in-sync claim) are all resolved and no longer appear. No findings.

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

## Skipped passes

*None.*
