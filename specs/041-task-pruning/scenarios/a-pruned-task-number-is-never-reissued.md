---
section: "Behavior"
---

# A-pruned-task-number-is-never-reissued

## Context

`append-task` numbers a new task one past the highest number left in `tasks.md`. Once `/{project}:prune` drops the highest-numbered task, the next append reissues its number, and a commit message, review record or decision citing the pruned task by number then names two tasks. Measured 2026-10-25 on 0.59.11 and HEAD f24c55c6: with tasks 2 and 3, removing 3 and appending gave `## 3.` again. Pruning is what creates the condition; the constitution classifies `tasks.md` as ephemeral and says nothing may treat it as a durable index, yet this repository's own commits cite tasks by number.

## Behavior

- After a prune, a task number that was in use before the prune is never assigned to a new task, so a number cited anywhere keeps naming one task.
- The rule holds on both paths: the runtime's `append-task` and a host appending by hand under the markdown-only reference.
- A `tasks.md` that was never pruned numbers exactly as today.

## Edge Cases

- Prune removing every task section (a reset to template state): AC13 says the next append yields task 1, which reissues every number the reset removed.
- Prune removing a middle task: no number is reissued today either, since numbering already runs past the highest left.
- A hand-edited `tasks.md` that deletes a task outside `/{project}:prune`.
- A phased `tasks.md` (`### N.` headings): the same rule over its numbers.

## Open Questions

- How is a pruned number kept from reissue: a high-water counter in `tasks.md` frontmatter (like `next-criterion`) that prune and `append-task` maintain — and does a reset restart it, amending AC13 or not — or no counter, with citations required to name a task by its title because `tasks.md` is not a durable index?

## Resolved Questions

*None yet.*
