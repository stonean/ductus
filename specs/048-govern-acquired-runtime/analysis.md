---
spec: 048-govern-acquired-runtime
last-run: 2026-11-22T04:42:43Z
analyzed-against: ae1fa21bfc8af9ce4291ce1c6c0ed52b9b63eaf7
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 19
analyzed-digest:
  data-model.md: bdf3bb0f65012793f70ad57a26ac55c823ceaf42106452501094c8db8a4e04d0
  plan.md: 95564e2aace85fb109471dd3043384b410dc67aa320853e1e4a96a95dbcf2b3e
  review.md: 6776dd53aff1428f387c2f68bc7a3e14f76cd2b747e0d59ddeb5e8091129746e
  scenarios/fetch-archive-reads-its-proxy-once.md: a14dcf0651874295a292eb8a0f56e009f27cc4ded857a7b55283755c1bebef1a
  scenarios/pin-is-readable-when-acquisition-needs-it.md: fb1368d6a85c6291628b23755e70841e55e1919235e7cbda6393f9e8402ae518
  scenarios/pointer-follows-the-store.md: 350974b3c619868c66f1c5459fbe94d7d0a3c88a750bbf5fcb4cdef963709e5f
  scenarios/release-halves-publish-together.md: 0448ad2cb2df94e5d6b97ce4660345e26619af68d5f05da29d8b8dd8a011306f
  scenarios/retired-namespace-tools-are-off-limits.md: 5ef3496d8bdb180f65970682d1070cd217acd391127cfbf05fa8ffd7032be637
  scenarios/state-a-version-checks-the-pin.md: af9f474c87d09ad7ddb9f1433e04fc42574fe5f4900375268e2db3aaadeff389
  scenarios/state-b-continues-in-session.md: eaf650d5444e8d060e46a97668779bfcf648b086b77bb6ba881afca0759f947a
  spec.md: 4ebc468fffb2f24b51c06fb2da18fe9c923aebaacc318fc79404a9f28b568e11
  tasks.md: 5e5fcbbdcf9102435e5aecd0adbd34318878e7891b3a801e99c7c1dcd25c4b14
