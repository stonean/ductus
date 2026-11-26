---
spec: 024-rule-loader
last-run: 2026-11-22T00:13:17Z
reviewed-against: ba69ef5d42d88adfb5e16a2f71f2a531340c9766
diff-base: 2983d2a34c07c501545bb76512b6c7c9555f11a5
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 1
scope: 1
unread: []
excluded: 64
contract-breaks: 0
skipped-passes: []
reviewed-digest: {}
criteria-judged: 9
criteria-met: 9
criteria-verdicts:
  - label: AC1
    digest: acb957916274aed9819a6fee046989860d49a5c9a7cf2faaf827e0de4e55c416
  - label: AC2
    digest: 2db2e52de9f63c17afb98fc31cf5738c6a7e3c5acbefb14dfbbaccb93db62c12
  - label: AC3
    digest: fb8f599a34e2aaae1b5ef91abbc0a9c38b80d408c4967b2f801f95fdbeb0e677
  - label: AC4
    digest: 54f9b48c9c8e7e86c053ade5bec4fbbd2c6ed962438f76ee395cec3cdc0ee704
  - label: AC5
    digest: 9e2a213602096db2b17451647beeb1744f4865b56b1a4e06e9951656448fdf71
  - label: AC6
    digest: 4585fbbf4534bf1886bf350e59e48083bd6a10c5d019585145f25b53bddeba7e
  - label: AC7
    digest: 19e6520b605a96d755d3c09259fe0f0551882c64c251f0bcf0ce4c0148c73339
  - label: AC8
    digest: 7e07427f9541968b6a6cc014f53f4d64ba2cab54a80e7de34e14a0ef9bc3f41b
  - label: AC9
    digest: f520f56a4f74f6a901acbd3a0649f0af47cb8f312ab4e1478901049fb0c203d7
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Review — 024-rule-loader

## Summary

Five-pass review of 024's reopening by 925c6905 (020's task 18, AC17), diff-base 2983d2a3, base-source reopen, at HEAD ba69ef5d. The scope is `specs/024-rule-loader/spec.md`, which 925c6905 edited to say a review leaves out a rule file whose header gives it to `/ductus:analyze`. Passes (part P1, one reviewer for 020 and the eight specs 925c6905 reopened) read those hunks and each scope file whole against the five rule files `discover-rule-files` selected (232,805 bytes, each read whole against `wc -c`): no finding and no observation. The seven other rule files are judged by analysis alone (020's AC17), so the efficiency pass ran on its own definition. The criteria step (part C5) judged every criterion individually: 9 of 9 met, driven with the release binary's `discover-rule-files` on a fixture (the backend surface, an unrecognized suffix's warning, the analysis-alone notice, `--all` selecting all 13 files), with `scripts/lint-rule-filenames.sh` and `scripts/lint-rule-ids.sh` exiting 0. Examined 1 of 1; nothing unread.

9 of 9 acceptance criteria judged met.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Deferred findings

*None.*

## Acceptance criteria

- AC1 — met: `scripts/lint-rule-filenames.sh` exits 0 over the 12 `framework/rules/*.md` files and is wired at `.github/workflows/framework-checks.yml`; `framework/constitution.md` section rules carries the closed-suffix paragraph.
- AC2 — met: `discover_rule_files.rs` classifies by suffix; driven on a fixture with --detected-surface backend it selected cross and backend files and no frontend one; `framework/commands/review.md` step 2 names no security-*.md as selection criteria; 020 AC17 narrowing shown by notices for the analyze-only headers.
- AC3 — met: Driven: `accessibility-frontend/api-backend/performance-frontend` are discovered by suffix with no `AGENTS.md` edit and are left out of review only by the AC17 header test (notice shown), as the narrowing in the criterion states; --all selects all 13 incl. these.
- AC4 — met: `framework/rules/configuration-cross.md` exists, `configuration.md` absent, CFG ids unchanged, `lint-rule-ids.sh` passes (308 IDs); no live ref to the old path except `check_artifacts.rs` rename-pattern `doc/test` fixtures; `specs/README.md` line 9 names the rename as in git history.
- AC5 — met: `framework/reference/review.md` Load rules and Notes for adopters restrict the `AGENTS.md` reference to files outside the rule-file directory (extra-files); `discover_rule_files.rs` test `extra_files_are_listed_and_an_unreadable_file_is_named`.
- AC6 — met: Driven: `internal-notes.md` loaded for all stacks with the exact warning text from `unrecognized_suffix_notice`; test `unrecognized_suffix_loads_for_all_stacks_with_warning`; AC17 narrowing is exercised by the analyze-only notices.
- AC7 — met: `discover_rule_files.rs` pushes the closing 'loading rule files: <list>' notice (seen in driven output as last notice); test `notice_ordering_unrecognized_then_disabled_then_loading`.
- AC8 — met: `framework/reference/review.md` Notes for adopters describes suffix auto-discovery and the residual `AGENTS.md` role; the old 'automatically loads anything ... referenced from `AGENTS.md`' line is gone.
- AC9 — met: `framework/reference/analyze.md` Rules section invokes discover-rule-files with all (no stack filter); driven --all returned all 13 files; test `all_selects_every_file_past_surfaces_and_the_disabled_filter`.

## Observations

*None.*

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/review.md`
- `.ductus/reference/analyze.md`
- `.ductus/reference/review.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-review.md`
- `.ductus/docs/agents/developing-ductus.md`
- `docs/rules.md`
- `framework/commands/analyze.md`
- `framework/commands/review.md`
- `framework/reference/analyze.md`
- `framework/reference/review.md`
- `runtime/CHANGELOG.md`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/primitives/brief_re_read.rs`
- `runtime/src/primitives/discover_rule_files.rs`
- `runtime/src/primitives/fix_rounds.rs`
- `runtime/src/primitives/resolve_anchor.rs`
- `runtime/src/primitives/start_fix_rounds.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/schema/descriptions.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/line_ending_discipline.rs`
- `scripts/analyze/check.py`
- `scripts/analyze/detect.py`
- `scripts/analyze/step15-brief.md`
- `scripts/mutation/analysis-design-fix-067.py`
- `scripts/mutation/command-reference-split.py`
- `scripts/mutation/exec-scenario-anchors-067.py`
- `scripts/mutation/owner-reviews-067.py`
- `scripts/tests/test-analyze-scripts.py`
- `specs/020-code-review/plan.md`
- `specs/020-code-review/spec.md`
- `specs/020-code-review/tasks.md`
- `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/034-performance-backend-rules/spec.md`
- `specs/037-observability-backend-rules/plan.md`
- `specs/037-observability-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/plan.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/039-reliability-backend-rules/plan.md`
- `specs/039-reliability-backend-rules/spec.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/068-api-backend-rules/spec.md`
- `specs/069-accessibility-frontend-rules/spec.md`
- `specs/070-performance-frontend-rules/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md`
- `specs/079-review-and-analysis-work-converges/analysis.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/inbox.md`

## Unexamined governance

*None.*
