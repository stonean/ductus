---
spec: 089-the-pipeline-s-own-cost-is-stated-and-bounded
last-run: 2026-11-27T15:54:07Z
analyzed-against: c729df9fb20ded09a2259b91dc61e5d725b00730
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 0
analyzed-digest:
  plan.md: db0b7137147895240bc0dc78c8b2eaa3c6a6d5c3bef68dce924d05c734d07579
  review.md: c0ed2286cec6911d8a9336b1f7a8655b0d56963a6f6e7855ba826ef326346c51
  spec.md: 0c79023f6ca88208e983e13f32563b540f6ce3619ca71daf9c9ec14e4bf45846
  tasks.md: fd23157c18fb530ceca3d3c9ccfc1f29fa4791a0b3cea288da5dbc2c22210ce2
rule-digest:
  accessibility-frontend.md: 20fb956e35bb749f061090adf631329bafe3e37662a4e47b0076b697784bbf50
  api-backend.md: 8e496f92a50b68b1ae8b1954e30a85dd142dc65d3b3882089f9c48560096d800
  concurrency-backend.md: 2650d82125bd67945fd327f7fb9f76c57ca10d165925733139afa3cd50562dd4
  configuration-cross.md: 050a5b8dd375fe4790f5d92753db27ae53ac730db1557cbee84b923b72499ca9
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 26c045df79b6a99319bd7dfa09490b4a2d5ac742eee70a6b3ad0235d49093d2a
  security-backend.md: c9d90cf6bfa6e4f5492e6d1a49e073fbee467ba08c7788bf6c7224ae8bd5d411
  security-cross.md: 26e287f06cdca2744d0972cbf7cf6a48f27d869526dd86c7c375ac4bdc0dadce
  security-frontend.md: 9a200916d11a5597a6860d1538840bf7d9aa573bddce953a0bef604379d1451b
section-digest:
  "plan.md": e3b0c44298fc1c14
  "plan.md#089 states its own cost": 049a0bb1081173ee
  "plan.md#089 — The pipeline's own cost is stated and bounded Plan": ee480df63e9cd4ce
  "plan.md#Affected Files": d0c9b6462fa01aae
  "plan.md#Cross-spec impact": fa47fa81b778f7af
  "plan.md#Overview": 698abc1ffb6c79b2
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#The analyze reference moves the primitives' rules into their descriptions": d6bc4607f3c16ec0
  "plan.md#The byte ledger, cut in feeding order (AC3)": 8297ed3d1e6bedc8
  "plan.md#The constitution keeps every rule, and must end at or under 49,000": a7e56015893a2598
  "plan.md#The cost rule is a §design-principles bullet (AC1)": 2d7e86c95c1fa766
  "plan.md#The lint governs four rows, two of them globs (AC2)": 1d49b75c2df49cb8
  "plan.md#The review reference cuts copies only": 679734f3f5be2d6b
  "plan.md#Trade-offs": 69265321018192b8
  "plan.md#`AGENTS.md` is cut by `/ductus:prune --project`": 1146b2231ae505ee
  "spec.md": 4e8795d03c46bf92
  "spec.md#089 — The pipeline's own cost is stated and bounded": 4bacdadb157f9dad
  "spec.md#Acceptance Criteria": fcb7820385823216
  "spec.md#Applicable Rules": b0212b03ee3ea81d
  "spec.md#Behavior": ac8abda5a57f0934
  "spec.md#Cost": 9b02f25fec2cdc6d
  "spec.md#Edge Cases": ddc18ff8156adb2a
  "spec.md#Motivation": f63c62c83c429401
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": 8009c7efc0a953a9
rule-verdicts:
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#The lint governs four rows, two of them globs (AC2)"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#The lint governs four rows, two of them globs (AC2)"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#The lint governs four rows, two of them globs (AC2)"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "spec.md#Edge Cases"}
  QUAL-TEST-001: {verdict: addressed, cite: "plan.md#The lint governs four rows, two of them globs (AC2)"}
rules-judged: 306
rules-carried: 0
blocking: false
dispositions:
  fixed: 4
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 089-the-pipeline-s-own-cost-is-stated-and-bounded

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 0 unexamined target(s). Dispositions: 4 fixed, 0 deferred, 0 waived, 0 undispositioned.

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

- grounding — The claim in 'The lint governs four rows' (Where it runs) that the hook and framework-checks.yml already run both scripts on every commit and push with no path filter carries no file-and-step citation or assumption marker (§Grounding). — `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/plan.md` — **fixed** (decided by the rule)
- grounding — The claim in 'The analyze reference moves the primitives' rules into their descriptions' (Its cost) that no golden holds a description cites no search or file (§Grounding). — `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/plan.md` — **fixed** (decided by the rule)
- grounding — The claim in the same Its cost bullet that the 4,096-character bound applies only to /target's always-loaded tools carries no file-and-symbol citation (§Grounding). — `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/plan.md` — **fixed** (decided by the rule)
- grounding — Motivation's claim that AGENTS.md's Workflow had each option state the reopens and review runs it starts (owner, 2026-11-04) cites no file or commit, unlike the preceding grep-backed claim (§Grounding). — `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/spec.md` — **fixed** (decided by the rule)
