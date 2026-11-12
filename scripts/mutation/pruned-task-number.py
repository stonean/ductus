"""Mutation list for the pruned task number (spec 041, scenario
`a-pruned-task-number-is-never-reissued`): `prune-tasks` writes a `next-task:`
frontmatter counter past the highest number before it removes a section,
`append-task` assigns the larger of the counter and the highest number present
plus one and advances it, and a reset keeps the frontmatter block. Each
mutation removes one of those behaviours, so a kill shows its own test holds it.

Run with `python3 scripts/mutation/run.py scripts/mutation/pruned-task-number.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 append-task ignores the counter and numbers from the highest present',
        'src/primitives/append_task.rs',
        '(super::tasks_counter::highest_present(content) + 1).max(counter.unwrap_or(0))',
        'super::tasks_counter::highest_present(content) + 1',
        'a_counter_past_the_highest',
    ),
    (
        'M2 append-task does not advance the counter',
        'src/primitives/append_task.rs',
        'Some(_) => super::tasks_counter::set(&new_content, next_number + 1, &tasks_path)?,',
        'Some(_) => new_content,',
        'a_counter_past_the_highest',
    ),
    (
        'M3 append-task lets a counter below the highest present win',
        'src/primitives/append_task.rs',
        '(super::tasks_counter::highest_present(content) + 1).max(counter.unwrap_or(0))',
        'counter.unwrap_or(super::tasks_counter::highest_present(content) + 1)',
        'a_counter_at_or_below',
    ),
    (
        'M4 append-task creates a counter in a file that has none',
        'src/primitives/append_task.rs',
        'None => new_content,',
        'None => super::tasks_counter::set(&new_content, next_number + 1, &tasks_path)?,',
        'a_file_without_a_counter_gains_none',
    ),
    (
        'M5 prune-tasks writes no counter',
        'src/primitives/prune_tasks.rs',
        'if removed > 0 || held.is_some() {',
        'if false {',
        'a_pruned_top_number',
    ),
    (
        'M6 prune-tasks lowers a counter already past the highest number',
        'src/primitives/prune_tasks.rs',
        'past.max(held.unwrap_or(0))',
        'past',
        'a_prune_never_lowers',
    ),
    (
        'M7 a reset drops the frontmatter block',
        'src/primitives/prune_tasks.rs',
        'let frontmatter = super::tasks_counter::frontmatter(content, tasks_path)?;',
        'let frontmatter = "";',
        'a_reset_keeps_the_frontmatter_block',
    ),
    (
        'M8 the highest number is read at the flat level only',
        'src/primitives/tasks_counter.rs',
        'iter_task_numbers_at_levels(content, &[2, 3])',
        'iter_task_numbers_at_levels(content, &[2])',
        'a_phased_prune_counts',
    ),
    (
        'M9 a zero counter is accepted',
        'src/primitives/tasks_counter.rs',
        'Ok(value) if value >= 1 => Ok(Some(value)),',
        'Ok(value) => Ok(Some(value)),',
        'an_unreadable_counter',
    ),
    (
        'M10 a counter that cannot be read stops no append',
        'src/primitives/append_task.rs',
        'let counter = super::tasks_counter::read(&existing, &tasks_path)?;',
        'let counter = super::tasks_counter::read(&existing, &tasks_path).unwrap_or(None);',
        'an_unreadable_counter_stops_the_append',
    ),
]
