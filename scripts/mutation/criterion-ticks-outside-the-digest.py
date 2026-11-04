"""Mutation list for 066's `checkbox-state-is-outside-the-analysis-digest`:
`spec.md`'s analyze digest normalizes each acceptance criterion's checkbox,
and only that, so a tick leaves the analysis current while every other edit
stales it (`runtime/src/primitives/analyze_subjects.rs`,
`without_criterion_ticks`).

Run with `python3 scripts/mutation/run.py scripts/mutation/criterion-ticks-outside-the-digest.py
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
        'M1 a criterion tick stales the analysis again',
        'src/primitives/analyze_subjects.rs',
        '                    hasher.update(without_criterion_ticks(&text).as_bytes());',
        '                    hasher.update(text.as_bytes());',
        'analyze_subjects::tests::a_criterion_tick_does_not_move_the_spec_digest check_review_gate::tests::a_criterion_tick_leaves_the_analysis_current_and_another_spec_edit_stales_it',
    ),
    (
        'M2 every checkbox in spec.md is normalized, not only a criterion\'s',
        'src/primitives/analyze_subjects.rs',
        '        crate::primitives::mark_criterion::collect_checkbox_line_indices(&lines)\n            .into_iter()\n            .collect();',
        '        lines.iter().enumerate().filter_map(|(idx, line)| crate::primitives::checkbox::find_checkbox_line(line).map(|(_, marker)| (idx, marker))).collect();',
        'analyze_subjects::tests::every_other_spec_edit_still_moves_its_digest',
    ),
    (
        'M3 a criterion\'s text leaves the digest with its tick',
        'src/primitives/analyze_subjects.rs',
        '            Some(&marker) => crate::primitives::checkbox::flip_checkbox_at(line, marker, false).1,',
        '            Some(_) => String::new(),',
        'analyze_subjects::tests::every_other_spec_edit_still_moves_its_digest check_review_gate::tests::a_criterion_tick_leaves_the_analysis_current_and_another_spec_edit_stales_it',
    ),
]
