"""Mutation list for spec 051 task 37 (`BE-TXN-002`): `invalidate-review` and
`rewrite-spec-links` write through the shared compare-before-persist guard
(`ReadSnapshot`, `runtime/src/primitives/mod.rs`; its own mutants are in
`compare-and-set-041.py`). Each mutation makes a write unguarded, or removes
what a refusal reports; one test per primitive kills it.

Run with `python3 scripts/mutation/run.py scripts/mutation/compare-and-set-051.py
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
        'M1 invalidate-review writes the record over one changed since its read',
        'src/primitives/invalidate_review.rs',
        '        self.snapshot.write_if_unchanged(&self.updated)\n',
        '        crate::primitives::write_atomic(&self.snapshot.repo.join(&self.snapshot.rel), &self.updated)\n',
        'invalidate_review::tests::a_record_changed_after_it_was_read_is_refused_and_the_other_writers_bytes_stand',
    ),
    (
        'M2 rewrite-spec-links writes a file over one changed since the sweep read it',
        'src/primitives/rewrite_spec_links.rs',
        '            match file.snapshot.write_if_unchanged(&rewrite.text) {\n',
        '            match crate::primitives::write_atomic(&file.path, &rewrite.text) {\n',
        'rewrite_spec_links::tests::a_file_changed_after_the_sweep_read_it_is_refused_and_the_others_are_written',
    ),
    (
        'M3 rewrite-spec-links stops the sweep at a refused file',
        'src/primitives/rewrite_spec_links.rs',
        '                Err(PrimitiveError::ChangedSinceRead { .. }) => {\n                    self.refused.push(shown.clone());\n',
        '                Err(PrimitiveError::ChangedSinceRead { .. }) if false => {\n                    self.refused.push(shown.clone());\n',
        'rewrite_spec_links::tests::a_file_changed_after_the_sweep_read_it_is_refused_and_the_others_are_written',
    ),
    (
        'M4 rewrite-spec-links does not name a refused file',
        'src/primitives/rewrite_spec_links.rs',
        '                    self.refused.push(shown.clone());\n',
        '                    let _ = shown.clone();\n',
        'rewrite_spec_links::tests::a_file_changed_after_the_sweep_read_it_is_refused_and_the_others_are_written',
    ),
    (
        'M5 rewrite-spec-links counts a refused file as rewritten',
        'src/primitives/rewrite_spec_links.rs',
        '        if !written {\n            return Ok(());\n        }\n',
        '        if false {\n            return Ok(());\n        }\n',
        'rewrite_spec_links::tests::a_file_changed_after_the_sweep_read_it_is_refused_and_the_others_are_written',
    ),
    (
        'M6 rewrite-spec-links lists the mentions of text it did not write',
        'src/primitives/rewrite_spec_links.rs',
        '        let text = if written { &rewrite.text } else { content };\n',
        '        let text = &rewrite.text;\n',
        'rewrite_spec_links::tests::a_file_changed_after_the_sweep_read_it_is_refused_and_the_others_are_written',
    ),
    (
        'M7 rewrite-spec-links leaves a referrer outside the spec root unguarded',
        'src/primitives/rewrite_spec_links.rs',
        '            match file.snapshot.write_if_unchanged(&rewrite.text) {\n',
        '            match if file.path.starts_with(self.specs_dir.as_path()) { file.snapshot.write_if_unchanged(&rewrite.text) } else { crate::primitives::write_atomic(&file.path, &rewrite.text) } {\n',
        'rewrite_spec_links::tests::a_referrer_outside_the_spec_root_changed_after_it_was_read_is_refused',
    ),
    (
        'M8 the rewrite-spec-links result drops its refused files',
        'src/primitives/rewrite_spec_links.rs',
        '        refused_count: sweep.refused.len(),\n        refused: Some(sweep.refused),\n',
        '        refused_count: 0,\n        refused: Some(Vec::new()),\n',
        'rewrite_spec_links::tests::the_result_names_each_refused_file_beside_the_rewritten_ones',
    ),
    (
        'M9 the rewrite-spec-links lists file omits the refused files',
        'src/primitives/rewrite_spec_links.rs',
        '        "refused": sweep.refused,\n',
        '',
        'rewrite_spec_links::tests::past_the_read_size_the_refused_files_move_to_the_lists_file',
    ),
    (
        'M10 the rewrite-spec-links file form keeps the refused files inline',
        'src/primitives/rewrite_spec_links.rs',
        '        result.refused = None;\n',
        '',
        'rewrite_spec_links::tests::past_the_read_size_the_refused_files_move_to_the_lists_file',
    ),
]
