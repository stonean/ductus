---
spec: 064-pi-host-support
last-run: 2026-11-11T13:19:28Z
reviewed-against: e2009d70b10c61d59e62ab2386af3251282287c2
diff-base: 50c60b1ff442c5bc4f077a5da1d7ecfd160d5ef8
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 15
scope: 72
excluded: 241
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  scenarios/pi-layout-is-dispatched.md: 0d0d2a6867c610e4d55252aa82fdf7a449ab0735ec16f47484a8c8dc332595fc
  scenarios/the-bridge-fails-loudly-and-recovers.md: 67570df65ceffa494936a5282eb62c100f14c125065562038aaa771077b1bf45
blocking: false
dispositions:
  fixed: 15
  deferred: 0
  waived: 18
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
---

# Review — 064-pi-host-support

## Summary

One in-depth review of 064's reopening window, 50c60b1f..e2009d70 (base-source reopen), and the first of the reviews that share b1d319f1, the chore with no spec that dropped Auggie, Antigravity and OpenCode. The passes read b1d319f1's change on every in-scope path in full, with 0afc0c00 and 064's own commits 4b6c5fe4, e48033c6, cbdaafc7 and 96af02d0; the window's other commits on the same paths belong to 080, 067, 022, 055 and 071, were read as context only, and are left to those specs' reviews. Three independent reviewers each followed one behavior across its files (command resolution and the runtime's host set; what the bootstrap installs and writes per host; the audit families, generators and prose) against the 203 rules of the nine backend and cross rule files, and drove each on fixtures: ductus exec on sessions naming a dropped host, install.sh for every agent key, and broken copies of each changed audit family. The 107 rules of the three frontend rule files govern browser-rendered pages and have no subject here: the scope holds only Markdown, Rust, shell, Python and TOML. One MUST, BE-INPUT-003 at ductus.md's Source resolution steps 5 and 6 and govern.md's mirror of them, was fixed in e2009d70 with fourteen chores, each re-checked: cargo fmt and clippy clean, 501 unit tests, scripts/audit/run-all.sh, test-install.sh with four install.sh mutants killed, and six family mutants killed. Eleven observations are waived by the rule with their reasons and seven stored decisions carried. No contract break: 064's criteria AC1 to AC15 hold at e2009d70, AC8 checked word for word against host.rs. Examined counts the 15 in-scope files the passes read in full, of 72: host.rs, the seven Pi exec fixtures, configure/pi.md, install.sh, the gitignore template, Families 14, 15 and 17, and tasks.md. For 50 more the passes read the window's whole change and the file around it, not the whole file: about 600 of ductus.md's lines were not read line by line, ductus-procedure.md and the large runtime modules were read around their hunks, and runtime/CHANGELOG.md for its Unreleased Removed and Fixed entries only. Not opened: govern.md, compared byte for byte with ductus.md and identical, and the six generated copies under .claude/, .pi/ and .ductus/, checked against their sources by gen-claude-commands.sh --check and in sync.

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

