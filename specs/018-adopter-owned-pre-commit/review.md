---
spec: 018-adopter-owned-pre-commit
reviewed-at: 2026-05-10T00:00:00Z
reviewed-against: 3d7c50beb1aa9e82783cb2a7f9ed5b0540068625
diff-base: 3d7c50beb1aa9e82783cb2a7f9ed5b0540068625
must-violations: 0
should-violations: 0
low-confidence: 0
skipped-passes: []
---

# Review — 018-adopter-owned-pre-commit

## Summary

Splits the adopter pre-commit hook into an outer adopter-owned stub and an inner ductus-owned `ductus-pre-commit` script. Deletes the standalone `install.sh` and inlines installation into `framework/bootstrap/ductus.md`'s Hook Installation section. The split is byte-preserving for the inner file (rename only), so CI sees no diff. Pure markdown + shell-script split — security rules do not apply at the framework level. All five passes ran; no findings. `blocking: no`.

## MUST violations (blocking)

_None._

## SHOULD violations (advisory)

_None._

## Low-confidence findings

_None._

## Waived findings

_None._

## Skipped passes

_None._

## Pass notes

### Security

The outer adopter-owned stub is unmanaged by ductus — adopters can extend it freely. The inner `ductus-pre-commit` preserves its `# managed-by: ductus` sentinel on line 2, allowing the installer ladder to detect and refresh ductus-owned content without clobbering adopter edits. No new attack surface.

### Reuse

The outer-stub / inner-script pattern is the canonical mechanism for ductus-shipped files that adopters extend (analogous to constitution/AGENTS.md handling for similar boundary).

### Quality

Migration subsection in `ductus.md` correctly handles the case where adopters already have a `pre-commit` from 017 (rename in place to `ductus-pre-commit`, install new stub). Idempotent across re-runs.

### Efficiency

N/A.

### Simplicity

Inlining `install.sh` into `ductus.md` collapses two files into one description; the operator follows the markdown rather than running an additional script.
