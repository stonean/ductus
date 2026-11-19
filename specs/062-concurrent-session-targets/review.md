---
spec: 062-concurrent-session-targets
last-run: 2026-11-19T12:56:56Z
reviewed-against: a9a219dbf4d10842a8bd406973e6b5a04e8b0a5a
diff-base: 573b11d31151748d0aebf0d70cb73f0cedae27e2
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 3
scope: 3
unread: []
excluded: 15
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 2f139ba060dd47cd8145c26285e922cf1c4953880bfc2a11f1f64426749916ab
  scenarios/first-writers-create-the-ignore-file-once.md: 10654412621bdf8d3809f10d1559179ba335acba2ea2bca183a8042a12a79ca6
criteria-judged: 24
criteria-met: 24
criteria-verdicts:
  - label: AC1
    digest: 33c85de2c268970aaf14d4e5da2b0f6fdde10049585a9e92a7c99ba7dbf238f8
  - label: AC2
    digest: 606396cad37a8e3da00d2245d39622ec8b45e6e5c4bfacc27a5d68433a015c48
  - label: AC3
    digest: 08a26d3be120ad8d8eaebb872fc23038f7116f9e1b3fc58bd0239c578437c8ca
  - label: AC4
    digest: 16d946c95c1f5961d9f4f68790ec54860c963107b2a27738b11ebb70f7551985
  - label: AC5
    digest: 2bd8461bab6ed6ba59a52653f0f4bc65814adb541cc92f2c21c6d80783bf3bd7
  - label: AC6
    digest: 47465394e63a048bf20936cb0acad9e53ab88b7989950c1d9409856b0f2476e9
  - label: AC7
    digest: 799596ffdb2c76d695ccd691f01a54f92b746f58b9d06e0861b5e9b1464aafdd
  - label: AC8
    digest: c8aa296d915f50da4e5b0cb2fb5932559a5e4098d8045e2e657d80097bec9c68
  - label: AC12
    digest: 288f641ee4bc550037c82b4edfe04f8124f75a0daef8c2e0337ffad3d1930356
  - label: AC13
    digest: 2407278dd8abb9db3725dc20a3e0c4c28e98d190f80c82d1dcef44e42bc8aff0
  - label: AC14
    digest: 316ae0435897a77bc8818ded72674df8e5bbf46749e042f5e0c4c9dd99f68dc8
  - label: AC15
    digest: 83ef4ce3e991f73dfa2cfeb69863756f2d7008a4bff3fa1915b2534a503b6863
  - label: AC16
    digest: fe8ee336ce3205928ecdcf1ebf5152753f33c000d8c9c2202a746460808eaefe
  - label: AC17
    digest: 90e02bfcb9ed3b0946d91eab83a7d434968fef385d614a745bc83adb78fdc8af
  - label: AC18
    digest: 86b0a0e7ef796bb3993c835752a4d4508fae4422f5aed14906ec78f5747cc37c
  - label: AC19
    digest: 0025de695c315a32c74cf5b0bf459c02078fad1c337539550ae3e33f1c141ecc
  - label: AC20
    digest: 7a745cdb6e21974e63b16ea767f8be138f93cee80acac85add56af032b7646ee
  - label: AC21
    digest: c9279a5fdfb402cbb198a364c2f47ecf6f6f57350df3319135db8d316e3ff3c7
  - label: AC22
    digest: 3f25484b1e9081df93d33bfe121a608b0fab13313498576643c3e54104feed3c
  - label: AC23
    digest: 8fe92b15689d582715dbc2bc17d59d8a12a6580921eb9a11872c16ad577c8f56
  - label: AC24
    digest: 50599df6723dcb12c229e0619f2deb778681b8bfe24cca2a79947b77242c05a9
  - label: AC10
    digest: 40cad532d2cbc0cf026756a6aa46e1f64c3b76f6676c282917ca698e4578eede
  - label: AC25
    digest: d87c2223cd7fd790b0fe52e1704e026fc0ed3b6ee7d7a6e5f4ed15296fdefff4
  - label: AC11
    digest: 9b5aaabd8b824d999a897709d56d08af4f560c3eaf101712cafdbbbb8be36991
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: "test-gap: no test exercises the advisory lock itself; AC4's test passes without it — task 13 — `runtime/tests/concurrent_sessions.rs:178`"
    outcome: deferred
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "simplicity: two enums represent the removal cause, and the recorded cause is never read back — `runtime/src/primitives/retarget_sessions.rs`"
    outcome: waived
    reason: one match translates between them, and retarget-sessions, the only caller, validates the cause against the target, so the two cannot disagree
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "efficiency: every resolution parses every session file, and peek takes the exclusive lock — `runtime/src/session.rs:580`"
    outcome: waived
    reason: the files are sub-kilobyte and bounded by seven days of sessions; the cost is negligible beside a command's own process spawn
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "input: session names have no length limit — `runtime/src/session.rs:71`"
    outcome: waived
    reason: a name past the filesystem's name limit is not a realistic operator input, and its failure names the path rather than passing silently
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "quality: a used-at stamped in the future is never reported — `runtime/src/session.rs:429`"
    outcome: waived
    reason: a future stamp still expires seven days after it, so nothing accumulates; a clock set ahead is outside the design
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "reuse: inbox_standing keeps its own civil-from-days copy — `runtime/src/primitives/inbox_standing.rs:208`"
    outcome: waived
    reason: it predates 062 in spec 058's file, which 062 did not touch; consolidating date helpers is a refactor outside this spec
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "exec-path: fold's and consolidate's session step dispatches nothing under exec, and §concurrent-features states no exec carve-out — `framework/commands/fold.md:76`"
    outcome: waived
    reason: the reduction is documented in each step, the convention the two-paths guarantee sets for exec reductions, as clarify.md's scope note does
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "quality: a write overwrites the own record before storing the default and sweeping, so an error part-way loses the notice it replaced — task 20 — `runtime/src/session.rs:1256`"
    outcome: deferred
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "security: with a proxy set, fetch-archive's connection is tunnelled and its SSRF address pin no longer holds (pre-existing) — `runtime/src/primitives/fetch_archive.rs:130`"
    outcome: deferred
    target: specs/048-govern-acquired-runtime/scenarios/fetch-archive-reads-its-proxy-once.md
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "test: two lock tests rely on sleeps to reach the contended state — `runtime/src/session.rs:1452`"
    outcome: waived
    reason: neither can fail falsely, and the deterministic timeout and fresh-repo lock tests carry the behavior
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "concurrency: the legacy layout still read-modify-writes the default without the lock — `runtime/src/session.rs:393`"
    outcome: waived
    reason: "deliberate and recorded in the plan: creating .ductus/sessions/ beside a legacy file breaks 042's cutover rule, and it lasts only until /ductus migrates the repo"
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "ux: /target X announces adopting the feature it is about to leave — `framework/commands/target.md:29`"
    outcome: waived
    reason: "truthful: the session did adopt that target before the write replaced it"
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "reuse: the same 'Display what the write reports' sentence appears in four command files — `framework/commands/target.md:44`"
    outcome: waived
    reason: each command stating its own procedure in full is the command files' self-contained convention
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "accuracy: 048's new scenario says REQUEST_METHOD makes the matcher ignore HTTP_PROXY; it ignores every proxy variable — task 25 — `specs/048-govern-acquired-runtime/scenarios/fetch-archive-reads-its-proxy-once.md:21`"
    outcome: deferred
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T23:13:22Z
    decided-by: andy@stone.dev
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
  - key: "edge: the spec-root walk drops a directory it cannot list, so it is neither counted nor skipped — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: waived
    reason: a directory the process cannot list inside a repository it writes is contrived, and the walk's filter predates this window
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
  - key: "security: a peer session's record fields (`source`, `used-at`, `feature`, `scenario`, and the file stem used as its key) are rendered into another session's co-target, fold and consolidation notices as read from disk, with no re-check on read, so a hand-edited or planted file under `.ductus/sessions/` can put control, bidi or forged text into what another agent reads, while the data model says names are held to an allowlist because they are rendered into those notices; the allowlist runs at write time only. — `runtime/src/session.rs`"
    outcome: waived
    reason: The files are gitignored state the operator's own runtime writes, each name held to the slug allowlist (`validate_session_feature`, `validate_session_scenario`) on every write path; a writer able to plant one already holds the working tree, so no boundary is crossed, and no loaded rule's Statement covers re-validating that state on read (`BE-INPUT-001` names client, queue and ingested-file input).
    decided-at: 2026-11-19T12:56:56Z
    decided-by: rule
  - key: "other: AC7 reads as every file holding session state, but neither `framework/templates/project/gitignore` nor `.gitignore` ignores the legacy tiers `.govern/session.toml` and `.govern.session.toml`, which `runtime/src/schema/paths.rs` `SESSION_CHAIN` still reads; judged met as the 062 state files (the default, `.ductus/sessions/` and its lock) are all covered and legacy tiers are pre-049 state migrated by `/ductus` — `specs/062-concurrent-session-targets/spec.md`"
    outcome: waived
    reason: The legacy tiers `.govern/session.toml` and `.govern.session.toml` predate 062 and are read only as a fallback until `/ductus` runs `framework/migrations/session-file-consolidate.md`; AC7's subject is the state 062 introduced, which both gitignores cover, and adding retired names to the shipped template would be a new requirement, not a mechanical fix.
    decided-at: 2026-11-19T12:56:56Z
    decided-by: rule
