---
spec: 058-findings-route-at-discovery
last-run: 2026-11-22T23:32:34Z
analyzed-against: 34295d615759b35ccc00a2cc8d41d8c34c4f928a
hard-fail: 0
blocking-findings: 0
advisory: 4
unexamined: 2
analyzed-digest:
  data-model.md: 3589826e782428c8941c2ac9aefa262a7ea498d0f6d47e6060caee9b6aae87e0
  plan.md: 8b695e6a5c66f737bab9a86f1e3822d206a0edea38fde4fc92c13a5d1e090ab9
  review.md: ff0b8e3b7124decbdcd3fb040ebc1f697c22fd53ddfbe4559017b421f0a50547
  scenarios/analysis-drift-judges-the-record-it-writes.md: 524d602fab04fdf2e208d9862af69e02fee41fc9f8e9767eccf3b51d3a3c45a9
  scenarios/analyze-findings-match-decisions-by-host-judgment.md: 1ec09d115b84951664fc4d57fb537ee44935b393482a6f28ed43d8b874160575
  scenarios/analyze-state-drift-judges-the-record-it-writes.md: 086887054c718293ed989707272272211d01b3ccaec334cef6f1b89cfca9730e
  scenarios/auto-records-disposition-tasks-without-pausing.md: 79217820badbfd130ba5e69a78927c3d434bcda01e3c66faec2be2b16dcf2c7e
  scenarios/findings-cost-in-proportion-to-risk.md: 908fca8623fc6fbe70a52e491f6f3efec028fdc8df7310fd958dacea9c48db55
  scenarios/only-unreadable-targets-retain-decisions.md: 76a12d565f6e1056f780906fed41de7d27e4a23c314232eddca0563026bcc3e1
  spec.md: 479186ccbc9e37f0ea043234a7a81c497f151fc463d5741499ebfad5a663893b
  tasks.md: 90d3686065261e3359afa1f12420eae92fef6d543ec6f358b5741da6abb0ab6d
