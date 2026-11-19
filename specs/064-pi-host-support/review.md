---
spec: 064-pi-host-support
last-run: 2026-11-19T20:24:24Z
reviewed-against: ebd15ffe7d5b48edea5e8d8d9a0ae7e0e5362326
diff-base: 7d590049d1c7757356a895bfbe79c979648270ac
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 7
scope: 12
unread:
  - .pi/extensions/ductus.ts
  - framework/bootstrap/govern.md
  - framework/bootstrap/ductus.md
  - runtime/CHANGELOG.md
  - specs/064-pi-host-support/plan.md
excluded: 2
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  scenarios/colon-form-commands-run-on-pi.md: c07825a14968e579c817cd098a61de4c07123b4548adacbda74815da8cc30180
  scenarios/pi-layout-is-dispatched.md: 0d0d2a6867c610e4d55252aa82fdf7a449ab0735ec16f47484a8c8dc332595fc
  scenarios/the-bridge-closes-its-server-at-shutdown.md: f852164bef881ed3a6f33eea81347d0bd23334182d9a5e810afcd46086a77f07
  scenarios/the-bridge-fails-loudly-and-recovers.md: 67570df65ceffa494936a5282eb62c100f14c125065562038aaa771077b1bf45
criteria-judged: 15
criteria-met: 15
criteria-verdicts:
  - label: AC1
    digest: 2ce9222d417f6ebb39a012f097568269f30cb03de4fbb178e7345efade4192fa
  - label: AC2
    digest: 3935b8fe4e07fe2774bdbc9b955a6e3ae84504c5dc2c927999f5b0c8e0eb6180
  - label: AC3
    digest: 359568112319701e44760f35032237dc239726ad23928e1edacb3ee9d250b58f
  - label: AC4
    digest: b3392642692c02a9fcf1f6453254baa65940e3ccf1b15afce7447c326fae9591
  - label: AC5
    digest: a447cc6dc65cfcd2690bc96d5e76e6dd74dc62564c158984629eff0c906b1729
  - label: AC6
    digest: f9f0d199440f45ae0a9be28b23a22bb34272ce073c84009d37b0956ca48b70aa
  - label: AC7
    digest: c69250eb73f50db60bca00b8bcf4b54a29e1d203cce441bec4d87c3102ef177e
  - label: AC8
    digest: 2ec4718a4907b110007279e562167940ff9269fb6b92a468071c70b66e0aeaec
  - label: AC9
    digest: 3e10b018525d8263842a09daeec406ab1b68eb6c2c569ba70de673122eeedc53
  - label: AC10
    digest: 8392ae51f4e4b19043fab823a1724a1b41fdca5fe5b5baaa9f558f766a04a242
  - label: AC11
    digest: 159d757b769e351ccef09b610e7fba55bb0ed57e24251c4d8e7542d7fc4182d3
  - label: AC12
    digest: 8df676c3da9927aa21f00c68fb1d70d18bdd5aee13c1887f459106b74c9c421b
  - label: AC13
    digest: c2e4a0a10ea1d48f98a59b5be0e8436e6e44d134e07aa854634ed4f02a43da64
  - label: AC14
    digest: 85886ac8851d1d93cb65c53c2d1e780af923f6b8705d8c635f778ee2081d0c1b
  - label: AC15
    digest: f436855fddf6d5e21f507fc5a2aa313f4a7c1ec406c536059e0fa7c9f836377e
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 13
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
  - key: "other: the status and help command handlers repeat one another's body, each calling its primitive, respelling the answer and showing it with no turn — `framework/bootstrap/pi/ductus-bridge.ts`"
    outcome: waived
    reason: 080 gave the two commands this shape, each handler differing in its primitive, its result reader and its message; this window added no third copy, and folding two bodies into one helper would change code no loaded rule faults
    decided-at: 2026-11-19T20:23:18Z
    decided-by: rule
  - key: "convention: the Pi bridge gotcha names only the status and help commands as what the bridge registers, not the colon aliases — `AGENTS.md`"
    outcome: waived
    reason: "the gotcha is incomplete, not false: it gives the two commands as why the bridge stays Pi's tool surface, which holds with or without the aliases; the bridge's header and the scenario state the aliases"
    decided-at: 2026-11-19T20:23:18Z
    decided-by: rule
  - key: "edge: a `[host] project` holding a space gives alias names Pi parses only up to the space, so no alias of it runs — `framework/bootstrap/pi/ductus-bridge.ts`"
    outcome: waived
    reason: the hyphen form cannot run under such a namespace either, the value comes from the project's own committed configuration, and the bootstrap accepts only lowercase letters, digits and hyphens for it
    decided-at: 2026-11-19T20:23:18Z
    decided-by: rule
