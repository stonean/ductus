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
- [x] Add the `runtime/CHANGELOG.md` entry, naming the `dispositions:` rename for adopters' own CI

- **Done when**: `cargo test --release` passes, the release binary is rebuilt, and the bless diff holds no unintended change.

## 8. Update the commands and references

- [x] Add `--defer`, `--target`, `--task` and `--work-item` to `framework/commands/review.md`, with the three exits in its steps and guidance
- [x] Update `framework/commands/analyze.md` and `implement.md`, and the analyze, implement and groom references, to fixed / deferred / waived, keeping the inbox `discard` route
- [x] Rewrite `framework/reference/review.md` §Waivers without `ticket`, with deferral as the fix-later path
- [x] Run `scripts/gen-help-tables.sh` and `scripts/gen-claude-commands.sh`

- **Done when**: AC6, AC12 and AC15 hold on the command sources, the generated mirrors match them, and `scripts/audit/run-all.sh` passes.

## 9. Sweep the live artifacts by meaning

The owner chose the full sweep (2026-11-03): every current-behavior claim in every spec, `done` bodies included, staged by path in specs another session is working on. Uses of *routed* and *discarded* sit in 90 files across 35 specs (10 `done`, 3 `draft`, 22 `in-progress`; `routed|discarded` over `specs/**/*.md`, excluding 076 and every `review.md` and `analysis.md`, at `1405eacc`). Classify each use first:

- a record — a checked task, a Motivation, a Resolved Question, a rejected alternative, an incident account — which keeps its wording;
- not a finding disposition — an inbox item's `discard` route, `routed-by`, generic English — which keeps it;
- a current-behavior claim, which is edited.

Swap a label word for word wherever the claim still holds, so `done` specs 047 and 057 stay `done` under the mechanical-rename rule (`runtime/src/primitives/mechanical_sweep.rs`); reword only where the contract changed — a "fixed or waived" list that now omits deferral (020, 050, 058, 066). Task 10 already covers 020's, 058's and 066's data models.

- [x] Grep the `AGENTS.md` sweep-target set for *routed*, *discarded*, *discard*, *waive* and *defer*, and classify each use by who emits it
- [x] Rewrite each finding-disposition use, leaving Resolved Questions, signposts, migrations, the inbox route and the alias-reading code
- [x] Re-read the doc comments of every runtime item touched in tasks 2–6

- **Done when**: AC7 and AC18 hold, and `npx markdownlint-cli2` passes on every file touched.

## 10. Discharge the cross-spec impacts

Edit 020 only when `git status` shows no one else's uncommitted work in `specs/020-code-review/`, or after asking the owner.

- [x] Add a blockquote signpost linking back to 076, with the contract it owes, to 020, 058, 066 and 073
- [x] Apply the key rename to the data models of 020, 058 and 066, pointing at 076's data model for the new fields
- [x] Reopen 058 with `set-status` from `done` to `in-progress`

- **Done when**: AC8 holds — `check-review-gate` against 076 reports every impact discharged — and 058 is `in-progress` with its signpost committed.

## 11. Name deferral in the gate's MUST and analyze guidance

Found while re-reading `check_review_gate.rs` in task 9: task 6 named deferral in the outstanding-findings guidance only, so a spec blocked on MUST violations or on analyze findings is still told to fix or waive.

- [x] Name deferral to another spec's work in the `must-violations` and `analyze-findings` guidance, with a test of each that fails without it
- [x] Update the quoted guidance in `framework/reference/implement.md`, `framework/reference/review.md` and 066's `data-model.md`; 020's *Blocking message* quotes are the record of what 020 specified and keep their text

- **Done when**: both guidance strings name `--defer` or deferral to another spec's work, the tests fail with the wording reverted, and the task 2 cargo commands pass.

## 12. Fix the defects the doc-comment re-read found

Found 2026-11-03 by task 9's independent re-read of tasks 2–6's runtime doc comments, which read the code beside them.

