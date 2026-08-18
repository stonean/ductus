---
spec: 026-framework-self-audit
reviewed-at: 2026-08-17T20:22:00Z
reviewed-against: 7af28c3
diff-base: f9dbc315c7a3801e64928c605ce48f603a7807ae
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

0 MUST violation(s), 0 SHOULD violation(s), 0 low-confidence finding(s). blocking: no.

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

- bug: gen-spec-deps.sh corrupts YAML block-list `dependencies:` frontmatter into invalid YAML while reporting success — given a `dependencies:` key whose value is an indented block-list item, it rewrites the key to `dependencies: []` and leaves the orphaned list item beneath it, then prints `Updated <path>` and exits 0. Every spec in this repo uses the inline flow form, which is why it has never surfaced here; an adopter who hand-writes block style has the file silently corrupted on commit by the pre-commit hook. Found while building Family 22's fixture. — `.ductus/scripts/gen-spec-deps.sh`
- convention: 026's plan.md Affected Files still lists scripts/audit/registry-equivalence.sh, deleted when Family 3 was retired by 043 — so compute-review-scope resolves a plan-affected set that is larger than the real modified set and wins the larger-of rule, scoping the review to files the change never touched while omitting the ones it did. — `specs/026-framework-self-audit/plan.md`

## Skipped passes

*None.*
