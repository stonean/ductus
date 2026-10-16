---
title: "006-bug-workflow — spec"
status: done
dependencies: []
tags: [process, scenarios, brownfield]
next-criterion: 20
---

# 006 — Bug Workflow

Bugs are unwritten scenarios. Rather than tracking defects in a separate system, `ductus` treats every bug as evidence that a spec is missing, ambiguous, or violated. This feature adds scenario support, a bug decision tree, and a brownfield inbox to the pipeline.

Most projects adopting `ductus` are not greenfield — they have existing code, existing bugs, and incomplete specifications. Scenarios are the primary mechanism for incrementally bringing brownfield projects under `ductus`. Every bug fix, edge case discovery, or behavior clarification produces a scenario that makes the specs more precise over time.

## Bug Decision Tree

When a bug is reported, the following decision tree determines the response — in order:

1. **No spec exists for the behavior** — the bug is a gap. Write the spec first, then fix the code.
2. **Spec exists but is ambiguous or incomplete** — the bug is a spec deficiency. Correct or enhance the spec, then fix the implementation.
3. **Spec is clear but implementation is wrong** — add a scenario capturing the correct behavior, then fix the code.

In all three cases, the spec becomes more precise. The scenario or spec update is the primary artifact, not a bug report.

## Scenarios as First-Class Artifacts

A scenario is a spec at a lower level of abstraction — same format, same discipline, narrower scope. Scenarios live in a `scenarios/` subdirectory alongside the spec they elaborate.

### Repository structure

```text
specs/
  {NNN-feature}/
    spec.md
    scenarios/
      token-expiry.md
      rounding-error.md
```

### Scenario format

Each scenario file follows a consistent structure:

- **section** — the parent spec section the scenario elaborates; the parent feature is implicit in the scenario's file path
- **Context** — the specific situation or precondition
- **Behavior** — what the system does in that situation
- **Edge Cases** — boundary conditions and exceptions (optional)

Scenarios use plain language. Given/When/Then syntax is not required.

### Scenario lifecycle

Scenarios do not have their own status field. A scenario is either written (merged) or not. When `/ductus:amend` creates a scenario file, it also appends a task to the parent spec's `tasks.md` referencing the scenario. The task carries the completion status — the scenario itself is a permanent requirement document.

- The parent spec's status remains `in-progress` while tasks are being worked
- The task in `tasks.md` shows what is being worked on and links to the scenario
- When the task is complete, the scenario stays as documentation of the expected behavior
- If a scenario becomes obsolete, it is deleted — not marked with a status

### When to create a scenario

- A bug surfaces that the spec covers at a high level but does not describe in sufficient detail
- An edge case is discovered during implementation or review
- A spec section is growing too large and needs to be decomposed

### When a scenario is not needed

- The spec itself was missing or ambiguous — fix the spec directly
- The behavior is already captured by an existing scenario — update the existing file

## Bug Files

A dedicated bug file is rarely needed. The scenario captures the correct behavior, and git history records when and why it was added.

A bug file is only justified when:

- The root cause is complex enough that losing it would be costly
- Reproduction requires context that does not belong in the spec or scenario
- A workaround must be documented while a fix is deferred

The rule: a bug file should never be the first artifact created. The spec or scenario always comes first.

## Brownfield Inbox

