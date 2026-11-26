---
spec: 069-accessibility-frontend-rules
last-run: 2026-11-25T19:29:35Z
reviewed-against: 7d3e2fd33f0fbde611b4abf7a9644ad030be37fd
diff-base: 17ff99fab0dedc0b037ad37e7e08e2068fef167b
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 2
scope: 5
unread:
  - specs/069-accessibility-frontend-rules/analysis.md
  - specs/069-accessibility-frontend-rules/plan.md
  - specs/inbox.md
excluded: 445
contract-breaks: 0
skipped-passes: []
fix-rounds: 0
re-reads: 0
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  scenarios/verifications-flag-only-what-the-statement-covers.md: 4a7f7d2da98dd304b9da859c1eacb3b59fc65695a4a8b1ca3825ec806b7b26df
criteria-judged: 14
criteria-met: 14
criteria-verdicts:
  - label: AC1
    digest: 2c82752f4110d488e55e7ed15a13d14d2ec71cdb45723900b5e685ac34eca39e
  - label: AC2
    digest: 1a2925496c666f85dc20203b36a1240fa04bef23366a780e309df52355993f48
  - label: AC3
    digest: 2b2c8b08457ec07769512f91da5fa9e1f267b8c121c2d68a38ec6525b05c8b85
  - label: AC4
    digest: f30780448763dbe10b57d317a304421fd8e934efcc0fdc4bd0a528275e8febe7
  - label: AC5
    digest: d4db9fbd295a2475609ad60547613e1cf86eff2e5fed191cea84ad9783c41307
  - label: AC6
    digest: f091ee61cad75a4f205d2ae290754c41c55a64619e6a14069ca333040d08c4df
  - label: AC7
    digest: 4e21bc5d03d7d97a883bf82f12a09e98b2c85d2533bb4dd8313f19fa69a50870
  - label: AC8
    digest: 426578dd8ac8879e254882bfa1773a7b121b318c19554dd9b338916042cc3d94
  - label: AC9
    digest: ec6a629bc7d938effdf228899bdb42b34ee6f984d13c332c536053123c3852f0
  - label: AC10
    digest: 499ab37e4375aaac14c3536b87704c7170e48f5257e4766dcf145a23ac475c6a
  - label: AC11
    digest: 24996e721115aef754c03c3e46e9567c0d4eb1c4905e193c12497fb270a9b54a
  - label: AC12
    digest: c6d3dcf87dbb11909d77a22a659277cf97b567ce6c30c3c6f9b6e988abc525aa
  - label: AC13
    digest: 7c9a8e3f97185e9409cff975e4bc7f9a29c8efbb75f230692a3459d1ca58a031
  - label: AC14
    digest: 5808dab9b973d1121e48ef9dd8173f4c67c787c863b5449d0828de96269478cd
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 4
  undispositioned: 0
decisions:
  - key: "`FE-A11YFORM-003`'s Not-flagged clause dropped its reason clause 'which the Statement does not cover'; the case and its example are intact and the scenario keeps the reason — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: no case or requirement changed, only a reason clause, and the owner's 51,200-byte bar asks for the cut
    decided-at: 2026-11-20T13:10:54Z
    decided-by: rule
  - key: "`plan.md` 'The file stays one read' records how the size was reached, which a design record would leave to commits; the owner asked for the measured result to be stated in the plan — `specs/069-accessibility-frontend-rules/plan.md`"
    outcome: waived
    reason: the owner's instruction of 2026-11-20 is to record the measured result and that the file was tightened to meet the 2026-11-08 cap
    decided-at: 2026-11-20T13:10:54Z
    decided-by: rule
  - key: "`specs/069-accessibility-frontend-rules/analysis.md` is stale against HEAD because this window changed its design subjects; the analysis that follows this review regenerates it — `specs/069-accessibility-frontend-rules/analysis.md`"
    outcome: waived
    reason: the analysis run after this review regenerates the record, which no hand edit may touch
    decided-at: 2026-11-20T13:10:54Z
    decided-by: rule
  - key: the five task-6 clauses appear in the rule file, the scenario, the spec's Resolved Question and the changelog; they record the flagged side, the not-flagged side and the release note, and no claim contradicts another — `specs/069-accessibility-frontend-rules/spec.md`
    outcome: waived
    reason: each copy records a different side of the decision and the changelog is release notes
    decided-at: 2026-11-20T13:10:54Z
    decided-by: rule
