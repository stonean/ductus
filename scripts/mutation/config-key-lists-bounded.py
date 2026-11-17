"""Mutation list for 022's task 32: `discover-rule-files`' `notices` and
`dashboard`'s `config.unknown-keys`, each a line per unknown config key, and
`discover-rule-files`' `unreadable` (M7, from 080's analysis), held to
`[artifacts] read-size-bytes` (BE-PAYLOAD-001, from 019's analysis). Past it
`notices` moves to the list file with or without `rules`, and `unknown-keys`
moves there under `config`, each with its count inline in both forms
(`runtime/src/primitives/discover_rule_files.rs` `bound`,
`runtime/src/primitives/dashboard.rs` `run_as` and `load_config`).

Run with `python3 scripts/mutation/run.py scripts/mutation/config-key-lists-bounded.py
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
        'M1 discover-rule-files leaves notices inline in the file form',
        'src/primitives/discover_rule_files.rs',
        '        result.selected = None;\n        result.notices = None;\n',
        '        result.selected = None;\n',
        'discover_rule_files::tests::past_the_read_size_the_notices_move_to_the_file',
    ),
    (
        'M2 discover-rule-files bounds its notices only with rules',
        'src/primitives/discover_rule_files.rs',
        '    bound(repo, &mut result, notices)?;\n    Ok(result)',
        '    if args.rules {\n        bound(repo, &mut result, notices)?;\n    } else {\n        result.notices_count = notices.len();\n        result.notices = Some(notices);\n    }\n    Ok(result)',
        'discover_rule_files::tests::past_the_read_size_the_notices_move_to_the_file',
    ),
    (
        'M3 discover-rule-files counts no notice',
        'src/primitives/discover_rule_files.rs',
        '    result.notices_count = notices.len();\n    result.notices = Some(notices);\n    result.unreadable_count = result.unreadable.len();',
        '    result.notices_count = 0;\n    result.notices = Some(notices);\n    result.unreadable_count = result.unreadable.len();',
        'discover_rule_files::tests::past_the_read_size_the_notices_move_to_the_file discover_rule_files::tests::rules_lists_each_selected_files_rules',
    ),
    (
        'M4 dashboard leaves unknown-keys inline in the file form',
        'src/primitives/dashboard.rs',
        '        result.config.unknown_keys = None;\n',
        '',
        'dashboard::tests::the_file_form_moves_the_unknown_config_keys',
    ),
    (
        'M5 dashboard writes no unknown-keys to the list file',
        'src/primitives/dashboard.rs',
        '            "unknown-keys": &result.config.unknown_keys,\n',
        '',
        'dashboard::tests::the_file_form_moves_the_unknown_config_keys',
    ),
    (
        'M6 dashboard counts no unknown key',
        'src/primitives/dashboard.rs',
        '            unknown_keys_count: unknown_keys.len(),\n',
        '            unknown_keys_count: 0,\n',
        'dashboard::tests::the_file_form_moves_the_unknown_config_keys',
    ),
    (
        'M7 discover-rule-files leaves unreadable inline in the file form',
        'src/primitives/discover_rule_files.rs',
        '        result.rule_files = None;\n        result.unreadable = Vec::new();\n',
        '        result.rule_files = None;\n',
        'discover_rule_files::tests::past_the_read_size_unreadable_moves_to_the_file',
    ),
]
