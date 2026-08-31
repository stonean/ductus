---
spec: 022-deterministic-runtime
reviewed-at: 2026-08-31T00:44:57Z
reviewed-against: 0a69e8c35f536891c4b595263393dcd22b1884fa
diff-base: 45ac2c848c6cda764c74bfab9caf8bdf1a957cfb
must-violations: 0
should-violations: 1
low-confidence: 0
captured-issues: 1
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

0 MUST violation(s), 1 SHOULD violation(s), 0 low-confidence finding(s). blocking: no.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

### SHOULD: QUAL-CLAIM-001 — an empty `constitution-excerpts` array cannot be told from a constitution that could not be read

- **File**: `runtime/src/interpreter/payload.rs:1239-1258`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish *"examined the subject and found nothing"* from *"could not examine the subject"*, rather than emitting the same value for both. When a code path skips part of its subject, cannot reach it, or has no basis to inspect it, its output SHOULD say so — through a distinct return variant, an accompanying status or guidance field, or a message naming what was not examined — instead of a bare zero, empty collection, or success string that a caller will read as positive assurance.
- **Finding**: `load_constitution_excerpts` returns a bare `Vec<String>`, and five distinct states collapse into the same value: the command file could not be located, the command file could not be read, the command declares no `Reference:` anchors, the constitution itself could not be read, and — via the closing `filter_map` — an anchor that resolves to nothing is silently dropped from an otherwise-populated array. Only the third is genuinely 'examined and found nothing'; the rest are 'could not examine', and the host receives an outbound `writeCode` payload that reads as 'no constitutional context applies' in every case. The repo already rejects this shape one call away: `resolve_anchor.rs:40` accumulates an `unresolved` set for exactly these anchors, and `read-spec` reports `scenario-files-unreadable` alongside its question list for exactly this reason.
- **Auto-fixable**: no
- **Suggested fix**: Return the unexaminable set alongside the excerpts rather than folding it into an empty vec, and surface it on `WriteCodeRequest` as a field carrying `skip_serializing_if = "Vec::is_empty"` — the shape task 89 used for `scenario-files-unreadable`, which keeps the clean payload byte-identical and re-blesses no parity golden. Distinguish at minimum an unreadable constitution from an anchor that did not resolve; a command file with no `Reference:` line stays the one honest empty case.

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
