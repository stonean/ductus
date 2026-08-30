---
spec: 027-bootstrap-migration-registry
reviewed-at: 2026-08-30T15:22:00Z
reviewed-against: d1c56d429153541bbdbb6111eaaca8db9968245f
diff-base: 0ce71ab99fe2268a8f52ba9e05787758016ea365
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 027-bootstrap-migration-registry

## Summary

0 MUST, 0 SHOULD, 0 low-confidence — non-blocking. Re-run correcting a defective first pass, which recorded 0/0/0 on the claim that this repo has no rule files. That is wrong: rules live at `framework/rules/` in ductus's own repo (`framework/commands/review.md:46`), and `discover-rule-files` selects eight for the backend surface. The first pass loaded none, so its zeros asserted a property it had no basis to assert. This pass loaded all eight. Scope is the AC30 delta, which is documentation only — §Adopter State and `framework/migrations.toml`'s header comment; no code changed under this spec, and the eight rule files govern code paths, so none of their verification clauses has a subject here. That is why the count is genuinely zero rather than unexamined, and the distinction is the point of this re-run. The substantive check on this delta is not rule-mapped but corpus-grounded, and it was performed: all twelve registry entries' `target_paths` were enumerated, and the two touching per-contributor state — `session-file-consolidate` and `ductus-rename`, both moving the gitignored session file — were confirmed self-healing, since a skipped contributor's next `/{project}:target` writes the active path under the newest-tier rule and the procedure exits silently when the legacy file is absent. That enumeration corrected an earlier draft of this scenario, which claimed every entry targets repo-shared state; the recorded rule is about remedy rather than paths precisely because the broader claim is false. `{config_dir}/commands/` was verified tracked rather than per-contributor (the shipped `.gitignore` excepts it), so `workflows-sunset` is not a counter-example. `target_paths` needed no change: `gitignore-marker-rename` already targets a file it edits rather than removes, and Family 10 constrains only `framework/`-prefixed entries, so Family 10 still passes.

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
