"""Mutation list for spec 066 tasks 29 and 30 (scenarios
`a-commit-after-the-scope-was-resolved-refuses-the-write` and
`a-write-result-states-what-it-wrote`): `compute-review-scope` returns the
head its scope was resolved against, `write-review` refuses a write whose
scope a later commit moved, naming each path once with each commit, and each
write result states the counts its record holds. Each mutation drops one part
of those rules; a test kills each.

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
        '        if paths_moved.len() == 1 {\n            "1 path".to_string()\n        } else {\n            format!("{} paths", paths_moved.len())\n',
        '        if moved.len() == 1 {\n            "1 path".to_string()\n        } else {\n            format!("{} paths", moved.len())\n',
        'primitives::write_review::moved_scope::tests::a_path_two_later_commits_changed_counts_once_naming_both',
    ),
    (
        'M6 the remedy omits the paths the refusal names',
        'src/primitives/write_review/moved_scope.rs',
        'read each path named here and each path \\\n         it adds, or list it in unread',
        'read each path it adds \\\n         or list it in unread',
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
]