- [x] A re-deferral after a lapse is stored and its work checked against the tree: `merge` counts a decision whose key the run drops through `expired-decisions` as fresh, and `refuse_missing_work` takes that list
- [x] `check-review-gate` names each holding deferral at the MUST and analyze-findings blocks too, which a MUST or gating analyze finding reaches before the last check (AC16)
- [x] `write-review` names `findings`, not `observations`, when two violations sharing a key conflict

- **Done when**: each has a test that fails with its behavior removed, and the task 2 cargo commands pass.

## 13. Fix what 076's review found

Found 2026-11-03 by the five-pass review of `a76c99e8..53a87a87`, four slices.

Handoff, 2026-11-03. 076's review is decided, and its record is written once task 14 lands. The passes read 35 of the 130 scope entries `compute-review-scope` resolved from base `a76c99e8` at `53a87a87`: 076's own changes in nineteen runtime files, six command and five reference sources, the constitution, the CHANGELOG entry, and 076's `spec.md`, `plan.md` and `tasks.md`, in four read-only slices (`decisions.rs` with `process_decisions.rs`; the two writers; `check_review_gate.rs` with 076's small hunks; the command, reference and CHANGELOG prose). The window's 077 commits and other sessions' edits were not examined. Rule findings: QUAL-CLAIM-001 twice, QUAL-GROUND-001, QUAL-TEST-001 and BE-INPUT-004 are fixed; BE-COMPAT-001 (must, low confidence — the 0.x rename, deliberate per `plan.md` Trade-offs and the CHANGELOG) stays a low-confidence finding. Five fix rounds followed, each re-read independently, as the items below record. The owner decided on 2026-11-03: the notices' raw stored text is fixed (the fifth round); `process-waivers`' raw waiver text, outside 076's diff, is deferred to 066 task 13 (scenario `process-waivers-notices-flatten-stored-text`); and sixteen observations are waived, each with its reason as re-verified against the tree: a dedup's dropped duplicate losing a deferral only it carried (`write_review.rs`; `--defer` names a rule, and duplicates share rule and file); repeated parses of the stored list and a named `tasks.md` per finding (`decisions.rs`; bounded by one run's findings); `Buckets` copying `Sorted` and the writers' alike `stored_decisions` loops (`write_review.rs`; no behavior change); `releases` comparing a raw feature string such as `001-x/` (`decisions.rs`; callers pass the canonical name); an unchecked target kind such as `README.md` (`decisions.rs`; the data model names no refusal, and it releases nothing and lapses the next run); a task with no checkboxes counting complete (`decisions.rs`; the data model's row, and 0 of 957 tasks across 70 `tasks.md` files have that shape); `Work::External` sharing `Pending`'s arm (`process_decisions.rs`; documents intent); no gate-level test of a pre-076 map (`check_review_gate.rs`; `Dispositions`' alias test covers the parse); the gate naming a gating deferral that may have lapsed (`check_review_gate.rs`; the gate reports the record, and the next run judges lapse); a work-item ID accepting vertical tab and form feed (`decisions.rs`; escaped in the record, flattened in every report, and no tracker issues one); a hand-edited task in a nested `tasks.md` judged by spec status (`decisions.rs`; writers refuse that target); a `..` target lapsing as complete (`decisions.rs`; it names no feature directory, nothing is opened, and writers refuse a traversal); the lists of deferrals that release nothing omitting a rule file and complete work (`framework/reference/implement.md` and others; true as written); 076's plan leaving the match rule and `expired-decisions` refusal to its data model (`plan.md`; its Data Model section points there); the analyze reference wording §implement-phase's deferral exit with the full condition (`framework/reference/analyze.md`; true, and descriptive); Affected Files omitting the sweep's files (`plan.md`; the review scope holds them as modified since the base). Every other observation is recorded `fixed`. Then `/ductus:analyze` 076 and propose `done`; 058 task 4 and 057 task 26 follow.

- [x] A deferral naming other work than the key's standing deferral is a new decision, its work checked against the tree (`merge`, `refuse_missing_work`); a waiver over a standing deferral is refused unless the deferral arrives in `expired-decisions`
- [x] A deferral releases a gating finding only to work still to do on another spec (`releases` reads `work_state`), and `work_state` screens a stored target's feature directory before opening it
- [x] `covering_key` reads the outcome through `to_ref`; the gate's held-by-deferral suffix reads only deferrals and says when it cannot read one; a work-item ID refuses every line break and is flattened in the report; the generated Summary counts released violations
- [x] Command, reference, data-model and CHANGELOG prose: the gate's guidance and suffix, release and reopen for `--defer`, the drop lists, a lapsed violation, a waiver naming work, the analyze field and drift texts, implement's external-only deferral of a blocking finding, groom and constitution wording
- [x] Doc and dead-code items: `DecisionRef.reason`, the write-review MCP description, `RawWaiverFull`, `decision_for`'s filter, `ProcessDecisionsResult.lapsed`'s default, the review-drift message in `check_artifacts.rs`
- [x] An independent re-read of the second round — the six defects the first re-read found: an observation's work item rendered raw, the release condition missing "still to do" across docs and prose, the held-by-deferral enumeration, 058's match rule, the `write-review` `# Errors` refusal, and the Purpose typo — confirmed all six and re-ran the probes, and found one MUST and three SHOULDs, which the third round below fixes
- [x] The third round: `work_state` screens the `spec.md` and `tasks.md` it reads, not only their directory (`BE-INPUT-004`); every sentence saying a deferral to another spec's work releases, leaves the counts or does not hold `done` says "work still to do" (076's AC16 and Behavior included, and 058's, 066's, 020's and 050's signposts, criteria and scenarios); `/{project}:review`'s Scope Boundaries names `--defer`'s reopen; the notice and gate flattening has tests; and 076's data model, plan and the review reference state the refusals, Affected Files and held-by-deferral cases the code has
- [x] An independent re-read of the third round, driving its build, confirmed the file screen and the sweep and found no MUST and three SHOULDs, which the fourth round below fixes
- [x] The fourth round: 058's data model says a deferral keeps an analyze finding blocking unless its target is work still to do on another spec; the generated review Summary labels its released count "deferred to work still to do on another spec"; 076's Refusals table gives each `task` refusal its own reason; and `/{project}:review`'s Scope Boundaries reopens only a `done` spec, once confirmed
- [x] An independent re-read of the fourth round finds no MUST or SHOULD
- [x] The fifth round, from the owner's decision on the proposed waivers: every stored key, target and reason a `process-decisions` notice prints, and the key in `check-review-gate`'s held-by-deferral suffix, is flattened, with a test that fails at each site
- [x] An independent re-read of the fifth round found no MUST, and two SHOULDs: this handoff paragraph, stale and now rewritten (prose, not re-read), and `process-waivers`' raw waiver text, outside 076's diff, which the owner deferred to 066 task 13

- **Done when**: each code fix has a test that fails with it reverted, the task 2 cargo commands pass, and an independent re-read of the round finds no MUST or SHOULD.

## 14. State the release condition once, and point every other statement at it

- [x] Make `framework/reference/review.md` §Deferrals, which adopters receive, the one full statement of when a deferral releases a gating finding and when its named work is complete, and point 076's data model's *Release and lapse* at it
- [x] Replace each other statement of the condition — in `framework/`, `runtime/src` doc comments and tool descriptions, `docs/`, `README.md`, and spec bodies, scenarios and data models — with a pointer to §Deferrals, keeping a clause only where it states a necessary condition or an instruction
- [x] Take the sweep's hit list from a grep broad enough to match every phrasing (`another spec`, `releases`, `leaves the counts`, `holds done`), record each site's classification in the commit, and give the re-read that list as its scope

- **Done when**: a search of the live artifacts for a deferral to another spec's work that releases, leaves the counts or does not hold `done` finds only §Deferrals, pointers to it, necessary-condition clauses and instructions; `npx markdownlint-cli2` and the task 2 cargo commands pass.
