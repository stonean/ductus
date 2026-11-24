"""Mutation list for 081's task 8: a review states what it read (`unread`,
written by `write-review` whenever the run states it, an entry outside the
run's scope refused; `runtime/src/primitives/write_review.rs`). The mutants on
the credit `compute-review-scope` gave a commit naming no spec went with that
credit, which spec 084 removed.

Run with `python3 scripts/mutation/run.py scripts/mutation/review-read-credit.py
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
        'M8 write-review takes an unread entry outside the scope',
        'src/primitives/write_review.rs',
        '    if outside.is_empty() {\n',
        '    if true {\n',
        'write_review::tests::an_unread_entry_outside_the_scope_is_refused',
    ),
    (
        'M9 an empty unread is written as nothing',
        'src/primitives/write_review.rs',
        '            let _ = writeln!(fm, "unread: []");\n',
        '',
        'write_review::tests::unread_is_written_whenever_the_run_states_it',
    ),
    (
        'M10 unread is never written',
        'src/primitives/write_review.rs',
        '    render_unread(&mut fm, args.unread.as_ref());\n',
        '',
        'write_review::tests::unread_is_written_whenever_the_run_states_it',
    ),
    (
        'M14 a refused unread entry is echoed raw',
        'src/primitives/write_review.rs',
        '        .map(|entry| crate::primitives::flatten_line(entry))\n',
        '        .map(|entry| entry.to_string())\n',
        'write_review::tests::an_unread_entry_outside_the_scope_is_refused',
    ),
]
