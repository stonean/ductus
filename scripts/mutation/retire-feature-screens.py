"""Mutation list for 052's 2026-11-14 review fixes to `retire-feature`: both
arguments pass the feature-name screen, so a name holding a path separator is
refused with `allow-sequential` too (BE-INPUT-002), and a feature that is its
own fold target is refused (`runtime/src/primitives/retire_feature.rs` `run`).

Run with `python3 scripts/mutation/run.py scripts/mutation/retire-feature-screens.py
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
        'M1 retire-feature screens neither argument as a feature name',
        'src/primitives/retire_feature.rs',
        '    super::validate_session_feature("retire-feature", "feature", &args.feature)?;\n    super::validate_session_feature("retire-feature", "fold-target", &args.fold_target)?;\n',
        '',
        'retire_feature::tests::a_name_holding_a_path_separator_is_refused_with_allow_sequential',
    ),
    (
        'M2 retire-feature removes a feature that is its own fold target',
        'src/primitives/retire_feature.rs',
        '    if args.feature == args.fold_target {',
        '    if false {',
        'retire_feature::tests::a_feature_that_is_its_own_fold_target_is_refused',
    ),
]
