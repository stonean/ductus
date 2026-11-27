"""Mutation list for spec 086's AC1: `write-review` refuses, writing nothing, a
finding or an observation on a file the run's scope does not hold
(`refuse_outcomes_outside_the_scope` in `runtime/src/primitives/write_review.rs`),
testing the scope the record counts through `RunScope::of`
(`runtime/src/primitives/compute_review_scope.rs`), for a spec's record and the
project's (`runtime/src/primitives/write_review/project_record.rs`), with an
empty `diff-base` refusing every anchored outcome.

Run with `python3 scripts/mutation/run.py scripts/mutation/outcomes-outside-the-scope-086.py
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
        'M1 an outcome on a file outside the scope is recorded',
        'src/primitives/write_review.rs',
        '        .find(|(_, file)| !scope.holds(file))',
        '        .find(|_| false)',
        'primitives::write_review::tests::an_outcome_on_a_file_outside_the_scope_is_refused_writing_nothing',
    ),
    (
        'M2 every outcome naming a file is refused, the scope\'s own included',
        'src/primitives/write_review.rs',
        '        .find(|(_, file)| !scope.holds(file))',
        '        .find(|(_, file)| { let _ = scope; !file.is_empty() })',
        'primitives::write_review::tests::an_outcome_the_scope_holds_is_accepted primitives::write_review::tests::a_re_reads_finding_on_a_file_a_fix_touched_is_accepted_at_the_last_write',
    ),
    (
        'M3 an observation\'s :line suffix is kept, so one in the scope is refused',
        'src/primitives/write_review.rs',
        '                format!("observations[{idx}].path"),\n                super::path_without_lines(&observation.path),',
        '                format!("observations[{idx}].path"),\n                observation.path.as_str(),',
        'primitives::write_review::tests::an_outcome_the_scope_holds_is_accepted',
    ),
    (
        'M4 an observation naming no file is refused',
        'src/primitives/write_review.rs',
        '        .filter(|(_, file)| !file.is_empty());',
        '        .filter(|_| true);',
        'primitives::write_review::tests::an_outcome_the_scope_holds_is_accepted',
    ),
    (
        'M5 a plan pattern among the scope\'s entries names no file',
        'src/primitives/compute_review_scope.rs',
        '        Self::Known {\n            plan: PlanEntries::new(&planned),\n            read,\n        }',
        '        let _ = planned;\n        Self::Known {\n            plan: PlanEntries::new(&[]),\n            read,\n        }',
        'primitives::write_review::tests::an_outcome_the_scope_holds_is_accepted',
    ),
    (
        'M6 an empty diff-base records an anchored outcome',
        'src/primitives/write_review.rs',
        '        refuse_outcomes_outside_the_scope(args, &RunScope::of([], &[]), true)?;\n',
        '',
        'primitives::write_review::tests::an_empty_diff_base_refuses_an_anchored_outcome_naming_the_base_to_pass',
    ),
    (
        'M7 an empty diff-base\'s refusal does not name the base to pass',
        'src/primitives/write_review.rs',
        '    let how = if empty_base {',
        '    let how = if !empty_base {',
        'primitives::write_review::tests::an_empty_diff_base_refuses_an_anchored_outcome_naming_the_base_to_pass',
    ),
    (
        'M8 a spec\'s record with a window records an outcome outside its scope',
        'src/primitives/write_review.rs',
        '    refuse_outcomes_outside_the_scope(args, &run_scope, false)?;\n',
        '    let _ = run_scope;\n',
        'primitives::write_review::tests::an_outcome_on_a_file_outside_the_scope_is_refused_writing_nothing primitives::write_review::departure_waivers::a_violation_outside_the_review_scope_is_refused_and_one_inside_is_waived primitives::write_review::departure_waivers::the_rule_firing_in_a_covered_file_outside_the_scope_is_refused',
    ),
    (
        'M9 the project record records an outcome outside its scope',
        'src/primitives/write_review.rs',
        '    refuse_outcomes_outside_the_scope(\n        args,\n        &RunScope::of(scope.scope.iter().cloned(), &[]),\n        false,\n    )?;\n',
        '',
        'primitives::write_review::project_record::a_project_review_refuses_an_outcome_outside_its_scope',
    ),
]
