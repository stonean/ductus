---
spec: 051-branch-scoped-spec-numbering
last-run: 2026-11-20T00:20:00Z
reviewed-against: a5382b902088c794f95bef6ac72d75ea92eed09d
diff-base: aed0880d998e471c6213d471d70a43a09ddc083b
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 9
unread:
  - runtime/CHANGELOG.md
  - runtime/src/primitives/rewrite_spec_links.rs
  - runtime/src/schema/descriptions.rs
  - scripts/mutation/rewrite-spec-links.py
excluded: 174
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 8e8baa256ea782bfb50a0620c598997cec119266cb41a5f8a0cd233879d6427a
  scenarios/a-definition-and-a-title-follow-the-link-grammar.md: fef8585771df20e1845e743ac4f6ef3d14684cabc8dc9f0b4caf4e8d3284b2e0
  scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md: 015b0ba736e27e91d0d86b37c65e6f7d0d947b5773cf6495313a8ff337b93a47
  scenarios/a-fold-target-is-held-to-the-name-grammar.md: dd15a827e2e30c67606ed2b49c965773d69e0356283264ed639f97c5f4adc901
  scenarios/a-link-inside-the-target-stays-inside-it.md: a367c3a7c1e260f98b27eb73a87958625422a4e458b758399fd5fa526eb2e1a4
  scenarios/a-pending-fold-never-holds-done.md: dc5c2daa3fd78c7251edf856c21f5d36737e3725d91c808231e112f84502e8b5
  scenarios/fold-target-checked-before-the-rewrite.md: 40bfa2011fb9af497ae85dac6f23f2ad8719dddcc8c5f5654dda7560aade7f32
  scenarios/rewrites-preserve-line-endings.md: 4389080643595553497a9a10a89e7320d157623df1a825094c2bb1599ba84d9c
  scenarios/the-numbering-grammar-reaches-every-surface.md: d4f11334754c65e3cf73346d224aaf8d782f5385a266a2bf22d3339a1079089a
  scenarios/the-spec-root-is-read-before-the-first-write.md: ccf10584e584b2f0f83808e7e1413a30cd98c1b5691a4c464da9432b0fd53ff9
