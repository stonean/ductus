"""Mutation list for spec 041 task 25: `prune-project`'s opens are contained
(BE-INPUT-004). Each mutant removes one containment site of
`runtime/src/primitives/prune_project.rs`, or of the `ReadSnapshot` read and
write it goes through (`runtime/src/primitives/mod.rs`), and its test builds a
symbolic link leaving the project.

Run with `python3 scripts/mutation/run.py scripts/mutation/prune-project-containment.py
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
        'M1 a project file is read through a link leaving the project',
        'src/primitives/mod.rs',
        '        validate_contained(repo, rel)?;\n        Ok(Self {',
        '        Ok(Self {',
        'prune_project::tests::a_linked_agents_md_is_outside_project_and_an_apply_writes_nothing prune_project::tests::a_linked_spec_root_makes_its_files_outside_project_and_refuses_a_create',
    ),
    (
        'M2 a rewrite writes through a link leaving the project',
        'src/primitives/mod.rs',
        '        hook(Stage::BeforeReread);\n        validate_contained(&self.repo, &self.rel)?;\n',
        '        hook(Stage::BeforeReread);\n',
        'prune_project::tests::a_file_swapped_for_an_outward_link_after_its_read_is_refused',
    ),
    (
        'M3 a home to create is not checked before the apply writes',
        'src/primitives/prune_project.rs',
        '        for kind in &args.create {\n            validate_contained(repo, &home(&root, *kind).0)?;\n        }\n',
        '',
        'prune_project::tests::a_refused_create_leaves_no_half_an_apply_on_disk prune_project::tests::a_linked_spec_root_makes_its_files_outside_project_and_refuses_a_create',
    ),
    (
        'M4 the search reads a tracked file through a link leaving the project',
        'src/primitives/prune_project.rs',
        '            match validate_contained(repo, path) {\n                Ok(()) => {}\n                Err(PrimitiveError::InvalidPath { .. }) => {',
        '            match Ok::<(), PrimitiveError>(()) {\n                Ok(()) => {}\n                Err(PrimitiveError::InvalidPath { .. }) => {',
        'prune_project::tests::the_search_names_a_tracked_link_out_and_does_not_read_it',
    ),
    (
        'M5 the outside-project list stays in the result past the read size',
        'src/primitives/prune_project.rs',
        '            search.outside_project = None;\n',
        '',
        'prune_project::tests::outside_project_moves_to_the_lists_file_with_its_count_inline',
    ),
    (
        'M6 the lists file omits the outside-project list',
        'src/primitives/prune_project.rs',
        '        lists["outside-project"] = serde_json::json!(outside);\n',
        '',
        'prune_project::tests::outside_project_moves_to_the_lists_file_with_its_count_inline',
    ),
]
