"""Mutation list for spec 022 task 38 (`BE-TXN-002`): the runtime-core writers
`append-question`, `label-criteria`, `mark-criterion`, `mark-task`,
`set-status`, `append-inbox`, `remove-inbox-item`, `merge-managed-block`,
`merge-permissions` and `append-task` write through the compare-before-persist
guard (`ReadSnapshot`, via `guarded::write_guarded`). M1-M10 each make one
primitive write the raw way it did before, so the write replaces a change
another writer landed after the read; the primitive's own test, which lands a
change between its read and its write, kills it. M11 adds a raw write to a
primitive outside the allowlist, which the allowlist test kills. M12 and M13
remove the guarded write calls from `tests/line_ending_discipline.rs`'s
subject, which its guarded-writer count then turns red. The tests
that kill M1-M10 hold the write refused and the other writer's change kept;
`append-task`'s holds a `mark-task` landing between its read and its write.

Run with `python3 scripts/mutation/run.py scripts/mutation/compare-and-set-022.py
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
        'M1 append-question writes over a change made after its read',
        'src/primitives/append_question.rs',
        '    guarded::write_guarded(&snapshot, &new_content)?;\n',
        '    crate::primitives::write_atomic(&target_path, &new_content)?;\n',
        'append_question::tests::a_spec_changed_after_the_read_is_refused_and_kept',
    ),
    (
        'M2 label-criteria writes over a change made after its read',
        'src/primitives/label_criteria.rs',
        '    guarded::write_guarded(&snapshot, &new_content)?;\n',
        '    crate::primitives::write_atomic(&spec_path, &new_content)?;\n',
        'label_criteria::tests::a_spec_changed_after_the_read_is_refused_and_kept',
    ),
    (
        'M3 mark-criterion writes over a change made after its read',
        'src/primitives/mark_criterion.rs',
        '        guarded::write_guarded(&snapshot, &new_content)?;\n',
        '        crate::primitives::write_atomic(&spec_path, &new_content)?;\n',
        'mark_criterion::tests::a_spec_changed_after_the_read_is_refused_and_kept',
    ),
    (
        'M4 mark-task writes over a change made after its read',
        'src/primitives/mark_task.rs',
        '        guarded::write_guarded(&snapshot, &new_content)?;\n',
        '        crate::primitives::write_atomic(&tasks_path, &new_content)?;\n',
        'mark_task::tests::a_tasks_file_changed_after_the_read_is_refused_and_kept',
    ),
    (
        'M5 set-status writes over a change made after its read',
        'src/primitives/set_status.rs',
        '        guarded::write_guarded(&snapshot, &new_content)?;\n',
        '        crate::primitives::write_atomic(&spec_path, &new_content)?;\n',
        'set_status::tests::a_spec_changed_after_the_read_is_refused_and_kept',
    ),
    (
        'M6 append-inbox writes over a change made after its read',
        'src/primitives/append_inbox.rs',
        '    guarded::write_guarded(&snapshot, &new_content)?;\n',
        '    crate::primitives::write_atomic(&inbox_path, &new_content)?;\n',
        'append_inbox::tests::an_inbox_changed_after_the_read_is_refused_and_kept append_inbox::tests::an_inbox_created_after_the_read_is_refused_and_kept',
    ),
    (
        'M7 remove-inbox-item writes over a change made after its read',
        'src/primitives/remove_inbox_item.rs',
        '            guarded::write_guarded(&snapshot, &new_content)?;\n',
        '            crate::primitives::write_atomic(&inbox_path, &new_content)?;\n',
        'remove_inbox_item::tests::an_inbox_changed_after_the_read_is_refused_and_kept',
    ),
    (
        'M8 merge-managed-block writes over a change made after its read',
        'src/primitives/merge_managed_block.rs',
        '        guarded::write_guarded(&snapshot, content)?;\n',
        '        crate::primitives::write_atomic(&path, content)?;\n',
        'merge_managed_block::tests::a_file_changed_after_the_read_is_refused_and_kept merge_managed_block::tests::a_file_created_after_the_read_is_refused_and_kept',
    ),
    (
        'M9 merge-permissions writes over a change made after its read',
        'src/primitives/merge_permissions.rs',
        '        guarded::write_guarded(&snapshot, &serialized)?;\n',
        '        crate::primitives::write_atomic(&target_path, &serialized)?;\n',
        'merge_permissions::tests::a_settings_file_changed_after_the_read_is_refused_and_kept merge_permissions::tests::a_settings_file_created_after_the_read_is_refused_and_kept',
    ),
    (
        'M10 append-task writes over a change made after its read',
        'src/primitives/append_task.rs',
        '    guarded::write_guarded(&snapshot, &new_content)?;\n',
        '    crate::primitives::write_atomic(&tasks_path, &new_content)?;\n',
        'append_task::tests::a_mark_task_between_the_read_and_the_write_keeps_both_changes',
    ),
    (
        'M11 a primitive outside the allowlist writes a file raw',
        'src/primitives/check_stuck.rs',
        'pub fn run(args: &CheckStuckArgs, repo: &Path) -> Result<CheckStuckResult> {\n',
        'pub fn run(args: &CheckStuckArgs, repo: &Path) -> Result<CheckStuckResult> {\n    let _ = crate::primitives::write_atomic(&repo.join("raw.md"), "");\n',
        'raw_writes_allowlist::every_raw_write_is_the_helpers_or_on_the_allowlist',
    ),
    (
        'M12 the line-ending lint no longer counts write_if_unchanged as a writer',
        'tests/line_ending_discipline.rs',
        '    "write_new_atomic(",\n    "write_if_unchanged(",\n    "write_guarded(",\n];',
        '    "write_new_atomic(",\n    "write_guarded(",\n];',
        '--test line_ending_discipline',
    ),
    (
        'M13 the line-ending lint no longer counts write_guarded as a writer',
        'tests/line_ending_discipline.rs',
        'const GUARDED_CALLS: &[&str] = &["write_if_unchanged(", "write_guarded("];',
        'const GUARDED_CALLS: &[&str] = &["write_if_unchanged("];',
        '--test line_ending_discipline',
    ),
]
