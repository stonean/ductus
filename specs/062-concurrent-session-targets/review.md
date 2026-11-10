---
spec: 062-concurrent-session-targets
last-run: 2026-11-10T01:49:44Z
reviewed-against: 56a92b87f0880588932dbcbd4b93fb458d2a64aa
diff-base: 10302897cec48ce0521105d36a94d3d7b9e90a06
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 38
scope: 59
excluded: 414
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 2a8f4408441a0f65666c7c86a648abc95036e837d91406b99e1a857965e474d8
  scenarios/first-writers-create-the-ignore-file-once.md: 150b95396db7f7e774f4da1726618b953070531f1810c4272a5c080f7e47c0b8
blocking: false
dispositions:
  fixed: 12
  deferred: 0
  waived: 4
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
---

# Review — 062-concurrent-session-targets

## Summary

Window `10302897..56a92b87` (base-source `reopen`, `review-owed` true): 079 task 10's reopening, whose corrections changed no code, scenario or data model of this spec. 023's (`3bbee201`) and 030's (`8339e0f6`) reviews read that shared window, so one read-only agent ran the five passes over what 062 claims after `3bbee201`, at `9ebca7b0`, driving the session primitives on fixtures over the CLI and MCP. This session read what changed in scope since: 080's `f9991d60` and `56a92b87` (the `read-section` registration and the reference-read line, which touch no session behavior), 055's `e8f17627` in `runtime/src/schema/primitives.rs` (no session type), and `079f5b75`'s handoff prose in 062's, 058's and 079's `tasks.md`, whose subject names 062 first. The rule files and `AGENTS.md` are unchanged since `9ebca7b0`.

This session read the five shared paths: `AGENTS.md` whole; the three runtime workflows' window diff, every `uses:` pinned by sha, `taiki-e/install-action` v2.90.3 and `softprops/action-gh-release` v3.0.4 resolved upstream to their pins, and ziglang 0.17.1's hash matched on PyPI; and `runtime/Cargo.lock`'s 17 bumps and 2 new crates, with `cargo audit` exiting 0 over 1,430 advisories. `SEC-DEPS-001` is met.

0 MUST, 0 SHOULD. Twelve observations, none a broken contract:

- ten fixed by the rule: the spec's and plan's stale claims about the retired markdown-only path, the exec seed, the status line, the identity edges and `cli-config-dir`'s readers; `DashboardResult`'s two doc comments; the `dashboard` description; `status.md`'s parenthetical;
- two fixed by the owner's delegation (2026-11-10): the plan's pre-062 present tense, and the test gap that no test failed when `render-status` dropped the process identity, now held by a unit test, the `concurrent_sessions.rs` view assertion and mutant M1, which both kill.

The criteria verification and the analysis, run beside this review, found more, which this run fixed too (task 30): AC1, AC19 and AC20 annotated never held as written, AC15 superseded in part; the data model's `peers[].last-used` representation, read off a real `write-session` result (`BE-REPR-001`), and its `set-at`, `seen-peers`, default-read and notice-wording passages; the scenario's Context and platform claim; the spec body's over-broad passages; and the plan's bound for the session tools' lists (`BE-PAYLOAD-001`), per-call identity refusal, legacy-migration and atomic-write sources. Each was checked against `runtime/src/session.rs` first.

Four stored waivers matched issues that still stand, each checked against its file: fold's and consolidate's exec reduction, `/target X`'s adoption notice, the repeated write-report sentence (now three files) and task 23's ticked item. Thirteen expired, their issues gone; 45 on files outside the scope are retained.

The fix-round loop ran one round from base `41e923b0`: an independent agent re-read its ten paths clean, and the next brief was empty.

`examined` is 38 of the 59 scope entries: the 31 the agent read (the spec, plan and tasks whole; the thirteen command sources, the constitution and the two goldens by hunk; the seven runtime sources, `runtime-tools.txt`, both configure sources and the `.claude` `configure.md` and `status.md` copies by 062's hunks) and the seven this session read (the five shared paths, and 058's and 079's `tasks.md` by `079f5b75`'s hunks). Of the other 21: the fourteen remaining `.claude` copies are generated from sources read, the generator re-run at `56a92b87` and in sync; and the seven entries this run wrote were re-read in its fix round. Not checked by the passes, and taken from elsewhere: Claude Code passing `CLAUDE_CODE_SESSION_ID` to its MCP server, shown by this session's own `resolve-session` (`claude-code:dcae072b`); AC4's concurrency on Linux and Windows, which rests on CI's three-OS `runtime` runs; Claude Code exporting the identity to hooks, which rests on 080's recorded verification (2026-11-08).

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

