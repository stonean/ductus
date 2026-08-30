---
spec: 023-govern-refinement
reviewed-at: 2026-08-30T15:22:00Z
reviewed-against: d1c56d429153541bbdbb6111eaaca8db9968245f
diff-base: 0ce71ab99fe2268a8f52ba9e05787758016ea365
must-violations: 0
should-violations: 1
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 023-govern-refinement

## Summary

0 MUST, 1 SHOULD, 0 low-confidence — non-blocking. Re-run correcting a defective first pass: that run recorded 0/0/0 on the claim that this repo has no rule files, which is wrong. Rules live at `framework/rules/` in ductus's own repo (`framework/commands/review.md:46`), and `discover-rule-files` selects eight for the backend surface: api-backend, concurrency-backend, configuration-cross, observability-backend, performance-backend, quality-cross, reliability-backend, security-backend. The first pass loaded none of them, so its zero counts asserted a property it had no basis to assert — itself the QUAL-CLAIM-001 shape. Scope: the AC34–AC40 delta (the `Write(path)` removal, `/ductus:configure` step 4's retired-entries list, `merge-permissions`' `revoke` argument, audit Family 32). BE-INPUT-004 is satisfied — `validate_no_traversal` still guards `path` before any filesystem operation, and `validate_revoke_disjoint` was placed after it so both run before the file is touched. BE-INPUT-002 is satisfied by construction: retirement matches an explicit allowlist of nine entries rather than denylisting a pattern shape, which is also what keeps adopter-authored entries safe. QUAL-STUB-001: no pass-through paths; every new branch either acts or returns a counted zero with an accompanying action field. One QUAL-CLAIM-001 violation was found in the new Family 32 and fixed during the pass (see findings). A second QUAL-GROUND-001 candidate was assessed and not raised as a finding: the nine retired entries in `configure/claude.md` §4 are literals with no binding proving they were ever canonical, so a typo there is a silent no-op that leaves an adopter's entry in place. Family 32b grounds only the inverse (that a retired entry is not *currently* canonical). The available guards — asserting the list against git history of the canonical set — are more fragile than the exposure, and the entries are covered by the end-to-end fixture, which is the "test that exercises the real contract" the rule accepts as compliant.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

### SHOULD: QUAL-CLAIM-001 — check 32b reported a skipped subject as an examined-clean one

- **File**: `scripts/audit/permission-entry-shape.sh:151-190`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish *"examined the subject and found nothing"* from *"could not examine the subject"*, rather than emitting the same value for both. When a code path skips part of its subject, cannot reach it, or has no basis to inspect it, its output SHOULD say so — through a distinct return variant, an accompanying status or guidance field, or a message naming what was not examined — instead of a bare zero, empty collection, or success string that a caller will read as positive assurance.
- **Finding**: Check 32b guards its retired-entries lookup with `if [ -n "$retired_start" ]`, correctly treating an absent section as a legitimate state rather than a parse failure. But the stderr summary then rendered that state as `${retired_count:-0} retired entries for overlap with the canonical set` — the same `0` a present-but-empty list would produce, and indistinguishable from a genuinely examined clean result. Since the family exits 0 in both cases, a maintainer who removed or renamed the section heading would read `0 retired entries` and a green family as confirmation that no overlap exists, when in fact no overlap was ever looked for. This is the exact shape the rule's own source catalogues in this repo's tooling.
- **Auto-fixable**: yes
- **Suggested fix**: Fixed during this pass. A `retired_summary` variable is initialised to "no retired-entries section present, so no overlap was examined" before the guard and reassigned to "$retired_count retired entries checked for overlap with the canonical set" inside it, so the two states read differently on stderr. The header comment records the distinction beside the existing empty-extraction note. Verified by renaming the section heading and confirming the summary switches.

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
