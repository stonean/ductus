---
spec: 087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file
last-run: 2026-11-26T21:03:40Z
reviewed-against: b2e953c7c2a8ba8ab5e76942312a321761ec87f1
diff-base: 41d996f46dd1c9798c2aa72c8e8ccebdeeaf278c
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 40
scope: 79
unread:
  - .claude/commands/ductus/analyze.md
  - .claude/commands/ductus/plan.md
  - .claude/commands/ductus/prune.md
  - .claude/commands/ductus/target.md
  - .ductus/docs/agents/developing-ductus.md
  - .ductus/reference/analyze.md
  - .ductus/reference/implement.md
  - .ductus/reference/plan.md
  - .ductus/reference/review.md
  - .github/workflows/framework-checks.yml
  - .pi/prompts/ductus-analyze.md
  - .pi/prompts/ductus-plan.md
  - .pi/prompts/ductus-prune.md
  - .pi/prompts/ductus-target.md
  - AGENTS.md
  - framework/bootstrap/ductus-procedure.md
  - framework/bootstrap/ductus.md
  - framework/bootstrap/govern.md
  - framework/constitution.md
  - framework/reference/analyze.md
  - framework/reference/review.md
  - runtime/src/interpreter/analyze_tally.rs
  - runtime/src/interpreter/mod.rs
  - runtime/src/main.rs
  - runtime/src/parser/mod.rs
  - runtime/src/primitives/check_corpus_links.rs
  - runtime/src/primitives/check_owed_work.rs
  - runtime/src/primitives/mod.rs
  - runtime/src/schema/descriptions.rs
  - runtime/src/schema/primitives.rs
  - runtime/src/session.rs
  - scripts/analyze/combine.py
  - scripts/mutation/exec-arguments.py
  - scripts/mutation/owner-reviews-067.py
  - scripts/mutation/run.py
  - scripts/mutation/stderr-warnings-022.py
  - scripts/stale-literals.sh
  - scripts/tests/test-analyze-scripts.py
  - scripts/tests/test-stale-literals.sh
excluded: 140
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest: {}
criteria-judged: 21
criteria-met: 21
criteria-verdicts:
  - label: AC1
    digest: 498ffdde7307cf635c69abc767afdbfaac6dc18c4d49689ebcd135a7e88cbb59
  - label: AC2
    digest: 5b6a9de1fd3f1e79fd96733fb1e4bc4835ae3a2c92c9672a271fe6f19175aa54
  - label: AC3
    digest: 97d9c54395435cb3562a1e445384153709df87767cfee7dfb28c5f72b898e35f
  - label: AC4
    digest: e0a8183f9a5a3bd472a67b6156e2cc948fde251f2c79394d18d0d49a87870d4c
  - label: AC5
    digest: 5f97af57ab8a96da2b0854d9e0c6d4056bfd8923a6ae03e23005c048fd51001b
  - label: AC6
    digest: 66ba3597dfa97f6b9d01f7f8f27ef2a8688db51aa48246006eb08d4958a26805
  - label: AC7
    digest: 092fd5f5cd42a404302057fad9b0d5633b68461f0087851b41783cf1478c3d61
  - label: AC8
    digest: d1f7c3fa06ae78efed937066e71e554290ed032ef8ea76729ad4019d40aa6730
  - label: AC9
    digest: 5c0314ff3a5f88d0340bdd0f203b10c53a24a10a288fb76730870badccbe3420
  - label: AC10
    digest: 0a41b00309bf28e0c5ef17ec6861b0e155689d0ab636b7c4ef4c792b4a2c83a6
  - label: AC11
    digest: 5b46d631ca970181cf2a6cc47f3017634c143aac5b6315b142ecf1fddec7f965
  - label: AC12
    digest: a4a84b8becf7bc2dd5abcdc7ae6eeb7bdc528c56d8f66d1b01686dcd51551167
  - label: AC20
    digest: 9075404db432652c6ddb73f39727bc9d68828d9fbd7f5fb4af3df76989150525
  - label: AC13
    digest: ea91519d9b333de4925fd6a831ed2ec4e7ba42656e7eee5d67eeb949c7e71b7d
  - label: AC19
    digest: 8f85d09f67168b882a7e37cba7dfd1df95067e72b44857418f56fd794f3010fa
  - label: AC14
    digest: 3293a3c88d0cd2438bc1d79427b221970aa1a7df4d9910fb92e3c4b2906c0c68
  - label: AC15
    digest: af222e4056db4a725cf49f4e41f3f212a3f2eaef77f46920b0dbb8bbb268e67f
  - label: AC16
    digest: 330886d7726024a80f98a3be8c3788fc66bc7ade6f684e072e856beb71176929
  - label: AC17
    digest: 31cca040468203b1e53df1a7d5da333e6692fac08dc8d417f12fed63629719e0
  - label: AC18
    digest: 31c2e8c3bfbc269322b9be078c1c8a003aa0b0286bccfad645244b2cbd1f1df1
  - label: AC21
    digest: 37ac7ef41fae531e89fb5bbcf8287360ad2d8cb99acea84714a8d596ccdb6684
