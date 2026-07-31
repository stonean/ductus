---
spec: 046-scenario-open-question-visibility
reviewed-at: 2026-07-31T01:42:26Z
reviewed-against: 96c97e5b571cec68cb0f5d1618b272b62ed6496d
diff-base: 8c4ead71b5e0a5947ea40b3c51647c77623ec9a1
must-violations: 0
should-violations: 3
low-confidence: 0
captured-issues: 2
skipped-passes: []
---

# Review — 046-scenario-open-question-visibility

## Summary

Reviewed the 046 implementation across all five dimensions against the 8 loaded rule files: 0 MUST, 3 SHOULD, 0 low-confidence. Scope is 24 files (+886/-24 runtime source lines) covering the open-question parser fix, the sibling scenario-question field, the third pre-done gate check, the fifth check-artifacts family, the dashboard rendering, and the command/constitution documentation. Security: no findings — the change adds no network, auth, secret, or query surface; its only external input is markdown file content parsed into strings, path components come from read_dir basenames filtered by the shared listing, and traversal is already validated upstream. Efficiency: no findings — the dashboard now reads every scenario file rather than counting them, so this was measured rather than assumed: 83 scenario files across the repo, full dashboard render in 11ms. Reuse is strong overall (the feature routes every surface through one collector, one parser, and one ordering comparator by design), with one exception recorded below where the name-derivation idiom was left triplicated. The remaining two findings are a duplicated path allocation and a spec-prose overpromise that the code does not honor. Deterministic corroboration: cargo test 881 passed, clippy -D warnings clean, markdownlint 0 issues across 361 files, framework self-audit exit 0, and all 30 acceptance criteria verified in task 9. Not blocking.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

### SHOULD: reuse — scenario-name dedup idiom duplicated across three primitives

- **File**: `runtime/src/primitives/dashboard.rs:467-476`
- **Rule**: Reuse pass: identify logic that duplicates existing utilities or that should be extracted into shared code.
- **Finding**: The three-line idiom that maps ScenarioOpenQuestion entries to their scenario slugs and collapses runs with dedup() appears verbatim in three places: check_review_gate.rs scenario_question_block, check_artifacts.rs check_scenario_open_questions, and dashboard.rs load_spec. The explanatory comment about entries arriving grouped in shared scenario order is duplicated with it. The feature deliberately centralised the question *collection* into one collector so the three surfaces cannot disagree; the name-derivation is the same class of shared knowledge and is currently not centralised, so a future change to grouping or dedup semantics must be made in three places or the surfaces will diverge.
- **Auto-fixable**: no
- **Suggested fix**: Extract a helper alongside collect_scenario_open_questions, e.g. `pub(crate) fn scenario_names(questions: &[ScenarioOpenQuestion]) -> Vec<&str>`, carrying the grouping precondition in one doc comment, and call it from all three sites.

### SHOULD: simplicity — scenarios_dir.join(&name) computed twice per scenario file

- **File**: `runtime/src/primitives/read_spec.rs:74-83`
- **Rule**: Simplicity pass: identify unnecessary indirection and mechanically derivable simpler forms.
- **Finding**: collect_scenario_open_questions builds the same PathBuf twice — once inline as the argument to read_text, and again two lines later as `let scenario_path = scenarios_dir.join(&name)` for the split_frontmatter error path. A second allocation per scenario file, and two expressions a reader must confirm are the same path.
- **Auto-fixable**: yes
- **Suggested fix**: Hoist the join above the read: `let scenario_path = scenarios_dir.join(&name);` then `let Ok(content) = read_text(&scenario_path) else { continue };` and pass `&scenario_path` to split_frontmatter.

### SHOULD: quality — spec promises an unreadable scenario 'surfaces as informational'; the code surfaces nothing

- **File**: `runtime/src/primitives/read_spec.rs:74-76`
- **Rule**: Quality pass: detect contract violations against the spec's stated behavior.
- **Finding**: The spec's Edge Cases states that an unreadable or malformed scenario file 'surfaces as informational, matching how an unreadable cross-reference target is classified'. The implementation silently `continue`s: no finding, no notice, no diagnostic on any surface. The code's own comment is honest (it claims only that an unknown is never escalated into a defect), so the overpromise is in the spec prose. Acceptance criterion 22 — never blocks the gate, never produces a blocking finding — is satisfied either way, so this is a documentation-versus-behavior gap rather than a functional defect, but a reader of the spec would expect a signal that does not exist.
- **Auto-fixable**: no
- **Suggested fix**: Correct the spec's Edge Cases entry to match the shipped behavior — an unreadable scenario contributes no questions and is not separately surfaced; markdown lint owns the malformed file — or add an informational channel if the surfacing is genuinely wanted. The former matches the existing sentence 'The defect is the file's, and lint owns it.'

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] Sweep `026-framework-self-audit`'s three scenario open questions into `## Resolved Questions` with their trigger conditions, per the convention spec 046 settled on 2026-07-30. 026 is `done`, so `check-artifacts` currently reports a blocking `scenario-open-questions` finding against it. Deferred to after 046 ships (user decision, option B).
- [ ] This repo's `.govern/config.toml` has no `[host]` block, so the runtime falls back to the directory basename `govern` while the installed commands are `/gov:*` — every rendered next-action names a namespace that does not exist. Fix is one block in `.govern/config.toml`.

## Skipped passes

*None.*
