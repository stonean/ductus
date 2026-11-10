"""Mutation list for 080's reference and rule reads (tasks 37 and 38).
`read-section` returns each named section of a document, cut at the next
heading of the same or a higher level by `resolve_anchor.rs`'s
`section_range`, refuses a path leaving the project, and moves its sections
to a file past the read size. `discover-rule-files` with `rules` lists each
selected file's rules through `rule_sections`, adds each Verification only
with `verification`, selects every file with `all` past `[rules] surfaces`
and the disabled-rule-files filter, lists an extra file after the selected
ones and refuses one leaving the project, names a file it cannot read under
`unreadable`, and moves its rules to a file past the read size
(`runtime/src/primitives/read_section.rs`, `discover_rule_files.rs`).

Run with `python3 scripts/mutation/run.py
scripts/mutation/reference-and-rule-reads.py [ID ...]`; see run.py for the
format. This file is data, read with ast.literal_eval: keep every value a
literal. An id is never reused: a mutation that no longer applies is
removed, and the next one takes the next free number. When the code a
mutation anchors on changes, update the anchor in the same change;
`run.py --check` names every anchor that no longer matches exactly once.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 a section is cut at the next heading of any level',
        'src/primitives/resolve_anchor.rs',
        '        .find(|h| h.level <= start.level)\n',
        '        .find(|h| h.level > 0)\n',
        'read_section::tests::each_heading_asked_for_comes_back_with_its_section_in_order',
    ),
    (
        'M2 read-section does not refuse a path leaving the project',
        'src/primitives/read_section.rs',
        '    super::validate_contained(repo, &args.path)?;\n',
        '    let _ = super::validate_contained;\n',
        'read_section::tests::a_path_outside_the_project_is_refused',
    ),
    (
        'M3 read-section keeps its sections inline past the read size',
        'src/primitives/read_section.rs',
        '    if let Some(file) = super::bounded_lists::place(repo, "read-section", key, &result, &lists)? {\n',
        '    if let Some(file) = super::bounded_lists::place(repo, "read-section", key, &result, &lists)?.filter(|_| false) {\n',
        'read_section::tests::past_the_read_size_the_sections_move_to_a_file',
    ),
    (
        'M4 rules is ignored',
        'src/primitives/discover_rule_files.rs',
        '    if args.rules {\n',
        '    if args.rules && !args.rules {\n',
        'discover_rule_files::tests::rules_lists_each_selected_files_rules',
    ),
    (
        'M5 the Verification comes back without verification',
        'src/primitives/discover_rule_files.rs',
        '                verification: section.verification.clone().filter(|_| args.verification),\n',
        '                verification: section.verification.clone(),\n',
        'discover_rule_files::tests::rules_lists_each_selected_files_rules',
    ),
    (
        'M6 all selects as review does',
        'src/primitives/discover_rule_files.rs',
        '            if args.all {\n',
        '            if args.all && !args.all {\n',
        'discover_rule_files::tests::all_selects_every_file_past_surfaces_and_the_disabled_filter',
    ),
    (
        'M7 all still applies the disabled-rule-files filter',
        'src/primitives/discover_rule_files.rs',
        '        if !args.all {\n',
        '        if !args.all || args.all {\n',
        'discover_rule_files::tests::all_selects_every_file_past_surfaces_and_the_disabled_filter',
    ),
    (
        'M8 an extra file is not listed',
        'src/primitives/discover_rule_files.rs',
        '        files.push(extra.clone());\n',
        '        let _ = extra;\n',
        'discover_rule_files::tests::extra_files_are_listed_and_an_unreadable_file_is_named',
    ),
    (
        'M9 a file that cannot be read is dropped silently',
        'src/primitives/discover_rule_files.rs',
        '                result.unreadable.push(UnreadableRuleFile {\n                    file,\n                    reason: err.to_string(),\n                });\n',
        '                let _ = (file, err);\n',
        'discover_rule_files::tests::extra_files_are_listed_and_an_unreadable_file_is_named',
    ),
    (
        'M10 an extra file leaving the project is not refused',
        'src/primitives/discover_rule_files.rs',
        '        super::validate_contained(repo, extra)?;\n',
        '        let _ = super::validate_contained;\n',
        'discover_rule_files::tests::extra_files_are_listed_and_an_unreadable_file_is_named',
    ),
    (
        'M11 the rules stay inline past the read size',
        'src/primitives/discover_rule_files.rs',
        '        result.form = Some(ListsForm::File);\n        result.rule_files = None;\n',
        '        result.form = Some(ListsForm::Inline);\n',
        'discover_rule_files::tests::past_the_read_size_the_rules_move_to_a_file',
    ),
    (
        'M12 a rule\'s heading line is not carried',
        'src/primitives/rule_sections.rs',
        '            heading: self.heading,\n',
        '            heading: String::new(),\n',
        'discover_rule_files::tests::rules_lists_each_selected_files_rules',
    ),
]
