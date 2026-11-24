---
spec: 073-decided-rule-departures-are-recorded
last-run: 2026-11-24T01:32:06Z
analyzed-against: 1fe7317eccf5c2ce56488de76fed771a0f98a734
hard-fail: 0
blocking-findings: 0
advisory: 4
unexamined: 2
analyzed-digest:
  data-model.md: 19bfe627fbe2d70113e46d131f22339ca4d0d7ca31c8341afab464e63c28bdd4
  plan.md: 8fd74d1c02926293959c30fda3d71c49f79ecc0a737a5d93830bcc6e890d6737
  review.md: ecc8d6c436f0779f5c8a77d5d3f44a3a67ea4c86d695f3860b785b7b2c63bc2d
  spec.md: ba9c111f3079f2827fd76ba2b21e7a28d60f791125222da988cfbe4159e55f38
  tasks.md: 8ef1e85783651ba507adc2a5b65497f9b428a1a3f1953099d06b80e11055960c
unexamined-by-reason:
  root-absent: 2
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
  "data-model.md": e3b0c44298fc1c14
  "data-model.md#073 — Decided Rule Departures Are Recorded Data Model": 9816e51484b7f357
  "data-model.md#Attribution": 8b85309e7c49637a
  "data-model.md#Departure entry": e68bfdadce21c406
  "data-model.md#Review waiver marker": bb6eff6111edcae4
  "data-model.md#`check-departures` result": a6eb91ef8b79820a
  "data-model.md#`process-waivers` result": b9b410619ce7812f
  "data-model.md#`waived` rule verdict": 182cb1bcd1dfcb25
  "plan.md": e3b0c44298fc1c14
  "plan.md#073 — Decided Rule Departures Are Recorded Plan": 325e8ab9324b49d2
  "plan.md#A `covers:` item is a file and a site in two code spans": 430f2dd0009b295e
  "plan.md#A scoped waiver is stored, applied and expired at its sites": c03e22f5872c49c1
  "plan.md#Affected Files": 161d5000566400f4
  "plan.md#Analyze needs no change for a scoped entry": 8129e6d1d159c763
  "plan.md#Attribution is the first commit of the entry's unbroken run": bac2c65224d67e9b
  "plan.md#Cross-spec impact": 07c474fde219359c
  "plan.md#Data Model": fe7ff349748dbaad
  "plan.md#Every agent": 4f13f509b625beea
  "plan.md#Finding tiers": 16b9d97a436032c3
  "plan.md#Lost coverage is a `write-review` notice": 73be0f1946e3ad7e
  "plan.md#One reader, one attribution rule, four consumers": d986fa0d9e080d34
  "plan.md#Open Questions Resolved": d3f7f15d72b285f7
  "plan.md#Overview": 9b02a27d4d97b187
  "plan.md#Review: `write-review` derives the waivers, `process-waivers` expires them": 8f5272f5d9798602
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#The constitution gains one bullet and loses two restated sentences": fbc9c07480ab5c58
  "plan.md#The entry grammar is a nested bullet": e7c68ee4282f0ec0
  "plan.md#The record and the report list waived rules apart": 0e69991b29961bb8
  "plan.md#The test that reads history runs in the job that has it": 349b58dfac990758
  "plan.md#Trade-offs": 486cc5c70d408506
  "plan.md#`check-departures` is a new primitive, owned by this spec": 88864c1b4c5362a7
  "plan.md#`waived` is a third verdict shape, which the host cannot fabricate": ba860fee5c49ed6e
  "spec.md": 589d944f1e5c110e
  "spec.md#073 — Decided Rule Departures Are Recorded": 1453ff7ea267091e
  "spec.md#Acceptance Criteria": 9d4944c73fa41cdc
  "spec.md#Departure entries": 58633f0c2a0d113c
  "spec.md#Edge Cases": 05ae0b0726c36040
  "spec.md#Motivation": 33bd041b32232219
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": 6bca711f7f657ed4
  "spec.md#Scoped entries": e93f7c4bb6f630f7
  "spec.md#The analyze waiver": 3a99866a96c10f94
