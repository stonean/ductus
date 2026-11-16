---
spec: 064-pi-host-support
last-run: 2026-11-16T21:58:56Z
reviewed-against: fbe519a7dc648fc5a3e2d0a01fc2659c7e211ac9
diff-base: 3f8e238cfcd253ef4e7f4b461cec3cb82c595bc4
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 7
scope: 7
unread: []
excluded: 3
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  scenarios/pi-layout-is-dispatched.md: 0d0d2a6867c610e4d55252aa82fdf7a449ab0735ec16f47484a8c8dc332595fc
  scenarios/the-bridge-closes-its-server-at-shutdown.md: f852164bef881ed3a6f33eea81347d0bd23334182d9a5e810afcd46086a77f07
  scenarios/the-bridge-fails-loudly-and-recovers.md: 67570df65ceffa494936a5282eb62c100f14c125065562038aaa771077b1bf45
blocking: false
dispositions:
  fixed: 7
  deferred: 0
  waived: 3
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
  - key: "edge: a session or config naming a dropped host is not diagnosed: .augment still resolves through the claude-style candidate, and .opencode or .agents ends in command-not-found naming the paths tried — `runtime/src/host.rs`"
    outcome: waived
    reason: Host::load documents that a config-dir mismatch surfaces as the command-file-not-found error, which names every path tried; the resolver is generic in cli-config-dir by design, so a custom directory keeps working, and the CHANGELOG's Removed entry announces the removal
    decided-at: 2026-11-11T13:19:28Z
    decided-by: rule
  - key: "test: every_agents_generated_copies_are_excluded derives its expected count from the fixture's own array — `runtime/src/primitives/check_corpus_links.rs`"
    outcome: waived
    reason: the broken-links assertion still fails when a copy stops being excluded, so the test is not vacuous
    decided-at: 2026-11-11T13:19:28Z
    decided-by: rule
  - key: "test: the check_artifacts REGISTRY fixture carries a scope column the parser does not read — `runtime/src/primitives/check_artifacts.rs`"
    outcome: waived
    reason: the fixture mirrors the real file's table shapes, and the real MCP registration table carries the column
    decided-at: 2026-11-11T13:19:28Z
    decided-by: rule
  - key: "other: an adopter with an installed .augment, .agents or .opencode tree gets no notice from /ductus, nothing removes it, and the managed .gitignore block no longer ignores it — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the owner dropped the three hosts as a breaking change; the CHANGELOG's Removed entry tells such an adopter to remove the tree by hand, the runtime refuses writes there by the owner's exception, and detecting a dropped host's tree would be new bootstrap behavior rather than a fix to this change
    decided-at: 2026-11-11T13:19:28Z
    decided-by: rule
  - key: "perf: install.sh validates the agent key only after the latest-release lookup and the bootstrap download — `install.sh`"
    outcome: waived
    reason: the order predates the removal, the arm still halts before writing anything, and moving the check is a behavior change with its own test
    decided-at: 2026-11-11T13:19:28Z
    decided-by: rule
  - key: "other: with surface-instruction gone, the MCP registration table's mechanism column holds one value — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the column is the extension point the Adding a new agent steps cite; dropping it is a design change, not removal residue
    decided-at: 2026-11-11T13:19:28Z
    decided-by: rule
  - key: "other: the swept migrations narrow three procedures to Claude, so a dropped-host adopter far enough behind loses their rewrites silently — `framework/migrations/runtime-store-path.md`"
    outcome: waived
    reason: no entry was removed or renumbered, so no last_applied names a missing id; claude and pi adopters run the same steps as before, and the hosts were dropped
    decided-at: 2026-11-11T13:19:28Z
    decided-by: rule
  - key: "edge: Family 14 Direction 3 iterates install.sh's seeds only, so a claude arm that stopped seeding would pass it — `scripts/audit/installer-registry-parity.sh`"
    outcome: waived
    reason: pre-existing for every host before the removal, and test-install.sh case X now fails when the claude arm stops seeding
    decided-at: 2026-11-11T13:19:28Z
    decided-by: rule
  - key: "test: test-gen-configure-mcp.sh has never asserted that --dry-run exits 1 on drift or that a run rewrites the block — `scripts/tests/test-gen-configure-mcp.sh`"
    outcome: waived
    reason: a gap the removal neither opened nor widened; check-zero runs the same --dry-run, and the reviewer proved both behaviors by hand on a broken copy
    decided-at: 2026-11-11T13:19:28Z
    decided-by: rule
  - key: "stale: the Derived values row label Command/skill path still names the skill layout — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: "a label, not a claim: it still names the command-file path both layouts use, no parser keys on it, and renaming it is a seven-spec sweep, two of them scenarios, for no behavior change"
    decided-at: 2026-11-11T13:19:28Z
    decided-by: rule
  - key: "other: Families 2, 29 and 32 hardcode configure/claude.md, so a later agent's permission file would go unchecked — `scripts/audit/manifest-parity.sh`"
    outcome: waived
    reason: each script states the bound, the same held for any further host before the removal, and a guard enumerating configure files would be a new family
    decided-at: 2026-11-11T13:19:28Z
    decided-by: rule
  - key: other — pi's extension documentation (Long-lived resources and shutdown) says not to start a process from the extension factory and to defer it to session_start or the command that needs it; the bridge spawns the runtime in the factory at load — `framework/bootstrap/pi/ductus-bridge.ts:358`
    outcome: waived
    reason: The tool list the bridge registers comes from the runtime, so registration has to start it; spec 064 plan Trade-offs records 'Spawn at load, not lazily' and a lazily started bridge would register nothing. The unreferenced child and the session_shutdown stop are what make the factory spawn safe.
    decided-at: 2026-11-16T21:57:56Z
    decided-by: rule
  - key: "other — the stdin unref line is observably unnecessary on Node 26.13.0 (macOS): a copy of the bridge without it, driven by a stub runtime that answers after 1.5 s, completed the call and the process exited by itself; copies without the stdout unref or without the child unref hung (exit 124 at 12 s) — `framework/bootstrap/pi/ductus-bridge.ts:190`"
    outcome: waived
    reason: The scenario Behavior names the child and both its pipes, and the line costs one statement; whether the stdin handle keeps the loop alive varies with a pending write and with Node versions the harness does not pin, so no test can isolate it here.
    decided-at: 2026-11-16T21:57:56Z
    decided-by: rule
  - key: "other — a call that is mid-handshake when session_shutdown fires fails with the bridge's 'is missing or unusable — run /ductus to re-acquire it' envelope, carrying the shutdown reason only in the closing parenthesis (driver: three runs, same text) — `framework/bootstrap/pi/ductus-bridge.ts:365`"
    outcome: waived
    reason: It is the existing wording for any failure of the handshake, a runtime death during it included, and it names the real reason; it arises only for a call racing the shutdown, which pi is tearing down.
    decided-at: 2026-11-16T21:57:56Z
    decided-by: rule
