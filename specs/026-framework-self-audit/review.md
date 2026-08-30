---
spec: 026-framework-self-audit
reviewed-at: 2026-08-30T19:57:37Z
reviewed-against: ee970460de78db1d3bc0eca849aecf73ed761065
diff-base: e21c4567
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

Scope was Family 33 (`scripts/audit/readme-command-parity.sh`), the extraction of the maintainer-only command set into `scripts/maintainer-only-commands.txt`, its `lib.sh` accessor, and the three consumers now reading it — Family 16, Family 33, and `gen-help-tables.sh` — plus the registrations in `run-all.sh`, `audit.md`, and `scripts/audit/README.md`.

All five passes ran. The quality pass found one defect, fixed in `65856ce`: under `set -e` with `pipefail`, the `grep -v '^$'` that filters blank lines out of the maintainer-only list exits 1 when it matches nothing, killing `gen-help-tables.sh` before its empty-list guard could explain itself — so the fail-closed path exited 1 silently instead of 6 with the reason. That is the same defect class the guard was added to prevent, one level down: a check that could not run must not be indistinguishable from something else. It was in the reviewed task's own scope and so was fixed rather than recorded. The same commit repointed the uncovered-command message, which still named an `excluded_commands` list that no longer lives in that script.

The three-consumer fail-closed behaviour was verified rather than reasoned about: emptying the list makes Family 16 and Family 33 each emit a precondition finding and `gen-help-tables.sh` exit 6. Family 33 itself was proven red against a seeded command with no README row and green once removed, which is the vacuity guard `AGENTS.md` requires of a new family — a family that has never been observed firing is a check nobody has evidence runs.

Nothing outstanding. The reuse pass is what the extraction answers: the exclusion set had already been copied once, and Family 33 would have been the third copy of a list the parity families themselves depend on. The simplicity pass raised nothing — the family is a set comparison with no parsing of its own. One observation maps to no loaded rule and is captured to the inbox.

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

- convention: Family 33 recognizes a documented command by the backticked token `/name`, so a command that only ever appeared inside a wider code span — `/specify --supersedes`, say — would be reported as undocumented. The failure is loud and in the safe direction (a false finding a maintainer resolves by adding the bare token), but it is an undocumented constraint on how the README may write a command name, and nothing states it where an author would meet it. — `scripts/audit/readme-command-parity.sh`

## Skipped passes

*None.*