criteria-judged: 34
criteria-met: 34
criteria-verdicts:
  - label: AC1
    digest: 422c1d1d515504f892acbe4dfc3d12c1c4bad665ddee0192c9e23e1c9b97a0ca
  - label: AC8
    digest: 9cd10115fa5b3a2532703cceb5ecc102dcc215ba2d033830f033b75633f423f4
  - label: AC9
    digest: bf15681c450ffebdf0872ea59e2bad8a424129d57d9e96b43408d97b9288852d
  - label: AC10
    digest: 14f4a4a6f19465ecbc4a3be007adf426a1ee10a8d9fffa110b3666ae82b4601b
  - label: AC11
    digest: 80b8ba3791b05a5aca62673d606b9c7295e84639b58bb9b60d8fa5f128960d10
  - label: AC12
    digest: 223ed4a77afb7d120e446c5e526daca7a1af7374eab8a4bcca3a68e0981280cd
  - label: AC13
    digest: 2191fe6bae06aa617401f16936d8948a817ad0c7f3a064a0ba17c94fe3df13a3
  - label: AC2
    digest: 98aac475ce5bc3f1a184582c1682585a2fb48de3050a867bef99e2ab79d72adf
  - label: AC3
    digest: 17405dbfee0ab6873c7af39808bca2a385a7b806492f5feddae5ac75f3324631
  - label: AC4
    digest: 8a9ca75962e01af4dc90b141627922fc67924f714e5657552478c83d230e54f9
  - label: AC14
    digest: e66949af79b9f606dfee7720d59a95222f08c8343f0e1285cc7d7bfec163ca1c
  - label: AC5
    digest: 9f8d1eb3866f0db2d4c98ed2b4afa11fcde7464c8e2377faa4b28990d24540aa
  - label: AC15
    digest: 61410e3ea66a2485fa47a70ce07a7a6a5999090f4a59242ed17fc6291f429cc1
  - label: AC16
    digest: dea57cf0f980e9d6e5030d1eb4db4ecd0b615d2fc4309697ac4505cd103811ba
  - label: AC17
    digest: 5f24fe37b93ea7b193a24af8f6ed3fe6afde29be19e2fb51513e3b0ed4acb060
  - label: AC6
    digest: e5214b7629518ce27552afcf0792ade78e7ef1c2e62e1a4984bf0bcc8f479412
  - label: AC18
    digest: 69c1ee2b88010ba3ce44ceec3140ee20c5de9d141aa74c92396b94b5f5060ed8
  - label: AC19
    digest: 1654821797a430c5788e509de5c08a6a43fb5b28aa6f980a63f72bacd912acde
  - label: AC31
    digest: ea39380f3f4e2eb0c1e1c5e5f6708e4e35cd45e3dea149bd5b54aab8cab7592f
  - label: AC32
    digest: bc91812740564c299dbf24847d90a82a9afcb43e93d959b8fee4d16d2462d0fe
  - label: AC33
    digest: 196081a20491f594e522af9772210e8438af45cffe346b2777d687975ecc2d62
  - label: AC20
    digest: f50ca584f2bd8a40192650e789dfcad513b1ca2f0268797c8bce925ecd89871a
  - label: AC21
    digest: 6794aaf16faa07215dac6d03a6c167d81963897506e817a8491576b3b42eecaa
  - label: AC34
    digest: cb886cebcc1d358120139e3c160299ac37909d09c215686b498244c5640ad473
  - label: AC35
    digest: 7c0d039ba55344a7e78c7792b0ccdd2d172d497327b3ccdcf202d5c26e54e25c
  - label: AC22
    digest: 75dba828a03d06e9fd95732a3a331eb9d84703e10447951af2e149edef1ac1ab
  - label: AC23
    digest: 63b19f42e3fd20fdd107f3862e267232301cd09a5c8d93c2324d4b68bab7fd0f
  - label: AC24
    digest: e7f6bc55c3ab97b4dcdfea84867a031315bf8b8c8241de3679abf31e914411d2
  - label: AC25
    digest: db0ce32d0dc086df3516dca4a2fc5b371dbd9a622dee424182c07d087569f44a
  - label: AC37
    digest: f8126ddbb9dba19bcae64054bdcc16e7c4d09d89d1c24f39ac117d306c6de039
  - label: AC26
    digest: 122cd3c618e39a160c3391ea79fbbc7fdf6deb71cbed84cfe10dfed86a86977a
  - label: AC27
    digest: b3c8e6986ed9c8aec1a48ff772a2c50b29bec854cccfc445e1d2c92603c01f79
  - label: AC28
    digest: 5ae2e91289656975bb33cf0dc184f188ee14185b0bbf73d1dd3d8ff1a04401b4
  - label: AC29
    digest: d336945af22d2b43c8a35a9e7130ba34c95065fe72f3bd3525e831454cc22d38
blocking: false
dispositions:
  fixed: 7
  deferred: 0
  waived: 9
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
  - key: "other: `check_corpus_links::link_targets` skips an inline link target holding whitespace, so a titled link is never checked for resolution, which makes the rewrite the only repair for one — `runtime/src/primitives/check_corpus_links.rs`"
    outcome: waived
    reason: "an observation about another spec's check, outside this spec's contract: AC6 and AC22 now state which links `check-corpus-links` reads, and the owner declined on 2026-11-19 to teach that check more of the link grammar"
    decided-at: 2026-11-20T00:20:00Z
    decided-by: rule
  - key: "convention: `derive_references.rs` `is_spec_slug` and `derive_dependencies.rs` `leading_slug` keep a local charset test beside the delegation to `is_feature_slug` — `runtime/src/primitives/derive_references.rs`"
    outcome: waived
    reason: both are documented as a deliberate trust boundary, and AC15's claim is about the form rule, which both delegate
    decided-at: 2026-11-20T00:20:00Z
    decided-by: rule
  - key: "convention: the `REWRITE_SPEC_LINKS` description reads as one list of what is re-pointed and omits the footnote case — `runtime/src/schema/descriptions.rs`"
    outcome: waived
    reason: the description's clause already says a bare `<path>` is left to `mentions`; the footnote case is stated in the scenario and the data model, and the description is size-limited and shared with six other close-outs
    decided-at: 2026-11-20T00:20:00Z
    decided-by: rule
---

# Review — 051-branch-scoped-spec-numbering

## Summary

