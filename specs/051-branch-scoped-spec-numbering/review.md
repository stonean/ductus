---
spec: 051-branch-scoped-spec-numbering
last-run: 2026-10-30T01:31:06Z
reviewed-against: d4b75e66692903e91ebc19036650362bf64e6cf7
diff-base: 6573844b830c4997101896095b72b6039b4c64f7
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 8
scope: 208
skipped-passes: []
reviewed-digest:
  data-model.md: c20019d89348ccca1008dacbf028c328aa0836c93eec6042f5f5c37c7ba9448e
  scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md: 6ae5b1d58174acda9aadcc1d96024e65f28eea998dd2fc857a84616991c9bc5b
  scenarios/a-fold-target-is-held-to-the-name-grammar.md: d2f77d8a21f7353c434f5f29cf9e47a144581ccc3ee49787bb9a505a6461dc5b
  scenarios/a-link-inside-the-target-stays-inside-it.md: a367c3a7c1e260f98b27eb73a87958625422a4e458b758399fd5fa526eb2e1a4
  scenarios/a-pending-fold-never-holds-done.md: 602c9f86e24d0a2cfa9cdebf78bf3d32e09255b00b189cd882d89d99e433e4b2
  scenarios/fold-target-checked-before-the-rewrite.md: 40bfa2011fb9af497ae85dac6f23f2ad8719dddcc8c5f5654dda7560aade7f32
  scenarios/rewrites-preserve-line-endings.md: 1ab09161757ade0d1f4a3959f574e54d68dab19f275ec84b345b8d16a75ccd26
  scenarios/the-numbering-grammar-reaches-every-surface.md: d4f11334754c65e3cf73346d224aaf8d782f5385a266a2bf22d3339a1079089a
  scenarios/the-spec-root-is-read-before-the-first-write.md: cd12facf1f7af5a4d7a241a909fe34f69fd1bc02984e066e80bc55e7b3be9106
blocking: false
dispositions:
  fixed: 21
  routed: 0
  discarded: 4
  undispositioned: 0
