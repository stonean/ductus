"""Mutation list for 084's measure of the bar (AC16): `measure-reopenings`'
`held-owed-work` and `never-owed-count`
(`runtime/src/primitives/measure_reopenings.rs`), and the count's row in the
bounded result's table (`runtime/src/primitives/bounded_lists.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/never-owed-084.py
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
        'M1 a reopening is judged by its oldest version alone',
        'src/primitives/measure_reopenings.rs',
        '            pass.held_owed = pass.held_owed || history.owes_work(*directory)?;\n',
        '            pass.held_owed = history.owes_work(*directory)?;\n',
        'measure_reopenings::tests::a_box_open_at_a_middle_commit_is_owed_work_though_ticked_by_the_end',
    ),
    (
        'M2 the walk reads no owed work',
        'src/primitives/measure_reopenings.rs',
        '            pass.held_owed = pass.held_owed || history.owes_work(*directory)?;\n',
        '',
        'measure_reopenings::tests::a_box_open_at_a_middle_commit_is_owed_work_though_ticked_by_the_end',
    ),
    (
        'M3 an unchecked box is no owed work',
        'src/primitives/measure_reopenings.rs',
        '            |text| !unchecked_boxes(text).is_empty(),\n',
        '            |_| false,\n',
        'measure_reopenings::tests::a_box_open_at_a_middle_commit_is_owed_work_though_ticked_by_the_end',
    ),
    (
        'M4 an unticked criterion is no owed work',
        'src/primitives/measure_reopenings.rs',
        '                    .is_ok_and(|(_, body)| !unticked_criteria_in(body).is_empty())\n',
        '                    .is_ok_and(|_| false)\n',
        'measure_reopenings::tests::a_criterion_unticked_at_a_middle_commit_is_owed_work_though_ticked_by_the_end',
    ),
    (
        'M5 a spec whose frontmatter does not split is read whole',
        'src/primitives/measure_reopenings.rs',
        [
            '                split_frontmatter(text, Path::new(SPEC_FILE))\n',
            '                    .is_ok_and(|(_, body)| !unticked_criteria_in(body).is_empty())\n',
        ],
        [
            '                !unticked_criteria_in(text).is_empty()\n',
            '',
        ],
        'measure_reopenings::tests::a_file_that_cannot_be_read_shows_no_work',
    ),
    (
        'M6 a file that is not UTF-8 is read lossily',
        'src/primitives/measure_reopenings.rs',
        '    let holds = std::str::from_utf8(blob.content()).is_ok_and(test);\n',
        '    let holds = test(&String::from_utf8_lossy(blob.content()));\n',
        'measure_reopenings::tests::a_file_that_cannot_be_read_shows_no_work',
    ),
    (
        'M7 a cached answer is inverted',
        'src/primitives/measure_reopenings.rs',
        '        return Ok(*cached);\n',
        '        return Ok(!*cached);\n',
        'measure_reopenings::tests::a_reopening_never_owing_work_is_counted_and_one_not_counted_is_not',
    ),
    (
        'M8 the reopening drops what its pass held',
        'src/primitives/measure_reopenings.rs',
        '            held_owed_work: self.held_owed,\n',
        '            held_owed_work: false,\n',
        'measure_reopenings::tests::a_box_open_at_a_middle_commit_is_owed_work_though_ticked_by_the_end',
    ),
    (
        'M9 the count takes the reopenings that held owed work',
        'src/primitives/measure_reopenings.rs',
        '            never_owed_count: self.reopenings.iter().filter(|r| !r.held_owed_work).count(),\n',
        '            never_owed_count: self.reopenings.iter().filter(|r| r.held_owed_work).count(),\n',
        'measure_reopenings::tests::a_reopening_never_owing_work_is_counted_and_one_not_counted_is_not',
    ),
    (
        'M10 a reopening not counted is counted never owed',
        'src/primitives/measure_reopenings.rs',
        '            never_owed_count: self.reopenings.iter().filter(|r| !r.held_owed_work).count(),\n',
        '            never_owed_count: self.reopenings.iter().filter(|r| !r.held_owed_work).count()\n'
        '                + self.not_counted.len(),\n',
        'measure_reopenings::tests::a_reopening_never_owing_work_is_counted_and_one_not_counted_is_not',
    ),
    (
        'M11 the result drops the count',
        'src/primitives/measure_reopenings.rs',
        '        never_owed_count: counts.never_owed_count,\n',
        '        never_owed_count: 0,\n',
        'measure_reopenings::tests::a_result_within_the_read_size_stays_inline',
    ),
    (
        'M12 never-owed-count names a list, halting a whole file',
        'src/primitives/bounded_lists.rs',
        '    ("measure-reopenings", "never-owed-count", None),\n',
        '',
        'measure_reopenings::tests::a_result_past_the_read_size_moves_its_lists_to_a_file',
    ),
]
