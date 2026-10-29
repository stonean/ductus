---
description: Resolve open questions and advance a spec from draft to clarified.
argument-hint: "[feature]"
---

# Clarify

Resolve open questions and advance a spec from `draft` to `clarified`, or resolve open questions in a targeted scenario.

## Purpose

Pipeline gate: `draft` → `clarified`. A spec cannot be planned until all open questions are resolved, edge cases documented, and acceptance criteria verified. When a scenario is targeted, resolves scenario-level open questions instead.

This command is the resolver, not the back-edge entry point. The `clarified` / `planned` / `in-progress` → `draft` back-edge is owned by `/ductus:amend` (see §spec-lifecycle in the constitution and spec 014). The hot path here walks open questions on a `draft` spec and advances to `clarified`. A recovery branch handles hand-edited specs that arrive at `/ductus:clarify` with a non-`draft` status and unresolved questions in the body — a state that should not occur via normal usage but can arise from manual frontmatter edits or migrations from other tools.

## Context

Use the session target: invoke `resolve-session` and display any notices and unreadable session files it returns (on the markdown-only path, read the shared default `.ductus/session.toml` instead — §concurrent-features). If `$ARGUMENTS` is provided, use it to override the session target. If no session target is set and no arguments provided, stop and tell the user to run `/ductus:target` first.

## Target File Detection

Resolve the session target as above. If it includes a `scenario` and `scenario-path`, operate on the scenario file (the scenario-targeted branch of the Instructions below; detailed walk under **§Scenario-targeted clarify** in `.ductus/reference/clarify.md`). Otherwise, operate on the feature spec.

## Gate

On a feature-targeted run, read the spec's frontmatter `status` field and count the list-item entries in the `## Open Questions` section (a question is one list item — `-`, `*`, `+` or numbered — at the list's own level, a deeper item folding into the question above it (and adding nothing after a blank line), and a bare marker taking its text from its continuation lines (with none, it is no entry); a paragraph, a thematic break such as `---`, and a list item inside an HTML comment or a fenced block are not entries — exactly the entries `read-spec`'s parser and `append-question`'s dedup count; treat the section as having zero entries when it is missing, empty, or contains only a placeholder line such as `*None — all resolved.*`). Branch on the pair `(status, open-question count)`:

| Status | Open questions? | Behavior |
| --- | --- | --- |
| `draft` | yes | Walk questions, then verify acceptance criteria, then advance to `clarified` (existing hot path) |
| `draft` | no | Verify acceptance criteria, then advance to `clarified` (existing hot path) |
| `clarified` / `planned` / `in-progress` | no | Stop with: "Spec is already `{status}`. Run `/ductus:plan` to create the technical plan." for `clarified`, or "Run `/ductus:implement` to continue implementation." for `planned` / `in-progress`. |
| `clarified` / `planned` / `in-progress` | yes | Run **§Recovery path: non-`draft` spec with open questions** in `.ductus/reference/clarify.md`. |
| `done` | (any) | Stop with: "Spec is `done`. Run `/ductus:amend` to capture this as a scenario instead." Exit without mutation. |

The "already `{status}`" branch and the `done` branch never modify any file.

## Scope Boundaries

Feature-targeted:

- Read only the target feature's spec file (frontmatter and body) and dependency spec frontmatter. For the Recovery path, also list (without reading) `plan.md`, `tasks.md`, `data-model.md`, and `specs/{feature}/scenarios/`. Do NOT read plan files, tasks, source code, test files, scenarios, or unrelated specs' bodies *speculatively* or to browse. **Grounding carve-out (§grounding):** when an open question is a factual question about existing reality — how current code behaves, what a schema or interface holds, what a dev database contains — you MAY read the specific source that settles it (and MUST cite it in the resolution), rather than resolve the question from conjecture. Read narrowly, only what answers the question. **Scenario carve-out (markdown-only path only):** to derive the **§Scenario open-question report** in `.ductus/reference/clarify.md` you MAY read the `## Open Questions` section of each file under `specs/{feature}/scenarios/` — those sections only, never a scenario's Behavior, Context, or Edge Cases. On the runtime path no such read happens: `read-spec` already returns the field.
- Scenario-level open questions are **surfaced but never resolved** here. A feature-targeted run reports which scenarios carry questions and names the scenario-targeted command that resolves them; it writes to no scenario file and walks no scenario question. Spec-level and scenario-level questions stay independent for **resolution** — that independence was never about discovery, and a command that holds the signal must not answer an unresolved spec with an affirmative next step.
- Do NOT begin planning or implementation work. This command resolves questions and verifies acceptance criteria only.
- Reference: §grounding, §spec-requirements, §spec-lifecycle, §pipeline-boundaries, §text-first-artifacts (constitution loaded by `/ductus:target` — do not re-read).

