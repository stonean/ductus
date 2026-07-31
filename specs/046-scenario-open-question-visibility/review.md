---
spec: 046-scenario-open-question-visibility
reviewed-at: 2026-07-31T02:10:51Z
reviewed-against: 931cfee1e2b94b40860a2f7c7740560a7c17106e
diff-base: 8c4ead71b5e0a5947ea40b3c51647c77623ec9a1
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 6
skipped-passes: []
---

# Review — 046-scenario-open-question-visibility

## Summary

Re-run against an unchanged implementation: 0 MUST, 0 SHOULD, 0 low-confidence. Verified rather than assumed — `git diff ddba6a4..HEAD -- runtime/src/ runtime/tests/` is empty, so no code has changed since the prior clean review; the only movement in the window is markdown (046's Motivation rewritten to past tense after the analyze grounding finding, this review.md, the inbox, and 022's status line returning to done). The five passes therefore reproduce the prior result, which is the documented idempotency invariant: review output is a function of code plus rules, never of session state. The three findings from the first pass remain fixed in ddba6a4 — the triplicated scenario-name dedup is now the single shared read_spec::scenario_names with the grouping precondition documented once and pinned by its own test; the duplicated per-file path allocation is hoisted; and the spec no longer promises that an unreadable scenario "surfaces as informational" when it correctly surfaces nothing. Security, efficiency, reuse, quality, and simplicity all clean against the 8 loaded rule files. Deterministic corroboration at this HEAD: cargo test 882 passed, clippy -D warnings clean, markdownlint clean, framework self-audit exit 0, /gov:analyze 0 blocking and 0 advisory. Captured issues lists all six inbox additions in the window per the documented mechanical diff; the last is explicitly marked as external to this work — it was filed from another project and is not an auto-capture from 046. Not blocking; 046 is already `done`.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] `/implement` hard-errors on an uncommitted spec directory: `derive-boundary` diffs against the spec dir's first commit, so an untracked spec dir halts the walk at step 2 with an operational error rather than a domain outcome. Nothing upstream catches it — specify/clarify/plan all advance status on an uncommitted dir.
- [ ] Sweep `026-framework-self-audit`'s three scenario open questions into `## Resolved Questions` with their trigger conditions, per the convention 046 settled. 026 is `done`, so `check-artifacts` reports a blocking finding against it. Deferred by user decision (option B).
- [ ] This repo's `.govern/config.toml` has no `[host]` block, so the runtime falls back to the directory basename `govern` while the installed commands are `/gov:*` — every rendered next-action names a namespace that does not exist.
- [ ] Spec 045's link-adjacent open-state check will fire on every completed spec's Motivation section unless it accounts for them. Evidence: three of 046's four Motivation bullets became false the moment 046 shipped; `041` is `done` and still claims "govern has no command to reclaim that space". Decide during 045's clarify — it changes 045's acceptance criteria.
- [ ] Release sequence for the 045/046 work — do not tag `gvrn-v` until 046 (done), 022 (done), 045, and inbox grooming are complete. 20 commits unpushed on local `main`.
- [ ] NOT captured by this work — external, present in the window diff: a `read-tasks` parser bug filed from the `magpie` project (a `- [ ] Done when: …` checkbox line is classified as the done-when clause rather than a checkbox).

## Skipped passes

*None.*
