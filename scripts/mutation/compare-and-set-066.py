"""Mutation list for spec 066 task 24 (`BE-TXN-002`): `write-review` and
`write-analysis` write their record through the shared compare-before-persist
guard, `ReadSnapshot` in `runtime/src/primitives/mod.rs`, taken before the
record's first read. Each mutation writes without the guard, or takes the
snapshot too late to guard anything; one test per primitive kills each.

Run with `python3 scripts/mutation/run.py scripts/mutation/compare-and-set-066.py
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
        'M1 write-review renames its record over one written since it read it',
        'src/primitives/write_review.rs',
        '    write_record(&record, &report, between)?;\n',
        '    between();\n    let _ = &record;\n    crate::primitives::write_atomic(&review_path, &report)?;\n',
        'primitives::write_review::tests::a_record_written_after_it_was_read_is_refused_and_kept',
    ),
    (
        'M2 write-review takes its snapshot only when it writes',
        'src/primitives/write_review.rs',
        '    write_record(&record, &report, between)?;\n',
        '    between();\n    let _ = &record;\n    super::ReadSnapshot::read(repo, &subject.record_rel(&root))?.write_if_unchanged(&report)?;\n',
        'primitives::write_review::tests::a_record_written_after_it_was_read_is_refused_and_kept',
    ),
    (
        'M3 write-review refuses a record that did not change',
        'src/primitives/write_review.rs',
        '    write_record(&record, &report, between)?;\n',
        '    between();\n    let _ = &record;\n    return Err(PrimitiveError::ChangedSinceRead { path: review_path.clone() });\n',
        'primitives::write_review::tests::a_record_unchanged_since_it_was_read_is_written',
    ),
    (
        'M4 write-analysis renames its record over one written since it read it',
        'src/primitives/write_analysis.rs',
        '    between();\n    record.write_if_unchanged(&report)?;\n',
        '    between();\n    let _ = &record;\n    crate::primitives::write_atomic(&analysis_path, &report)?;\n',
        'primitives::write_analysis::tests::a_record_written_after_it_was_read_is_refused_and_kept',
    ),
    (
        'M5 write-analysis takes its snapshot only when it writes',
        'src/primitives/write_analysis.rs',
        '    between();\n    record.write_if_unchanged(&report)?;\n',
        '    between();\n    let _ = &record;\n    super::ReadSnapshot::read(repo, &format!("{root}/{}/{}", args.feature, super::ANALYSIS_RECORD_FILE))?.write_if_unchanged(&report)?;\n',
        'primitives::write_analysis::tests::a_record_written_after_it_was_read_is_refused_and_kept',
    ),
    (
        'M6 write-analysis refuses a record that did not change',
        'src/primitives/write_analysis.rs',
        '    between();\n    record.write_if_unchanged(&report)?;\n',
        '    between();\n    let _ = &record;\n    return Err(PrimitiveError::ChangedSinceRead { path: analysis_path.clone() });\n',
        'primitives::write_analysis::tests::a_record_unchanged_since_it_was_read_is_written',
    ),
    (
        'M7 write-review takes a second snapshot just before the write, after the point a test changes the record',
        'src/primitives/write_review.rs',
        '    write_record(&record, &report, between)?;\n',
        '    let record = record_snapshot(repo, &root, subject)?;\n    write_record(&record, &report, between)?;\n',
        'primitives::write_review::tests::a_record_written_just_after_the_snapshot_is_refused_and_kept',
    ),
    (
        'M8 write-analysis takes a second snapshot just before the write, after the point a test changes the record',
        'src/primitives/write_analysis.rs',
        '    between();\n    record.write_if_unchanged(&report)?;\n',
        '    let record = super::ReadSnapshot::read(\n        repo,\n        &format!("{root}/{}/{}", args.feature, super::ANALYSIS_RECORD_FILE),\n    )?;\n    between();\n    record.write_if_unchanged(&report)?;\n',
        'primitives::write_analysis::tests::a_record_written_just_after_the_snapshot_is_refused_and_kept',
    ),
]