Scenario-targeted:

- Read the targeted scenario file (frontmatter and body). May read the parent spec's frontmatter `status` field to decide which next-step suggestion to display. Do NOT read the parent spec's open questions or body, plan files, tasks, source code, test files, or unrelated specs *speculatively* or to browse. The **Grounding carve-out (§grounding)** above applies here too: a scenario question about existing reality may be settled by reading the specific source that answers it, cited in the resolution.
- Do NOT begin planning or implementation work. This command resolves scenario-level questions only.
- Reference: §grounding, §scenarios, §text-first-artifacts (constitution loaded by `/ductus:target` — do not re-read).

## Instructions

> **For agent runtimes**: the Invoke steps below call the MCP tools of the ductus runtime; the host-integration contract — bare↔prefixed tool names, lazy ToolSearch schema fetch, the no-shell-utilities rule, and the two-paths guarantee — lives once in the constitution, §runtime-host-integration. Before the server is registered — the window between acquisition and the restart that loads it — walk the same prose using the host file-reading tools (Read, Edit, Write) per `.ductus/reference/clarify.md`.

**Where a named section lives.** This command's markdown-only reference — the procedure for the path without the runtime, and the policy these steps apply — is `.ductus/reference/clarify.md`. A section named here, or cited elsewhere as part of this command, that has no heading in this file is there; read it when a step points at it.

Steps 1–13 are the feature-targeted walk; a scenario-targeted session runs steps 1, 6, and 14. The detailed walk lives in the markdown-only reference — the question-resolution sub-procedure under **§Feature-targeted clarify (hot path: `draft` spec)** in `.ductus/reference/clarify.md`, the recovery prompt wording under **§Recovery path: non-`draft` spec with open questions** in `.ductus/reference/clarify.md`, and the scenario-targeted variant under **§Scenario-targeted clarify** in `.ductus/reference/clarify.md`.

**Exec-path scope** (`ductus exec clarify`): steps 7–8 are semantic host work with no extension marker, so the subprocess walker no-ops them by design — they cannot fold into the `askClarifyQuestion` round trip, which is one question per trip, because they are spec-wide passes that must run even when the question loop has nothing to walk (the zero-questions short-circuit in step 2). A host walking this command file directly (the MCP path) and the markdown-only path both perform steps 7–8 in full; a host driving exec performs them itself before accepting the step-12 gate. This scope reduction is deliberate and recorded in the spec 022 data-model's exec-path note — not a silent gap.

<!-- audit:ignore-promotion -->
1. Invoke `resolve-session` to resolve the target, and display any notices and unreadable session files it returns; `$ARGUMENTS` overrides the session target. If no session target is set and no arguments are provided, stop and tell the user to run `/ductus:target` first. When the session includes a `scenario` and `scenario-path`, this is a **scenario-targeted** run: read the scenario file, run the question loop (step 6) against it, then wrap up at step 14 — steps 2–5 and 7–13 are feature-spec work and do not apply.

