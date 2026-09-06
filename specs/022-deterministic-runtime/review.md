---
spec: 022-deterministic-runtime
reviewed-at: 2026-09-06T14:46:11Z
reviewed-against: 8c4ca744bdfb9fcd758c52fe1338d436d1cd7d2b
diff-base: 769f49ec48b7c5b7167219dcc4044da47c438dc8
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

artifact-unreadable promoted to a blocking finding at done. Zero MUST, zero SHOULD, zero low-confidence. The judgement worth recording is what was NOT changed: three of the four could-not-be-read reasons keep the never-escalate rule, and each for its own reason — no-readable-state is the check correctly declining to guess, target-missing is already gated harder by check-corpus-links at commit time, and target-unparseable is partly a deliberate symlink refusal. Only artifact-unreadable names the spec's own artifact, and only it closed a real hole: scenario-open-questions blocks at done, and an unreadable scenario contributed no questions and no finding, so a scenario with unresolved questions that would not parse passed the gate built to catch it. Both sides of the done boundary are pinned by test, and the corpus has zero instances, so the gate lands with no backlog behind it.

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
