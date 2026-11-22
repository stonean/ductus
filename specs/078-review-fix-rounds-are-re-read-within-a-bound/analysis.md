---
spec: 078-review-fix-rounds-are-re-read-within-a-bound
last-run: 2026-11-22T02:12:39Z
analyzed-against: 27dc36020c142b4b1631fe1de7b5bf96c4f43e02
hard-fail: 0
blocking-findings: 0
advisory: 4
unexamined: 0
analyzed-digest:
  data-model.md: 168e5910602ec2917f2b05ea82a7359ebf3961dabb571dfccdb2179c5f5be526
  plan.md: 0a10bccad75fcbde514215c5b62acc75cb4230c95faebca5d7657870d90a98bd
  review.md: 2235ac196037533057865be4eec80545cd0cce01dc0c8099a1a08d706d1e7193
  scenarios/a-reviews-own-record-is-no-fix-round.md: 8a0190ef5a926425199a4381d558f52db626cdd81803b4e23c9aa1055b130440
  scenarios/fix-round-results-have-a-bounded-size.md: 3cf9dc7ebb4213d371eadee7db64826448d2da4ed0bf3ffa96b4fecb897f78d9
  spec.md: c7657fe51162a5b9cab63415c998e5276ea0651d5eaa619a1700e766227bcf89
  tasks.md: 5a3b984be9a91aa30f899e70470a4a2fec22068021a0ae9e32518d00e9f257f0
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
  "data-model.md#078 — Review fix rounds are re-read within a bound: Data Model": 40879fe4a29eba24
  "data-model.md#Result file": 0e19e16fb21a3913
  "data-model.md#Round directory": e492c1ae98c4b5cc
  "data-model.md#`[review] re-reads-off`": 2a05c9a8d3e2849b
  "data-model.md#`brief-re-read`": dfdefa13b5873dd7
  "data-model.md#`check-prescriptions`": 03ed796b190b3535
  "data-model.md#`check-review-gate` — the fix-round check": b8713ded490c86dd
  "data-model.md#`read-re-read`": 387d60471b04d3f2
  "data-model.md#`start-fix-rounds`": 3c32d2f1992531df
  "data-model.md#`write-review` — the loop's argument and record fields": 7ec1da947fed384a
  "plan.md": e3b0c44298fc1c14
  "plan.md#078 — Review fix rounds are re-read within a bound Plan": 402f9bd506161815
  "plan.md#A round's paths are a tree-to-tree diff": ee6c925be5c7b7d4
  "plan.md#Affected Files": 7231037ce483d6e9
  "plan.md#Cross-spec impact": 61e2bcd9ccb4ba66
  "plan.md#Data Model": 612c8030662cf56d
  "plan.md#Every agent gets the same procedure": 855805f377eb9169
  "plan.md#Exemption is a path predicate the runtime evaluates": 41f77aa34e9aca42
  "plan.md#Four primitives, owned by this feature": f0e5f9223bf25e1e
  "plan.md#Overview": 7277cf2a54e1ee5d
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#The exec path makes no round": cd0f15518f8a60e9
  "plan.md#The loop's end and its budget": f32686b195dec18a
  "plan.md#The re-read reports through a file the runtime reads": 7471b4594c20c31f
  "plan.md#The record and the gate are 066's": 1c15f82f548cd402
  "plan.md#The round directory ignores itself": ea3f14850d27063f
  "plan.md#The runtime writes the brief": 73c4e25f2892af48
  "plan.md#The snapshot is an unreferenced commit object": e65135f7a471a7b8
  "plan.md#The switch is read by the runtime": a4db01d04ccc371b
  "plan.md#Trade-offs": 97562be797245232
  "scenarios/a-reviews-own-record-is-no-fix-round.md": 9bad63fce44c0b72
  "scenarios/a-reviews-own-record-is-no-fix-round.md#A-reviews-own-record-is-no-fix-round": a4e5ff5be5a9945c
  "scenarios/a-reviews-own-record-is-no-fix-round.md#Behavior": 822878e85d168158
  "scenarios/a-reviews-own-record-is-no-fix-round.md#Context": e2683faa0c1632cd
  "scenarios/a-reviews-own-record-is-no-fix-round.md#Edge Cases": b529a0d2c904987d
  "scenarios/a-reviews-own-record-is-no-fix-round.md#Open Questions": 9fc5979d590cdb97
  "scenarios/a-reviews-own-record-is-no-fix-round.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/fix-round-results-have-a-bounded-size.md": 950580b43a29d1d1
  "scenarios/fix-round-results-have-a-bounded-size.md#Behavior": 944315d93029c1ff
  "scenarios/fix-round-results-have-a-bounded-size.md#Context": 843dda83f3bdec5c
  "scenarios/fix-round-results-have-a-bounded-size.md#Edge Cases": ea550395c6b36cf6
  "scenarios/fix-round-results-have-a-bounded-size.md#Fix-round-results-have-a-bounded-size": de186b4ebc296908
  "scenarios/fix-round-results-have-a-bounded-size.md#Open Questions": 9fc5979d590cdb97
  "scenarios/fix-round-results-have-a-bounded-size.md#Resolved Questions": b5f7b98fb3491576
  "spec.md": cd848d376d1b048d
  "spec.md#078 — Review fix rounds are re-read within a bound": 8f6723f817eacc73
  "spec.md#A prescribed edit is checked by diff": cc1bbef59da9609a
  "spec.md#Acceptance Criteria": e7be4b164643ba26
  "spec.md#Applicable Rules": bd26e5c22e91f4c8
  "spec.md#Behavior": 29ab98cdbf8ff7c1
  "spec.md#Edge Cases": 7483f80ae33ad838
  "spec.md#Hosts": d11c66ea78fd2a44
  "spec.md#Motivation": 6a01d3c44afa77f2
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": 79a4122dc6bd5211
  "spec.md#The budget": c0ad31930edf4097
  "spec.md#The loop": 1b2c1536965f39af
  "spec.md#The record": e1f9cf073a10065c
  "spec.md#The scope is derived": 1abb065a3c44cd1b
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#The record and the gate are 066's"}
  BE-INPUT-001: {verdict: addressed, cite: "data-model.md#Result file"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#Round directory"}
  BE-INPUT-003: {verdict: addressed, cite: "data-model.md#`brief-re-read`"}
  BE-INPUT-004: {verdict: addressed, cite: "data-model.md#Round directory"}
  BE-INPUT-008: {verdict: addressed, cite: "data-model.md#Result file"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "scenarios/fix-round-results-have-a-bounded-size.md#Behavior"}
  BE-RACE-001: {verdict: addressed, cite: "plan.md#Trade-offs"}
  BE-RACE-002: {verdict: addressed, cite: "plan.md#Trade-offs"}
  BE-RACE-003: {verdict: addressed, cite: "data-model.md#Round directory"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#The loop's end and its budget"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#The runtime writes the brief"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#The loop's end and its budget"}
  CFG-CONST-004: {verdict: addressed, cite: "plan.md#The switch is read by the runtime"}
  CFG-ENV-001: {verdict: addressed, cite: "data-model.md#`[review] re-reads-off`"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#The switch is read by the runtime"}
  CFG-ENV-005: {verdict: addressed, cite: "data-model.md#`[review] re-reads-off`"}
  CFG-ENV-008: {verdict: addressed, cite: "data-model.md#`[review] re-reads-off`"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "plan.md#The loop's end and its budget"}
  QUAL-DELEG-001: {verdict: addressed, cite: "plan.md#A round's paths are a tree-to-tree diff"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 4
  undispositioned: 0
decisions:
  - key: grounding — the Motivation's account of 076's avoidable rounds cites no commit or record
    outcome: waived
    reason: History that motivates the spec and governs no behavior; 076's review commits keep it.
    decided-at: 2026-11-04T14:34:33Z
    decided-by: andy@stone.dev
  - key: "grounding — Technical Decisions, 'The runtime writes the brief': asserts an adopter's report of a review on 0.60.0 found the earlier bar admitted contrived NOTEs, with no citation to the report or any artifact holding it (Grounding, form check)."
    outcome: waived
    reason: "advisory and no chore: the figure comes from an adopter's report, recorded where the decision it motivated was made; this repository names no adopter and cannot open its tree (AGENTS.md), so no reachable artifact can be cited, and no behavior rests on it"
    decided-at: 2026-11-22T02:12:39Z
    decided-by: rule
  - key: "grounding — Technical Decisions, 'The loop's end and its budget': asserts an adopter's review on 0.60.0 ran six fix rounds and four re-reads over about four hours, with no citation or stated measurement method (Grounding, form check)."
    outcome: waived
    reason: "advisory and no chore: the figure comes from an adopter's report, recorded where the decision it motivated was made; this repository names no adopter and cannot open its tree (AGENTS.md), so no reachable artifact can be cited, and no behavior rests on it"
    decided-at: 2026-11-22T02:12:39Z
    decided-by: rule
  - key: "decision-reasons — Resolved Questions, 'Does check-review-gate hold done...': rejects 'blocking on every such round' because declining then has no path to done but a whole /review re-run, a review-run cost stated as the reason rather than the result the option gives up."
    outcome: waived
    reason: "advisory and no chore: the entry states the result the rejected option delivers, not only its cost: under it a declined re-read has no path to done, removing the decline the first resolution gives the operator, and the passing gate's guidance is named as the caveat §design-principles rules out"
    decided-at: 2026-11-22T02:12:39Z
    decided-by: rule
---

# Analysis — 078-review-fix-rounds-are-re-read-within-a-bound

## Summary

0 hard-fail, 0 blocking, 4 advisory; not blocking. 0 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 4 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Technical Decisions, 'The runtime writes the brief': asserts an adopter's report of a review on 0.60.0 found the earlier bar admitted contrived NOTEs, with no citation to the report or any artifact holding it (Grounding, form check). — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — **waived**: advisory and no chore: the figure comes from an adopter's report, recorded where the decision it motivated was made; this repository names no adopter and cannot open its tree (AGENTS.md), so no reachable artifact can be cited, and no behavior rests on it (decided by the rule)
- grounding — Technical Decisions, 'The loop's end and its budget': asserts an adopter's review on 0.60.0 ran six fix rounds and four re-reads over about four hours, with no citation or stated measurement method (Grounding, form check). — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — **waived**: advisory and no chore: the figure comes from an adopter's report, recorded where the decision it motivated was made; this repository names no adopter and cannot open its tree (AGENTS.md), so no reachable artifact can be cited, and no behavior rests on it (decided by the rule)
- grounding — Motivation: asserts spec 076's review ran five fix rounds and four re-reads in one session and that the second re-read found a MUST earlier rounds had recorded fixed, citing only the spec number, no review record or commit (Grounding, form check). — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **waived**: History that motivates the spec and governs no behavior; 076's review commits keep it.
- decision-reasons — Resolved Questions, 'Does check-review-gate hold done...': rejects 'blocking on every such round' because declining then has no path to done but a whole /review re-run, a review-run cost stated as the reason rather than the result the option gives up. — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md` — **waived**: advisory and no chore: the entry states the result the rejected option delivers, not only its cost: under it a declined re-read has no path to done, removing the decline the first resolution gives the operator, and the passing gate's guidance is named as the caveat §design-principles rules out (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

*None — every target was examined.*

## Fixed in this run

*None.*