---

# Review — 064-pi-host-support

## Summary

064's review of its reopening's window, from `3f8e238c` to HEAD: the new scenario the-bridge-closes-its-server-at-shutdown and task 17 (`2e2ca395`), and the bridge's session_shutdown stop and unref with two harness cases (`a89b3e52`). One agent ran all five passes, read all nine loaded backend and cross rule files whole, 064's spec, plan and three scenarios whole, and read 7 of the 7 scope entries whole, the generated `.pi/extensions/ductus.ts` by byte comparison with its source; it read the 47 stored decisions' keys by search rather than the frontmatter whole, and none names a file in the scope. No finding: BE-INPUT-003, BE-TIMEOUT-001, CFG-CONST-003 and QUAL-TEST-001 engage the bridge and are met, and the design-time, service-only and API rules do not fire against a local stdio extension. Ten observations: a broken contract, a call queued behind the one in flight at shutdown was not failed and respawned the runtime, fixed in the run with a harness case; six chores fixed (the ExtensionAPI slice's session_shutdown overload, the harness header, the scenario's edge case on a shutdown before any call, the plan's D2, the in-flight case's missing test, the harness's duplicated process pattern); three waived by the rule with their reasons. The fix round was re-read by an independent agent against 064's spec and three scenarios, read whole: clean, and the loop's closing brief was empty.

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

