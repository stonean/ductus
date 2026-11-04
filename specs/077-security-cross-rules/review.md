---
spec: 077-security-cross-rules
last-run: 2026-11-04T03:41:31Z
reviewed-against: 65ec2d7f70c6ac37f7ea71b5486d20108575c435
diff-base: 898a49ecbde8e13c9bff79aff165fc0e8116a39c
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 35
scope: 195
skipped-passes: []
fix-rounds: 5
re-reads: 4
diff-checked: 0
not-re-read: 0
budget-stop: true
reviewed-digest:
  data-model.md: cbf317f3d8635097191d82c1b4a9cc35aedd9275dd026eaeb68a9685f241be0d
blocking: false
dispositions:
  fixed: 16
  deferred: 1
  waived: 2
  undispositioned: 0
decisions:
  - key: "other: once 0.62.0 removes the four deprecated rules, check-rule-ids will report a Blocking unknown-rule finding for each record that must keep the old ID (077's spec, 008's and 017's signposts) — `specs/077-security-cross-rules/spec.md`"
    outcome: deferred
    target: specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md
    decided-at: 2026-11-03T19:11:22Z
    decided-by: andy@stone.dev
  - key: rule QUAL-GROUND-001 at scripts/audit/deprecation-removal.sh
    outcome: deferred
    target: specs/067-analyze-and-audit-checks/tasks.md
    task: "21"
    decided-at: 2026-11-03T19:11:22Z
    decided-by: andy@stone.dev
  - key: "convention: step 2's parenthetical in `.ductus/reference/analyze.md` §Rules (blocking and advisory) can read as describing the checks rather than naming the heading — `framework/bootstrap/ductus-procedure.md:198`"
    outcome: waived
    reason: "Style only: the checks applied are the same either way, and Family 39 needs the full heading (owner, 2026-11-03, option a)."
    decided-at: 2026-11-04T03:41:31Z
    decided-by: andy@stone.dev
  - key: "other: main's bootstrap calls `resolve-constitutions --unmeasured`, which the pinned 0.59.11 runtime lacks, so a `--ref=main` run errors at that step and falls back to the unflagged call until a release moves the pin — `framework/bootstrap/ductus.md:924`"
    outcome: waived
    reason: "An observation about the pipeline's own machinery made while verifying framework work (AGENTS.md): it concerns 055's step, not 077's, blocks nothing here, and ends when the next release tag moves the runtime pin to one carrying the flag."
    decided-at: 2026-11-04T03:41:31Z
    decided-by: andy@stone.dev
---

# Review — 077-security-cross-rules

## Summary

0 MUST and 0 SHOULD violations counted, 0 low-confidence findings. QUAL-GROUND-001 on Family 40's restated label parse still fires and stays deferred to 067 task 21 under the stored decision, so it leaves the counts. A low-confidence QUAL-GROUND-001 at `framework/bootstrap/ductus-procedure.md` §Loading rule files step 2, which restated the security files' ID formats, was fixed: step 2 now checks each file's IDs against the format `/{project}:analyze` reads for it (owner, 2026-11-03, option a). The five passes ran on 2026-11-03 over base `898a49ec`, the parent of 077's `in-progress` commit, measured and not overridden, in three read-only agent slices of what changed since the first review's `6e2d64bf` through `f6a26443`: task 11's procedure change with 077's deliverable and artifacts; Family 40's code in `d869bb9f`; and `d869bb9f`'s document and spec fixes. `examined: 35` counts the in-scope files the passes read, whole or by the window's hunks, of the 195 the scope resolves at write time. Not examined (160): 94 changed only in the first review's window and not since, which that review's record covered; 35 changed by 078's commits (`6fabd051` to `e380be8d`), which 078's own review reads; 29 changed otherwise only by 066's commits (`a3beb4c0`, and `58d4311b` to `65ec2d7f`) or `82bb5d2e`, which 066's review reads, as it reads the changelog entry for 066 task 13 that `8359fbf0` carried; 050's scenario `findings-route-by-scope`, changed only by this run's fix rounds and read by their re-reads; and this record. 19 observations: 16 fixed, 1 deferred, 2 waived, by the owner's decisions of 2026-11-03: the 12 reviewer-prescribed fixes in one approval (option a); fix round 1's finding that Family 40's test header V claimed an untested case (option a); the first review's deferral of the brownfield audit's yielded cases, fixed by §Per-rule check item 4, so its stored decision expires; the fourth round's re-read NOTEs, two fixed in task 11 and one waived (option a); and records citing the four deprecated IDs after 0.62.0, still deferred to 067's scenario `a-record-cites-a-removed-rule`; and the bootstrap's `--unmeasured` finding, waived under AGENTS.md's rule for an observation about the pipeline's own machinery. NOTEs below the bar from the passes and re-reads 1 to 3, contrived or cosmetic, are recorded in no list, as `8359fbf0` records. Five fix rounds. Re-reads 1 to 3 were hand-rolled briefs, since this run began before 078's loop landed and took no `start-fix-rounds` base; each finished, finding SHOULDs, SHOULDs, then one MUST, and the owner let the loop go on after each budget stop. Round 4 was re-read through the loop's primitives in a scratch clone at `8359fbf0`, its base that commit with the round's two edits reverted (owner, option b): one SHOULD, fixed, and three NOTEs. Round 5 changed only task 11, so it is exempt. The real adopter bootstraps of the step-2 change, two brownfield first adoptions run with `--ref=main` whose bootstrap and four rule files hash to `8359fbf0`'s, met every expected result, neither printing a `failed to load` line or writing a file: the backend-only one loaded `security-backend.md`, `security-cross.md` and `configuration-cross.md` and printed the `f6a26443` run's three lines (`BE-DATA-012`, `BE-LOG-005`, `CFG-ENV-006`); the frontend-only one loaded `security-frontend.md`, `security-cross.md` and `configuration-cross.md` and printed `CFG-ENV-006` for its configuration dump and `FE-STORAGE-001` and `SEC-LOG-001` for its session-restore spec, beside `FE-AUTHN-002`, whose authenticated-UI trigger that spec also fires. They also found main's bootstrap calling `resolve-constitutions --unmeasured`, which the pinned 0.59.11 runtime lacks, waived above.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Deferred findings

