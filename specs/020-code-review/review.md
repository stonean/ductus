---
spec: 020-code-review
reviewed-at: 2026-08-27T22:43:13Z
reviewed-against: bb96fef3d83dec618fbadbccc7e021a73720ce5d
diff-base: 7e98cc48963acaad87b9c2d86071bc8d5eaa5c27
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 2
skipped-passes: []
---

# Review — 020-code-review

## Summary

0 MUST, 0 SHOULD, 0 low-confidence across all five passes; not blocking.

**Diff base was overridden with `--since`, deliberately.** `compute-review-scope` resolves the base to the commit at which the spec advanced to `in-progress`, which here is `31348ff` — the same commit that carries task 12's work, because the back-edge flip and the implementation landed together. Reviewing "since" that commit would have excluded `check_command_flags.rs`, `command-flag-hint-parity.sh`, and the `argument-hint` correction: the entire subject. The base is its parent instead, so the scope covers what was actually built. Worth noting as a general shape — whenever an amend's status flip is committed with the work it authorises, the default base excludes the work, and the review reads clean because it looked at nothing.

Scope covered the new `check-command-flags` primitive, `/audit` Family 30's entry-point script, `framework/commands/review.md`'s `$ARGUMENTS` parse step and corrected `argument-hint`, and the three-way family registration across `run-all.sh`, `audit.md`, and `scripts/audit/README.md`.

Security: the primitive reads a fixed repo-relative directory with no caller-supplied path, so there is no traversal surface; the family script interpolates nothing into a shell word — the runtime's JSON reaches `python3` on stdin and every `emit` argument is quoted. Quality: the byte scanner's slice bounds are safe on non-ASCII input (a `0x2D` byte can only be ASCII `-` in UTF-8, so both ends of every harvested span are char boundaries), and the finding loop is fed by a here-doc rather than a pipe, so `drift` persists in the current shell — the subshell bug this shape is prone to is absent. Efficiency and simplicity found nothing against the loaded rules.

One reuse item in `check_command_flags.rs` maps to no loaded rule; it was already captured to the inbox by 022's review of the same file and is not re-rendered here. Re-supplying it on this run did append a second bullet before it was removed: `write-review`'s dedup key is the whole rendered line, and that line ends with the reviewing feature's name, so the same observation recorded from two specs' reviews does not dedup against itself. Recording it once, on the spec whose review first saw it, is the discipline that avoids this.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] convention: `check_command_flags::argument_hint` hand-rolls frontmatter-block extraction that `primitives::split_frontmatter` already provides — `runtime/src/primitives/check_command_flags.rs` (captured during review of 022-deterministic-runtime)
- [ ] perf: the adopter pre-commit hook now performs two independent full walks of the tracked spec corpus per commit — `framework/bootstrap/hooks/ductus-pre-commit` (captured during review of 022-deterministic-runtime)

## Observations

*None.*

## Skipped passes

*None.*