---

# Review — 069-accessibility-frontend-rules

## Summary

Review of 069's code changed since its last review, from `17ff99fa` (`last-review`) to `7d3e2fd3`, for 084's task 19. The five passes read the spec's own change, the hunks of `2f072674` (`chore(069)`) in `framework/rules/accessibility-frontend.md`, against the five loaded rule files, each read whole (231,962 bytes): no finding and no new observation. The criteria step judged all 14 criteria met against the tree, reading the rule file whole (51,128 bytes). AC4, that the file's claims match the current text of its cited sources, was judged by fetching them, about 50 pages in all, the WCAG 2.2, ARIA in HTML, WAI-ARIA 1.2 and Media Queries 5 texts and the 52 cited Understanding documents among them, checked for each technique and failure ID the file cites. Directive (EU) 2019/882 was read from the Publications Office's copy, since EUR-Lex returned a challenge: it names neither WCAG nor EN 301 549, presumes conformity through harmonised standards (Article 15), and applies from 28 June 2025 (Article 2), as the file's header says. Ontario Regulation 191/11's own text was not reachable: its page needs JavaScript and CanLII refused the request. Its claim was checked against Ontario's official guidance page, which states that sections 9 to 19 require WCAG 2.0 Level AA except 1.2.4 and 1.2.5, as the file does. Stored decisions: four on scope files were re-fired because their issues still stand, and they carry their stored waivers: the passes judged the one on `FE-A11YFORM-003`, and the host judged the other three against the tree. `plan.md` still holds "The file stays one read". The analysis is stale again, since `2f072674` changed the rule file after its 2026-11-22 run. Task 6's clauses are untouched, since `2f072674` changed only clauses the earlier record calls untouched by task 6. One expired: `FE-A11YFORM-002`'s and `FE-KBD-001`'s Not-flagged clauses, which `2f072674` rewrote. Examined 2 of 5: `framework/rules/accessibility-frontend.md` and `specs/069-accessibility-frontend-rules/spec.md`, read whole. Unread, by the operator's decision to read each spec's own change (084's `tasks.md`, task 19): `specs/069-accessibility-frontend-rules/analysis.md`, `plan.md` and `specs/inbox.md`, not opened by the passes.

0 fix round(s): 0 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

8 of 291 commits in the window no spec claimed.

14 of 14 acceptance criteria judged met.

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

