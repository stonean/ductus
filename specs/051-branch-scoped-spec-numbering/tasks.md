# 051 — Branch-scoped spec numbering Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Spent task sections are pruned per §tasks-phase; git history holds them, and the plan's Technical Decisions hold the decisions they settled.

## 28. Prune plan.md §Implementation notes into the design record

- [x] Move the durable decisions under §Implementation notes into §Technical Decisions or §Trade-offs, and drop the implementation journal, which git history holds
- [x] Re-run /ductus:analyze: the plan-record finding no longer fires

- **Done when**: plan.md has no section outside the design record, every decision the section carried survives in the design record, and analyze no longer reports plan-record for 051

## 29. A fold leaves no in-repo reference dangling (AC6, AC22)

- [x] `rewrite-spec-links` re-points a retiring directory's inbound links only in the `.md` files under the spec root, so a reference to it from `README.md`, `AGENTS.md`, `CLAUDE.md` or `.githooks/pre-commit` survives the fold and `check-orphaned-references` reports it. Never true: the walk and the referrer set were the same at `1657a4d6`, when AC22 was ticked. Found by the 2026-10-27 criteria verification.
- [x] Extend `rewrite-spec-links` to those four files, the referrers `check-orphaned-references` reads outside the spec root, rewriting whatever path token that check would flag, with the same whole-segment matching; the owner chose the shared primitive (2026-10-27), so `/{project}:consolidate` gets it too, which is cross-spec impact on `052-spec-supersession-and-consolidation` (its AC7).
- [x] Correct the prose that bounds the rewrite to the spec root: `framework/commands/fold.md`, `framework/commands/consolidate.md`, `framework/reference/fold.md`, and the `check_orphaned_references.rs` doc comment saying the framework repairs only what it authored.
- [x] Prove each new test fails without the change.

- **Done when**: A fold or consolidation leaves no reference to the retired directory in the spec root or the four referrer files, `check-orphaned-references` reports clean afterwards in a fixture that held one, AC6 and AC22 hold as written, and 052 carries its back-link

## 30. Disposition: a `./` or `../` path to the spec root is invisible to the fold and the orphan check (AC6)

- [x] Found by task 29's verification (2026-10-27): the path scanner `check-orphaned-references` and `rewrite-spec-links` share counts a `/` before the spec root as a path character, so `[x](./specs/1234.1-staged/spec.md)` in `README.md` is neither re-pointed by a fold nor reported once the directory is gone (probed through the release binary). A root-absolute `[x](/specs/012-auth/spec.md)` is missed the same way (the 2026-10-27 review's probe). Pre-existing and shared by both primitives; AC6 says no reference dangles.
- [x] The scanner is `067-analyze-and-audit-checks`' surface (scenario `orphaned-reference-check`), which another session holds; whether to route it there or fix it here is the owner's decision. The owner chose to fix it here (2026-10-28), declaring the cross-spec impact on 067 with a signpost back there.
- [x] The shared scanner (`managed_path_spans`, `runtime/src/primitives/check_orphaned_references.rs`) takes per-referrer prefixes that name the project root — `./` and a leading `/` in a root markdown file, `./` in `.githooks/pre-commit`, `../` and a leading `/` in `system.md` — so `check-orphaned-references` reports and `rewrite-spec-links` re-points a path behind one, the prefix kept; 067's `data-model.md` states the rule and its spec carries the signpost. Written 2026-10-28; the re-read rounds settled the token rule as 067's `data-model.md` states it — a prefix counts wherever the path written bare would, except a leading `/` that ends another directory's name — and `root_prefixes(referrer)` picks the set from the referrer's path. 067's data-model rule, its signpost, 051's `cross-spec-impact` entry, `framework/reference/fold.md` and `framework/reference/analyze.md` already state it
- [x] Tests show each prefix found in each referrer kind and each other prefix left alone, on both primitives, and each fails when its rule is removed (`scripts/mutation/root-prefixed-references.py`)

- **Done when**: the finding is fixed, routed, or discarded, with a discard's reason written on the task

## 31. Verify the fix rounds' runtime changes (2026-10-28 handoff)

- [x] Run `python3 scripts/mutation/run.py scripts/mutation/rewrite-spec-links.py` whole: 35 mutations, every one expected killed. An interrupted run on 2026-10-28 killed M1–M29 against the working tree that became the commit after `f33096a5`; M30–M35 (the carried-scenario mapping, the unheld-scenario listing) were not run. Run whole on 2026-10-28 against the tree of task 30's commit: 35 of 35 killed, and 35 of 35 again on the tree of the round after `3fc5755a`
- [x] Run `python3 scripts/mutation/run.py scripts/mutation/create-feature-claim.py` whole: M1 and M2 killed when run alone, M3 the one expected survivor. The re-read of round 3 made M3 killable and added M4: 4 of 4 killed at `54715126`, as the re-read of `3fc5755a` confirmed
- [x] Run the same for the task-30 list `scripts/mutation/root-prefixed-references.py` once task 30 writes it: run whole on 2026-10-28 against the tree of task 30's commit, 24 of 24 killed, and 26 of 26 at `54715126`; the list was rewritten with the rule in the round after `3fc5755a`: 41 of 41 killed on that round's tree, and 46 of 46 on the tree of the round after `d1853444`
- [x] An independent re-read covers `f33096a5`, the round-4 commit after it and task 30's commit, per `AGENTS.md`'s review entries, before 051's review record is written. Rounds (2026-10-28), each commit's message listing its findings and their dispositions: the re-read of `f33096a5`, `77fff147`, `38fc39f6` and `4e82686d`/`db6ae819` found 10 SHOULD, fixed in `54715126`; of `54715126`, 1 SHOULD, fixed in `3fc5755a`; of `3fc5755a`, 3 SHOULD, fixed in `d1853444`; of `d1853444`, 0 MUST and 2 SHOULD, both gaps in its tests, fixed in `c1cf51ff`; of `c1cf51ff`, 0 MUST and 0 SHOULD, its 4 NOTE disposed of in the commit after it, a prose-only round that is not re-read

