---
spec: 060-exec-analyze-assesses-each-loaded-rule
last-run: 2026-11-25T19:51:00Z
reviewed-against: 1b4779d9ba9344a8e1dce8ab758996c5eeaf9591
diff-base: 76d5449ec6bd987b1e195b0479d971b3db4429ac
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 1
scope: 6
unread:
  - docs/developing-ductus.md
  - docs/runtime.md
  - scripts/analyze/judge-brief.md
  - specs/060-exec-analyze-assesses-each-loaded-rule/analysis.md
  - specs/060-exec-analyze-assesses-each-loaded-rule/plan.md
excluded: 703
contract-breaks: 0
skipped-passes: []
fix-rounds: 0
re-reads: 0
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  scenarios/an-assessment-sees-the-loaded-rule-files.md: 7a1341466362f69527aecd8d09472a6d9ba5ec6d9db5e30a0e316814fae06007
  scenarios/an-assessment-sees-the-rule-it-asks-about.md: 03489f5510e4b9cf5f38194f6a0e8ab6ffcff7272359914c991129ba72a9b70f
  scenarios/rule-judging-runs-from-repo-scripts.md: 19aaa6a8d61cbcfb40306a9c46c2e264d1d61d35b1b322bf38686e7abc93cd96
criteria-judged: 10
criteria-met: 10
criteria-verdicts:
  - label: AC1
    digest: e976632c7b44b5561aeb64ff42dd2025a79ebdab99aedd186260f828158e972f
  - label: AC2
    digest: 953beb08463e8a3a00315bf21b53fe683f98c7e72599aab50a83fb9f17db6f90
  - label: AC3
    digest: 5a437ea2a0549a3dc773de63b6424d0dd83e146af7916eee3f48267ff5c6de5b
  - label: AC4
    digest: 307a1543a2c11225c7e58b609669fe1054e95bc7f820e8ad0fd726379f0bc2b3
  - label: AC5
    digest: e6dded43a731b0062d1219c18b1e450f5ff5572519b8f0e144c2efc6909eaf9b
  - label: AC6
    digest: 8c1672c681dbb5630d9c375835911fb2b2acca68d5e786e080fbe4eaa601aae3
  - label: AC7
    digest: fc058e175ec8f126a1aca1778cb2f7cb31b6960443fb2915f6895fed8db0c222
  - label: AC8
    digest: 5ce357472e69b57cd9b23e1774a5af82b9ab4fcd7814ec846a24b5562a367427
  - label: AC9
    digest: fab958d6744fb48228212d0ae1a3e51c10f97ff9dc012425af6c6f5dfbdc8c9b
  - label: AC10
    digest: 7fcbe3cea9e2b047210c0ce0ef8bc3f10966642faab61ba307e8904ec0c19b55
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: "perf: build_assess_spec_quality_request re-reads the spec file for every rule request, up to 167 times per step — `runtime/src/interpreter/payload.rs`"
    outcome: waived
    reason: each read is a small local file the OS caches, beside an LLM round trip per request; reading it once would thread a second value through the walker context for a gain nobody has measured
    decided-at: 2026-09-27T19:20:13Z
    decided-by: andy@stone.dev
  - key: "other: 039's ticked task 5 describes BE-DRAIN-002's propagation delay as counted within the grace period, which 034's review reworded — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: 036's ticked task 2 names the manifest slot it used at the time — `specs/036-quality-cross-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: QUAL-GROUND-001 cedes configuration to CFG rules that do not cover a required key missing from a file or flag — `framework/rules/quality-cross.md`"
    outcome: deferred
    target: specs/017-derive-dont-ask/scenarios/required-config-values-from-any-source.md
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md`"
    outcome: deferred
    target: specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: 025's task 10 named only AC4 among the places that describe skip-and-warn — `specs/025-rule-opt-out/tasks.md`"
    outcome: deferred
    target: specs/025-rule-opt-out/tasks.md
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: 063's task 12 missed AC5, plan.md, README.md, the command sources and the tool description that still describe the fallback — `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md`"
    outcome: deferred
    target: specs/063-oversized-artifacts-warn-with-a-fix/tasks.md
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: 033's task 10 names no acceptance criterion for the chosen mechanism to amend — `specs/033-rule-surface-setting/tasks.md`"
    outcome: deferred
    target: specs/033-rule-surface-setting/tasks.md
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: 019's task 1 did not cover a host reading config by hand on the markdown-only path — `specs/019-config-decisions/tasks.md`"
    outcome: deferred
    target: specs/019-config-decisions/tasks.md
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: FE-SEMHTML-001 and FE-ARIA-001 both forbid a div with role=button — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: consistent overlap of two MUSTs with no contradiction; consolidating them changes rule ownership, which 069's clarify owns
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: FE-KBD-002 and FE-CONTRAST-001 both require 3:1 for focus indicators — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: the owner decided FE-KBD-002's MUST carries the 3:1 ratio (069 Resolved Questions, 2026-10-19); the overlap is consistent
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: accessibility-frontend.md's first header paragraph is long — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: style only; the content is accurate, and 069's first open question owns what the header claims about conformance
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: FE-STORAGE-006 confines its Cache API MUST to service-worker code — `framework/rules/security-frontend.md`"
    outcome: waived
    reason: the rule is accurate as scoped; widening a MUST to page scripts is a new requirement for the owner, not an accuracy defect
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: BE-REPR-001's Verification names specs/system.md for the API-wide representation — `framework/rules/api-backend.md`"
    outcome: waived
    reason: the Statement's one representation across the API needs a declared one to be checkable; the Verification names where it is declared
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: BE-SCHEMA-001's SDL-versus-introspection split and BE-ERRENV-001's overlap with BE-ERR-002 — `framework/rules/api-backend.md`"
    outcome: waived
    reason: both splits are recorded as intended in 068; low
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: BE-AUTHN-010's factor list reads as closed — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the sentence carries no obligation keyword and the Verification enforces MFA presence, the SMS-sole ban and no skip path, consistent with BE-AUTHN-021; whether push is acceptable is a substantive owner change
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: security-backend.md's header lists its categories in a different order from its sections — `framework/rules/security-backend.md`"
    outcome: waived
    reason: the category set matches; no artifact specifies the order, and the header matches 008's data-model table order
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: BE-COORD-001's Verification asks a plan to say whether a lock guards correctness or efficiency — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: the Statement's MAY turns on that classification; the Verification asks for exactly the fact it needs
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: concurrency, observability, reliability, quality and configuration section titles are sentence case — `framework/rules/concurrency-backend.md`"
    outcome: waived
    reason: 017's and 036's data models record their files' category names in sentence case (017 task 38, an owner decision); 008's title-case bullet describes the security files
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: ticked task-1 items in 037, 038 and 039 say one RFC 2119 keyword, and 039's task 4 names an alert — `specs/039-reliability-backend-rules/tasks.md`"
    outcome: waived
    reason: "spent task history: each records what the task said when it was done, and rewriting it records a fiction"
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: 008's first resolved question reasons from CI gating — `specs/008-security-rules/spec.md`"
    outcome: waived
    reason: a resolved question records the reasoning at the time; rewriting it records a fiction
    decided-at: 2026-10-20T01:01:34Z
    decided-by: andy@stone.dev
  - key: "other: fix-round 3 re-read: tree_state ignores git's exit status, so a failing git diff digests as empty, and text mode folds CRLF, so a line-ending-only edit is invisible — `scripts/analyze/analyze_lib.py`"
    outcome: waived
    reason: git diff HEAD fails only with no HEAD commit, which head() refuses before the digest is compared, and the feature, rule files and constitution, the files detection judges by content, are digested byte for byte; the re-read judged both cases contrived
    decided-at: 2026-11-18T17:31:07Z
    decided-by: rule
  - key: "other: fix-round re-read: _settle's DUCTUS_BIN check runs for every script, so briefs.py, which calls no primitive, and a usage error also fail at startup with no runtime built — `scripts/analyze/analyze_lib.py`"
    outcome: waived
    reason: briefs.py runs only after split.py, which needs the runtime, so the check fails no run that could otherwise succeed, and its message names the build command
    decided-at: 2026-11-18T18:45:59Z
    decided-by: rule
  - key: "other: `scripts/analyze/judge-brief.md` says `check.py` matches the share's key \"whole\" and `docs/developing-ductus.md` says it refuses a read account keyed by anything but the exact path, but `read_account` in `scripts/analyze/check.py` also accepts the basename of an absolute share path (its docstring says so), so a basename key passes. The instruction to key by the exact path is safe; only the stated reason is overstated. — `scripts/analyze/judge-brief.md`"
    outcome: waived
    reason: the instruction to key the share by its exact path is correct and passes `check.py`; only its stated reason overstates, by omitting a basename fallback no judge following the instruction reaches, and the observation is about this repository's own analysis tooling, which `AGENTS.md` waives unless the fix is mechanical, and rewording the reason is not
    decided-at: 2026-11-25T19:51:00Z
    decided-by: rule
---

# Review — 060-exec-analyze-assesses-each-loaded-rule

## Summary

Review of 060's code changed since its last review, from `76d5449e` (`last-review`) to `1b4779d9`, for 084's task 19. The five passes read the spec's own change, the hunks of five commits (`c2d2f800`, `d77a904d`, `a31efae8`, `b6a6ac87`, `e48e4b4f`) in `docs/developing-ductus.md` and `scripts/analyze/judge-brief.md`, against the five loaded rule files, each read whole (231,962 bytes): no finding and one observation. They read `scripts/analyze/check.py` whole, and `record.py`, `split.py` and `briefs.py` in part, to check the new bullets' claims about the analysis scripts. The observation, that `judge-brief.md` and an older `docs/developing-ductus.md` bullet say `check.py` takes only the share's exact path while `read_account` also accepts an absolute share path's basename, is waived by the rule: the instruction to key by the exact path is correct, only its reason overstates, and it concerns this repository's own analysis tooling. The new bullets' incident counts and dates are session history, which no source at hand could re-check. The criteria step judged all 10 criteria met against the tree, reading `spec.md` whole and the interpreter, tally, payload and rule-section code each criterion names, with the tests that prove them. Stored decisions: 23 retained on files this scope does not hold; none names a file the passes read. Examined 1 of 6: `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md`, read whole by the criteria step. Unread, by the operator's decision to read each spec's own change (084's `tasks.md`, task 19): `docs/developing-ductus.md`, read at the slice's hunks and their neighbouring bullets; `scripts/analyze/judge-brief.md`, read to its 60th of 61 lines; and `docs/runtime.md`, 060's `analysis.md` and `plan.md`, not opened by the passes.

0 fix round(s): 0 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

10 of 763 commits in the window no spec claimed.

10 of 10 acceptance criteria judged met.

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

- AC1 — met: `Walker::handle_assessments` in `runtime/src/interpreter/mod.rs` filters `LoadedRules::assessable` by each rule's own tier (`RuleSection::tier` in `runtime/src/primitives/rule_sections.rs`); proven by `an_exec_analyze_asks_about_each_loaded_rule_once_in_its_own_tier` (passed).
- AC2 — met: `AnalyzeTally::record_assessment` in `runtime/src/interpreter/analyze_tally.rs` runs once per request and each rule is asked at one step only; proven by `an_exec_analyze_asks_about_each_loaded_rule_once_in_its_own_tier` and `analyze_completes_on_the_session_write_session_writes` (blocking-findings 1, advisory 0), both passed.
- AC3 — met: `load_rules` counts rules with no Verification or no keyword and `record_rule_set` records them under `rule-assessments-not-checked` and unreadable files under `rule-file-unreadable` (both in the could-not-be-read class of `framework/reference/analyze.md`); proven by `a_rule_set_records_its_unreadable_files_and_unassessable_rules` (passed).
- AC4 — met: `runtime/tests/golden/analyze-basic.jsonl` holds one `llm-request` for `CFG-CONST-001`; `analyze_basic_stream_matches_golden` and `analyze_completes_on_the_session_write_session_writes` pass, and `an_exec_analyze_asks_about_each_loaded_rule_once_in_its_own_tier` shows 3 rules asked and 2 unexamined.
- AC5 — met: `handle_assessments` calls `exchange` once per asked rule with `ASSESSED_RULE_KEY` and `LOADED_RULE_FILES_KEY` seeded and evaluates no trigger, `build_assess_spec_quality_request` in `runtime/src/interpreter/payload.rs` carries them, and 082's `split_reused` skips only current verdicts; proven by `an_exec_analyze_asks_about_each_loaded_rule_once_in_its_own_tier` (asserts `rule` and `loaded-rule-files`).
- AC6 — met: `RuleSection::tier` in `runtime/src/primitives/rule_sections.rs` returns Must on a MUST token (so MUST NOT), Should on SHOULD only, else None, which `load_rules` counts unassessable; proven by `a_statement_s_keyword_gives_its_tier`, `a_statement_carrying_both_keywords_is_must_tier`, `a_statement_with_no_keyword_has_no_tier` and `every_shipped_rule_is_assessable` (all passed).
- AC7 — met: `load_rules` counts a rule file `read_repo_file` cannot read in `unreadable_files` and `record_step_asking_nothing` records one `rule-assessments-not-checked` per step when no rule is loaded; proven by `analyze_with_no_rule_directory_records_its_citations_unexamined` (`rule-assessments-not-checked: 2`), `with_no_rule_loaded_each_step_asks_nothing_and_records_itself` and the `rule-file-unreadable: 1` assertion in `an_exec_analyze_asks_about_each_loaded_rule_once_in_its_own_tier` (all passed).
- AC8 — met: `framework/commands/analyze.md` steps 11 and 12 state the walker evaluates no trigger and the host answers `passed: true`, and the `assessSpecQuality` section of `specs/022-deterministic-runtime/data-model.md` says the same (`The walker evaluates no Verification trigger`); read, no test applies.
- AC9 — met: `specs/066-audit-records-and-the-completion-gate/data-model.md` defines `rule-assessments-not-checked` as counting the loaded rules steps 11 and 12 did not ask about, and `specs/022-deterministic-runtime/spec.md` carries a Signpost linking back to 060; read, no test applies.
- AC10 — met: `framework/commands/analyze.md` step 11 (shared by step 12) and `framework/reference/analyze.md` `## Rule judging in a subagent` state the fresh-subagent hand-off, that an unanswered or short-read rule is left out of `judged-rules` and recorded under `unjudged-rules`, and that a host without a subagent judges itself while `ductus exec` asks per rule; read, no test applies.

