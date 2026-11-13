"""Mutation list for 066's task 22: `process-decisions` keeps retained
decisions, which ask nothing of the host, out of its inline result. Whenever
one is retained, the file `lists-file` names holds the `retained` list and
each one's own notice (`retained-notices`), and `notices` carries one line per
reason instead, the scope that could not be read on its own line
(`runtime/src/primitives/process_decisions.rs`, through `bounded_lists.rs`,
whose own mutations are in `scope-results-bounded.py`).

Run with `python3 scripts/mutation/run.py scripts/mutation/process-decisions-retained.py
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
        'M1 retained decisions stay inline',
        'src/primitives/process_decisions.rs',
        '        retained: (!any_retained).then(Vec::new),\n',
        '        retained: Some(classified.retained.clone()),\n',
        'process_decisions::tests::retained_decisions_reach_the_file_and_not_the_result',
    ),
    (
        'M2 no file is written for a result that fits',
        'src/primitives/process_decisions.rs',
        '    } else if any_retained {\n',
        '    } else if false {\n',
        'process_decisions::tests::retained_decisions_reach_the_file_and_not_the_result',
    ),
    (
        'M3 each retained decision keeps its own notice inline',
        'src/primitives/process_decisions.rs',
        '        .partition(|(index, _)| retained_at.binary_search(index).is_ok());\n',
        '        .partition(|_| false);\n',
        'process_decisions::tests::retained_decisions_reach_the_file_and_not_the_result',
    ),
    (
        'M4 no summary line says why decisions were retained',
        'src/primitives/process_decisions.rs',
        '        if count > 0 {\n            notices.push(class.summary(count));\n',
        '        if false {\n            notices.push(class.summary(count));\n',
        'process_decisions::tests::retained_decisions_reach_the_file_and_not_the_result',
    ),
    (
        'M5 a scope that could not be read is summarized as a file not read',
        'src/primitives/process_decisions.rs',
        '            let class = if matches!(scope.get(), Some(RunScope::Unreadable)) {\n',
        '            let class = if false {\n',
        'process_decisions::tests::a_scope_that_cannot_be_read_has_its_own_summary_line',
    ),
    (
        'M6 a run that retains nothing writes a file anyway',
        'src/primitives/process_decisions.rs',
        '    } else if any_retained {\n',
        '    } else if true {\n',
        'process_decisions::tests::a_run_that_retains_nothing_writes_no_file',
    ),
]
