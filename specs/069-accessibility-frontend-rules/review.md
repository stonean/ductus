---
spec: 069-accessibility-frontend-rules
last-run: 2026-11-09T00:22:58Z
reviewed-against: 25a9936b206661cc1123f32ed028c783feacec00
diff-base: 841b85f3f2ba9b883c096273e19cd0de780120f3
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
scope: 4
excluded: 52
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest: {}
blocking: false
dispositions:
  fixed: 4
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Review — 069-accessibility-frontend-rules

## Summary

Review of 069's first reopening over `841b85f3..25a9936b` (base-source `reopen`, the parent of `f59604aa`, the commit that took 069 in-progress), scope 4, examined 4. `framework/rules/accessibility-frontend.md` was read whole, all 40 rules, against the loaded rule files, the rule schema in `specs/008-security-rules/data-model.md` (§Rule entry fields, §Verification phrasing, §Severity classification) and 069's AC1–AC14; `spec.md`, `plan.md` and `tasks.md` were read whole, each concrete claim against the rule file. AC13 and AC14, ticked before task 4's 22 edits (`12b26fab`), were re-verified against the file at HEAD: by script against the WCAG 2.2 copy fetched 2026-11-08, all 55 Level A and AA criteria are cited, and all 69 titled criterion citations match the copy's number and title, the header's §4.1.1 and §5.2.2 aside as the header uses them, with each stated Level AAA matching; `FE-ARIA-001` cites §4.1.2 and carries no MUST against restating an implicit role. The window's external claims were re-checked by script against task 4's copies (`12b26fab` names each document and URL): the 38 passages behind task 1's §4.1.2 citation, task 2's disclaimer and seven-criterion text, and task 4's corrections are each in their copy, as is the *ARIA in HTML* §3.2 passage a corrected Resolved Question keeps. Claims the window left unchanged rest on task 4's check of every claim, which this run did not repeat. `check-rule-ids` resolves all 43 cited IDs (AC9), `no_shipped_statement_mixes_tiers` passes and `scripts/lint-rule-ids.sh` exits 0. 0 MUST, 0 SHOULD. Four observations, each fixed by the rule: `FE-SEMHTML-004`'s Verification counts its four commitments, `spec.md`'s ARIA scope row and the second review's Resolved Question follow tasks 1 and 4, and `plan.md`'s design record matches the tree; task 2's spent bullet in `tasks.md` still says 2.5.4's exceptions are "as MAY" and is left as written. The rule file is 50,939 bytes (`wc -c`), under the owner's 51,200-byte bar. Fix rounds: round 1 (the rule file; `spec.md` and `plan.md` exempt) re-read clean by an independent agent, then empty; budget-stop false. The loop ran in a detached worktree of `25a9936b`, because another session's uncommitted `runtime/` files sat in the main tree; they are outside this record and were not read. The primitives ran from a copy of `runtime/target/release/ductus` as built 2026-11-08T23:50:57Z, driven over stdio. The 52 excluded paths are 51 from 080's commits and `specs/079-review-and-analysis-work-converges/tasks.md` from 079's `8b3a9c01`.

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

## Observations

- bug: FE-SEMHTML-004's Verification closed with "flags a spec that breaks any of the three" over the four commitments it lists, a count left behind when task 4 split help's order from navigation's — `framework/rules/accessibility-frontend.md` — **fixed** (decided by the rule)
- convention: spec.md's Rule set scope row for ARIA said ARIA fills gaps in native semantics "rather than overriding them", while FE-ARIA-001 forbids overriding them only where ARIA in HTML forbids it, as task 4 corrected — `specs/069-accessibility-frontend-rules/spec.md` — **fixed** (decided by the rule)
- convention: spec.md's Resolved Question on the second review's loosenings said in the present tense that FE-ARIA-001 permits role="list" on a styled list, the MAY AC14's edit removed, and gave the Dialog pattern's next-step exception without its condition that users be very unlikely to need to re-invoke the dialog immediately — `specs/069-accessibility-frontend-rules/spec.md` — **fixed** (decided by the rule)
- convention: plan.md's design record said the spec needed no change for AC14 and FE-ARIA-001's other clauses were unchanged, which the stale Resolved Question and task 4's override and combobox corrections disprove, named only 008's task bullets as naming FE-ARIA-001 outside the spec where stored review decisions name it too, gave 2.5.4's exceptions as MAY where the rule states them as an unless clause, and said Enforcement still pointed at Open Questions — `specs/069-accessibility-frontend-rules/plan.md` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md` — 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/implement.md` — 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/review.md` — 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-analyze.md` — 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-implement.md` — 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-review.md` — 080-deterministic-runtime-efficiency
- `README.md` — 080-deterministic-runtime-efficiency
- `framework/bootstrap/ductus.md` — 080-deterministic-runtime-efficiency
- `framework/bootstrap/govern.md` — 080-deterministic-runtime-efficiency
- `framework/commands/analyze.md` — 080-deterministic-runtime-efficiency
- `framework/commands/implement.md` — 080-deterministic-runtime-efficiency
- `framework/commands/review.md` — 080-deterministic-runtime-efficiency
- `runtime/src/interpreter/mod.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/interpreter/payload.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/main.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/bounded_lists.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/check_artifacts.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/check_corpus_links.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/check_review_gate.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/check_unfolded_specs.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/dashboard.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/derive_boundary.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/lint_markdown.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/mark_task.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/measure_reopenings.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/prune_plan.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/prune_tasks.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/read_spec.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/read_tasks.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/rewrite_spec_links.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/schema/descriptions.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/schema/primitives.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/common/corpus.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/config_unknown_keys.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/crlf_preservation.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/golden/implement-basic.jsonl` — 080-deterministic-runtime-efficiency
- `runtime/tests/golden/plan-basic.jsonl` — 080-deterministic-runtime-efficiency
- `runtime/tests/golden/review-basic.jsonl` — 080-deterministic-runtime-efficiency
- `runtime/tests/mcp.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/mixed_corpus.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/result_sizes.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/specs_root_override.rs` — 080-deterministic-runtime-efficiency
- `scripts/audit/broken-relative-links.sh` — 080-deterministic-runtime-efficiency
- `scripts/audit/lib.sh` — 080-deterministic-runtime-efficiency
- `scripts/audit/manifest-destination-links.sh` — 080-deterministic-runtime-efficiency
- `scripts/audit/review-freshness.sh` — 080-deterministic-runtime-efficiency
- `scripts/measure-result-sizes.sh` — 080-deterministic-runtime-efficiency
- `specs/079-review-and-analysis-work-converges/tasks.md` — 079-review-and-analysis-work-converges
- `specs/080-deterministic-runtime-efficiency/data-model.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/plan.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/research.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/tasks.md` — 080-deterministic-runtime-efficiency

## Unexamined governance

*None.*