---

# Review — 064-pi-host-support

## Summary

Review of the window `7d590049..HEAD`, 064's reopening for the scenario `colon-form-commands-run-on-pi` (tasks 18 and 19), in three parts: the security pass against the three `security-*.md` files, each read whole (194,749 bytes); the reuse, quality, efficiency and simplicity passes against the other nine rule files, each read whole, `AGENTS.md` from Gotchas to its end and `specs/system.md`; and the criteria step. No MUST or SHOULD violation. All 15 criteria were judged met, each against the code it names and driven where it can be: the harness, the two named `runtime/src/host.rs` tests, Families 14, 15, 17 and 21, and `scripts/gen-claude-commands.sh --check`, which was not shown to fail on drift. Six observations: three fixed in the run (two stale failure messages in the harness, two untested branches of `registerAliases` now each killed by a mutant, and a dated record under the plan's D9), three waived by rule. Ten stored decisions on in-scope files were read against the tree and still stand; two no longer stand and expire: a numbered step of `framework/bootstrap/ductus.md` now names the managed-block merges, and Agent Selection now gives dropped hosts' trees a notice. Examined counts the seven entries read whole: `framework/bootstrap/pi/ductus-bridge.ts`, `scripts/tests/pi-bridge-harness.mjs`, the scenario, `specs/064-pi-host-support/spec.md`, `specs/064-pi-host-support/tasks.md`, `README.md` and `framework/bootstrap/ductus-procedure.md`. Not counted, and named in `unread`: `.pi/extensions/ductus.ts`, a generated copy `cmp` shows byte-identical to the bridge read whole; `framework/bootstrap/govern.md`, byte-identical by `cmp` to `framework/bootstrap/ductus.md`; `framework/bootstrap/ductus.md`, read by its window diff and its Derived values, MCP registration, Adding a new agent, Pi layout, Agent Selection and detection sections (about 34 KB of 164,778 bytes); `runtime/CHANGELOG.md`, read by the hunks of the two commits naming this spec; and `specs/064-pi-host-support/plan.md`, read in its D9 section only, which the fix round changed. The harness passes 23 of 23 cases against the release runtime. Real pi 1.5.2 in RPC mode ran the aliases in this repository's Pi layout and in an adopter-shaped scratch project, a compaction held open included; the commit of task 19 holds those runs, which the reviewers checked against Pi's source and did not repeat. One fix round: the run's fixes to the harness were re-read by an independent agent, which found nothing; the plan's and task list's edits are exempt paths, which the analysis reads.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

15 of 15 acceptance criteria judged met.

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

