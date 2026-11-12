"""Mutation list for the suffixed task number (spec 041, scenario
`a-suffixed-task-number-is-a-task`): `split_numbered_heading` reads digits and
at most one lowercase letter before the dot, `TaskNumber` orders them, and
`read-tasks`, `prune-tasks`, `mark-task`, `append-task` and `check-artifacts`
read the number through them. Each mutation puts one consumer back on the
digits-only reading it had before, so a kill shows that consumer's own test
holds the behaviour. M1 reverts the grammar itself, which `read-tasks`,
`prune-tasks` and `mark-task` read directly.

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
        'M8 a number orders by its suffix before its digits',
        'src/primitives/mod.rs',
        '    digits: u32,\n    suffix: Option<char>,\n}',
        '    suffix: Option<char>,\n    digits: u32,\n}',
        'suffixed_task',
    ),
    (
        'M9 an uppercase letter is accepted as a suffix',
        'src/primitives/mod.rs',
        'suffix.is_some_and(|c| !c.is_ascii_lowercase())',
        'suffix.is_some_and(|c| !c.is_ascii_alphabetic())',
        'suffixed_task',
    ),
]
