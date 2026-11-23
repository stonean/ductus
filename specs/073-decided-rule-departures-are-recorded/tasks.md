---
next-task: 21
---

# 073 — Decided Rule Departures Are Recorded Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 16. Read `covers:` in the entry reader

- [x] `runtime/src/primitives/departures.rs`: the `covers:` child and its items, two code spans joined by `in` found by `inline_code_spans`; `Entry.covers`; the `CoversNotAList`, `CoversEmpty`, `CoversItem`, `CoversRepeated` and `CoversPath` problems; `UnknownKey`'s message naming the three keys
- [x] Fixtures in `departures.rs` for a well-formed scoped entry, a site holding a backtick written with a longer delimiter, and each problem; fixtures in `runtime/src/primitives/check_departures.rs` for each AC13 form as a Blocking finding
- [x] `scripts/mutation/departures.py`: a mutant per `covers:` problem, each killed

- **Done when**: a scoped entry is in `entries` with each item's file, site and line; each AC13 form is a Blocking `check-departures` finding and keeps its entry out of `entries`; every new mutant is killed; `cargo test` passes.

## 17. Waive a scoped entry's rule at its sites only

Blocked on task 16: the derivation reads `Entry.covers`.

- [x] `runtime/src/primitives/write_review.rs`: `departure_waiver_entries` keeps only the findings at a covered `(file, site)` for a scoped entry, applies site anchors, stores `anchors:` with no `covers` (gated by `holds_anchor`), and passes `superseded_deferrals` the covered findings alone
- [x] `runtime/src/primitives/process_waivers.rs`: `Kind::DepartureSite` in `read_anchors` and `place`; `departure_gone` expires an anchor the entry no longer lists, a scoped-to-unscoped switch and back included; the module doc comment
- [x] Fixtures in `runtime/src/primitives/write_review/departure_waivers.rs`: AC10 both directions, a finding naming no site included; AC11 both directions; AC13's `malformed-departures` half; a dropped `covers:` item and each form switch expiring with a notice
- [x] `scripts/mutation/departure-waivers.py`: a mutant per scoped behavior, each killed

- **Done when**: the new fixtures pass, `git diff` shows the fixtures proving AC5 and AC8 unedited and passing (AC9), and every new mutant is killed.

## 18. Report lost coverage in `departure-notices`

Blocked on task 17: the notices read the covered findings it selects.

- [x] `runtime/src/primitives/write_review.rs`: per scoped entry, a notice for each covered `file` not in the tree and for each in-scope finding of its rule in a covered file at a site it does not list or naming none; `departure-notices` and its count in `WriteReviewResult` (`runtime/src/schema/primitives.rs`), bounded in `bound_lists`
- [x] Fixtures for AC12's three cases; `runtime/tests/mcp.rs`'s file form naming `departure-notices`; mutants in `scripts/mutation/departure-waivers.py` and `scripts/mutation/review-departures-bounded.py` for the notices and their bound, each killed
- [x] `framework/commands/review.md` step 11 prints `departure-notices`; run `scripts/gen-claude-commands.sh`; re-bless `review-basic` with its diff read

- **Done when**: AC12's three fixtures and the MCP file-form test pass, every new mutant is killed, and the re-blessed golden's diff is the review-basic fixture's `reviewed-against` sha alone, which moves with the command source.

## 19. Prove analyze waives a scoped entry's rule

- [x] `runtime/src/primitives/write_analysis.rs`: a fixture over a spec whose entry carries `covers:`, recording its rule `waived` with the entry's reason, attribution and citation (AC15)
- [x] A mutant in `scripts/mutation/departures.py` leaving a scoped entry out of `departures::parse`'s `entries`, killed by that fixture

- **Done when**: the fixture passes, and fails under the mutant.

## 20. State `covers:` where the grammar and the waiver are described

Blocked on tasks 16 to 19: the data model's **Size** is measured on the binary they build, and the descriptions state what they do. Once every task is ticked, 073 owes `/ductus:review` and `/ductus:analyze`, and 020, reopened here, owes its own, over the window this change shares with 073.

- [ ] `specs/073-decided-rule-departures-are-recorded/data-model.md`: the `covers:` row and its malformed forms, a scoped waiver's stored `anchors:`, and `departure-notices` in the departure lists' **Size**, its file form re-measured on the built binary
- [ ] The Applicable Rules comment of `framework/templates/spec/spec.md`; `check-departures` and `write-review` in `runtime/src/schema/descriptions.rs`
- [ ] `framework/reference/review.md` §Waivers, `docs/rules.md`, and `specs/066-audit-records-and-the-completion-gate/plan.md`'s account of `write-review`'s bounded file
- [ ] `specs/020-code-review/scenarios/waiver-expiry.md`: rule 5's exception and the paragraph on a finding naming no site; reopen 020 `done → in-progress` with `set-status`
- [ ] `runtime/CHANGELOG.md` `[Unreleased]`; the prose-claim sweep, `scripts/stale-literals.sh` included

- **Done when**: every AC14 location states `covers:` or a scoped entry's waiver, the sweep finds no live claim that every entry covers every in-scope file, 020 is `in-progress`, and the pre-commit hook passes.
