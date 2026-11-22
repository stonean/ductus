---
spec: 076-waived-and-deferred-findings-are-distinct-dispositions
last-run: 2026-11-22T23:25:26Z
analyzed-against: 5cd059d165846cd94f7fc77f48a7c5c0957e1eda
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 0
analyzed-digest:
  data-model.md: 971db1cad4efa20324d243b7701015eae95ebf09c39e8bffdf1052c7f2c3016a
  plan.md: 678d56810ac0646bcf12814ed42458801dc03ed53d52c2a540586e8efc55be19
  review.md: f89da03d6caf129aaf0f437035133e60c83bde8f826302b7c125f54a2eb4a63c
  spec.md: 160cf4066e5dba55a461a04358028d364bf1a88d3fc0ee6e59760365733a24a9
  tasks.md: 302b02a555ab616e1356e427a963767d9a9dc60a3b80fe09873dc1a07374f252
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
  security-backend.md: f892282383d7377ded2e5fd1ee3e30421d0d6ad3f9bb1fd354ff0d9d114e807a
  security-cross.md: 26e287f06cdca2744d0972cbf7cf6a48f27d869526dd86c7c375ac4bdc0dadce
  security-frontend.md: 5fddb21530faa805cb00f23c929e9402548ed0d6a0ed3e8e9a910561b74a45a3
section-digest:
  "data-model.md": e3b0c44298fc1c14
  "data-model.md#076 — Waived and deferred findings are distinct dispositions Data Model": e3f44ca9e4b7d67b
  "data-model.md#Release and lapse — derived on every run, never stored": dd3ae2fcd9275a2a
  "data-model.md#`/{project}:review` flags": 8b5cbad44da94102
  "data-model.md#`Disposition` — writer input": 289a64bd30293355
  "data-model.md#`ReviewFinding` — the `write-review` `findings` entry": d3bf08190a72b0cd
  "data-model.md#`analysis.md` gating counts": 36c9fc54cffd2370
  "data-model.md#`decisions` entry — both records": ac4483acd5ff1b57
  "data-model.md#`dispositions` — both records": 77148ff0431bfa21
  "data-model.md#`process-decisions` result": 0b975a7dac3bb4a6
  "plan.md": e3b0c44298fc1c14
  "plan.md#076 — Waived and deferred findings are distinct dispositions Plan": 6e14c6daddcb5d18
  "plan.md#A deferral's companion is a target, a work item, or both": 8b0acd8ef5debe1e
  "plan.md#A review violation's deferral is a stored decision, not a new list": b13e35f268f77f91
  "plan.md#Affected Files": fd309680059607cc
  "plan.md#Cross-spec impact": 3cb6af4ebbf83c94
  "plan.md#Data Model": 87514765b5089693
  "plan.md#Lapse is a `process-decisions` classification": 620e4c2444de815c
  "plan.md#No migration entry": 888b17cd41f202e5
  "plan.md#One disposition per finding goes through the existing drop lists": ed07f12ccd78560b
  "plan.md#Overview": 492e6d3e7097f210
  "plan.md#Release is computed by the writers, so the gate reads counts": 9a73d587d2578d78
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#The definitions live once, in the constitution's Finding dispositions": 1d3d15fe9b403a5d
  "plan.md#The runtime renames, reading the old names only from disk": f8fe40bc852fbcf2
  "plan.md#The sweep is by meaning, over the declared live-artifact set": 5c0afde95c5d7df0
  "plan.md#The waiver example loses its work reference": 00a43be9c1e7874f
  "plan.md#Trade-offs": 2e80d3e54ddddd6c
  "spec.md": 0f600a3b5e848887
  "spec.md#076 — Waived and deferred findings are distinct dispositions": 766ad8ebc1a876bd
  "spec.md#Acceptance Criteria": de1f0e70703c5cd1
  "spec.md#Behavior": def15a0af81bf7fa
  "spec.md#Edge Cases": fb6ccadf06e63982
  "spec.md#Motivation": edc66fb58e74e763
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": 0eb099db54101667
  "spec.md#Terms": 1201e3e7d9bc2db3
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#Trade-offs"}
  BE-INPUT-001: {verdict: addressed, cite: "data-model.md#`Disposition` — writer input"}
  BE-INPUT-004: {verdict: addressed, cite: "data-model.md#`process-decisions` result"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "plan.md#Lapse is a `process-decisions` classification"}
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: "grounding — Resolved Questions, 'Is an external work-item ID free text': 'trackers disagree on form' is stated as fact about external systems with no source or assumption marker (§Grounding)."
    outcome: waived
    reason: "advisory and no chore: a Resolved Question's reason for storing a work-item ID as free text, recorded with the decision; grounding it would need a survey of external trackers, not an edit, and no behavior rests on the claim beyond the free-text rule the data model states"
    decided-at: 2026-11-22T23:25:26Z
    decided-by: rule
  - key: "grounding — Resolved Questions, 'Are waived and discarded one term': 'both lapsed when their finding stopped firing' describes existing behavior with no cited source (§Grounding)."
    outcome: waived
    reason: "advisory and no chore: a Resolved Question recording how the two terms behaved before 076 merged them, history the decision cites as its reason; git keeps the record of the earlier behavior, and no current behavior rests on it"
    decided-at: 2026-11-22T23:25:26Z
    decided-by: rule
---

# Analysis — 076-waived-and-deferred-findings-are-distinct-dispositions

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 0 unexamined target(s). Dispositions: 3 fixed, 0 deferred, 2 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Resolved Questions, 'Is an external work-item ID free text': 'trackers disagree on form' is stated as fact about external systems with no source or assumption marker (§Grounding). — `specs/076-waived-and-deferred-findings-are-distinct-dispositions/spec.md` — **waived**: advisory and no chore: a Resolved Question's reason for storing a work-item ID as free text, recorded with the decision; grounding it would need a survey of external trackers, not an edit, and no behavior rests on the claim beyond the free-text rule the data model states (decided by the rule)
- grounding — Resolved Questions, 'Are waived and discarded one term': 'both lapsed when their finding stopped firing' describes existing behavior with no cited source (§Grounding). — `specs/076-waived-and-deferred-findings-are-distinct-dispositions/spec.md` — **waived**: advisory and no chore: a Resolved Question recording how the two terms behaved before 076 merged them, history the decision cites as its reason; git keeps the record of the earlier behavior, and no current behavior rests on it (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

*None — every target was examined.*

## Fixed in this run

- grounding — Trade-offs, 'The gate computing release from decisions:': 'a stored decision does not know its finding's tier' is a descriptive claim about the current decision record, uncited (§Grounding). — `specs/076-waived-and-deferred-findings-are-distinct-dispositions/plan.md` — **fixed** (decided by the rule)
- grounding — Technical Decisions, 'A deferral's companion': 'the companion text it flattens today' describes decision_for's current behavior without citing a file path or test (§Grounding). — `specs/076-waived-and-deferred-findings-are-distinct-dispositions/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-INPUT-004: The lapse check opens a stored deferral's `target` path read from committed review.md/analysis.md text, but containment (canonicalize and stay within the project) is committed only for a new deferral's `target` at recording. — `specs/076-waived-and-deferred-findings-are-distinct-dispositions/data-model.md` — **fixed**
