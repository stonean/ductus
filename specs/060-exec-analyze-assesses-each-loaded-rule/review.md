---
spec: 060-exec-analyze-assesses-each-loaded-rule
last-run: 2026-11-27T18:16:46Z
reviewed-against: f21892fc15ae59ec6176a067d538ee7bf6231846
diff-base: 861c18ae239001c17472c284220cd2ba05b62922
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 16
scope: 16
unread: []
excluded: 324
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  scenarios/an-assessment-sees-the-loaded-rule-files.md: 7a1341466362f69527aecd8d09472a6d9ba5ec6d9db5e30a0e316814fae06007
  scenarios/an-assessment-sees-the-rule-it-asks-about.md: 03489f5510e4b9cf5f38194f6a0e8ab6ffcff7272359914c991129ba72a9b70f
  scenarios/rule-judging-runs-from-repo-scripts.md: 3c3d8b962193afb84c80fddaf0d8e6144baa956d4f039fdb27e5dcca6593c08b
criteria-judged: 10
criteria-met: 10
criteria-verdicts:
  - label: AC1
    digest: e976632c7b44b5561aeb64ff42dd2025a79ebdab99aedd186260f828158e972f
  - label: AC2
    digest: 953beb08463e8a3a00315bf21b53fe683f98c7e72599aab50a83fb9f17db6f90
  - label: AC3
    digest: 9d550792938cb3a537180a53deabc62d622d76bf2c9825fd2b17916955c5960c
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
  fixed: 10
  deferred: 0
  waived: 5
  undispositioned: 0
decisions:
  - key: "perf: build_assess_spec_quality_request re-reads the spec file for every rule request, up to 167 times per step — `runtime/src/interpreter/payload.rs`"
    outcome: waived
    reason: each read is a small local file the OS caches, beside an LLM round trip per request; reading it once would thread a second value through the walker context for a gain nobody has measured
    decided-at: 2026-09-27T19:20:13Z
    decided-by: andy@stone.dev
  - key: "other: configuration-cross.md's header names only analyze as enforcing it; which review pass applies the file is open — `framework/rules/configuration-cross.md`"
    outcome: deferred
    target: specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md
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
  - key: "other: `scripts/analyze/judge-brief.md` says `check.py` matches the share's key \"whole\" and `.ductus/docs/agents/developing-ductus.md` says it refuses a read account keyed by anything but the exact path, but `read_account` in `scripts/analyze/check.py` also accepts the basename of an absolute share path (its docstring says so), so a basename key passes. The instruction to key by the exact path is safe; only the stated reason is overstated. — `scripts/analyze/judge-brief.md`"
    outcome: waived
    reason: the instruction to key the share by its exact path is correct and passes `check.py`; only its stated reason overstates, by omitting a basename fallback no judge following the instruction reaches, and the observation is about this repository's own analysis tooling, which `AGENTS.md` waives unless the fix is mechanical, and rewording the reason is not
    decided-at: 2026-11-25T19:51:00Z
    decided-by: rule
  - key: "other: the edge case 'Cells that disagree on a rule' in scenario `rule-judging-runs-from-repo-scripts` says they are never settled silently and settling is the host's, but `combine.py` resolves addressed-against-not-fired itself (printed as 'cells differ', not UNSETTLED); only finding-against-other is left to the host — `specs/060-exec-analyze-assesses-each-loaded-rule/scenarios/rule-judging-runs-from-repo-scripts.md`"
    outcome: waived
    reason: combine.py prints 'cells differ' when an addressed answer stands beside not fired, so no disagreement passes silently, and it applies the merge rule `.ductus/docs/agents/developing-ductus.md` §Sizing an analysis's judges states, leaving only a finding beside a cell that found none to the host; narrowing the scenario's requirement to that rule rewrites a requirement, which is the owner's call, not a chore
    decided-at: 2026-11-27T18:16:46Z
    decided-by: rule
  - key: "bug: with `--rules-json FILE` older than a rule-file edit, `split.py` takes the rule text from FILE but records `rule-file-digest` from the files as they stand, so the new refusal cannot see that the shares hold old text — `scripts/analyze/split.py`"
    outcome: waived
    reason: "no procedure passes `--rules-json`: `.ductus/docs/agents/developing-ductus.md` runs `split.py` after `detect.py` without it, and `split.py` refuses detection older than any rule-file edit, so the stale shares need a host to save the call's result, edit a rule file and re-run detection before passing the old file, an input this repository's analyses never produce"
    decided-at: 2026-11-27T18:16:46Z
    decided-by: rule
