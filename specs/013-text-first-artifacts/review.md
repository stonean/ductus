---
spec: 013-text-first-artifacts
reviewed-at: 2026-08-03T02:44:40Z
reviewed-against: 8891da925ff7b5f8d5c2892ffd1689bb8f8d4915
diff-base: 096dbc0cf65a2322c91bfa895a825ea60c5a23f8
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 1
skipped-passes: []
---

# Review — 013-text-first-artifacts

## Summary

Reopened by the scenario back-edge for `past-tense-motivation-convention` (task 23). The change in this window is a single addition to the `## {Section}` scaffolding comment in `framework/templates/spec/spec.md` — authoring guidance telling a spec author to write `## Motivation` in the past tense, since a Motivation describes the world before the feature and its present-tense claims go false on ship. No code, no schema, no command behavior. All five passes ran and found nothing: there is no executable surface for the security, quality, efficiency, or reuse rules to bind to, and the addition is prose inside an HTML comment that never reaches a rendered spec body. Scope note: `compute-review-scope` selected a stale, malformed `plan-affected` set for this spec (it contains table-header cells and parenthetical prose, and the "larger set wins" rule chose it over the accurate `modified-since` set); the review was run against the files this window actually changed. That scope defect is logged to the inbox for routing under 022.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] bug: `compute-review-scope` returns an unusable scope and a polluted captured-issues list — plan-affected is not parsed as a table, and captured-issues takes raw added lines rather than the shared comment-aware bullet grammar.

## Skipped passes

*None.*
