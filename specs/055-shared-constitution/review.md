---
spec: 055-shared-constitution
last-run: 2026-11-10T18:58:49Z
reviewed-against: c2400448f5daf86756fbd7b3d507620ef3d20eba
diff-base: 74ade8887ac45c9bc785f5e9026f1b8faedd5b62
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 11
scope: 12
excluded: 79
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 15cf5405bd3d5f0dd859356aa83fb22d893d4c709b5a5e01895234a3f916fa40
  scenarios/a-registered-source-is-named-with-its-description.md: f04ffbef1f2349e9f23d8446c20490833b592ae5edb7dad11ed94bbe9c2166f3
  scenarios/target-reports-each-constitutions-size.md: 7661b1fccbf76a02af4467a2a40ebf845cfc302b6104f84b831ff25238b6bd3c
blocking: false
dispositions:
  fixed: 7
  deferred: 0
  waived: 7
  undispositioned: 0
decisions:
  - key: "other: the module doc keeps 'a dangling @import yields a constitution that is simply not loaded' — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: it recounts the 048 AC10 incident, which was an @import, and 027's migration-chain-reference-integrity scenario keeps the same sentence as its record
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: a BOM, or punctuation such as ( or \" before the `@`, keeps an @path import from being read as a reference — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: "contrived: the template and the bootstrap write each import at the start of a line, and 067's an-at-path-import-is-a-reference records both cases as edge cases"
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: --ref=main adopters get the migration before the @path orphan-check fix, which ships only in the runtime — `framework/migrations/claude-md-import-syntax.md`"
    outcome: waived
    reason: a runtime change reaches adopters only through a ductus-v tag, and the next release is the owner's to cut; the gap closes there
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: a legacy root `@import constitution.md` becomes `@constitution.md`, an orphan no check reports — `framework/migrations/claude-md-import-syntax.md`"
    outcome: waived
    reason: the root path is not a managed root, the line was equally silent before, and `@import constitution.md` never loaded anything
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: a Windows absolute document path already holds backslashes, and the space escaping may misread them — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: resolve-constitutions writes the document path through rel_path, which replaces every backslash with a forward slash, so the block line holds no backslash but the spaces the bootstrap escapes
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: 040's spec-root-rule-stated-once rests a rejected option on the CLAUDE.md premise 017 AC16 states — `specs/040-configurable-specs-dir/scenarios/spec-root-rule-stated-once.md`"
    outcome: deferred
    target: specs/050-constitution/tasks.md
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "security: the migration turns @import lines that loaded nothing into live imports — `framework/migrations/claude-md-import-syntax.md`"
    outcome: waived
    reason: step 6 names every rewritten path for the operator, Claude Code asks before loading an import from outside the working directory, and no loaded security rule fires on a local text rewrite
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: the migration's code-span sentence repeats what 'begins with @import' already implies — `framework/migrations/claude-md-import-syntax.md`"
    outcome: waived
    reason: "harmless: it names the edge case the scenario lists"
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: the adopter runs met pre-existing bootstrap behavior: v0.28.0 cannot be walked at its tag, the Frontmatter Migration's clean-tree precheck refuses after a migration writes under specs/, AGENTS.md placeholders an old release left literal are never repaired, and v0.59.11's dead old-form constitutions region survives — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: observations about the bootstrap's own machinery made while reviewing this change, none caused by it and none blocking it; the dead region is already an inbox item
    decided-at: 2026-10-27T01:38:52Z
    decided-by: andy@stone.dev
  - key: "other: length_on_disk repeats check_artifact_size's open-metadata-len chain — `runtime/src/primitives/resolve_constitutions.rs`"
    outcome: waived
    reason: "A two-file refactor with no behavior change: the repeated chain is three standard-library calls, and lifting it changes check-artifact-size's code for no reader."
    decided-at: 2026-11-10T00:22:24Z
    decided-by: rule
  - key: "bug: duplicate_paths compares raw path strings, so gov and ./gov are not grouped — `runtime/src/schema/constitutions.rs`"
    outcome: waived
    reason: "The contract, the spec's Edge Case and the data model, groups entries naming the same path as written, and gov and ./gov are different values, each measured; the DuplicateConstitutionPath doc comment that said \"checkout\" is fixed in this run."
    decided-at: 2026-11-10T00:22:24Z
    decided-by: rule
  - key: "convention: framework/bootstrap/ductus.md §Project Configuration says the description renders wherever ductus names a source (fix rounds 1 and 3's NOTEs) — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: A comment the installer writes into adopters' config; correcting it is a bootstrap change, which this repository verifies only with a real adopter run, out of proportion to one clause; the data model states which surfaces carry the description.
    decided-at: 2026-11-10T00:22:24Z
    decided-by: rule
  - key: "other: 'truncated to 100 characters' keeps 100 characters and appends an ellipsis (fix round 1's NOTE) — `specs/055-shared-constitution/data-model.md`"
    outcome: waived
    reason: 100 is the characters kept and the ellipsis marks the cut, as the tests assert; no reader is misled about the bound.
    decided-at: 2026-11-10T00:22:24Z
    decided-by: rule
  - key: "other: the result's bound rests on the registry's own size rather than a fixed maximum (fix round 1's NOTE) — `specs/055-shared-constitution/data-model.md`"
    outcome: waived
    reason: "The bound the owner chose (2026-11-09): descriptions capped, the result bounded by the project's own operator-written registry."
    decided-at: 2026-11-10T00:22:24Z
    decided-by: rule
  - key: "convention: docs/shared-constitution.md's authoring guideline 'Anything that loosens a framework rule' leaves out the exception (fix round 1's NOTE) — `docs/shared-constitution.md`"
    outcome: waived
    reason: An authoring guideline for shared sources; the exception is a framework rule's own opt-in, which a shared author meets in that rule's text.
    decided-at: 2026-11-10T00:22:24Z
    decided-by: rule
  - key: "convention: the mutation list's docstring says one line per source, which fits M15 but not M16 (fix round 3's NOTE) — `scripts/mutation/resolve-constitutions-size.py`"
    outcome: waived
    reason: Each mutant's own description states what it guards; fixing the docstring's wording would start a fourth re-read round for no reader's gain.
    decided-at: 2026-11-10T00:22:24Z
    decided-by: rule
  - key: "other: no test pins the exec walker's halt on a rejected `[paths] specs-root`, which Walker::new reaches through derive_analyze_seeds since Host::load does not validate it — `runtime/src/interpreter/mod.rs`"
    outcome: waived
    reason: The walker path is 022's task 12 and outside 055's contract; the halt was driven on a fixture here (exit 1, config-unreadable, naming the file), and a review creates work only for a gating finding.
    decided-at: 2026-11-10T18:58:49Z
    decided-by: rule
  - key: "perf: write-review loads [paths] four times per call, and 022's task 12 made own_record_path and analyze_freshness_of fallible to carry the repeat, so a config edited mid-call can fail the call after review.md is written — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: Each load predates this window, which made them fallible as 022's task 12 requires; passing the loaded root down is a refactor no caller sees, and the post-write error, reachable only by a config edited mid-call, is the one analyze_freshness_of's doc comment states.
    decided-at: 2026-11-10T18:58:49Z
    decided-by: rule
  - key: "other: under ductus exec a malformed shared session default halts as config-unreadable, since Host::load reads it first, so main.rs's session-unreadable branch is reached only by the process's own record — `runtime/src/main.rs`"
    outcome: waived
    reason: The branch is still reached by a malformed own record, the exec test pins the code the default halts with, and either code labels a halt that names the file.
    decided-at: 2026-11-10T18:58:49Z
    decided-by: rule
  - key: "other: resolve_constitutions, check_artifact_size and discover_rule_files probe the config with is_file or exists, so a directory named config.toml reads as absent to them while Paths::load halts with an I/O error — `runtime/src/primitives/resolve_constitutions.rs`"
    outcome: waived
    reason: "Contrived input: every writer and the exec walk reach Paths::load first, which halts naming the file, and unifying the readers changes no behavior a caller sees."
    decided-at: 2026-11-10T18:58:49Z
    decided-by: rule
---

# Review — 055-shared-constitution

## Summary

Window `74ade888..c2400448` (`base-source: reopen`, `review-owed: true`): 12 scope entries, each a file; 79 paths the window changed are excluded as other specs' commits, and none is deleted. Two commits touch the scope: `92120f45`, 022's task 12, which reopened 055 by making a config that does not parse halt both record writers, and `c2400448`, task 18's corrections of fact, which the criteria verification at `ec6c2088` found before the runs. This is the first review over `92120f45`, shared with 022, so its hunks in `runtime/src/main.rs`, `runtime/src/interpreter/mod.rs` and `runtime/src/primitives/mod.rs` were read in depth.

Examined 11 of 12. One independent read-only agent ran the five passes. It read each entry's window hunks with named context (callers, error paths, tests) and checked `data-model.md` whole, every claim against `schema/constitutions.rs` and `resolve_constitutions.rs`. It drove the release binary on fixtures: `ductus exec analyze` over a malformed config and a rejected `specs-root`; `resolve-constitutions` over five entries, measured and unmeasured, and each malformed form; `write-analysis` over four configs; and a malformed `session.toml` repaired through `write-session`. The generated `.claude/commands/ductus/target.md` is not counted: `scripts/gen-claude-commands.sh --check` exits 0. This session showed the four writer tests fail with their behavior removed: with `Paths::load` falling back, the two halt tests fail; with the registry error folded to clean, the two malformed-entry tests fail.

Of the 12 rule files, `CFG-ENV-008` and `QUAL-TEST-001` have a subject in these files, and both hold: each halt names the file and the value, and no changed test passes with its behavior removed.

Observations:

- one contract break, fixed: the data model scoped the recorded run to a config that parses, while a parseable config with a rejected `[paths] specs-root` halts both writers too;
- six chores fixed by the rule: the spec's signpost, the two writer comments, `has_own_record`'s doc, which 022's task 12 made false, and two missing blank lines;
- four waived by the rule: the exec walker's untested halt on a rejected `specs-root`, write-review's four `[paths]` loads, a `session-unreadable` branch only an own record reaches, and three readers' `is_file` probes.

Stored decisions: three of 055's match and still stand, the two on `data-model.md` and one on `docs/shared-constitution.md`; 13 on files outside the scope are retained. Of other records' decisions on in-scope files, only 022's and 063's on write-review's `[paths]` loads was observed again, and it is not carried: its outcome is the retired `discarded`, and its reason, that a malformed config prints one line per reader, describes the fallback 022's task 12 removed. The observation is waived by the rule anew.

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

- other: 'truncated to 100 characters' keeps 100 characters and appends an ellipsis (fix round 1's NOTE) — `specs/055-shared-constitution/data-model.md` — **waived**: 100 is the characters kept and the ellipsis marks the cut, as the tests assert; no reader is misled about the bound. (decided by the rule)
- other: the result's bound rests on the registry's own size rather than a fixed maximum (fix round 1's NOTE) — `specs/055-shared-constitution/data-model.md` — **waived**: The bound the owner chose (2026-11-09): descriptions capped, the result bounded by the project's own operator-written registry. (decided by the rule)
- convention: docs/shared-constitution.md's authoring guideline 'Anything that loosens a framework rule' leaves out the exception (fix round 1's NOTE) — `docs/shared-constitution.md` — **waived**: An authoring guideline for shared sources; the exception is a framework rule's own opt-in, which a shared author meets in that rule's text. (decided by the rule)
- bug: the data model says the record writers record whenever the config parses, but a parseable config whose `[paths] specs-root` is rejected halts both with no record — `specs/055-shared-constitution/data-model.md` (breaks the spec's contract) — **fixed** (decided by the rule)
- convention: the spec's 022 task 12 signpost says only a config that does not parse halts the writers, though a rejected `[paths] specs-root` halts them too — `specs/055-shared-constitution/spec.md` — **fixed** (decided by the rule)
- convention: write-review's governance comment names only a config that does not parse among what halts the run at Paths::load — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)
- convention: write-analysis's registry comment names only a config that does not parse among what halts the run at Paths::load — `runtime/src/primitives/write_analysis.rs` — **fixed** (decided by the rule)
- convention: has_own_record's doc says the own record is the one session file the exec seed holds strict, though since 022's task 12 the shared default is read strictly too — `runtime/src/main.rs` — **fixed** (decided by the rule)
- convention: no blank line separates the new unparseable-config halt test from the next section comment — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)
- convention: no blank line separates the new unparseable-config halt test from the next section comment — `runtime/src/primitives/write_analysis.rs` — **fixed** (decided by the rule)
- other: no test pins the exec walker's halt on a rejected `[paths] specs-root`, which Walker::new reaches through derive_analyze_seeds since Host::load does not validate it — `runtime/src/interpreter/mod.rs` — **waived**: The walker path is 022's task 12 and outside 055's contract; the halt was driven on a fixture here (exit 1, config-unreadable, naming the file), and a review creates work only for a gating finding. (decided by the rule)
- perf: write-review loads [paths] four times per call, and 022's task 12 made own_record_path and analyze_freshness_of fallible to carry the repeat, so a config edited mid-call can fail the call after review.md is written — `runtime/src/primitives/write_review.rs` — **waived**: Each load predates this window, which made them fallible as 022's task 12 requires; passing the loaded root down is a refactor no caller sees, and the post-write error, reachable only by a config edited mid-call, is the one analyze_freshness_of's doc comment states. (decided by the rule)
- other: under ductus exec a malformed shared session default halts as config-unreadable, since Host::load reads it first, so main.rs's session-unreadable branch is reached only by the process's own record — `runtime/src/main.rs` — **waived**: The branch is still reached by a malformed own record, the exec test pins the code the default halts with, and either code labels a halt that names the file. (decided by the rule)
- other: resolve_constitutions, check_artifact_size and discover_rule_files probe the config with is_file or exists, so a directory named config.toml reads as absent to them while Paths::load halts with an I/O error — `runtime/src/primitives/resolve_constitutions.rs` — **waived**: Contrived input: every writer and the exec walk reach Paths::load first, which halts naming the file, and unifying the readers changes no behavior a caller sees. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.ductus/reference/analyze.md` — 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-target.md` — 022-deterministic-runtime, 062-concurrent-session-targets
- `docs/developing-ductus.md` — 079-review-and-analysis-work-converges
- `framework/reference/analyze.md` — 080-deterministic-runtime-efficiency
- `runtime/src/host.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/interpreter/payload.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/append_inbox.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/append_question.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/brief_re_read.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/check_artifact_size.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/check_artifacts.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/check_corpus_links.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/check_orphaned_references.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/check_prescriptions.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/check_review_freshness.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/check_review_gate.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/check_stuck.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/check_unfolded_specs.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/classify_spec_edit.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/compute_review_scope.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/create_feature.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/create_plan_artifacts.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/dashboard.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/decisions.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/derive_boundary.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/derive_dependencies.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/derive_references.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/derive_routing_candidates.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/diff_cross_spec.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/discover_rule_files.rs` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/inbox_standing.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/invalidate_review.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/label_criteria.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/mark_criterion.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/mark_task.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/measure_reopenings.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/merge_permissions.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/merge_status_hook.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/process_decisions.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/process_waivers.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/prune_plan.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/prune_project.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/prune_tasks.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/read_re_read.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/read_spec.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/read_tasks.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/relocate_audit_records.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/remove_inbox_item.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/render_help.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/resolve_feature.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/resolve_references.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/retire_feature.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/rewrite_spec_links.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/rule_sections.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/set_status.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/start_fix_rounds.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/tool_loading.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/primitives/traverse_deps.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/schema/descriptions.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/schema/paths.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/src/schema/primitives.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/session.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/tests/exec_subprocess.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/tests/specify_command.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `runtime/tests/walker.rs` — 022-deterministic-runtime, 062-concurrent-session-targets
- `scripts/mutation/reference-and-rule-reads.py` — 080-deterministic-runtime-efficiency
- `scripts/mutation/scope-attribution.py` — 022-deterministic-runtime, 062-concurrent-session-targets
- `specs/022-deterministic-runtime/scenarios/cli-config-dir-per-contributor.md` — 022-deterministic-runtime, 062-concurrent-session-targets
- `specs/022-deterministic-runtime/spec.md` — 022-deterministic-runtime, 062-concurrent-session-targets
- `specs/022-deterministic-runtime/tasks.md` — 022-deterministic-runtime, 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/plan.md` — 022-deterministic-runtime, 062-concurrent-session-targets
- `specs/067-analyze-and-audit-checks/tasks.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/071-command-reference-split/tasks.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/079-review-and-analysis-work-converges/tasks.md` — 079-review-and-analysis-work-converges
- `specs/080-deterministic-runtime-efficiency/data-model.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/plan.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/research.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/scenarios/rules-load-without-shell-reads.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/tasks.md` — 080-deterministic-runtime-efficiency

## Unexamined governance

*None.*
