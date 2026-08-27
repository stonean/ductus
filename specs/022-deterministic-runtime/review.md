---
spec: 022-deterministic-runtime
reviewed-at: 2026-08-27T22:41:40Z
reviewed-against: 78dd6238bb6deaf0cf02d2d4ac565223ac4ae09a
diff-base: 31348ff7dfd8b8b3cd108b8d0e7829c8b184dd14
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 2
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Re-run after the previous run's single SHOULD was fixed rather than carried. 0 MUST, 0 SHOULD, 0 low-confidence across all five passes; not blocking.

The window covers three changes to the derive primitives and one to `append-task`. `derive-references` now enumerates every tracked spec and filters only the write, reporting drifted-but-unstaged specs in `unwritten` — closing a gap that let a `[services]` alias rename leave dead references in place for nine commits while the pre-commit hook reported the tree in sync. `append-task` no longer discards a supplied `slug` when `body` is given. Both primitives gained `absent`, which was the previous run's `QUAL-CLAIM-001` finding: they enumerated the git index, skipped `!path.is_file()`, and still counted those specs in `examined`, asserting a subject the run never read. `examined` now counts specs actually read, and the MCP tool descriptions name the field so the caller-facing contract matches the result.

Security: no new input-handling surface — `slug` already passed `BE-INPUT-002`'s allowlist validation unconditionally, and the change makes that validation load-bearing on a path where the value was previously discarded. Reuse, efficiency, and simplicity passes found nothing against the loaded rules; two things they surfaced map to no loaded rule and are recorded as observations rather than invented findings.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] convention: `check_command_flags::argument_hint` hand-rolls frontmatter-block extraction (first-line `---`, scan to closing fence) that `primitives::split_frontmatter` already provides, including its CRLF-opener and empty-block handling. Reusing it via `.ok()` would drop ~15 lines and remove a second definition of where frontmatter ends. Maps to no loaded rule — there is no reuse rule ID in the rule set. — `runtime/src/primitives/check_command_flags.rs` (captured during review of 022-deterministic-runtime)
- [ ] perf: the adopter pre-commit hook now performs two independent full walks of the tracked spec corpus per commit — `derive-dependencies` and `derive-references` each list the index and read every spec, since `derive-references` no longer narrows its enumeration under `--staged`. Negligible at 51 specs and deliberate (the narrowing was the defect), but the cost is now O(corpus) twice on every commit rather than once, and the two run as separate processes so nothing shares the read. Worth revisiting if a large corpus makes commits slow. — `framework/bootstrap/hooks/ductus-pre-commit` (captured during review of 022-deterministic-runtime)

## Observations

- convention: `check_command_flags::argument_hint` hand-rolls frontmatter-block extraction (first-line `---`, scan to closing fence) that `primitives::split_frontmatter` already provides, including its CRLF-opener and empty-block handling. Reusing it via `.ok()` would drop ~15 lines and remove a second definition of where frontmatter ends. Maps to no loaded rule — there is no reuse rule ID in the rule set. — `runtime/src/primitives/check_command_flags.rs`
- perf: the adopter pre-commit hook now performs two independent full walks of the tracked spec corpus per commit — `derive-dependencies` and `derive-references` each list the index and read every spec, since `derive-references` no longer narrows its enumeration under `--staged`. Negligible at 51 specs and deliberate (the narrowing was the defect), but the cost is now O(corpus) twice on every commit rather than once, and the two run as separate processes so nothing shares the read. Worth revisiting if a large corpus makes commits slow. — `framework/bootstrap/hooks/ductus-pre-commit`

## Skipped passes

*None.*
