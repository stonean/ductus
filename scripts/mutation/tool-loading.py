"""Mutation list for 080's step-tool derivation and tool-search line (tasks 9
and 10; AC21, AC29). A command's steps name a served tool as a whole token,
backticked or bare, in any kind of step, never a CLI-only one, and a
legacy-prose command's whole Instructions section stands in for its steps.
The installed `target` command is found through every candidate, derived when
it parses, and reported when it cannot be. The line closes the preamble with
the tools left after target's, in the file's own line ending, and only a
flagged `apply-manifest` entry gets it (`runtime/src/primitives/tool_loading.rs`,
`apply_manifest.rs`).

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
    (
        'M8 a legacy-prose section is read from its preamble',
        'src/primitives/tool_loading.rs',
        "    while first < end && lines[first].starts_with('>') {\n        first += 1;\n    }\n",
        '',
        'tool_loading::tests::a_legacy_prose_command_names_the_tools_under_its_sub_headings',
    ),
    (
        'M9 a legacy-prose section runs past its end',
        'src/primitives/tool_loading.rs',
        '        .find(|h| h.0 > start && h.1 <= level)\n',
        '        .find(|h| h.0 > start && h.1 == 0)\n',
        'tool_loading::tests::a_legacy_prose_command_names_the_tools_under_its_sub_headings',
    ),
    (
        'M10 the line names the tools that load with the session',
        'src/primitives/tool_loading.rs',
        '        .difference(always_loaded)\n',
        '        .difference(&BTreeSet::new())\n',
        'tool_loading::tests::the_line_closes_the_preamble_naming_the_tools_left_sorted',
    ),
    (
        'M11 the line ends in LF in a CRLF file',
        'src/primitives/tool_loading.rs',
        '    let eol = line_ending_of(text);\n',
        '    let eol = "\\n";\n',
        'tool_loading::tests::the_line_takes_the_files_line_ending',
    ),
    (
        'M12 a heading inside a fence is a heading',
        'src/primitives/tool_loading.rs',
        '        if let Some(open) = fence {\n',
        '        if let Some(open) = fence.filter(|_| false) {\n',
        'tool_loading::tests::a_heading_inside_a_fence_is_not_the_section',
    ),
    (
        'M13 an unflagged entry gets the line',
        'src/primitives/apply_manifest.rs',
        '    let Some(always_loaded) = always_loaded.filter(|_| entry.tool_search_line) else {\n',
        '    let Some(always_loaded) = always_loaded else {\n',
        'apply_manifest::tests::a_flagged_copy_carries_the_line_and_parses_to_its_sources_procedure',
    ),
    (
        'M14 a missing target.md leaves every tool in the line',
        'src/primitives/apply_manifest.rs',
        '    let path = source_root.join("commands").join("target.md");\n',
        '    let path = source_root.join("commands").join("target.md");\n    if !path.exists() {\n        return Ok(BTreeSet::new());\n    }\n',
        'apply_manifest::tests::a_flagged_entry_without_target_md_halts_before_any_write',
    ),
]
