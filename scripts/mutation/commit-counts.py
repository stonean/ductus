"""Mutation list for 083's window commit counts and tick attribution:
`compute-review-scope` counts the commits its window walked and those no spec
claimed although they changed a path, before any credit, and reports neither
without a window; a commit holding a task's tick is its spec's to review
whatever its subject, while one holding none stays in every window; and
`write-review`'s Summary states both counts, or that the scope had no window
(`runtime/src/primitives/compute_review_scope.rs`,
`runtime/src/primitives/commit_attribution.rs`,
`runtime/src/primitives/write_review.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/commit-counts.py
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
        'M1 a clean merge counts as unclaimed',
        'src/primitives/compute_review_scope.rs',
        '.filter(|(_, claims)| claims.claimants.is_empty() && !claims.paths.is_empty())',
        '.filter(|(_, claims)| claims.claimants.is_empty())',
        'compute_review_scope::tests::a_clean_merge_is_examined_and_never_unclaimed',
    ),
    (
        'M2 no commit counts as unclaimed',
        'src/primitives/compute_review_scope.rs',
        '.filter(|(_, claims)| claims.claimants.is_empty() && !claims.paths.is_empty())',
        '.filter(|_| false)',
        'compute_review_scope::tests::the_window_counts_its_commits_and_those_no_spec_claimed',
    ),
    (
        'M3 the walk examines no commit',
        'src/primitives/compute_review_scope.rs',
        'examined: commits.len(),',
        'examined: 0,',
        'compute_review_scope::tests::a_window_whose_commits_are_all_claimed_reports_zero_unclaimed',
    ),
    (
        'M4 the walk keeps no counts',
        'src/primitives/compute_review_scope.rs',
        'window.commits = Some(WindowCommits::of(&commits));',
        'window.commits = None;',
        'compute_review_scope::tests::a_window_whose_commits_are_all_claimed_reports_zero_unclaimed',
    ),
    (
        'M5 the result drops commits-examined',
        'src/primitives/compute_review_scope.rs',
        'commits_examined: scope.commits.map(|commits| commits.examined),',
        'commits_examined: None,',
        'compute_review_scope::tests::the_window_counts_its_commits_and_those_no_spec_claimed',
    ),
    (
        'M6 the result reports examined commits as unclaimed',
        'src/primitives/compute_review_scope.rs',
        'commits_unclaimed: scope.commits.map(|commits| commits.unclaimed),',
        'commits_unclaimed: scope.commits.map(|commits| commits.examined),',
        'compute_review_scope::tests::the_window_counts_its_commits_and_those_no_spec_claimed',
    ),
    (
        'M7 a spec with no window reports zero counts',
        'src/primitives/compute_review_scope.rs',
        '    let window = if diff_base.is_empty() {\n        WindowClaims::default()',
        '    let window = if diff_base.is_empty() {\n        WindowClaims {\n            commits: Some(WindowCommits::default()),\n            ..WindowClaims::default()\n        }',
        'compute_review_scope::tests::a_spec_with_no_window_reports_neither_commit_count',
    ),
    (
        'M8 the count leaves out a commit a review credited',
        'src/primitives/compute_review_scope.rs',
        'let records = read_records(project, specs_root, &commits)?;',
        'let records = read_records(project, specs_root, &commits)?;\n'
        '    if let Some(counts) = window.commits.as_mut() {\n'
        '        counts.unclaimed = commits\n'
        '            .iter()\n'
        '            .filter(|(oid, claims)| {\n'
        '                claims.claimants.is_empty()\n'
        '                    && claims\n'
        '                        .paths\n'
        '                        .iter()\n'
        '                        .any(|path| !records.iter().any(|record| record.read(*oid, path)))\n'
        '            })\n'
        '            .count();\n'
        '    }',
        'compute_review_scope::tests::a_credited_unclaimed_commit_is_still_counted',
    ),
    (
        'M9 touching a spec directory claims no commit',
        'src/primitives/commit_attribution.rs',
        '.filter_map(|slug| parse_feature_dir(&slug))',
        '.filter_map(|_slug| None::<FeatureForm>)',
        'compute_review_scope::tests::a_ticked_commit_is_its_specs_whatever_its_subject',
    ),
    (
        'M10 a commit no spec claims leaves every window',
        'src/primitives/compute_review_scope.rs',
        '                self.own.insert(path);',
        '                self.others.insert(path);',
        'compute_review_scope::tests::an_unticked_commit_under_the_same_subject_stays_in_every_window',
    ),
    (
        'M11 a supplied Summary leaves out the commit line',
        'src/primitives/write_review.rs',
        '        let _ = write!(summary, "\\n\\n{}", commits_summary(record.commits));\n',
        '',
        'write_review::tests::a_supplied_summary_carries_the_loops_account',
    ),
    (
        'M12 a generated Summary leaves out the commit line',
        'src/primitives/write_review.rs',
        "    summary.push(' ');\n    summary.push_str(&commits_summary(record.commits));\n",
        '',
        'write_review::tests::the_summary_counts_the_windows_unclaimed_commits',
    ),
    (
        'M13 write-review drops the window counts',
        'src/primitives/write_review.rs',
        '        result.excluded,\n        result.commits,\n    ))',
        '        result.excluded,\n        None,\n    ))',
        'write_review::tests::the_summary_counts_the_windows_unclaimed_commits',
    ),
    (
        'M14 a scope with no window reads as one fully attributed',
        'src/primitives/write_review.rs',
        'None => "The scope had no window, so no commit was examined.".to_string(),',
        'None => "0 of 0 commits in the window no spec claimed.".to_string(),',
        'write_review::tests::the_summary_counts_the_windows_unclaimed_commits',
    ),
    (
        'M15 the Summary swaps the two counts',
        'src/primitives/write_review.rs',
        'commits.unclaimed, commits.examined',
        'commits.examined, commits.unclaimed',
        'write_review::tests::the_summary_counts_the_windows_unclaimed_commits',
    ),
]