- AC1 — met: `framework/rules/accessibility-frontend.md` header and the 11 `## FE-` headings name exactly SEMHTML, KBD, ARIA, CONTRAST, A11YFORM, A11YMEDIA, POINTER, LAYOUT, MOTION, TIMING, A11YTEST; all 40 `### FE-` IDs fall in them (scripted heading parse).
- AC2 — met: The `## FE-` headings of `framework/rules/performance-frontend.md` and `framework/rules/security-frontend.md` share no category with the file (grep of `## FE-` across `framework/rules/*.md`).
- AC3 — met: 40 unique `### FE-{CAT}-{NNN}` headings, each with a block-quoted Statement and Verification, no Rationale, no statement mixing MUST and SHOULD (scripted); `scripts/lint-rule-ids.sh` exits 0 (12 files, 306 headings); the test `no_shipped_statement_mixes_tiers` in `runtime/src/primitives/rule_sections.rs` was not run.
- AC4 — met: Every external-standard claim in `framework/rules/accessibility-frontend.md` checked against fetched current text with no discrepancy: header regimes (508 WCAG 2.0 A/AA, ADA WCAG 2.1 AA, Ontario WCAG 2.0 AA except 1.2.4 and 1.2.5, GOV.UK WCAG 2.2 AA, EAA Directive 2019/882 names no WCAG and presumes conformity via Article 15 harmonised standards, applies after 28 June 2025), WCAG 2.2 and W3C/ARIA/ACT/APG/HTML sources named in each Source line.
- AC5 — met: Every §N.N.N cited in `framework/rules/accessibility-frontend.md` (59 numbers, plus 5.2.2 and 5.4) exists in w3.org/TR/WCAG22/ (Recommendation 12 December 2024) with the same title; stated levels (2.3.3 and 2.4.13 AAA, 4.1.2 A) match.
- AC6 — met: The 4 SHOULD rules are `FE-KBD-005` and `FE-MOTION-003` (AAA, labelled in Statement and Source respectively) and `FE-A11YTEST-002` and `FE-A11YTEST-003` (testing); each of the 36 MUST rules cites an A or AA criterion except `FE-A11YTEST-001`, which Severity posture names (scripted Source check, levels per WCAG 2.2 fetch).
- AC7 — met: No Statement in `framework/rules/accessibility-frontend.md` mentions CI or continuous integration (scripted grep); `FE-A11YTEST-001` requires a check before each release and `FE-A11YTEST-003` is the SHOULD for every change.
- AC8 — met: `FE-KBD-002` Statement requires visible focus plus 3:1 for author-styled indicators (Sources §2.4.7, §1.4.11), and §2.4.13 appears in Statements only in `FE-KBD-005` (SHOULD).
- AC9 — met: Every cited ID resolves: all 40 in-file IDs are headings; `FE-PII-002` and `FE-AUTHN-002` are headings in `framework/rules/security-frontend.md`, `BE-AUTHN-005` in `framework/rules/security-backend.md`.
- AC10 — met: `framework/constitution.md` §rules `#### Lifecycle` holds no backfill exception and no mention of `accessibility-frontend.md` (grep for 'Recorded exception' and 'backfill' returns none).
- AC11 — met: `framework/bootstrap/ductus.md` Shared Files table lists `framework/rules/accessibility-frontend.md` to `specs/rules/accessibility-frontend.md`; `ductus discover-rule-files` omits it from a review's selection with the analysis-alone notice and `--all` lists it, as the narrowed AC states (020 AC17).
- AC12 — met: `specs/069-accessibility-frontend-rules/` holds no `data-model.md` (analysis.md, plan.md, review.md, scenarios, spec.md, tasks.md), and the file's header declares the format and categories.
- AC13 — met: Scripted comparison of the §-citations in `framework/rules/accessibility-frontend.md` against WCAG 2.2's 31 Level A and 24 Level AA criteria (4.1.1 excluded) finds all cited, 1.3.3, 1.4.5, 2.4.5, 2.5.4, 3.2.3, 3.2.4 and 3.2.6 included; the header states that meeting the rules is not a WCAG conformance claim.
- AC14 — met: `FE-ARIA-001` Source cites WCAG 2.2 §4.1.2, Severity posture names no exception for it, and no MUST in the file restates an implicit role; ARIA in HTML (fetched) says setting implicit values is only NOT RECOMMENDED.

## Observations

