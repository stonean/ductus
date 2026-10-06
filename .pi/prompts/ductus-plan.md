---
description: Create a technical plan and task breakdown for a clarified spec.
argument-hint: "[feature]"
parity:
  strict-fields:
    - status-transition
  semantic-fields:
    - plan-body
---

# Plan

Create a technical plan and task breakdown for a clarified spec.

## Purpose

Pipeline gate: clarified → planned. A spec cannot be implemented until it has a plan with technical decisions, affected files, and an ordered task list. This command produces both `plan.md` and `tasks.md`.

## Context

Use the session target: invoke `resolve-session` and display any notices and unreadable session files it returns (on the markdown-only path, read the shared default `.ductus/session.toml` instead — §concurrent-features). If `$ARGUMENTS` is provided, use it to override the session target. If no session target is set and no arguments provided, stop and tell the user to run `/ductus:target` first.

## Spec File Detection

Read `spec.md`. If it does not exist, stop and report: "Spec does not exist. Run `/ductus:specify` first."

## Gate

Read the spec's `status` field from the YAML frontmatter at the top of the file. If `status` is not `clarified`, stop and report:

- `draft` → "Spec has unresolved open questions. Run `/ductus:clarify` first."
- `planned` / `in-progress` → "Spec is already planned. Run `/ductus:implement` to begin implementation."
- `done` → "Spec is `done`. Run `/ductus:amend` to capture new work as a scenario."

## Scope Boundaries

- Read only files needed for planning: the target spec, `specs/system.md`, and cross-spec files per the markdown-only reference below. Do NOT read source code, test files, or unrelated specs *speculatively* or to browse the codebase. **Grounding carve-out (§grounding):** you MAY read the specific existing source, schema, or interface a technical decision directly depends on — read narrowly to substantiate that one decision, never to survey — and MUST cite what you read (`path:line`) or mark the claim an assumption. This is the only sanctioned source read; the ban on speculative reads stands.
- Do NOT begin implementation. This command produces `plan.md` and `tasks.md` only.
- Reference: §grounding, §plan-phase, §tasks-phase, §readiness-check, §text-first-artifacts (constitution loaded by `/ductus:target` — do not re-read).

## Instructions

> **For agent runtimes**: the Invoke steps below call the MCP tools of the ductus runtime; the host-integration contract — bare↔prefixed tool names, lazy ToolSearch schema fetch, the no-shell-utilities rule, and the two-paths guarantee — lives once in the constitution, §runtime-host-integration. Before the server is registered — the window between acquisition and the restart that loads it — walk the same prose using the host file-reading tools (Read, Edit, Write).

**Exec-path scope** (`ductus exec plan`): steps 4–6 cross the boundary at the `writeSpecBody` extension point, but the task breakdown (step 7) and the substantive readiness checks (the **Validation gate** reference below) are spec-wide semantic host work with no extension marker, so the subprocess walker no-ops them by design — the runtime owns no primitive for the task breakdown or the criteria/consistency judgments. A host driving `ductus exec` (and the markdown-only path) performs them itself before accepting the step-9 gate. `markdownlint` (steps 2, 11) and the artifact-size check (step 8) are advisory on every path — neither blocks the clarified → planned transition. This scope reduction mirrors clarify's and is not a silent gap.

1. Invoke `read-spec` against the targeted feature to load the spec's frontmatter, sections, acceptance criteria, and open-question count. The result drives downstream prompts; the procedure refuses to proceed when the spec's status is not clarified.

2. Invoke `lint-markdown` against the feature directory's markdown files. Pre-plan violations are surfaced as advisory findings; the procedure continues regardless.

3. Invoke `create-plan-artifacts` against the targeted feature to copy the plan and tasks templates into the feature directory — pass `include-data-model` when the feature introduces or modifies domain entities or data structures. Missing artifacts are created (atomic, mode-preserving); pre-existing ones are never touched and report back `kept`. When any artifact is `kept`, run the **Detect existing artifacts** prompt below: on **keep** (the default), proceed with the files as they stand; on **replace**, re-invoke with `overwrite: true` to copy fresh templates over them.

4. <!-- llm:writeSpecBody --> Fill the Technical Decisions section of the plan. The host returns the markdown body for the section; the walker forwards the response through the context.

5. <!-- llm:writeSpecBody --> Fill the Affected Files section of the plan. The host returns a table listing files this feature creates or modifies, alongside an action and purpose for each row. The runtime write boundary used by `/ductus:implement` is derived from git history; this section is a planning aid, not authoritative.