rule-verdicts:
  BE-AUTHZ-007: {verdict: addressed, cite: "data-model.md#Departure entry"}
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#Review: `write-review` derives the waivers, `process-waivers` expires them"}
  BE-DEPS-007: {verdict: finding, tier: should, at: "plan.md#The test that reads history runs in the job that has it", message: "the plan adds a framework-checks.yml step, CI configuration, and does not commit to an automated scan on dependency changes"}
  BE-INPUT-001: {verdict: addressed, cite: "data-model.md#Departure entry"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#Departure entry"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Lost coverage is a `write-review` notice"}
  BE-INPUT-008: {verdict: addressed, cite: "data-model.md#Departure entry"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#`check-departures` is a new primitive, owned by this spec"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "plan.md#Attribution is the first commit of the entry's unbroken run"}
  QUAL-TEST-001: {verdict: addressed, cite: "plan.md#Affected Files"}
blocking: false
dispositions:
  fixed: 2
  deferred: 0
  waived: 4
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-DEPS-007: the plan adds a framework-checks.yml step, CI configuration, and does not commit to an automated scan on dependency changes"
    outcome: waived
    reason: The step runs a repository script and alters no dependency set; runtime.yml's cargo audit job already scans every change to the runtime's manifest and lockfile, as 081's plan records. 017's and 072's analyses waived this rule on the same kind of CI addition.
    decided-at: 2026-11-13T18:42:18Z
    decided-by: rule
  - key: "grounding — spec.md Motivation: the adopter's two MUST departures (compatibility and idempotency) and its shared constitution's three advance decisions are stated as fact with no citation or assumption marker"
    outcome: waived
    reason: "The source is an adopter report from a tree nobody here can open (AGENTS.md: an adopter's tree is out of reach, and no project is named); the requirement stands on the first Motivation paragraph's dated, cited reading of the code, not on this account."
    decided-at: 2026-11-13T18:42:18Z
    decided-by: rule
  - key: grounding — Motivation states as fact that one adopter's governance requires an entry in four places and that a review record there already splits one rule (key custody waived, restore test deferred), with no citation or assumption marker.
    outcome: waived
    reason: "The source is an adopter report from a tree nobody here can open (AGENTS.md: an adopter's tree is out of reach, and no project is named); the scoped entry's requirement stands on its own Resolved Questions and criteria, not on this account."
    decided-at: 2026-11-24T01:32:06Z
    decided-by: rule
  - key: "decision-reasons — Resolved Question 'Why scoped entries live here rather than in a spec of their own' rests on process cost: a spec of its own 'would reopen 073 regardless and add a lifecycle, a review and an analysis to the same change'; it does not state the result a separate spec would have delivered or what the chosen option gives up."
    outcome: waived
    reason: "The Resolved Question's ground is ownership: 073 owns the entry's grammar, data model and check-departures, which a scoped entry changes, so the change lands with its surfaces (AGENTS.md: a feature-specific primitive lives with its feature); the reopen and runs it names are the cost a separate spec would add, and the decision is the operator's adoption at the routing gate (2026-11-23), not reworded here."
    decided-at: 2026-11-24T01:32:06Z
    decided-by: rule
---

# Analysis — 073-decided-rule-departures-are-recorded

## Summary

0 hard-fail, 0 blocking, 4 advisory; not blocking. 2 unexamined target(s). Dispositions: 2 fixed, 0 deferred, 4 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Motivation states as fact that an adopter's spec listed two MUST rules (compatibility, idempotency) as deliberately unmet and its analysis recorded blocking-findings: 0 with no trace of either, with no citation or assumption marker (an adopter's tree is out of reach, so the claim is uncheckable here). — `specs/073-decided-rule-departures-are-recorded/spec.md` — **waived**: The source is an adopter report from a tree nobody here can open (AGENTS.md: an adopter's tree is out of reach, and no project is named); the requirement stands on the first Motivation paragraph's dated, cited reading of the code, not on this account. (decided by the rule)
- grounding — Motivation states as fact that one adopter's governance requires an entry in four places and that a review record there already splits one rule (key custody waived, restore test deferred), with no citation or assumption marker. — `specs/073-decided-rule-departures-are-recorded/spec.md` — **waived**: The source is an adopter report from a tree nobody here can open (AGENTS.md: an adopter's tree is out of reach, and no project is named); the scoped entry's requirement stands on its own Resolved Questions and criteria, not on this account. (decided by the rule)
- decision-reasons — Resolved Question 'Why scoped entries live here rather than in a spec of their own' rests on process cost: a spec of its own 'would reopen 073 regardless and add a lifecycle, a review and an analysis to the same change'; it does not state the result a separate spec would have delivered or what the chosen option gives up. — `specs/073-decided-rule-departures-are-recorded/spec.md` — **waived**: The Resolved Question's ground is ownership: 073 owns the entry's grammar, data model and check-departures, which a scoped entry changes, so the change lands with its surfaces (AGENTS.md: a feature-specific primitive lives with its feature); the reopen and runs it names are the cost a separate spec would add, and the decision is the operator's adoption at the routing gate (2026-11-23), not reworded here. (decided by the rule)
- rule-assessment — BE-DEPS-007: the plan adds a framework-checks.yml step, CI configuration, and does not commit to an automated scan on dependency changes — `specs/073-decided-rule-departures-are-recorded/plan.md` — **waived**: The step runs a repository script and alters no dependency set; runtime.yml's cargo audit job already scans every change to the runtime's manifest and lockfile, as 081's plan records. 017's and 072's analyses waived this rule on the same kind of CI addition. (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- root-absent: 2

## Fixed in this run

- grounding — Technical Decisions, The constitution gains one bullet: 'framework/constitution.md is 49,831 bytes' is stated without a command or date, though the following figure cites wc -c 2026-11-13. — `specs/073-decided-rule-departures-are-recorded/plan.md` — **fixed** (decided by the rule)
- plan-consistency — Technical Decision 'A `covers:` item is a file and a site in two code spans' records who decided and the three options but gives no reason the two-code-span `in` form was chosen over the keyed line or grouped-by-file forms. — `specs/073-decided-rule-departures-are-recorded/plan.md` — **fixed**
