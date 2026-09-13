---
spec: 022-deterministic-runtime
reviewed-at: 2026-09-13T00:43:52Z
reviewed-against: 317d6782e18eea6a3bbf7c68e2c7f4ceb42ce766
diff-base: d041432c6d442240cb8a1c2945e210f52572f62a
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 19
scope: 43
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Five-pass review of the write-review denominator (scenario a-review-states-what-it-read), scoped with --since=d041432 because 022's default base resolves 89 entries / 37.9k lines against a change touching six files. Read in full: check_review_agreement.rs, the changed paths of write_review.rs, the three schema structs, the new MCP wire test, framework/commands/review.md, analyze.md, implement.md, plan.md, specify.md, status.md, target.md, README.md, framework/constitution.md, 020's spec, and 022's spec, tasks and new scenario. Security: no new input crosses a trust boundary — examined is an integer, scope is derived internally, and both render through the single-line-validated frontmatter path. Quality: proven red before green on fixtures and against the live corpus (a review written without --examined raised examined-unstated on 017; the same review with it came back clean), and the pre-field grandfather is structural rather than dated, since such a record carries no scope to judge against. Reuse: scope resolution delegates to compute-review-scope; the new findings extend the existing ReviewAgreementFinding shape. Efficiency: one extra git walk per review. Simplicity: clippy's arg-count and line-count limits were met by extracting Buckets, RecordedCounts and validate_scalar_fields rather than by an allow. NOT examined, stated rather than implied: runtime.yml, runtime-tools.txt, legacy-prose-commands.txt, CHANGELOG.md, Cargo.lock, io.rs, lib.rs, main.rs, lint-procedure-parseability.sh, and 022's plan.md and the unchanged bulk of tasks.md — 19 of 32 live scope entries were read, which is what examined records. 0 MUST, 0 SHOULD, 0 low-confidence.

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

## Unexamined governance

*None.*
