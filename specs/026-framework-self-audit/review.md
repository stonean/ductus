---
spec: 026-framework-self-audit
reviewed-at: 2026-08-19T00:57:34Z
reviewed-against: 9dead357eec9a932ae6191ff1f14c3be5131afac
diff-base: 1c1b6225b397ef7370cdbac768ace046544b1573
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

Clean — 0 MUST, 0 SHOULD, 0 low-confidence, 0 observations.

Scope: Families 24 and 25 — `scripts/audit/rename-sweep-residue.sh`, `scripts/audit/unbalanced-inline-markup.sh`, their wiring in `run-all.sh`, the `scripts/audit/README.md` entries, both scenarios, and AC17/AC18.

Both families were **measured before being designed**, per §recommendations: the deciding quantity was computed for each candidate detector rather than argued from shape. Family 24's union reports 8 findings at the pre-repair commit — exactly the 8 real sites — and 0 at HEAD. Family 25 reports 2 — exactly the two malformed entries — and 0 at HEAD. Family 25's two-file scope is a measured choice, not an omission: the wider corpus carries 283 lines of legitimately wrapped bold, so widening would trade 2 real findings for 283 false ones.

Both were **proven red before being trusted green**, per §design-principles. Family 24 against two seeded residue sites; Family 25 against both original defect shapes, a seeded wrapped bullet, and a missing target file. Each returns exit 1 on the seeded input and 0 after restore.

Neither family can report clean without saying what it examined. Family 24 emits its examined-file count and treats a zero-file scan as a finding; Family 25 names its two targets on stderr, states that corpus-wide balance is *not* checked, treats a missing target as a finding rather than a skip, and reports a wrapped bullet rather than narrowing its own scope once the single-line convention lapses.

Contract conformance: both source `lib.sh` with `|| exit 1`, call `audit_family`, emit through `emit`, end on `exit "$drift"`, are directly invocable from any working directory, and are read-only. Bash 3.2 compatible with POSIX sed/grep/awk only — the portability trap that left Family 7 dead on macOS. `shellcheck` clean, with one scoped `SC2016` disable carrying its rationale.

One defect was found and fixed during construction rather than filed: Family 24 initially reported its own README entry, which quotes the residue as an example. Suppressing by file would have been the wrong repair, so the detector now strips code spans and quoted spans before matching — a discriminator with a real basis, since the genuine residue reads as ordinary prose, which is precisely what let it survive.

Verification at HEAD: 25 families green, markdownlint clean over 432 files, `shellcheck` clean.

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
