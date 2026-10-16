---
title: "010-agent-autonomy — spec"
status: done
dependencies: [000-slash-commands]
tags: [agent, process]
next-criterion: 15
---

# 010 — Agent Autonomy

Evaluate capabilities found in autonomous agent orchestration tools (e.g., GSD-2) and determine which can be adopted within `ductus`'s constraints: zero dependencies, markdown-only artifacts, platform-agnostic, and human-in-the-loop pipeline gates.

> **Note (post-completion, [043-workflows-sunset](../043-workflows-sunset/spec.md)):** the `skills/` → `workflows/` rename this spec delivered was itself sunset — the workflows feature no longer exists, and the `workflows-sunset` migration cleans up both directory generations in adopter projects. Body references to workflows below are historical.

Each capability is evaluated independently. The outcome for each is one of: **adopt** (add to `ductus`), **adapt** (modify the concept to fit `ductus`'s model), or **decline** (not a fit).

## Completion record (2026-08-17)

**Closed.** Task 17 — the next-step offer — shipped in `ductus-v0.29.2`, and the
scenario's open question was resolved by the operator: **name one task, and say
how many remain after it.** A menu re-renders a slice of `tasks.md` after every
completion; the count recovers most of its value, and looking further ahead is a
free-text redirect like any other. The reasoning is in the scenario's Resolved
Questions.

What actually held the spec open at the end was a stale review:
[`implement-offers-the-next-step`](scenarios/implement-offers-the-next-step.md)
was edited after the recorded `reviewed-against`, a real contract change rather
than a rename, so the mechanical-sweep exemption correctly did not apply. The
closing `/{project}:review` run reported one advisory `QUAL-CLAIM-001` against
`SweepIndex::build` — the staleness machinery itself — which was **fixed** rather
than carried, per this repo's rule that a SHOULD gates `done` exactly as a MUST
does. That fix shipped as `ductus-v0.29.3`.

Three acceptance criteria are annotated **superseded** in place: AC7 and AC8
(the `[simple]` marker and its `/{project}:plan` proposal step, dropped by
`017-derive-dont-ask`) and half of AC14 (`framework/workflows/`, sunset by
`043-workflows-sunset`). They stay checked because 010 delivered them; the
removals belong to the later specs. The annotations exist because nothing
detects a later spec invalidating an earlier one's criterion — an inbox item
records that gap.

## Capabilities Under Evaluation

### Skills system

Composable, context-specific instruction sets loaded based on task type. Currently `ductus` uses a single monolithic `AGENTS.md` per project. A skills system would split this into reusable skill files that commands selectively reference.

- Current state: `AGENTS.md` covers all conventions in one file
- GSD-2 approach: 16 bundled skill profiles loaded by task context
- `ductus` opportunity: skill files in a `skills/` directory, selectively `@import`-ed by commands or CLAUDE.md based on task type

**Verdict: adapt.** Adopt Anthropic/Claude Code's "skills" terminology for context-loaded instruction packs (the only mainstream vendor using the term — they use it for agent-loaded specialized capabilities, matching this concept). Layering, not replacement: `AGENTS.md` stays as the always-loaded baseline; the `AGENTS.md` template gains an optional "Skills" index section listing available skill files and the task types/topics that activate them. `ductus` documents the pattern and the per-platform mapping (Claude Code skills, Cursor rules, etc.) but does not prescribe a fixed directory or ship platform-specific files. Adopters who don't decompose leave the index empty.

Cross-spec impact on 005: 005's "skills" are functionally tech-stack-conditional development **workflows** (lint, test, format, migrate — scaffolded as slash commands into `.claude/commands/{slug}/`), not skills in the Anthropic sense. To free the term for 010's use, 005's concept is renamed to "workflows" — see Acceptance Criteria.

### Complexity routing

Classify tasks by complexity to inform model selection and batching. Currently `ductus` treats all tasks uniformly regardless of scope.

- Current state: lightweight-vs-standard track decision at spec level only
- GSD-2 approach: simple/standard/complex classification with automatic model routing
- `ductus` opportunity: complexity field in `tasks.md` entries; `implement` command uses this to suggest model or batch simple tasks

> **Note (post-completion, `017-derive-dont-ask`):** the `[simple]` marker this section adopts was delivered and then dropped — it exists nowhere under `framework/` today, and neither does the `/ductus:plan` proposal step. AC7 and AC8 below carry the same record. The verdict as written is historical.

**Verdict: adapt.** Cost matters and is rising; platform-level autorouting is opaque and unpredictable. Add an optional inline `[simple]` marker on tasks (no marker = default tier — whatever the adopter's platform config maps to "standard"). `/ductus:plan` proposes the marker on tasks it judges trivial; the user may add or remove markers during review (mirrors how the lightweight-track decision works). `/ductus:implement` reads the marker and surfaces a suggested model — does not auto-switch, since platforms differ in how model selection is exposed and the user remains in the loop.

Durable signal vs. volatile mapping: the marker (`[simple]`) lives in `tasks.md` and never changes with model releases; the per-platform mapping (which model "simple" routes to today) lives in adopter config (`AGENTS.md` or `system.md`), which adopters update as models evolve. `ductus` defines only the marker.

A `[complex]` tier was considered and declined — the default is already "use the strongest model," so a `complex` marker would not change behavior. May be added later if a concrete need arises.

### Stuck detection

Detect when an agent is cycling on the same task without progress. Currently `ductus` has no mechanism to identify repeated failed attempts.

- Current state: no detection — user must notice manually
- GSD-2 approach: sliding-window analysis of dispatch history catches cycles
- `ductus` opportunity: append-only execution log per spec; `implement` command checks for repeated attempts and suggests decomposition

**Verdict: decline the artifact, adopt the behavior.** No execution log file. An append-only markdown log duplicates information already encoded in `git log`, creates per-invocation git churn that fights the text-first-artifacts principle (artifacts should change with intent), introduces merge-conflict surface on parallel work, and gets out of sync the moment commits are squashed or rebased. Platform transcripts (Claude Code session history, Cursor chat) already capture richer signal than a markdown log can carry.

The stuck-detection *behavior* is worth adopting using existing signals: `/ductus:implement` instructions gain a step that reads `git log` for `tasks.md` and the current `tasks.md` checkbox state to detect when a task has been touched across N invocations without flipping to `[x]`. When detected, surface to the user and suggest decomposition. No new file, no schema, no git noise. (This sentence read *"for the affected paths"* until 2026-09-13; the plan narrowed the signal to `tasks.md` at plan time and said why — see its §Trade-offs — and that is what shipped.)

### Autonomous execution

Chain task execution without pausing for user approval between individual tasks within a phase. Currently every status change requires explicit approval.

- Current state: user approves every transition
- GSD-2 approach: full auto mode — agent loops until milestone is complete
- `ductus` opportunity: auto-advance between tasks within a phase while preserving approval gates at phase transitions (planned→done)

**Verdict: adapt.** Opt-in via `/ductus:implement --auto`, default off. Per-invocation flag (not session state, not spec frontmatter) — decision happens in execution context, not selection context, and matches CLI convention (flags modify the verb). The session file does not gain an `autoAdvance` field.

Constraints that still gate even with `--auto` on:

- Phase completion transition (`in-progress`→`done`) — per §pipeline-boundaries, unchanged. (The `planned`→`in-progress` transition is no longer gated: invoking `/ductus:implement` is itself the approval to start work, so it never prompts, with or without `--auto`. Superseded by [000-slash-commands](../000-slash-commands/spec.md)'s `implement-skips-planned-prompt` scenario per §cross-spec-impact.)
- Stuck-detection events (from the Stuck detection adoption above) — auto mode does not power through cycles.
- Spec edits, plan edits, or new tasks discovered mid-implement.
- Risky actions per the agent's safety rules (destructive ops, secrets, force pushes, etc.).

Without the flag, behavior is unchanged: user confirms each task. With the flag, `/ductus:implement` runs tasks in order, marks them complete, and advances within the current phase until one of the gates above fires.

### Parallel milestones

Work on multiple features concurrently with isolated git state. Currently `ductus` targets one feature at a time via a single session target.

- Current state: single-feature session (`session.json` holds one target)
- GSD-2 approach: multiple worktrees running concurrently with file-based coordination
- `ductus` opportunity: multi-target session, `--feature` flag on commands, guidance for worktree-based isolation

**Verdict: decline.** Single-target sessions stay. The pipeline is serial within a feature by design, and the rare case of working on two truly independent features at once is best served by two independent sessions in two terminals — git and the agent platform already provide isolation (`git worktree`, Claude Code's `isolation: "worktree"` agent parameter, Cursor's worktree integration). Multi-target session state would invite ambiguity (which target does the next command operate on? what if two targets have conflicting cross-spec impact?) without solving a real problem `ductus` has. Worktree workflow itself is git's job, not `ductus`'s.

The session file keeps holding one target. Documentation gains a one-paragraph note (in the constitution or `AGENTS.md` template) directing users to `git worktree` and platform isolation for concurrent feature work.

> **Note (post-completion, superseded by [062-concurrent-session-targets](../062-concurrent-session-targets/spec.md), 2026-09-29):** this verdict no longer holds. It rested on two premises that did not survive use: running two or more agent processes at once turned out to be the common case, not the rare one, and the worktree remedy does not put two worktrees on the same branch, which trunk-based work needs — by default `git worktree add` refuses a branch already checked out by another worktree (git-worktree(1), under `--force`). 062 gives each agent process its own session target — keyed by `DUCTUS_SESSION` or the agent's own session id, with `.ductus/session.toml` kept as the shared default — and answers the ambiguity this verdict declined over by keeping one target *per process*: "which target does the next command operate on?" still has exactly one answer. `git worktree` stays the answer for isolating edits, and the constitution's §concurrent-features now says so. The verdict and its reasoning are left above as the record of the decision as it was made.

### Cost controls

Track token usage and enforce budget limits. Currently `ductus` has no cost awareness.

- Current state: no cost tracking — delegated entirely to the AI platform
- GSD-2 approach: per-task token/cost metrics, budget ceilings with warnings/pauses/halts
- `ductus` opportunity: unclear — `ductus` has nothing that observes a model call; this may remain the platform's responsibility

**Verdict: decline budget tooling, adopt a documentation cross-reference.** Per-task token tracking and budget ceilings require instrumenting the model call itself — that part is platform-level by definition. But several decisions in this spec are themselves cost-aware patterns; the remaining work is to name them and point at platform tooling for the runtime piece.

> **Note (post-completion, premise corrected 2026-09-13):** this verdict was argued in 2026 as *"require a runtime `ductus` does not have"*, which was true before spec 022. `ductus` now ships a required runtime ([§runtime-boundary](../../framework/constitution.md#runtime-boundary)), so the premise moved — but **the decision stands and is now better founded**: principle 2 forbids that runtime from calling an LLM, so it can never observe token usage. The wording was corrected here and in the constitution's `### Cost levers` paragraph in the same change.

The "Cost-conscious" principle already exists in the constitution (§principles → Business). It currently has no operational guidance. Add a short cross-reference paragraph (location TBD during planning — likely in the constitution near the principle, or in the `AGENTS.md` template) that names `ductus`'s existing cost levers:

- Lightweight track — skip the plan phase for small features — **removed by `023-govern-refinement`**
- `[simple]` tier marker on tasks (Q2 above) — route trivial work to cheaper models — **removed by `017-derive-dont-ask`**
- Stuck detection (Q3 above) — catch runaway loops before they compound spend
- Default-off autonomy (Q4 above) — human-in-the-loop gating contains blast radius

…and points at the adopter's platform (Claude Code's `/cost`, Anthropic usage dashboard, Cursor's request limits, etc.) for runtime cost controls.

No new artifact, no per-task estimates, no budget files. The cross-reference paragraph is the entire deliverable.

## Acceptance Criteria

### Evaluation completeness

- [x] AC1: Each capability has a recommendation: adopt, adapt, or decline
- [x] AC2: Adopted/adapted capabilities have a clear description of what changes to `ductus` artifacts
- [x] AC3: Declined capabilities have a rationale explaining why they don't fit
- [x] AC4: No capability introduces a runtime dependency or requires a specific AI platform
- [x] AC5: Changes respect command file parity (`framework/commands/` and `.claude/commands/ductus/`)
- [x] AC6: Changes respect bootstrap installer parity (`framework/bootstrap/` variants stay in sync)

### Concrete deliverables (from adapted capabilities)

- [x] AC7: `tasks.md` template documents the optional `[simple]` inline marker convention (one tier; no marker = default) — **superseded, delivered then removed**: the `[simple]` marker no longer exists anywhere under `framework/`, dropped by `017-derive-dont-ask`. Checked because 010 did deliver it; the removal is 017's, and 017 declares no dependency on 010, so this line is the only thing tying the two together.
- [x] AC8: `/ductus:plan` command instructions include a step to propose `[simple]` markers on tasks the agent judges trivial — **superseded, delivered then removed**: the proposal step no longer exists, dropped by `017-derive-dont-ask` alongside the marker in AC7.
- [x] AC9: `/ductus:implement` command instructions include a stuck-detection step that reads `git log` for `tasks.md` and its checkbox state, surfaces cycles, and suggests decomposition — this line read *"`git log` for affected paths"* until 2026-09-13; the plan declined affected-files history at plan time (§Trade-offs, *Stuck detection reads `tasks.md` commits, not affected-files commits*) because that history is noisy and would force the implementation to parse the plan's affected-files table, and neither the shipped step nor `check-stuck` ever read it. The criterion is corrected to the mechanism that was built, not annotated: nothing superseded it, the anticipated one was never delivered.
- [x] AC10: `/ductus:implement` command accepts an `--auto` flag that skips per-task confirmations within a phase, with the documented gates (phase transitions, stuck detection, spec/plan edits, mid-implement discovery, risky actions) still firing
- [x] AC11: Constitution `## Guiding Principles` → `Cost-conscious` (or a new dedicated subsection) gains a cross-reference paragraph naming `ductus`'s cost levers (lightweight track, `[simple]` marker, stuck detection, default-off autonomy) and pointing at platform tooling for runtime controls — **partially superseded, delivered then reduced**: `### Cost levers` exists and names **two** of the four. 010 delivered all four in `fc68cf1c`; `017-derive-dont-ask` removed the `[simple]` lever (`fc737946`, the same removal AC7 and AC8 record) and `023-govern-refinement` removed the lightweight track along with the section it linked (`dd8f7b41`). Checked because 010 delivered the enumeration as specified; the reductions belong to 017 and 023, neither of which declares 010.
- [x] AC12: `AGENTS.md` project template gains an optional "Skills" index section listing available skill files and their activation conditions (empty by default)
- [x] AC13: Documentation note added (constitution or `AGENTS.md` template) directing users to `git worktree` and platform isolation for concurrent feature work

### Cross-spec deliverable

- [x] AC14: If the skills capability is delivered, 005's concept is renamed from "skills" to "workflows" (cross-spec impact: reopens 005 to `in-progress` per §cross-spec-impact). Affected paths in `ductus`: `framework/skills/` → `framework/workflows/` (flattened — registry and workflow files sit at the same level, no inner `templates/` directory), `skills/registry.json` → `workflows/registry.json`, `specs/005-skills-and-plugins/` → `specs/005-workflows/` (spec directory rename), and prose updates in 005's spec, plan, tasks, and any project templates that reference the term. — **partially superseded**: the `specs/005-workflows/` rename stands, but `framework/workflows/` no longer exists, sunset by `043-workflows-sunset` (which does declare 010, so that half is traceable from its frontmatter). Checked because 010 delivered the rename as specified; the later sunset is 043's.

## Open Questions

(none — all resolved; see Resolved Questions below)

## Resolved Questions

1. **Skills system: replace `AGENTS.md` or layer on top?** — Layer on top. `AGENTS.md` remains the always-loaded baseline; the template gains an optional "Skills" index section listing available skill files and the task types/topics that activate them (empty by default). Verdict for the capability is **adapt**: `ductus` documents the pattern and per-platform mapping (Claude Code skills, Cursor rules, etc.) without prescribing a fixed directory or shipping platform-specific files. Terminology aligns with Anthropic/Claude Code's "skills" — the only mainstream vendor using the term, and they use it for context-loaded instruction packs (matching this concept). 005's "skills" are functionally tech-stack-conditional development workflows (lint, test, format, migrate); per §cross-spec-impact, 005 is renamed to "workflows" so 010 can use the standard term — see Acceptance Criteria.
2. **Complexity routing: who assigns?** — Hybrid: `/ductus:plan` proposes, user may override. Granularity: single optional `[simple]` inline marker on tasks (no marker = default tier). Verdict for the capability is **adapt**. Rationale: agentic-coding cost is rising and platform autorouting is opaque, so an explicit signal in `tasks.md` gives the user visibility and control. The marker is the durable signal; the per-platform model mapping ("simple" → which model today) lives in adopter config so model churn doesn't rot specs. `[complex]` tier declined as redundant against the default.
3. **Execution log: worth maintaining?** — No. Verdict for the capability is **decline the artifact, adopt the behavior**. No persisted execution log: an append-only markdown file duplicates `git log`, creates per-invocation git churn that fights text-first-artifacts (artifacts should change with intent), and goes out of sync on squash/rebase. The stuck-detection *behavior* is adopted using existing signals — `/ductus:implement` reads `git log` for `tasks.md` and its checkbox state to detect tasks touched across N invocations without completing, then surfaces the cycle and suggests decomposition. (Read *"for affected paths"* until 2026-09-13; the plan declined affected-files history at plan time, and `tasks.md` is what shipped — see AC9.)
4. **Autonomous execution: opt-in or default?** — Opt-in via `/ductus:implement --auto`, default off. Verdict for the capability is **adapt**. Per-invocation flag rather than session state or spec frontmatter — autonomy is an execution-time decision, not a selection-time or spec-level property, and putting it on the verb that does work matches CLI convention. The session file does not gain an `autoAdvance` field. Phase boundaries, stuck-detection events, spec/plan edits, mid-implement task discovery, and risky actions all continue to gate even with the flag on; the flag only skips per-task confirmations within a phase. Default-off matches the constitutional spirit (§pipeline-boundaries) of keeping the human in the loop unless explicitly opted out.
5. **Parallel milestones: prescribe worktree management?** — No. Verdict for the capability is **decline** — **superseded by 062-concurrent-session-targets**, which gives each agent process its own session target; see the note under §Parallel milestones. Single-target sessions stay. The pipeline is serial within a feature by design; concurrent work on independent features uses independent sessions in independent terminals, with isolation provided by `git worktree` and platform features (Claude Code's `isolation: "worktree"`, Cursor's worktree integration). Multi-target session state would introduce ambiguity (which target does the next command operate on?) without solving a real `ductus` problem. Documentation gains a one-paragraph note pointing users at the platform/git mechanisms — staying focused on one spec at a time supports parallel work via separate sessions, not via in-session multi-targeting.
6. **Cost guidance beyond platform controls?** — Yes, but only as documentation. Verdict for the capability is **decline budget tooling, adopt a documentation cross-reference**. Per-task token tracking and budget ceilings require instrumenting the model call itself, which `ductus`'s runtime is forbidden to do. The "Cost-conscious" constitutional principle (§principles → Business) gains a short paragraph naming `ductus`'s existing cost levers (lightweight track, `[simple]` tier marker, stuck detection, default-off autonomy) and pointing at the adopter's platform tooling for runtime controls. No new artifact, no per-task estimates, no budget files. **The decision stands; two of the four levers do not** — see AC11 and the §Cost controls note above.

## References

Declared dependencies for this spec, surfaced here so the `derive-dependencies` runtime primitive sees them in the body.

- [000-slash-commands](../000-slash-commands/spec.md)
