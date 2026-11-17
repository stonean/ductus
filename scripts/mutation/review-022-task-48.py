"""Mutation list for spec 022 task 48 (from task 46's implementation): the
spec-root listing is contained before it is listed (`BE-INPUT-004`), refused
where the root is the call's subject and read as unread where the call keeps
running. M1 and M3-M10 remove the containment inside `list_feature_dirs`,
each held by the test of one caller class; M2 and M11-M16 make one caller
swallow the refusal instead of propagating it.

`create-feature` is held by the central mutants only through the listing's own
test: its call refuses an outward root a step later in
`validate_feature_contained`, so a caller-level mutant there is equivalent.

Run with `python3 scripts/mutation/run.py scripts/mutation/review-022-task-48.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the feature listing lists a spec root that is a link out of the project',
        'src/primitives/mod.rs',
        '    validate_contained(repo, specs_root)?;\n    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        '    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        'primitives::spec_corpus_tests::the_feature_listing_refuses',
    ),
    (
        'M2 resolve-feature reads an outward spec root as one that matched nothing',
        'src/primitives/resolve_feature.rs',
        '    let features = list_feature_dirs(repo, &root)?;',
        '    let features = list_feature_dirs(repo, &root).unwrap_or_default();',
        'primitives::resolve_feature::tests::a_spec_root_linked_outside_the_project',
    ),
    (
        'M3 resolve-feature lists a spec root that is a link out of the project',
        'src/primitives/mod.rs',
        '    validate_contained(repo, specs_root)?;\n    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        '    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        'primitives::resolve_feature::tests::a_spec_root_linked_outside_the_project',
    ),
    (
        'M4 an attributor lists a spec root that is a link out of the project',
        'src/primitives/mod.rs',
        '    validate_contained(repo, specs_root)?;\n    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        '    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        'primitives::commit_attribution::tests::an_attributor_refuses',
    ),
    (
        'M5 prune-plan walks a spec root that is a link out of the project',
        'src/primitives/mod.rs',
        '    validate_contained(repo, specs_root)?;\n    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        '    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        'primitives::prune_plan::tests::a_walk_of_a_spec_root',
    ),
    (
        'M6 prune-tasks walks a spec root that is a link out of the project',
        'src/primitives/mod.rs',
        '    validate_contained(repo, specs_root)?;\n    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        '    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        'primitives::prune_tasks::tests::a_walk_of_a_spec_root',
    ),
    (
        'M7 the dashboard lists a spec root that is a link out of the project',
        'src/primitives/mod.rs',
        '    validate_contained(repo, specs_root)?;\n    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        '    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        'primitives::dashboard::tests::a_spec_root_linked_outside',
    ),
    (
        'M8 derive-routing-candidates offers specs from a spec root that is a link out of the project',
        'src/primitives/mod.rs',
        '    validate_contained(repo, specs_root)?;\n    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        '    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        'primitives::derive_routing_candidates::tests::a_spec_root_linked_outside',
    ),
    (
        'M9 the available-specs payload lists a spec root that is a link out of the project',
        'src/primitives/mod.rs',
        '    validate_contained(repo, specs_root)?;\n    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        '    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        'interpreter::payload::tests::available_specs_read_nothing_outside_the_project',
    ),
    (
        'M10 check-unfolded-specs lists a spec root that is a link out of the project',
        'src/primitives/mod.rs',
        '    validate_contained(repo, specs_root)?;\n    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        '    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        'primitives::check_unfolded_specs::tests::a_spec_root_that_leaves_the_project_halts_unlisted',
    ),
    (
        'M11 the worktree spec listing swallows the refusal of an outward spec root',
        'src/primitives/mod.rs',
        '    for feature in list_feature_dirs(repo, specs_root)? {',
        '    for feature in list_feature_dirs(repo, specs_root).unwrap_or_default() {',
        'primitives::spec_corpus_tests::the_worktree_walk_contains',
    ),
    (
        'M12 an attributor swallows the refusal of an outward spec root',
        'src/primitives/commit_attribution.rs',
        '        let known = list_feature_dirs(repo, specs_root)?\n',
        '        let known = list_feature_dirs(repo, specs_root).unwrap_or_default()\n',
        'primitives::commit_attribution::tests::an_attributor_refuses',
    ),
    (
        'M13 prune-plan swallows the refusal of an outward spec root',
        'src/primitives/prune_plan.rs',
        '    let names = list_feature_dirs(repo, root)?;',
        '    let names = list_feature_dirs(repo, root).unwrap_or_default();',
        'primitives::prune_plan::tests::a_walk_of_a_spec_root',
    ),
    (
        'M14 prune-tasks swallows the refusal of an outward spec root',
        'src/primitives/prune_tasks.rs',
        '    let names = list_feature_dirs(repo, root)?;',
        '    let names = list_feature_dirs(repo, root).unwrap_or_default();',
        'primitives::prune_tasks::tests::a_walk_of_a_spec_root',
    ),
    (
        'M15 the dashboard swallows the refusal of an outward spec root',
        'src/primitives/dashboard.rs',
        '    for slug in list_feature_dirs(repo, &layout.specs_root)? {',
        '    for slug in list_feature_dirs(repo, &layout.specs_root).unwrap_or_default() {',
        'primitives::dashboard::tests::a_spec_root_linked_outside',
    ),
    (
        'M16 the available-specs payload swallows the refusal of an outward spec root',
        'src/interpreter/payload.rs',
        '    let features = refuse_outside(\n        crate::primitives::list_feature_dirs(repo, &specs_root),\n        &specs_root,\n    )?;',
        '    let features = crate::primitives::list_feature_dirs(repo, &specs_root).unwrap_or_default();',
        'interpreter::payload::tests::available_specs_read_nothing_outside_the_project',
    ),
]
