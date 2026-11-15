"""Mutation list for spec 026 task 6: `check-step-references`' result is
bounded (BE-PAYLOAD-001). Each mutation removes one part of the bound in
`bound`: the file form's form tag and list file, each of the five lists
leaving the result or entering the file, a count, and the segment the file
sits under. The tests that kill them are
`the_lists_are_inline_while_the_result_fits` and
`the_lists_are_in_a_file_past_the_read_size`.

Run with `python3 scripts/mutation/run.py scripts/mutation/step-references-bounded.py
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
        'M1 the file form is never tagged',
        'src/primitives/check_step_references.rs',
        '        result.form = ListsForm::File;\n',
        '',
        'check_step_references::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        'M2 the file form names no list file',
        'src/primitives/check_step_references.rs',
        '        result.lists_file = Some(file);\n',
        '        let _ = file;\n',
        'check_step_references::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        'M3 the file form keeps findings inline',
        'src/primitives/check_step_references.rs',
        '        result.findings = None;\n',
        '',
        'check_step_references::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        'M4 the file form keeps examined inline',
        'src/primitives/check_step_references.rs',
        '        result.examined = None;\n',
        '',
        'check_step_references::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        'M5 the file form keeps with-steps inline',
        'src/primitives/check_step_references.rs',
        '        result.with_steps = None;\n',
        '',
        'check_step_references::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        'M6 the file form keeps not-a-procedure inline',
        'src/primitives/check_step_references.rs',
        '        result.not_a_procedure = None;\n',
        '',
        'check_step_references::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        'M7 the file form keeps skipped inline',
        'src/primitives/check_step_references.rs',
        '        result.skipped = None;\n',
        '',
        'check_step_references::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        'M8 the file omits findings',
        'src/primitives/check_step_references.rs',
        '        "findings": scan.findings,\n',
        '',
        'check_step_references::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        'M9 the file omits skipped',
        'src/primitives/check_step_references.rs',
        '        "skipped": scan.skipped,\n',
        '',
        'check_step_references::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        'M10 the file sits under another segment',
        'src/primitives/check_step_references.rs',
        '        super::bounded_lists::ALL_SPECS,\n',
        '        "elsewhere",\n',
        'check_step_references::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        'M11 the findings count is not the list length',
        'src/primitives/check_step_references.rs',
        '        findings_count: scan.findings.len(),\n',
        '        findings_count: 0,\n',
        'check_step_references::tests::the_lists_are_inline_while_the_result_fits',
    ),
    (
        'M12 the examined count is not the list length',
        'src/primitives/check_step_references.rs',
        '        examined_count: scan.examined.len(),\n',
        '        examined_count: 0,\n',
        'check_step_references::tests::the_lists_are_inline_while_the_result_fits',
    ),
]
