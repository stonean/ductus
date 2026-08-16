---
section: "Behavior"
---

# Implement-offers-the-next-step

## Context

`--auto` and the default mode differ in *what is confirmed*, not in how much the operator is told. Today, a default-mode `/{project}:implement` run finishing one task renders its per-task completion summary — the task processed, cross-spec impact, captured issues, a reminder to commit — and stops. The operator then has to re-derive what comes next and re-issue the command, even though the walk already holds the ordered task list and knows exactly which task is next.

That is friction in the mode that is supposed to be the *considered* one. Confirming each step is the point of not passing `--auto`; making the operator reconstruct the step before they can confirm it is not. It also loses the moment where redirection is cheapest: the operator has just seen what landed and is best placed to say "not that next, this instead" — but only if the run tells them what "that next" is.

## Behavior

**On completing a task without `--auto`, the run names the next step and asks.** After the per-task completion summary and before exiting, when the ordered task list holds at least one unchecked task, the run names the next recommended task — its number and heading — and prompts the operator to continue with it.

**The prompt is a fork, not a gate.** Answering yes continues the walk onto that task within the same run. Answering no exits cleanly, exactly as the run ends today. Answering with instructions instead redirects: the operator's text is the next input, so "do task 12 first" or "stop and re-plan" is a first-class reply rather than a rejection followed by a fresh invocation.

**It fires only when there is something to offer.** With every task checked, the run proceeds to the completion gate as it does today — the prompt is never rendered as "no next step". With `--auto` set, nothing changes: the walk already continues without asking, and §pipeline-boundaries' gates that fire even under `--auto` are untouched.

**It confirms work, not transitions.** This is the per-task boundary, not the pipeline-completion gate. The `in-progress → done` transition keeps its own separate confirmation, which `--auto` does not bypass; offering the next *task* never advances the spec's *status*.

## Edge Cases

- Last task in the list: no prompt; the run continues into the completion gate as today.
- Every remaining task already checked but the block's `Done when` clause is unticked: that is the existing "all subtasks checked, block not complete" state the completion gate names — it is surfaced there, not turned into a next-step offer.
- A task whose `Done when` failed: the run halts on that as it does today; the offer is for a *completed* task, not a way past a failure.
- The operator replies with instructions that target a different spec entirely: the run exits cleanly rather than retargeting mid-walk — retargeting is `/{project}:target`'s job, and a walk that silently changed feature would be worse than one that stopped.
- A run driven by `ductus exec`: the prompt is host-facing, so it takes the same extension round trip the walk's other confirmations use rather than blocking the subprocess.
- Nothing about the prompt writes: declining, redirecting, or exiting all leave the same files on disk that the per-task walk had already written.

## Open Questions

- Should the offer surface more than one next step — for example the next two or three unchecked tasks — so the operator can redirect further ahead without re-reading `tasks.md`? Naming one keeps the prompt short and matches "the next recommended step"; naming several turns it into a menu, which may be more useful when tasks are small and sequential but risks re-rendering the task list on every completion.

## Resolved Questions

*None yet.*
