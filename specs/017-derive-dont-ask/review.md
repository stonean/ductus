---
spec: 017-derive-dont-ask
reviewed-at: 2026-08-03T02:44:40Z
reviewed-against: 8891da925ff7b5f8d5c2892ffd1689bb8f8d4915
diff-base: 096dbc0cf65a2322c91bfa895a825ea60c5a23f8
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 1
skipped-passes: []
---

# Review — 017-derive-dont-ask

## Summary

Reopened by the scenario back-edge for `generator-sync-claim-honesty` (task 35). The change is bash: a new `untracked_specs()` and `report_no_changes()` in `.govern/scripts/lib/specs-root.sh`, a drifted-but-unwritten counter in `gen-spec-deps.sh`, and both generators' zero-rewrite message routed through the shared reporter. The `list_specs` git-ls-files exclusion is byte-for-byte unchanged, which was the point: the adopter report's proposed remedy (revert it) would have restored the worse bug 017 fixed, and only the reporting was ever wrong. Reviewing the fix surfaced a residual instance of the same defect one level in — `gen-spec-deps.sh` enumerates every tracked spec but writes only its rewrite targets, so a drifted unstaged spec was examined and then reported "in sync" — which is fixed and verified end to end in the same window rather than logged. Shell-injection surface was checked and is unchanged: both new functions interpolate only `$SPECS_ROOT`, already constrained to `[A-Za-z0-9_-]` by `specs_root_of` for exactly this reason, and `$ROOT`, which is passed to `git -C` rather than to a shell. All five passes ran; no findings.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] bug: `compute-review-scope` returns an unusable scope and a polluted captured-issues list — plan-affected is not parsed as a table, and captured-issues takes raw added lines rather than the shared comment-aware bullet grammar.

## Skipped passes

*None.*