- security: BE-INPUT-003 — Source resolution steps 5 and 6 interpolated ${DUCTUS_REPO:-stonean/ductus} into unquoted curl URLs, the shape step 3's fix closed, so the CHANGELOG's Fixed entry overstated; both are quoted and govern.md mirrored — `framework/bootstrap/ductus.md` — **fixed** (decided by the rule)
- stale: eight sites in the audit scripts, their README and audit.md called configure/claude.md the only per-agent configure file, though configure/pi.md exists — `scripts/audit/README.md` — **fixed** (decided by the rule)
- stale: Family 17's comment counted four contracts and gave a fifth agent as its drift example — `scripts/audit/host-namespace-parity.sh` — **fixed** (decided by the rule)
- stale: Family 29 kept a 29a label with no 29b — `scripts/audit/permission-wildcard-position.sh` — **fixed** (decided by the rule)
- convention: Family 32c kept an identity retired_item and a dict-only display branch that no seed reaches — `scripts/audit/permission-entry-shape.sh` — **fixed** (decided by the rule)
- convention: Family 15's skip_agent kept a cli-marker parameter and loop that no caller reaches — `scripts/audit/runtime-probe-parity.sh` — **fixed** (decided by the rule)
- stale: configure/pi.md named the other agents' configure commands, now a set of one — `framework/bootstrap/configure/pi.md` — **fixed** (decided by the rule)
- stale: host_config, apply_manifest, enforce_manifest, merge_managed_block and apply-manifest's description listed a skill file among what /ductus writes, two in emitted error text, and host_config's module doc omitted the other-tools refusal — `runtime/src/primitives/host_config.rs` — **fixed** (decided by the rule)
- stale: two check_artifacts test comments said the arm takes the project-scoped MCP targets after PROJECT_SCOPES was removed — `runtime/src/primitives/check_artifacts.rs` — **fixed** (decided by the rule)
- test: the real-registry test still asserted no home-relative path, guarding a filter removed with PROJECT_SCOPES, so that half could fail on no behavior of agent_scaffold_destinations — `runtime/src/primitives/check_artifacts.rs` — **fixed** (decided by the rule)
- test: a merge_permissions test comment said Pi keeps its settings at .pi/settings.json, where Pi has no permission-gating settings — `runtime/src/primitives/merge_permissions.rs` — **fixed** (decided by the rule)
- test: no install.sh test reached the unknown-agent arm, where every dropped host now lands, and none asserted the claude arm's settings seed — `scripts/tests/test-install.sh` — **fixed** (decided by the rule)
- convention: four comment lines the removal edited ran past 100 columns — `runtime/src/host.rs` — **fixed** (decided by the rule)
- convention: the generator reported mcp-allow blocks in sync for its one block — `scripts/gen-configure-mcp.sh` — **fixed** (decided by the rule)
- convention: D1 lost its full stop when the antigravity parenthetical was cut, and the opening called Pi the agent ductus scaffolds into — `specs/064-pi-host-support/spec.md` — **fixed** (decided by the rule)
- edge: a session or config naming a dropped host is not diagnosed: .augment still resolves through the claude-style candidate, and .opencode or .agents ends in command-not-found naming the paths tried — `runtime/src/host.rs` — **waived**: Host::load documents that a config-dir mismatch surfaces as the command-file-not-found error, which names every path tried; the resolver is generic in cli-config-dir by design, so a custom directory keeps working, and the CHANGELOG's Removed entry announces the removal (decided by the rule)
- test: every_agents_generated_copies_are_excluded derives its expected count from the fixture's own array — `runtime/src/primitives/check_corpus_links.rs` — **waived**: the broken-links assertion still fails when a copy stops being excluded, so the test is not vacuous (decided by the rule)
- test: the check_artifacts REGISTRY fixture carries a scope column the parser does not read — `runtime/src/primitives/check_artifacts.rs` — **waived**: the fixture mirrors the real file's table shapes, and the real MCP registration table carries the column (decided by the rule)
- other: an adopter with an installed .augment, .agents or .opencode tree gets no notice from /ductus, nothing removes it, and the managed .gitignore block no longer ignores it — `framework/bootstrap/ductus.md` — **waived**: the owner dropped the three hosts as a breaking change; the CHANGELOG's Removed entry tells such an adopter to remove the tree by hand, the runtime refuses writes there by the owner's exception, and detecting a dropped host's tree would be new bootstrap behavior rather than a fix to this change (decided by the rule)
- perf: install.sh validates the agent key only after the latest-release lookup and the bootstrap download — `install.sh` — **waived**: the order predates the removal, the arm still halts before writing anything, and moving the check is a behavior change with its own test (decided by the rule)
- other: with surface-instruction gone, the MCP registration table's mechanism column holds one value — `framework/bootstrap/ductus.md` — **waived**: the column is the extension point the Adding a new agent steps cite; dropping it is a design change, not removal residue (decided by the rule)
- other: the swept migrations narrow three procedures to Claude, so a dropped-host adopter far enough behind loses their rewrites silently — `framework/migrations/runtime-store-path.md` — **waived**: no entry was removed or renumbered, so no last_applied names a missing id; claude and pi adopters run the same steps as before, and the hosts were dropped (decided by the rule)
- edge: Family 14 Direction 3 iterates install.sh's seeds only, so a claude arm that stopped seeding would pass it — `scripts/audit/installer-registry-parity.sh` — **waived**: pre-existing for every host before the removal, and test-install.sh case X now fails when the claude arm stops seeding (decided by the rule)
- test: test-gen-configure-mcp.sh has never asserted that --dry-run exits 1 on drift or that a run rewrites the block — `scripts/tests/test-gen-configure-mcp.sh` — **waived**: a gap the removal neither opened nor widened; check-zero runs the same --dry-run, and the reviewer proved both behaviors by hand on a broken copy (decided by the rule)
- stale: the Derived values row label Command/skill path still names the skill layout — `framework/bootstrap/ductus.md` — **waived**: a label, not a claim: it still names the command-file path both layouts use, no parser keys on it, and renaming it is a seven-spec sweep, two of them scenarios, for no behavior change (decided by the rule)
- other: Families 2, 29 and 32 hardcode configure/claude.md, so a later agent's permission file would go unchecked — `scripts/audit/manifest-parity.sh` — **waived**: each script states the bound, the same held for any further host before the removal, and a guard enumerating configure files would be a new family (decided by the rule)
- other: the constitution line naming plan.md carries § references resolve-anchor classes as qualified and does not judge — `framework/constitution.md` — **waived**: both targets, §brownfield-inbox and §drift-prevention, carry markers (checked by hand), and citing a subsection's marked section is the constitution's own anchor rule
- convention: no numbered step of the bootstrap names the two managed-block merges — `framework/bootstrap/ductus.md` — **waived**: Instructions step 4 lists what apply-manifest replaces, not every Shared Files step, and two real adopter bootstraps on this text wrote both blocks from Shared Files
- convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md` — **waived**: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
- convention: the Closing restart gives no single form for Pi wired in the run beside Claude already registered — `framework/bootstrap/ductus-procedure.md` — **waived**: the per-agent forms compose into an accurate notice, as the subject's run shows, so no host reads it wrong
- convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md` — **waived**: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
- convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md` — **waived**: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
- convention: the Frontmatter Migration's specs/**/spec.md walk reports specs/templates/spec.md as already frontmatter on every run — `framework/bootstrap/ductus-procedure.md` — **waived**: the template has frontmatter, so the walk skips it with a report line and writes nothing

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/amend.md` — 067-analyze-and-audit-checks
- `.claude/commands/ductus/analyze.md` — 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/clarify.md` — 067-analyze-and-audit-checks
- `.claude/commands/ductus/review.md` — 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/status.md` — 080-deterministic-runtime-efficiency
- `.claude/commands/ductus/target.md` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets, 067-analyze-and-audit-checks
- `.ductus/reference/clarify.md` — 067-analyze-and-audit-checks
- `.ductus/reference/implement.md` — 067-analyze-and-audit-checks
- `.ductus/reference/plan.md` — 067-analyze-and-audit-checks
- `.github/workflows/framework-checks.yml` — 022-deterministic-runtime
- `.github/workflows/generators.yml` — 080-deterministic-runtime-efficiency
- `.github/workflows/runtime.yml` — 080-deterministic-runtime-efficiency
- `.pi/extensions/ductus.ts` — 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-amend.md` — 067-analyze-and-audit-checks
- `.pi/prompts/ductus-analyze.md` — 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-clarify.md` — 067-analyze-and-audit-checks
- `.pi/prompts/ductus-review.md` — 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-status.md` — 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-target.md` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets, 067-analyze-and-audit-checks
- `AGENTS.md` — 050-constitution
- `framework/bootstrap/pi/ductus-bridge.ts` — 080-deterministic-runtime-efficiency
- `framework/commands/amend.md` — 067-analyze-and-audit-checks
- `framework/commands/analyze.md` — 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `framework/commands/clarify.md` — 067-analyze-and-audit-checks
- `framework/commands/review.md` — 080-deterministic-runtime-efficiency
- `framework/commands/status.md` — 080-deterministic-runtime-efficiency
- `framework/commands/target.md` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets, 067-analyze-and-audit-checks
- `framework/reference/clarify.md` — 067-analyze-and-audit-checks
- `framework/reference/implement.md` — 067-analyze-and-audit-checks
- `framework/reference/plan.md` — 067-analyze-and-audit-checks
- `runtime/src/hook.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/interpreter/analyze_tally.rs` — 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `runtime/src/interpreter/mod.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/append_inbox.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/append_question.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/bounded_lists.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/brief_re_read.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/check_artifact_size.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/check_orphaned_references.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/check_prescriptions.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/check_review_freshness.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/check_review_gate.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/check_stuck.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/check_unfolded_specs.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/classify_spec_edit.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/compute_review_scope.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/create_feature.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/create_plan_artifacts.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/decisions.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/derive_boundary.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/derive_dependencies.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets, 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/derive_references.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets, 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/derive_routing_candidates.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/diff_cross_spec.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/discover_rule_files.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/inbox_standing.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/invalidate_review.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/label_criteria.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/mark_criterion.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/mark_task.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/measure_reopenings.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/merge_status_hook.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/mod.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/pipeline_view.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/process_decisions.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/process_waivers.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/prune_plan.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/prune_project.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/prune_tasks.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/read_re_read.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/read_spec.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/read_tasks.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/relocate_audit_records.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/remove_inbox_item.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/render_help.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/render_status.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/resolve_feature.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/resolve_references.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/retire_feature.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/rewrite_spec_links.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/rule_sections.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/set_status.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/spec_links.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/start_fix_rounds.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/tool_loading.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/traverse_deps.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/write_analysis.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/primitives/write_review.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/schema/paths.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/src/session.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/tests/derive_unwritable_cli.rs` — 067-analyze-and-audit-checks
- `runtime/tests/golden/review-basic.jsonl` — 080-deterministic-runtime-efficiency
- `runtime/tests/merge_status_hook.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/specify_command.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `runtime/tests/walker.rs` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `scripts/audit/done-spec-criteria.sh` — 026-framework-self-audit
- `scripts/gen-claude-commands.sh` — 080-deterministic-runtime-efficiency
- `scripts/measure-command-cost.py` — 080-deterministic-runtime-efficiency
- `scripts/mutation/owner-reviews-067.py` — 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `scripts/mutation/reference-and-rule-reads.py` — 080-deterministic-runtime-efficiency
- `scripts/mutation/scope-attribution.py` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `scripts/mutation/tool-loading.py` — 080-deterministic-runtime-efficiency
- `scripts/tests/pi-bridge-harness.mjs` — 080-deterministic-runtime-efficiency
- `scripts/tests/test-gen-claude-commands.sh` — 080-deterministic-runtime-efficiency
- `scripts/tests/test-host-namespace-parity.sh` — 022-deterministic-runtime
- `specs/000-slash-commands/scenarios/analyze-reads-the-layout-derived-command-set.md` — 000-slash-commands
- `specs/000-slash-commands/scenarios/dashboard-dependencies-column.md` — 000-slash-commands
- `specs/000-slash-commands/scenarios/target-clear-flag.md` — 000-slash-commands
- `specs/000-slash-commands/tasks.md` — 000-slash-commands
- `specs/002-project-scaffolding/scenarios/claude-md-template-imports-load.md` — 002-project-scaffolding
- `specs/002-project-scaffolding/spec.md` — 002-project-scaffolding
- `specs/002-project-scaffolding/tasks.md` — 002-project-scaffolding
- `specs/003-bootstrap-automation/scenarios/curl-sh-installer.md` — 003-bootstrap-automation
- `specs/003-bootstrap-automation/tasks.md` — 003-bootstrap-automation
- `specs/007-govern-workflow/plan.md` — 007-govern-workflow
- `specs/007-govern-workflow/scenarios/ductus-self-update-precheck.md` — 007-govern-workflow
- `specs/007-govern-workflow/spec.md` — 007-govern-workflow
- `specs/007-govern-workflow/tasks.md` — 007-govern-workflow
- `specs/009-scenario-targeting/spec.md` — 009-scenario-targeting
- `specs/010-agent-autonomy/tasks.md` — 010-agent-autonomy
- `specs/011-brownfield-process/tasks.md` — 011-brownfield-process
- `specs/012-multi-agent-govern/data-model.md` — 012-multi-agent-govern
- `specs/012-multi-agent-govern/plan.md` — 012-multi-agent-govern
- `specs/012-multi-agent-govern/scenarios/settings-template-bash-allowlist.md` — 012-multi-agent-govern
- `specs/012-multi-agent-govern/spec.md` — 012-multi-agent-govern
- `specs/012-multi-agent-govern/tasks.md` — 012-multi-agent-govern
- `specs/015-tarball-fetch/plan.md` — 015-tarball-fetch
- `specs/015-tarball-fetch/spec.md` — 015-tarball-fetch
- `specs/015-tarball-fetch/tasks.md` — 015-tarball-fetch
- `specs/017-derive-dont-ask/plan.md` — 017-derive-dont-ask
- `specs/017-derive-dont-ask/scenarios/a-commit-stages-only-its-own-generated-files.md` — 017-derive-dont-ask
- `specs/017-derive-dont-ask/tasks.md` — 017-derive-dont-ask
- `specs/022-deterministic-runtime/data-model.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/plan.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/ask-consolidation.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/cli-config-dir-per-contributor.md` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `specs/022-deterministic-runtime/scenarios/commands-dir-parameterization.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/config-resolution-single-probe.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/coverage-residue-cleanup.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/dashboard-primitive.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/framework-list-dedup.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/govern-bootstrap.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/merge-managed-block-subsection-insertion.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/the-constitutions-registry-validates-its-values.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/the-pi-command-candidate.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/writecode-payload-bundling.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/spec.md` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `specs/022-deterministic-runtime/tasks.md` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `specs/023-govern-refinement/plan.md` — 023-govern-refinement
- `specs/023-govern-refinement/scenarios/configure-dedup-permissions.md` — 023-govern-refinement
- `specs/023-govern-refinement/scenarios/configure-inert-write-path-entries.md` — 023-govern-refinement
- `specs/023-govern-refinement/scenarios/configure-permission-pattern-safety.md` — 023-govern-refinement
- `specs/023-govern-refinement/scenarios/configure-retires-formerly-canonical-entries.md` — 023-govern-refinement
- `specs/023-govern-refinement/spec.md` — 023-govern-refinement
- `specs/023-govern-refinement/tasks.md` — 023-govern-refinement
- `specs/026-framework-self-audit/plan.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/families-follow-the-command-reference-split.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/host-namespace-parity.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/spec.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/tasks.md` — 026-framework-self-audit
- `specs/029-bootstrap-runtime-autowire/plan.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/scenarios/archive-fetch-direct-codeload.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/scenarios/project-inputs-asked-once.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/scenarios/runtime-probe-parity-audit.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/scenarios/state-a-deterministic-path-forcing.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/spec.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/tasks.md` — 029-bootstrap-runtime-autowire
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/plan.md` — 042-consolidate-govern-per-project-files-under-govern-directory
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/spec.md` — 042-consolidate-govern-per-project-files-under-govern-directory
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/tasks.md` — 042-consolidate-govern-per-project-files-under-govern-directory
- `specs/043-workflows-sunset/plan.md` — 043-workflows-sunset
- `specs/043-workflows-sunset/spec.md` — 043-workflows-sunset
- `specs/043-workflows-sunset/tasks.md` — 043-workflows-sunset
- `specs/045-decision-state-drift-detection/data-model.md` — 045-decision-state-drift-detection
- `specs/045-decision-state-drift-detection/spec.md` — 045-decision-state-drift-detection
- `specs/048-govern-acquired-runtime/data-model.md` — 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/plan.md` — 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/scenarios/retired-namespace-tools-are-off-limits.md` — 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/scenarios/state-b-continues-in-session.md` — 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/spec.md` — 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/tasks.md` — 048-govern-acquired-runtime
- `specs/049-rename-govern-to-ductus/plan.md` — 049-rename-govern-to-ductus
- `specs/049-rename-govern-to-ductus/spec.md` — 049-rename-govern-to-ductus
- `specs/049-rename-govern-to-ductus/tasks.md` — 049-rename-govern-to-ductus
- `specs/050-constitution/plan.md` — 050-constitution
- `specs/050-constitution/scenarios/memory-routing-lives-in-agents-md.md` — 050-constitution
- `specs/050-constitution/scenarios/the-constitution-fits-one-read.md` — 050-constitution
- `specs/050-constitution/tasks.md` — 050-constitution
- `specs/054-remove-supersession/plan.md` — 054-remove-supersession
- `specs/054-remove-supersession/spec.md` — 054-remove-supersession
- `specs/054-remove-supersession/tasks.md` — 054-remove-supersession
- `specs/055-shared-constitution/analysis.md` — 055-shared-constitution
- `specs/055-shared-constitution/data-model.md` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `specs/055-shared-constitution/plan.md` — 055-shared-constitution
- `specs/055-shared-constitution/review.md` — 055-shared-constitution
- `specs/055-shared-constitution/spec.md` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `specs/055-shared-constitution/tasks.md` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `specs/056-bootstrap-archive-boundary-split/spec.md` — 056-bootstrap-archive-boundary-split
- `specs/058-findings-route-at-discovery/plan.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/spec.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/tasks.md` — 058-findings-route-at-discovery
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — 060-exec-analyze-assesses-each-loaded-rule
- `specs/060-exec-analyze-assesses-each-loaded-rule/scenarios/an-assessment-sees-the-rule-it-asks-about.md` — 060-exec-analyze-assesses-each-loaded-rule
- `specs/061-updates-track-the-latest-release-tag/plan.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/spec.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/tasks.md` — 061-updates-track-the-latest-release-tag
- `specs/062-concurrent-session-targets/plan.md` — 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/tasks.md` — 062-concurrent-session-targets
- `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — 063-oversized-artifacts-warn-with-a-fix
- `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md` — 063-oversized-artifacts-warn-with-a-fix
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 066-audit-records-and-the-completion-gate
- `specs/067-analyze-and-audit-checks/data-model.md` — 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `specs/067-analyze-and-audit-checks/scenarios/criterion-adopter-scope-destinations.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md` — 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `specs/067-analyze-and-audit-checks/scenarios/every-agents-generated-copies-are-excluded.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/orphaned-reference-check.md` — 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `specs/067-analyze-and-audit-checks/scenarios/per-agent-scaffold-paths-ship-to-adopter.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/pointers-into-shipped-files-are-judged.md` — 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `specs/067-analyze-and-audit-checks/tasks.md` — 067-analyze-and-audit-checks, 071-command-reference-split, 080-deterministic-runtime-efficiency
- `specs/069-accessibility-frontend-rules/plan.md` — 069-accessibility-frontend-rules
- `specs/071-command-reference-split/plan.md` — 071-command-reference-split
- `specs/071-command-reference-split/spec.md` — 071-command-reference-split
- `specs/071-command-reference-split/tasks.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/072-governing-documents-stay-within-one-read/plan.md` — 072-governing-documents-stay-within-one-read
- `specs/075-agents-md-holds-what-every-session-needs/plan.md` — 075-agents-md-holds-what-every-session-needs
- `specs/075-agents-md-holds-what-every-session-needs/spec.md` — 075-agents-md-holds-what-every-session-needs
- `specs/075-agents-md-holds-what-every-session-needs/tasks.md` — 075-agents-md-holds-what-every-session-needs
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/079-review-and-analysis-work-converges/plan.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/tasks.md` — 079-review-and-analysis-work-converges
- `specs/080-deterministic-runtime-efficiency/analysis.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/data-model.md` — 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/plan.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/research.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/review.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/configure-types-no-permission-entries.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/help-answers-with-no-model-turn.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/rules-load-without-shell-reads.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/spec.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/tasks.md` — 080-deterministic-runtime-efficiency

## Unexamined governance

*None.*
