# implement — reference

The numbered steps in `implement.md`'s Instructions invoke the primitives that automate each phase; the setup, walk-through, completion gate and stuck-detection details they apply are below.

**Where a named section lives.** A section named below with no heading here is in the command file, `implement.md`, with the numbered steps this file cites. A bare `§name` is a heading here or a constitution marker; one with another file's path on its line is a heading there, unless it is a constitution marker.

## Setup details

- `resolve-session` returns the session target, including optional `scenario` and `scenario-path` fields.
- Read `specs/{feature}/tasks.md` for the ordered task list (primitive: `read-tasks`).
- Read `specs/{feature}/plan.md` for technical decisions and affected files.
- Read the spec file for acceptance criteria and contracts.
- If a scenario is targeted, read the scenario file for scenario-specific context, behavior, and edge cases. The scenario scopes which part of the feature is the primary focus for this implementation session.
- **Recompute dependencies (safety net).** Invoke `derive-dependencies` (report-only by default; it walks every spec — there is no per-spec mode). If it reports drift, the `dependencies:` frontmatter is stale from uncommitted body edits; surface that and recommend committing (the pre-commit hook syncs it) or running `.ductus/bin/ductus derive-dependencies --write` manually. Do not pass `--write` from this command.

## Stuck-detection details

When `check-stuck` (step 3) reports stuck — at least 3 commits on `tasks.md` since the spec's reopening started, with the same task still the first incomplete one — surface the cycle to the user with this message: `Task {N} ({title}) has been touched in {count} prior implement runs without completing. Consider decomposing it into smaller subtasks before continuing.` Pause and wait for user direction; do not auto-decompose. The threshold of 3 is fixed (not configurable in v1) — smallest count that distinguishes routine multi-session work from a cycle.

## Progressive context loading

- **At setup:** read only the spec, plan, tasks, and scenario file (if targeted). Do NOT read `system.md`, `events.md`, `errors.md`, or source code yet.
- **Per task:** read only the source files relevant to that task from the plan's affected files list.
- **At completion:** tally the `tasks.md` checkboxes (primitive: `read-tasks`) and re-read the acceptance criteria from the spec (primitive: `read-spec`) to verify. Do NOT re-read the full plan.

## Walk through tasks (per task, in order)

