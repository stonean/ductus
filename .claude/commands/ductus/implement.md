---
description: Execute implementation tasks for the targeted feature.
argument-hint: "[--auto] [feature]"
parity:
  strict-fields:
    - task-checkbox-state
  strict-files:
    - "specs/{feature}/tasks.md"
  semantic-fields:
    - "code-edits[].content"
---

# Implement

Execute implementation tasks for the targeted feature.

## Purpose

Pipeline gate: planned → in-progress → done. Walks through `tasks.md` step by step, implementing each task according to the plan. This is the only command that writes application code.

## Context

Use the session target from `.ductus/session.toml`. If `$ARGUMENTS` is provided, use it to override the session target — resolve that override through `resolve-feature` (exact directory name, feature number, or unique partial slug; `ambiguous` and `not-found` are domain outcomes to surface). If no session target is set and no arguments provided, stop and tell the user to run `/ductus:target` first.

### Flags

`$ARGUMENTS` may include the `--auto` flag in any position. Strip it before treating remaining text as a feature override. The flag is per-invocation and is not persisted to the session file — autonomy is an execution-time decision, not session state.

When `--auto` is set:

- Skip the per-task "prompt the user to commit and push changes" confirmation. Commit on your own and proceed to the next task.
- **Commit, do not push.** Push is hard-to-reverse and externally visible; it stays gated even with `--auto`.

The following gates **still fire and pause** even with `--auto` on:

- Pipeline completion gate (in-progress → done) — confirmation required per §pipeline-boundaries. (The planned → in-progress transition is *not* gated: invoking the command is the user's approval to start work — see step 4.)
- Stuck-detection events — auto mode does not power through cycles.
- Out-of-bounds file writes — modifying a file outside the runtime boundary still requires user notification.
- Spec edits, plan edits, or new tasks discovered mid-implement.
- Risky actions per the agent's safety rules (destructive ops, secrets, force pushes, etc.).

Default is unset — without the flag, the user confirms each task as today.

## Scope Boundaries

- The runtime write boundary is derived in step 2 from git history; the plan's **Affected Files** section is a planning aid, not authoritative.
- Do NOT read or modify files belonging to other features' spec directories.
- Do NOT read source code speculatively — only read files relevant to the current task.
- Reference: §implement-phase, §pipeline-boundaries, §text-first-artifacts, §brownfield-inbox (Automatic issue capture), §spec-phase (spec-root resolution), plus the rule-file directory's `configuration-cross.md` (`specs/rules/configuration-cross.md` in adopter projects; `framework/rules/configuration-cross.md` in ductus's own repo) for constants and env-vars (constitution loaded by `/ductus:target` — do not re-read).
- Appending an incidentally-discovered issue to `specs/inbox.md` (per §brownfield-inbox Automatic issue capture) is a ductus-artifact write, in the same category as the `mark-task` write to `tasks.md` — it is **not** subject to the runtime write boundary and does not trigger an out-of-boundary halt. The deterministic path for the append is the `append-inbox` primitive; if unavailable, append the bullet with the host's file tools per the markdown-only path (Walk through tasks, step 5).

## Instructions

> **For agent runtimes**: the Invoke steps below call the MCP tools of the ductus runtime; the host-integration contract — bare↔prefixed tool names, lazy ToolSearch schema fetch, the no-shell-utilities rule, and the two-paths guarantee — lives once in the constitution, §runtime-host-integration. Before the server is registered — the window between acquisition and the restart that loads it — walk the same prose using the host file-reading tools (Read, Edit, Write).

1. Invoke `read-tasks` against the targeted feature to load the ordered task list and the per-task "done when" conditions. The host threads the per-primitive addressing arguments (task-number, subtask-index, checked, write-boundary, threshold, criterion-index) as **typed** context to the calls that consume them — these are per-call inputs supplied by the driving host, not session-file state, and (on `ductus exec`) not string CLI overrides, since the primitives type them as integers/booleans.

