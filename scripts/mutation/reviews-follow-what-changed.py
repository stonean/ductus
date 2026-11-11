"""Mutation list for spec 081: the project's artifacts owe no review
(`is_project_artifact` in `runtime/src/primitives/mod.rs`, read by
`compute_review_scope.rs`'s owed rule), and a commit whose subject names a spec
has its code reviewed by the specs it names (`CommitClaims::named` in
`commit_attribution.rs`, `attribute_window` in `compute_review_scope.rs`),
while a commit naming no spec keeps 079's attribution.

Each mutation removes the behavior one new or rewritten unit test names, and
its filter selects that test. The owed-rule and attribution mutations 079's
list already holds are in `scripts/mutation/scope-attribution.py`.

Run with `python3 scripts/mutation/run.py
scripts/mutation/reviews-follow-what-changed.py [ID ...]`; see run.py for the
format. This file is data, read with ast.literal_eval: keep every value a
literal. An id is never reused: a mutation that no longer applies is removed,
and the next one takes the next free number. When the code a mutation anchors
on changes, update the anchor in the same change; `run.py --check` names
every anchor that no longer matches exactly once.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    # -- the artifact set -------------------------------------------------
    (
        'M1 the spec root is code',
        'src/primitives/mod.rs',
        '    under(specs_root) || under(".ductus") || matches!(path, "AGENTS.md" | "CLAUDE.md")',
        '    under(".ductus") || matches!(path, "AGENTS.md" | "CLAUDE.md")',
        'project_artifact_tests compute_review_scope::tests::a_reopening_that_changed_code_owes_a_review_and_a_contract_edit_does_not compute_review_scope::tests::a_path_under_the_spec_root_that_is_in_no_spec_owes_no_review compute_review_scope::tests::a_path_in_another_specs_directory_owes_no_review',
    ),
    (
        'M2 .ductus/ is code',
        'src/primitives/mod.rs',
        '    under(specs_root) || under(".ductus") || matches!(path, "AGENTS.md" | "CLAUDE.md")',
        '    under(specs_root) || matches!(path, "AGENTS.md" | "CLAUDE.md")',
        'project_artifact_tests compute_review_scope::tests::an_artifact_outside_the_spec_root_owes_no_review_and_code_does',
    ),
    (
        'M3 the agent files are code',
        'src/primitives/mod.rs',
        '    under(specs_root) || under(".ductus") || matches!(path, "AGENTS.md" | "CLAUDE.md")',
        '    under(specs_root) || under(".ductus")',
        'project_artifact_tests compute_review_scope::tests::an_artifact_outside_the_spec_root_owes_no_review_and_code_does compute_review_scope::tests::a_subdirectory_project_reads_its_artifacts_from_the_project_root',
    ),
    (
        'M4 the spec root is always specs',
        'src/primitives/mod.rs',
        '    under(specs_root) || under(".ductus")',
        '    under("specs") || under(".ductus")',
        'project_artifact_tests compute_review_scope::tests::a_configured_spec_root_is_the_artifact_root',
    ),
    (
        'M5 a root matches without its separator',
        'src/primitives/mod.rs',
        '            .is_some_and(|rest| rest.starts_with(\'/\'))',
        '            .is_some()',
        'project_artifact_tests',
    ),
    (
        'M6 an artifact at any depth matches the agent files',
        'src/primitives/mod.rs',
        'matches!(path, "AGENTS.md" | "CLAUDE.md")',
        '(path.ends_with("AGENTS.md") || path.ends_with("CLAUDE.md"))',
        'project_artifact_tests compute_review_scope::tests::an_artifact_outside_the_spec_root_owes_no_review_and_code_does',
    ),
    # -- the owed rule ----------------------------------------------------
    (
        'M7 a scenario edit owes a review',
        'src/primitives/compute_review_scope.rs',
        '.filter(|path| !is_project_artifact(path, specs_root))',
        '.filter(|path| !is_project_artifact(path, specs_root) || path.contains("/scenarios/"))',
        'compute_review_scope::tests::a_reopening_that_changed_code_owes_a_review_and_a_contract_edit_does_not',
    ),
    (
        'M8 a stale digest holds the record again',
        'src/primitives/compute_review_scope.rs',
        'let review_owed = run.is_none_or(holds_done_until_reviewed) || !owed_by.is_empty();',
        'let review_owed = run.is_none_or(|record| holds_done_until_reviewed(record) || matches!(review_freshness(repo, &rel_dir, Some(record)), RecordFreshness::Stale { .. })) || !owed_by.is_empty();',
        'compute_review_scope::tests::a_stale_digest_puts_its_contract_in_scope_and_owes_no_review compute_review_scope::tests::a_stale_digest_whose_contract_is_gone_owes_no_review compute_review_scope::tests::a_contract_changed_before_the_window_joins_the_scope',
    ),
    # -- attribution of a named commit --------------------------------------
    (
        'M9 the subject names no spec',
        'src/primitives/commit_attribution.rs',
        '            named,\n            paths,',
        '            named: Vec::new(),\n            paths,',
        'commit_attribution::tests',
    ),
    (
        'M10 a named commit is attributed as one naming none',
        'src/primitives/compute_review_scope.rs',
        'let by_subject = if claims.named.is_empty() {',
        'let by_subject = if true {',
        'compute_review_scope::tests::a_named_commits_code_goes_to_every_named_spec_whatever_the_plans_list compute_review_scope::tests::a_spec_claiming_a_named_commit_only_by_its_directory_reviews_only_that_directory',
    ),
    (
        'M11 a spec claiming a named commit by its directory takes its code',
        'src/primitives/compute_review_scope.rs',
        '                named_here && !anothers\n',
        '                !anothers\n',
        'compute_review_scope::tests::a_spec_claiming_a_named_commit_only_by_its_directory_reviews_only_that_directory',
    ),
    (
        "M12 a named spec takes another claimant's directory",
        'src/primitives/compute_review_scope.rs',
        '                named_here && !anothers\n',
        '                named_here\n',
        'compute_review_scope::tests::a_subject_naming_none_leaves_each_claimants_directory_to_it compute_review_scope::tests::a_path_in_another_specs_directory_owes_no_review',
    ),
    (
        'M13 only the first named spec takes the code',
        'src/primitives/compute_review_scope.rs',
        'let named_here = claims.named.contains(&form);',
        'let named_here = claims.named.first() == Some(&form);',
        'compute_review_scope::tests::a_named_commits_code_goes_to_every_spec_it_names compute_review_scope::tests::a_named_commits_code_goes_to_every_named_spec_whatever_the_plans_list',
    ),
    # -- a commit naming no spec keeps 079's rule ---------------------------
    (
        'M14 a commit naming no spec and touching none is no one\'s',
        'src/primitives/compute_review_scope.rs',
        '            window.own.extend(claims.paths);\n            continue;',
        '            for path in claims.paths {\n                window.others.entry(path).or_default().insert(String::from("no spec"));\n            }\n            continue;',
        'compute_review_scope::tests::a_code_commit_naming_no_spec_owes_the_review_of_each_window_holding_it',
    ),
    # -- the gate reads the owed rule ------------------------------------------
    (
        'M15 a contract edit owes a review at the gate',
        'src/primitives/compute_review_scope.rs',
        '.filter(|path| !is_project_artifact(path, specs_root))',
        '.filter(|path| !is_project_artifact(path, specs_root) || path.contains("/scenarios/"))',
        'check_review_gate::tests::a_contract_change_since_the_review_holds_done_only_on_the_analysis',
    ),
    (
        'M16 a code change owes no review at the gate',
        'src/primitives/compute_review_scope.rs',
        '.filter(|path| !is_project_artifact(path, specs_root))',
        '.filter(|_| false)',
        'check_review_gate::tests::a_contract_change_since_the_review_holds_done_only_on_the_analysis',
    ),
]
