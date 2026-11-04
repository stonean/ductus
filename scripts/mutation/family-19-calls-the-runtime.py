"""Mutation list for 066 task 11, *Family 19 calls the runtime*:
`check-review-freshness` answers with the completion gate's own comparison,
sets an absent `review.md` apart from a present one, and never calls a record
it cannot compare current (`runtime/src/primitives/check_review_freshness.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/family-19-calls-the-runtime.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

run.py runs unit tests only. `family_19_reports_each_answer_the_runtime_gives`
in `runtime/tests/mechanical_sweep_parity.rs` holds the script's half: it
fails when `review-freshness.sh` reads a stale or an undeterminable answer as
clean, checked by hand with each mutant.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 every review reads as current',
        'src/primitives/check_review_freshness.rs',
        '            crate::primitives::analyze_subjects::review_freshness(repo, &rel_dir, Some(&review)),',
        '            { let _ = (&rel_dir, &review); RecordFreshness::Current { last_run: String::new(), analyzed_against: String::new() } },',
        'check_review_freshness::tests::a_done_specs_review_is_current_until_a_contract_changes',
    ),
    (
        'M2 an absent review.md reads as present',
        'src/primitives/check_review_freshness.rs',
        '        RecordLoad::Absent => (false, RecordFreshness::NeverRun),',
        '        RecordLoad::Absent => (true, RecordFreshness::NeverRun),',
        'check_review_freshness::tests::a_record_that_cannot_be_compared_is_never_current',
    ),
    (
        'M3 a review.md that does not parse reads as never run',
        'src/primitives/check_review_freshness.rs',
        '            RecordFreshness::Undeterminable {\n                reason: format!("review.md\'s frontmatter does not parse: {reason}"),\n            },',
        '            { let _ = reason; RecordFreshness::NeverRun },',
        'check_review_freshness::tests::a_record_that_cannot_be_compared_is_never_current',
    ),
]
