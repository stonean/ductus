"""Mutation list for spec 058's §Decisions persist across runs on the analyze
record: a rule-decided waiver sent for a finding a person already decided
carries the person's stored decision, so `write-analysis` counts and renders
what `decisions:` keeps, unless the run expired that decision
(`runtime/src/primitives/write_analysis.rs`, as `write-review`'s
`standing_decision` does for observations). Found by 058's review
(2026-11-10): the counts and the tier render followed the rule while the
record kept the person's decision.

Run with `python3 scripts/mutation/run.py scripts/mutation/analysis-standing-decision.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

The killing test is `a_stored_decision_by_a_person_wins_over_the_rule` in
runtime/src/primitives/write_analysis.rs.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the rule outcome is counted and rendered over the person\'s',
        'src/primitives/write_analysis.rs',
        '    let settled = settled(args, &feature_dir)?;\n    let args = settled.as_ref();\n',
        '    let _ = settled(args, &feature_dir)?;\n',
        'write_analysis::tests::a_stored_decision_by_a_person_wins_over_the_rule',
    ),
    (
        'M2 a decision this run expired still wins over the rule',
        'src/primitives/write_analysis.rs',
        '    if expired\n        .iter()\n        .any(|gone| decisions::same_key(&gone.key, &key))\n    {\n        return None;\n    }\n    let decision = decisions::stored_decision(stored, &key)?;\n',
        '    let _ = expired;\n    let decision = decisions::stored_decision(stored, &key)?;\n',
        'write_analysis::tests::a_stored_decision_by_a_person_wins_over_the_rule',
    ),
]
