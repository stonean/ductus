"""Mutation list for spec 058's `findings-cost-in-proportion-to-risk`: only
gating findings hold `done`. `check-review-gate`'s last check counts unwaived
review MUST and SHOULD violations and analyze's hard-fail and blocking
findings, never the `dispositions:` map's `undispositioned`
(`runtime/src/primitives/check_review_gate.rs`, 066), and analyze's
`disposition-drift` family is advisory (`runtime/src/primitives/check_artifacts.rs`,
067).

Run with `python3 scripts/mutation/run.py scripts/mutation/findings-cost-in-proportion-to-risk.py
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
        'M1 an undispositioned advisory finding or observation holds done again',
        'src/primitives/check_review_gate.rs',
        '    let owed: Vec<(&str, &str, u32)> = gating.into_iter().filter(|(_, _, n)| *n > 0).collect();',
        '    let _ = gating;\n    let owed: Vec<(&str, &str, u32)> = records.iter().filter_map(|(file, command, map)| map.filter(|counts| counts.undispositioned > 0).map(|counts| (*file, *command, counts.undispositioned))).collect();',
        'check_review_gate::tests::undispositioned_advisory_findings_do_not_hold_done',
    ),
    (
        'M2 an outstanding SHOULD no longer holds done',
        'src/primitives/check_review_gate.rs',
        '                .saturating_add(review.should_violations),',
        '                .saturating_add(0),',
        'check_review_gate::tests::an_outstanding_should_holds_done',
    ),
    (
        'M3 disposition drift blocks a done spec again',
        'src/primitives/check_artifacts.rs',
        '        family: "disposition-drift".into(),\n        severity: AnalyzeSeverity::Advisory,',
        '        family: "disposition-drift".into(),\n        severity: AnalyzeSeverity::Blocking,',
        'check_artifacts::tests::a_done_spec_whose_review_owes_decisions_is_disposition_drift',
    ),
    (
        'M4 a MUST beside a false blocking flag no longer holds done',
        'src/primitives/check_review_gate.rs',
        '            review\n                .must_violations\n                .saturating_add(review.should_violations),',
        '            0u32.saturating_add(review.should_violations),',
        'check_review_gate::tests::a_gating_count_beside_a_false_blocking_flag_still_holds_done',
    ),
    (
        'M5 an analysis whose blocking flag disagrees with its counts no longer holds done',
        'src/primitives/check_review_gate.rs',
        '                record.hard_fail.saturating_add(record.blocking_findings)',
        '                let _ = record;\n                0',
        'check_review_gate::tests::a_gating_count_beside_a_false_blocking_flag_still_holds_done',
    ),
]
