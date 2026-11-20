---
spec: 080-deterministic-runtime-efficiency
last-run: 2026-11-20T00:26:49Z
analyzed-against: e3f191334517150fd8360ba8c9c92b061a7dced4
hard-fail: 0
blocking-findings: 0
advisory: 7
unexamined: 6
analyzed-digest:
  data-model.md: 8d6d914d57d77dc1dd020c6209abeb647abc79a4708a0e97cbb83a40b7329d08
  plan.md: 7952f3a389a0d5bb711cbf3fbfc3ed2731b40a3f47fb6afe2fc31c8cac8a5651
  research.md: 8cb129845003758f3a835b5701b0ac0f4beaeb5cb4addec9488fccfe2553e56b
  review.md: 872d5cfbfbaabdb7ef6f9a966bf37e19b29b6803702be4162e7a67c1df705eff
  scenarios/a-step-gates-on-a-bounded-list-by-its-count.md: bdf77659a5e3c0b7dac784691889a9a0160fbe5859a072adb33f7a4134217e83
  scenarios/configure-types-no-permission-entries.md: 6c764c4d44c20f0b7d88582dbd52de2d9c1efd915080813f41eb858e16ca7313
  scenarios/groom-derives-routing-candidates.md: 516690b45261a38cad315305e8b1ca19fa51a072137a7bfbb1a12c9217fe42da
  scenarios/help-answers-with-no-model-turn.md: 8962bafa8cbf7224dc43d5d73a0507d3bd1088492c5977c287eac52acff33189
  scenarios/one-search-covers-referenced-sections.md: 8f024532fe05b863b06fdb6facb8cae8ebe1e979dbfc8fd28035b6336fec97c3
  scenarios/reference-reads-follow-the-cited-sections.md: 66a004c23e55c8e1a4823a5fa13370220af42c42563194fc693c7067998d9c5f
  scenarios/rules-load-without-shell-reads.md: 052d22a0ed61649128bdada4bf6cef556aba1568aa64f6d6055eb5dbe84b92dd
  scenarios/the-search-goes-out-with-the-first-call.md: fb2cfb2453ca185c97d8fd9e81daad0c09fe92280d925c2ee2b52d6273dc000e
  spec.md: d2996cbb392abb285425a225c83e11f4196f0d1981f60badd155287600c53c12
  tasks.md: 74b690526b9bf38c7a0519ea98553a49527360ebc6032c6ba6185a827a3034d2
unexamined-by-reason:
  never-existed: 1
  root-absent: 4
  ships-to-adopter: 1
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
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#`render-status` serves the view, and `dashboard` stops carrying it"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#Bounded results"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-INPUT-008: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-INPUT-011: {verdict: addressed, cite: "plan.md#One view model, two renderings, one bound"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "spec.md#Bounded tool results"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-TXN-002: {verdict: waived, reason: "concurrent `/{project}:configure` runs read-modify-write the per-contributor `settings.local.json` through `merge-status-hook` with no version check or lock, so one run's hook entry can be lost; both merges are idempotent, so the next run restores it, and a lock would serialize a file two runs rarely share (Edge Cases; owner, 2026-11-14)", decided-by: "andy@stone.dev", decided-at: "2026-11-14T17:31:43Z"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#Claude Code's hook runs `ductus hook status`, a host surface, not a primitive"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  CFG-CONST-004: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-001: {verdict: addressed, cite: "plan.md#Claude Code's hook runs `ductus hook status`, a host surface, not a primitive"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  CFG-ENV-004: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-005: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-007: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-008: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "spec.md#Edge Cases"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 7
  undispositioned: 0