- `FE-A11YFORM-003`'s Not-flagged clause dropped its reason clause 'which the Statement does not cover'; the case and its example are intact and the scenario keeps the reason — `framework/rules/accessibility-frontend.md` — **waived**: no case or requirement changed, only a reason clause, and the owner's 51,200-byte bar asks for the cut (decided by the rule)
- `plan.md` 'The file stays one read' records how the size was reached, which a design record would leave to commits; the owner asked for the measured result to be stated in the plan — `specs/069-accessibility-frontend-rules/plan.md` — **waived**: the owner's instruction of 2026-11-20 is to record the measured result and that the file was tightened to meet the 2026-11-08 cap (decided by the rule)
- `specs/069-accessibility-frontend-rules/analysis.md` is stale against HEAD because this window changed its design subjects; the analysis that follows this review regenerates it — `specs/069-accessibility-frontend-rules/analysis.md` — **waived**: the analysis run after this review regenerates the record, which no hand edit may touch (decided by the rule)
- the five task-6 clauses appear in the rule file, the scenario, the spec's Resolved Question and the changelog; they record the flagged side, the not-flagged side and the release note, and no claim contradicts another — `specs/069-accessibility-frontend-rules/spec.md` — **waived**: each copy records a different side of the decision and the changelog is release notes (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/amend.md`
- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/configure.md`
- `.claude/commands/ductus/consolidate.md`
- `.claude/commands/ductus/fold.md`
- `.claude/commands/ductus/groom.md`
- `.claude/commands/ductus/help.md`
- `.claude/commands/ductus/implement.md`
- `.claude/commands/ductus/prune.md`
- `.claude/commands/ductus/review.md`
- `.claude/commands/ductus/specify.md`
- `.claude/commands/ductus/status.md`
- `.ductus/config.toml`
- `.ductus/reference/analyze.md`
- `.ductus/reference/audit.md`
- `.ductus/reference/clarify.md`
- `.ductus/reference/fold.md`
- `.ductus/reference/groom.md`
- `.ductus/reference/implement.md`
- `.ductus/reference/prune.md`
- `.ductus/reference/review.md`
- `.gitattributes`
- `.githooks/pre-push`
- `.github/workflows/framework-checks.yml`
- `.github/workflows/runtime-currency.yml`
- `.github/workflows/runtime-release.yml`
- `.github/workflows/runtime.yml`
- `.pi/prompts/ductus-amend.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-consolidate.md`
- `.pi/prompts/ductus-fold.md`
- `.pi/prompts/ductus-groom.md`
- `.pi/prompts/ductus-help.md`
- `.pi/prompts/ductus-implement.md`
- `.pi/prompts/ductus-prune.md`
- `.pi/prompts/ductus-review.md`
- `.pi/prompts/ductus-specify.md`
- `.pi/prompts/ductus-status.md`
- `AGENTS.md`
- `README.md`
- `docs/analyze.md`
- `.ductus/docs/agents/developing-ductus.md`
- `docs/rules.md`
- `docs/shared-constitution.md`
- `docs/slash-commands.md`
- `framework/bootstrap/configure/claude.md`
- `framework/bootstrap/ductus-procedure.md`
- `framework/bootstrap/ductus.md`
- `framework/bootstrap/govern.md`
- `framework/bootstrap/hooks/ductus-pre-push`
- `framework/bootstrap/hooks/pre-push`
- `framework/commands/amend.md`
- `framework/commands/analyze.md`
- `framework/commands/consolidate.md`
- `framework/commands/fold.md`
- `framework/commands/groom.md`
- `framework/commands/help.md`
- `framework/commands/implement.md`
- `framework/commands/prune.md`
- `framework/commands/review.md`
- `framework/commands/specify.md`
- `framework/commands/status.md`
- `framework/constitution.md`
- `framework/migrations.toml`
- `framework/migrations/audit-record-relocate.md`
- `framework/migrations/criterion-label-backfill.md`
- `framework/migrations/project-review-record.md`
- `framework/reference/analyze.md`
- `framework/reference/audit.md`
- `framework/reference/clarify.md`
- `framework/reference/fold.md`
- `framework/reference/groom.md`
- `framework/reference/implement.md`
- `framework/reference/prune.md`
- `framework/reference/review.md`
- `framework/rules/security-backend.md`
- `framework/rules/security-frontend.md`
- `framework/runtime-tools.txt`
- `framework/templates/ci/adopter-generators.yml`
- `framework/templates/spec/spec.md`
- `install.sh`
- `runtime/CHANGELOG.md`
- `runtime/Cargo.lock`
- `runtime/Cargo.toml`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/interpreter/payload.rs`
- `runtime/src/main.rs`
- `runtime/src/mcp/server.rs`
- `runtime/src/parser/mod.rs`
- `runtime/src/primitives/analyze_subjects.rs`
- `runtime/src/primitives/append_inbox.rs`
- `runtime/src/primitives/append_question.rs`
- `runtime/src/primitives/bounded_lists.rs`
- `runtime/src/primitives/brief_re_read.rs`
- `runtime/src/primitives/check_artifact_size.rs`
- `runtime/src/primitives/check_artifacts.rs`
- `runtime/src/primitives/check_corpus_links.rs`
- `runtime/src/primitives/check_departures.rs`
- `runtime/src/primitives/check_owed_work.rs`
- `runtime/src/primitives/check_prescriptions.rs`
- `runtime/src/primitives/check_review_gate.rs`
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
- `runtime/src/primitives/read_re_read.rs`
- `runtime/src/primitives/read_spec.rs`
- `runtime/src/primitives/read_tasks.rs`
- `runtime/src/primitives/relocate_audit_records.rs`
- `runtime/src/primitives/remove_inbox_item.rs`
- `runtime/src/primitives/resolve_anchor.rs`
- `runtime/src/primitives/resolve_constitutions.rs`
- `runtime/src/primitives/review_subject.rs`
- `runtime/src/primitives/set_status.rs`
- `runtime/src/primitives/shipped_rule_ids.txt`
- `runtime/src/primitives/start_fix_rounds.rs`
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
- `runtime/tests/check_unreviewed_code_cli.rs`
- `runtime/tests/crlf_preservation.rs`
- `runtime/tests/exec_subprocess.rs`
- `runtime/tests/fix_rounds.rs`
- `runtime/tests/golden/implement-basic.jsonl`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/golden/specify-basic.jsonl`
- `runtime/tests/mcp.rs`
- `runtime/tests/measure_reopenings.rs`
- `runtime/tests/mechanical_sweep_parity.rs`
- `runtime/tests/merge_status_hook.rs`
- `runtime/tests/parity.rs`
- `runtime/tests/result_sizes.rs`
- `runtime/tests/review_command.rs`
- `runtime/tests/walker.rs`
- `scripts/analyze/detect.py`
- `scripts/analyze/judge-brief.md`
- `scripts/analyze/record.py`
- `scripts/audit/README.md`
- `scripts/audit/adopter-shell-behavior.sh`
- `scripts/audit/archive-contents.sh`
- `scripts/audit/cross-doc-consistency.sh`
- `scripts/audit/done-spec-criteria.sh`
- `scripts/audit/permission-wildcard-position.sh`
- `scripts/audit/rename-sweep-residue.sh`
- `scripts/audit/run-all.sh`
- `scripts/audit/ssot-invariants.sh`
- `scripts/audit/sweep-target-manifest-parity.sh`
- `scripts/check-release-tag.sh`
- `scripts/mutation/commit-counts.py`
- `scripts/mutation/compare-and-set-066.py`
- `scripts/mutation/criteria-in-review-066.py`
- `scripts/mutation/departure-waivers.py`
- `scripts/mutation/departures.py`
- `scripts/mutation/exec-arguments.py`
- `scripts/mutation/freshness-results-have-a-bounded-size.py`
- `scripts/mutation/line-citation-strip-066.py`
- `scripts/mutation/loop-inputs-screened.py`
- `scripts/mutation/moved-scope-066.py`
- `scripts/mutation/never-owed-084.py`
- `scripts/mutation/owed-work-084.py`
- `scripts/mutation/plan-edit-takes-no-back-edge.py`
- `scripts/mutation/process-decisions-results-bounded.py`
- `scripts/mutation/project-record-084.py`
- `scripts/mutation/quoted-heading-forms-067.py`
- `scripts/mutation/reachable-trigger-bar-078.py`
- `scripts/mutation/reopen-measure.py`
- `scripts/mutation/review-022-task-41.py`
- `scripts/mutation/review-022-task-42.py`
- `scripts/mutation/review-022-task-46.py`
- `scripts/mutation/review-departures-bounded.py`
- `scripts/mutation/review-read-credit.py`
- `scripts/mutation/reviews-follow-what-changed.py`
- `scripts/mutation/routing-path-claims.py`
- `scripts/mutation/rule-files-left-to-analysis-020.py`
- `scripts/mutation/scope-attribution.py`
- `scripts/mutation/site-anchors-020.py`
- `scripts/mutation/status-count-084.py`
- `scripts/mutation/suffixed-task-number.py`
- `scripts/mutation/unreviewed-code-084.py`
- `scripts/mutation/waiver-notices-flatten.py`
- `scripts/mutation/waivers-bounded.py`
- `scripts/tests/test-analyze-scripts.py`
- `scripts/tests/test-archive-contents.sh`
- `scripts/tests/test-audit-path-containment.sh`
- `scripts/tests/test-check-release-tag.sh`
- `scripts/tests/test-done-spec-owed-work.sh`
- `scripts/tests/test-run-workflow-steps.sh`
- `specs/000-slash-commands/review.md`
- `specs/000-slash-commands/scenarios/criterion-route-after-draft.md`
- `specs/000-slash-commands/scenarios/scenario-without-task-visibility.md`
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
- `specs/012-multi-agent-govern/review.md`
- `specs/012-multi-agent-govern/spec.md`
- `specs/013-text-first-artifacts/plan.md`
- `specs/013-text-first-artifacts/scenarios/criterion-identifiers.md`
- `specs/014-reclarify-backedge/plan.md`
- `specs/014-reclarify-backedge/scenarios/reopen-after-informal-edits.md`
- `specs/014-reclarify-backedge/spec.md`
- `specs/015-tarball-fetch/analysis.md`
- `specs/015-tarball-fetch/plan.md`
- `specs/015-tarball-fetch/review.md`
- `specs/015-tarball-fetch/spec.md`
- `specs/015-tarball-fetch/tasks.md`
- `specs/016-cross-cutting-rules/spec.md`
- `specs/017-derive-dont-ask/scenarios/a-path-list-commit-leaves-the-shared-index-current.md`
- `specs/017-derive-dont-ask/spec.md`
- `specs/017-derive-dont-ask/tasks.md`
- `specs/018-adopter-owned-pre-commit/spec.md`
- `specs/020-code-review/analysis.md`
- `specs/020-code-review/data-model.md`
- `specs/020-code-review/plan.md`
- `specs/020-code-review/review.md`
- `specs/020-code-review/scenarios/waiver-expiry.md`
- `specs/020-code-review/scenarios/waiver-file-lists.md`
- `specs/020-code-review/spec.md`
- `specs/020-code-review/tasks.md`
- `specs/022-deterministic-runtime/analysis.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/plan.md`
- `specs/022-deterministic-runtime/review.md`
- `specs/022-deterministic-runtime/scenarios/ask-consolidation.md`
- `specs/022-deterministic-runtime/scenarios/groom-command-acceleration.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/scenarios/routing-ranks-by-claimed-path.md`
- `specs/022-deterministic-runtime/scenarios/scenario-open-question-signal.md`
- `specs/022-deterministic-runtime/scenarios/spec-side-parser-hardening.md`
- `specs/022-deterministic-runtime/scenarios/specify-routes-before-scaffolding.md`
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
- `specs/024-rule-loader/spec.md`
- `specs/025-rule-opt-out/review.md`
- `specs/025-rule-opt-out/spec.md`
- `specs/026-framework-self-audit/scenarios/family-23-sweep-target-manifest-parity.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/030-cross-service-references/plan.md`
- `specs/030-cross-service-references/spec.md`
- `specs/034-performance-backend-rules/analysis.md`
- `specs/034-performance-backend-rules/spec.md`
- `specs/035-groom-session-target/scenarios/confirmation-names-reopen.md`
- `specs/035-groom-session-target/scenarios/reopen-done-spec-on-scenario.md`
- `specs/037-observability-backend-rules/analysis.md`
- `specs/037-observability-backend-rules/plan.md`
- `specs/037-observability-backend-rules/review.md`
- `specs/037-observability-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/analysis.md`
- `specs/038-concurrency-backend-rules/plan.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/039-reliability-backend-rules/analysis.md`
- `specs/039-reliability-backend-rules/plan.md`
- `specs/039-reliability-backend-rules/spec.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/plan.md`
- `specs/041-task-pruning/scenarios/a-plan-edit-does-not-reopen-a-done-spec.md`
- `specs/041-task-pruning/scenarios/a-suffixed-task-number-is-a-task.md`
- `specs/041-task-pruning/scenarios/prune-all-walks-every-spec.md`
- `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md`
- `specs/041-task-pruning/scenarios/prune-walks-the-project-files.md`
- `specs/041-task-pruning/spec.md`
- `specs/046-scenario-open-question-visibility/plan.md`
- `specs/046-scenario-open-question-visibility/spec.md`
- `specs/047-analyze-findings-durability/scenarios/analyze-record-freshness.md`
- `specs/047-analyze-findings-durability/spec.md`
- `specs/048-govern-acquired-runtime/analysis.md`
- `specs/048-govern-acquired-runtime/plan.md`
- `specs/048-govern-acquired-runtime/review.md`
- `specs/048-govern-acquired-runtime/scenarios/pointer-follows-the-store.md`
- `specs/048-govern-acquired-runtime/spec.md`
- `specs/048-govern-acquired-runtime/tasks.md`
- `specs/049-rename-govern-to-ductus/plan.md`
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
- `specs/051-branch-scoped-spec-numbering/plan.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-pending-fold-never-holds-done.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/fold-target-checked-before-the-rewrite.md`
- `specs/051-branch-scoped-spec-numbering/spec.md`
- `specs/054-remove-supersession/plan.md`
- `specs/054-remove-supersession/spec.md`
- `specs/055-shared-constitution/analysis.md`
- `specs/055-shared-constitution/plan.md`
- `specs/055-shared-constitution/review.md`
- `specs/055-shared-constitution/spec.md`
- `specs/055-shared-constitution/tasks.md`
- `specs/057-analyze-artifact-and-record-relocation/spec.md`
- `specs/058-findings-route-at-discovery/analysis.md`
- `specs/058-findings-route-at-discovery/data-model.md`
- `specs/058-findings-route-at-discovery/plan.md`
- `specs/058-findings-route-at-discovery/review.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/062-concurrent-session-targets/review.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/plan.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/a-plan-trim-does-not-reopen-a-done-spec.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/project-files-are-measured.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`
- `specs/066-audit-records-and-the-completion-gate/analysis.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/plan.md`
- `specs/066-audit-records-and-the-completion-gate/review.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-commit-after-the-scope-was-resolved-refuses-the-write.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-write-result-states-what-it-wrote.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/criteria-are-verified-in-review.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/process-waivers-file-lists.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-scope-includes-uncommitted-work.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-staleness-on-done-specs.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/066-audit-records-and-the-completion-gate/tasks.md`
- `specs/067-analyze-and-audit-checks/analysis.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/review.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-none-list-item-is-no-open-question.md`
- `specs/067-analyze-and-audit-checks/scenarios/anchor-reference-kinds.md`
- `specs/067-analyze-and-audit-checks/scenarios/disposition-drift-is-advisory.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/068-api-backend-rules/analysis.md`
- `specs/068-api-backend-rules/review.md`
- `specs/068-api-backend-rules/spec.md`
- `specs/070-performance-frontend-rules/analysis.md`
- `specs/070-performance-frontend-rules/review.md`
- `specs/070-performance-frontend-rules/spec.md`
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
- `specs/079-review-and-analysis-work-converges/analysis.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/review.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/079-review-and-analysis-work-converges/tasks.md`
- `specs/080-deterministic-runtime-efficiency/plan.md`
- `specs/080-deterministic-runtime-efficiency/scenarios/groom-derives-routing-candidates.md`
- `specs/080-deterministic-runtime-efficiency/spec.md`
- `specs/081-reviews-and-checks-follow-what-changed/plan.md`
- `specs/081-reviews-and-checks-follow-what-changed/spec.md`
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
- `specs/review.md`
- `version`

## Unexamined governance

*None.*
