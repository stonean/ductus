"""Mutation list for 073's task 12: a `check-departures` result is inline
while it fits `[artifacts] read-size-bytes`, and past it names a file holding
its three lists whole, with every count inline (BE-PAYLOAD-001;
`runtime/src/primitives/check_departures.rs`, through `bounded_lists.rs`,
whose own mutations are in `scope-results-bounded.py`).

Run with `python3 scripts/mutation/run.py scripts/mutation/departures-bounded.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

run.py runs unit tests only. The integration test the bound also answers to,
`a_departures_result_past_the_read_size_reaches_the_host_through_its_list_file_via_mcp`
in `runtime/tests/mcp.rs`, fails under M1, run by hand with
`cargo test --locked --test mcp -- departures_result_past`. The exec walker
restoring the file form before `AnalyzeTally` reads it is `exec-departures.py`'s
M5.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the bound removed: every check-departures result stays inline',
        'src/primitives/check_departures.rs',
        '        super::bounded_lists::place(repo, "check-departures", feature, &result, &lists)?\n',
        '        super::bounded_lists::place(repo, "check-departures", feature, &result, &lists)?.filter(|_| false)\n',
        'check_departures::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M2 the lists stay inline in the file form',
        'src/primitives/check_departures.rs',
        '        result.entries = None;\n        result.findings = None;\n        result.unexamined = None;\n',
        '',
        'check_departures::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M3 the list file drops the entries the host waives rules from',
        'src/primitives/check_departures.rs',
        '        "entries": entries,\n',
        '        "entries": Vec::<DepartureEntry>::new(),\n',
        'check_departures::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M4 the list file drops the findings the tally counts',
        'src/primitives/check_departures.rs',
        '        "findings": findings,\n',
        '        "findings": Vec::<DepartureFinding>::new(),\n',
        'check_departures::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M5 the list file drops the unexamined targets',
        'src/primitives/check_departures.rs',
        '        "unexamined": unexamined,\n',
        '        "unexamined": Vec::<DepartureUnexamined>::new(),\n',
        'check_departures::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
    (
        'M6 a file-form result reads as having no findings',
        'src/primitives/check_departures.rs',
        '        findings_count: findings.len(),\n',
        '        findings_count: 0,\n',
        'check_departures::tests::a_result_past_the_read_size_names_a_file_holding_every_list',
    ),
]
