"""Mutation list for spec 067 task 35: `resolve-anchor`'s and
`read-deprecation-labels`' result lists are bounded (`BE-PAYLOAD-001`). Each
mutation drops one half of the bound: a list left inline in the file form, a
count that does not name its list, a list the file does not hold, the file
form's marker, the segment the file sits under, and the count naming the
exec walker reads a file form back by. The tests that kill them hold the
inline form under the read size and the file form one byte past it.

Run with `python3 scripts/mutation/run.py scripts/mutation/anchor-and-label-results-bounded.py
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
        "M1 resolve-anchor's references stay inline in the file form",
        'src/primitives/resolve_anchor.rs',
        '        result.references = None;\n',
        '',
        'resolve_anchor::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        "M2 resolve-anchor's unresolved stays inline in the file form",
        'src/primitives/resolve_anchor.rs',
        '        result.unresolved = None;\n',
        '',
        'resolve_anchor::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        "M3 resolve-anchor's references-count names nothing",
        'src/primitives/resolve_anchor.rs',
        '        references_count: scan.references.len(),',
        '        references_count: 0,',
        'resolve_anchor::tests::the_lists_are_inline_while_the_result_fits',
    ),
    (
        "M4 resolve-anchor's unresolved-count names nothing",
        'src/primitives/resolve_anchor.rs',
        '        unresolved_count: scan.unresolved.len(),',
        '        unresolved_count: 0,',
        'resolve_anchor::tests::the_lists_are_inline_while_the_result_fits',
    ),
    (
        "M5 resolve-anchor's list file is not marked file form",
        'src/primitives/resolve_anchor.rs',
        '        result.form = ListsForm::File;\n',
        '',
        'resolve_anchor::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        "M6 resolve-anchor's list file holds no unresolved list",
        'src/primitives/resolve_anchor.rs',
        '        "unresolved": scan.unresolved,\n',
        '',
        'resolve_anchor::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        "M7 resolve-anchor's list file sits under no path's stem",
        'src/primitives/resolve_anchor.rs',
        '    let key = super::bounded_lists::path_key(path);',
        '    let key = "document";',
        'resolve_anchor::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        "M8 read-deprecation-labels' labels stay inline in the file form",
        'src/primitives/read_deprecation_labels.rs',
        '        result.labels = None;\n',
        '',
        'read_deprecation_labels::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        "M9 read-deprecation-labels' unreadable stays inline in the file form",
        'src/primitives/read_deprecation_labels.rs',
        '        result.unreadable = None;\n',
        '',
        'read_deprecation_labels::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        "M10 read-deprecation-labels' labels-count names nothing",
        'src/primitives/read_deprecation_labels.rs',
        '        labels_count: read.labels.len(),',
        '        labels_count: 0,',
        'read_deprecation_labels::tests::the_lists_are_inline_while_the_result_fits',
    ),
    (
        "M11 read-deprecation-labels' unreadable-count names nothing",
        'src/primitives/read_deprecation_labels.rs',
        '        unreadable_count: read.unreadable.len(),',
        '        unreadable_count: 0,',
        'read_deprecation_labels::tests::the_lists_are_inline_while_the_result_fits',
    ),
    (
        "M12 read-deprecation-labels' list file is not marked file form",
        'src/primitives/read_deprecation_labels.rs',
        '        result.form = ListsForm::File;\n',
        '',
        'read_deprecation_labels::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        "M13 read-deprecation-labels' list file holds no labels list",
        'src/primitives/read_deprecation_labels.rs',
        '        "labels": read.labels,\n',
        '',
        'read_deprecation_labels::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        "M14 read-deprecation-labels' list file sits under no scope",
        'src/primitives/read_deprecation_labels.rs',
        'const LISTS_SCOPE: &str = "paths";',
        'const LISTS_SCOPE: &str = "scope";',
        'read_deprecation_labels::tests::the_lists_are_in_a_file_past_the_read_size',
    ),
    (
        "M15 the exec walker restores no list a count names",
        'src/primitives/bounded_lists.rs',
        '                None => key.strip_suffix("-count").map(str::to_string),',
        '                None => None,',
        'interpreter::payload::tests::resolve_anchor_and_read_deprecation_labels_get_their_lists_back',
    ),
]
