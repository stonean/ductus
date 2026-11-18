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
]
