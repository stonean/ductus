---
spec: 070-performance-frontend-rules
last-run: 2026-11-22T00:14:43Z
reviewed-against: 17ff99fab0dedc0b037ad37e7e08e2068fef167b
diff-base: 2983d2a34c07c501545bb76512b6c7c9555f11a5
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 1
scope: 1
unread: []
excluded: 69
contract-breaks: 0
skipped-passes: []
reviewed-digest: {}
criteria-judged: 12
criteria-met: 12
criteria-verdicts:
  - label: AC1
    digest: 45e7fed35160219839a3a9649aa88d8319a0df5ab7ae4ce52bcc6402089c2381
  - label: AC2
    digest: d683903025c67f2eaef6ae685eb6f85a6973cc610b1d5104d83ee52c159d015f
  - label: AC3
    digest: 3686e107e330c86e5b94353963f2eeec153cf6dceb254fc06457c1d56f4446a8
  - label: AC4
    digest: a84aa967e92438bf09d5b5a4f46c5c97c4dc25937b6ce4c35652955241be307d
  - label: AC5
    digest: 2d96142c18f93e81eab7e202651145cb05abaccf4fd317bee74bc69ec037b0ad
  - label: AC6
    digest: b5033763ab53f17ec3403c8c042e97990ebf6ba48d1176801f939e61d7dbce84
  - label: AC7
    digest: 0bf637a2f0388384da7ccf9346ae39c2edfa9d6df79d915756b6eb26cd002959
  - label: AC8
    digest: 03b0ad860244103daebcc7394a0cca984b01481eea233c9684ece3cf677c230b
  - label: AC9
    digest: 88623925a01ad1ee3436cd19083392bbf52a9b5ee4743cb4cf94a664a0a47b50
  - label: AC10
    digest: 0cb09b3fdc9815bb0ec86f15d3717736ac2bac32ec3e361dabeb2813674fe13b
  - label: AC11
    digest: b92cf3bdbd5e466f9907ea4662cc709fd4d84a1310818d671bb1e390cf2847a7
  - label: AC12
    digest: 1b29f384928c7bf398420e1ae6d4dfecd7d49d9b1b3c24ba488f88163b1d2cde
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Review — 070-performance-frontend-rules

## Summary

Five-pass review of 070's reopening by 925c6905 (020's task 18, AC17), diff-base 2983d2a3, base-source reopen, at HEAD ba69ef5d. The scope is `specs/070-performance-frontend-rules/spec.md`, which 925c6905 edited to say a review leaves out a rule file whose header gives it to `/ductus:analyze`. Passes (part P1, one reviewer for 020 and the eight specs 925c6905 reopened) read those hunks and each scope file whole against the five rule files `discover-rule-files` selected (232,805 bytes, each read whole against `wc -c`): no finding and no observation. The seven other rule files are judged by analysis alone (020's AC17), so the efficiency pass ran on its own definition. The criteria step (part C8) judged every criterion individually: 12 of 12 met. AC3 was judged from 54 fetched documents with no mismatch (three guessed URLs replaced by the documents the Sources cite), `FE-LOAD-009` and `FE-LOAD-006` supported by their Chrome and MDN Sources; AC4's thresholds from the three Sources `FE-VITALS-001` cites. Examined 1 of 1; nothing unread.

12 of 12 acceptance criteria judged met.

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

- AC1 — met: Header lists exactly VITALS, BUNDLE, IMAGE, LOAD, FONT, MAIN; six `## FE-` headings, none for another; file ends `-frontend.md`.
- AC2 — met: 22 IDs all in the six; none of the six appears in the categories of `security-frontend.md` or `accessibility-frontend.md`; `scripts/lint-rule-ids.sh` exit 0.
- AC3 — met: Structure holds on all 22 rules and every cited Source was fetched (52 URLs, incl. the earlier 11); no claim contradicted by a fetched document. Caveats: pages were read through WebFetch's summarizer, not whole, and CSP3 was read to char 300000 of 317207; a few rule claims are normative choices the documents do not state (see note in report).
- AC4 — met: Fetched web.dev 'Web Vitals' (LCP 2.5 s, INP 200 ms, CLS 0.1 good; 75th percentile of page loads segmented across mobile and desktop), the thresholds article (same three values, 75th percentile) and the INP-replaces-FID post (INP becomes a Core Web Vital replacing FID); the three Sources FE-VITALS-001 cites were all fetched and match.
- AC5 — met: Only FE-VITALS-002 (MUST) mentions CI, to forbid synthetic-only measurement; no MUST Statement or Verification requires CI; BUNDLE-001 states the release outcome, FE-BUNDLE-003 is SHOULD.
- AC6 — met: FE-BUNDLE-001 Statement sets no figure; its Verification says 'Not flagged: a budget's figure, which the rule leaves to the project'.
- AC7 — met: Citations parsed per rule: LOAD-007 -> BE-API-001, BE-CACHE-004, BE-PAYLOAD-003, FE-PII-003; LOAD-009 -> BE-API-001, FE-PII-003 (and FE-LOAD-007); LOAD-004 -> FE-CSP-002; MAIN-002 -> BE-PAGE; BE-CACHE-004 in `performance-backend.md` defers build-emitted frontend assets to FE-LOAD-007 where loaded.
- AC8 — met: `ductus check-rule-ids --path framework/rules/performance-frontend.md` against all 12 rule files: 27 citations, 0 missing, 0 deprecated; `## BE-PAGE` is a section of `api-backend.md`.
- AC9 — met: Row in the Shared Files table of `framework/bootstrap/ductus.md`; `ductus discover-rule-files` run on a scratch copy leaves `performance-frontend.md` out of a review with the analysis-alone notice and `--all` selects it, as the narrowing states.
- AC10 — met: `framework/constitution.md` has no backfill-exception text and no mention of `performance-frontend.md` (grep).
- AC11 — met: Header states the posture ('MUST only when its absence blocks a user regardless of scale ... SHOULD where it is one means among others'); BUNDLE-002, IMAGE-001, IMAGE-002, LOAD-001, FONT-002 are SHOULD; parse gives 6 MUST (`VITALS-001/002`, BUNDLE-001, `LOAD-002/005`, FONT-001) and 16 SHOULD, 0 mixed, matching the spec's counts.
- AC12 — met: FE-VITALS-001 requires budgets 'in `specs/system.md`', as FE-BUNDLE-001 does.

## Observations

*None.*

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/review.md`
- `.ductus/reference/analyze.md`
- `.ductus/reference/review.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-review.md`
- `.ductus/docs/agents/developing-ductus.md`
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
- `specs/069-accessibility-frontend-rules/review.md`
- `specs/069-accessibility-frontend-rules/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md`
- `specs/079-review-and-analysis-work-converges/analysis.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/inbox.md`

## Unexamined governance

*None.*