blocking: false
dispositions:
  fixed: 25
  deferred: 0
  waived: 4
  undispositioned: 0
decisions:
  - key: "convention: the shipped hook's capability probe passes a pre-087 runtime, which checks no part — `framework/bootstrap/hooks/ductus-pre-commit`"
    outcome: waived
    reason: The probe exists to turn the bare usage error a runtime without `--project-files` and `--staged` gives into a halt naming the fix, as its comment says; a pre-087 runtime gives no such error, and a hook and runtime from different releases is outside the lockstep versioning of the constitution's runtime boundary, under which `/ductus` acquires the pinned runtime with the hook.
    decided-at: 2026-11-26T21:01:22Z
    decided-by: rule
  - key: "other: a move leaves an inbound link written relative to a sibling directory dangling, since the referrer search matches the project-relative path literally — `runtime/src/primitives/prune_project/moves.rs`"
    outcome: waived
    reason: "A recorded decision, not a defect: `framework/commands/prune.md` (Move proposals) and 041's data model (*The search*) state the search is literal and a link written relative to another directory is not found, and the preview says so."
    decided-at: 2026-11-26T21:01:22Z
    decided-by: rule
  - key: "bug: a link from a parts directory to an ancestor (`.ductus/docs/system/up -> ..`) walks the sibling parts directories, so a real part of another project file is reported as a `nested-file` of this one — `runtime/src/primitives/project_parts.rs`"
    outcome: waived
    reason: The walk ends, and the result is an advisory finding naming a path that is literally a markdown file under a subdirectory of the parts directory, as the spec's Integrity bullet defines one; only a layout holding a link to an ancestor shows it, and confining the walk to the parts directory's own tree is a design change, not a mechanical fix.
    decided-at: 2026-11-26T21:01:22Z
    decided-by: rule
  - key: "convention: `is_project_artifact` in `runtime/src/primitives/mod.rs` keeps its own `under` closure, the same prefix test as `project_parts::in_dir` — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: Pre-existing code of spec 081's, outside 087's change, which only replaced its bare `.ductus` literal with `DUCTUS_DIR`.
    decided-at: 2026-11-26T21:01:22Z
    decided-by: rule
---

# Review — 087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file

## Summary

