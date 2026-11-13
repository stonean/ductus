"""Mutation list for the `waived` rule verdict (spec 073, task 3):
`write-analysis` refuses `waived` for a rule with no departure entry and
`finding` or `addressed` for one with an entry, fills every `waived` verdict
from its entry and the entry's history on every write, supplied or carried,
and the exec walker's tally records a fired answer for a departed rule as
`waived`, counted into no tier. Each mutation removes one of those, so a kill
shows its own test holds it.

Run with `python3 scripts/mutation/run.py scripts/mutation/waived-verdict.py
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
        'M1 a waived verdict for a rule with no entry is accepted',
        'src/primitives/write_analysis.rs',
        '(RuleVerdict::Waived { .. }, false) => {',
        '(RuleVerdict::Waived { .. }, false) if false => {',
        'a_waiver_without_an_entry_and_a_finding_against_one_are_refused_with_no_record',
    ),
    (
        'M2 a finding or addressed verdict for a rule with an entry is accepted',
        'src/primitives/write_analysis.rs',
        '(RuleVerdict::Finding { .. } | RuleVerdict::Addressed { .. }, true) => {',
        '(RuleVerdict::Finding { .. } | RuleVerdict::Addressed { .. }, true) if false => {',
        'a_waiver_without_an_entry_and_a_finding_against_one_are_refused_with_no_record',
    ),
    (
        'M3 a waived verdict is not filled from its entry',
        'src/primitives/write_analysis.rs',
        'rule_verdicts::fill_waived(repo, spec_rel, &entries, &mut fields.verdicts);',
        'let _ = spec_rel;',
        'a_fired_must_rule_with_an_entry_is_waived_and_blocks_nothing a_commit_after_an_unattributed_run_attributes_the_next_write',
    ),
    (
        'M4 a carried waiver is not recomputed',
        'src/primitives/write_analysis.rs',
        'rule_verdicts::fill_waived(repo, spec_rel, &entries, &mut fields.verdicts);',
        'if !fields.verdicts.is_empty() && args.rule_verdicts.is_empty() {\n        let _ = spec_rel;\n    } else {\n        rule_verdicts::fill_waived(repo, spec_rel, &entries, &mut fields.verdicts);\n    }',
        'a_commit_after_an_unattributed_run_attributes_the_next_write',
    ),
    (
        'M5 a carried waiver whose entry is gone is kept',
        'src/primitives/write_analysis.rs',
        'Currency::Fired(RuleVerdict::Waived { .. }) if !entries.contains_key(id) => {',
        'Currency::Fired(RuleVerdict::Waived { .. }) if false => {',
        'a_carried_waiver_with_no_entry_is_recorded_unjudged',
    ),
    (
        'M6 the entry names no citation on the verdict',
        'src/primitives/rule_verdicts.rs',
        'cited: entry.cited.clone(),',
        'cited: None,',
        'a_fired_must_rule_with_an_entry_is_waived_and_blocks_nothing',
    ),
    (
        'M7 an unattributed waiver carries a time',
        'src/primitives/rule_verdicts.rs',
        '_ => ("unattributed".to_owned(), None),',
        '_ => ("unattributed".to_owned(), Some("1970-01-01T00:00:00Z".to_owned())),',
        'a_commit_after_an_unattributed_run_attributes_the_next_write',
    ),
    (
        'M8 the record drops the citation of a waived verdict',
        'src/primitives/write_analysis.rs',
        'if let Some(cite) = cited {',
        'if let Some(cite) = cited.as_ref().filter(|_| false) {',
        'an_exec_walk_records_a_departed_rule_that_fires_as_waived',
    ),
    (
        'M9 the tally records a pass citing a heading as no verdict',
        'src/interpreter/analyze_tally.rs',
        'if !passed || cited.is_some() {',
        'if !passed {',
        'a_departed_rule_that_fires_is_waived_and_counts_into_no_tier an_exec_walk_records_a_departed_rule_that_fires_as_waived',
    ),
    (
        'M10 the tally counts a departed rule as a finding',
        'src/interpreter/analyze_tally.rs',
        'if departed {',
        'if false {',
        'a_departed_rule_that_fires_is_waived_and_counts_into_no_tier an_exec_walk_records_a_departed_rule_that_fires_as_waived',
    ),
    (
        'M11 the walker tells the tally of no departure',
        'src/interpreter/mod.rs',
        '.is_some_and(|rules| rules.departed.contains(&rule.id));',
        '.is_some_and(|_| false);',
        'an_exec_walk_records_a_departed_rule_that_fires_as_waived',
    ),
    (
        'M12 a reused waiver is counted as a blocking finding',
        'src/interpreter/mod.rs',
        'RuleVerdict::Addressed { .. } | RuleVerdict::Waived { .. } => None,',
        'RuleVerdict::Addressed { .. } => None,\n                RuleVerdict::Waived { .. } => Some(FindingTier::Must),',
        'an_exec_walk_records_a_departed_rule_that_fires_as_waived',
    ),
    (
        'M20 a write holding no waiver reads history',
        'src/primitives/rule_verdicts.rs',
        'if waived.is_empty() {',
        'if false {',
        'a_write_holding_no_waiver_reads_no_history',
    ),
    (
        'M21 the last entry for a rule waives it',
        'src/primitives/rule_verdicts.rs',
        'by_rule.entry(entry.rule.clone()).or_insert(entry);',
        'by_rule.insert(entry.rule.clone(), entry);',
        'a_rule_two_entries_name_is_waived_by_the_first',
    ),
    (
        'M22 the record renders an absent citation as empty',
        'src/primitives/write_analysis.rs',
        'if let Some(cite) = cited {',
        'if let Some(cite) = Some(cited.as_deref().unwrap_or(\"\")) {',
        'an_entry_with_no_citation_records_no_cited_key',
    ),
    (
        'M23 a waiver below the repository root is not attributed',
        'src/primitives/rule_verdicts.rs',
        'departures::attribute(repo, spec_rel, &waived);',
        'departures::attribute(repo, &format!("../{spec_rel}"), &waived);',
        'a_waiver_in_a_project_below_the_repository_root_is_attributed',
    ),
    (
        'M13 the section lists a SHOULD rule before a MUST rule',
        'src/primitives/write_analysis.rs',
        ['(a.tier != FindingTier::Must, &a.rule)', '(b.tier != FindingTier::Must, &b.rule)'],
        ['(a.tier == FindingTier::Must, &a.rule)', '(b.tier == FindingTier::Must, &b.rule)'],
        'waived_rules_list_must_first_then_the_rest_each_in_id_order',
    ),
    (
        'M14 the body renders no Waived rules section',
        'src/primitives/write_analysis.rs',
        'if waived.is_empty() {\n        return String::new();',
        'if true {\n        return String::new();',
        'a_waived_rule_and_a_satisfied_one_differ_in_the_record_and_the_result waived_rules_list_must_first_then_the_rest_each_in_id_order',
    ),
    (
        'M15 the result carries no waived list',
        'src/primitives/write_analysis.rs',
        '            waived: rules.fields.waived,\n',
        '            waived: Vec::new(),\n',
        'a_waived_rule_and_a_satisfied_one_differ_in_the_record_and_the_result waived_within_the_read_size_is_inline_with_its_count',
    ),
    (
        'M16 the bounded file omits the waived list',
        'src/primitives/write_analysis.rs',
        '        "waived": &result.waived,\n',
        '        "waived": Vec::<WaivedRule>::new(),\n',
        'waived_past_the_read_size_is_in_a_file',
    ),
    (
        'M17 the file form keeps waived inline',
        'src/primitives/write_analysis.rs',
        '        result.waived = Vec::new();\n',
        '',
        'waived_past_the_read_size_is_in_a_file',
    ),
    (
        'M18 the waived count is not set',
        'src/primitives/write_analysis.rs',
        '    result.waived_count = result.waived.len();\n',
        '',
        'a_waived_rule_and_a_satisfied_one_differ_in_the_record_and_the_result waived_past_the_read_size_is_in_a_file',
    ),
    (
        'M19 the section drops an unattributed waiver\'s attribution text',
        'src/primitives/write_analysis.rs',
        '"- {} ({tier}) — {} — decided by {}",',
        '"- {} ({tier}) — {} — by {}",',
        'an_unattributed_waiver_renders_without_a_time',
    ),
    (
        'M24 the email is rendered bare, outside a code span',
        'src/primitives/write_analysis.rs',
        '                code_span(&rule.decided_by)\n',
        '                single_line(&rule.decided_by)\n',
        'a_record_holding_an_attributed_waiver_passes_the_project_markdown_lint',
    ),
    (
        'M25 a record with no waived rule gains a blank line',
        'src/primitives/write_analysis.rs',
        '        return String::new();\n    }\n    let lines: Vec<String> = waived',
        '        return "\\n".to_string();\n    }\n    let lines: Vec<String> = waived',
        'a_waived_rule_and_a_satisfied_one_differ_in_the_record_and_the_result',
    ),
    (
        'M26 a reason is trimmed of a leading list marker',
        'src/primitives/write_analysis.rs',
        '                single_line(&rule.reason),\n',
        '                plain(&rule.reason),\n',
        'a_reason_is_rendered_verbatim_whatever_it_starts_with',
    ),
    (
        'M27 the section breaks a tier tie by heading order, not rule ID',
        'src/primitives/write_analysis.rs',
        '(a.tier != FindingTier::Must, &a.rule).cmp(&(b.tier != FindingTier::Must, &b.rule))',
        '(a.tier != FindingTier::Must).cmp(&(b.tier != FindingTier::Must))',
        'waived_rules_are_ordered_by_id_whatever_the_heading_order',
    ),
    (
        'M28 a backtick in a citation ends its code span',
        'src/primitives/write_analysis.rs',
        ".replace('`', \"'\")",
        '',
        'a_backtick_in_a_citation_renders_inside_one_code_span',
    ),
]
