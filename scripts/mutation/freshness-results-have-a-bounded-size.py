"""Mutation list for 066's `freshness-results-have-a-bounded-size`:
`RecordFreshness.paths` is the one list in a freshness answer that grows with
the tree, so `write-review` and `check-review-gate` move it to a file past
`[artifacts] read-size-bytes`, with its count inline in both forms
(`runtime/src/primitives/write_review.rs`,
`runtime/src/primitives/check_review_gate.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/freshness-results-have-a-bounded-size.py
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
        'M1 write-review keeps the paths inline beside the file',
        'src/primitives/write_review.rs',
        '            paths.clear();\n',
        '',
        'write_review::tests::a_stale_analysis_past_the_read_size_names_a_file_holding_every_path',
    ),
    (
        'M2 write-review writes an empty list to the file',
        'src/primitives/write_review.rs',
        '        "analyze-stale-paths": stale_paths,\n',
        '        "analyze-stale-paths": Vec::<String>::new(),\n',
        'write_review::tests::a_stale_analysis_past_the_read_size_names_a_file_holding_every_path',
    ),
    (
        'M3 write-review counts no stale path',
        'src/primitives/write_review.rs',
        '    result.analyze_stale_paths_count = stale_paths.len();',
        '    result.analyze_stale_paths_count = 0;',
        'write_review::tests::a_stale_analysis_past_the_read_size_names_a_file_holding_every_path',
    ),
    (
        'M4 write-review never bounds the paths',
        'src/primitives/write_review.rs',
        '    if let Some(file) = super::bounded_lists::place(repo, "write-review", feature, &result, &lists)?\n    {',
        '    if let Some(file) = None::<String>\n    {',
        'write_review::tests::a_stale_analysis_past_the_read_size_names_a_file_holding_every_path',
    ),
    (
        'M5 check-review-gate keeps the paths inline beside the file',
        'src/primitives/check_review_gate.rs',
        '        result.analyze_stale_paths = None;\n',
        '',
        'check_review_gate::tests::a_stale_analysis_past_the_read_size_names_a_file_holding_every_path',
    ),
    (
        'M6 check-review-gate writes an empty list to the file',
        'src/primitives/check_review_gate.rs',
        '        "analyze-stale-paths": verdict.analyze_stale_paths.as_deref().unwrap_or_default(),',
        '        "analyze-stale-paths": Vec::<String>::new(),',
        'check_review_gate::tests::a_stale_analysis_past_the_read_size_names_a_file_holding_every_path',
    ),
    (
        'M7 check-review-gate counts no stale path',
        'src/primitives/check_review_gate.rs',
        '        analyze_stale_paths_count: stale.len(),',
        '        analyze_stale_paths_count: 0,',
        'check_review_gate::tests::a_stale_analysis_past_the_read_size_names_a_file_holding_every_path',
    ),
    (
        'M8 the stale-analysis block carries no path',
        'src/primitives/check_review_gate.rs',
        '        analyze_stale_paths: Some(paths.clone().into_boxed_slice()),',
        '        analyze_stale_paths: None,',
        'check_review_gate::tests::a_stale_analysis_past_the_read_size_names_a_file_holding_every_path',
    ),
    (
        'M9 write-review reads the read size when no path is stale',
        'src/primitives/write_review.rs',
        '    if stale_paths.is_empty()\n        && result.malformed_departures.is_empty()\n        && result.superseded_deferrals.is_empty()\n    {\n        return Ok(result);\n    }\n',
        '',
        'resolve_constitutions::tests::write_review_and_write_analysis_keep_the_unmeasured_resolution',
    ),
]
