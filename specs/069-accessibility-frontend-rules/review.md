---
spec: 069-accessibility-frontend-rules
last-run: 2026-11-22T00:14:36Z
reviewed-against: 60113bfd4f5843c8469d85f0ec650ff92270ceef
diff-base: 2983d2a34c07c501545bb76512b6c7c9555f11a5
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 1
scope: 1
unread: []
excluded: 68
contract-breaks: 0
skipped-passes: []
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
  waived: 1
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
  - key: "`FE-A11YFORM-002`'s Not-flagged clause ends in a garbled phrase and `FE-KBD-001`'s lists a flagged case, both untouched by this window — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: outside the window and outside the owner's no-claim-changed instruction
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

Five-pass review of 069's reopening by 925c6905 (020's task 18, AC17), diff-base 2983d2a3, base-source reopen, at HEAD ba69ef5d. The scope is `specs/069-accessibility-frontend-rules/spec.md`, which 925c6905 edited to say a review leaves out a rule file whose header gives it to `/ductus:analyze`. Passes (part P1, one reviewer for 020 and the eight specs 925c6905 reopened) read those hunks and each scope file whole against the five rule files `discover-rule-files` selected (232,805 bytes, each read whole against `wc -c`): no finding and no observation. The seven other rule files are judged by analysis alone (020's AC17), so the efficiency pass ran on its own definition. The criteria step (parts C8 and C9) judged every criterion individually: 14 of 14 met. AC4 was judged from 62 fetched documents, read as full text, every claim of the 40 rules and the header matching (the EUR-Lex page for Directive 2019/882 returned an empty body, so its text was read from the Publications Office's XHTML of the same Directive); AC5 from the fetched WCAG 2.2 Recommendation, every cited number, title and stated level matching. The stored waiver of the five task-6 clauses still applies. Examined 1 of 1; nothing unread.

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

