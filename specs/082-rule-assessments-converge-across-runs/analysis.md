---
spec: 082-rule-assessments-converge-across-runs
last-run: 2026-11-21T21:29:42Z
analyzed-against: a093738665d542f94bafb6da7dea0b3297454ffd
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 0
analyzed-digest:
  data-model.md: 4d2166e42b8f55b378577c4ef2603e04a3a61917d2ab868fdeee11e9aa7b9686
  plan.md: 44bef71bdc13ac55e2e3a6abeef64a571286688d4657463a9f6f03e4c8794628
  review.md: 41a71e83d8b66f7b26aa38633b0c23720d81b68333253f22738bc8b5cb2ac892
  spec.md: 6f230995198708dbe5074e414f5ac3fcb0a4a336fbe5aeb591df77af8c4b3f23
  tasks.md: 09ddf8f10797f84c7ce292ef44adc1a22a7405b30fbfd76577c00f34ad75e36d
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
  "data-model.md": e3b0c44298fc1c14
  "data-model.md#082 — Rule assessments converge across runs Data Model": dd37e08bc1766e3f
  "data-model.md#Record fields": df060606646ca35d
  "data-model.md#Relations": 55e3c71b1c21029e
  "data-model.md#`assessSpecQuality` response": 6f176e699dda1a58
  "data-model.md#`discover-rule-files` with a `feature`": c3f42e11c8d4624b
  "data-model.md#`write-analysis` arguments and result": 1ceb2c22c455ccdc
  "plan.md": e3b0c44298fc1c14
  "plan.md#082 — Rule assessments converge across runs Plan": 4ac38ffe676fefe3
  "plan.md#A contradiction is derived from the prior record": d36bf6b3c7797594
  "plan.md#A finding at an unchanged section holds nothing (AC7)": f9414b6288fdd994
  "plan.md#A section is a heading's own text (AC7)": 6ebc578debc1a3ec
  "plan.md#A stored verdict is current while four things match": 012c8cfc5afe225a
  "plan.md#Affected Files": d54894db365aaf25
  "plan.md#An `addressed` citation is checked by `write-analysis`": 78dd477e501d24ca
  "plan.md#Cross-spec impact": 27016e190059e250
  "plan.md#Data Model": f0770ee49b487bfb
  "plan.md#Open Questions Resolved": f7ad2ba9db626f11
  "plan.md#Overview": e40b39ec3d869709
  "plan.md#Primitive interface changes, classified under `BE-COMPAT-001`": 026d2085285904d8
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#The completion gate holds `done` while a loaded rule is unjudged": 8696d66e3be4bdf3
  "plan.md#The constitution pays about 160 bytes": 1a8bb6b2881e35c3
  "plan.md#The exec response says whether a fired rule was addressed": 3a3cb947417d2a89
  "plan.md#The record holds four fields": 49e446d6ec40eb9d
  "plan.md#Trade-offs": 7aa43c146445c8ef
  "plan.md#Where the decision is made: a library, reached through an extended `discover-rule-files`": 73176c4c7826c251
  "plan.md#`rule_verdicts` is one library": fb0d576e6ccf4635
  "plan.md#`write-analysis` derives the rest": 8bfa60c97f51971f
  "spec.md": a133a7dd46be0ac6
  "spec.md#082 — Rule assessments converge across runs": 89d2ac9b8c1766e1
  "spec.md#Acceptance Criteria": f9895090107d3a33
  "spec.md#Applicable Rules": 4e09d046d86ec43c
  "spec.md#Edge Cases": a13edbc54e86ba24
  "spec.md#Motivation": dd98ee3ad2293e20
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": ead670898f6bc92b
rule-verdicts:
  BE-CACHE-001: {verdict: addressed, cite: "plan.md#A stored verdict is current while four things match"}
  BE-CACHE-002: {verdict: addressed, cite: "plan.md#A stored verdict is current while four things match"}
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#Primitive interface changes, classified under `BE-COMPAT-001`"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#`write-analysis` derives the rest"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#An `addressed` citation is checked by `write-analysis`"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#Where the decision is made: a library, reached through an extended `discover-rule-files`"}
  BE-RACE-001: {verdict: addressed, cite: "spec.md#Edge Cases"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#`write-analysis` derives the rest"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#`write-analysis` derives the rest"}
  BE-TXN-004: {verdict: addressed, cite: "plan.md#`write-analysis` derives the rest"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#`write-analysis` derives the rest"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "plan.md#`write-analysis` derives the rest"}
blocking: false
dispositions:
  fixed: 9
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 082-rule-assessments-converge-across-runs

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 0 unexamined target(s). Dispositions: 9 fixed, 0 deferred, 0 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

*None — every target was examined.*

## Fixed in this run

- grounding — Trade-offs said a rule edit is rare beside a spec edit, which already judges every rule: the frequency had no measurement, and since AC7 a spec edit judges each rule against its changed sections only (§Grounding (advisory)). — `specs/082-rule-assessments-converge-across-runs/plan.md` — **fixed** (decided by the rule)
- grounding — The completion-gate decision said a spec edit stales every verdict, which since AC7 holds only for a record with no section-digest (§Grounding (advisory)). — `specs/082-rule-assessments-converge-across-runs/plan.md` — **fixed** (decided by the rule)
- grounding — write-analysis' derivation put unjudged-rules' count sentence under section-digest, as 70c4f3da split it, and cited the unresolved-constitution count with no symbol (§Grounding (advisory)). — `specs/082-rule-assessments-converge-across-runs/plan.md` — **fixed** (decided by the rule)
- grounding — BE-COMPAT-001's behavior changes said a host passing no judged-rules names every assessable rule unjudged, though write-analysis carries each current verdict (§Grounding (advisory)). — `specs/082-rule-assessments-converge-across-runs/plan.md` — **fixed** (decided by the rule)
- grounding — Trade-offs said check-artifact-size does not measure analysis.md, and BE-COMPAT-001 that the runtime is pre-1.0, each with no citation (§Grounding (advisory)). — `specs/082-rule-assessments-converge-across-runs/plan.md` — **fixed** (decided by the rule)
- grounding — Where do the verdicts live? said check-artifact-size does not measure analysis.md, with no citation (§Grounding (advisory)). — `specs/082-rule-assessments-converge-across-runs/spec.md` — **fixed** (decided by the rule)
- rule-assessment — BE-TXN-002: write-analysis' derivation named no lost-update guard for the record's read-modify-write of its decisions and rule verdicts, saying the later of two concurrent runs stands, though since 91771eec write-analysis refuses a write over a record changed since it read it. — `specs/082-rule-assessments-converge-across-runs/plan.md` — **fixed** (decided by the rule)
- grounding — Edge Cases said the later of two runs writing the record stands, which since 91771eec (066 task 24) is refused with changed-since-read (§Grounding (advisory)). — `specs/082-rule-assessments-converge-across-runs/spec.md` — **fixed** (decided by the rule)
- rule-assessment — CFG-CONST-001: write-analysis' derivation said the reason keys more than one module writes are constants in one place without naming that location. — `specs/082-rule-assessments-converge-across-runs/plan.md` — **fixed** (decided by the rule)
