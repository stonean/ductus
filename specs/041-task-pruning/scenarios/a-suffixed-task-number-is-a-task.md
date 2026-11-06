---
section: "Behavior"
---

# A-suffixed-task-number-is-a-task

## Context

An adopter on 0.59.11 hand-inserted a task between two others rather than renumber: `### 11. Spent task` and `### 11a. Spent task with a suffixed number` under `## Phase 1`, each fully ticked. `split_numbered_heading` (`runtime/src/primitives/mod.rs`) takes only digits before the dot, so `prune-tasks` returned `removed-count: 1` and `kept-count: 0`, and its `apply` removed `## Phase 1` and task 11, leaving `### 11a.` directly under the `#` title, which fails MD001. With an unticked box under `11a`, `read-tasks` lists no such task, so `/{project}:implement`'s tally misses it. Reproduced at `35a2ed11` (079's task 17).

Since 079's AC27 a box under that heading already holds the gate, drift, `classify-spec-edit`, `prune-plan` and `waiting`, so nothing reaches `done` past it. What remains is `read-tasks`, `mark-task` and `prune-tasks`. No primitive writes the form, so it comes only from a person; this repository holds it only in 023, as `### 1b.`, `### 7b.` and `### 12b.`, all ticked.

## Behavior

- A task number is digits followed by at most one lowercase letter, then the dot. `11.` and `11a.` are task numbers; `11ab.` and `11A.` are not.
- Numbers order by their digits, then the suffix: `11` before `11a` before `12`. `check-artifacts`' numbering check compares a suffixed number in that order rather than skipping it.
- `read-tasks` lists a suffixed task, and `mark-task` addresses it as `11a`.
- `prune-tasks` treats a suffixed task as any task: spent, it leaves, and its phase follows the same rule, so no heading is stranded under the title; pending, it stays.
- `append-task` numbers past the highest digits, so with `11a` the highest it writes `12`, and it never writes a suffix.
- `append-task`'s `task-number` result is a string, since a dedup guard may match `11a`, and reporting `11` would name another task. This is a breaking change on the wire, which the changelog states.
- A suffixed task is held to every rule a numbered one is, `Done when` included.
- `split_numbered_heading` stays the grammar's one home: `iter_task_numbers_at_levels`, `heading_task_number` (`runtime/src/primitives/mark_task.rs`), and `task_number_referencing_scenario` and `pending_task_matching` (`runtime/src/primitives/append_task.rs`) read through it rather than parsing their own digits.

## Edge Cases

- A heading with digits and a letter but no dot (`3a quick wins`) is no task, as `3 quick wins` is not.
- Two sections with one suffixed number, or `11a` ahead of `11`, report as not strictly increasing.
- A suffixed heading above the task level in a mixed file reads as a flat remnant, as `## 11.` does, not as a phase.
- A pruned suffixed number is covered by [the entry on reissuing a number](a-pruned-task-number-is-never-reissued.md): `append-task` never writes a suffix, so its answer does not change.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Is a suffixed number valid?** Yes (owner, 2026-11-06), since it fixes the report with nothing for the adopter to do, where the other answer makes them do the renumbering they were avoiding, and the grammar keeps one home and loses `mark_task.rs`' copy. Each option was prototyped over `64d0be93`'s code and diffed, tests included, mutants, prose and this scenario not. This one is +87 and -66 lines in 7 files, with the adopter's file passing: both sections leave with their phase, and a pending `11a` is listed. It also makes `append-task`'s `task-number` a string, the one place a script could read it as an integer, since in `framework/commands/`, `framework/reference/` and the compiled descriptions it appears only as an argument name. It starts no reopen and no run beyond this spec's owed review: this spec, 022 and 023 are `in-progress`. What this gives up is a stable integer on the wire and a finding against hand-numbering. Rejected: not valid, +83 and -0 lines in 3 files with no wire change, where `check-artifacts` blocks on the heading until a person renumbers it, a correction that reopens nothing, `prune-tasks` keeps it and its phase, and `mark-task` cannot tick it first, while 023 drops its three spent sections, 48 lines, or renumbers 26 headings and 2 citations. The 6 unnumbered level-3 headings in `tasks.md` files sit under flat tasks, below the task level, so neither answer reaches them. Adopters holding the form: at least the reporter, otherwise unknown.
