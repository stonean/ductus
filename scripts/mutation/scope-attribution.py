"""Mutation list for 079's commit attribution, window and scope: a commit belongs
to each spec whose directory it touches and each spec its subject's scope names
(`runtime/src/primitives/commit_attribution.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/scope-attribution.py
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
        'M1 touching a spec directory claims nothing',
        'src/primitives/commit_attribution.rs',
        '            .filter_map(|slug| parse_feature_dir(&slug))\n',
        '            .filter_map(|slug| parse_feature_dir(&slug))\n            .filter(|_| false)\n',
        'commit_attribution::tests::a_directory_touch_claims_with_no_subject_scope',
    ),
    (
        'M2 naming a spec in the subject claims nothing',
        'src/primitives/commit_attribution.rs',
        'let named: Vec<FeatureForm> = subject_scope_forms(subject)',
        'let named: Vec<FeatureForm> = subject_scope_forms(&subject[..0])',
        'commit_attribution::tests::a_subject_scope_claims_with_no_directory_touch',
    ),
    (
        'M3 the last-named spec takes the unplanned path',
        'src/primitives/commit_attribution.rs',
        'let first_named = named.first().cloned();',
        'let first_named = named.last().cloned();',
        'commit_attribution::tests::a_subject_naming_several_specs_keeps_the_first_named',
    ),
    (
        'M4 a merge is attributed like any commit',
        'src/primitives/commit_attribution.rs',
        'if commit.parent_count() > 1 {',
        'if commit.parent_count() > 99 {',
        'commit_attribution::tests::a_clean_merge_claims_nothing_and_contributes_no_path',
    ),
    (
        'M5 a number no spec has claims',
        'src/primitives/commit_attribution.rs',
        '.filter(|form| self.known.contains(form) || touched.contains(form))',
        '.filter(|_| true)',
        'commit_attribution::tests::a_number_no_spec_has_claims_nothing_and_the_commit_counts_whole',
    ),
    (
        'M6 a retired spec named by a commit touching its directory is not first named',
        'src/primitives/commit_attribution.rs',
        '.filter(|form| self.known.contains(form) || touched.contains(form))',
        '.filter(|form| self.known.contains(form))',
        'commit_attribution::tests::a_retired_specs_directory_still_claims_though_its_number_alone_would_not',
    ),
    (
        "M7 a merge's paths are its first-parent diff",
        'src/primitives/commit_attribution.rs',
        'paths.retain(|path| others.binary_search(path).is_ok());',
        'let _ = &others;',
        'commit_attribution::tests::a_clean_merge_claims_nothing_and_contributes_no_path commit_attribution::tests::a_merge_contributes_only_what_it_changed_itself',
    ),
    (
        'M8 a root commit contributes no path',
        'src/primitives/commit_attribution.rs',
        'return self.paths_from(None, &tree);',
        'return Ok(Vec::new());',
        'commit_attribution::tests::a_root_commit_diffs_against_nothing',
    ),
    (
        'M9 a spec both named and touched is claimed twice',
        'src/primitives/commit_attribution.rs',
        'if !claimants.contains(&form) {',
        'if claimants.len() < 99 {',
        'commit_attribution::tests::a_spec_both_named_and_touched_is_claimed_once',
    ),
    (
        'M10 a touched directory precedes the named spec',
        'src/primitives/commit_attribution.rs',
        'for form in named.into_iter().chain(touched) {',
        'for form in touched.into_iter().chain(named) {',
        'commit_attribution::tests::a_scope_and_a_directory_touch_each_claim_and_the_named_spec_stays_first',
    ),
    (
        'M11 a scope holds one token only',
        'src/primitives/commit_attribution.rs',
        "    scope\n        .split(',')\n",
        "    scope\n        .split(';')\n",
        'commit_attribution::tests::a_scope_token_is_a_number_a_branch_id_or_a_directory_and_nothing_else',
    ),
    (
        "M12 a scope token's trailing words spoil it",
        'src/primitives/commit_attribution.rs',
        '.filter_map(|token| token.split_whitespace().next())',
        '.map(str::trim)',
        'commit_attribution::tests::a_scope_token_is_a_number_a_branch_id_or_a_directory_and_nothing_else',
    ),
    (
        'M13 a bare number is not a spec',
        'src/primitives/commit_attribution.rs',
        'parse_feature_dir(&format!("{token}-x"))',
        'parse_feature_dir(token)',
        'commit_attribution::tests::a_scope_token_is_a_number_a_branch_id_or_a_directory_and_nothing_else',
    ),
    (
        'M14 words before the parenthesis still carry a scope',
        'src/primitives/commit_attribution.rs',
        'if kind.is_empty() || kind.contains(char::is_whitespace) {',
        'if kind.is_empty() {',
        'commit_attribution::tests::a_scope_token_is_a_number_a_branch_id_or_a_directory_and_nothing_else',
    ),
    (
        'M15 a subject opening with its scope still carries one',
        'src/primitives/commit_attribution.rs',
        'if kind.is_empty() || kind.contains(char::is_whitespace) {',
        'if kind.contains(char::is_whitespace) {',
        'commit_attribution::tests::a_scope_token_is_a_number_a_branch_id_or_a_directory_and_nothing_else',
    ),
    (
        "M16 a subdirectory project's paths are read as the work tree names them",
        'src/primitives/compute_review_scope.rs',
        '&& let Some(inside) = project.to_project(&path.to_string_lossy())\n            {\n                files.insert(inside.to_string());',
        '&& let Some(inside) = Some(path.to_string_lossy())\n            {\n                files.insert(inside.to_string());',
        'commit_attribution::tests::a_project_in_a_subdirectory_claims_only_its_own_spec_directories',
    ),
]
