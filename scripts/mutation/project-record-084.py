"""Mutation list for 084's project record: `write-review` with no `feature`
and its seed (`runtime/src/primitives/write_review.rs`, `write_review/seed.rs`),
the window and scope of the code no spec claims (`check_unreviewed_code.rs`,
`commit_attribution.rs`), the project subject of a fix round (`fix_rounds.rs`,
AC14) and of `process-waivers` and `process-decisions`.

Run with `python3 scripts/mutation/run.py scripts/mutation/project-record-084.py
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
        'M1 a commit no spec claims gives the project nothing',
        'src/primitives/commit_attribution.rs',
        '    if claims.is_unclaimed() {\n        (claims.paths, Vec::new())',
        '    if false {\n        (claims.paths, Vec::new())',
        'commit_attribution::tests::the_projects_share_is_the_commits_no_spec_claims',
    ),
    (
        'M2 a commit a spec claims is the project\'s too',
        'src/primitives/commit_attribution.rs',
        '    if claims.is_unclaimed() {\n        (claims.paths, Vec::new())',
        '    if true {\n        (claims.paths, Vec::new())',
        'commit_attribution::tests::the_projects_share_is_the_commits_no_spec_claims',
    ),
    (
        'M3 a tree\'s attributor lists the specs on disk',
        'src/primitives/check_unreviewed_code.rs',
        'let attributor = Attributor::at_tree(project, repo, specs_root, &tree)?;\n    let mut walk',
        'let attributor = Attributor::new(project, repo, specs_root)?;\n    let mut walk',
        'check_unreviewed_code::tests::an_uncommitted_spec_directory_claims_no_commit',
    ),
    (
        'M4 a file under the spec root named as a spec is one',
        'src/primitives/commit_attribution.rs',
        '                .filter(|entry| entry.kind() == Some(git2::ObjectType::Tree))\n',
        '',
        'commit_attribution::tests::a_trees_attributor_lists_the_specs_the_tree_holds',
    ),
    (
        'M5 a project round\'s share is a spec\'s, not its inverse',
        'src/primitives/fix_rounds.rs',
        '            None => project_share(claims),',
        '            None => {\n                let (mine, left) = project_share(claims);\n                (left, mine)\n            }',
        '--test fix_rounds a_project_reviews_rounds_leave_out_its_record_and_what_a_spec_claims',
    ),
    (
        'M6 a project round keeps the project record',
        'src/primitives/fix_rounds.rs',
        'let record = ReviewSubject::of(subject.feature).record_rel(subject.specs_root);',
        'let record = ReviewSubject::Feature(subject.feature.unwrap_or("none")).record_rel(subject.specs_root);',
        '--test fix_rounds a_project_reviews_rounds_leave_out_its_record_and_what_a_spec_claims',
    ),
    (
        'M7 an empty feature names the project',
        'src/primitives/review_subject.rs',
        'feature.map_or(Self::Project, Self::Feature)',
        'feature.filter(|name| !name.is_empty()).map_or(Self::Project, Self::Feature)',
        'review_subject::tests::only_an_absent_feature_is_the_project write_review::project_record::an_empty_feature_is_refused_and_is_not_the_project',
    ),
    (
        'M8 a record\'s window starts at each commit holding it, not the one that added it',
        'src/primitives/check_unreviewed_code.rs',
        'starts.insert(oid, carried.unwrap_or(oid));',
        'starts.insert(oid, oid);',
        'check_unreviewed_code::tests::a_seeds_window_starts_after_the_commit_that_added_it',
    ),
    (
        'M9 the walk does not stop at the window\'s start',
        'src/primitives/check_unreviewed_code.rs',
        '    walk.hide(start)?;\n',
        '    let _ = start;\n',
        'check_unreviewed_code::tests::a_seeds_window_starts_after_the_commit_that_added_it',
    ),
    (
        'M10 a finished review\'s window starts before the commit finishing it',
        'src/primitives/check_unreviewed_code.rs',
        '            start: Oid::from_str(&commit)?,',
        '            start: repository.find_commit(Oid::from_str(&commit)?)?.parent_id(0)?,',
        'check_unreviewed_code::tests::a_finished_reviews_window_starts_after_the_commit_finishing_it',
    ),
    (
        'M11 a held review\'s window starts at the commit read from',
        'src/primitives/check_unreviewed_code.rs',
        '            start: held.against,',
        '            start: at.id(),',
        'check_unreviewed_code::tests::a_held_reviews_window_starts_at_its_reviewed_against',
    ),
    (
        'M12 what a record leaves open is not held in the scope',
        'src/primitives/check_unreviewed_code.rs',
        '    scope.extend(handed_on);',
        '    let _ = handed_on.count();',
        'check_unreviewed_code::tests::a_held_reviews_window_starts_at_its_reviewed_against',
    ),
    (
        'M13 a commit with no record places a window at itself',
        'src/primitives/check_unreviewed_code.rs',
        '            None => Err(NotPlaced::NoRecord),',
        '            None => Ok(ProjectWindow {\n                start: at.id(),\n                carried: Vec::new(),\n            }),',
        'check_unreviewed_code::tests::a_commit_with_no_project_record_places_no_window',
    ),
    (
        'M14 a shallow cut at the record places a window',
        'src/primitives/check_unreviewed_code.rs',
        '                if repository.is_shallow()\n',
        '                if false\n                    && repository.is_shallow()\n',
        'check_unreviewed_code::tests::a_shallow_clone_cut_at_the_seed_places_no_window',
    ),
    (
        'M15 an artifact is code no spec claims',
        'src/primitives/check_unreviewed_code.rs',
        '        .filter(|path| !is_project_artifact(path, specs_root))\n',
        '',
        'check_unreviewed_code::tests::the_scope_holds_only_committed_code_no_spec_claims',
    ),
    (
        'M16 a path gone from the tree stays in the scope',
        'src/primitives/check_unreviewed_code.rs',
        '            if holds(project, &tree, &path) {\n                code.paths.insert(path);',
        '            if true {\n                code.paths.insert(path);',
        'check_unreviewed_code::tests::the_scope_holds_only_committed_code_no_spec_claims',
    ),
    (
        'M17 a named start is ignored',
        'src/primitives/check_unreviewed_code.rs',
        '    let window = match since {',
        '    let window = match since.filter(|_| false) {',
        'check_unreviewed_code::tests::a_named_start_is_used_verbatim',
    ),
    (
        'M18 the project record digests contracts it does not have',
        'src/primitives/write_review.rs',
        '    let contracts = subject.feature().map(|_| {',
        '    let contracts = Some(()).map(|()| {',
        'write_review::project_record::a_project_review_records_the_unclaimed_commit_in_the_spec_roots_record',
    ),
    (
        'M19 criteria supplied for the project are dropped',
        'src/primitives/write_review.rs',
        '        ("criteria", args.criteria.is_some()),',
        '        ("criteria", false),',
        'write_review::project_record::a_project_review_refuses_what_only_a_spec_has_and_an_empty_base',
    ),
    (
        'M20 a scenario supplied for the project is recorded',
        'src/primitives/write_review.rs',
        '        ("scenario", args.scenario.is_some()),',
        '        ("scenario", false),',
        'write_review::project_record::a_project_review_refuses_what_only_a_spec_has_and_an_empty_base',
    ),
    (
        'M21 an unread entry outside the project scope is recorded',
        'src/primitives/write_review.rs',
        '    refuse_unread_outside_the_scope(args, &scope.scope)?;\n',
        '',
        'write_review::project_record::a_project_review_refuses_what_only_a_spec_has_and_an_empty_base',
    ),
    (
        'M22 a project scope that moved is recorded',
        'src/primitives/write_review.rs',
        '    moved_scope::refuse_a_moved_scope(&args.reviewed_against, repo, &scope.scope)?;\n',
        '',
        'write_review::project_record::a_project_review_refuses_a_scope_that_moved_after_it_was_read',
    ),
    (
        'M23 no deferral releases a project finding',
        'src/primitives/decisions.rs',
        '.is_some_and(|other| Some(other) != feature)',
        '.is_some_and(|other| feature.is_some_and(|own| own != other))',
        'write_review::project_record::a_deferral_to_a_specs_pending_task_releases_a_project_finding',
    ),
    (
        'M24 a seed overwrites the record it finds',
        'src/primitives/write_review/seed.rs',
        '    let outcome = if record.bytes().is_some() {',
        '    let outcome = if false {',
        'write_review::project_record::seed_leaves_an_existing_record_as_it_is',
    ),
    (
        'M25 a seed takes other arguments and drops them',
        'src/primitives/write_review/seed.rs',
        '    if *args != alone {',
        '    if false {',
        'write_review::project_record::seed_refuses_any_other_argument',
    ),
    (
        'M26 a seed states no reviewed-against',
        'src/primitives/write_review/seed.rs',
        '    if let Some(head) = head {\n        let _ = writeln!(text, "reviewed-against: {head}");',
        '    if let Some(head) = head.filter(|_| false) {\n        let _ = writeln!(text, "reviewed-against: {head}");',
        'write_review::project_record::seed_writes_a_record_no_review_wrote',
    ),
    (
        'M27 a seed fails in a repository with no commit',
        'src/primitives/write_review/seed.rs',
        '        Err(err) if err.code() == git2::ErrorCode::UnbornBranch => Ok(None),\n',
        '',
        'write_review::project_record::seed_on_an_unborn_head_states_no_reviewed_against',
    ),
    (
        'M28 the project record\'s waivers are read against no scope',
        'src/primitives/compute_review_scope.rs',
        '                    read: scope.scope,\n                    plan: PlanEntries::new(&[]),',
        '                    read: BTreeSet::new(),\n                    plan: PlanEntries::new(&[]),',
        'process_waivers::tests::the_project_records_waivers_are_classified_with_no_feature',
    ),
    (
        'M29 a departure waiver in the project record reads a spec',
        'src/primitives/process_waivers.rs',
        'let spec_path = subject.feature().map(|_| record_dir.join("spec.md"));',
        'let spec_path = Some(record_dir.join("spec.md")).filter(|_| subject.name() != "");',
        'process_waivers::tests::a_departure_waiver_in_the_project_record_expires',
    ),
    (
        'M30 the analysis record is read for the project',
        'src/primitives/process_decisions.rs',
        'DecisionRecord::Analysis if args.feature.is_none() => Err(PrimitiveError::InvalidArgument {',
        'DecisionRecord::Analysis if false => Err(PrimitiveError::InvalidArgument {',
        'process_decisions::tests::the_project_records_decisions_are_classified_with_no_feature',
    ),
    (
        'M31 the project record is not contained',
        'src/primitives/review_subject.rs',
        '                super::validate_contained(repo, &subject.record_rel(specs_root))?;\n',
        '',
        'review_subject::tests::the_project_record_is_contained_and_an_absent_one_resolves',
    ),
]
