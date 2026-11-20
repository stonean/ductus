---
spec: 069-accessibility-frontend-rules
last-run: 2026-11-20T04:20:00Z
reviewed-against: 80f5352da93a9e29b1b3031cfcff632c46e24df0
diff-base: fa4ae5c75b7d68e907f18cee94eb8f8157f83f52
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 6
unread:
  - runtime/CHANGELOG.md
excluded: 34
contract-breaks: 0
skipped-passes: []
fix-rounds: 2
re-reads: 2
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
    digest: f6da642fe44729fd0ffea6ad57ce071d386b7313107b74529b599e5266e7e1b0
  - label: AC12
    digest: c6d3dcf87dbb11909d77a22a659277cf97b567ce6c30c3c6f9b6e988abc525aa
  - label: AC13
    digest: 7c9a8e3f97185e9409cff975e4bc7f9a29c8efbb75f230692a3459d1ca58a031
  - label: AC14
    digest: 5808dab9b973d1121e48ef9dd8173f4c67c787c863b5449d0828de96269478cd
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Review — 069-accessibility-frontend-rules

## Summary

Review of 069's second reopening (task 6, the owner's rule decision of 2026-11-19) over `fa4ae5c7..80f5352d` (base-source `reopen`), scope 6 (the plan's five in-scope entries at the window's start, and `specs/069-accessibility-frontend-rules/plan.md`, which this run's fix added), examined 5. Three readers each read their files whole against measured sizes: the security pass over `framework/rules/security-backend.md`, `framework/rules/security-cross.md` and `framework/rules/security-frontend.md`; the reuse, quality, efficiency and simplicity passes over the other nine rule files (`accessibility-frontend.md` among them), `AGENTS.md` and `specs/system.md`; and the criteria step over the spec, plan, tasks, scenario and rule file. In scope they read `framework/rules/accessibility-frontend.md` (all 40 rules), `specs/069-accessibility-frontend-rules/spec.md`, `specs/069-accessibility-frontend-rules/tasks.md`, the scenario and `specs/069-accessibility-frontend-rules/plan.md` whole. `runtime/CHANGELOG.md` (610,176 bytes) was read only at its `[Unreleased]` entry naming 069 (803 bytes), so it is named unread. The criteria step judged AC1 to AC14 met; AC4's sources were not re-fetched in this run, and it rests on the fetches recorded 2026-11-08 and 2026-11-19. 0 MUST, 0 SHOULD, no stored decisions. Three observations, each fixed by the rule: `FE-LAYOUT-002`'s clause reason, `plan.md`'s stale design record, and the missing byte measurement. Task 6's measurement (`wc -c`; bytes of each rule section from its heading's next line to the next heading, at `fa4ae5c7` then after the fix): `FE-KBD-004` 1,079 to 1,313; `FE-A11YFORM-003` 971 to 1,122; `FE-A11YFORM-006` 1,192 to 1,507; `FE-LAYOUT-002` 1,078 to 1,317; `FE-MOTION-002` 796 to 918; the file 50,939 to 52,000 bytes. Fix rounds: two, the rule file and spec and plan edits (exempt) and the `runtime/CHANGELOG.md` sentence, each re-read clean by an independent agent, then empty; budget-stop false. Commit `fix(069): FE-LAYOUT-002's Not-flagged reason matches its Statement, and the plan records task 6 and the rule file's size` holds the fixes.

2 fix round(s): 2 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

