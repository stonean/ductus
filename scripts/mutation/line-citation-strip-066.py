"""Mutation list for spec 066 task 25 (AC6): a removed line-number citation is a
mechanical edit, through the one definition in
`runtime/src/primitives/line_citations.rs` that `mechanical_sweep` and
`classify-spec-edit` read, and nothing else becomes mechanical. Each mutation
widens or drops one part of that rule; a test kills each.

Run with `python3 scripts/mutation/run.py scripts/mutation/line-citation-strip-066.py
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
        'M1 a line whose only change strips a citation is judged token by token',
        'src/primitives/mechanical_sweep.rs',
        '        if line_citations::strip(old_line) == *new_line {\n            continue;\n        }\n',
        '',
        'primitives::mechanical_sweep::tests::a_stripped_citation_is_no_rewrite primitives::analyze_subjects::tests::a_strip_only_commit_leaves_the_records_current primitives::classify_spec_edit::tests::a_citation_stripped_from_a_scenario_is_mechanical',
    ),
    (
        'M2 a stripped token beside another change is a rewrite',
        'src/primitives/mechanical_sweep.rs',
        '            if a != b && line_citations::strip(a) != b {\n',
        '            if a != b {\n',
        'primitives::mechanical_sweep::tests::a_strip_beside_another_change_is_judged_on_that_change',
    ),
    (
        'M3 the added side is stripped too, so a renumbered citation is exempt',
        'src/primitives/mechanical_sweep.rs',
        '            if a != b && line_citations::strip(a) != b {\n',
        '            if a != b && line_citations::strip(a) != line_citations::strip(b) {\n',
        'primitives::mechanical_sweep::tests::an_added_or_renumbered_citation_is_a_rewrite',
    ),
    (
        'M4 any extension makes a bare token a path, so a host and port is a citation',
        'src/primitives/line_citations.rs',
        "        && (token.contains('/') || EXTENSIONS.iter().any(|e| e.eq_ignore_ascii_case(extension)))\n",
        '',
        'primitives::line_citations::tests::hosts_ports_and_other_colons_are_not_citations primitives::mechanical_sweep::tests::a_port_removed_from_a_host_is_a_rewrite',
    ),
    (
        'M5 a token after a colon or an at sign takes a colon suffix',
        'src/primitives/line_citations.rs',
        "            Some(b':') if !after_colon_or_at => colon_suffix_end(line, at),\n",
        "            Some(b':') => colon_suffix_end(line, at),\n",
        'primitives::line_citations::tests::hosts_ports_and_other_colons_are_not_citations',
    ),
    (
        'M6 a suffix that runs on into a word or a path still ends a citation',
        'src/primitives/line_citations.rs',
        "    !line[end..]\n        .chars()\n        .next()\n        .is_some_and(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '/'))\n",
        '    !line[end..].is_empty() || line.len() >= end\n',
        'primitives::line_citations::tests::hosts_ports_and_other_colons_are_not_citations',
    ),
    (
        'M7 a criterion whose only change strips a citation is a changed criterion',
        'src/primitives/classify_spec_edit.rs',
        '            .position(|candidate| *candidate == stripped)\n',
        '            .position(|candidate| *candidate == item.as_str())\n',
        'primitives::classify_spec_edit::tests::a_citation_stripped_from_a_criterion_is_a_correction',
    ),
    (
        'M8 both sides of a criterion are stripped, so an added citation is no change',
        'src/primitives/classify_spec_edit.rs',
        '            .position(|candidate| *candidate == stripped)\n',
        '            .position(|candidate| line_citations::strip(candidate) == stripped)\n',
        'primitives::classify_spec_edit::tests::a_citation_added_to_a_criterion_reopens',
    ),
    (
        'M9 a strip hides an untick',
        'src/primitives/classify_spec_edit.rs',
        '            [text, stripped]\n',
        '            [text.clone(), text]\n',
        'primitives::classify_spec_edit::tests::a_strip_does_not_hide_an_untick',
    ),
    (
        'M10 an open question is compared exactly, so a stripped citation changes it',
        'src/primitives/classify_spec_edit.rs',
        '    if !same_items(&question_texts(&base_body), &question_texts(&now_body)) {\n',
        '    if question_texts(&base_body) != question_texts(&now_body) {\n',
        'primitives::classify_spec_edit::tests::a_citation_stripped_from_an_open_question_is_a_correction',
    ),
    (
        'M11 a colon range is cut at its first number',
        'src/primitives/line_citations.rs',
        "        end = range_end;\n    }\n    if line[end..].starts_with(':')\n",
        "        let _ = range_end;\n    }\n    if line[end..].starts_with(':')\n",
        'primitives::line_citations::tests::each_citation_form_is_found',
    ),
    (
        'M12 an en dash does not join a range',
        'src/primitives/line_citations.rs',
        "        .filter(|c| matches!(c, '-' | '–'))\n",
        "        .filter(|c| matches!(c, '-'))\n",
        'primitives::line_citations::tests::each_citation_form_is_found',
    ),
    (
        'M13 a column is cut off',
        'src/primitives/line_citations.rs',
        '        end = column_end;\n',
        '        let _ = column_end;\n',
        'primitives::line_citations::tests::each_citation_form_is_found',
    ),
    (
        'M14 a line anchor is no citation',
        'src/primitives/line_citations.rs',
        "            Some(b'#') => anchor_suffix_end(line, at),\n",
        '',
        'primitives::line_citations::tests::each_citation_form_is_found',
    ),
    (
        'M15 a line anchor range is cut at its first line',
        'src/primitives/line_citations.rs',
        '            end = range_end;\n',
        '            let _ = range_end;\n',
        'primitives::line_citations::tests::each_citation_form_is_found',
    ),
    (
        'M16 an edit left uncommitted over a committed strip is exempted with it',
        'src/primitives/analyze_subjects.rs',
        '            uncommitted_change_digested(\n                repository,\n                &head_tree,\n                &git_path,\n                &repo.join(path),\n                is_spec,\n            ) || index.changed_beyond_spelling(&git_path)\n',
        '            index.changed_beyond_spelling(&git_path)\n',
        'primitives::analyze_subjects::tests::an_uncommitted_edit_over_a_committed_strip_still_stales',
    ),
    (
        'M17 a criterion tick left uncommitted over a committed strip is an uncommitted change',
        'src/primitives/analyze_subjects.rs',
        '        if is_spec {\n            without_criterion_ticks(&text)\n        } else {\n            text\n        }\n',
        '        let _ = is_spec;\n        text\n',
        'primitives::analyze_subjects::tests::a_tick_left_uncommitted_over_a_committed_strip_keeps_the_analysis_current',
    ),
    (
        'M18 line endings a checkout converted are an uncommitted change',
        'src/primitives/analyze_subjects.rs',
        '        let text = text.replace("\\r\\n", "\\n");\n',
        '        let text = text.to_string();\n',
        'primitives::analyze_subjects::tests::converted_line_endings_over_a_committed_strip_keep_the_exemption',
    ),
]