- bug — a call queued behind an in-flight call when session_shutdown fires is not failed: stop() drops only the pending requests of the child and leaves callChain alone, so the queued call then runs, respawns the runtime after the shutdown, and waits on it (arguable against the scenario's 'fails any call still waiting on it'; a driver with a stub runtime that never answers showed the queued call running a fresh child after the shutdown and settling only at a second shutdown or the request timeout) — `framework/bootstrap/pi/ductus-bridge.ts:289` (breaks the spec's contract) — **fixed** (decided by the rule)
- convention — the file's own ExtensionAPI slice, documented as what the bridge uses, declares on() for session_start only, so pi.on("session_shutdown", ...) does not type-check against it — `framework/bootstrap/pi/ductus-bridge.ts:72` — **fixed** (decided by the rule)
- convention — the harness header comment lists what it covers and omits the two cases task 17 added (session_shutdown stopping the runtime, and a process that loaded the bridge exiting by itself) — `scripts/tests/pi-bridge-harness.mjs:5` — **fixed** (decided by the rule)
- other — the scenario's second Edge Case says a shutdown before any call finds no child, but the bridge spawns the runtime at load, so a shutdown before any tool call finds and stops that child (driver: runtime running after load and before any call, gone after the shutdown); the sentence holds only when the runtime never started or has died — `specs/064-pi-host-support/scenarios/the-bridge-closes-its-server-at-shutdown.md:23` — **fixed** (decided by the rule)
- other — plan.md D2 states the bridge keeps the runtime alive and respawns it, and records neither the session_shutdown stop nor the unreferenced child and pipes, so the design record omits what task 17 added (the scenario carries the requirement) — `specs/064-pi-host-support/plan.md:56` — **fixed** (decided by the rule)
- other — pi's extension documentation (Long-lived resources and shutdown) says not to start a process from the extension factory and to defer it to session_start or the command that needs it; the bridge spawns the runtime in the factory at load — `framework/bootstrap/pi/ductus-bridge.ts:358` — **waived**: The tool list the bridge registers comes from the runtime, so registration has to start it; spec 064 plan Trade-offs records 'Spawn at load, not lazily' and a lazily started bridge would register nothing. The unreferenced child and the session_shutdown stop are what make the factory spawn safe. (decided by the rule)
- other — the stdin unref line is observably unnecessary on Node 26.13.0 (macOS): a copy of the bridge without it, driven by a stub runtime that answers after 1.5 s, completed the call and the process exited by itself; copies without the stdout unref or without the child unref hung (exit 124 at 12 s) — `framework/bootstrap/pi/ductus-bridge.ts:190` — **waived**: The scenario Behavior names the child and both its pipes, and the line costs one statement; whether the stdin handle keeps the loop alive varies with a pending write and with Node versions the harness does not pin, so no test can isolate it here. (decided by the rule)
- other — the scenario's first Edge Case (a call in flight at shutdown settles with the shutdown's error) has no harness case; the shutdown case runs with the runtime idle. A driver with a stub that never answers showed the in-flight call settle in 1 ms with 'ductus: probe failed: the ductus bridge was shut down' — `scripts/tests/pi-bridge-harness.mjs:363` — **fixed** (decided by the rule)
- other — a call that is mid-handshake when session_shutdown fires fails with the bridge's 'is missing or unusable — run /ductus to re-acquire it' envelope, carrying the shutdown reason only in the closing parenthesis (driver: three runs, same text) — `framework/bootstrap/pi/ductus-bridge.ts:365` — **waived**: It is the existing wording for any failure of the handshake, a runtime death during it included, and it names the real reason; it arises only for a call racing the shutdown, which pi is tearing down. (decided by the rule)
- other — the harness spells the runtime process pattern twice, in killRuntime and in runtimeRunning — `scripts/tests/pi-bridge-harness.mjs:129` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `specs/012-multi-agent-govern/tasks.md` — 012-multi-agent-govern
- `specs/022-deterministic-runtime/scenarios/merge-managed-block-renamed-subsection.md` — 022-deterministic-runtime
- `specs/048-govern-acquired-runtime/scenarios/pointer-follows-the-store.md` — 048-govern-acquired-runtime

## Unexamined governance

*None.*
