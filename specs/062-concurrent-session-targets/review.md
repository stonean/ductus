---
spec: 062-concurrent-session-targets
last-run: 2026-10-28T20:22:24Z
reviewed-against: c1cf51ffa655635ef1b56b977023762f596c1aaa
diff-base: f056b2096de068854870be0b8de1e4e756326ffc
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 17
scope: 135
skipped-passes: []
reviewed-digest:
  data-model.md: 9ce5458a99a6ba857f8778df877a8b2b0a2708037b23b05319588b4c06aaa10f
  scenarios/first-writers-create-the-ignore-file-once.md: 02149b44fd9c6d76d8e9d97af6235df746bf028cad34e693bb57302dea1b1f15
blocking: false
dispositions:
  fixed: 76
  routed: 4
  discarded: 27
  undispositioned: 0
decisions:
  - key: "contract: `/target X` and `--clear` overwrite a pending fold or consolidation notice undelivered, since target resolves only without an argument (AC21) — task 17 — `framework/commands/target.md:29`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "contract: no command displays write-session's `peers`, so the writing session is never told who shares its feature (AC19) — task 17 — `framework/commands/target.md:44`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "contract: fold's and consolidate's report steps name neither the other sessions re-pointed or cleared nor unreadable files (AC21, AC22) — task 17 — `framework/commands/consolidate.md:80`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "contract: `ductus exec` consumes resolve-session's notices without emitting them (AC19, AC21) — task 18 — `runtime/src/interpreter/mod.rs:237`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "test-gap: no exec test runs with an identity set, so AC23 is untested where run_exec reads it — task 18 — `runtime/src/main.rs:586`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "test-gap: no test exercises the advisory lock itself; AC4's test passes without it — task 13 — `runtime/tests/concurrent_sessions.rs:178`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "simplicity: two enums represent the removal cause, and the recorded cause is never read back — `runtime/src/primitives/retarget_sessions.rs`"
    outcome: discarded
    reason: one match translates between them, and retarget-sessions, the only caller, validates the cause against the target, so the two cannot disagree
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "efficiency: every resolution parses every session file, and peek takes the exclusive lock — `runtime/src/session.rs:580`"
    outcome: discarded
    reason: the files are sub-kilobyte and bounded by seven days of sessions; the cost is negligible beside a command's own process spawn
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "input: session names have no length limit — `runtime/src/session.rs:71`"
    outcome: discarded
    reason: a name past the filesystem's name limit is not a realistic operator input, and its failure names the path rather than passing silently
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "quality: a used-at stamped in the future is never reported — `runtime/src/session.rs:429`"
    outcome: discarded
    reason: a future stamp still expires seven days after it, so nothing accumulates; a clock set ahead is outside the design
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "reuse: inbox_standing keeps its own civil-from-days copy — `runtime/src/primitives/inbox_standing.rs:208`"
    outcome: discarded
    reason: it predates 062 in spec 058's file, which 062 did not touch; consolidating date helpers is a refactor outside this spec
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "reuse: consolidate.md restates §concurrent-features' reasoning in its step and its markdown-only reference — `framework/commands/consolidate.md:77`"
    outcome: discarded
    reason: the restatement predates 062, and a step plus its markdown-only reference both describing the procedure is the command files' two-path convention
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "terminology: 'agent identity' (cli-config-dir) now sits beside 'session identity' — `framework/constitution.md:775`"
    outcome: discarded
    reason: "'agent identity' names cli-config-dir in ten passages that predate 062; renaming it is a terminology sweep outside this spec"
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "exec-path: fold's and consolidate's session step dispatches nothing under exec, and §concurrent-features states no exec carve-out — `framework/commands/fold.md:76`"
    outcome: discarded
    reason: the reduction is documented in each step, the convention the two-paths guarantee sets for exec reductions, as clarify.md's scope note does
    decided-at: 2026-09-29T20:47:19Z
    decided-by: andy@stone.dev
  - key: "quality: a write overwrites the own record before storing the default and sweeping, so an error part-way loses the notice it replaced — task 20 — `runtime/src/session.rs:1256`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "regression: /target X now fails at step 1 on a malformed own session file it used to repair by overwriting — task 20 — `framework/commands/target.md:29`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "security: with a proxy set, fetch-archive's connection is tunnelled and its SSRF address pin no longer holds (pre-existing) — `runtime/src/primitives/fetch_archive.rs:130`"
    outcome: routed
    target: specs/048-govern-acquired-runtime/scenarios/fetch-archive-reads-its-proxy-once.md
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "test: two lock tests rely on sleeps to reach the contended state — `runtime/src/session.rs:1452`"
    outcome: discarded
    reason: neither can fail falsely, and the deterministic timeout and fresh-repo lock tests carry the behavior
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "concurrency: the legacy layout still read-modify-writes the default without the lock — `runtime/src/session.rs:393`"
    outcome: discarded
    reason: "deliberate and recorded in the plan: creating .ductus/sessions/ beside a legacy file breaks 042's cutover rule, and it lasts only until /ductus migrates the repo"
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "ux: /target X announces adopting the feature it is about to leave — `framework/commands/target.md:29`"
    outcome: discarded
    reason: "truthful: the session did adopt that target before the write replaced it"
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "reuse: the same 'Display what the write reports' sentence appears in four command files — `framework/commands/target.md:44`"
    outcome: discarded
    reason: each command stating its own procedure in full is the command files' self-contained convention
    decided-at: 2026-09-29T22:31:00Z
    decided-by: andy@stone.dev
  - key: "regression: with a malformed shared default, /target X can no longer repair it and every exec walk halts at the seed — task 24 — `framework/commands/target.md:29`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T23:13:22Z
    decided-by: andy@stone.dev
  - key: "exec-path: target's continue-past-a-malformed-own-file path has no exec equivalent, and the step does not say so — task 24 — `runtime/src/main.rs:505`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T23:13:22Z
    decided-by: andy@stone.dev
  - key: "accuracy: 048's new scenario says REQUEST_METHOD makes the matcher ignore HTTP_PROXY; it ignores every proxy variable — task 25 — `specs/048-govern-acquired-runtime/scenarios/fetch-archive-reads-its-proxy-once.md:21`"
    outcome: routed
    target: specs/062-concurrent-session-targets/tasks.md
    decided-at: 2026-09-29T23:13:22Z
    decided-by: andy@stone.dev
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
---

