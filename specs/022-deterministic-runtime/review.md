---
spec: 022-deterministic-runtime
reviewed-at: 2026-08-19T01:07:52Z
reviewed-against: 9c06b2dfd5f16618c50fd3a0186caf534a517778
diff-base: 38b97b7413e04a6bf5e7f6dd712a7c60e7862f95
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Clean — 0 MUST, 0 SHOULD, 0 low-confidence, 0 observations.

Scope: four prose-claim corrections in the spec body, no code. Each described `.github/workflows/markdown-only-pipeline.yml` in present tense; `048-govern-acquired-runtime` renamed that workflow to `framework-checks.yml` and retired the absent-from-`PATH` invariant it asserted.

The corrections are grounded rather than assumed. Before rewriting, both lints the prose makes claims about were located in current CI: `lint-tool-coverage.sh` at `framework-checks.yml:61` and `lint-procedure-parseability.sh` at `framework-checks.yml:133`. So the §Bash script relationships claim was stale prose about a live check, not a dead check — a distinction that changes the fix from "restore a lost lint" to "correct a claim", and one worth making before editing rather than after.

The §Non-Goals entry keeps its reasoning intact: the rejected per-command opt-out is still rejected, and only the supporting clause about the retired workflow moved to past tense. Correcting a rationale's factual support must not silently relitigate the decision it supports.

AC6 and AC12 already carried supersession annotations and were not touched.

This spec was closed once with these claims left standing, on the reasoning that the fix would reopen it and that Family 8's disposition permits leaving prose as-is. That was the wrong call: the disposition is the maintainer's, not the reviewer's, and a release is the moment for everything to be true. Reopened and fixed.

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