Window `41d996f4` to `b2e953c7` (diff-base the commit before 087 went `in-progress`; base-source `reopen`): 56 commits examined, 2 claimed by no spec; 140 paths excluded, listed under Excluded from scope; `docs/developing-ductus.md` deleted, moved by task 12 to `.ductus/docs/agents/developing-ductus.md`. Run from a clean worktree of `origin/main`. 79 scope entries: the 76 the scope resolved at the start, then `runtime/src/schema/paths.rs`, `scripts/mutation/reviews-follow-what-changed.py` and 063's `data-model.md` once the run's own fixes joined them. 40 examined, each read whole by a passes part. Unread, for the next review: 13 generated copies (`.claude/commands/ductus/`, `.pi/prompts/` and `.ductus/reference/` copies of `analyze`, `plan`, `prune`, `target`, `implement` and `review`, and `framework/bootstrap/govern.md`), whose sources were read whole, `scripts/gen-claude-commands.sh --check` reporting them in sync and `cmp` showing `govern.md` byte-identical to `ductus.md`; the 10 entries only task 12's sweep commit `2086cf6e` changed, each checked as its parent blob with the old path replaced by the new (the moved file differing only in its two relative links, corrected for its new directory); and 16 large or shared files read at 087's own hunks and the code around them: `AGENTS.md`, `framework/bootstrap/ductus.md`, `framework/bootstrap/ductus-procedure.md`, `framework/constitution.md`, `framework/reference/analyze.md`, `framework/reference/review.md`, `runtime/src/main.rs`, `runtime/src/primitives/mod.rs`, `runtime/src/primitives/check_corpus_links.rs`, `runtime/src/interpreter/mod.rs`, `runtime/src/interpreter/analyze_tally.rs`, `runtime/src/schema/descriptions.rs` (its three changed constants whole), `runtime/src/schema/primitives.rs`, `runtime/src/session.rs`, `scripts/mutation/owner-reviews-067.py` and `scripts/tests/test-analyze-scripts.py`. The read modes and the split, four passes parts by behaviour each running all five passes and two criteria parts, are the agent's recommendation, adopted by the operator, 2026-11-26. Loaded rule files: `configuration-cross.md`, `quality-cross.md`, `security-backend.md`, `security-cross.md` and `security-frontend.md` (231,962 bytes), each read whole by each passes part; no pass was skipped, and seven rule files are judged by analysis alone, by their headers. `specs/system.md` names no part, so the reuse pass read none. Findings, all fixed in the run: `BE-INPUT-011` (MUST) at `report_refused_sizes`, a staged part's committed name printed raw to stderr, now escaped by `display_text`; `CFG-CONST-001` (low confidence) at `candidates`, the bare `.ductus` literal in two modules, now `DUCTUS_DIR`; a low-confidence `QUAL-CLAIM-001` on the migration entry's prose was dropped as a finding, since its Verification flags code paths, and recorded as an observation. The passes, re-run over the whole fix diff with each changed function read whole, found nothing new. Observations: four contract breaks fixed (the nested walk's 2^32 paths under two links, a false `missing-part` under a directory that cannot be searched, the `.ductus/docs/constitution/` exemption the spec did not state, and the migration's silence on a project file it could not read), 21 other fixes, and four waived by the rule. Two of the fixes are the owner's decisions, 2026-11-26, each the agent's recommendation, adopted by the operator: the exemption narrowed to a project whose root holds `constitution.md`, and a non-markdown link leaving the project kept reported as not looked at, with AC11, AC12 and the spec's passages corrected to state both. Criteria: every criterion judged individually against the code its claim names, each driven on fixtures with the release binary rebuilt from the fixes, AC8 against a binary built from `41d996f4`; AC10 rests on `scripts/mutation/project-parts-087.py` run whole on the fixed tree in a clone carrying the fix diff (111 of 111 killed; 82 of 82 at `b2e953c7` before the fixes) and on nine hand mutants of `scripts/analyze/`, each failing its suite. What the criteria parts did not repeat: task 12's real-tree move (`2086cf6e`) behind AC14's real-tree half and AC21, read from its record; AC15's selection of pending entries by `/ductus`, since the entry's command was run by hand on four fixture adopters. Not tested: `nested_files`' branch for a directory whose canonical path cannot be taken, reachable only when a listed directory vanishes between two calls.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

2 of 56 commits in the window no spec claimed.

21 of 21 acceptance criteria judged met.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Deferred findings

*None.*

## Acceptance criteria