unexamined-by-reason:
  not-a-live-claim: 5
  root-absent: 14
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
  "data-model.md#048 — Ductus-Acquired Runtime Data Model": 2986995e14ed5575
  "data-model.md#Acquisition command set": 986d805e015fe2a1
  "data-model.md#Detection states": 70f22fd0d9a7a67b
  "data-model.md#Filesystem layout": 633d5cd0e3eba799
  "data-model.md#Release assets": a5b31c546ac5a9e0
  "data-model.md#The `[runtime]` config section": 8554de59849599ee
  "data-model.md#The `version` file": 8920800682d8a096
  "plan.md": e3b0c44298fc1c14
  "plan.md#048 — Ductus-Acquired Runtime Plan": 2a7c58f6b21407eb
  "plan.md#Acquisition is host shell work, and that is not a workaround": 0bfeba22e34f8984
  "plan.md#Affected Files": ed93d811cffd9dc7
  "plan.md#Detection states collapse from three to two": b5245acedb6b626b
  "plan.md#Overview": 8c1a50153c07a451
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#The amendment and the sweep land together": 7fa21b2891df3fb9
  "plan.md#The pin lives in a repo-root `version` file": 4e68adea72c8708a
  "plan.md#The pointer: symlink attempted, copy on failure, compared on every run": e4c6063490ecb771
  "plan.md#The release surface changes are independently landable": 150ef63f3f6ec003
  "plan.md#The version probe executes the binary": f900a3b1225bc8b8
  "plan.md#Trade-offs": 5ab50537361d587f
  "plan.md#`fetch-archive`'s HTTP configuration is read once": 9b80cf17657a3a9a
  "scenarios/fetch-archive-reads-its-proxy-once.md": c9feb2732316d562
  "scenarios/fetch-archive-reads-its-proxy-once.md#Behavior": a45402c0116004df
  "scenarios/fetch-archive-reads-its-proxy-once.md#Context": 0fcbce25adef7501
  "scenarios/fetch-archive-reads-its-proxy-once.md#Edge Cases": 1c2c9a8448017c3a
  "scenarios/fetch-archive-reads-its-proxy-once.md#Fetch-archive-reads-its-proxy-once": 5bf96dbdf1f81c02
  "scenarios/fetch-archive-reads-its-proxy-once.md#Open Questions": 20edda8881516e4e
  "scenarios/fetch-archive-reads-its-proxy-once.md#Resolved Questions": 26a3da198569d977
  "scenarios/pin-is-readable-when-acquisition-needs-it.md": c9feb2732316d562
  "scenarios/pin-is-readable-when-acquisition-needs-it.md#Behavior": 502a8ac116a7656f
  "scenarios/pin-is-readable-when-acquisition-needs-it.md#Context": 69bf3bfaef11c464
  "scenarios/pin-is-readable-when-acquisition-needs-it.md#Edge Cases": 931e5d05c51a3ee6
  "scenarios/pin-is-readable-when-acquisition-needs-it.md#Open Questions": 9fc5979d590cdb97
  "scenarios/pin-is-readable-when-acquisition-needs-it.md#Pin-is-readable-when-acquisition-needs-it": 779c6bac27524e16
  "scenarios/pin-is-readable-when-acquisition-needs-it.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/pointer-follows-the-store.md": b67e8da28f51b0fe
  "scenarios/pointer-follows-the-store.md#Behavior": c2cc330e930f3a04
  "scenarios/pointer-follows-the-store.md#Context": 988bb60bed228992
  "scenarios/pointer-follows-the-store.md#Edge Cases": 4befce48a0a15d0b
  "scenarios/pointer-follows-the-store.md#Open Questions": 9fc5979d590cdb97
  "scenarios/pointer-follows-the-store.md#Pointer-follows-the-store": 17cad6394ab88f0d
  "scenarios/pointer-follows-the-store.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/release-halves-publish-together.md": c9feb2732316d562
  "scenarios/release-halves-publish-together.md#Behavior": 8422a0428ef35307
  "scenarios/release-halves-publish-together.md#Context": d8c7dcde3cde2222
  "scenarios/release-halves-publish-together.md#Edge Cases": 083791568a02bf70
  "scenarios/release-halves-publish-together.md#Open Questions": 9fc5979d590cdb97
  "scenarios/release-halves-publish-together.md#Release-halves-publish-together": bc608b0fee9a9459
  "scenarios/release-halves-publish-together.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/retired-namespace-tools-are-off-limits.md": f4036d4aa3763d17
  "scenarios/retired-namespace-tools-are-off-limits.md#Behavior": a7dabd963703f296
  "scenarios/retired-namespace-tools-are-off-limits.md#Context": cf687e6e8b69038c
  "scenarios/retired-namespace-tools-are-off-limits.md#Edge Cases": 293757efa52420f6
  "scenarios/retired-namespace-tools-are-off-limits.md#Open Questions": 9fc5979d590cdb97
  "scenarios/retired-namespace-tools-are-off-limits.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/retired-namespace-tools-are-off-limits.md#Retired-namespace-tools-are-off-limits": 8dc8778de330e010
  "scenarios/state-a-version-checks-the-pin.md": c9feb2732316d562
  "scenarios/state-a-version-checks-the-pin.md#Behavior": 6ff388d4a70ffe46
  "scenarios/state-a-version-checks-the-pin.md#Context": fc31c467c0f567f7
  "scenarios/state-a-version-checks-the-pin.md#Edge Cases": f109d870013a74a6
  "scenarios/state-a-version-checks-the-pin.md#Open Questions": 9fc5979d590cdb97
  "scenarios/state-a-version-checks-the-pin.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/state-a-version-checks-the-pin.md#State-a-version-checks-the-pin": e3d61641b998663c
  "scenarios/state-b-continues-in-session.md": f4036d4aa3763d17
  "scenarios/state-b-continues-in-session.md#Behavior": 28b844138b461bf3
  "scenarios/state-b-continues-in-session.md#Context": 7ca8ddf51585a349
  "scenarios/state-b-continues-in-session.md#Edge Cases": effdfd20bd4ea653
  "scenarios/state-b-continues-in-session.md#Open Questions": a474af12b9f56e5a
  "scenarios/state-b-continues-in-session.md#Resolved Questions": bf3dadf44e62219a
  "scenarios/state-b-continues-in-session.md#State-b-continues-in-session": 06ee6c7e57324078
  "spec.md": 2c92d97c2eb1f7cb
  "spec.md#048 — Ductus-Acquired Runtime": 5885131f8a176fcc
  "spec.md#Acceptance Criteria": 95bcde7252643125
  "spec.md#Acquisition": 31f3ca92b4aa4fc8
  "spec.md#Adopter migration": 940bc19f05415b7d
  "spec.md#Detection states": 9e5df063f3f8b7c2
  "spec.md#Edge Cases": 6b98d92f7625a091
  "spec.md#Install location": d5209ae8a6e3c292
  "spec.md#MCP registration": cf1dacf3f40a7158
  "spec.md#Motivation": d149a66b688b2ae7
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Reference migration": ef0d21b254ead23f
  "spec.md#Resolved Questions": f4f196e7aac341db
  "spec.md#Runtime requirement": 76ea80e7a8315066
  "spec.md#Version currency, and what the store gives up": 4f7307853d330ba7
