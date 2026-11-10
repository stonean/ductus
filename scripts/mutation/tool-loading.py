"""Mutation list for 080's step-tool derivation and tool-search line (tasks 9
and 10; AC21, AC29). A command's steps name a served tool as a whole token,
backticked or bare, in any kind of step, never a CLI-only one, and a
legacy-prose command's whole Instructions section stands in for its steps.
The installed `target` command is found through every candidate, derived when
it parses, and reported when it cannot be. The line closes the preamble with
the tools left after target's, in the file's own line ending, and only a
flagged `apply-manifest` entry gets it, taking target's tools from the
`target.md` the manifest installs, in the bootstrap's layout and the
generator's alike (`runtime/src/primitives/tool_loading.rs`,
`apply_manifest.rs`). Tasks 36 and 40: the line also names each tool a cited
section invokes, a section being cited by a `§` or a bold heading, matched
longest first, outside fences, through the references a line names, and
followed transitively; a reference the command names is read beside target's
`commands/` directory under the entry's substitutions, and one missing is an
error; and the line asks for the search to go out with the first call
(`resolve_anchor.rs`'s `cited_heading`).

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
        '    let remaining: Vec<&str> = run_tools.difference(always_loaded).copied().collect();\n',
        '    let remaining: Vec<&str> = run_tools.difference(&BTreeSet::new()).copied().collect();\n',
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
        '    let Some(tool_search) = tool_search.filter(|_| entry.tool_search_line) else {\n',
        '    let Some(tool_search) = tool_search else {\n',
        'apply_manifest::tests::a_flagged_copy_carries_the_line_and_parses_to_its_sources_procedure',
    ),
    (
        'M14 a manifest installing no target.md is not refused for that',
        'src/primitives/apply_manifest.rs',
        '        [] => {\n            return Err(invalid(\n',
        '        [] if false => {\n            return Err(invalid(\n',
        'apply_manifest::tests::a_flagged_entry_without_target_md_halts_before_any_write',
    ),
    (
        'M15 target is read from the source root\'s commands/target.md, not the manifest\'s',
        'src/primitives/apply_manifest.rs',
        '        .filter(|source| Path::new(source).ends_with("commands/target.md"))\n',
        '        .filter(|source| *source == "commands/target.md")\n',
        'apply_manifest::tests::the_bootstraps_layout_takes_target_from_the_manifest',
    ),
    (
        'M16 two different target.md sources: the first wins',
        'src/primitives/apply_manifest.rs',
        '        [source] => *source,\n',
        '        [source, ..] => *source,\n',
        'apply_manifest::tests::an_ambiguous_or_absent_target_md_halts_before_any_write',
    ),
    (
        'M17 a cited section counts every name it holds, not only those it invokes',
        'src/primitives/tool_loading.rs',
        '        tools.extend(invoked(&body));\n',
        '        tools.extend(registry::served_primitives().filter(|name| names_token(&body, name)));\n',
        'tool_loading::tests::a_cited_section_adds_the_tools_it_invokes_and_not_those_it_names',
    ),
    (
        'M18 a bold heading cites nothing',
        'src/primitives/tool_loading.rs',
        '                if inner.starts_with(\'§\') {\n',
        '                if inner.starts_with(\'§\') || inner.is_empty() || !inner.is_empty() {\n',
        'tool_loading::tests::a_bold_heading_cites_a_section',
    ),
    (
        'M19 a line naming a reference also resolves against the command\'s own documents',
        'src/primitives/tool_loading.rs',
        '            let candidates: Vec<Doc> = if named.is_empty() {\n',
        '            let candidates: Vec<Doc> = if named.is_empty() || !named.is_empty() {\n',
        'tool_loading::tests::a_line_naming_a_reference_resolves_only_against_it',
    ),
    (
        'M20 a cited section\'s own citations are not followed',
        'src/primitives/tool_loading.rs',
        '        cite(&doc, &body, &mut docs, &mut queue)?;\n',
        '        let _ = &doc;\n',
        'tool_loading::tests::citations_are_followed_and_matched_longest_first',
    ),
    (
        'M21 the shortest heading the text begins with is the one cited',
        'src/primitives/resolve_anchor.rs',
        '        .max_by_key(|h| h.len())\n',
        '        .min_by_key(|h| h.len())\n',
        'tool_loading::tests::citations_are_followed_and_matched_longest_first',
    ),
    (
        'M22 a citation inside a fenced block cites',
        'src/primitives/tool_loading.rs',
        '            if skip.skip(line) {\n                continue;\n            }\n',
        '            let _ = skip.skip(line);\n',
        'tool_loading::tests::no_named_reference_is_read_and_a_fenced_citation_cites_nothing',
    ),
    (
        'M23 a reference that cannot be read is skipped',
        'src/primitives/tool_loading.rs',
        '                    let reference = read_reference(name)?;\n',
        '                    let reference = read_reference(name).unwrap_or_default();\n',
        'tool_loading::tests::a_named_reference_that_cannot_be_read_is_an_error',
    ),
    (
        'M24 the cited tools are left out of the line',
        'src/primitives/tool_loading.rs',
        '    run_tools.extend(cited);\n',
        '    let _ = cited;\n',
        'tool_loading::tests::the_line_names_the_cited_tools_beside_the_step_tools',
    ),
    (
        'M25 the line no longer asks for the search to go out with the first call',
        'src/primitives/tool_loading.rs',
        '        "> **Claude Code:** load this command\'s `ductus` tools in one `ToolSearch` call, and send \\\n         the command\'s first `ductus` call in that same message, since a tool can be called beside \\\n         the search that loads it: `select:{query}`"\n',
        '        "> **Claude Code:** load this command\'s `ductus` tools in one `ToolSearch` call before \\\n         step 1: `select:{query}`"\n',
        'tool_loading::tests::the_line_closes_the_preamble_naming_the_tools_left_sorted',
    ),
    (
        'M26 a reference is read from the source root\'s reference/, not beside target\'s commands/',
        'src/primitives/apply_manifest.rs',
        '        .map_or_else(|| source_root.to_path_buf(), Path::to_path_buf)\n        .join("reference");\n',
        '        .map(|_| source_root.join("reference"))\n        .unwrap_or_default();\n',
        'apply_manifest::tests::a_cited_reference_sections_tool_joins_the_line_in_both_layouts',
    ),
    (
        'M27 a reference is read without the entry\'s substitutions',
        'src/primitives/apply_manifest.rs',
        '        let (bytes, _) = match read_and_substitute(&path, None, substitutions) {\n',
        '        let (bytes, _) = match read_and_substitute(&path, None, &BTreeMap::new()) {\n',
        'apply_manifest::tests::a_cited_reference_sections_tool_joins_the_line_in_both_layouts',
    ),
    (
        'M28 a name invoked outside the steps and the cited sections is left out',
        'src/primitives/tool_loading.rs',
        "    let mut tools: BTreeSet<&'static str> = lines.iter().flat_map(|line| invoked(line)).collect();\n",
        "    let mut tools: BTreeSet<&'static str> = BTreeSet::new();\n",
        'tool_loading::tests::a_name_invoked_outside_the_steps_counts',
    ),
]
