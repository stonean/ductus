---
spec: 080-deterministic-runtime-efficiency
last-run: 2026-11-11T00:26:02Z
analyzed-against: 963c3bf5a6d6cfcfa228903a50693a53a1f56f74
hard-fail: 0
blocking-findings: 0
advisory: 20
unexamined: 0
analyzed-digest:
  data-model.md: e341b9c089dbcd966a03da2f8d45634e40a3d3ee083ba1091b640111db55a1a6
  plan.md: f92984454930f2fc6d0a65289539a4922a12e86b921066e7ba7256ec08a3233c
  research.md: 2c47cffe0bfeec4b0aeadc1a37ca5eb0f6ff1e67552b7d0ddeac7e5b52ec9ff9
  review.md: cf840ecb1e9a111657a15a3ff7a2ac47c3538f4fc6b6b270ddc189d06323e92b
  scenarios/antigravity-keeps-the-model-path.md: 06e76b9b8d9c68a59edca631b88bfbfaf33ae313974ae52f6a27a894c6fe6be1
  scenarios/configure-types-no-permission-entries.md: 32fa7bcfc2ba3bb064fe3f3bcd6c23ed637340b8319a8b231d1eb3be62ec62ad
  scenarios/groom-derives-routing-candidates.md: 516690b45261a38cad315305e8b1ca19fa51a072137a7bfbb1a12c9217fe42da
  scenarios/help-answers-with-no-model-turn.md: be093aa1eadf2df589f65651f73c79537222b659bdf109c4f07820188bfa5bf5
  scenarios/one-search-covers-referenced-sections.md: 8f024532fe05b863b06fdb6facb8cae8ebe1e979dbfc8fd28035b6336fec97c3
  scenarios/opencode-keeps-the-model-path.md: 753136883e25ef235d7279e3c696f5e8fce5683f4db0dea1717870a85f6083f8
  scenarios/reference-reads-follow-the-cited-sections.md: 66a004c23e55c8e1a4823a5fa13370220af42c42563194fc693c7067998d9c5f
  scenarios/rules-load-without-shell-reads.md: 052d22a0ed61649128bdada4bf6cef556aba1568aa64f6d6055eb5dbe84b92dd
  scenarios/the-search-goes-out-with-the-first-call.md: fb2cfb2453ca185c97d8fd9e81daad0c09fe92280d925c2ee2b52d6273dc000e
  spec.md: 7bad765886a87a8f9d1e14c948f2bd64a42d66978fec2f12efe7f59811ed77d9
  tasks.md: 3170e7c3494d9cb3df45ac24f113bf9eceefe8f892bf375e1c0b627f327c7be5
blocking: false
dispositions:
  fixed: 16
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

