"""Mutation list for 079's decisions from history and from a diff: whether an edit
reopens a spec (`runtime/src/primitives/classify_spec_edit.rs`), and the hints
that follow it (`check_artifact_size.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/reopen-measure.py
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
        'M1 a run\'s record counts as an edit',
        'src/primitives/classify_spec_edit.rs',
        '        if file == REVIEW_RECORD_FILE || file == ANALYSIS_RECORD_FILE {\n            continue;\n        }\n',
        '',
        'classify_spec_edit::tests::a_record_write_alone_is_none',
    ),
    (
        'M2 only the review record is skipped',
        'src/primitives/classify_spec_edit.rs',
        'if file == REVIEW_RECORD_FILE || file == ANALYSIS_RECORD_FILE {',
        'if file == REVIEW_RECORD_FILE {',
        'classify_spec_edit::tests::a_record_write_alone_is_none',
    ),
    (
        'M3 only the analysis record is skipped',
        'src/primitives/classify_spec_edit.rs',
        'if file == REVIEW_RECORD_FILE || file == ANALYSIS_RECORD_FILE {',
        'if file == ANALYSIS_RECORD_FILE {',
        'classify_spec_edit::tests::a_record_write_alone_is_none',
    ),
    (
        'M4 a deleted plan or tasks is a correction',
        'src/primitives/classify_spec_edit.rs',
        '"plan.md" | "tasks.md" if deleted => {',
        '"plan.md" | "tasks.md" if deleted && false => {',
        'classify_spec_edit::tests::a_deleted_plan_or_tasks_reopens',
    ),
    (
        'M5 a data-model edit is a correction',
        'src/primitives/classify_spec_edit.rs',
        '"data-model.md" => {\n                reasons.insert(SpecEditReason::DataModel);\n            }',
        '"data-model.md" => {}',
        'classify_spec_edit::tests::a_data_model_edit_reopens',
    ),
    (
        'M6 a scenario edit is a correction',
        'src/primitives/classify_spec_edit.rs',
        'other if is_review_contract(other) => {\n                reasons.insert(SpecEditReason::Scenario);\n            }',
        'other if is_review_contract(other) => {}',
        'classify_spec_edit::tests::a_scenario_edit_reopens',
    ),
    (
        'M7 another file is a correction',
        'src/primitives/classify_spec_edit.rs',
        '_ => {\n                reasons.insert(SpecEditReason::OtherFile);\n            }',
        '_ => {}',
        'classify_spec_edit::tests::another_file_reopens',
    ),
    (
        'M8 an edit with no reason reads as nothing',
        'src/primitives/classify_spec_edit.rs',
        '} else if edited {\n        SpecEditKind::Correction',
        '} else if edited && false {\n        SpecEditKind::Correction',
        'classify_spec_edit::tests::spec_body_prose_is_a_correction',
    ),
    (
        'M9 reasons reopen nothing',
        'src/primitives/classify_spec_edit.rs',
        'let kind = if !reasons.is_empty() {',
        'let kind = if reasons.len() > 99 {',
        'classify_spec_edit::tests::a_changed_criterion_reopens',
    ),
    (
        'M10 an untracked file is not seen',
        'src/primitives/classify_spec_edit.rs',
        '        .include_untracked(true)\n        .recurse_untracked_dirs(true);',
        '        .include_untracked(false);',
        'classify_spec_edit::tests::an_untracked_scenario_reopens',
    ),
    (
        'M11 a new scenario directory is not walked',
        'src/primitives/classify_spec_edit.rs',
        '        .recurse_untracked_dirs(true);',
        '        .recurse_untracked_dirs(false);',
        'classify_spec_edit::tests::an_untracked_scenario_in_a_new_directory_reopens',
    ),
    (
        'M12 a deletion is not told apart',
        'src/primitives/classify_spec_edit.rs',
        'files.insert(file.to_string(), delta.status() == Delta::Deleted);',
        'files.insert(file.to_string(), false && delta.status() == Delta::Deleted);',
        'classify_spec_edit::tests::a_deleted_plan_or_tasks_reopens',
    ),
    (
        'M13 the project path is not converted to the repository\'s',
        'src/primitives/classify_spec_edit.rs',
        '.pathspec(project.to_git(rel_dir))',
        '.pathspec(rel_dir)',
        'classify_spec_edit::tests::a_project_in_a_subdirectory_reads_its_own_paths',
    ),
    (
        'M14 a base blob is read at the project\'s path',
        'src/primitives/classify_spec_edit.rs',
        'let git_path = self.project.to_git(&format!("{}/{file}", self.rel_dir));',
        'let git_path = format!("{}/{file}", self.rel_dir);',
        'classify_spec_edit::tests::a_project_in_a_subdirectory_reads_its_own_paths',
    ),
    (
        'M15 a spec absent at the base or deleted is a correction',
        'src/primitives/classify_spec_edit.rs',
        '        reasons.insert(SpecEditReason::OtherFile);\n        return Ok(());',
        '        return Ok(());',
        'classify_spec_edit::tests::a_deleted_spec_reopens',
    ),
    (
        'M16 a deleted spec is read rather than named',
        'src/primitives/classify_spec_edit.rs',
        'let (Some(at_base), false) = (at_base, deleted) else {',
        'let (Some(at_base), false) = (at_base, false) else {',
        'classify_spec_edit::tests::a_deleted_spec_reopens',
    ),
    (
        'M17 criteria are not compared',
        'src/primitives/classify_spec_edit.rs',
        'if criterion_texts(base_body) != criterion_texts(now_body) {',
        'if false && criterion_texts(base_body) != criterion_texts(now_body) {',
        'classify_spec_edit::tests::a_changed_criterion_reopens',
    ),
    (
        'M18 questions are not compared',
        'src/primitives/classify_spec_edit.rs',
        'if question_texts(base_body) != question_texts(now_body) {',
        'if false && question_texts(base_body) != question_texts(now_body) {',
        'classify_spec_edit::tests::an_added_question_reopens',
    ),
    (
        'M19 a criterion\'s label is compared',
        'src/primitives/classify_spec_edit.rs',
        'unlabelled.map_or_else(|| criterion.text.clone(), str::to_string)',
        'unlabelled.map_or_else(|| criterion.text.clone(), |_| criterion.text.clone())',
        'classify_spec_edit::tests::labelling_is_a_correction',
    ),
    (
        'M20 a moved criterion reads as changed',
        'src/primitives/classify_spec_edit.rs',
        '        .collect();\n    texts.sort();\n    texts\n}\n\n/// What the open questions say, sorted.',
        '        .collect();\n    texts\n}\n\n/// What the open questions say, sorted.',
        'classify_spec_edit::tests::moving_a_criterion_is_a_correction',
    ),
    (
        'M21 a moved question reads as changed',
        'src/primitives/classify_spec_edit.rs',
        '        .map(|question| question.text)\n        .collect();\n    texts.sort();',
        '        .map(|question| question.text)\n        .collect();',
        'classify_spec_edit::tests::moving_an_open_question_is_a_correction',
    ),
    (
        'M22 an unchecked item is judged by presence, not count',
        'src/primitives/classify_spec_edit.rs',
        '.any(|(item, count)| at_base.get(item).copied().unwrap_or(0) < *count)',
        '.any(|(item, _)| !at_base.contains_key(item))',
        'classify_spec_edit::tests::a_repeated_unchecked_item_reopens',
    ),
    (
        'M23 any difference in unchecked items reopens, so ticking and pruning do',
        'src/primitives/classify_spec_edit.rs',
        '    if now\n        .iter()\n        .any(|(item, count)| at_base.get(item).copied().unwrap_or(0) < *count)\n    {',
        '    if now != at_base {',
        'classify_spec_edit::tests::ticking_renumbering_or_pruning_a_pending_task_is_a_correction',
    ),
    (
        'M24 a checked item counts as unchecked',
        'src/primitives/classify_spec_edit.rs',
        '.filter(|subtask| !subtask.checked)',
        '.filter(|_| true)',
        'classify_spec_edit::tests::a_task_edit_adding_a_checked_item_is_a_correction',
    ),
    (
        'M25 a heading is no part of an item',
        'src/primitives/classify_spec_edit.rs',
        '*items.entry((task.heading.clone(), text)).or_insert(0) += 1;',
        '*items.entry((String::new(), text)).or_insert(0) += 1;',
        'classify_spec_edit::tests::a_reworded_pending_task_reopens',
    ),
    (
        'M26 an unchecked clause counts for nothing',
        'src/primitives/classify_spec_edit.rs',
        '.filter(|_| task.done_when_checked == Some(false));',
        '.filter(|_| false);',
        'classify_spec_edit::tests::a_new_unchecked_clause_reopens',
    ),
    (
        'M27 a bold-form clause counts as an unchecked box',
        'src/primitives/classify_spec_edit.rs',
        '.filter(|_| task.done_when_checked == Some(false));',
        '.filter(|_| task.done_when_checked != Some(true));',
        'classify_spec_edit::tests::a_task_edit_adding_no_unchecked_task_is_a_correction',
    ),
    (
        'M28 the base defaults to HEAD\'s parent',
        'src/primitives/classify_spec_edit.rs',
        'const DEFAULT_BASE: &str = "HEAD";',
        'const DEFAULT_BASE: &str = "HEAD~1";',
        'classify_spec_edit::tests::base_decides_where_the_comparison_starts',
    ),
    (
        'M29 the base argument is ignored',
        'src/primitives/classify_spec_edit.rs',
        '.revparse_single(args.base.as_deref().unwrap_or(DEFAULT_BASE))?',
        '.revparse_single(DEFAULT_BASE)?',
        'classify_spec_edit::tests::base_decides_where_the_comparison_starts',
    ),
    (
        'M30 the feature argument is not screened',
        'src/primitives/classify_spec_edit.rs',
        '    super::validate_feature_argument(repo, &layout.specs_root, &args.feature)?;\n',
        '',
        'classify_spec_edit::tests::a_feature_that_is_absent_or_escapes_is_refused',
    ),
    (
        'M31 an absent feature reads as nothing changed',
        'src/primitives/classify_spec_edit.rs',
        'if !repo.join(&rel_dir).is_dir() {',
        'if false && !repo.join(&rel_dir).is_dir() {',
        'classify_spec_edit::tests::a_feature_that_is_absent_or_escapes_is_refused',
    ),
    (
        'M32 a file keeps its leading slash, so spec.md is another file',
        'src/primitives/classify_spec_edit.rs',
        'let prefix = format!("{rel_dir}/");',
        'let prefix = format!("{rel_dir}");',
        'classify_spec_edit::tests::spec_body_prose_is_a_correction',
    ),
    (
        'M33 a data-model trim follows the requirement rule',
        'src/primitives/check_artifact_size.rs',
        '(_, FixKind::Split | FixKind::Promote) | (ArtifactKind::DataModel, FixKind::Trim) => {',
        '(_, FixKind::Split | FixKind::Promote) => {',
        'check_artifact_size::tests::a_data_model_trim_reopens_a_done_spec',
    ),
    (
        'M34 a spec trim always reopens',
        'src/primitives/check_artifact_size.rs',
        '(_, FixKind::Trim) => OnDone::IfRequirementChanges,',
        '(_, FixKind::Trim) => OnDone::Always,',
        'check_artifact_size::tests::the_effect_on_done_is_named_only_at_done',
    ),
]
