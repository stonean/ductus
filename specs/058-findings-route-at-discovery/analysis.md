---
spec: 058-findings-route-at-discovery
last-run: 2026-11-10T02:30:20Z
analyzed-against: c871a550bac2ce91323df9d599fe8416fd52bc7d
hard-fail: 0
blocking-findings: 0
advisory: 4
unexamined: 0
analyzed-digest:
  data-model.md: 8f13841cd953f001eb49276042456f2659fbaa3d1104f9f6faa3d173dae57eca
  plan.md: 503d0aabd4eb6f456f20b4201e43ae72d9c4bb812ddad9276e9926db4376f03e
  review.md: 4b6dbf1296ca7b2bbbd65457ea2fef1f8c03634996a6f6c7266f366ce42728a9
  scenarios/analysis-drift-judges-the-record-it-writes.md: 524d602fab04fdf2e208d9862af69e02fee41fc9f8e9767eccf3b51d3a3c45a9
  scenarios/analyze-findings-match-decisions-by-host-judgment.md: 1ec09d115b84951664fc4d57fb537ee44935b393482a6f28ed43d8b874160575
  scenarios/analyze-state-drift-judges-the-record-it-writes.md: 086887054c718293ed989707272272211d01b3ccaec334cef6f1b89cfca9730e
  scenarios/auto-records-disposition-tasks-without-pausing.md: 79217820badbfd130ba5e69a78927c3d434bcda01e3c66faec2be2b16dcf2c7e
  scenarios/findings-cost-in-proportion-to-risk.md: 908fca8623fc6fbe70a52e491f6f3efec028fdc8df7310fd958dacea9c48db55
  scenarios/only-unreadable-targets-retain-decisions.md: 76a12d565f6e1056f780906fed41de7d27e4a23c314232eddca0563026bcc3e1
  spec.md: 50f0ddd8692b594e0bae758085a920cd29dcf0936563fb5087093b15f61b1741
  tasks.md: 9c1ebd6060643f5f2c2a9344f415e6e85f5b5bbcebd360ab97c670c408b2b437
blocking: false
dispositions:
  fixed: 5
  deferred: 0
  waived: 4
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-REPR-002: the new kebab-case fields and enums cite no project-wide casing convention in specs/system.md"
    outcome: waived
    reason: declaring a casing convention is a durable, project-wide requirement on the runtime surface 022 owns; 022's spec fixes kebab-case for tool names, and 058's names are consistent with it and depart from nothing
    decided-at: 2026-11-10T02:30:20Z
    decided-by: rule
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
---

# Analysis — 058-findings-route-at-discovery

## Summary

0 hard-fail, 0 blocking, 4 advisory; not blocking. 0 unexamined target(s). Dispositions: 5 fixed, 0 deferred, 4 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-REPR-002: the new kebab-case fields and enums cite no project-wide casing convention in specs/system.md — `specs/058-findings-route-at-discovery/data-model.md` — **waived**: declaring a casing convention is a durable, project-wide requirement on the runtime surface 022 owns; 022's spec fixes kebab-case for tool names, and 058's names are consistent with it and depart from nothing (decided by the rule)
- grounding — plan.md says .ductus/bin/ductus is a symlink to runtime/target/release/ductus, local state no artifact grounds — `specs/058-findings-route-at-discovery/plan.md` — **waived**: it describes this checkout's runtime link behind the Overview's landing order; every runtime task has landed, and no artifact or check reads it (decided by the rule)
- grounding — plan.md says 022's 95 other scenarios are not re-read, a count with no method — `specs/058-findings-route-at-discovery/plan.md` — **waived**: the count priced 022's discharge review when the plan was written; that discharge has landed, so the figure gates nothing, and re-deriving it is a measurement, not a mechanical edit (decided by the rule)
- grounding — plan.md says dedup-prefix is refused by name on MCP and the CLI without a source — `specs/058-findings-route-at-discovery/plan.md` — **waived**: the refusal is the MCP server's and the CLI's unknown-argument handling, which the data model states beside it; checking it means reading code, which is the review's work, not the plan's grounding (decided by the rule)

## Deferred to other specs

*None.*

## Unexamined targets

*None — every target was examined.*

## Fixed in this run

- rule-assessment — BE-PAYLOAD-001: process-decisions returns matched, expired and retained lists, and no 058 artifact commits to their bound or points at 066's — `specs/058-findings-route-at-discovery/plan.md` — **fixed**
- grounding — spec.md says the dependency generator skips blockquote lines without naming it — `specs/058-findings-route-at-discovery/spec.md` — **fixed** (decided by the rule)
- grounding — spec.md says the inbox is installed once and never updated without citing its install strategy — `specs/058-findings-route-at-discovery/spec.md` — **fixed** (decided by the rule)
- grounding — plan.md cites a constitution rule, Re-capture is idempotent, the constitution no longer states — `specs/058-findings-route-at-discovery/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md says the signpost induces no dependencies: edge without naming the mechanism — `specs/058-findings-route-at-discovery/plan.md` — **fixed** (decided by the rule)
