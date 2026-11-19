"""Mutation list for spec 026 task 8: `check-step-references` judges the
reference sections a command's steps reach (`reach`, `judge_sections`, and
the anchor-link arm of the citation walk in `tool_loading`). Each mutation
removes one part of the rule: the reach test, the exemption, one of the three
exemption findings, the no-sections count, the skip, the `##` level, the
walk's following of a reached section and of an anchor link, and the CLI's
exit status, and the sixth list's place in the bounded result. The tests that kill them are named per mutation.

Run with `python3 scripts/mutation/run.py scripts/mutation/step-references-reach.py
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
        'M1 a section no step reaches is never a finding',
        'src/primitives/check_step_references.rs',
        '        if !is_reached && !exempted {\n',
        '        if false {\n',
        'check_step_references::tests::an_unreached_section_is_reported_and_named',
    ),
    (
        'M2 every section counts as reached',
        'src/primitives/check_step_references.rs',
        '        let is_reached = reached.contains(section.text.as_str());\n',
        '        let is_reached = true;\n',
        'check_step_references::tests::an_unreached_section_is_reported_and_named',
    ),
    (
        'M3 a marker with a reason exempts nothing',
        'src/primitives/check_step_references.rs',
        '                exempted = true;\n',
        '',
        'check_step_references::tests::an_exempt_section_with_a_reason_is_exempt_and_counted',
    ),
    (
        'M4 an exemption on a reached section is never stale',
        'src/primitives/check_step_references.rs',
        '            Some(_) if is_reached => out.findings.push(finding(\n',
        '            Some(_) if false => out.findings.push(finding(\n',
        'check_step_references::tests::an_empty_stale_or_misplaced_exemption_is_a_finding',
    ),
    (
        'M5 an empty reason is accepted',
        'src/primitives/check_step_references.rs',
        '            Some(reason) if reason.is_empty() => out.findings.push(finding(\n',
        '            Some(reason) if false && reason.is_empty() => out.findings.push(finding(\n',
        'check_step_references::tests::an_empty_stale_or_misplaced_exemption_is_a_finding',
    ),
    (
        'M6 a marker beneath no heading is never misplaced',
        'src/primitives/check_step_references.rs',
        '        if !skip.skip(line) && kept_regex().is_match(line) && !attached.contains(&i) {\n',
        '        if !skip.skip(line) && kept_regex().is_match(line) && false {\n',
        'check_step_references::tests::an_empty_stale_or_misplaced_exemption_is_a_finding',
    ),
    (
        'M7 a subsection counts as a section',
        'src/primitives/check_step_references.rs',
        '        .filter(|h| h.level == 2)\n',
        '        .filter(|h| h.level >= 2)\n',
        'check_step_references::tests::a_cited_subsection_does_not_reach_its_parent_section',
    ),
    (
        'M8 a heading in a fenced block counts as a section',
        'src/primitives/check_step_references.rs',
        '    let sections: Vec<_> = heading_lines(reference)\n        .into_iter()\n        .filter(|h| h.level == 2)\n        .collect();\n',
        '    let sections: Vec<crate::primitives::resolve_anchor::HeadingLine> = lines\n        .iter()\n        .enumerate()\n        .filter(|(_, l)| l.starts_with("## "))\n        .map(|(i, l)| crate::primitives::resolve_anchor::HeadingLine {\n            line: i,\n            level: 2,\n            text: l[3..].trim().to_string(),\n        })\n        .collect();\n    let _ = heading_lines;\n',
        'check_step_references::tests::a_heading_in_a_fenced_block_is_not_a_section',
    ),
    (
        'M9 a reference with no section is not counted',
        'src/primitives/check_step_references.rs',
        '            scan.reachability.iter().filter(|r| r.sections == 0).count(),\n',
        '            scan.reachability.iter().filter(|r| r.sections == 99).count(),\n',
        'check_step_references::tests::a_reference_with_no_sections_is_reported_as_having_none',
    ),
    (
        'M10 a reference that cannot be judged is not reported skipped',
        'src/primitives/check_step_references.rs',
        '        skipped.extend(reached.skipped);\n',
        '        let _ = reached.skipped;\n',
        'check_step_references::tests::an_unreadable_reference_is_a_skip',
    ),
    (
        'M11 the walk does not follow a reached section',
        'src/primitives/tool_loading.rs',
        '        cite(&doc, &body, &mut docs, &mut queue)?;\n',
        '        let _ = &body;\n',
        'check_step_references::tests::a_section_reached_through_a_reached_one_or_by_an_anchor_link_is_reached',
    ),
    (
        'M12 the walk does not follow an anchor link',
        'src/primitives/tool_loading.rs',
        '                    queue.push((from.clone(), heading.clone()));\n',
        '                    let _ = heading;\n',
        'tool_loading::tests::an_anchor_link_reaches_the_section_it_names_in_its_own_document check_step_references::tests::a_section_reached_through_a_reached_one_or_by_an_anchor_link_is_reached',
    ),
    (
        'M13 the CLI exits zero over a finding',
        'src/main.rs',
        '                r.findings_count > 0 || r.skipped_count > 0 || !r.guidance.is_empty()\n',
        '                r.skipped_count > 0 || !r.guidance.is_empty()\n',
        '--test check_step_references_cli',
    ),
    (
        'M14 the file form keeps reachability inline',
        'src/primitives/check_step_references.rs',
        '        result.reachability = None;\n',
        '',
        'check_step_references::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        'M15 the file omits reachability',
        'src/primitives/check_step_references.rs',
        '        "reachability": scan.reachability,\n',
        '',
        'check_step_references::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
]
