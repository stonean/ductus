"""Mutation list for spec 051's atomic feature-directory claim (`BE-RACE-003`):
`create-feature` makes the directory with `create_dir`, so of two processes
racing for one name exactly one creates it and writes its `spec.md`.

Run with `python3 scripts/mutation/run.py scripts/mutation/create-feature-claim.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

The killing test is `a_feature_directory_is_claimed_by_exactly_one_racer` in
runtime/src/primitives/create_feature.rs.
"""

EXPECTED_SURVIVORS = {
    'M3': "run's use of a lost claim is reached only when another process creates the directory between run's existence check and its claim, a race no portable test can make happen on demand; the claim itself is killed by M1 and M2",
}

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
]
