---
spec: 049-rename-govern-to-ductus
reviewed-at: 2026-08-16T02:19:31Z
reviewed-against: 34b8f22ff997d54dd9e3344226b4d03032959914
diff-base: 2c207682a9b89a9e845d0d08d6a54fa1e1fca4f4
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 049-rename-govern-to-ductus

## Summary

0 MUST violation(s), 0 SHOULD violation(s), 0 low-confidence finding(s). blocking: no.

Two findings were raised during this review and **both were fixed before it was
written**, so the counts above state what is outstanding, not what was found.
They are recorded below with a **Status** line naming the commit that closed
each, per the report's reconciliation rule.

The scope is unusual: 508 files changed in the window, of which all but a
handful are the rename sweep itself — a uniform token substitution whose
correctness was established separately (a residue classification proving every
surviving occurrence is a legacy path constant, a published version reference,
a historical migration id, or ordinary English; plus `scripts/audit/run-all.sh`
at exit 0 and 11 green test suites). Reviewing 500 substituted lines against
the rule files would find nothing the sweep audit did not. The passes therefore
concentrated on the code this spec *wrote*: the three-tier resolution chain in
`runtime/src/schema/paths.rs`, the mechanical-sweep exemption in
`scripts/audit/review-freshness.sh`, the `ductus-rename` migration procedure,
and the release workflow's tag scheme. Both findings came out of that set.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None outstanding.* Both findings below were fixed in-window.

### SHOULD: QUAL-CLAIM-001 — a deleted file's lines were attributed to the previous file

- **File**: `scripts/audit/review-freshness.sh:186-215`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish *"examined the subject and found nothing"* from *"could not examine the subject"*, rather than emitting the same value for both.
- **Finding**: The diff parser advanced its current path on `+++ b/`, which a deleted file's `+++ /dev/null` header does not match. The deleted file's removed lines therefore accumulated against the previously-seen file and flushed into its run, marking a pure rename non-uniform. Any spec whose durable contract happened to be listed immediately before a deleted `.md` in the diff would have read stale indefinitely, with the report asserting a contract change that never happened. It did not fire in this release only because the `.md` files this rename deleted are generated command copies rather than scenarios.
- **Auto-fixable**: no
- **Status**: fixed in `34b8f22`. Reproduced first against a scratch repository (a rename plus an unrelated deletion in one window), which showed a pure-rename file reported as `None`; the parser now resets on the `diff --git` header and treats a non-`b/` target as no path.

### SHOULD: QUAL-CLAIM-001 — an unreachable fallback resolved to the repo root

- **File**: `runtime/src/schema/paths.rs:103-111`
- **Rule**: When a code path skips part of its subject, cannot reach it, or has no basis to inspect it, its output SHOULD say so — instead of a bare zero, empty collection, or success string that a caller will read as positive assurance.
- **Finding**: `newest_existing` and `active_path` ended in `unwrap_or_default()` over a `&[&'static str]`, yielding `""` for an empty chain — and `repo.join("")` is the repo root, so every config and session read would silently resolve to a directory rather than a file. The state was guarded by a `debug_assert!` that compiles out of release builds, so the guard was absent exactly where the wrong answer would ship.
- **Auto-fixable**: no
- **Status**: fixed in `34b8f22`. The chains are `[&str; 3]` now, so an empty chain cannot be constructed and the fallback is gone rather than defended — the impossible state is unrepresentable instead of handled.

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Skipped passes

*None.*