---

# Review — 060-exec-analyze-assesses-each-loaded-rule

## Summary

Review of 060's change since its last review, from `861c18ae` (`last-review`) to `f21892fc`: task 15 and the five entries the last record left unread. One passes part ran all five passes over the 16 scope entries, the record this run replaces (`review.md`) and the five loaded rule files, each read whole (302,571 bytes with the record, and 231,962, by `wc -c`), and one criteria part judged every criterion against the code, by symbol in `runtime/src/interpreter/` and in the scripts, running `cargo test --test parity`, `cargo test --lib` and `scripts/tests/test-analyze-scripts.py` (the agent's recommendation, adopted by the operator, 2026-11-27). All 10 criteria are met; AC3's text was corrected to 082's reuse of a current verdict, since the code follows 082 (the agent's recommendation, adopted by the operator, 2026-11-27). The passes found three SHOULD violations, `QUAL-CLAIM-001` at `scripts/analyze/record.py` and `QUAL-GROUND-001` at `scripts/analyze/record.py` and `scripts/analyze/split.py`, and one low-confidence `CFG-ENV-002` at `scripts/analyze/analyze_lib.py`, all fixed in the run: the scripts after `split.py` refuse a loaded rule file changed since it ran, a judged rule's tier other than `must` or `should` refuses, `selected` is read strictly, and `docs/runtime.md` names `TMPDIR` and `ANALYZE_SCRIPTS`. Of 15 observations, the broken contract, the scenario's promise of a test that fails when each script's behaviour is removed, is fixed by new cases that kill each of the 20 mutant patches that survived the suite, the tier mapping's included, and the guard a 21st patch removed is gone, since nothing could reach it; nine more are fixed as chores, and five are waived, three by their stored decisions. The passes re-ran over the 62,566-byte fix diff, reading it whole and the changed files at its hunks, with `scripts/analyze/analyze_lib.py`, `scripts/analyze/record.py` and `scripts/analyze/split.py` re-read in part as they now stand; the fix rounds' re-read reads each changed path. Out of scope, so recorded nowhere: `.github/workflows/framework-checks.yml` step (ab)'s comment omits case Z; `scripts/analyze/detect.py` prints `examined=None` for a result that carries no count and reads `selected` with a default; and `specs/022-deterministic-runtime/data-model.md`'s `assessSpecQuality` says each rule is asked about once, omitting 082's reuse. `docs/developing-ductus.md` was deleted in the window, moved to `.ductus/docs/agents/developing-ductus.md`, which the passes read. Stored decisions: 3 matched, 14 dropped under spec 086 as on files 060 does not own, 7 retained on files this scope does not hold. Steps 1, 2, 8 and 9 first ran through this session's MCP server, which predates 086's runtime, and were derived again through `runtime/target/release/ductus`, built from `c729df9f`, HEAD's runtime source, which ran every later primitive: the scope and the rule selection came out the same, and the stored decisions as given here.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

8 of 128 commits in the window no spec claimed.

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

- AC1 — met: runtime/src/interpreter/mod.rs Walker::handle_assessments filters assessable rules by rule.severity from RuleSection::tier (rule_sections.rs); proven by interpreter::tests::an_exec_analyze_asks_about_each_loaded_rule_once_in_its_own_tier (TS-BOTH-001 asked once, at must) and parity analyze_completes_on_the_session_write_session_writes
- AC2 — met: AnalyzeTally::record_assessment (analyze_tally.rs) writes one verdicts entry and one tier count per asked rule; reused rules are never asked (Walker::split_reused); proven by analyze_tally::tests::each_assessment_leaves_its_verdict_and_judges_its_rule and an_exec_step_reuses_the_current_verdicts_and_asks_about_the_rest
- AC3 — met: text corrected to 082's reuse (the agent's recommendation, adopted by the operator, 2026-11-27): Walker::split_reused and AnalyzeTally::record_reused carry a reused rule's verdict, and record_rule_set and record_step_asking_nothing record every rule neither asked nor reused as rule-file-unreadable or rule-assessments-not-checked, both in framework/reference/analyze.md Unexamined targets and 066's data model; proven by an_exec_analyze_asks_about_each_loaded_rule_once_in_its_own_tier, an_exec_step_reuses_the_current_verdicts_and_asks_about_the_rest and an_exec_walk_that_asks_about_fewer_rules_than_were_loaded_names_each_unjudged
- AC4 — met: parity analyze_completes_on_the_session_write_session_writes asserts one request for CFG-CONST-001 and blocking-findings 1, advisory 0; an_exec_analyze_asks_about_each_loaded_rule_once_in_its_own_tier runs three rules plus two unassessable and one unreadable file and asserts each asked or recorded unexamined
- AC5 — met: Walker::handle_assessments loops over asked rules, seeding ASSESSED_RULE_KEY and LOADED_RULE_FILES_KEY per exchange, built by payload.rs build_assess_spec_quality_request; split_reused skips only rules with a current verdict (082); proven by an_exec_analyze_asks_about_each_loaded_rule_once_in_its_own_tier (loaded-rule-files asserted per request) and an_exec_step_reuses_the_current_verdicts_and_asks_about_the_rest
- AC6 — met: rule_sections.rs RuleSection::tier (MUST wins, SHOULD only without MUST, else None) feeds payload.rs load_rules where a no-tier rule increments unassessable; proven by rule_sections::tests::a_statement_s_keyword_gives_its_tier, a_statement_carrying_both_keywords_is_must_tier, a_statement_with_no_keyword_has_no_tier and payload::tests::load_rules_sorts_each_rule_by_whether_it_can_be_assessed
- AC7 — met: payload.rs load_rules counts an unreadable rule file in unreadable_files (record_rule_set -> rule-file-unreadable) and Walker::handle_assessments calls record_step_asking_nothing when loaded_none; proven by parity analyze_with_no_rule_directory_records_its_citations_unexamined (rule-assessments-not-checked: 2) and the rule-file-unreadable: 1 assertion in an_exec_analyze_asks_about_each_loaded_rule_once_in_its_own_tier
- AC8 — met: framework/commands/analyze.md steps 11 and 12 ('evaluates no trigger itself ... answers passed: true when it does not' and 'the host judges the trigger, answering passed: true') and specs/022-deterministic-runtime/data-model.md ### assessSpecQuality ('The walker evaluates no Verification trigger ... answers passed: true') both state it; read in full, no test needed for prose
- AC9 — met: specs/066-audit-records-and-the-completion-gate/data-model.md rule-assessments-not-checked entry counts loaded rules steps 11 and 12 did not ask about (and unjudged-rules), not requests; specs/022-deterministic-runtime/spec.md Signpost links back to 060-exec-analyze-assesses-each-loaded-rule/spec.md
- AC10 — met: framework/commands/analyze.md step 11 and framework/reference/analyze.md Rule judging in a subagent state the one-fresh-subagent hand-back and the unanswered or short-read rule left out of judged-rules; this repo's side is scripts/analyze/check.py (bytes-read account, every ID answered) and judge-brief.md, proven by python3 -I scripts/tests/test-analyze-scripts.py (all passed, incl. AT, Z) and write_analysis/interpreter unjudged tests