---

# Review — 062-concurrent-session-targets

## Summary

Criteria review of 062's reopening for 048's sweep: the window runs from the reopen `573b11d3` and holds one 062 commit, `d9d06941`, and the record predates 066 task 27's criteria step, so all 24 criteria were owed. Passes and criteria were read at `9f372f24` and recorded against `a9a219db`, which changes no 062, runtime or rule file. The security pass is the previous run's part A, read whole at `d9d06941` on byte-identical rule files and scope files; this run's fix rewords three data-model sentences about a record's timestamp, its sweep and a symbol pointer, which no security Statement reaches, so that pass was not re-run over it. The other four passes read the nine other rule files, `AGENTS.md`, `specs/system.md` and `runtime/src/session.rs` whole, checked every claim in the data model and the scenario against the code, and re-ran over the fix diff and the changed data model. Every pass read all 3 scope entries. 0 MUST, 0 SHOULD, 0 low-confidence. 5 observations: 3 fixed in the run in `data-model.md`, a Group B directory, with the owner's approval (the shared default's shape names `DefaultRecord`, `used-at` is refreshed by resolutions and target or clear writes only, and the sweep also keeps a record whose `used-at` will not parse), and 2 waived by the rule. All 24 criteria judged met, each by reading its code and driving it: `write-session`, `resolve-session`, `retarget-sessions`, `render-status` and `ductus exec` on fixture projects with identities set and unset, sixteen concurrent writers, a seven-day expiry by an aged `used-at`, and `git check-ignore` against both gitignores; `cargo test --lib session` (146), the session primitives' filters (46) and `--test concurrent_sessions` (5) passed. Not run: `runtime/tests/exec_subprocess.rs`' identified-exec test, whose scenario AC23's verdict reproduced by hand, and the fold and consolidate command steps end to end, judged through the `retarget-sessions` primitive they call. Not re-derived: the dated 2026-09-29 measurements and CI run `36653211031`.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

