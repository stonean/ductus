---
spec: 041-task-pruning
last-run: 2026-10-27T04:23:55Z
analyzed-against: 88768fe67738169b56f4f60554eb67b383858398
hard-fail: 0
blocking-findings: 0
advisory: 12
unexamined: 14
analyzed-digest:
  data-model.md: 618d6ecf7942d79fb7f7d74344e4e29e790e0f7991044fac25b32621d0f8bda9
  plan.md: 32e671c2ed42e4ca32521e52b34f3ea3b5822bfdd37f44954cf880aefc03e232
  review.md: ee22173d56ef97323c2fca5ec0e2d697fe50d8b69eec29547eec21357beb5a81
  scenarios/a-plan-edit-does-not-reopen-a-done-spec.md: 0cb04f76649997738f1c988046a93e9b2e9e2c7d000f6e0f732940c89e79acdc
  scenarios/plan-records-the-design-as-it-stands.md: d31c168a226de2a29d27b58f1fa9e188c281387cd4340dd5e68ba3eefbc0e30f
  scenarios/prune-all-walks-every-spec.md: 4b605a7c3d07bff5e6a90bd02e982e93f7360d12f3934f9663a0eb44ebc9fc92
  scenarios/prune-reduces-the-spec-directory.md: 7a987ae34945c37103ef63ff84756a64c3ca30ce8fd373ee66e88def90e5c8f3
  spec.md: fe8af8ab096265c5424c867490c037ab50cdc76720279d3fe459a4e559c9fe3e
  tasks.md: 8fc9ad39147f73127de73b6f5be458fd7839764fc550b6213edf6f8b958e729d
unexamined-by-reason:
  root-absent: 13
  ships-to-adopter: 1
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 12
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-METRIC-001: the prune-plan MCP handler commits to no rate, error or duration metrics"
    outcome: discarded
    reason: the runtime is a per-agent stdio process with no metrics pipeline, and no primitive emits metrics; adding the first belongs to a spec about the runtime's observability (as discarded in 062's analysis)
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md gives 842 sections and 161,841 bytes over 60 specs with no command
    outcome: discarded
    reason: a measurement the sentence dates to when the walk was built, kept as the reason the walk lists lines; re-deriving it changes no decision
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md says every back-edge, amend and plan/implement cycle appends tasks
    outcome: discarded
    reason: motivation describing how task lists grow, not a claim the design rests on
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md says ductus has no command to reclaim that space
    outcome: discarded
    reason: motivation stating the gap this spec filled; /prune now exists by this spec
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md says a reset target's commented example headings would parse as phantom tasks without comment-awareness
    outcome: discarded
    reason: the reason for an alignment this spec delivered, held by AC13 and its tests
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md says an adopter tree holds only its own copy of the tasks template
    outcome: discarded
    reason: the bounded divergence AC11 records; prune.md's markdown-only reference states it
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: "grounding — plan.md counts 7 fenced ## lines across 010, 026 and 027 and a code-font comment in 058, with no command"
    outcome: discarded
    reason: a measurement this run re-derived (2026-10-01) naming the files it counts
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says a done spec's artifacts are committed
    outcome: discarded
    reason: the premise of the trigger's absent-at-HEAD rule, a property of the pipeline's completion gate
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says the --all batch shape is the one /analyze --all already has
    outcome: discarded
    reason: a comparison to a command in this repository, named by its invocation
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says a filled tasks.md has already stripped the guidance comment
    outcome: discarded
    reason: rationale for a rejected alternative in Trade-offs
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md describes 022's data model as the registry of check families, at 105 scenarios and 1,529 lines
    outcome: discarded
    reason: the rationale of a decision, dated by this run to when it was made
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says every other tasks primitive trusts checkbox state
    outcome: discarded
    reason: a known limitation stated in Trade-offs, not a claim the design rests on
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
---

# Analysis — 041-task-pruning

## Summary

0 hard-fail, 0 blocking, 12 advisory; not blocking. 14 unexamined target(s). Dispositions: 0 fixed, 0 routed, 12 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-METRIC-001: the prune-plan MCP handler commits to no rate, error or duration metrics — `specs/041-task-pruning/spec.md` — **discarded**: the runtime is a per-agent stdio process with no metrics pipeline, and no primitive emits metrics; adding the first belongs to a spec about the runtime's observability (as discarded in 062's analysis)
- grounding — plan.md gives 842 sections and 161,841 bytes over 60 specs with no command — `specs/041-task-pruning/plan.md` — **discarded**: a measurement the sentence dates to when the walk was built, kept as the reason the walk lists lines; re-deriving it changes no decision
- grounding — spec.md says every back-edge, amend and plan/implement cycle appends tasks — `specs/041-task-pruning/spec.md` — **discarded**: motivation describing how task lists grow, not a claim the design rests on
- grounding — spec.md says ductus has no command to reclaim that space — `specs/041-task-pruning/spec.md` — **discarded**: motivation stating the gap this spec filled; /prune now exists by this spec
- grounding — spec.md says a reset target's commented example headings would parse as phantom tasks without comment-awareness — `specs/041-task-pruning/spec.md` — **discarded**: the reason for an alignment this spec delivered, held by AC13 and its tests
- grounding — spec.md says an adopter tree holds only its own copy of the tasks template — `specs/041-task-pruning/spec.md` — **discarded**: the bounded divergence AC11 records; prune.md's markdown-only reference states it
- grounding — plan.md counts 7 fenced ## lines across 010, 026 and 027 and a code-font comment in 058, with no command — `specs/041-task-pruning/plan.md` — **discarded**: a measurement this run re-derived (2026-10-01) naming the files it counts
- grounding — plan.md says a done spec's artifacts are committed — `specs/041-task-pruning/plan.md` — **discarded**: the premise of the trigger's absent-at-HEAD rule, a property of the pipeline's completion gate
- grounding — plan.md says the --all batch shape is the one /analyze --all already has — `specs/041-task-pruning/plan.md` — **discarded**: a comparison to a command in this repository, named by its invocation
- grounding — plan.md says a filled tasks.md has already stripped the guidance comment — `specs/041-task-pruning/plan.md` — **discarded**: rationale for a rejected alternative in Trade-offs
- grounding — plan.md describes 022's data model as the registry of check families, at 105 scenarios and 1,529 lines — `specs/041-task-pruning/plan.md` — **discarded**: the rationale of a decision, dated by this run to when it was made
- grounding — plan.md says every other tasks primitive trusts checkbox state — `specs/041-task-pruning/plan.md` — **discarded**: a known limitation stated in Trade-offs, not a claim the design rests on

## Unexamined targets

- root-absent: 13
- ships-to-adopter: 1

## Fixed in this run

*None.*
