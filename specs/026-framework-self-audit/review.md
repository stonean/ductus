---
spec: 026-framework-self-audit
reviewed-at: 2026-08-19T01:07:52Z
reviewed-against: 9c06b2dfd5f16618c50fd3a0186caf534a517778
diff-base: 38b97b7413e04a6bf5e7f6dd712a7c60e7862f95
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

Clean — 0 MUST, 0 SHOULD, 0 low-confidence, 0 observations.

Scope: two corrections to the spec body's Resolved Questions and two to the `audit-ci-hard-gate` scenario. No code, no check-family behavior.

The two in this spec were the most consequential of the nine corrected across four documents, because neither was merely descriptive:

- The bootstrap-order entry told future authors to register new generators in `markdown-only-pipeline.yml`'s orchestration step. That is a forward-looking instruction, and it pointed at a file `048-govern-acquired-runtime` removed — an author following it would have gone looking for something that does not exist. Now names `framework-checks.yml`.
- The `audit-ci-hard-gate` scenario carried two markdown links to that same deleted file, which resolve to nothing, and described the gate flip as outstanding work. Verified against CI rather than assumed: `continue-on-error` is absent from the audit step in `framework-checks.yml` and from `runtime-release.yml`'s `audit` job, so both flips have shipped. The scenario now records that, and names the workflow without a link precisely because the target is gone.

A scenario is a durable requirement document, so a broken link and a shipped-but-described-as-pending gate are worse there than in a spec body: the scenario is what a reader consults to learn what the system currently promises.

No audit family covers a markdown link to a deleted CI file — Family 8 looks for renamed *commands*, and `check-orphaned-references` scopes to adopter-owned referrers. Deliberately not filed as a new family: the corpus turned up exactly one instance, and the measurement that would justify a check has not been done. Recorded here rather than in the inbox, per the standing rule against frontfilling machinery observations.

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
