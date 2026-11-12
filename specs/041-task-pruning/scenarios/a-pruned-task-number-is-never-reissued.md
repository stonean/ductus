---
section: "Behavior"
---

# A-pruned-task-number-is-never-reissued

## Context

`append-task` numbers a new task one past the highest number left in `tasks.md`. Once `/{project}:prune` drops the highest-numbered task, the next append reissues its number, and a commit message, review record or decision citing the pruned task by number then names two tasks. Measured 2026-10-25 on 0.59.11 and HEAD f24c55c6: with tasks 2 and 3, removing 3 and appending gave `## 3.` again. Pruning is what creates the condition; the constitution classifies `tasks.md` as ephemeral and says nothing may treat it as a durable index, yet this repository's own commits cite tasks by number.

## Behavior

- After a prune, a task number that was in use before the prune is never assigned to a new task, so a number cited anywhere keeps naming one task.
- `tasks.md` carries a `next-task:` frontmatter counter, the number the next appended task takes. `prune-tasks` writes it, set past the highest number before the removal, and `append-task` assigns the larger of it and the highest number present plus one, then advances it. Absent, the next number is the highest present plus one, as today.
- The rule is `append-task`'s, with or without the MCP server.
- A `tasks.md` that was never pruned, and carries no counter, numbers exactly as today.

## Edge Cases

- Prune removing every task section (a reset to template state): the reset keeps the counter, so the next append continues past every number the reset removed; a reset `tasks.md` with no counter yields task 1, as AC13 says today.
- Prune removing a middle task: no number is reissued today either, since numbering already runs past the highest left.
- A hand-edited `tasks.md` that deletes a task outside `/{project}:prune`: once the file carries a counter, the deletion lowers nothing, so no number is reissued; a file that never carried one numbers from the highest present, as today.
- A counter at or below the highest number present (a hand edit, a merge): the highest present plus one wins, so a counter can never cause a duplicate.
- A phased `tasks.md` (`### N.` headings): the same rule over its numbers.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **How is a pruned number kept from reissue?** A `next-task:` high-water counter in `tasks.md` frontmatter, like `next-criterion`, which `prune-tasks` writes before removing and `append-task` maintains; a reset keeps it, so AC13's "yields task number 1" holds only for a file with no counter and is amended to say so (owner, 2026-11-12). Measured at `ab2ad085`: 928 of 3,643 commit subjects and 953 mentions in 186 spec files outside `tasks.md` cite a task by number, so a reissued number misdirects real citations. Rejected: citing tasks by title, which depends on every author remembering to (§design-principles) and leaves the existing number citations ambiguous; and deriving the mark from `tasks.md`'s git history, which a shallow clone would under-count without saying so. Numbers reissued before the counter existed stay reissued.
