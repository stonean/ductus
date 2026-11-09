"""Mutation list for 066's `the-fix-round-loop-is-recorded-and-gated`: the
fix-round loop's record and gate (spec 078). `write-review` derives the six
loop fields from its `fix-rounds` argument, refuses a malformed round,
applies the `[review] re-reads-off` reason, and appends the loop's account to
a Summary the caller supplies (`runtime/src/primitives/write_review.rs`); `check-review-gate`'s fix-round check
blocks an un-re-read round without a waiver and its passing notice names a
record without the fields (`runtime/src/primitives/check_review_gate.rs`);
`invalidate-review` drops the fields with the run
(`runtime/src/primitives/invalidate_review.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/fix-round-record.py
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
        'M1 re-reads are counted as diff-checked rounds',
        'src/primitives/write_review.rs',
        '        re_reads: count(FixRoundCheck::ReRead),',
        '        re_reads: count(FixRoundCheck::DiffChecked),',
        'write_review::tests::each_loop_field_is_derived_from_the_rounds',
    ),
    (
        'M2 the budget stop ignores whether the last re-read found anything',
        'src/primitives/write_review.rs',
        'budget_stop: with_findings >= fix_rounds::RE_READ_BUDGET\n            && results.last() == Some(&ReReadOutcome::Findings),',
        'budget_stop: with_findings >= fix_rounds::RE_READ_BUDGET,',
        'write_review::tests::the_budget_stop_needs_two_finding_re_reads_and_a_finding_last',
    ),
    (
        'M3 the budget stop needs one re-read with findings, not two',
        'src/primitives/write_review.rs',
        'budget_stop: with_findings >= fix_rounds::RE_READ_BUDGET\n            && results.last() == Some(&ReReadOutcome::Findings),',
        'budget_stop: with_findings >= 1\n            && results.last() == Some(&ReReadOutcome::Findings),',
        'write_review::tests::the_budget_stop_needs_two_finding_re_reads_and_a_finding_last',
    ),
    (
        'M4 the switch waives no round',
        'src/primitives/write_review.rs',
        '        match own.or(switch) {',
        '        match own {',
        'write_review::tests::the_switch_waives_each_round_left_without_a_reason',
    ),
    (
        "M5 the switch overrides a round's own reason",
        'src/primitives/write_review.rs',
        '        match own.or(switch) {',
        '        match switch.or(own) {',
        'write_review::tests::the_switch_waives_each_round_left_without_a_reason',
    ),
    (
        'M6 an absent list writes the loop fields as an empty one',
        'src/primitives/write_review.rs',
        '    let Some(rounds) = args.fix_rounds.as_deref() else {\n        return Ok(None);\n    };',
        '    let rounds = args.fix_rounds.as_deref().unwrap_or_default();',
        'write_review::tests::an_absent_list_writes_no_loop_field_and_an_empty_one_writes_zeros',
    ),
    (
        'M7 a run without rounds is halted by a malformed switch',
        'src/primitives/write_review.rs',
        '    let loop_record = loop_record_of(args, repo)?;',
        '    let _ = fix_rounds::re_reads_off(repo)?;\n    let loop_record = loop_record_of(args, repo)?;',
        'write_review::tests::a_malformed_switch_halts_only_a_run_with_rounds',
    ),
    (
        'M8 a malformed switch no longer halts a run with rounds',
        'src/primitives/write_review.rs',
        '    let switch = fix_rounds::re_reads_off(repo)?;',
        '    let switch = fix_rounds::re_reads_off(repo).unwrap_or(None);',
        'write_review::tests::a_malformed_switch_halts_only_a_run_with_rounds',
    ),
    (
        'M9 a waived round needs no decided-by',
        'src/primitives/write_review.rs',
        '            .is_none_or(|who| who.trim().is_empty())',
        '            .is_some_and(|who| who.trim().is_empty())',
        'write_review::tests::each_malformed_round_is_refused_before_any_write',
    ),
    (
        'M10 a result is accepted on a round that was not re-read',
        'src/primitives/write_review.rs',
        '            (_, Some(_)) => return Err(invalid("result", "only a re-read round has a result")),',
        '            (_, Some(_)) => {}',
        'write_review::tests::each_malformed_round_is_refused_before_any_write',
    ),
    (
        'M11 a re-read round without a result is accepted',
        'src/primitives/write_review.rs',
        '            (FixRoundCheck::ReRead, None) => {',
        '            (FixRoundCheck::ReRead, None) if false => {',
        'write_review::tests::each_malformed_round_is_refused_before_any_write',
    ),
    (
        'M12 a reason is accepted on a round that was re-read, checked or exempt',
        'src/primitives/write_review.rs',
        '        if round.check != FixRoundCheck::NotReRead {\n            return Err(invalid(',
        '        if false && round.check != FixRoundCheck::NotReRead {\n            return Err(invalid(',
        'write_review::tests::each_malformed_round_is_refused_before_any_write',
    ),
    (
        'M13 a blank reason waives its round',
        'src/primitives/write_review.rs',
        '        if reason.trim().is_empty() {\n            return Err(invalid("reason", "a waiver states its reason"));',
        '        if false && reason.trim().is_empty() {\n            return Err(invalid("reason", "a waiver states its reason"));',
        'write_review::tests::each_malformed_round_is_refused_before_any_write',
    ),
    (
        'M14 a reason spanning lines is accepted',
        'src/primitives/write_review.rs',
        '        super::validate_single_line("write-review", &format!("fix-rounds[{idx}].reason"), reason)?;\n',
        '',
        'write_review::tests::each_malformed_round_is_refused_before_any_write',
    ),
    (
        'M15 a misspelled key in a round is ignored',
        'src/schema/primitives.rs',
        '#[serde(rename_all = "kebab-case", deny_unknown_fields)]\npub struct FixRound {',
        '#[serde(rename_all = "kebab-case")]\npub struct FixRound {',
        'write_review::tests::each_malformed_round_is_refused_before_any_write',
    ),
    (
        'M16 the round waivers are not written',
        'src/primitives/write_review.rs',
        '    if record.waivers.is_empty() {\n        return;\n    }',
        '    if true {\n        return;\n    }',
        'write_review::tests::each_loop_field_is_derived_from_the_rounds',
    ),
    (
        'M17 a waiver names the round before its own',
        'src/primitives/write_review.rs',
        '                round: u32::try_from(n).unwrap_or(u32::MAX),',
        '                round: u32::try_from(n - 1).unwrap_or(u32::MAX),',
        'write_review::tests::each_loop_field_is_derived_from_the_rounds',
    ),
    (
        'M18 the Summary leaves the rounds out',
        'src/primitives/write_review.rs',
        '        summary.push_str(&loop_summary(loop_record));',
        '        let _ = loop_summary(loop_record);',
        'write_review::tests::each_loop_field_is_derived_from_the_rounds write_review::tests::an_absent_list_writes_no_loop_field_and_an_empty_one_writes_zeros',
    ),
    (
        'M19 an un-re-read round no longer holds done',
        'src/primitives/check_review_gate.rs',
        '    if let Some(blocked) = fix_round_block(review, rel_dir, project) {\n        return Err(blocked);\n    }',
        '    let _ = fix_round_block;',
        'check_review_gate::tests::an_un_re_read_round_without_a_waiver_holds_done check_review_gate::tests::fewer_waivers_than_un_re_read_rounds_block_whatever_else_the_record_says',
    ),
    (
        'M20 a waiver with a blank reason releases its round',
        'src/primitives/check_review_gate.rs',
        '        .filter(|waiver| !waiver.reason.trim().is_empty())',
        '        .filter(|_| true)',
        'check_review_gate::tests::a_waived_round_passes_and_a_blank_waiver_waives_nothing',
    ),
    (
        'M21 a round its waiver covers still holds done',
        'src/primitives/check_review_gate.rs',
        '    if not_re_read <= waived {',
        '    if not_re_read < waived {',
        'check_review_gate::tests::a_waived_round_passes_and_a_blank_waiver_waives_nothing',
    ),
    (
        'M22 a record without the loop fields passes in silence',
        'src/primitives/check_review_gate.rs',
        '    if !fix_rounds_recorded {',
        '    if false && !fix_rounds_recorded {',
        'check_review_gate::tests::a_record_without_the_loop_fields_passes_and_says_so',
    ),
    (
        'M23 a record with the loop fields is reported as lacking them',
        'src/primitives/check_review_gate.rs',
        '            review.not_re_read.is_some(),\n            owed_unknown.as_deref(),',
        '            false,\n            owed_unknown.as_deref(),',
        'check_review_gate::tests::a_record_without_the_loop_fields_passes_and_says_so',
    ),
    (
        'M24 the fix-round check runs before the outstanding-findings check',
        'src/primitives/check_review_gate.rs',
        '    let gating = review\n        .must_violations\n',
        '    if let Some(blocked) = fix_round_block(review, rel_dir, project) {\n        return Err(blocked);\n    }\n    let gating = review\n        .must_violations\n',
        'check_review_gate::tests::an_outstanding_should_is_reported_before_an_un_re_read_round',
    ),
    (
        'M25 an invalidated record keeps its round waivers',
        'src/primitives/invalidate_review.rs',
        [
            '    const SCALARS: [&str; 24] = [',
            '        "round-waivers",\n',
        ],
        [
            '    const SCALARS: [&str; 23] = [',
            '',
        ],
        'invalidate_review::tests::the_fix_round_fields_go_with_the_run',
    ),
    (
        'M26 an invalidated record keeps its un-re-read count',
        'src/primitives/invalidate_review.rs',
        [
            '    const SCALARS: [&str; 24] = [',
            '        "not-re-read",\n',
        ],
        [
            '    const SCALARS: [&str; 23] = [',
            '',
        ],
        'invalidate_review::tests::the_fix_round_fields_go_with_the_run',
    ),
    (
        'M27 an invalidated record keeps its excluded count',
        'src/primitives/invalidate_review.rs',
        [
            '    const SCALARS: [&str; 24] = [',
            '        "excluded",\n',
        ],
        [
            '    const SCALARS: [&str; 23] = [',
            '',
        ],
        'invalidate_review::tests::a_runs_excluded_count_breaks_and_open_mark_go_and_its_open_findings_stay',
    ),
    (
        'M28 an invalidated record keeps its broken contracts',
        'src/primitives/invalidate_review.rs',
        [
            '    const SCALARS: [&str; 24] = [',
            '        "contract-breaks",\n',
        ],
        [
            '    const SCALARS: [&str; 23] = [',
            '',
        ],
        'invalidate_review::tests::a_runs_excluded_count_breaks_and_open_mark_go_and_its_open_findings_stay',
    ),
    (
        'M29 an invalidated record keeps its open mark',
        'src/primitives/invalidate_review.rs',
        [
            '    const SCALARS: [&str; 24] = [',
            '        "fix-rounds-open",\n',
        ],
        [
            '    const SCALARS: [&str; 23] = [',
            '',
        ],
        'invalidate_review::tests::a_runs_excluded_count_breaks_and_open_mark_go_and_its_open_findings_stay',
    ),
    (
        'M30 an invalidated record drops what it left open',
        'src/primitives/invalidate_review.rs',
        [
            '    const SCALARS: [&str; 24] = [',
            '        "excluded",\n',
        ],
        [
            '    const SCALARS: [&str; 25] = [',
            '        "excluded",\n        "open-findings",\n',
        ],
        'invalidate_review::tests::a_runs_excluded_count_breaks_and_open_mark_go_and_its_open_findings_stay',
    ),
    (
        "M31 a Summary the caller supplies drops the loop's account",
        'src/primitives/write_review.rs',
        '            Some(loop_record) => format!("{} {}", summary.trim_end(), loop_summary(loop_record)),',
        '            Some(_) => summary.to_string(),',
        'write_review::tests::a_supplied_summary_carries_the_loops_account',
    ),
]
