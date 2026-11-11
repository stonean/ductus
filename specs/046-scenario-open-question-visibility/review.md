---
spec: 046-scenario-open-question-visibility
last-run: 2026-11-11T14:00:08Z
reviewed-against: fb587c87b2cb9b42301d81b43ed3c99fe63a6185
diff-base: 5d0dfb0191e1e567987595d075b5e8d6dfe4a50e
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 2
scope: 69
excluded: 257
contract-breaks: 0
skipped-passes: []
fix-rounds: 0
re-reads: 0
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest: {}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 17
  undispositioned: 0
decisions:
  - key: "edge: a session or config naming a dropped host is not diagnosed: .augment still resolves through the claude-style candidate, and .opencode or .agents ends in command-not-found naming the paths tried — `runtime/src/host.rs`"
    outcome: waived
    reason: Host::load documents that a config-dir mismatch surfaces as the command-file-not-found error, which names every path tried; the resolver is generic in cli-config-dir by design, so a custom directory keeps working, and the CHANGELOG's Removed entry announces the removal
    decided-at: 2026-11-11T14:00:08Z
    decided-by: rule
  - key: "test: every_agents_generated_copies_are_excluded derives its expected count from the fixture's own array — `runtime/src/primitives/check_corpus_links.rs`"
    outcome: waived
    reason: the broken-links assertion still fails when a copy stops being excluded, so the test is not vacuous
    decided-at: 2026-11-11T14:00:08Z
    decided-by: rule
  - key: "test: the check_artifacts REGISTRY fixture carries a scope column the parser does not read — `runtime/src/primitives/check_artifacts.rs`"
    outcome: waived
    reason: the fixture mirrors the real file's table shapes, and the real MCP registration table carries the column
    decided-at: 2026-11-11T14:00:08Z
    decided-by: rule
  - key: "other: an adopter with an installed .augment, .agents or .opencode tree gets no notice from /ductus, nothing removes it, and the managed .gitignore block no longer ignores it — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the owner dropped the three hosts as a breaking change; the CHANGELOG's Removed entry tells such an adopter to remove the tree by hand, the runtime refuses writes there by the owner's exception, and detecting a dropped host's tree would be new bootstrap behavior rather than a fix to this change
    decided-at: 2026-11-11T14:00:08Z
    decided-by: rule
  - key: "perf: install.sh validates the agent key only after the latest-release lookup and the bootstrap download — `install.sh`"
    outcome: waived
    reason: the order predates the removal, the arm still halts before writing anything, and moving the check is a behavior change with its own test
    decided-at: 2026-11-11T14:00:08Z
    decided-by: rule
  - key: "other: with surface-instruction gone, the MCP registration table's mechanism column holds one value — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the column is the extension point the Adding a new agent steps cite; dropping it is a design change, not removal residue
    decided-at: 2026-11-11T14:00:08Z
    decided-by: rule
  - key: "other: the swept migrations narrow three procedures to Claude, so a dropped-host adopter far enough behind loses their rewrites silently — `framework/migrations/runtime-store-path.md`"
    outcome: waived
    reason: no entry was removed or renumbered, so no last_applied names a missing id; claude and pi adopters run the same steps as before, and the hosts were dropped
    decided-at: 2026-11-11T14:00:08Z
    decided-by: rule
  - key: "test: test-gen-configure-mcp.sh has never asserted that --dry-run exits 1 on drift or that a run rewrites the block — `scripts/tests/test-gen-configure-mcp.sh`"
    outcome: waived
    reason: a gap the removal neither opened nor widened; check-zero runs the same --dry-run, and the reviewer proved both behaviors by hand on a broken copy
    decided-at: 2026-11-11T14:00:08Z
    decided-by: rule
  - key: "stale: the Derived values row label Command/skill path still names the skill layout — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: "a label, not a claim: it still names the command-file path both layouts use, no parser keys on it, and renaming it is a seven-spec sweep, two of them scenarios, for no behavior change"
    decided-at: 2026-11-11T14:00:08Z
    decided-by: rule
  - key: "other: Families 2, 29 and 32 hardcode configure/claude.md, so a later agent's permission file would go unchecked — `scripts/audit/manifest-parity.sh`"
    outcome: waived
    reason: each script states the bound, the same held for any further host before the removal, and a guard enumerating configure files would be a new family
    decided-at: 2026-11-11T14:00:08Z
    decided-by: rule
  - key: "other: the constitution line naming plan.md carries § references resolve-anchor classes as qualified and does not judge — `framework/constitution.md`"
    outcome: waived
    reason: both targets, §brownfield-inbox and §drift-prevention, carry markers (checked by hand), and citing a subsection's marked section is the constitution's own anchor rule
    decided-at: 2026-11-11T14:00:08Z
    decided-by: andy@stone.dev
  - key: "convention: no numbered step of the bootstrap names the two managed-block merges — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: Instructions step 4 lists what apply-manifest replaces, not every Shared Files step, and two real adopter bootstraps on this text wrote both blocks from Shared Files
    decided-at: 2026-11-11T14:00:08Z
    decided-by: andy@stone.dev
  - key: "convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
    decided-at: 2026-11-11T14:00:08Z
    decided-by: andy@stone.dev
  - key: "convention: the Closing restart gives no single form for Pi wired in the run beside Claude already registered — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: the per-agent forms compose into an accurate notice, as the subject's run shows, so no host reads it wrong
    decided-at: 2026-11-11T14:00:08Z
    decided-by: andy@stone.dev
  - key: "convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
    decided-at: 2026-11-11T14:00:08Z
    decided-by: andy@stone.dev
  - key: "convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
    decided-at: 2026-11-11T14:00:08Z
    decided-by: andy@stone.dev
  - key: "convention: the Frontmatter Migration's specs/**/spec.md walk reports specs/templates/spec.md as already frontmatter on every run — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: the template has frontmatter, so the walk skips it with a report line and writes nothing
    decided-at: 2026-11-11T14:00:08Z
    decided-by: andy@stone.dev