decisions:
  - key: grounding — plan.md says the parser ends the Instructions section at its first sub-heading, so amend parses as legacy prose, with no file-and-symbol citation
    outcome: waived
    reason: "advisory and no chore: a claim about existing repository, library or host behavior whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore; it predates this reopening and the claim is true"
    decided-at: 2026-11-20T00:26:49Z
    decided-by: rule
  - key: grounding — plan.md says a name costs about a third of a row, so 1,701 planned specs name 1,498 where rows would name 515, with no method or corpus cited
    outcome: waived
    reason: "advisory and no chore: a measurement recorded where the decision was made, re-derivable by the command its section names; it predates this reopening and states no false figure"
    decided-at: 2026-11-20T00:26:49Z
    decided-by: rule
  - key: grounding — plan.md states the 279,572-byte and 791,817-byte list-file sizes at the 1,701-spec corpus with no command or research.md citation
    outcome: waived
    reason: "advisory and no chore: a measurement recorded where the decision was made, re-derivable by the command its section names; it predates this reopening and states no false figure"
    decided-at: 2026-11-20T00:26:49Z
    decided-by: rule
  - key: grounding — plan.md says feature is a stopword in the routing-candidate derivation, with no file and symbol cited
    outcome: waived
    reason: "advisory and no chore: a claim about existing repository, library or host behavior whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore; it predates this reopening and the claim is true"
    decided-at: 2026-11-20T00:26:49Z
    decided-by: rule
  - key: grounding — spec.md Loading tools says Claude Code defers each tool schema until searched and a search returns in about 0.01 s, with no citation in that section
    outcome: waived
    reason: "advisory and no chore: the 353-call median is recorded in the Resolved Questions of the same spec and the claim is true; it predates this reopening"
    decided-at: 2026-11-20T00:26:49Z
    decided-by: rule
  - key: grounding — spec.md Edge Cases says collapsing took the 77-spec table from 7,625 to 4,271 characters, with no method, corpus or date
    outcome: waived
    reason: "advisory and no chore: a measurement recorded where the decision was made, re-derivable by the command its section names; it predates this reopening and states no false figure"
    decided-at: 2026-11-20T00:26:49Z
    decided-by: rule
  - key: artifact-size — `specs/080-deterministic-runtime-efficiency/plan.md` is 65389 bytes, 2 read pages at the 50000-byte read size
    outcome: waived
    reason: plan.md is 080's design record, over one read before this reopening (it was 64,683 bytes at 0.59.11's review); its finished Affected Files rows are trimmed with /ductus:prune once 080 is done
    decided-at: 2026-11-20T00:26:49Z
    decided-by: rule
---

# Analysis — 080-deterministic-runtime-efficiency

## Summary

0 hard-fail, 0 blocking, 7 advisory; not blocking. 6 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 7 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — plan.md says the parser ends the Instructions section at its first sub-heading, so amend parses as legacy prose, with no file-and-symbol citation — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: advisory and no chore: a claim about existing repository, library or host behavior whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore; it predates this reopening and the claim is true (decided by the rule)
- grounding — plan.md says a name costs about a third of a row, so 1,701 planned specs name 1,498 where rows would name 515, with no method or corpus cited — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: advisory and no chore: a measurement recorded where the decision was made, re-derivable by the command its section names; it predates this reopening and states no false figure (decided by the rule)
- grounding — plan.md states the 279,572-byte and 791,817-byte list-file sizes at the 1,701-spec corpus with no command or research.md citation — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: advisory and no chore: a measurement recorded where the decision was made, re-derivable by the command its section names; it predates this reopening and states no false figure (decided by the rule)
- grounding — plan.md says feature is a stopword in the routing-candidate derivation, with no file and symbol cited — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: advisory and no chore: a claim about existing repository, library or host behavior whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore; it predates this reopening and the claim is true (decided by the rule)
- grounding — spec.md Loading tools says Claude Code defers each tool schema until searched and a search returns in about 0.01 s, with no citation in that section — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: advisory and no chore: the 353-call median is recorded in the Resolved Questions of the same spec and the claim is true; it predates this reopening (decided by the rule)
- grounding — spec.md Edge Cases says collapsing took the 77-spec table from 7,625 to 4,271 characters, with no method, corpus or date — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: advisory and no chore: a measurement recorded where the decision was made, re-derivable by the command its section names; it predates this reopening and states no false figure (decided by the rule)
- artifact-size — `specs/080-deterministic-runtime-efficiency/plan.md` is 65389 bytes, 2 read pages at the 50000-byte read size — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: plan.md is 080's design record, over one read before this reopening (it was 64,683 bytes at 0.59.11's review); its finished Affected Files rows are trimmed with /ductus:prune once 080 is done (decided by the rule)

## Deferred to other specs

*None.*

## Waived rules

- BE-TXN-002 (MUST) — concurrent `/{project}:configure` runs read-modify-write the per-contributor `settings.local.json` through `merge-status-hook` with no version check or lock, so one run's hook entry can be lost; both merges are idempotent, so the next run restores it, and a lock would serialize a file two runs rarely share (Edge Cases; owner, 2026-11-14) — decided by `andy@stone.dev` at `2026-11-14T17:31:43Z`

## Contradictions

*None.*

## Unexamined targets

- never-existed: 1
- root-absent: 4
- ships-to-adopter: 1

## Fixed in this run

*None.*
