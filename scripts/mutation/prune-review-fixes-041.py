"""Mutation list for spec 041's review fixes: no task number or counter wraps
past `u32::MAX` (`prune-tasks` and `append-task` refuse with `malformed-tasks`
and write nothing), a `#` line in `tasks.md`'s frontmatter is no heading in
either `prune-tasks` mode, and a signed number (`+5.`) is no task. Each
mutation puts one of those back, so a kill shows its own test holds it.

Run with `python3 scripts/mutation/run.py scripts/mutation/prune-review-fixes-041.py
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
        'M1 the counter successor wraps instead of refusing',
        'src/primitives/tasks_counter.rs',
        'number\n        .checked_add(1)\n        .ok_or_else(',
        'Some(number.wrapping_add(1))\n        .ok_or_else(',
        'a_number_at_the_ceiling',
    ),
    (
        'M2 prune-tasks wraps the counter it writes past the highest number',
        'src/primitives/prune_tasks.rs',
        'let past = super::tasks_counter::after(\n            super::tasks_counter::highest_present(&content),\n            &tasks_path,\n        )?;',
        'let past = super::tasks_counter::highest_present(&content).wrapping_add(1);',
        'a_number_with_no_counter_past_it',
    ),
    (
        'M3 append-task wraps the number it assigns past the highest present',
        'src/primitives/append_task.rs',
        'let past = super::tasks_counter::after(super::tasks_counter::highest_present(content), path)?;',
        'let past = super::tasks_counter::highest_present(content).wrapping_add(1);',
        'a_number_with_no_successor',
    ),
    (
        'M4 append-task wraps the counter it advances past the number it assigns',
        'src/primitives/append_task.rs',
        'super::tasks_counter::after(next_number, &tasks_path)?,',
        'next_number.wrapping_add(1),',
        'a_number_with_no_successor',
    ),
    (
        'M5 keep-pending reads a frontmatter line as structure',
        'src/primitives/prune_tasks.rs',
        'cur.lines.extend(frontmatter.lines().map(str::to_string));\n    let mut region = OpenRegion::default();',
        'let mut region = OpenRegion::default();',
        'a_comment_in_the_frontmatter_is_no_heading',
    ),
    (
        'M6 keep-pending scans the frontmatter block for headings',
        'src/primitives/prune_tasks.rs',
        'for (index, line) in body.lines().enumerate() {',
        'for (index, line) in content.lines().enumerate() {',
        'a_comment_in_the_frontmatter_is_no_heading',
    ),
    (
        'M7 a reset takes the first `#` line of the whole file as the heading',
        'src/primitives/prune_tasks.rs',
        'let h1 = content[frontmatter.len()..]\n        .lines()',
        'let h1 = content\n        .lines()',
        'a_comment_in_the_frontmatter_is_no_heading',
    ),
    (
        'M8 the shared grammar reads a leading sign as part of a number',
        'src/primitives/mod.rs',
        'pub(crate) fn split_numbered_heading(heading: &str) -> Option<(&str, &str)> {\n',
        'pub(crate) fn split_numbered_heading(heading: &str) -> Option<(&str, &str)> {\n    let heading = heading.strip_prefix(\'+\').unwrap_or(heading);\n',
        'a_signed_number_is_no_task',
    ),
]
