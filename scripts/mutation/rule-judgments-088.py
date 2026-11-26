"""Mutation list for spec 088: the counts `write-analysis` records of the
rules its run judged and carried (`runtime/src/primitives/write_analysis.rs`,
task 1), and a changed rule file judged rule by rule with the rules naming a
changed one (`rule_verdicts.rs`, `discover_rule_files.rs` and the exec walker's
`payload.rs`, task 2), and the backfill of `section-digest`
(`backfill_section_digest.rs` and its CLI exit in `main.rs`, task 3). Each
behavior is mutated once and killed by the test that names it.

Run with `python3 scripts/mutation/run.py scripts/mutation/rule-judgments-088.py
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
        'M1 a rule the run judged is not counted judged',
        'src/primitives/write_analysis.rs',
        '            fields.judged += 1;\n',
        '',
        'primitives::write_analysis::tests::a_run_carrying_one_verdict_and_judging_one_rule_records_both_counts',
    ),
    (
        'M2 a rule whose verdict stood is not counted carried',
        'src/primitives/write_analysis.rs',
        '                fields.carried += 1;\n',
        '',
        'primitives::write_analysis::tests::a_run_carrying_one_verdict_and_judging_one_rule_records_both_counts',
    ),
    (
        'M3 a judged rule left unjudged counts as judged',
        'src/primitives/write_analysis.rs',
        '        if fields.unjudged.len() == unjudged_before {\n            fields.judged += 1;',
        '        if true {\n            fields.judged += 1;',
        'primitives::write_analysis::tests::an_unresolved_addressed_verdict_counts_in_neither',
    ),
    (
        'M4 a rule left unjudged without a judgment counts as carried',
        'src/primitives/write_analysis.rs',
        '            if fields.unjudged.len() == unjudged_before {\n                fields.carried += 1;',
        '            if true {\n                fields.carried += 1;',
        'primitives::write_analysis::tests::the_counts_and_the_unjudged_rules_partition_a_mixed_run',
    ),
    (
        'M5 the judged count is written only when non-zero',
        'src/primitives/write_analysis.rs',
        '    let _ = writeln!(out, "rules-judged: {}", rules.judged);\n',
        '    if rules.judged > 0 {\n        let _ = writeln!(out, "rules-judged: {}", rules.judged);\n    }\n',
        'primitives::write_analysis::tests::a_carried_quiet_rule_counts_as_carried_and_no_rule_loaded_records_zero',
    ),
    (
        'M6 a changed file whose copy stood in is judged whole',
        'src/primitives/rule_verdicts.rs',
        '                    edits.narrowed.insert(file.name.clone(), edited);\n',
        '                    edits.rejudged_files.push((file.name.clone(), CopyUnavailable::HistoryUnreadable));\n',
        'primitives::rule_verdicts::tests::an_edit_to_one_rule_judges_it_and_each_rule_naming_it_alone',
    ),
    (
        'M7 a copy that is not the text judged stands in',
        'src/primitives/rule_verdicts.rs',
        '    if digest != recorded {\n',
        '    if false && digest != recorded {\n',
        'primitives::rule_verdicts::tests::a_copy_at_analyzed_against_that_is_not_the_text_judged_judges_the_whole_file',
    ),
    (
        'M8 a rule naming a changed rule keeps its verdict',
        'src/primitives/rule_verdicts.rs',
        '                && rule.names.is_disjoint(&edits.changed)\n',
        '',
        'primitives::rule_verdicts::tests::an_edit_to_one_rule_judges_it_and_each_rule_naming_it_alone',
    ),
    (
        'M9 a rule its file no longer holds stales no rule naming it',
        'src/primitives/rule_verdicts.rs',
        '                    edits\n                        .changed\n                        .extend(judged.into_keys().filter(|id| !file.texts.contains_key(id)));\n',
        '',
        'primitives::rule_verdicts::tests::a_removed_rule_or_a_file_judged_whole_judges_each_rule_naming_one',
    ),
    (
        'M10 a file judged whole stales no rule naming one of its rules',
        'src/primitives/rule_verdicts.rs',
        '                    edits.changed.extend(file.texts.keys().cloned());\n',
        '',
        'primitives::rule_verdicts::tests::a_removed_rule_or_a_file_judged_whole_judges_each_rule_naming_one',
    ),
    (
        'M11 a commit the repository does not hold reads as history unreadable',
        'src/primitives/rule_verdicts.rs',
        '        Err(err) if err.code() == git2::ErrorCode::NotFound => {\n',
        '        Err(err) if false && err.code() == git2::ErrorCode::NotFound => {\n',
        'primitives::rule_verdicts::tests::a_shallow_clone_past_analyzed_against_judges_the_whole_file',
    ),
    (
        'M12 a record naming no analyzed-against is not named so',
        'src/primitives/rule_verdicts.rs',
        '        .ok_or(CopyUnavailable::NoAnalyzedAgainst)?;\n',
        '        .unwrap_or("");\n',
        'primitives::rule_verdicts::tests::a_copy_history_cannot_supply_is_named_with_its_reason',
    ),
    (
        'M13 a file the commit lacks reads as history unreadable',
        'src/primitives/rule_verdicts.rs',
        '        .ok_or(CopyUnavailable::NotAtCommit)?;\n',
        '        .ok_or(CopyUnavailable::HistoryUnreadable)?;\n',
        'primitives::rule_verdicts::tests::a_copy_history_cannot_supply_is_named_with_its_reason',
    ),
    (
        'M14 a rule new to its file keeps a verdict',
        'src/primitives/rule_verdicts.rs',
        '                        .filter(|(id, text)| judged.get(*id) != Some(*text))\n',
        '                        .filter(|(id, text)| judged.get(*id).is_some_and(|old| old != *text))\n',
        'primitives::rule_verdicts::tests::a_reorder_keeps_every_verdict_and_a_rule_moved_in_is_judged',
    ),
    (
        'M15 the rules of a file judged whole read as current',
        'src/primitives/rule_verdicts.rs',
        '            None => !self\n                .rejudged_files\n                .iter()\n                .any(|(name, _)| *name == file.name),\n',
        '            None => true,\n',
        'primitives::rule_verdicts::tests::a_removed_rule_or_a_file_judged_whole_judges_each_rule_naming_one',
    ),
    (
        'M16 the copy is read at the project path, not the work tree path',
        'src/primitives/rule_verdicts.rs',
        '    let text = blob_text(git, &tree, &project_repository.to_git(&file.path))\n',
        '    let text = blob_text(git, &tree, &file.path)\n',
        'primitives::rule_verdicts::tests::a_project_in_a_subdirectory_reads_its_rule_file_s_copy',
    ),
    (
        'M17 discover-rule-files reports no rejudged file',
        'src/primitives/discover_rule_files.rs',
        '        rejudged_files: (!rejudged_files.is_empty()).then_some(rejudged_files),\n',
        '        rejudged_files: None,\n',
        'primitives::discover_rule_files::tests::an_edited_rule_file_is_listed_to_judge_again',
    ),
    (
        'M18 past the read size rejudged-files stays inline',
        'src/primitives/discover_rule_files.rs',
        '            block.rejudged_files = None;\n',
        '',
        'primitives::discover_rule_files::tests::past_the_read_size_rejudged_files_moves_to_the_file',
    ),
    (
        'M19 write-analysis lists a narrowed rule unjudged',
        'src/primitives/write_analysis.rs',
        '    let standing = rule_verdicts::standing(repo, feature_dir, &universe.files);\n',
        '    let standing = rule_verdicts::standing(\n        std::path::Path::new("/nonexistent-088"),\n        feature_dir,\n        &universe.files,\n    );\n',
        'primitives::write_analysis::tests::after_one_rule_s_edit_judging_it_carries_the_rest_of_its_file',
    ),
    (
        'M20 the exec walker judges a changed file whole',
        'src/interpreter/payload.rs',
        '        .map(|feature_dir| rule_verdicts::standing(repo, feature_dir, &digested));\n',
        '        .map(|feature_dir| rule_verdicts::standing(Path::new("/nonexistent-088"), feature_dir, &digested));\n',
        'interpreter::tests::an_exec_step_asks_about_an_edited_rule_and_its_namer_alone',
    ),
    (
        'M21 a record matching as it stands is not written',
        'src/primitives/backfill_section_digest.rs',
        '    if design_subjects_match(feature_dir, record, None) == Some(true) {\n',
        '    if false {\n',
        'primitives::backfill_section_digest::tests::a_current_record_gains_its_digest_and_every_other_byte_stands',
    ),
    (
        'M22 no other status is tried',
        'src/primitives/backfill_section_digest.rs',
        '    let (spec, (current, span)) = spec.zip(status_at)?;\n',
        '    let (spec, (current, span)) = None::<(&str, (String, std::ops::Range<usize>))>?;\n',
        'primitives::backfill_section_digest::tests::a_record_analyzed_at_another_status_gains_that_text_s_digest',
    ),
    (
        'M23 the digests written are the text on disk, not the text that matched',
        'src/primitives/backfill_section_digest.rs',
        '        let sections = section_digest_with(&feature_dir, spec_text.as_deref());\n',
        '        let sections = section_digest_with(&feature_dir, None);\n',
        'primitives::backfill_section_digest::tests::a_record_analyzed_at_another_status_gains_that_text_s_digest',
    ),
    (
        'M24 a record carrying section-digest is written again',
        'src/primitives/backfill_section_digest.rs',
        '        if !record.section_digest.is_empty() {\n',
        '        if false && !record.section_digest.is_empty() {\n',
        'primitives::backfill_section_digest::tests::a_second_run_writes_nothing',
    ),
    (
        'M25 a pinned record is written',
        'src/primitives/backfill_section_digest.rs',
        '        if pinned.contains(&rel.as_str()) {\n',
        '        if false {\n',
        'primitives::backfill_section_digest::tests::a_pinned_record_and_an_unreadable_one_are_named_and_left_untouched',
    ),
    (
        'M26 a refused write fails the call',
        'src/primitives/backfill_section_digest.rs',
        '            Err(PrimitiveError::ChangedSinceRead { .. }) => refused.push(feature),\n',
        '',
        'primitives::backfill_section_digest::tests::a_record_changed_since_it_was_read_is_refused',
    ),
    (
        'M27 the field is placed after the frontmatter',
        'src/primitives/backfill_section_digest.rs',
        '    Ok(format!("{}{field}{}", &content[..at], &content[at..]))\n',
        '    Ok(format!("{content}{field}"))\n',
        'primitives::backfill_section_digest::tests::a_current_record_gains_its_digest_and_every_other_byte_stands',
    ),
    (
        'M28 a record matching at no status is written',
        'src/primitives/backfill_section_digest.rs',
        '            (design_subjects_match(feature_dir, record, Some(&text)) == Some(true))\n',
        '            (design_subjects_match(feature_dir, record, Some(&text)) != Some(true) || true)\n',
        'primitives::backfill_section_digest::tests::a_stale_record_is_named_and_left_as_it_was',
    ),
    (
        'M29 a record with no section is counted written',
        'src/primitives/backfill_section_digest.rs',
        '        if sections.is_empty() {\n',
        '        if false {\n',
        'primitives::backfill_section_digest::tests::a_record_with_no_design_subject_is_counted_and_left',
    ),
    (
        'M30 past the read size the lists stay inline',
        'src/primitives/backfill_section_digest.rs',
        '        result.written = None;\n',
        '',
        'primitives::backfill_section_digest::tests::past_the_read_size_the_lists_move_to_the_file',
    ),
    (
        'M31 an unreadable record is not named',
        'src/primitives/backfill_section_digest.rs',
        '                unreadable.push(unread(reason));\n',
        '',
        'primitives::backfill_section_digest::tests::a_pinned_record_and_an_unreadable_one_are_named_and_left_untouched',
    ),
    (
        'M32 a refused record does not fail the CLI',
        'src/main.rs',
        '    r.refused_count > 0\n}\n',
        '    false\n}\n',
        '--bin ductus tests::a_refused_record_alone_blocks_the_backfill_in_either_form',
    ),
    (
        'M33 the field keeps LF in a CRLF record',
        'src/primitives/backfill_section_digest.rs',
        '        let field = with_line_ending(&field, line_ending_of(&content));\n',
        '',
        '--test crlf_preservation backfill_section_digest_preserves_crlf',
    ),
]