### DEFERRED: QUAL-GROUND-001 — Family 40 restates the runtime's label parse instead of binding to it

- **File**: `scripts/audit/deprecation-removal.sh:136-173`
- **Finding**: The family's correctness depends on check-rule-ids' section_is_deprecated, heading_id_regex and parse_atx_heading, and its embedded python restates them; no test runs the runtime's parse against the family's. The owner's 2026-11-02 decision keeps the parse here until a primitive replaces it.
- **Auto-fixable**: no
- **Suggested fix**: 067 task 21: a primitive returning each label, sharing check-rule-ids' reader, with this script as its entry point.
- **Deferred**: to `specs/067-analyze-and-audit-checks/tasks.md` task 21

## Observations

- other: 077's §Edge cases said a copy pinned from main after 0.59.11 keeps all four moved rules before ad8a36f9, though two arrived only in 1598b87a — `specs/077-security-cross-rules/spec.md:64` — **fixed**
- other: 077 task 11's handoff ordered its re-read after the bootstrap, still listed the finished re-read, and left out the bootstrap fixture's split across spec.md and plan.md and that the 059b830e run does not count — `specs/077-security-cross-rules/tasks.md:106` — **fixed**
- other: 077's plan called CFG-ENV-006 the only rule outside the backend surface naming BE-LOG-002 or BE-LOG-008, though SEC-LOG-001 names both — `specs/077-security-cross-rules/plan.md:65` — **fixed**
- convention: 077's plan Affected Files row for ductus-procedure.md restated §Distribution's decision clause for clause — `specs/077-security-cross-rules/plan.md:120` — **fixed**
- other: Family 40's test header item I says each outside label is counted, but the after-category case asserted no count — `scripts/tests/test-deprecation-removal.sh:287` — **fixed**
- other: Family 40's test header item M says a malformed version fails with a coverage line, but the long and prefixed cases asserted none — `scripts/tests/test-deprecation-removal.sh:329` — **fixed**
- other: audit.md item 40 and the audit README said the label form is stated in 026's scenario, though its home is 008's data-model.md Deprecated field — `framework/reference/audit.md:64` — **fixed**
- other: docs/rules.md's one case needing a decision no longer held once a same-named file of your own also needs one — `docs/rules.md:70` — **fixed**
- other: the changelog offered pinning an adopter-authored security-cross.md as equal to renaming it, though pinning keeps the shipped file out and leaves every case another file yields to a SEC- rule under no rule — `runtime/CHANGELOG.md:98` — **fixed**
- other: 067's scenario listed 077 spec.md's sections citing the four IDs without §Rule set scope and §Edge cases — `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md:11` — **fixed**
- convention: 076 sweep residue: 047's spec and AC1 and 020's data-model said routed or discarded in present-tense claims — `specs/047-analyze-findings-durability/spec.md:27` — **fixed**
- other: 020's data-model should-violations and low-confidence rows did not match how write_review.rs sort_findings files them — `specs/020-code-review/data-model.md:113` — **fixed**
- other: Family 40's test header V claimed a due label in one file and a malformed one in a later file, which no case tested — `scripts/tests/test-deprecation-removal.sh:64` — **fixed**
- bug: the brownfield security audit left a case yielded to a rule in a file it did not read unpreviewed; §Per-rule check item 4 now previews it under the owner's ID — `framework/bootstrap/ductus-procedure.md:210` — **fixed**
- other: 077 task 11's last item named a budget stop with no antecedent — `specs/077-security-cross-rules/tasks.md:118` — **fixed**
- other: 077 task 11's last item pointed at a session scratchpad file outside git — `specs/077-security-cross-rules/tasks.md:118` — **fixed**
- convention: step 2's parenthetical in `.ductus/reference/analyze.md` §Rules (blocking and advisory) can read as describing the checks rather than naming the heading — `framework/bootstrap/ductus-procedure.md:198` — **waived**: Style only: the checks applied are the same either way, and Family 39 needs the full heading (owner, 2026-11-03, option a).
- other: main's bootstrap calls `resolve-constitutions --unmeasured`, which the pinned 0.59.11 runtime lacks, so a `--ref=main` run errors at that step and falls back to the unflagged call until a release moves the pin — `framework/bootstrap/ductus.md:924` — **waived**: An observation about the pipeline's own machinery made while verifying framework work (AGENTS.md): it concerns 055's step, not 077's, blocks nothing here, and ends when the next release tag moves the runtime pin to one carrying the flag.
- other: once 0.62.0 removes the four deprecated rules, check-rule-ids will report a Blocking unknown-rule finding for each record that must keep the old ID (077's spec, 008's and 017's signposts) — `specs/077-security-cross-rules/spec.md` — **deferred** to `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`

## Skipped passes

*None.*

## Unexamined governance

*None.*
