---
spec: 080-deterministic-runtime-efficiency
last-run: 2026-11-16T14:32:18Z
analyzed-against: 6be0fc3932d19f70b73c27984f3c37cb1a849ced
hard-fail: 0
blocking-findings: 0
advisory: 23
unexamined: 5
analyzed-digest:
  data-model.md: 8d6d914d57d77dc1dd020c6209abeb647abc79a4708a0e97cbb83a40b7329d08
  plan.md: d80249b89df93828c652638eecd368263ce83d7f6b3cb8d6d819f42aa94fc858
  research.md: f3fe9bd9319dc4dfd1f6da663e2457cb44018560d0e6491e80422c0985d8981e
  review.md: d24d1b1b5b45da035d96d943eef1762ed9e8ecec3daeac23a01441600cee254b
  scenarios/configure-types-no-permission-entries.md: 6c764c4d44c20f0b7d88582dbd52de2d9c1efd915080813f41eb858e16ca7313
  scenarios/groom-derives-routing-candidates.md: 516690b45261a38cad315305e8b1ca19fa51a072137a7bfbb1a12c9217fe42da
  scenarios/help-answers-with-no-model-turn.md: 8962bafa8cbf7224dc43d5d73a0507d3bd1088492c5977c287eac52acff33189
  scenarios/one-search-covers-referenced-sections.md: 8f024532fe05b863b06fdb6facb8cae8ebe1e979dbfc8fd28035b6336fec97c3
  scenarios/reference-reads-follow-the-cited-sections.md: 66a004c23e55c8e1a4823a5fa13370220af42c42563194fc693c7067998d9c5f
  scenarios/rules-load-without-shell-reads.md: 052d22a0ed61649128bdada4bf6cef556aba1568aa64f6d6055eb5dbe84b92dd
  scenarios/the-search-goes-out-with-the-first-call.md: fb2cfb2453ca185c97d8fd9e81daad0c09fe92280d925c2ee2b52d6273dc000e
  spec.md: b4025f1a33fa00ad95a9e47944ba77c131368b49f4700e4ff8a5c647eedda539
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
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#Bounded results"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Reference sections are read through `read-section`"}
  BE-INPUT-008: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-INPUT-011: {verdict: addressed, cite: "plan.md#One view model, two renderings, one bound"}
  BE-LOCK-001: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "spec.md#Bounded tool results"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-TXN-002: {verdict: waived, reason: "concurrent `/{project}:configure` runs read-modify-write the per-contributor `settings.local.json` through `merge-status-hook` with no version check or lock, so one run's hook entry can be lost; both merges are idempotent, so the next run restores it, and a lock would serialize a file two runs rarely share (Edge Cases; owner, 2026-11-14)", decided-by: "andy@stone.dev", decided-at: "2026-11-14T17:31:43Z"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#Claude Code's hook runs `ductus hook status`, a host surface, not a primitive"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#A `claude -p` harness records each command's cost"}
  CFG-CONST-004: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-001: {verdict: addressed, cite: "plan.md#Claude Code's hook runs `ductus hook status`, a host surface, not a primitive"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-004: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-005: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-008: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 23
  undispositioned: 0
decisions:
  - key: artifact-size — `specs/080-deterministic-runtime-efficiency/plan.md` is 63372 bytes, 2 read pages at the 50000-byte read size
    outcome: waived
    reason: plan.md is 080's design record, grown by this analysis's rule commitments; its finished Affected Files rows are trimmed with /ductus:prune once 080 is done.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — spec.md says Claude Code transcripts are removed after `cleanupPeriodDays`, 30 days by default, naming no source
    outcome: waived
    reason: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — spec.md says `MessageDisplay` transforms only assistant text and a `systemMessage` without a block lets the prompt reach the model, naming no source
    outcome: waived
    reason: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — spec.md says Claude Code reads only user, project and local settings, naming no source
    outcome: waived
    reason: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — spec.md says the output schemas never reach the model, naming no source
    outcome: waived
    reason: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — spec.md says Claude Code cuts an upfront-loaded tool's description at 4,096 characters against 16,384 through tool search, naming no source
    outcome: waived
    reason: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — plan.md says the parser ends `## Instructions` at its first sub-heading, citing no path or symbol
    outcome: waived
    reason: A design note on repository code whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — plan.md says rmcp 3.9.0's `Tool.meta` serializes as `_meta`, naming no source
    outcome: waived
    reason: A library behavior whose re-sourcing needs the library's documentation or source, which is research rather than a chore.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — plan.md says `apply-manifest` holds both roots inside the directory it runs in, citing no source
    outcome: waived
    reason: A design note on repository code whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — plan.md says clap otherwise generates `ductus hook help` itself, naming no source
    outcome: waived
    reason: A library behavior whose re-sourcing needs the library's documentation or source, which is research rather than a chore.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — plan.md says implement, review and analyze walk groom's decision tree and derive no routing candidates, citing none of those commands
    outcome: waived
    reason: A design note on other commands whose re-sourcing needs reading their sources, which is research rather than a chore.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — plan.md says implement runs five bounded primitives in one walk, citing neither the command nor a walk
    outcome: waived
    reason: A design note on another command whose re-sourcing needs reading its source or a walk, which is research rather than a chore.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — plan.md says no command, walker step or script read read-spec's `review` and `analyze`, naming no search
    outcome: waived
    reason: A design note whose re-sourcing needs a corpus search outside analyze's scope, which is research rather than a chore.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — plan.md says the first tool the steps name is not the first call in review, implement or groom, citing neither the commands nor a run
    outcome: waived
    reason: A design note on other commands whose re-sourcing needs reading their sources or a run, which is research rather than a chore.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — plan.md says the allow side already ignores a pinned configure copy's edits, citing no code or test
    outcome: waived
    reason: A design note on repository code whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — plan.md says an omitted list and an empty one arrive alike over MCP, naming no source
    outcome: waived
    reason: A protocol and library behavior whose re-sourcing needs their documentation or a run, which is research rather than a chore.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: grounding — plan.md (Trade-offs, Renaming merge-status-hook) says the name is in 28 live files (115 occurrences, 7 specs), with no command or method for the count
    outcome: waived
    reason: "advisory and no chore: a count of where the name occurs, which a grep of the name across the live-artifact set re-derives; the trade-off rejects the rename on the adopters' permission entries it would retire, which the count does not carry"
    decided-at: 2026-11-14T02:14:06Z
    decided-by: rule
  - key: grounding — plan.md 'Bound (AC24)' states fixture-corpus sizes (1,701 planned specs naming 1,498 specs against 515 by rows; a 279,572-byte view file against dashboard's 791,817 bytes) with no method or pointer to research.md
    outcome: waived
    reason: Figures are measurements of this spec's own generated fixture corpus, which scripts/measure-result-sizes.sh and the AC24 test reproduce; they carry no research.md pointer in the plan, but they are not claims about a system outside the spec's own work, and no gate reads them.
    decided-at: 2026-11-15T02:10:11Z
    decided-by: rule
  - key: grounding — spec.md Edge Cases states 'Collapsing took the 77-spec table from 7,625 to 4,271 characters' with no source, command or method
    outcome: waived
    reason: A measurement of the view's own collapse step, dated by the surrounding owner decisions (2026-11-08, 2026-11-10); the criterion it illustrates (AC24) is checked by a test, so the figure is illustration, not a contract, and no gate reads it.
    decided-at: 2026-11-15T02:10:11Z
    decided-by: rule
  - key: grounding — plan.md (Bound (AC24)) says the view would pass the read size at about 660 unresolved questions on this repository's slugs, with no per-line size or method for the estimate
    outcome: waived
    reason: A derived estimate of the view's own cut behavior, illustrating the decision to cut question lines (owner, 2026-11-10); AC24's test, not the figure, checks the bound, and no gate reads it.
    decided-at: 2026-11-16T14:12:27Z
    decided-by: rule
  - key: grounding — plan.md (Trade-offs, Leaving every body search to the model) states the failed round's three greps cost 6.99 s and 3 of 7 model requests, with no pointer to research.md
    outcome: waived
    reason: A measurement from the cost round that scenario groom-derives-routing-candidates cites to research.md R6; the plan omits the pointer but the figure is a rejected option's illustration, and no gate reads it.
    decided-at: 2026-11-16T14:12:27Z
    decided-by: rule
  - key: grounding — plan.md (Corpus-growing results, Consumers) states the sweep reached '12 command sources and 5 references' and 'the 8 branches' with no command or method for the counts
    outcome: waived
    reason: A count of the sweep's own targets, owner-decided 2026-11-08 and re-derivable by grepping the lists-file wording across framework/; it is no claim about a system outside the spec's work and no gate reads it.
    decided-at: 2026-11-16T14:32:18Z
    decided-by: rule
  - key: grounding — plan.md (Help answers through the status surfaces, ductus hook help) states the help copy holds 'five pipe tables (28 lines, the longest 480 characters)' with no method or pointer
    outcome: waived
    reason: A measurement of the generated help copy, re-derivable by reading it, offered to explain why the guide shows as markdown source in a terminal; the criterion it illustrates is checked by the hook's tests, and no gate reads the figure.
    decided-at: 2026-11-16T14:32:18Z
    decided-by: rule
---

# Analysis — 080-deterministic-runtime-efficiency

## Summary

0 hard-fail, 0 blocking, 23 advisory; not blocking. 5 unexamined target(s). Dispositions: 1 fixed, 0 deferred, 23 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — grounding — Resolved Questions (measuring a command) still says transcripts are removed after cleanupPeriodDays, 30 days by default, naming no source — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
- grounding — grounding — Resolved Questions (block presentation) still says MessageDisplay transforms only assistant text and a systemMessage without a block lets the prompt reach the model, naming no source — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
- grounding — grounding — Resolved Questions (older releases) still says Claude Code reads only user, project and local settings, naming no source — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
- grounding — grounding — Resolved Questions (alwaysLoad) still says the output schemas never reach the model, naming no source; plan.md's method-trap bullet repeats it — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
- grounding — grounding — Resolved Questions (alwaysLoad, Constraint) still says Claude Code cuts an upfront-loaded description at 4,096 characters against 16,384 through tool search, naming no source; plan.md cites it back to the spec only — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
- grounding — grounding — Tool loading still says the parser ends ## Instructions at its first sub-heading, citing no path or symbol in that sentence — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on repository code whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore.
- grounding — grounding — Always loaded still says rmcp 3.9.0's Tool.meta serializes as _meta, naming no source for the serialization (the server.rs citation covers the post-processing only) — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A library behavior whose re-sourcing needs the library's documentation or source, which is research rather than a chore.
- grounding — grounding — Batched (This repo's copies) still says apply-manifest holds both roots inside the directory it runs in, citing no source — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on repository code whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore.
- grounding — grounding — Help answers through the status surfaces still says clap otherwise generates ductus hook help itself, naming no source — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A library behavior whose re-sourcing needs the library's documentation or source, which is research rather than a chore.
- grounding — grounding — groom derives its routing candidates still says implement, review and analyze walk groom's decision tree and derive no routing candidates, citing none of those commands — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on other commands whose re-sourcing needs reading their sources, which is research rather than a chore.
- grounding — grounding — Corpus-growing results (The walker) still says implement runs five bounded primitives in one walk, citing neither the command nor a walk — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on another command whose re-sourcing needs reading its source or a walk, which is research rather than a chore.
- grounding — grounding — Corpus-growing results (read-spec) still says no command, walker step or script read read-spec's review and analyze, naming no search — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note whose re-sourcing needs a corpus search outside analyze's scope, which is research rather than a chore.
- grounding — grounding — Trade-offs (Naming the command's first call) still says the first tool the steps name is not the first call in review, implement or groom, citing neither the commands nor a run — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on other commands whose re-sourcing needs reading their sources or a run, which is research rather than a chore.
- grounding — grounding — Trade-offs (Reading configure's sets from its installed copy) still says the allow side already ignores a pinned configure copy's edits, citing no code or test — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on repository code whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore.
- grounding — grounding — Trade-offs (Making the canonical sets the default) still says an omitted list and an empty one arrive alike over MCP, naming no source — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A protocol and library behavior whose re-sourcing needs their documentation or a run, which is research rather than a chore.
- grounding — grounding — Trade-offs (Renaming merge-status-hook) still says the name is in 28 live files (115 occurrences, 7 specs), with no command or method for the count — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: advisory and no chore: a count of where the name occurs, which a grep of the name across the live-artifact set re-derives; the trade-off rejects the rename on the adopters' permission entries it would retire, which the count does not carry
- grounding — grounding — One view model (Bound (AC24)) still states fixture-corpus sizes (1,701 planned specs naming 1,498 against 515 by rows; a 279,572-byte view file against dashboard's 791,817 bytes) with no method or pointer to research.md — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: Figures are measurements of this spec's own generated fixture corpus, which scripts/measure-result-sizes.sh and the AC24 test reproduce; they carry no research.md pointer in the plan, but they are not claims about a system outside the spec's own work, and no gate reads them.
- grounding — grounding — Edge Cases still states 'Collapsing took the 77-spec table from 7,625 to 4,271 characters' with no source, command or method — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A measurement of the view's own collapse step, dated by the surrounding owner decisions (2026-11-08, 2026-11-10); the criterion it illustrates (AC24) is checked by a test, so the figure is illustration, not a contract, and no gate reads it.
- grounding — grounding — One view model (Bound (AC24)) still says the view would pass the read size at about 660 unresolved questions on this repository's slugs, with no per-line size or method — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A derived estimate of the view's own cut behavior, illustrating the decision to cut question lines (owner, 2026-11-10); AC24's test, not the figure, checks the bound, and no gate reads it.
- grounding — grounding — Trade-offs (Leaving every body search to the model) still states the failed round's three greps cost 6.99 s and 3 of 7 model requests, with no pointer to research.md — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A measurement from the cost round that scenario groom-derives-routing-candidates cites to research.md R6; the plan omits the pointer but the figure is a rejected option's illustration, and no gate reads it.
- grounding — plan.md (Corpus-growing results, Consumers) states the sweep reached '12 command sources and 5 references' and 'the 8 branches' with no command or method for the counts — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A count of the sweep's own targets, owner-decided 2026-11-08 and re-derivable by grepping the lists-file wording across framework/; it is no claim about a system outside the spec's work and no gate reads it. (decided by the rule)
- grounding — plan.md (Help answers through the status surfaces, ductus hook help) states the help copy holds 'five pipe tables (28 lines, the longest 480 characters)' with no method or pointer — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A measurement of the generated help copy, re-derivable by reading it, offered to explain why the guide shows as markdown source in a terminal; the criterion it illustrates is checked by the hook's tests, and no gate reads the figure. (decided by the rule)
- artifact-size — `specs/080-deterministic-runtime-efficiency/plan.md` is 62662 bytes, 2 read pages at the 50000-byte read size — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: plan.md is 080's design record, grown by this analysis's rule commitments; its finished Affected Files rows are trimmed with /ductus:prune once 080 is done.

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

- rule-assessment — CFG-CONST-004: DUCTUS_GEN_TIMEOUT_SECONDS has a valid range (positive whole number) but the plan names no constant for its bound beside DEFAULT_GEN_TIMEOUT_SECONDS — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed**
