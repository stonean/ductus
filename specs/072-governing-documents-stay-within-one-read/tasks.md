# 072 — Governing documents stay within one read Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Classify every `AGENTS.md` entry

- [ ] Split the entries into section chunks and fan them out to parallel agents with the plan's disposition rubric; each returns a disposition, replacement text, and for **moved** the enforcement site and whether its message names the fix
- [ ] Verify every **moved** and **deleted** proposal against the tree: open the check it names, confirm it reports the failure, and confirm a deleted entry's subject is gone or merged
- [ ] Fill the plan's §Entry dispositions table, one row per entry in file order

- **Done when**: every entry in `AGENTS.md` has exactly one row, and every **moved** or **deleted** row names its destination or reason

## 2. Make the enforcement-site edits

- [ ] For each **moved** entry whose enforcement message does not name the fix, add the fix to the message or as a comment at the site, outside `runtime/src/`
- [ ] Run each edited script's own test, or the suite that covers the edited test

- **Done when**: every **moved** entry's enforcement site names the fix, and the edited checks still pass

## 3. Rewrite `AGENTS.md` from the table

- [ ] Apply every disposition: pointers as the verbatim lead plus `§anchor`, trimmed entries without their stories, moved and deleted entries removed; keep the sweep-target marker list whole
- [ ] Remove or re-key `specs/050-constitution/plan.md` §Classification rows for every removed entry, so Family 38 reports no unmatched key
- [ ] Measure the file: at most 50,000 bytes, aiming for 40,000–45,000
- [ ] Run markdownlint, `check-corpus-links` and `bash scripts/audit/run-all.sh`, which includes the families that read `AGENTS.md`

- **Done when**: `AGENTS.md` is at most 50,000 bytes, AC2–AC4 hold entry by entry against the table, and lint, the link check and the audit pass

## 4. Add the ceiling check and its test

- [ ] `scripts/lint-governing-size.sh`: the ceiling table (`AGENTS.md` 50000, the constitution at its measured size), index measurement, and the three failure messages
- [ ] `scripts/tests/test-lint-governing-size.sh`: cases (a)–(f) from the plan
- [ ] `shellcheck` both scripts

- **Done when**: the test passes all six cases, with (b), (d) and (e) failing the lint as designed

## 5. Wire the check into the hook and CI

Blocked on task 3: wired before the cut, the check would fail every commit.

- [ ] `.githooks/pre-commit`: run the lint after `check-corpus-links`, and describe it in the header comment
- [ ] `.github/workflows/framework-checks.yml`: step `(o) Governing-document size ceilings` running the lint and its test
- [ ] Prove the hook path: a scratch commit that takes the staged `AGENTS.md` over the ceiling is refused with the message, then backed out

- **Done when**: a commit runs the check and refuses an over-ceiling `AGENTS.md`, and the CI step runs both scripts

## 6. Rewrite `CLAUDE.md` and prove the import loads

Blocked on task 3: the import loads the guide whole into every session.

- [ ] Negative control: before editing, run the plan's tool-less `claude -p` probe and confirm it cannot answer
- [ ] `CLAUDE.md`: `@AGENTS.md`; remove Non-negotiables and the constitution import; reduce Auto-Memory Routing to a pointer plus the no-auto-memory fact
- [ ] Positive run: the same probe answers correctly with no tool use in its stream

- **Done when**: AC6, AC7 and AC8 hold, and the probe's negative and positive runs are both recorded in the commit message

## 7. Disposition out-of-spec finding: the shipped `CLAUDE.md` template's imports load nothing

Found 2026-10-26 while planning. `framework/templates/project/claude-md.md` lines 3–4 write `@import .ductus/constitution.md` and `@import AGENTS.md`. The bootstrap writes `@import {document-path}` for shared constitutions (`framework/bootstrap/ductus.md` line 911, and `govern.md`). `check-orphaned-references` and two migrations read that form, and `002-project-scaffolding` AC3 requires it. Claude Code's syntax is `@path`, so an adopter's `CLAUDE.md` loads neither file. Converting the form would also start loading the 119 KB constitution into every adopter session, so the fix is a design decision for the owner.

- [ ] Present the finding and its options to the owner, then fix, route or discard it with the reason

- **Done when**: the finding is fixed, routed to a spec or scenario that a gate reads, or discarded with its reason recorded on this task

## 8. Verify the acceptance criteria against the tree

- [ ] Read each criterion AC1–AC11 against the tree and tick it only when its claim holds
- [ ] Re-measure `AGENTS.md` and the constitution against the check's ceilings

- **Done when**: every criterion is ticked on evidence, and the ceiling check passes on the committed tree
