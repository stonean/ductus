---
spec: 026-framework-self-audit
reviewed-at: 2026-08-19T01:24:46Z
reviewed-against: 4bfaaeec9e5af5d89d92811828412ca950d63cec
diff-base: e2779e5
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

Clean — 0 MUST, 0 SHOULD, 0 low-confidence, 0 observations.

Scope: Family 26 (`scripts/audit/broken-relative-links.sh`), its wiring, its scenario, AC19, and eight link repairs in this spec's own scenarios.

Two defects were found and fixed during construction rather than shipped:

- **Line numbers were wrong after any fenced block.** The first draft stripped fences with a whole-text regex, which deletes their newlines and shifts every subsequent line number — findings pointed at the wrong line, worse the deeper into the file. Caught only because a reported line came back blank when read back, which is the check's own output failing the standard it exists to enforce. Fences now toggle line by line.
- **Seven false positives from inline code spans.** Documents that discuss linking quote link syntax constantly, always inside a span. Every one was a doc correctly *describing* a link rather than making a broken one. Spans are now stripped before matching.

The exclusions are principled and counted, not silent: generated command copies have links broken by construction (the generator changes directory depth without rewriting them, so auditing them would report the generator on every run) and adopter templates resolve in the adopter's root. Both counts go to stderr, so a clean exit never reads as broader than it is. A failed file listing is a finding.

Proven red before trusted green: a seeded depth error reports the correct file, line, and correction, and a failed file listing fires the degenerate-scan guard — the latter verified by running the family's own python from a non-git directory, since `lib.sh` cds to the repo root and the branch is otherwise unreachable.

Contract conformance: sources `lib.sh` with `|| exit 1`, calls `audit_family`, renders through `emit`, exits `$drift`, read-only apart from a `$TMPDIR` intermediate the directory's contract permits. `shellcheck` clean. The python runs at top level rather than inside `$( ... )` because bash parses a backtick there as a legacy sub-shell even inside a quoted heredoc — a constraint discovered by hitting it.

The eight link repairs in this spec's scenarios change no claim and are mechanical; the spec would have stayed `done` for those alone. The reopen is for Family 26.

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

## Observations

*None.*

## Skipped passes

*None.*
