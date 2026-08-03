---
spec: 026-framework-self-audit
reviewed-at: 2026-08-03T15:43:22Z
reviewed-against: 071460a217f81a257e8ee5c652bb08eb89344299
diff-base: 1eda6f6f626eb368473b1dcae957392ba0e210d0
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 026-framework-self-audit

## Summary

Re-review after the CI fix that Family 19's own release gate forced. **0 MUST, 0 SHOULD — not blocking.** The diff is two workflow files gaining `fetch-depth: 0` on the checkout feeding `run-all.sh`, plus the scenario edge case and CHANGELOG entry recording why. No script logic changed. The finding this closes is worth stating plainly: Family 19 resolves each spec's `review.reviewed-against` against history, `actions/checkout` clones a single commit by default, and so the first tag after wiring the family reported "not a commit in this repo" for all 48 specs and skipped the publish — while passing locally on every run, because a developer checkout has full history. Local green was not evidence the check worked in CI; the environments differ in exactly the dimension the check depends on. The deliberate non-change is the other half: the unresolvable-sha case stays a **finding** rather than becoming a skip, because softening it would have turned a gate that was *wrong* into one that was *vacuous* in the environment that matters, which is the worse failure and the one `QUAL-CLAIM-001` exists to prevent. Shellcheck clean, both workflow edits structurally verified (`with:`/`fetch-depth` at the correct indent under each checkout step), markdownlint clean across 390 files, the 19-family audit exit 0 locally, and `check-review-gate` passing on all 48 specs.

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

## Skipped passes

*None.*
