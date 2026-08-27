---
spec: 022-deterministic-runtime
reviewed-at: 2026-08-27T22:32:15Z
reviewed-against: 8779616d5f1799fd3dfa91e51c6680b01b0477f6
diff-base: 31348ff7dfd8b8b3cd108b8d0e7829c8b184dd14
must-violations: 0
should-violations: 1
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

0 MUST violation(s), 1 SHOULD violation(s), 0 low-confidence finding(s). blocking: no.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

### SHOULD: QUAL-CLAIM-001 — `examined` counts tracked specs the `is_file()` guard skipped without reading

- **File**: `runtime/src/primitives/derive_references.rs:124-156`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish "examined the subject and found nothing" from "could not examine the subject", rather than emitting the same value for both. When a code path skips part of its subject, cannot reach it, or has no basis to inspect it, its output SHOULD say so — through a distinct return variant, an accompanying status or guidance field, or a message naming what was not examined — instead of a bare zero, empty collection, or success string that a caller will read as positive assurance.
- **Finding**: Both derive primitives enumerate `list_tracked_specs` (the git index) and then `continue` on `!path.is_file()`, but report `examined: tracked.len()`. A spec that is tracked yet absent from the worktree — in the index and deleted without staging the deletion — is skipped without being read, and lands in none of `updated`, `unwritten`, `unparseable`, or `untracked-skipped` while still being counted as examined. This is the same defect one level over from the one this change just closed: the count asserts a subject size the run did not actually inspect. `derive_dependencies.rs:76,116` carries the identical pattern.
- **Auto-fixable**: no
- **Suggested fix**: Collect the skipped paths into a field (e.g. `absent`, alongside the existing `unparseable`) and either subtract them from `examined` or report both numbers, so a caller can tell a fully-read corpus from a partially-read one. Apply to both primitives in the same change — they share the enumeration and would otherwise disagree about what `examined` means.

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Observations

- convention: `check_command_flags::argument_hint` hand-rolls frontmatter-block extraction (first-line `---`, scan to closing fence) that `primitives::split_frontmatter` already provides, including its CRLF-opener and empty-block handling. Reusing it via `.ok()` would drop ~15 lines and remove a second definition of where frontmatter ends. Maps to no loaded rule — there is no reuse rule ID in the rule set. — `runtime/src/primitives/check_command_flags.rs`
- perf: the adopter pre-commit hook now performs two independent full walks of the tracked spec corpus per commit — `derive-dependencies` and `derive-references` each list the index and read every spec, since `derive-references` no longer narrows its enumeration under `--staged`. Negligible at 51 specs and deliberate (the narrowing was the defect), but the cost is now O(corpus) twice on every commit rather than once, and the two run as separate processes so nothing shares the read. Worth revisiting if a large corpus makes commits slow. — `framework/bootstrap/hooks/ductus-pre-commit`

## Skipped passes

*None.*
