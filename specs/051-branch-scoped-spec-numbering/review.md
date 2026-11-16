---
spec: 051-branch-scoped-spec-numbering
last-run: 2026-11-16T03:24:18Z
reviewed-against: be9b0405310d148cf62e1554fff3883897021130
diff-base: 7ab30b2736a3d1a2f4f1f4ea7e7a2ba6f8badeee
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 13
scope: 24
unread:
  - .claude/commands/ductus/consolidate.md
  - .claude/commands/ductus/fold.md
  - .ductus/reference/fold.md
  - .pi/prompts/ductus-consolidate.md
  - .pi/prompts/ductus-fold.md
  - runtime/CHANGELOG.md
  - runtime/src/primitives/rewrite_spec_links.rs
  - runtime/src/schema/descriptions.rs
  - runtime/src/schema/primitives.rs
  - runtime/tests/mcp.rs
  - scripts/mutation/rewrite-spec-links.py
excluded: 62
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 8e8baa256ea782bfb50a0620c598997cec119266cb41a5f8a0cd233879d6427a
  scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md: 015b0ba736e27e91d0d86b37c65e6f7d0d947b5773cf6495313a8ff337b93a47
  scenarios/a-fold-target-is-held-to-the-name-grammar.md: 6bb12cc93d494ce3de1b50243e07fbcaf8c654713a3d6a72c71e97412bdb2f04
  scenarios/a-link-inside-the-target-stays-inside-it.md: a367c3a7c1e260f98b27eb73a87958625422a4e458b758399fd5fa526eb2e1a4
  scenarios/a-pending-fold-never-holds-done.md: dc5c2daa3fd78c7251edf856c21f5d36737e3725d91c808231e112f84502e8b5
  scenarios/fold-target-checked-before-the-rewrite.md: 40bfa2011fb9af497ae85dac6f23f2ad8719dddcc8c5f5654dda7560aade7f32
  scenarios/rewrites-preserve-line-endings.md: 1ab09161757ade0d1f4a3959f574e54d68dab19f275ec84b345b8d16a75ccd26
  scenarios/the-numbering-grammar-reaches-every-surface.md: d4f11334754c65e3cf73346d224aaf8d782f5385a266a2bf22d3339a1079089a
  scenarios/the-spec-root-is-read-before-the-first-write.md: ccf10584e584b2f0f83808e7e1413a30cd98c1b5691a4c464da9432b0fd53ff9
blocking: false
dispositions:
  fixed: 5
  deferred: 0
  waived: 5
  undispositioned: 0