# Review — 062-concurrent-session-targets

## Summary

One in-depth review of the window f056b209..c1cf51ff, recorded in each spec it touched. The window opens after f056b209, the round that reopened this spec, so its passes are the re-reads below. Independent re-reads followed each fix round that changed code or a durable contract, each re-running the earlier probes: of f056b209 (5 SHOULD, 11 NOTE, fixed in 47e68233); of 47e68233 (5 SHOULD, 9 NOTE, fixed in 77fff147 beside f33096a5's analysis-round fixes); of f33096a5, 77fff147, 38fc39f6 (051 task 30) and 4e82686d with db6ae819 (050 task 41), 10 SHOULD and 8 NOTE, fixed in 54715126; of 54715126 (1 SHOULD, fixed in 3fc5755a); of 3fc5755a (3 SHOULD, fixed in d1853444); of d1853444 (2 SHOULD, both test gaps, fixed in c1cf51ff); and of c1cf51ff, 0 MUST, 0 SHOULD and 4 NOTE: the two comment sentences it left unclear or incomplete were fixed in a prose-only round that was not re-read, and its two test gaps were discarded as contrived. The 107 observations below are those findings and the reviewers' notes: 76 fixed, 4 routed, 27 discarded with their reasons. Mutation lists for the window's runtime changes ran whole and every mutation was killed. Examined counts the 17 in-scope files the passes read in full, of 135 in scope. Not counted: files read in part, among them the large runtime modules' test sections and the 021, 062 and 066 analysis records; the generated .claude/, .pi/ and .ductus/ copies, checked only against their sources; another session's rules sweep, 70bb4af5, inside this window, whose files the passes did not read; and plan Affected Files the window did not touch. Stored decisions are carried byte-identical through process-decisions' restricted mode: this window's passes did not re-read the artifacts they concern.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- bug: a referrer that is not UTF-8 text stopped rewrite-spec-links after the spec-root walk had written files, where check-orphaned-references skips the same file — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- bug: a referrer that is a symbolic link was skipped with nothing in the result naming it (QUAL-CLAIM-001) — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- bug: a referrer reached through a directory linked outside the project was rewritten there — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- bug: a reference re-pointed at a feature-only target kept a tail such as plan.md and landed on the target's file of that name, which the fold never wrote, while the orphan check stayed clean, and fold step 11 said inbound links land on the file the content reached — `framework/commands/fold.md` — **fixed**
- bug: a referrer line whose first token's fragment ran into a second token panicked rewrite-spec-links after the spec root was written — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- bug: the ductus:memory-routing block no longer said the framework's constitution is upstream, so an agent could file a learning into .ductus/constitution.md, which the next run replaces — `framework/bootstrap/ductus.md` — **fixed**
- stale: knowledge-routes-by-population-not-by-kind said the corrected routing reaches new adoptions only, and named four places where its table lists two — `specs/050-constitution/scenarios/knowledge-routes-by-population-not-by-kind.md` — **fixed**
- stale: 051's plan said the rewrite is scoped to markdown under the spec root, though it now rewrites the root referrers too — `specs/051-branch-scoped-spec-numbering/plan.md` — **fixed**
- convention: a #fragment containing a dot is cut at the dot on a fold into a scenario — `runtime/src/primitives/rewrite_spec_links.rs` — **discarded**: GitHub's heading slugs carry no dots, so such a fragment names no heading a link in this corpus can reach; a contrived input
- stale: 050's plan recorded the window base's constitution character counts and a total matching neither commit — `specs/050-constitution/plan.md` — **fixed**
- stale: memory-routing-lives-in-agents-md described the migration's match as byte-identical and its bound as the next second-level heading — `specs/050-constitution/scenarios/memory-routing-lives-in-agents-md.md` — **fixed**
- convention: no numbered step of the bootstrap names the two managed-block merges — `framework/bootstrap/ductus.md` — **discarded**: Instructions step 4 lists what apply-manifest replaces, not every Shared Files step, and two real adopter bootstraps on this text wrote both blocks from Shared Files
- convention: whether Auggie resolves CLAUDE.md's @AGENTS.md import, and so reads the routing block, is unverified — `framework/templates/project/claude-md.md` — **discarded**: the rule's canonical statement is the constitution's drift-prevention section, which target step 4 loads whenever an import did not, so an Auggie session holds it either way
- bug: 3b359742's mutations were never registered, so its 11-of-11 claim could not be re-run — `scripts/mutation/rewrite-spec-links.py` — **fixed**
- bug: memory-routing-leaves-claude-md ran the section to the end of the file, so a ductus:constitutions block an earlier run appended after it made every shipped section read as edited — `framework/migrations/memory-routing-leaves-claude-md.md` — **fixed**
- bug: every file apply-manifest installs landed group-writable, the pre-commit hook and Pi's bridge among them, because extract-archive kept the tarball's 0664 and 0775 — `runtime/src/primitives/extract_archive.rs` — **fixed**
- bug: adopters from v0.47.0 to v0.59.11 keep a ductus:constitutions comment region whose comment says ductus maintains it, beside the block ductus does maintain — `framework/migrations/legacy-constitutions-region.md` — **fixed**
- bug: a run the Frontmatter Migration precheck halts after generator-primitives leaves a pre-0.30 hook calling the scripts that migration deleted, so the commit the refusal asks for fails — `framework/bootstrap/ductus-procedure.md` — **routed** to `specs/013-text-first-artifacts/scenarios/the-precheck-ignores-what-the-same-run-wrote.md`
- convention: the bootstrap's memory-routing step left the host to tell an append from a fill — `framework/bootstrap/ductus.md` — **fixed**
- convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md` — **discarded**: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
- convention: generator-primitives and audit-record-relocate state no report line for their no-op cases — `framework/migrations/generator-primitives.md` — **discarded**: an observation about the migration machinery outside this window; the host's own wording reported each outcome truthfully
- convention: the Closing restart gives no single form for Pi wired in the run beside Claude already registered — `framework/bootstrap/ductus-procedure.md` — **discarded**: the per-agent forms compose into an accurate notice, as the subject's run shows, so no host reads it wrong
- convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md` — **discarded**: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
- convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md` — **discarded**: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
- convention: the Frontmatter Migration's specs/**/spec.md walk reports specs/templates/spec.md as already frontmatter on every run — `framework/bootstrap/ductus-procedure.md` — **discarded**: the template has frontmatter, so the walk skips it with a report line and writes nothing
- convention: fetch-archive sends no User-Agent — `runtime/src/primitives/fetch_archive.rs` — **routed** to `specs/048-govern-acquired-runtime/tasks.md`
- convention: a .githooks linked into a host configuration directory inside the project is not judged by host_config::check_destination — `runtime/src/primitives/rewrite_spec_links.rs` — **discarded**: a contrived layout; the write stays inside the project, and the referrer list is fixed to five files none of which is a host's configuration
- bug: a scenario-route fold re-pointed a reference to one of the staged spec's carried scenarios at the routed scenario, listing nothing — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- stale: 051 AC22 said the orphan check reports clean afterwards save for skipped referrers, though a kept tail naming a file the target lacks is reported — `specs/051-branch-scoped-spec-numbering/spec.md` — **fixed**
- stale: 051's data-model said skipped holds referrers outside the spec root, though system.md is listed too — `specs/051-branch-scoped-spec-numbering/data-model.md` — **fixed**
- stale: legacy-constitutions-region's report and Background said the BEGIN/END block carries the imports in AGENTS.md, which a claude-only project never gets — `framework/migrations/legacy-constitutions-region.md` — **fixed**
- bug: the mode fix reached only files a run rewrote, since apply-manifest returned unchanged before re-applying the mode, so an installed hook stayed group-writable — `runtime/src/primitives/apply_manifest.rs` — **fixed**
- bug: a carried scenario whose slug was already taken upstream was reported and the fold went on, so step 11 re-pointed its references at an unrelated scenario and step 12 deleted the only copy — `framework/commands/fold.md` — **fixed**
- convention: fold's resumption row for step 11 says a re-run rewrites nothing, which a fragment running into a second token contradicts — `framework/commands/fold.md` — **discarded**: a contrived input: a fragment that is itself a second spec-root path, which the rewrite now leaves for the orphan check to report
- edge: legacy-constitutions-region did not decide a region followed only by blank lines — `framework/migrations/legacy-constitutions-region.md` — **fixed**
- stale: a rewrite-spec-links test doc called a CLAUDE.md symlinked to AGENTS.md a common layout — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- stale: fold step 14 said the orphan check reports what a skipped referrer still names, untrue of a file that is not UTF-8 — `framework/commands/fold.md` — **fixed**
- stale: installed-files-are-not-group-writable described the old rwxrwxrwx mask in the present tense — `specs/022-deterministic-runtime/scenarios/installed-files-are-not-group-writable.md` — **fixed**
- test: the read_referrers arm for a referrer behind a path component that is a regular file had no test — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- edge: the spec-root walk drops a directory it cannot list, so it is neither counted nor skipped — `runtime/src/primitives/rewrite_spec_links.rs` — **discarded**: a directory the process cannot list inside a repository it writes is contrived, and the walk's filter predates this window
- edge: a root-absolute /specs/ link is neither re-pointed nor reported — `runtime/src/primitives/check_orphaned_references.rs` — **routed** to `specs/051-branch-scoped-spec-numbering/tasks.md`
- edge: an archive entry whose only mode bits are group or other write lands at the umask default — `runtime/src/primitives/extract_archive.rs` — **discarded**: a contrived entry: no archive the framework ships carries a file with neither owner read nor owner write
- stale: 050's plan dated its character figures to the last commit to change the three files, which the next AGENTS.md edit ends — `specs/050-constitution/plan.md` — **fixed**
- bug: apply-manifest's re-applied mode was fatal on a file the runner may not chmod, stopping the walk mid-manifest — `runtime/src/primitives/apply_manifest.rs` — **fixed**
- bug: fold step 7's recovery, renaming one of two colliding scenarios, re-pointed the old slug's links silently at the unrelated upstream scenario — `framework/commands/fold.md` — **fixed**
- stale: the rewrite-spec-links MCP tool description said a scenario target replaces every tail — `runtime/src/mcp/server.rs` — **fixed**
- stale: 051 AC22's kept-tails clause held only for the root referrers, and AC6 named only the skipped exception — `specs/051-branch-scoped-spec-numbering/spec.md` — **fixed**
- bug: the session-lock poll and 062's plan stated no maximum attempt count, which BE-RETRY-001's Statement asks for beside the deadline — `runtime/src/session.rs` — **fixed**
- stale: scope-results-have-a-bounded-size credited the review reference with the redirect instruction the review command's step 1 gives — `specs/066-audit-records-and-the-completion-gate/scenarios/scope-results-have-a-bounded-size.md` — **fixed**
- stale: session-lock-backoff.py claimed mutation coverage of the WouldBlock-only retry it has no mutation for — `scripts/mutation/session-lock-backoff.py` — **fixed**
- stale: 071 task 23's evidence named the 12-case walk after the 15-case re-walk — `specs/071-command-reference-split/tasks.md` — **fixed**
- edge: legacy-constitutions-region was edited in place after a --ref=main push, so an adopter who ran the earlier text keeps its trailing blank lines — `framework/migrations/legacy-constitutions-region.md` — **discarded**: the migration is unreleased, reached only --ref=main adopters for the hours between f056b209 and 47e68233, and the residue is trailing blank lines after a removed comment region, which change no behavior
- convention: legacy-constitutions-region step 7 said the blank lines before the start line, meaning the adjacent run — `framework/migrations/legacy-constitutions-region.md` — **fixed**
- stale: legacy-constitutions-region's Background said a file no selected agent reads keeps no block, untrue after another agent's run wrote one — `framework/migrations/legacy-constitutions-region.md` — **fixed**
- stale: 062 task 23's checked item still gives the fixed-interval rationale for the lock poll — `specs/062-concurrent-session-targets/tasks.md` — **discarded**: a ticked task records the decision as it was taken, task 28 records its reversal, and tasks.md is ephemeral work tracking that prune may drop
- stale: rewrite_spec_links's panic-guard comment said a scenario target always drops the fragment — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- test: installed-file-modes M5's kill depends on ctime resolution finer than 50 ms — `scripts/mutation/installed-file-modes.py` — **discarded**: disclosed in the list's docstring; APFS and ext4 stamp ctime in nanoseconds, and the test itself passes on any file system
- stale: the rewrite-spec-links `to` argument doc said a scenario target lists nothing, which 77fff147's listing of an unheld scenario tail made false — `runtime/src/schema/primitives.rs` — **fixed**
- stale: the check-orphaned-references MCP description listed four referrers where the check reads five — `runtime/src/mcp/server.rs` — **fixed**
- bug: the CLI help gave check-corpus-links the orphan check's line and check-orphaned-references its Args doc, since a variant was inserted between that doc comment and its own — `runtime/src/main.rs` — **fixed**
- edge: a leading / after ~, } or % was read as naming the project root, so ~/.ductus/bin/ductus was checked and re-pointed as a project path — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- security: create-feature did not contain its derived directory, so a spec root linked outside the project received the new spec (BE-INPUT-004), though 051's plan said it was validated — `runtime/src/primitives/create_feature.rs` — **fixed**
- test: create-feature-claim M3 was declared a survivor reached only by a race, though a dangling link at the derived name reaches the lost claim every time (QUAL-TEST-001) — `scripts/mutation/create-feature-claim.py` — **fixed**
- stale: the cut sentence naming the checks the completion gate composes with is one 046's Resolved Question rests on, though the plan said no tracked file quoted a cut — `framework/constitution.md` — **fixed**
- stale: apply-manifest's Errors doc said an unchangeable mode is an Io error, where PermissionDenied names the file in modes-kept — `runtime/src/primitives/apply_manifest.rs` — **fixed**
- convention: the completion message's list had no modes-kept line, which bootstrap step 4 promises — `framework/bootstrap/ductus-procedure.md` — **fixed**
- stale: five shipped statements said every tail but a held scenario's collapses on a scenario target, where any scenarios/{name} tail is kept — `framework/reference/fold.md` — **fixed**
- stale: rewrite-spec-links' kept-tail comments named only the feature-target predicate — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- convention: a test's doc comment was split from its test by an inserted test — `runtime/src/primitives/apply_manifest.rs` — **fixed**
- stale: the revision-qualified-path scenario stated the token rule without the root-prefix exception — `specs/067-analyze-and-audit-checks/scenarios/a-revision-qualified-path-is-not-a-working-tree-reference.md` — **fixed**
- stale: create-feature's comment and 051's plan said concurrent creation under one identifier refuses the loser, which holds for one name only — `specs/051-branch-scoped-spec-numbering/plan.md` — **fixed**
- edge: an @/specs/x import in CLAUDE.md is read as project-relative, where Claude Code reads an absolute path — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: an absolute import of a top-level /specs directory is a contrived input
- convention: a kept-tails entry gives the reference from the spec root, without a prefix kept on the line — `runtime/src/primitives/rewrite_spec_links.rs` — **discarded**: its path and line locate the reference, and the check reports the same span from the root
- edge: a ./ prefix in the hook or a root markdown file assumes the project root is the work-tree root, which a project in a repository subdirectory breaks — `specs/067-analyze-and-audit-checks/data-model.md` — **discarded**: a ./ resolves as the bare paths in those files already do, the assumption every bare reference carries, not one task 30 adds
- stale: 062 task 28's ticked item still said its mutation run was owed — `specs/062-concurrent-session-targets/tasks.md` — **fixed**
- stale: the session lock's docs named only the timeout as a bound on the attempts — `runtime/src/session.rs` — **fixed**
- convention: 050's plan narrated the search behind the cut, verification evidence the commit holds — `specs/050-constitution/plan.md` — **fixed**
- stale: the analyze reference listed the managed roots without the historical .govern/, scripts/gen- and scripts/lib/ — `framework/reference/analyze.md` — **fixed**
- test: the Family 19 parity tests run git in their fixtures under a hook's inherited GIT_DIR, so a commit from a linked worktree fails four of them — `runtime/tests/mechanical_sweep_parity.rs` — **routed** to `specs/066-audit-records-and-the-completion-gate/tasks.md`
- edge: the round-3 fix's opener allowlist refused a prefix after |, >, & or a typographic quote, though the bare spelling is found, so a fold left such a reference dangling and the check silent — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- edge: a quoted or backticked variable before a prefix ("$HOME"/, `pwd`/) was read as the project root — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- test: seven of the eleven opener characters had no test, and removing any one passed both modules — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- edge: a leading / was read from the project root in a project below its repository's root, where a host resolves it from the repository's root — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: the framework reads a root-absolute path from the project root, as check-corpus-links reads a root-absolute link (spec 059); reading it from the repository's root here alone split the fold's link and token passes, so 3fc5755a's gate was reverted
- stale: 051 task 31's ticked items recorded the mutation lists before round 4 changed them — `specs/051-branch-scoped-spec-numbering/tasks.md` — **fixed**
- stale: 050's plan attributed the round-4 constitution figures to the tree that added the length rule — `specs/050-constitution/plan.md` — **fixed**
- edge: 3fc5755a's closing-quote rule read an opening backtick after emphasis or strikethrough as closing a variable, so **`./specs/x`** was missed though **`specs/x`** is found — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- test: a prefix at the start of a line had no test, so the arms deciding it survived their mutations (QUAL-TEST-001) — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- stale: the shipped references, the module doc and the mutation list listed a leading / unconditionally while 3fc5755a dropped it below the repository root — `framework/reference/fold.md` — **fixed**
- edge: ~~./specs/x~~ and (a)./specs/x were missed though their bare spellings are found — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- edge: in a project below its repository's root, system.md's link pass re-pointed a leading / that its token pass left — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- edge: in a project below its repository's root, /proj/specs/x, the project's own path spelled from the repository's root, is neither checked nor re-pointed — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: a project's README spelling its own spec from the repository's root is a contrived input, and the project-root reading names it nothing
- test: two members of 3fc5755a's rule, the typographic closing quotes and the prefixed-only guard, had no killing test — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- stale: 051 task 31 did not record 3fc5755a's runs — `specs/051-branch-scoped-spec-numbering/tasks.md` — **fixed**
- test: the rewrite test for markup and shell forms carried only prefixed spellings, and the check's blockquote pair sat on two lines — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- edge: in a project below its repository's root the hook runs from the repository's root, so its ./ and bare paths resolve there — `specs/067-analyze-and-audit-checks/data-model.md` — **discarded**: the same assumption every bare reference in the hook carries, which task 30 does not add; prefixed and bare agree
- edge: bare-rule cases outside task 30 — ~/".ductus/x", "$X"specs/x and ${X}specs/x in the hook, ${X:-./specs/x}, *specs/x*, and @/specs/x in CLAUDE.md — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: the bare rule predates task 30 and prefixed and bare agree on each; every input is contrived
- test: no test had a bare managed path at a line's start, so a mutant refusing only that survived (QUAL-TEST-001) — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- test: the subdirectory test held its spec at neither root, so it could not tell a project-root reading from a repository-root one — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- test: no fixture put % or a digit before a closing quote — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- edge: the leading-/ exception also covers (a)/specs/x, x~/specs/x and a quote after a letter — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: contrived text; requiring $( before ) would add a parser for no real input
- stale: the root-prefixed mutation list's docstring omitted two functions and two killing tests and misgrouped M12 — `scripts/mutation/root-prefixed-references.py` — **fixed**
- convention: a rewrite test's doc comment broke mid-phrase onto a one-word line — `runtime/src/primitives/rewrite_spec_links.rs` — **fixed**
- unclear: the root-prefixed mutation list's docstring could read as if a leading / ending another directory's name were a reference — `scripts/mutation/root-prefixed-references.py` — **fixed**
- stale: the agreement test's comment omitted the hook's line-start lines, among them the bare-first line the round added — `runtime/src/primitives/check_orphaned_references.rs` — **fixed**
- test gap: no agreement test puts a path at column 0 in system.md, so a mutant refusing a line-start path only there survives, in the check's tests and the rewrite's — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: the scanner has no per-referrer branch, so a defect of that shape is contrived; the binary reports a bare, ../ and / path at column 0 in system.md
- test gap: the subdirectory test asserts that a spec only the repository's root holds is reported, not that one only the project holds is not — `runtime/src/primitives/check_orphaned_references.rs` — **discarded**: the realistic regression, reading existence from the repository's root, is killed by M46; skipping only a path present at both roots is contrived, and the binary does not report the project-only spec
- stale: two ticked items in 017's tasks.md name a rule's Rationale, which 70bb4af5 removed from the format — `specs/017-derive-dont-ask/tasks.md` — **discarded**: tasks.md is ephemeral work tracking whose ticked items record what was built then (§tasks-phase); the format's canonical statement, 017's data-model.md, is current

## Skipped passes

*None.*

## Unexamined governance

*None.*
