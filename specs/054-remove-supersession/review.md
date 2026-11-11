---
spec: 054-remove-supersession
last-run: 2026-11-11T16:23:01Z
reviewed-against: 642e18672aaaebc033c891f16c3f040dcd82e1a7
diff-base: 27ad0c88b857523340b8d70cb7a2377073c8e3b6
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 3
scope: 5
excluded: 126
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
  waived: 8
  undispositioned: 0
decisions:
  - key: "other: docs/slash-commands.md says /consolidate re-points only the spec corpus's share of pointers, though it also re-points the root files check-orphaned-references reads — `docs/slash-commands.md`"
    outcome: waived
    reason: It understates rather than overstates what /consolidate re-points (README.md, AGENTS.md, CLAUDE.md and the pre-commit hook too), so no reader leaves a pointer dangling by trusting it, and its rewording is not mechanical.
    decided-at: 2026-11-09T23:27:44Z
    decided-by: rule
  - key: "convention: no numbered step of the bootstrap names the two managed-block merges — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: Instructions step 4 lists what apply-manifest replaces, not every Shared Files step, and two real adopter bootstraps on this text wrote both blocks from Shared Files
    decided-at: 2026-11-11T16:23:01Z
    decided-by: andy@stone.dev
  - key: "convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
    decided-at: 2026-11-11T16:23:01Z
    decided-by: andy@stone.dev
  - key: "convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
    decided-at: 2026-11-11T16:23:01Z
    decided-by: andy@stone.dev
  - key: "convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
    decided-at: 2026-11-11T16:23:01Z
    decided-by: andy@stone.dev
  - key: "other: an adopter with an installed .augment, .agents or .opencode tree gets no notice from /ductus, nothing removes it, and the managed .gitignore block no longer ignores it — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the owner dropped the three hosts as a breaking change; the CHANGELOG's Removed entry tells such an adopter to remove the tree by hand, the runtime refuses writes there by the owner's exception, and detecting a dropped host's tree would be new bootstrap behavior rather than a fix to this change
    decided-at: 2026-11-11T16:23:01Z
    decided-by: rule
  - key: "other: with surface-instruction gone, the MCP registration table's mechanism column holds one value — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the column is the extension point the Adding a new agent steps cite; dropping it is a design change, not removal residue
    decided-at: 2026-11-11T16:23:01Z
    decided-by: rule
  - key: "stale: the Derived values row label Command/skill path still names the skill layout — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: "a label, not a claim: it still names the command-file path both layouts use, no parser keys on it, and renaming it is a seven-spec sweep, two of them scenarios, for no behavior change"
    decided-at: 2026-11-11T16:23:01Z
    decided-by: rule
  - key: "stale: task 13's handoff prose still describes the 2026-10-29 reopen as open, though its done-when was met when 054 closed at 0ab463a4 — `specs/054-remove-supersession/tasks.md`"
    outcome: waived
    reason: tasks.md is ephemeral work tracking that /prune may drop; task 13 carries no box, so no gate reads it, and rewriting a spent task's prose adds nothing a reader of the spec or its records needs
    decided-at: 2026-11-11T16:23:01Z
    decided-by: rule
---

# Review — 054-remove-supersession

## Summary

054's reopening window, 27ad0c88..642e1867 (base-source reopen), holds 5 scope entries from three commits. f6ed8d12 is its sweep, which dropped Auggie's configure file from the removal surface, AC14 and the plan, and the antigravity and opencode rows from the cleanup table, AC16 and two tasks. 0afc0c00 is the two-line edit to the bootstrap's Pi prompt-template paragraph that b1d319f1's agent-host removal owed, with govern.md mirroring it. 8c7c0ad6 is this spec's, made after the criteria verification and before this review: it annotates AC3 as never true as written, and adds 067's anchor-reference-kinds scenario (46793ce2) to AC19's Superseded in part note. Per AGENTS.md's shared-window rule, b1d319f1 and 0afc0c00 were read in depth once, by 064's review 2411dca9, whose window holds 0afc0c00. Since 064's close ductus.md has changed only in 015's abort-message line (4b6f0411, 015's), which none of the carried decisions concerns. This review re-read 0afc0c00's hunk and leans on 064's read for the rest of ductus.md. The seven decisions on ductus.md that 064's record stores are carried with byte-identical text and disposition. One of them waives the missing notice for a dropped host's leftover tree, which the owner has since scheduled as 012's task 13 (2026-11-11); each clause of its reason still holds. 054's own stored decision, on docs/slash-commands.md, is retained, as this window did not read that file. Examined counts the 3 in-scope files the passes read in full, of 5: 054's spec.md, plan.md and tasks.md. Not opened: ductus.md beyond 0afc0c00's paragraph, and govern.md, byte-identical to ductus.md by cmp. The removal's deliverables are unchanged in the window. All 12 rule files were selected. The 3 frontend files have no subject in this scope, and no rule of the 9 backend and cross files bears on the window's change, which narrowed mentions and annotated two criteria. 0 MUST, 0 SHOULD, 0 low-confidence. One new observation, task 13's spent handoff prose, is waived by the rule. This run made no fix.

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

