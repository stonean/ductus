"""Mutation list for 084's status count (AC9): `dashboard`'s `unreviewed-code`
block (`runtime/src/primitives/dashboard.rs`), the counts it carries
(`check_unreviewed_code::counts`), and the view's `Unreviewed code:` line
(`runtime/src/primitives/pipeline_view.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/status-count-084.py
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
        'M1 the dashboard carries no check',
        'src/primitives/dashboard.rs',
        '        unreviewed_code: super::check_unreviewed_code::counts(repo),\n',
        '        unreviewed_code: crate::schema::primitives::UnreviewedCodeCounts::default(),\n',
        'dashboard::tests::the_unreviewed_code_block_is_the_check_at_head_in_each_state',
    ),
    (
        'M2 a check that failed reads as no code owed',
        'src/primitives/check_unreviewed_code.rs',
        '        Err(error) => UnreviewedCodeCounts {\n            state: UnreviewedState::NotExamined,',
        '        Err(error) => UnreviewedCodeCounts {\n            state: UnreviewedState::None,',
        'dashboard::tests::the_unreviewed_code_block_is_the_check_at_head_in_each_state',
    ),
    (
        'M3 a subject not examined carries no reason',
        'src/primitives/check_unreviewed_code.rs',
        '                reason: reason.map(|text| super::flatten_line(&text)),\n',
        '                reason: reason.map(|_| String::new()),\n',
        'dashboard::tests::the_unreviewed_code_block_is_the_check_at_head_in_each_state',
    ),
    (
        'M4 the view leaves the line out',
        'src/primitives/pipeline_view.rs',
        '    items.push(whole(unreviewed_line(data.unreviewed, project)));\n',
        '',
        'dashboard::tests::the_unreviewed_code_block_is_the_check_at_head_in_each_state pipeline_view::tests::the_summary_is_one_item_per_line_and_ends_with_the_one_target_hint',
    ),
    (
        'M5 code found reads as none',
        'src/primitives/pipeline_view.rs',
        '        UnreviewedState::None => "Unreviewed code: none".to_string(),\n        UnreviewedState::Found => format!(',
        '        UnreviewedState::None | UnreviewedState::Found => "Unreviewed code: none".to_string(),\n        UnreviewedState::NotExamined if false => format!(',
        'pipeline_view::tests::the_unreviewed_code_line_says_each_state_in_its_own_words',
    ),
    (
        'M6 a subject not examined drops what was found',
        'src/primitives/pipeline_view.rs',
        '            if counts.path_count == 0 {',
        '            if true {',
        'pipeline_view::tests::the_unreviewed_code_line_says_each_state_in_its_own_words',
    ),
    (
        'M7 the path count leaves out a done spec\'s paths',
        'src/primitives/check_unreviewed_code.rs',
        '                path_count: checked.reported_paths(),\n',
        '                path_count: checked.unclaimed_paths(),\n',
        'dashboard::tests::the_unreviewed_code_block_is_the_check_at_head_in_each_state',
    ),
]