- AC1 — met: framework/rules/accessibility-frontend.md header lists the 11 categories and the 40 rule IDs use only them
- AC2 — met: the 11 categories are disjoint from those of performance-frontend.md and security-frontend.md
- AC3 — met: 40 rules each with one-tier Statement, Verification and Source; scripts/lint-rule-ids.sh exits 0; the five Not-flagged clauses sit in Verifications and change no Statement
- AC4 — met: every rule has a Source; claims were re-fetched 2026-11-08 (12b26fab) and task 6's one new WCAG quote 2026-11-19 (spec.md Resolved Questions); this run fetched nothing
- AC5 — met: 58 distinct section numbers cited in Sources, titles and levels match WCAG 2.2, 2.3.3 and 2.4.13 labelled AAA
- AC6 — met: SHOULDs are FE-KBD-005, FE-MOTION-003, FE-A11YTEST-002 and FE-A11YTEST-003; the only MUST with no section citation is FE-A11YTEST-001, the named exception
- AC7 — met: no Statement requires continuous integration; FE-A11YTEST-001 requires a check before each release
- AC8 — met: FE-KBD-002 holds 2.4.7 and 3:1 for an author-styled indicator; 2.4.13 appears only in FE-KBD-005 (SHOULD)
- AC9 — met: all 43 cited FE and BE IDs resolve to one heading each under framework/rules
- AC10 — met: framework/constitution.md has no backfill mention and does not name accessibility-frontend.md
- AC11 — met: framework/bootstrap/ductus.md manifest row ships the file and discover_rule_files.rs keeps the -frontend.md surface
- AC12 — met: no data-model.md in specs/069-accessibility-frontend-rules; header keeps FE-{CATEGORY}-{NNN}
- AC13 — met: all 55 Level A and AA criteria are cited in Sources; the header disclaims a conformance claim
- AC14 — met: FE-ARIA-001 cites WCAG 2.2 4.1.2 and no Statement restricts restating an implicit role

## Observations

- `FE-LAYOUT-002`'s new Not-flagged clause gave its reason as the Statement barring only truncation with no way to reveal it, though the Statement also bars clipping and overlap as their own items; the clause and `spec.md`'s Resolved Question now say a way to reveal the text answers the truncation bar — `framework/rules/accessibility-frontend.md` — **fixed** (decided by the rule)
- `plan.md`'s design record described the state before task 6 (its Overview, its deliverables) and asserted a 51,200-byte bound the file passes at 52,000 bytes; it now records task 6's clauses and the owner's 2026-11-19 settlement that rule files are not size-limited reads — `specs/069-accessibility-frontend-rules/plan.md` — **fixed** (decided by the rule)
- task 6's Done-when asks for bytes before and after per rule and per file, and no committed artifact carried them; the Summary of `review.md` now does — `specs/069-accessibility-frontend-rules/tasks.md` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `framework/rules/api-backend.md`
- `framework/rules/concurrency-backend.md`
- `framework/rules/configuration-cross.md`
- `framework/rules/reliability-backend.md`
- `framework/rules/security-backend.md`
- `framework/rules/security-cross.md`
- `framework/rules/security-frontend.md`
- `runtime/src/primitives/shipped_rule_ids.txt`
- `scripts/audit/README.md`
- `specs/008-security-rules/scenarios/security-rules-flag-what-their-sources-support.md`
- `specs/008-security-rules/spec.md`
- `specs/008-security-rules/tasks.md`
- `specs/017-derive-dont-ask/scenarios/a-throwaway-credential-is-not-a-secret.md`
- `specs/017-derive-dont-ask/spec.md`
- `specs/017-derive-dont-ask/tasks.md`
- `specs/026-framework-self-audit/scenarios/family-40-deprecated-rules-are-removed-on-time.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/038-concurrency-backend-rules/scenarios/a-declared-loss-tolerant-message-owes-no-outbox.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/tasks.md`
- `specs/039-reliability-backend-rules/plan.md`
- `specs/039-reliability-backend-rules/scenarios/expendable-messages-may-be-dropped-when-recorded.md`
- `specs/039-reliability-backend-rules/spec.md`
- `specs/039-reliability-backend-rules/tasks.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/068-api-backend-rules/scenarios/repr-003-bounded-numbers-and-patch-001-format-home.md`
- `specs/068-api-backend-rules/spec.md`
- `specs/068-api-backend-rules/tasks.md`
- `specs/077-security-cross-rules/data-model.md`
- `specs/077-security-cross-rules/plan.md`
- `specs/077-security-cross-rules/scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md`
- `specs/077-security-cross-rules/spec.md`
- `specs/077-security-cross-rules/tasks.md`

## Unexamined governance

*None.*
