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
2. **Decisions expire.** A person's waiver on `src/a.rs`, then a reopening changing `src/c.rs`: `process-waivers` with nothing fired reports "waiver expired", `process-decisions` "the finding no longer fires". The empty-scope branch's `restricted` retained them, so both primitives now read the scope and retain an anchor outside it, its file still on disk. At `f2e8c70a`, parsing each `review.md` key's own shape against the sweep's scope, all 1,075 stored keys name a file; of the 849 on `in-progress` specs, 168 are in scope, 676 outside it on disk and 5 outside it and gone. A key with no file therefore expires as before, with none to change.
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
15. **Override.** Spec: `--waive` and `--defer` override any decision; both take a rule ID. A person's same-outcome waiver over the rule's keeps `decided-by: rule`, as `merge` sees no change. Narrow the sentence to violations, or let them name a key; `merge` cannot just test by-rule, since a carried rule waiver arrives alike. The owner chose the narrowing (2026-11-06), and the body now follows the record.
16. **Groom.** `framework/reference/groom.md` and `framework/reference/analyze.md` say a finding against a managed rule file is waived; `write-analysis` refuses that for a gating one. Scope both, and name the deferral to the spec's own task.
17. **Damaged decisions.** `analysis.md` with `decisions: not-a-list`: the gate passes, while `validate-frontmatter` says hard fail. Block through `unreadable_record_block`.
18. **Stale contract unread.** A scenario edited while `done`, then a reopen: `review-owed: true`, the gate says stale, and the scope omits the scenario, so a run refreshes the digest without reading it. Add each contract `review_freshness` names to the scope.
19. **Run to record.** A run at one commit, later code, then the record alone: `base-source: last-review` at the record's commit, `review-owed: false`. `reviewed-against` says where it read: add what lies between, or start there when an ancestor. AC5 says "commit", so it is the owner's: waived with its counts (2026-11-06).
20. **Retired claimant.** A folded branch-scoped spec's `src/s.rs` stays excluded, named by a directory gone from disk, though `invalidate-review` demands a fresh review of what a fold brought. The owner chose a fold below `done` passing its commits to its target, wherever the window starts (2026-11-06); *What item 20 measured* holds each option's reads.
21. **Dies in step 10.** A fix written, then stopped: no mark, the gate passes, only `review-owed: true` shows it. `start-fix-rounds` could set the mark on an existing record, whose description then changes.
22. **Cut-off re-run.** An open record committed, then the re-run the gate demands: reviews 2, over the bound. The owner chose a run still carrying the mark counting as leaving a gating finding (2026-11-06).
23. **`write-boundary`.** No flag, `resolve-session` does not return it, and the next target write drops it; only `ductus exec` reads it. Say so in the step and the guidance, or add it.
24. **Open mark on `done`.** `write-review --loop-open` on a `done` spec: `check-artifacts` clean, the gate `already-done`. Add it to review-state drift.
25. **Two reopen tests.** `prune-plan`'s `adds_unchecked_box` and `classify-spec-edit`'s `unchecked_items` disagree on a box under an unnumbered heading, above the first task, moved between tasks, and with `tasks.md` absent at `HEAD`. Use one, and the owner picks whether a moved box reopens: the owner chose no, by the line test, and that the gate and `check-artifacts` also read an unchecked box (2026-11-06, the spec's Resolved Questions).
26. **Classifier and sweeps.** The constitution's "`classify-spec-edit` decides from the diff. Two kinds take no back-edge", and the primitive reads both as `reopens`. The owner chose teaching it both (2026-11-06), the spec's Resolved Questions holding the counts.
27. **Signpost route.** `framework/reference/implement.md` sends it through `/amend`, whose route on a `done` spec reopens. Reword to the gate's sentence.
28. **Untick.** `- [x] AC2` to `- [ ] AC2` on `done` reads `correction`, and nothing an adopter runs reports `done` with an unticked criterion, though the constitution says it reopens. The owner chose both the classifier reopening on an untick and `check-artifacts` reporting the state (2026-11-06).
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

**What item 20 measured.** Item 20 is two gaps. A commit touching a staging directory stays excluded by a claimant gone from disk, as it says. And a fold into a `done` upstream reopens it at the fold, so the staging spec's earlier code lies before the window, in no list. Prototypes behind an environment switch at `f2e8c70a`, each input folded through `set-status`, `invalidate-review` and `retire-feature` on a fixture with a real linter and read by `compute-review-scope` from the release build. Each cell is how many of the staging spec's two files, `src/s.rs` touching its directory and `src/t.rs` not, the upstream's scope holds; `src/s.rs` is excluded where it is not held and the commit was in the window.

| Staging spec, upstream | Today (also the description corrected) | No directory excludes nothing | Fold passes commits, in the window | Fold passes commits, all | Fold passes commits, all, below `done` only |
| --- | --- | --- | --- | --- | --- |
| `done`, `done` | 0 | 0 | 0 | 2 | 0 |
| Below `done`, `done` | 0 | 0 | 0 | 2 | 2 |
| Below `done`, `in-progress` | 1 | 2 | 2 | 2 | 2 |
| `done`, `in-progress` | 1 | 2 | 2 | 2 | 1 |

Below `done` into a `done` upstream, today, one review of that scope and one analysis pass the gate with neither file in any list. Excluding nothing is the only column that reaches an unrelated spec: a consolidated spec's one commit touching its directory takes a current record's gate from passing to `review-owed`, and a fix round's brief gains that commit's path when it lands in the round. A code-only commit naming a spec reads as unclaimed once its directory is gone, in every column, so a spec overlapping it is owed a review of that path today.

Over all 77 specs at `f2e8c70a`, from clean worktrees, the excluding-nothing and fold-passes-all prototypes read the same base, scope, `review-owed` and gate as the unchanged code; the below-`done` form fires on a subset of the second. The retired numbers are 004, 005 and 053, since 027 and 048 were renamed and stay on disk by number. 0 of the 126 closings since 2026-10-01 hold a commit touching their directories or naming them; the newest is `cc870e18` of 2026-09-15, which also touches live specs, so those keep excluding its paths. No branch-scoped spec is in this repository's history, and the first fix round, on 2026-11-03, postdates every such commit. With a fold in the window, `compute-review-scope` on a clone of this history (3,113 commits) took 1.42 s without the prototype and 2.35 s with it, medians of three, the prototype walking the window twice; the shipped form, which uses the window's own walk, took 1.48 s against 1.46 s.

Not examined by any flow: real agent hosts, Windows, `--all`, `--fix`'s second write, `/fold`, `/consolidate`, the CI template's record check, and the re-read agents' path end to end.

## What the fresh review found

The five passes of `/ductus:review --since=6aef0472` at `1a0623f2`, by three agents following one behavior each with the release binary: what a run records, where a window starts and what the gate holds, and what reopens with every shipped sentence. Scope 116, of which 27 are generated copies not read, each generated from a source read. Rule files: the 12 `discover-rule-files` selects. Each item is an input, what it read, and the fix; the task 18 boxes group them. The stored decisions' fates are on task 18.

**Rule violations.**

- **R1, `QUAL-CLAIM-001`, SHOULD, `runtime/src/primitives/write_review.rs`.** `write-review --empty-scope` over a record the gate calls unreadable wrote `must-violations: 0` and "Zero findings", as if none existed. Refuse on an unreadable record.
- **R2, `QUAL-CLAIM-001`, SHOULD, `runtime/src/primitives/measure_reopenings.rs`.** A tip dated before the window ended the walk: one closing read `nothing-to-compare`. 0 of 3,122 commits here have a child older than its parent. Skip such a commit rather than stop.
- **R3, `QUAL-CLAIM-001`, SHOULD, `runtime/src/primitives/check_stuck.rs`.** After a slug rename `since-sha` holds but the count and first-box comparison read the new path: `commit-count: 4, stuck: true` became `1, false`. Resolve each commit's path by number.
- **R4, `QUAL-TEST-001`, SHOULD, `runtime/src/primitives/classify_spec_edit.rs`.** `a_reference_added_or_removed_as_a_list_item_is_mechanical` never removes one. Add that input.
- **L1, `QUAL-CLAIM-001`, low confidence, `runtime/src/primitives/compute_review_scope.rs` `was_folded`.** A staging spec whose frontmatter does not parse at removal reads as no fold, with no notice. `/fold`'s `read-spec` should stop on it first, not driven, so it needs a by-hand removal. Recorded low-confidence.

**Contract breaks.**

- **B1, `framework/commands/review.md` step 11 and `write_review.rs`.** One `--waive` through a run's two writes stores two identical entries; `process-waivers` then warns of a duplicate, and `measure-reopenings` reads `decisions-by-person: 2`. Dropping it from the last write brings the finding back, and passing it after a fix cleared the finding is refused, leaving the mark. Reproduced independently. An anchor recorded applies and is not stored again, and a rule this run's earlier write stored asks nothing of the last.
- **B2, `runtime/src/primitives/decisions.rs`, step 9.** A deferral of a break to a work item alone: run 2's `process-decisions` matches it with no break flag, so written that way `contract-breaks: 0`, no list, and the gate passes with the break unfixed. The hold depended on a host re-marking it. The stored entry carries `contract-break: true`, and `write-review` re-marks a match unless the entry is in `expired-decisions`.
- **B3, `write_review.rs`, `runtime/src/interpreter/mod.rs`.** `ductus exec review since=HEAD` over a record with `must-violations: 1` and no list writes 0: the walker never sets `empty-scope`, and the refusal read only the flag. Reproduced independently. Refuse on a derived empty scope too.
- **B4, `decisions.rs`, `write_review.rs`.** `src/b.rs:3:9`: a person's decision reads `expired` with the file unread, and a break lists `src/b.rs:3`, which the next scope cannot stat. One shared stripper, repeating.
- **B5, `compute_review_scope.rs` `why_unread`.** A plan's `src/*.rs` in scope: a decision on `src/b.rs` reads `retained`, "did not read". Test through `PlanEntries`.
- **B6, `check_stuck.rs:206`.** Close, reopen, code, a commit with `status: [` or `spec.md` removed, then its repair: the base moves to before the repair, `review-owed: false`, and one analysis passes. Carry the reopening through a status-less commit. 0 of 2,408 pairs here.
- **B7, `compute_review_scope.rs` `last_review_commit`.** Code with the flip, run B, `git revert` B to the run the reopening carried: base at the revert, scope 0, the gate passes. A stamp held at the transition finished nothing here, so the window stays at the reopen. A record restored from before the one the transition holds still reads as a finish, a limitation to state in `plan.md`.
- **B8, same function.** Reverting away the only run: base at the reverted run, scope 0, `not-reviewed`, then an empty-scope write and an analysis pass. No record at `HEAD` starts at the reopen. `last-run: null`, which a fold's `invalidate-review` writes, keeps the start, since what the fold brought joins by its own rule.
- **B9, `runtime/src/primitives/commit_attribution.rs` `belongs_to`.** A staging spec renamed before its fold loses commits touching its old slug. Match the directory entry by `FeatureForm`.
- **B10, `compute_review_scope.rs` `record_at`.** With an uncommitted rename the gate blocks naming `src/b.rs` while the scope does not hold it. Resolve through `feature_file_in_tree`.
- **B11, `compute_review_scope.rs` `paths_owing_a_review`.** A rename-only reopening holding a scenario owes a review. A contract whose digest is current owes none, with no digest it still does.
- **B12, `runtime/src/primitives/mod.rs` `SkipScanner`.** A box in a `~~~` fence counts as owed, against AC27. Recognize both fences, only its own kind closing.
- **B13, `runtime/src/primitives/derive_references.rs:390`.** `See [b](../../034-b/spec.md), the timeout is 30s, then [api](…)` with `30s` changed reads `mechanical`, as does a `[here](#anchor)` or `![i](a.png)` first; multibyte text between panics, `start byte index 74 is not a char boundary`. Take a URL only when it follows that `](`. 0 such lines here.
- **B14, `framework/reference/groom.md:62`, `framework/commands/groom.md`.** The spec-edit reopen has no step, no consent, and reads an error as staying `done`. Add a step mirroring `/amend` step 5, naming the reopen before it writes.
- **B15, retired "without the server the agent runs the CLI".** `specs/000-slash-commands/scenarios/target-names-the-step-the-gate-would-name.md:22`, `specs/022-deterministic-runtime/scenarios/the-cli-surface-nothing-pins.md:16`, `runtime/src/main.rs:1113`, `runtime/src/schema/registry.rs:22`. Every command now says to stop.
- **B16, `framework/constitution.md`'s "an edit elsewhere".** A sweep through `research.md` in two specs read `reopens` with `other-file`. Route that arm through the same test, keeping a real change `other-file`.

**The owner's call.**

- **O1, a stopped run and the bound.** A record with `budget-stop: true` and no finding: `stopped_early` keeps the window, the gate blocks `review-owed`, and the re-run measures `reviews: 2, analyses: 1, over-bound: true`. A restricted run reads the same. Reproduced independently. `review_fields` reads neither field, while the window treats both as a stop. 3 commits in all history touch a budget stop in a `review.md` and none a non-empty `skipped-passes`: `f47d2b03`, 078's first review, which also counted a MUST and so was gating anyway; `d62e72f3`, which cleared it; and `94c31465`, 077's last run, with no re-run after it. `specs/077-security-cross-rules/review.md` is the 1 record at `HEAD` holding one. By a prototype of the first option, the batch's binary against the same with `review_fields` changed, at `d3c869b4`: 0 of the 126 closings since 2026-10-01 read differently (261 runs and 28 over the bound in both), and 0 of all 445 closings (410 runs and 42 over). On a fixture it does differ: a stopped and a restricted run go from over the bound to within, and a plain second review stays over. Options: count a stopped run as leaving a gating finding, as the entry on a cut-off does for the open mark, 2 lines in `review_fields` and AC1's wording; or leave it a miss, no code, which asks a task on this spec with nothing to fix for each. The owner chose the first (2026-11-06).

**Mechanical.**

- `runtime/src/primitives/invalidate_review.rs`: the record keeps `excluded`, `contract-breaks` and `fix-rounds-open` beside `last-run: null`; `open-findings` stays.
- `write_review.rs`: the empty-scope Summary omits a break it holds.
- `framework/reference/review.md`: the `observations` row's sample says "it does not hold done" with a break among them.
- `framework/commands/analyze.md` step 22: its `--fix` list omits an outstanding SHOULD, a listed break, the open mark, an unticked criterion and an unchecked box.
- `specs/066-audit-records-and-the-completion-gate/scenarios/the-fix-round-loop-is-recorded-and-gated.md:9` and `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md:45`: "check 12", renamed in 078's data model.
- Doc comments on the wrong item: `write_review.rs` (`surviving_waivers`), `check_review_gate.rs` (`unreadable_record_block`, `scenario_question_block`).
- `data-model.md`'s accounting sentence omits what a fold brings; it gains `new-waivers`' two-write rule and `decisions:`' `contract-break`.
- Enumerations of review-state drift: `CHECK_ARTIFACTS` in `runtime/src/schema/descriptions.rs`, `docs/analyze.md`, `specs/067-analyze-and-audit-checks/data-model.md`, `check_artifacts.rs`' module doc, each omitting a condition, a listed break among them.
- `classify_spec_edit.rs`' module doc is corrected; `plan.md`'s "What reopens" still says the two agree on a reference-only edit.
- `runtime/CHANGELOG.md`'s size figures, now 419,612, 45,007 and 48,553 by `wc -c`.
- M115's name in `scripts/mutation/reopen-measure.py`: "a tick reopens" makes a criterion left unticked reopen.

**The two builds across the corpus.** `e6250b77` against `64d0be93`, each binary copied out and checked by a literal only the new one holds, run from a clean worktree of `64d0be93` with `node_modules` present and ignored, by `compute-review-scope`, `check-review-gate` and `check-artifacts` on each of the 77 specs. 0 differ in base, base source, scope count, excluded count, `review-owed` or finding families, and neither build errored. 26 differ in the gate's first block, each moving to `unchecked-task`: 21 from `review-stale`, 4 from `review-owed`, 1 from `not-reviewed`. New counts by first block: 26 `unchecked-task`, 25 `already-done`, 12 `scenario-open-questions`, 7 `review-owed`, 3 `not-reviewed`, 2 `review-stale`, 2 `undischarged-cross-spec-impact`.

**Waived by the rule, each with this reason for its observation.**

- `framework/commands/analyze.md` step 8 names no family: it points at the description, as AC9 asks.
- `1. [ ]`, `> - [ ]`, `* [ ]` and `+ [ ]` are no boxes to `find_checkbox_line`: one documented grammar, `mark-task`'s too, every surface alike, 0 such lines here.
- A claim carried only in a reference's label reads `mechanical`: the scrub removes label and target by its stated design; add it to `plan.md`'s known limitations.
- A reference added as an ordered item or table row reads `reopens`: it errs toward reopening.
- A reference-only edit reads `mechanical` while staleness reads `stale`: exempting references from the digest is a change of its own, with 0 occurrences since no spec here carries `references:`.
