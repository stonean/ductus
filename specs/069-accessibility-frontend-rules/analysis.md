---
spec: 069-accessibility-frontend-rules
last-run: 2026-11-22T02:12:25Z
analyzed-against: 75bebcd90012f37ee32fd07dbc5e6f0f2d46a7ab
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 1
analyzed-digest:
  plan.md: 63beb075c2169c3c37f5c783967075923cf0476bbc4e908b0cc9426e1d583aa0
  review.md: 2324f856e1a61e147e23dc2f972ebbefa96d52689b8da1324e83f2335202fa1d
  scenarios/verifications-flag-only-what-the-statement-covers.md: 4a7f7d2da98dd304b9da859c1eacb3b59fc65695a4a8b1ca3825ec806b7b26df
  spec.md: df47672f7fdc3fea51212ca7851d8b2e993051b46d8008786f3c856b6bc23596
  tasks.md: bce7be1c6396bb3752cf2f477e41126f35aac91e463545dc7b5066dc9e89147d
unexamined-by-reason:
  ships-to-adopter: 1
rule-digest:
  accessibility-frontend.md: 4d7f5ac09d457c99774f5fc341222a464d2b461cb0467cd58a13dc31c928359b
  api-backend.md: 8e496f92a50b68b1ae8b1954e30a85dd142dc65d3b3882089f9c48560096d800
  concurrency-backend.md: 2650d82125bd67945fd327f7fb9f76c57ca10d165925733139afa3cd50562dd4
  configuration-cross.md: 050a5b8dd375fe4790f5d92753db27ae53ac730db1557cbee84b923b72499ca9
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 26c045df79b6a99319bd7dfa09490b4a2d5ac742eee70a6b3ad0235d49093d2a
  security-backend.md: 511a68b25ceb148f8292cda2a829aaff03a38bc3453732e33a70bd5d6acf5df5
  security-cross.md: 26e287f06cdca2744d0972cbf7cf6a48f27d869526dd86c7c375ac4bdc0dadce
  security-frontend.md: 5fddb21530faa805cb00f23c929e9402548ed0d6a0ed3e8e9a910561b74a45a3
section-digest:
  "plan.md": e3b0c44298fc1c14
  "plan.md#069 — Accessibility rules (frontend) Plan": 27db560fc09548de
  "plan.md#AC13 is a completeness claim over all 55 criteria": 99ecccdf1a61d670
  "plan.md#Affected Files": 7bb8505e48505ef5
  "plan.md#Cross-spec impact": a1397fdbe335161e
  "plan.md#Each of the seven criteria lands in the category its guideline sits beside (AC13)": 7480b50b9a51c4ee
  "plan.md#Every Source is re-fetched after the edits (AC4)": 53ebd696c45da822
  "plan.md#Overview": aff6a211f3c3d6f3
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#The file stays one read": 4a6b66711c2668f2
  "plan.md#The header disclaims a conformance claim (AC13)": 47414775df0ecf15
  "plan.md#The spec's own stale passages are corrected": fec3c294ead7297a
  "plan.md#Trade-offs": 2226f2dd1b65bd59
  "plan.md#`FE-ARIA-001` rests on §4.1.2 alone (AC14)": 2abd9d9e0e527c18
  "scenarios/verifications-flag-only-what-the-statement-covers.md": c04f248dd37d5ca4
  "scenarios/verifications-flag-only-what-the-statement-covers.md#Behavior": de08a19f89607a5e
  "scenarios/verifications-flag-only-what-the-statement-covers.md#Context": ce43f536e5471736
  "scenarios/verifications-flag-only-what-the-statement-covers.md#Edge Cases": 64a9722ccb89c1aa
  "scenarios/verifications-flag-only-what-the-statement-covers.md#Open Questions": 9fc5979d590cdb97
  "scenarios/verifications-flag-only-what-the-statement-covers.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/verifications-flag-only-what-the-statement-covers.md#Verifications-flag-only-what-the-statement-covers": f8ba953a9955d66c
  "spec.md": 3a78b8faafbde180
  "spec.md#069 — Accessibility rules (frontend)": 4ea49641989e842a
  "spec.md#Acceptance Criteria": c0d45e30868be69e
  "spec.md#Boundaries (cross-reference, do not duplicate)": 87191fcbf78efc19
  "spec.md#Enforcement": de5a4c3ba229de2a
  "spec.md#Motivation": 3aeb866b29643513
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": 33aa60581c0a6bd9
  "spec.md#Rule ID format": 30e929c42002c4a8
  "spec.md#Rule set scope": ad072fbc449d72eb
  "spec.md#See also": ccc5891ac3ad555e
  "spec.md#Severity posture": 295a0e49acec3c5b
  "spec.md#Standard baseline": e7593638d502934b
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: grounding — Trade-offs states that `MOTION` holds animation and flashing (§2.2.2, §2.3.x) as a fact about the existing rule file, with no citation to the file or a dated read (Grounding advisory).
    outcome: waived
    reason: "advisory and no chore: true of `framework/rules/accessibility-frontend.md`, whose FE-MOTION-001 Source cites WCAG 2.2 §2.2.2 and FE-MOTION-002 and -003 cite §2.3.1 and §2.3.3 (checked 2026-11-22); a Trade-off's premise about the file 069 owns"
    decided-at: 2026-11-22T02:12:25Z
    decided-by: rule
  - key: grounding — Trade-offs states that `FE-A11YFORM-006` and `FE-POINTER-002` already each cite several criteria, as a fact about the existing file, with no citation (Grounding advisory).
    outcome: waived
    reason: "advisory and no chore: true of the rule file, whose FE-A11YFORM-006 Source cites WCAG 2.2 §3.2.1 and §3.2.2 and FE-POINTER-002 §2.5.7, §2.5.1 and §2.5.4 (checked 2026-11-22); a Trade-off's premise about the file 069 owns"
    decided-at: 2026-11-22T02:12:25Z
    decided-by: rule
---

# Analysis — 069-accessibility-frontend-rules

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 1 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 2 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Trade-offs states that `MOTION` holds animation and flashing (§2.2.2, §2.3.x) as a fact about the existing rule file, with no citation to the file or a dated read (Grounding advisory). — `specs/069-accessibility-frontend-rules/plan.md` — **waived**: advisory and no chore: true of `framework/rules/accessibility-frontend.md`, whose FE-MOTION-001 Source cites WCAG 2.2 §2.2.2 and FE-MOTION-002 and -003 cite §2.3.1 and §2.3.3 (checked 2026-11-22); a Trade-off's premise about the file 069 owns (decided by the rule)
- grounding — Trade-offs states that `FE-A11YFORM-006` and `FE-POINTER-002` already each cite several criteria, as a fact about the existing file, with no citation (Grounding advisory). — `specs/069-accessibility-frontend-rules/plan.md` — **waived**: advisory and no chore: true of the rule file, whose FE-A11YFORM-006 Source cites WCAG 2.2 §3.2.1 and §3.2.2 and FE-POINTER-002 §2.5.7, §2.5.1 and §2.5.4 (checked 2026-11-22); a Trade-off's premise about the file 069 owns (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- ships-to-adopter: 1

## Fixed in this run

*None.*
