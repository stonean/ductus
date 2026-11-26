"""Mutation list for spec 088: the counts `write-analysis` records of the
rules its run judged and carried (`runtime/src/primitives/write_analysis.rs`,
task 1). Each behavior is mutated once and killed by the test that names it.

Run with `python3 scripts/mutation/run.py scripts/mutation/rule-judgments-088.py
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
        'M1 a rule the run judged is not counted judged',
        'src/primitives/write_analysis.rs',
        '            fields.judged += 1;\n',
        '',
        'primitives::write_analysis::tests::a_run_carrying_one_verdict_and_judging_one_rule_records_both_counts',
    ),
    (
        'M2 a rule whose verdict stood is not counted carried',
        'src/primitives/write_analysis.rs',
        '                fields.carried += 1;\n',
        '',
        'primitives::write_analysis::tests::a_run_carrying_one_verdict_and_judging_one_rule_records_both_counts',
    ),
    (
        'M3 a judged rule left unjudged counts as judged',
        'src/primitives/write_analysis.rs',
        '        if fields.unjudged.len() == unjudged_before {\n            fields.judged += 1;',
        '        if true {\n            fields.judged += 1;',
        'primitives::write_analysis::tests::an_unresolved_addressed_verdict_counts_in_neither',
    ),
    (
        'M4 a rule left unjudged without a judgment counts as carried',
        'src/primitives/write_analysis.rs',
        '            if fields.unjudged.len() == unjudged_before {\n                fields.carried += 1;',
        '            if true {\n                fields.carried += 1;',
        'primitives::write_analysis::tests::the_counts_and_the_unjudged_rules_partition_a_mixed_run',
    ),
    (
        'M5 the judged count is written only when non-zero',
        'src/primitives/write_analysis.rs',
        '    let _ = writeln!(out, "rules-judged: {}", rules.judged);\n',
        '    if rules.judged > 0 {\n        let _ = writeln!(out, "rules-judged: {}", rules.judged);\n    }\n',
        'primitives::write_analysis::tests::a_carried_quiet_rule_counts_as_carried_and_no_rule_loaded_records_zero',
    ),
]
