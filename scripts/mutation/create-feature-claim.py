"""Mutation list for spec 051's atomic feature-directory claim (`BE-RACE-003`):
`create-feature` makes the directory with `create_dir`, so of two processes
racing for one name exactly one creates it and writes its `spec.md`; and the
directory is contained before it is tested or created (`BE-INPUT-004`).

Run with `python3 scripts/mutation/run.py scripts/mutation/create-feature-claim.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

The killing tests are `a_feature_directory_is_claimed_by_exactly_one_racer`,
`a_dangling_link_at_the_derived_name_is_a_lost_claim` (a dangling link inside
the project passes the existence check and loses the claim, the path a race
takes) and `a_spec_root_linked_outside_the_project_is_refused` in
runtime/src/primitives/create_feature.rs.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the directory is made with create_dir_all, which succeeds on a taken name',
        'src/primitives/create_feature.rs',
        '    match std::fs::create_dir(feature_dir) {\n',
        '    match std::fs::create_dir_all(feature_dir) {\n',
        'create_feature',
    ),
    (
        'M2 a taken name is reported as claimed',
        'src/primitives/create_feature.rs',
        '        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => Ok(false),\n',
        '        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => Ok(true),\n',
        'create_feature',
    ),
    (
        'M3 run writes spec.md though its claim was lost',
        'src/primitives/create_feature.rs',
        '    if !claim_feature_dir(&feature_dir)? {\n',
        '    if claim_feature_dir(&feature_dir).map(|_| false)? {\n',
        'create_feature',
    ),
    (
        'M4 the derived directory is not contained before it is created',
        'src/primitives/create_feature.rs',
        '    super::validate_feature_contained(repo, &root, &feature)?;\n',
        '',
        'create_feature',
    ),
]