decisions:
  - key: "convention: a-status-commit-holds-only-the-transition's Behavior reads as requiring the bullet to say a transition staged alone is lost — `specs/050-constitution/scenarios/a-status-commit-holds-only-the-transition.md`"
    outcome: waived
    reason: the Behavior requires the bullet to state its condition, that the spec file holds nothing but the transition, and its reason, that the pre-commit hook restages the file whole; the bullet states both, and the loss of a transition staged alone is the scenario's own account of the incident
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: claude-md-import-syntax's shared-import bullet contrasts its rewrite with a removal the procedure no longer performs — `framework/migrations/claude-md-import-syntax.md`"
    outcome: waived
    reason: the bullet still guards against de-duplicating an import the adopter added by hand, which removal would drop if the registration were withdrawn; its sense does not depend on the constitution line
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: 017's plan and 072's Affected Files still name the old import form — `specs/017-derive-dont-ask/plan.md`"
    outcome: waived
    reason: each records that spec's own edit at the time, a decision record, not a claim about the tree today
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: a CLAUDE.md symlinked to AGENTS.md becomes a copy when constitution-import writes it by temporary file and rename — `framework/migrations/constitution-import.md`"
    outcome: waived
    reason: every bootstrap write of CLAUDE.md already does this, merge-managed-block's managed-block render on every run included, so the migration adds no new case; a symlinked CLAUDE.md is not a layout the framework supports
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: 022's constitution-excerpts-as-skill-resources says every CLAUDE.md carries the constitution import — `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md`"
    outcome: waived
    reason: this change makes the premise true again, the template's third line being @.ductus/constitution.md; 022 is another session's and owes no edit
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: the constitution line naming plan.md carries § references resolve-anchor classes as qualified and does not judge — `framework/constitution.md`"
    outcome: waived
    reason: both targets, §brownfield-inbox and §drift-prevention, carry markers (checked by hand), and citing a subsection's marked section is the constitution's own anchor rule
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: constitution-import accepts a four-space-indented line, misses a match behind a leading byte-order mark, and reports the @import form on a file unpinned after claude-md-import-syntax was skipped — `framework/migrations/constitution-import.md`"
    outcome: waived
    reason: "contrived inputs: claude-md-import-syntax treats indentation the same way, the template writes no byte-order mark, and an operator who unpins a file has the earlier migration's pinned warning in hand"
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "convention: a #fragment containing a dot is cut at the dot on a fold into a scenario — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: waived
    reason: GitHub's heading slugs carry no dots, so such a fragment names no heading a link in this corpus can reach; a contrived input
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: no numbered step of the bootstrap names the two managed-block merges — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: Instructions step 4 lists what apply-manifest replaces, not every Shared Files step, and two real adopter bootstraps on this text wrote both blocks from Shared Files
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: whether Auggie resolves CLAUDE.md's @AGENTS.md import, and so reads the routing block, is unverified — `framework/templates/project/claude-md.md`"
    outcome: waived
    reason: the rule's canonical statement is the constitution's drift-prevention section, which target step 4 loads whenever an import did not, so an Auggie session holds it either way
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "bug: a run the Frontmatter Migration precheck halts after generator-primitives leaves a pre-0.30 hook calling the scripts that migration deleted, so the commit the refusal asks for fails — `framework/bootstrap/ductus-procedure.md`"
    outcome: deferred
    target: specs/013-text-first-artifacts/scenarios/the-precheck-ignores-what-the-same-run-wrote.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: generator-primitives and audit-record-relocate state no report line for their no-op cases — `framework/migrations/generator-primitives.md`"
    outcome: waived
    reason: an observation about the migration machinery outside this window; the host's own wording reported each outcome truthfully
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the Closing restart gives no single form for Pi wired in the run beside Claude already registered — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: the per-agent forms compose into an accurate notice, as the subject's run shows, so no host reads it wrong
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the Frontmatter Migration's specs/**/spec.md walk reports specs/templates/spec.md as already frontmatter on every run — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: the template has frontmatter, so the walk skips it with a report line and writes nothing
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: fetch-archive sends no User-Agent — `runtime/src/primitives/fetch_archive.rs`"
    outcome: deferred
    target: specs/048-govern-acquired-runtime/tasks.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: a .githooks linked into a host configuration directory inside the project is not judged by host_config::check_destination — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: waived
    reason: a contrived layout; the write stays inside the project, and the referrer list is fixed to five files none of which is a host's configuration
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: fold's resumption row for step 11 says a re-run rewrites nothing, which a fragment running into a second token contradicts — `framework/commands/fold.md`"
    outcome: waived
    reason: "a contrived input: a fragment that is itself a second spec-root path, which the rewrite now leaves for the orphan check to report"
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: a root-absolute /specs/ link is neither re-pointed nor reported — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: deferred
    target: specs/051-branch-scoped-spec-numbering/tasks.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: an archive entry whose only mode bits are group or other write lands at the umask default — `runtime/src/primitives/extract_archive.rs`"
    outcome: waived
    reason: "a contrived entry: no archive the framework ships carries a file with neither owner read nor owner write"
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: legacy-constitutions-region was edited in place after a --ref=main push, so an adopter who ran the earlier text keeps its trailing blank lines — `framework/migrations/legacy-constitutions-region.md`"
    outcome: waived
    reason: the migration is unreleased, reached only --ref=main adopters for the hours between f056b209 and 47e68233, and the residue is trailing blank lines after a removed comment region, which change no behavior
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "stale: 062 task 23's checked item still gives the fixed-interval rationale for the lock poll — `specs/062-concurrent-session-targets/tasks.md`"
    outcome: waived
    reason: a ticked task records the decision as it was taken, task 28 records its reversal, and tasks.md is ephemeral work tracking that prune may drop
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test: installed-file-modes M5's kill depends on ctime resolution finer than 50 ms — `scripts/mutation/installed-file-modes.py`"
    outcome: waived
    reason: disclosed in the list's docstring; APFS and ext4 stamp ctime in nanoseconds, and the test itself passes on any file system
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: an @/specs/x import in CLAUDE.md is read as project-relative, where Claude Code reads an absolute path — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: an absolute import of a top-level /specs directory is a contrived input
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: a kept-tails entry gives the reference from the spec root, without a prefix kept on the line — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: waived
    reason: its path and line locate the reference, and the check reports the same span from the root
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: a ./ prefix in the hook or a root markdown file assumes the project root is the work-tree root, which a project in a repository subdirectory breaks — `specs/067-analyze-and-audit-checks/data-model.md`"
    outcome: waived
    reason: a ./ resolves as the bare paths in those files already do, the assumption every bare reference carries, not one task 30 adds
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test: the Family 19 parity tests run git in their fixtures under a hook's inherited GIT_DIR, so a commit from a linked worktree fails four of them — `runtime/tests/mechanical_sweep_parity.rs`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: a leading / was read from the project root in a project below its repository's root, where a host resolves it from the repository's root — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: the framework reads a root-absolute path from the project root, as check-corpus-links reads a root-absolute link (spec 059); reading it from the repository's root here alone split the fold's link and token passes, so 3fc5755a's gate was reverted
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: in a project below its repository's root, /proj/specs/x, the project's own path spelled from the repository's root, is neither checked nor re-pointed — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: a project's README spelling its own spec from the repository's root is a contrived input, and the project-root reading names it nothing
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: in a project below its repository's root the hook runs from the repository's root, so its ./ and bare paths resolve there — `specs/067-analyze-and-audit-checks/data-model.md`"
    outcome: waived
    reason: the same assumption every bare reference in the hook carries, which task 30 does not add; prefixed and bare agree
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: bare-rule cases outside task 30 — ~/\".ductus/x\", \"$X\"specs/x and ${X}specs/x in the hook, ${X:-./specs/x}, *specs/x*, and @/specs/x in CLAUDE.md — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: the bare rule predates task 30 and prefixed and bare agree on each; every input is contrived
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: the leading-/ exception also covers (a)/specs/x, x~/specs/x and a quote after a letter — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: contrived text; requiring $( before ) would add a parser for no real input
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test gap: no agreement test puts a path at column 0 in system.md, so a mutant refusing a line-start path only there survives, in the check's tests and the rewrite's — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: the scanner has no per-referrer branch, so a defect of that shape is contrived; the binary reports a bare, ../ and / path at column 0 in system.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test gap: the subdirectory test asserts that a spec only the repository's root holds is reported, not that one only the project holds is not — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: the realistic regression, reading existence from the repository's root, is killed by M46; skipping only a path present at both roots is contrived, and the binary does not report the project-only spec
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "stale: two ticked items in 017's tasks.md name a rule's Rationale, which 70bb4af5 removed from the format — `specs/017-derive-dont-ask/tasks.md`"
    outcome: waived
    reason: tasks.md is ephemeral work tracking whose ticked items record what was built then (§tasks-phase); the format's canonical statement, 017's data-model.md, is current
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: git_command leaves GIT_CONFIG_PARAMETERS and GIT_CONFIG_COUNT, so an outer git -c commit passes its config to a fixture — `runtime/tests/common/mod.rs`"
    outcome: waived
    reason: an outer git -c commit whose config changes a fixture's outcome is contrived; the helper clears what a hook exports to locate a repository, the defect 066 task 8 records, and the fixture already blanks global and system config
    decided-at: 2026-10-30T01:31:06Z
    decided-by: andy@stone.dev
  - key: "errors: rewrite-spec-links checks the path escape before the name grammar, so a `to` failing both is refused as InvalidPath — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: waived
    reason: contrived input; refused before any write either way, and the error names the more serious defect, the escape
    decided-at: 2026-10-30T01:31:06Z
    decided-by: andy@stone.dev
  - key: other — rewrite-spec-links exits 0 on the CLI when refused is non-empty, an unfinished sweep, while derive-dependencies and derive-references exit non-zero for the same BE-TXN-002 refusal (main.rs derive_*_blocks). The data model states the design ('raises no error for one'), the host commands read refused and the only CLI callers are tests, so nothing in the repository runs the CLI as a gate. Recorded so the difference is a decision, not an accident. — `runtime/src/main.rs:1176`
    outcome: waived
    reason: the data model's The write guard states that rewrite-spec-links reports a refusal in refused and raises no error for one; the callers that act on it are the fold and consolidate commands, which read the result, and no script or workflow runs the CLI as a gate
    decided-at: 2026-11-16T03:24:18Z
    decided-by: rule
  - key: other — the CHANGELOG entry and the data model give two new result fields as the reason for BREAKING under BE-COMPAT-001, but the Statement permits new response fields; what is breaking is the new changed-since-read error where invalidate-review used to write, and the narrowed meaning of an empty rewritten (it now needs refused empty too). The BREAKING label is right and matches 041, 057, 066 and 067's entries for the same class; the stated reason over-lists. — `runtime/CHANGELOG.md:36`
    outcome: waived
    reason: the classification BE-COMPAT-001 asks for is given and correct as breaking; only the list of reasons in the entry is loose, and the same wording is the precedent the prune and relocation entries set for this class
    decided-at: 2026-11-16T03:24:18Z
    decided-by: rule
  - key: other — PendingFile::text maps an absent snapshot to a NotFound error that no constructor can reach, since snapshot_of returns an error for an absent file before a PendingFile exists; the arm is defensive, not live logic. — `runtime/src/primitives/rewrite_spec_links.rs:264`
    outcome: waived
    reason: the crate denies unwrap and expect outside tests, so a Result return is the lint-clean way to read the snapshot's optional text; removing the arm means holding the text a second time beside the snapshot
    decided-at: 2026-11-16T03:24:18Z
    decided-by: rule
  - key: "other — invalidate-review's claim that the snapshot is taken before anything reads review.md (the plan, the module doc and plan()'s doc comment) has no test or mutant: moving ReadSnapshot::read below read_recorded_waivers would fail none of the new tests, because both change the file only after plan() returns. The seam to change the file between two reads inside plan() does not exist, and the guard's own compare semantics are proved in the helper's tests. — `runtime/src/primitives/invalidate_review.rs:105`"
    outcome: waived
    reason: testing the position needs a hook between two reads inside plan(), which the code does not carry and the pipeline's other guarded primitives do not carry either; the helper's tests prove the compare, and the position is one line a reader sees
    decided-at: 2026-11-16T03:24:18Z
    decided-by: rule
  - key: other — fold step 10 states no handling for the changed-since-read error in the step itself; it lives only in the resumption row of the table at the top of the command, in the primitive's description and in the data model, while step 11 carries its stop-and-run-again clause in its own text. A host that reads step 10 alone meets the error with the handling one table away. — `framework/commands/fold.md:71`
    outcome: waived
    reason: the command's own preamble says an interrupted fold is completed by re-running and the table is where each step's re-run behavior is stated; restating the row in the step is the copy the drift rules ask not to keep, and the primitive's error text already tells the caller to re-run
    decided-at: 2026-11-16T03:24:18Z
    decided-by: rule
