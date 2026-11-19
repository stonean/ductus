"""Mutation list for spec 066 task 27 (scenario `criteria-are-verified-in-review`):
`/{project}:review` judges every acceptance criterion, `write-review` records
each verdict, a not-met criterion blocks, and the completion gate judges
nothing. Each mutation drops or widens one part of those rules; a test kills
each.

Run with `python3 scripts/mutation/run.py scripts/mutation/criteria-in-review-066.py
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
        'M1 a not-met criterion does not block',
        'src/primitives/write_review.rs',
        '    let blocking = must_n > 0 || criteria.as_ref().is_some_and(|c| !c.not_met.is_empty());\n',
        '    let blocking = must_n > 0;\n',
        'primitives::write_review::tests::criteria_verdicts_are_recorded_and_a_not_met_one_blocks primitives::write_review::tests::an_empty_scope_reads_blocking_from_a_not_met_criterion',
    ),
    (
        'M2 a not-met verdict is recorded as met',
        'src/primitives/write_review.rs',
        '        if verdict.met {\n            record.met = record.met.saturating_add(1);\n',
        '        if true {\n            record.met = record.met.saturating_add(1);\n',
        'primitives::write_review::tests::criteria_verdicts_are_recorded_and_a_not_met_one_blocks',
    ),
    (
        'M3 a second verdict on one criterion is accepted',
        'src/primitives/write_review.rs',
        '        if judged.insert(position, verdict).is_some() {\n',
        '        if judged.insert(position, verdict).is_some() && false {\n',
        'primitives::write_review::tests::a_verdict_that_addresses_no_criterion_once_is_refused',
    ),
    (
        'M4 a verdict naming both a label and an index is read by its label',
        'src/primitives/write_review.rs',
        '            (Some(label), None) => criteria\n',
        '            (Some(label), _) => criteria\n',
        'primitives::write_review::tests::a_verdict_that_addresses_no_criterion_once_is_refused',
    ),
    (
        'M5 the criteria fields are not written',
        'src/primitives/write_review.rs',
        '    render_criteria(&mut fm, record.criteria);\n',
        '',
        'primitives::write_review::tests::criteria_verdicts_are_recorded_and_a_not_met_one_blocks primitives::write_review::tests::criteria_all_met_leave_the_record_non_blocking',
    ),
    (
        "M6 a criterion's digest keeps its label",
        'src/primitives/criterion_verdicts.rs',
        '    super::hex(&Sha256::digest(text(criterion).as_bytes()))\n',
        '    super::hex(&Sha256::digest(criterion.text.as_bytes()))\n',
        'primitives::criterion_verdicts::tests::a_label_and_a_tick_leave_the_digest_and_a_word_moves_it',
    ),
    (
        'M7 a note spanning lines is accepted',
        'src/primitives/write_review.rs',
        '            super::validate_single_line("write-review", &format!("criteria[{idx}].note"), note)?;\n',
        '',
        'primitives::write_review::tests::a_verdict_that_addresses_no_criterion_once_is_refused',
    ),
    (
        "M8 the empty-scope Summary says blocking: no whatever the record says",
        'src/primitives/write_review.rs',
        '        let blocking = if blocking { "yes" } else { "no" };\n',
        '        let blocking = "no";\n',
        'primitives::write_review::tests::an_empty_scope_reads_blocking_from_a_not_met_criterion',
    ),
    (
        "M9 an invalidated record keeps its run's criteria verdicts",
        'src/primitives/invalidate_review.rs',
        '        "criteria-verdicts",\n',
        '        "criteria-verdicts-kept",\n',
        'primitives::invalidate_review::tests::a_runs_excluded_count_breaks_and_open_mark_go_and_its_open_findings_stay',
    ),
    (
        'M10 an unticked criterion judged met owes no review',
        'src/primitives/criterion_verdicts.rs',
        '        .filter(|(_, criterion)| !criterion.checked || !judged.contains(digest(criterion).as_str()))\n',
        '        .filter(|(_, criterion)| !judged.contains(digest(criterion).as_str()))\n',
        'primitives::criterion_verdicts::tests::a_criterion_owes_a_review_until_judged_met_with_its_text_and_ticked',
    ),
    (
        'M11 a ticked criterion is judged by its tick alone',
        'src/primitives/criterion_verdicts.rs',
        '        .filter(|(_, criterion)| !criterion.checked || !judged.contains(digest(criterion).as_str()))\n',
        '        .filter(|(_, criterion)| !criterion.checked)\n',
        'primitives::criterion_verdicts::tests::a_criterion_owes_a_review_until_judged_met_with_its_text_and_ticked primitives::criterion_verdicts::tests::a_record_without_verdicts_leaves_every_criterion_owing primitives::compute_review_scope::tests::an_artifact_only_window_owes_a_review_while_a_criterion_is_unjudged primitives::check_review_gate::tests::a_criterion_no_review_judged_owes_a_review_and_is_named',
    ),
    (
        'M12 review-owed ignores the criteria',
        'src/primitives/compute_review_scope.rs',
        '    run.is_none_or(holds_done_until_reviewed) || !owed_by.is_empty() || !criteria_owing.is_empty()\n',
        '    run.is_none_or(holds_done_until_reviewed) || !owed_by.is_empty()\n',
        'primitives::compute_review_scope::tests::an_artifact_only_window_owes_a_review_while_a_criterion_is_unjudged primitives::check_review_gate::tests::a_criterion_no_review_judged_owes_a_review_and_is_named',
    ),
    (
        'M13 a spec.md that cannot be read owes nothing',
        'src/primitives/criterion_verdicts.rs',
        '    let text = super::read_text(spec_path)?;\n',
        '    let Ok(text) = super::read_text(spec_path) else {\n        return Ok(Vec::new());\n    };\n',
        'primitives::criterion_verdicts::tests::an_unreadable_spec_is_an_error_not_a_spec_owing_nothing',
    ),
    (
        "M14 the gate's review-owed block names no criterion",
        'src/primitives/check_review_gate.rs',
        '        (true, false) => criteria,\n',
        '        (true, false) => String::new(),\n',
        'primitives::check_review_gate::tests::a_criterion_no_review_judged_owes_a_review_and_is_named',
    ),
    (
        'M15 the result leaves the owing criteria out',
        'src/primitives/compute_review_scope.rs',
        '        criteria_owing: scope.criteria_owing.clone(),\n',
        '        criteria_owing: Vec::new(),\n',
        'primitives::compute_review_scope::tests::the_result_names_each_criterion_owing_a_review',
    ),
    (
        'M16 an unticked criterion does not hold done',
        'src/primitives/check_review_gate.rs',
        '    if let Some(blocked) = unticked_criterion_block(body, &rel_dir, &project) {\n        return Ok(blocked);\n    }\n',
        '',
        'primitives::check_review_gate::tests::an_unticked_criterion_holds_done_after_an_unchecked_task',
    ),
    (
        'M17 the gate counts a ticked criterion as unticked',
        'src/primitives/check_review_gate.rs',
        '            .filter(|(_, criterion)| !criterion.checked)\n',
        '            .filter(|_| true)\n',
        'primitives::check_review_gate::tests::an_unticked_criterion_holds_done_after_an_unchecked_task',
    ),
    (
        'M18 the exec walk marks only the seeded criterion',
        'src/interpreter/mod.rs',
        '        return mark_each_verdict(verify, &bindings, repo);\n',
        '        let _ = verify;\n',
        'interpreter::tests::mark_criterion_applies_every_verdict_both_ways interpreter::tests::the_review_walk_judges_every_criterion_and_writes_its_tick',
    ),
    (
        'M19 every verdict ticks its criterion, met or not',
        'src/interpreter/mod.rs',
        '            checked: verdict.met,\n',
        '            checked: true,\n',
        'interpreter::tests::mark_criterion_applies_every_verdict_both_ways interpreter::tests::the_review_walk_judges_every_criterion_and_writes_its_tick interpreter::tests::mark_criterion_skips_flip_when_verdict_not_met',
    ),
    (
        "M20 the exec walk's write-review receives no verdicts",
        'src/interpreter/mod.rs',
        '        bindings.insert("criteria".into(), results);\n',
        '        let _ = results;\n',
        'interpreter::tests::the_review_walk_judges_every_criterion_and_writes_its_tick',
    ),
    (
        'M21 a record blocking only through a not-met criterion is called MUST violations',
        'src/primitives/check_review_gate.rs',
        '        (0, false) => format!("the review judged acceptance criteria not met: {criteria}"),\n',
        '        (0, false) => format!("spec has 0 MUST violation(s)"),\n',
        'primitives::check_review_gate::tests::a_not_met_criterion_blocks_by_name_not_as_must_violations',
    ),
    (
        'M22 the criteria section is listed before the violations',
        'src/primitives/write_review.rs',
        '        sections.insert(\n            7,\n',
        '        sections.insert(\n            0,\n',
        'primitives::write_review::tests::an_unlabelled_not_met_criterion_is_recorded_quoted_and_listed_after_the_violations',
    ),
    (
        "M23 an unlabelled not-met criterion's name is written unquoted",
        'src/primitives/write_review.rs',
        '            let _ = writeln!(fm, "  - {}", yaml_string(name));\n',
        '            let _ = writeln!(fm, "  - {name}");\n',
        'primitives::write_review::tests::an_unlabelled_not_met_criterion_is_recorded_quoted_and_listed_after_the_violations',
    ),
    (
        'M24 a supplied Summary drops the criteria sentence',
        'src/primitives/write_review.rs',
        '            let _ = write!(summary, "\\n\\n{}", criteria_summary(criteria));\n',
        '            let _ = criteria;\n',
        'primitives::write_review::tests::a_supplied_summary_carries_the_criteria_sentence',
    ),
    (
        'M25 the lists file leaves out the owing criteria',
        'src/primitives/compute_review_scope.rs',
        '        "criteria-owing": scope.criteria_owing,\n',
        '        "criteria-owing": Vec::<String>::new(),\n',
        'primitives::compute_review_scope::tests::past_the_read_size_the_owing_criteria_are_in_the_lists_file',
    ),
    (
        'M26 the file form keeps the owing criteria inline',
        'src/primitives/compute_review_scope.rs',
        '        result.criteria_owing = Vec::new();\n',
        '        let _ = ();\n',
        'primitives::compute_review_scope::tests::past_the_read_size_the_owing_criteria_are_in_the_lists_file',
    ),
    (
        'M27 a review owed for paths and criteria names only the paths',
        'src/primitives/check_review_gate.rs',
        '            "what changed since the last one or what it left open: {}; and {criteria}",\n',
        '            "what changed since the last one or what it left open: {}",\n',
        'primitives::check_review_gate::tests::a_review_owed_for_paths_and_criteria_names_both',
    ),
    (
        'M28 a block only through a not-met criterion offers a waiver or a deferral',
        'src/primitives/check_review_gate.rs',
        '        (0, false) => criteria_advice.to_string(),\n',
        '        (0, false) => format!("{violations} {criteria_advice}"),\n',
        'primitives::check_review_gate::tests::a_not_met_criterion_blocks_by_name_not_as_must_violations',
    ),
]
