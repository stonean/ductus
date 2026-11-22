"""Mutation list for spec 066 tasks 29, 30 and 31 (scenarios
`a-commit-after-the-scope-was-resolved-refuses-the-write` and
`a-write-result-states-what-it-wrote`): `compute-review-scope` returns the
head its scope was resolved against, `write-review` refuses a write whose
scope a later commit moved, in a refusal whose size is bounded whatever moved
(task 31, BE-PAYLOAD-001), and each write result states the counts its record
holds. Each mutation drops one part of those rules; a test kills each.

Run with `python3 scripts/mutation/run.py scripts/mutation/moved-scope-066.py
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
        'M1 the result reports the base as its head',
        'src/primitives/compute_review_scope.rs',
        '        head: scope.head.clone(),\n',
        '        head: scope.diff_base.clone(),\n',
        'primitives::compute_review_scope::tests::head_is_the_commit_the_scope_was_resolved_against_not_the_base',
    ),
    (
        'M2 write-review does not guard a moved scope',
        'src/primitives/write_review.rs',
        '    moved_scope::refuse_a_moved_scope(&args.reviewed_against, repo, &entries)?;\n',
        '',
        'primitives::write_review::moved_scope::tests::write_review_refuses_a_write_whose_scope_a_later_commit_moved',
    ),
    (
        'M3 a commit after the head that changed no scope path refuses',
        'src/primitives/write_review/moved_scope.rs',
        '            .filter(|path| entries.contains(path))\n',
        '            .filter(|_| true)\n',
        'primitives::write_review::moved_scope::tests::a_commit_after_the_head_that_changed_no_scope_path_writes',
    ),
    (
        'M4 a head that does not descend from reviewed-against is accepted',
        'src/primitives/write_review/moved_scope.rs',
        '    if !repository.graph_descendant_of(head.id(), against.id())? {\n',
        '    if !repository.graph_descendant_of(head.id(), against.id())? && false {\n',
        'primitives::write_review::moved_scope::tests::a_head_that_does_not_descend_from_it_is_an_argument_error',
    ),
    (
        'M5 a path two commits changed counts twice',
        'src/primitives/write_review/moved_scope.rs',
        '            if let Some(&at) = position.get(&path) {\n',
        '            if let Some(&at) = None::<&usize> {\n',
        'primitives::write_review::moved_scope::tests::a_path_two_later_commits_changed_counts_once_naming_both',
    ),
    (
        'M6 the remedy omits the moved paths the scope already held',
        'src/primitives/write_review/moved_scope.rs',
        'read each moved path and \\\n         each path it adds',
        'read each path \\\n         it adds',
        'primitives::write_review::moved_scope::tests::a_commit_after_the_head_that_changed_a_scope_path_refuses_naming_it',
    ),
    (
        "M7 write-analysis's result echoes the hard-fail it was passed",
        'src/primitives/write_analysis.rs',
        '            hard_fail,\n            blocking_findings,\n            advisory: args.advisory,\n',
        '            hard_fail: args.hard_fail,\n            blocking_findings,\n            advisory: args.advisory,\n',
        'primitives::write_analysis::tests::the_result_states_each_count_the_record_holds',
    ),
    (
        "M8 write-analysis's result zeroes advisory",
        'src/primitives/write_analysis.rs',
        '            hard_fail,\n            blocking_findings,\n            advisory: args.advisory,\n',
        '            hard_fail,\n            blocking_findings,\n            advisory: 0,\n',
        'primitives::write_analysis::tests::the_result_states_each_count_the_record_holds',
    ),
    (
        'M9 every moved path is named',
        'src/primitives/write_review/moved_scope.rs',
        '        .take(NAMED_PATHS)\n',
        '',
        'primitives::write_review::moved_scope::tests::a_refusal_names_three_moved_paths_and_counts_the_rest',
    ),
    (
        "M10 every one of a moved path's commits is named",
        'src/primitives/write_review/moved_scope.rs',
        '                .take(NAMED_COMMITS)\n',
        '',
        'primitives::write_review::moved_scope::tests::a_moved_path_names_three_of_its_commits_and_counts_the_rest',
    ),
    (
        'M11 a long commit subject is kept whole',
        'src/primitives/write_review/moved_scope.rs',
        '    if subject.chars().count() <= SUBJECT_CHARS {\n',
        '    if true {\n',
        'primitives::write_review::moved_scope::tests::a_long_commit_subject_is_cut',
    ),
    (
        'M12 the refusal omits the command that lists every changed path',
        'src/primitives/write_review/moved_scope.rs',
        '`git diff --name-only {} HEAD` lists',
        '{} lists',
        'primitives::write_review::moved_scope::tests::a_refusal_names_three_moved_paths_and_counts_the_rest',
    ),
    (
        'M13 the paths past the named ones are not counted',
        'src/primitives/write_review/moved_scope.rs',
        '        format!(", and {} more", moved.len() - NAMED_PATHS)\n',
        '        String::new()\n',
        'primitives::write_review::moved_scope::tests::a_refusal_names_three_moved_paths_and_counts_the_rest',
    ),
]