2. Invoke `read-spec` against the target feature (with `include-body`) and branch on the pair `(status, open-question count)` per the Gate table above — the result's frontmatter carries the status and its open-questions list carries the count (the Gate's entry-counting rule; placeholder lines are not entries). The same result carries `scenario-open-questions`, a **separate** field listing the questions carried by `specs/{feature}/scenarios/*.md`; it never merges into the count this step branches on, and it changes no branch. Append the **Scenario open-question report** (see **§Scenario open-question report** in `.ductus/reference/clarify.md`) to whichever branch is taken whenever that field is non-empty — including the two branches that terminate without modifying a file. When the field is empty the report is suppressed entirely, never rendered as "0 outstanding":
   - Missing feature or `spec.md`: stop and report: "Spec does not exist. Run `/ductus:specify` first." (No report — nothing was read.)
   - `draft` with open questions: continue the full walk (steps 4–13); the report is rendered at step 13, before the next-step line.
   - `draft` with zero open questions: short-circuit — skip the question loop (step 6 runs no extension round trip) and continue at step 7 toward the status-advance gate; the report is rendered at step 13, before the next-step line.
   - `clarified` / `planned` / `in-progress` with zero open questions: stop with the "already `{status}`" message from the Gate table, **followed by the report**. No file is modified.
   - `clarified` / `planned` / `in-progress` with one or more open questions: take the **recovery branch** — display the inconsistency and prompt the user per **§Recovery path: non-`draft` spec with open questions** in `.ductus/reference/clarify.md`, then hand off to step 3 for the guarded revert. Recovery is the more upstream defect and governs the walk; the report still renders at step 13 when the walk reaches it, and the questions remain to be resolved after recovery returns the spec to `clarified`.
   - `done` (any question count): stop with the `done` message from the Gate table, **followed by the report**. Exit without mutation.

3. **Recovery-branch revert** (only when step 2 took the recovery branch): on the user's confirmation, invoke `set-status` (from the current status, to `draft`) and continue the full walk (steps 4–13); on decline, exit without modifying any file.

4. **Recompute dependencies (safety net).** Invoke `derive-dependencies`; it is report-only unless `--write` is passed, so this step never rewrites anything, and it walks every spec — there is no per-spec mode. When it reports drift, the `dependencies:` frontmatter is stale relative to the body's inline links (uncommitted edits the pre-commit hook has not yet synced) — surface that and recommend committing (which runs the hook) or running `.ductus/bin/ductus derive-dependencies --write` manually. Do **not** pass `--write` here: this command writes only the spec's questions/status and the session file, while a writing run rewrites `dependencies:` across every spec. Evaluate dependency readiness against the current frontmatter, noting the caveat when drift was reported.

5. Invoke `traverse-deps` against the feature to check dependency readiness. Read each returned edge's `status` and require it to exist and be `clarified` or later — the clarify-time readiness rule (the same clarified-or-later threshold the dashboard's blocked-by computation uses). Do **not** gate on the result's top-level `compatible` flag: that flag encodes the stricter *planned*-or-later rule used at plan/implement time, so a dependency sitting at `clarified` reports `compatible: false` while still satisfying clarify's gate. Flag blockers — the validation gate (step 11) does not pass while a dependency is below `clarified`.

6. <!-- llm:askClarifyQuestion --> Resolve open questions **one at a time** — one extension round trip per open question, in sequence — following the question-resolution sub-procedure in **§Feature-targeted clarify (hot path: `draft` spec)** in `.ductus/reference/clarify.md`, or in **§Scenario-targeted clarify** in `.ductus/reference/clarify.md` on a scenario-targeted run (the per-question round trip, the no-batching rule, skip-and-revisit handling, and the `## Open Questions` → `## Resolved Questions` movement; items already in `## Resolved Questions` are never re-walked). Spec-body edits applying each answer remain LLM work on both paths — no primitive writes prose.

<!-- audit:ignore-promotion -->
7. **Enumerate edge cases and confirm error scenarios** — for each behavior, identify what happens with empty inputs, missing data, duplicates, boundary values, and concurrent access; verify every failure mode has a defined behavior (HTTP status, error code, message) and flag gaps. Update the spec body with the resolved questions and any new edge cases or acceptance criteria.

<!-- audit:ignore-promotion -->
8. **Verify acceptance criteria and cross-spec impact** — check each criterion is concrete, testable, and unambiguous; rewrite vague ones; flag missing criteria. Then list every sibling spec referenced by inline markdown link in the body (the union the dependency scan already computed) and ask: "Do any of these referenced specs need an update because of decisions made here?" If yes, the §cross-spec-impact rule applies — the change goes in the affected spec as a new acceptance criterion or scenario, with a back-link to this spec. This check is informational; it does not block the transition.