---

# Review — 051-branch-scoped-spec-numbering

## Summary

051's review of task 37 (`b42ac14e`), the window from `7ab30b27`, the reopen, to HEAD, with task 37's text and reopen (`b88a8404`) and the plan and data-model citation commit `fb319f2d`. One agent ran all five passes and read all nine loaded backend and cross rule files whole. It read 13 of the 24 scope entries whole: fold's and consolidate's command sources and fold's reference, `invalidate_review.rs`, the guard's mutation list, and 051's spec, plan, data model, tasks and the four scenarios in the scope; and the other four scenarios, outside it. The five generated mirrors were compared with their sources by `diff` and are counted unread; `rewrite_spec_links.rs`, `descriptions.rs`, `primitives.rs`, the CHANGELOG and `rewrite-spec-links.py` were read at the window's hunks with every touched function, type, constant and new test whole; `runtime/tests/mcp.rs` entered the scope with the fix round and was read at its row by the round's re-read. No rule finding. Ten observations: one broken contract, fixed in the run (the data model said a refused file's mentions were read from the file as it stands, while the sweep keeps the text it read; the data model and the code comment now say so), and four chores fixed (`be9b0405`): fold step 11, consolidate step 4, their resumption rows, fold's reference, the data model, the description and the result's doc comments stop on `refused-count`, which is inline in both forms, rather than on the `refused` list, which is absent past the read size; consolidate step 7 reports refusals and every run's kept tails; 052's spec and the bounded-members test row name `refused`. The rule waived five observations with their reasons. The fix round was re-read by an independent agent that read the whole contract, spec, data model and all eight scenarios, 88,539 bytes: clean, and the loop's closing round was empty. Of the eight stored decisions on files the run read, five stand and are carried, and three no longer hold and expire.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

