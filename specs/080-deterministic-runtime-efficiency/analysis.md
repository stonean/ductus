---
spec: 080-deterministic-runtime-efficiency
last-run: 2026-11-11T14:43:26Z
analyzed-against: c810427e4d2426279b96153d7910ba4e0fd31847
hard-fail: 0
blocking-findings: 0
advisory: 20
unexamined: 0
analyzed-digest:
  data-model.md: 08e290ce310f85f4ebea70052dbce6bcfbb9d85f666daf6e2dd73ee6db22cb14
  plan.md: d097e7147f3f90d1bdc6263ae63352ef715a80380618a2bfeb7336ebc23f1361
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
  tasks.md: 7f6c36bf5e3550d9a7991a192d9748b74b22a8e5051ef0ad802c63b26038b2a8
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 20
  undispositioned: 0
decisions:
  - key: artifact-size — `specs/080-deterministic-runtime-efficiency/plan.md` is 63372 bytes, 2 read pages at the 50000-byte read size
    outcome: waived
    reason: plan.md is 080's design record, grown by this analysis's rule commitments; its finished Affected Files rows are trimmed with /ductus:prune once 080 is done.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: artifact-size — `specs/080-deterministic-runtime-efficiency/tasks.md` is 60296 bytes, 2 read pages at the 50000-byte read size
    outcome: waived
    reason: tasks.md holds the out-of-spec findings' disposition tasks until 080 closes; its spent sections are pruned with /ductus:prune once 080 is done.
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
  - key: "rule-assessment — BE-METRIC-001: plan.md commits the new request-handling paths, the render-status, render-help, merge-status-hook and read-section handlers and the ductus hook surface, to no RED metrics"
    outcome: waived
    reason: The runtime is a per-agent stdio process with no metrics pipeline, and no primitive emits metrics; adding the first belongs to a spec about the runtime's observability, as 041's and 062's analyses decided.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: "rule-assessment — BE-TRACE-002: plan.md wraps the claude --version probe, externally dependent work, in no span"
    outcome: waived
    reason: The runtime is a per-agent stdio process with no tracing pipeline; adding the first span belongs to a spec about the runtime's observability, as 022's analysis decided for fetch-archive.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
  - key: "rule-assessment — BE-REPR-002: data-model.md mixes Claude Code's camelCase hook-output and `_meta` names with ductus's kebab-case fields"
    outcome: waived
    reason: The camelCase names are Claude Code's hook-output and `_meta` schema, emitted as the host defines them, the names ductus chooses are kebab-case, and the project declares no casing convention (it has no specs/system.md), as 058's and 078's analyses decided.
    decided-at: 2026-11-11T00:26:02Z
    decided-by: rule
---

# Analysis — 080-deterministic-runtime-efficiency

## Summary

0 hard-fail, 0 blocking, 20 advisory; not blocking. 0 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 20 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- artifact-size — `specs/080-deterministic-runtime-efficiency/plan.md` is 63372 bytes, 2 read pages at the 50000-byte read size — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: plan.md is 080's design record, grown by this analysis's rule commitments; its finished Affected Files rows are trimmed with /ductus:prune once 080 is done.
- artifact-size — `specs/080-deterministic-runtime-efficiency/tasks.md` is 60296 bytes, 2 read pages at the 50000-byte read size — `specs/080-deterministic-runtime-efficiency/tasks.md` — **waived**: tasks.md holds the out-of-spec findings' disposition tasks until 080 closes; its spent sections are pruned with /ductus:prune once 080 is done.
- grounding — spec.md says Claude Code transcripts are removed after `cleanupPeriodDays`, 30 days by default, naming no source — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
- grounding — spec.md says `MessageDisplay` transforms only assistant text and a `systemMessage` without a block lets the prompt reach the model, naming no source — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
- grounding — spec.md says Claude Code reads only user, project and local settings, naming no source — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
- grounding — spec.md says the output schemas never reach the model, naming no source — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
- grounding — spec.md says Claude Code cuts an upfront-loaded tool's description at 4,096 characters against 16,384 through tool search, naming no source — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore.
- grounding — plan.md says the parser ends `## Instructions` at its first sub-heading, citing no path or symbol — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on repository code whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore.
- grounding — plan.md says rmcp 3.9.0's `Tool.meta` serializes as `_meta`, naming no source — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A library behavior whose re-sourcing needs the library's documentation or source, which is research rather than a chore.
- grounding — plan.md says `apply-manifest` holds both roots inside the directory it runs in, citing no source — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on repository code whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore.
- grounding — plan.md says clap otherwise generates `ductus hook help` itself, naming no source — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A library behavior whose re-sourcing needs the library's documentation or source, which is research rather than a chore.
- grounding — plan.md says implement, review and analyze walk groom's decision tree and derive no routing candidates, citing none of those commands — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on other commands whose re-sourcing needs reading their sources, which is research rather than a chore.
- grounding — plan.md says implement runs five bounded primitives in one walk, citing neither the command nor a walk — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on another command whose re-sourcing needs reading its source or a walk, which is research rather than a chore.
- grounding — plan.md says no command, walker step or script read read-spec's `review` and `analyze`, naming no search — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note whose re-sourcing needs a corpus search outside analyze's scope, which is research rather than a chore.
- grounding — plan.md says the first tool the steps name is not the first call in review, implement or groom, citing neither the commands nor a run — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on other commands whose re-sourcing needs reading their sources or a run, which is research rather than a chore.
- grounding — plan.md says the allow side already ignores a pinned configure copy's edits, citing no code or test — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on repository code whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore.
- grounding — plan.md says an omitted list and an empty one arrive alike over MCP, naming no source — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A protocol and library behavior whose re-sourcing needs their documentation or a run, which is research rather than a chore.
- rule-assessment — BE-METRIC-001: plan.md commits the new request-handling paths, the render-status, render-help, merge-status-hook and read-section handlers and the ductus hook surface, to no RED metrics — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: The runtime is a per-agent stdio process with no metrics pipeline, and no primitive emits metrics; adding the first belongs to a spec about the runtime's observability, as 041's and 062's analyses decided.
- rule-assessment — BE-TRACE-002: plan.md wraps the claude --version probe, externally dependent work, in no span — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: The runtime is a per-agent stdio process with no tracing pipeline; adding the first span belongs to a spec about the runtime's observability, as 022's analysis decided for fetch-archive.
- rule-assessment — BE-REPR-002: data-model.md mixes Claude Code's camelCase hook-output and `_meta` names with ductus's kebab-case fields — `specs/080-deterministic-runtime-efficiency/data-model.md` — **waived**: The camelCase names are Claude Code's hook-output and `_meta` schema, emitted as the host defines them, the names ductus chooses are kebab-case, and the project declares no casing convention (it has no specs/system.md), as 058's and 078's analyses decided.

## Deferred to other specs

*None.*

## Unexamined targets

*None — every target was examined.*

## Fixed in this run

*None.*
