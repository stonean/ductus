"""Mutation list for 078's `fix-round-results-have-a-bounded-size`: a
`brief-re-read`, `read-re-read` or `check-prescriptions` result is inline
while it fits `[artifacts] read-size-bytes`, and past it names a file
holding its lists, with every count inline
(`runtime/src/primitives/brief_re_read.rs`, `read_re_read.rs`,
`check_prescriptions.rs`, each one's `bounded`).

Run with `python3 scripts/mutation/run.py scripts/mutation/fix-round-results-bounded.py
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
        'M1 a round past the read size keeps its paths inline',
        'src/primitives/brief_re_read.rs',
        '        super::bounded_lists::place(repo, "brief-re-read", feature, &result, &lists)?',
        '        None::<String>',
        'brief_re_read::tests::a_round_past_the_read_size_names_a_file_holding_its_paths',
    ),
    (
        'M2 a round names its list file but keeps its paths inline',
        'src/primitives/brief_re_read.rs',
        '        result.paths = None;\n        result.lists_file = Some(file);',
        '        result.lists_file = Some(file);',
        'brief_re_read::tests::a_round_past_the_read_size_names_a_file_holding_its_paths',
    ),
    (
        'M3 a round counts no paths',
        'src/primitives/brief_re_read.rs',
        '    result.paths_count = paths.len();',
        '    result.paths_count = 0;',
        'brief_re_read::tests::a_round_past_the_read_size_names_a_file_holding_its_paths brief_re_read::tests::a_spec_body_edit_with_code_needs_a_re_read',
    ),
    (
        'M4 a re-read past the read size keeps its findings inline',
        'src/primitives/read_re_read.rs',
        '    if let Some(file) = super::bounded_lists::place(repo, "read-re-read", feature, &result, &lists)?',
        '    if let Some(file) = None::<String>',
        'read_re_read::tests::findings_past_the_read_size_are_in_a_file',
    ),
    (
        'M5 a re-read counts no findings',
        'src/primitives/read_re_read.rs',
        '    result.findings_count = findings.len();',
        '    result.findings_count = 0;',
        'read_re_read::tests::findings_past_the_read_size_are_in_a_file read_re_read::tests::a_complete_file_is_counted',
    ),
    (
        'M6 a check past the read size keeps its lists inline',
        'src/primitives/check_prescriptions.rs',
        '        super::bounded_lists::place(repo, "check-prescriptions", feature, &result, &lists)?',
        '        None::<String>',
        'check_prescriptions::tests::lists_past_the_read_size_are_in_a_file',
    ),
    (
        'M7 a check counts no unprescribed files',
        'src/primitives/check_prescriptions.rs',
        '    result.unprescribed_count = unprescribed.len();',
        '    result.unprescribed_count = 0;',
        'check_prescriptions::tests::lists_past_the_read_size_are_in_a_file',
    ),
]