6. <!-- llm:writeSpecBody --> Fill the Trade-offs section of the plan. The host enumerates the considered-and-rejected alternatives plus known limitations.

<!-- audit:ignore-promotion -->
7. **Author the task breakdown.** Break the plan into discrete, ordered work items in `tasks.md`, following the **Create the task breakdown** reference below. Step 3 copied the `tasks.md` template; this step fills it. This is spec-wide semantic host work with no extension marker (see the exec-path scope note above) — the runtime provides no primitive for the breakdown itself, so it is authored the same way on the MCP and markdown-only paths.

8. Invoke `check-artifact-size` against the feature to measure the spec's artifacts, the plan and tasks just written among them, against the configured read size — `.ductus/config.toml` `[artifacts] read-size-bytes`, 50,000 bytes by default. Print the `warning` of each entry in `oversized`, which says the file may not be read in one call and names the fixes for its kind, and each of the result's `notices`, and name each `skipped` subject as not examined. The result is advisory, like markdownlint: it never blocks the clarified → planned transition. On the markdown-only path, measure each subject — `spec.md`, `plan.md`, `tasks.md`, `data-model.md` and each scenario — with the host's file tools against the same threshold; a subject whose size those tools cannot report is not examined, never under the threshold.

9. Invoke `gate-confirm` with a prompt that presents a summary of the plan body and the task breakdown and asks the user to approve the transition from clarified to planned. On confirmation, continue to step 10; on denial, the walker exits cleanly without modifying the spec.

10. Invoke `set-status` to flip the spec frontmatter's status from clarified to planned; the primitive guards against a stale "from" value so concurrent edits surface as an operational error rather than a silent overwrite.

11. Invoke `lint-markdown` a second time. Any violations surface as advisory findings the user resolves before running `/ductus:implement` — markdownlint is advisory on both paths, never a transition blocker.

## Markdown-only reference

The full plan-creation procedure (existing-artifact protection, cross-spec context checklist, plan section contents, task breakdown rules, readiness gate, and cross-spec impact check) is documented below for the markdown-only path. The numbered steps above invoke the mechanical primitives that automate the deterministic phases; the host applies the same procedure against the markdown-only path when the runtime is unavailable.

### Recompute dependencies (safety net)

Invoke `derive-dependencies` (report-only by default; it walks every spec — there is no per-spec mode). If it reports drift, the `dependencies:` frontmatter is stale from uncommitted body edits; surface that and recommend committing (the pre-commit hook syncs it) or running `ductus derive-dependencies --write` manually, then evaluate cross-spec context against the current frontmatter. Do not pass `--write` from this command.

### Detect existing artifacts

Before generating any artifacts, check the feature directory for existing plan files. This protects work the user may have already invested — including plans that survived a `/ductus:amend` back-edge cycle.

1. Check the feature directory for `plan.md`, `tasks.md`, and `data-model.md`.
2. If none of those files exist, skip this section and proceed to the cross-spec context checklist with the standard template-copy flow unchanged.
3. If any of those files exists, list each one that exists with its last-modified timestamp (stat the file for the mtime — `create-plan-artifacts` reports each pre-existing artifact as `kept` but carries no wall-clock data), then prompt: "Plan artifacts exist from a prior `/ductus:plan` run. Keep them and run the readiness check, or replace with fresh templates?" The default is **keep**.
4. **Keep** — skip the template copy entirely. Do not overwrite or modify the existing artifacts during this step. Proceed to the cross-spec context checklist; in **Create the plan** and **Create the task breakdown**, skip the "copy template" steps and treat the existing files as the working artifacts. Then run the validation gate. Advance status to planned only if all readiness checks pass; on failure, report the specific failures and exit without advancing.
5. **Replace** — copy fresh templates over the existing files. The user is responsible for re-applying any kept content.

### Cross-spec context checklist

Before creating the plan, load only the cross-spec context this feature actually needs:

- **Always read:** `specs/system.md` — architecture patterns and shared conventions.
- **Read if the feature emits or consumes events:** `specs/events.md` — check for naming conflicts and reuse opportunities.
- **Read if the feature introduces error codes:** `specs/errors.md` — check code ranges and format conventions.
- **Read if the feature has dependencies:** the spec file (not plan or tasks) of each dependency listed in this spec's frontmatter `dependencies` field — confirm `status` and understand the contracts this feature builds on.
- **Read if the feature introduces or modifies domain entities or data structures:** `data-model.md` files from related specs — check for structural conflicts.
- **Read to ground a decision (§grounding):** the specific existing source, schema, or interface a technical decision directly depends on — read narrowly, and cite what you read (`path:line`) in the decision's rationale. A claim about how existing code behaves that you did not read is an assumption and must be labeled one.
- **Do NOT read** plans, tasks, or scenarios from other features, and do NOT read source code *speculatively* or to browse — the targeted grounding read above is the only sanctioned source read.