---

# Review — 046-scenario-open-question-visibility

## Summary

046's reopening window, 5d0dfb01..fb587c87 (base-source reopen), shares b1d319f1, the chore with no spec that dropped Auggie, Antigravity and OpenCode, with every spec that chore widened. Per AGENTS.md's shared-window rule this review reads only what is 046's own and leans on the one in-depth read of b1d319f1, 064's review 2411dca9: there three independent reviewers read b1d319f1's change on all 68 of its live paths in full against the nine backend and cross rule files, drove it on fixtures, and found one MUST and fourteen chores, fixed in e2009d70 (064's commit) and re-read clean. Its ten observations still holding at HEAD that the rule waived, and the seven stored decisions that review carried, are carried here byte for byte; its waiver of Family 14's Direction 3 is not, since 071's c9b2155c fixed it. What is 046's own: its reopening edit, 37693d4d, which reworded AC24 and its edge case for 080's pipeline view, and its deliverable. The pipeline view's cut was checked against the claim: past the read size its scenario-question item shrinks to a question-line count followed by the not-shown words, which name the file holding the whole view when one exists (pipeline_view.rs, not_shown and the ScenarioQuestions cut, with its test), so AC24 holds. b1d319f1 changed 046's deliverable only in comments (dashboard.rs, check_artifacts.rs) and in a constitution line beside, not in, the done row and readiness check AC7 rests on, both of which still name scenario questions. No finding and no new observation, and no fix, so no fix round. Examined counts the two in-scope files read in full by this run, spec.md and framework/constitution.md, of 69 in scope; the other 67 are b1d319f1's, read in 064's review as named above and not re-read here. The three frontend rule files have no subject in this scope.

0 fix round(s): 0 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

