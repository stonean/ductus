# 046 — Scenario open-question visibility Plan

Implements [046 — Scenario open-question visibility](spec.md).

## Overview

Scenario open questions become a first-class signal that is **separate from** the spec body's count, gates `done`, and surfaces as remaining work — while resolution stays scenario-targeted and unchanged.

Per the spec's Implementation ownership section, the work splits across two specs. This plan sequences both, but only the constitution amendments and this spec's own artifacts are authored here; the primitive and command-doc changes are authored as scenarios under [022 — Deterministic Runtime](../022-deterministic-runtime/spec.md), each back-linking to this spec.

The critical path is the parser fix. Everything else reads the count the parser produces, so it lands first and alone.

## Technical Decisions

### The parser fix is a prerequisite, not a parallel task

`parse_open_questions` (`runtime/src/primitives/read_spec.rs:141`) walks `section_lines`, which is not comment- or fence-aware; the acceptance-criteria parser at `read_spec.rs:110` walks `section_line_indices`, which is. Verified by scaffolding a spec from `framework/templates/spec/spec.md` and calling `read-spec`: `acceptance-criteria` returned `[]` while `open-questions` returned the template's three commented-out examples.

Under this feature a phantom question blocks `done`, so the fix must land before any consumer reads the count. It is task 1, and no other task starts until it is green.

The placeholder guard (`read_spec.rs:168`) widens in the same change: it currently exact-matches `*None — all resolved.*`, while `create_scenario.rs:69` emits `*None — captured during scenario authoring.*`. Both become members of a skip set.

**Blast radius.** `parse_open_questions` is `pub(crate)` and shared with `append-question`'s dedup, so the fix changes both readers at once — which is the point: they must agree. Existing specs whose Open Questions sections contain no commented-out bullets see no change in reported count.

### Scenario questions are a sibling field, never merged

`read-spec`'s result gains a field parallel to `open-questions`, each entry carrying its source scenario. `open-questions` itself is untouched — the spec-body count keeps its current meaning, its current value, and its current routing effect. This is what makes feature-targeted `/{project}:clarify` unchanged: it reads the field it always read.

Rejected: unioning into `open-questions`. It would route a feature-level target to feature-targeted clarify, which does not read scenarios (`framework/commands/clarify.md:43`), producing a command with nothing to act on.

### The gate extends `check-review-gate` rather than adding a primitive

`check-review-gate` (`runtime/src/primitives/check_review_gate.rs`) already owns the pre-done gate: ordered checks, first failure wins, canonical `blocked:` message, every verdict a domain outcome rather than an error. The scenario check becomes a third `blocked-by` variant between `markdown-lint` and the `review:` block, per the spec's ordering decision.

This is a strictly additive change to a closed enum and its message set — no new primitive, no new call site in `/{project}:implement` (step 13 already invokes it).

### Detection mirrors review-state drift exactly

`check-artifacts` gains a `scenario-open-questions` family alongside its existing four (`check_artifacts.rs:139,159,224,320`). It is blocking on a `done` spec and advisory otherwise, and `--fix` reverts `done → in-progress` through `set-status` with the same non-silent notice the review-state-drift revert emits.

No grandfather clause, unlike review-state drift. An absent `review:` block genuinely marks a spec as predating its feature; an unresolved scenario question is a present-tense defect whenever it arrived.

### Rendering reuses the recovery-state shapes

The dashboard already carries the two shapes needed: a Next Action override and a callout naming specs. The Scenarios column gains a suffix rather than the table gaining a column, so the glance stays glanceable. Recovery state wins the Next Action cell when both apply; both callouts render.

### No `data-model.md` for this feature

The schema changes are primitive request/response shapes, whose canonical source is [022's data-model](../022-deterministic-runtime/data-model.md) — `read-spec` at its line 128, `check-review-gate` at 498, `check-artifacts` at 598, `dashboard` at 408. Restating them here would create a second source for one fact, which §drift-prevention forbids. The 022 scenarios update those sections in place.

### Constitution amendments stay with this spec

`§spec-lifecycle`'s `done` row (`framework/constitution.md:113`) and `§readiness-check`'s "All open questions are resolved" (`:174`) both currently read as spec-body-only. They are amended here, not in 022 — 022 owns the runtime, not the lifecycle definition.

## Affected Files

Authored under this spec:

| File | Action | Purpose |
| --- | --- | --- |
| `framework/constitution.md` | Modify | §spec-lifecycle `done` row and §readiness-check include scenario questions |
| `specs/046-scenario-open-question-visibility/tasks.md` | Modify | Track this feature's work |

Authored as 022 scenarios (each back-linking here):

| File | Action | Purpose |
| --- | --- | --- |
| `specs/022-deterministic-runtime/scenarios/scenario-question-parser-fix.md` | Create | Comment-aware parsing + widened placeholder guard |
| `specs/022-deterministic-runtime/scenarios/scenario-open-question-signal.md` | Create | The sibling field, the gate check, the finding family, the rendering |
| `specs/022-deterministic-runtime/data-model.md` | Modify | New field on `read-spec`; new `blocked-by` variant; fifth `check-artifacts` family |
| `runtime/src/primitives/read_spec.rs` | Modify | Comment-aware parser, placeholder set, scenario-question field |
| `runtime/src/primitives/create_scenario.rs` | Modify | Placeholder string alignment (if the skip set is not preferred) |
| `runtime/src/primitives/check_review_gate.rs` | Modify | Third ordered check |
| `runtime/src/primitives/check_artifacts.rs` | Modify | `scenario-open-questions` family |
| `runtime/src/primitives/dashboard.rs` | Modify | Column suffix, Next Action override, callout |
| `framework/commands/target.md` | Modify | Feature-level readout of outstanding scenario questions |
| `framework/commands/status.md` | Modify | Column, override, callout rendering rules |
| `framework/commands/implement.md` | Modify | Completion-gate step names the third check |
| `framework/commands/analyze.md` | Modify | New family in the markdown-only reference + `--fix` behavior |

## Trade-offs

**Two specs instead of one.** The requirement and its implementation live apart, so a reader of 022's scenarios must follow a back-link to learn why. Accepted: 022 is the canonical owner of primitive behavior, and duplicating the rationale would drift. The back-links are the mitigation.

**046 cannot reach `done` before 022's scenarios do.** Its criteria are verified against behavior 022 produces. This is real coupling, not an artifact of how it was written — and adding those scenarios to a `done` 022 reopens it to `in-progress`, exercising the very back-edge this spec's first resolved question affirms.

**Fixing a pre-existing parser bug inside a feature change.** The comment-awareness fix is not strictly part of "make scenario questions visible" and could have been a standalone bug fix. Folded in at the user's direction because the gate is unsafe to ship without it. Cost: the diff carries a change whose motivation lives in this spec rather than in a bug report.

**The gate blocks on a condition the contributor may not have authored.** A scenario written by someone else can block your `done`. Accepted — that is the intended semantics of "the spec is not complete while its scenarios carry questions" — but it makes the blocked message's scenario naming load-bearing rather than cosmetic.

**Rejected: a separate `check-scenario-questions` primitive.** Cleaner isolation, but it would add a second call site to the completion gate and a second place for the "is it blocked?" question to be answered. Extending `check-review-gate` keeps one gate, one verdict.

**Rejected: making the parser fix its own spec.** Correct in principle and it would let the bug fix ship immediately, but it puts a hard dependency between two unshipped specs for a change that is a few lines. Reconsider if the parser fix proves larger than task 1 assumes.
