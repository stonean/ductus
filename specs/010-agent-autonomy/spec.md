---
title: "010-agent-autonomy — spec"
status: done
dependencies: [000-slash-commands]
tags: [agent, process]
next-criterion: 15
---

# 010 — Agent Autonomy

Evaluate capabilities found in autonomous agent orchestration tools (e.g., GSD-2) and determine which can be adopted within `ductus`'s constraints: zero dependencies, markdown-only artifacts, platform-agnostic, and human-in-the-loop pipeline gates.

Each capability is evaluated independently. The outcome for each is one of: **adopt** (add to `ductus`), **adapt** (modify the concept to fit `ductus`'s model), or **decline** (not a fit). Each capability below records the state of `ductus` when it was evaluated, the verdict, and what of the verdict holds today.

## Capabilities Under Evaluation

### Skills system

Composable, context-specific instruction sets loaded based on task type. When evaluated, `ductus` used a single monolithic `AGENTS.md` per project, and a skills system would split it into reusable skill files that commands selectively reference.

- State when evaluated: `AGENTS.md` covered all conventions in one file
- GSD-2 approach: 16 bundled skill profiles loaded by task context
- `ductus` opportunity: skill files in a `skills/` directory, selectively `@import`-ed by commands or CLAUDE.md based on task type

**Verdict: adapt.** Adopt Anthropic/Claude Code's "skills" terminology for context-loaded instruction packs (the only mainstream vendor using the term — they use it for agent-loaded specialized capabilities, matching this concept). Layering, not replacement: `AGENTS.md` stays as the always-loaded baseline; the `AGENTS.md` template gains an optional "Skills" index section listing available skill files and the task types/topics that activate them. `ductus` documents the pattern and the per-platform mapping (Claude Code skills, Cursor rules, etc.) but does not prescribe a fixed directory or ship platform-specific files. Adopters who don't decompose leave the index empty.

### Complexity routing

Classify tasks by complexity to inform model selection and batching. When evaluated, `ductus` treated all tasks uniformly regardless of scope.

- State when evaluated: a lightweight-vs-standard track decision at spec level only
- GSD-2 approach: simple/standard/complex classification with automatic model routing
- `ductus` opportunity: complexity field in `tasks.md` entries; `implement` command uses this to suggest model or batch simple tasks

**Verdict: decline.** `ductus` has no complexity routing: no `[simple]` marker on tasks and no `/ductus:plan` proposal step exist under `framework/`. Cost matters and is rising, but platform-level autorouting is opaque and unpredictable, and an inline tier marker is an input an author must remember to set, with no viable derivation, so it fails silently exactly when it matters (`017-derive-dont-ask`'s discipline principle). A `[complex]` tier would be redundant too — the default is already "use the strongest model."

### Stuck detection

Detect when an agent is cycling on the same task without progress. When evaluated, `ductus` had no mechanism to identify repeated failed attempts.

- State when evaluated: no detection — the user had to notice manually
- GSD-2 approach: sliding-window analysis of dispatch history catches cycles
- `ductus` opportunity: append-only execution log per spec; `implement` command checks for repeated attempts and suggests decomposition

**Verdict: decline the artifact, adopt the behavior.** No execution log file. An append-only markdown log duplicates information already encoded in `git log`, creates per-invocation git churn that fights the text-first-artifacts principle (artifacts should change with intent), introduces merge-conflict surface on parallel work, and gets out of sync the moment commits are squashed or rebased. Platform transcripts (Claude Code session history, Cursor chat) already capture richer signal than a markdown log can carry.

The stuck-detection *behavior* is adopted using existing signals: `/ductus:implement` reads `git log` for `tasks.md` and the current `tasks.md` checkbox state to detect when a task has been touched across N invocations without flipping to `[x]`. When detected, it surfaces the cycle to the user and suggests decomposition. No new file, no schema, no git noise.

### Autonomous execution

Chain task execution without pausing for user approval between individual tasks within a phase. When evaluated, every status change required explicit approval.

- State when evaluated: the user approved every transition
- GSD-2 approach: full auto mode — agent loops until milestone is complete
- `ductus` opportunity: auto-advance between tasks within a phase while preserving approval gates at phase transitions (planned→done)

**Verdict: adapt.** Opt-in via `/ductus:implement --auto`, default off. Per-invocation flag (not session state, not spec frontmatter) — decision happens in execution context, not selection context, and matches CLI convention (flags modify the verb). The session file does not gain an `autoAdvance` field.

Constraints that still gate even with `--auto` on:

- Phase completion transition (`in-progress`→`done`) — per §pipeline-boundaries, unchanged. (The `planned`→`in-progress` transition is not gated: invoking `/ductus:implement` is itself the approval to start work, so it never prompts, with or without `--auto`, per [000-slash-commands](../000-slash-commands/spec.md)'s `implement-skips-planned-prompt` scenario.)
- Stuck-detection events (from the Stuck detection adoption above) — auto mode does not power through cycles.
- Spec edits, plan edits, or new tasks discovered mid-implement.
- Risky actions per the agent's safety rules (destructive ops, secrets, force pushes, etc.).

Without the flag, behavior is unchanged: user confirms each task. With the flag, `/ductus:implement` runs tasks in order, marks them complete, and advances within the current phase until one of the gates above fires.

### Parallel milestones

Work on multiple features concurrently with isolated git state. When evaluated, `ductus` targeted one feature at a time via a single session target.

- State when evaluated: single-feature session (`session.json` held one target)
- GSD-2 approach: multiple worktrees running concurrently with file-based coordination
- `ductus` opportunity: multi-target session, `--feature` flag on commands, guidance for worktree-based isolation

**Verdict: decline multi-target session state within one process; each agent process holds its own session target.** The pipeline is serial within a feature, and one process holds one target, so "which target does the next command operate on?" has exactly one answer. Multi-target session state in one process would invite ambiguity (what if two targets have conflicting cross-spec impact?) without solving a real problem `ductus` has. Running two or more agent processes at once is the common case, and the worktree remedy does not put two worktrees on the same branch, which trunk-based work needs — by default `git worktree add` refuses a branch already checked out by another worktree (git-worktree(1), under `--force`). So each agent process has its own session target, keyed by `DUCTUS_SESSION` or the agent's own session id, with `.ductus/session.toml` kept as the shared default. `git worktree` and platform isolation (Claude Code's `isolation: "worktree"` agent parameter, Cursor's worktree integration) remain the answer for isolating edits; worktree workflow itself is git's job, not `ductus`'s. The constitution's §concurrent-features says so.

> **Note:** per-process session targets are [062-concurrent-session-targets](../062-concurrent-session-targets/spec.md)'s.

### Cost controls

Track token usage and enforce budget limits. When evaluated, `ductus` had no cost awareness.

- State when evaluated: no cost tracking — delegated entirely to the AI platform
- GSD-2 approach: per-task token/cost metrics, budget ceilings with warnings/pauses/halts
- `ductus` opportunity: unclear — `ductus` has nothing that observes a model call; this may remain the platform's responsibility

**Verdict: decline budget tooling, adopt a documentation cross-reference.** Per-task token tracking and budget ceilings require instrumenting the model call itself — that part is platform-level by definition, and out of reach of `ductus`'s runtime, which §runtime-boundary's principle 2 forbids from calling an LLM, so it can never observe token usage. But several decisions in this spec are themselves cost-aware patterns, so the remaining work was to name them and point at platform tooling for the runtime piece.

The constitution's `### Cost levers` (§cost-levers) names `ductus`'s cost levers:

- Stuck detection (Q3 above) — catch runaway loops before they compound spend
- Default-off autonomy (Q4 above) — human-in-the-loop gating contains blast radius

…and says token budgets are the AI platform's, which is where runtime cost controls live.

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

- [x] AC9: `/ductus:implement` command instructions include a stuck-detection step that reads `git log` for `tasks.md` and its checkbox state, surfaces cycles, and suggests decomposition; it does not read affected-files history, which is noisy and would force the implementation to parse the plan's affected-files table (`specs/010-agent-autonomy/plan.md` §Trade-offs, *Stuck detection reads `tasks.md` commits, not affected-files commits*)
- [x] AC10: `/ductus:implement` command accepts an `--auto` flag that skips per-task confirmations within a phase, with the documented gates (phase transitions, stuck detection, spec/plan edits, mid-implement discovery, risky actions) still firing
- [x] AC11: The constitution's `### Cost levers` subsection of `## Guiding Principles` names `ductus`'s cost levers (stuck detection, default-off autonomy) and says token budgets are the AI platform's
- [x] AC12: `AGENTS.md` project template gains an optional "Skills" index section listing available skill files and their activation conditions (empty by default)
- [x] AC13: Documentation note added (constitution or `AGENTS.md` template) directing users to `git worktree` and platform isolation for concurrent feature work

## Open Questions

(none — all resolved; see Resolved Questions below)

## Resolved Questions

1. **Skills system: replace `AGENTS.md` or layer on top?** — Layer on top. `AGENTS.md` remains the always-loaded baseline; the template gains an optional "Skills" index section listing available skill files and the task types/topics that activate them (empty by default). Verdict for the capability is **adapt**: `ductus` documents the pattern and per-platform mapping (Claude Code skills, Cursor rules, etc.) without prescribing a fixed directory or shipping platform-specific files. Terminology aligns with Anthropic/Claude Code's "skills" — the only mainstream vendor using the term, and they use it for context-loaded instruction packs (matching this concept).
2. **Complexity routing: who assigns?** — No one: `ductus` has no complexity routing. Verdict for the capability is **decline**. Agentic-coding cost is rising and platform autorouting is opaque, but an inline `[simple]` tier marker on tasks is an input an author must remember to set, with no viable derivation, so it is not a fit; a `[complex]` tier would be redundant against the default. See §Complexity routing.
3. **Execution log: worth maintaining?** — No. Verdict for the capability is **decline the artifact, adopt the behavior**. No persisted execution log: an append-only markdown file duplicates `git log`, creates per-invocation git churn that fights text-first-artifacts (artifacts should change with intent), and goes out of sync on squash/rebase. The stuck-detection *behavior* is adopted using existing signals — `/ductus:implement` reads `git log` for `tasks.md` and its checkbox state to detect tasks touched across N invocations without completing, then surfaces the cycle and suggests decomposition.
4. **Autonomous execution: opt-in or default?** — Opt-in via `/ductus:implement --auto`, default off. Verdict for the capability is **adapt**. Per-invocation flag rather than session state or spec frontmatter — autonomy is an execution-time decision, not a selection-time or spec-level property, and putting it on the verb that does work matches CLI convention. The session file does not gain an `autoAdvance` field. Phase boundaries, stuck-detection events, spec/plan edits, mid-implement task discovery, and risky actions all continue to gate even with the flag on; the flag only skips per-task confirmations within a phase. Default-off matches the constitutional spirit (§pipeline-boundaries) of keeping the human in the loop unless explicitly opted out.
5. **Parallel milestones: prescribe worktree management?** — No. Verdict for the capability is **decline** multi-target session state within one process; each agent process holds its own session target (062-concurrent-session-targets), and the pipeline is serial within a feature. Isolation of edits is provided by `git worktree` and platform features (Claude Code's `isolation: "worktree"`, Cursor's worktree integration). Multi-target session state in one process would introduce ambiguity (which target does the next command operate on?) without solving a real `ductus` problem. Documentation gains a one-paragraph note pointing users at the platform/git mechanisms; see the note under §Parallel milestones.
6. **Cost guidance beyond platform controls?** — Yes, but only as documentation. Verdict for the capability is **decline budget tooling, adopt a documentation cross-reference**. Per-task token tracking and budget ceilings require instrumenting the model call itself, which `ductus`'s runtime is forbidden to do. The constitution's §cost-levers names `ductus`'s cost levers (stuck detection, default-off autonomy) and says token budgets are the AI platform's. No new artifact, no per-task estimates, no budget files.

## References

Declared dependencies for this spec, surfaced here so the `derive-dependencies` runtime primitive sees them in the body.

- [000-slash-commands](../000-slash-commands/spec.md)