- edge: a session or config naming a dropped host is not diagnosed: .augment still resolves through the claude-style candidate, and .opencode or .agents ends in command-not-found naming the paths tried — `runtime/src/host.rs` — **waived**: Host::load documents that a config-dir mismatch surfaces as the command-file-not-found error, which names every path tried; the resolver is generic in cli-config-dir by design, so a custom directory keeps working, and the CHANGELOG's Removed entry announces the removal (decided by the rule)
- test: every_agents_generated_copies_are_excluded derives its expected count from the fixture's own array — `runtime/src/primitives/check_corpus_links.rs` — **waived**: the broken-links assertion still fails when a copy stops being excluded, so the test is not vacuous (decided by the rule)
- test: the check_artifacts REGISTRY fixture carries a scope column the parser does not read — `runtime/src/primitives/check_artifacts.rs` — **waived**: the fixture mirrors the real file's table shapes, and the real MCP registration table carries the column (decided by the rule)
- other: an adopter with an installed .augment, .agents or .opencode tree gets no notice from /ductus, nothing removes it, and the managed .gitignore block no longer ignores it — `framework/bootstrap/ductus.md` — **waived**: the owner dropped the three hosts as a breaking change; the CHANGELOG's Removed entry tells such an adopter to remove the tree by hand, the runtime refuses writes there by the owner's exception, and detecting a dropped host's tree would be new bootstrap behavior rather than a fix to this change (decided by the rule)
- perf: install.sh validates the agent key only after the latest-release lookup and the bootstrap download — `install.sh` — **waived**: the order predates the removal, the arm still halts before writing anything, and moving the check is a behavior change with its own test (decided by the rule)
- other: with surface-instruction gone, the MCP registration table's mechanism column holds one value — `framework/bootstrap/ductus.md` — **waived**: the column is the extension point the Adding a new agent steps cite; dropping it is a design change, not removal residue (decided by the rule)
- other: the swept migrations narrow three procedures to Claude, so a dropped-host adopter far enough behind loses their rewrites silently — `framework/migrations/runtime-store-path.md` — **waived**: no entry was removed or renumbered, so no last_applied names a missing id; claude and pi adopters run the same steps as before, and the hosts were dropped (decided by the rule)
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
- `.claude/commands/ductus/target.md` — 000-slash-commands, 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets, 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `.ductus/reference/clarify.md` — 067-analyze-and-audit-checks
- `.ductus/reference/implement.md` — 067-analyze-and-audit-checks
- `.ductus/reference/plan.md` — 067-analyze-and-audit-checks
- `.github/workflows/framework-checks.yml` — 022-deterministic-runtime
- `.github/workflows/generators.yml` — 080-deterministic-runtime-efficiency
- `.github/workflows/runtime.yml` — 080-deterministic-runtime-efficiency
- `.pi/extensions/ductus.ts` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-amend.md` — 067-analyze-and-audit-checks
- `.pi/prompts/ductus-analyze.md` — 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-clarify.md` — 067-analyze-and-audit-checks
- `.pi/prompts/ductus-review.md` — 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-status.md` — 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-target.md` — 000-slash-commands, 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets, 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `AGENTS.md` — 050-constitution, 064-pi-host-support, 067-analyze-and-audit-checks, 071-command-reference-split, 080-deterministic-runtime-efficiency
- `framework/bootstrap/pi/ductus-bridge.ts` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `framework/commands/amend.md` — 067-analyze-and-audit-checks
- `framework/commands/analyze.md` — 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `framework/commands/clarify.md` — 067-analyze-and-audit-checks
- `framework/commands/review.md` — 080-deterministic-runtime-efficiency
- `framework/commands/status.md` — 080-deterministic-runtime-efficiency
- `framework/commands/target.md` — 000-slash-commands, 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets, 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
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
- `specs/000-slash-commands/scenarios/dashboard-dependencies-column.md` — 000-slash-commands, 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `specs/000-slash-commands/scenarios/target-clear-flag.md` — 000-slash-commands, 022-deterministic-runtime, 080-deterministic-runtime-efficiency
- `specs/000-slash-commands/tasks.md` — 000-slash-commands
- `specs/002-project-scaffolding/scenarios/claude-md-template-imports-load.md` — 002-project-scaffolding
- `specs/002-project-scaffolding/spec.md` — 002-project-scaffolding
- `specs/002-project-scaffolding/tasks.md` — 002-project-scaffolding
- `specs/003-bootstrap-automation/scenarios/curl-sh-installer.md` — 003-bootstrap-automation
- `specs/003-bootstrap-automation/tasks.md` — 003-bootstrap-automation
- `specs/007-govern-workflow/plan.md` — 007-govern-workflow
- `specs/007-govern-workflow/scenarios/ductus-self-update-precheck.md` — 007-govern-workflow
- `specs/007-govern-workflow/spec.md` — 007-govern-workflow
- `specs/007-govern-workflow/tasks.md` — 007-govern-workflow, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate
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
- `specs/022-deterministic-runtime/scenarios/cli-config-dir-per-contributor.md` — 000-slash-commands, 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `specs/022-deterministic-runtime/scenarios/commands-dir-parameterization.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/config-resolution-single-probe.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/coverage-expansion-primitives.md` — 022-deterministic-runtime
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
- `specs/062-concurrent-session-targets/plan.md` — 007-govern-workflow, 022-deterministic-runtime, 055-shared-constitution, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate
- `specs/062-concurrent-session-targets/tasks.md` — 062-concurrent-session-targets
- `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — 063-oversized-artifacts-warn-with-a-fix
- `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — 007-govern-workflow, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate
- `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md` — 063-oversized-artifacts-warn-with-a-fix
- `specs/064-pi-host-support/analysis.md` — 064-pi-host-support
- `specs/064-pi-host-support/plan.md` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `specs/064-pi-host-support/review.md` — 064-pi-host-support
- `specs/064-pi-host-support/spec.md` — 064-pi-host-support, 080-deterministic-runtime-efficiency
- `specs/064-pi-host-support/tasks.md` — 064-pi-host-support
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 007-govern-workflow, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/scenarios/write-analysis-and-the-second-gate.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 066-audit-records-and-the-completion-gate
- `specs/067-analyze-and-audit-checks/data-model.md` — 067-analyze-and-audit-checks, 071-command-reference-split, 080-deterministic-runtime-efficiency
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/scenarios/body-path-existence-family.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/scenarios/criterion-adopter-scope-destinations.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/derive-references-unstaged-drift-is-reported.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md` — 067-analyze-and-audit-checks, 071-command-reference-split, 080-deterministic-runtime-efficiency
- `specs/067-analyze-and-audit-checks/scenarios/every-agents-generated-copies-are-excluded.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/frontmatter-severity-tiers-on-both-sides.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/scenarios/orphaned-reference-check.md` — 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `specs/067-analyze-and-audit-checks/scenarios/per-agent-scaffold-paths-ship-to-adopter.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/pointers-into-shipped-files-are-judged.md` — 067-analyze-and-audit-checks, 080-deterministic-runtime-efficiency
- `specs/067-analyze-and-audit-checks/scenarios/traverse-deps-cycle-check.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/tasks.md` — 067-analyze-and-audit-checks, 071-command-reference-split, 080-deterministic-runtime-efficiency
- `specs/069-accessibility-frontend-rules/plan.md` — 069-accessibility-frontend-rules
- `specs/071-command-reference-split/analysis.md` — 071-command-reference-split
- `specs/071-command-reference-split/plan.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/071-command-reference-split/review.md` — 067-analyze-and-audit-checks, 071-command-reference-split
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
- `specs/080-deterministic-runtime-efficiency/tasks.md` — 000-slash-commands, 022-deterministic-runtime, 080-deterministic-runtime-efficiency

## Unexamined governance

*None.*
