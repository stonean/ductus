---
spec: 063-oversized-artifacts-warn-with-a-fix
last-run: 2026-11-14T02:02:39Z
analyzed-against: e6f633341eeec1c66ebfc113ca28ebeb53bb28bf
hard-fail: 0
blocking-findings: 0
advisory: 4
unexamined: 4
analyzed-digest:
  data-model.md: aa648374269201ebb2318b0e5ba6d3b827cc9ac606c9e31fd1bdd78ded8c6b58
  plan.md: 53d74cbc382bd2840cf1ecb3cf827e5f538df80d03d47f5716cc15ebc2d3100f
  review.md: 7e2c022ee70276026482bcc6d734da0b30a64ad5bd5df612bbddd1cd4a48043b
  scenarios/a-plan-trim-does-not-reopen-a-done-spec.md: c77ea68f76643287537d7bd22ce126885e5fc1d56680d59e7970b862f1c2a6cf
  scenarios/an-invalid-threshold-fails-fast.md: 5945eec14f64769ce85570033863a2f209ed8784a7fd1f00d0e9d7b647cd9112
  scenarios/an-unknown-artifacts-key-is-named.md: 912ce563e772f628884c9096a8fafb00526379e935c99f40d7ca96090d986e1a
  scenarios/project-files-are-measured.md: 81e7abfe526e4441091a21a8ff81ba688f36f3e484acc87f22a3ca6d8397e78a
  spec.md: 0e92f3a4fa8113be8c40afa2a60e6c4a19a903fae3d867e66ca4cb3a943824a1
  tasks.md: 0de61ce805c942b1c3d2c8d0f1c0afca065cd25c4fdc95ad3838fe0c9382d433
unexamined-by-reason:
  root-absent: 4
rule-digest:
  accessibility-frontend.md: 5c1e7a9b5f625f14e6a1879c145ee30e25c5f5aff04ae4c853ee589e21b1efbb
  api-backend.md: 0caddd70fd1a546d80a5e77d254a38c36d158fcad464ee2493491a23cde06ea5
  concurrency-backend.md: 15209bb99ee1c7a63cfcde08a73f6de5df857834776cea43fadb7f0f1040370c
  configuration-cross.md: 2a807ba8facdfd1c3738383e6d44a1f6f2c6f91b387eac0ca5ed90d75dc9e178
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 0522a169c04113521bad128dfef283cd7e910264bf49c4a5320f27ba625fcdee
  security-backend.md: d89d6dad3ab51586ae4e68944b0c8e9ad4d5da721560f10590de3cdecc904d3f
  security-cross.md: dc98261801405fccaf5de9eb8f8ed9b020ebc78fcb5bfb124778a5fcc287950b
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#The result is bounded to one read"}
  BE-ERR-002: {verdict: addressed, cite: "data-model.md#`CheckArtifactSizeArgs`"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#Subjects, absence, and readability"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#Subjects, absence, and readability"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Subjects, absence, and readability"}
  BE-INPUT-008: {verdict: addressed, cite: "plan.md#`/analyze`: the `artifact-size` family and the decided rule"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#The result is bounded to one read"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#The threshold: `[artifacts] read-size-bytes`"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#The threshold: `[artifacts] read-size-bytes`"}
  CFG-CONST-004: {verdict: addressed, cite: "plan.md#The threshold: `[artifacts] read-size-bytes`"}
  CFG-ENV-001: {verdict: addressed, cite: "data-model.md#Configuration"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Documentation"}
  CFG-ENV-005: {verdict: addressed, cite: "plan.md#The threshold: `[artifacts] read-size-bytes`"}
  CFG-ENV-007: {verdict: addressed, cite: "plan.md#The threshold: `[artifacts] read-size-bytes`"}
  CFG-ENV-008: {verdict: addressed, cite: "plan.md#The threshold: `[artifacts] read-size-bytes`"}
  CFG-ENV-012: {verdict: addressed, cite: "scenarios/an-unknown-artifacts-key-is-named.md#Behavior"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 4
  undispositioned: 0
decisions:
  - key: grounding — spec.md says an adopter's 2,200-line plan.md was never read in full, citing no source
    outcome: waived
    reason: motivation that cannot be cited without naming another project, which AGENTS.md forbids; the threshold rests on the 2026-09-28 and 2026-10-04 measurements the spec does cite
    decided-at: 2026-10-04T18:18:21Z
    decided-by: andy@stone.dev
  - key: grounding — Cross-spec impact states spec 081 retired Family 19 and review staleness with no link or citation
    outcome: waived
    reason: Historical aside explaining why 058 owes no review; it asserts no contract this spec depends on, and adding a link would create a dependencies edge for a citation
    decided-at: 2026-11-14T00:36:06Z
    decided-by: rule
  - key: grounding — Signpost describes render-status's collapsing and cutting behavior as fact, carrying only a link to 080 and no file-and-symbol citation
    outcome: waived
    reason: "advisory and no chore: the signpost's link is to 080, the spec that owns render-status' bound and states it in its data model, which is the substantiating artifact §Grounding accepts; a file-and-symbol citation would copy 080's canonical statement into 063"
    decided-at: 2026-11-14T02:02:39Z
    decided-by: rule
  - key: grounding — New 075 bullet states that the shipped hook's comments call steps 3 and 4 read-only and that check-corpus-links writes a list file in .ductus/results/ since 080, naming no file or symbol
    outcome: waived
    reason: "advisory: the bullet records why the hooks' comments stay, and the facts it rests on are cited in the same plan's 'The result is bounded to one read' (bounded_lists::place, .ductus/results/) and in 075's plan, which names the hook's placement comment; adding the file-and-symbol citation here is what this waiver gives up, since editing plan.md now would change a design subject all 310 rule verdicts were judged against"
    decided-at: 2026-11-14T02:02:39Z
    decided-by: rule
---

# Analysis — 063-oversized-artifacts-warn-with-a-fix

## Summary

0 hard-fail, 0 blocking, 4 advisory; not blocking. 4 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 4 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Motivation states as fact that an adopter's 2,200-line plan.md was never read in full and that no command reported an artifact's size, with no citation or marker — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **waived**: motivation that cannot be cited without naming another project, which AGENTS.md forbids; the threshold rests on the 2026-09-28 and 2026-10-04 measurements the spec does cite
- grounding — Signpost describes render-status's collapsing and cutting behavior as fact, carrying only a link to 080 and no file-and-symbol citation — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **waived**: advisory and no chore: the signpost's link is to 080, the spec that owns render-status' bound and states it in its data model, which is the substantiating artifact §Grounding accepts; a file-and-symbol citation would copy 080's canonical statement into 063 (decided by the rule)
- grounding — Cross-spec impact aside states that spec 081 retired Family 19 and review staleness and describes Family 19's sweep exemption, uncited — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — **waived**: Historical aside explaining why 058 owes no review; it asserts no contract this spec depends on, and adding a link would create a dependencies edge for a citation
- grounding — New 075 bullet states that the shipped hook's comments call steps 3 and 4 read-only and that check-corpus-links writes a list file in .ductus/results/ since 080, naming no file or symbol — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — **waived**: advisory: the bullet records why the hooks' comments stay, and the facts it rests on are cited in the same plan's 'The result is bounded to one read' (bounded_lists::place, .ductus/results/) and in 075's plan, which names the hook's placement comment; adding the file-and-symbol citation here is what this waiver gives up, since editing plan.md now would change a design subject all 310 rule verdicts were judged against (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- root-absent: 4

## Fixed in this run

*None.*