Window `aed0880d..` (reopen): 9 scope entries as `write-review` derives them at this head, the entries `compute-review-scope` measured at the start less `a-fold-target-is-held-to-the-name-grammar.md`, which the window no longer holds once the fix commit landed, and plus `plan.md`, which the fix touched; 5 read whole (`spec.md`, `plan.md`, `tasks.md` and two window scenarios, with `a-fold-target-is-held-to-the-name-grammar.md` also read whole and its claims checked); the other 4 (`runtime/CHANGELOG.md`, `runtime/src/primitives/rewrite_spec_links.rs`, `runtime/src/schema/descriptions.rs`, `scripts/mutation/rewrite-spec-links.py`) were read in part (the CHANGELOG's other specs' entries, the older tests of `rewrite_spec_links.rs`, the other constants of `descriptions.rs` and the older mutants went unread) and are listed unread. Security pass: 0 findings, all three security rule files read whole. Reuse, quality, efficiency and simplicity passes: nine rule files read whole, 1 SHOULD (QUAL-TEST-001, four definition-grammar edges with no killing test), fixed in the run with `a_definition_keeps_to_the_link_grammar_at_its_edges` and mutants M60 to M63 (each killed; M56 to M63 8 of 8 killed). Criteria step: 34 judged, 31 met as written; AC6, AC15 and AC22 were not met as worded and were corrected in `specs/051-branch-scoped-spec-numbering/spec.md` to state what the checks report today, then judged met with the new text. One fix round, re-read clean by an independent agent; its `spec.md` and `plan.md` edits were exempt from the re-read. Observations: 16 recorded, 7 fixed and 9 waived (six carried stored decisions and three new waivers). Not driven: the scenario edge cases of a definition outside the spec root and one in `system.md`, and a symlinked referrer leaving the project (rests on `a_referrer_that_leaves_the_project_is_skipped_and_not_read`). No sibling-spec finding was deferred.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

34 of 34 acceptance criteria judged met.

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

