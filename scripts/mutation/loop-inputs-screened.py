"""Mutation list for 078's fix-round loop inputs, each screened before the
run writes anything: `start-fix-rounds`, `brief-re-read`, `read-re-read` and
`check-prescriptions` refuse a `feature` outside the runtime's feature-name
allowlist (`BE-INPUT-002`; `runtime/src/primitives/fix_rounds.rs`,
`validate_feature`), so `.` cannot empty every feature's rounds, and
`start-fix-rounds` reads the `[review] re-reads-off` switch, so a malformed
one halts the run before its first fix (AC14).

Run with `python3 scripts/mutation/run.py scripts/mutation/loop-inputs-screened.py
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
        'M1 the loop screens a feature by its denylist alone',
        'src/primitives/fix_rounds.rs',
        '    super::validate_session_feature(primitive, "feature", feature)?;\n',
        '',
        'start_fix_rounds::tests::a_name_outside_the_allowlist_clears_nothing brief_re_read::tests::a_name_outside_the_allowlist_is_refused read_re_read::tests::a_name_outside_the_allowlist_is_refused check_prescriptions::tests::a_name_outside_the_allowlist_is_refused',
    ),
    (
        'M2 start-fix-rounds skips the allowlist',
        'src/primitives/start_fix_rounds.rs',
        '    fix_rounds::validate_feature("start-fix-rounds", repo, &layout.specs_root, &args.feature)?;',
        '    super::validate_feature_argument(repo, &layout.specs_root, &args.feature)?;',
        'start_fix_rounds::tests::a_name_outside_the_allowlist_clears_nothing',
    ),
    (
        'M3 brief-re-read skips the allowlist',
        'src/primitives/brief_re_read.rs',
        '    fix_rounds::validate_feature("brief-re-read", repo, &layout.specs_root, &args.feature)?;',
        '    super::validate_feature_argument(repo, &layout.specs_root, &args.feature)?;',
        'brief_re_read::tests::a_name_outside_the_allowlist_is_refused',
    ),
    (
        'M4 read-re-read skips the allowlist',
        'src/primitives/read_re_read.rs',
        '    fix_rounds::validate_feature("read-re-read", repo, &layout.specs_root, &args.feature)?;',
        '    super::validate_feature_argument(repo, &layout.specs_root, &args.feature)?;',
        'read_re_read::tests::a_name_outside_the_allowlist_is_refused',
    ),
    (
        'M5 check-prescriptions skips the allowlist',
        'src/primitives/check_prescriptions.rs',
        '    fix_rounds::validate_feature(\n        "check-prescriptions",\n        repo,\n        &layout.specs_root,\n        &args.feature,\n    )?;',
        '    super::validate_feature_argument(repo, &layout.specs_root, &args.feature)?;',
        'check_prescriptions::tests::a_name_outside_the_allowlist_is_refused',
    ),
    (
        'M6 start-fix-rounds no longer reads the switch',
        'src/primitives/start_fix_rounds.rs',
        '    fix_rounds::re_reads_off(repo)?;\n',
        '',
        'start_fix_rounds::tests::a_malformed_switch_halts_before_anything_is_cleared',
    ),
    (
        'M7 an empty re-reads-off halts the run instead of running the loop',
        'src/primitives/fix_rounds.rs',
        '    if length == 0 {\n        return Ok(None);\n    }\n',
        '',
        'fix_rounds::tests::an_empty_value_counts_as_absent',
    ),
]
