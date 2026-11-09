# 070 — Performance rules (frontend) Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. The header's posture and five rules to SHOULD (AC11)

- [x] The header states the posture after its design-time sentence, in the sibling form, as one paragraph
- [x] `FE-BUNDLE-002`, `FE-IMAGE-001`, `FE-IMAGE-002`, `FE-LOAD-001` and `FE-FONT-002`: each Statement in one tier, MUST to SHOULD and MUST NOT to SHOULD NOT
- [x] The same five Verifications: "SHOULD commit" and "emits a warning when", each **Not flagged** clause and yield kept
- [x] spec.md: *Severity posture* states 22 rules, 6 MUST and 16 SHOULD

- **Done when**: the header states the posture, the five are SHOULD-tier under their IDs, `cargo test --lib no_shipped_statement_mixes_tiers`, `scripts/lint-rule-ids.sh`, markdownlint and `scripts/audit/run-all.sh` pass, and AC11 is ticked.

## 2. `FE-VITALS-001` publishes its budgets in `specs/system.md` (AC12)

- [x] The Statement names `specs/system.md` as the budgets' home
- [x] The Verification flags a frontend project with no Web Vitals budgets published there
- [x] spec.md: the `VITALS` row names the home

- **Done when**: `FE-VITALS-001` reads as AC12 says, `cargo test --lib no_shipped_statement_mixes_tiers`, `scripts/lint-rule-ids.sh`, markdownlint and `scripts/audit/run-all.sh` pass, and AC12 is ticked.

## 3. Re-fetch every Source (AC3's claims, AC4)

The set is every document a Source line, or a claim in any field, cites after tasks 1 and 2. Fetch it into this session's scratchpad, never committed, by the plan's method. A page that renders client-side is read from its documentation source repository's raw file. A claim no copy confirms is rewritten or removed.

- [x] Name and fetch the set
- [x] Check each rule's claims and the header's against the copies, through read-only agents, confirming every quote behind an edit before the edit
- [x] Check `FE-VITALS-001`'s three metrics, thresholds and measurement basis against web.dev (AC4)
- [x] The landing commit names each document, its URL, the fetch date and the method

- **Done when**: every claim matches a fetched copy or was rewritten or removed, `cargo test --lib no_shipped_statement_mixes_tiers`, `scripts/lint-rule-ids.sh`, markdownlint and `scripts/audit/run-all.sh` pass, and AC3's claims half and AC4 hold.

## 4. Verify the remaining criteria (AC1, AC2, AC3's schema, AC5–AC10)

Blocked on task 3, which can rewrite rule text. Once every box in this file is ticked, `/ductus:review` runs, in a clean worktree when another session's uncommitted files sit in the tree, then `/ductus:analyze`, then the completion gate with the owner's approval.

- [x] AC1 and AC2: the six categories and their sections, every rule ID's category one of them, none shared with `security-frontend.md` or `accessibility-frontend.md`, and `scripts/lint-rule-ids.sh`
- [x] AC3: each rule's heading, one-tier Statement, Verification and Source, no Rationale, and the corpus test; AC3 ticked with task 3's half
- [x] AC5 and AC6: no MUST Statement or Verification requires CI, and `FE-BUNDLE-001` sets no budget figure
- [x] AC7 and AC8: the citations AC7 names, `check-rule-ids` against every file in `framework/rules/`, `BE-PAGE` a section of `api-backend.md`, and `BE-CACHE-004`'s deferral
- [x] AC9 and AC10: the manifest row and `discover-rule-files` under `frontend`; the constitution names no `performance-frontend.md`
- [x] AC4 ticked from task 3's check

- **Done when**: each criterion holds against the tree, with a failure fixed in this task, and AC1–AC10 are ticked.
