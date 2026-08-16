---
section: "Behavior"
---

# Implement-offers-the-next-step

## Context

`--auto` and the default mode differ in *what is confirmed*, not in how much the operator is told. Today, a default-mode `/{project}:implement` run finishing one task renders its per-task completion summary — the task processed, cross-spec impact, captured issues, a reminder to commit — and stops. The operator then has to re-derive what comes next and re-issue the command, even though the walk already holds the ordered task list and knows exactly which task is next.

That is friction in the mode that is supposed to be the *considered* one. Confirming each step is the point of not passing `--auto`; making the operator reconstruct the step before they can confirm it is not. It also loses the moment where redirection is cheapest: the operator has just seen what landed and is best placed to say "not that next, this instead" — but only if the run tells them what "that next" is.

## Behavior

**On completing a task without `--auto`, the run names the next step and asks.** After the per-task completion summary and before exiting, when the ordered task list holds at least one unchecked task, the run names the next recommended task — its number and heading — plus how many unchecked tasks remain after it, and prompts the operator to continue with it. **One** task, not a menu; the count is what lets the operator judge the queue without re-reading `tasks.md` (see Resolved Questions).

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

*None — see Resolved Questions.*

## Resolved Questions

- **Should the offer surface more than one next step?** No — **name one, and say
  how many remain.** Resolved 2026-08-16 by the operator.

  Naming several turns the prompt into a menu that re-renders a slice of
  `tasks.md` after every completion, which is the cost the question named. The
  count recovers most of what the menu was for: the operator learns whether the
  queue is one task or twelve without reading it, and the free-text reply the
  Behavior section already specifies is how they look further ahead when they
  want to — "what's after that?" is a redirect like any other. So the prompt
  stays one line and the information the menu carried is available on demand
  rather than by default.

  The count is of **unchecked tasks remaining after the one being offered**, so
  the last task offers `(0 remaining)` rather than suppressing the count — a
  bare "next: task 91" and "next: task 91 (0 remaining)" answer different
  questions, and the second is the one an operator deciding whether to continue
  is actually asking.
