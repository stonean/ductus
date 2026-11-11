---
description: Add a question or a scenario to the targeted spec (classifier-driven).
argument-hint: "[input text]"
---

# Amend

Add input to the targeted spec or scenario. `/amend` classifies the input as a **question** (an unresolved decision recorded under `## Open Questions`), a **scenario** (a concrete behavior captured under `scenarios/{slug}.md`), or a **criterion** (a testable requirement recorded under `## Acceptance Criteria`), routes through the matching path, and on the spec target performs whichever back-edge keeps the lifecycle invariant.

## Purpose

Captures additions to a spec that arise at any point in the pipeline. `/amend` is the single verb for "I have a thing to add to this spec": the framework classifies the input and routes it, and the user approves the classification (or flips it) at the same approval gate as the refined wording.

The back-edges that keep the spec lifecycle honest (§spec-lifecycle) are owned by `/amend`:

- **Question route — `clarified` / `planned` / `in-progress` → `draft`**, in the same write as the new open question. The user's acceptance of the refined question at the approval gate is the consent; no separate prompt fires.
- **Scenario route — `done` → `in-progress`.** The scenario's task is implemented, and the spec returns to `done`.
- **Criterion route — `done` → `in-progress`.** A new acceptance criterion on a `done` spec is new scope, so it takes the *meaningful body edit* back-edge. The criterion is appended unlabelled and the label assigned by `label-criteria`, never by hand. On a spec below `done` the criterion is recorded with no status change. Its behavior and edge cases are [000's `criterion-route-after-draft`](https://github.com/stonean/ductus/blob/main/specs/000-slash-commands/scenarios/criterion-route-after-draft.md).

## Context

Use the session target: invoke `resolve-session` and display any notices and unreadable session files it returns. If `$ARGUMENTS` is provided, use it as the initial input text. If no session target is set and no arguments provided, stop and tell the user to run `/{project}:target` first.

## Target File Detection

Resolve the session target as above. If it includes a `scenario` and `scenario-path`, the target artifact is the scenario file and the input is always treated as a question (scenarios do not nest under scenarios; the classifier is bypassed). Otherwise, the target artifact is the feature's `spec.md`. If that file does not exist, stop and report: "Spec does not exist. Run `/{project}:specify` first."

## Scope Boundaries

- This command reads the target artifact, appends to its `## Open Questions` section or writes a new `scenarios/{slug}.md` file and appends a linked task to `tasks.md`, and — when a back-edge applies — updates the spec's frontmatter `status` field. No other artifact contents are modified. Plan files and source code are never read or written — except the files a confirmed chore fix names (see **Refine the input (scenario route)**), the one case in which this command writes outside the spec directory, and it records nothing on the spec.
- Spec `status` is mutated by this command only on a back-edge (clarified+ → draft or done → in-progress).
- For the impact display, this command may read sibling specs' frontmatter (only) under `specs/` to detect dependents. It does not read sibling spec bodies.
- For the re-open precondition and the reconcile pass, this command may run `git status --porcelain` scoped to the feature directory to detect uncommitted scenario/task edits, and no other git command; `classify-spec-edit` reads the diff. The reconcile pass additionally reads `specs/{feature}/tasks.md` to find the tasks referencing each candidate scenario, and appends a task there on confirmation; it never reads or rewrites the scenario bodies.
- Reference: §spec-requirements, §spec-lifecycle, §scenarios, §text-first-artifacts, §bug-handling, §brownfield-inbox (Finding dispositions), §spec-phase (spec-root resolution) (constitution loaded by `/{project}:target` — do not re-read).

## Instructions

> **For agent runtimes**: the Invoke steps below call the ductus runtime's MCP tools; without the server, stop and say to restart or re-run `/ductus` (§runtime-host-integration).

### Confirm target

1. Invoke `resolve-session` to get the session target's feature and optional scenario, and display any notices and unreadable session files it returns.
2. Read the target artifact (scenario file if targeted, otherwise `spec.md`).
3. **Recompute dependencies (safety net).** If the target is a spec, invoke `derive-dependencies` in its report-only default. When it reports drift, the `dependencies:` frontmatter is stale from uncommitted body edits; surface that and recommend committing (the pre-commit hook syncs it) or running `.ductus/bin/ductus derive-dependencies --write` manually. A non-zero `unwritable-count` counts specs whose derived `dependencies:` cannot be written without breaking their frontmatter: name each, from `unwritable` or, past the read size, the file `lists-file` names, and say to edit that key by hand, since the pre-commit hook stops every commit until then. A non-zero `unparseable-count` counts specs whose frontmatter never closes: name each the same way, as not derived, never as in sync. Neither stops this command. Do **not** pass `--write` here: this command's writes are limited to the target's questions/scenario/task/status and the session file (see Scope Boundaries). (Skip on scenario targets, which have no `dependencies` field.)
4. If the target is a spec, read its frontmatter `status` field now, for the gate, the impact display, the classifier's status tiebreaker, and the post-record mutation.
5. Display the feature name, scenario name (if targeted), status, and a brief summary of what the artifact covers.

### Re-open precondition and reconcile pass (spec target)

Before gathering input, inspect the feature directory for an on-disk delta. The user may have already added scenario or task content informally and only need the status flipped to match, with no new input to classify. And a scenario added by hand carries **no task**, so the **reconcile pass** below offers one. Detection is a host responsibility. Scenario-targeted `/{project}:amend` skips this section (scenarios have no status field).

Which half runs depends on status and input:

- **Status flip** — `done` specs only.
- **Reconcile pass** — `done`, `planned`, and `in-progress` specs, and only when the invocation carries no input. With `$ARGUMENTS` supplied the user has something specific to add and the classifier owns the turn.
- **`draft` / `clarified`** — both halves skip. `/{project}:plan` regenerates `tasks.md` from the plan, and the pass must not fight the forward path.

1. From the project root, run `git status --porcelain --untracked-files=all -- specs/{feature}/scenarios/ specs/{feature}/spec.md specs/{feature}/tasks.md` and parse the output, naming each path from the project root by dropping whatever precedes `specs/{feature}/` (spec 059). The delta consists of:
   - Untracked files under `specs/{feature}/scenarios/` (status `??`).
   - Modified `specs/{feature}/spec.md` or `specs/{feature}/tasks.md` (any status code with `M` in either the index column or the working-tree column).
2. If the delta is empty, emit one line naming what went unexamined, then continue to **Gather the input**:

   ```text
   reconcile: no uncommitted scenario or task edits — {N} scenario(s) under this spec were not examined
   ```

   Omit the line when the feature has no `scenarios/` directory or no scenario files: there is no subject, so there is nothing to overstate.
<!-- audit:ignore-promotion -->
3. **Reconcile pass.** For each scenario file in the delta, read `specs/{feature}/tasks.md` and collect **every** task referencing that scenario — checked and unchecked alike. A task references the scenario when the **slug appears in the task's heading, in a subtask line, or in its `Done when` clause**, a slug match, not a path match: the rule `/{project}:analyze`'s `scenario-consistency` family applies (see §drift-prevention's canonical-source map), so the pass offers no duplicate for a scenario the family considers mapped. The checkbox state selects the prompt, never whether to look.
   - **A pending (unchecked) referencing task exists** — skip the scenario silently: the work is already queued.
   - **Only completed (checked) referencing tasks exist** — offer, naming them, so the operator decides with the existing tasks in view.
   - **No referencing task at all** — offer plainly.

   Prompt **per scenario**, never batched into one accept-all:

   ```text
   scenarios/{slug}.md has no pending task.
     task {N} ({title}) references it and is complete     ← omitted when no task references it
   Append a task for it{, reopening the spec to `in-progress`}?
   ```

   On **confirm**, append the task with the append-task primitive, passing the scenario's `slug` and omitting `body`, the same shape the scenario route writes. Per-scenario arguments are host-supplied, so this call is host work rather than a walker-dispatched step. On **decline**, write nothing and move to the next candidate. The scenario file itself is never created, renamed, duplicated, or rewritten.

   When the pass finishes without offering anything, say what it examined:

   ```text
   reconcile: examined {N} scenario(s) in the delta; each already has a pending task
   ```

4. When the reconcile pass appended at least one task to a `done` spec, invoke `set-status` with `from: done`, `to: in-progress` as part of that same action — the prompt in step 3 named the reopen, so it is already consented (§spec-lifecycle). Skip step 5's prompt in that case. On a `planned` or `in-progress` spec no status mutation occurs.
5. If no task was appended and the status is `done`, invoke `classify-spec-edit` against the feature. On `correction`, `mechanical` or `none`, report that the spec stays `done` because the delta changes no criterion, open question or unchecked task, and is at most a sweep's or a cross-service reference's in a contract, and continue to **Gather the input**, changing nothing. On `reopens`, or an error, which means the classification did not run and is said so, display the prior status (`done`) and each delta path with its filesystem mtime, then prompt:

   ```text
   Spec is `done` but the feature directory has un-tracked scenario or task edits:
     {path-1}  ({untracked|modified}, mtime {ts})
     {path-2}  ({untracked|modified}, mtime {ts})
     ...
   They reopen it for: {reasons}     ← omitted when the classification did not run
   Revert status to `in-progress` to reflect the on-disk delta?
   ```

6. On **confirm**, invoke `set-status` with `from: done`, `to: in-progress` to flip the frontmatter. Display: "Spec reopened to `in-progress`. The on-disk delta is now tracked. Run `/{project}:plan` or `/{project}:implement` next." Exit without entering the classifier and without recording any new input.
7. On **decline**, continue to **Gather the input** without modifying any file. The spec remains `done` and the on-disk delta is left alone; new content the user has routes through the classifier.

Declining a status flip or a reconcile offer is not recorded: the same candidate is offered on the next invocation while it remains in the delta.

Scope note: the delta above is the working tree, so a **committed** hand-added scenario is not a candidate, which is what step 2's line reports. Nothing here distinguishes a scenario documenting already-shipped behavior from one describing unimplemented work, which is why every offer is a prompt.

### Gather the input

If `$ARGUMENTS` is provided, use it as the initial input. Otherwise, ask the user: "What do you want to add to this spec?"

When a scenario is the target artifact, skip the classifier (next section) — scenarios accept questions only, not nested scenarios. Continue directly to **Refine the input (question route)**.

### Classify the input

Apply the heuristic to route the input. The classification is provisional — the user can flip it at the approval gate.

**Question signals (route → question):**

- The input ends with `?`.
- The input starts with an interrogative: `how`, `what`, `when`, `should`, `could`, `would`, `is`, `are`, `do`, `does`, `can`, `which`, `why`, `who`.
- The input contains hedge words: `maybe`, `perhaps`, `not sure`, `unclear`, `unsure`.

**Scenario signals (route → scenario):**

- Declarative or imperative phrasing: `when X happens, Y`; `X must Y`; `X should do Y` (without `?`).
- Concrete event/state language: `on`, `when`, `if`, `after`, `during`, `before`.
- No terminal `?`; no interrogative starter.

**Status tiebreaker:** when signals are mixed or absent on a **`done` spec**, default to scenario (the back-edge from `done` is owned by the scenario path; the question path refuses on `done`). When signals are mixed on any other status, default to question.

### Refine the input (question route)

The goal is a question that is precise, actionable, and self-contained — someone reading it during `/{project}:clarify` should understand exactly what needs to be decided without extra context.

1. **Understand intent** — read the target artifact to understand how the question relates to its behaviors, contracts, acceptance criteria, or open areas. If the question's connection to the artifact is unclear, ask the user to explain how it applies.
2. **Draft a refined version** — rewrite the question so it is specific to the spec's domain and terminology, identifies which behavior or criterion it affects, states what decision or information is needed, and stands alone.
3. **Check for duplicates** — compare against entries already in the target artifact's `## Open Questions` section. Use a normalized-whitespace comparison (collapse runs of whitespace, trim, case-insensitive). If the refined form matches an existing entry, report: "An equivalent question is already recorded: '{existing entry}'. Skip or refine further?" On skip, exit without recording; on refine further, incorporate feedback and loop.

### Refine the input (scenario route)

The goal is a scenario that captures a specific situation and the concrete behavior it triggers. Scenarios live at a lower level of abstraction than the parent spec — narrower scope, plain language.

1. **Walk the bug decision tree** (§bug-handling):
   - **Does a spec exist for the behavior?** If no, stop. Tell the user to create the spec first via `/{project}:specify`, then come back. (`/amend` requires a session target with a real spec file.)
   - **Is the spec ambiguous or incomplete?** If yes — the right fix is to update the spec directly, not record a scenario. Offer to help edit the spec; exit without recording.
   - **Is this a chore rather than a spec addition?** If the input is project maintenance (lint or formatting cleanup, dependency cleanup, repo hygiene, a standalone refactor) that adds no durable requirement and is not really about this spec (§bug-handling, durability test), do not write a scenario or touch the spec: **fix it in this run** (§brownfield-inbox, Finding dispositions). Name the files the fix touches and make it once the user confirms, then verify it with the check that covers it (the project's lint, formatter, or tests). A fix that proves blocked or not mechanical is not a chore: revert it and walk this tree again from the top, since an input that is not mechanical carries a requirement. If the user declines the fix, nothing is written. Exit without recording anything on the spec.
   - **Is the spec clear but the behavior needs lower-level elaboration?** Proceed to draft the scenario.
2. **Derive a slug** — lowercase, hyphenated, no whitespace, no punctuation beyond hyphens. Check `specs/{feature}/scenarios/` for slug conflicts; if a file with that slug exists, ask the user for a different name.
3. **Identify the parent-spec section** — the `section:` frontmatter value names the spec section the scenario elaborates. Read the spec's body to pick an appropriate section, or ask the user.
4. **Draft Context, Behavior, and (optional) Edge Cases** for the scenario — plain language; Given/When/Then syntax is not required.

### Approval gate (both routes)

Show the user:

```text
Recording as [question|scenario] — preview drafted at [`## Open Questions` entry | `scenarios/{slug}.md`].

{preview of the refined content}

Accept this form, refine further, or `flip` to switch route?
```

- **Accept** → proceed to **Record the input**.
- **Refine further** → incorporate feedback, redraft, re-present.
- **`flip`** → switch the classification to the other route. Discard the current refined draft. Re-enter the appropriate **Refine the input** section under the new route. The flip keyword is recognized only as a standalone command at this prompt, not mid-sentence in user-provided content. **On a `done` spec, `flip` toward the question route is rejected**, so `append-question` is never called on a `done` spec. Reject with: "A `done` spec accepts only scenarios — `flip` to the question route is unavailable. Reopen the spec first (record a scenario, or reflect an on-disk delta) to raise open questions."

The user's acceptance at this gate is the consent for any status mutation that follows. Do not prompt again for the back-edge.

### Impact display (spec target, question route, status ∈ {clarified, planned, in-progress})

When the question route is recording on a non-`draft` spec, display the impact before performing the write:

- The spec's prior status (the value that will be reverted from).
- Existence and last-modified timestamp of `plan.md`, `tasks.md`, and `data-model.md` in the feature directory. Omit files that do not exist.
- The list of files in `specs/{feature}/scenarios/` if that directory exists.
- A one-line dependency note when this spec is named in any other spec's frontmatter `dependencies` field. Scan sibling specs' frontmatter only (no body reads). When matches exist, render: "Note: this spec is a dependency of {comma-separated dependent slugs}; their pipeline checks will block until this spec returns to `clarified`."

This display is informational only — the user's prior acceptance is the consent.

### Impact display (spec target, scenario route, status = done)

When the scenario route is recording on a `done` spec, display the reopen impact:

- The spec's prior status (`done`, which will revert to `in-progress`).
- The new scenario's path: `scenarios/{slug}.md`.
- A note that the scenario adds a task to `tasks.md` and must be implemented before the spec returns to `done`.

Informational; no separate confirmation prompt.

### Record the input

**Question route:**

1. Invoke `append-question` with the accepted question against the targeted feature — pass the scenario slug when a scenario is the session target. A `duplicate-of` result means an equivalent existing entry suppressed the write: report it. On a spec target whose status is `clarified`, `planned`, or `in-progress` the primitive performs the `→ draft` back-edge in the same atomic write.
2. Invoke `lint-markdown` on the modified file.

**Scenario route:**

1. Invoke `create-scenario` to write `specs/{feature}/scenarios/{slug}.md` with the accepted `section` and the assembled `body` — the `## Context` … `## Edge Cases` markdown passed as one payload. It refuses on slug conflict.
2. Invoke `append-task` with the new scenario's `slug`, omitting `body`, to append a numbered task block to `specs/{feature}/tasks.md`, with the done-when condition "the scenario's described behavior is correctly implemented and tested."
3. If the spec's `status` is `done`, invoke `set-status` to flip `done → in-progress`. (For other spec statuses, no status mutation occurs.)
4. Invoke `write-session` to set the new scenario as the session target: pass the feature slug as the feature argument, the repo-relative spec directory as the path argument, the new scenario slug as the scenario argument, and `specs/{feature}/scenarios/{slug}.md` as the scenario-path argument. Display what the write reports: any `notices` it delivered, the `peers` — each other session on this feature, with its scenario when it has one and when it last used its target (§concurrent-features) — and any `expired` or `unreadable` session.
5. Invoke `lint-markdown` on every modified file.

### Status mutation summary

| Target | Prior status | Route | Behavior |
| --- | --- | --- | --- |
| Spec | `draft` | question | Append question only. No status mutation. |
| Spec | `clarified` / `planned` / `in-progress` | question | Show impact display, append question, revert `status` to `draft` in the same write. |
| Spec | `done` | question | Not reachable: the tiebreaker routes a `done` spec to scenario, and `flip` toward the question route is rejected. |
| Spec | `draft` / `clarified` / `planned` / `in-progress` | scenario | Create scenario, append task, update session target. No status mutation. |
| Spec | `done` | scenario | Show reopen impact, create scenario, append task, revert `status` to `in-progress` in the same write, update session target. |
| Spec | any | chore (scenario-route guard) | Fix it in this run once the user confirms, and exit. No question, scenario, task, or status mutation. |
| Spec | `done` (on-disk delta `classify-spec-edit` reads as `correction`, `mechanical` or `none`) | (precondition) | No prompt and no status mutation: the spec stays `done`. |
| Spec | `done` (on-disk delta that reopens, user confirms re-open precondition) | (precondition) | Flip `status` to `in-progress` via `set-status`. No question, no scenario, no task. |
| Spec | `done` (no input, delta scenario with no pending task, user confirms the offer) | (reconcile) | Append a task via `append-task` and flip `status` to `in-progress` in the same action. No new scenario file, no question. |
| Spec | `planned` / `in-progress` (same offer confirmed) | (reconcile) | Append the task only. No status mutation — the spec already accepts work. |
| Spec | `draft` / `clarified` | (reconcile) | Not reached: `/{project}:plan` regenerates `tasks.md` from the plan. |
| Scenario | (no status field) | (forced question) | Append question to the scenario's Open Questions section. The parent spec's status is not read or mutated. |

### Prompt for another

Ask: "Do you have another input to add?" If yes, loop back to **Gather the input**. The mutation rules apply per input — once a spec has reverted to `draft` or reopened to `in-progress`, subsequent inputs in the same session just append.

When the user is done, display the next step:

- If a question was recorded on a spec: "Question recorded. Run `/{project}:clarify` to resolve it." On a spec, the status is now `draft` regardless of where it started.
- If a question was recorded on a scenario: "Question recorded. Run `/{project}:clarify` to resolve it." The parent spec's status is unchanged.
- If a scenario was recorded: "Scenario recorded at `specs/{feature}/scenarios/{slug}.md` and set as the session target. Run `/{project}:implement` to work on the new task."
- If the input was a chore: "That's general maintenance, not a spec addition — fixed in {files}." Nothing was recorded on the spec. If the user declined the fix, say that nothing was written.
- If the user aborted before accepting any input, exit silently — no input was recorded and no status mutation occurred.
