"""Mutation list for 073's task 13: `write-review`'s `malformed-departures`
and `superseded-deferrals` are inline while the result fits `[artifacts]
read-size-bytes`, and past it leave the result for the one file `lists-file`
names, beside `analyze-stale-paths`, with each count inline in both forms
(BE-PAYLOAD-001; `runtime/src/primitives/write_review.rs`, through
`bounded_lists.rs`, whose own mutations are in `scope-results-bounded.py`;
the stale-paths list's are in `freshness-results-have-a-bounded-size.py`).

Run with `python3 scripts/mutation/run.py scripts/mutation/review-departures-bounded.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

run.py runs unit tests only. The integration test the bound also answers to,
`a_review_result_past_the_read_size_reaches_the_host_through_its_list_file_via_mcp`
in `runtime/tests/mcp.rs`, fails under M1, run by hand with
`cargo test --locked --test mcp -- a_review_result_past`.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the bound removed: every write-review result keeps its departure lists inline',
        'src/primitives/write_review.rs',
        '    if let Some(file) = super::bounded_lists::place(repo, "write-review", feature, &result, &lists)?\n    {',
        '    if let Some(file) = None::<String>\n    {',
        'departure_waivers::the_departure_lists_past_the_read_size',
    ),
    (
        'M2 malformed-departures stays inline in the file form',
        'src/primitives/write_review.rs',
        '        result.malformed_departures.clear();\n',
        '',
        'departure_waivers::the_departure_lists_past_the_read_size',
    ),
    (
        'M3 superseded-deferrals stays inline in the file form',
        'src/primitives/write_review.rs',
        '        result.superseded_deferrals.clear();\n',
        '',
        'departure_waivers::the_departure_lists_past_the_read_size',
    ),
    (
        'M4 the list file drops malformed-departures',
        'src/primitives/write_review.rs',
        '        "malformed-departures": result.malformed_departures,\n',
        '        "malformed-departures": Vec::<String>::new(),\n',
        'departure_waivers::the_departure_lists_past_the_read_size',
    ),
    (
        'M5 the list file drops superseded-deferrals',
        'src/primitives/write_review.rs',
        '        "superseded-deferrals": result.superseded_deferrals,\n',
        '        "superseded-deferrals": Vec::<String>::new(),\n',
        'departure_waivers::the_departure_lists_past_the_read_size',
    ),
    (
        'M6 a file-form result counts no malformed entry',
        'src/primitives/write_review.rs',
        '    result.malformed_departures_count = result.malformed_departures.len();\n',
        '    result.malformed_departures_count = 0;\n',
        'departure_waivers::the_departure_lists_past_the_read_size',
    ),
    (
        'M7 a file-form result counts no superseded deferral',
        'src/primitives/write_review.rs',
        '    result.superseded_deferrals_count = result.superseded_deferrals.len();\n',
        '    result.superseded_deferrals_count = 0;\n',
        'departure_waivers::the_departure_lists_past_the_read_size',
    ),
    (
        'M8 a result whose only list is malformed-departures is never bounded',
        'src/primitives/write_review.rs',
        '    if stale_paths.is_empty()\n        && result.malformed_departures.is_empty()\n',
        '    if stale_paths.is_empty()\n',
        'departure_waivers::the_departure_lists_past_the_read_size',
    ),
    (
        'M9 a result whose only list is superseded-deferrals is never bounded',
        'src/primitives/write_review.rs',
        '        && result.superseded_deferrals.is_empty()\n    {\n        return Ok(result);',
        '    {\n        return Ok(result);',
        'departure_waivers::the_departure_lists_past_the_read_size',
    ),
    (
        'M10 the bound measures the lists alone, not the whole result',
        'src/primitives/write_review.rs',
        '"write-review", feature, &result, &lists)?',
        '"write-review", feature, &lists, &lists)?',
        'departure_waivers::the_departure_lists_past_the_read_size',
    ),
]
