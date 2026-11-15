"""Mutation list for spec 067 task 33: `check-artifacts`' `criterion-path-existence`
and `body-path-existence` probes never follow a symbolic link that leaves the
project (`BE-INPUT-004`). Each mutation drops one probe's link guard, so the
probe stats the file the link leads to. The tests that kill them link a path
to a file outside the project, which the lexical screen passes, and expect an
`outside-project` skip.

Run with `python3 scripts/mutation/run.py scripts/mutation/criterion-body-path-links.py
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
        'M1 the criterion probe follows a link out of the project',
        'src/primitives/check_artifacts.rs',
        '            if leaves_project_by_link(repo, trimmed) {',
        '            if false && leaves_project_by_link(repo, trimmed) {',
        'check_artifacts::tests::a_criterion_path_through_a_link_out_of_the_project_is_recorded_not_probed',
    ),
    (
        'M2 the body probe follows a link out of the project',
        'src/primitives/check_artifacts.rs',
        '                if leaves_project_by_link(repo, &resolved) {',
        '                if false && leaves_project_by_link(repo, &resolved) {',
        'check_artifacts::tests::a_body_path_through_a_link_out_of_the_project_is_recorded_not_probed',
    ),
]
