"""Mutation list for spec 086's AC6: `compute-review-scope`
(`runtime/src/primitives/compute_review_scope.rs`), for a spec whose record at
`HEAD` names no run, puts in the scope each file a commit attributed to the
spec changed that is still on disk and inside the project, taking it out of
`excluded`, beside each plan entry (`first_review_reads`), asking
`spec_ownership::of_spec_or_why` (`runtime/src/primitives/spec_ownership.rs`);
a history it cannot read whole adds no such file, and the result's
`ownership` says so, absent on a later review.

Run with `python3 scripts/mutation/run.py scripts/mutation/first-review-scope-086.py
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
        "M1 a first review adds no file the spec's commits changed",
        'src/primitives/compute_review_scope.rs',
        '        hold_in_scope(owned.changed().iter(), repo, scope, excluded);',
        '        let _ = (owned, repo, &excluded);',
        'primitives::compute_review_scope::tests::a_first_review_reads_each_file_the_specs_commits_changed primitives::compute_review_scope::tests::no_in_progress_and_no_since_yields_an_empty_window primitives::compute_review_scope::tests::a_record_with_no_run_is_a_first_review_and_owes_one',
    ),
    (
        'M2 an owned file another spec also changed stays excluded',
        'src/primitives/compute_review_scope.rs',
        '        hold_in_scope(owned.changed().iter(), repo, scope, excluded);',
        '        scope.extend(owned.changed().iter().filter(|path| repo.join(path).symlink_metadata().is_ok() && !excluded.contains(*path)).cloned());',
        'primitives::compute_review_scope::tests::a_first_review_reads_each_file_the_specs_commits_changed primitives::compute_review_scope::tests::a_first_review_reads_an_owned_file_another_open_plan_holds',
    ),
    (
        'M3 an owned file gone from disk joins the scope',
        'src/primitives/compute_review_scope.rs',
        '        hold_in_scope(owned.changed().iter(), repo, scope, excluded);',
        '        hold_in_scope(owned.changed().iter(), repo, scope, excluded);\n        scope.extend(owned.changed().iter().cloned());',
        'primitives::compute_review_scope::tests::a_first_review_reads_each_file_the_specs_commits_changed',
    ),
    (
        'M4 the scope reports no ownership',
        'src/primitives/compute_review_scope.rs',
        '        ownership: ownership.as_ref().map(|(state, _)| *state),',
        '        ownership: None,',
        'primitives::compute_review_scope::tests::a_first_review_reads_each_file_the_specs_commits_changed primitives::compute_review_scope::tests::a_cut_history_gives_a_first_review_its_plan_alone_and_says_so',
    ),
    (
        'M5 the result drops the scope\'s ownership',
        'src/primitives/compute_review_scope.rs',
        '        ownership: scope.ownership,',
        '        ownership: None,',
        'primitives::compute_review_scope::tests::a_first_review_reads_each_file_the_specs_commits_changed',
    ),
    (
        'M6 a history not examined gives no reason',
        'src/primitives/compute_review_scope.rs',
        '        ownership_reason: ownership.and_then(|(_, reason)| reason),',
        '        ownership_reason: ownership.and(None),',
        'primitives::compute_review_scope::tests::a_cut_history_gives_a_first_review_its_plan_alone_and_says_so',
    ),
    (
        'M7 a later review reports ownership',
        'src/primitives/compute_review_scope.rs',
        '            (&mut scope, &mut excluded),\n        ))\n    } else {\n        None\n    };',
        '            (&mut scope, &mut excluded),\n        ))\n    } else {\n        Some((OwnershipState::Examined, None))\n    };',
        'primitives::compute_review_scope::tests::a_first_review_reads_each_file_the_specs_commits_changed',
    ),
    (
        'M8 an ownership not examined reads as examined',
        'src/primitives/spec_ownership.rs',
        '            Self::NotExamined { reason } => (OwnershipState::NotExamined, Some(reason.clone())),',
        '            Self::NotExamined { .. } => (OwnershipState::Examined, None),',
        'primitives::compute_review_scope::tests::a_cut_history_gives_a_first_review_its_plan_alone_and_says_so primitives::process_decisions::tests::a_shallow_history_drops_nothing_and_says_why',
    ),
]
