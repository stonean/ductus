---
spec: 048-govern-acquired-runtime
last-run: 2026-11-19T14:20:03Z
analyzed-against: 1f345e200a8de38c127446420eb4227c01bcad4f
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 19
analyzed-digest:
  data-model.md: bdf3bb0f65012793f70ad57a26ac55c823ceaf42106452501094c8db8a4e04d0
  plan.md: 6cb7ce305642864842a0ea037447675b64b8b798450afb3f54f27c36c30fe5a9
  review.md: 55a76594ff384dc19f57ac253fd0a7ffe80253e6fae8c3ebce384e5fdc19895b
  scenarios/fetch-archive-reads-its-proxy-once.md: a14dcf0651874295a292eb8a0f56e009f27cc4ded857a7b55283755c1bebef1a
  scenarios/pin-is-readable-when-acquisition-needs-it.md: fb1368d6a85c6291628b23755e70841e55e1919235e7cbda6393f9e8402ae518
  scenarios/pointer-follows-the-store.md: 366957d76f4403d0a34aa662dd034802d1f18da93d6c1e8a4d6883786960101a
  scenarios/release-halves-publish-together.md: 0448ad2cb2df94e5d6b97ce4660345e26619af68d5f05da29d8b8dd8a011306f
  scenarios/retired-namespace-tools-are-off-limits.md: 5ef3496d8bdb180f65970682d1070cd217acd391127cfbf05fa8ffd7032be637
  scenarios/state-a-version-checks-the-pin.md: af9f474c87d09ad7ddb9f1433e04fc42574fe5f4900375268e2db3aaadeff389
  scenarios/state-b-continues-in-session.md: eaf650d5444e8d060e46a97668779bfcf648b086b77bb6ba881afca0759f947a
  spec.md: 4ebc468fffb2f24b51c06fb2da18fe9c923aebaacc318fc79404a9f28b568e11
  tasks.md: 25ed7db979185fdc3b203dc8d776277f08de94b566e647ea7aa5498d2234c0ad
