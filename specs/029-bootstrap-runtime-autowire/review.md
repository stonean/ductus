---
spec: 029-bootstrap-runtime-autowire
reviewed-at: 2026-08-16T12:53:08Z
reviewed-against: c24f40e6b870ff46ef399f6ab6a85f8e0724d60c
diff-base: 2cca7d6d729848a3cafc78b4f7498b5fbdce197b
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 029-bootstrap-runtime-autowire

## Summary

No findings. Scope resolved to 48 files, of which exactly one is code — scripts/audit/sibling-coupling.sh. The other 47 are command sources, bootstrap prose, the constitution, migrations, workflow YAML, a lockfile, a golden fixture and spec artifacts; those are `/ductus:analyze`'s subject, not the five code passes'. This report states that count deliberately rather than asserting a clean bill over the whole spec: the code passes examined one file.

That file is exemplary against the rule set. It uses POSIX-only match() with RSTART/RLENGTH rather than the 3-argument GNU form, and it opens with an explicit precondition probe — `awk 'BEGIN { if (match("x", /x/)) exit 0; exit 1 }'` — that emits a finding and returns 1 when awk cannot evaluate match(), with an inline comment citing QUAL-CLAIM-001 and the incident it came from. This is the direct remediation of the defect AGENTS.md's first Design Principle records, and it was exercised rather than assumed: the family runs clean here under BSD awk (20200816), the exact environment where the original GNU-extension abort produced a silent pass. QUAL-STUB-001, QUAL-GROUND-001 and QUAL-CLAIM-001 are clean on it.

The broader observation that the repo runs no shellcheck over its shell surface is recorded against 013 and 026, whose scopes contain the bulk of those scripts; it is not re-filed here, where the single in-scope script is compliant on inspection.

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