2. Invoke `derive-boundary` against the feature to compute the runtime write boundary from `git diff` against the spec dir's first commit. The result lists the feature's directory zones — the spec-dir glob plus a `{dir}/**` glob per changed path's parent directory (root-level files stay exact paths) — so writeCode may create new files inside zones the feature already touched. The boundary feeding the writeCode validator below is the **union** of this derivation and any session-seeded write-boundary: a seed is a deliberate grant the derivation never revokes, and on a fresh feature (no non-spec history yet) the seed is what admits the first code edit; with neither, enforcement stays fail-closed and the first out-of-spec edit halts. An **uncommitted spec directory** — no commit touches `{specs-root}/{feature}` — is a domain outcome, not an error: the result carries the spec-dir glob alone plus a `guidance` string, since the boundary is unknowable rather than broken. Surface that guidance (commit the spec directory, or seed a `write-boundary` in the session) and continue; enforcement stays fail-closed, so the first out-of-spec edit halts with `out-of-boundary-edit` and a legible next-step rather than the walk dying here. `/ductus:plan`'s validation gate is what normally prevents this state — reaching it means the spec was advanced another way.

3. Invoke `check-stuck` against the feature with a threshold of 3 to detect stuck cycles before starting work. When the result reports stuck, surface the cycle to the user and pause for direction before proceeding — auto mode does not power through cycles.

4. **When the spec is still `planned`**, invoke `set-status` to flip its status from planned to in-progress. Invoking `/ductus:implement` is itself the user's approval to begin work, so this transition is not separately gated — no confirmation is prompted, with or without `--auto`. The primitive guards against a stale "from" value so concurrent edits surface as an operational error rather than a silent overwrite. **Skip this step when the spec is already `in-progress`** — per-task runs and the completion gate re-invoke `/ductus:implement` on an in-progress spec (see steps 7–8 and the stuck-detection details), and calling `set-status` with `from: planned` on an in-progress spec would halt on the stale-`from` guard. Read the current status (from step 1's context, which already loaded the task list and target) and only transition on the planned → in-progress edge.

5. <!-- llm:writeCode --> Implement the first incomplete task. The host receives the task description, plan-relevant files, the derived write boundary, and constitution excerpts; it returns an edits array plus a one-line summary. The walker validates every edit's path against the write boundary and emits an `out-of-boundary-edit` error envelope (halting the procedure) when any edit escapes the boundary.

6. Invoke `mark-task` to flip the first incomplete subtask's checkbox from unchecked to checked in `tasks.md` (atomic write via tempfile + rename). The primitive returns the previous and current states; a previous value of `true` surfaces as a no-op result. When that flip completes the task's last real subtask and the task's "Done when" clause is authored in the **checkbox form** (`- [ ] Done when: …`, which the task breakdown tends to produce), the primitive ticks that clause in the same write — the clause is still not addressable by subtask index, but the block a reader sees must not show an unchecked box under a task the tally calls complete.

7. Invoke `diff-cross-spec` against the feature and render the per-task completion summary (host responsibility): list the task processed, surface the cross-spec impact from the result's `cross-spec-paths` (sibling-spec paths changed since the feature's first commit, working tree included — the same filter step 12 re-checks at the gate), surface the captured issues from `inbox-additions` (per §brownfield-inbox Automatic issue capture — list each captured item and suggest `/ductus:groom` to route them), remind the user to commit, and prompt for the next pipeline gate. The in-progress → done transition is its own invocation — re-run `/ductus:implement` after every task has been marked complete and review is clean; that run walks the completion-gate steps below.

