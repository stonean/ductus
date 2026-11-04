"""Mutation list for 078's task 14: the re-read budget lives once, as
`RE_READ_BUDGET` (`runtime/src/primitives/fix_rounds.rs`), which
`start-fix-rounds` reports as `budget` and `write-review`'s `budget-stop`
derivation reads.

Run with `python3 scripts/mutation/run.py scripts/mutation/re-read-budget.py
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
        'M1 the budget is three re-reads, not the two AC6 sets',
        'src/primitives/fix_rounds.rs',
        'pub(crate) const RE_READ_BUDGET: usize = 2;',
        'pub(crate) const RE_READ_BUDGET: usize = 3;',
        'write_review::tests::the_budget_stop_needs_two_finding_re_reads_and_a_finding_last',
    ),
    (
        'M2 start-fix-rounds reports no budget',
        'src/primitives/start_fix_rounds.rs',
        '        budget: fix_rounds::RE_READ_BUDGET,',
        '        budget: 0,',
        'start_fix_rounds::tests::the_snapshot_is_the_working_tree',
    ),
]