24 of 24 acceptance criteria judged met.

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

- AC1 — met: `runtime/src/session.rs` `write` sets only the caller's own file and the default, and `retarget` rewrites only records naming `from`; `framework/commands/{specify,groom,amend,fold,consolidate}.md` write through `write-session` or `retarget-sessions`; proved by `runtime/tests/concurrent_sessions.rs` `a_write_in_one_process_never_moves_the_other`, lib `a_target_write_in_one_process_leaves_the_others_target_alone` and `a_fold_retargets_every_session_naming_the_spec_and_notifies_the_others` (bystander file byte-identical), and a CLI fixture fold and consolidation that left a session on another feature and scenario untouched
- AC2 — met: `runtime/src/session.rs` `resolve` returns the caller's own record and `framework/commands/*.md` resolve through `resolve-session`; proved by `runtime/tests/concurrent_sessions.rs` `a_write_in_one_process_never_moves_the_other` (resolve-session and render-status per process) and a CLI fixture where `DUCTUS_SESSION=a` and `b` resolved `055-a` and `056-b`; no command reads `.ductus/session.toml` directly except the host-config write in `framework/bootstrap/govern.md`
- AC3 — met: `runtime/src/session.rs` `DefaultRecord` keeps `feature`, `path`, `scenario`, `scenario-path`, `set-at`, `cli-config-dir` in the old order and `resolve_unidentified` reads only the default; proved by `runtime/src/primitives/write_session.rs` `a_single_unidentified_agent_writes_todays_bytes_and_nothing_else` and `runtime/src/session.rs` `cli_config_dir_survives_every_write` (exact bytes); no migration is involved in `write` or `resolve`
- AC4 — met: `runtime/src/session.rs` `write` holds `lock_for_write` across the read-modify-write of the default and the own-file store, with jittered backoff in `lock_file_waiting`; proved by `runtime/tests/concurrent_sessions.rs` `concurrent_writers_each_keep_their_own_target` (16 processes, each resolves its own target, default whole) and a CLI run of 8 identified plus 8 unidentified concurrent writers that all kept their targets
- AC5 — met: `runtime/src/session.rs` `retarget` re-points or clears the default and every `ProcessRecord` naming `from`, and writes no other record; proved by lib `a_fold_retargets_every_session_naming_the_spec_and_notifies_the_others`, `a_consolidation_clears_every_session_naming_the_spec`, `runtime/src/primitives/retarget_sessions.rs` `a_fold_moves_every_session_on_the_folded_spec`, and CLI fold then consolidate on a fixture where no target named the removed spec afterward and a session on `057-c` kept its scenario
- AC6 — met: `runtime/src/session.rs` `write` loads the default with `read_default_lenient` and keeps `cli_config_dir` unless overridden, for target, clear and identified writes alike; proved by lib `cli_config_dir_survives_every_write`, `runtime/src/primitives/write_session.rs` `an_identified_clear_write_clears_its_own_file_and_keeps_cli_config_dir` and `preserves_cli_config_dir_across_a_target_switch`, and a CLI fixture whose `.pi` value survived identified writes, a fold and a consolidation
- AC7 — met: `framework/templates/project/gitignore` and `.gitignore` each carry `/.ductus/session.toml` and `/.ductus/sessions/`, and `ignore_everything` in `runtime/src/session.rs` writes `.ductus/sessions/.gitignore`; proved by `git check-ignore -v` in a fixture repo against both files (session.toml, a per-process file and `.lock` all ignored, `.ductus/config.toml` not) and lib `the_sessions_directory_ignores_itself`; the pre-049 legacy tiers are migrated state and not ignored by either file, see notes
- AC8 — met: `runtime/src/primitives/render_status.rs` `run` passes `process_identity()` to `dashboard::view_model`, which peeks the process's own target, and `runtime/src/primitives/pipeline_view.rs` prints `Target: ... — session {label}`; proved by `runtime/tests/concurrent_sessions.rs` `a_write_in_one_process_never_moves_the_other` (first line `Target: 055-a ... — session alpha`), lib `the_view_names_this_process_target_and_session`, and CLI render-status runs as `y`, `x` and unidentified naming each process's own target or `No session target.`
- AC12 — met: `runtime/src/session.rs` `identity_from_env` checks `DUCTUS_SESSION` before `PLATFORM_IDENTITIES` and `own_path` keys the file on the sanitized name; proved by lib `ductus_session_takes_precedence_over_the_platform_identity` and `two_processes_with_the_same_name_share_one_identity`, and CLI runs where `CLAUDE_CODE_SESSION_ID` plus `DUCTUS_SESSION=zz` wrote `zz.toml`, `a` and `b` held separate targets, and `same` and `SAME` shared one `same.toml`
- AC13 — met: `runtime/src/session.rs` `PLATFORM_IDENTITIES` maps `CLAUDE_CODE_SESSION_ID` to source `claude-code` and `init_process_identity` captures it at startup; proved by `runtime/tests/concurrent_sessions.rs` `two_claude_code_sessions_hold_separate_targets` (separate targets, identity `claude-code:3f2a9c1d`), lib `claude_code_session_id_identifies_a_process_with_no_name` and `distinct_platform_ids_are_distinct_identities`, and a CLI write that created `.ductus/sessions/3f2a9c1d-....toml`
- AC14 — met: `runtime/src/session.rs` `identity_from_env` treats a blank `DUCTUS_SESSION` as unset, sanitizes through `derive_slug`, and returns `InvalidArgument` naming `DUCTUS_SESSION` when it sanitizes to nothing; proved by lib `empty_ductus_session_is_unset`, `a_name_is_sanitized_by_the_slug_rule`, `a_name_that_sanitizes_to_nothing_is_refused_naming_the_variable`, `runtime/tests/concurrent_sessions.rs` `an_unusable_session_name_is_refused_at_the_edge`, and CLI runs (`My Review!` wrote `my-review.toml`, empty resolved the default, `!!!` exited 1 naming the variable)
- AC15 — met: `runtime/src/session.rs` `resolve_unidentified` reads only the default and `write` stores no own record without an identity while `lock_for_write` takes the lock on the `.ductus/` layout, as the Superseded annotation states; proved by `runtime/tests/concurrent_sessions.rs` `two_unidentified_processes_share_the_default` (dotfiles only in `.ductus/sessions/`), lib `an_unidentified_process_touches_only_the_default` and `an_unidentified_write_in_a_fresh_repo_takes_the_lock`, and a CLI unidentified render-status and resolve that read the default
- AC16 — met: `runtime/src/session.rs` `write` calls `default.set_target` for every Target and Clear shape before storing the default, identified or not, and `adoption` seeds a new identity from it; proved by lib `the_default_always_holds_the_latest_target_change` and `a_restarted_agent_resumes_the_most_recent_target`, and a CLI fixture where the default held the latest writer's target after `a` then `b` wrote and an unidentified resolve returned it
- AC17 — met: `runtime/src/session.rs` `resolve`/`adoption` pin and announce the default via `adopted_notice`; CLI drive: x adopted `055-a` with an adopted notice, then y's write left x `own` `055-a`; proved by lib test `a_process_adopts_the_default_once_and_is_pinned_to_it`.
- AC18 — met: `runtime/src/session.rs` `sweep`/`idle_past_expiry`/`IDLE_EXPIRY` (7 days, `>` strict) run from `write` under the lock, and `resolve` refreshes `used-at`; CLI drive: an aged `y.toml` was reported `expired:[y]` by x's write; tests `a_target_idle_past_seven_days_is_swept_and_one_within_it_is_kept` and `resolving_counts_as_use_and_the_writer_is_never_swept`.
- AC19 — met: `runtime/src/session.rs` `peers_of`/`due_notices`/`co_target_notice` and `write` `outcome.peers`; CLI drive: y's write returned `peers:[x]` with `last-used`, x was told once then silent, a no-identity write returned no peers; `peek` shows without consuming (test `a_peek_answers_like_resolve_and_writes_nothing`); test `co_targeting_is_noticed_once_per_side_and_again_when_the_set_changes`, annotation holds.
- AC20 — met: `runtime/src/session.rs` `ProcessRecord::set_target` sets `cleared`, `resolve` returns `Cleared` and never calls `adoption` for an existing file; CLI drive: cleared x stayed `cleared` after the default moved, a consolidation cleared b, an aged cleared record was swept and x then adopted with an announcement; tests `a_cleared_process_never_adopts_the_default`, `a_consolidation_clears_every_session_naming_the_spec`.
- AC21 — met: `runtime/src/session.rs` `retarget` and `RemovalNotice`/`removal_notice`, `runtime/src/primitives/retarget_sessions.rs` `run_as`, `framework/commands/fold.md` step 13 and `framework/commands/consolidate.md` step 6 call it and their reports name the sessions changed; CLI drive: fold re-pointed default, a and b and b was told `folded ... by session a`, consolidation cleared all and told b `consolidated`, default changed with no notice; tests `a_fold_retargets_every_session_naming_the_spec_and_notifies_the_others`, `a_write_delivers_the_removal_notice_it_replaces`.
- AC22 — met: `runtime/src/session.rs` `ProcessRecord::load`, `read_default_strict`, `peers_of`/`sweep`/`retarget` listing `unreadable`; CLI drive: a malformed own file and a malformed default each made `resolve-session` exit 1 naming the file, with no adoption; tests `a_malformed_own_file_is_an_error_never_an_adoption`, `a_resolution_names_the_peer_files_it_could_not_check`, `a_removal_reports_files_it_cannot_read_and_leaves_them`.
- AC23 — met: `runtime/src/main.rs` `run_exec` calls `process_identity` and `seed_context`, which peeks the identified process's own record; CLI drive of `ductus exec` on a fixture: with `CLAUDE_CODE_SESSION_ID` it read specs `056-b` (own), with no identity it failed on the default's `055-a`; main.rs tests `an_identified_exec_seeds_its_own_target_without_writing`, `an_unidentified_exec_seeds_the_shared_default`, and `runtime/tests/exec_subprocess.rs` `exec_run_as_an_identified_process_acts_on_its_own_target` (read, not run).
- AC24 — met: Judged as annotated (superseded by 079): `framework/constitution.md` §Host integration says without the server a command stops, `framework/commands/target.md` step 1 invokes `resolve-session`, and `runtime/src/primitives/resolve_session.rs` `run` reads `session::process_identity`; grep of `framework`, `runtime/src` and `docs` finds no markdown-only path text.
- AC10 — met: `framework/constitution.md` §Concurrent Features opens with `Each agent process holds its own session target`, identified by `DUCTUS_SESSION` or `CLAUDE_CODE_SESSION_ID`, in `.ductus/sessions/`, in place of the single-target text; read whole.
- AC25 — met: Judged as annotated (superseded in part by 050): `framework/constitution.md` §Concurrent Features keeps `git worktree` and platform isolation for edits and states both bounds (processes with no identity share one target; two agent CLIs read command files from the agent `/ductus` last recorded); the git-refuses-one-branch-twice note is absent, as the annotation records.
- AC11 — met: `specs/010-agent-autonomy/spec.md` §Parallel milestones carries a `> **Note (post-completion, superseded by [062-concurrent-session-targets](../062-concurrent-session-targets/spec.md), 2026-09-29)` blockquote linking 062 beneath the verdict, and Resolved Question 5 repeats the pointer; 062 `cross-spec-impact` lists 010.