- **Done when**: every mutation in the three lists is killed or is its list's declared survivor, and the independent re-read of those commits reports no MUST or SHOULD

## 32. Implement scenario: a fold target is held to the name grammar

- [x] Implement the behavior described in `scenarios/a-fold-target-is-held-to-the-name-grammar.md`

- **Done when**: `rewrite-spec-links` refuses a `to` whose feature half is not a feature directory name or whose scenario half is not a scenario slug, before any file is read or written, leaving the corpus byte-identical; a test refuses `templates` and a scenario half holding a space, a further `/`, `$(...)` or a capital letter, and fails without the check, with its mutation in `scripts/mutation/rewrite-spec-links.py` killed; routed from 048's review (BE-INPUT-002)

## 33. Implement scenario: the spec root is read before the first write

- [x] Implement the behavior described in `scenarios/the-spec-root-is-read-before-the-first-write.md`

- **Done when**: `rewrite-spec-links` reads every markdown file under the spec root before its first write and stops before any write on one it cannot read as UTF-8 text or a directory it cannot list; tests show each, and that a pending `system.md` is not written first, failing under M39, M40, M41 and M42 in `scripts/mutation/rewrite-spec-links.py`, with M5 and M16 re-anchored and still killed (M38, which made the stop the mutant, removed); the fold and consolidate resumption rows, the fold reference and the result's `kept-tails` doc say what a mid-run write failure still loses; routed from 048's review, the stop chosen over a skip by its owner after the re-read found a skip left 051 AC6 and 052 AC7 untrue

## 34. Implement scenario: a link inside the target stays inside it

- [x] Implement the behavior described in `scenarios/a-link-inside-the-target-stays-inside-it.md`

- **Done when**: a link inside the target is rewritten relative to its own file and `mentions` is in the result, both covered by tests that fail without them, and `fold.md` step 14 reports `mentions`

## 35. Fix 048's third re-read round of ced8c7ca: a linked spec root, AC22's third case, and the symlink wording

- [ ] Found by 048's review, third independent re-read round (2026-10-29), of `ced8c7ca`; routed here by the owner because this spec's work was rewriting the same files. Each item is a correctness defect or a sentence that round wrote or left untrue.
- [ ] `read_spec_root`'s symbolic-link branch, added by `ced8c7ca`, also runs on the walk's depth-0 entry. walkdir follows a root link but still reports that entry as a symlink, so when `specs` is itself a link, `skipped` lists `specs` as "symbolic link, not followed" while the files under it are rewritten, and *examined and nothing pointed here* can never be reported. Exclude depth 0 (`entry.depth() > 0`, or `min_depth(1)`); add a test on a linked spec root asserting `skipped: []` with its files rewritten, and a mutation that removes the guard; the module doc and `read_spec_root`'s comment say the walk follows no symbolic link below the spec root
- [ ] AC22 counts "two cases the result names" and misses the exception `ced8c7ca` added: a symbolic link under the spec root in `skipped`, whose broken links `check-corpus-links` reports. Add it or state the cases without a count; spec.md's "re-points every in-repo link" sentences (§Motivation and the fold-back section) are untrue for any skipped file too
- [ ] The stated reason "the write would turn it into a copy" (module doc and `read_spec_root`'s comment) holds for a link to a file only: following a linked directory would write into the tree it names
- [ ] `framework/reference/fold.md` line 39's "one that cannot be read as UTF-8 text, or a directory that cannot be listed, stops the run" now reads as being about a symbolic link; say a markdown file there that cannot be read as UTF-8 text
- [ ] The code lists a link whose own name ends in `.md` or that resolves to a directory, while AC6, 052's AC7, the scenario `the-spec-root-is-read-before-the-first-write`, plan.md, data-model.md, the fold and consolidate steps, the result docs in `schema/primitives.rs` and the MCP description say "a symbolic link to a markdown file or a directory": say a symbolic link named `*.md`, or one to a directory

- **Done when**: A linked spec root yields an empty `skipped` and is rewritten, with a test that fails when the depth-0 guard is removed; AC22 and every sentence named above match the code; an independent re-read of the change reports no MUST or SHOULD