unexamined-by-reason:
  anchor-target-unjudgeable: 1
  root-absent: 1
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
  "data-model.md#058 — Findings route at discovery Data Model": 1584ed660dc5cf88
  "data-model.md#Changed primitive inputs": 62de2aafc3946dce
  "data-model.md#New `check-artifacts` family — `disposition-drift`": aa7f8a31ae3568ac
  "data-model.md#New gate blocks — `ReviewGateBlock`": 49f80a8da0ed3867
  "data-model.md#New primitive: `process-decisions`": 74aba2001d40aaf8
  "data-model.md#Removed fields": c110f23cae22c5dd
  "data-model.md#`AnalysisFinding` — the new `write-analysis` `findings` entry": 0aeee6e9fcb0341b
  "data-model.md#`AppendTaskArgs.dedup-title`": d2a6af00f3146dad
  "data-model.md#`DashboardResult.inbox-standing`": 82d7ae33f52578cf
  "data-model.md#`ReviewObservation` — the `write-review` `observations` entry": 9377700d5bc940ff
  "data-model.md#`decisions` — the persisted list, in both records": e11d59a384ad0c27
  "data-model.md#`dispositions` — in both `review.md` and `analysis.md` frontmatter": d267b01782dcb493
  "data-model.md#`write-analysis` args": 29fd97911905be23
  "data-model.md#`write-review` args": 348096e59365d766
  "plan.md": e3b0c44298fc1c14
  "plan.md#058 — Findings route at discovery Plan": 204fa5a45eba33ff
  "plan.md#A new `disposition-drift` family, rather than a new condition inside analyze-state drift": 890edd4795af4fd9
  "plan.md#A new read-only primitive, `process-decisions`, classifies stored decisions before the host proposes": f3247038e6db6462
  "plan.md#Affected Files": 7f5e993df9f3acdd
  "plan.md#Command procedures: the fix-and-route step is host work that `ductus exec` skips": 91d1ba5a863ffe12
  "plan.md#Cross-spec discharge is priced by destination and batched per spec": 8931064bda31bba7
  "plan.md#Data Model": 1c788d092e650251
  "plan.md#Overview": 19e041aebe81004c
  "plan.md#Release": eb6af8a299856cdd
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#The adoption security audit reports instead of capturing": aaab5b9ebd2b3f4a
  "plan.md#The constitution states the rule once, and every command points at it": b1388081be355912
  "plan.md#The gate gains two blocks per record, predates before count": ba2611e683e9ab36
  "plan.md#The inbox template changes, and a migration carries its guidance to existing adopters": e6ad47a7c3a1e816
  "plan.md#The inbox windows and the standing row leave the review and implement primitives": df112ad45fb3adde
  "plan.md#The record shape is owned by the two writers, so the new fields are theirs": c7376ff327e427c8
  "plan.md#Trade-offs": 83b125cb7b7e6862
  "plan.md#`/{project}:status` renders the standing row, and the runtime now renders its strings": e864b9bddb57b8fc
  "plan.md#`append-task` gains an opt-in title dedup, so disposition tasks stay idempotent": ce61b42984a8540c
  "plan.md#`write-analysis` receives the findings, because capture was its only source of finding text": cad5e3f32492a085
  "plan.md#`write-review` stops capturing and starts recording dispositions": 76e9043e8a3f8eaf
  "scenarios/analysis-drift-judges-the-record-it-writes.md": 888060b539543b07
  "scenarios/analysis-drift-judges-the-record-it-writes.md#Analysis-drift-judges-the-record-it-writes": cafb3dbb83e7fb62
  "scenarios/analysis-drift-judges-the-record-it-writes.md#Behavior": 56cb8fe5f05f9bac
  "scenarios/analysis-drift-judges-the-record-it-writes.md#Context": ee7a2fb0872a480c
  "scenarios/analysis-drift-judges-the-record-it-writes.md#Edge Cases": c014d80f452e3caa
  "scenarios/analysis-drift-judges-the-record-it-writes.md#Open Questions": 9fc5979d590cdb97
  "scenarios/analysis-drift-judges-the-record-it-writes.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/analyze-findings-match-decisions-by-host-judgment.md": 91ab4f94e8d86ff6
  "scenarios/analyze-findings-match-decisions-by-host-judgment.md#Analyze-findings-match-decisions-by-host-judgment": 8037a08382373736
  "scenarios/analyze-findings-match-decisions-by-host-judgment.md#Behavior": bd03f6ef022ba7d1
  "scenarios/analyze-findings-match-decisions-by-host-judgment.md#Context": 2df6f6606f72f34e
  "scenarios/analyze-findings-match-decisions-by-host-judgment.md#Edge Cases": c6d6b8c0acf3dce7
  "scenarios/analyze-findings-match-decisions-by-host-judgment.md#Open Questions": 9fc5979d590cdb97
  "scenarios/analyze-findings-match-decisions-by-host-judgment.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/analyze-state-drift-judges-the-record-it-writes.md": 888060b539543b07
  "scenarios/analyze-state-drift-judges-the-record-it-writes.md#Analyze-state-drift-judges-the-record-it-writes": bef8258a1391b7db
  "scenarios/analyze-state-drift-judges-the-record-it-writes.md#Behavior": eb2879491b0f9dde
  "scenarios/analyze-state-drift-judges-the-record-it-writes.md#Context": 1df448aa7f24634a
  "scenarios/analyze-state-drift-judges-the-record-it-writes.md#Edge Cases": c9b155695dcff48c
  "scenarios/analyze-state-drift-judges-the-record-it-writes.md#Open Questions": 9fc5979d590cdb97
  "scenarios/analyze-state-drift-judges-the-record-it-writes.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/auto-records-disposition-tasks-without-pausing.md": 4e281a179654381b
  "scenarios/auto-records-disposition-tasks-without-pausing.md#Auto-records-disposition-tasks-without-pausing": 6f5cb4092efce2e1
  "scenarios/auto-records-disposition-tasks-without-pausing.md#Behavior": 2f7dc4a7835fa193
  "scenarios/auto-records-disposition-tasks-without-pausing.md#Context": d1fab3f899530e88
  "scenarios/auto-records-disposition-tasks-without-pausing.md#Edge Cases": 28f02c7d5029a1e4
  "scenarios/auto-records-disposition-tasks-without-pausing.md#Open Questions": 9fc5979d590cdb97
  "scenarios/auto-records-disposition-tasks-without-pausing.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/findings-cost-in-proportion-to-risk.md": 5561946539d1b6ef
  "scenarios/findings-cost-in-proportion-to-risk.md#Behavior": 67a016229292e8cf
  "scenarios/findings-cost-in-proportion-to-risk.md#Context": 5120d04b5c23b27d
  "scenarios/findings-cost-in-proportion-to-risk.md#Edge Cases": 3672633c64b2a2b3
  "scenarios/findings-cost-in-proportion-to-risk.md#Findings Cost in Proportion to Risk": 41fbc822c7b2527d
  "scenarios/findings-cost-in-proportion-to-risk.md#Open Questions": 35737a7fd1b7cc55
  "scenarios/findings-cost-in-proportion-to-risk.md#Resolved Questions": 96256baf65f08f21
  "scenarios/only-unreadable-targets-retain-decisions.md": 91ab4f94e8d86ff6
  "scenarios/only-unreadable-targets-retain-decisions.md#Behavior": 75fa3cf7d94631de
  "scenarios/only-unreadable-targets-retain-decisions.md#Context": e9c42a0893b61629
  "scenarios/only-unreadable-targets-retain-decisions.md#Edge Cases": 7bcdad162067371b
  "scenarios/only-unreadable-targets-retain-decisions.md#Only-unreadable-targets-retain-decisions": b6ae028a1f8e07ea
  "scenarios/only-unreadable-targets-retain-decisions.md#Open Questions": 9fc5979d590cdb97
  "scenarios/only-unreadable-targets-retain-decisions.md#Resolved Questions": b5f7b98fb3491576
  "spec.md": 9cff80829e65c4a9
  "spec.md#058 — Findings route at discovery": 75f11f58916d7386
  "spec.md#Acceptance Criteria": 8858b217daee0a2f
  "spec.md#Behavior": 29ab98cdbf8ff7c1
  "spec.md#Constitution": dac33958694065bc
  "spec.md#Cross-spec impact": c7f8a3c91facb0c4
  "spec.md#Decisions persist across runs": 89f90e6d06d0ea60
  "spec.md#Edge Cases": e51fc59b036d59c0
  "spec.md#Motivation": 6e72cfcb7ceb67af
  "spec.md#Only gating findings hold `done`": 749227c6b4e4cbac
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Order within a run": 37b5449fffb4c2b2
  "spec.md#Resolved Questions": c61fe344abf49e33
  "spec.md#Run records": 26ef78fee4800bef
  "spec.md#Runtime": 91217a2c12d1a78c
  "spec.md#The inbox": c25e5750b98a676a
  "spec.md#Three dispositions": f6b40995221377fb
  "spec.md#`/ductus` adoption security audit": b004d78cbe9633a8
  "spec.md#`/{project}:analyze`": 2991889ac656a9ec
  "spec.md#`/{project}:implement`": 00d63946a0443069
  "spec.md#`/{project}:review`": 192978f82c7f0e22
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#Trade-offs"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#A new read-only primitive, `process-decisions`, classifies stored decisions before the host proposes"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#`ReviewObservation` — the `write-review` `observations` entry"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#A new read-only primitive, `process-decisions`, classifies stored decisions before the host proposes"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#A new read-only primitive, `process-decisions`, classifies stored decisions before the host proposes"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#The record shape is owned by the two writers, so the new fields are theirs"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "data-model.md#`dispositions` — in both `review.md` and `analysis.md` frontmatter"}
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 4
  undispositioned: 0