unexamined-by-reason:
  not-a-live-claim: 5
  root-absent: 14
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
  security-backend.md: 9873cc885c4bdc76eff79a7269a5a6858a0b84c3010ff4b1bc27e7989f708183
  security-cross.md: bf70990f24e7e67926772be0d95deb7e1c1492bc12bdcb650a23b25385bcd285
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-BULK-002: {verdict: addressed, cite: "spec.md#Acquisition"}
  BE-COMPAT-001: {verdict: addressed, cite: "scenarios/state-b-continues-in-session.md#Behavior"}
  BE-DATA-001: {verdict: addressed, cite: "spec.md#Acquisition"}
  BE-DATA-011: {verdict: addressed, cite: "spec.md#Acquisition"}
  BE-DEPS-001: {verdict: addressed, cite: "plan.md#Affected Files"}
  BE-DEPS-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  BE-DEPS-003: {verdict: addressed, cite: "data-model.md#Release assets"}
  BE-DEPS-004: {verdict: addressed, cite: "plan.md#Affected Files"}
  BE-DEPS-007: {verdict: addressed, cite: "plan.md#Affected Files"}
  BE-ERR-005: {verdict: addressed, cite: "spec.md#Acquisition"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#Acquisition is host shell work, and that is not a workaround"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#Acquisition is host shell work, and that is not a workaround"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#Acquisition is host shell work, and that is not a workaround"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Acquisition is host shell work, and that is not a workaround"}
  BE-INPUT-007: {verdict: addressed, cite: "scenarios/fetch-archive-reads-its-proxy-once.md#Behavior"}
  BE-INPUT-008: {verdict: addressed, cite: "scenarios/state-b-continues-in-session.md#Behavior"}
  BE-INPUT-011: {verdict: addressed, cite: "scenarios/state-a-version-checks-the-pin.md#Edge Cases"}
  BE-INPUT-016: {verdict: addressed, cite: "scenarios/fetch-archive-reads-its-proxy-once.md#Behavior"}
  BE-LOCK-001: {verdict: addressed, cite: "spec.md#Edge Cases"}
  BE-RACE-001: {verdict: addressed, cite: "plan.md#`fetch-archive`'s HTTP configuration is read once"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#`fetch-archive`'s HTTP configuration is read once"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "spec.md#Acquisition"}
  BE-TIMEOUT-002: {verdict: addressed, cite: "plan.md#`fetch-archive`'s HTTP configuration is read once"}
  CFG-CONST-001: {verdict: addressed, cite: "spec.md#Acquisition"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#`fetch-archive`'s HTTP configuration is read once"}
  CFG-CONST-003: {verdict: addressed, cite: "spec.md#Acquisition"}
  CFG-ENV-001: {verdict: addressed, cite: "plan.md#`fetch-archive`'s HTTP configuration is read once"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  CFG-ENV-006: {verdict: addressed, cite: "plan.md#`fetch-archive`'s HTTP configuration is read once"}
  CFG-ENV-007: {verdict: addressed, cite: "scenarios/fetch-archive-reads-its-proxy-once.md#Resolved Questions"}
  CFG-ENV-008: {verdict: addressed, cite: "scenarios/fetch-archive-reads-its-proxy-once.md#Edge Cases"}
  CFG-ENV-012: {verdict: addressed, cite: "data-model.md#The `[runtime]` config section"}
  SEC-DEPS-001: {verdict: addressed, cite: "plan.md#Affected Files"}
blocking: false
dispositions:
  fixed: 12
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: artifact-size — `specs/048-govern-acquired-runtime/spec.md` is 51886 bytes, 2 read pages at the 50000-byte read size
    outcome: waived
    reason: Bringing the spec under the read size means cutting required content by judgment, since its Resolved Questions alone hold about 17 KB of recorded decisions, and no slice of 048 reaches done on its own to split off; an agent reads it in two calls, the read-size rule's stated fallback.
    decided-at: 2026-11-19T14:20:03Z
    decided-by: rule
---

# Analysis — 048-govern-acquired-runtime

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 19 unexamined target(s). Dispositions: 12 fixed, 0 deferred, 1 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- artifact-size — `specs/048-govern-acquired-runtime/spec.md` is 51886 bytes, 2 read pages at the 50000-byte read size — `specs/048-govern-acquired-runtime/spec.md` — **waived**: Bringing the spec under the read size means cutting required content by judgment, since its Resolved Questions alone hold about 17 KB of recorded decisions, and no slice of 048 reaches done on its own to split off; an agent reads it in two calls, the read-size rule's stated fallback. (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 5
- root-absent: 14

## Fixed in this run

- grounding — Motivation said the runtime had published 52 releases in three months, with no source; it now cites `runtime/CHANGELOG.md`'s dated headings, 0.1.0 to 0.27.2, and 0.15.0's breaking `create-scenario` change (§Grounding (advisory)). — `specs/048-govern-acquired-runtime/spec.md` — **fixed** (decided by the rule)
- grounding — Acquisition said GitHub's auto-generated source tarballs ship without a sidecar, with no source; it now cites GitHub Docs, *Downloading source code archives* (§Grounding (advisory)). — `specs/048-govern-acquired-runtime/spec.md` — **fixed** (decided by the rule)
- grounding — A Resolved Question said the Windows asset relied on the bsdtar Windows 10+ ships, with no source; it now cites acquisition run `44432479285`'s Windows leg (§Grounding (advisory)). — `specs/048-govern-acquired-runtime/spec.md` — **fixed** (decided by the rule)
- grounding — Motivation said `ships-to-adopter` lived in the runtime for a full release with no reference counterpart, with no source; it now cites `6e37efa5` and `4bba52d0`, five releases apart (§Grounding (advisory)). — `specs/048-govern-acquired-runtime/spec.md` — **fixed** (decided by the rule)
- grounding — An Edge Case said `Move-Item -Force` fails over a running store on Windows, with no run; it now cites the measurement's `store-running-powershell` row in Windows job 131593547621 (§Grounding (advisory)). — `specs/048-govern-acquired-runtime/spec.md` — **fixed** (decided by the rule)
- grounding — An Edge Case said `MSYS=winsymlinks:native` makes a real symlink, with no run; it now cites `pointer-control-ln-native` in the same job (§Grounding (advisory)). — `specs/048-govern-acquired-runtime/spec.md` — **fixed** (decided by the rule)
- grounding — A Resolved Question said curl, tar and five more were already granted in the seed of the time, naming no revision; it now names `framework/bootstrap/govern.md` at `da7c45c8` (§Grounding (advisory)). — `specs/048-govern-acquired-runtime/spec.md` — **fixed** (decided by the rule)
- grounding — A Resolved Question said every MCP call ran a binary predating `label-criteria`, with no source; it now cites the clarification `da7c45c8` and `8c0e5b20` (§Grounding (advisory)). — `specs/048-govern-acquired-runtime/spec.md` — **fixed** (decided by the rule)
- grounding — Three Resolved Questions sentences said Pi has no permission prompts or gate, with no source; each now cites `064-pi-host-support`'s host survey (§Grounding (advisory)). — `specs/048-govern-acquired-runtime/spec.md` — **fixed** (decided by the rule)
- grounding — Motivation said no mechanism enforced the pin, a negative claim with no source; it now says the bootstrap at `da7c45c8` never ran the runtime's `--version` (§Grounding (advisory)). — `specs/048-govern-acquired-runtime/spec.md` — **fixed** (decided by the rule)
- grounding — A Trade-off said nothing in CI proves an adopter's pointer is correct, naming no workflow; it now says no workflow reads an adopter's pointer and what `runtime.yml` measures instead (§Grounding (advisory)). — `specs/048-govern-acquired-runtime/plan.md` — **fixed** (decided by the rule)
- grounding — Technical Decisions said Pi, which has no permission gate, needs none, with no source; it now cites `064-pi-host-support`'s host survey (§Grounding (advisory)). — `specs/048-govern-acquired-runtime/plan.md` — **fixed** (decided by the rule)
