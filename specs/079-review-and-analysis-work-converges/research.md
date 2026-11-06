# 079 — Review and analysis work converges Research

Two read-only sweeps at `2abc19aa` that size the plan's correction and cut tasks. Both are floors or estimates, as each section says; re-measure before relying on a figure.

## Claims this work makes false (AC12)

Method: regexes per behavior over `framework/`, `docs/`, `README.md`, `runtime/src` doc comments and MCP text, `scripts/`, and each spec's `spec.md`, `plan.md`, `data-model.md` and scenarios, with the hits read in context. A unit is one line, paragraph or comment block holding a false statement. Resolved Questions, past-tense accounts, `path:line` citations and bare names are excluded. Tests, fixtures and mutation lists are not counted, though many assert the old behavior. A stale claim sharing no search term is missed, so spec counts are a floor.

Total: 203 claims, 197 distinct lines (six count under both window and scope), 111 outside specs and 92 in 20 specs.

| Behavior | Commands | References | Constitution | Docs, README | Runtime text | Specs: `spec.md`+`plan.md` / scenarios+data models |
| --- | --- | --- | --- | --- | --- | --- |
| Window (AC5) | 3 | 2 | 0 | 0 | 5 | 1 / 9 |
| Scope (AC3, AC6) | 2 | 2 | 0 | 0 | 5 | 2 / 13 |
| Fix-round paths (AC3, AC13) | 0 | 2 | 0 | 0 | 5 | 5 / 3 |
| `diff-cross-spec` (AC7) | 2 | 1 | 0 | 0 | 7 | 1 / 2 |
| Dispositions (AC2, AC17) | 9 | 11 | 2 | 7 | 5 | 19 / 5 |
| Reopen (AC4, AC15) | 3 | 3 | 4 | 1 | 5 | 10 / 7 |
| Gate check 12 (AC14) | 0 | 2 | 0 | 0 | 3 | 1 / 3 |
| Canonical surface (AC9) | 0 | 1 | 0 | 0 | 19 | 5 / 6 |

| Spec | Status | `spec.md`+`plan.md` | Scenarios+data model |
| --- | --- | --- | --- |
| 000 | in-progress | 1 | 0 |
| 011 | done | 1 | 0 |
| 016 | in-progress | 2 | 0 |
| 017 | in-progress | 2 | 0 |
| 020 | in-progress | 0 | 3 |
| 022 | in-progress | 1 | 8 |
| 023 | done | 0 | 1 |
| 041 | in-progress | 1 | 1 |
| 045 | in-progress | 0 | 1 |
| 046 | done | 2 | 0 |
| 047 | in-progress | 2 | 0 |
| 050 | in-progress | 1 | 6 |
| 054 | in-progress | 2 | 0 |
| 058 | in-progress | 16 | 2 |
| 059 | done | 1 | 0 |
| 063 | in-progress | 3 | 1 |
| 066 | in-progress | 2 | 19 |
| 075 | in-progress | 1 | 0 |
| 076 | done | 0 | 1 |
| 078 | in-progress | 6 | 5 |

Of the `done` specs, correcting 023's scenario (`scenarios/living-specs.md`) and 076's data model (`data-model.md`) reopens them and owes a review; correcting 059's ticked AC3 changes a criterion, so it reopens and owes an analysis; 011 and 046 stay `done` under AC15.

Separately, 112 lines in 30 specs name the markdown-only path, by a text search for `two paths`, `walk the same prose`, `neither wraps`, `markdown-only path`, `runtime is unavailable` and `when no runtime`; 17 are `done`, 7 with the lines in a scenario or data model (010, 013, 019, 023, 030, 032, 040). That is a ceiling on what AC21 falsifies, since most are procedural lines to read, and the search was not read in context.

Most-read targets: `framework/commands/review.md:127`, `framework/reference/review.md:472` and `:589`, `framework/commands/implement.md:85`, `framework/constitution.md:160`, `:162`, `:295` and `:296`, `runtime/src/mcp/server.rs:638`, `specs/066-audit-records-and-the-completion-gate/data-model.md:42` and `scenarios/review-scope-union.md:37`, and `specs/058-findings-route-at-discovery/spec.md:480`. Commit line numbers rot; grep the quoted claim.

## Task 10's searches