decisions:
  - key: grounding — plan.md says .ductus/bin/ductus is a symlink to runtime/target/release/ductus, local state no artifact grounds
    outcome: waived
    reason: it describes this checkout's runtime link behind the Overview's landing order; every runtime task has landed, and no artifact or check reads it
    decided-at: 2026-11-10T02:30:20Z
    decided-by: rule
  - key: "decision-reasons — plan.md Trade-off 'Known limitation: 022's full re-review is not bought here' passes over an option and gives no reason beyond what the review run would cost (low confidence)"
    outcome: waived
    reason: the line bounds this plan's scope, naming another spec's re-review as that spec's decision to spend; it rejects no option for the review it would start
    decided-at: 2026-11-14T03:42:00Z
    decided-by: rule
  - key: grounding — Motivation states as fact that `check-review-gate` blocked on unchecked tasks, unresolved scenario questions, an undischarged fold or cross-spec impact, and a missing, blocking or stale record, with no file-and-symbol citation, command or assumption marker.
    outcome: waived
    reason: "advisory and no chore: the Motivation records what the gate blocked on before 058, the problem the spec was written to solve; git keeps that state, and the current gate's checks are its compiled description, which the spec's criteria are judged against"
    decided-at: 2026-11-22T23:32:34Z
    decided-by: rule
  - key: grounding — Resolved Questions (disposition counts question) records that the upgrade cost was measured at zero because all 55 specs were `done` and none `in-progress`, stating no command, query or date, so the measurement cannot be re-derived.
    outcome: waived
    reason: "advisory and no chore: a Resolved Question's reason, the corpus state when the upgrade's cost was decided; that state is git's record, and no current behavior rests on the count"
    decided-at: 2026-11-22T23:32:34Z
    decided-by: rule