<!-- audit:ignore-promotion -->
8. Render the next-step offer (host responsibility; no primitive — the ordered task list is already in context from step 1). When at least one task remains unchecked, name **the next one** — its number and heading — plus how many unchecked tasks remain after it, and ask whether to continue with it: `Next: task {N} — {heading} ({M} remaining). Continue? (Y/n)`. Answering yes continues the walk onto that task in the same run; no exits cleanly, exactly as the run ends without this step; free-text instructions redirect, so "do task 12 first" or "stop and re-plan" is a first-class reply rather than a rejection to be re-issued. One task, not a menu — the count is what lets the operator judge the queue without re-reading `tasks.md`, and looking further ahead is a redirect like any other. **Suppress the offer entirely when nothing is unchecked** — it never renders as "no next step"; the run proceeds to the completion gate below as it does today. With `--auto` the walk already continues without asking, so nothing changes. Nothing here writes: yes, no, and a redirect all leave the same files the per-task walk had already written, and the offer never touches spec status — the `in-progress → done` transition keeps its own separate confirmation, which `--auto` does not bypass. **On `ductus exec`** the walk is linear and cannot loop back to an earlier step, so the offer is rendered in the per-task summary and continuing is a fresh invocation; the reduction is documented rather than silent (§runtime-host-integration's two-paths guarantee), matching `clarify.md`'s exec-path scope note.

9. Invoke `read-tasks` against the feature to tally completion across the ordered task list — every task checkbox and nested subtask checkbox, including scenario-linked tasks. Report "all subtasks checked" and "task block fully checked" as distinct states: a task whose `done-when-checked` is `false` carries an unticked checkbox-form clause, so its block still shows an unchecked box even with every subtask complete. Never round that up to complete — name it, since it means the clause was authored as a checkbox and never reconciled (step 6 ticks it on the completing flip, so reaching here unticked implies the task was completed some other way). When any checkbox remains unchecked, halt with the incomplete-tasks report: name the specific unchecked tasks and do not proceed to criteria verification — the user finishes them (re-running the per-task walk) and re-runs the command.

10. Invoke `read-spec` against the feature to load the acceptance criteria (text and checkbox state, in body order) and the frontmatter `review:` block the review gate below reads.

11. <!-- llm:verifyCriteria --> Verify each acceptance criterion against the implementation. The host receives the spec path and content plus the criteria list (index, text, checked) from the read-spec result and returns one met / not-met verdict per criterion, each with an optional note. Verification stays semantic — the LLM judges every criterion individually against the code; never batch-judge.

12. Invoke `mark-criterion` for each criterion the verification confirmed met — one call per passing criterion, flipping its checkbox to `- [x]` in the spec at the time of verification (the primitive addresses criteria by 0-based index, in the same body order the read-spec result lists them). A criterion that failed verification stays unchecked and its failure is reported — never batch-mark. When any criterion remains unchecked after this step, report the specific failures and do not propose the transition; the user resolves them and re-runs the command.

13. Invoke `diff-cross-spec` against the feature for the cross-spec impact check: the primitive owns the filter (previously re-derived by hand here and at step 7) — the diff from the feature's first spec-dir commit to the working tree, scoped to the spec root and filtered to paths outside `specs/{feature}/`, with the inbox reported separately as captured additions. If any sibling spec path shows changes, surface the list to the user and ask whether the changes were intentional cross-spec updates per §cross-spec-impact. Informational; does not block. A result carrying `guidance` has no window to diff (no commit touches the spec dir), so report cross-spec impact as **unknown** rather than absent — the empty lists are not a clean bill of health. On the markdown-only path, run `git diff --stat <first-commit>..HEAD -- specs/` and apply the same filter by hand.

14. Invoke `check-review-gate` against the feature to evaluate the pre-done review gate: the feature directory's markdown lint (through the lint-markdown machinery, replacing the raw `npx markdownlint-cli2` invocation), then unresolved scenario open questions, then the spec frontmatter `review:` block. A blocked result names the first failing check (markdown lint violations; unresolved scenario questions; not reviewed; blocking MUST violations) and carries the canonical blocked message — plus the resolve-or-waive guidance on the MUST-violations branch; halt with that message and do not propose the transition. On `passed: true`, proceed. The full check order and message texts are documented in the completion gate's step 5 below, which is also the markdown-only path.

<!-- audit:ignore-promotion -->
15. If all checks pass, present a summary — including any issues captured to `specs/inbox.md` during this feature's implementation (per §brownfield-inbox Automatic issue capture), each listed with a pointer to run `/ductus:groom` to route them — and ask the user to approve the transition to done. Do not update the status until the user confirms; on denial, the walker exits cleanly without writing.

16. On confirmation, invoke `set-status` to flip the spec frontmatter's status from in-progress to done. The primitive guards against a stale "from" value so concurrent edits surface as an operational error rather than a silent overwrite.

## Markdown-only reference

The full setup, walk-through, completion gate, and stuck-detection details are documented below for the markdown-only path. The numbered steps above invoke the mechanical primitives that automate each phase; the host applies the same procedure against the markdown-only path when the runtime is unavailable.

### Setup details

- Read `.ductus/session.toml` for the session target, including optional `scenario` and `scenario-path` fields.
- Read `specs/{feature}/tasks.md` for the ordered task list (primitive: `read-tasks`).
- Read `specs/{feature}/plan.md` for technical decisions and affected files.
- Read the spec file for acceptance criteria and contracts.
- If a scenario is targeted, read the scenario file for scenario-specific context, behavior, and edge cases. The scenario scopes which part of the feature is the primary focus for this implementation session.
- **Recompute dependencies (safety net).** Invoke `derive-dependencies` (report-only by default; it walks every spec — there is no per-spec mode). If it reports drift, the `dependencies:` frontmatter is stale from uncommitted body edits; surface that and recommend committing (the pre-commit hook syncs it) or running `ductus derive-dependencies --write` manually. Do not pass `--write` from this command.

### Stuck-detection details

If the spec's status is already in-progress, run `git log --oneline -- specs/{feature}/tasks.md` and count commits since the spec entered in-progress. Identify the first incomplete task (first `- [ ]` checkbox group) in `tasks.md`. If `git log` shows ≥ 3 commits on `tasks.md` AND the same task is still the first incomplete one (no checkbox flipped to `- [x]` between those commits for that task), surface the cycle to the user with this message: `Task {N} ({title}) has been touched in {count} prior implement runs without completing. Consider decomposing it into smaller subtasks before continuing.` Pause and wait for user direction; do not auto-decompose. The threshold of 3 is fixed (not configurable in v1) — smallest count that distinguishes routine multi-session work from a cycle.

### Progressive context loading

- **At setup:** read only the spec, plan, tasks, and scenario file (if targeted). Do NOT read `system.md`, `events.md`, `errors.md`, or source code yet.
- **Per task:** read only the source files relevant to that task from the plan's affected files list.
- **At completion:** tally the `tasks.md` checkboxes (primitive: `read-tasks`) and re-read the acceptance criteria from the spec (primitive: `read-spec`) to verify. Do NOT re-read the full plan.

### Walk through tasks (per task, in order)

1. Display the task number, description, and "done when" condition.
2. Read the relevant technical decisions from the plan.
3. Read only the existing code files relevant to this task from the plan's affected files.
4. Implement the task: write code, tests, and migrations as needed. Follow conventions in `AGENTS.md` and `specs/system.md`; respect the contracts defined in the spec. If a write would land outside the runtime boundary, notify the user, explain why, and wait for confirmation before proceeding. Once accepted, the file is part of the boundary for the rest of the session.
5. **Capture incidental issues.** If implementing this task surfaces an issue outside the task's scope — a security weakness, a memory or resource leak, a violated convention, a latent bug in adjacent code — append it to `specs/inbox.md` automatically, without prompting, and keep working (per §brownfield-inbox Automatic issue capture). The deterministic path is the `append-inbox` primitive (its dedup guard keeps a re-run from double-logging the same finding); without the runtime, append the bullet with the host's file tools. Do not derail to fix out-of-scope findings; an issue *inside* this task's scope is fixed as part of the task, not logged. The append follows the inbox auto-capture form (see `specs/inbox.md`).
6. Verify the "done when" condition is met.
7. Mark the task as complete in `tasks.md` — update each checkbox to `- [x]`, including nested sub-item checkboxes and a checkbox-form "Done when" clause (`- [ ] Done when: …`), before proceeding. The clause is not an addressable subtask, but leaving it unticked shows an unchecked box under a task the tally calls complete.
8. Prompt the user to commit and push changes. With `--auto` set, skip the prompt: commit on your own, do not push.
9. **Offer the next step.** When at least one task is still unchecked, name the next one and how many remain after it, then ask whether to continue: `Next: task {N} — {heading} ({M} remaining). Continue? (Y/n)`. Yes continues the walk onto that task in the same run; no exits cleanly; free-text instructions redirect ("do task 12 first"), which is a reply rather than a rejection. Name **one** task, not a menu — the count is what lets the operator judge the queue without re-reading `tasks.md`, and looking further ahead is itself a redirect. Suppress the offer when nothing is unchecked, so it never renders as "no next step"; the run continues into the completion gate as it does today. With `--auto` the walk already continues without asking. Nothing here writes, and the offer never advances spec status — the `in-progress → done` transition keeps its own confirmation.

   Edge cases: a task whose "done when" failed halts the run as today (the offer is for a *completed* task, not a way past a failure); every subtask checked with an unticked checkbox-form "Done when" clause is the completion gate's state to surface, not a next-step offer; and a reply targeting a different spec exits cleanly rather than retargeting mid-walk — retargeting is `/ductus:target`'s job, and a walk that silently changed feature would be worse than one that stopped.

10. Before starting the next task, assess whether sufficient context remains to complete it. If context is low, suggest starting a new session.

### Completion gate (after all tasks)

Same order as Instructions steps 8–15, with the same primitives named as fallbacks.

1. **Task tally.** Re-read `specs/{feature}/tasks.md` and tally every checkbox (primitive: `read-tasks`) — top-level tasks, nested sub-items, and scenario-linked tasks. Distinguish "all subtasks checked" from "task block fully checked": a checkbox-form "Done when" clause (`- [ ] Done when: …`) is not an addressable subtask, so a task can have every subtask checked while an unchecked box is still visible in its block (`done-when-checked` is `false`). Surface that rather than rounding it up. If any remains unchecked, report the specific incomplete tasks and stop — the gate ends here, before criteria verification. The user finishes them and re-runs the command.
2. **Load acceptance criteria.** Re-read the spec's Acceptance Criteria checkboxes and the frontmatter `review:` block (primitive: `read-spec`).
3. **Verify each criterion.** Walk through each acceptance criterion from the spec and verify it is met against the implementation — semantic judgment, one criterion at a time (the verifyCriteria extension seam on the runtime path). Mark each passing criterion `- [x]` in the spec file at the time of verification (primitive: `mark-criterion`, one call per passing criterion, addressed by 0-based body-order index). If a criterion fails, leave it unchecked and report the failure. Do not batch-mark — verify each individually. If any criterion remains unchecked, report the specific failures and do not propose the transition; the user fixes the issues and re-runs the command.
4. **Cross-spec impact check.** Run `git diff --stat <first-commit>..HEAD -- specs/`, filtered to paths outside `specs/{feature}/`. If any sibling spec dir shows changes, surface the list to the user and ask whether the changes were intentional cross-spec updates per §cross-spec-impact. Informational; does not block. (Primitive: `diff-cross-spec`, which diffs against the working tree so uncommitted edits surface too; the git command above is the markdown-only path.)
5. **Pre-done review gate** (primitive: `check-review-gate`, which evaluates all three checks below in this order and returns the verdict with the canonical blocked message). First confirm all `.md` files in the feature directory pass `npx markdownlint-cli2`; on failure, report the specific violations and do not propose the transition. Then confirm no scenario under the feature carries unresolved open questions: a scenario is an organizational split of the spec, so its questions are the spec's questions for completeness and the spec is not `done` while any remain (§spec-lifecycle; spec 046). On failure, halt with `blocked: {N} unresolved open question(s) in scenario(s) {names} — resolve them before completing`, followed by guidance to run `/ductus:target {feature}/<scenario>` then `/ductus:clarify` on each — or, when a question is deferred rather than undecided ("not now; revisit when X lands"), to record it in that scenario's `## Resolved Questions` with its trigger, since a deferral is a resolution with a condition and only `## Open Questions` entries count. This check is ordered ahead of the `review:` block because an unresolved design question is the more upstream defect: reviewing a design that is about to change wastes the review. Then read the target spec's frontmatter `review:` block before asking for the done transition. If `review.last-run` is missing, null, or the review block is absent, halt with: `blocked: spec has not been reviewed — run /ductus:review before completing`. If `review.blocking: true`, halt with: `blocked: spec has {must-violations} MUST violation(s) — see specs/NNN-feature/review.md` followed by guidance to either resolve the violations and re-run `/ductus:review`, or run `/ductus:review --waive <rule-id> --reason "..."` for each waivable finding. Otherwise, proceed.
6. If all checks pass, present a summary — including any issues captured to `specs/inbox.md` during this feature's implementation (per §brownfield-inbox Automatic issue capture), each listed with a pointer to run `/ductus:groom` to route them — and ask the user to approve the transition to done. Do not update the status until the user confirms.
7. On confirmation, update the spec's frontmatter status from in-progress to done (primitive: `set-status`, guarding against a stale "from" value).
