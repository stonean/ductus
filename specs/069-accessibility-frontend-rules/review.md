---
spec: 069-accessibility-frontend-rules
last-run: 2026-11-20T13:11:01Z
reviewed-against: f21743416fdb789b055682e1fcc74ce7e17d13aa
diff-base: 07bd04639ba5874d97d964ad1432e2ec8fb8a263
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
scope: 4
unread: []
excluded: 26
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
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
  waived: 6
  undispositioned: 0
decisions:
  - key: "`FE-SEMHTML-004`'s tightened Verification says help repeats 'in the same relative order' where the Statement says relative to other page content; the flag ('breaks any of the four') and the Statement govern, and the owner's byte bar leaves no room to restore 24 bytes — `framework/rules/accessibility-frontend.md`"
    outcome: waived
    reason: the Statement governs what the rule requires and the Verification's flag is unchanged
    decided-at: 2026-11-20T13:10:54Z
    decided-by: rule
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

Review of 069's second pass over `07bd0463` to the head of this run (base-source `last-review`), scope 4, examined 4: the window holds the owner's correction of 2026-11-20 (`framework/rules/accessibility-frontend.md` tightened from 52,000 to 51,183 bytes to meet the 51,200-byte bar of 2026-11-08, with no claim dropped), the plan's and spec's source citations, and the previous analysis record. Two readers read their files whole against measured sizes: the security pass over `framework/rules/security-backend.md`, `framework/rules/security-cross.md` and `framework/rules/security-frontend.md`, and the other passes over the other nine rule files (the seven it first read in part were then read whole), `AGENTS.md`, `specs/system.md` and the four in-scope files, checking each of the 20 reworded Verification lines of the rule file against its pre-image. `runtime/CHANGELOG.md` and the scenario were read at the 069 entry and whole. No criterion owed a verdict by `compute-review-scope`; the 14 verdicts of the first pass are carried (spec text unchanged for the criteria, and the reader confirmed the tightening falsifies none of AC1 to AC14). 0 MUST, 0 SHOULD, no stored decisions. Nine observations: three fixed (the `FE-A11YFORM-006` and `FE-MOTION-002` clause wording, the plan's `FE-LAYOUT-002` size boundary), six waived by the rule. Measurement (`wc -c`; each rule section from the line after its `### FE-` heading to the next heading of any level, at `fa4ae5c7` then now): `FE-KBD-004` 1,079 to 1,162; `FE-A11YFORM-003` 971 to 1,063; `FE-A11YFORM-006` 1,192 to 1,472; `FE-LAYOUT-002` 1,019 to 1,218; `FE-MOTION-002` 796 to 854; the file 50,939 to 51,183. Fix rounds: one, the wording fixes (`plan.md` exempt), re-read clean by an independent agent, then empty; budget-stop false.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

- `FE-A11YFORM-006`'s tightened Not-flagged clause said 'loading nothing' where the case is a handler that loads no different page, which changed the carve-out's wording; the clause is restored to the case as stated in the scenario — `framework/rules/accessibility-frontend.md` — **fixed** (decided by the rule)
- `FE-MOTION-002`'s tightened Not-flagged clause said 'those thresholds' with no antecedent; it names the general flash and red flash thresholds again — `framework/rules/accessibility-frontend.md` — **fixed** (decided by the rule)
- `plan.md` gave the `FE-LAYOUT-002` section size without a boundary, so it could not be re-derived (the figure counted the next category heading); the plan states the boundary and the sizes measured by it — `specs/069-accessibility-frontend-rules/plan.md` — **fixed** (decided by the rule)
- `FE-SEMHTML-004`'s tightened Verification says help repeats 'in the same relative order' where the Statement says relative to other page content; the flag ('breaks any of the four') and the Statement govern, and the owner's byte bar leaves no room to restore 24 bytes — `framework/rules/accessibility-frontend.md` — **waived**: the Statement governs what the rule requires and the Verification's flag is unchanged (decided by the rule)
- `FE-A11YFORM-003`'s Not-flagged clause dropped its reason clause 'which the Statement does not cover'; the case and its example are intact and the scenario keeps the reason — `framework/rules/accessibility-frontend.md` — **waived**: no case or requirement changed, only a reason clause, and the owner's 51,200-byte bar asks for the cut (decided by the rule)
- `plan.md` 'The file stays one read' records how the size was reached, which a design record would leave to commits; the owner asked for the measured result to be stated in the plan — `specs/069-accessibility-frontend-rules/plan.md` — **waived**: the owner's instruction of 2026-11-20 is to record the measured result and that the file was tightened to meet the 2026-11-08 cap (decided by the rule)
- `specs/069-accessibility-frontend-rules/analysis.md` is stale against HEAD because this window changed its design subjects; the analysis that follows this review regenerates it — `specs/069-accessibility-frontend-rules/analysis.md` — **waived**: the analysis run after this review regenerates the record, which no hand edit may touch (decided by the rule)
- `FE-A11YFORM-002`'s Not-flagged clause ends in a garbled phrase and `FE-KBD-001`'s lists a flagged case, both untouched by this window — `framework/rules/accessibility-frontend.md` — **waived**: outside the window and outside the owner's no-claim-changed instruction (decided by the rule)
- the five task-6 clauses appear in the rule file, the scenario, the spec's Resolved Question and the changelog; they record the flagged side, the not-flagged side and the release note, and no claim contradicts another — `specs/069-accessibility-frontend-rules/spec.md` — **waived**: each copy records a different side of the decision and the changelog is release notes (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `framework/rules/configuration-cross.md`
- `runtime/CHANGELOG.md`
- `specs/008-security-rules/analysis.md`
- `specs/008-security-rules/plan.md`
- `specs/008-security-rules/review.md`
- `specs/008-security-rules/spec.md`
- `specs/017-derive-dont-ask/analysis.md`
- `specs/017-derive-dont-ask/data-model.md`
- `specs/017-derive-dont-ask/plan.md`
- `specs/017-derive-dont-ask/review.md`
- `specs/017-derive-dont-ask/spec.md`
- `specs/039-reliability-backend-rules/analysis.md`
- `specs/039-reliability-backend-rules/plan.md`
- `specs/039-reliability-backend-rules/review.md`
- `specs/039-reliability-backend-rules/spec.md`
- `specs/068-api-backend-rules/analysis.md`
- `specs/068-api-backend-rules/plan.md`
- `specs/068-api-backend-rules/review.md`
- `specs/068-api-backend-rules/spec.md`
- `specs/077-security-cross-rules/analysis.md`
- `specs/077-security-cross-rules/data-model.md`
- `specs/077-security-cross-rules/plan.md`
- `specs/077-security-cross-rules/review.md`
- `specs/077-security-cross-rules/scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md`
- `specs/077-security-cross-rules/spec.md`
- `specs/077-security-cross-rules/tasks.md`

## Unexamined governance

*None.*