## Observations

- security: a peer session's record fields (`source`, `used-at`, `feature`, `scenario`, and the file stem used as its key) are rendered into another session's co-target, fold and consolidation notices as read from disk, with no re-check on read, so a hand-edited or planted file under `.ductus/sessions/` can put control, bidi or forged text into what another agent reads, while the data model says names are held to an allowlist because they are rendered into those notices; the allowlist runs at write time only. — `runtime/src/session.rs` — **waived**: The files are gitignored state the operator's own runtime writes, each name held to the slug allowlist (`validate_session_feature`, `validate_session_scenario`) on every write path; a writer able to plant one already holds the working tree, so no boundary is crossed, and no loaded rule's Statement covers re-validating that state on read (`BE-INPUT-001` names client, queue and ingested-file input). (decided by the rule)
- convention: the data model points the shared default's shape at `SessionRecord` in `runtime/src/primitives/write_session.rs`, a symbol that file no longer holds (the shape is now `DefaultRecord` in `runtime/src/session.rs`; it does exist at the pinned `25c946ea`), so a reader at HEAD cannot find it. — `specs/062-concurrent-session-targets/data-model.md` — **fixed** (decided by the rule)
- other: the data model says `used-at` is refreshed on every resolution and write, but `session::write` refreshes it only for a target or clear write, and a host-config write or a `peek` leaves the own file untouched. — `specs/062-concurrent-session-targets/data-model.md` — **fixed** (decided by the rule)
- other: the data model says the sweep reports an unparseable file and leaves it, but `sweep` also leaves a file whose `used-at` is missing or not an ISO 8601 UTC stamp, listing it in `unreadable`, since idleness that cannot be established is never assumed. — `specs/062-concurrent-session-targets/data-model.md` — **fixed** (decided by the rule)
- other: AC7 reads as every file holding session state, but neither `framework/templates/project/gitignore` nor `.gitignore` ignores the legacy tiers `.govern/session.toml` and `.govern.session.toml`, which `runtime/src/schema/paths.rs` `SESSION_CHAIN` still reads; judged met as the 062 state files (the default, `.ductus/sessions/` and its lock) are all covered and legacy tiers are pre-049 state migrated by `/ductus` — `specs/062-concurrent-session-targets/spec.md` — **waived**: The legacy tiers `.govern/session.toml` and `.govern.session.toml` predate 062 and are read only as a fallback until `/ductus` runs `framework/migrations/session-file-consolidate.md`; AC7's subject is the state 062 introduced, which both gitignores cover, and adding retired names to the shipped template would be a new requirement, not a mechanical fix. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `docs/developing-ductus.md` — 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/derive_routing_candidates.rs` — 079-review-and-analysis-work-converges
- `scripts/audit/promotion-coverage.sh` — 067-analyze-and-audit-checks
- `specs/020-code-review/spec.md` — 020-code-review
- `specs/048-govern-acquired-runtime/tasks.md` — 048-govern-acquired-runtime
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 066-audit-records-and-the-completion-gate
- `specs/067-analyze-and-audit-checks/review.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/spec.md` — 067-analyze-and-audit-checks
- `specs/079-review-and-analysis-work-converges/analysis.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/plan.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/review.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/spec.md` — 079-review-and-analysis-work-converges
- `specs/081-reviews-and-checks-follow-what-changed/review.md` — 081-reviews-and-checks-follow-what-changed
- `specs/081-reviews-and-checks-follow-what-changed/spec.md` — 081-reviews-and-checks-follow-what-changed

## Unexamined governance

*None.*
