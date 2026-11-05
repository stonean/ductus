"""Mutation list for 078's `a-reviews-own-record-is-no-fix-round`: the
reviewed feature's own `review.md` is never a fix round's path
(`runtime/src/primitives/fix_rounds.rs`, `round_paths`, which `brief-re-read`
and `check-prescriptions` call), and `brief-re-read` reports the `[review]
re-reads-off` reason whatever the round's kind.

Run with `python3 scripts/mutation/run.py scripts/mutation/own-record-is-no-round.py
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
        'M1 the review record is a round path again',
        'src/primitives/fix_rounds.rs',
        '        .filter(|path| *path != record)',
        '        .filter(|path| !path.is_empty() || record.is_empty())',
        'brief_re_read::tests::the_reviews_own_record_is_no_round_path check_prescriptions::tests::the_reviews_own_record_is_not_unprescribed',
    ),
    (
        'M2 brief-re-read diffs every path, the record included',
        'src/primitives/fix_rounds.rs',
        '        .filter(|path| *path != record)\n',
        '        .filter(|_| true)\n',
        'brief_re_read::tests::the_reviews_own_record_is_no_round_path',
    ),
    (
        'M3 the switch is reported on a re-read round alone',
        'src/primitives/brief_re_read.rs',
        '        re_reads_off,\n        ..BriefReReadResult::default()',
        '        re_reads_off: re_reads_off.filter(|_| kind == RoundKind::ReRead),\n        ..BriefReReadResult::default()',
        'brief_re_read::tests::the_switch_is_reported_on_an_empty_or_exempt_round',
    ),
]