Run at `8bc0b631` by `git grep -nE` over `framework/`, `docs/`, `README.md`, `scripts/`, `runtime/src`, `runtime/tests/*.rs`, `runtime/CHANGELOG.md`, `.github/` and `specs/[0-9]*`, leaving out `specs/079-*` and each spec's `tasks.md`, `review.md` and `analysis.md`: 1,294 lines. Re-running them shows what task 10 read and left; its commits say why. The `noserver` search is case-sensitive, so run it with `-i` too.

```text
window: last reopen|parent of .*in-progress|in-progress. commit|last `?in-progress|since (its|the spec.s|the feature.s) (last )?(re)?open|window (starts|opens|begins)|diff[- ]base
scope: review-scope|modified-since|union of|the union|scope union|the scope is|scope holds|resolved scope|Affected Files.{0,40}(union|scope)|(union|scope).{0,60}Affected Files
fixround: round_paths|mixed round|whole working tree|working tree.{0,40}round|round.{0,40}working tree|exempt round|round.s paths|round is exempt
crossspec: diff-cross-spec|cross-spec-count|first spec-dir|first-commit|first spec dir|first commit (that )?touch
dispositions: each write|every write|write is confirmed|is confirmed first|gate-confirm|Scope decides|scope decides|every disposition|each disposition|every finding.{0,60}(confirm|operator|ask)|confirm(s|ed|ation)?.{0,60}(waiv|defer|disposition)|(waiv|defer|disposition).{0,60}confirm
reopen: factual correction|meaningful (body )?edit|if-claim-changes|if-requirement-changes|a correction|correction.{0,40}(reopen|back-edge)|(reopen|back-edge).{0,40}correction
gate12: fix-round-not-re-read|fix-round-loop-open|check 12|gate check|last check|twelve checks|12 checks|eleven checks|thirteen
canonical: (canonical|authoritative).{0,80}(data[- ]model|reference|primitive|description|shapes|behavior|behaviour|rule)|(data[- ]model|reference|description).{0,60}(is|are|as|the) (the )?(canonical|authoritative)|authoritative shape
noserver: two paths|walk the same|neither wraps|markdown-only path|runtime is unavailable|when no runtime|no ductus runtime|without the runtime|one contract|host file tools|its own file tools|host.s file tools|not on `PATH`|on the runtime path|byte-identical reports
```

## Prose to cut (AC18, AC21)

Method: each section read and given a kept fraction; cut = measured paragraph bytes times one minus the fraction, with 75% as the planning yield. No section was trial-rewritten. Strict keeps every rule a walker needs; pointer also drops a primitive's rules, which AC21 allows.

| File | Bytes at `2abc19aa` | Must lose | Strict cut | Pointer cut | At 75% of pointer |
| --- | --- | --- | --- | --- | --- |
| `framework/reference/review.md` | 64,606 | 14,606 | 23,460 | 33,240 | About 39,700 |
| `framework/reference/analyze.md` | 79,745 | 29,745 | 29,650 | 42,400 | About 47,900 |

Largest sections, with pointer cuts: in `review.md`, Write `review.md` (11,520, cut 7,000), Resolve target and scope (10,127, cut 7,500), Fix-round loop (6,190, cut 3,650); in `analyze.md`, Rules (8,914, cut 2,870), Project-level consistency (8,306, cut 4,720), Acceptance-criterion path existence (7,361, cut 5,420), Finding dispositions (7,256, cut 3,720).

Elsewhere in commands and references: `> **For agent runtimes**` is 7,464 bytes over 15 hits, saving about 6,500, and `**Where a named section lives.**` 11,104 bytes over 26 hits, saving about 6,000. Restatement in the five largest commands is about 23,900 bytes, of which about 16,700 is removable (sampled). `amend`, `help`, `link` and `target` have no reference file, so their pointer stays.

Every heading keeps its exact text, since a command step, `framework/bootstrap/ductus.md` or `AGENTS.md` cites it: `runtime/src/primitives/resolve_anchor.rs` holds the corpus test, `scripts/audit/template-alignment.sh` hardcodes nine blocking headings, and `scripts/audit/marker-list-parity.sh` matches three sentences in `analyze.md`, which must change with them. `framework/bootstrap/ductus.md` cites `review.md`'s §Resolve target and scope item 5 by number, so that list must not renumber.

The ten `check-artifacts` families hold about 27,000 reference bytes against a 1,333-byte description. They are dropped, not moved.

Constitution headroom: 64 bytes at `2abc19aa`, so AC2, AC14, AC15, AC17, AC21 and AC9's table row must be net-negative together.

## What the flow pass found

Three agents at `6aef0472`'s tree, plus the fold edits, followed one behavior each from command to primitive to gate on fixtures with the release binary: dispositions (20 hand-offs, 20 fixtures), a review's life across a reopening (14, 34), and what reopens a `done` spec (19, 43). Their counts: 14 contract breaks and 20 gaps, 11 and 19 once the same finding from two flows is merged. Only the first item was reproduced by the session that ran them, up to the rewritten record, since its fixture's gate stopped at lint. In the order of task 16's items:

1. **Window.** `must-violations: 1` committed on an unchanged `src/a.rs`: `compute-review-scope` reads `base-source: last-review`, `review-owed: true`, `scope-count: 0`; step 1's empty-scope branch, `write-review --empty-scope`, writes `must-violations: 0`; with an analysis the gate passes. A SHOULD beside an unrelated own change, and an unwaived `not-re-read` round, clear the same way. `framework/reference/review.md`'s §Idempotency and item 3, and `write_review.rs`' "no implementation files", state otherwise; `--since` is the only route to the old file today.
2. **Decisions expire.** A person's waiver on `src/a.rs`, then a reopening changing `src/c.rs`: `process-waivers` with nothing fired reports "waiver expired", `process-decisions` "the finding no longer fires". The empty-scope branch's `restricted` retained them, so extend that trigger: give both primitives the scope, or have steps 8 and 9 mark each anchor out of scope. A key with no path needs a rule from the owner.
3. **Gate order.** A task added, reopened, code and tick committed: gate says `analyze-stale`; after `/analyze` it passes with `review-owed: true`; running the owed review stales the analysis, so analyze again: reviews 1, analyses 2, over the bound. Review first measures 1 and 1. Options: `/implement` step 14 also calls `compute-review-scope` and halts naming `/review` when owed, which clears only once the record is committed; or the gate reads it; or the order is only documented.
4. **Draft back-edge.** Reopen, code in `src/a.rs` and new `src/extra.rs`, `/amend` to draft, back through to `in-progress`: `base-source: reopen` at the last entry, `review-owed: false`, scope only `spec.md`; the gate passes after an analysis; measured reviews 0, analyses 1. The window, `diff-cross-spec` and the measure each start at the last entry into `in-progress`, and `check-stuck` shares the lookup. The owner chose one reopening on 2026-11-06, starting where the spec last left `done` or, never `done`, first entered `in-progress`; `git log -p -G'^status:'` over `specs/*/spec.md` at `9129d914` shows 0 of 938 transitions to `draft`.
5. **Rename.** Reopen with work, then a commit renaming the slug: the base is just before the rename, `src/a.rs` and `src/b.rs` are in no list, and `diff-cross-spec` counts the old directory's files as cross-spec. `find_in_progress_commit` looks status up by path; compare by `FeatureForm`.
6. **Shallow.** `git clone --depth 1`: `base-source: reopen`, `review-owed: false`, `scope-count: 0`; `diff-cross-spec` 0, no guidance; at depth 3 the base is the reopen itself and drops its changes. `measure-reopenings` reads `shallow-history` on the same clones. Check `is_shallow` where the walk reaches a cut, then a distinct `base-source`, or an error.
7. **Revert.** `git revert` of a review's commit reads `base-source: last-review` at the revert, since its parent's stamp differs. A commit finishes a run only when its stamp is not already held at the transition's parent or earlier, the measurement's carried-in test.
8. **Person's decisions uncounted.** `measure-reopenings` reads only `decisions:`: `waivers:` and `round-waivers:` entries by a person read 0. Count them by `waived-at`. A declined deferral, and a confirmed one recorded `fixed`, leave no trace: the owner chose AC17 narrowed to stored decisions (2026-11-06), over a new field.
9. **`ductus exec`.** A `performReview` response with `contract-break: true` writes `contract-breaks: 0`, since `PassObservation` (`runtime/src/schema/extensions.rs`) lacks the field. Add it through the walker, schema and golden.
10. **Rule waiver over a break.** Run 1 stores a rule waiver; run 2 matches its key and marks it a contract break: accepted with step 11's fields, `decided-by: rule`, nobody asked. `write-review` should refuse it, and step 9 should say such a match does not dispose of it.
11. **Rules left in prose.** `framework/reference/status.md`'s §Rendering reference states `dashboard`'s render, though step 2 only emits `rendered-markdown`, which `--help` never names; `framework/commands/implement.md` and its reference hold the rules of `derive-boundary`, `mark-task` and `check-stuck`, each `--help` one line. The owner's earlier call kept only analyze's phrase lists and review's rule-file selection and waiver lifecycle as policy, so this is policy or cut: the owner chose the cut (2026-11-06).
12. **No server.** `write-review --finding x` exits 2; with only the required flags it exits 0 writing `must-violations: 0`, and the gate passes. `process-decisions` with no `--fired` reports a person's waiver expired. Options: the constitution's pointer sentence names `ductus mcp` over stdio, or these writers take a JSON file; refusing an empty `write-review` is out, as it refuses a clean review. The owner's answer (2026-11-06): the server is required, so a command stops without it and the pointer stops offering the CLI.
13. **Drift remedy.** `disposition-drift` says re-run `/review`, which stops on `review-owed: false`; forced, it erases the undecided findings. Item 1's fix covers a record the gate holds; this one's undecided observations hold nothing, so the message names `--since` otherwise.
14. **`--waive`.** `applied-waivers` clears the run and writes no entry, so the next run counts the MUST again; only a hand edit holds. Give `write-review` a new-waivers argument stamped from `reviewed-at` and `decided-by`.
15. **Override.** Spec: `--waive` and `--defer` override any decision; both take a rule ID. A person's same-outcome waiver over the rule's keeps `decided-by: rule`, as `merge` sees no change. Narrow the sentence to violations, or let them name a key; `merge` cannot just test by-rule, since a carried rule waiver arrives alike.
16. **Groom.** `framework/reference/groom.md` and `framework/reference/analyze.md` say a finding against a managed rule file is waived; `write-analysis` refuses that for a gating one. Scope both, and name the deferral to the spec's own task.
17. **Damaged decisions.** `analysis.md` with `decisions: not-a-list`: the gate passes, while `validate-frontmatter` says hard fail. Block through `unreadable_record_block`.
18. **Stale contract unread.** A scenario edited while `done`, then a reopen: `review-owed: true`, the gate says stale, and the scope omits the scenario, so a run refreshes the digest without reading it. Add each contract `review_freshness` names to the scope.
19. **Run to record.** A run at one commit, later code, then the record alone: `base-source: last-review` at the record's commit, `review-owed: false`. `reviewed-against` says where it read: add what lies between, or start there when an ancestor. AC5 says "commit", so it is the owner's.
20. **Retired claimant.** A folded branch-scoped spec's `src/s.rs` stays excluded, named by a directory gone from disk, though `invalidate-review` demands a fresh review of what a fold brought. A claimant with no directory excludes nothing, or only a fold passes its commits to its target, or the description is corrected: the owner's.
21. **Dies in step 10.** A fix written, then stopped: no mark, the gate passes, only `review-owed: true` shows it. `start-fix-rounds` could set the mark on an existing record, whose description then changes.
22. **Cut-off re-run.** An open record committed, then the re-run the gate demands: reviews 2, over the bound. Treat a run still carrying the mark as leaving a gating finding, or as no run, or record that a cut-off counts as a miss: the owner's.
23. **`write-boundary`.** No flag, `resolve-session` does not return it, and the next target write drops it; only `ductus exec` reads it. Say so in the step and the guidance, or add it.
24. **Open mark on `done`.** `write-review --loop-open` on a `done` spec: `check-artifacts` clean, the gate `already-done`. Add it to review-state drift.
25. **Two reopen tests.** `prune-plan`'s `adds_unchecked_box` and `classify-spec-edit`'s `unchecked_items` disagree on a box under an unnumbered heading, above the first task, moved between tasks, and with `tasks.md` absent at `HEAD`. Use one, and the owner picks whether a moved box reopens.
26. **Classifier and sweeps.** The constitution's "`classify-spec-edit` decides from the diff. Two kinds take no back-edge", and the primitive reads both as `reopens`. Teach it, or say it does not decide them; `mechanical_sweep` has no test for a cross-service link: the owner's.
27. **Signpost route.** `framework/reference/implement.md` sends it through `/amend`, whose route on a `done` spec reopens. Reword to the gate's sentence.
28. **Untick.** `- [x] AC2` to `- [ ] AC2` on `done` reads `correction`, and nothing an adopter runs reports `done` with an unticked criterion, though the constitution says it reopens. Reopen on an untick, narrowing `data-model.md` and `a_tick_either_way_is_a_correction`, or amend the constitution: the owner's.
29. **`/amend`'s criterion route.** Line 8 claims it; its classifier, record steps and status table lack it, as at `3360dea3`. Add the steps or drop the claim.
30. **Impact on `done`.** A `cross-spec-impact:` entry added on `done` reads `correction`, and nothing reads it. Have `check-artifacts` report it, advisory.
31. **Wording.** See task 16's item. Also: a person's stored deferral beside a counted by-rule waiver; `AGENTS.md`'s "CLI cannot pass false"; descriptions naming `/ductus:` and `check-artifact-size` a literal `/{project}:`.

**What item 1's fix leaves.** Measured on fixtures with the release binary built from the fix, each record counting a MUST unless it says otherwise. `contract-breaks: 1` is not held, so it reads `base-source: last-review`, `review-owed: false` and `scope-count: 0`, and a forced run, or one after an unrelated own commit, writes 0. A record from before the reopening reads `reopen` with only `spec.md` in scope; a pass through draft read the same until item 4's fix. With no transition it reads `none`, `scope-count: 0`. `--since=HEAD` reads `scope-count: 0`, and `write-review --empty-scope` then writes `must-violations: 0`, `blocking: false`. By `compute-review-scope` and each `review.md`'s frontmatter over all 77 specs at this fix's tree: 0 of 71 records count a MUST, a SHOULD, an unread round, `blocking` or a contract break, the 5 specs reading `none` have no record, and at `3b5093da` 0 of the 126 records at closings since 2026-10-01 count any of them, so none of these is reachable in this repository today, and an adopter's records are unknown. Options, none prototyped, each starting no review or reopen here: (1) a counted contract break stops the start moving, a held record's next window starts no later than its own `diff-base`, reaching the earlier reopening and no-transition cases, and `write-review --empty-scope` refuses over a record counting a finding, reaching `--since=HEAD`, while a narrowed `--since` with files in scope stays the operator's choice; (2) the contract break alone, one term, leaving the rest for an adopter's older records; (3) waive all of them with these counts as the reason. `framework/reference/review.md`'s advice to pass the current commit as the base for a spec with no transition clears a held record too, so option 1 or 2 corrects that sentence. The owner's.

**What settled it.** Prototypes behind an environment switch at `d6594496`, each input a real `write-review` record over MCP on a fixture with a real linter, reading `compute-review-scope` and `check-review-gate` from the release build. As first written option 1 made a person's waiver of a break never close, because `contract_breaks` counts everything but `Fixed`, and its held record reached back through `holds_done_until_reviewed`'s no-map arm: 26 of the 126 closings since 2026-10-01 start on a record with no map, and over the 24 whose `diff-base` still exists their first reviews' scope goes from 588 to 3,202 paths, by `compute-review-scope` with and without `--since` from a clean checkout at the parent of the commit first writing the reopening's record. At `d6594496`, 032 goes from 7 to 104, 033 from 69 to 174 and 046 from 97 to 125. With both narrowed, to a break not fixed or waived and to a record counting a finding, it reads 5 of the 8 inputs, the chosen list 7, the list with plan entries kept instead 7 (a different one missing), and option 2 narrowed 1. With the list on, all 77 specs read the same base, scope count, `review-owed` and gate as without it, and the 2,619 tests pass. The spec's entry holds the decision.

**What the gate reading `review-owed` costs.** Method: `measure-reopenings --since 2026-10-01` at `fc20a939`, 126 closings holding 112 reviews and 149 analyses, 28 over the bound; then `compute-review-scope` from a worktree at each closing commit, each scope path classed as this spec's contract, another spec's directory, a non-spec path a commit this spec claims changed, or one only unclaimed commits changed, leaving out what only the closing commit changed, since the gate runs before that commit exists. The binary with the refinement read owed at the same 20 closings this classing gives.

| Why a review reads as owed | Closings with no review (28) | Closings with a review (98) |
| --- | --- | --- |
| Only another spec's text | 11 | 22 |
| Only an unclaimed commit | 10 | 0 |
| The spec's own change | 4 | 6 |
| Not owed | 3 | 70 |

Of the 11 closed analyze, review, analyze, all over the bound, the new order leaves 1 and 1, so over-bound closings fall from 28 to 17, plus up to 4 of the 6 second reviews, the other 2 being over already; without the refinement, up to 17 of the 28 second reviews. At `fc20a939`, 47 of 47 `in-progress` specs read owed; the gate named `/analyze` first on 11, of which 015 and 061 hold only an unclaimed commit's paths. `compute-review-scope` took a median of 1.31 s and at most 2.13 s per spec over those 47.

Not examined by any flow: real agent hosts, Windows, `--all`, `--fix`'s second write, `/fold`, `/consolidate`, the CI template's record check, and the re-read agents' path end to end.
