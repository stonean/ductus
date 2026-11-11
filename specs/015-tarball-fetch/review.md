---
spec: 015-tarball-fetch
last-run: 2026-11-11T15:52:57Z
reviewed-against: 788971619cbf64a76e152dab17a64cb50d6b1489
diff-base: 581673ae330ba418fbdfcf801e2b15f8fa0bcc1b
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 3
scope: 6
excluded: 168
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 0
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest: {}
blocking: false
dispositions:
  fixed: 5
  deferred: 0
  waived: 9
  undispositioned: 0
decisions:
  - key: "convention: no numbered step of the bootstrap names the two managed-block merges — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: Instructions step 4 lists what apply-manifest replaces, not every Shared Files step, and two real adopter bootstraps on this text wrote both blocks from Shared Files
    decided-at: 2026-11-11T15:52:57Z
    decided-by: andy@stone.dev
  - key: "convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
    decided-at: 2026-11-11T15:52:57Z
    decided-by: andy@stone.dev
  - key: "convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
    decided-at: 2026-11-11T15:52:57Z
    decided-by: andy@stone.dev
  - key: "convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
    decided-at: 2026-11-11T15:52:57Z
    decided-by: andy@stone.dev
  - key: "other: an adopter with an installed .augment, .agents or .opencode tree gets no notice from /ductus, nothing removes it, and the managed .gitignore block no longer ignores it — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the owner dropped the three hosts as a breaking change; the CHANGELOG's Removed entry tells such an adopter to remove the tree by hand, the runtime refuses writes there by the owner's exception, and detecting a dropped host's tree would be new bootstrap behavior rather than a fix to this change
    decided-at: 2026-11-11T15:52:57Z
    decided-by: rule
  - key: "other: with surface-instruction gone, the MCP registration table's mechanism column holds one value — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the column is the extension point the Adding a new agent steps cite; dropping it is a design change, not removal residue
    decided-at: 2026-11-11T15:52:57Z
    decided-by: rule
  - key: "stale: the Derived values row label Command/skill path still names the skill layout — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: "a label, not a claim: it still names the command-file path both layouts use, no parser keys on it, and renaming it is a seven-spec sweep, two of them scenarios, for no behavior change"
    decided-at: 2026-11-11T15:52:57Z
    decided-by: rule
  - key: "convention: the Closing restart gives no single form for Pi wired in the run beside Claude already registered — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: the per-agent forms compose into an accurate notice, as the subject's run shows, so no host reads it wrong
    decided-at: 2026-11-11T15:52:57Z
    decided-by: andy@stone.dev
  - key: "convention: the Frontmatter Migration's specs/**/spec.md walk reports specs/templates/spec.md as already frontmatter on every run — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: the template has frontmatter, so the walk skips it with a report line and writes nothing
    decided-at: 2026-11-11T15:52:57Z
    decided-by: andy@stone.dev
---

# Review — 015-tarball-fetch

## Summary

015's reopening window, 581673ae..78897161 (base-source reopen), holds 6 scope entries from three commits. edebca93 is its sweep, which dropped Auggie from the spec, plan and tasks. 0afc0c00 is the two-line edit to the bootstrap's Pi prompt-template paragraph that b1d319f1's agent-host removal owed, with govern.md mirroring it. 4b6f0411 is task 6: the criteria verification before this review found AC6 held only by the Cleanup paragraph, as neither template the host renders carried the temp path, and the owner chose to have them carry it (2026-11-11). The archive abort message in ductus.md and govern.md now names {tempdir}, and the procedure's Post-Scaffolding Output gains a temp-directory line; {tempdir} exists on every run, created first by the Pre-flight Phase. 012's task 12 holds the real adopter run that checks it. Per AGENTS.md's shared-window rule, b1d319f1 and 0afc0c00 were read in depth once, by 064's review 2411dca9, whose window holds 0afc0c00; ductus.md and ductus-procedure.md are unchanged since 064's close at fb587c87 but for task 6's two lines. This review read task 6's hunks with the sections around them (Archive fetch and extract and Cleanup in ductus.md; Post-Scaffolding Output in the procedure), re-read 0afc0c00's hunk, and leans on 064's read for the rest of both files. The seven decisions on ductus.md and two on ductus-procedure.md that 064's record stores are carried with byte-identical text and disposition. One of them waives the missing notice for a dropped host's leftover tree, which the owner has since scheduled as 012's task 13 (2026-11-11); each clause of its reason still holds. Examined counts the 3 in-scope files the passes read in full, of 6: 015's spec.md, plan.md and tasks.md. Not opened in full: ductus.md and ductus-procedure.md, read as above; govern.md, byte-identical to ductus.md by cmp. All 12 rule files were selected. The 3 frontend files have no subject in this scope, and no rule of the 9 backend and cross files fires on the window's change: the sweep's prose, and two template lines naming a temp path, which SEC-LOG-001 does not cover, since a temp directory is no secret. 0 MUST, 0 SHOULD, 0 low-confidence. Five new observations, each a stale claim the passes found in 015's own spec or plan, are fixed by the rule in this run: the temp directory's creation order, the narrowed mktemp seed, the plan's hardcoded framework root and archive name, its retired CLAUDE.md citation, and its Affected Files and question count. The fixes' one round holds only spec.md and plan.md, which the analysis reads again before done, so it is exempt from a re-read.