decisions:
  - key: "convention: a-status-commit-holds-only-the-transition's Behavior reads as requiring the bullet to say a transition staged alone is lost — `specs/050-constitution/scenarios/a-status-commit-holds-only-the-transition.md`"
    outcome: discarded
    reason: the Behavior requires the bullet to state its condition, that the spec file holds nothing but the transition, and its reason, that the pre-commit hook restages the file whole; the bullet states both, and the loss of a transition staged alone is the scenario's own account of the incident
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: claude-md-import-syntax's shared-import bullet contrasts its rewrite with a removal the procedure no longer performs — `framework/migrations/claude-md-import-syntax.md`"
    outcome: discarded
    reason: the bullet still guards against de-duplicating an import the adopter added by hand, which removal would drop if the registration were withdrawn; its sense does not depend on the constitution line
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: 017's plan and 072's Affected Files still name the old import form — `specs/017-derive-dont-ask/plan.md`"
    outcome: discarded
    reason: each records that spec's own edit at the time, a decision record, not a claim about the tree today
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: a CLAUDE.md symlinked to AGENTS.md becomes a copy when constitution-import writes it by temporary file and rename — `framework/migrations/constitution-import.md`"
    outcome: discarded
    reason: every bootstrap write of CLAUDE.md already does this, merge-managed-block's managed-block render on every run included, so the migration adds no new case; a symlinked CLAUDE.md is not a layout the framework supports
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: 022's constitution-excerpts-as-skill-resources says every CLAUDE.md carries the constitution import — `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md`"
    outcome: discarded
    reason: this change makes the premise true again, the template's third line being @.ductus/constitution.md; 022 is another session's and owes no edit
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: the constitution line naming plan.md carries § references resolve-anchor classes as qualified and does not judge — `framework/constitution.md`"
    outcome: discarded
    reason: both targets, §brownfield-inbox and §drift-prevention, carry markers (checked by hand), and citing a subsection's marked section is the constitution's own anchor rule
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "other: constitution-import accepts a four-space-indented line, misses a match behind a leading byte-order mark, and reports the @import form on a file unpinned after claude-md-import-syntax was skipped — `framework/migrations/constitution-import.md`"
    outcome: discarded
    reason: "contrived inputs: claude-md-import-syntax treats indentation the same way, the template writes no byte-order mark, and an operator who unpins a file has the earlier migration's pinned warning in hand"
    decided-at: 2026-10-27T15:08:29Z
    decided-by: andy@stone.dev
  - key: "convention: a #fragment containing a dot is cut at the dot on a fold into a scenario — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: discarded
    reason: GitHub's heading slugs carry no dots, so such a fragment names no heading a link in this corpus can reach; a contrived input
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: no numbered step of the bootstrap names the two managed-block merges — `framework/bootstrap/ductus.md`"
    outcome: discarded
    reason: Instructions step 4 lists what apply-manifest replaces, not every Shared Files step, and two real adopter bootstraps on this text wrote both blocks from Shared Files
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: whether Auggie resolves CLAUDE.md's @AGENTS.md import, and so reads the routing block, is unverified — `framework/templates/project/claude-md.md`"
    outcome: discarded
    reason: the rule's canonical statement is the constitution's drift-prevention section, which target step 4 loads whenever an import did not, so an Auggie session holds it either way
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "bug: a run the Frontmatter Migration precheck halts after generator-primitives leaves a pre-0.30 hook calling the scripts that migration deleted, so the commit the refusal asks for fails — `framework/bootstrap/ductus-procedure.md`"
    outcome: routed
    target: specs/013-text-first-artifacts/scenarios/the-precheck-ignores-what-the-same-run-wrote.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md`"
    outcome: discarded
    reason: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: generator-primitives and audit-record-relocate state no report line for their no-op cases — `framework/migrations/generator-primitives.md`"
    outcome: discarded
    reason: an observation about the migration machinery outside this window; the host's own wording reported each outcome truthfully
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the Closing restart gives no single form for Pi wired in the run beside Claude already registered — `framework/bootstrap/ductus-procedure.md`"
    outcome: discarded
    reason: the per-agent forms compose into an accurate notice, as the subject's run shows, so no host reads it wrong
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md`"
    outcome: discarded
    reason: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md`"
    outcome: discarded
    reason: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: the Frontmatter Migration's specs/**/spec.md walk reports specs/templates/spec.md as already frontmatter on every run — `framework/bootstrap/ductus-procedure.md`"
    outcome: discarded
    reason: the template has frontmatter, so the walk skips it with a report line and writes nothing
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: fetch-archive sends no User-Agent — `runtime/src/primitives/fetch_archive.rs`"
    outcome: routed
    target: specs/048-govern-acquired-runtime/tasks.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: a .githooks linked into a host configuration directory inside the project is not judged by host_config::check_destination — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: discarded
    reason: a contrived layout; the write stays inside the project, and the referrer list is fixed to five files none of which is a host's configuration
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: fold's resumption row for step 11 says a re-run rewrites nothing, which a fragment running into a second token contradicts — `framework/commands/fold.md`"
    outcome: discarded
    reason: "a contrived input: a fragment that is itself a second spec-root path, which the rewrite now leaves for the orphan check to report"
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: the spec-root walk drops a directory it cannot list, so it is neither counted nor skipped — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: discarded
    reason: a directory the process cannot list inside a repository it writes is contrived, and the walk's filter predates this window
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: a root-absolute /specs/ link is neither re-pointed nor reported — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: routed
    target: specs/051-branch-scoped-spec-numbering/tasks.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: an archive entry whose only mode bits are group or other write lands at the umask default — `runtime/src/primitives/extract_archive.rs`"
    outcome: discarded
    reason: "a contrived entry: no archive the framework ships carries a file with neither owner read nor owner write"
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: legacy-constitutions-region was edited in place after a --ref=main push, so an adopter who ran the earlier text keeps its trailing blank lines — `framework/migrations/legacy-constitutions-region.md`"
    outcome: discarded
    reason: the migration is unreleased, reached only --ref=main adopters for the hours between f056b209 and 47e68233, and the residue is trailing blank lines after a removed comment region, which change no behavior
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "stale: 062 task 23's checked item still gives the fixed-interval rationale for the lock poll — `specs/062-concurrent-session-targets/tasks.md`"
    outcome: discarded
    reason: a ticked task records the decision as it was taken, task 28 records its reversal, and tasks.md is ephemeral work tracking that prune may drop
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test: installed-file-modes M5's kill depends on ctime resolution finer than 50 ms — `scripts/mutation/installed-file-modes.py`"
    outcome: discarded
    reason: disclosed in the list's docstring; APFS and ext4 stamp ctime in nanoseconds, and the test itself passes on any file system
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: an @/specs/x import in CLAUDE.md is read as project-relative, where Claude Code reads an absolute path — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: an absolute import of a top-level /specs directory is a contrived input
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "convention: a kept-tails entry gives the reference from the spec root, without a prefix kept on the line — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: discarded
    reason: its path and line locate the reference, and the check reports the same span from the root
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: a ./ prefix in the hook or a root markdown file assumes the project root is the work-tree root, which a project in a repository subdirectory breaks — `specs/067-analyze-and-audit-checks/data-model.md`"
    outcome: discarded
    reason: a ./ resolves as the bare paths in those files already do, the assumption every bare reference carries, not one task 30 adds
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test: the Family 19 parity tests run git in their fixtures under a hook's inherited GIT_DIR, so a commit from a linked worktree fails four of them — `runtime/tests/mechanical_sweep_parity.rs`"
    outcome: routed
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: a leading / was read from the project root in a project below its repository's root, where a host resolves it from the repository's root — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: the framework reads a root-absolute path from the project root, as check-corpus-links reads a root-absolute link (spec 059); reading it from the repository's root here alone split the fold's link and token passes, so 3fc5755a's gate was reverted
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: in a project below its repository's root, /proj/specs/x, the project's own path spelled from the repository's root, is neither checked nor re-pointed — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: a project's README spelling its own spec from the repository's root is a contrived input, and the project-root reading names it nothing
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: in a project below its repository's root the hook runs from the repository's root, so its ./ and bare paths resolve there — `specs/067-analyze-and-audit-checks/data-model.md`"
    outcome: discarded
    reason: the same assumption every bare reference in the hook carries, which task 30 does not add; prefixed and bare agree
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: bare-rule cases outside task 30 — ~/\".ductus/x\", \"$X\"specs/x and ${X}specs/x in the hook, ${X:-./specs/x}, *specs/x*, and @/specs/x in CLAUDE.md — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: the bare rule predates task 30 and prefixed and bare agree on each; every input is contrived
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "edge: the leading-/ exception also covers (a)/specs/x, x~/specs/x and a quote after a letter — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: contrived text; requiring $( before ) would add a parser for no real input
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test gap: no agreement test puts a path at column 0 in system.md, so a mutant refusing a line-start path only there survives, in the check's tests and the rewrite's — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: the scanner has no per-referrer branch, so a defect of that shape is contrived; the binary reports a bare, ../ and / path at column 0 in system.md
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "test gap: the subdirectory test asserts that a spec only the repository's root holds is reported, not that one only the project holds is not — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: discarded
    reason: the realistic regression, reading existence from the repository's root, is killed by M46; skipping only a path present at both roots is contrived, and the binary does not report the project-only spec
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "stale: two ticked items in 017's tasks.md name a rule's Rationale, which 70bb4af5 removed from the format — `specs/017-derive-dont-ask/tasks.md`"
    outcome: discarded
    reason: tasks.md is ephemeral work tracking whose ticked items record what was built then (§tasks-phase); the format's canonical statement, 017's data-model.md, is current
    decided-at: 2026-10-28T20:13:41Z
    decided-by: andy@stone.dev
  - key: "claim: fold.md step 11's never records a dependency on itself fails for a repo-root-relative specs/ link inside the upstream — `framework/commands/fold.md`"
    outcome: discarded
    reason: the only form that breaks it is a repo-root-relative specs/ link inside a spec, which check-corpus-links reports broken and the hook refuses, so no committed corpus holds one
    decided-at: 2026-10-30T01:31:06Z
    decided-by: andy@stone.dev
  - key: "edge: git_command leaves GIT_CONFIG_PARAMETERS and GIT_CONFIG_COUNT, so an outer git -c commit passes its config to a fixture — `runtime/tests/common/mod.rs`"
    outcome: discarded
    reason: an outer git -c commit whose config changes a fixture's outcome is contrived; the helper clears what a hook exports to locate a repository, the defect 066 task 8 records, and the fixture already blanks global and system config
    decided-at: 2026-10-30T01:31:06Z
    decided-by: andy@stone.dev
  - key: "errors: rewrite-spec-links checks the path escape before the name grammar, so a `to` failing both is refused as InvalidPath — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: discarded
    reason: contrived input; refused before any write either way, and the error names the more serious defect, the escape
    decided-at: 2026-10-30T01:31:06Z
    decided-by: andy@stone.dev
  - key: "concurrency: the read-all-then-write phase widens the window in which another session's edit between read and write is overwritten — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: discarded
    reason: "contrived timing: reading the 656 spec-root markdown files takes about 0.02 s; BE-TXN-002 concerns a service's persisted state, and §concurrent-features says sessions in one tree are not isolated from each other's edits"
    decided-at: 2026-10-30T01:31:06Z
    decided-by: andy@stone.dev
---

# Review — 051-branch-scoped-spec-numbering

## Summary

Review of 051's reopen window from 6573844b, the parent of the reopen 2f79a9ee (task 32), to d4b75e66. 048's review and its three re-read rounds already covered 2f79a9ee, c1ca45b5, 2741fb12, f395900f and ced8c7ca; this review read in depth what no review had: edd08387 (task 34, with 052 task 3), c8314288 and ea859f7c (task 35), and 051's artifacts. The five passes ran against the eleven selected rule files: quality-cross.md read in full; the backend rules applied through the statements the code and earlier records cite (BE-INPUT-002, BE-INPUT-004, BE-PAYLOAD-001, BE-TXN-002), the files not read whole; the three frontend files have no subject in a Rust CLI and markdown. No rule fired, so the counts are zero; 25 observations are listed below. Examined counts the 8 in-scope files read in full: consolidate_hook.rs, the three 051 scenarios the window changed, 051's data-model.md, 052's content-move scenario, framework/commands/consolidate.md and framework/reference/consolidate.md. Read only as the window's diffs and their surroundings, so not counted: rewrite_spec_links.rs, schema/primitives.rs, mcp/server.rs, create_feature.rs, the mutation list, fold.md, framework/reference/fold.md, docs/slash-commands.md, runtime.yml, tests/common/mod.rs, and 051's spec, plan and tasks (the criteria through read-spec). Not read: every other in-scope entry, which is other sessions' work in the window, committed or not (041, 048, 055, 075 and others); the generated .claude/, .pi/ and .ductus/ copies, checked with gen-claude-commands.sh --check. Two fix rounds, uncommitted at this write. An independent re-read of round 1 found 1 MUST (050 AC16 lost its reason), 3 SHOULD (the helper's doc, two unswept copies) and 6 NOTE, all dispositioned below; round 2 changed prose and doc comments only and was not re-read. Proof, in a /tmp tree of HEAD plus round 1: fmt, clippy --release --all-targets -D warnings, and the full release suite, 2249 tests in 24 binaries, pass; after round 2, clippy, the 2071 unit tests and consolidate_hook pass. With GIT_DIR naming a scratch repository the fixed consolidate_hook passes and leaves it untouched, while HEAD's version fails both cases and leaves it with core.hooksPath set, 5 index entries and 2 commits. Mutations M46 to M55 ran: 10 of 10 killed; M1 to M45 were not re-run. process-decisions ran restricted and retained all 38 stored decisions, whose subjects this window's passes did not re-read; two of 048's decisions on rewrite_spec_links.rs are carried with their text and reasons.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- bug: since ced8c7ca a linked spec root was listed in `skipped` as not followed while its files were rewritten, so examined-and-nothing-pointed-here could never be reported — fixed ea859f7c, 051 task 35 — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- claim: 051 AC22 counted two cases the result names and missed the spec-root symbolic link ced8c7ca added to `skipped` — fixed ea859f7c, 051 task 35 — `specs/051-branch-scoped-spec-numbering/spec.md` — **fixed**
- doc: the reason a spec-root link is not rewritten, that the write would turn it into a copy, held for a file link only — fixed ea859f7c, 051 task 35 — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- doc: the fold reference's stop clause after the symlink sentence read as being about a symbolic link — fixed ea859f7c, 051 task 35 — `framework/reference/fold.md` — **fixed**
- claim: the docs said a symbolic link to a markdown file is listed, while the code tests the link's own name for `.md` — fixed ea859f7c, 051 task 35 — `specs/051-branch-scoped-spec-numbering/scenarios/the-spec-root-is-read-before-the-first-write.md` — **fixed**
- claim: spec.md's every-in-repo-link sentences and AC6's no-reference-dangles were untrue for files the rewrite does not read — fixed ea859f7c, its re-read's MUST — `specs/051-branch-scoped-spec-numbering/spec.md` — **fixed**
- claim: create_feature.rs's next_branch_number doc comment carried the same every-in-repo-link claim — fixed ea859f7c, its re-read's MUST — `runtime/src/primitives/create_feature.rs` — **fixed**
- claim: AC22 said the orphan check does not read the spec root, though it reads system.md there — fixed ea859f7c, its re-read's SHOULD — `specs/051-branch-scoped-spec-numbering/spec.md` — **fixed**
- convention: two content-framed sentences, in a code comment and in the walk scenario — fixed ea859f7c — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- convention: the fold reference counted its three lists — fixed ea859f7c — `framework/reference/fold.md` — **fixed**
- stale: the help copies ab5eaa49 left ungenerated — fixed ea859f7c, whose hook regenerated them — `.claude/commands/ductus/help.md` — **fixed**
- generated: ea859f7c committed target.md's generated copies from another session's staged source edit, so HEAD's copies did not match framework/commands/target.md — fixed c6a0dca2, which committed that source — `.claude/commands/ductus/target.md` — **fixed**
- test: consolidate_hook.rs ran git in its fixtures without clearing GIT_DIR, GIT_WORK_TREE and GIT_INDEX_FILE, so under an inherited GIT_DIR its cases committed into the outer repository — fixed by a shared git_command helper, proved with GIT_DIR naming a scratch repository — `runtime/tests/consolidate_hook.rs` — **fixed**
- claim: the constitution and consolidate.md's description and opening line said consolidate re-points every inbound pointer so none dangles, though it re-points only those in the files it reads — bounded, the constitution 1 byte over HEAD — `framework/constitution.md` — **fixed**
- claim: the first bound dropped the reason 050 AC16 requires §spec-lifecycle to state, why pointers are re-pointed first — restored as lest it dangle — `framework/constitution.md` — **fixed**
- claim: ten more copies of the every-pointer claim — specs/README.md, 000, 052's summary, Motivation and AC7, 050's a-retired-feature-leaves-no-spec, 054's Motivation, README.md and two in docs/slash-commands.md — bounded; 054 reopened done to in-progress — `specs/052-spec-supersession-and-consolidation/spec.md` — **fixed**
- claim: nine sentences said a link inside the target is written relative to its file, though a root-absolute one is re-pointed as written — in consolidate.md, its reference, fold.md step 11, 052's AC9 and scenario, the MCP description and a test doc; each qualified to a link that climbs to the spec root — `framework/commands/consolidate.md` — **fixed**
- test gap: no test held the scenario's edge case that a link inside the target at the wrong depth is re-pointed as written, so dropping either localization condition survived — a test with M54 and M55, both killed — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- ci: runtime.yml's comment for the shipped hook's path named only mod.rs's shell-pattern test, though consolidate_hook.rs commits through that hook — `.github/workflows/runtime.yml` — **fixed**
- claim: the new git_command doc said every test that spawns git builds its command there, though mechanical_sweep_parity.rs does not — the exception named, and 066 task 8 pointed at the helper — `runtime/tests/common/mod.rs` — **fixed**
- convention: three sentences the first fix round wrote read ambiguously — consolidate's description, runtime.yml's that file, and the new test's not at all — `framework/commands/consolidate.md` — **fixed**
- claim: fold.md step 11's never records a dependency on itself fails for a repo-root-relative specs/ link inside the upstream — `framework/commands/fold.md` — **discarded**: the only form that breaks it is a repo-root-relative specs/ link inside a spec, which check-corpus-links reports broken and the hook refuses, so no committed corpus holds one
- edge: git_command leaves GIT_CONFIG_PARAMETERS and GIT_CONFIG_COUNT, so an outer git -c commit passes its config to a fixture — `runtime/tests/common/mod.rs` — **discarded**: an outer git -c commit whose config changes a fixture's outcome is contrived; the helper clears what a hook exports to locate a repository, the defect 066 task 8 records, and the fixture already blanks global and system config
- errors: rewrite-spec-links checks the path escape before the name grammar, so a `to` failing both is refused as InvalidPath — `runtime/src/primitives/rewrite_spec_links.rs` — **discarded**: contrived input; refused before any write either way, and the error names the more serious defect, the escape
- concurrency: the read-all-then-write phase widens the window in which another session's edit between read and write is overwritten — `runtime/src/primitives/rewrite_spec_links.rs` — **discarded**: contrived timing: reading the 656 spec-root markdown files takes about 0.02 s; BE-TXN-002 concerns a service's persisted state, and §concurrent-features says sessions in one tree are not isolated from each other's edits

## Skipped passes

*None.*

## Unexamined governance

*None.*
