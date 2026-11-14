---
spec: 080-deterministic-runtime-efficiency
last-run: 2026-11-14T02:14:06Z
analyzed-against: 040eb2d84c339ee0a19525344877fc53744b2b64
hard-fail: 0
blocking-findings: 0
advisory: 17
unexamined: 5
analyzed-digest:
  data-model.md: dd03eac77f44f8048506c53655abc00e5d8052291c0547a04926edee5de7fffe
  plan.md: 9a183db6f4f3101a7c25ecdb59478e1e08dc24b92c8dd203423d26faa6ba2f03
  research.md: 33cc92a17e6fa7d6c68496729e6294da409409271b033b2807ab2804d859794b
  review.md: f7f71d28576bd49886418b0376c2832a7dbd9796feb60c4c3482adbb8a40c1a7
  scenarios/configure-types-no-permission-entries.md: 6c764c4d44c20f0b7d88582dbd52de2d9c1efd915080813f41eb858e16ca7313
  scenarios/groom-derives-routing-candidates.md: 516690b45261a38cad315305e8b1ca19fa51a072137a7bfbb1a12c9217fe42da
  scenarios/help-answers-with-no-model-turn.md: 8962bafa8cbf7224dc43d5d73a0507d3bd1088492c5977c287eac52acff33189
  scenarios/one-search-covers-referenced-sections.md: 8f024532fe05b863b06fdb6facb8cae8ebe1e979dbfc8fd28035b6336fec97c3
  scenarios/reference-reads-follow-the-cited-sections.md: 66a004c23e55c8e1a4823a5fa13370220af42c42563194fc693c7067998d9c5f
  scenarios/rules-load-without-shell-reads.md: 052d22a0ed61649128bdada4bf6cef556aba1568aa64f6d6055eb5dbe84b92dd
  scenarios/the-search-goes-out-with-the-first-call.md: fb2cfb2453ca185c97d8fd9e81daad0c09fe92280d925c2ee2b52d6273dc000e
  spec.md: 9e91d336e2b2d03965f163726a549c376544eb1e5ff6ff7b53fedfded2585a7c
  tasks.md: 508851a1ac6f7095f26d7edab4e0b524966fcf4afa10c9ff4b9433f76606217b
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
  security-backend.md: d89d6dad3ab51586ae4e68944b0c8e9ad4d5da721560f10590de3cdecc904d3f
  security-cross.md: dc98261801405fccaf5de9eb8f8ed9b020ebc78fcb5bfb124778a5fcc287950b
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-BULK-002: {verdict: addressed, cite: "spec.md#Edge Cases"}
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#`render-status` serves the view, and `dashboard` stops carrying it"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#Bounded results"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Reference sections are read through `read-section`"}
  BE-INPUT-008: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  BE-INPUT-011: {verdict: addressed, cite: "plan.md#One view model, two renderings, one bound"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "data-model.md#Bounded results"}
  BE-PAYLOAD-002: {verdict: addressed, cite: "data-model.md#`discover-rule-files`' rules"}
  BE-RACE-003: {verdict: addressed, cite: "spec.md#Edge Cases"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#Claude Code's hook runs `ductus hook status`, a host surface, not a primitive"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#`merge-status-hook` installs the hook and gates it on the version"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#A `claude -p` harness records each command's cost"}
  CFG-ENV-001: {verdict: addressed, cite: "plan.md#Claude Code's hook runs `ductus hook status`, a host surface, not a primitive"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  CFG-ENV-004: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-005: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-007: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-008: {verdict: addressed, cite: "plan.md#Tool loading derives from each command's parsed steps"}
  CFG-ENV-009: {verdict: addressed, cite: "plan.md#Claude Code's hook runs `ductus hook status`, a host surface, not a primitive"}
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 17
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
---

# Analysis — 080-deterministic-runtime-efficiency

## Summary

0 hard-fail, 0 blocking, 17 advisory; not blocking. 5 unexamined target(s). Dispositions: 1 fixed, 0 deferred, 17 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — spec.md (Resolved Questions, How is a command's cost measured) says transcripts are removed after `cleanupPeriodDays`, 30 days by default, naming no source — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
- grounding — spec.md (Does Claude Code's block presentation read acceptably, No alternative) says `MessageDisplay` transforms only assistant text and a `systemMessage` without a block lets the prompt reach the model, naming no source — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
- grounding — spec.md (What does a release older than UserPromptExpansion do, Rejected alternatives) says Claude Code reads only user, project and local settings, naming no source — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
- grounding — spec.md (Which tools set alwaysLoad, What a load adds) says the output schemas never reach the model, naming no source; plan.md's method-trap bullet in the first Technical Decision repeats it — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
- grounding — spec.md (Which tools set alwaysLoad, Constraint) says Claude Code cuts an upfront-loaded description at 4,096 characters against 16,384 through tool search, naming no source; plan.md's Always-loaded test bullet cites it back to the spec — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
- grounding — plan.md (Tool loading derives from each command's parsed steps) says the parser ends `## Instructions` at its first sub-heading, citing no path or symbol — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on repository code whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore.
- grounding — plan.md (Always loaded, AC21) says rmcp 3.9.0's `Tool.meta` serializes as `_meta`, naming no source — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A library behavior whose re-sourcing needs the library's documentation or source, which is research rather than a chore.
- grounding — plan.md (Batched, This repo's copies) says `apply-manifest` holds both roots inside the directory it runs in, citing no source — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on repository code whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore.
- grounding — plan.md (Help answers through the status surfaces, ductus hook help) says clap otherwise generates `ductus hook help` itself, naming no source — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A library behavior whose re-sourcing needs the library's documentation or source, which is research rather than a chore.
- grounding — plan.md (groom derives its routing candidates) says implement, review and analyze walk groom's decision tree and derive no routing candidates, citing none of those commands — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on other commands whose re-sourcing needs reading their sources, which is research rather than a chore.
- grounding — plan.md (Corpus-growing results, The walker) says implement runs five bounded primitives in one walk, citing neither the command nor a walk — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on another command whose re-sourcing needs reading its source or a walk, which is research rather than a chore.
- grounding — plan.md (Corpus-growing results, read-spec carries no audit record) says no command, walker step or script read read-spec's `review` and `analyze`, naming no search — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note whose re-sourcing needs a corpus search outside analyze's scope, which is research rather than a chore.
- grounding — plan.md (Trade-offs, Naming the command's first call in the line) says the first tool the steps name is not the first call in review, implement or groom, citing neither the commands nor a run — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on other commands whose re-sourcing needs reading their sources or a run, which is research rather than a chore.
- grounding — plan.md (Trade-offs, Reading configure's sets from its installed copy) says the allow side already ignores a pinned copy's edits, citing no code or test — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on repository code whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore.
- grounding — plan.md (Trade-offs, Making the canonical sets the default) says an omitted list and an empty one arrive alike over MCP, naming no source — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A protocol and library behavior whose re-sourcing needs their documentation or a run, which is research rather than a chore.
- grounding — plan.md (Trade-offs, Renaming merge-status-hook) says the name is in 28 live files (115 occurrences, 7 specs), with no command or method for the count — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: advisory and no chore: a count of where the name occurs, which a grep of the name across the live-artifact set re-derives; the trade-off rejects the rename on the adopters' permission entries it would retire, which the count does not carry (decided by the rule)
- artifact-size — `specs/080-deterministic-runtime-efficiency/plan.md` is 62571 bytes, 2 read pages at the 50000-byte read size — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: plan.md is 080's design record, grown by this analysis's rule commitments; its finished Affected Files rows are trimmed with /ductus:prune once 080 is done.

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- never-existed: 1
- root-absent: 4

## Fixed in this run

- rule-assessment — CFG-ENV-005: DUCTUS_GEN_TIMEOUT_SECONDS is valid only as a positive whole number, but its out-of-range failure is committed to name only the variable, not the offending value and the violated bound CFG-ENV-005 requires. — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