1 fix round(s): 0 re-read, 0 diff-checked, 1 exempt, 0 not re-read. Exempt: round 1.

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
- convention: the Closing restart gives no single form for Pi wired in the run beside Claude already registered — `framework/bootstrap/ductus-procedure.md` — **waived**: the per-agent forms compose into an accurate notice, as the subject's run shows, so no host reads it wrong
- convention: the Frontmatter Migration's specs/**/spec.md walk reports specs/templates/spec.md as already frontmatter on every run — `framework/bootstrap/ductus-procedure.md` — **waived**: the template has frontmatter, so the walk skips it with a report line and writes nothing
- stale: the Extract section had the temp directory created after the archive fetch, where the Pre-flight Phase creates it before anything is fetched — `specs/015-tarball-fetch/spec.md` — **fixed** (decided by the rule)
- stale: the spec's and the plan's Claude seed edit give Bash(mktemp *), which 022's task 18 narrowed to the exact command Bash(mktemp -d -t ductus-XXXXXX) — `specs/015-tarball-fetch/spec.md` — **fixed** (decided by the rule)
- stale: the plan hardcoded {tempdir}/ductus-main/ as the framework root and named the archive ductus.tar.gz, where the root is the extraction's one top-level directory and the archive is framework.tar.gz — `specs/015-tarball-fetch/plan.md` — **fixed** (decided by the rule)
- stale: the plan cited the project's CLAUDE.md for ductus.md being hand-maintained, a line CLAUDE.md no longer carries — `specs/015-tarball-fetch/plan.md` — **fixed** (decided by the rule)
- stale: the plan's Affected Files said no other file changes, missing task 6's govern.md and ductus-procedure.md, and its resolved-questions summary still counted four after the sweep removed Auggie's — `specs/015-tarball-fetch/plan.md` — **fixed** (decided by the rule)

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
- `specs/017-derive-dont-ask/plan.md` — 017-derive-dont-ask
- `specs/017-derive-dont-ask/scenarios/a-commit-stages-only-its-own-generated-files.md` — 017-derive-dont-ask
- `specs/017-derive-dont-ask/tasks.md` — 017-derive-dont-ask
- `specs/022-deterministic-runtime/data-model.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/plan.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/ask-consolidation.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/cli-config-dir-per-contributor.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/commands-dir-parameterization.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/coverage-residue-cleanup.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/framework-list-dedup.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/govern-bootstrap.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/merge-managed-block-subsection-insertion.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/the-constitutions-registry-validates-its-values.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/the-pi-command-candidate.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/writecode-payload-bundling.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/spec.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/tasks.md` — 022-deterministic-runtime
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
- `specs/046-scenario-open-question-visibility/analysis.md` — 046-scenario-open-question-visibility
- `specs/046-scenario-open-question-visibility/plan.md` — 046-scenario-open-question-visibility
- `specs/046-scenario-open-question-visibility/review.md` — 046-scenario-open-question-visibility
- `specs/046-scenario-open-question-visibility/spec.md` — 046-scenario-open-question-visibility
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
- `specs/054-remove-supersession/plan.md` — 054-remove-supersession
- `specs/054-remove-supersession/spec.md` — 054-remove-supersession
- `specs/054-remove-supersession/tasks.md` — 054-remove-supersession
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
- `specs/009-scenario-targeting/spec.md` — left out of fix round 1
- `specs/067-analyze-and-audit-checks/scenarios/a-scenario-file-is-checked-against-the-scenario-schema.md` — left out of fix round 1
- `specs/067-analyze-and-audit-checks/tasks.md` — left out of fix round 1

## Unexamined governance

*None.*
