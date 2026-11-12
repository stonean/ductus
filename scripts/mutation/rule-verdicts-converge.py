"""Mutation list for spec 082: a stored rule verdict is reused while the design
subjects and the rule's file are unchanged (`standing` in
`runtime/src/primitives/rule_verdicts.rs`), `write-analysis` carries, derives
and checks the rule fields (`derive_rule_fields` in `write_analysis.rs`), and
the exec walker asks only about a rule whose verdict is not current
(`split_reused` in `interpreter/mod.rs`).

Each mutation removes the behavior one unit test names, and its filter selects
that test. By behavior: what makes a verdict current is held by M1 to M3 and
M27 (AC1 by M1 to M3, M7 and M9; the rule-file set a record was judged under
by M27; AC5 by M2); the derived unjudged list by M5 and M10 (AC2); the
contradiction note by M6 (AC3); the addressed citation by M4 (AC6, on the
interactive and the exec path); carrying a verdict forward by M7; the tier
count floor by M8; the walker's reuse by M9 and M13; the interactive list by
M11 and M12; the tally's addressed-by and judged set by M14 and M15; the
stored tier by M16 to M18; the refusal of a tier a single-tier rule cannot
fail in by M19 and M20; the gate's hold on unjudged rules by M21 and M22; the
bounded `contradictions` result by M23 and M24; a caller's bare `unexamined`
total by M25; and a single-tier rule's finding counted in its own tier by M26.
AC4 is held by `write_analysis::tests::the_rule_fields_read_back_through_the_loader`
(no mutant: it holds the record's shape, not a behavior one line removes).

Run with `python3 scripts/mutation/run.py
scripts/mutation/rule-verdicts-converge.py [ID ...]`; see run.py for the
format. This file is data, read with ast.literal_eval: keep every value a
literal. An id is never reused: a mutation that no longer applies is removed,
and the next one takes the next free number. When the code a mutation anchors
on changes, update the anchor in the same change; `run.py --check` names every
anchor that no longer matches exactly once.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    # -- what makes a verdict current -------------------------------------
    (
        'M1 the design-subject comparison is dropped',
        'src/primitives/rule_verdicts.rs',
        '    recorded == current\n',
        '    recorded == current || true\n',
        'rule_verdicts::tests::a_design_subject_edit_judges_every_rule_and_a_tasks_or_review_edit_none rule_verdicts::tests::a_design_subject_added_or_removed_judges_every_rule discover_rule_files::tests::a_second_call_over_an_unchanged_spec_lists_no_rule_to_judge',
    ),
    (
        'M2 the rule-digest comparison is dropped',
        'src/primitives/rule_verdicts.rs',
        'let file_current = prior.rule_digest.get(&file.name) == Some(&file.digest);',
        'let file_current = true;',
        'rule_verdicts::tests::an_edited_rule_file_judges_its_own_rules_only write_analysis::tests::an_edited_rule_file_is_judged_again_over_an_unchanged_spec discover_rule_files::tests::an_edited_rule_file_is_listed_to_judge_again interpreter::tests::an_exec_walk_asks_again_about_a_rule_that_was_edited',
    ),
    (
        'M3 a tasks.md edit counts as a design subject',
        'src/primitives/analyze_subjects.rs',
        'matches!(rel_within_feature, "spec.md" | "plan.md" | "data-model.md")',
        'matches!(rel_within_feature, "spec.md" | "plan.md" | "data-model.md" | "tasks.md")',
        'analyze_subjects::tests::a_tasks_or_review_edit_leaves_the_design_subject_digest rule_verdicts::tests::a_design_subject_edit_judges_every_rule_and_a_tasks_or_review_edit_none write_analysis::tests::a_task_tick_between_runs_leaves_the_verdicts_carried discover_rule_files::tests::a_second_call_over_an_unchanged_spec_lists_no_rule_to_judge',
    ),
    (
        'M4 an addressed citation is not checked',
        'src/primitives/write_analysis.rs',
        '                if !rule_verdicts::cite_resolves(feature_dir, cite) =>',
        '                if false && !rule_verdicts::cite_resolves(feature_dir, cite) =>',
        'write_analysis::tests::an_addressed_verdict_whose_citation_does_not_resolve_is_unjudged interpreter::tests::an_exec_pass_citing_a_missing_heading_is_recorded_unjudged',
    ),
    (
        'M5 the derived unjudged list is emptied',
        'src/primitives/write_analysis.rs',
        '                Currency::Stale => fields.unjudged.push(rule.id.clone()),',
        '                Currency::Stale => {}',
        'write_analysis::tests::a_run_judging_fewer_rules_than_were_loaded_names_each_one_unjudged write_analysis::tests::an_edited_rule_file_is_judged_again_over_an_unchanged_spec interpreter::tests::an_exec_walk_that_asks_about_fewer_rules_than_were_loaded_names_each_unjudged',
    ),
    (
        'M6 the contradiction comparison is removed',
        'src/primitives/write_analysis.rs',
        'if let Some(earlier) = rule_verdicts::contradiction(current, verdict) {',
        'if let Some(earlier) = None::<&str> {',
        'write_analysis::tests::a_finding_after_a_quiet_or_addressed_verdict_is_a_recorded_contradiction write_analysis::tests::a_contradiction_over_a_should_rule_holds_nothing',
    ),
    (
        'M7 a current verdict is not carried forward',
        'src/primitives/write_analysis.rs',
        '''                Currency::Fired(verdict) => {
                    fields.verdicts.insert(rule.id.clone(), verdict.clone());
                }
                Currency::NotFired => {}''',
        '''                Currency::Fired(_) => {}
                Currency::NotFired => {}''',
        'write_analysis::tests::a_second_run_over_an_unchanged_spec_carries_every_verdict_and_judges_nothing write_analysis::tests::a_finding_verdict_the_tier_counts_do_not_cover_is_refused',
    ),
    (
        'M8 the tier-count floor is removed',
        'src/primitives/write_analysis.rs',
        '        if supplied < count {',
        '        if false && supplied < count {',
        'write_analysis::tests::a_finding_verdict_the_tier_counts_do_not_cover_is_refused',
    ),
    (
        'M9 the walker asks about a rule whose verdict is current',
        'src/interpreter/mod.rs',
        '''                standing
                    .currency(&rule.id)
                    .is_none_or(|currency| *currency == Currency::Stale)''',
        '''                standing.currency(&rule.id).is_none_or(|_| true)''',
        'interpreter::tests::a_second_exec_walk_over_an_unchanged_spec_asks_about_no_rule interpreter::tests::an_exec_step_reuses_the_current_verdicts_and_asks_about_the_rest interpreter::tests::an_exec_walk_asks_again_about_a_rule_that_was_edited',
    ),
    (
        'M10 the unjudged rules are not counted in rule-assessments-not-checked',
        'src/primitives/write_analysis.rs',
        '            .or_default() += u32::try_from(rules.fields.unjudged.len()).unwrap_or(u32::MAX);',
        '            .or_default() += 0;',
        'write_analysis::tests::a_run_judging_fewer_rules_than_were_loaded_names_each_one_unjudged write_analysis::tests::an_addressed_verdict_whose_citation_does_not_resolve_is_unjudged interpreter::tests::an_exec_walk_that_asks_about_fewer_rules_than_were_loaded_names_each_unjudged',
    ),
    # -- the interactive list ---------------------------------------------
    (
        'M11 discover-rule-files reads a feature without all',
        'src/primitives/discover_rule_files.rs',
        '''    if args.all
        && let Some(feature) = &args.feature''',
        '''    if let Some(feature) = &args.feature''',
        'discover_rule_files::tests::a_feature_without_all_and_rules_adds_no_verdicts_and_a_missing_one_is_refused',
    ),
    (
        'M12 discover-rule-files lists no rule to judge',
        'src/primitives/discover_rule_files.rs',
        '    let judge: Vec<String> = standing.judge().map(|rule| rule.id.clone()).collect();',
        '    let judge: Vec<String> = Vec::new();',
        'discover_rule_files::tests::a_second_call_over_an_unchanged_spec_lists_no_rule_to_judge discover_rule_files::tests::a_rule_the_prior_run_left_unjudged_is_listed_to_judge discover_rule_files::tests::an_unreadable_record_is_named_and_judges_every_rule',
    ),
    # -- the exec tally ---------------------------------------------------
    (
        'M13 a reused finding is not counted in its tier',
        'src/interpreter/mod.rs',
        '                tally.record_reused(stored);',
        '                let _ = stored;',
        'interpreter::tests::a_second_exec_walk_over_an_unchanged_spec_asks_about_no_rule',
    ),
    (
        'M14 addressed-by is ignored',
        'src/interpreter/analyze_tally.rs',
        '            if let Some(cite) = cited {',
        '            if let Some(cite) = cited.filter(|_| false) {',
        'analyze_tally::tests::each_assessment_leaves_its_verdict_and_judges_its_rule interpreter::tests::a_second_exec_walk_over_an_unchanged_spec_asks_about_no_rule',
    ),
    (
        'M15 an asked rule is not judged',
        'src/interpreter/analyze_tally.rs',
        '        self.judged.insert(rule.to_string());',
        '        let _ = rule;',
        'analyze_tally::tests::each_assessment_leaves_its_verdict_and_judges_its_rule interpreter::tests::an_exec_walk_that_asks_about_fewer_rules_than_were_loaded_names_each_unjudged',
    ),
    (
        'M16 the floor reads the rule text tier, not the tier the finding counted in',
        'src/primitives/write_analysis.rs',
        '            owed[usize::from(*tier == FindingTier::Should)] += 1;',
        '            let _ = tier;\n            owed[0] += 1;',
        'write_analysis::tests::a_finding_floors_in_the_tier_it_was_counted_in_not_its_rules_text_tier interpreter::tests::a_mixed_rule_counts_floors_and_is_reused_in_the_tier_it_failed_in',
    ),
    (
        'M17 a finding verdict stores the rule tier, not the tier it counted in',
        'src/interpreter/analyze_tally.rs',
        '                tier: counted,',
        '                tier: if asked == RuleSeverity::Should { FindingTier::Should } else { FindingTier::Must },',
        'interpreter::tests::a_mixed_rule_counts_floors_and_is_reused_in_the_tier_it_failed_in analyze_tally::tests::each_assessment_leaves_its_verdict_and_judges_its_rule',
    ),
    (
        'M18 a reused finding is reported in the rule tier, not its stored tier',
        'src/interpreter/mod.rs',
        '                tally.record_reused(stored);',
        '                let _ = stored;\n                tally.record_reused(FindingTier::Must);',
        'interpreter::tests::a_mixed_rule_counts_floors_and_is_reused_in_the_tier_it_failed_in',
    ),
    (
        'M19 a finding may be labelled in a tier its single-tier rule cannot fail in',
        'src/primitives/write_analysis.rs',
        '                && !rule.mixed\n',
        '                && rule.mixed\n',
        'write_analysis::tests::a_finding_labelled_in_a_tier_its_rule_cannot_fail_in_is_refused',
    ),
    (
        'M20 the tier check ignores the rule tier',
        'src/primitives/write_analysis.rs',
        '                && (rule.tier == RuleSeverity::Should) != (*tier == FindingTier::Should)',
        '                && false',
        'write_analysis::tests::a_finding_labelled_in_a_tier_its_rule_cannot_fail_in_is_refused',
    ),
    (
        'M21 the gate ignores a record naming unjudged rules',
        'src/primitives/check_review_gate.rs',
        '    if analyze.unjudged_rules.is_empty() {',
        '    if true || analyze.unjudged_rules.is_empty() {',
        'primitives::check_review_gate::tests::an_analysis_naming_unjudged_rules_blocks_and_names_them',
    ),
    (
        'M22 the gate blocks on an analysis naming no unjudged rule',
        'src/primitives/check_review_gate.rs',
        '    if analyze.unjudged_rules.is_empty() {',
        '    if false && analyze.unjudged_rules.is_empty() {',
        'primitives::check_review_gate::tests::an_analysis_with_no_unjudged_rules_passes',
    ),
    # -- the bounded result, the bare total, the exec tier, the rule-file set --
    (
        'M23 contradictions stay inline past the read size',
        'src/primitives/write_analysis.rs',
        '        result.contradictions = Vec::new();\n',
        '        let _ = &result.contradictions;\n',
        'write_analysis::tests::contradictions_past_the_read_size_are_in_a_file',
    ),
    (
        'M24 the contradictions count is not set',
        'src/primitives/write_analysis.rs',
        '    result.contradictions_count = result.contradictions.len();\n',
        '',
        'write_analysis::tests::contradictions_within_the_read_size_are_inline_with_their_count write_analysis::tests::contradictions_past_the_read_size_are_in_a_file',
    ),
    (
        'M25 a bare unexamined total is dropped when no breakdown is supplied',
        'src/primitives/write_analysis.rs',
        '    if by_reason.is_empty() && args.unexamined > 0 {',
        '    if false && by_reason.is_empty() && args.unexamined > 0 {',
        'write_analysis::tests::a_bare_total_is_recorded_as_unclassified write_analysis::tests::a_bare_total_survives_a_derived_reason_beside_it',
    ),
    (
        'M26 a single-tier rule takes the host label as its finding tier',
        'src/interpreter/analyze_tally.rs',
        'Some(tier @ (RuleSeverity::Must | RuleSeverity::Should)) if mixed => tier,',
        'Some(tier @ (RuleSeverity::Must | RuleSeverity::Should)) => tier,',
        'analyze_tally::tests::a_single_tier_rule_counts_in_its_own_tier_and_a_mixed_rule_keeps_the_label interpreter::tests::an_exec_walk_counts_a_single_tier_rules_finding_in_the_rules_own_tier',
    ),
    (
        'M27 a verdict stays reusable after a rule file is added or removed',
        'src/primitives/rule_verdicts.rs',
        '    if loaded_now != judged_under {',
        '    if false && loaded_now != judged_under {',
        'rule_verdicts::tests::a_changed_rule_file_set_makes_no_verdict_reusable rule_verdicts::tests::an_unjudged_or_newly_loaded_rule_is_judged rule_verdicts::tests::a_removed_rule_s_verdict_is_dropped',
    ),
]
