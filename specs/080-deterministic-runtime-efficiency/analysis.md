---
spec: 080-deterministic-runtime-efficiency
last-run: 2026-11-18T13:09:38Z
analyzed-against: 828a18f540096640b96f627b69a48290a5fd27c4
hard-fail: 0
blocking-findings: 0
advisory: 6
unexamined: 5
analyzed-digest:
  data-model.md: 8d6d914d57d77dc1dd020c6209abeb647abc79a4708a0e97cbb83a40b7329d08
  plan.md: 661bd835dd1174efdc4792f9494ae4a9b87d264002bc47ef53a21b5213495b80
  research.md: 8cb129845003758f3a835b5701b0ac0f4beaeb5cb4addec9488fccfe2553e56b
  review.md: d24d1b1b5b45da035d96d943eef1762ed9e8ecec3daeac23a01441600cee254b
  scenarios/configure-types-no-permission-entries.md: 6c764c4d44c20f0b7d88582dbd52de2d9c1efd915080813f41eb858e16ca7313
  scenarios/groom-derives-routing-candidates.md: 516690b45261a38cad315305e8b1ca19fa51a072137a7bfbb1a12c9217fe42da
  scenarios/help-answers-with-no-model-turn.md: 8962bafa8cbf7224dc43d5d73a0507d3bd1088492c5977c287eac52acff33189
  scenarios/one-search-covers-referenced-sections.md: 8f024532fe05b863b06fdb6facb8cae8ebe1e979dbfc8fd28035b6336fec97c3
  scenarios/reference-reads-follow-the-cited-sections.md: 66a004c23e55c8e1a4823a5fa13370220af42c42563194fc693c7067998d9c5f
  scenarios/rules-load-without-shell-reads.md: 052d22a0ed61649128bdada4bf6cef556aba1568aa64f6d6055eb5dbe84b92dd
  scenarios/the-search-goes-out-with-the-first-call.md: fb2cfb2453ca185c97d8fd9e81daad0c09fe92280d925c2ee2b52d6273dc000e
  spec.md: 61f91589c1a6bfc443089ea6d22bc509480f4d7391738d96cbc4e32fb3d08d9d
  tasks.md: b14364f15f2d5f8adcaaaaa237f2087859ec979637e65122dcad45c7b9b5d81e
