"""Mutation list for spec 022 task 35: `append-task` holds one advisory lock
across its read of `tasks.md`, its numbering and the write that replaces it
(`BE-TXN-002`), in runtime/src/primitives/append_task.rs. M1 takes no lock;
M2 takes it and releases it at once, before the read, which guards nothing.
The killing tests are `an_append_waits_for_the_lock_and_writes_after_it`,
which holds the lock itself, and `concurrent_appends_on_one_tasks_md_each_land_their_task`,
sixteen calls on one `tasks.md`.

Run with `python3 scripts/mutation/run.py scripts/mutation/append-task-lost-update.py
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
        'M1 append-task reads and writes tasks.md with no lock',
        'src/primitives/append_task.rs',
        '    let _lock = crate::session::lock_append_task(repo)?;\n',
        '',
        'append_task::tests::',
    ),
    (
        'M2 append-task releases the lock before it reads tasks.md',
        'src/primitives/append_task.rs',
        '    let _lock = crate::session::lock_append_task(repo)?;\n',
        '    drop(crate::session::lock_append_task(repo)?);\n',
        'append_task::tests::',
    ),
]