- convention: spec §Process identity says a host runs ductus resolve-session before the server is registered, false since bbff7a53 — `specs/062-concurrent-session-targets/spec.md` — **fixed** (decided by the rule)
- convention: the plan's two rationales cite the retired markdown-only path — `specs/062-concurrent-session-targets/plan.md` — **fixed** (decided by the rule)
- convention: the plan says the exec seed seeds from the core's resolve, where it peeks — `specs/062-concurrent-session-targets/plan.md` — **fixed** (decided by the rule)
- convention: the plan says framework/commands/status.md describes the new line, false since 57a63c35 — `specs/062-concurrent-session-targets/plan.md` — **fixed** (decided by the rule)
- convention: the plan's list of edges reading process_identity omits dashboard, render-status and the hook — `specs/062-concurrent-session-targets/plan.md` — **fixed** (decided by the rule)
- convention: DashboardResult's doc says one call returns everything /status needs to render the view — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- convention: DashboardResult.session_target's doc predates per-process targets — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- convention: status.md's parenthetical names a payload the command no longer reads — `framework/commands/status.md` — **fixed** (decided by the rule)
- other: no test fails if render-status drops the process identity — `runtime/tests/concurrent_sessions.rs` — **fixed**
- convention: the cli-config-dir Resolved Question lists check-corpus-links as a reader and omits render-help and target's tool list — `specs/062-concurrent-session-targets/spec.md` — **fixed** (decided by the rule)
- convention: the plan describes the pre-062 state in the present tense — `specs/062-concurrent-session-targets/plan.md` — **fixed**
- convention: the dashboard description omits session-unreadable — `runtime/src/schema/descriptions.rs` — **fixed** (decided by the rule)
- exec-path: fold's and consolidate's session step dispatches nothing under exec, and §concurrent-features states no exec carve-out — `framework/commands/fold.md:76` — **waived**: the reduction is documented in each step, the convention the two-paths guarantee sets for exec reductions, as clarify.md's scope note does
- ux: /target X announces adopting the feature it is about to leave — `framework/commands/target.md:29` — **waived**: truthful: the session did adopt that target before the write replaced it
- reuse: the same 'Display what the write reports' sentence appears in four command files — `framework/commands/target.md:44` — **waived**: each command stating its own procedure in full is the command files' self-contained convention
- stale: 062 task 23's checked item still gives the fixed-interval rationale for the lock poll — `specs/062-concurrent-session-targets/tasks.md` — **waived**: a ticked task records the decision as it was taken, task 28 records its reversal, and tasks.md is ephemeral work tracking that prune may drop

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/help.md` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/link.md` — 030-cross-service-references
- `.ductus/reference/analyze.md` — 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 077-security-cross-rules, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.ductus/reference/audit.md` — 079-review-and-analysis-work-converges
- `.ductus/reference/clarify.md` — 079-review-and-analysis-work-converges
- `.ductus/reference/consolidate.md` — 079-review-and-analysis-work-converges
- `.ductus/reference/fold.md` — 051-branch-scoped-spec-numbering, 079-review-and-analysis-work-converges
- `.ductus/reference/groom.md` — 000-slash-commands, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.ductus/reference/implement.md` — 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 041-task-pruning, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.ductus/reference/log.md` — 079-review-and-analysis-work-converges
- `.ductus/reference/plan.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.ductus/reference/prune.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `.ductus/reference/review.md` — 000-slash-commands, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.ductus/reference/specify.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `.ductus/reference/status.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.githooks/pre-commit` — 080-deterministic-runtime-efficiency
- `.github/workflows/generators.yml` — 080-deterministic-runtime-efficiency
- `.pi/extensions/ductus.ts` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-amend.md` — 022-deterministic-runtime, 030-cross-service-references, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `.pi/prompts/ductus-analyze.md` — 000-slash-commands, 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 041-task-pruning, 048-govern-acquired-runtime, 050-constitution, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-audit.md` — 022-deterministic-runtime, 030-cross-service-references, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-clarify.md` — 022-deterministic-runtime, 030-cross-service-references, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-configure.md` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-consolidate.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-fold.md` — 022-deterministic-runtime, 030-cross-service-references, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-groom.md` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-help.md` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-implement.md` — 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-link.md` — 030-cross-service-references
- `.pi/prompts/ductus-log.md` — 022-deterministic-runtime, 030-cross-service-references, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-plan.md` — 022-deterministic-runtime, 030-cross-service-references, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-prune.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-review.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 017-derive-dont-ask, 019-config-decisions, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 041-task-pruning, 048-govern-acquired-runtime, 050-constitution, 052-spec-supersession-and-consolidation, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-specify.md` — 022-deterministic-runtime, 030-cross-service-references, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-target.md` — 022-deterministic-runtime, 030-cross-service-references, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `README.md` — 017-derive-dont-ask, 020-code-review, 050-constitution, 055-shared-constitution, 058-findings-route-at-discovery, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `docs/analyze.md` — 000-slash-commands, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 041-task-pruning, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `docs/cross-service-references.md` — 030-cross-service-references
- `docs/developing-ductus.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `docs/rules.md` — 008-security-rules, 077-security-cross-rules
- `docs/runtime.md` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `docs/shared-constitution.md` — 050-constitution, 055-shared-constitution
- `docs/slash-commands.md` — 000-slash-commands, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 050-constitution, 058-findings-route-at-discovery, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `framework/bootstrap/configure/pi.md` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `framework/bootstrap/ductus-procedure.md` — 008-security-rules, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules
- `framework/bootstrap/ductus.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 064-pi-host-support, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `framework/bootstrap/govern.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 064-pi-host-support, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `framework/bootstrap/pi/ductus-bridge.ts` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `framework/commands/audit.md` — 022-deterministic-runtime, 030-cross-service-references, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `framework/commands/help.md` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `framework/commands/link.md` — 030-cross-service-references
- `framework/commands/log.md` — 022-deterministic-runtime, 030-cross-service-references, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `framework/reference/analyze.md` — 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 077-security-cross-rules, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `framework/reference/audit.md` — 079-review-and-analysis-work-converges
- `framework/reference/clarify.md` — 079-review-and-analysis-work-converges
- `framework/reference/consolidate.md` — 079-review-and-analysis-work-converges
- `framework/reference/fold.md` — 051-branch-scoped-spec-numbering, 079-review-and-analysis-work-converges
- `framework/reference/groom.md` — 000-slash-commands, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `framework/reference/implement.md` — 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 041-task-pruning, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `framework/reference/log.md` — 079-review-and-analysis-work-converges
- `framework/reference/plan.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `framework/reference/prune.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `framework/reference/review.md` — 000-slash-commands, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `framework/reference/specify.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `framework/reference/status.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `framework/rules/accessibility-frontend.md` — 069-accessibility-frontend-rules
- `framework/rules/api-backend.md` — 008-security-rules, 017-derive-dont-ask, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `framework/rules/concurrency-backend.md` — 038-concurrency-backend-rules
- `framework/rules/configuration-cross.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `framework/rules/performance-frontend.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 048-govern-acquired-runtime, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `framework/rules/quality-cross.md` — 036-quality-cross-rules
- `framework/rules/security-backend.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 048-govern-acquired-runtime, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `framework/rules/security-cross.md` — 008-security-rules, 017-derive-dont-ask, 020-code-review, 077-security-cross-rules
- `framework/rules/security-frontend.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `framework/templates/project/errors.md` — 008-security-rules
- `framework/templates/spec/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `install.sh` — 080-deterministic-runtime-efficiency
- `runtime/CHANGELOG.md` — 000-slash-commands, 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 017-derive-dont-ask, 019-config-decisions, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 036-quality-cross-rules, 038-concurrency-backend-rules, 041-task-pruning, 048-govern-acquired-runtime, 050-constitution, 051-branch-scoped-spec-numbering, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 060-exec-analyze-assesses-each-loaded-rule, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 076-waived-and-deferred-findings-are-distinct-dispositions, 077-security-cross-rules, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/hook.rs` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `runtime/src/interpreter/payload.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/lib.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/apply_manifest.rs` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/bounded_lists.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/brief_re_read.rs` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 052-spec-supersession-and-consolidation, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/check_artifact_size.rs` — 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `runtime/src/primitives/check_artifacts.rs` — 000-slash-commands, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 041-task-pruning, 058-findings-route-at-discovery, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/check_corpus_links.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/check_prescriptions.rs` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/check_review_gate.rs` — 000-slash-commands, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/check_stuck.rs` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/check_unfolded_specs.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/classify_spec_edit.rs` — 000-slash-commands, 022-deterministic-runtime, 030-cross-service-references, 041-task-pruning, 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/commit_attribution.rs` — 000-slash-commands, 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/compute_review_scope.rs` — 000-slash-commands, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/decisions.rs` — 000-slash-commands, 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/derive_boundary.rs` — 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/derive_references.rs` — 000-slash-commands, 022-deterministic-runtime, 030-cross-service-references, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/diff_cross_spec.rs` — 079-review-and-analysis-work-converges
- `runtime/src/primitives/discover_rule_files.rs` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/fix_rounds.rs` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 052-spec-supersession-and-consolidation, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/invalidate_review.rs` — 000-slash-commands, 022-deterministic-runtime, 051-branch-scoped-spec-numbering, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/lint_markdown.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/mark_task.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/measure_reopenings.rs` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/merge_permissions.rs` — 022-deterministic-runtime, 023-govern-refinement, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/merge_status_hook.rs` — 022-deterministic-runtime, 023-govern-refinement, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/pipeline_view.rs` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/process_decisions.rs` — 020-code-review, 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `runtime/src/primitives/process_waivers.rs` — 020-code-review, 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `runtime/src/primitives/prune_plan.rs` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/prune_project.rs` — 008-security-rules
- `runtime/src/primitives/prune_tasks.rs` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/read_section.rs` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/read_spec.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/read_tasks.rs` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 041-task-pruning, 048-govern-acquired-runtime, 050-constitution, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/remove_inbox_item.rs` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `runtime/src/primitives/render_help.rs` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/resolve_anchor.rs` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/resolve_constitutions.rs` — 050-constitution, 055-shared-constitution, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `runtime/src/primitives/resolve_references.rs` — 030-cross-service-references
- `runtime/src/primitives/rewrite_spec_links.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/rule_sections.rs` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/tool_loading.rs` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/write_analysis.rs` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 041-task-pruning, 048-govern-acquired-runtime, 050-constitution, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/primitives/write_review.rs` — 000-slash-commands, 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 017-derive-dont-ask, 019-config-decisions, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 041-task-pruning, 048-govern-acquired-runtime, 050-constitution, 052-spec-supersession-and-consolidation, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/schema/config_keys.rs` — 030-cross-service-references, 050-constitution, 055-shared-constitution, 078-review-fix-rounds-are-re-read-within-a-bound
- `runtime/src/schema/constitutions.rs` — 050-constitution, 055-shared-constitution
- `runtime/src/schema/extensions.rs` — 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `runtime/src/schema/mod.rs` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/src/schema/services.rs` — 030-cross-service-references
- `runtime/tests/common/corpus.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/common/mod.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/config_unknown_keys.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/crlf_preservation.rs` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `runtime/tests/cross_service.rs` — 030-cross-service-references
- `runtime/tests/fix_rounds.rs` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `runtime/tests/fixtures/review-basic/.govern.session.toml` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `runtime/tests/fixtures/review-basic/specs/007-review/spec.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `runtime/tests/golden/plan-basic.jsonl` — 080-deterministic-runtime-efficiency
- `runtime/tests/golden/review-basic.jsonl` — 022-deterministic-runtime, 050-constitution, 052-spec-supersession-and-consolidation, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `runtime/tests/hook_status.rs` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `runtime/tests/line_ending_discipline.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/mcp.rs` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `runtime/tests/measure_reopenings.rs` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `runtime/tests/merge_status_hook.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/mixed_corpus.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/parity.rs` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `runtime/tests/result_sizes.rs` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `runtime/tests/specs_root_override.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/walker.rs` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `scripts/audit/broken-relative-links.sh` — 080-deterministic-runtime-efficiency
- `scripts/audit/lib.sh` — 080-deterministic-runtime-efficiency
- `scripts/audit/manifest-destination-links.sh` — 080-deterministic-runtime-efficiency
- `scripts/audit/rename-sweep-residue.sh` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `scripts/audit/review-freshness.sh` — 080-deterministic-runtime-efficiency
- `scripts/check-release-tag.sh` — 008-security-rules, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules
- `scripts/gen-claude-commands.sh` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `scripts/measure-command-cost.py` — 022-deterministic-runtime, 023-govern-refinement, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `scripts/measure-result-sizes.sh` — 080-deterministic-runtime-efficiency
- `scripts/mutation/cross-service-references.py` — 030-cross-service-references
- `scripts/mutation/declined-note-edits.py` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `scripts/mutation/destination-checks.py` — 080-deterministic-runtime-efficiency
- `scripts/mutation/findings-cost-in-proportion-to-risk.py` — 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `scripts/mutation/fix-round-record.py` — 000-slash-commands, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 052-spec-supersession-and-consolidation, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `scripts/mutation/loop-inputs-screened.py` — 078-review-fix-rounds-are-re-read-within-a-bound
- `scripts/mutation/merge-permissions-canonical.py` — 022-deterministic-runtime, 023-govern-refinement, 080-deterministic-runtime-efficiency
- `scripts/mutation/merge-status-hook.py` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `scripts/mutation/own-record-is-no-round.py` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `scripts/mutation/plan-edit-takes-no-back-edge.py` — 041-task-pruning, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `scripts/mutation/reference-and-rule-reads.py` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `scripts/mutation/render-help.py` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `scripts/mutation/reopen-measure.py` — 000-slash-commands, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `scripts/mutation/resolve-constitutions-size.py` — 050-constitution, 055-shared-constitution
- `scripts/mutation/run.py` — 079-review-and-analysis-work-converges
- `scripts/mutation/scope-attribution.py` — 000-slash-commands, 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 017-derive-dont-ask, 019-config-decisions, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 041-task-pruning, 048-govern-acquired-runtime, 050-constitution, 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `scripts/mutation/scope-results-bounded.py` — 080-deterministic-runtime-efficiency
- `scripts/mutation/tool-loading.py` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `scripts/tests/pi-bridge-harness.mjs` — 022-deterministic-runtime, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `scripts/tests/test-check-release-tag.sh` — 008-security-rules, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules
- `scripts/tests/test-gen-claude-commands.sh` — 080-deterministic-runtime-efficiency
- `specs/000-slash-commands/scenarios/a-done-stop-says-the-request-is-not-built.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/000-slash-commands/scenarios/target-names-the-step-the-gate-would-name.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/007-govern-workflow/tasks.md` — 007-govern-workflow, 080-deterministic-runtime-efficiency
- `specs/008-security-rules/analysis.md` — 008-security-rules
- `specs/008-security-rules/data-model.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 048-govern-acquired-runtime, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/008-security-rules/plan.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 048-govern-acquired-runtime, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/008-security-rules/review.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/008-security-rules/scenarios/a-custom-credential-header-does-not-follow-a-redirect.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/008-security-rules/scenarios/rules-are-as-short-as-their-required-content.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/008-security-rules/scenarios/single-host-appliance-rule-fit.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/008-security-rules/scenarios/x-frame-options-carries-one-strength.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/008-security-rules/spec.md` — 008-security-rules, 017-derive-dont-ask, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/008-security-rules/tasks.md` — 000-slash-commands, 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 017-derive-dont-ask, 019-config-decisions, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 036-quality-cross-rules, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 076-waived-and-deferred-findings-are-distinct-dispositions, 077-security-cross-rules, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/010-agent-autonomy/analysis.md` — 010-agent-autonomy
- `specs/010-agent-autonomy/review.md` — 010-agent-autonomy
- `specs/010-agent-autonomy/scenarios/implement-offers-the-next-step.md` — 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 030-cross-service-references, 032-opencode-agent, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 079-review-and-analysis-work-converges
- `specs/010-agent-autonomy/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/014-reclarify-backedge/analysis.md` — 014-reclarify-backedge
- `specs/014-reclarify-backedge/review.md` — 014-reclarify-backedge
- `specs/014-reclarify-backedge/scenarios/reopen-after-informal-edits.md` — 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 030-cross-service-references, 032-opencode-agent, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 079-review-and-analysis-work-converges
- `specs/014-reclarify-backedge/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/015-tarball-fetch/analysis.md` — 015-tarball-fetch
- `specs/015-tarball-fetch/plan.md` — 015-tarball-fetch
- `specs/015-tarball-fetch/review.md` — 015-tarball-fetch
- `specs/015-tarball-fetch/spec.md` — 015-tarball-fetch
- `specs/016-cross-cutting-rules/analysis.md` — 016-cross-cutting-rules
- `specs/016-cross-cutting-rules/plan.md` — 016-cross-cutting-rules
- `specs/016-cross-cutting-rules/review.md` — 016-cross-cutting-rules
- `specs/016-cross-cutting-rules/spec.md` — 016-cross-cutting-rules, 020-code-review, 023-govern-refinement, 054-remove-supersession, 060-exec-analyze-assesses-each-loaded-rule, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 076-waived-and-deferred-findings-are-distinct-dispositions, 079-review-and-analysis-work-converges
- `specs/017-derive-dont-ask/data-model.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/017-derive-dont-ask/spec.md` — 008-security-rules, 017-derive-dont-ask, 020-code-review, 058-findings-route-at-discovery, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 077-security-cross-rules, 079-review-and-analysis-work-converges
- `specs/019-config-decisions/analysis.md` — 019-config-decisions
- `specs/019-config-decisions/data-model.md` — 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 030-cross-service-references, 032-opencode-agent, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 079-review-and-analysis-work-converges
- `specs/019-config-decisions/review.md` — 019-config-decisions
- `specs/019-config-decisions/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/020-code-review/data-model.md` — 020-code-review, 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/020-code-review/scenarios/review-reports-a-defect-under-its-owner.md` — 008-security-rules, 017-derive-dont-ask, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/020-code-review/scenarios/waiver-expiry.md` — 020-code-review, 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `specs/020-code-review/spec.md` — 017-derive-dont-ask, 020-code-review, 058-findings-route-at-discovery, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/020-code-review/tasks.md` — 016-cross-cutting-rules, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 060-exec-analyze-assesses-each-loaded-rule, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/data-model.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `specs/022-deterministic-runtime/plan.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/scenarios/a-closed-stdout-is-a-quiet-exit.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/022-deterministic-runtime/scenarios/apply-manifest-substitution-contract.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/022-deterministic-runtime/scenarios/commands-dir-parameterization.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/scenarios/config-readers-name-unknown-keys.md` — 022-deterministic-runtime, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `specs/022-deterministic-runtime/scenarios/coverage-expansion-primitives.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `specs/022-deterministic-runtime/scenarios/dashboard-primitive.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `specs/022-deterministic-runtime/scenarios/derive-boundary-uncommitted-spec-dir.md` — 022-deterministic-runtime, 030-cross-service-references, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/scenarios/framework-list-dedup.md` — 022-deterministic-runtime, 023-govern-refinement, 080-deterministic-runtime-efficiency
- `specs/022-deterministic-runtime/scenarios/groom-command-acceleration.md` — 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md` — 020-code-review, 022-deterministic-runtime, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/scenarios/review-scope-parse-fidelity.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/scenarios/the-cli-surface-nothing-pins.md` — 000-slash-commands, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/022-deterministic-runtime/spec.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `specs/022-deterministic-runtime/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/023-govern-refinement/analysis.md` — 023-govern-refinement
- `specs/023-govern-refinement/plan.md` — 023-govern-refinement
- `specs/023-govern-refinement/review.md` — 023-govern-refinement
- `specs/023-govern-refinement/scenarios/configure-dedup-permissions.md` — 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 030-cross-service-references, 032-opencode-agent, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 079-review-and-analysis-work-converges
- `specs/023-govern-refinement/scenarios/configure-retires-formerly-canonical-entries.md` — 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 030-cross-service-references, 032-opencode-agent, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 079-review-and-analysis-work-converges
- `specs/023-govern-refinement/scenarios/living-specs.md` — 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/023-govern-refinement/spec.md` — 010-agent-autonomy, 014-reclarify-backedge, 016-cross-cutting-rules, 019-config-decisions, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 030-cross-service-references, 032-opencode-agent, 054-remove-supersession, 059-project-in-a-repository-subdirectory, 060-exec-analyze-assesses-each-loaded-rule, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 076-waived-and-deferred-findings-are-distinct-dispositions, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `specs/023-govern-refinement/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/026-framework-self-audit/spec.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/026-framework-self-audit/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 047-analyze-findings-durability, 048-govern-acquired-runtime, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/029-bootstrap-runtime-autowire/analysis.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/review.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/scenarios/state-a-deterministic-path-forcing.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/spec.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/029-bootstrap-runtime-autowire/tasks.md` — 029-bootstrap-runtime-autowire
- `specs/030-cross-service-references/analysis.md` — 030-cross-service-references
- `specs/030-cross-service-references/data-model.md` — 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 030-cross-service-references, 032-opencode-agent, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 079-review-and-analysis-work-converges
- `specs/030-cross-service-references/plan.md` — 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/030-cross-service-references/review.md` — 030-cross-service-references
- `specs/030-cross-service-references/scenarios/referenced-service-spec-root.md` — 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 030-cross-service-references, 032-opencode-agent, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 079-review-and-analysis-work-converges
- `specs/030-cross-service-references/spec.md` — 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 022-deterministic-runtime, 023-govern-refinement, 030-cross-service-references, 032-opencode-agent, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 076-waived-and-deferred-findings-are-distinct-dispositions, 079-review-and-analysis-work-converges
- `specs/030-cross-service-references/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/032-opencode-agent/analysis.md` — 032-opencode-agent
- `specs/032-opencode-agent/data-model.md` — 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/032-opencode-agent/plan.md` — 032-opencode-agent
- `specs/032-opencode-agent/review.md` — 032-opencode-agent
- `specs/032-opencode-agent/spec.md` — 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 030-cross-service-references, 032-opencode-agent, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 079-review-and-analysis-work-converges
- `specs/032-opencode-agent/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/034-performance-backend-rules/analysis.md` — 034-performance-backend-rules
- `specs/034-performance-backend-rules/review.md` — 034-performance-backend-rules
- `specs/034-performance-backend-rules/spec.md` — 034-performance-backend-rules
- `specs/035-groom-session-target/analysis.md` — 035-groom-session-target
- `specs/035-groom-session-target/review.md` — 035-groom-session-target
- `specs/035-groom-session-target/scenarios/confirmation-names-reopen.md` — 035-groom-session-target, 041-task-pruning, 079-review-and-analysis-work-converges
- `specs/035-groom-session-target/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 041-task-pruning, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/036-quality-cross-rules/analysis.md` — 036-quality-cross-rules
- `specs/036-quality-cross-rules/data-model.md` — 036-quality-cross-rules
- `specs/036-quality-cross-rules/plan.md` — 036-quality-cross-rules
- `specs/036-quality-cross-rules/review.md` — 036-quality-cross-rules
- `specs/036-quality-cross-rules/scenarios/a-bound-test-pins-the-stated-figure.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/036-quality-cross-rules/spec.md` — 036-quality-cross-rules
- `specs/036-quality-cross-rules/tasks.md` — 036-quality-cross-rules
- `specs/037-observability-backend-rules/analysis.md` — 037-observability-backend-rules
- `specs/037-observability-backend-rules/review.md` — 037-observability-backend-rules
- `specs/037-observability-backend-rules/spec.md` — 037-observability-backend-rules
- `specs/038-concurrency-backend-rules/analysis.md` — 038-concurrency-backend-rules
- `specs/038-concurrency-backend-rules/plan.md` — 038-concurrency-backend-rules
- `specs/038-concurrency-backend-rules/review.md` — 038-concurrency-backend-rules
- `specs/038-concurrency-backend-rules/scenarios/lock-the-parent-before-rewriting-its-children.md` — 038-concurrency-backend-rules
- `specs/038-concurrency-backend-rules/spec.md` — 038-concurrency-backend-rules
- `specs/038-concurrency-backend-rules/tasks.md` — 038-concurrency-backend-rules, 079-review-and-analysis-work-converges
- `specs/039-reliability-backend-rules/analysis.md` — 039-reliability-backend-rules
- `specs/039-reliability-backend-rules/review.md` — 039-reliability-backend-rules
- `specs/039-reliability-backend-rules/spec.md` — 039-reliability-backend-rules
- `specs/041-task-pruning/data-model.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/041-task-pruning/plan.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/041-task-pruning/scenarios/a-suffixed-task-number-is-a-task.md` — 035-groom-session-target, 041-task-pruning, 079-review-and-analysis-work-converges
- `specs/041-task-pruning/scenarios/plan-records-the-design-as-it-stands.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/041-task-pruning/scenarios/prune-all-walks-every-spec.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/041-task-pruning/spec.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/041-task-pruning/tasks.md` — 035-groom-session-target, 041-task-pruning, 079-review-and-analysis-work-converges
- `specs/045-decision-state-drift-detection/analysis.md` — 045-decision-state-drift-detection
- `specs/045-decision-state-drift-detection/review.md` — 045-decision-state-drift-detection
- `specs/045-decision-state-drift-detection/spec.md` — 045-decision-state-drift-detection
- `specs/047-analyze-findings-durability/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/048-govern-acquired-runtime/spec.md` — 008-security-rules, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules
- `specs/048-govern-acquired-runtime/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/050-constitution/plan.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `specs/050-constitution/scenarios/a-declared-cross-spec-impact-gates-done.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `specs/050-constitution/scenarios/the-constitution-fits-one-read.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/050-constitution/spec.md` — 050-constitution, 063-oversized-artifacts-warn-with-a-fix, 079-review-and-analysis-work-converges
- `specs/050-constitution/tasks.md` — 050-constitution, 055-shared-constitution
- `specs/051-branch-scoped-spec-numbering/spec.md` — 051-branch-scoped-spec-numbering, 079-review-and-analysis-work-converges
- `specs/052-spec-supersession-and-consolidation/tasks.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/054-remove-supersession/analysis.md` — 054-remove-supersession
- `specs/054-remove-supersession/plan.md` — 054-remove-supersession
- `specs/054-remove-supersession/review.md` — 054-remove-supersession
- `specs/054-remove-supersession/spec.md` — 016-cross-cutting-rules, 020-code-review, 023-govern-refinement, 054-remove-supersession, 060-exec-analyze-assesses-each-loaded-rule, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 076-waived-and-deferred-findings-are-distinct-dispositions, 079-review-and-analysis-work-converges
- `specs/054-remove-supersession/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 047-analyze-findings-durability, 048-govern-acquired-runtime, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/055-shared-constitution/analysis.md` — 055-shared-constitution
- `specs/055-shared-constitution/data-model.md` — 050-constitution, 055-shared-constitution
- `specs/055-shared-constitution/plan.md` — 050-constitution, 055-shared-constitution
- `specs/055-shared-constitution/review.md` — 050-constitution, 055-shared-constitution
- `specs/055-shared-constitution/scenarios/a-registered-source-is-named-with-its-description.md` — 050-constitution, 055-shared-constitution
- `specs/055-shared-constitution/scenarios/target-reports-each-constitutions-size.md` — 050-constitution, 055-shared-constitution
- `specs/055-shared-constitution/spec.md` — 050-constitution, 055-shared-constitution
- `specs/055-shared-constitution/tasks.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 050-constitution, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 063-oversized-artifacts-warn-with-a-fix, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/058-findings-route-at-discovery/data-model.md` — 017-derive-dont-ask, 020-code-review, 058-findings-route-at-discovery, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/058-findings-route-at-discovery/plan.md` — 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/058-findings-route-at-discovery/scenarios/findings-cost-in-proportion-to-risk.md` — 017-derive-dont-ask, 020-code-review, 058-findings-route-at-discovery, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/058-findings-route-at-discovery/spec.md` — 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/059-project-in-a-repository-subdirectory/analysis.md` — 059-project-in-a-repository-subdirectory
- `specs/059-project-in-a-repository-subdirectory/plan.md` — 059-project-in-a-repository-subdirectory
- `specs/059-project-in-a-repository-subdirectory/review.md` — 059-project-in-a-repository-subdirectory
- `specs/059-project-in-a-repository-subdirectory/spec.md` — 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 030-cross-service-references, 032-opencode-agent, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 079-review-and-analysis-work-converges
- `specs/059-project-in-a-repository-subdirectory/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 041-task-pruning, 048-govern-acquired-runtime, 050-constitution, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/060-exec-analyze-assesses-each-loaded-rule/scenarios/an-assessment-sees-the-loaded-rule-files.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/060-exec-analyze-assesses-each-loaded-rule/scenarios/an-assessment-sees-the-rule-it-asks-about.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md` — 016-cross-cutting-rules, 020-code-review, 023-govern-refinement, 054-remove-supersession, 060-exec-analyze-assesses-each-loaded-rule, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 076-waived-and-deferred-findings-are-distinct-dispositions, 079-review-and-analysis-work-converges
- `specs/061-updates-track-the-latest-release-tag/analysis.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/plan.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/review.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/spec.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/tasks.md` — 061-updates-track-the-latest-release-tag
- `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/project-files-are-measured.md` — 016-cross-cutting-rules, 020-code-review, 023-govern-refinement, 054-remove-supersession, 060-exec-analyze-assesses-each-loaded-rule, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 076-waived-and-deferred-findings-are-distinct-dispositions, 079-review-and-analysis-work-converges
- `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — 022-deterministic-runtime, 026-framework-self-audit, 047-analyze-findings-durability, 050-constitution, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges, 080-deterministic-runtime-efficiency
- `specs/064-pi-host-support/plan.md` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `specs/064-pi-host-support/spec.md` — 022-deterministic-runtime, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 016-cross-cutting-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 060-exec-analyze-assesses-each-loaded-rule, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-review-states-what-it-read.md` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-supplied-summary-carries-the-loops-account.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/scenarios/an-empty-scope-flag-does-not-decide-the-denominator.md` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-staleness-gate.md` — 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/scenarios/the-fix-round-loop-is-recorded-and-gated.md` — 000-slash-commands, 022-deterministic-runtime, 052-spec-supersession-and-consolidation, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/scenarios/write-analysis-and-the-second-gate.md` — 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/spec.md` — 017-derive-dont-ask, 020-code-review, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/067-analyze-and-audit-checks/data-model.md` — 000-slash-commands, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 041-task-pruning, 058-findings-route-at-discovery, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/067-analyze-and-audit-checks/scenarios/disposition-drift-is-advisory.md` — 017-derive-dont-ask, 020-code-review, 058-findings-route-at-discovery, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/067-analyze-and-audit-checks/spec.md` — 067-analyze-and-audit-checks, 079-review-and-analysis-work-converges
- `specs/067-analyze-and-audit-checks/tasks.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/068-api-backend-rules/analysis.md` — 068-api-backend-rules
- `specs/068-api-backend-rules/plan.md` — 068-api-backend-rules
- `specs/068-api-backend-rules/review.md` — 068-api-backend-rules
- `specs/068-api-backend-rules/spec.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/068-api-backend-rules/tasks.md` — 068-api-backend-rules
- `specs/069-accessibility-frontend-rules/analysis.md` — 069-accessibility-frontend-rules
- `specs/069-accessibility-frontend-rules/plan.md` — 069-accessibility-frontend-rules
- `specs/069-accessibility-frontend-rules/review.md` — 069-accessibility-frontend-rules
- `specs/069-accessibility-frontend-rules/spec.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/069-accessibility-frontend-rules/tasks.md` — 069-accessibility-frontend-rules
- `specs/070-performance-frontend-rules/analysis.md` — 070-performance-frontend-rules
- `specs/070-performance-frontend-rules/plan.md` — 070-performance-frontend-rules
- `specs/070-performance-frontend-rules/review.md` — 070-performance-frontend-rules
- `specs/070-performance-frontend-rules/spec.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/070-performance-frontend-rules/tasks.md` — 070-performance-frontend-rules
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/analysis.md` — 076-waived-and-deferred-findings-are-distinct-dispositions
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/data-model.md` — 010-agent-autonomy, 014-reclarify-backedge, 016-cross-cutting-rules, 019-config-decisions, 020-code-review, 023-govern-refinement, 030-cross-service-references, 032-opencode-agent, 054-remove-supersession, 059-project-in-a-repository-subdirectory, 060-exec-analyze-assesses-each-loaded-rule, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 076-waived-and-deferred-findings-are-distinct-dispositions, 079-review-and-analysis-work-converges
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/plan.md` — 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 030-cross-service-references, 032-opencode-agent, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 079-review-and-analysis-work-converges
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/review.md` — 076-waived-and-deferred-findings-are-distinct-dispositions
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/spec.md` — 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 030-cross-service-references, 032-opencode-agent, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 079-review-and-analysis-work-converges
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 048-govern-acquired-runtime, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/077-security-cross-rules/analysis.md` — 077-security-cross-rules
- `specs/077-security-cross-rules/data-model.md` — 008-security-rules, 017-derive-dont-ask, 020-code-review, 077-security-cross-rules
- `specs/077-security-cross-rules/plan.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/077-security-cross-rules/review.md` — 008-security-rules, 077-security-cross-rules
- `specs/077-security-cross-rules/spec.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/077-security-cross-rules/tasks.md` — 008-security-rules, 077-security-cross-rules
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/analysis.md` — 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md` — 000-slash-commands, 022-deterministic-runtime, 052-spec-supersession-and-consolidation, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — 000-slash-commands, 022-deterministic-runtime, 052-spec-supersession-and-consolidation, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/review.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/scenarios/a-reviews-own-record-is-no-fix-round.md` — 052-spec-supersession-and-consolidation, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 052-spec-supersession-and-consolidation, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 019-config-decisions, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 047-analyze-findings-durability, 048-govern-acquired-runtime, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/analysis.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/data-model.md` — 000-slash-commands, 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 017-derive-dont-ask, 019-config-decisions, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 041-task-pruning, 048-govern-acquired-runtime, 050-constitution, 054-remove-supersession, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/plan.md` — 000-slash-commands, 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 017-derive-dont-ask, 019-config-decisions, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 041-task-pruning, 047-analyze-findings-durability, 048-govern-acquired-runtime, 050-constitution, 051-branch-scoped-spec-numbering, 052-spec-supersession-and-consolidation, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/research.md` — 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 030-cross-service-references, 035-groom-session-target, 041-task-pruning, 051-branch-scoped-spec-numbering, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/review.md` — 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 041-task-pruning, 050-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/spec.md` — 008-security-rules, 010-agent-autonomy, 014-reclarify-backedge, 017-derive-dont-ask, 019-config-decisions, 020-code-review, 022-deterministic-runtime, 023-govern-refinement, 026-framework-self-audit, 029-bootstrap-runtime-autowire, 030-cross-service-references, 032-opencode-agent, 035-groom-session-target, 041-task-pruning, 047-analyze-findings-durability, 048-govern-acquired-runtime, 050-constitution, 051-branch-scoped-spec-numbering, 054-remove-supersession, 055-shared-constitution, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 076-waived-and-deferred-findings-are-distinct-dispositions, 078-review-fix-rounds-are-re-read-within-a-bound, 079-review-and-analysis-work-converges
- `specs/080-deterministic-runtime-efficiency/data-model.md` — 022-deterministic-runtime, 023-govern-refinement, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/plan.md` — 022-deterministic-runtime, 023-govern-refinement, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/research.md` — 022-deterministic-runtime, 023-govern-refinement, 064-pi-host-support, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/antigravity-keeps-the-model-path.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/configure-types-no-permission-entries.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/groom-derives-routing-candidates.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/help-answers-with-no-model-turn.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/one-search-covers-referenced-sections.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/opencode-keeps-the-model-path.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/reference-reads-follow-the-cited-sections.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/rules-load-without-shell-reads.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/the-search-goes-out-with-the-first-call.md` — 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/spec.md` — 022-deterministic-runtime, 023-govern-refinement, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/tasks.md` — 007-govern-workflow, 022-deterministic-runtime, 023-govern-refinement, 063-oversized-artifacts-warn-with-a-fix, 064-pi-host-support, 080-deterministic-runtime-efficiency

## Unexamined governance

*None.*
