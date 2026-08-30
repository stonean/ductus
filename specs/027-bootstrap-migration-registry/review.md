---
spec: 027-bootstrap-migration-registry
reviewed-at: 2026-08-30T14:57:14Z
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

0 MUST, 0 SHOULD, 0 low-confidence. Reviewed the AC30 delta, which is documentation only: §Adopter State and `framework/migrations.toml`'s header now record that the registry applies an entry once per repository, and that a migration must not be the sole remedy for a defect in per-contributor state. No code changed under this spec. This repo has no `specs/rules/` directory, so no rule files were loaded and no finding can be rule-cited; the zero counts should be read as an uncited correctness and consistency pass. The claim was checked against the corpus rather than asserted: all twelve registry entries' `target_paths` were enumerated, and the two that touch per-contributor state — `session-file-consolidate` and `ductus-rename`, both moving the gitignored session file — were confirmed to self-heal, since a skipped contributor's next `/{project}:target` writes the active path under the newest-tier rule and the procedure exits silently when the legacy file is absent. That check corrected an earlier draft of this scenario, which had claimed every entry targets repo-shared state; the recorded rule is about remedy rather than paths precisely because that broader claim is false. `{config_dir}/commands/` was verified tracked, not per-contributor (the shipped `.gitignore` excepts it), so `workflows-sunset` is not a counter-example. The `target_paths` question raised when the permission retirement was first routed here is recorded as resolved by existing precedent — `gitignore-marker-rename` already targets a file it edits rather than removes, and Family 10 constrains only `framework/`-prefixed entries — so no registry field changed and Family 10 still passes.

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