## Observations

- other: no test fails when the tier mapping or 12 further guards and recorded classes of `scripts/analyze/` are removed, though scenario `rule-judging-runs-from-repo-scripts` says each script has a test that fails when its behavior is removed (QUAL-TEST-001) — `scripts/tests/test-analyze-scripts.py` (breaks the spec's contract) — **fixed** (decided by the rule)
- other: `.ductus/docs/agents/developing-ductus.md` says to give a re-judging judge its share's 'new path' and that `check.py` 'refuses an account keyed to the old one', but `split.py` writes the share at the fixed path `<work>/share-<letter>.md` every run, so there is no new share path — `.ductus/docs/agents/developing-ductus.md` — **fixed** (decided by the rule)
- other: `scripts/analyze/judge-brief.md` says `check.py` matches the share's key whole and `.ductus/docs/agents/developing-ductus.md` says it refuses a read account keyed by anything but the exact path, but `read_account` also accepts the basename of an absolute share path — `scripts/analyze/judge-brief.md` — **waived**: the instruction to key the share by its exact path is correct and passes `check.py`; only its stated reason overstates, by omitting a basename fallback no judge following the instruction reaches, and the observation is about this repository's own analysis tooling, which `AGENTS.md` waives unless the fix is mechanical, and rewording the reason is not (decided by the rule)
- other: `tree_state` ignores git's exit status and runs git in text mode, so a failing git diff digests as empty and a line-ending-only edit can fold to the same digest — `scripts/analyze/analyze_lib.py:248` — **waived**: git diff HEAD fails only with no HEAD commit, which head() refuses before the digest is compared, and the feature, rule files and constitution, the files detection judges by content, are digested byte for byte; the re-read judged both cases contrived (decided by the rule)
- other: `_settle`'s DUCTUS_BIN executable check runs for every script, so `briefs.py`, which calls no primitive, and a usage error also fail at startup when no runtime is built — `scripts/analyze/analyze_lib.py:92` — **waived**: briefs.py runs only after split.py, which needs the runtime, so the check fails no run that could otherwise succeed, and its message names the build command (decided by the rule)
- other: the docstring of `scripts/analyze/record.py` says a dry run prints the write-analysis call, but run() builds that command and prints it nowhere without --write (a stale doc comment) — `scripts/analyze/record.py:47` — **fixed** (decided by the rule)
- other: the dry run of `scripts/analyze/record.py` is not write-free: it calls process-decisions, which writes a gitignored lists file under `.ductus/results/` whenever a decision is retained, against the plan's and the scenario's 'write no repository file' — `scripts/analyze/record.py:263` — **fixed** (decided by the rule)
- bug: a plan's `settled` entry bypasses every check the cells' verdicts get in `scripts/analyze/record.py`: a finding with no `at` ends in a bare KeyError traceback (exit 2), and a `waived` for a rule with no departure entry, an unknown verdict kind, or an `addressed` with no cite is recorded — `scripts/analyze/record.py:216` — **fixed** (decided by the rule)
- other: the edge case 'Cells that disagree on a rule' in scenario `rule-judging-runs-from-repo-scripts` says they are never settled silently and settling is the host's, but `combine.py` resolves addressed-against-not-fired itself (printed as 'cells differ', not UNSETTLED); only finding-against-other is left to the host — `specs/060-exec-analyze-assesses-each-loaded-rule/scenarios/rule-judging-runs-from-repo-scripts.md` — **waived**: combine.py prints 'cells differ' when an addressed answer stands beside not fired, so no disagreement passes silently, and it applies the merge rule `.ductus/docs/agents/developing-ductus.md` §Sizing an analysis's judges states, leaving only a finding beside a cell that found none to the host; narrowing the scenario's requirement to that rule rewrites a requirement, which is the owner's call, not a chore (decided by the rule)
- other: the `DUCTUS_BIN` row of `docs/runtime.md` says the variable is a path taken from where the script started; for `scripts/analyze/` a bare name is also resolved on PATH (plan: 'the path, or a name on PATH') — `docs/runtime.md` — **fixed** (decided by the rule)
- other: `.ductus/docs/agents/developing-ductus.md` says detect.py's own findings (check-artifact-size's, say) are recorded only when the plan lists them, but `record.py` writes check-artifact-size's part-findings itself (family project-parts); only its oversized findings need the plan — `.ductus/docs/agents/developing-ductus.md` — **fixed** (decided by the rule)
- other: `scripts/analyze/analyze_lib.py` named `valid_feature` a second time as `feature_dir`, and `combine.merge`'s `unresolved=()` default was never used, since both callers pass it: dead code — `scripts/analyze/analyze_lib.py` — **fixed** (decided by the rule)
- other: the design record in `plan.md` ('What is recorded is what the judges wrote') lists three `record.py` refusals and says detection staleness names 'a rule file', but the fix adds two more: a loaded rule file changed or gone since `split.py` (also in `check.py` and `combine.py`), and a settled verdict that fails the cell checks, plus an unreadable tier — `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — **fixed** (decided by the rule)
- bug: with `--rules-json FILE` older than a rule-file edit, `split.py` takes the rule text from FILE but records `rule-file-digest` from the files as they stand, so the new refusal cannot see that the shares hold old text — `scripts/analyze/split.py` — **waived**: no procedure passes `--rules-json`: `.ductus/docs/agents/developing-ductus.md` runs `split.py` after `detect.py` without it, and `split.py` refuses detection older than any rule-file edit, so the stale shares need a host to save the call's result, edit a rule file and re-run detection before passing the old file, an input this repository's analyses never produce (decided by the rule)
- other: `docs/runtime.md` named `TMPDIR` among the platform variables it does not list, then documented the `scripts/analyze/` scripts' read of it — `docs/runtime.md` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/amend.md`
- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/configure.md`
- `.claude/commands/ductus/fold.md`
- `.claude/commands/ductus/help.md`
- `.claude/commands/ductus/plan.md`
- `.claude/commands/ductus/prune.md`
- `.claude/commands/ductus/review.md`
- `.claude/commands/ductus/target.md`
- `.ductus/config.toml`
- `.ductus/reference/analyze.md`
- `.ductus/reference/audit.md`
- `.ductus/reference/fold.md`
- `.ductus/reference/implement.md`
- `.ductus/reference/plan.md`
- `.ductus/reference/prune.md`
- `.ductus/reference/review.md`
- `.githooks/pre-commit`
- `.github/workflows/framework-checks.yml`
- `.pi/prompts/ductus-amend.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-fold.md`
- `.pi/prompts/ductus-help.md`
- `.pi/prompts/ductus-plan.md`
- `.pi/prompts/ductus-prune.md`
- `.pi/prompts/ductus-review.md`
- `.pi/prompts/ductus-target.md`
- `AGENTS.md`
- `README.md`
- `docs/analyze.md`
- `docs/slash-commands.md`
- `framework/bootstrap/configure/claude.md`
- `framework/bootstrap/ductus-procedure.md`
- `framework/bootstrap/ductus.md`
- `framework/bootstrap/govern.md`
- `framework/bootstrap/hooks/ductus-pre-commit`
- `framework/commands/amend.md`
- `framework/commands/analyze.md`
- `framework/commands/fold.md`
- `framework/commands/help.md`
- `framework/commands/plan.md`
- `framework/commands/prune.md`
- `framework/commands/review.md`
- `framework/commands/target.md`
- `framework/constitution.md`
- `framework/migrations.toml`
- `framework/migrations/report-project-file-links-into-docs.md`
- `framework/migrations/section-digest-backfill.md`
- `framework/reference/analyze.md`
- `framework/reference/audit.md`
- `framework/reference/fold.md`
- `framework/reference/implement.md`
- `framework/reference/plan.md`
- `framework/reference/prune.md`
- `framework/reference/review.md`
- `framework/runtime-tools.txt`
- `framework/templates/ci/adopter-generators.yml`
- `install.sh`
- `runtime/CHANGELOG.md`
- `runtime/src/interpreter/analyze_tally.rs`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/interpreter/payload.rs`
- `runtime/src/main.rs`
- `runtime/src/mcp/server.rs`
- `runtime/src/parser/mod.rs`
- `runtime/src/primitives/analyze_subjects.rs`
- `runtime/src/primitives/append_question.rs`
- `runtime/src/primitives/backfill_section_digest.rs`
- `runtime/src/primitives/bounded_lists.rs`
- `runtime/src/primitives/check_artifact_size.rs`
- `runtime/src/primitives/check_audit_records.rs`
- `runtime/src/primitives/check_corpus_links.rs`
- `runtime/src/primitives/check_owed_work.rs`
- `runtime/src/primitives/check_rule_ids.rs`
- `runtime/src/primitives/check_unreviewed_code.rs`
- `runtime/src/primitives/compute_review_scope.rs`
- `runtime/src/primitives/criterion_verdicts.rs`
- `runtime/src/primitives/discover_rule_files.rs`
- `runtime/src/primitives/fix_rounds.rs`
- `runtime/src/primitives/guarded.rs`
- `runtime/src/primitives/mod.rs`
- `runtime/src/primitives/pipeline_view.rs`
- `runtime/src/primitives/process_decisions.rs`
- `runtime/src/primitives/process_waivers.rs`
- `runtime/src/primitives/project_parts.rs`
- `runtime/src/primitives/prune_project.rs`
- `runtime/src/primitives/prune_project/moves.rs`
- `runtime/src/primitives/prune_review.rs`
- `runtime/src/primitives/relocate_audit_records.rs`
- `runtime/src/primitives/resolve_constitutions.rs`
- `runtime/src/primitives/review_subject.rs`
- `runtime/src/primitives/rule_sections.rs`
- `runtime/src/primitives/rule_verdicts.rs`
- `runtime/src/primitives/spec_ownership.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/primitives/write_review.rs`
- `runtime/src/primitives/write_review/departure_waivers.rs`
- `runtime/src/primitives/write_review/project_record.rs`
- `runtime/src/schema/config_keys.rs`
- `runtime/src/schema/descriptions.rs`
- `runtime/src/schema/paths.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/src/schema/registry.rs`
- `runtime/src/session.rs`
- `runtime/tests/check_artifact_size_cli.rs`
- `runtime/tests/check_audit_records_cli.rs`
- `runtime/tests/check_unreviewed_code_cli.rs`
- `runtime/tests/common/mod.rs`
- `runtime/tests/crlf_preservation.rs`
- `runtime/tests/fixtures/review-basic/specs/007-review/plan.md`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/mcp.rs`
- `runtime/tests/mechanical_sweep_parity.rs`
- `runtime/tests/parity.rs`
- `runtime/tests/result_sizes.rs`
- `runtime/tests/size_hook.rs`
- `runtime/tests/walker.rs`
- `scripts/analyze/detect.py`
- `scripts/analyze/step15-brief.md`
- `scripts/audit/cross-doc-consistency.sh`
- `scripts/audit/run-all.sh`
- `scripts/lint-governing-size.sh`
- `scripts/lint-tool-coverage.sh`
- `scripts/mutation/artifact-size-bounded.py`
- `scripts/mutation/audit-records-085.py`
- `scripts/mutation/compare-and-set-041.py`
- `scripts/mutation/departure-waivers.py`
- `scripts/mutation/exec-arguments.py`
- `scripts/mutation/first-review-scope-086.py`
- `scripts/mutation/loop-inputs-screened.py`
- `scripts/mutation/outcomes-outside-the-scope-086.py`
- `scripts/mutation/owner-reviews-067.py`
- `scripts/mutation/process-decisions-results-bounded.py`
- `scripts/mutation/process-decisions-retained.py`
- `scripts/mutation/project-parts-087.py`
- `scripts/mutation/project-record-084.py`
- `scripts/mutation/prune-project-containment.py`
- `scripts/mutation/prune-project.py`
- `scripts/mutation/prune-review-086.py`
- `scripts/mutation/resolve-constitutions-size.py`
- `scripts/mutation/review-drops-unowned-086.py`
- `scripts/mutation/reviews-follow-what-changed.py`
- `scripts/mutation/rule-judgments-088.py`
- `scripts/mutation/rule-verdicts-converge.py`
- `scripts/mutation/run.py`
- `scripts/mutation/scope-attribution.py`
- `scripts/mutation/spec-ownership-086.py`
- `scripts/mutation/status-count-084.py`
- `scripts/mutation/stderr-warnings-022.py`
- `scripts/mutation/unreviewed-code-084.py`
- `scripts/mutation/waivers-bounded.py`
- `scripts/stale-literals.sh`
- `scripts/tests/test-audit-record-gate.sh`
- `scripts/tests/test-done-spec-owed-work.sh`
- `scripts/tests/test-lint-governing-size.sh`
- `scripts/tests/test-pre-commit-staging.sh`
- `scripts/tests/test-stale-literals.sh`
- `specs/000-slash-commands/review.md`
- `specs/002-project-scaffolding/analysis.md`
- `specs/003-bootstrap-automation/analysis.md`
- `specs/003-bootstrap-automation/review.md`
- `specs/003-bootstrap-automation/tasks.md`
- `specs/009-scenario-targeting/analysis.md`
- `specs/009-scenario-targeting/review.md`
- `specs/012-multi-agent-govern/review.md`
- `specs/012-multi-agent-govern/tasks.md`
- `specs/014-reclarify-backedge/review.md`
- `specs/016-cross-cutting-rules/review.md`
- `specs/017-derive-dont-ask/analysis.md`
- `specs/017-derive-dont-ask/plan.md`
- `specs/017-derive-dont-ask/review.md`
- `specs/017-derive-dont-ask/scenarios/a-path-list-commit-leaves-the-shared-index-current.md`
- `specs/017-derive-dont-ask/spec.md`
- `specs/017-derive-dont-ask/tasks.md`
- `specs/018-adopter-owned-pre-commit/review.md`
- `specs/019-config-decisions/analysis.md`
- `specs/019-config-decisions/review.md`
- `specs/020-code-review/analysis.md`
- `specs/020-code-review/scenarios/waiver-expiry.md`
- `specs/020-code-review/spec.md`
- `specs/021-runtime-boundary/review.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/plan.md`
- `specs/022-deterministic-runtime/scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md`
- `specs/022-deterministic-runtime/scenarios/coverage-expansion-primitives.md`
- `specs/022-deterministic-runtime/scenarios/parser-walker-conventions.md`
- `specs/022-deterministic-runtime/scenarios/resolve-references-cli-exec-wiring.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/scenarios/scaffolding-primitives.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/023-govern-refinement/review.md`
- `specs/024-rule-loader/review.md`
- `specs/025-rule-opt-out/review.md`
- `specs/026-framework-self-audit/plan.md`
- `specs/026-framework-self-audit/scenarios/family-27-done-spec-unchecked-criteria.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/027-bootstrap-migration-registry/analysis.md`
- `specs/027-bootstrap-migration-registry/review.md`
- `specs/027-bootstrap-migration-registry/tasks.md`
- `specs/029-bootstrap-runtime-autowire/review.md`
- `specs/029-bootstrap-runtime-autowire/spec.md`
- `specs/033-rule-surface-setting/review.md`
- `specs/034-performance-backend-rules/review.md`
- `specs/035-groom-session-target/review.md`
- `specs/036-quality-cross-rules/analysis.md`
- `specs/036-quality-cross-rules/review.md`
- `specs/037-observability-backend-rules/review.md`
- `specs/038-concurrency-backend-rules/review.md`
- `specs/039-reliability-backend-rules/review.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/plan.md`
- `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md`
- `specs/041-task-pruning/scenarios/prune-walks-the-project-files.md`
- `specs/041-task-pruning/spec.md`
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/analysis.md`
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/review.md`
- `specs/043-workflows-sunset/review.md`
- `specs/045-decision-state-drift-detection/plan.md`
- `specs/045-decision-state-drift-detection/review.md`
- `specs/047-analyze-findings-durability/review.md`
- `specs/047-analyze-findings-durability/scenarios/analyze-record-freshness.md`
- `specs/048-govern-acquired-runtime/spec.md`
- `specs/049-rename-govern-to-ductus/review.md`
- `specs/050-constitution/plan.md`
- `specs/050-constitution/scenarios/a-decision-is-attributed-to-whoever-made-it.md`
- `specs/050-constitution/scenarios/a-primitive-is-probed-in-a-local-clone.md`
- `specs/050-constitution/scenarios/findings-route-by-scope.md`
- `specs/050-constitution/spec.md`
- `specs/052-spec-supersession-and-consolidation/analysis.md`
- `specs/052-spec-supersession-and-consolidation/review.md`
- `specs/054-remove-supersession/review.md`
- `specs/055-shared-constitution/data-model.md`
- `specs/055-shared-constitution/review.md`
- `specs/055-shared-constitution/spec.md`
- `specs/055-shared-constitution/tasks.md`
- `specs/056-bootstrap-archive-boundary-split/analysis.md`
- `specs/056-bootstrap-archive-boundary-split/review.md`
- `specs/057-analyze-artifact-and-record-relocation/spec.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/059-project-in-a-repository-subdirectory/review.md`
- `specs/061-updates-track-the-latest-release-tag/review.md`
- `specs/062-concurrent-session-targets/review.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/plan.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/project-files-are-measured.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`
- `specs/064-pi-host-support/analysis.md`
- `specs/066-audit-records-and-the-completion-gate/analysis.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/plan.md`
- `specs/066-audit-records-and-the-completion-gate/review.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-base-includes-the-transition-commit.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-scope-includes-uncommitted-work.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/066-audit-records-and-the-completion-gate/tasks.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/review.md`
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/068-api-backend-rules/review.md`
- `specs/069-accessibility-frontend-rules/review.md`
- `specs/070-performance-frontend-rules/review.md`
- `specs/071-command-reference-split/plan.md`
- `specs/071-command-reference-split/review.md`
- `specs/071-command-reference-split/spec.md`
- `specs/072-governing-documents-stay-within-one-read/spec.md`
- `specs/073-decided-rule-departures-are-recorded/plan.md`
- `specs/073-decided-rule-departures-are-recorded/review.md`
- `specs/075-agents-md-holds-what-every-session-needs/plan.md`
- `specs/075-agents-md-holds-what-every-session-needs/review.md`
- `specs/075-agents-md-holds-what-every-session-needs/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/review.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/review.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/080-deterministic-runtime-efficiency/analysis.md`
- `specs/080-deterministic-runtime-efficiency/data-model.md`
- `specs/080-deterministic-runtime-efficiency/plan.md`
- `specs/080-deterministic-runtime-efficiency/research.md`
- `specs/080-deterministic-runtime-efficiency/review.md`
- `specs/080-deterministic-runtime-efficiency/spec.md`
- `specs/081-reviews-and-checks-follow-what-changed/plan.md`
- `specs/081-reviews-and-checks-follow-what-changed/review.md`
- `specs/081-reviews-and-checks-follow-what-changed/spec.md`
- `specs/082-rule-assessments-converge-across-runs/data-model.md`
- `specs/082-rule-assessments-converge-across-runs/plan.md`
- `specs/082-rule-assessments-converge-across-runs/spec.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/plan.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/review.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/analysis.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/data-model.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/plan.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/review.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/spec.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/tasks.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/analysis.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/data-model.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/plan.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/review.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/spec.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/tasks.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/plan.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/spec.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/tasks.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/analysis.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/review.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/tasks.md`
- `specs/088-rule-judgments-cost-only-what-changed/analysis.md`
- `specs/088-rule-judgments-cost-only-what-changed/data-model.md`
- `specs/088-rule-judgments-cost-only-what-changed/plan.md`
- `specs/088-rule-judgments-cost-only-what-changed/review.md`
- `specs/088-rule-judgments-cost-only-what-changed/spec.md`
- `specs/088-rule-judgments-cost-only-what-changed/tasks.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/analysis.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/plan.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/review.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/spec.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/tasks.md`
- `specs/inbox.md`

## Unexamined governance

*None.*
