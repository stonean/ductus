"""Mutation list for 078's task 13: `check-prescriptions` matches a
prescribed file with every edit applied or with only its MUST and SHOULD
findings' edits applied, so a NOTE's declined edit forces no re-read, while
an unapplied MUST or SHOULD edit still differs, and a mode change beside the
prescribed edits differs too (`runtime/src/primitives/check_prescriptions.rs`,
`compare` and `apply_edits`).

Run with `python3 scripts/mutation/run.py scripts/mutation/declined-note-edits.py
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
        'M1 a declined NOTE edit differs again',
        'src/primitives/check_prescriptions.rs',
        '        if !matches(&all)? && !matches(&blocking)? {',
        '        if !matches(&all)? && (!matches(&blocking)? || blocking.1.is_empty()) {',
        'check_prescriptions::tests::a_declined_note_edit_still_matches',
    ),
    (
        'M2 the blocking text keeps the NOTE edits',
        'src/primitives/check_prescriptions.rs',
        '        let blocking = apply_edits(text.as_deref(), edits, path, |&(_, _, note, _, _)| !note);',
        '        let blocking = apply_edits(text.as_deref(), edits, path, |_| true);',
        'check_prescriptions::tests::a_declined_note_edit_still_matches',
    ),
    (
        'M3 the blocking text drops the SHOULD edits instead',
        'src/primitives/check_prescriptions.rs',
        '        let blocking = apply_edits(text.as_deref(), edits, path, |&(_, _, note, _, _)| !note);',
        '        let blocking = apply_edits(text.as_deref(), edits, path, |&(_, _, note, _, _)| note);',
        'check_prescriptions::tests::a_declined_should_edit_still_differs',
    ),
    (
        "M4 a file's mode is not compared, so a chmod beside the prescribed edit matches",
        'src/primitives/check_prescriptions.rs',
        '                .zip(mode);',
        '                .zip(actual.map(|(_, mode)| mode));',
        'check_prescriptions::tests::a_mode_change_beside_the_prescribed_edit_differs',
    ),
]
