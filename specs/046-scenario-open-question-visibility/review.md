---
spec: 046-scenario-open-question-visibility
reviewed-at: 2026-07-31T01:49:27Z
reviewed-against: ddba6a401faf977d43d6d8c5d0f27edc9dfc7df2
diff-base: 8c4ead71b5e0a5947ea40b3c51647c77623ec9a1
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 2
skipped-passes: []
---

# Review — 046-scenario-open-question-visibility

## Summary

Re-review after resolving every finding from the prior pass: 0 MUST, 0 SHOULD, 0 low-confidence. Scope is 25 files (+886/-24 runtime source lines at first pass) covering the open-question parser fix, the sibling scenario-question field, the third pre-done gate check, the fifth check-artifacts family, the dashboard rendering, and the command/constitution documentation. All three prior SHOULD findings are fixed in ddba6a4: the triplicated scenario-name dedup idiom is now the single shared read_spec::scenario_names, whose doc comment carries the grouping precondition once instead of three times and which gained a direct test pinning both halves of that contract (adjacent duplicates collapse; ungrouped input is not silently repaired); the duplicated scenarios_dir.join per scenario file is hoisted above the read; and the spec's Edge Cases no longer promises that an unreadable scenario "surfaces as informational" when the code correctly surfaces nothing — the prose was corrected rather than the behavior, since markdown lint owns the malformed file and runs ahead of the scenario check in the same gate. Security: no findings — the change adds no network, auth, secret, or query surface; its only external input is markdown file content parsed into strings, path components come from read_dir basenames filtered by the shared listing, and traversal is validated upstream. Efficiency: no findings, measured rather than assumed — the dashboard now reads every scenario file rather than counting them, which is 83 files and an 11ms full render in this repo. Reuse is now clean: every surface routes through one collector, one parser, one ordering comparator, and one name-derivation helper. Deterministic corroboration: cargo test 882 passed, clippy -D warnings clean, framework self-audit exit 0, markdownlint clean, and all 30 acceptance criteria verified in task 9. Not blocking.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] Sweep `026-framework-self-audit`'s three scenario open questions into `## Resolved Questions` with their trigger conditions, per the convention spec 046 settled on 2026-07-30. 026 is `done`, so `check-artifacts` currently reports a blocking `scenario-open-questions` finding against it. Deferred to after 046 ships (user decision, option B).
- [ ] This repo's `.govern/config.toml` has no `[host]` block, so the runtime falls back to the directory basename `govern` while the installed commands are `/gov:*` — every rendered next-action names a namespace that does not exist. Fix is one block in `.govern/config.toml`.

## Skipped passes

*None.*
