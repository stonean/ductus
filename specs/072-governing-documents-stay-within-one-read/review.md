---
spec: 072-governing-documents-stay-within-one-read
last-run: 2026-10-26T21:28:16Z
reviewed-against: 1ebc2a200a77f171e40c22ba1ea843e7e70b8129
diff-base: e83ed85c7d8ad7ba67b32099fc5cf7b1743eeea3
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 27
scope: 28
skipped-passes: []
reviewed-digest: {}
blocking: false
dispositions:
  fixed: 15
  routed: 1
  discarded: 2
  undispositioned: 0
decisions:
  - key: "perf: 002's scenario imported shared constitutions at any size while withholding the shipped one, and /target would read each a second time — `specs/002-project-scaffolding/scenarios/claude-md-template-imports-load.md`"
    outcome: routed
    target: specs/002-project-scaffolding/scenarios/claude-md-template-imports-load.md
    decided-at: 2026-10-26T21:28:16Z
    decided-by: andy@stone.dev
  - key: "note: a ceiling over 19 digits passes the digit guard and makes `[ -gt ]` error — `scripts/lint-governing-size.sh`"
    outcome: discarded
    reason: "contrived: a ceiling that large admits any file, so the silent pass it allows cannot hide an oversized one"
    decided-at: 2026-10-26T21:28:16Z
    decided-by: andy@stone.dev
  - key: "note: the check-artifacts skipped-worklist entry's pointer is approximate; no constitution section states its lead — `AGENTS.md`"
    outcome: discarded
    reason: §spec-requirements is the nearest statement and the verbatim lead carries the rule; Family 38 matches by lead, not target
    decided-at: 2026-10-26T21:28:16Z
    decided-by: andy@stone.dev
---

# Review — 072-governing-documents-stay-within-one-read

## Summary

Five passes over the window e83ed85c..1ebc2a20 (the parent of the planned -> in-progress commit, so the whole implementation is inside it), run as three independent reviewers. The first ran security and efficiency against security-backend, reliability-backend, concurrency-backend, performance-backend, observability-backend and api-backend, read in full. The second ran quality, reuse and simplicity against quality-cross and configuration-cross, read in full. The third ran quality over every concrete claim in the new AGENTS.md. The three frontend rule files were judged inapplicable without a full read: the scope has no frontend surface. Round 1 found 0 MUST, 2 SHOULD and 15 observations. Both SHOULDs were fixed in 9912689f: a malformed ceiling passed the lint (QUAL-CLAIM-001), and the 50,000 ceiling was an unchecked copy of DEFAULT_READ_SIZE_BYTES (QUAL-GROUND-001). Fourteen observations were fixed in the same commit, and the fifteenth was routed by the owner to 002's scenario (1ebc2a20). An independent re-read of 9912689f re-ran every probe and returned 0 MUST and 0 SHOULD, plus two sentences that round made untrue, fixed in bb26182f. That last round is prose-only and was not re-read, per the stop rule. The examined count comes from the three reviewers' own lists. 23 in-scope files were read in full or by named ranges, and four were read by their window hunks: 002's spec.md and tasks.md, and 050's plan.md and tasks.md. specs/050-constitution/scenarios/the-constitution-fits-one-read.md entered scope after the passes ran, in 1ebc2a20, which records the owner's startup-load decision; it is not counted. The same commit's sentence in AGENTS.md's prompt entry and its edits to 002's scenario were not re-read by a pass.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention: CLAUDE.md restated the AGENTS.md no-auto-memory entry (AC8) — `CLAUDE.md` — **fixed**
- bug: the lib.sh comment said `bash -n` confirms a paired apostrophe; only shellcheck SC1011 does — `scripts/audit/lib.sh` — **fixed**
- bug: the parity mismatch message named framework/commands/{command}.md, which does not exist for the install case — `runtime/tests/parity.rs` — **fixed**
- other: the parity message forbade a corpus-wide re-bless that the BLESS comment reserves for protocol-level changes — `runtime/tests/parity.rs` — **fixed**
- bug: the prune comment omitted configure.md, which has no framework/commands/ source — `scripts/gen-claude-commands.sh` — **fixed**
- bug: the hook header listed its steps out of run order — `.githooks/pre-commit` — **fixed**
- other: the hook comment counted two git calls; the lint makes four — `.githooks/pre-commit` — **fixed**
- other: test case D did not check the constitution's fix text, so AC10's same message was proven for AGENTS.md only — `scripts/tests/test-lint-governing-size.sh` — **fixed**
- other: the plan listed six test cases; the test had seven — `specs/072-governing-documents-stay-within-one-read/plan.md` — **fixed**
- bug: an AGENTS.md lead claimed pre-0.49.0 review records lack reviewed-digest, which shipped in 0.46.0 — `AGENTS.md` — **fixed**
- bug: the AGENTS.md mutation entry read as if --release were a scripts/mutation/run.py flag — `AGENTS.md` — **fixed**
- other: the AGENTS.md check-artifacts skipped-worklist pointer named §implement-phase rather than §spec-requirements — `AGENTS.md` — **fixed**
- other: an AGENTS.md lead counted five Record destinations; §pipeline-boundaries states no count — `AGENTS.md` — **fixed**
- other: AGENTS.md said .mcp.json survives a git reset; only an untracked one does — `AGENTS.md` — **fixed**
- other: the plan's 'three failures, three messages' lead and the lint's --help exit line omitted the malformed-ceiling failure (re-read of 9912689f) — `scripts/lint-governing-size.sh` — **fixed**
- perf: 002's scenario imported shared constitutions at any size while withholding the shipped one, and /target would read each a second time — `specs/002-project-scaffolding/scenarios/claude-md-template-imports-load.md` — **routed** to `specs/002-project-scaffolding/scenarios/claude-md-template-imports-load.md`
- note: a ceiling over 19 digits passes the digit guard and makes `[ -gt ]` error — `scripts/lint-governing-size.sh` — **discarded**: contrived: a ceiling that large admits any file, so the silent pass it allows cannot hide an oversized one
- note: the check-artifacts skipped-worklist entry's pointer is approximate; no constitution section states its lead — `AGENTS.md` — **discarded**: §spec-requirements is the nearest statement and the verbatim lead carries the rule; Family 38 matches by lead, not target

## Skipped passes

*None.*

## Unexamined governance

*None.*
