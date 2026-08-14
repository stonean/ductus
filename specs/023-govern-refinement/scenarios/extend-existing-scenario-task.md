---
section: "3. /elaborate consolidated into /amend"
---

# Extend-existing-scenario-task

## Context

The scenario branch specified in section 3 above creates `scenarios/{slug}.md` and appends a linked task to `tasks.md`. It is the only route in the command surface that calls `append-task`, and it always calls `create-scenario` first — which refuses on slug conflict.

So a scenario whose requirement is *extended* rather than created has no route to a task. The concrete path in: `/{project}:clarify`, scenario-targeted, resolves one of a scenario's open questions into new behavior. The scenario file now describes work that does not exist in `tasks.md`, and no command will put it there:

- `/{project}:amend` — the scenario route refuses on the existing slug; the question route records a question, which is not what the input is.
- `/{project}:plan` — gates on `clarified`; a spec on the reopen cycle sits at `in-progress` and fails the gate.
- `/{project}:implement` — reads `tasks.md` and finds nothing to do.

[§implement-phase](../../framework/constitution.md#implement-phase)'s "if new work is discovered, add it as a task first" therefore has no mechanism on the reopen cycle, and the standing workaround is calling the `append-task` primitive directly — the runtime doing what no command exposes. This is the same shape as the gap 046 closed: a reachable pipeline state with no command routing out of it.

## Behavior

`/{project}:amend` invoked with no input runs a **reconcile pass** over the feature's scenarios, extending the existing re-open precondition (`framework/commands/amend.md:47-68`) rather than adding a branch beside it.

- **Candidate set** — the scenarios in the precondition's delta. The delta's definition belongs to `000-slash-commands`'s `scenario-without-task-visibility` scenario; this route consumes it and does not redefine it. Scenarios outside the delta are not examined, which is what keeps the pass from re-offering long-settled scenarios on every invocation.
- **Per candidate**, read every task in `tasks.md` referencing that scenario, then act on the checkbox state per the second resolved question: a pending task skips the scenario silently; only-completed tasks make it a candidate whose prompt names them; no referencing task offers plainly.
- **On accept**, append a task referencing the existing scenario — `append-task` with the slug, no `create-scenario` call. The scenario file is not created, renamed, duplicated, or rewritten; the body the contributor just authored or clarified is left exactly as it is.
- **Prompting is per scenario**, never batched into one accept-all, since each is a separate judgment about whether the scenario describes unimplemented work.
- **On any task appended**, a `done` parent spec takes the `done → in-progress` back-edge in the same action — the new task is unimplemented behavior by definition. A parent already at `draft`, `clarified`, `planned`, or `in-progress` is left unchanged.
- **The appended task is indistinguishable in shape** from one the create route produces — same heading form, same default body, same `Done when` clause — so the `scenario-consistency` family reads the linkage identically whichever route wrote it.
- **Nothing is written when every candidate is declined**, including no status change.

## Edge Cases

- **The scenario has a pending (unchecked) referencing task** — skipped silently, no prompt. The work is queued and the updated body is what the contributor implements.
- **A slug that does not exist** — this is the create case, unchanged: the existing scenario route runs.
- **The parent spec's `tasks.md` was pruned or reset** (041) — no referencing task survives, so the scenario is offered with the plain prompt. The prompt cannot name a prior task because pruning removed the evidence; that is the pruning decision working as designed, not a duplicate.
- **Several tasks reference the same scenario** — all are read before deciding. One pending task among them skips the scenario; the prompt for an only-completed set names each of them rather than the first.
- **A referencing task names a scenario file that no longer exists** — it is not a candidate (there is no scenario to extend) and the pass leaves it alone. A dangling reference is the `scenario-consistency` family's to report, not this route's to repair.
- **`tasks.md` is absent entirely** — `append-task` creates it, the same as on the create route; no separate handling.
- **A phased `tasks.md`** — the appended task lands under the follow-on phase the way `append-task` already handles it; no new placement rule.
- **The extension arrives while the spec is `draft` or `clarified`** — the spec is already pre-plan, so `/{project}:plan` regenerates tasks from the plan; the route should not fight the forward path.
- **No scenarios directory, or no candidate in the delta** — the precondition behaves exactly as it does today; the reconcile pass adds no output when it has nothing to offer.
- **Every candidate declined** — nothing is written, and the same candidates are offered on the next invocation while they remain in the delta. Declining is not recorded, because recording it would need per-scenario state no artifact holds today.

## Open Questions

*None — all resolved.*

## Resolved Questions

**Which command owns the route — `/{project}:amend`'s scenario branch treating an existing slug as "extend", or scenario-targeted `/{project}:clarify` appending the task as part of resolving a question into behavior?**

`/{project}:amend` owns it, and not as a variant of the create branch: as a **reconcile pass** on the no-argument invocation. `/{project}:amend` with no input walks the feature's scenarios, finds those with no pending task, offers to append one **per scenario**, and takes the `done → in-progress` back-edge when any is added.

`/{project}:clarify` was rejected as the owner. Its scope boundary confines it to resolving questions and forbids planning or implementation work, and it cannot tell a resolution that introduces new behavior from one that confirms existing behavior — most resolutions add no task, so it would have to ask every time. More decisively, the trigger is broader than clarify: a scenario gains behavior when a contributor edits the file directly, with no clarify session involved. A clarify-owned route would not cover that path.

The reconcile shape also subsumes the two narrower entry points considered first — the slug-conflict prompt offering "extend" (`framework/commands/amend.md:111`) and a scenario-targeted declarative input routing to extend instead of the forced question route (`:25`, status table `:180`). Both required the contributor to already know a task was missing; the reconcile pass finds it for them.

Its home is the existing **Re-open precondition** (`framework/commands/amend.md:47-68`), which already fires on a no-argument invocation against a `done` spec and already inspects the feature directory for an on-disk delta. Today it offers only a status flip and appends no task (status table, `:179`). This route extends it rather than adding a section beside it.

Two constraints bound the work:

1. **Ownership split.** Detection — surfacing that a spec carries an untasked scenario, in `/{project}:analyze` and in the precondition's trigger — belongs to `000-slash-commands`'s `scenario-without-task-visibility` scenario, including the fix for the precondition seeing only *uncommitted* deltas. This scenario owns the route that appends the task. The two back-link per §cross-spec-impact rather than both specifying the same behavior.
2. **The discriminator is the operator, for now.** "Untasked" is not "unimplemented": a `done` spec's implemented scenario tasks are expected to have been pruned ([041-task-pruning](../../041-task-pruning/spec.md), spec body and its `/{project}:analyze` criterion). The reconcile pass therefore **prompts per scenario** rather than appending automatically. A mechanical discriminator is the open question on 000's scenario; until it is answered, this route must not decide on its own.

**Does the route dedup against an existing unchecked task that already references the same scenario, or append unconditionally?**

Neither: it reads **every** referencing task for the scenario — checked and unchecked — and the checkbox state selects the prompt rather than whether it looks.

- **A pending (unchecked) referencing task exists** — skip the scenario silently. The work is already queued, and the contributor working that task reads the updated scenario body; a second task would double-count one piece of work.
- **Only completed (checked) referencing tasks exist** — still a candidate, and the prompt names them ("task N references this scenario and is complete"). A checked task means "was implemented", which the extension has just invalidated, so it is not evidence against offering — but the operator makes that call with the existing tasks in front of them rather than answering a bare question.
- **No referencing task at all** — offer plainly. This is the case the `scenario-consistency` family already recognizes.

Deciding from the unchecked set alone was rejected: it would append blind to a scenario that already carries a completed task, which is exactly the duplicate this dedup exists to prevent. Reading half the linkage to decide is weaker than reading all of it and choosing the prompt.

The key widens `scenario-consistency`'s by one parameter rather than introducing a second linkage derivation. That family checks that "every `scenarios/*.md` has a referencing task", checkbox state ignored (`specs/022-deterministic-runtime/data-model.md`, the check-artifacts families). Same scenario listing, same task-reference matching, one added filter — the reuse discipline 046 applied to the question parser.

"Same task-reference matching" is a binding requirement, not a description: a task references a scenario when the **slug** appears in the task's heading, a subtask line, or its `Done when` clause — deliberately tolerant of a hand-written task that names the scenario without the `scenarios/{slug}.md` path. A narrower rule on either surface (matching the full path, say) makes the two disagree, and the disagreement is not symmetric: the reconcile pass would offer a task for a scenario the family already considers mapped, producing exactly the duplicate this dedup exists to prevent.

A shared primitive is **not** what makes the two agree. The markdown-only path has to state the matching rule in prose whatever the runtime exposes (§runtime-host-integration, two paths one contract), so the rule *is* the contract and any primitive is an accelerator over it. Correctness lives in both surfaces naming the same rule, which is why the command file states it inline and cites the family rather than deferring to an implementation.

041 stays intact under this key: a pruned checked task and a present checked task both mean "no pending work", so pruning changes what the prompt can *say* but not what the pass offers.