- convention: no numbered step of the bootstrap names the two managed-block merges — `framework/bootstrap/ductus.md` — **waived**: Instructions step 4 lists what apply-manifest replaces, not every Shared Files step, and two real adopter bootstraps on this text wrote both blocks from Shared Files
- convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md` — **waived**: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
- convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md` — **waived**: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
- convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md` — **waived**: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
- other: an adopter with an installed .augment, .agents or .opencode tree gets no notice from /ductus, nothing removes it, and the managed .gitignore block no longer ignores it — `framework/bootstrap/ductus.md` — **waived**: the owner dropped the three hosts as a breaking change; the CHANGELOG's Removed entry tells such an adopter to remove the tree by hand, the runtime refuses writes there by the owner's exception, and detecting a dropped host's tree would be new bootstrap behavior rather than a fix to this change (decided by the rule)
- other: with surface-instruction gone, the MCP registration table's mechanism column holds one value — `framework/bootstrap/ductus.md` — **waived**: the column is the extension point the Adding a new agent steps cite; dropping it is a design change, not removal residue (decided by the rule)
- stale: the Derived values row label Command/skill path still names the skill layout — `framework/bootstrap/ductus.md` — **waived**: a label, not a claim: it still names the command-file path both layouts use, no parser keys on it, and renaming it is a seven-spec sweep, two of them scenarios, for no behavior change (decided by the rule)
- stale: task 13's handoff prose still describes the 2026-10-29 reopen as open, though its done-when was met when 054 closed at 0ab463a4 — `specs/054-remove-supersession/tasks.md` — **waived**: tasks.md is ephemeral work tracking that /prune may drop; task 13 carries no box, so no gate reads it, and rewriting a spent task's prose adds nothing a reader of the spec or its records needs (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md` — 067-analyze-and-audit-checks
- `.claude/commands/ductus/target.md` — 009-scenario-targeting
- `.ductus/reference/analyze.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `.ductus/reference/audit.md` — 064-pi-host-support
- `.githooks/pre-commit` — 067-analyze-and-audit-checks, 071-command-reference-split
- `.github/workflows/framework-checks.yml` — 022-deterministic-runtime
- `.pi/prompts/ductus-analyze.md` — 067-analyze-and-audit-checks
- `.pi/prompts/ductus-configure.md` — 064-pi-host-support
- `.pi/prompts/ductus-target.md` — 009-scenario-targeting
- `AGENTS.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `README.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `docs/developing-ductus.md` — 071-command-reference-split
- `framework/bootstrap/configure/pi.md` — 064-pi-host-support
- `framework/bootstrap/ductus-procedure.md` — 015-tarball-fetch, 067-analyze-and-audit-checks, 071-command-reference-split
- `framework/bootstrap/hooks/ductus-pre-commit` — 012-multi-agent-govern, 067-analyze-and-audit-checks
- `framework/commands/analyze.md` — 067-analyze-and-audit-checks
- `framework/commands/target.md` — 009-scenario-targeting
- `framework/reference/analyze.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `framework/reference/audit.md` — 064-pi-host-support
- `runtime/CHANGELOG.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `runtime/src/host.rs` — 064-pi-host-support
- `runtime/src/primitives/apply_manifest.rs` — 064-pi-host-support
- `runtime/src/primitives/check_artifacts.rs` — 064-pi-host-support
- `runtime/src/primitives/enforce_manifest.rs` — 064-pi-host-support
- `runtime/src/primitives/host_config.rs` — 064-pi-host-support
- `runtime/src/primitives/merge_managed_block.rs` — 064-pi-host-support
- `runtime/src/primitives/merge_permissions.rs` — 064-pi-host-support
- `runtime/src/schema/descriptions.rs` — 064-pi-host-support
- `runtime/src/schema/primitives.rs` — 064-pi-host-support
- `scripts/audit/README.md` — 064-pi-host-support
- `scripts/audit/done-spec-criteria.sh` — 026-framework-self-audit
- `scripts/audit/host-namespace-parity.sh` — 022-deterministic-runtime, 064-pi-host-support
- `scripts/audit/installer-registry-parity.sh` — 067-analyze-and-audit-checks, 071-command-reference-split
- `scripts/audit/manifest-parity.sh` — 064-pi-host-support
- `scripts/audit/permission-entry-shape.sh` — 064-pi-host-support
- `scripts/audit/permission-wildcard-position.sh` — 064-pi-host-support
- `scripts/audit/runtime-probe-parity.sh` — 064-pi-host-support
- `scripts/gen-configure-mcp.sh` — 064-pi-host-support, 067-analyze-and-audit-checks, 071-command-reference-split
- `scripts/tests/test-host-namespace-parity.sh` — 022-deterministic-runtime
- `scripts/tests/test-install.sh` — 064-pi-host-support
- `specs/002-project-scaffolding/analysis.md` — 002-project-scaffolding
- `specs/002-project-scaffolding/plan.md` — 002-project-scaffolding
- `specs/002-project-scaffolding/review.md` — 002-project-scaffolding
- `specs/002-project-scaffolding/spec.md` — 002-project-scaffolding
- `specs/003-bootstrap-automation/tasks.md` — 003-bootstrap-automation
- `specs/009-scenario-targeting/analysis.md` — 009-scenario-targeting
- `specs/009-scenario-targeting/plan.md` — 009-scenario-targeting
- `specs/009-scenario-targeting/review.md` — 009-scenario-targeting
- `specs/009-scenario-targeting/spec.md` — 009-scenario-targeting
- `specs/009-scenario-targeting/tasks.md` — 009-scenario-targeting
- `specs/012-multi-agent-govern/tasks.md` — 012-multi-agent-govern, 067-analyze-and-audit-checks
- `specs/015-tarball-fetch/analysis.md` — 015-tarball-fetch
- `specs/015-tarball-fetch/plan.md` — 015-tarball-fetch
- `specs/015-tarball-fetch/review.md` — 015-tarball-fetch
- `specs/015-tarball-fetch/spec.md` — 015-tarball-fetch
- `specs/015-tarball-fetch/tasks.md` — 015-tarball-fetch
- `specs/022-deterministic-runtime/scenarios/govern-bootstrap.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/the-constitutions-registry-validates-its-values.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/tasks.md` — 022-deterministic-runtime
- `specs/029-bootstrap-runtime-autowire/tasks.md` — 029-bootstrap-runtime-autowire
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/analysis.md` — 042-consolidate-govern-per-project-files-under-govern-directory
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/plan.md` — 042-consolidate-govern-per-project-files-under-govern-directory
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/review.md` — 042-consolidate-govern-per-project-files-under-govern-directory
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/spec.md` — 042-consolidate-govern-per-project-files-under-govern-directory
- `specs/043-workflows-sunset/analysis.md` — 043-workflows-sunset
- `specs/043-workflows-sunset/plan.md` — 043-workflows-sunset
- `specs/043-workflows-sunset/review.md` — 043-workflows-sunset
- `specs/043-workflows-sunset/spec.md` — 043-workflows-sunset
- `specs/046-scenario-open-question-visibility/analysis.md` — 046-scenario-open-question-visibility
- `specs/046-scenario-open-question-visibility/plan.md` — 046-scenario-open-question-visibility
- `specs/046-scenario-open-question-visibility/review.md` — 046-scenario-open-question-visibility
- `specs/046-scenario-open-question-visibility/spec.md` — 046-scenario-open-question-visibility
- `specs/055-shared-constitution/plan.md` — 055-shared-constitution
- `specs/055-shared-constitution/tasks.md` — 055-shared-constitution
- `specs/056-bootstrap-archive-boundary-split/spec.md` — 056-bootstrap-archive-boundary-split
- `specs/058-findings-route-at-discovery/plan.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/spec.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/tasks.md` — 058-findings-route-at-discovery
- `specs/061-updates-track-the-latest-release-tag/plan.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/spec.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/tasks.md` — 061-updates-track-the-latest-release-tag
- `specs/062-concurrent-session-targets/plan.md` — 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/tasks.md` — 062-concurrent-session-targets
- `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — 063-oversized-artifacts-warn-with-a-fix
- `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md` — 063-oversized-artifacts-warn-with-a-fix
- `specs/064-pi-host-support/analysis.md` — 064-pi-host-support
- `specs/064-pi-host-support/plan.md` — 064-pi-host-support
- `specs/064-pi-host-support/review.md` — 064-pi-host-support
- `specs/064-pi-host-support/spec.md` — 064-pi-host-support
- `specs/064-pi-host-support/tasks.md` — 064-pi-host-support
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 066-audit-records-and-the-completion-gate
- `specs/067-analyze-and-audit-checks/data-model.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/scenarios/a-scenario-file-is-checked-against-the-scenario-schema.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/scenarios/body-path-existence-family.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/scenarios/criterion-adopter-scope-destinations.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/derive-references-unstaged-drift-is-reported.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/scenarios/every-agents-generated-copies-are-excluded.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/frontmatter-severity-tiers-on-both-sides.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/scenarios/per-agent-scaffold-paths-ship-to-adopter.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/status-gated-families-are-not-silent.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/traverse-deps-cycle-check.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/tasks.md` — 012-multi-agent-govern, 067-analyze-and-audit-checks
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
- `specs/080-deterministic-runtime-efficiency/data-model.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/plan.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/research.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/review.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/configure-types-no-permission-entries.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/help-answers-with-no-model-turn.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/spec.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/tasks.md` — 080-deterministic-runtime-efficiency

## Unexamined governance

*None.*
