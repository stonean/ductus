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

Use the session target: invoke `resolve-session` and display any notices and unreadable session files it returns. If `$ARGUMENTS` is provided, use it to override the session target, resolved through `resolve-feature` (`ambiguous` and `not-found` are domain outcomes to surface). If no session target is set and no arguments provided, stop and tell the user to run `/ductus:target` first.

### Flags

`$ARGUMENTS` may include the `--auto` flag in any position. Strip it before treating remaining text as a feature override. The flag is per-invocation and is not persisted to the session file — autonomy is an execution-time decision, not session state.

When `--auto` is set:

- Skip the per-task "prompt the user to commit and push changes" confirmation. Commit on your own and proceed to the next task.
- **Commit, do not push.** Push is hard-to-reverse and externally visible; it stays gated even with `--auto`.

The following gates **still fire and pause** even with `--auto` on:

- Pipeline completion gate (in-progress → done) — confirmation required per §pipeline-boundaries. (The planned → in-progress transition is *not* gated: invoking the command is the user's approval to start work — see step 4.)
- Stuck-detection events — auto mode does not power through cycles.
- Out-of-bounds file writes — modifying a file outside the runtime boundary still requires user notification.
- Spec edits, plan edits, or new tasks discovered mid-implement — **except a disposition task** (`Disposition out-of-spec finding: …`, step 5 of **§Walk through tasks (per task, in order)** in `.ductus/reference/implement.md`), which `--auto` records and keeps working past; the run pauses when it reaches the task, because working it confirms each fix, deferral, or waiver. A task that implements a fix inside the targeted spec is a new task and still pauses.
- Risky actions per the agent's safety rules (destructive ops, secrets, force pushes, etc.).

Default is unset — without the flag, the user confirms each task as today.

## Scope Boundaries

- The runtime write boundary is derived in step 2 from git history; the plan's **Affected Files** section is a planning aid, not authoritative.
- Do NOT read or modify files belonging to other features' spec directories — except what working a disposition task needs: a candidate covering spec's `spec.md` and `tasks.md` for the **§Groom decision tree** in `.ductus/reference/groom.md`, as `/ductus:groom` reads them, and the deferral written to one after its confirmation.
- Do NOT read source code speculatively — only read files relevant to the current task.
- Reference: §implement-phase, §pipeline-boundaries, §text-first-artifacts, §bug-handling, §brownfield-inbox (Finding dispositions), §spec-phase (spec-root resolution), plus the rule-file directory's `configuration-cross.md` (`specs/rules/configuration-cross.md` in adopter projects; `framework/rules/configuration-cross.md` in ductus's own repo) for constants and env-vars (constitution loaded by `/ductus:target` — do not re-read).
- Nothing is written to `specs/inbox.md`. An incidental issue outside the task in hand becomes a task on the targeted spec's `tasks.md` (step 5 of **§Walk through tasks (per task, in order)** in `.ductus/reference/implement.md`), or, when it is non-blocking and surfaced while working a disposition task, a body item on that task. A deferral to another spec or a rule file, outside this feature's boundary, names its target in its confirmation, which is the notification an out-of-boundary write requires.

## Instructions

> **For agent runtimes**: the Invoke steps below call the ductus runtime's MCP tools; without the server, run `.ductus/bin/ductus <name>` (§runtime-host-integration).

**Where a named section lives.** This command's reference is `.ductus/reference/implement.md`. A section named here, or cited elsewhere as part of this command, that has no heading in this file is there; read it when a step points at it.

1. Invoke `read-tasks` against the targeted feature to load the ordered task list and the per-task "done when" conditions. The host threads the per-primitive addressing arguments (task-number, subtask-index, checked, write-boundary, threshold, criterion-index) as **typed** context to the calls that consume them: per-call inputs, not session-file state and (on `ductus exec`) not string CLI overrides.

   1. Invoke `derive-dependencies` as the safety net, per the **Recompute dependencies (safety net)** bullet of §Setup details in `.ductus/reference/implement.md`.

2. Invoke `derive-boundary` against the feature to compute the runtime write boundary from `git diff` against the spec dir's first commit: the spec-dir glob plus a `{dir}/**` glob per changed path's parent directory (root-level files stay exact paths), so writeCode may create new files inside zones the feature already touched. The boundary feeding the writeCode validator below is the **union** of this derivation and any session-seeded write-boundary, a deliberate grant the derivation never revokes; with neither, enforcement stays fail-closed and the first out-of-spec edit halts. An **uncommitted spec directory** is a domain outcome, not an error: the result carries the spec-dir glob alone plus a `guidance` string. Surface that guidance (commit the spec directory, or seed a `write-boundary` in the session) and continue.

3. Invoke `check-stuck` against the feature with a threshold of 3 to detect stuck cycles before starting work. When the result reports stuck, surface the cycle to the user and pause for direction before proceeding — auto mode does not power through cycles.

4. **When the spec is `draft`, `clarified` or `done`, stop here** — before any task is walked, changing nothing — and report: `draft` → "Spec has unresolved open questions. Run `/ductus:clarify` first."; `clarified` → "No plan exists. Run `/ductus:plan` first."; `done` → "Spec is `done`. Run `/ductus:amend` to capture new work as a scenario." A `tasks.md` on disk is no evidence of readiness: recording a question reverts an in-progress spec to `draft` and leaves its tasks in place. **When the spec is still `planned`**, invoke `set-status` to flip its status from planned to in-progress. Invoking `/ductus:implement` is itself the user's approval to begin work, so no confirmation is prompted, with or without `--auto`. **Skip this step when the spec is already `in-progress`** — per-task runs and the completion gate re-invoke `/ductus:implement` there (see steps 7–8 and **§Stuck-detection details** in `.ductus/reference/implement.md`), and `from: planned` would halt on the stale-`from` guard.