- AC1 — met: `install.sh` `pi)` arm: dest `.pi/prompts/ductus.md`, `cp` verbatim, in-arm comment 'No settings seed', no heredoc; error line reads "expected: claude or pi"; Family 14 `installer-registry-parity.sh` exit 0 asserts the pi arm seeds no settings
- AC2 — met: `framework/bootstrap/ductus.md` Agent Registry row `pi` | Pi | `.pi` | `pi` | `{}` with rules_file_note 'Pi reads `AGENTS.md` natively'; Family 14 greps the `{}` cell
- AC3 — met: `framework/bootstrap/ductus.md` §Derived values `pi` column holds all seven values (path, invocation, install path, settings file with no-op, native rules file `AGENTS.md`, cleanup glob `{project}-*.md` in `prompts/`); Invocation also names the colon alias (task 18) without altering the claimed value
- AC4 — met: `framework/bootstrap/ductus.md` §MCP registration row `pi`: `.pi/extensions/ductus.ts`, `project-local`, `write-file`; §MCP wiring ('write-file agents') defers Pi's copy to the Pi-layout Bridge step and writes no MCP config
- AC5 — met: `framework/bootstrap/ductus.md` §Pi layout: slash-command manifest has 16 `framework/commands/` rows (audit excluded) plus configure, verbatim copy to `{config_dir}/prompts/{project}-{name}.md` with substitution; Bridge step verbatim, no substitution; `.pi/` in `framework/templates/project/gitignore`; Trust reminder carried by completion message (step 11)
- AC6 — met: `framework/bootstrap/pi/ductus-bridge.ts` imports only `node:` builtins, spawns `<root>/.ductus/bin/ductus mcp`, hand-rolled JSON-RPC (`initialize`, `notifications/initialized`, `tools/list`, `tools/call`), `registerTool` as `ductus__<name>` with `tool.inputSchema`, `setEncoding("utf8")`, timeout `drop`+`kill`, respawn via `connect`; `node scripts/tests/pi-bridge-harness.mjs` exit 0, all 23 cases PASS incl. missing binary, mid-session exit, timeout, split character
- AC7 — met: `framework/bootstrap/configure/pi.md` Verify-and-repair steps 1-4 fetch the bridge at the `[source] ref` /ductus resolves, byte-compare, overwrite on divergence, report no-permission and trust facts; `scripts/gen-configure-mcp.sh` header names the Pi exclusion
- AC8 — met: `runtime/src/host.rs` `Host::command_file_candidates` returns `commands/{project}/{name}.md` then `prompts/{project}-{name}.md`; `cargo test --release --locked --lib host::tests::command_file_candidates` 2 passed (both named tests); `git show fcaa7d2a` confirms the correction note (the existing test's expected list was extended)
- AC9 — met: `specs/022-deterministic-runtime/scenarios/the-pi-command-candidate.md` links `../../064-pi-host-support/spec.md`; `specs/022-deterministic-runtime/data-model.md` §Command-file candidates lists the pi candidate 'appended last'
- AC10 — met: run in the tree: `installer-registry-parity.sh` exit 0 (pi branch at the layout switch plus the pi-arm no-seed assertion), `runtime-probe-parity.sh` exit 0 with stderr 'skipped pi', `host-namespace-parity.sh` exit 0 (scans `{config_dir}/prompts/<ns>-*.md`, compared 2 config dirs)
- AC11 — met: `scripts/gen-claude-commands.sh --check` exit 0 and its Pi pass builds `.pi/prompts/ductus-*.md` and `.pi/extensions/ductus.ts`; `git ls-files .pi` tracks 17 prompts plus configure and the bridge; `.gitignore` carries `.pi/*` with `!.pi/prompts/` and `!.pi/extensions/` (a drift mutant was not run: copying the tree to scratch was denied)
- AC12 — met: `framework/constitution.md` §runtime-host-integration reads 'Pi `ductus__`, through the `.pi/extensions/ductus.ts` bridge rather than pi's own MCP client', matching the criterion's own supersession note; the wrap of the runtime's MCP server over stdio is stated in `framework/bootstrap/ductus.md` Pi layout Bridge step
- AC13 — met: `README.md`: line 7 'Claude Code and Pi', per-agent install table row `pi` (`.pi/prompts/ductus.md`), and the invocation paragraph names `/my-project-specify` for Pi (and now the colon alias)
- AC14 — met: `specs/064-pi-host-support/plan.md` D9 holds the 2026-09-20 smoke-test record with all four evidence points and the 2026-11-19 record of real pi 1.5.2 RPC runs that loaded the bridge as it stands, registered both spellings, made a live `render-status` call and expanded a `.pi/prompts/` template; `host::tests::command_file_candidates_pi_shape_from_session_cli_config_dir` passes for the `ductus exec` candidate
- AC15 — met: `cmp framework/bootstrap/ductus.md framework/bootstrap/govern.md` exit 0 (identical); `transitional-bootstrap-parity.sh` (Family 21) exit 0

## Observations

- convention: two failure messages in the respelling cases said a colon hint is one Pi cannot run, which task 18 made false; reworded to say the hint was not respelled — `scripts/tests/pi-bridge-harness.mjs` — **fixed** (decided by the rule)
- other: two branches of `registerAliases`, the dropped rejection after the idle wait and the return when `.pi/prompts` cannot be listed, had no harness case failing without them; two assertions added, each killed by its mutant — `scripts/tests/pi-bridge-harness.mjs` — **fixed** (decided by the rule)
- other: the smoke-test record under D9 still said nothing exercised pi's own loader on the rewritten bridge, though real pi 1.5.2 runs since have; a dated record of the 2026-11-19 run added — `specs/064-pi-host-support/plan.md` — **fixed** (decided by the rule)
- other: the status and help command handlers repeat one another's body, each calling its primitive, respelling the answer and showing it with no turn — `framework/bootstrap/pi/ductus-bridge.ts` — **waived**: 080 gave the two commands this shape, each handler differing in its primitive, its result reader and its message; this window added no third copy, and folding two bodies into one helper would change code no loaded rule faults (decided by the rule)
- convention: the Pi bridge gotcha names only the status and help commands as what the bridge registers, not the colon aliases — `AGENTS.md` — **waived**: the gotcha is incomplete, not false: it gives the two commands as why the bridge stays Pi's tool surface, which holds with or without the aliases; the bridge's header and the scenario state the aliases (decided by the rule)
- edge: a `[host] project` holding a space gives alias names Pi parses only up to the space, so no alias of it runs — `framework/bootstrap/pi/ductus-bridge.ts` — **waived**: the hyphen form cannot run under such a namespace either, the value comes from the project's own committed configuration, and the bootstrap accepts only lowercase letters, digits and hyphens for it (decided by the rule)
- convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md` — **waived**: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
- convention: the Closing restart gives no single form for Pi wired in the run beside Claude already registered — `framework/bootstrap/ductus-procedure.md` — **waived**: the per-agent forms compose into an accurate notice, as the subject's run shows, so no host reads it wrong
- convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md` — **waived**: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
- convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md` — **waived**: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
- convention: the Frontmatter Migration's specs/**/spec.md walk reports specs/templates/spec.md as already frontmatter on every run — `framework/bootstrap/ductus-procedure.md` — **waived**: the template has frontmatter, so the walk skips it with a report line and writes nothing
- other: with surface-instruction gone, the MCP registration table's mechanism column holds one value — `framework/bootstrap/ductus.md` — **waived**: the column is the extension point the Adding a new agent steps cite; dropping it is a design change, not removal residue (decided by the rule)
- stale: the Derived values row label Command/skill path still names the skill layout — `framework/bootstrap/ductus.md` — **waived**: a label, not a claim: it still names the command-file path both layouts use, no parser keys on it, and renaming it is a seven-spec sweep, two of them scenarios, for no behavior change (decided by the rule)
- other — pi's extension documentation (Long-lived resources and shutdown) says not to start a process from the extension factory and to defer it to session_start or the command that needs it; the bridge spawns the runtime in the factory at load — `framework/bootstrap/pi/ductus-bridge.ts:358` — **waived**: The tool list the bridge registers comes from the runtime, so registration has to start it; spec 064 plan Trade-offs records 'Spawn at load, not lazily' and a lazily started bridge would register nothing. The unreferenced child and the session_shutdown stop are what make the factory spawn safe. (decided by the rule)
- other — the stdin unref line is observably unnecessary on Node 26.13.0 (macOS): a copy of the bridge without it, driven by a stub runtime that answers after 1.5 s, completed the call and the process exited by itself; copies without the stdout unref or without the child unref hung (exit 124 at 12 s) — `framework/bootstrap/pi/ductus-bridge.ts:190` — **waived**: The scenario Behavior names the child and both its pipes, and the line costs one statement; whether the stdin handle keeps the loop alive varies with a pending write and with Node versions the harness does not pin, so no test can isolate it here. (decided by the rule)
- other — a call that is mid-handshake when session_shutdown fires fails with the bridge's 'is missing or unusable — run /ductus to re-acquire it' envelope, carrying the shutdown reason only in the closing parenthesis (driver: three runs, same text) — `framework/bootstrap/pi/ductus-bridge.ts:365` — **waived**: It is the existing wording for any failure of the handshake, a runtime death during it included, and it names the real reason; it arises only for a call racing the shutdown, which pi is tearing down. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `specs/071-command-reference-split/spec.md` — 071-command-reference-split, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/plan.md` — 071-command-reference-split, 080-deterministic-runtime-efficiency

## Unexamined governance

*None.*
