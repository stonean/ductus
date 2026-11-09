"""Mutation list for 080's `merge-permissions` canonical call (task 35;
scenario `configure-types-no-permission-entries`): with `canonical` the call
merges Claude's canonical allow, deny and retired sets, which the runtime
holds, and ensures `permissions.additionalDirectories` holds the spec root and
the project's command directory, appending only a directory missing and
naming each retired entry it removed
(`runtime/src/primitives/merge_permissions.rs`).

Run with `python3 scripts/mutation/run.py
scripts/mutation/merge-permissions-canonical.py [ID ...]`; see run.py for the
format. This file is data, read with ast.literal_eval: keep every value a
literal. An id is never reused: a mutation that no longer applies is removed,
and the next one takes the next free number. When the code a mutation anchors
on changes, update the anchor in the same change; `run.py --check` names every
anchor that no longer matches exactly once.

run.py runs unit tests only; every killing test here is one.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 canonical is ignored',
        'src/primitives/merge_permissions.rs',
        '    let (allow, deny, revoke, directories) = if args.canonical {\n',
        '    let (allow, deny, revoke, directories) = if false {\n',
        'merge_permissions::tests::a_canonical_call_writes_the_canonical_sets_and_both_directories',
    ),
    (
        'M2 the canonical call merges no deny set',
        'src/primitives/merge_permissions.rs',
        '            joined(&to_strings(CANONICAL_DENY), &args.deny),\n',
        '            args.deny.clone(),\n',
        'merge_permissions::tests::a_canonical_call_writes_the_canonical_sets_and_both_directories',
    ),
    (
        'M3 the canonical call retires nothing',
        'src/primitives/merge_permissions.rs',
        '            joined(&to_strings(RETIRED_ALLOW), &args.revoke),\n',
        '            args.revoke.clone(),\n',
        'merge_permissions::tests::a_canonical_call_retires_names_and_keeps_the_adopters_entries',
    ),
    (
        'M4 a fresh file gets no directories',
        'src/primitives/merge_permissions.rs',
        '        post_value["permissions"]["additionalDirectories"] = json!(sets.directories);\n',
        '',
        'merge_permissions::tests::a_canonical_call_writes_the_canonical_sets_and_both_directories',
    ),
    (
        'M5 a directory already held is appended again',
        'src/primitives/merge_permissions.rs',
        '        if !array.iter().any(|held| held == directory.as_str()) {\n',
        '        if true {\n',
        'merge_permissions::tests::a_canonical_call_retires_names_and_keeps_the_adopters_entries',
    ),
    (
        'M6 a missing directory is never appended',
        'src/primitives/merge_permissions.rs',
        '            array.push(Value::String(directory.clone()));\n',
        '',
        'merge_permissions::tests::a_canonical_call_retires_names_and_keeps_the_adopters_entries',
    ),
    (
        'M7 the retired entries removed go unnamed',
        'src/primitives/merge_permissions.rs',
        '                removed.push(entry);\n',
        '',
        'merge_permissions::tests::a_canonical_call_retires_names_and_keeps_the_adopters_entries',
    ),
]
