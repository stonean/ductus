# 076 — Waived and deferred findings are distinct dispositions Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Define the terms in the constitution

- [x] Rewrite Finding dispositions' *Three dispositions* as Fixed / Deferred / Waived, with both definitions and the question meaning of *deferred*
- [x] Point §design-principles' dispositions sentence at it, listing a waiver under *decided out of scope*
- [x] Point §spec-requirements' deferred-question rule at the definition
- [x] Update §implement-phase's SHOULD line and the *only a gating one holds `done`* bullet

- **Done when**: AC1, AC2 and AC9 hold on `framework/constitution.md`, and `npx markdownlint-cli2 framework/constitution.md` is clean.

## 2. Rename the outcomes in the runtime, reading the old names from disk

- [x] Rename `DispositionOutcome`, `DecisionOutcome` and the `Dispositions` fields, with serde read aliases on the map
- [x] Accept `routed` and `discarded` in `RawDecision::to_ref` and `covering_key`
- [x] Emit only the new names from `render_dispositions` and `disposition_suffix`
- [x] Add tests: a record holding `routed`/`discarded` parses and counts, a re-render writes `deferred`/`waived`, and a map holding a key and its alias is refused

- **Done when**: AC14 is covered by tests that fail with the aliases removed, and `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` pass under `runtime/`.

## 3. Give a deferral its companions and refusals

- [x] Add `task` and `work-item` to `Disposition`, `DecisionRef` and `RawDecision`
- [x] Generalize `require_companion` to the data model's refusals, keeping `work-item` byte-identical
- [x] Check a `target`, and a `task` in it, against the tree at write time in both writers
- [x] Add a test for each refusal and for a byte-identical round trip of a work-item ID

- **Done when**: AC3, AC4, AC10 and AC11 are covered by tests that fail with the behavior removed, and the task 2 cargo commands pass.

## 4. Classify lapsed deferrals in `process-decisions`

- [x] Add the `lapsed` result and its notices
- [x] Decide completion per the data model's table, reading tasks through `read-tasks`' parser and status from frontmatter
- [x] Treat an unreadable target as matched, with a notice
- [x] Test each completion row, the external-only case, an absent target, and an unreadable one

- **Done when**: AC17 is covered by tests that fail with the behavior removed, and the task 2 cargo commands pass.

## 5. Defer review violations, and release gating findings in both writers

- [x] Add `disposition` and `decision-key` to `ReviewFinding`; store a violation's deferral keyed `rule {rule-id} at {file}`
- [x] Exclude a released violation from the MUST/SHOULD counts and render it under Deferred findings; keep a non-releasing one counted, marked `gating: true`
- [x] Refuse a finding that is both deferred and matched by an applied waiver
- [x] Record `analysis.md`'s `hard-fail`/`blocking-findings` net of released findings, render them under Deferred to other specs, and mark non-releasing gating deferrals `gating: true`
- [x] Test release, non-release (same spec, rule file, work item only), the refusal and the net counts

- **Done when**: AC5 and AC13 are covered by tests that fail with the behavior removed, and the task 2 cargo commands pass.

## 6. Name holding deferrals at the gate

- [x] Make `check-review-gate`'s outstanding-findings block name each `gating: true` deferral with its work reference, in both records
- [x] Name deferral as the fix-later exit in its guidance
- [x] Test the released, same-spec-task and work-item-only cases

- **Done when**: AC16 is covered by tests that fail with the behavior removed, and the task 2 cargo commands pass.

## 7. Carry the change to the runtime's surfaces

The CHANGELOG entry waits until `git status` shows no one else's uncommitted edits in `runtime/CHANGELOG.md`, so neither commit sweeps in the other's lines. Add it under the unreleased section, naming the `dispositions:` key rename and the read aliases (`runtime/src/schema/primitives.rs`, `Dispositions`), the review `--defer` flags, and the new `lapsed`, `task`, `work-item` and `gating` fields (data model).

- [x] Update the tool descriptions in `runtime/src/mcp/server.rs` and the wire-shape tests in `runtime/tests/mcp.rs`
- [x] Regenerate the goldens with `BLESS=1`, reading every diff
- [ ] Add the `runtime/CHANGELOG.md` entry, naming the `dispositions:` rename for adopters' own CI

- **Done when**: `cargo test --release` passes, the release binary is rebuilt, and the bless diff holds no unintended change.

## 8. Update the commands and references

- [x] Add `--defer`, `--target`, `--task` and `--work-item` to `framework/commands/review.md`, with the three exits in its steps and guidance
- [x] Update `framework/commands/analyze.md` and `implement.md`, and the analyze, implement and groom references, to fixed / deferred / waived, keeping the inbox `discard` route
- [x] Rewrite `framework/reference/review.md` §Waivers without `ticket`, with deferral as the fix-later path
- [x] Run `scripts/gen-help-tables.sh` and `scripts/gen-claude-commands.sh`

- **Done when**: AC6, AC12 and AC15 hold on the command sources, the generated mirrors match them, and `scripts/audit/run-all.sh` passes.

## 9. Sweep the live artifacts by meaning

The spec-body reach of this sweep is an owner decision not yet taken. Uses of *routed* and *discarded* sit in 90 files across 36 specs (10 `done`, 3 `draft`, 23 `in-progress`; counted excluding 076 and every `review.md` and `analysis.md`). Classify each use first:

- a record — a checked task, a Motivation, a Resolved Question, a rejected alternative, an incident account — which keeps its wording;
- not a finding disposition — an inbox item's `discard` route, `routed-by`, generic English — which keeps it;
- a current-behavior claim, which is edited.

Count the `done` specs a claim edit would reopen, and the `in-progress` specs whose `scenarios/` or `data-model.md` would change, then put the scope to the owner with those counts. Task 10 already covers 020's, 058's and 066's data models. The rest of the set needs no decision: `framework/` beyond the files task 8 edited, `scripts/`, the runtime doc comments, `.github/`, `docs/`, `README.md` and `AGENTS.md`. Edit `README.md` and `docs/rules.md` only when `git status` shows no one else's uncommitted edits in them.

- [ ] Grep the `AGENTS.md` sweep-target set for *routed*, *discarded*, *discard*, *waive* and *defer*, and classify each use by who emits it
- [ ] Rewrite each finding-disposition use, leaving Resolved Questions, signposts, migrations, the inbox route and the alias-reading code
- [ ] Re-read the doc comments of every runtime item touched in tasks 2–6

- **Done when**: AC7 and AC18 hold, and `npx markdownlint-cli2` passes on every file touched.

## 10. Discharge the cross-spec impacts

Edit 020 only when `git status` shows no one else's uncommitted work in `specs/020-code-review/`, or after asking the owner.

- [ ] Add a blockquote signpost linking back to 076, with the contract it owes, to 020, 058, 066 and 073
- [ ] Apply the key rename to the data models of 020, 058 and 066, pointing at 076's data model for the new fields
- [ ] Reopen 058 with `set-status` from `done` to `in-progress`

- **Done when**: AC8 holds — `check-review-gate` against 076 reports every impact discharged — and 058 is `in-progress` with its signpost committed.