rule-verdicts:
  BE-COORD-002: {verdict: addressed, cite: "spec.md#Acquisition"}
  BE-DATA-001: {verdict: addressed, cite: "spec.md#Acquisition"}
  BE-DATA-003: {verdict: addressed, cite: "plan.md#`fetch-archive`'s HTTP configuration is read once"}
  BE-DATA-011: {verdict: addressed, cite: "spec.md#Acquisition"}
  BE-DEPS-001: {verdict: addressed, cite: "plan.md#Affected Files"}
  BE-DEPS-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  BE-DEPS-003: {verdict: addressed, cite: "data-model.md#Release assets"}
  BE-DEPS-004: {verdict: addressed, cite: "plan.md#Affected Files"}
  BE-DEPS-005: {verdict: addressed, cite: "plan.md#Affected Files"}
  BE-DEPS-007: {verdict: addressed, cite: "plan.md#Affected Files"}
  BE-ERR-005: {verdict: addressed, cite: "scenarios/pointer-follows-the-store.md#Edge Cases"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#Acquisition is host shell work, and that is not a workaround"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#Acquisition is host shell work, and that is not a workaround"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#Acquisition is host shell work, and that is not a workaround"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Acquisition is host shell work, and that is not a workaround"}
  BE-INPUT-007: {verdict: addressed, cite: "scenarios/fetch-archive-reads-its-proxy-once.md#Behavior"}
  BE-INPUT-008: {verdict: addressed, cite: "scenarios/state-b-continues-in-session.md#Behavior"}
  BE-INPUT-011: {verdict: addressed, cite: "scenarios/state-a-version-checks-the-pin.md#Edge Cases"}
  BE-INPUT-016: {verdict: addressed, cite: "scenarios/fetch-archive-reads-its-proxy-once.md#Behavior"}
  BE-LOCK-001: {verdict: addressed, cite: "spec.md#Edge Cases"}
  BE-LOG-002: {verdict: addressed, cite: "plan.md#`fetch-archive`'s HTTP configuration is read once"}
  BE-RACE-001: {verdict: addressed, cite: "plan.md#`fetch-archive`'s HTTP configuration is read once"}
  BE-RACE-002: {verdict: addressed, cite: "plan.md#`fetch-archive`'s HTTP configuration is read once"}
  BE-RACE-003: {verdict: addressed, cite: "spec.md#Edge Cases"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#`fetch-archive`'s HTTP configuration is read once"}
  CFG-CONST-001: {verdict: addressed, cite: "spec.md#Acquisition"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#`fetch-archive`'s HTTP configuration is read once"}
  CFG-ENV-001: {verdict: addressed, cite: "plan.md#`fetch-archive`'s HTTP configuration is read once"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  CFG-ENV-006: {verdict: addressed, cite: "plan.md#`fetch-archive`'s HTTP configuration is read once"}
  CFG-ENV-008: {verdict: addressed, cite: "plan.md#`fetch-archive`'s HTTP configuration is read once"}
  CFG-ENV-009: {verdict: addressed, cite: "plan.md#Acquisition is host shell work, and that is not a workaround"}
  CFG-ENV-012: {verdict: addressed, cite: "data-model.md#The `[runtime]` config section"}
  SEC-DEPS-001: {verdict: addressed, cite: "plan.md#Affected Files"}
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: artifact-size — `specs/048-govern-acquired-runtime/spec.md` is 51886 bytes, 2 read pages at the 50000-byte read size
    outcome: waived
    reason: Bringing the spec under the read size means cutting required content by judgment, since its Resolved Questions alone hold about 17 KB of recorded decisions, and no slice of 048 reaches done on its own to split off; an agent reads it in two calls, the read-size rule's stated fallback.
    decided-at: 2026-11-19T14:20:03Z
    decided-by: rule
  - key: "grounding — Technical Decisions, 'Acquisition is host shell work': 'a cross-host redirect defeats a pre-granted curl permission' is stated as fact about the host permission grammar, citing only that ductus.md contains the precedent, with no file-and-symbol, command or measurement behind the redirect claim itself."
    outcome: waived
    reason: a claim about Claude Code's permission matching, an external host's behavior the plan relies on; grounding it needs the host's documentation fetched and read, so the fix is not mechanical, and the advisory holds nothing
    decided-at: 2026-11-22T04:42:43Z
    decided-by: rule
---

# Analysis — 048-govern-acquired-runtime

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 19 unexamined target(s). Dispositions: 1 fixed, 0 deferred, 2 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Technical Decisions, 'Acquisition is host shell work': 'a cross-host redirect defeats a pre-granted curl permission' is stated as fact about the host permission grammar, citing only that ductus.md contains the precedent, with no file-and-symbol, command or measurement behind the redirect claim itself. — `specs/048-govern-acquired-runtime/plan.md` — **waived**: a claim about Claude Code's permission matching, an external host's behavior the plan relies on; grounding it needs the host's documentation fetched and read, so the fix is not mechanical, and the advisory holds nothing (decided by the rule)
- artifact-size — `specs/048-govern-acquired-runtime/spec.md` is 51886 bytes, 2 read pages at the 50000-byte read size — `specs/048-govern-acquired-runtime/spec.md` — **waived**: Bringing the spec under the read size means cutting required content by judgment, since its Resolved Questions alone hold about 17 KB of recorded decisions, and no slice of 048 reaches done on its own to split off; an agent reads it in two calls, the read-size rule's stated fallback. (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 5
- root-absent: 14

## Fixed in this run

- rule-ids — plan.md's Affected Files row for runtime/Cargo.toml and runtime/Cargo.lock cited BE-DEPS-006, removed in 5ee93f96 (008, 2026-11-19), and omitted BE-DEPS-005, which 022's plan commits to; corrected to BE-DEPS-001 to 005 and 007 in ae1fa21b. — `specs/048-govern-acquired-runtime/plan.md` — **fixed** (decided by the rule)
