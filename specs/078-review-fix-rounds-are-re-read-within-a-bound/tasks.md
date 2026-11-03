# 078 — Review fix rounds are re-read within a bound Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. The round directory and `start-fix-rounds`

- [x] Create `runtime/src/primitives/fix_rounds.rs`: the `.ductus/review-rounds/{feature}/` path, its self-ignoring `.gitignore`, and removal of a feature's earlier rounds
- [x] Create `runtime/src/primitives/start_fix_rounds.rs`: the snapshot as `data-model.md` defines it, from an in-memory index, with a fixed `ductus` identity and no ref update
- [x] Register it at the five sites and in `framework/runtime-tools.txt`; run `scripts/gen-configure-mcp.sh`
- [x] Tests: an untracked file is in the snapshot and an ignored one is not, a deletion is applied, the index file's bytes, `HEAD` and every ref are unchanged, an unborn `HEAD` gives a parentless snapshot, and earlier rounds are removed; each fails with its behavior removed

- **Done when**: `start-fix-rounds` returns a snapshot whose tree is the working tree, and the tests prove it changed no index, ref or `HEAD` (AC10's first half).

## 2. The `[review] re-reads-off` switch

- [x] Add the shared reader to `fix_rounds.rs`, validating the reason as `validate_disabled_entry` does and halting with the setting, the value and the config file
- [x] Register `re-reads-off` in `runtime/src/schema/config_keys.rs` `SECTIONS` and document it in `framework/bootstrap/ductus.md` §Project Configuration
- [x] Tests: absent, a valid reason, a short reason, a non-string, each config-chain file named in the error

- **Done when**: the reader returns the reason or halts on a malformed value naming the file, and `the_installer_documents_the_registry` passes with the key documented.

## 3. `brief-re-read`

Blocked on tasks 1 and 2: it writes into the round directory and reads the switch.

- [x] Derive the round's `paths` as a tree-to-tree diff from the base to a snapshot taken now, bounded by the project-diff helpers extracted from `compute_review_scope::diff_since`
- [x] Classify `empty`, `exempt` and `re-read` with `is_analyze_subject` and `is_review_contract`
- [x] On `re-read` with the loop on, take the snapshot and write `round.toml` and `brief.md`, the brief's text a constant in `brief_re_read.rs` holding the spec's severity bar and the result-file contract
- [x] Register it at the five sites
- [x] Tests: each `kind`; `analysis.md` and a non-markdown file in the feature directory are not exempt; the brief carries exactly the derived paths and no caller text; the switch writes no brief

- **Done when**: a brief is written only for a round that needs a re-read, holds only derived paths and fixed text, and its snapshot is the next round's base (AC2, AC3, AC10).

## 4. `read-re-read`

Blocked on task 1.

- [x] Parse `result.json` in `fix_rounds.rs`; classify `complete`, `incomplete` and `unparseable`, and count findings by severity and those without edits
- [x] Register it at the five sites
- [x] Tests: no file, `complete` missing or false, malformed JSON, an empty `old`, and a complete file with each severity

- **Done when**: only a complete, parseable file reads as a finished re-read, and a cut-off one reads as incomplete (AC7, AC15).

## 5. `check-prescriptions`

Blocked on tasks 3 and 4: it reads the previous round's `round.toml` and `result.json`.

- [x] Apply each edit exactly once to the snapshot's copy of its file, compare with the working tree, and list `failed`, `differing` and `unprescribed`
- [x] Return `could-not-run` with its `problem` when either file is missing, incomplete or unparseable, and `open` from the previous re-read
- [x] Register it at the five sites
- [x] Tests: matched; an edit whose `old` occurs zero times and twice; an extra unprescribed file; a prescribed file edited further; a formatter's change; each `could-not-run` cause

- **Done when**: a round that applies exactly the prescribed edits matches, and any other change, or an edit that cannot apply, does not (AC5, AC11, AC15).

## 6. One loop, end to end

The four primitives are public modules of the `ductus` library (`ductus::primitives::{start_fix_rounds, brief_re_read, read_re_read, check_prescriptions}`, each a `run(&args, repo)`), but `fix_rounds` and `git_fixture` are crate-private, so the test builds its own repository and resolves round paths from `brief-re-read`'s `brief` and `result` fields. `check_prescriptions.rs`' `briefed` test helper is the shape to follow.

- [ ] `runtime/tests/fix_rounds.rs`: on a fixture repository, start, change a file, brief, write a result with an edit, apply it, check it, and brief again after an unprescribed change; the scope of each brief is only its round's paths

- **Done when**: the test drives the four primitives through two rounds and each brief's paths are its round's alone (AC3, AC10).

## 7. The loop in `/{project}:review`

Blocked on 066 task 15, whose `fix-rounds` argument the loop's `write-review` call passes; tasks 1–5 have landed.

The run's first base is taken when the passes end, before step 10 fixes anything: step 10 already re-runs the affected passes after a chore fix, and `--fix` calls `write-review` a second time, so the loop sits after both and before the final `write-review`. Each round: `brief-re-read` with the round's base; on `exempt` or `empty` no re-read; on `re-read`, hand the agent the `brief` path alone (a host without subagents prints it for the operator), then `read-re-read` with the same base. After a re-read with findings whose fixes apply only its edits, `check-prescriptions` with that re-read's base: `matched` with `open` 0 ends the loop and its `snapshot` is the next base; otherwise brief the round again from the re-read's `snapshot`. Count toward the budget only re-reads with a MUST or SHOULD. `framework/commands/review.md`'s Writes line says no other file is modified, which the round directory and the snapshots' git objects now contradict. `BLESS=1` re-blesses the golden; read its diff for a templated version (AGENTS.md Gotchas).

- [ ] `framework/commands/review.md`: call `start-fix-rounds` when the passes end, and add the loop step between step 10 and `write-review`, its per-round primitives unbackticked behind `<!-- audit:ignore-promotion -->`; update the Writes line
- [ ] `framework/reference/review.md`: the loop's walk — the end condition, the budget and the operator's decision, an incomplete or failed check counting as not re-read, the host without subagents running the brief in a fresh session, a decline's reason, out-of-round NOTEs and open findings at a stop as observations — pointing at the primitives for the brief's text
- [ ] Run `scripts/gen-claude-commands.sh`, re-bless `runtime/tests/golden/review-basic.jsonl` and read its diff
- [ ] Walk `framework/bootstrap/ductus.md` §Derived values' layout-derived rows and record each layout's impact in the commit

- **Done when**: the command runs the loop as the spec's Behavior states, the parity suite passes, and the generated copies match their source (AC1, AC4, AC6, AC8).

## 8. The record and the gate, verified

Blocked on 066 task 15.

- [ ] Confirm against that task's code and tests that the record carries the loop's fields from `fix-rounds` and that the gate blocks an un-re-read round without a waiver, and that `re-reads-off` waives each such round with its reason

- **Done when**: AC12, AC13 and AC14 hold against the tree.

## 9. Point at the framework, and sweep

Blocked on task 7: AGENTS.md's rule points at the statement that task writes.

The rule is the Workflow bullet beginning "Have an independent agent re-read each fix round"; AGENTS.md is held to 50,000 bytes by `scripts/lint-governing-size.sh`, so the pointer replaces the bullet rather than adding to it.

- [ ] Replace AGENTS.md's re-read rule with a pointer to `framework/reference/review.md`'s loop section
- [ ] Sweep `framework/`, `docs/`, `README.md` and the specs for prose describing a review with no fix-round loop
- [ ] Add the unreleased `runtime/CHANGELOG.md` entry

- **Done when**: AGENTS.md restates no part of the loop (AC9), and no live artifact describes a review without one.
