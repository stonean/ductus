"""Mutation list for 073's task 14: `process-waivers`' `applied`, `expired`,
`retained` and `notices` are inline while the result fits `[artifacts]
read-size-bytes`, and past it leave the result for the one file `lists-file`
names, with each count inline in both forms (BE-PAYLOAD-001;
`runtime/src/primitives/process_waivers.rs`, through `bounded_lists.rs`, whose
own mutations are in `scope-results-bounded.py`).

Run with `python3 scripts/mutation/run.py scripts/mutation/waivers-bounded.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

run.py runs unit tests only. Two integration tests answer to the bound as
well, run by hand: `process_waivers_past_the_read_size_reaches_the_host_through_its_list_file_via_mcp`
in `runtime/tests/mcp.rs` fails under M1 (`cargo test --locked --test mcp --
process_waivers_past`), and `file_form_process_waivers_reaches_write_review_whole`
in `runtime/tests/walker.rs`, the exec walker's proof that `write-review`
reads the file form whole, fails under M1 (it expects the list file) and with
`restore_lists` removed from the walker (`cargo test --locked --test walker --
file_form_process`).
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the bound removed: every process-waivers result keeps its lists inline',
        'src/primitives/process_waivers.rs',
        '    if let Some(file) =\n        bounded_lists::place(repo, "process-waivers", &args.feature, &result, &lists)?\n    {',
        '    if let Some(file) = None::<String>\n    {',
        'process_waivers::tests::the_lists_past_the_read_size',
    ),
    (
        'M2 applied stays inline in the file form',
        'src/primitives/process_waivers.rs',
        '        result.applied = None;\n',
        '',
        'process_waivers::tests::the_lists_past_the_read_size',
    ),
    (
        'M3 expired stays inline in the file form',
        'src/primitives/process_waivers.rs',
        '        result.expired = None;\n',
        '',
        'process_waivers::tests::the_lists_past_the_read_size',
    ),
    (
        'M4 retained stays inline in the file form',
        'src/primitives/process_waivers.rs',
        '        result.retained = None;\n',
        '',
        'process_waivers::tests::the_lists_past_the_read_size',
    ),
    (
        'M5 notices stay inline in the file form',
        'src/primitives/process_waivers.rs',
        '        result.notices = None;\n',
        '',
        'process_waivers::tests::the_lists_past_the_read_size',
    ),
    (
        'M6 the list file drops applied',
        'src/primitives/process_waivers.rs',
        '        "applied": &classified.applied,\n',
        '        "applied": Vec::<WaiverRef>::new(),\n',
        'process_waivers::tests::the_lists_past_the_read_size',
    ),
    (
        'M7 the list file drops expired',
        'src/primitives/process_waivers.rs',
        '        "expired": &classified.expired,\n',
        '        "expired": Vec::<WaiverRef>::new(),\n',
        'process_waivers::tests::the_lists_past_the_read_size',
    ),
    (
        'M8 the list file drops retained',
        'src/primitives/process_waivers.rs',
        '        "retained": &classified.retained,\n',
        '        "retained": Vec::<WaiverRef>::new(),\n',
        'process_waivers::tests::the_lists_past_the_read_size',
    ),
    (
        'M9 the list file drops notices',
        'src/primitives/process_waivers.rs',
        '        "notices": &classified.notices,\n',
        '        "notices": Vec::<String>::new(),\n',
        'process_waivers::tests::the_lists_past_the_read_size',
    ),
    (
        'M10 a file-form result counts no applied anchor',
        'src/primitives/process_waivers.rs',
        '        applied_count: classified.applied.len(),\n',
        '        applied_count: 0,\n',
        'process_waivers::tests::the_lists_past_the_read_size',
    ),
    (
        'M11 a file-form result counts no expired anchor',
        'src/primitives/process_waivers.rs',
        '        expired_count: classified.expired.len(),\n',
        '        expired_count: 0,\n',
        'process_waivers::tests::the_lists_past_the_read_size',
    ),
    (
        'M12 a file-form result counts no retained anchor',
        'src/primitives/process_waivers.rs',
        '        retained_count: classified.retained.len(),\n',
        '        retained_count: 0,\n',
        'process_waivers::tests::the_lists_past_the_read_size',
    ),
    (
        'M13 a file-form result counts no notice',
        'src/primitives/process_waivers.rs',
        '        notices_count: classified.notices.len(),\n',
        '        notices_count: 0,\n',
        'process_waivers::tests::the_lists_past_the_read_size',
    ),
    (
        'M14 a result with every list empty is bounded too, and reads the setting',
        'src/primitives/process_waivers.rs',
        '    if nothing {\n        return Ok(result);\n    }\n',
        '',
        'process_waivers::tests::the_lists_past_the_read_size',
    ),
    (
        'M15 a result whose only list is applied is never bounded',
        'src/primitives/process_waivers.rs',
        '    let nothing = classified.applied.is_empty()\n        && classified.expired.is_empty()',
        '    let nothing = classified.expired.is_empty()',
        'process_waivers::tests::the_lists_past_the_read_size',
    ),
    (
        'M16 a result whose only list is notices is never bounded',
        'src/primitives/process_waivers.rs',
        '        && classified.retained.is_empty()\n        && classified.notices.is_empty();',
        '        && classified.retained.is_empty();',
        'process_waivers::tests::the_lists_past_the_read_size',
    ),
    (
        'M17 the bound measures the lists alone, not the whole result',
        'src/primitives/process_waivers.rs',
        '&args.feature, &result, &lists)?',
        '&args.feature, &lists, &lists)?',
        'process_waivers::tests::the_lists_past_the_read_size',
    ),
]