## Observations

- bug — the data model's The write guard says a refused file's mentions are read from the file as it stands, but Sweep::rewrite takes them from the content the sweep read: text is the original read for an unwritten file, never the other writer's version. Their added or removed lines, and every shifted line number, are absent from mentions, and the test pins only the path (a_file_changed_after_the_sweep_read_it_is_refused_and_the_others_are_written), so it holds for either reading. The code comment says the mentions are the ones the file still holds, which is also not what it reads. Two ways to make the artifacts agree: reword the data-model sentence to as the sweep read it (one sentence in 051's own data-model.md), or re-read each refused file for its mentions (a new read outside the guard, a new test, a new mutant). — `runtime/src/primitives/rewrite_spec_links.rs:306` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug — the gate that stops fold step 11 and consolidate step 4 before retire-feature is worded on the refused list ('When refused is not empty, stop'), but past the read size the list is absent from the result (result.refused is None and not serialized; only refused-count stays inline, as the new test past_the_read_size_the_refused_files_move_to_the_lists_file shows). A host reading an absent refused as empty would go on to retire-feature with files still naming the directory, which is the dangling-pointer outcome the stop exists to prevent. The count is present in both forms, so the stop condition should read it. The same wording is in the resumption rows, reference/fold.md and the data model. — `framework/commands/fold.md:73` — **fixed** (decided by the rule)
- other — specs/052-spec-supersession-and-consolidation/spec.md still says rewrite-spec-links' rewritten, skipped, kept-tails and mentions lists are held to read-size-bytes; the window made it five lists with refused. A stale enumeration in a done spec's behavior prose, not in a criterion, so a correction (no back-edge) by classify-spec-edit's rule for spec.md outside the criteria. The fix lands in 052's spec directory, which this window does not otherwise touch. — `specs/052-spec-supersession-and-consolidation/spec.md:25` — **fixed** (decided by the rule)
- other — spec 080's AC7 population test lists the lists rewrite-spec-links moves as rewritten, skipped, kept-tails and mentions, so the MCP-level check that every moved list leaves the result past the read size, with its count inline, does not cover refused. 051's own unit test covers the file form (M9 and M10 kill it), so a regression is caught, but the shared population is one list short of the primitive. — `runtime/tests/mcp.rs:1837` — **fixed** (decided by the rule)
- other — consolidate step 7's report still says each file step 4 re-pointed with its count, while fold step 14 now also reports each file a run refused and the run that finally re-pointed it, and the consolidate resumption row promises a report that carries each run's kept-tails. After a refusal and a re-run, step 7 gives the host no instruction to report the first run's refusals or to carry its kept tails. — `framework/commands/consolidate.md:91` — **fixed** (decided by the rule)
- other — rewrite-spec-links exits 0 on the CLI when refused is non-empty, an unfinished sweep, while derive-dependencies and derive-references exit non-zero for the same BE-TXN-002 refusal (main.rs derive_*_blocks). The data model states the design ('raises no error for one'), the host commands read refused and the only CLI callers are tests, so nothing in the repository runs the CLI as a gate. Recorded so the difference is a decision, not an accident. — `runtime/src/main.rs:1176` — **waived**: the data model's The write guard states that rewrite-spec-links reports a refusal in refused and raises no error for one; the callers that act on it are the fold and consolidate commands, which read the result, and no script or workflow runs the CLI as a gate (decided by the rule)
- other — the CHANGELOG entry and the data model give two new result fields as the reason for BREAKING under BE-COMPAT-001, but the Statement permits new response fields; what is breaking is the new changed-since-read error where invalidate-review used to write, and the narrowed meaning of an empty rewritten (it now needs refused empty too). The BREAKING label is right and matches 041, 057, 066 and 067's entries for the same class; the stated reason over-lists. — `runtime/CHANGELOG.md:36` — **waived**: the classification BE-COMPAT-001 asks for is given and correct as breaking; only the list of reasons in the entry is loose, and the same wording is the precedent the prune and relocation entries set for this class (decided by the rule)
- other — PendingFile::text maps an absent snapshot to a NotFound error that no constructor can reach, since snapshot_of returns an error for an absent file before a PendingFile exists; the arm is defensive, not live logic. — `runtime/src/primitives/rewrite_spec_links.rs:264` — **waived**: the crate denies unwrap and expect outside tests, so a Result return is the lint-clean way to read the snapshot's optional text; removing the arm means holding the text a second time beside the snapshot (decided by the rule)
- other — invalidate-review's claim that the snapshot is taken before anything reads review.md (the plan, the module doc and plan()'s doc comment) has no test or mutant: moving ReadSnapshot::read below read_recorded_waivers would fail none of the new tests, because both change the file only after plan() returns. The seam to change the file between two reads inside plan() does not exist, and the guard's own compare semantics are proved in the helper's tests. — `runtime/src/primitives/invalidate_review.rs:105` — **waived**: testing the position needs a hook between two reads inside plan(), which the code does not carry and the pipeline's other guarded primitives do not carry either; the helper's tests prove the compare, and the position is one line a reader sees (decided by the rule)
- other — fold step 10 states no handling for the changed-since-read error in the step itself; it lives only in the resumption row of the table at the top of the command, in the primitive's description and in the data model, while step 11 carries its stop-and-run-again clause in its own text. A host that reads step 10 alone meets the error with the handling one table away. — `framework/commands/fold.md:71` — **waived**: the command's own preamble says an interrupted fold is completed by re-running and the table is where each step's re-run behavior is stated; restating the row in the step is the copy the drift rules ask not to keep, and the primitive's error text already tells the caller to re-run (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md` — 066-audit-records-and-the-completion-gate
- `.claude/commands/ductus/prune.md` — 022-deterministic-runtime, 041-task-pruning
- `.claude/commands/ductus/review.md` — 066-audit-records-and-the-completion-gate
- `.pi/prompts/ductus-analyze.md` — 066-audit-records-and-the-completion-gate
- `.pi/prompts/ductus-prune.md` — 022-deterministic-runtime, 041-task-pruning
- `.pi/prompts/ductus-review.md` — 066-audit-records-and-the-completion-gate
- `AGENTS.md` — 050-constitution
- `framework/commands/analyze.md` — 066-audit-records-and-the-completion-gate
- `framework/commands/prune.md` — 022-deterministic-runtime, 041-task-pruning
- `framework/commands/review.md` — 066-audit-records-and-the-completion-gate
- `runtime/src/main.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/bounded_lists.rs` — 022-deterministic-runtime
- `runtime/src/primitives/derive_dependencies.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/derive_references.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/mod.rs` — 022-deterministic-runtime, 041-task-pruning, 057-analyze-artifact-and-record-relocation
- `runtime/src/primitives/prune_plan.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/src/primitives/prune_project.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/src/primitives/prune_tasks.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/src/primitives/relocate_audit_records.rs` — 057-analyze-artifact-and-record-relocation
- `runtime/src/primitives/write_analysis.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/write_review.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/schema/paths.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/src/session.rs` — 022-deterministic-runtime, 041-task-pruning
- `runtime/tests/golden/review-basic.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/line_ending_discipline.rs` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/compare-and-set-041.py` — 022-deterministic-runtime, 041-task-pruning
- `scripts/mutation/compare-and-set-057.py` — 057-analyze-artifact-and-record-relocation
- `scripts/mutation/compare-and-set-066.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/compare-and-set-067.py` — 067-analyze-and-audit-checks
- `scripts/mutation/containment-and-bounds-067.py` — 067-analyze-and-audit-checks
- `scripts/mutation/owner-reviews-067.py` — 067-analyze-and-audit-checks
- `scripts/mutation/prune-project-containment.py` — 022-deterministic-runtime, 041-task-pruning
- `scripts/mutation/prune-project.py` — 022-deterministic-runtime, 041-task-pruning
- `specs/022-deterministic-runtime/plan.md` — 022-deterministic-runtime, 041-task-pruning, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/022-deterministic-runtime/tasks.md` — 022-deterministic-runtime, 041-task-pruning, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/041-task-pruning/analysis.md` — 041-task-pruning
- `specs/041-task-pruning/data-model.md` — 022-deterministic-runtime, 041-task-pruning
- `specs/041-task-pruning/plan.md` — 022-deterministic-runtime, 041-task-pruning
- `specs/041-task-pruning/review.md` — 022-deterministic-runtime, 041-task-pruning
- `specs/041-task-pruning/spec.md` — 022-deterministic-runtime, 041-task-pruning
- `specs/041-task-pruning/tasks.md` — 022-deterministic-runtime, 041-task-pruning
- `specs/050-constitution/analysis.md` — 050-constitution
- `specs/050-constitution/plan.md` — 050-constitution
- `specs/050-constitution/review.md` — 050-constitution
- `specs/050-constitution/scenarios/the-constitution-fits-one-read.md` — 050-constitution
- `specs/050-constitution/spec.md` — 050-constitution
- `specs/052-spec-supersession-and-consolidation/spec.md` — 052-spec-supersession-and-consolidation
- `specs/057-analyze-artifact-and-record-relocation/data-model.md` — 022-deterministic-runtime, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/057-analyze-artifact-and-record-relocation/plan.md` — 022-deterministic-runtime, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/057-analyze-artifact-and-record-relocation/spec.md` — 022-deterministic-runtime, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/057-analyze-artifact-and-record-relocation/tasks.md` — 022-deterministic-runtime, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 022-deterministic-runtime, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/066-audit-records-and-the-completion-gate/plan.md` — 022-deterministic-runtime, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 022-deterministic-runtime, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/data-model.md` — 022-deterministic-runtime, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/plan.md` — 022-deterministic-runtime, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/review.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/derive-references-results-are-bounded.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/tasks.md` — 022-deterministic-runtime, 057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/data-model.md` — 057-analyze-artifact-and-record-relocation, 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/spec.md` — 057-analyze-artifact-and-record-relocation, 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency

## Unexamined governance

*None.*