5. <!-- llm:writeCode --> Implement the first incomplete task. The host receives the task description, plan-relevant files, the derived write boundary, and constitution excerpts; it returns an edits array plus a one-line summary. The walker validates every edit's path against the write boundary and emits an `out-of-boundary-edit` error envelope (halting the procedure) when any edit escapes the boundary.

6. Invoke `mark-task` to flip the first incomplete subtask's checkbox from unchecked to checked in `tasks.md`. A previous value of `true` surfaces as a no-op result. When that flip completes the task's last real subtask and its "Done when" clause is in the **checkbox form** (`- [ ] Done when: …`), the primitive ticks that clause in the same write.

7. Invoke `diff-cross-spec` against the feature and render the per-task completion summary (host responsibility): list the task processed, surface the cross-spec impact from the result's `cross-spec-paths`, or, when its `form` is `file`, from the list in the file its `lists-file` names, list the disposition tasks still pending on `tasks.md` — each one this task appended (step 5 of **§Walk through tasks (per task, in order)** in `.ductus/reference/implement.md`), and any earlier one not yet worked — by number and title, plus any disposition task this run worked, with its outcome (fixed, deferred with its work, or waived with its reason), remind the user to commit, and prompt for the next pipeline gate. The in-progress → done transition is its own invocation — re-run `/ductus:implement` after every task has been marked complete, review is clean with its record committed or `/ductus:review` reported `review-owed: false`, and `/ductus:analyze` has then run; that run walks the completion-gate steps below.

<!-- audit:ignore-promotion -->
8. Render the next-step offer (host responsibility; no primitive — the ordered task list is already in context from step 1), as step 9 of **§Walk through tasks (per task, in order)** in `.ductus/reference/implement.md` says. **Suppress the offer entirely when nothing is unchecked**; the run proceeds to the completion gate below. **On `ductus exec`** the walk is linear and cannot loop back to an earlier step, so the offer is rendered in the per-task summary and continuing is a fresh invocation; the reduction is documented rather than silent, matching `clarify.md`'s exec-path scope note.

9. Invoke `read-tasks` against the feature to tally completion across the ordered task list — every task checkbox and nested subtask checkbox, including scenario-linked tasks. Report "all subtasks checked" and "task block fully checked" as distinct states: a task whose `done-when-checked` is `false` carries an unticked checkbox-form clause, so its block still shows an unchecked box. Never round that up to complete — name it. When any checkbox remains unchecked, halt with the incomplete-tasks report: name the specific unchecked tasks and do not proceed to criteria verification — the user finishes them (re-running the per-task walk) and re-runs the command.

10. Invoke `read-spec` against the feature to load the acceptance criteria (text and checkbox state, in body order).

11. <!-- llm:verifyCriteria --> Verify each acceptance criterion against the implementation. The host receives the spec path and content plus the criteria list (index, text, checked) from the read-spec result and returns one met / not-met verdict per criterion, each with an optional note. Verification stays semantic — the LLM judges every criterion individually against the code; never batch-judge.

12. Invoke `mark-criterion` for each criterion the verification confirmed met — one call per passing criterion, at the time of verification, addressed by 0-based index in the same body order the read-spec result lists them. A criterion that failed verification stays unchecked and its failure is reported — never batch-mark. When any criterion remains unchecked after this step, report the specific failures and do not propose the transition; the user resolves them and re-runs the command.

13. Invoke `diff-cross-spec` against the feature for the cross-spec impact check. When `cross-spec-count` is non-zero, surface the list — `cross-spec-paths`, or the list in the file `lists-file` names when `form` is `file` — to the user and ask whether the changes were intentional cross-spec updates per §cross-spec-impact. Informational; does not block. A result carrying `guidance` has no window to diff, so report cross-spec impact as **unknown** rather than absent.

14. Invoke `check-review-gate` against the feature to evaluate the pre-done review gate, whose description states its checks in order. A blocked result names the first failing check and carries its canonical message and guidance: halt with that message and do not propose the transition. On `passed: true`, proceed — but surface any `guidance` the result carries even then: a passing gate names either record whose freshness it could **not** determine, a review record that does not say whether its fixes were re-read, and one counting contract breaks the run did not fix, and that notice is the operator's to weigh before approving the transition, not the walker's to drop. See step 5 of **§Completion gate (after all tasks)** in `.ductus/reference/implement.md`.

<!-- audit:ignore-promotion -->
15. If all checks pass, present a summary — including every disposition task this feature's implementation appended (step 5 of **§Walk through tasks (per task, in order)** in `.ductus/reference/implement.md`), each with its recorded outcome: fixed, deferred with its work, or waived with its reason — and ask the user to approve the transition to done. Do not update the status until the user confirms; on denial, the walker exits cleanly without writing.

    A disposition task is checked only once every finding recorded on it has one of the three dispositions, so by the time step 9's tally passes, every one of them is decided; the list is what ties each out-of-spec finding to the work that surfaced it and says where it went. Nothing is read from `specs/inbox.md` and no inbox row renders here: no finding a run produces reaches the inbox, and the standing count of the todos a person has logged has no bearing on this spec — `/ductus:status` renders it on every run (§brownfield-inbox).

16. On confirmation, invoke `set-status` to flip the spec frontmatter's status from in-progress to done.
