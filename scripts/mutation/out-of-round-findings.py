"""Mutation list for 078's task 12: a finding a re-read labelled MUST or
SHOULD on a path its round did not change counts as a NOTE, by the round's
own `round.toml` (AC4; `runtime/src/primitives/fix_rounds.rs`,
`demote_out_of_round`, which `read-re-read` and `check-prescriptions` call).

Run with `python3 scripts/mutation/run.py scripts/mutation/out-of-round-findings.py
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
        'M1 a finding outside the round keeps its severity',
        'src/primitives/fix_rounds.rs',
        '        if finding.severity != ReReadSeverity::Note && !paths.contains(&path) {',
        '        if finding.severity != ReReadSeverity::Note && paths.is_empty() {',
        'read_re_read::tests::a_finding_outside_the_round_counts_as_a_note check_prescriptions::tests::a_finding_outside_the_round_leaves_nothing_open',
    ),
    (
        'M2 check-prescriptions counts a finding outside the round as open',
        'src/primitives/check_prescriptions.rs',
        '    fix_rounds::demote_out_of_round(&mut findings, &record.paths);',
        '    let _ = (&mut findings, &record.paths);',
        'check_prescriptions::tests::a_finding_outside_the_round_leaves_nothing_open',
    ),
    (
        'M3 a complete result with no round reads as complete',
        'src/primitives/read_re_read.rs',
        '                            state: ReReadState::Unparseable,\n                            problem: Some(problem),\n                            ..ReadReReadResult::default()\n                        },\n                    );',
        '                            state: ReReadState::Complete,\n                            problem: Some(problem),\n                            ..ReadReReadResult::default()\n                        },\n                    );',
        'read_re_read::tests::a_result_without_its_round_is_unparseable',
    ),
    (
        'M4 a finding path is compared as the re-read spelled it',
        'src/primitives/fix_rounds.rs',
        '        let path = lexical(&finding.path);',
        '        let path = finding.path.clone();',
        'fix_rounds::tests::a_round_path_spelled_another_way_is_not_demoted',
    ),
]
