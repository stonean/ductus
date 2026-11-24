"""Mutation list for spec 084 task 1: the owed-work test
(`runtime/src/primitives/owed_work.rs`) and `check-owed-work`
(`runtime/src/primitives/check_owed_work.rs`). A spec owes work while its
`tasks.md` holds an unchecked box or its `spec.md` an unticked acceptance
criterion. Each half of the helper is mutated once per reader, and killed by
that reader's own test rather than the helper's, so a reader that stopped
reading through the helper would let its mutant survive.

Run with `python3 scripts/mutation/run.py scripts/mutation/owed-work-084.py
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
        'M1 check-artifacts reads no unchecked box on a done spec',
        'src/primitives/owed_work.rs',
        '    unchecked_lines(tasks)\n}',
        '    let _ = tasks;\n    BTreeMap::new()\n}',
        'check_artifacts::tests::a_done_spec_holding_an_unchecked_box_is_review_state_drift',
    ),
    (
        'M2 the gate reads no unchecked box',
        'src/primitives/owed_work.rs',
        '    unchecked_lines(tasks)\n}',
        '    let _ = tasks;\n    BTreeMap::new()\n}',
        'check_review_gate::tests::an_unchecked_box_blocks_wherever_it_sits',
    ),
    (
        'M3 measure-reopenings reads an in-progress spec with a box as waiting',
        'src/primitives/owed_work.rs',
        '    unchecked_lines(tasks)\n}',
        '    let _ = tasks;\n    BTreeMap::new()\n}',
        'measure_reopenings::tests::waiting_is_the_in_progress_specs_with_no_unchecked_task',
    ),
    (
        'M4 check-owed-work reads no unchecked box',
        'src/primitives/owed_work.rs',
        '    unchecked_lines(tasks)\n}',
        '    let _ = tasks;\n    BTreeMap::new()\n}',
        'check_owed_work::tests::a_box_alone_or_a_criterion_alone_owes_work',
    ),
    (
        'M5 check-artifacts reads no unticked criterion on a done spec',
        'src/primitives/owed_work.rs',
        '        .filter(|(_, criterion)| !criterion.checked)\n}',
        '        .filter(|_| false)\n}',
        'check_artifacts::tests::a_done_spec_with_an_unticked_criterion_has_drifted',
    ),
    (
        'M6 the gate reads no unticked criterion',
        'src/primitives/owed_work.rs',
        '        .filter(|(_, criterion)| !criterion.checked)\n}',
        '        .filter(|_| false)\n}',
        'check_review_gate::tests::an_unticked_criterion_holds_done_after_an_unchecked_task',
    ),
    (
        'M7 check-owed-work reads no unticked criterion',
        'src/primitives/owed_work.rs',
        '        .filter(|(_, criterion)| !criterion.checked)\n}',
        '        .filter(|_| false)\n}',
        'check_owed_work::tests::a_box_alone_or_a_criterion_alone_owes_work',
    ),
    (
        'M8 check-artifacts reads a ticked criterion as unticked',
        'src/primitives/owed_work.rs',
        '        .filter(|(_, criterion)| !criterion.checked)\n}',
        '        .filter(|_| true)\n}',
        'check_artifacts::tests::a_done_spec_with_every_criterion_ticked_has_not',
    ),
    (
        'M9 check-owed-work reads a ticked criterion as unticked',
        'src/primitives/owed_work.rs',
        '        .filter(|(_, criterion)| !criterion.checked)\n}',
        '        .filter(|_| true)\n}',
        'check_owed_work::tests::a_spec_with_neither_owes_nothing',
    ),
    (
        'M10 a spec owes work only when it holds both a box and a criterion',
        'src/primitives/owed_work.rs',
        '        !self.unchecked_boxes.is_empty() || !self.unticked_criteria.is_empty()',
        '        !self.unchecked_boxes.is_empty() && !self.unticked_criteria.is_empty()',
        'check_owed_work::tests::a_box_alone_or_a_criterion_alone_owes_work',
    ),
    (
        'M11 the corpus form judges a spec that is not done',
        'src/primitives/check_owed_work.rs',
        '    if status != DONE {\n        return Ok(Judged::NotDone);\n    }\n',
        '    let _ = DONE;\n',
        'check_owed_work::tests::the_corpus_form_names_only_the_owing_done_specs_and_counts_each_examined',
    ),
    (
        'M12 the corpus form names a done spec that owes nothing',
        'src/primitives/check_owed_work.rs',
        '                if owes {\n',
        '                if owes || !owes {\n',
        'check_owed_work::tests::the_corpus_form_names_only_the_owing_done_specs_and_counts_each_examined',
    ),
    (
        'M13 the corpus form counts no spec it examined',
        'src/primitives/check_owed_work.rs',
        '                found.examined += 1;\n',
        '',
        'check_owed_work::tests::the_corpus_form_names_only_the_owing_done_specs_and_counts_each_examined',
    ),
    (
        'M14 an untracked spec is judged',
        'src/primitives/check_owed_work.rs',
        '    if untracked.contains(&format!("{root}/{feature}/spec.md")) {',
        '    if untracked.is_empty() && !untracked.is_empty() {',
        'check_owed_work::tests::an_untracked_spec_is_skipped_and_named',
    ),
    (
        'M15 a spec linked out of the project is read',
        'src/primitives/check_owed_work.rs',
        '    match validate_feature_argument(repo, root, feature) {',
        '    match Ok::<(), PrimitiveError>(()) {',
        'check_owed_work::tests::a_spec_linked_out_of_the_project_is_skipped_and_refused',
    ),
    (
        'M16 a spec whose status will not read is passed over as not done',
        'src/primitives/check_owed_work.rs',
        '        return Ok(Judged::Skipped(OwedWorkSkipReason::StatusUnreadable));',
        '        return Ok(Judged::NotDone);',
        'check_owed_work::tests::a_spec_the_corpus_form_cannot_judge_is_skipped_with_its_reason',
    ),
    (
        'M17 a directory with no spec.md is passed over',
        'src/primitives/check_owed_work.rs',
        '        return Ok(Judged::Skipped(OwedWorkSkipReason::NoSpecFile));',
        '        return Ok(Judged::NotDone);',
        'check_owed_work::tests::a_spec_the_corpus_form_cannot_judge_is_skipped_with_its_reason',
    ),
    (
        'M18 an unreadable tasks.md reads as holding no box',
        'src/primitives/check_owed_work.rs',
        '        let Ok(tasks) = read_text(&tasks_path) else {\n            return Ok(Judged::Skipped(OwedWorkSkipReason::Unreadable));\n        };\n        Some(tasks)',
        '        read_text(&tasks_path).ok()',
        'check_owed_work::tests::a_spec_the_corpus_form_cannot_judge_is_skipped_with_its_reason',
    ),
    (
        'M19 a call naming neither feature nor all reads the corpus',
        'src/primitives/check_owed_work.rs',
        '        (None, true) => bound_corpus(repo, corpus_owed(repo, &root)?),',
        '        (None, _) => bound_corpus(repo, corpus_owed(repo, &root)?),',
        'check_owed_work::tests::a_call_names_a_feature_or_all_never_both_or_neither',
    ),
    (
        'M20 a call naming both reads the feature',
        'src/primitives/check_owed_work.rs',
        '        (Some(feature), false) => {',
        '        (Some(feature), _) => {',
        'check_owed_work::tests::a_call_names_a_feature_or_all_never_both_or_neither',
    ),
    (
        'M21 a named spec with no spec.md owes nothing',
        'src/primitives/check_owed_work.rs',
        '    if !spec_path.is_file() {\n        return Err(PrimitiveError::MissingSpecFile {',
        '    if !spec_path.is_file() {\n        return Ok((owed_spec(feature, &OwedWork::default()), false));\n    }\n    if !spec_path.is_file() {\n        return Err(PrimitiveError::MissingSpecFile {',
        'check_owed_work::tests::a_missing_spec_file_is_an_error_never_owing_nothing',
    ),
    (
        'M22 the corpus form owes the exec walker an examined list',
        'src/primitives/bounded_lists.rs',
        '    ("check-owed-work", "examined-count", None),\n',
        '',
        'check_owed_work::tests::past_the_read_size_the_lists_move_to_a_file',
    ),
    # Task 2 (AC2): a done spec owes work from its state, not from what an edit
    # touched, so an edit to another file of the spec reopens nothing.
    (
        'M23 a named spec owes the boxes its data model quotes',
        'src/primitives/check_owed_work.rs',
        '    let tasks_path = dir.join("tasks.md");\n    let tasks = if tasks_path.is_file() {\n        Some(read_text(&tasks_path)?)',
        '    let tasks_path = dir.join("data-model.md");\n    let tasks = if tasks_path.is_file() {\n        Some(read_text(&tasks_path)?)',
        'check_owed_work::tests::only_an_added_criterion_or_an_appended_task_makes_a_done_spec_owe_work',
    ),
    (
        'M24 an added criterion owes nothing',
        'src/primitives/owed_work.rs',
        'const ACCEPTANCE_CRITERIA: &str = "Acceptance Criteria";',
        'const ACCEPTANCE_CRITERIA: &str = "Resolved Questions";',
        'check_owed_work::tests::only_an_added_criterion_or_an_appended_task_makes_a_done_spec_owe_work',
    ),
]
