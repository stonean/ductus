"""Mutation list for 066's `review-scope-reads-every-cell-path-and-sets-deleted-files-apart`:
every backticked path in a plan's Affected Files cell joins `plan-affected`
(`parse_affected_files`, `runtime/src/primitives/mod.rs`), and a path deleted
since the diff base stays in `modified-since`, is named in `deleted` and
leaves `scope` (`runtime/src/primitives/compute_review_scope.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/review-scope-cells-and-deletions.py
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
        'M1 a cell yields its first path only again',
        'src/primitives/mod.rs',
        '                .chain(spans.filter(|span| is_cell_path(span)))',
        '                .chain(spans.filter(|_| false))',
        'primitives::tests::parse_affected_files_takes_every_path_a_cell_names compute_review_scope::tests::a_two_path_cell_and_a_deleted_file_resolve_as_the_scenario_says',
    ),
    (
        'M2 a command, a flag or an anchor in a cell joins the scope',
        'src/primitives/mod.rs',
        "    !span.is_empty() && !span.chars().any(char::is_whitespace) && !span.starts_with(['-', '/', '§'])",
        '    !span.is_empty()',
        'primitives::tests::parse_affected_files_takes_every_path_a_cell_names',
    ),
    (
        'M3 a deleted path stays in scope',
        'src/primitives/compute_review_scope.rs',
        '        .filter(|path| !deleted.contains(*path))',
        '        .filter(|_| true)',
        'compute_review_scope::tests::a_two_path_cell_and_a_deleted_file_resolve_as_the_scenario_says',
    ),
    (
        'M4 a changed file present on disk is called deleted',
        'src/primitives/compute_review_scope.rs',
        '        .filter(|path| std::fs::symlink_metadata(repo.join(path)).is_err())',
        '        .filter(|_| true)',
        'compute_review_scope::tests::a_two_path_cell_and_a_deleted_file_resolve_as_the_scenario_says',
    ),
    (
        'M5 the file form keeps deleted inline',
        'src/primitives/compute_review_scope.rs',
        '        result.deleted = None;\n',
        '',
        'compute_review_scope::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M6 the list file omits deleted',
        'src/primitives/compute_review_scope.rs',
        '        "deleted": scope.deleted,\n',
        '',
        'compute_review_scope::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M7 the result counts no deleted path',
        'src/primitives/compute_review_scope.rs',
        '        deleted_count: scope.deleted.len(),',
        '        deleted_count: 0,',
        'compute_review_scope::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M8 the list file names no deleted path',
        'src/primitives/compute_review_scope.rs',
        '        "deleted": scope.deleted,',
        '        "deleted": Vec::<String>::new(),',
        'compute_review_scope::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M9 the inline result lists no deleted path',
        'src/primitives/compute_review_scope.rs',
        '        deleted: Some(scope.deleted.clone()),',
        '        deleted: Some(Vec::new()),',
        'compute_review_scope::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
]
