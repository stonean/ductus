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
]