> **Signpost.** `triage` was renamed to `inbox` by [011-brownfield-process](../011-brownfield-process/spec.md), which also replaced the `/triage` command with `/{project}:groom`. Recorded here rather than only in 011 because [§cross-spec-impact](../../framework/constitution.md#cross-spec-impact) puts a change in the spec it affects, and 011's AC14 is discharged by this back-link.

For projects adopting `ductus` incrementally, a `specs/inbox.md` file serves as a temporary inbox for known issues not yet assigned to a feature spec.

### Inbox rules

- Do not frontfill bugs that are not being actively worked on
- Write specs for areas being actively touched — let adoption spread naturally
- As specs are written for each feature area, items migrate from the inbox into their proper home (spec updates or new scenarios)
- The brownfield backlog drains toward empty as adoption completes; the incidental-capture role is ongoing, so the file persists

## Framework Artifacts

This feature produces the following changes to the framework:

- **New template:** `framework/templates/spec/scenario.md` — starter file for scenario documents
- **New template:** `framework/templates/project/inbox.md` — temporary inbox format for brownfield adoption
- **Updated template:** `framework/templates/spec/spec.md` — reference to scenarios directory convention
- **Updated document:** `framework/constitution.md` — bug handling section with decision tree and scenario lifecycle
- **New command:** `/ductus:amend` — standalone command that requires an active session target (set via `/ductus:target`), confirms the target is correct, walks the decision tree, creates scenario files in the correct feature's `scenarios/` directory, and appends a linked task to the parent spec's `tasks.md`
- **New command:** `/ductus:groom` — reviews `specs/inbox.md`, walks each item through the decision tree, migrates items to the appropriate spec or scenario, and removes resolved items from the inbox
- **Updated command:** `/ductus:help` — documents `/ductus:amend`, `/ductus:groom`, scenario conventions, and bug workflow
- **Updated command:** `/ductus:status` — displays scenario counts per spec in the pipeline dashboard
- **Updated command:** `/ductus:next` — suggested `/ductus:amend` as a next action when appropriate (e.g., bug reported, spec is `in-progress`). Delivered as written; `/ductus:next` was later retired outright, so this artifact no longer exists. Next-action guidance now lives in `/ductus:status`'s Next Action column and `/ductus:target`'s Status → next action table.
- **Updated command:** `/ductus:analyze` — checks that scenario-linked tasks are complete during validation
- **Updated document:** `README.md` — documents bug workflow and scenario convention

## Acceptance Criteria

- [x] AC1: `framework/templates/spec/scenario.md` exists with a `section` frontmatter field, Context, Behavior, and Edge Cases sections
- [x] AC2: `framework/templates/project/inbox.md` exists with a flat inbox format and migration rules
- [x] AC3: `framework/templates/spec/spec.md` references the scenarios directory convention
- [x] AC4: `framework/constitution.md` includes a bug handling section with the decision tree
- [x] AC5: `framework/constitution.md` defines scenarios as part of the spec lifecycle
- [x] AC6: `framework/constitution.md` documents the scenario directory convention in the spec phase file structure
- [x] AC7: `/ductus:groom` command exists and walks each inbox item through the decision tree
- [x] AC8: `/ductus:groom` migrates resolved items from `specs/inbox.md` to the appropriate spec or scenario
- [x] AC9: `/ductus:groom` removes migrated items from `specs/inbox.md`
- [x] AC10: `/ductus:help` documents `/ductus:amend`, `/ductus:groom`, scenario conventions, and the bug workflow
- [x] AC11: `/ductus:amend` command exists and creates scenario files under the correct feature's `scenarios/` directory
- [x] AC12: `/ductus:amend` requires an active session target and confirms the target before proceeding
- [x] AC13: `/ductus:amend` follows the decision tree — checks for existing spec before creating a scenario
- [x] AC14: `/ductus:amend` appends a task to the parent spec's `tasks.md` referencing the new scenario
- [x] AC15: `/ductus:status` displays scenario counts per spec in the pipeline dashboard
- [x] AC16: `/ductus:next` suggests `/ductus:amend` as a next action when context warrants it. Delivered as written; the command was later retired, so the behaviour this asserts no longer has a subject. Unlike the renames swept elsewhere in this spec, the claim itself stopped holding rather than its name moving, which is what annotation is for — next-action guidance is now `/ductus:status`'s Next Action column.
- [x] AC17: `/ductus:analyze` checks that scenario-linked tasks in `tasks.md` are complete
- [x] AC18: `README.md` documents the bug workflow and scenario conventions
- [x] AC19: All new and modified markdown files pass `npx markdownlint-cli2`

## Edge Cases

- **No session target set** — `/ductus:amend` stops and tells the user to run `/ductus:target` first
- **Session target points to a spec that has no `tasks.md`** — `/ductus:amend` creates `tasks.md` before appending the task
- **Scenario file already exists with the same name** — `/ductus:amend` stops and reports the conflict; user must choose a different name or update the existing scenario
- **Parent spec is `done`** — `/ductus:amend` still allows creating a scenario (a bug can surface after completion); the spec status reverts to `in-progress`
- **Inbox item matches an existing spec** — migration path: move the item into a scenario under the matching spec and remove it from `specs/inbox.md`
- **Bug spans multiple specs** — create a scenario under the most relevant spec; reference the other spec(s) in the scenario's `section` field
- **No spec exists for the bug** — decision tree step 1: create the spec first via `/ductus:specify`, then create the scenario
- **`specs/inbox.md` does not exist** — `/ductus:groom` stops and reports nothing to groom
- **`specs/inbox.md` is empty** — `/ductus:groom` reports the inbox is clean; the file is kept to preserve git history

## Open Questions

None — all resolved.