- AC1 — met: create-feature run in runtime/src/primitives/create_feature.rs (next_branch_number, format {identifier}.{n}-{slug}); test branch_scoped_creation_counts_within_its_identifier passes (1234.1-staged-change, 1234.2-another-change); CLI probe gave proj-2-x.1-probe-thing
- AC8 — met: next_branch_number in create_feature.rs is max+1 filtered to the same identifier; test branch_scoped_creation_counts_within_its_identifier (5678 starts at .1) passes; probe: 1-PROJ gave .1 then .2 while PROJ-1 stayed .1
- AC9 — met: resolve_branch_scope uses derive_slug on an opaque token; test branch_identifier_is_sanitized_and_reported passes (proj-1111.1-ticket-work, 1111-proj.1-item-work)
- AC10 — met: derive_slug sanitizes (never rejects) and CreateFeatureResult.identifier returns the sanitized value; framework/commands/specify.md step 4 'What the identifier will become' names the sanitized form in the prompt before step 5 creates anything; tests branch_identifier_is_sanitized_and_reported, an_existing_branch_scoped_path_is_refused_not_overwritten
- AC11 — met: derive_slug collapses '.'; test branch_identifier_is_sanitized_and_reported (proj.7 -> proj-7.1-dotted); parse_feature_dir splits on first '.' (runtime/src/primitives/mod.rs, parse_branch_scoped)
- AC12 — met: test identifiers_differing_only_in_case_share_one_namespace passes (PROJ-1111 .1, proj-1111 .2)
- AC13 — met: framework/commands/specify.md flags table (--branch) and step 4 first bullet: prompt for identifier, candidate from git rev-parse --abbrev-ref HEAD only when extractable, no candidate otherwise, confirmed value used; prose step with audit:ignore-promotion, no runtime test possible
- AC2 — met: next_feature_number counts only FeatureForm::Sequential via sequential_number(); test branch_scoped_creation_leaves_the_sequential_counter_alone passes (050-existing + 1234.1 -> 051), also a_branch_scoped_only_root_still_starts_the_sequence_at_001
- AC3 — met: distinct identifiers give distinct names by construction (create_feature.rs next_branch_number, test branch_scoped_creation_counts_within_its_identifier); real git probe in a fixture: branch a created proj-1.1-alpha, branch b 1-proj.1-beta and 1-proj.2-gamma, git merge a into b exited 0 with all three present; no dedicated automated test
- AC4 — met: resolve_branch_scope returns None without branch-id and the sequential path is unchanged (probe: plain create gave 1235-plain; tests numbers_from_max_existing_prefix_plus_one, creates_first_feature_as_001); no config key read by create-feature; specify.md step 4 decides nothing without --branch/--branch-id
- AC14 — met: create-feature writes only the new {root}/{feature}/spec.md (create_feature.rs run); probe git status showed only the new untracked directories, no config or other committed file touched; specify.md Resolved Questions reject a committed setting
- AC5 — met: parse_feature_dir (runtime/src/primitives/mod.rs) feeds list_feature_dirs/is_spec_path; runtime/tests/mixed_corpus.rs 5 tests pass (dashboard, derive-dependencies, derive-references, resolve-feature, fold-target survival); probes on a mixed fixture: dashboard, resolve-feature, check-corpus-links, check-unfolded-specs, traverse-deps (branch-scoped edge both ways) all saw both forms; audit scripts take the corpus from dashboard (scripts/audit/lib.sh spec_corpus); 81f73f41 (2026-09-14) exists, leading_slug/is_spec_slug delegate to is_feature_slug, harvests_branch_scoped_and_four_digit_siblings passes, scenario sibling-link-grammar-is-the-shared-one exists under 067
- AC15 — met: holds with its corrected text: parse_feature_dir is the single definition, Family 22 drives the shipped hook on 1000-thousandth and 1234.1-staged, Family 41 (scripts/audit/feature-dir-grammar-parity.sh) ran: compared 3 of 3 copies, exit 0
- AC16 — met: Clause 1 holds: resolve_feature.rs match_identifier matches sequential by number and branch by identifier string; test a_number_never_matches_a_branch_scoped_directory passes; probe: 123 resolves 123-seq only. Clause 2 is superseded and correctly annotated: probe with 1234-seqfour and 1234.1-staged returned ambiguous [1234-seqfour,1234.1-staged] (tests a_branch_identifier_matches_its_whole_namespace, a_string_naming_both_forms_is_ambiguous); the unconditional wording of clause 2 is the annotated-false text a reader may judge strictly
- AC17 — met: framework/constitution.md '## Numbering Convention' defines Sequential and Branch-scoped forms; '### Spec lifecycle' paragraph 'A branch-scoped spec is a staging form, not a fourth state' states fold-back discharge
- AC6 — met: holds with its corrected text: rewrite_spec_links.rs skipped reasons, check_orphaned_references.rs LEAVES_PROJECT and check_corpus_links.rs OUTWARD_LINK as the criterion now states, kept tails in a definition reported by the fold alone, next-line definition destination listed in mentions; tests a_referrer_that_leaves_the_project_is_skipped_and_not_read and a_reference_definition_is_re_pointed_like_an_inline_link
- AC18 — met: framework/commands/fold.md step 4 (routeFold extension, body-edit or scenario; payload.rs build_route_fold_request) then step 5 gate-confirm 'No write happens before this step'; steps 1-3 are read-only; reference/fold.md Choosing the route; payload.rs routeFold tests pass
- AC19 — met: create_feature.rs stamp_fold_target writes folds-into; runtime/tests/mixed_corpus.rs the_fold_target_survives_every_frontmatter_writer (set-status, derive-dependencies, label-criteria) passes and derive_references.rs splice_references splices only its key (derive-references --write left folds-into byte-identical in fx2)
- AC31 — met: validate_frontmatter.rs validate_folds_into never tests existence (tests an_absent_fold_target_is_never_a_finding, a_fold_target_naming_an_absent_feature_is_not_a_finding; fx2 absent-target spec validates clean); check_unfolded_specs.rs and dashboard.rs fold_target_missing report absence as a report not a finding (dashboard tests an_unresolvable_fold_target_is_called_out_on_the_row); analyze.md step 10 'unresolved here, never as broken'
- AC32 — met: validate_frontmatter.rs validate_folds_into checks string and parse_feature_dir == Sequential only (tests a_malformed_fold_target_is_blocking, a_fold_target_naming_a_branch_scoped_spec_is_blocking, a_fold_target_naming_an_absent_feature_is_not_a_finding); existence enforced in retire_feature.rs and rewrite_spec_links.rs plan (fx2: ghost target refused 'feature directory not found', directory left in place)
- AC33 — met: rewrite_spec_links.rs rewrite_folds_into runs in the same rewrite_content pass as body links (tests a_folds_into_naming_the_moved_directory_moves_with_the_links, folds_into_takes_the_feature_even_when_the_target_names_a_scenario; fx2 rename 050-up to 051-up re-pointed folds-into and the body link in one call)
- AC20 — met: derive_dependencies reads body links only: fx2 spec 1234.2-nolink carries folds-into: 050-up and derive-dependencies --write left dependencies: [] (the spec with a body link got the edge from the link alone); mixed_corpus.rs survival test
- AC21 — met: check_unfolded_specs.rs compute reports every BranchScoped directory with folds-into and status (6 tests pass; fx2 returned both staged specs, examined 3); analyze.md step 10 invokes it and reference/analyze.md 'Un-folded branch-scoped specs' states the advisory finding
- AC34 — met: dashboard.rs/pipeline_view.rs status cell qualified '(fold pending)' at every status and fold as next action at done (tests a_declared_fold_is_never_reported_as_done, below_done_a_declared_fold_leaves_the_ordinary_next_action, outstanding_folds_are_collected_into_a_callout pass); commands/status.md states it
- AC35 — met: check_review_gate.rs carries no folds-into check (only a comment and the test fixture DECLARES_FOLD); tests a_pending_fold_never_holds_done_wherever_its_target_lives and a_declared_fold_leaves_every_other_check_to_decide pass; no 'pending-fold' block value remains in src
- AC22 — met: holds with its corrected text: same cases as AC6 on fixtures (in-project symlink referrer reported by the orphan check, outward ones skipped by both checks, definition kept tail unreported, next-line definition in mentions); tests in rewrite_spec_links.rs and check_orphaned_references.rs
- AC23 — met: .githooks/pre-commit lines 207-208 and framework/bootstrap/hooks/ductus-pre-commit lines 207-208 run derive-dependencies and derive-references --write; fx2 derive-dependencies --write regenerated dependencies from the rewritten bodies and left folds-into; fold.md step 14 recommends committing; rewrite_spec_links.rs never edits dependencies:/references: (rewrite_folds_into touches only folds-into)
- AC24 — met: fold.md steps 9-10; driven in fx2: set-status --from done --to in-progress succeeded, invalidate-review nulled last-run, check-review-gate then answered blocked-by not-reviewed; tests invalidate_review::tests (waivers_survive_the_invalidation, invalidating_twice_converges) and check_review_gate null_last_run_blocks_not_reviewed pass
- AC25 — met: framework/constitution.md section 'A branch-scoped spec is a staging form' says 'adding no new back-edge'; fold.md step 9 calls set-status done to in-progress, the edge reference/fold.md 'Reopening a done upstream spec' names; set_status.rs adds no edge
- AC37 — met: create_feature.rs resolve_branch_scope refuses branch-id without fold-into; test branch_scoped_creation_requires_a_fold_target passes; fx2: `create-feature --branch-id 1234` without --fold-into refused, and --fold-into without --branch-id refused
- AC26 — met: create_feature.rs resolve_branch_scope refuses an identifier that derive_slug empties before claim_feature_dir (test branch_scoped_argument_groups_are_validated; fx2: --branch-id '!!!' refused, 'sanitizes to an empty identifier', no directory created)
- AC27 — met: create_feature.rs run returns created:false when the directory exists and claim_feature_dir uses create_dir AlreadyExists for the race (tests an_existing_branch_scoped_path_is_refused_not_overwritten, a_feature_directory_is_claimed_by_exactly_one_racer, a_dangling_link_at_the_derived_name_is_a_lost_claim pass)
- AC28 — met: retire_feature.rs refusal 2 (target must hold spec.md) and rewrite_spec_links.rs plan refuse before any write (tests an_absent_fold_target_refuses_and_leaves_the_spec_in_place, an_absent_fold_target_refuses_with_the_corpus_untouched pass; fx2 confirmed both refusals and the staged directory left in place)
- AC29 — met: fold.md 'Process specs one at a time' and its re-run table (steps 6-12); retire_feature.rs already-gone returns retired:false; invalidate-review converges (fx2 second call invalidated:false); append_task appended:false dedup (tests pass); rewrite sweep idempotent once refused-count is zero (tests running_the_sweep_again_after_a_refusal_rewrites_the_refused_file)