9. Invoke `label-criteria` against the feature so any criterion added or rewritten during clarification carries its stable `AC{n}:` label before the spec advances. It runs **after** the criteria pass above, not before: a criterion rewritten in step 8 keeps the label it already had (a rewrite that changes the requirement's meaning is a new criterion with a new label, but that is an authoring judgment nothing mechanical can make), and a criterion *added* in step 8 gets one. Already-labelled criteria are left byte-identical, so the pass is safe to run on every clarification. Skipped for scenario-targeted runs — scenarios carry behavior and edge cases, not acceptance criteria.

10. Invoke `check-artifact-size` against the feature to measure its artifacts against the configured read size — `.ductus/config.toml` `[artifacts] read-size-bytes`, 50,000 bytes by default — now that the answers are written into the spec. Print the `warning` of each entry in `oversized`, which says the file may not be read in one call and names the fixes for its kind, and each of the result's `notices`, and name each `skipped` subject as not examined. Splitting a spec is cheapest now, before a plan and tasks exist for the part that moves. The result is advisory: it never fails the validation gate below and never blocks the transition. A `read-size-bytes` that is not a positive whole number of bytes, or an `artifacts` that is not a table, is a configuration error, not a result: the primitive fails naming the setting, the value, the file and the accepted form, and the command halts rather than measuring at the default. On the markdown-only path, measure each subject — `spec.md`, `plan.md`, `tasks.md`, `data-model.md` and each scenario — with the host's file tools against the same threshold, halting the same way on such a value and naming any key in `[artifacts]` other than `read-size-bytes` as the primitive's notice does; a subject whose size those tools cannot report is not examined, never under the threshold.

11. Run the **validation gate** before proposing the status transition — every check must pass: all open questions are resolved (none remain in the Open Questions section — if questions remain that need user input, list them and keep `status` at `draft`); acceptance criteria are concrete and testable with no empty placeholders; dependencies are at `clarified` or later (step 5); and invoke `lint-markdown` against the modified spec file, requiring a clean result. If any check fails, report the specific failures and do not propose the transition — the user fixes the issues and re-runs the command.

12. Invoke `gate-confirm` with a prompt that presents a summary of the changes and the resolved questions and asks the user to approve the transition from `draft` to `clarified`. On confirmation, continue to step 13; on denial, the walker exits cleanly without modifying the spec.

13. Invoke `set-status` to flip the spec frontmatter's status from `draft` to `clarified`; the primitive guards against a stale "from" value so concurrent edits surface as an operational error rather than a silent overwrite. Then, when step 2's `scenario-open-questions` field was non-empty, render the report described in **§Scenario open-question report** in `.ductus/reference/clarify.md` — before the next-step line, so the outstanding work is read ahead of the affirmative next action rather than after it. Finally display the next step: "Run `/ductus:plan` to create the technical plan."

    The transition is **not** gated on the report: a spec advances `draft → clarified` carrying scenario questions, exactly as it did before. `done` remains the only mechanized block — the pre-done review gate's scenario-open-questions check — and §readiness-check already counts both sets at `planned → implement`. The report exists so the advance is not the *only* thing the run says.

14. **Scenario-targeted wrap-up** (scenario-targeted runs only): after the question loop, enumerate edge cases specific to the scenario's behavior (empty inputs, missing data, boundary values, concurrent access) and add them to the scenario's `## Edge Cases` section; confirm the scenario's Behavior section is unambiguous and complete; if questions remain that need user input, list them. The scenario has no status field — resolution is complete when all open questions are removed from the Open Questions section. Invoke `lint-markdown` against the modified scenario file. Read the parent spec's frontmatter `status` field (a host read — this step already dispatches `lint-markdown`, so it does not also invoke read-spec), display "Scenario clarification complete.", and suggest `/ductus:implement` if the parent spec is `planned` or `in-progress` (both states are accepted by `/ductus:implement`'s gate); for other parent-spec states (`draft`, `clarified`, `done`), display the completion message without a next-step suggestion — the parent spec's own pipeline state determines what comes next.