- AC1 — met: Header Categories list names exactly the 11; 11 `## FE-` headings match; all 40 `### FE-` IDs fall in them (python parse of `framework/rules/accessibility-frontend.md`).
- AC2 — met: The 6 FE- categories of `security-frontend.md` (`XSS/CSRF/STORAGE/AUTHN/CSP/DEPS/PII`) and `performance-frontend.md` (`VITALS/BUNDLE/IMAGE/LOAD/FONT/MAIN`) share none with the 11; headings read.
- AC3 — met: `scripts/lint-rule-ids.sh` exit 0 (12 files, 308 headings); all 40 sections have a heading holding only the ID, a Statement of one tier (0 mixed), Verification and Source, no Rationale; test `no_shipped_statement_mixes_tiers` in `runtime/src/primitives/rule_sections.rs` exists.
- AC4 — met: Every Source's document was fetched 2026-11-21 (the file cites titles, not URLs; the URLs are the canonical locations) and every claim checked matches; EUR-Lex HTML refused automated fetch (HTTP 202, empty) so the Directive was read from the Publications Office copy of the same OJ text.
- AC5 — met: Fetched https://`www.w3.org/TR/WCAG22`/ (chars 0-200000 of 269706, covering 1.1.1 to 4.1.3 and 5.2.2): all 59 criterion numbers the file cites exist, each title beside them matches the fetched title exactly (scripted comparison), the only levels the file states for a criterion are AAA for 2.4.13 and 2.3.3 and both are AAA, 4.1.1 is marked obsolete, and 5.2.2 'Full pages' text matches.
- AC6 — met: Python parse: 36 MUST, 4 SHOULD (KBD-005, MOTION-003, A11YTEST-002, A11YTEST-003); KBD-005 and MOTION-003 label Level AAA in text; the two A11YTEST SHOULDs are testing practices; every MUST but FE-A11YTEST-001 has a § criterion in its Source (all `A/AA`).
- AC7 — met: No Statement contains 'CI' or 'continuous integration' (parse); A11YTEST-001's Verification says the engine runs wherever the project chooses; A11YTEST-003 is the SHOULD for every-change.
- AC8 — met: FE-KBD-002 MUST cites §2.4.7 and §1.4.11 (3:1) only; Level AAA §2.4.13 appears only in FE-KBD-005 (SHOULD), grep of 'Level AAA' lines `77/81` and 311.
- AC9 — met: `ductus check-rule-ids --path framework/rules/accessibility-frontend.md` with all 12 rule files: 43 citations, 0 missing, 0 deprecated; FE-PII-002 and FE-AUTHN-002 in `security-frontend.md`, BE-AUTHN-005 in `security-backend.md`.
- AC10 — met: `framework/constitution.md` has no 'backfill' or 'Recorded exception' text and no mention of `accessibility-frontend.md` (grep).
- AC11 — met: Row in `framework/bootstrap/ductus.md` Shared Files table with the Rule-file surface filter (suffix `-frontend.md`); `ductus discover-rule-files` on a copy under scratch: review selection excludes `accessibility-frontend.md` with a 'judged by analysis alone' notice, `--all` selects it (12 files), surfaces=[backend] selects no frontend file.
- AC12 — met: `specs/069-accessibility-frontend-rules` has no `data-model.md`; the header carries the categories and ID format.
- AC13 — met: Of the 55 WCAG 2.2 `A/AA` SCs (4.1.1 excluded), every one is cited as a § in the file, including 1.3.3, 1.4.5, 2.4.5, 2.5.4, 3.2.3, 3.2.4, 3.2.6; header says 'Meeting these rules is not a WCAG conformance claim'. SC list from my knowledge of WCAG 2.2.
- AC14 — met: FE-ARIA-001 Source cites §4.1.2 and is MUST; Severity posture names no exception for it; no rule has a MUST (NOT) on restating an implicit role (grep 'implicit' only hits ARIA-004's `output` note; role="list" absent).

## Observations

- the five task-6 clauses appear in the rule file, the scenario, the spec's Resolved Question and the changelog; they record the flagged side, the not-flagged side and the release note, and no claim contradicts another — `specs/069-accessibility-frontend-rules/spec.md` — **waived**: each copy records a different side of the decision and the changelog is release notes (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/review.md`
- `.ductus/reference/analyze.md`
- `.ductus/reference/review.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-review.md`
- `docs/developing-ductus.md`
- `docs/rules.md`
- `framework/commands/analyze.md`
- `framework/commands/review.md`
- `framework/reference/analyze.md`
- `framework/reference/review.md`
- `runtime/CHANGELOG.md`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/primitives/brief_re_read.rs`
- `runtime/src/primitives/discover_rule_files.rs`
- `runtime/src/primitives/fix_rounds.rs`
- `runtime/src/primitives/resolve_anchor.rs`
- `runtime/src/primitives/start_fix_rounds.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/schema/descriptions.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/line_ending_discipline.rs`
- `scripts/analyze/check.py`
- `scripts/analyze/detect.py`
- `scripts/analyze/step15-brief.md`
- `scripts/mutation/analysis-design-fix-067.py`
- `scripts/mutation/command-reference-split.py`
- `scripts/mutation/exec-scenario-anchors-067.py`
- `scripts/mutation/owner-reviews-067.py`
- `scripts/tests/test-analyze-scripts.py`
- `specs/020-code-review/plan.md`
- `specs/020-code-review/spec.md`
- `specs/020-code-review/tasks.md`
- `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/024-rule-loader/review.md`
- `specs/024-rule-loader/spec.md`
- `specs/034-performance-backend-rules/review.md`
- `specs/034-performance-backend-rules/spec.md`
- `specs/037-observability-backend-rules/plan.md`
- `specs/037-observability-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/plan.md`
- `specs/038-concurrency-backend-rules/review.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/039-reliability-backend-rules/plan.md`
- `specs/039-reliability-backend-rules/review.md`
- `specs/039-reliability-backend-rules/spec.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/068-api-backend-rules/spec.md`
- `specs/070-performance-frontend-rules/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md`
- `specs/079-review-and-analysis-work-converges/analysis.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/inbox.md`

## Unexamined governance

*None.*