## Observations

- convention: a #fragment containing a dot is cut at the dot on a fold into a scenario — `runtime/src/primitives/rewrite_spec_links.rs` — **waived**: GitHub's heading slugs carry no dots, so such a fragment names no heading a link in this corpus can reach; a contrived input
- convention: a .githooks linked into a host configuration directory inside the project is not judged by host_config::check_destination — `runtime/src/primitives/rewrite_spec_links.rs` — **waived**: a contrived layout; the write stays inside the project, and the referrer list is fixed to five files none of which is a host's configuration
- convention: a kept-tails entry gives the reference from the spec root, without a prefix kept on the line — `runtime/src/primitives/rewrite_spec_links.rs` — **waived**: its path and line locate the reference, and the check reports the same span from the root
- errors: rewrite-spec-links checks the path escape before the name grammar, so a `to` failing both is refused as InvalidPath — `to` — **waived**: contrived input; refused before any write either way, and the error names the more serious defect, the escape
- other — the CHANGELOG entry and the data model give two new result fields as the reason for BREAKING under BE-COMPAT-001, but the Statement permits new response fields; what is breaking is the new changed-since-read error where invalidate-review used to write, and the narrowed meaning of an empty rewritten (it now needs refused empty too). The BREAKING label is right and matches 041, 057, 066 and 067's entries for the same class; the stated reason over-lists. — `runtime/CHANGELOG.md` — **waived**: the classification BE-COMPAT-001 asks for is given and correct as breaking; only the list of reasons in the entry is loose, and the same wording is the precedent the prune and relocation entries set for this class (decided by the rule)
- other — PendingFile::text maps an absent snapshot to a NotFound error that no constructor can reach, since snapshot_of returns an error for an absent file before a PendingFile exists; the arm is defensive, not live logic. — `runtime/src/primitives/rewrite_spec_links.rs` — **waived**: the crate denies unwrap and expect outside tests, so a Result return is the lint-clean way to read the snapshot's optional text; removing the arm means holding the text a second time beside the snapshot (decided by the rule)
- other: the `REWRITE_SPEC_LINKS` byte figure in the `runtime/CHANGELOG.md` entry for the definition fix was stale at the window head (3,982 against 3,964); the entry now says the figure is the constant's string body at the fix and names the later value — `runtime/CHANGELOG.md` — **fixed** (decided by the rule)
- test gap: four behaviours of the definition grammar (angle destination, three-space indent, empty label, definition kept tail) survived mutants in `runtime/src/primitives/rewrite_spec_links.rs` (QUAL-TEST-001); test `a_definition_keeps_to_the_link_grammar_at_its_edges` and mutants M60 to M63 in `scripts/mutation/rewrite-spec-links.py` now kill each — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed** (decided by the rule)
- convention: AC15's annotation in `specs/051-branch-scoped-spec-numbering/spec.md` said the dogfooded hook and the frontmatter lint are held by nothing, which Family 41 now falsifies; the criterion states both clauses hold — `specs/051-branch-scoped-spec-numbering/spec.md` — **fixed** (decided by the rule)
- bug: AC6 and AC22 in `specs/051-branch-scoped-spec-numbering/spec.md` said the checks report every file the fold skips and every kept tail; an outward symlink is unread by both checks and a kept tail in a reference definition is reported by neither, and a definition with its destination on the next line is only listed in `mentions`; both criteria now state it — `specs/051-branch-scoped-spec-numbering/spec.md` (breaks the spec's contract) — **fixed** (decided by the rule)
- convention: the scenario `specs/051-branch-scoped-spec-numbering/scenarios/rewrites-preserve-line-endings.md` stated its as-found defect in the present tense; it now says the ten writers share `line_ending_of` and names the test file holding the seven — `specs/051-branch-scoped-spec-numbering/scenarios/rewrites-preserve-line-endings.md` — **fixed** (decided by the rule)
- convention: the Resolved Question on what `rewrite-spec-links` leaves alone gave a two-pass-agreement reason that holds only for `system.md`; it now says so — `specs/051-branch-scoped-spec-numbering/spec.md` — **fixed** (decided by the rule)
- other: the module doc of `runtime/src/primitives/rewrite_spec_links.rs` and the `rewrite-spec-links` bullet of `specs/051-branch-scoped-spec-numbering/plan.md` did not state the definition and title rules; both now do — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed** (decided by the rule)
- other: `check_corpus_links::link_targets` skips an inline link target holding whitespace, so a titled link is never checked for resolution, which makes the rewrite the only repair for one — `runtime/src/primitives/check_corpus_links.rs` — **waived**: an observation about another spec's check, outside this spec's contract: AC6 and AC22 now state which links `check-corpus-links` reads, and the owner declined on 2026-11-19 to teach that check more of the link grammar (decided by the rule)
- convention: `derive_references.rs` `is_spec_slug` and `derive_dependencies.rs` `leading_slug` keep a local charset test beside the delegation to `is_feature_slug` — `runtime/src/primitives/derive_references.rs` — **waived**: both are documented as a deliberate trust boundary, and AC15's claim is about the form rule, which both delegate (decided by the rule)
- convention: the `REWRITE_SPEC_LINKS` description reads as one list of what is re-pointed and omits the footnote case — `runtime/src/schema/descriptions.rs` — **waived**: the description's clause already says a bare `<path>` is left to `mentions`; the footnote case is stated in the scenario and the data model, and the description is size-limited and shared with six other close-outs (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/clarify.md`
- `.claude/commands/ductus/consolidate.md`
- `.claude/commands/ductus/fold.md`
- `.claude/commands/ductus/help.md`
- `.claude/commands/ductus/implement.md`
- `.claude/commands/ductus/plan.md`
- `.claude/commands/ductus/prune.md`
- `.claude/commands/ductus/review.md`
- `.claude/commands/ductus/specify.md`
- `.claude/commands/ductus/target.md`
- `.ductus/reference/analyze.md`
- `.ductus/reference/audit.md`
- `.ductus/reference/clarify.md`
- `.ductus/reference/consolidate.md`
- `.ductus/reference/fold.md`
- `.ductus/reference/plan.md`
- `.ductus/reference/review.md`
- `.ductus/reference/specify.md`
- `.ductus/reference/status.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-clarify.md`
- `.pi/prompts/ductus-consolidate.md`
- `.pi/prompts/ductus-fold.md`
- `.pi/prompts/ductus-help.md`
- `.pi/prompts/ductus-implement.md`
- `.pi/prompts/ductus-plan.md`
- `.pi/prompts/ductus-prune.md`
- `.pi/prompts/ductus-review.md`
- `.pi/prompts/ductus-specify.md`
- `.pi/prompts/ductus-target.md`
- `framework/commands/analyze.md`
- `framework/commands/clarify.md`
- `framework/commands/consolidate.md`
- `framework/commands/fold.md`
- `framework/commands/help.md`
- `framework/commands/implement.md`
- `framework/commands/plan.md`
- `framework/commands/prune.md`
- `framework/commands/review.md`
- `framework/commands/specify.md`
- `framework/commands/target.md`
- `framework/reference/analyze.md`
- `framework/reference/audit.md`
- `framework/reference/clarify.md`
- `framework/reference/consolidate.md`
- `framework/reference/fold.md`
- `framework/reference/plan.md`
- `framework/reference/review.md`
- `framework/reference/specify.md`
- `framework/reference/status.md`
- `runtime/Cargo.toml`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/interpreter/payload.rs`
- `runtime/src/io.rs`
- `runtime/src/main.rs`
- `runtime/src/parser/mod.rs`
- `runtime/src/primitives/analyze_subjects.rs`
- `runtime/src/primitives/append_inbox.rs`
- `runtime/src/primitives/check_artifacts.rs`
- `runtime/src/primitives/check_command_flags.rs`
- `runtime/src/primitives/check_corpus_links.rs`
- `runtime/src/primitives/check_orphaned_references.rs`
- `runtime/src/primitives/check_promotion_coverage.rs`
- `runtime/src/primitives/check_review_gate.rs`
- `runtime/src/primitives/check_step_references.rs`
- `runtime/src/primitives/classify_spec_edit.rs`
- `runtime/src/primitives/commit_attribution.rs`
- `runtime/src/primitives/compute_review_scope.rs`
- `runtime/src/primitives/derive_routing_candidates.rs`
- `runtime/src/primitives/merge_managed_block.rs`
- `runtime/src/primitives/mod.rs`
- `runtime/src/primitives/prune_plan.rs`
- `runtime/src/primitives/prune_tasks.rs`
- `runtime/src/primitives/read_spec.rs`
- `runtime/src/primitives/relocate_audit_records.rs`
- `runtime/src/primitives/rule_sections.rs`
- `runtime/src/primitives/spec_links.rs`
- `runtime/src/primitives/tool_loading.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/primitives/write_review.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/tests/bounded_list_gates.rs`
- `runtime/tests/check_step_references_cli.rs`
- `runtime/tests/derive_dependencies_golden.rs`
- `runtime/tests/derive_references_golden.rs`
- `runtime/tests/exec_stderr_gone.rs`
- `runtime/tests/golden/implement-basic.jsonl`
- `runtime/tests/golden/plan-basic.jsonl`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/mcp.rs`
- `scripts/audit/README.md`
- `scripts/audit/step-reference-integrity.sh`
- `scripts/mutation/exec-arguments.py`
- `scripts/mutation/installed-file-modes.py`
- `scripts/mutation/owner-reviews-067.py`
- `scripts/mutation/prune-open-region.py`
- `scripts/mutation/prune-review-fixes-041.py`
- `scripts/mutation/record-path-quoting-066.py`
- `scripts/mutation/relocate-merge-children-057.py`
- `scripts/mutation/reopen-measure.py`
- `scripts/mutation/review-022-task-44.py`
- `scripts/mutation/review-read-credit.py`
- `scripts/mutation/scope-attribution.py`
- `scripts/mutation/stderr-warnings-022.py`
- `scripts/mutation/step-references-bounded.py`
- `scripts/mutation/step-references-reach.py`
- `scripts/tests/test-step-reference-integrity.sh`
- `specs/020-code-review/analysis.md`
- `specs/020-code-review/data-model.md`
- `specs/020-code-review/review.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/scenarios/a-closed-stdout-is-a-quiet-exit.md`
- `specs/022-deterministic-runtime/scenarios/append-inbox-comment-aware-write.md`
- `specs/022-deterministic-runtime/scenarios/dashboard-primitive.md`
- `specs/022-deterministic-runtime/scenarios/host-protocol-conformance.md`
- `specs/022-deterministic-runtime/scenarios/installed-files-are-not-group-writable.md`
- `specs/022-deterministic-runtime/scenarios/scenario-open-question-signal.md`
- `specs/022-deterministic-runtime/scenarios/skipscanner-inline-code-exemption.md`
- `specs/022-deterministic-runtime/scenarios/spec-side-parser-hardening.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/022-deterministic-runtime/tasks.md`
- `specs/026-framework-self-audit/scenarios/families-follow-the-command-reference-split.md`
- `specs/026-framework-self-audit/scenarios/family-34-reference-section-reachability.md`
- `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/026-framework-self-audit/tasks.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/scenarios/an-open-fenced-block-or-comment-is-not-examined.md`
- `specs/041-task-pruning/spec.md`
- `specs/041-task-pruning/tasks.md`
- `specs/046-scenario-open-question-visibility/spec.md`
- `specs/055-shared-constitution/plan.md`
- `specs/057-analyze-artifact-and-record-relocation/analysis.md`
- `specs/057-analyze-artifact-and-record-relocation/data-model.md`
- `specs/057-analyze-artifact-and-record-relocation/review.md`
- `specs/057-analyze-artifact-and-record-relocation/scenarios/a-merge-carries-each-keys-whole-value.md`
- `specs/057-analyze-artifact-and-record-relocation/spec.md`
- `specs/057-analyze-artifact-and-record-relocation/tasks.md`
- `specs/066-audit-records-and-the-completion-gate/analysis.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/plan.md`
- `specs/066-audit-records-and-the-completion-gate/review.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/record-paths-are-quoted-only-when-they-need-it.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/066-audit-records-and-the-completion-gate/tasks.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-bare-port-span-names-its-remedy.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-none-list-item-is-no-open-question.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-plan-heading-is-matched-without-regard-to-case.md`
- `specs/067-analyze-and-audit-checks/scenarios/check-artifacts-skipped-targets.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md`
- `specs/067-analyze-and-audit-checks/scenarios/orphan-check-skips-ignored-local-state.md`
- `specs/067-analyze-and-audit-checks/scenarios/orphaned-reference-check.md`
- `specs/067-analyze-and-audit-checks/scenarios/promotion-coverage-header-without-a-key.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/071-command-reference-split/scenarios/reference-sections-are-reached-by-a-step.md`
- `specs/071-command-reference-split/spec.md`
- `specs/071-command-reference-split/tasks.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/scenarios/the-excluded-section-lists-paths-only.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/079-review-and-analysis-work-converges/tasks.md`
- `specs/080-deterministic-runtime-efficiency/plan.md`
- `specs/080-deterministic-runtime-efficiency/scenarios/a-step-gates-on-a-bounded-list-by-its-count.md`
- `specs/080-deterministic-runtime-efficiency/spec.md`
- `specs/080-deterministic-runtime-efficiency/tasks.md`
- `specs/081-reviews-and-checks-follow-what-changed/analysis.md`
- `specs/081-reviews-and-checks-follow-what-changed/plan.md`
- `specs/081-reviews-and-checks-follow-what-changed/review.md`
- `specs/081-reviews-and-checks-follow-what-changed/spec.md`

## Unexamined governance

*None.*