unexamined-by-reason:
  never-existed: 1
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
  security-backend.md: 9873cc885c4bdc76eff79a7269a5a6858a0b84c3010ff4b1bc27e7989f708183
  security-cross.md: bf70990f24e7e67926772be0d95deb7e1c1492bc12bdcb650a23b25385bcd285
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#`render-status` serves the view, and `dashboard` stops carrying it"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Reference sections are read through `read-section`"}
  BE-INPUT-008: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-INPUT-011: {verdict: addressed, cite: "plan.md#One view model, two renderings, one bound"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "spec.md#Bounded tool results"}
  BE-PAYLOAD-002: {verdict: addressed, cite: "plan.md#Rules reach the model through `discover-rule-files`"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-TXN-002: {verdict: waived, reason: "concurrent `/{project}:configure` runs read-modify-write the per-contributor `settings.local.json` through `merge-status-hook` with no version check or lock, so one run's hook entry can be lost; both merges are idempotent, so the next run restores it, and a lock would serialize a file two runs rarely share (Edge Cases; owner, 2026-11-14)", decided-by: "andy@stone.dev", decided-at: "2026-11-14T17:31:43Z"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#Claude Code's hook runs `ductus hook status`, a host surface, not a primitive"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#Claude Code's hook runs `ductus hook status`, a host surface, not a primitive"}
  CFG-CONST-004: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-001: {verdict: addressed, cite: "plan.md#Claude Code's hook runs `ductus hook status`, a host surface, not a primitive"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  CFG-ENV-004: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-005: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-007: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-008: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 6
  undispositioned: 0
decisions:
  - key: artifact-size — `specs/080-deterministic-runtime-efficiency/plan.md` is 63372 bytes, 2 read pages at the 50000-byte read size
    outcome: waived
    reason: plan.md is 080's design record, grown by this analysis's rule commitments; its finished Affected Files rows are trimmed with /ductus:prune once 080 is done.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — plan.md says the parser ends `## Instructions` at its first sub-heading, citing no path or symbol
    outcome: waived
    reason: A design note on repository code whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — plan.md says clap otherwise generates `ductus hook help` itself, naming no source
    outcome: waived
    reason: A library behavior whose re-sourcing needs the library's documentation or source, which is research rather than a chore.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — plan.md (Trade-offs, Renaming merge-status-hook) says the name is in 28 live files (115 occurrences, 7 specs), with no command or method for the count
    outcome: waived
    reason: "advisory and no chore: a count of where the name occurs, which a grep of the name across the live-artifact set re-derives; the trade-off rejects the rename on the adopters' permission entries it would retire, which the count does not carry"
    decided-at: 2026-11-14T02:14:06Z
    decided-by: rule
  - key: decision-reasons — plan.md Trade-offs rejects renaming merge-status-hook for the sweep it would start (28 files, 7 specs) and the adopter permission entries it would retire
    outcome: waived
    reason: "the reason beside the sweep is a result for adopters: every permission entry for the old name would stop matching and need retiring, so the decision does not rest on the work in this repository alone"
    decided-at: 2026-11-18T13:09:38Z
    decided-by: rule
  - key: decision-reasons — plan.md Trade-offs rejects one file per reference section for the pointer, manifest, anchor and audit changes and the adopter migration it would start
    outcome: waived
    reason: an owner decision (2026-11-09) recorded with its cost to adopters, a migration each would run, beside the changes here; re-deciding it is the owner's, and the advisory proves no defect
    decided-at: 2026-11-18T13:09:38Z
    decided-by: rule
---

# Analysis — 080-deterministic-runtime-efficiency

## Summary

0 hard-fail, 0 blocking, 6 advisory; not blocking. 5 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 6 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- artifact-size — `specs/080-deterministic-runtime-efficiency/plan.md` is 62994 bytes, 2 read pages at the 50000-byte read size — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: plan.md is 080's design record, grown by this analysis's rule commitments; its finished Affected Files rows are trimmed with /ductus:prune once 080 is done. (decided by the rule)
- grounding — plan.md says the parser ends the Instructions section at its first sub-heading, so amend parses as legacy prose, with no source cited — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on repository code whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore. (decided by the rule)
- grounding — plan.md says clap generates ductus hook help itself, with no source cited — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A library behavior whose re-sourcing needs the library's documentation or source, which is research rather than a chore. (decided by the rule)
- grounding — plan.md says merge-status-hook is in 28 live files, 115 occurrences, 7 specs, with no query cited — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: advisory and no chore: a count of where the name occurs, which a grep of the name across the live-artifact set re-derives; the trade-off rejects the rename on the adopters' permission entries it would retire, which the count does not carry (decided by the rule)
- decision-reasons — plan.md Trade-offs rejects renaming merge-status-hook for the sweep it would start (28 files, 7 specs) and the adopter permission entries it would retire — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: the reason beside the sweep is a result for adopters: every permission entry for the old name would stop matching and need retiring, so the decision does not rest on the work in this repository alone (decided by the rule)
- decision-reasons — plan.md Trade-offs rejects one file per reference section for the pointer, manifest, anchor and audit changes and the adopter migration it would start — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: an owner decision (2026-11-09) recorded with its cost to adopters, a migration each would run, beside the changes here; re-deciding it is the owner's, and the advisory proves no defect (decided by the rule)

## Deferred to other specs

*None.*

## Waived rules

- BE-TXN-002 (MUST) — concurrent `/{project}:configure` runs read-modify-write the per-contributor `settings.local.json` through `merge-status-hook` with no version check or lock, so one run's hook entry can be lost; both merges are idempotent, so the next run restores it, and a lock would serialize a file two runs rarely share (Edge Cases; owner, 2026-11-14) — decided by `andy@stone.dev` at `2026-11-14T17:31:43Z`

## Contradictions

*None.*

## Unexamined targets

- never-existed: 1
- root-absent: 4

## Fixed in this run

*None.*