### Create the plan

1. **If the user picked "keep" in the existing-artifact prompt above**, skip the template copy — `plan.md` is already on disk and is the working artifact. Otherwise (no prior artifacts, or "replace"), copy `specs/templates/plan.md` into the feature directory as `plan.md`.
2. Fill in (or, on the keep path, edit/extend the existing content):
   - **Technical Decisions**: each decision with rationale. Name the code a decision concerns by path, citing `path:line`, rather than reproducing it: a code sketch goes stale the moment the code lands, and it regrows a plan that was trimmed to stay readable in one call. **Ground every claim about existing code, schema, or interfaces in the source — read the specific file (or query the dev database) and cite it (`path:line`), or label the claim an assumption (§grounding). Do not assert how existing code behaves from memory or conversation.**
   - **Affected Files**: a *planning aid* — list the files you expect to create or modify so reviewers can sanity-check scope.
   - **Data Model**: data structure definitions. Create `data-model.md` if the feature introduces or modifies domain entities or data structures.
   - **Trade-offs**: what was considered and rejected, known limitations.
3. Cross-validate against the files loaded in the checklist above:
   - Plan must not conflict with `specs/system.md`.
   - Data model must be consistent with related specs.
   - Event types must align with `specs/events.md`.

### Create the task breakdown

1. **If the user picked "keep" in the existing-artifact prompt above**, skip the template copy — `tasks.md` is already on disk and is the working artifact. Otherwise (no prior artifacts, or "replace"), copy `specs/templates/tasks.md` into the feature directory as `tasks.md`.
2. Break the plan into discrete, ordered work items:
   - Each task is small enough to complete and verify in a single session.
   - Each task closes with its completion condition written on its own line as `- **Done when**: <condition>` — this exact marker is what `read-tasks`/`check-artifacts` recognize (a checkbox-nested `- [x] Done when: …` or a bulletless `Done when: …` is tolerated on read, but author the canonical bold form).
   - Tasks respect dependency order.
   - Tasks are derived from the plan, not invented independently.

### Validation gate

Before proposing the status transition, run the readiness check. The substantive checks must pass — failures block the transition:

- **The spec directory is committed** — at least one commit touches `{specs-root}/{feature}`. `/ductus:implement` derives its write boundary from that history, so a spec that reaches `planned` with an uncommitted directory provably cannot start: the gap would surface one command later, as a failure to begin work rather than a failure to plan. Evaluate it with `derive-boundary` (a result carrying `guidance` is the no-history signal) or, markdown-only, with `git log -1 -- {specs-root}/{feature}`. On failure, report the guidance — commit the spec directory, or seed a `write-boundary` in the session — and do not advance.
- Acceptance criteria are concrete and testable
- All open questions are resolved
- Data model exists if the feature introduces or modifies domain entities or data structures
- Plan does not conflict with `system.md` or other feature specs
- Data model is consistent with related specs
- Event types align with `events.md`
- Tasks are ordered and each has a clear definition of done

Markdownlint (`npx markdownlint-cli2` over the feature directory's `.md` files) runs as an **advisory** check on both paths — surface any violations for the user to resolve before `/ductus:implement`, but do not block the transition on them (this matches runtime step 11).

The artifact-size check (runtime step 8, `check-artifact-size`) is **advisory** on both paths too. It measures `spec.md`, `plan.md`, `tasks.md`, `data-model.md` and each scenario against `.ductus/config.toml` `[artifacts] read-size-bytes` (50,000 bytes by default) and prints a warning, with the fixes for its kind, for each one an agent may not read in one call. Surface the warnings and do not block the transition on them.

If any substantive check fails, report the specific failures and do not propose the transition. The user fixes the issues and re-runs the command.

### Cross-spec impact check

After the plan is written and before finalizing, list every sibling spec referenced by inline markdown link in the spec or plan body. Ask: "Do any of these referenced specs need an update because of decisions made here?" If yes, the §cross-spec-impact rule applies — record the change in the affected spec as a new acceptance criterion or scenario, with a back-link to this spec. Informational; does not block.

### Finalize

1. Present a summary of the plan, task breakdown, and validation gate results. Ask the user to approve the transition to planned. Do not update the status until the user confirms.
2. On confirmation, update the spec's frontmatter `status` field from clarified to planned.
3. Display the next step: "Run `/ductus:implement` to begin implementation."