- AC1 — met: rebuilt binary on `fix-C1/ra` lists each part under its project-file with state and linked; `framework/commands/plan.md` (named when presented), `framework/reference/plan.md` Always read and `framework/commands/analyze.md` step 15 direct reading every measured part and naming them
- AC2 — met: chmod-000 part and outward-symlink part list state unreadable, no bytes, and sit in skipped as artifact-unreadable (`fix-C1/ra`); `framework/reference/analyze.md` Plan consistency records the judgment unexamined for that part
- AC3 — met: `framework/reference/implement.md` step 4 says to follow `specs/system.md` and the parts its pointers name for the task and say which parts were read; `framework/reference/review.md` Reuse pass cross-references `system.md` and the parts its pointers name and names the parts read in the Summary (58c226dd); the uncommitted fix also makes the Quality pass check contracts against `specs/errors.md` and the parts its pointers name, consistent.
- AC4 — met: `fix-C1/ra` at 100 bytes: a 101-byte part is oversized project-part with path, bytes, pages, split-part fix and the 100-byte message, a 100-byte part is not reported (mutant M108 kills the boundary)
- AC5 — met: git fixture `fix-C1/rb` staged: new 101-byte and 200->201 parts exit 1 naming path, size, HEAD size, limit and fix; 300->150, deleted, unchanged and new 100-byte parts exit 0
- AC6 — met: `fix-C1/ra` part-findings lists missing-part `gone.md` and unlinked-part `orphan.md`; `analyze.md` step 17 records each Advisory project-parts, `analyze_tally.rs` counts part-findings-count advisory
- AC7 — met: Re-driven on the fixed tree's rebuilt binary (`fix-C2/r2/p1`): prune-project preview on a fixture with `system.md` index and part `.ductus/docs/system/network.md` lists the part in part-files (part-of `specs/system.md`) and walks its entries after the four files, and an apply --remove on a part entry wrote the part; `framework/commands/prune.md` 'Moves to its home' sends architecture to `system.md` or the part it belongs in.
- AC8 — met: pre-087 vs rebuilt binary on `fix-C1/rc` (no parts): feature-only byte-identical; project-files and prune-project keep every earlier key and value, new lists empty (move-candidates non-empty only when a project file links a `docs/` markdown file)
- AC9 — met: `framework/constitution.md` §spec-phase tree has '`.ductus/docs/` {file}`/` # A project file's parts; the file is their index', canonical-sources table has the 087 row, and the Shared-knowledge routing says 'a part of it' and '`.ductus/docs/agents/`'.
- AC10 — met: `scripts/mutation/run.py scripts/mutation/project-parts-087.py` run whole on the fixed tree in a clone: 111 of 111 mutants killed, naming each of the 79 tests 087's 20 commits and the review's fixes added; `scripts/tests/test-analyze-scripts.py` cases AR and Z fail under each of nine hand mutants of `scripts/analyze/`
- AC11 — met: `fix-C1/ra`: every `.md` directly under the four directories is a part linked or not, `.txt` files are unreported, and a non-markdown symlink leaving the project (file or directory) is in skipped as artifact-unreadable, as the corrected text says
- AC12 — met: `fix-C1/ra`: nested-file `system/deep/inner.md` and not-a-parts-directory bogus listed; `constitution/` is a finding with no root `constitution.md` and exempt once the root holds one
- AC20 — met: `fix-C1/ra`: `.ductus/docs/stray.md` listed as not-a-part in part-findings and recorded advisory by analyze step 17
- AC13 — met: `framework/commands/prune.md` names '`.ductus/docs/agents/`' as a procedure's home; `specs/075-agents-md-holds-what-every-session-needs/spec.md` routing row reads 'a part under `.ductus/docs/agents/`' (diff vs 41d996f4 replaced `docs/`); constitution routing line names `.ductus/docs/agents/`.
- AC19 — met: `framework/bootstrap/ductus.md` ductus:memory-routing block (Memory routing step 1 line) reads 'a procedure in a part under `.ductus/docs/agents/` or a skill'; 41d996f4 read 'in `docs/` or a skill'.
- AC14 — met: Re-driven on the fixed tree's rebuilt binary (`fix-C2/r2/p1`, source mode 0755 kept as 755 on the moved file): preview of move `docs/guide.md` to `.ductus/docs/agents/guide.md` wrote nothing (git status 0) and listed referrers `AGENTS.md`, `README.md`, `specs/001-x/spec.md` via move-search `ran/8` files; apply with two confirmed referrers re-pointed both, rewrote the moved file's relative links, removed `docs/guide.md`, left the unconfirmed spec untouched; `prune.md` shows the search, confirms via gate-confirm, and says declined writes nothing; the real-tree run 2086cf6e is recorded and I re-checked its result (git grep finds no old path outside 087's own move records).
- AC15 — met: Re-driven on the fixed tree with four adopters (`fix-C2/r2`) via the procedure's command (prune-project preview), procedure read as it now stands: a symlinked-out `AGENTS.md` gives files state outside-project and zero candidates, which the new step `3/4` reports as a not-checked line, never none; `AGENTS.md` linking `docs/guide.md` gives move-candidates `docs/guide.md` linked-from `AGENTS.md` and the tree is byte-identical after; no `docs/` link gives only a `non-docs/` candidate which step 3 filters out so nothing is reported; the pre-087 binary's result has no move-candidates-count so the procedure's not-checked line applies; `framework/migrations/report-project-file-links-into-docs.md` and its `framework/migrations.toml` entry write nothing.
- AC16 — met: resolve-constitutions on `fix-C1/rd`: `at.md` 2000 bytes not over, `over.md` 2001 bytes over-limit with a warning naming the repo, a not-checked-out entry lists no parts and `target.md` step 4 reports it not measured
- AC17 — met: analyze uses the unmeasured resolve-constitutions (--unmeasured result has no parts) and records no part finding; exec test fixture now has a root `constitution.md` (M82)
- AC18 — met: Each 087 commit that ships a command, runtime or bootstrap surface (e984e5dc, 9f46bdfb, 2551f5d4, 5d87f512, 58c226dd, 5935e9cf, e7a71f5a, 06221d55, e672d428, fad793ba, 1ffaf951) records claude-style and pi impact, and each claim matches its file list and the Layout-derived rows of `framework/bootstrap/ductus.md`; commits touching no shipped surface (11941009, 52ea4076, 2086cf6e) record none.
- AC21 — met: `framework/commands/prune.md` Project files section orders 'Preview', 'Move proposals come first', then 'Judge each entry'; it says the operator may end the run after the moves and the Report names each entry not judged as not examined, never as kept; the real-tree run 2086cf6e recorded ending after the move (not repeatable here, text verified).

## Observations

- security: the move's referrer search had no test that a tracked link out of the project is named and never read (test M92) — `runtime/src/primitives/prune_project.rs` — **fixed** (decided by the rule)
- bug: `nested_files` bounded its walk by depth alone (`MAX_NESTED_DEPTH` 32), so two links to `.` in a parts directory made `check-artifact-size --project-files` and `prune-project` walk 2^32 paths and never return; it now enters each directory once by its canonical path (tests M85, M86) — `runtime/src/primitives/project_parts.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: `missing_parts` tested a link with `.exists()`, so a parts directory that cannot be searched reported an existing part as a `missing-part`, against the spec's Integrity bullet (a link whose file does not exist); it now judges missing only on NotFound (test M89) — `runtime/src/primitives/project_parts.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: `.ductus/docs/constitution/` was exempt from `not-a-parts-directory` in every project, which AC12, the spec's Integrity bullet and its stray-directory Resolved Question did not state, and in a project that is no shared constitution's repository its content went unmeasured with no signal; the exemption is narrowed to a project whose root holds `constitution.md`, and AC12 and the spec state it (the owner's decision; tests M87, M88) — `runtime/src/primitives/project_parts.rs` (breaks the spec's contract) — **fixed**
- bug: the migration entry reported nothing when a project file or part could not be read whole, was a link leaving the project or held a comment it never closes, the same silence as a project with no link into `docs/`, against `AGENTS.md`'s design principle that a check that cannot run never looks like one that passed; it now reports a not-checked line for each, `unexamined-tail` included — `framework/migrations/report-project-file-links-into-docs.md` (breaks the spec's contract) — **fixed** (decided by the rule)
- other: AC11 said a non-markdown file in a parts directory is neither measured nor reported, while a non-markdown link leaving the project is reported as not looked at, since whether it is a file is never probed; the code stands, and AC11, the spec's Edge Case and the plan's *A part* bullet state the exception (the owner's decision; tests M90, M91) — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md` — **fixed**
- convention: four tests used `crate::primitives::outward` without the `#[cfg(unix)]` its other users carry — `runtime/src/primitives/project_parts.rs` — **fixed** (decided by the rule)
- convention: the doc comments on `CheckArtifactSizeResult` and its `lists_file` omitted `refused`, which the staged result moves to the lists file — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- convention: the refusal predicate was written twice in `check_artifact_size.rs`, and parts directories were paired with project files by position; it is one `is_refused`, and the pairing is by kind — `runtime/src/primitives/check_artifact_size.rs` — **fixed** (decided by the rule)
- convention: `under` in `moves.rs` repeated `project_parts::in_dir`, now shared — `runtime/src/primitives/prune_project/moves.rs` — **fixed** (decided by the rule)
- convention: the `ProjectFile` doc comment said one of the four project files, though `prune-project` returns one per part too — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- convention: `file_mode` and `restore_mode` were cfg pairs around a bare mode; the move now captures and restores the source's `Permissions`, and says why `mirror_source_mode` is not used — `runtime/src/primitives/prune_project/moves.rs` — **fixed** (decided by the rule)
- convention: `scalar_count` was inserted between `rule_tier`'s doc line and `rule_tier` — `runtime/src/interpreter/analyze_tally.rs` — **fixed** (decided by the rule)
- convention: `write_move` re-imported `std::fmt::Write`, which the file already imports — `runtime/tests/result_sizes.rs` — **fixed** (decided by the rule)
- convention: `a_checkout_without_parts_lists_none_and_an_unmeasured_call_reads_none`'s doc claimed a not-checked-out registry the test never built; it now builds it (M93) — `runtime/src/primitives/resolve_constitutions.rs` — **fixed** (decided by the rule)
- convention: the quality pass checked contracts against `specs/errors.md` alone, while 087's Reading bullet has `/review` read the parts a pointer names — `framework/reference/review.md` — **fixed** (decided by the rule)
- convention: `/plan`'s Scope Boundaries directed reading the parts `check-artifact-size` lists but not naming them, which AC1 asks of it — `framework/commands/plan.md` — **fixed** (decided by the rule)
- convention: the `/prune` entry named `docs/` among a plan section's homes, where the routing now names a part under `.ductus/docs/agents/` — `docs/slash-commands.md` — **fixed** (decided by the rule)
- convention: the `/prune` bullet omitted the parts `--project` walks and the move it proposes — `README.md` — **fixed** (decided by the rule)
- convention: the Cost section cited `4e5ee251`, a commit no ref reaches, as where its figures were measured — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md` — **fixed** (decided by the rule)
- convention: task 14 cited `f8911594`, which no ref reaches, as task 10's commit (`e672d428`) — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/tasks.md` — **fixed** (decided by the rule)
- convention: task 1 named `link_destination` as made `pub(crate)` and task 3 said `main.rs` reads the lists file, neither of which the code does — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/tasks.md` — **fixed** (decided by the rule)
- convention: the plan's Affected Files omitted 18 paths 087's commits changed, its first Technical Decisions entry recorded a wait on 084 that ended, and a Trade-offs entry priced the added call with a figure taken over a different population — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md` — **fixed** (decided by the rule)
- convention: the `CONSTITUTION_PARTS_DIR` doc comment attributed its exemption to the owner, where the plan records the agent's recommendation, adopted by the operator — `runtime/src/primitives/project_parts.rs` — **fixed** (decided by the rule)
- convention: 15 tests 087 added had no mutant that removes their behaviour, among them AC2's unreadable part, AC4's threshold, AC5's unchanged and deleted parts and AC8's no-parts project, and `result_sizes.rs` tested its parts and move cases only in its ignored measurement; mutants M94 to M111 and one test cover them, and `resolve-constitutions-size.py` M10, equivalent since `bound` halts itself, is a two-anchor mutant (AC10) — `scripts/mutation/project-parts-087.py` — **fixed** (decided by the rule)
- convention: the shipped hook's capability probe passes a pre-087 runtime, which checks no part — `framework/bootstrap/hooks/ductus-pre-commit` — **waived**: The probe exists to turn the bare usage error a runtime without `--project-files` and `--staged` gives into a halt naming the fix, as its comment says; a pre-087 runtime gives no such error, and a hook and runtime from different releases is outside the lockstep versioning of the constitution's runtime boundary, under which `/ductus` acquires the pinned runtime with the hook. (decided by the rule)
- other: a move leaves an inbound link written relative to a sibling directory dangling, since the referrer search matches the project-relative path literally — `runtime/src/primitives/prune_project/moves.rs` — **waived**: A recorded decision, not a defect: `framework/commands/prune.md` (Move proposals) and 041's data model (*The search*) state the search is literal and a link written relative to another directory is not found, and the preview says so. (decided by the rule)
- bug: a link from a parts directory to an ancestor (`.ductus/docs/system/up -> ..`) walks the sibling parts directories, so a real part of another project file is reported as a `nested-file` of this one — `runtime/src/primitives/project_parts.rs` — **waived**: The walk ends, and the result is an advisory finding naming a path that is literally a markdown file under a subdirectory of the parts directory, as the spec's Integrity bullet defines one; only a layout holding a link to an ancestor shows it, and confining the walk to the parts directory's own tree is a design change, not a mechanical fix. (decided by the rule)
- convention: `is_project_artifact` in `runtime/src/primitives/mod.rs` keeps its own `under` closure, the same prefix test as `project_parts::in_dir` — `runtime/src/primitives/mod.rs` — **waived**: Pre-existing code of spec 081's, outside 087's change, which only replaced its bare `.ductus` literal with `DUCTUS_DIR`. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/configure.md`
- `.claude/commands/ductus/review.md`
- `.githooks/pre-commit`
- `.pi/prompts/ductus-review.md`
- `framework/bootstrap/configure/claude.md`
- `framework/commands/review.md`
- `framework/runtime-tools.txt`
- `framework/templates/ci/adopter-generators.yml`
- `runtime/CHANGELOG.md`
- `runtime/src/mcp/server.rs`
- `runtime/src/primitives/bounded_lists.rs`
- `runtime/src/primitives/check_audit_records.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/schema/registry.rs`
- `runtime/tests/check_audit_records_cli.rs`
- `runtime/tests/common/mod.rs`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/mechanical_sweep_parity.rs`
- `scripts/mutation/audit-records-085.py`
- `scripts/mutation/rule-judgments-088.py`
- `scripts/tests/test-audit-record-gate.sh`
- `scripts/tests/test-pre-commit-staging.sh`
- `specs/000-slash-commands/review.md`
- `specs/003-bootstrap-automation/review.md`
- `specs/003-bootstrap-automation/tasks.md`
- `specs/009-scenario-targeting/review.md`
- `specs/012-multi-agent-govern/review.md`
- `specs/012-multi-agent-govern/tasks.md`
- `specs/014-reclarify-backedge/review.md`
- `specs/016-cross-cutting-rules/review.md`
- `specs/017-derive-dont-ask/analysis.md`
- `specs/017-derive-dont-ask/plan.md`
- `specs/017-derive-dont-ask/review.md`
- `specs/017-derive-dont-ask/scenarios/a-path-list-commit-leaves-the-shared-index-current.md`
- `specs/017-derive-dont-ask/spec.md`
- `specs/017-derive-dont-ask/tasks.md`
- `specs/018-adopter-owned-pre-commit/review.md`
- `specs/019-config-decisions/review.md`
- `specs/020-code-review/analysis.md`
- `specs/020-code-review/spec.md`
- `specs/021-runtime-boundary/review.md`
- `specs/022-deterministic-runtime/plan.md`
- `specs/022-deterministic-runtime/scenarios/coverage-expansion-primitives.md`
- `specs/022-deterministic-runtime/scenarios/resolve-references-cli-exec-wiring.md`
- `specs/022-deterministic-runtime/scenarios/scaffolding-primitives.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/023-govern-refinement/review.md`
- `specs/024-rule-loader/review.md`
- `specs/025-rule-opt-out/review.md`
- `specs/026-framework-self-audit/plan.md`
- `specs/027-bootstrap-migration-registry/review.md`
- `specs/027-bootstrap-migration-registry/tasks.md`
- `specs/029-bootstrap-runtime-autowire/review.md`
- `specs/029-bootstrap-runtime-autowire/spec.md`
- `specs/033-rule-surface-setting/review.md`
- `specs/034-performance-backend-rules/review.md`
- `specs/035-groom-session-target/review.md`
- `specs/036-quality-cross-rules/review.md`
- `specs/037-observability-backend-rules/review.md`
- `specs/038-concurrency-backend-rules/review.md`
- `specs/039-reliability-backend-rules/review.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/plan.md`
- `specs/041-task-pruning/scenarios/prune-walks-the-project-files.md`
- `specs/041-task-pruning/spec.md`
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/review.md`
- `specs/043-workflows-sunset/review.md`
- `specs/045-decision-state-drift-detection/plan.md`
- `specs/045-decision-state-drift-detection/review.md`
- `specs/047-analyze-findings-durability/review.md`
- `specs/049-rename-govern-to-ductus/review.md`
- `specs/050-constitution/plan.md`
- `specs/050-constitution/spec.md`
- `specs/052-spec-supersession-and-consolidation/review.md`
- `specs/054-remove-supersession/review.md`
- `specs/055-shared-constitution/data-model.md`
- `specs/055-shared-constitution/review.md`
- `specs/055-shared-constitution/spec.md`
- `specs/055-shared-constitution/tasks.md`
- `specs/056-bootstrap-archive-boundary-split/review.md`
- `specs/057-analyze-artifact-and-record-relocation/spec.md`
- `specs/059-project-in-a-repository-subdirectory/review.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/review.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/tasks.md`
- `specs/061-updates-track-the-latest-release-tag/review.md`
- `specs/062-concurrent-session-targets/review.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/plan.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/project-files-are-measured.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`
- `specs/066-audit-records-and-the-completion-gate/analysis.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/plan.md`
- `specs/066-audit-records-and-the-completion-gate/review.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/066-audit-records-and-the-completion-gate/tasks.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/review.md`
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/068-api-backend-rules/review.md`
- `specs/069-accessibility-frontend-rules/review.md`
- `specs/070-performance-frontend-rules/review.md`
- `specs/071-command-reference-split/plan.md`
- `specs/071-command-reference-split/review.md`
- `specs/071-command-reference-split/spec.md`
- `specs/073-decided-rule-departures-are-recorded/plan.md`
- `specs/073-decided-rule-departures-are-recorded/review.md`
- `specs/075-agents-md-holds-what-every-session-needs/review.md`
- `specs/075-agents-md-holds-what-every-session-needs/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/review.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/review.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/080-deterministic-runtime-efficiency/research.md`
- `specs/080-deterministic-runtime-efficiency/review.md`
- `specs/080-deterministic-runtime-efficiency/spec.md`
- `specs/081-reviews-and-checks-follow-what-changed/plan.md`
- `specs/081-reviews-and-checks-follow-what-changed/review.md`
- `specs/082-rule-assessments-converge-across-runs/plan.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/review.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/plan.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/tasks.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/analysis.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/data-model.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/plan.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/review.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/spec.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/tasks.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/plan.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/tasks.md`
- `specs/088-rule-judgments-cost-only-what-changed/data-model.md`
- `specs/088-rule-judgments-cost-only-what-changed/plan.md`
- `specs/088-rule-judgments-cost-only-what-changed/spec.md`
- `specs/088-rule-judgments-cost-only-what-changed/tasks.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/spec.md`
- `specs/inbox.md`

## Unexamined governance

*None.*
