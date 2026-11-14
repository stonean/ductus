"""Mutation list for 022's task 33: `create-scenario` creates its file
exclusively, through `write_new_atomic`'s no-clobber persist, so a scenario
another call wrote is never replaced (BE-RACE-003, from 023's review;
`runtime/src/primitives/mod.rs` `write_new_atomic`,
`runtime/src/primitives/create_scenario.rs` `run`).

Run with `python3 scripts/mutation/run.py scripts/mutation/create-scenario-exclusive.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

M2 restores the old exists-then-rename. Its killing test races 16 calls on
one slug, so the mutant's failure is a race the test makes near-certain
rather than one it forces: it failed 5 of 5 runs when the task landed.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 write_new_atomic replaces an existing file',
        'src/primitives/mod.rs',
        '    match tmp.persist_noclobber(path) {',
        '    match tmp.persist(path) {',
        'primitives::tests::write_new_atomic_never_replaces_an_existing_file create_scenario::tests::calls_racing_on_one_slug_create_it_once',
    ),
    (
        'M2 create-scenario checks the slug is free, then writes with a replacing rename',
        'src/primitives/create_scenario.rs',
        '    let body = render(args);\n    if !write_new_atomic(&dest, &body)? {\n        return Err(PrimitiveError::ScenarioConflict { path: dest });\n    }',
        '    if dest.exists() {\n        return Err(PrimitiveError::ScenarioConflict { path: dest });\n    }\n    let body = render(args);\n    crate::primitives::write_atomic(&dest, &body)?;\n    let _ = write_new_atomic;',
        'create_scenario::tests::calls_racing_on_one_slug_create_it_once',
    ),
]
