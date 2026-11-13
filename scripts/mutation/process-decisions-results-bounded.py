"""Mutation list for 066's `process-decisions-results-have-a-bounded-size`:
a `process-decisions` result is inline while it fits `[artifacts]
read-size-bytes`, and past it names a file holding its five lists whole, with
every count inline (`runtime/src/primitives/process_decisions.rs`, through
`bounded_lists.rs`, whose own mutations are in `scope-results-bounded.py`).

Run with `python3 scripts/mutation/run.py scripts/mutation/process-decisions-results-bounded.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

run.py runs unit tests only. The integration test the bound also answers to,
`a_decisions_result_past_the_read_size_reaches_the_host_through_its_list_file_via_mcp`
in `runtime/tests/mcp.rs`, fails under M1, run by hand with
`cargo test --locked --test mcp -- decisions_result_past`.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the bound removed: every process-decisions result stays inline',
        'src/primitives/process_decisions.rs',
        '        super::bounded_lists::place(repo, "process-decisions", &args.feature, &result, &lists)?\n',
        '        super::bounded_lists::place(repo, "process-decisions", &args.feature, &result, &lists)?.filter(|_| false)\n',
        'process_decisions::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M2 the lists stay inline in the file form',
        'src/primitives/process_decisions.rs',
        '        result.matched = None;\n        result.expired = None;\n        result.retained = None;\n        result.lapsed = None;\n        result.notices = None;\n',
        '',
        'process_decisions::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M3 the list file drops the expired decisions the writer prunes',
        'src/primitives/process_decisions.rs',
        '        "expired": &classified.expired,\n',
        '        "expired": Vec::<DecisionRef>::new(),\n',
        'process_decisions::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M4 the list file drops the lapsed deferrals the host asks about again',
        'src/primitives/process_decisions.rs',
        '        "lapsed": &classified.lapsed,\n',
        '        "lapsed": Vec::<DecisionRef>::new(),\n',
        'process_decisions::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M5 the list file drops the notices the host emits',
        'src/primitives/process_decisions.rs',
        '        "notices": &notices,\n',
        '        "notices": Vec::<String>::new(),\n',
        'process_decisions::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M6 a file-form result reads as having nothing expired',
        'src/primitives/process_decisions.rs',
        '        expired_count: classified.expired.len(),\n',
        '        expired_count: 0,\n',
        'process_decisions::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M7 a file-form result reads as having nothing lapsed',
        'src/primitives/process_decisions.rs',
        '        lapsed_count: classified.lapsed.len(),\n',
        '        lapsed_count: 0,\n',
        'process_decisions::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
]
