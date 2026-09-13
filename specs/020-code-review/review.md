---
spec: 020-code-review
reviewed-at: 2026-09-13T00:43:52Z
reviewed-against: 317d6782e18eea6a3bbf7c68e2c7f4ceb42ce766
diff-base: d041432c6d442240cb8a1c2945e210f52572f62a
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
examined: 16
scope: 20
skipped-passes: []
---

# Review — 020-code-review

## Summary

Five-pass review of the examined/scope mechanism this spec's AC15 requires, scoped with --since=d041432 because 020's default diff base resolves 283 files (89k lines) against a change touching six. Read in full: write_review.rs's changed paths and check_review_agreement.rs in full, schema/primitives.rs's three structs, the new MCP wire test, framework/commands/review.md, framework/constitution.md, README.md, the adopter CI template, and this spec plus its data-model. Security: the new fields are integers rendered through the same single-line-validated frontmatter path as the counts; scope is derived, not caller-supplied, so it cannot be shrunk to hide a subject. Quality: the numerator/denominator split is the point — a caller can overstate examined but cannot hide scope, and an unstated claim records as absent rather than a computed zero. Reuse: the scope resolution calls compute-review-scope rather than a second walk, and the two new Family 31 kinds extend the existing finding shape. Efficiency: one extra git walk per review, against the run's own diff-base. Simplicity: the arg-count and line-count refactors it forced (Buckets, RecordedCounts, validate_scalar_fields) were taken rather than suppressed with allow. One defect found and fixed in 317d678: AC11 and eight siblings still told adopters to configure .govern.toml, which 049 renamed. Proven red before green on fixtures and on the live corpus. 0 MUST, 0 SHOULD, 0 low-confidence.

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