1. Display the task number, description, and "done when" condition.
2. Read the relevant technical decisions from the plan.
3. Read only the existing code files relevant to this task from the plan's affected files.
4. Implement the task: write code, tests, and migrations as needed. Follow conventions in `AGENTS.md` and `specs/system.md`; respect the contracts defined in the spec. If a write would land outside the runtime boundary, notify the user, explain why, and wait for confirmation before proceeding. Once accepted, the file is part of the boundary for the rest of the session.
5. **Route an incidental issue by scope.** When implementing this task surfaces an issue — a security weakness, a memory or resource leak, a violated convention, a latent bug in adjacent code — the scope it falls in decides where it goes (§brownfield-inbox, Finding dispositions). An issue *inside this task's* scope is fixed as part of the task, not logged. Anything else — except a non-blocking finding met while working a disposition task (below) — becomes an unchecked task on the targeted spec's `tasks.md` the moment it surfaces (the `append-task` primitive):

   - **Inside the targeted spec**, outside this task — a task that implements the fix. A finding that is a requirement gap rather than a defect routes through `/ductus:amend` per §bug-handling, with the task as the work item that implements it.
   - **Outside the spec** — a **disposition task**, titled `Disposition out-of-spec finding: {summary}`, with the body item `{path} — {detail}` and the done-when *the finding is fixed, deferred, or waived, with a waiver's reason written on the task*. Append it with `dedup-title` set, so a re-run of an interrupted walk returns the pending task rather than recording the same finding twice. It is never written to `specs/inbox.md`.

   In every case, record and keep working — do not derail to fix a finding outside this task. The task is on disk as soon as the finding surfaces, so an interrupted run loses nothing, and as an unchecked task it already holds the spec out of `done`. The cost is deliberate: a finding unrelated to the spec holds it until the finding is decided, because this spec's work surfaced it.

   **Working a disposition task.** When the walk reaches one, each finding on it takes exactly one of three dispositions, and the task is checked only once every one has its own. **Fix** it when it is a chore — mechanical, adding no durable requirement (§bug-handling's durability test); a fix that fails or proves not to be mechanical is reverted and is not a chore. **Defer** it, with confirmation, to the home the **§Groom decision tree** in `.ductus/reference/groom.md` chooses — its single canonical statement; do not restate it here — with an external work-item ID beside it or in its place when the operator gives one, naming a `done → in-progress` reopen in the confirmation when the spec deferred to is `done`, and performing it with `set-status` (`from: done`); a deferral to a spec that is not `done` follows `/ductus:amend`'s rules for its status and reopens nothing. Or **waive** it with its reason. Record the outcome on the task's body item — `— fixed`, `— deferred to {work}`, or `— waived: {reason}` — then check it; the waiver's reason written there is the finding's record. A confirmed deferral whose write fails leaves the task unchecked, with the failure reported. A blocking finding — a MUST or SHOULD violation, or a hard-fail or blocking analyze finding — deferred to an external work item alone leaves its task unchecked too, since the task is what holds `done` and no gate reads the tracker (spec 076); its task is checked once it is fixed, waived where its tier allows, or deferred where the deferral releases it (**§Deferrals** in `.ductus/reference/review.md`). A **non-blocking** finding surfaced while working the task — one that is not a MUST or SHOULD violation, nor a hard-fail or blocking analyze finding — is recorded on this same task as a further body item with its own outcome, not appended as a new task, so disposition work does not spawn disposition work. A blocking finding still becomes its own task, as above.
6. Verify the "done when" condition is met.
7. Mark the task as complete in `tasks.md` — update each checkbox to `- [x]`, including nested sub-item checkboxes and a checkbox-form "Done when" clause (`- [ ] Done when: …`), before proceeding. The clause is not an addressable subtask, but leaving it unticked shows an unchecked box under a task the tally calls complete.
8. Prompt the user to commit and push changes. With `--auto` set, skip the prompt: commit on your own, do not push.
9. **Offer the next step.** When at least one task is still unchecked, name the next one and how many remain after it, then ask whether to continue: `Next: task {N} — {heading} ({M} remaining). Continue? (Y/n)`. Yes continues the walk onto that task in the same run; no exits cleanly; free-text instructions redirect ("do task 12 first"), which is a reply rather than a rejection. Name **one** task, not a menu — the count is what lets the operator judge the queue without re-reading `tasks.md`, and looking further ahead is itself a redirect. Suppress the offer when nothing is unchecked, so it never renders as "no next step"; the run continues into the completion gate as it does today. With `--auto` the walk already continues without asking. Nothing here writes, and the offer never advances spec status — the `in-progress → done` transition keeps its own confirmation.

   Edge cases: a task whose "done when" failed halts the run as today (the offer is for a *completed* task, not a way past a failure); every subtask checked with an unticked checkbox-form "Done when" clause is the completion gate's state to surface, not a next-step offer; and a reply targeting a different spec exits cleanly rather than retargeting mid-walk — retargeting is `/ductus:target`'s job, and a walk that silently changed feature would be worse than one that stopped.

10. Before starting the next task, assess whether sufficient context remains to complete it. If context is low, suggest starting a new session.

## Completion gate (after all tasks)

Same order as Instructions steps 9–16, which carry the procedure; these items hold what they leave out.

1. **Task tally**: step 9. If any checkbox remains unchecked, the gate ends there, before criteria verification.
2. **Load acceptance criteria**: step 10.
3. **Verify each criterion**: steps 11 and 12, one criterion at a time, by running its behavior on an input, including one its tests do not name, since a test's name proves only that the test exists.
4. **Cross-spec impact check**: step 13. This detects the cross-spec update you **made**; the one you **owe** is the `cross-spec-impact:` declaration the pre-done gate checks in step 5. An empty result here says nothing about that: the two answer opposite questions.
5. **Pre-done gate** (primitive: `check-review-gate`, step 14; the name is historical, since it gates both `/ductus:review` and `/ductus:analyze`). Its description states the checks in order, first failure wins, and each blocked result carries that check's canonical message and guidance: halt with them and do not propose the transition. A markdown-lint block also carries `violations`: report each one.

   A spec already at `status: done` returns `not gated`, deliberately **not** `passed: true`: a gate reporting a pass for a spec it did not examine could be read as authorizing a transition that has already happened. Nothing is owed; changing what the spec requires means reopening it first through `/ductus:amend`.

   **The gate never writes a signpost.** Which section of an affected spec carries a `cross-spec-impact:` back-link is a routing judgment, and a wrong auto-write into another spec is worse than a precise refusal. Record the change in each affected spec, using `/ductus:amend` against one that is already `done`, and re-run.

   **Freshness compares content, not commits**, since each run reads the working tree while its recorded sha names a commit: `reviewed-digest` against the spec's durable contracts, `analyzed-digest` against its analyze subjects, `reviewed-against` and `analyzed-against` being provenance read only for the mechanical-sweep rename exemption. A record with no digest is **undeterminable**, not stale and not current: it does not block, the passing verdict's guidance says which, and the next run clears it.

   **A review owed holds `done` ahead of every analyze check**, since `review.md` is one of an analysis's subjects, so an analysis run first is stale when the review writes and the reopening takes two. The message names what changed, and a review clears it once its record is committed, because its window moves only at that commit.

   **Of the findings, only a gating one holds `done`** (§brownfield-inbox, Finding dispositions): an undispositioned advisory finding or observation stays recorded and does not block, while a record with no `dispositions:` map blocks, since absence is not zero, and a broken contract the record lists in `open-findings` blocks, since it counts as gating. A record still carrying `fix-rounds-open: true` was cut off before its last write, and a fix round that needed a re-read and has neither one nor a reason in `round-waivers` holds `done` too: both are answered by re-running `/ductus:review`.

6. **Approve the transition**: step 15.
7. **Set `done`**: step 16.
