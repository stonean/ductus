"""Mutation list for spec 067 task 36 (`BE-TXN-002`): `derive-dependencies` and
`derive-references` write each rewritten `spec.md` through the shared
compare-before-persist guard, `ReadSnapshot` in `runtime/src/primitives/mod.rs`
(its own mutants are in `compare-and-set-041.py`). Each mutation makes a
derivation write without the guard, stop at a refused spec, count one as
updated, or lose it from the bounded result; the unit tests beside each
derivation kill them.

The CLI half lives in `src/main.rs`, which `cargo test --lib` does not build:
`derive_dependencies_blocks` and `derive_references_blocks` reading
`refused-count`, and `report_refused` naming each refused spec on stderr from
the whole derivation. Each was proven by hand to fail the bin tests
`a_refused_spec_blocks_both_derivations_in_either_form` and
`each_refused_spec_gets_one_line_naming_it_and_saying_to_run_again` with
`cargo test --bin ductus -- refused` on 2026-11-16: dropping `refused_count`
from either gate, and shortening the stderr line. The `report_refused` calls in
`main` are not killable by any test, since no subprocess can induce a
refusal, which needs a change between the read and the write.

Run with `python3 scripts/mutation/run.py scripts/mutation/compare-and-set-067.py
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
        "M1 derive-dependencies writes a spec over another writer's change",
        'src/primitives/derive_dependencies.rs',
        '            match snapshot.write_if_unchanged(&rewritten) {\n',
        '            match crate::primitives::write_atomic(&path, &rewritten) {\n',
        'derive_dependencies::tests::a_spec_changed_after_it_was_read_is_refused_and_the_rest_is_written',
    ),
    (
        'M2 derive-dependencies stops writing at the first refused spec',
        'src/primitives/derive_dependencies.rs',
        '                    refused.push(spec.clone());\n                    continue;\n',
        '                    refused.push(spec.clone());\n                    break;\n',
        'derive_dependencies::tests::a_spec_changed_after_it_was_read_is_refused_and_the_rest_is_written',
    ),
    (
        'M3 derive-dependencies counts a refused spec as updated',
        'src/primitives/derive_dependencies.rs',
        '                    refused.push(spec.clone());\n                    continue;\n',
        '                    refused.push(spec.clone());\n',
        'derive_dependencies::tests::a_spec_changed_after_it_was_read_is_refused_and_the_rest_is_written',
    ),
    (
        'M4 derive-dependencies leaves a refused spec out of the lists file',
        'src/primitives/derive_dependencies.rs',
        '        "refused": derived.refused,\n    });\n',
        '    });\n',
        'derive_dependencies::tests::a_refused_spec_is_listed_inline_and_moves_to_the_list_file_past_the_read_size',
    ),
    (
        'M5 derive-dependencies keeps `refused` inline past the read size',
        'src/primitives/derive_dependencies.rs',
        '        result.refused = None;\n',
        '',
        'derive_dependencies::tests::a_refused_spec_is_listed_inline_and_moves_to_the_list_file_past_the_read_size',
    ),
    (
        'M6 derive-dependencies reports a refused count of zero',
        'src/primitives/derive_dependencies.rs',
        '        refused_count: derived.refused.len(),\n',
        '        refused_count: 0,\n',
        'derive_dependencies::tests::a_refused_spec_is_listed_inline_and_moves_to_the_list_file_past_the_read_size',
    ),
    (
        "M7 derive-references writes a spec over another writer's change",
        'src/primitives/derive_references.rs',
        '            match snapshot.write_if_unchanged(&rewritten) {\n',
        '            match crate::primitives::write_atomic(&path, &rewritten) {\n',
        'derive_references::tests::a_spec_changed_after_it_was_read_is_refused_and_the_rest_is_written',
    ),
    (
        'M8 derive-references stops writing at the first refused spec',
        'src/primitives/derive_references.rs',
        '                    refused.push(spec.clone());\n                    continue;\n',
        '                    refused.push(spec.clone());\n                    break;\n',
        'derive_references::tests::a_spec_changed_after_it_was_read_is_refused_and_the_rest_is_written',
    ),
    (
        'M9 derive-references counts a refused spec as updated',
        'src/primitives/derive_references.rs',
        '                    refused.push(spec.clone());\n                    continue;\n',
        '                    refused.push(spec.clone());\n',
        'derive_references::tests::a_spec_changed_after_it_was_read_is_refused_and_the_rest_is_written',
    ),
    (
        'M10 derive-references leaves a refused spec out of the lists file',
        'src/primitives/derive_references.rs',
        '        "refused": derived.refused,\n',
        '',
        'derive_references::tests::a_refused_spec_is_listed_inline_and_moves_to_the_list_file_past_the_read_size',
    ),
    (
        'M11 derive-references keeps `refused` inline past the read size',
        'src/primitives/derive_references.rs',
        '        result.refused = None;\n',
        '',
        'derive_references::tests::a_refused_spec_is_listed_inline_and_moves_to_the_list_file_past_the_read_size',
    ),
    (
        'M12 derive-references reports a refused count of zero',
        'src/primitives/derive_references.rs',
        '        refused_count: derived.refused.len(),\n',
        '        refused_count: 0,\n',
        'derive_references::tests::a_refused_spec_is_listed_inline_and_moves_to_the_list_file_past_the_read_size',
    ),
    (
        'M13 derive-dependencies refuses every spec, changed or not',
        'src/primitives/derive_dependencies.rs',
        '            match snapshot.write_if_unchanged(&rewritten) {\n',
        '            match Err::<(), PrimitiveError>(PrimitiveError::ChangedSinceRead { path: spec.into() }) {\n',
        'derive_dependencies::tests::an_unchanged_spec_is_written_and_none_is_refused',
    ),
    (
        'M14 derive-references refuses every spec, changed or not',
        'src/primitives/derive_references.rs',
        '            match snapshot.write_if_unchanged(&rewritten) {\n',
        '            match Err::<(), PrimitiveError>(PrimitiveError::ChangedSinceRead { path: spec.into() }) {\n',
        'derive_references::tests::an_unchanged_spec_is_written_and_none_is_refused',
    ),
]
