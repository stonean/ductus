# 069 — Accessibility rules (frontend) Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. `FE-ARIA-001` rests on §4.1.2 (AC14)

- [x] Drop the Statement's MUST NOT on restating an element's implicit role and the styled-list MAY that qualifies it
- [x] Drop the Verification's restated-role flag and its styled-list exception
- [x] Source: add WCAG 2.2 §4.1.2 (Name, Role, Value) for the MUST, and drop *ARIA in HTML* §3.2

- **Done when**: `FE-ARIA-001` reads as AC14 says, `cargo test no_shipped_statement_mixes_tiers`, `scripts/lint-rule-ids.sh` and `scripts/audit/run-all.sh` pass, and AC14 is ticked.

## 2. The disclaimer and the seven criteria (AC13, first part)

- [x] The header's first paragraph ends with the conformance disclaimer, citing WCAG 2.2 §5.2.2
- [x] `FE-CONTRAST-002` gains §1.3.3: instructions do not rely solely on shape, color, size, visual location, orientation or sound
- [x] `FE-A11YMEDIA-001` gains §1.4.5: text rather than images of text, with its Customizable and Essential exceptions as MAY
- [x] `FE-POINTER-002` gains §2.5.4: functionality operated by device or user motion is also operable by UI components, and responding to motion can be disabled, with its two exceptions as MAY
- [x] `FE-SEMHTML-004`: across a set of web pages, §2.4.5 Multiple Ways, §3.2.3 Consistent Navigation, §3.2.4 Consistent Identification and §3.2.6 Consistent Help, MUST, with a Verification and Source
- [x] spec.md: the scope table's `SEMHTML`, `CONTRAST`, `A11YMEDIA` and `POINTER` rows, and Severity posture's counts (40 rules, 36 MUST, 4 SHOULD)
- [x] Measure the file; if it is over 51,200 bytes, the documented single-read limit, tighten wording, dropping no claim, and state the size in the commit

- **Done when**: each criterion is cited by the rule the plan names, the header disclaims conformance, `cargo test no_shipped_statement_mixes_tiers`, `scripts/lint-rule-ids.sh` and `scripts/audit/run-all.sh` pass, and the file's size is in the commit.

## 3. Every Level A and AA criterion is cited (AC13, second part)

- [x] List the 55 Level A and AA success criteria of the fetched WCAG 2.2 copy, 31 and 24
- [x] Check each by number against the rules' `§` citations, recording a design-time reason for any not cited

- **Done when**: each of the 55 is cited by a rule or recorded with its reason in the commit message, and AC13 is ticked.

## 4. Re-fetch every Source (AC4, AC5)

The set is every document a Source line, or the header, cites after tasks 1–3, the regime sources included. Reuse the clarify session's WCAG 2.2 and *ARIA in HTML* copies. Fetch the rest into this session's scratchpad, never committed, by 068's task 6 method (`a8f419f7`). A claim no copy confirms is rewritten or removed.

- [x] Fetch the set
- [x] Check each rule's claims and the header's against the copies, through read-only agents, confirming every quote behind an edit before the edit
- [x] Check each cited criterion's number, title and level against the WCAG copy (AC5)
- [x] The landing commit names each document, its URL, the fetch date and the method

- **Done when**: every claim matches a fetched copy or was rewritten or removed, `cargo test no_shipped_statement_mixes_tiers`, `scripts/lint-rule-ids.sh` and `scripts/audit/run-all.sh` pass, and AC4 and AC5 are ticked.

## 5. Verify the remaining criteria (AC1–AC3, AC6–AC12)

Blocked on task 4, which can rewrite rule text. Once every box in this file is ticked, `/ductus:review` runs, in a clean worktree when another session's uncommitted files sit in the tree, then `/ductus:analyze`, then the completion gate with the owner's approval.

- [ ] AC1: the eleven categories, a `## FE-{CATEGORY}` section each, and every rule ID's category one of them
- [ ] AC2: no category shared with `security-frontend.md` or `performance-frontend.md`
- [ ] AC3: each rule's heading, one-tier Statement and Verification, no Rationale, no repeated ID, and `scripts/lint-rule-ids.sh`
- [ ] AC6: each AAA rule SHOULD and labelled, each SHOULD rule AAA or testing, each MUST rule citing an A or AA criterion but those Severity posture names
- [ ] AC7: no MUST requires CI; `FE-A11YTEST-001` and `FE-A11YTEST-003` as stated
- [ ] AC8: `FE-KBD-002`'s MUST is §2.4.7 plus §1.4.11's 3:1, and §2.4.13 only in `FE-KBD-005`
- [ ] AC9: `check-rule-ids` over the file, `FE-PII-002`, `FE-AUTHN-002` and `BE-AUTHN-005` included
- [ ] AC10 and AC11: the constitution names no `accessibility-frontend.md`; the manifest row, and `discover-rule-files` under `frontend`
- [ ] AC12: no `data-model.md`, and the header governs the ID format
- [ ] spec.md: Enforcement's "(see Open Questions)" points at the Resolved Question that defers the review pass to 020's scenario

- **Done when**: each criterion holds against the tree, with a failure fixed in this task, and AC1–AC3 and AC6–AC12 are ticked.
