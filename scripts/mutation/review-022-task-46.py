"""Mutation list for spec 022 task 46 (from 022's review of 2026-11-17): the
project-file reads the 66-row grep of task 42 missed are contained
(`BE-INPUT-004`), and the interpreter tests' `feature_argument_cases` table
holds every caller of a `validate_feature_*` helper that takes a feature from
a call's arguments. Each mutation removes one containment check or one site's
check, and the tests named in its filter kill it.

Run with `python3 scripts/mutation/run.py scripts/mutation/review-022-task-46.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 resolve-anchor reads an installer that is a link out of the project',
        'src/primitives/resolve_anchor.rs',
        '    let Ok(path) = contained_path(repo, BOOTSTRAP) else {\n        return HashMap::new();\n    };\n    let Ok(text) = read_text(&path) else {',
        '    let path = repo.join(BOOTSTRAP);\n    let Ok(text) = read_text(&path) else {',
        'primitives::resolve_anchor::tests::an_installer_that_is_a_link_out',
    ),
    (
        'M2 the worktree spec listing lists a spec root that is a link out of the project',
        'src/primitives/mod.rs',
        '    validate_contained(repo, specs_root)?;\n    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        '    let Ok(entries) = std::fs::read_dir(repo.join(specs_root)) else {',
        'primitives::spec_corpus_tests::the_worktree_walk_contains',
    ),
    (
        'M3 the worktree spec listing probes a feature that is a link out of the project',
        'src/primitives/mod.rs',
        '            if leaves_project(repo, &rel) || specs_dir.join(&feature).join(name).is_file() {',
        '            if specs_dir.join(&feature).join(name).is_file() {',
        'primitives::spec_corpus_tests::the_worktree_walk_contains',
    ),
    (
        'M4 the spec-file binding probes a context path that leaves the project',
        'src/interpreter/mod.rs',
        '    if !primitives::leaves_project(repo, path) && repo.join(path).is_dir() {',
        '    if repo.join(path).is_dir() {',
        'interpreter::tests::a_context_path_that_leaves_the_project_is_not_probed',
    ),
    (
        'M5 a session peek probes an own file that is a link out of the project',
        'src/session.rs',
        '    contained(repo, &own)?;\n    let _lock = lock_if_shared(repo)?;',
        '    let _lock = lock_if_shared(repo)?;',
        'session::tests::a_peek_refuses_an_outward_own_link',
    ),
    (
        'M6 the exec seed probes an own session file that is a link out of the project',
        'src/main.rs',
        '    ductus::session::contained(repo, &own)?;\n    Ok(own.exists())',
        '    Ok(own.exists())',
        '--bin ductus tests::an_exec_seed_refuses_an_own_record_that_is_a_link_out',
    ),
    (
        'M7 the four fix-round primitives skip the feature check',
        'src/primitives/fix_rounds.rs',
        '    super::validate_feature_argument(repo, specs_root, feature)?;\n    if !repo.join(specs_root)',
        '    if !repo.join(specs_root)',
        'interpreter::tests::every_feature_argument_site_refuses_an_outward_feature_link',
    ),
    (
        'M9 discover-rule-files skips the feature check of its verdicts',
        'src/primitives/discover_rule_files.rs',
        '    super::validate_feature_argument(repo, &root, feature)?;\n    let feature_dir = repo.join(&root).join(feature);',
        '    let feature_dir = repo.join(&root).join(feature);',
        'interpreter::tests::every_feature_argument_site_refuses_an_outward_feature_link',
    ),
    (
        'M10 create-feature skips the check of the directory it creates',
        'src/primitives/create_feature.rs',
        '    super::validate_feature_contained(repo, &root, &feature)?;\n',
        '',
        'interpreter::tests::every_feature_argument_site_refuses_an_outward_feature_link',
    ),
    (
        'M11 dashboard skips the check of each directory it lists',
        'src/primitives/dashboard.rs',
        '        super::validate_feature_argument(repo, &layout.specs_root, &slug)?;\n',
        '',
        'interpreter::tests::every_feature_argument_site_refuses_an_outward_feature_link',
    ),
    (
        'M12 dashboard checks each directory it lists without walking it',
        'src/primitives/dashboard.rs',
        '        super::validate_feature_argument(repo, &layout.specs_root, &slug)?;',
        '        super::validate_feature_contained(repo, &layout.specs_root, &slug)?;',
        'interpreter::tests::every_feature_argument_site_walks_the_feature_directory',
    ),
    (
        'M13 rewrite-spec-links skips the walk of a scenario target\'s feature',
        'src/primitives/rewrite_spec_links.rs',
        '    super::validate_feature_argument(repo, &layout.specs_root, &target.feature)?;\n',
        '',
        'interpreter::tests::every_feature_argument_site_walks_the_feature_directory',
    ),
]
