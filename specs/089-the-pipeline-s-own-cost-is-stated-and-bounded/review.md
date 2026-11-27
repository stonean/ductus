---
spec: 089-the-pipeline-s-own-cost-is-stated-and-bounded
last-run: 2026-11-27T15:38:56Z
reviewed-against: e13a76e455a87fbce8c96157a73692b013f8e6f4
diff-base: 9379381e1549c7fe68dab2171d0b9c6db8579a5c
must-violations: 0
should-violations: 0
low-confidence: 1
examined: 17
scope: 23
unread:
  - .claude/commands/ductus/*.md
  - .ductus/reference/*.md
  - .ductus/reference/analyze.md
  - .ductus/reference/review.md
  - .pi/prompts/*.md
  - runtime/CHANGELOG.md
excluded: 65
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest: {}
criteria-judged: 4
criteria-met: 4
criteria-verdicts:
  - label: AC1
    digest: eee5084793d2a8400df994d90a4a7030e23c3e455b9455cc92681548f565d83e
  - label: AC2
    digest: 0175a51f2e008233d601d5f3a3a53c31832487047b966af9d660d34d5ffc7f3c
  - label: AC4
    digest: c632d970097b1f7691dcb7813659a126d5944ccf916048e4d34d19c53dd29c07
  - label: AC3
    digest: c659491fef668a261f6f48fae834368fd7acf571ec8d159ec74a984fba35f59f
blocking: false
dispositions:
  fixed: 9
  deferred: 0
  waived: 5
  undispositioned: 0
decisions:
  - key: "other: check-artifacts' candidate grammar does not name an absolute path or NNN among its rejections — `runtime/src/schema/descriptions.rs`"
    outcome: waived
    reason: "'a repo-relative path' already excludes an absolute one and 'a placeholder' covers NNN, so naming them would change no claim"
    decided-at: 2026-11-27T15:38:56Z
    decided-by: rule
  - key: "other: scripts/lint-governing-size.sh measures by two paths and writes its over-ceiling message three times — `scripts/lint-governing-size.sh`"
    outcome: waived
    reason: "no defect: cases A to M cover both paths and the hook states its eight git calls; folding them is a refactor of tested code, not a chore"
    decided-at: 2026-11-27T15:38:56Z
    decided-by: rule
  - key: "other: the constitution's as-short-as bullet says nothing enforces it beyond check-artifact-size and the hook's project-file refusal, though scripts/lint-governing-size.sh holds byte ceilings in this repository — `framework/constitution.md`"
    outcome: waived
    reason: the constitution states what enforces the rule in every project running the pipeline; scripts/lint-governing-size.sh is this repository's own lint, which no Shared Files entry ships, so the claim holds for its population
    decided-at: 2026-11-27T15:38:56Z
    decided-by: rule
  - key: "convention: task 1 moved four code-convention Gotchas (the set -e trap, check-artifacts' skipped array, the ductus-vX.Y.Z literal, the CLI boolean flag) into developing-ductus.md, which /ductus:review does not load as normative — `AGENTS.md`"
    outcome: waived
    reason: the owner approved task 1's disposition table (4a4ab858), 089's spec and plan set no route for code conventions, and the four take 1,375 bytes where AGENTS.md has 257 under its 39,000 ceiling, so returning them is the owner's call, not a chore
    decided-at: 2026-11-27T15:38:56Z
    decided-by: rule
  - key: "other: 050's AC23 says the lint holds the constitution at most 50,000 bytes while the lint now holds it to 49,000 — `specs/050-constitution/spec.md`"
    outcome: waived
    reason: "AC23 still holds as written: a 49,000 ceiling keeps the constitution at most 50,000 bytes, so no claim is false"
    decided-at: 2026-11-27T15:38:56Z
    decided-by: rule
---

# Review — 089-the-pipeline-s-own-cost-is-stated-and-bounded

## Summary

Window `9379381e` to `e13a76e4` (base-source `reopen`, 089's first review), run in this checkout with a clean tree at `e13a76e4`. 23 scope entries: the 21 the scope resolved at the start, then 075's and 079's `spec.md` once the run's own fixes joined them. 17 examined, holding 20 distinct files, each read whole by one of three passes parts (code; governing documents; specs), which ran all five passes over the five loaded rule files (231,962 bytes, read whole by each part) beside slices of about 241, 229 and 289 KB, and a criteria part that judged AC1 to AC4 and drove `scripts/tests/test-lint-governing-size.sh` in a clone (the agent's recommendation, adopted by the operator, 2026-11-27). Unread, for the next review: the generated copies `.claude/commands/ductus/*.md` and `.pi/prompts/*.md`, which no 089 commit changed, and `.ductus/reference/*.md` with its `analyze.md` and `review.md`, generated from the two references read whole, `scripts/gen-claude-commands.sh --check` reporting every copy in sync; and `runtime/CHANGELOG.md`, read at its `## [Unreleased]` section alone (29,914 of 665,224 bytes), the rest released history. Seven rule files are judged by analysis alone and were not loaded. Nine observations were chores fixed in the run and re-judged by their parts over the fix diff; on the fixed tree `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --test mcp` (55 passed) and the lint test pass.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

0 of 17 commits in the window no spec claimed.

4 of 4 acceptance criteria judged met.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: QUAL-TEST-001 — the glob rows' 50,000-byte figure is tested only against the lint's constant and the runtime's DEFAULT_READ_SIZE_BYTES

- **File**: `scripts/tests/test-lint-governing-size.sh:169-183`
- **Site**: case H
- **Rule**: A test SHOULD fail when the behavior it names is removed or broken. A test that passes whether or not the code under test behaves as named SHOULD NOT be added or counted as coverage.
- **Finding**: Case H holds both glob rows equal to the runtime's DEFAULT_READ_SIZE_BYTES, so moving the rows and the runtime constant together still passes, while AC2 states the read size as 50,000 bytes. Low confidence: AC2 names the read size, which is what H tracks.
- **Auto-fixable**: no

## Waived findings

*None.*

## Deferred findings

*None.*

## Acceptance criteria

- AC1 — met: framework/constitution.md ### Design principles carries the cost bullet as AC1 states it: a spec adding a check, an audit-record field or a primitive states its run's cost in a measurable unit and what it retires, or why none.
- AC2 — met: scripts/lint-governing-size.sh CEILINGS: AGENTS.md 39000 (cut to 38,673 at 4a4ab858), constitution 49000 (48,999 at 24ca810b), framework/commands/*.md and framework/reference/*.md 50000, each failing with file, size, ceiling and fix; scripts/tests/test-lint-governing-size.sh cases B, D, J and K (one byte over fails) and C (every row exactly at passes) pass in a clone.
- AC4 — met: 085, 086, 087 and 088 each carry a measured ## Cost section saying what it retires, beside a signpost to 089; Cost commits bde7391a, 3ce771fe, 52ea4076 and 08510de3 are ancestors of done commits ccbaefab, 4b7b05b8, 4aeab814 and 5594bd7b, and 086 is in-progress.
- AC3 — met: At 54f697d7 AGENTS.md is 38,743 of 39,000, the constitution 48,999 of 49,000 and all 30 command and reference files at most 49,748 of 50,000 (git ls-tree -l); CI's framework-checks run on that sha passed, its step (h) running scripts/audit/run-all.sh.

## Observations

- bug: check-artifact-size's description said no value switches the check off, though configured_threshold sets no maximum, so a large enough value reports nothing — `runtime/src/schema/descriptions.rs` — **fixed** (decided by the rule)
- bug: test case H's DEFAULT_READ_SIZE_BYTES-not-found branch was unreachable under set -euo pipefail, so a renamed constant aborted the test with no FAIL line and cases I to M unrun — `scripts/tests/test-lint-governing-size.sh` — **fixed** (decided by the rule)
- other: check-artifacts' description left whitespace, `$`, `|` and non-ASCII spans out of the candidate grammar is_path_like applies — `runtime/src/schema/descriptions.rs` — **fixed** (decided by the rule)
- other: check-artifacts' candidate grammar does not name an absolute path or NNN among its rejections — `runtime/src/schema/descriptions.rs` — **waived**: 'a repo-relative path' already excludes an absolute one and 'a placeholder' covers NNN, so naming them would change no claim (decided by the rule)
- other: scripts/lint-governing-size.sh measures by two paths and writes its over-ceiling message three times — `scripts/lint-governing-size.sh` — **waived**: no defect: cases A to M cover both paths and the hook states its eight git calls; folding them is a refactor of tested code, not a chore (decided by the rule)
- other: the constitution's as-short-as bullet says nothing enforces it beyond check-artifact-size and the hook's project-file refusal, though scripts/lint-governing-size.sh holds byte ceilings in this repository — `framework/constitution.md` — **waived**: the constitution states what enforces the rule in every project running the pipeline; scripts/lint-governing-size.sh is this repository's own lint, which no Shared Files entry ships, so the claim holds for its population (decided by the rule)
- convention: developing-ductus.md's Landing a runtime change step 3 said 'do these three things first' over five bullets — `.ductus/docs/agents/developing-ductus.md` — **fixed** (decided by the rule)
- convention: task 1 moved four code-convention Gotchas (the set -e trap, check-artifacts' skipped array, the ductus-vX.Y.Z literal, the CLI boolean flag) into developing-ductus.md, which /ductus:review does not load as normative — `AGENTS.md` — **waived**: the owner approved task 1's disposition table (4a4ab858), 089's spec and plan set no route for code conventions, and the four take 1,375 bytes where AGENTS.md has 257 under its 39,000 ceiling, so returning them is the owner's call, not a chore (decided by the rule)
- other: 089's plan kept point-in-time claims the cuts settled: estimated end sizes, AC4 awaiting 086, a dated at-planning list and two known limitations — `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/plan.md` — **fixed** (decided by the rule)
- other: 089's plan accounted for one 079 signpost where this review added a second, and did not name 075's limit-question correction — `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/plan.md` — **fixed** (decided by the rule)
- other: 089's ceilings Resolved Question stated the 2026-11-25 estimates the cuts superseded — `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/spec.md` — **fixed** (decided by the rule)
- other: 050's AC23 says the lint holds the constitution at most 50,000 bytes while the lint now holds it to 49,000 — `specs/050-constitution/spec.md` — **waived**: AC23 still holds as written: a 49,000 ceiling keeps the constitution at most 50,000 bytes, so no claim is false (decided by the rule)
- other: 075's question on each file's limit said 50,000 is the figure 072 holds this repository's AGENTS.md to, which 089 lowered to 39,000 — `specs/075-agents-md-holds-what-every-session-needs/spec.md` — **fixed** (decided by the rule)
- other: 079's question 'Can a reference stop stating a primitive's rules?' read 'dropped, not moved into descriptions' with no signpost to 089, which moved them — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/configure.md`
- `.claude/commands/ductus/help.md`
- `.claude/commands/ductus/prune.md`
- `.ductus/reference/prune.md`
- `.pi/prompts/ductus-help.md`
- `.pi/prompts/ductus-prune.md`
- `README.md`
- `docs/slash-commands.md`
- `framework/bootstrap/configure/claude.md`
- `framework/commands/help.md`
- `framework/commands/prune.md`
- `framework/reference/prune.md`
- `framework/runtime-tools.txt`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/main.rs`
- `runtime/src/mcp/server.rs`
- `runtime/src/primitives/compute_review_scope.rs`
- `runtime/src/primitives/mod.rs`
- `runtime/src/primitives/process_waivers.rs`
- `runtime/src/primitives/prune_review.rs`
- `runtime/src/primitives/spec_ownership.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/src/schema/registry.rs`
- `runtime/tests/crlf_preservation.rs`
- `runtime/tests/result_sizes.rs`
- `runtime/tests/walker.rs`
- `scripts/analyze/analyze_lib.py`
- `scripts/analyze/briefs.py`
- `scripts/analyze/check.py`
- `scripts/analyze/combine.py`
- `scripts/analyze/record.py`
- `scripts/analyze/scenario-brief.md`
- `scripts/analyze/split.py`
- `scripts/mutation/prune-review-086.py`
- `scripts/mutation/review-drops-unowned-086.py`
- `scripts/mutation/scope-attribution.py`
- `scripts/tests/test-analyze-scripts.py`
- `specs/020-code-review/spec.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md`
- `specs/022-deterministic-runtime/scenarios/parser-walker-conventions.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/plan.md`
- `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md`
- `specs/041-task-pruning/spec.md`
- `specs/050-constitution/plan.md`
- `specs/050-constitution/scenarios/a-decision-is-attributed-to-whoever-made-it.md`
- `specs/050-constitution/scenarios/a-primitive-is-probed-in-a-local-clone.md`
- `specs/050-constitution/scenarios/findings-route-by-scope.md`
- `specs/050-constitution/spec.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/scenarios/rule-judging-runs-from-repo-scripts.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/tasks.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/072-governing-documents-stay-within-one-read/spec.md`
- `specs/075-agents-md-holds-what-every-session-needs/plan.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/plan.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/spec.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/plan.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/spec.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/tasks.md`

## Unexamined governance

*None.*