## Observations

- other: `scripts/analyze/judge-brief.md` says `check.py` matches the share's key "whole" and `docs/developing-ductus.md` says it refuses a read account keyed by anything but the exact path, but `read_account` in `scripts/analyze/check.py` also accepts the basename of an absolute share path (its docstring says so), so a basename key passes. The instruction to key by the exact path is safe; only the stated reason is overstated. — `scripts/analyze/judge-brief.md` — **waived**: the instruction to key the share by its exact path is correct and passes `check.py`; only its stated reason overstates, by omitting a basename fallback no judge following the instruction reaches, and the observation is about this repository's own analysis tooling, which `AGENTS.md` waives unless the fix is mechanical, and rewording the reason is not (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/amend.md`
- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/clarify.md`
- `.claude/commands/ductus/configure.md`
- `.claude/commands/ductus/consolidate.md`
- `.claude/commands/ductus/fold.md`
- `.claude/commands/ductus/groom.md`
- `.claude/commands/ductus/help.md`
- `.claude/commands/ductus/implement.md`
- `.claude/commands/ductus/link.md`
- `.claude/commands/ductus/plan.md`
- `.claude/commands/ductus/prune.md`
- `.claude/commands/ductus/review.md`
- `.claude/commands/ductus/specify.md`
- `.claude/commands/ductus/status.md`
- `.claude/commands/ductus/target.md`
- `.ductus/config.toml`
- `.ductus/reference/analyze.md`
- `.ductus/reference/audit.md`
- `.ductus/reference/clarify.md`
- `.ductus/reference/consolidate.md`
- `.ductus/reference/fold.md`
- `.ductus/reference/groom.md`
- `.ductus/reference/implement.md`
- `.ductus/reference/plan.md`
- `.ductus/reference/prune.md`
- `.ductus/reference/review.md`
- `.ductus/reference/specify.md`
- `.ductus/reference/status.md`
- `.gitattributes`
- `.githooks/pre-commit`
- `.githooks/pre-push`
- `.github/workflows/framework-checks.yml`
- `.github/workflows/runtime-acquisition.yml`
- `.github/workflows/runtime-currency.yml`
- `.github/workflows/runtime-release.yml`
- `.github/workflows/runtime.yml`
- `.pi/extensions/ductus.ts`
- `.pi/prompts/ductus-amend.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-clarify.md`
- `.pi/prompts/ductus-consolidate.md`
- `.pi/prompts/ductus-fold.md`
- `.pi/prompts/ductus-groom.md`
- `.pi/prompts/ductus-help.md`
- `.pi/prompts/ductus-implement.md`
- `.pi/prompts/ductus-link.md`
- `.pi/prompts/ductus-plan.md`
- `.pi/prompts/ductus-prune.md`
- `.pi/prompts/ductus-review.md`
- `.pi/prompts/ductus-specify.md`
- `.pi/prompts/ductus-status.md`
- `.pi/prompts/ductus-target.md`
- `AGENTS.md`
- `README.md`
- `docs/analyze.md`
- `docs/rules.md`
- `docs/shared-constitution.md`
- `docs/slash-commands.md`
- `framework/bootstrap/configure/claude.md`
- `framework/bootstrap/ductus-procedure.md`
- `framework/bootstrap/ductus.md`
- `framework/bootstrap/govern.md`
- `framework/bootstrap/hooks/ductus-pre-commit`
- `framework/bootstrap/hooks/ductus-pre-push`
- `framework/bootstrap/hooks/pre-push`
- `framework/bootstrap/pi/ductus-bridge.ts`
- `framework/commands/amend.md`
- `framework/commands/analyze.md`
- `framework/commands/clarify.md`
- `framework/commands/consolidate.md`
- `framework/commands/fold.md`
- `framework/commands/groom.md`
- `framework/commands/help.md`
- `framework/commands/implement.md`
- `framework/commands/link.md`
- `framework/commands/plan.md`
- `framework/commands/prune.md`
- `framework/commands/review.md`
- `framework/commands/specify.md`
- `framework/commands/status.md`
- `framework/commands/target.md`
- `framework/constitution.md`
- `framework/migrations.toml`
- `framework/migrations/audit-record-relocate.md`
- `framework/migrations/criterion-label-backfill.md`
- `framework/migrations/project-review-record.md`
- `framework/migrations/remove-legacy-constitutions-region.md`
- `framework/reference/analyze.md`
- `framework/reference/audit.md`
- `framework/reference/clarify.md`
- `framework/reference/consolidate.md`
- `framework/reference/fold.md`
- `framework/reference/groom.md`
- `framework/reference/implement.md`
- `framework/reference/plan.md`
- `framework/reference/prune.md`
- `framework/reference/review.md`
- `framework/reference/specify.md`
- `framework/reference/status.md`
- `framework/rules/accessibility-frontend.md`
- `framework/rules/api-backend.md`
- `framework/rules/concurrency-backend.md`
- `framework/rules/configuration-cross.md`
- `framework/rules/reliability-backend.md`
- `framework/rules/security-backend.md`
- `framework/rules/security-cross.md`
- `framework/rules/security-frontend.md`
- `framework/runtime-tools.txt`
- `framework/templates/ci/adopter-generators.yml`
- `framework/templates/spec/spec.md`
- `install.sh`
- `runtime/CHANGELOG.md`
- `runtime/Cargo.lock`
- `runtime/Cargo.toml`
- `runtime/README.md`
- `runtime/src/host.rs`
- `runtime/src/interpreter/analyze_tally.rs`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/interpreter/payload.rs`
- `runtime/src/io.rs`
- `runtime/src/main.rs`
- `runtime/src/mcp/flag_help.rs`
- `runtime/src/mcp/mod.rs`
- `runtime/src/mcp/server.rs`
- `runtime/src/parser/mod.rs`
- `runtime/src/primitives/analyze_subjects.rs`
- `runtime/src/primitives/append_inbox.rs`
- `runtime/src/primitives/append_question.rs`
- `runtime/src/primitives/apply_manifest.rs`
- `runtime/src/primitives/bounded_lists.rs`
- `runtime/src/primitives/brief_re_read.rs`
- `runtime/src/primitives/check_artifact_size.rs`
- `runtime/src/primitives/check_artifacts.rs`
- `runtime/src/primitives/check_command_flags.rs`
- `runtime/src/primitives/check_corpus_links.rs`
- `runtime/src/primitives/check_departures.rs`
- `runtime/src/primitives/check_orphaned_references.rs`
- `runtime/src/primitives/check_owed_work.rs`
- `runtime/src/primitives/check_prescriptions.rs`
- `runtime/src/primitives/check_promotion_coverage.rs`
- `runtime/src/primitives/check_review_gate.rs`
- `runtime/src/primitives/check_step_references.rs`
- `runtime/src/primitives/check_stuck.rs`
- `runtime/src/primitives/check_unreviewed_code.rs`
- `runtime/src/primitives/commit_attribution.rs`
- `runtime/src/primitives/compute_review_scope.rs`
- `runtime/src/primitives/criterion_verdicts.rs`
- `runtime/src/primitives/dashboard.rs`
- `runtime/src/primitives/decisions.rs`
- `runtime/src/primitives/departures.rs`
- `runtime/src/primitives/derive_references.rs`
- `runtime/src/primitives/derive_routing_candidates.rs`
- `runtime/src/primitives/discover_rule_files.rs`
- `runtime/src/primitives/fix_rounds.rs`
- `runtime/src/primitives/guarded.rs`
- `runtime/src/primitives/host_config.rs`
- `runtime/src/primitives/invalidate_review.rs`
- `runtime/src/primitives/label_criteria.rs`
- `runtime/src/primitives/line_citations.rs`
- `runtime/src/primitives/lint_markdown.rs`
- `runtime/src/primitives/mark_criterion.rs`
- `runtime/src/primitives/mark_task.rs`
- `runtime/src/primitives/measure_reopenings.rs`
- `runtime/src/primitives/mechanical_sweep.rs`
- `runtime/src/primitives/merge_managed_block.rs`
- `runtime/src/primitives/merge_permissions.rs`
- `runtime/src/primitives/merge_status_hook.rs`
- `runtime/src/primitives/mod.rs`
- `runtime/src/primitives/owed_work.rs`
- `runtime/src/primitives/pipeline_view.rs`
- `runtime/src/primitives/process_decisions.rs`
- `runtime/src/primitives/process_waivers.rs`
- `runtime/src/primitives/prune_plan.rs`
- `runtime/src/primitives/prune_project.rs`
- `runtime/src/primitives/prune_tasks.rs`
- `runtime/src/primitives/read_re_read.rs`
- `runtime/src/primitives/read_spec.rs`
- `runtime/src/primitives/read_tasks.rs`
- `runtime/src/primitives/relocate_audit_records.rs`
- `runtime/src/primitives/remove_inbox_item.rs`
- `runtime/src/primitives/resolve_anchor.rs`
- `runtime/src/primitives/resolve_constitutions.rs`
- `runtime/src/primitives/review_subject.rs`
- `runtime/src/primitives/rewrite_spec_links.rs`
- `runtime/src/primitives/rule_sections.rs`
- `runtime/src/primitives/rule_verdicts.rs`
- `runtime/src/primitives/set_status.rs`
- `runtime/src/primitives/shipped_rule_ids.txt`
- `runtime/src/primitives/spec_links.rs`
- `runtime/src/primitives/start_fix_rounds.rs`
- `runtime/src/primitives/tool_loading.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/primitives/write_review.rs`
- `runtime/src/primitives/write_review/departure_waivers.rs`
- `runtime/src/primitives/write_review/moved_scope.rs`
- `runtime/src/primitives/write_review/project_record.rs`
- `runtime/src/primitives/write_review/seed.rs`
- `runtime/src/schema/config_keys.rs`
- `runtime/src/schema/descriptions.rs`
- `runtime/src/schema/extensions.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/src/schema/protocol.rs`
- `runtime/src/schema/registry.rs`
- `runtime/src/schema/severity.rs`
- `runtime/src/session.rs`
- `runtime/tests/bounded_list_gates.rs`
- `runtime/tests/check_step_references_cli.rs`
- `runtime/tests/check_unreviewed_code_cli.rs`
- `runtime/tests/crlf_preservation.rs`
- `runtime/tests/derive_dependencies_golden.rs`
- `runtime/tests/derive_references_golden.rs`
- `runtime/tests/exec_stderr_gone.rs`
- `runtime/tests/exec_subprocess.rs`
- `runtime/tests/fix_rounds.rs`
- `runtime/tests/fixtures/ductus-basic/framework/bootstrap/install.md`
- `runtime/tests/fixtures/implement-basic/.govern.session.toml`
- `runtime/tests/fixtures/implement-basic/stdin.jsonl`
- `runtime/tests/fixtures/review-basic/stdin.jsonl`
- `runtime/tests/golden/implement-basic.jsonl`
- `runtime/tests/golden/plan-basic.jsonl`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/golden/specify-basic.jsonl`
- `runtime/tests/line_ending_discipline.rs`
- `runtime/tests/mcp.rs`
- `runtime/tests/measure_reopenings.rs`
- `runtime/tests/mechanical_sweep_parity.rs`
- `runtime/tests/merge_status_hook.rs`
- `runtime/tests/parity.rs`
- `runtime/tests/result_sizes.rs`
- `runtime/tests/review_command.rs`
- `runtime/tests/walker.rs`
- `scripts/analyze/briefs.py`
- `scripts/analyze/check.py`
- `scripts/analyze/detect.py`
- `scripts/analyze/record.py`
- `scripts/analyze/split.py`
- `scripts/analyze/step15-brief.md`
- `scripts/audit/README.md`
- `scripts/audit/adopter-shell-behavior.sh`
- `scripts/audit/archive-contents.sh`
- `scripts/audit/cross-doc-consistency.sh`
- `scripts/audit/done-spec-criteria.sh`
- `scripts/audit/migration-coverage.sh`
- `scripts/audit/permission-wildcard-position.sh`
- `scripts/audit/promotion-coverage.sh`
- `scripts/audit/rename-sweep-residue.sh`
- `scripts/audit/run-all.sh`
- `scripts/audit/ssot-invariants.sh`
- `scripts/audit/step-reference-integrity.sh`
- `scripts/audit/sweep-target-manifest-parity.sh`
- `scripts/check-release-tag.sh`
- `scripts/measure-replace-under-server.sh`
- `scripts/mutation/analysis-design-fix-067.py`
- `scripts/mutation/command-reference-split.py`
- `scripts/mutation/commit-counts.py`
- `scripts/mutation/compare-and-set-066.py`
- `scripts/mutation/criteria-in-review-066.py`
- `scripts/mutation/criterion-ticks-outside-the-digest.py`
- `scripts/mutation/departure-waivers.py`
- `scripts/mutation/departures.py`
- `scripts/mutation/exec-arguments.py`
- `scripts/mutation/exec-scenario-anchors-067.py`
- `scripts/mutation/fix-round-record.py`
- `scripts/mutation/freshness-results-have-a-bounded-size.py`
- `scripts/mutation/installed-file-modes.py`
- `scripts/mutation/json-flag-help-022.py`
- `scripts/mutation/line-citation-strip-066.py`
- `scripts/mutation/loop-inputs-screened.py`
- `scripts/mutation/moved-scope-066.py`
- `scripts/mutation/never-owed-084.py`
- `scripts/mutation/owed-work-084.py`
- `scripts/mutation/owner-reviews-067.py`
- `scripts/mutation/plan-edit-takes-no-back-edge.py`
- `scripts/mutation/process-decisions-results-bounded.py`
- `scripts/mutation/project-record-084.py`
- `scripts/mutation/prune-open-region.py`
- `scripts/mutation/prune-project.py`
- `scripts/mutation/prune-review-fixes-041.py`
- `scripts/mutation/quoted-heading-forms-067.py`
- `scripts/mutation/reachable-trigger-bar-078.py`
- `scripts/mutation/record-path-quoting-066.py`
- `scripts/mutation/relocate-merge-children-057.py`
- `scripts/mutation/reopen-measure.py`
- `scripts/mutation/resolve-constitutions-size.py`
- `scripts/mutation/review-022-fixes-2026-11-19.py`
- `scripts/mutation/review-022-task-41.py`
- `scripts/mutation/review-022-task-42.py`
- `scripts/mutation/review-022-task-44.py`
- `scripts/mutation/review-022-task-46.py`
- `scripts/mutation/review-departures-bounded.py`
- `scripts/mutation/review-read-credit.py`
- `scripts/mutation/reviews-follow-what-changed.py`
- `scripts/mutation/rewrite-spec-links.py`
- `scripts/mutation/routing-path-claims.py`
- `scripts/mutation/rule-files-left-to-analysis-020.py`
- `scripts/mutation/rule-verdicts-converge.py`
- `scripts/mutation/run.py`
- `scripts/mutation/scope-attribution.py`
- `scripts/mutation/site-anchors-020.py`
- `scripts/mutation/status-count-084.py`
- `scripts/mutation/stderr-warnings-022.py`
- `scripts/mutation/step-references-bounded.py`
- `scripts/mutation/step-references-reach.py`
- `scripts/mutation/suffixed-task-number.py`
- `scripts/mutation/unreviewed-code-084.py`
- `scripts/mutation/waiver-notices-flatten.py`
- `scripts/mutation/waivers-bounded.py`
- `scripts/tests/pi-bridge-harness.mjs`
- `scripts/tests/test-analyze-scripts.py`
- `scripts/tests/test-archive-contents.sh`
- `scripts/tests/test-audit-path-containment.sh`
- `scripts/tests/test-check-release-tag.sh`
- `scripts/tests/test-done-spec-owed-work.sh`
- `scripts/tests/test-migration-coverage.sh`
- `scripts/tests/test-pre-commit-staging.sh`
- `scripts/tests/test-run-workflow-steps.sh`
- `scripts/tests/test-step-reference-integrity.sh`
- `specs/000-slash-commands/analysis.md`
- `specs/000-slash-commands/plan.md`
- `specs/000-slash-commands/review.md`
- `specs/000-slash-commands/scenarios/criterion-route-after-draft.md`
- `specs/000-slash-commands/scenarios/scenario-without-task-visibility.md`
- `specs/000-slash-commands/scenarios/validation-gates.md`
- `specs/000-slash-commands/spec.md`
- `specs/003-bootstrap-automation/analysis.md`
- `specs/003-bootstrap-automation/plan.md`
- `specs/003-bootstrap-automation/review.md`
- `specs/003-bootstrap-automation/spec.md`
- `specs/003-bootstrap-automation/tasks.md`
- `specs/007-govern-workflow/analysis.md`
- `specs/007-govern-workflow/plan.md`
- `specs/007-govern-workflow/review.md`
- `specs/007-govern-workflow/scenarios/ductus-self-update-precheck.md`
- `specs/007-govern-workflow/spec.md`
- `specs/007-govern-workflow/tasks.md`
- `specs/008-security-rules/analysis.md`
- `specs/008-security-rules/plan.md`
- `specs/008-security-rules/review.md`
- `specs/008-security-rules/scenarios/a-cache-partition-is-purged-when-the-session-ends.md`
- `specs/008-security-rules/scenarios/a-statement-carries-one-obligation-keyword.md`
- `specs/008-security-rules/scenarios/a-throwaway-credential-owes-no-revocation-path.md`
- `specs/008-security-rules/scenarios/an-absent-origin-on-a-cookie-connection-is-flagged.md`
- `specs/008-security-rules/scenarios/be-deps-005-flags-what-its-statement-requires.md`
- `specs/008-security-rules/scenarios/rules-are-as-short-as-their-required-content.md`
- `specs/008-security-rules/scenarios/security-rules-flag-what-their-sources-support.md`
- `specs/008-security-rules/scenarios/the-tag-check-reads-every-entry-above-the-newest-version.md`
- `specs/008-security-rules/spec.md`
- `specs/008-security-rules/tasks.md`
- `specs/010-agent-autonomy/plan.md`
- `specs/011-brownfield-process/spec.md`
- `specs/012-multi-agent-govern/analysis.md`
- `specs/012-multi-agent-govern/plan.md`
- `specs/012-multi-agent-govern/review.md`
- `specs/012-multi-agent-govern/spec.md`
- `specs/012-multi-agent-govern/tasks.md`
- `specs/013-text-first-artifacts/analysis.md`
- `specs/013-text-first-artifacts/plan.md`
- `specs/013-text-first-artifacts/scenarios/criterion-identifiers.md`
- `specs/013-text-first-artifacts/spec.md`
- `specs/014-reclarify-backedge/plan.md`
- `specs/014-reclarify-backedge/scenarios/reopen-after-informal-edits.md`
- `specs/014-reclarify-backedge/spec.md`
- `specs/015-tarball-fetch/analysis.md`
- `specs/015-tarball-fetch/plan.md`
- `specs/015-tarball-fetch/review.md`
- `specs/015-tarball-fetch/spec.md`
- `specs/015-tarball-fetch/tasks.md`
- `specs/016-cross-cutting-rules/spec.md`
- `specs/017-derive-dont-ask/analysis.md`
- `specs/017-derive-dont-ask/data-model.md`
- `specs/017-derive-dont-ask/plan.md`
- `specs/017-derive-dont-ask/review.md`
- `specs/017-derive-dont-ask/scenarios/a-path-list-commit-leaves-the-shared-index-current.md`
- `specs/017-derive-dont-ask/scenarios/a-throwaway-credential-is-not-a-secret.md`
- `specs/017-derive-dont-ask/scenarios/generator-sync-claim-honesty.md`
- `specs/017-derive-dont-ask/spec.md`
- `specs/017-derive-dont-ask/tasks.md`
- `specs/018-adopter-owned-pre-commit/analysis.md`
- `specs/018-adopter-owned-pre-commit/plan.md`
- `specs/018-adopter-owned-pre-commit/review.md`
- `specs/018-adopter-owned-pre-commit/spec.md`
- `specs/020-code-review/analysis.md`
- `specs/020-code-review/data-model.md`
- `specs/020-code-review/plan.md`
- `specs/020-code-review/review.md`
- `specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md`
- `specs/020-code-review/scenarios/waiver-expiry.md`
- `specs/020-code-review/scenarios/waiver-file-lists.md`
- `specs/020-code-review/spec.md`
- `specs/020-code-review/tasks.md`
- `specs/022-deterministic-runtime/analysis.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/plan.md`
- `specs/022-deterministic-runtime/review.md`
- `specs/022-deterministic-runtime/scenarios/a-closed-stdout-is-a-quiet-exit.md`
- `specs/022-deterministic-runtime/scenarios/append-inbox-comment-aware-write.md`
- `specs/022-deterministic-runtime/scenarios/ask-consolidation.md`
- `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md`
- `specs/022-deterministic-runtime/scenarios/dashboard-primitive.md`
- `specs/022-deterministic-runtime/scenarios/done-when-authoring-forms.md`
- `specs/022-deterministic-runtime/scenarios/every-mcp-argument-has-a-cli-form.md`
- `specs/022-deterministic-runtime/scenarios/govern-bootstrap.md`
- `specs/022-deterministic-runtime/scenarios/groom-command-acceleration.md`
- `specs/022-deterministic-runtime/scenarios/host-protocol-conformance.md`
- `specs/022-deterministic-runtime/scenarios/implement-completion-gate.md`
- `specs/022-deterministic-runtime/scenarios/installed-files-are-not-group-writable.md`
- `specs/022-deterministic-runtime/scenarios/lint-markdown-tool-resolution.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/scenarios/routing-ranks-by-claimed-path.md`
- `specs/022-deterministic-runtime/scenarios/scenario-open-question-signal.md`
- `specs/022-deterministic-runtime/scenarios/severity-is-a-closed-set-not-a-string.md`
- `specs/022-deterministic-runtime/scenarios/skipscanner-inline-code-exemption.md`
- `specs/022-deterministic-runtime/scenarios/spec-side-parser-hardening.md`
- `specs/022-deterministic-runtime/scenarios/specify-routes-before-scaffolding.md`
- `specs/022-deterministic-runtime/scenarios/the-pi-command-candidate.md`
- `specs/022-deterministic-runtime/scenarios/unchecked-done-when-clause-tally.md`
- `specs/022-deterministic-runtime/scenarios/writecode-payload-bundling.md`
- `specs/022-deterministic-runtime/scenarios/writecode-payload-canonicalize-paths.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/022-deterministic-runtime/tasks.md`
- `specs/023-govern-refinement/analysis.md`
- `specs/023-govern-refinement/plan.md`
- `specs/023-govern-refinement/review.md`
- `specs/023-govern-refinement/scenarios/configure-inert-write-path-entries.md`
- `specs/023-govern-refinement/scenarios/configure-permission-pattern-safety.md`
- `specs/023-govern-refinement/scenarios/living-specs.md`
- `specs/023-govern-refinement/spec.md`
- `specs/023-govern-refinement/tasks.md`
- `specs/024-rule-loader/analysis.md`
- `specs/024-rule-loader/plan.md`
- `specs/024-rule-loader/review.md`
- `specs/024-rule-loader/spec.md`
- `specs/025-rule-opt-out/review.md`
- `specs/025-rule-opt-out/spec.md`
- `specs/026-framework-self-audit/analysis.md`
- `specs/026-framework-self-audit/plan.md`
- `specs/026-framework-self-audit/review.md`
- `specs/026-framework-self-audit/scenarios/families-follow-the-command-reference-split.md`
- `specs/026-framework-self-audit/scenarios/family-10-migration-coverage.md`
- `specs/026-framework-self-audit/scenarios/family-23-sweep-target-manifest-parity.md`
- `specs/026-framework-self-audit/scenarios/family-34-reference-section-reachability.md`
- `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md`
- `specs/026-framework-self-audit/scenarios/family-36-self-url-resolution.md`
- `specs/026-framework-self-audit/scenarios/family-40-deprecated-rules-are-removed-on-time.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/026-framework-self-audit/tasks.md`
- `specs/027-bootstrap-migration-registry/analysis.md`
- `specs/027-bootstrap-migration-registry/plan.md`
- `specs/027-bootstrap-migration-registry/review.md`
- `specs/027-bootstrap-migration-registry/scenarios/a-first-adoption-runs-no-migration.md`
- `specs/027-bootstrap-migration-registry/scenarios/a-projects-own-constitution-is-not-relocated.md`
- `specs/027-bootstrap-migration-registry/scenarios/migration-chain-reference-integrity.md`
- `specs/027-bootstrap-migration-registry/scenarios/migrations-apply-once-per-repo.md`
- `specs/027-bootstrap-migration-registry/scenarios/seed-files-keep-no-literal-placeholder.md`
- `specs/027-bootstrap-migration-registry/spec.md`
- `specs/027-bootstrap-migration-registry/tasks.md`
- `specs/029-bootstrap-runtime-autowire/analysis.md`
- `specs/029-bootstrap-runtime-autowire/plan.md`
- `specs/029-bootstrap-runtime-autowire/review.md`
- `specs/029-bootstrap-runtime-autowire/scenarios/state-a-deterministic-path-forcing.md`
- `specs/029-bootstrap-runtime-autowire/spec.md`
- `specs/029-bootstrap-runtime-autowire/tasks.md`
- `specs/030-cross-service-references/plan.md`
- `specs/030-cross-service-references/spec.md`
- `specs/034-performance-backend-rules/analysis.md`
- `specs/034-performance-backend-rules/review.md`
- `specs/034-performance-backend-rules/spec.md`
- `specs/035-groom-session-target/scenarios/confirmation-names-reopen.md`
- `specs/035-groom-session-target/scenarios/reopen-done-spec-on-scenario.md`
- `specs/037-observability-backend-rules/analysis.md`
- `specs/037-observability-backend-rules/plan.md`
- `specs/037-observability-backend-rules/review.md`
- `specs/037-observability-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/analysis.md`
- `specs/038-concurrency-backend-rules/plan.md`
- `specs/038-concurrency-backend-rules/review.md`
- `specs/038-concurrency-backend-rules/scenarios/a-declared-loss-tolerant-message-owes-no-outbox.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/tasks.md`
- `specs/039-reliability-backend-rules/analysis.md`
- `specs/039-reliability-backend-rules/plan.md`
- `specs/039-reliability-backend-rules/review.md`
- `specs/039-reliability-backend-rules/scenarios/expendable-messages-may-be-dropped-when-recorded.md`
- `specs/039-reliability-backend-rules/spec.md`
- `specs/039-reliability-backend-rules/tasks.md`
- `specs/040-configurable-specs-dir/spec.md`
- `specs/041-task-pruning/analysis.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/plan.md`
- `specs/041-task-pruning/review.md`
- `specs/041-task-pruning/scenarios/a-plan-edit-does-not-reopen-a-done-spec.md`
- `specs/041-task-pruning/scenarios/a-suffixed-task-number-is-a-task.md`
- `specs/041-task-pruning/scenarios/an-open-fenced-block-or-comment-is-not-examined.md`
- `specs/041-task-pruning/scenarios/prune-all-walks-every-spec.md`
- `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md`
- `specs/041-task-pruning/scenarios/prune-walks-the-project-files.md`
- `specs/041-task-pruning/spec.md`
- `specs/041-task-pruning/tasks.md`
- `specs/046-scenario-open-question-visibility/plan.md`
- `specs/046-scenario-open-question-visibility/spec.md`
- `specs/047-analyze-findings-durability/scenarios/analyze-record-freshness.md`
- `specs/047-analyze-findings-durability/spec.md`
- `specs/048-govern-acquired-runtime/analysis.md`
- `specs/048-govern-acquired-runtime/data-model.md`
- `specs/048-govern-acquired-runtime/plan.md`
- `specs/048-govern-acquired-runtime/review.md`
- `specs/048-govern-acquired-runtime/scenarios/pointer-follows-the-store.md`
- `specs/048-govern-acquired-runtime/scenarios/state-b-continues-in-session.md`
- `specs/048-govern-acquired-runtime/spec.md`
- `specs/048-govern-acquired-runtime/tasks.md`
- `specs/049-rename-govern-to-ductus/plan.md`
- `specs/049-rename-govern-to-ductus/review.md`
- `specs/049-rename-govern-to-ductus/spec.md`
- `specs/050-constitution/analysis.md`
- `specs/050-constitution/plan.md`
- `specs/050-constitution/review.md`
- `specs/050-constitution/scenarios/a-canonical-source-is-pointed-at-not-copied.md`
- `specs/050-constitution/scenarios/a-decision-is-attributed-to-whoever-made-it.md`
- `specs/050-constitution/scenarios/a-declared-cross-spec-impact-gates-done.md`
- `specs/050-constitution/scenarios/a-primitive-is-probed-in-a-local-clone.md`
- `specs/050-constitution/scenarios/a-retired-filename-leaves-a-decision-record.md`
- `specs/050-constitution/scenarios/adopter-practices-classified-for-promotion.md`
- `specs/050-constitution/spec.md`
- `specs/050-constitution/tasks.md`
- `specs/051-branch-scoped-spec-numbering/analysis.md`
- `specs/051-branch-scoped-spec-numbering/plan.md`
- `specs/051-branch-scoped-spec-numbering/review.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-definition-and-a-title-follow-the-link-grammar.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-pending-fold-never-holds-done.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/fold-target-checked-before-the-rewrite.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/rewrites-preserve-line-endings.md`
- `specs/051-branch-scoped-spec-numbering/spec.md`
- `specs/051-branch-scoped-spec-numbering/tasks.md`
- `specs/054-remove-supersession/plan.md`
- `specs/054-remove-supersession/spec.md`
- `specs/055-shared-constitution/analysis.md`
- `specs/055-shared-constitution/plan.md`
- `specs/055-shared-constitution/review.md`
- `specs/055-shared-constitution/spec.md`
- `specs/055-shared-constitution/tasks.md`
- `specs/056-bootstrap-archive-boundary-split/analysis.md`
- `specs/056-bootstrap-archive-boundary-split/plan.md`
- `specs/056-bootstrap-archive-boundary-split/review.md`
- `specs/056-bootstrap-archive-boundary-split/scenarios/the-walker-places-every-archive-half-section.md`
- `specs/056-bootstrap-archive-boundary-split/spec.md`
- `specs/056-bootstrap-archive-boundary-split/tasks.md`
- `specs/057-analyze-artifact-and-record-relocation/analysis.md`
- `specs/057-analyze-artifact-and-record-relocation/data-model.md`
- `specs/057-analyze-artifact-and-record-relocation/review.md`
- `specs/057-analyze-artifact-and-record-relocation/scenarios/a-merge-carries-each-keys-whole-value.md`
- `specs/057-analyze-artifact-and-record-relocation/spec.md`
- `specs/057-analyze-artifact-and-record-relocation/tasks.md`
- `specs/058-findings-route-at-discovery/analysis.md`
- `specs/058-findings-route-at-discovery/data-model.md`
- `specs/058-findings-route-at-discovery/plan.md`
- `specs/058-findings-route-at-discovery/review.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/061-updates-track-the-latest-release-tag/plan.md`
- `specs/062-concurrent-session-targets/analysis.md`
- `specs/062-concurrent-session-targets/data-model.md`
- `specs/062-concurrent-session-targets/plan.md`
- `specs/062-concurrent-session-targets/review.md`
- `specs/062-concurrent-session-targets/scenarios/first-writers-create-the-ignore-file-once.md`
- `specs/062-concurrent-session-targets/spec.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/plan.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/a-plan-trim-does-not-reopen-a-done-spec.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/project-files-are-measured.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`
- `specs/064-pi-host-support/analysis.md`
- `specs/064-pi-host-support/plan.md`
- `specs/064-pi-host-support/review.md`
- `specs/064-pi-host-support/scenarios/colon-form-commands-run-on-pi.md`
- `specs/064-pi-host-support/spec.md`
- `specs/064-pi-host-support/tasks.md`
- `specs/066-audit-records-and-the-completion-gate/analysis.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/plan.md`
- `specs/066-audit-records-and-the-completion-gate/review.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-commit-after-the-scope-was-resolved-refuses-the-write.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-write-result-states-what-it-wrote.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/criteria-are-verified-in-review.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/process-waivers-file-lists.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/record-paths-are-quoted-only-when-they-need-it.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-scope-includes-uncommitted-work.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-staleness-on-done-specs.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/066-audit-records-and-the-completion-gate/tasks.md`
- `specs/067-analyze-and-audit-checks/analysis.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/review.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-bare-port-span-names-its-remedy.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-none-list-item-is-no-open-question.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-plan-heading-is-matched-without-regard-to-case.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md`
- `specs/067-analyze-and-audit-checks/scenarios/anchor-reference-kinds.md`
- `specs/067-analyze-and-audit-checks/scenarios/check-artifacts-skipped-targets.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-references-results-are-bounded.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-references-unstaged-drift-is-reported.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md`
- `specs/067-analyze-and-audit-checks/scenarios/disposition-drift-is-advisory.md`
- `specs/067-analyze-and-audit-checks/scenarios/orphan-check-skips-ignored-local-state.md`
- `specs/067-analyze-and-audit-checks/scenarios/orphaned-reference-check.md`
- `specs/067-analyze-and-audit-checks/scenarios/promotion-coverage-header-without-a-key.md`
- `specs/067-analyze-and-audit-checks/scenarios/shared-constitution-references-are-judged.md`
- `specs/067-analyze-and-audit-checks/scenarios/traverse-deps-cycle-check.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/068-api-backend-rules/analysis.md`
- `specs/068-api-backend-rules/plan.md`
- `specs/068-api-backend-rules/review.md`
- `specs/068-api-backend-rules/scenarios/repr-003-bounded-numbers-and-patch-001-format-home.md`
- `specs/068-api-backend-rules/spec.md`
- `specs/068-api-backend-rules/tasks.md`
- `specs/069-accessibility-frontend-rules/analysis.md`
- `specs/069-accessibility-frontend-rules/plan.md`
- `specs/069-accessibility-frontend-rules/review.md`
- `specs/069-accessibility-frontend-rules/scenarios/verifications-flag-only-what-the-statement-covers.md`
- `specs/069-accessibility-frontend-rules/spec.md`
- `specs/069-accessibility-frontend-rules/tasks.md`
- `specs/070-performance-frontend-rules/analysis.md`
- `specs/070-performance-frontend-rules/review.md`
- `specs/070-performance-frontend-rules/spec.md`
- `specs/071-command-reference-split/analysis.md`
- `specs/071-command-reference-split/plan.md`
- `specs/071-command-reference-split/review.md`
- `specs/071-command-reference-split/scenarios/reference-sections-are-reached-by-a-step.md`
- `specs/071-command-reference-split/spec.md`
- `specs/071-command-reference-split/tasks.md`
- `specs/073-decided-rule-departures-are-recorded/analysis.md`
- `specs/073-decided-rule-departures-are-recorded/data-model.md`
- `specs/073-decided-rule-departures-are-recorded/plan.md`
- `specs/073-decided-rule-departures-are-recorded/review.md`
- `specs/073-decided-rule-departures-are-recorded/spec.md`
- `specs/073-decided-rule-departures-are-recorded/tasks.md`
- `specs/075-agents-md-holds-what-every-session-needs/plan.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/analysis.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/data-model.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/plan.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/review.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/spec.md`
- `specs/077-security-cross-rules/analysis.md`
- `specs/077-security-cross-rules/data-model.md`
- `specs/077-security-cross-rules/plan.md`
- `specs/077-security-cross-rules/review.md`
- `specs/077-security-cross-rules/scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md`
- `specs/077-security-cross-rules/spec.md`
- `specs/077-security-cross-rules/tasks.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/analysis.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/review.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md`
- `specs/079-review-and-analysis-work-converges/analysis.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/research.md`
- `specs/079-review-and-analysis-work-converges/review.md`
- `specs/079-review-and-analysis-work-converges/scenarios/the-excluded-section-lists-paths-only.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/079-review-and-analysis-work-converges/tasks.md`
- `specs/080-deterministic-runtime-efficiency/analysis.md`
- `specs/080-deterministic-runtime-efficiency/plan.md`
- `specs/080-deterministic-runtime-efficiency/review.md`
- `specs/080-deterministic-runtime-efficiency/scenarios/a-step-gates-on-a-bounded-list-by-its-count.md`
- `specs/080-deterministic-runtime-efficiency/scenarios/groom-derives-routing-candidates.md`
- `specs/080-deterministic-runtime-efficiency/spec.md`
- `specs/080-deterministic-runtime-efficiency/tasks.md`
- `specs/081-reviews-and-checks-follow-what-changed/analysis.md`
- `specs/081-reviews-and-checks-follow-what-changed/plan.md`
- `specs/081-reviews-and-checks-follow-what-changed/review.md`
- `specs/081-reviews-and-checks-follow-what-changed/spec.md`
- `specs/082-rule-assessments-converge-across-runs/analysis.md`
- `specs/082-rule-assessments-converge-across-runs/data-model.md`
- `specs/082-rule-assessments-converge-across-runs/plan.md`
- `specs/082-rule-assessments-converge-across-runs/review.md`
- `specs/082-rule-assessments-converge-across-runs/spec.md`
- `specs/082-rule-assessments-converge-across-runs/tasks.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/analysis.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/plan.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/review.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/spec.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/tasks.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/data-model.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/plan.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/spec.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/tasks.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/spec.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/plan.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/spec.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/tasks.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/tasks.md`
- `specs/README.md`
- `specs/inbox.md`
- `specs/review.md`
- `version`

## Unexamined governance

*None.*
