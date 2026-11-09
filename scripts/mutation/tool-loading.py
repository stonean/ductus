"""Mutation list for 080's step-tool derivation (task 9; AC21): a command's
steps name a served tool as a whole token, backticked or bare, in any kind of
step, never a CLI-only one; and the installed `target` command is found
through every candidate, derived when it parses, and reported when it cannot
be (`runtime/src/primitives/tool_loading.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/tool-loading.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

run.py runs unit tests only. The server's flagging, which these tools feed,
is held by `the_tools_targets_steps_name_are_always_loaded_and_no_other_is`
and `with_no_installed_target_no_tool_carries_meta` in
`runtime/tests/mcp.rs`: removing the pass fails the first, and flagging every
route fails both, run by hand with `cargo test --locked --test mcp`.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 a name preceded by a name character still counts',
        'src/primitives/tool_loading.rs',
        '        !before.is_some_and(is_name_char) && !after.is_some_and(is_name_char)\n',
        '        let _ = before;\n        !after.is_some_and(is_name_char)\n',
        'tool_loading::tests::a_name_counts_backticked_or_bare_and_never_inside_a_longer_token',
    ),
    (
        'M2 a name followed by a name character still counts',
        'src/primitives/tool_loading.rs',
        '        !before.is_some_and(is_name_char) && !after.is_some_and(is_name_char)\n',
        '        let _ = after;\n        !before.is_some_and(is_name_char)\n',
        'tool_loading::tests::a_name_counts_backticked_or_bare_and_never_inside_a_longer_token',
    ),
    (
        'M3 a CLI-only name is kept',
        'src/primitives/tool_loading.rs',
        '        tools.extend(registry::served_primitives().filter(|name| names_token(text, name)));\n',
        '        tools.extend(\n            registry::PRIMITIVE_REGISTRY\n                .iter()\n                .copied()\n                .filter(|name| names_token(text, name)),\n        );\n',
        'tool_loading::tests::a_cli_only_name_is_left_out',
    ),
    (
        'M4 a prose step names nothing',
        'src/primitives/tool_loading.rs',
        '            Step::Prose { text, .. } => text,\n',
        '            Step::Prose { .. } => "",\n',
        'tool_loading::tests::every_kind_of_step_contributes_its_prose',
    ),
    (
        'M5 a missing candidate ends the search as unreadable',
        'src/primitives/tool_loading.rs',
        '                Err(err) if err.kind() == std::io::ErrorKind::NotFound => continue,\n',
        '',
        'tool_loading::tests::no_copy_flags_nothing_and_names_every_path_tried',
    ),
    (
        'M6 a derived copy flags nothing',
        'src/primitives/tool_loading.rs',
        '            Self::Derived { tools, .. } => tools.clone(),\n',
        '            Self::Derived { .. } => BTreeSet::new(),\n',
        'tool_loading::tests::an_installed_copy_is_derived_and_needs_no_notice',
    ),
    (
        'M7 no copy is silent',
        'src/primitives/tool_loading.rs',
        '            Self::Derived { .. } => None,\n',
        '            Self::Derived { .. } | Self::NoCopy { .. } => None,\n',
        'tool_loading::tests::no_copy_flags_nothing_and_names_every_path_tried',
    ),
]