---

# Analysis — 058-findings-route-at-discovery

## Summary

0 hard-fail, 0 blocking, 4 advisory; not blocking. 2 unexamined target(s). Dispositions: 1 fixed, 0 deferred, 4 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Motivation states as fact that `check-review-gate` blocked on unchecked tasks, unresolved scenario questions, an undischarged fold or cross-spec impact, and a missing, blocking or stale record, with no file-and-symbol citation, command or assumption marker. — `specs/058-findings-route-at-discovery/spec.md` — **waived**: advisory and no chore: the Motivation records what the gate blocked on before 058, the problem the spec was written to solve; git keeps that state, and the current gate's checks are its compiled description, which the spec's criteria are judged against (decided by the rule)
- grounding — Resolved Questions (disposition counts question) records that the upgrade cost was measured at zero because all 55 specs were `done` and none `in-progress`, stating no command, query or date, so the measurement cannot be re-derived. — `specs/058-findings-route-at-discovery/spec.md` — **waived**: advisory and no chore: a Resolved Question's reason, the corpus state when the upgrade's cost was decided; that state is git's record, and no current behavior rests on the count (decided by the rule)
- grounding — Overview states as fact that `.ductus/bin/ductus` is a symlink to `runtime/target/release/ductus`, citing no file, command or artifact. — `specs/058-findings-route-at-discovery/plan.md` — **waived**: it describes this checkout's runtime link behind the Overview's landing order; every runtime task has landed, and no artifact or check reads it (decided by the rule)
- decision-reasons — Trade-offs 'Known limitation: 022's full re-review is not bought here' (with Cross-spec discharge's 'remains its own decision to spend') passes over a full re-review of 022 with the review run itself as the only stated reason and no statement of the result given up; low confidence, since it is framed as a limitation rather than a rejected option. — `specs/058-findings-route-at-discovery/plan.md` — **waived**: the line bounds this plan's scope, naming another spec's re-review as that spec's decision to spend; it rejects no option for the review it would start (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 1
- root-absent: 1

## Fixed in this run

- grounding — Edge Cases state as fact that `create-scenario` refuses a slug that already exists, citing no source (primitive description, test or file). — `specs/058-findings-route-at-discovery/spec.md` — **fixed** (decided by the rule)
