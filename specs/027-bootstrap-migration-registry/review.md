---
spec: 027-bootstrap-migration-registry
last-run: 2026-10-15T22:47:52Z
reviewed-against: af60cd47e482b6f41af6925a2e255f687895b55a
diff-base: b8eedc430f12e398efe15d2ea82e4c7b498af241
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
scope: 21
skipped-passes: []
reviewed-digest:
  scenarios/migration-chain-reference-integrity.md: b187024e6ed8fee90fe31be0f2bbcd4786782f8e4395a89c09142f20218f914d
  scenarios/migrations-apply-once-per-repo.md: ef57af25a1eb071076c60b005ffdbf9f3a1b2afad889ba939b230679b9d4f9b4
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 027-bootstrap-migration-registry

## Summary

Scoped re-review at af60cd47 of one mechanical repair to a done spec. Base overridden with --since=b8eedc43, the commit before the repair, because the change being re-reviewed is five tokens and the default window spans 027's whole history since its last reopen. The repair restores five substitutions 050's retired-filename sweep (510eb25c) landed in sentences whose subject is the old name: the migrations scenario's "`ductus-rename` moves `.govern/`" (the sweep had made it move `.ductus/`), AC2's and the plan overview's "`.governance.toml` rename" in the list of migrations by the file each removed, and the plan row and tasks heading naming the governance-config-rename procedure "`.governance.toml` → `.govern.toml`" (the sweep had made both sides `.ductus/config.toml`). Each was checked against the procedure it names: framework/migrations/governance-config-rename.md renames `.governance.toml` to `.govern.toml`, and framework/migrations/ductus-rename.md moves `.govern/`. The scenario is byte-identical to its pre-sweep version. Sweep residue is mechanical under §spec-lifecycle, so 027 is not reopened. Examined 4 of the scope: the scenario read in full, and the repaired lines of spec.md, plan.md and tasks.md with their surrounding sections. The window's other modified files belong to 019 and 022; the rest of the scope is migration procedures and audit scripts this repair does not touch. Rule files: all eleven framework/rules/*.md; quality-cross.md (QUAL-CLAIM-001) applies and nothing else fires. No finding, no observation.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
