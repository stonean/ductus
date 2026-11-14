---
spec: 058-findings-route-at-discovery
last-run: 2026-11-14T14:16:37Z
analyzed-against: 6310d4b8069123780a9692fb46897f58332eedd5
hard-fail: 0
blocking-findings: 0
advisory: 5
unexamined: 1
analyzed-digest:
  data-model.md: 3c0dd92608a493ee61d3bb7e078e77af339958045ab269ae5390016178d99af1
  plan.md: b3aae07018b46f59269faf5a46f3dfee64f18f86a38620b4199097511bfa0100
  review.md: 4b6dbf1296ca7b2bbbd65457ea2fef1f8c03634996a6f6c7266f366ce42728a9
  scenarios/analysis-drift-judges-the-record-it-writes.md: 524d602fab04fdf2e208d9862af69e02fee41fc9f8e9767eccf3b51d3a3c45a9
  scenarios/analyze-findings-match-decisions-by-host-judgment.md: 1ec09d115b84951664fc4d57fb537ee44935b393482a6f28ed43d8b874160575
  scenarios/analyze-state-drift-judges-the-record-it-writes.md: 086887054c718293ed989707272272211d01b3ccaec334cef6f1b89cfca9730e
  scenarios/auto-records-disposition-tasks-without-pausing.md: 79217820badbfd130ba5e69a78927c3d434bcda01e3c66faec2be2b16dcf2c7e
  scenarios/findings-cost-in-proportion-to-risk.md: 908fca8623fc6fbe70a52e491f6f3efec028fdc8df7310fd958dacea9c48db55
  scenarios/only-unreadable-targets-retain-decisions.md: 76a12d565f6e1056f780906fed41de7d27e4a23c314232eddca0563026bcc3e1
  spec.md: ca9e0a9394bdd01650872b4c38c3f30330788f9840bf5ab7216928ef80feaf61
  tasks.md: 90d3686065261e3359afa1f12420eae92fef6d543ec6f358b5741da6abb0ab6d
unexamined-by-reason:
  root-absent: 1
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
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#Trade-offs"}
  BE-DEPS-007: {verdict: finding, tier: should, at: "plan.md#Release", message: "plan changes runtime/Cargo.lock (release refresh) but commits to no automated vulnerability scan on changes to the lockfile"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#`write-analysis` receives the findings, because capture was its only source of finding text"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#`AnalysisFinding` — the new `write-analysis` `findings` entry"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#A new read-only primitive, `process-decisions`, classifies stored decisions before the host proposes"}
  BE-INPUT-008: {verdict: addressed, cite: "plan.md#The record shape is owned by the two writers, so the new fields are theirs"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#A new read-only primitive, `process-decisions`, classifies stored decisions before the host proposes"}
  BE-SCHEMA-002: {verdict: addressed, cite: "plan.md#The record shape is owned by the two writers, so the new fields are theirs"}
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 5
  undispositioned: 0
decisions:
  - key: grounding — plan.md says .ductus/bin/ductus is a symlink to runtime/target/release/ductus, local state no artifact grounds
    outcome: waived
    reason: it describes this checkout's runtime link behind the Overview's landing order; every runtime task has landed, and no artifact or check reads it
    decided-at: 2026-11-10T02:30:20Z
    decided-by: rule
  - key: grounding — plan.md says 022's 95 other scenarios are not re-read, a count with no method
    outcome: waived
    reason: the count priced 022's discharge review when the plan was written; that discharge has landed, so the figure gates nothing, and re-deriving it is a measurement, not a mechanical edit
    decided-at: 2026-11-10T02:30:20Z
    decided-by: rule
  - key: grounding — plan.md says dedup-prefix is refused by name on MCP and the CLI without a source
    outcome: waived
    reason: the refusal is the MCP server's and the CLI's unknown-argument handling, which the data model states beside it; checking it means reading code, which is the review's work, not the plan's grounding
    decided-at: 2026-11-10T02:30:20Z
    decided-by: rule
  - key: "decision-reasons — plan.md Trade-off 'Known limitation: 022's full re-review is not bought here' passes over an option and gives no reason beyond what the review run would cost (low confidence)"
    outcome: waived
    reason: the line bounds this plan's scope, naming another spec's re-review as that spec's decision to spend; it rejects no option for the review it would start
    decided-at: 2026-11-14T03:42:00Z
    decided-by: rule
  - key: "rule-assessment — BE-DEPS-007: plan changes runtime/Cargo.lock (release refresh) but commits to no automated vulnerability scan on changes to the lockfile"
    outcome: waived
    reason: the lockfile change is the release's own version refresh (plan.md §Release), which alters no dependency, and .github/workflows/runtime.yml already runs cargo audit on every change under runtime/, Cargo.lock included
    decided-at: 2026-11-14T14:16:37Z
    decided-by: rule
---

# Analysis — 058-findings-route-at-discovery

## Summary

0 hard-fail, 0 blocking, 5 advisory; not blocking. 1 unexamined target(s). Dispositions: 1 fixed, 0 deferred, 5 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-DEPS-007: plan changes runtime/Cargo.lock (release refresh) but commits to no automated vulnerability scan on changes to the lockfile — `specs/058-findings-route-at-discovery/plan.md` — **waived**: the lockfile change is the release's own version refresh (plan.md §Release), which alters no dependency, and .github/workflows/runtime.yml already runs cargo audit on every change under runtime/, Cargo.lock included (decided by the rule)
- grounding — plan.md says .ductus/bin/ductus is a symlink to runtime/target/release/ductus, local state no artifact grounds — `specs/058-findings-route-at-discovery/plan.md` — **waived**: it describes this checkout's runtime link behind the Overview's landing order; every runtime task has landed, and no artifact or check reads it
- grounding — plan.md says 022's 95 other scenarios are not re-read, a count with no method — `specs/058-findings-route-at-discovery/plan.md` — **waived**: the count priced 022's discharge review when the plan was written; that discharge has landed, so the figure gates nothing, and re-deriving it is a measurement, not a mechanical edit
- grounding — plan.md says dedup-prefix is refused by name on MCP and the CLI without a source — `specs/058-findings-route-at-discovery/plan.md` — **waived**: the refusal is the MCP server's and the CLI's unknown-argument handling, which the data model states beside it; checking it means reading code, which is the review's work, not the plan's grounding
- decision-reasons — plan.md Trade-off 'Known limitation: 022's full re-review is not bought here' passes over an option and gives no reason beyond what the review run would cost (low confidence) — `specs/058-findings-route-at-discovery/plan.md` — **waived**: the line bounds this plan's scope, naming another spec's re-review as that spec's decision to spend; it rejects no option for the review it would start

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- root-absent: 1

## Fixed in this run

- rule-assessment — BE-INPUT-004: process-decisions reads specs/{feature}/{review,analysis}.md and stored deferral targets from host-supplied arguments and a --fired-json FILE, and no artifact commits them to canonicalization and containment or points at 022's plan Path arguments — `specs/058-findings-route-at-discovery/plan.md` — **fixed**
