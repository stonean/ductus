"""Mutation list for spec 086: the one definition of which files belong to a spec
(`runtime/src/primitives/spec_ownership.rs`): each one its plan's Affected Files
name, and each one a commit attributed to it changed, divided by `SpecShare::split`
(`runtime/src/primitives/commit_attribution.rs`), in one walk of `HEAD`'s history
for any number of specs, pre-filtered by `Attributor::may_claim`, with a shallow
history and an unrecognized directory not examined.

Run with `python3 scripts/mutation/run.py scripts/mutation/spec-ownership-086.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a mutation
that no longer applies is removed, and the next one takes the next free number.
When the code a mutation anchors on changes, update the anchor in the same
change; `run.py --check` names every anchor that no longer matches exactly
once.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 a file a plan entry names does not belong',
        'src/primitives/spec_ownership.rs',
        '        self.plan.names(path) || self.changed.contains(path)',
        '        self.changed.contains(path)',
        'primitives::spec_ownership::tests::a_file_a_plan_entry_names_belongs',
    ),
    (
        'M2 a file a commit attributed to the spec changed does not belong',
        'src/primitives/spec_ownership.rs',
        '        self.plan.names(path) || self.changed.contains(path)',
        '        self.plan.names(path)',
        'primitives::spec_ownership::tests::a_file_a_commit_attributed_to_the_spec_changed_belongs',
    ),
    (
        'M3 a commit the spec claims gives it every path, another claimant\'s share included',
        'src/primitives/spec_ownership.rs',
        '                let (own, _) = shares[index].split(claims.clone())?;',
        '                let own = claims.paths.clone();\n                let _ = &mut shares;',
        'primitives::spec_ownership::tests::a_file_only_an_unclaimed_commit_or_another_specs_share_changed_does_not_belong',
    ),
    (
        'M4 a merge claims what it changed itself',
        'src/primitives/commit_attribution.rs',
        [
            '        if commit.parent_count() > 1 {\n            return Ok(CommitClaims {',
            '        if commit.parent_count() > 1 {\n            return Ok(Vec::new());',
        ],
        [
            '        if false {\n            return Ok(CommitClaims {',
            '        if false {\n            return Ok(Vec::new());',
        ],
        'primitives::spec_ownership::tests::a_merge_touching_the_specs_directory_gives_it_nothing',
    ),
    (
        'M5 the walk pre-filters by belongs_to, which a rename-only commit passes by',
        'src/primitives/spec_ownership.rs',
        '        let may = attributor.may_claim(&commit)?;\n        if !asked.iter().any(|(_, form, _)| may.contains(form)) {',
        '        let mut admitted = false;\n        for (_, form, _) in asked {\n            admitted = admitted || attributor.belongs_to(&commit, form)?;\n        }\n        if !admitted {',
        'primitives::spec_ownership::tests::a_commit_only_renaming_the_specs_directory_gives_it_the_new_paths',
    ),
    (
        'M6 a shallow history is examined',
        'src/primitives/spec_ownership.rs',
        '    if repository.is_shallow() {',
        '    if false {',
        'primitives::spec_ownership::tests::a_shallow_history_and_an_unrecognized_directory_are_not_examined',
    ),
    (
        'M7 an unrecognized directory is examined and owns nothing',
        'src/primitives/spec_ownership.rs',
        '    let Some(form) = parse_feature_dir(feature) else {\n        return Ok(unrecognized(feature));\n    };',
        '    let Some(form) = parse_feature_dir(feature) else {\n        let _ = unrecognized;\n        return Ok(Ownership::Examined(Owned {\n            plan: PlanEntries::new(&[]),\n            changed: BTreeSet::new(),\n        }));\n    };',
        'primitives::spec_ownership::tests::a_shallow_history_and_an_unrecognized_directory_are_not_examined',
    ),
    (
        'M8 one walk over several specs gives every spec\'s paths to the first',
        'src/primitives/spec_ownership.rs',
        '                changed[index].extend(own);',
        '                changed[0].extend(own);',
        'primitives::spec_ownership::tests::one_walk_over_two_specs_gives_each_its_own_answer',
    ),
    (
        'M9 a repository with no commit is an error, not a spec owning its plan alone',
        'src/primitives/spec_ownership.rs',
        '        Err(err) if err.code() == git2::ErrorCode::UnbornBranch => return Ok(None),\n',
        '',
        'primitives::spec_ownership::tests::a_file_a_plan_entry_names_belongs',
    ),
]
