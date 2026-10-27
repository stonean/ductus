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

Use the session target: invoke `resolve-session` and display any notices and unreadable session files it returns (on the markdown-only path, read the shared default `.ductus/session.toml` instead — §concurrent-features). If `$ARGUMENTS` is provided, use it to override the session target. If no session target is set and no arguments provided, stop and tell the user to run `/{project}:target` first.

## Spec File Detection

Read `spec.md`. If it does not exist, stop and report: "Spec does not exist. Run `/{project}:specify` first."

## Gate

Read the spec's `status` field from the YAML frontmatter at the top of the file. If `status` is not `clarified`, stop and report:

- `draft` → "Spec has unresolved open questions. Run `/{project}:clarify` first."
- `planned` / `in-progress` → "Spec is already planned. Run `/{project}:implement` to begin implementation."
- `done` → "Spec is `done`. Run `/{project}:amend` to capture new work as a scenario."

## Scope Boundaries

- Read only files needed for planning: the target spec, `specs/system.md`, and cross-spec files per **§Cross-spec context checklist** in `.ductus/reference/plan.md`. Do NOT read source code, test files, or unrelated specs *speculatively* or to browse the codebase. **Grounding carve-out (§grounding):** you MAY read the specific existing source, schema, or interface a technical decision directly depends on — read narrowly to substantiate that one decision, never to survey — and MUST cite what you read (`path:line`) or mark the claim an assumption. This is the only sanctioned source read; the ban on speculative reads stands.
- Do NOT begin implementation. This command produces `plan.md` and `tasks.md` only.
- Reference: §grounding, §plan-phase, §tasks-phase, §readiness-check, §text-first-artifacts (constitution loaded by `/{project}:target` — do not re-read).

## Instructions

> **For agent runtimes**: the Invoke steps below call the MCP tools of the ductus runtime; the host-integration contract — bare↔prefixed tool names, lazy ToolSearch schema fetch, the no-shell-utilities rule, and the two-paths guarantee — lives once in the constitution, §runtime-host-integration. Before the server is registered — the window between acquisition and the restart that loads it — walk the same prose using the host file-reading tools (Read, Edit, Write).

**Where a named section lives.** This command's markdown-only reference — the procedure for the path without the runtime, and the policy these steps apply — is `.ductus/reference/plan.md`. A section named here, or cited elsewhere as part of this command, that has no heading in this file is there; read it when a step points at it.

**Exec-path scope** (`ductus exec plan`): steps 4–6 cross the boundary at the `writeSpecBody` extension point, but the task breakdown (step 7) and the substantive readiness checks (**§Validation gate** in `.ductus/reference/plan.md`) are spec-wide semantic host work with no extension marker, so the subprocess walker no-ops them by design — the runtime owns no primitive for the task breakdown or the criteria/consistency judgments. A host driving `ductus exec` (and the markdown-only path) performs them itself before accepting the step-9 gate. `markdownlint` (steps 2, 11) and the artifact-size check (step 8) are advisory on every path — neither blocks the clarified → planned transition. This scope reduction mirrors clarify's and is not a silent gap.

1. Invoke `read-spec` against the targeted feature to load the spec's frontmatter, sections, acceptance criteria, and open-question count. The result drives downstream prompts; the procedure refuses to proceed when the spec's status is not clarified.

   1. Invoke `derive-dependencies` as the safety net, per **§Recompute dependencies (safety net)** in `.ductus/reference/plan.md`. It reports without writing, so drift from uncommitted body edits surfaces before the plan reads its cross-spec context; do not pass `--write`.

2. Invoke `lint-markdown` against the feature directory's markdown files. Pre-plan violations are surfaced as advisory findings; the procedure continues regardless.

3. Invoke `create-plan-artifacts` against the targeted feature to copy the plan and tasks templates into the feature directory — pass `include-data-model` when the feature introduces or modifies domain entities or data structures. Missing artifacts are created (atomic, mode-preserving); pre-existing ones are never touched and report back `kept`. When any artifact is `kept`, run the prompt under **§Detect existing artifacts** in `.ductus/reference/plan.md`: on **keep** (the default), proceed with the files as they stand; on **replace**, re-invoke with `overwrite: true` to copy fresh templates over them.

4. <!-- llm:writeSpecBody --> Fill the Technical Decisions section of the plan. The host returns the markdown body for the section; the walker forwards the response through the context.

5. <!-- llm:writeSpecBody --> Fill the Affected Files section of the plan. The host returns a table listing files this feature creates or modifies, alongside an action and purpose for each row. The runtime write boundary used by `/{project}:implement` is derived from git history; this section is a planning aid, not authoritative.

6. <!-- llm:writeSpecBody --> Fill the Trade-offs section of the plan. The host enumerates the considered-and-rejected alternatives plus known limitations.

<!-- audit:ignore-promotion -->
7. **Author the task breakdown.** Break the plan into discrete, ordered work items in `tasks.md`, following **§Create the task breakdown** in `.ductus/reference/plan.md`. Step 3 copied the `tasks.md` template; this step fills it. This is spec-wide semantic host work with no extension marker (see the exec-path scope note above) — the runtime provides no primitive for the breakdown itself, so it is authored the same way on the MCP and markdown-only paths.

8. Invoke `check-artifact-size` against the feature to measure the spec's artifacts, the plan and tasks just written among them, against the configured read size — `.ductus/config.toml` `[artifacts] read-size-bytes`, 50,000 bytes by default. Print the `warning` of each entry in `oversized`, which says the file may not be read in one call and names the fixes for its kind, and each of the result's `notices`, and name each `skipped` subject as not examined. The result is advisory, like markdownlint: it never blocks the clarified → planned transition. A `read-size-bytes` that is not a positive whole number of bytes, or an `artifacts` that is not a table, is a configuration error, not a result: the primitive fails naming the setting, the value, the file and the accepted form, and the command halts rather than measuring at the default. On the markdown-only path, measure each subject — `spec.md`, `plan.md`, `tasks.md`, `data-model.md` and each scenario — with the host's file tools against the same threshold, halting the same way on such a value and naming any key in `[artifacts]` other than `read-size-bytes` as the primitive's notice does; a subject whose size those tools cannot report is not examined, never under the threshold.

9. Run **§Cross-spec impact check** in `.ductus/reference/plan.md`, then invoke `gate-confirm` with a prompt that presents its result and a summary of the plan body and the task breakdown and asks the user to approve the transition from clarified to planned. On confirmation, continue to step 10; on denial, the walker exits cleanly without modifying the spec.

10. Invoke `set-status` to flip the spec frontmatter's status from clarified to planned; the primitive guards against a stale "from" value so concurrent edits surface as an operational error rather than a silent overwrite.

11. Invoke `lint-markdown` a second time. Any violations surface as advisory findings the user resolves before running `/{project}:implement` — markdownlint is advisory on both paths, never a transition blocker.