0 hard-fail, 0 blocking, 20 advisory; not blocking. 0 unexamined target(s). Dispositions: 16 fixed, 0 deferred, 20 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- artifact-size — `specs/080-deterministic-runtime-efficiency/plan.md` is 63372 bytes, 2 read pages at the 50000-byte read size — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: plan.md is 080's design record, grown by this analysis's rule commitments; its finished Affected Files rows are trimmed with /ductus:prune once 080 is done. (decided by the rule)
- artifact-size — `specs/080-deterministic-runtime-efficiency/tasks.md` is 60296 bytes, 2 read pages at the 50000-byte read size — `specs/080-deterministic-runtime-efficiency/tasks.md` — **waived**: tasks.md holds the out-of-spec findings' disposition tasks until 080 closes; its spent sections are pruned with /ductus:prune once 080 is done. (decided by the rule)
- grounding — spec.md says Claude Code transcripts are removed after `cleanupPeriodDays`, 30 days by default, naming no source — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore. (decided by the rule)
- grounding — spec.md says `MessageDisplay` transforms only assistant text and a `systemMessage` without a block lets the prompt reach the model, naming no source — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore. (decided by the rule)
- grounding — spec.md says Claude Code reads only user, project and local settings, naming no source — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore. (decided by the rule)
- grounding — spec.md says the output schemas never reach the model, naming no source — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore. (decided by the rule)
- grounding — spec.md says Claude Code cuts an upfront-loaded tool's description at 4,096 characters against 16,384 through tool search, naming no source — `specs/080-deterministic-runtime-efficiency/spec.md` — **waived**: A host behavior whose re-sourcing needs the host's documentation or a run, which is research rather than a chore. (decided by the rule)
- grounding — plan.md says the parser ends `## Instructions` at its first sub-heading, citing no path or symbol — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on repository code whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore. (decided by the rule)
- grounding — plan.md says rmcp 3.9.0's `Tool.meta` serializes as `_meta`, naming no source — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A library behavior whose re-sourcing needs the library's documentation or source, which is research rather than a chore. (decided by the rule)
- grounding — plan.md says `apply-manifest` holds both roots inside the directory it runs in, citing no source — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on repository code whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore. (decided by the rule)
- grounding — plan.md says clap otherwise generates `ductus hook help` itself, naming no source — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A library behavior whose re-sourcing needs the library's documentation or source, which is research rather than a chore. (decided by the rule)
- grounding — plan.md says implement, review and analyze walk groom's decision tree and derive no routing candidates, citing none of those commands — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on other commands whose re-sourcing needs reading their sources, which is research rather than a chore. (decided by the rule)
- grounding — plan.md says implement runs five bounded primitives in one walk, citing neither the command nor a walk — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on another command whose re-sourcing needs reading its source or a walk, which is research rather than a chore. (decided by the rule)
- grounding — plan.md says no command, walker step or script read read-spec's `review` and `analyze`, naming no search — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note whose re-sourcing needs a corpus search outside analyze's scope, which is research rather than a chore. (decided by the rule)
- grounding — plan.md says the first tool the steps name is not the first call in review, implement or groom, citing neither the commands nor a run — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on other commands whose re-sourcing needs reading their sources or a run, which is research rather than a chore. (decided by the rule)
- grounding — plan.md says the allow side already ignores a pinned configure copy's edits, citing no code or test — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A design note on repository code whose re-sourcing needs a source read outside analyze's scope, which is research rather than a chore. (decided by the rule)
- grounding — plan.md says an omitted list and an empty one arrive alike over MCP, naming no source — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: A protocol and library behavior whose re-sourcing needs their documentation or a run, which is research rather than a chore. (decided by the rule)
- rule-assessment — BE-METRIC-001: plan.md commits the new request-handling paths, the render-status, render-help, merge-status-hook and read-section handlers and the ductus hook surface, to no RED metrics — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: The runtime is a per-agent stdio process with no metrics pipeline, and no primitive emits metrics; adding the first belongs to a spec about the runtime's observability, as 041's and 062's analyses decided. (decided by the rule)
- rule-assessment — BE-TRACE-002: plan.md wraps the claude --version probe, externally dependent work, in no span — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: The runtime is a per-agent stdio process with no tracing pipeline; adding the first span belongs to a spec about the runtime's observability, as 022's analysis decided for fetch-archive. (decided by the rule)
- rule-assessment — BE-REPR-002: data-model.md mixes Claude Code's camelCase hook-output and `_meta` names with ductus's kebab-case fields — `specs/080-deterministic-runtime-efficiency/data-model.md` — **waived**: The camelCase names are Claude Code's hook-output and `_meta` schema, emitted as the host defines them, the names ductus chooses are kebab-case, and the project declares no casing convention (it has no specs/system.md), as 058's and 078's analyses decided. (decided by the rule)

## Deferred to other specs

*None.*

## Unexamined targets

*None — every target was examined.*

## Fixed in this run

- rule-assessment — BE-TIMEOUT-001: plan.md named no bound for the version probe, the generator's runtime calls, the cost harness's children or the bridge's calls; naming the generator's renamed DUCTUS_GEN_TIMEOUT_S to DUCTUS_GEN_TIMEOUT_SECONDS for CFG-ENV-004 (owner, 2026-11-10) — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed**
- rule-assessment — CFG-CONST-003: plan.md gave the hook entry's 10 s timeout as a literal with no named constant — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
- rule-assessment — CFG-CONST-001: plan.md named no single definition for the hook event name and CLAUDE_PROJECT_DIR, which two modules use — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
- rule-assessment — CFG-ENV-001: plan.md committed CLAUDE_PROJECT_DIR, DUCTUS_BIN and the generator timeout to no read-once resolution or default constant — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed**
- rule-assessment — CFG-ENV-008: plan.md named no type or fail-fast for DUCTUS_BIN and the generator timeout — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-COMPAT-001: plan.md classified `_meta` on tools/list and discover-rule-files' `unreadable` neither additive nor breaking — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-INPUT-004: plan.md committed merge-status-hook's, read-section's and extra-files' paths and the batching line's reference paths to no canonical containment check — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-INPUT-003: plan.md described no safe invocation for the version probe, the generator's runtime calls, the harness's claude runs or the hook entry's command line — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-INPUT-001: plan.md stated no refusal for a malformed settings file or `additionalDirectories` — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-INPUT-011: the pipeline view rendered spec directory names, scenario file names and the namespace unflattened and kept tabs, and plan.md committed to no safeguard (owner, 2026-11-10) — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed**
- rule-assessment — CFG-ENV-002: plan.md's Affected Files row for docs/runtime.md named only CLAUDE_PROJECT_DIR of the three variables plan.md introduces — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
- rules-listing — discover-rule-files' listing, which analyze reads instead of the rule files, and the exec walker's requests cut BE-API-001's Verification at its paragraph, dropping its header table (owner, 2026-11-10) — `specs/080-deterministic-runtime-efficiency/scenarios/rules-load-without-shell-reads.md` — **fixed**
- grounding — plan.md's added sentence said flatten_line covers every file-derived view value, which spec directory names, scenario file names and the namespace bypassed — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed**
- grounding — plan.md's added sentence said the version probe runs through no shell, which Windows' claude.cmd fallback does through cmd.exe — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md's added refusal, containment, namespace and precedent claims cited no file or symbol — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md's added hook-entry sentence said how the host's shell expands the command, citing no source — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
