---
spec: 050-constitution
scenario: findings-route-by-scope
reviewed-at: 2026-08-25T18:36:17Z
reviewed-against: 09d954565dea9f7787f04d0097b4017f54b99a8e
diff-base: 670308bf522d3ef43174d429ff0d9832afe9bb81
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 050-constitution

## Summary

Final review of this back-edge. Five passes over the scope — `framework/constitution.md`, `AGENTS.md`, `framework/commands/implement.md` and its generated copy, `framework/templates/project/inbox.md`, `specs/inbox.md`, `runtime/tests/golden/implement-basic.jsonl`, and this spec's own artifacts — against the eleven selected rule files: 0 MUST, 0 SHOULD, 0 low-confidence, `blocking: false`. The scope is governance prose and one golden fixture, and the loaded rule files verify code patterns, so no rule ID anchors here. The counts describe what the rules could examine; the substantive findings of this back-edge came from reading the artifacts against each other, and all three are resolved below.

**The prose-claim sweep (first review, task 11).** `implement.md` step 5 sent every issue outside the current task's scope to `specs/inbox.md`, which was wrong for the entire middle tier the new rule defines — and that step, not the constitution, is what an agent reads at the moment it decides. It now routes by scope explicitly and names `/{project}:amend` for a requirement gap. The shipped inbox template and this repo's inbox carry the same test in their header blocks, which are the whole specification of the inbox for anyone who never opens the constitution.

**A wrong reason in the record.** `e7eda2a`'s message justified leaving `append-task` unbackticked by claiming the exec parser dispatches on the code span rather than on the section. False — `runtime/src/parser/mod.rs:7` scopes the parser to `## Instructions`, which is why the backticked `append-inbox` beside it was always safe. The claim was inferred from a passing lint instead of read from the parser, the shortcut §grounding names. Corrected in `6474430`; recorded here because a commit message cannot be rewritten.

**AC15 was ticked before it held.** The criterion asserts the `AGENTS.md` mirror points without restating; the How-to-apply enumerated all three tiers, a second copy of the constitution's list and the thing §Promotion mechanism forbids. Caught by verifying the criterion against the tree at the completion gate rather than trusting its checkbox — the rule this spec itself promoted in task 4, doing the work it was promoted to do. Fixed in `09d9545`; AC3's grep now resolves "Scope decides the destination" to one statement and one pointer.

Both `implement-basic.jsonl` re-blessings were filtered to that one golden and word-diffed before acceptance: each diff is the two git-derived sha fields and nothing else, and `{{runtime-version}}` survives both. Full gate green on the committed tree — 1013 unit tests, 11 parity goldens, `npx markdownlint-cli2`, `scripts/audit/run-all.sh` (Families 1 and 6 included), the six `lint-*.sh` scripts, `scripts/tests/*.sh`. All 11 tasks and all 15 acceptance criteria are checked, with AC15 verified against the tree rather than assumed.

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
