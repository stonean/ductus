"""Mutation list for the suffixed task number (spec 041, scenario
`a-suffixed-task-number-is-a-task`): `split_numbered_heading` reads digits and
at most one lowercase letter before the dot, `TaskNumber` parses them, and
`read-tasks`, `prune-tasks`, `mark-task`, `append-task` and `check-artifacts`
read the number through them. Each of M1 to M11 puts one consumer back on the
digits-only reading it had before, so a kill shows that consumer's own test
holds the behaviour. M1 reverts the grammar itself, which `read-tasks`,
`prune-tasks` and `mark-task` read directly. M12 to M14 hold `check-artifacts`'
numbering check to its rule (spec 084, AC22): a number repeated anywhere down
the file is reported, and an order is not.

Run with `python3 scripts/mutation/run.py scripts/mutation/suffixed-task-number.py
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
        'M1 the grammar reads digits only, so a suffixed heading is no task',
        'src/primitives/mod.rs',
        'heading.split_at(digits + suffix)',
        'heading.split_at(digits)',
        'suffixed_task',
    ),
    (
        'M2 the numbering walk skips a suffixed number',
        'src/primitives/mod.rs',
        'TaskNumber::parse(split_numbered_heading(&text)?.0)',
        'TaskNumber::parse(split_numbered_heading(&text)?.0).filter(|n| n.suffix.is_none())',
        'suffixed_task',
    ),
    (
        'M3 mark-task reads only the digits of a heading\'s number',
        'src/primitives/mark_task.rs',
        'super::split_numbered_heading(heading).map(|(number, _)| number.to_string())',
        'super::split_numbered_heading(heading).map(|(number, _)| number.trim_end_matches(char::is_lowercase).to_string())',
        'suffixed_task',
    ),
    (
        'M4 the slug guard reports the digits of a suffixed number',
        'src/primitives/append_task.rs',
        'split_numbered_heading(&text).and_then(|(number, _)| TaskNumber::parse(number));',
        'split_numbered_heading(&text).and_then(|(number, _)| TaskNumber::parse(number.trim_end_matches(char::is_lowercase)));',
        'suffixed_task',
    ),
    (
        'M5 the title guard reports the digits of a suffixed number',
        'src/primitives/append_task.rs',
        'TaskNumber::parse(number).map(|number| Section {',
        'TaskNumber::parse(number.trim_end_matches(char::is_lowercase)).map(|number| Section {',
        'suffixed_task',
    ),
    (
        'M6 the numbering check compares a suffixed number as its digits',
        'src/primitives/check_artifacts.rs',
        'super::TaskNumber::parse(&task.number)',
        'super::TaskNumber::parse(task.number.trim_end_matches(char::is_lowercase))',
        'suffixed_task',
    ),
    (
        'M7 the numbering check skips a suffixed number',
        'src/primitives/check_artifacts.rs',
        'super::TaskNumber::parse(&task.number)',
        'super::TaskNumber::parse(&task.number).filter(|n| n.to_string() == n.digits().to_string())',
        'suffixed_task',
    ),
    (
        'M9 an uppercase letter is accepted as a suffix',
        'src/primitives/mod.rs',
        'suffix.is_some_and(|c| !c.is_ascii_lowercase())',
        'suffix.is_some_and(|c| !c.is_ascii_alphabetic())',
        'suffixed_task',
    ),
    (
        'M10 a suffixed task is spared the Done when rule',
        'src/primitives/check_artifacts.rs',
        'if task.done_when.is_none() {',
        'if task.done_when.is_none() && super::TaskNumber::parse(&task.number).is_some_and(|n| n.to_string() == n.digits().to_string()) {',
        'a_suffixed_task_is_held_to_done_when',
    ),
    (
        'M11 a suffixed heading above the task level reads as a phase',
        'src/primitives/mod.rs',
        'split_numbered_heading(heading).is_some()\n}',
        "heading.starts_with(|c: char| c.is_ascii_digit()) && heading.trim_start_matches(|c: char| c.is_ascii_digit()).starts_with('.')\n}",
        'a_suffixed_flat_remnant',
    ),
    (
        'M12 the numbering check reports no repeated number',
        'src/primitives/check_artifacts.rs',
        '&& !seen.insert(number)',
        '&& { seen.insert(number); false }',
        'a_repeated_task_number a_repeated_suffixed_task_number',
    ),
    (
        'M13 the numbering check reports a number below an earlier one',
        'src/primitives/check_artifacts.rs',
        '&& !seen.insert(number)',
        '&& (!seen.insert(number) || seen.iter().any(|earlier| earlier.digits() > number.digits()))',
        'task_numbers_out_of_order_yield_no_finding',
    ),
    (
        'M14 the numbering check compares a number only with the task before it',
        'src/primitives/check_artifacts.rs',
        '&& !seen.insert(number)',
        '&& { let repeat = seen.contains(&number); seen.clear(); seen.insert(number); repeat }',
        'a_repeated_task_number_yields_blocking_finding',
    ),
]
